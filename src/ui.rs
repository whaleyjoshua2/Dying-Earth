//! The interface (spec 17): views, the top bar, panels, popups, the Battle Report, and picking.

use crate::app::*;
use crate::geo;
use crate::icons::Icons;
use crate::scene::{Globe, MainCamera, SceneHandles, GLOBE_RADIUS};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};
use dying_earth_engine::combat::first_round_odds;
use dying_earth_engine::*;
use egui::{Color32, FontId, Pos2, RichText, Ui};

/// Ticket #55: the Temperature bar on the Climate Panel. It runs from the Base Temperature to the
/// Collapse Line, with a notch for every Break across the whole bar and a shorter one along the
/// foot for every Sea Level threshold and for Antarctica's opening; the notches already crossed are
/// filled and the ones ahead are thin and dim; the Temperature now carries a filled marker and the
/// Temperature the CO2 Stock has already committed the world to a hollow one, with the warming
/// between them shaded; and the line beneath names the next Break ahead.
/// Ticket #153 (version 0.07.4): **the Emissions history**, a hand-painted line chart of every
/// Climate phase so far. The designer: *"mouse over on emissions on top bar should proc a history
/// graph."* Three lines against a plain zero line: what the world emitted, what the Natural Sink and
/// the Scrubbers removed, and the net between them, which is the top bar's figure and the CO2
/// Stock's change each turn; a red tick on the turn axis where a Break fired; the last net figure
/// written at the line's end. Drawn at whatever size the caller allots -- small in the bar's hover,
/// wide on the Climate Panel -- with no charting crate, as the Temperature bar is drawn.
fn emissions_history(ui: &mut Ui, game: &Game, size: egui::Vec2) {
    const EMITTED: Color32 = Color32::from_rgb(232, 140, 90);
    const REMOVED: Color32 = Color32::from_rgb(110, 190, 130);
    const NET: Color32 = Color32::from_rgb(235, 235, 240);
    const BREAK: Color32 = Color32::from_rgb(236, 88, 76);
    let h = &game.climate.history;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        ui.label(RichText::new("emitted").color(EMITTED).small());
        ui.label(RichText::new("removed").color(REMOVED).small());
        ui.label(RichText::new("net").color(NET).small());
        ui.label(RichText::new("ppm a turn; a red tick is a Break").weak().small());
    });
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, 3.0, Color32::from_rgb(38, 38, 44));
    if h.is_empty() {
        painter.text(rect.center(), egui::Align2::CENTER_CENTER, "No turn resolved yet.", FontId::proportional(12.0), Color32::from_gray(150));
        return;
    }
    // Room at the right for the last figure and at the bottom for the turn axis.
    let plot = egui::Rect::from_min_max(rect.min + egui::vec2(6.0, 6.0), rect.max - egui::vec2(44.0, 20.0));
    let emitted: Vec<f64> = h.iter().map(|r| r.breakdown.total()).collect();
    let removed: Vec<f64> = h.iter().map(|r| r.breakdown.total_sink()).collect();
    let net: Vec<f64> = h.iter().map(|r| r.breakdown.net()).collect();
    let top = emitted.iter().chain(&removed).chain(&net).cloned().fold(0.0_f64, f64::max).max(1.0) * 1.08;
    let bottom = net.iter().cloned().fold(0.0_f64, f64::min).min(0.0) * 1.08;
    let (first, last) = (h[0].turn, h[h.len() - 1].turn);
    let span = (last.max(first + 1) - first) as f32;
    let x = |turn: u32| plot.left() + (turn - first) as f32 / span * plot.width();
    let y = |v: f64| plot.bottom() - (((v - bottom) / (top - bottom)) as f32) * plot.height();
    // The zero line is the point of the picture: above it the Stock rose, below it fell.
    painter.line_segment([Pos2::new(plot.left(), y(0.0)), Pos2::new(plot.right(), y(0.0))], egui::Stroke::new(1.0, Color32::from_gray(150)));
    // Its label only when the line stands clear of the turn axis, which it does not while nothing
    // has yet gone below zero.
    if plot.bottom() - y(0.0) > 10.0 {
        painter.text(Pos2::new(plot.right() + 3.0, y(0.0)), egui::Align2::LEFT_CENTER, "0", FontId::proportional(10.0), Color32::from_gray(150));
    }
    let line = |values: &[f64], colour: Color32, width: f32| {
        let points: Vec<Pos2> = h.iter().zip(values).map(|(r, v)| Pos2::new(x(r.turn), y(*v))).collect();
        if points.len() == 1 {
            painter.circle_filled(points[0], width + 1.0, colour);
        } else {
            painter.add(egui::Shape::line(points, egui::Stroke::new(width, colour)));
        }
    };
    line(&emitted, EMITTED, 1.2);
    line(&removed, REMOVED, 1.2);
    line(&net, NET, 2.0);
    for r in h.iter().filter(|r| !r.breaks.is_empty()) {
        let bx = x(r.turn);
        painter.line_segment([Pos2::new(bx, plot.bottom() + 2.0), Pos2::new(bx, plot.bottom() + 6.0)], egui::Stroke::new(2.0, BREAK));
    }
    painter.text(Pos2::new(plot.left(), rect.bottom() - 2.0), egui::Align2::LEFT_BOTTOM, game.date(first).text(), FontId::proportional(10.0), Color32::from_gray(150));
    if last > first {
        painter.text(Pos2::new(plot.right(), rect.bottom() - 2.0), egui::Align2::RIGHT_BOTTOM, game.date(last).text(), FontId::proportional(10.0), Color32::from_gray(150));
    }
    let end = net[net.len() - 1];
    painter.text(Pos2::new(plot.right() + 3.0, y(end)), egui::Align2::LEFT_CENTER, format!("{end:+.1}"), FontId::proportional(11.0), NET);
}

/// Ticket #166 (version 0.07.5): **the population history**, the third of the top bar's charts. The
/// designer: *"get a mouse over graph for the population as well."* Two lines on **two scales** --
/// Earth read against the left of the chart and space against the right, each with its own ends
/// written small at its own side -- because Earth is counted in the hundreds of units and space in
/// single figures, and on one scale the space line lies flat on the floor for the whole game and
/// says nothing. Drawn that way the chart says the thing worth seeing: Earth falling while space
/// rises. The Breaks are ticked red on the turn axis, as on its two siblings, because the heat is
/// what takes the people.
fn population_history(ui: &mut Ui, game: &Game, size: egui::Vec2) {
    const EARTH: Color32 = Color32::from_rgb(150, 190, 240);
    const SPACE: Color32 = Color32::from_rgb(235, 235, 240);
    const BREAK: Color32 = Color32::from_rgb(236, 88, 76);
    let h = &game.climate.history;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        ui.label(RichText::new("Earth").color(EARTH).small());
        ui.label(RichText::new("space").color(SPACE).small());
        ui.label(RichText::new("on scales of their own; a red tick is a Break").weak().small());
    });
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, 3.0, Color32::from_rgb(38, 38, 44));
    if h.is_empty() {
        painter.text(rect.center(), egui::Align2::CENTER_CENTER, "No turn resolved yet.", FontId::proportional(12.0), Color32::from_gray(150));
        return;
    }
    // Room at both sides for a scale's ends, and at the bottom for the turn axis.
    let plot = egui::Rect::from_min_max(rect.min + egui::vec2(34.0, 6.0), rect.max - egui::vec2(34.0, 20.0));
    let (first, last) = (h[0].turn, h[h.len() - 1].turn);
    let span = (last.max(first + 1) - first) as f32;
    let x = |turn: u32| plot.left() + (turn - first) as f32 / span * plot.width();
    // Each line on its own range, with a margin, so both are legible whatever the other does.
    let scale = |lo: f64, hi: f64| {
        let pad = ((hi - lo) * 0.15).max(0.5);
        (lo - pad, hi + pad)
    };
    let (e_lo, e_hi) = {
        let (lo, hi) = h.iter().fold((f64::MAX, f64::MIN), |(a, b), r| (a.min(r.earth_population), b.max(r.earth_population)));
        scale(lo, hi)
    };
    let (s_lo, s_hi) = {
        let (lo, hi) = h.iter().fold((f64::MAX, f64::MIN), |(a, b), r| (a.min(r.space_population as f64), b.max(r.space_population as f64)));
        scale(lo, hi)
    };
    let y = |v: f64, lo: f64, hi: f64| plot.bottom() - (((v - lo) / (hi - lo)).clamp(0.0, 1.0) as f32) * plot.height();
    let line = |values: Vec<Pos2>, colour: Color32| {
        if values.len() == 1 {
            painter.circle_filled(values[0], 3.0, colour);
        } else {
            painter.add(egui::Shape::line(values, egui::Stroke::new(2.0, colour)));
        }
    };
    line(h.iter().map(|r| Pos2::new(x(r.turn), y(r.earth_population, e_lo, e_hi))).collect(), EARTH);
    line(h.iter().map(|r| Pos2::new(x(r.turn), y(r.space_population as f64, s_lo, s_hi))).collect(), SPACE);
    for r in h.iter().filter(|r| !r.breaks.is_empty()) {
        let bx = x(r.turn);
        painter.line_segment([Pos2::new(bx, plot.bottom() + 2.0), Pos2::new(bx, plot.bottom() + 6.0)], egui::Stroke::new(2.0, BREAK));
    }
    // Each scale's ends at its own side, in its own colour, in people rather than units.
    let small = FontId::proportional(9.0);
    painter.text(Pos2::new(plot.left() - 3.0, plot.top()), egui::Align2::RIGHT_TOP, Game::people_text(e_hi), small.clone(), EARTH);
    painter.text(Pos2::new(plot.left() - 3.0, plot.bottom()), egui::Align2::RIGHT_BOTTOM, Game::people_text(e_lo.max(0.0)), small.clone(), EARTH);
    painter.text(Pos2::new(plot.right() + 3.0, plot.top()), egui::Align2::LEFT_TOP, Game::people_text(s_hi), small.clone(), SPACE);
    painter.text(Pos2::new(plot.right() + 3.0, plot.bottom()), egui::Align2::LEFT_BOTTOM, Game::people_text(s_lo.max(0.0)), small.clone(), SPACE);
    painter.text(Pos2::new(plot.left(), rect.bottom() - 2.0), egui::Align2::LEFT_BOTTOM, game.date(first).text(), small.clone(), Color32::from_gray(150));
    if last > first {
        painter.text(Pos2::new(plot.right(), rect.bottom() - 2.0), egui::Align2::RIGHT_BOTTOM, game.date(last).text(), small, Color32::from_gray(150));
    }
}

/// Ticket #264 (version 0.08.4): **the Victory history**, the Emissions history's fourth sibling,
/// on the Faction window under the Victory progress block: one Faction's progress turn by turn --
/// its score, the lower of its two parts' fractions, in the Faction's own colour -- and a second
/// line for its share of the table's Blame, on a scale of its own at the right with the fair
/// quarter as a faint line across it. The Breaks ticked red on the date axis as every sibling has
/// them; Antarctica's opening ticked in the ice's blue on every Faction's chart; the Faction's gate
/// Tech done, and the Archive complete, ticked in white on the chart of the Faction they belong
/// to. Drawn from the per-seat record the Climate phase writes beside the Emissions record.
fn victory_history(ui: &mut Ui, game: &Game, seat: Seat, size: egui::Vec2) {
    const BLAME: Color32 = Color32::from_rgb(190, 150, 210);
    const BREAK: Color32 = Color32::from_rgb(236, 88, 76);
    const ICE: Color32 = Color32::from_rgb(150, 195, 235);
    const MARK: Color32 = Color32::from_rgb(235, 235, 240);
    let own = rgb(game.tables.faction(game.kind(seat)).colour);
    let h = &game.seat(seat).victory_history;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        ui.label(RichText::new("progress").color(own).small());
        ui.label(RichText::new("Blame share").color(BLAME).small());
        ui.label(RichText::new("ticks: a Break red, Antarctica blue, the gate Tech and the Archive white").weak().small());
    });
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, 3.0, Color32::from_rgb(38, 38, 44));
    if h.is_empty() {
        painter.text(rect.center(), egui::Align2::CENTER_CENTER, "No turn resolved yet.", FontId::proportional(12.0), Color32::from_gray(150));
        return;
    }
    let plot = egui::Rect::from_min_max(rect.min + egui::vec2(34.0, 6.0), rect.max - egui::vec2(34.0, 20.0));
    let (first, last) = (h[0].turn, h[h.len() - 1].turn);
    let span = (last.max(first + 1) - first) as f32;
    let x = |turn: u32| plot.left() + (turn - first) as f32 / span * plot.width();
    // Both lines run 0 to 1 and are drawn on that whole range, so a chart reads the same for every
    // Faction and across a game: a rising line is progress, and the height means the same thing on
    // turn 3 as on turn 30.
    let y = |v: f64| plot.bottom() - (v.clamp(0.0, 1.0) as f32) * plot.height();
    // The fair quarter of the table's Blame, the one figure on the share's axis that means anything.
    let fair = game.tables.influence.blame.fair_share;
    painter.line_segment([Pos2::new(plot.left(), y(fair)), Pos2::new(plot.right(), y(fair))], egui::Stroke::new(1.0, BLAME.gamma_multiply(0.35)));
    let line = |values: Vec<Pos2>, colour: Color32| {
        if values.len() == 1 {
            painter.circle_filled(values[0], 3.0, colour);
        } else {
            painter.add(egui::Shape::line(values, egui::Stroke::new(2.0, colour)));
        }
    };
    line(h.iter().map(|r| Pos2::new(x(r.turn), y(r.blame_share))).collect(), BLAME);
    line(h.iter().map(|r| Pos2::new(x(r.turn), y(r.score))).collect(), own);
    // The ticks along the foot: a Break in red from the world's record on the same turn; the first
    // turn each flag stands, in its colour, a little taller so two on one turn both show.
    let tick = |turn: u32, colour: Color32, tall: f32| {
        let bx = x(turn);
        painter.line_segment([Pos2::new(bx, plot.bottom() + 2.0), Pos2::new(bx, plot.bottom() + 2.0 + tall)], egui::Stroke::new(2.0, colour));
    };
    for r in game.climate.history.iter().filter(|r| !r.breaks.is_empty() && r.turn >= first && r.turn <= last) {
        tick(r.turn, BREAK, 4.0);
    }
    let first_where = |pick: &dyn Fn(&dying_earth_engine::VictoryRecord) -> bool| h.iter().find(|r| pick(r)).map(|r| r.turn);
    if let Some(t) = first_where(&|r| r.antarctica_open) {
        tick(t, ICE, 7.0);
    }
    if let Some(t) = first_where(&|r| r.gate_done).filter(|_| game.tables.victory_gate(game.kind(seat)).is_some()) {
        tick(t, MARK, 10.0);
    }
    if let Some(t) = first_where(&|r| r.archive_complete) {
        tick(t, MARK, 10.0);
    }
    // Each scale's ends at its own side, in its own colour.
    let small = FontId::proportional(9.0);
    painter.text(Pos2::new(plot.left() - 3.0, plot.top()), egui::Align2::RIGHT_TOP, "100%", small.clone(), own);
    painter.text(Pos2::new(plot.left() - 3.0, plot.bottom()), egui::Align2::RIGHT_BOTTOM, "0%", small.clone(), own);
    painter.text(Pos2::new(plot.right() + 3.0, plot.top()), egui::Align2::LEFT_TOP, "all", small.clone(), BLAME);
    painter.text(Pos2::new(plot.right() + 3.0, y(fair)), egui::Align2::LEFT_CENTER, "fair", small.clone(), BLAME);
    painter.text(Pos2::new(plot.right() + 3.0, plot.bottom()), egui::Align2::LEFT_BOTTOM, "none", small.clone(), BLAME);
    painter.text(Pos2::new(plot.left(), rect.bottom() - 2.0), egui::Align2::LEFT_BOTTOM, game.date(first).text(), small.clone(), Color32::from_gray(150));
    if last > first {
        painter.text(Pos2::new(plot.right(), rect.bottom() - 2.0), egui::Align2::RIGHT_BOTTOM, game.date(last).text(), small, Color32::from_gray(150));
    }
}

/// Ticket #158 (version 0.07.4): **the Temperature history**, the Emissions history's sibling on
/// the top bar's Temperature figure: the Temperature turn by turn on the data's own range (the
/// designer's choice over the base-to-Collapse scale, for the detail); the Breaks' Temperatures
/// that fall in the range as faint lines across it, the Collapse line when it does, and the Breaks
/// fired ticked red on the turn axis; the last figure at the line's end. The heading-to figure is
/// a projection, not a record, and is not drawn.
fn temperature_history(ui: &mut Ui, game: &Game, size: egui::Vec2) {
    const LINE: Color32 = Color32::from_rgb(235, 235, 240);
    const BREAK: Color32 = Color32::from_rgb(236, 88, 76);
    let h = &game.climate.history;
    let c = &game.tables.climate;
    ui.label(RichText::new("Temperature by turn; red is a Break").weak().small());
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, 3.0, Color32::from_rgb(38, 38, 44));
    if h.is_empty() {
        painter.text(rect.center(), egui::Align2::CENTER_CENTER, "No turn resolved yet.", FontId::proportional(12.0), Color32::from_gray(150));
        return;
    }
    let plot = egui::Rect::from_min_max(rect.min + egui::vec2(6.0, 6.0), rect.max - egui::vec2(44.0, 20.0));
    // The data's own range, at the designer's word, with a margin above and below and never
    // narrower than half a degree, so the first turns do not read as a cliff.
    let (min, max) = h.iter().fold((f64::MAX, f64::MIN), |(lo, hi), r| (lo.min(r.temperature), hi.max(r.temperature)));
    let pad = ((max - min) * 0.15).max(0.25);
    let (lo, hi) = (min - pad, max + pad);
    let (first, last) = (h[0].turn, h[h.len() - 1].turn);
    let span = (last.max(first + 1) - first) as f32;
    let x = |turn: u32| plot.left() + (turn - first) as f32 / span * plot.width();
    let y = |v: f64| plot.bottom() - (((v - lo) / (hi - lo)).clamp(0.0, 1.0) as f32) * plot.height();
    // The Breaks' Temperatures that fall in the range, faint, and the Collapse line if it does.
    for b in c.breaks.iter().filter(|b| b.temperature > lo && b.temperature < hi) {
        painter.line_segment([Pos2::new(plot.left(), y(b.temperature)), Pos2::new(plot.right(), y(b.temperature))], egui::Stroke::new(1.0, BREAK.gamma_multiply(0.35)));
    }
    if c.collapse_line > lo && c.collapse_line < hi {
        painter.line_segment([Pos2::new(plot.left(), y(c.collapse_line)), Pos2::new(plot.right(), y(c.collapse_line))], egui::Stroke::new(1.0, Color32::from_gray(150)));
        painter.text(Pos2::new(plot.right() + 3.0, y(c.collapse_line)), egui::Align2::LEFT_CENTER, format!("{:+.1}", c.collapse_line), FontId::proportional(10.0), Color32::from_gray(150));
    }
    painter.text(Pos2::new(plot.left() + 2.0, plot.bottom()), egui::Align2::LEFT_BOTTOM, format!("{lo:+.1}"), FontId::proportional(9.0), Color32::from_gray(120));
    painter.text(Pos2::new(plot.left() + 2.0, plot.top()), egui::Align2::LEFT_TOP, format!("{hi:+.1}"), FontId::proportional(9.0), Color32::from_gray(120));
    let points: Vec<Pos2> = h.iter().map(|r| Pos2::new(x(r.turn), y(r.temperature))).collect();
    if points.len() == 1 {
        painter.circle_filled(points[0], 3.0, LINE);
    } else {
        painter.add(egui::Shape::line(points, egui::Stroke::new(2.0, LINE)));
    }
    for r in h.iter().filter(|r| !r.breaks.is_empty()) {
        let bx = x(r.turn);
        painter.line_segment([Pos2::new(bx, plot.bottom() + 2.0), Pos2::new(bx, plot.bottom() + 6.0)], egui::Stroke::new(2.0, BREAK));
    }
    painter.text(Pos2::new(plot.left(), rect.bottom() - 2.0), egui::Align2::LEFT_BOTTOM, game.date(first).text(), FontId::proportional(10.0), Color32::from_gray(150));
    if last > first {
        painter.text(Pos2::new(plot.right(), rect.bottom() - 2.0), egui::Align2::RIGHT_BOTTOM, game.date(last).text(), FontId::proportional(10.0), Color32::from_gray(150));
    }
    let end = h[h.len() - 1].temperature;
    painter.text(Pos2::new(plot.right() + 3.0, y(end)), egui::Align2::LEFT_CENTER, format!("{end:+.1}"), FontId::proportional(11.0), LINE);
}

fn temperature_bar(ui: &mut Ui, game: &Game) {
    const BREAK: Color32 = Color32::from_rgb(236, 88, 76);
    const SEA: Color32 = Color32::from_rgb(96, 156, 236);
    const ICE: Color32 = Color32::from_rgb(206, 226, 244);
    let c = &game.tables.climate;
    let (lo, hi) = (c.base_temperature, c.collapse_line);
    let now = game.climate.temperature;
    let committed = game.target_temperature();
    // A Break is crossed when it has FIRED, not when the Temperature happens to stand past it: what
    // it did is permanent and the Temperature may come back down afterwards. `foot` is the notch
    // that runs along the bottom of the bar rather than the whole height, so a Break and a Sea Level
    // threshold at the same Temperature are both visible.
    struct Notch {
        at: f64,
        what: String,
        colour: Color32,
        crossed: bool,
        foot: bool,
    }
    let mut notches: Vec<Notch> = Vec::new();
    for (i, b) in c.breaks.iter().enumerate() {
        notches.push(Notch { at: b.temperature, what: b.name.clone(), colour: BREAK, crossed: game.climate.breaks_fired[i], foot: false });
    }
    for (i, t) in c.sea_level_thresholds.iter().enumerate() {
        let crossed = game.states.iter().any(|s| s.thresholds_fired[i]);
        notches.push(Notch { at: *t, what: "Sea Level".to_string(), colour: SEA, crossed, foot: true });
    }
    notches.push(Notch { at: c.antarctica_opens_at, what: "Antarctica opens".to_string(), colour: ICE, crossed: now >= c.antarctica_opens_at, foot: true });
    notches.sort_by(|a, b| a.at.partial_cmp(&b.at).unwrap_or(std::cmp::Ordering::Equal));

    let (rect, response) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 50.0), egui::Sense::hover());
    let bar = egui::Rect::from_min_max(egui::pos2(rect.left(), rect.top() + 8.0), egui::pos2(rect.right(), rect.top() + 34.0));
    let painter = ui.painter();
    let at = |t: f64| bar.left() + (((t - lo) / (hi - lo)).clamp(0.0, 1.0) as f32) * bar.width();
    let column = |x: f32, half: f32, top: f32, bottom: f32, colour: Color32| {
        painter.rect_filled(egui::Rect::from_min_max(egui::pos2(x - half, top), egui::pos2(x + half, bottom)), 0.0, colour);
    };
    painter.rect_filled(bar, 3.0, Color32::from_rgb(38, 38, 44));
    // The warming the Stock has already committed the world to, then the warming that has arrived.
    if committed > now {
        painter.rect_filled(
            egui::Rect::from_min_max(egui::pos2(at(now), bar.top()), egui::pos2(at(committed), bar.bottom())),
            0.0,
            Color32::from_rgb(104, 62, 40),
        );
    }
    painter.rect_filled(egui::Rect::from_min_max(bar.min, egui::pos2(at(now), bar.bottom())), 3.0, Color32::from_rgb(206, 112, 54));
    for n in &notches {
        let x = at(n.at);
        let (top, bottom) = if n.foot { (bar.top() + 15.0, bar.bottom()) } else { (bar.top(), bar.bottom()) };
        // Every notch stands on a dark backing, so a crossed one reads against the warmed fill as
        // well as against the ground ahead of it.
        if n.crossed {
            column(x, 3.0, top, bottom, Color32::from_rgb(18, 18, 22));
            column(x, 1.5, top, bottom, n.colour);
        } else {
            column(x, 2.0, top + 4.0, bottom - 4.0, Color32::from_rgb(18, 18, 22));
            column(x, 0.75, top + 5.0, bottom - 5.0, n.colour.gamma_multiply(0.7));
        }
    }
    // The Temperature now, filled, and the Temperature the Stock commits the world to, hollow.
    let pointer = |x: f32, colour: Color32, filled: bool| {
        let tip = egui::pos2(x, bar.top() - 1.0);
        let points = vec![tip, egui::pos2(x - 4.5, bar.top() - 8.0), egui::pos2(x + 4.5, bar.top() - 8.0)];
        let stroke = egui::Stroke::new(1.2, colour);
        let fill = if filled { colour } else { Color32::TRANSPARENT };
        painter.add(egui::Shape::convex_polygon(points, fill, stroke));
    };
    column(at(committed), 1.0, bar.top(), bar.bottom(), Color32::from_rgb(186, 186, 194));
    pointer(at(committed), Color32::from_rgb(186, 186, 194), false);
    column(at(now), 1.5, bar.top(), bar.bottom(), Color32::WHITE);
    pointer(at(now), Color32::WHITE, true);
    let small = FontId::proportional(11.0);
    let grey = Color32::from_gray(150);
    painter.text(egui::pos2(bar.left() + 3.0, bar.bottom() + 2.0), egui::Align2::LEFT_TOP, format!("{lo:+.1}"), small.clone(), grey);
    painter.text(egui::pos2(bar.right() - 3.0, bar.bottom() + 2.0), egui::Align2::RIGHT_TOP, format!("Collapse {hi:+.1}"), small, grey);
    // What is ahead, in words, under the bar.
    let line = match game.next_break() {
        Some(b) => format!("next: {} at {:+.1}", b.name, b.temperature),
        None => match notches.iter().find(|n| !n.crossed) {
            Some(n) => format!("next: {} at {:+.1}", n.what, n.at),
            None => "every notch on the bar is behind us".to_string(),
        },
    };
    ui.label(RichText::new(line).size(13.0).color(BREAK));
    // The whole list on hover, since no bar this wide can label nine notches.
    let list: Vec<String> = notches
        .iter()
        .map(|n| format!("{:+.1}  {}{}", n.at, n.what, if n.crossed { "  - crossed" } else { "" }))
        .collect();
    response.on_hover_text(format!(
        "The filled marker is the Temperature now, the hollow one what the CO2 Stock already commits the world to.\nBreaks (red), Sea Level thresholds and Antarctica's opening (along the foot):\n{}",
        list.join("\n")
    ));
}

/// What the drawn interface asks the session to do, applied after drawing.
enum Action {
    Place(Order),
    Cancel(usize),
    /// Ticket #323 (version 0.08.8): a sentence for the panel's notice line, where a refusal shows.
    Notice(String),
    EndTurn,
    PickTech(TechId),
    /// Ticket #58: a Report line was clicked; go where it points.
    GoTo(ReportPlace),
    ChooseFaction(FactionKind),
    NewGame(FactionKind, StateId),
    /// Ticket #169 (version 0.07.5): the Tutorial button on the title screen.
    /// Ticket #174 (version 0.07.6): the `Play Tutorial` tick at the foot of the Custodians' card
    /// was pressed. The card is drawn from a read-only Session, so the tick travels as an action.
    SetTutorialTick(bool),
    /// Ticket #169: the reader pressed on through a tutorial note.
    TutorialNoteRead,
    /// Ticket #64: Spectate, the fifth button on the choice screen.
    Spectate,
    /// Ticket #64: the Auto box beside End Turn.
    SetAuto(bool),
    /// Ticket #59: the Save button in the top bar.
    Save,
    /// Ticket #59: the Load button on the title screen.
    OpenLoad,
    LoadSave(std::path::PathBuf),
    DeleteSave(std::path::PathBuf),
    AskDelete(Option<std::path::PathBuf>),
    OpenSavesFolder,
    ToTitle,
    Quit,
}

struct Hotspot {
    pos: Pos2,
    radius: f32,
    hit: Hit,
}

#[derive(Clone, Copy)]
enum Hit {
    Select(Selection),
    Enter(BodyId),
    /// Ticket #311 (version 0.08.7): the ring that marks last turn's Battle, by its index in the
    /// Report; a hover reads the record and a click opens the Report.
    Battle(usize),
    /// Ticket #323 (version 0.08.8): the player's own shield at a Region: a click selects the
    /// Region AND arms its stack for a right-click march.
    Shield(StateId),
}

pub fn keyboard(keys: Res<ButtonInput<KeyCode>>, mut view: ResMut<ViewState>, mut session: ResMut<Session>, contexts: Option<Res<bevy_egui::input::EguiWantsInput>>) {
    if session.screen != Screen::Playing {
        return;
    }
    if contexts.map(|c| c.wants_keyboard_input()).unwrap_or(false) {
        return;
    }
    // Ticket #128 (version 0.07.2): while a popup is up, Escape is the one key that does anything --
    // it closes the popup -- with one exception: Enter on the End Turn confirmation confirms it,
    // being the same key that raised it, asked the same question. Tab and C used to fire from
    // behind the Report; an Enter that ended the turn from there would have been an accident.
    if view.popup != Popup::None {
        if keys.just_pressed(KeyCode::Enter) && view.popup == Popup::ConfirmEndTurn {
            view.hotkey = Some(HotKey::EndTurn);
        }
    } else {
        // Ticket #162 (version 0.07.5): M swaps between the Solar System Map and the surface, the
        // job Tab held until now. The designer freed M by retiring the Hab View window and asked
        // for it here: *"use it to bring up the solar system map replace tab."*
        if keys.just_pressed(KeyCode::KeyM) {
            view.swap();
        }
        // Ticket #163 (version 0.07.5): and Tab, freed by that move, brings the roster back. The
        // panel shows the roster whenever nothing is selected, so clearing the selection is what
        // "bring up the roster" means. The designer: *"should bring up the roster."*
        if keys.just_pressed(KeyCode::Tab) {
            view.selection = Selection::None;
            view.slot_box = None;
            view.hab_tile = None;
        }
        // Ticket #41: C toggles the Climate Panel, a second way back once it is closed.
        if keys.just_pressed(KeyCode::KeyC) {
            toggle_climate(&mut view);
        }
        // Ticket #128: a key for every button on the bar. Each toggles its window as the button
        // does; Save takes Ctrl, since a bare key that writes a file is a hazard; Enter is End Turn.
        if keys.just_pressed(KeyCode::KeyT) {
            view.show_tech = !view.show_tech;
        }
        if keys.just_pressed(KeyCode::KeyV) {
            view.show_victory = !view.show_victory;
        }
        // Ticket #203 (version 0.08.1): F opens the Faction window, as its button does.
        if keys.just_pressed(KeyCode::KeyF) {
            view.show_factions = !view.show_factions;
        }
        if keys.just_pressed(KeyCode::KeyR) && !session.spectator {
            view.show_trade = !view.show_trade;
        }
        let ctrl = keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight);
        if ctrl && keys.just_pressed(KeyCode::KeyS) {
            view.hotkey = Some(HotKey::Save);
        }
        if keys.just_pressed(KeyCode::Enter) {
            view.hotkey = Some(HotKey::EndTurn);
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        // Ticket #64: Escape unticks Auto, so a spectator can always stop the clock.
        if session.auto {
            session.auto = false;
            session.auto_elapsed = 0.0;
        }
        if view.popup != Popup::None {
            let moments = session.game.as_ref().map(|g| view.moments_of(&session.tables, &g.report).len()).unwrap_or(0);
            // Ticket #205 (version 0.08.1): Escape is the path that used to lose a tutorial turn's
            // Event, since it never carried the flag the note's own button checked for.
            let has_event = session.game.as_ref().and_then(|g| g.last_event.as_ref()).is_some();
            advance_popup(&mut view, moments, has_event);
        } else if view.armed_stack.is_some() {
            // Ticket #323 (version 0.08.8): Esc disarms the stack before anything else.
            view.armed_stack = None;
        } else if view.hab_tile.is_some() {
            // Ticket #162 (version 0.07.5): Esc clears a clicked Module tile before it leaves a
            // Surface Map, as it closed the Hab View before the window retired.
            view.hab_tile = None;
        } else if matches!(view.view, View::Surface(_)) {
            view.view = View::Solar;
            view.selection = Selection::None;
        }
    }
}

/// Ticket #41: show or hide the Climate Panel; a reopened panel returns to its home position, so a
/// panel dragged off the picture and closed is not lost.
pub fn toggle_climate(view: &mut ViewState) {
    view.show_climate = !view.show_climate;
    view.climate_reopen = view.show_climate;
}

/// Ticket #169 (version 0.07.5): the tutorial's note for a turn, if it has one. The notes are
/// words and nothing else, so they live in `assets/data/tutorial.toml` with every other sentence
/// the game says, and the turn they open is a field on the note rather than their order in the file.
fn tutorial_note(tables: &Tables, turn: u32) -> Option<&dying_earth_engine::TutorialNote> {
    tables.tutorial.note.iter().find(|n| n.turn == turn)
}

/// Ticket #58: Event, then the turn's Moments one after another, then the Report.
///
/// Ticket #205 (version 0.08.1): `has_event` closes a hole. A tutorial note dismissed by its BUTTON
/// went on to the Event when the turn had drawn one, because `Action::TutorialNoteRead` checked for
/// itself; a note dismissed with ESCAPE came through here, where there was no Event arm at all, and
/// the turn's Event was never shown. So a new player -- the only player a tutorial has -- could be
/// hit by an Event and never told, and the one who pressed Escape was the one it happened to. Both
/// paths now pass through the same arm, and `Action::TutorialNoteRead` no longer checks separately.
fn advance_popup(view: &mut ViewState, moments: usize, has_event: bool) {
    view.popup = match view.popup {
        // Ticket #169 (version 0.07.5): the tutorial's note comes first and hands on to whatever
        // the turn would have opened with.
        Popup::Tutorial if has_event => Popup::Event,
        Popup::Tutorial if moments > 0 => Popup::Moment(0),
        Popup::Tutorial => Popup::Report,
        Popup::Event if moments > 0 => Popup::Moment(0),
        Popup::Event => Popup::Report,
        Popup::Moment(i) if i + 1 < moments => Popup::Moment(i + 1),
        Popup::Moment(_) => Popup::Report,
        _ => Popup::None,
    };
}

/// Recompose the Earth Map whenever the board changed.
pub fn recompose_earth(
    mut session: ResMut<Session>,
    mut view: ResMut<ViewState>,
    textures: Res<Textures>,
    handles: Option<Res<SceneHandles>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ticket #126 (version 0.07.2): on the start screen the globe is also recomposed when the lit
    // Region -- chosen, else under the pointer -- changes, and only then; a recompose is two million
    // pixels and a hover is not a reason to do it every frame.
    let lit = if session.game.is_none() { view.start_selected.or(view.start_hover) } else { None };
    if !session.earth_dirty && !(session.game.is_none() && lit != view.start_lit_drawn) {
        return;
    }
    let Some(handles) = handles else { return };
    let rgba = match &session.game {
        Some(game) => textures.compose_earth(game, &session.colours()),
        // Ticket #126 (version 0.07.2): the start globe shows every Region in its own colour, with
        // its borders, so a player can choose one by clicking it.
        None => textures.compose_regions(&session.tables, lit),
    };
    view.start_lit_drawn = lit;
    let img = images.add(rgba.to_image());
    if let Some(mut m) = materials.get_mut(&handles.earth_material) {
        m.base_color_texture = Some(img);
    }
    session.earth_dirty = false;
}

/// A Faction colour from the data tables as an egui colour.
fn rgb(c: [f32; 3]) -> Color32 {
    Color32::from_rgb((c[0] * 255.0) as u8, (c[1] * 255.0) as u8, (c[2] * 255.0) as u8)
}

/// Ticket #263 (version 0.08.4): "a Mine", "an Observatory", "the Industry Level" -- the article a
/// build's name wants in a sentence.
fn with_article(what: &str) -> String {
    let lower = what.to_lowercase();
    if lower == "industry level" {
        return format!("the {what}");
    }
    let vowel = lower.starts_with(['a', 'e', 'i', 'o', 'u']);
    format!("{} {what}", if vowel { "an" } else { "a" })
}

fn seat_colour(session: &Session, seat: Seat) -> Color32 {
    rgb(session.colours()[seat.index()])
}

/// Ticket #50: every other seat with Ships at a Body, in seat order, with its stack strength.
fn rivals_at(game: &Game, seat: Seat, body: BodyId) -> Vec<(Seat, i64)> {
    seat.others().into_iter().filter(|s| !game.ships_at(*s, body).is_empty()).map(|s| (s, game.ship_stack_strength(s, body))).collect()
}

/// "Prospectors 6 and Archivists 3", the Factions a Battle here would be against.
fn rivals_text(game: &Game, rivals: &[(Seat, i64)]) -> String {
    let names: Vec<String> = rivals.iter().map(|(s, str_)| format!("{} {}", game.seat_name(*s), str_)).collect();
    match names.len() {
        0 => "nobody".to_string(),
        1 => names[0].clone(),
        _ => format!("{} and {}", names[..names.len() - 1].join(", "), names[names.len() - 1]),
    }
}

/// Ticket #127 (version 0.07.2): the kind of thing a name belongs to, which decides the glyph drawn
/// in front of it. The designer asked for five -- "one for military one for colony ship, than one
/// for stations and one for colonies and one for nation states ... keep these off white" -- and,
/// asked about Armies, took the shield the Earth Map already draws for one. Every glyph is
/// off-white: on this board a colour says WHOSE, and a kind glyph says WHAT.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Warship,
    ColonyShip,
    Station,
    Colony,
    Region,
    Army,
    /// Ticket #317 (version 0.08.8): a Battle fought last turn, crossed blades; the mark on the
    /// map and the glyph on the Battles list.
    Battle,
}

/// The side of a kind glyph in a row of text at the panel's ordinary size.
const KIND_GLYPH: f32 = 16.0;

impl Kind {
    /// The icon's name in `assets/icons/`. The Army has none: its shield is drawn.
    fn icon(self) -> Option<&'static str> {
        match self {
            Kind::Warship => Some("warship"),
            Kind::ColonyShip => Some("colony_ship"),
            Kind::Station => Some("station"),
            Kind::Colony => Some("colony"),
            Kind::Region => Some("region"),
            Kind::Army => None,
            Kind::Battle => Some("battle"),
        }
    }

    fn of_colony(c: &Colony) -> Kind {
        if c.in_orbit { Kind::Station } else { Kind::Colony }
    }

    /// A stack with one warship in it is a warship stack; a stack of transports is not. A Carrier
    /// is a transport, so it wears the Colony Ship's glyph, the designer having named two kinds
    /// of Ship and not three.
    fn of_ships<'a>(ships: impl IntoIterator<Item = &'a Ship>) -> Kind {
        if ships.into_iter().any(|s| s.kind.is_warship()) { Kind::Warship } else { Kind::ColonyShip }
    }

    fn of_unit(kind: UnitKind) -> Kind {
        if kind.is_warship() { Kind::Warship } else { Kind::ColonyShip }
    }

    fn image(self, ctx: &egui::Context, size: f32) -> Option<egui::Image<'static>> {
        Icons::from_ctx(ctx, self.icon()?, size)
    }
}

/// The kind glyph in a `Ui` row, at `size`: the icon where one is loaded, the Army's drawn shield,
/// and nothing at all where the art is missing, so a name is never pushed about by a hole.
/// Ticket #210 (version 0.08.1): **whose it is, as a mark rather than a sentence.** The designer
/// asked for the Faction's symbol on a station's card *"similar to the flags for nations"* -- a
/// Region card wears its Nation's flag beside its name (ticket #122) and says who controls it only
/// in words below. A Colony card said `Held by the Archivists` and wore nothing, so this puts the
/// holder's symbol where the flag sits, in the Faction's own colour, and on Ships for the same
/// reason. Neutral places wear nothing, exactly as a Region with no flag stands alone.
///
/// This is the third caller to pick an icon's colour rather than read it from `icons::fill`, after
/// the Faction card and the Faction window, and for the same reason: a Faction symbol's colour means
/// WHOSE, which is the one thing that rule exists to express.
fn faction_glyph(ui: &mut Ui, session: &Session, game: &Game, seat: Option<Seat>, size: f32) {
    let Some(seat) = seat else { return };
    let key = crate::icons::faction_symbol(game.kind(seat));
    if let Some(image) = Icons::from_ctx(ui.ctx(), key, size) {
        ui.add(image.tint(seat_colour(session, seat))).on_hover_text(format!("The {}", game.seat_name(seat)));
    }
}

fn kind_glyph(ui: &mut Ui, kind: Kind, size: f32) {
    if kind == Kind::Army {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());
        shield_glyph(ui.painter(), rect);
    } else if let Some(image) = kind.image(ui.ctx(), size) {
        ui.add(image);
    }
}

/// Ticket #138 (version 0.07.3): a button with a kind glyph inside it for a kind whose glyph is
/// drawn rather than loaded (the Army's shield), so it looks and behaves like `Button::image_and_text`
/// does for the kinds that have art. A clickable group in the button's own visuals, as `priced_button`.
/// Ticket #218 (version 0.08.2): the founding button, carrying the site's four yields ON ITS FACE in
/// glyph and number rather than on a hover. The player already reads that same row under every slot
/// label on the Body Surface Map (`slot_yield_label`), so the moment of the decision uses the
/// notation they have been reading all along instead of a second one. Ticket #211 put these figures
/// on a hover one version ago; this supersedes that at the designer's word, and the hover is dropped
/// entirely -- it held the same four figures in words, so it duplicated the face and nothing else.
///
/// Built as `glyph_button` is, a clickable frame of its own, because a plain egui button's face is
/// text and cannot carry the glyphs.
fn found_button(ui: &mut Ui, yields: &dying_earth_engine::SlotYields, label: &str) -> egui::Response {
    ui.scope_builder(egui::UiBuilder::new().sense(egui::Sense::click()), |ui| {
        let resp = ui.response();
        let visuals = *ui.style().interact(&resp);
        egui::Frame::new()
            .inner_margin(egui::Margin::symmetric(6, 4))
            .corner_radius(visuals.corner_radius)
            .fill(visuals.weak_bg_fill)
            .stroke(visuals.bg_stroke)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new(label).color(visuals.text_color()));
                    // Ticket #258 (version 0.08.4): drawn glyph-first; see `slot_yield_row`.
                    slot_yield_row(ui, slot_yield_figures(yields), 13.0, visuals.text_color());
                });
            });
    })
    .response
}

fn glyph_button(ui: &mut Ui, kind: Kind, text: &str) -> egui::Response {
    ui.scope_builder(egui::UiBuilder::new().sense(egui::Sense::click()), |ui| {
        let resp = ui.response();
        let visuals = *ui.style().interact(&resp);
        egui::Frame::new()
            .inner_margin(egui::Margin::symmetric(4, 1))
            .corner_radius(visuals.corner_radius)
            .fill(visuals.weak_bg_fill)
            .stroke(visuals.bg_stroke)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    kind_glyph(ui, kind, KIND_GLYPH);
                    ui.label(RichText::new(text).color(visuals.text_color()));
                });
            });
    })
    .response
}

/// The Army's shield as a kind glyph: the Earth Map's shape, in the kind fill, with no number on it.
fn shield_glyph(painter: &egui::Painter, rect: egui::Rect) {
    let (w, h) = (rect.width() * 0.8, rect.height() * 0.95);
    let centre = rect.center();
    let pts = vec![
        centre + egui::vec2(-w / 2.0, -h / 2.0),
        centre + egui::vec2(w / 2.0, -h / 2.0),
        centre + egui::vec2(w / 2.0, 0.0),
        centre + egui::vec2(0.0, h / 2.0),
        centre + egui::vec2(-w / 2.0, 0.0),
    ];
    painter.add(egui::Shape::convex_polygon(pts, crate::icons::kind_fill(), egui::Stroke::NONE));
}

/// A shield with a number on it: the Army icon of the Earth Map.
/// Ticket #311 (version 0.08.7): `hurt` puts a red pip at the shield's top-right corner when the
/// stack carries damage, at the designer's word. The outline in the aggressor's colour that came
/// with it was dropped by ticket #317 (version 0.08.8), at the designer's word; the Battle mark
/// beside the label says a Battle was fought here.
fn shield(painter: &egui::Painter, centre: Pos2, fill: Color32, text: &str, hurt: bool) {
    let (w, h) = (20.0, 24.0);
    let pts = vec![
        centre + egui::vec2(-w / 2.0, -h / 2.0),
        centre + egui::vec2(w / 2.0, -h / 2.0),
        centre + egui::vec2(w / 2.0, 0.0),
        centre + egui::vec2(0.0, h / 2.0),
        centre + egui::vec2(-w / 2.0, 0.0),
    ];
    painter.add(egui::Shape::convex_polygon(pts, fill, egui::Stroke::new(1.5, Color32::BLACK)));
    painter.text(centre + egui::vec2(0.0, -2.0), egui::Align2::CENTER_CENTER, text, FontId::proportional(12.0), Color32::BLACK);
    if hurt {
        let pip = centre + egui::vec2(w / 2.0 - 1.0, -h / 2.0 + 1.0);
        painter.circle(pip, 4.0, Color32::from_rgb(230, 50, 40), egui::Stroke::new(1.0, Color32::BLACK));
    }
}

/// Ticket #112 (version 0.07.1): a map label is lightened before it is drawn. A Faction's colour
/// does two jobs -- it tints that Faction's territory on the globe AND prints the label that sits on
/// top of it -- so a dark Faction colour costs its own labels their legibility. The Archivists moved
/// from a pale blue-white of lightness L* 83 to a crimson of 55 and their labels all but vanished
/// into their own land; the Arkwrights went from 48 to 39 in the same direction. Rather than forbid
/// dark Faction colours, every map label is raised toward white by a fixed fraction, which also
/// helps the Prospectors' orange-on-orange, a case that was already poor before this version.
const MAP_LABEL_LIFT: f32 = 0.4;

fn on_map(colour: Color32) -> Color32 {
    let lift = |c: u8| (c as f32 + (255.0 - c as f32) * MAP_LABEL_LIFT) as u8;
    Color32::from_rgb(lift(colour.r()), lift(colour.g()), lift(colour.b()))
}

fn label_at(painter: &egui::Painter, pos: Pos2, text: &str, colour: Color32, size: f32) {
    let colour = on_map(colour);
    let galley = painter.layout_no_wrap(text.to_string(), FontId::proportional(size), colour);
    let rect = egui::Rect::from_center_size(pos, galley.size() + egui::vec2(8.0, 4.0));
    painter.rect_filled(rect, 3.0, Color32::from_black_alpha(170));
    painter.galley(rect.min + egui::vec2(4.0, 2.0), galley, colour);
}

/// Ticket #127 (version 0.07.2): a map label wearing its kind glyph at the left, in the kind fill and
/// sized to a line of the text. Where there is no art for it the bare label is drawn, so the map
/// never goes mute; the whole of glyph and text is centred on `pos`, as the bare label is.
fn label_kind_at(painter: &egui::Painter, pos: Pos2, kind: Option<Kind>, text: &str, colour: Color32, size: f32) {
    let texture = kind.and_then(Kind::icon).and_then(|name| Icons::texture_from_ctx(painter.ctx(), name));
    let Some(texture) = texture else {
        label_at(painter, pos, text, colour, size);
        return;
    };
    let colour = on_map(colour);
    let galley = painter.layout_no_wrap(text.to_string(), FontId::proportional(size), colour);
    let glyph = size + 2.0;
    let gap = 4.0;
    let rect = egui::Rect::from_center_size(pos, galley.size() + egui::vec2(8.0 + glyph + gap, 4.0));
    painter.rect_filled(rect, 3.0, Color32::from_black_alpha(170));
    let glyph_rect = egui::Rect::from_min_size(rect.min + egui::vec2(4.0, 2.0), egui::vec2(glyph, glyph));
    painter.image(texture, glyph_rect, egui::Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), crate::icons::kind_fill());
    painter.galley(rect.min + egui::vec2(4.0 + glyph + gap, 2.0), galley, colour);
}

/// Ticket #136 (version 0.07.3): an orbit as a projected polyline. `points` are the ring's samples
/// in order, `None` where a sample is hidden behind the globe or off screen; a **dashed** ring is
/// an empty orbit, drawn three samples on and three off, so room to build is visible without a card.
fn orbit_polyline(painter: &egui::Painter, points: &[Option<Pos2>], dashed: bool, colour: Color32) {
    let n = points.len();
    for i in 0..n {
        let (Some(a), Some(b)) = (points[i], points[(i + 1) % n]) else { continue };
        if dashed && (i / 3) % 2 == 1 {
            continue;
        }
        painter.line_segment([a, b], egui::Stroke::new(1.2, colour));
    }
}

/// Ticket #136: a kind glyph painted at a point in a colour, on a dark disc so it reads over a
/// photograph. On the orbit rings the colour is the holder's, since on a map a colour says whose.
fn glyph_at(painter: &egui::Painter, kind: Kind, centre: Pos2, size: f32, tint: Color32) {
    if let Some(texture) = kind.icon().and_then(|name| Icons::texture_from_ctx(painter.ctx(), name)) {
        painter.circle_filled(centre, size * 0.72, Color32::from_black_alpha(170));
        let rect = egui::Rect::from_center_size(centre, egui::vec2(size, size));
        painter.image(texture, rect, egui::Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), tint);
    }
}

/// Ticket #317 (version 0.08.8): **the Battle mark**, at the designer's word: the crossed-blades
/// glyph, off-white as every kind glyph is, on a disc in the aggressor's colour (grey when the
/// record names none), beside the label of the place where a Battle was fought last turn -- a
/// Region, a Colony, or a Body in orbit -- for the one Orders phase the record lives. It replaces
/// 0.08.7's ring, which marked nothing in orbit, where most Battles are. The caller pushes its
/// hotspot: the hover reads the record, the click opens the Report.
fn battle_mark(painter: &egui::Painter, centre: Pos2, colour: Color32) {
    let size = 18.0;
    painter.circle(centre, size * 0.78, colour, egui::Stroke::new(1.0, Color32::BLACK));
    if let Some(texture) = Kind::Battle.icon().and_then(|name| Icons::texture_from_ctx(painter.ctx(), name)) {
        let rect = egui::Rect::from_center_size(centre, egui::vec2(size, size));
        painter.image(texture, rect, egui::Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), crate::icons::kind_fill());
    }
}

/// The colour a Battle's mark wears: its aggressor's, or nobody's grey.
fn battle_colour(session: &Session, game: &Game, i: usize) -> Color32 {
    game.report.battles.get(i).and_then(|b| b.aggressor()).map(|s| seat_colour(session, s)).unwrap_or(Color32::from_gray(150))
}

/// The warship sitting in an Orbital Slot, if one is: a Frigate or Battleship at the Body that chose
/// that slot when its leg was ordered (Blockade, version 0.07.0).
fn warship_in_slot(game: &Game, body: BodyId, slot: u32) -> Option<&Ship> {
    game.ships.iter().find(|s| s.at == ShipAt::Body(body) && s.slot == Some(slot) && matches!(s.kind, UnitKind::Frigate | UnitKind::Battleship))
}

/// Ticket #136 (version 0.07.3): **one ring per Orbital Slot round the globe** on a Body Surface
/// Map. The designer: *"Want to see icons representative of orbitals orbiting their parent bodies
/// each slot a separate orbit."* Ticket #151 (version 0.07.4) redrew them: *"each should be on
/// slightly different orbital plane and should rotate with the globe allow them to move slowly so
/// they can be clicked."* Each ring is a circle in the globe's own frame -- so it turns with a drag
/// -- on a plane of its own, leaning thirty to sixty degrees from the equator with its own heading,
/// so the five cross one another rather than stack, and at most one is near edge-on at any turn
/// of the globe; the part behind the globe is not drawn. A built station's glyph **travels slowly
/// round its ring** (one revolution in about a minute and a half, each slot's period its own),
/// in its holder's colour with its name beneath, clickable as it goes; in `shot:` mode the clock
/// is stopped so the pictures are reproducible. An empty slot is a solid grey ring; a warship
/// blockading the slot is drawn beside the station in its Faction's colour, which is the first
/// time Blockade has been visible on a map.
#[allow(clippy::too_many_arguments)]
fn orbit_rings_on_globe(
    painter: &egui::Painter,
    session: &Session,
    game: &Game,
    body: BodyId,
    globe_gt: &GlobalTransform,
    cam_pos: Vec3,
    cam_right: Vec3,
    project: &dyn Fn(Vec3) -> Option<Pos2>,
    hotspots: &mut Vec<Hotspot>,
) {
    let n = game.tables.body(body).orbital_slots;
    if n == 0 {
        return;
    }
    let center = globe_gt.translation();
    let Some(c2) = project(center) else { return };
    let rim = project(center + cam_right * GLOBE_RADIUS).map(|r| (r - c2).length()).unwrap_or(0.0);
    let to_cam = cam_pos - center;
    let hidden = |world: Vec3, screen: Pos2| (world - center).dot(to_cam) < 0.0 && (screen - c2).length() < rim;
    // The rings live in the globe's upright frame (its pole up, before the mesh's own correction),
    // so a drag turns them with it. Version 0.07.3 had pinned them to the camera because rings in
    // the equatorial plane were edge-on from where this camera sits, five near-vertical lines; a
    // ring leaning well off the equator, on a heading of its own, is an ellipse from almost every
    // side and edge-on only for a moment as the globe turns past its line of nodes.
    let rot = globe_gt.rotation() * geo::upright().inverse();
    // The clock the stations travel by. Stopped in `shot:` mode, so a picture is the same twice.
    let clock = if session.shot_prefix.is_empty() { painter.ctx().input(|i| i.time) as f32 } else { 0.0 };
    for slot in 0..n {
        // A step tighter to the globe than the first try, at the designer's word ("just slightly
        // tighter"), so the outer rings stay nearer the window at the default zoom.
        let radius = GLOBE_RADIUS * (1.08 + 0.04 * slot as f32);
        // Each slot's own plane: a lean from the equator of 32 to 61 degrees, and a heading for
        // the line of nodes a good step round from the last slot's.
        let incline = 0.55 + 0.13 * slot as f32;
        let node = 1.3 * slot as f32;
        let u = Vec3::new(node.cos(), 0.0, node.sin());
        let v = Vec3::new(-node.sin() * incline.cos(), incline.sin(), node.cos() * incline.cos());
        let world_at = |a: f32| center + rot * (radius * (a.cos() * u + a.sin() * v));
        let samples = 128;
        let points: Vec<Option<Pos2>> = (0..samples)
            .map(|i| {
                let a = i as f32 / samples as f32 * std::f32::consts::TAU;
                let w = world_at(a);
                project(w).filter(|p| !hidden(w, *p))
            })
            .collect();
        let station = game.colonies.iter().find(|c| c.in_orbit && c.body == body && c.slot == slot);
        let colour = station.and_then(|c| c.control.director()).map(|s| seat_colour(session, s)).unwrap_or(Color32::from_gray(150));
        // Ticket #151: an empty slot's ring is a solid line in the empty grey, no longer dashed --
        // the designer: *"lets make them solid lines, same color."*
        orbit_polyline(painter, &points, false, colour.gamma_multiply(0.8));
        // The glyph starts a step further round its ring than the last slot's, so five glyphs fan
        // out rather than line up, and travels on from there: one revolution in ninety seconds
        // for the first slot and eight seconds longer for each after it, so they drift apart.
        let period = 90.0 + 8.0 * slot as f32;
        let a = std::f32::consts::PI * (1.18 + 0.14 * slot as f32) + clock * std::f32::consts::TAU / period;
        let w = world_at(a);
        let Some(p) = project(w).filter(|p| !hidden(w, *p)) else { continue };
        if let Some(c) = station {
            glyph_at(painter, Kind::Station, p, 18.0, colour);
            label_at(painter, p + egui::vec2(0.0, 17.0), &game.station_name(body, slot), colour, 11.0);
            // A moving target: the circle is wider than a fixed glyph's, and the click is taken
            // where the mouse was pressed, so a station cannot slip out from under its own click.
            hotspots.push(Hotspot { pos: p, radius: 16.0, hit: Hit::Select(Selection::Colony(c.id)) });
        }
        if let Some(s) = warship_in_slot(game, body, slot) {
            glyph_at(painter, Kind::Warship, p + egui::vec2(20.0, 0.0), 16.0, seat_colour(session, s.seat));
        }
    }
}

/// The same, slid sideways so a long line stays on screen (ticket #57: the launch-window tooltip is
/// wider than a Body's other labels, and Mars can stand at the edge of its ring).
fn label_on_screen(painter: &egui::Painter, pos: Pos2, text: &str, colour: Color32, size: f32) {
    // The galley here is only measured, never drawn; label_at below does the lifting and the
    // drawing, so lifting a second time here would double it.
    let galley = painter.layout_no_wrap(text.to_string(), FontId::proportional(size), colour);
    let half = galley.size().x / 2.0 + 6.0;
    let clip = painter.clip_rect();
    let x = pos.x.clamp(clip.left() + half, (clip.right() - half).max(clip.left() + half));
    label_at(painter, Pos2::new(x, pos.y), text, colour, size);
}

#[allow(clippy::too_many_arguments)]
pub fn draw(
    mut contexts: EguiContexts,
    mut session: ResMut<Session>,
    mut view: ResMut<ViewState>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    globes: Query<(&Globe, &GlobalTransform)>,
    window: Query<&Window, With<PrimaryWindow>>,
    textures: Res<Textures>,
    mut icons: ResMut<Icons>,
    time: Res<Time>,
    mut exit: MessageWriter<AppExit>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let mut root = Ui::new(ctx.clone(), "viewport".into(), egui::UiBuilder::new().layer_id(egui::LayerId::background()).max_rect(ctx.viewport_rect()));
    let mut actions: Vec<Action> = Vec::new();
    // Ticket #109 (version 0.07.0): the resource icons are rendered from SVG once, on the first
    // frame that has an egui context to hand them to.
    icons.load(ctx, &crate::assets_root().join("icons"));
    icons.load_flags(ctx, &crate::assets_root().join("flags"));
    let icons = &*icons;
    // Ticket #100 (version 0.07.0): the start globe turns on its own only until a hand is put on it.
    // Ticket #152 (version 0.07.4): once every 75 seconds, a third of the speed it opened at; the
    // designer: *"slow the rotation of the earth in the territory select screen."*
    if !view.start_grabbed {
        view.spin += time.delta_secs() * std::f32::consts::TAU / START_GLOBE_PERIOD_SECS;
    }
    let _ = window;
    // Ticket #59: "Saved." stands in the top bar for a few seconds and then goes.
    if let Some((_, left)) = &mut session.save_notice {
        *left -= time.delta_secs();
        if *left <= 0.0 {
            session.save_notice = None;
        }
    }
    // Ticket #64: Auto runs a turn every three seconds. The clock stops while a Moment, the Report
    // or the game-over popup is up, and picks up again where it left off when the popup closes.
    if session.screen == Screen::Playing {
        let blocked = view.popup != Popup::None;
        if blocked || !session.auto {
            session.auto_elapsed = 0.0;
        } else {
            session.auto_elapsed += time.delta_secs();
        }
        if crate::app::auto_should_advance(session.auto, blocked, session.auto_elapsed) {
            session.auto_elapsed = 0.0;
            actions.push(Action::EndTurn);
        }
    }
    // Ticket #128 (version 0.07.2): a shortcut the keyboard recorded is acted on here, with the
    // game and the action list in hand, and by the same function its button calls.
    if let Some(key) = view.hotkey.take()
        && session.screen == Screen::Playing
        && let Some(game) = session.game.as_ref()
    {
        match key {
            HotKey::Save => {
                if dying_earth_engine::save::can_save_now(session.pending.len()) {
                    actions.push(Action::Save);
                }
            }
            HotKey::EndTurn => press_end_turn(&session, game, &mut view, &mut actions),
        }
    }
    match session.screen.clone() {
        Screen::Title => title_screen(&mut root, &mut session, &mut actions),
        Screen::Load => load_screen(&mut root, &session, &mut actions),
        Screen::Credits => credits_screen(&mut root, &mut session, icons),
        Screen::ChooseFaction => faction_screen(&mut root, &session, &mut actions),
        Screen::ChooseStart { faction } => {
            let cam = camera.single().ok();
            start_screen(&mut root, &session, faction, &mut view, cam, &globes, &textures, &mut actions)
        }
        Screen::Playing | Screen::GameOver => {
            let cam = camera.single().ok();
            game_screen(&mut root, ctx, &session, &mut view, cam, &globes, &textures, icons, &mut actions);
        }
    }
    for a in actions {
        match a {
            Action::Place(o) => {
                session.place(o);
            }
            Action::Notice(text) => session.last_error = Some(text),
            Action::Cancel(i) => {
                if i < session.pending.len() {
                    // Ticket #134 (version 0.07.3): cancelling the Influence order the standing Max
                    // placed ends the standing order too -- the designer's addition -- so a
                    // cancelled Max is not placed again next turn behind the player's back.
                    let standing = session.game.as_ref().and_then(|g| g.seat(Seat(0)).max_standing);
                    if let (Some(place), Order::Influence { target, .. }) = (standing, &session.pending[i])
                        && *target == place
                        && !session.pending.iter().any(|o| matches!(o, Order::SetMaxStanding { .. }))
                    {
                        session.pending.push(Order::SetMaxStanding { target: None });
                    }
                    session.pending.remove(i);
                    // Later orders may have leaned on the cancelled one; keep only what still checks.
                    let list = std::mem::take(&mut session.pending);
                    for o in list {
                        session.place(o);
                    }
                }
            }
            Action::EndTurn => {
                session.end_turn();
                // Ticket #105: a refusal raises its own popup, every time, so the button never just
                // does nothing.
                if session.refusal.is_some() {
                    view.popup = Popup::Refused;
                }
                view.selection = Selection::None;
                let moments = session.game.as_ref().map(|g| view.moments_of(&session.tables, &g.report).len()).unwrap_or(0);
                // Ticket #169 (version 0.07.5): a tutorial game opens its first turns with a note
                // saying what the turn is for, before the Event, the Moments and the Report.
                let has_note = session.tutorial && session.game.as_ref().map(|g| tutorial_note(&session.tables, g.turn).is_some()).unwrap_or(false);
                view.popup = if has_note {
                    Popup::Tutorial
                } else if session.game.as_ref().and_then(|g| g.last_event.as_ref()).is_some() {
                    Popup::Event
                } else if moments > 0 {
                    Popup::Moment(0)
                } else {
                    Popup::Report
                };
                view.attack_preview = false;
            }
            Action::PickTech(t) => {
                let result = session.game.as_mut().map(|g| g.pick_tech(Seat(0), t));
                if let Some(Err(e)) = result {
                    session.last_error = Some(e);
                }
            }
            Action::GoTo(place) => {
                view.popup = Popup::None;
                match place {
                    ReportPlace::State(s) => {
                        if view.view != View::Surface(BodyId::Earth) {
                            view.enter_surface(BodyId::Earth);
                        }
                        view.selection = Selection::State(s);
                    }
                    ReportPlace::Colony(c) => {
                        let body = session.game.as_ref().and_then(|g| g.colony(c)).map(|col| col.body);
                        if let Some(b) = body {
                            if view.view != View::Surface(b) {
                                view.enter_surface(b);
                            }
                            view.selection = Selection::Colony(c);
                        }
                    }
                    ReportPlace::Body(BodyId::Earth) => {
                        view.enter_surface(BodyId::Earth);
                    }
                    ReportPlace::Body(_) => {
                        view.view = View::Solar;
                        view.selection = Selection::None;
                    }
                }
                view.attack_preview = false;
            }
            Action::Spectate => {
                session.spectate();
                *view = ViewState::default();
                view.popup = Popup::Report;
                let start = session.game.as_ref().map(|g| g.controlled_states(Seat(0))[0]).unwrap_or(StateId::EastAsia);
                let (lon, lat) = geo::state_lonlat(start);
                view.yaw = geo::yaw_facing(lon, lat);
            }
            Action::SetAuto(on) => {
                session.auto = on;
                session.auto_elapsed = 0.0;
            }
            Action::ChooseFaction(f) => {
                session.screen = Screen::ChooseStart { faction: f };
                session.earth_dirty = true;
            }
            Action::NewGame(f, s) => {
                // Ticket #174 (version 0.07.6): the tutorial is asked for on the Custodians' card
                // and read here, once the start has been chosen -- a ticked card still picks its own
                // Region, at the designer's word. A tutorial game opens on its first note rather
                // than on the Report, because the note says what the turn is for.
                let tutorial = crate::app::tutorial_wanted(session.tutorial_ticked, f);
                session.new_game(f, s);
                session.tutorial = tutorial;
                session.tutorial_ticked = false;
                *view = ViewState::default();
                view.popup = if tutorial { Popup::Tutorial } else { Popup::Report };
                let (lon, lat) = geo::state_lonlat(s);
                view.yaw = geo::yaw_facing(lon, lat);
            }
            Action::SetTutorialTick(on) => session.tutorial_ticked = on,
            // Ticket #169: on from a note to whatever the turn would have opened with. The tutorial
            // ends itself once the last note has been read, and the game carries on as any other.
            Action::TutorialNoteRead => {
                let turn = session.game.as_ref().map(|g| g.turn).unwrap_or(0);
                if session.tables.tutorial.note.iter().map(|n| n.turn).max() == Some(turn) {
                    session.tutorial = false;
                }
                let moments = session.game.as_ref().map(|g| view.moments_of(&session.tables, &g.report).len()).unwrap_or(0);
                let has_event = session.game.as_ref().and_then(|g| g.last_event.as_ref()).is_some();
                advance_popup(&mut view, moments, has_event);
            }
            Action::Save => {
                session.save_now();
            }
            Action::OpenLoad => {
                session.refresh_saves();
                session.screen = Screen::Load;
            }
            Action::LoadSave(path) => match session.load_save(&path) {
                Ok(()) => {
                    *view = ViewState::default();
                    // Ticket #59: a loaded game comes back where it was: the Earth Map, the Report
                    // popup open, and the globe facing the Faction's own state.
                    view.popup = Popup::Report;
                    let start = session
                        .game
                        .as_ref()
                        .and_then(|g| g.controlled_states(Seat(0)).first().copied())
                        .unwrap_or(StateId::EastAsia);
                    let (lon, lat) = geo::state_lonlat(start);
                    view.yaw = geo::yaw_facing(lon, lat);
                }
                Err(e) => {
                    session.last_error = Some(e);
                }
            },
            Action::AskDelete(path) => {
                session.confirm_delete = path;
            }
            Action::DeleteSave(path) => {
                session.delete_save(&path);
            }
            Action::OpenSavesFolder => {
                let result = match &session.saves {
                    Ok(dir) => crate::saves::open_folder(dir),
                    Err(e) => Err(e.clone()),
                };
                if let Err(e) = result {
                    session.last_error = Some(e);
                }
            }
            Action::ToTitle => {
                session.game = None;
                session.pending.clear();
                session.screen = Screen::Title;
                session.earth_dirty = true;
                *view = ViewState::default();
            }
            Action::Quit => {
                exit.write(AppExit::Success);
            }
        }
    }
    Ok(())
}

// ------------------------------------------------------------------ screens before the game

fn title_screen(root: &mut Ui, session: &mut Session, actions: &mut Vec<Action>) {
    egui::CentralPanel::default().show(root, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(160.0);
            ui.label(RichText::new("DYING EARTH").size(48.0).strong());
            ui.label(RichText::new("Colonize the solar system before ecological collapse overtakes Earth.").size(16.0));
            ui.add_space(40.0);
            // Ticket #174 (version 0.07.6): the Tutorial button stood here until the designer moved
            // the choice onto the Custodians' card on the Faction screen, where the Faction it
            // teaches is chosen. New Game is the top of the list again.
            if ui.add(egui::Button::new(RichText::new("New Game").size(22.0)).min_size(egui::vec2(220.0, 44.0))).clicked() {
                session.screen = Screen::ChooseFaction;
            }
            ui.add_space(10.0);
            // Ticket #59: every save this machine holds, newest first.
            if ui.add(egui::Button::new(RichText::new("Load").size(22.0)).min_size(egui::vec2(220.0, 44.0))).clicked() {
                actions.push(Action::OpenLoad);
            }
            ui.add_space(10.0);
            // Ticket #109 (version 0.07.0): the resource icons are CC BY 3.0, and their licence
            // wants their authors named somewhere a player can see them.
            if ui.add(egui::Button::new(RichText::new("Credits").size(22.0)).min_size(egui::vec2(220.0, 44.0))).clicked() {
                session.screen = Screen::Credits;
            }
            ui.add_space(10.0);
            if ui.add(egui::Button::new(RichText::new("Quit").size(22.0)).min_size(egui::vec2(220.0, 44.0))).clicked() {
                actions.push(Action::Quit);
            }
            ui.add_space(30.0);
            ui.label(RichText::new(format!("Seed {}", session.seed)).weak());
        });
    });
}

/// Ticket #109 (version 0.07.0): the credits. It exists because the resource icons are
/// game-icons.net's under CC BY 3.0, whose one condition is that the authors are credited; this is
/// where anything else the game owes a credit to goes as it grows.
fn credits_screen(root: &mut Ui, session: &mut Session, icons: &Icons) {
    egui::CentralPanel::default().show(root, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(60.0);
            ui.label(RichText::new("Credits").size(40.0).strong());
            ui.add_space(24.0);
            ui.label(RichText::new("Icons").size(20.0).strong());
            ui.label(RichText::new("From game-icons.net, used under Creative Commons BY 3.0.").size(15.0));
            ui.add_space(10.0);
            // Ticket #146 (version 0.07.3): thirty-two credits and a drawing no longer fit one column
            // in an 800-pixel window -- the first picture of this screen ran off its bottom -- so
            // the list is two columns, the figures and kinds on the left and the buildings on the
            // right, each row an icon at 22 pixels beside its line.
            let mut rows: Vec<(String, String)> = crate::icons::CREDITS.iter().map(|c| (c.key(), format!("{}: \"{}\" by {}", c.resource, c.icon, c.author))).collect();
            // Ticket #135 (version 0.07.3): the game's own drawings, named so the list is complete.
            for d in crate::icons::DRAWN {
                let name = format!("{}{}", d[..1].to_uppercase(), &d[1..]);
                rows.push((d.to_string(), format!("{name}: drawn for Dying Earth, no credit owed")));
            }
            let half = rows.len().div_ceil(2);
            ui.horizontal_top(|ui| {
                ui.add_space(ui.available_width() / 2.0 - 400.0);
                for column in [&rows[..half], &rows[half..]] {
                    ui.vertical(|ui| {
                        ui.set_width(390.0);
                        for (key, line) in column {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 8.0;
                                if let Some(image) = icons.image(key, 20.0) {
                                    ui.add(image);
                                }
                                ui.label(RichText::new(line).size(13.0));
                            });
                        }
                    });
                }
            });
            ui.add_space(12.0);
            ui.label(RichText::new("https://game-icons.net").size(14.0).weak());
            // Ticket #122 (version 0.07.2): the Nations' flags. MIT asks nothing on screen; they are
            // named here anyway, since a player who wonders where the art came from should not
            // have to open a folder to find out, and the provenance caveat the research raised is
            // worth a line.
            ui.add_space(18.0);
            ui.label(RichText::new("Flags").size(20.0).strong());
            ui.label(RichText::new("From flag-icons (github.com/lipis/flag-icons), under the MIT licence.").size(15.0));
            ui.label(RichText::new("Its licence text ships beside the flags, in assets/flags.").size(14.0).weak());
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.add_space(ui.available_width() / 2.0 - 6.0 * 40.0);
                ui.spacing_mut().item_spacing.x = 10.0;
                for sid in StateId::ALL {
                    let card = session.tables.state(sid);
                    if let Some(flag) = Icons::flag_from_ctx(ui.ctx(), &card.flag, 24.0) {
                        ui.add(flag).on_hover_text(&card.name);
                    }
                }
            });
            ui.add_space(30.0);
            if ui.add(egui::Button::new(RichText::new("Back").size(20.0)).min_size(egui::vec2(180.0, 38.0))).clicked() {
                session.screen = Screen::Title;
            }
        });
    });
}

/// Ticket #59: the Load screen. Every save the folder holds, newest first, each row naming the
/// Faction (or Spectating), the turn and its month, the Temperature, the seed and when it was
/// written, with Load and Delete beside it. Delete asks once.
fn load_screen(root: &mut Ui, session: &Session, actions: &mut Vec<Action>) {
    egui::Panel::bottom("load_bar").show(root, |ui| {
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            if ui.add(egui::Button::new(RichText::new("Back").size(17.0)).min_size(egui::vec2(120.0, 32.0))).clicked() {
                actions.push(Action::ToTitle);
            }
            if ui.add(egui::Button::new(RichText::new("Open saves folder").size(17.0)).min_size(egui::vec2(200.0, 32.0))).clicked() {
                actions.push(Action::OpenSavesFolder);
            }
            match &session.saves {
                Ok(dir) => {
                    ui.label(RichText::new(dir.display().to_string()).weak());
                }
                Err(e) => {
                    ui.label(RichText::new(e).color(Color32::from_rgb(230, 130, 110)));
                }
            }
        });
        if let Some(e) = &session.last_error {
            ui.label(RichText::new(e).color(Color32::from_rgb(230, 130, 110)));
        }
        ui.add_space(6.0);
    });
    egui::CentralPanel::default().show(root, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(14.0);
            ui.label(RichText::new("Load a game").size(30.0).strong());
            ui.label(RichText::new("A save is the beginning of a turn. Loading one puts the game back exactly where it stood.").size(15.0));
            ui.add_space(10.0);
        });
        if session.saves_list.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                let text = match &session.saves {
                    Ok(dir) if dir.exists() => "There are no saves in the folder yet. The game saves itself every three turns, and the Save button in the top bar takes one at any turn start.",
                    Ok(_) => "The saves folder is not there yet. It is made the first time the game saves.",
                    Err(_) => "The game has nowhere to save on this machine, so there is nothing to load.",
                };
                ui.label(RichText::new(text).size(16.0).weak());
            });
            return;
        }
        egui::ScrollArea::vertical().show(ui, |ui| {
            for entry in &session.saves_list {
                let h = &entry.header;
                egui::Frame::NONE.fill(Color32::from_gray(28)).inner_margin(8.0).outer_margin(egui::vec2(0.0, 3.0)).show(ui, |ui| {
                    // A row is as wide as the list, so the saves read as a column of rows rather
                    // than as blocks of whatever width their text happens to want.
                    ui.set_width(ui.available_width());
                    ui.horizontal_wrapped(|ui| {
                        let colour = match session.tables.factions.iter().find(|f| f.name == h.faction) {
                            Some(card) => rgb(card.colour),
                            None => Color32::from_gray(200),
                        };
                        ui.label(RichText::new(&h.faction).strong().size(17.0).color(colour));
                        ui.separator();
                        ui.label(RichText::new(format!("Turn {}, {}", h.turn, h.date)).strong());
                        ui.separator();
                        ui.label(format!("{:+.1} C", h.temperature));
                        ui.separator();
                        ui.label(RichText::new(format!("Seed {}", h.seed)).weak());
                        ui.separator();
                        ui.label(RichText::new(crate::saves::when_text(entry.saved)).weak());
                        if h.kind.is_autosave() {
                            ui.separator();
                            ui.label(RichText::new("autosave").weak().italics());
                        }
                    });
                    ui.horizontal(|ui| {
                        if session.confirm_delete.as_deref() == Some(entry.path.as_path()) {
                            ui.label(RichText::new("Delete this save for good?").color(Color32::from_rgb(230, 160, 110)));
                            if ui.button("Delete").clicked() {
                                actions.push(Action::DeleteSave(entry.path.clone()));
                            }
                            if ui.button("Keep it").clicked() {
                                actions.push(Action::AskDelete(None));
                            }
                        } else {
                            if ui.add(egui::Button::new(RichText::new("Load").strong())).clicked() {
                                actions.push(Action::LoadSave(entry.path.clone()));
                            }
                            if ui.button("Delete").clicked() {
                                actions.push(Action::AskDelete(Some(entry.path.clone())));
                            }
                            ui.label(RichText::new(entry.path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default()).weak().small());
                        }
                    });
                });
            }
        });
    });
}

/// Ticket #50: every game seats all four Factions, so the choice screen deals four cards in two
/// rows of two. The three not picked are played by the computer.
fn faction_screen(root: &mut Ui, session: &Session, actions: &mut Vec<Action>) {
    // Ticket #64: or take no seat at all and watch the four of them play. It stands under the
    // cards and outside their scroll, so it is on screen whatever the cards do.
    egui::Panel::bottom("spectate_bar").show(root, |ui| {
        ui.add_space(6.0);
        ui.vertical_centered(|ui| {
            if ui.add(egui::Button::new(RichText::new("Spectate").size(19.0)).min_size(egui::vec2(260.0, 40.0))).clicked() {
                actions.push(Action::Spectate);
            }
            ui.label(RichText::new("Spectate: the computer plays all four; you watch.").size(15.0).weak());
        });
        ui.add_space(6.0);
    });
    egui::CentralPanel::default().show(root, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.label(RichText::new("Choose your Faction").size(30.0).strong());
            ui.label(RichText::new("All four sit at every table: you take one seat, the computer plays the other three.").size(15.0));
            ui.add_space(10.0);
        });
        // Ticket #217 (version 0.08.2): the scrollbar shows ONLY when scrolling is required and is
        // quiet when everything fits. That is the fix for the real defect this ticket found: at the
        // default 1280x800 and at 1600x900 the second row is clipped and NEITHER the Arkwrights' nor
        // the Archivists' Play button is on screen, with nothing saying the page scrolls at all. The
        // grid is allowed to scroll -- no fit target was set -- so the cue is the whole remedy.
        // `VisibleWhenNeeded` alone is not enough, and a picture is what showed it: egui's default
        // scrollbar FLOATS, so it fades to nothing while no pointer is near it -- which is exactly
        // the state a player is in when the screen opens. A non-floating bar is laid out solidly
        // whenever the content overflows, and is absent entirely when it does not.
        ui.style_mut().spacing.scroll.floating = false;
        egui::ScrollArea::vertical()
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
            .show(ui, |ui| {
                // All four cards take ONE height, the tallest's, so the screen reads as a 2x2 matrix
                // rather than as four cards of their own lengths. The height cannot be known before
                // the cards are drawn, so it is measured and carried to the next frame through
                // egui's own temporary memory: one frame of raggedness on the very first paint, and
                // stable afterwards. `ui.columns` equalises width and never height, which is why the
                // screen looked ragged despite already being a 2x2.
                let id = egui::Id::new("faction-card-height");
                let want: f32 = ui.ctx().memory(|m| m.data.get_temp(id).unwrap_or(0.0));
                let mut tallest: f32 = 0.0;
                for row in FactionKind::ALL.chunks(2) {
                    ui.columns(2, |cols| {
                        for (i, kind) in row.iter().enumerate() {
                            let h = faction_card(&mut cols[i], session, *kind, want, actions);
                            tallest = tallest.max(h);
                        }
                    });
                    ui.add_space(6.0);
                }
                ui.ctx().memory_mut(|m| m.data.insert_temp(id, tallest));
            });
    });
}

/// Ticket #203 (version 0.08.1): the Faction's symbol and its name, at the head of a setup card
/// and of the Faction window alike. `glyph` is how many pixels the symbol gets: 28 on the card, 64
/// in the window, the window's size chosen by looking at 48, 64 and 96 side by side.
///
/// Ticket #168 (version 0.07.5): the symbol stands where the Faction's colour swatch stood, drawn
/// in the Faction's own colour, so the head says which Faction and which colour in one mark and
/// grows by nothing. The designer: *"I want to pick four symbols to represent the factions - for
/// now these symbols should only appear on the faction selection screen in their respective
/// cards."* **That last clause is superseded at the designer's word on ticket #203**: the Faction
/// window is the setup card brought in-game, and a card without its symbol is not that card.
///
/// **This is the second place in the codebase where the CALLER picks an icon's colour**, and the
/// first is the setup screen's `faction_card`, which now reaches it through here. `icons::fill`
/// decides an icon's colour everywhere else. The rule is bent in this one function and nowhere
/// else, for the reason the rule exists: a colour on this board means WHOSE, and whose is the only
/// thing a Faction symbol is for. The research (#167) measured that the symbol wants 28 pixels
/// rather than the swatch's 24 -- at 24 a glyph has a quarter fewer pixels and three of the four
/// candidates stopped naming themselves.
fn faction_heading(ui: &mut Ui, session: &Session, kind: FactionKind, glyph: f32) {
    let card = session.tables.faction(kind);
    ui.horizontal(|ui| {
        let key = crate::icons::faction_symbol(kind);
        match Icons::from_ctx(ui.ctx(), key, glyph) {
            Some(image) => {
                ui.add(image.tint(rgb(card.colour)));
            }
            None => {
                // No art loaded: the swatch the symbol replaced, so the row is never empty. It
                // keeps the 24-to-28 proportion it had on the card at whatever size is asked for.
                let side = glyph * 24.0 / 28.0;
                let (swatch, _) = ui.allocate_exact_size(egui::vec2(side, side), egui::Sense::hover());
                ui.painter().rect_filled(swatch, 4.0, rgb(card.colour));
            }
        }
        // The name at the size the setup card uses, whatever the symbol beside it is doing.
        ui.label(RichText::new(&card.name).size(24.0).strong().color(rgb(card.colour)));
    });
}

/// **The Faction rulebook**: blurb, multipliers, Unique Facility, signature rule, Victory Condition
/// and gate Tech, in that order, with no head of its own -- `faction_heading` draws that.
///
/// Ticket #203 (version 0.08.1): factored out of `faction_card` so the setup screen's card and the
/// Faction window's collapsing header are the SAME code and can never drift. Everything a player
/// reads ABOUT a Faction is here; the two callers differ in exactly two things, and both of them
/// belong to the setup screen alone -- the `Play the X` button and the Custodians' tutorial tick.
fn faction_rulebook(ui: &mut Ui, session: &Session, kind: FactionKind) {
    let card = session.tables.faction(kind);
    // Ticket #260 (version 0.08.4): the motto, directly under the name -- which the caller has just
    // drawn -- in italics and the Faction's own colour, before the blurb: the one line on the card
    // where the Faction speaks rather than the rules describing it. The designer: "yes that
    // exactly"; and here and nowhere else, since the rulebook is this same code in-game.
    ui.label(RichText::new(&card.motto).italics().color(rgb(card.colour)));
    ui.label(&card.blurb);
    ui.add_space(6.0);
    ui.label(RichText::new("Multipliers").strong());
    // Ticket #132 (version 0.07.3): the four multipliers as a compact glyph row, every one shown
    // (x1 included) so the same glyph sits in the same place on all four cards and the eye can
    // compare Factions across the screen; the phrase each glyph replaced is on its hover.
    let part = |before: &str, icon: Option<&'static str>, after: String, hover: &str| RowPart { before: before.to_string(), icon, after, hover: Some(hover.to_string()) };
    glyph_row(
        ui,
        &[
            part("Output", None, format!("x{}", card.output_multiplier), &format!("Facility and Module output x{}", card.output_multiplier)),
            part("", Some("emissions"), format!("x{}", card.emissions_multiplier), &format!("Emissions from Earth sources it controls x{}", card.emissions_multiplier)),
            part("", Some("research"), format!("x{}", card.research_multiplier), &format!("Research x{}", card.research_multiplier)),
            part("", Some("influence"), format!("x{}", card.influence_multiplier), &format!("Influence Allotment x{}", card.influence_multiplier)),
        ],
        15.0,
    );
    // Ticket #51: a card may carry figures of its own beyond the four; list only the ones it moved.
    // Ticket #132: a mixed row -- only the eight Figures have glyphs; a Habitat or a Ship is a
    // piece and stays a word.
    let plain = |before: &str, icon: Option<&'static str>, after: String| RowPart { before: before.to_string(), icon, after, hover: None };
    let mut extras: Vec<RowPart> = Vec::new();
    if card.habitat_capacity_multiplier != 1.0 {
        extras.push(plain("Habitat capacity", None, format!("x{}", card.habitat_capacity_multiplier)));
    }
    if card.transit_fuel_multiplier != 1.0 {
        extras.push(plain("transit", Some("fuel"), format!("x{}", card.transit_fuel_multiplier)));
    }
    if card.colony_ship_capacity_multiplier != 1.0 {
        extras.push(plain("Colony Ship capacity", None, format!("x{}", card.colony_ship_capacity_multiplier)));
    }
    if card.lift_population_multiplier != 1.0 {
        extras.push(plain("", Some("population"), format!("per lifted Colonist x{}", card.lift_population_multiplier)));
    }
    if let Some(m) = card.colony_ship_materials {
        extras.push(plain(&format!("a Colony Ship {m}"), Some("materials"), String::new()));
    }
    // Ticket #83: the Arkwrights' Ships, the Prospectors' Ducats and market.
    // Ticket #332 (version 0.09.0): a Faction's discount on a Ship or a Module reaches its Widget
    // figure too, at the designer's word, so the row says both halves.
    if card.ship_materials_multiplier != 1.0 {
        extras.push(plain(&format!("every Ship x{}", card.ship_materials_multiplier), Some("materials"), String::new()));
        extras.push(plain("and", Some("widgets"), String::new()));
    }
    if card.ducats_multiplier != 1.0 {
        extras.push(plain("a state's", Some("ducats"), format!("x{}", card.ducats_multiplier)));
    }
    if card.market_multiplier != 1.0 {
        extras.push(plain("the Trading window's prices", None, format!("x{}", card.market_multiplier)));
    }
    if card.station_materials_multiplier != 1.0 {
        extras.push(plain(&format!("a Space Station x{}", card.station_materials_multiplier), Some("materials"), String::new()));
    }
    if card.module_materials_multiplier != 1.0 {
        extras.push(plain(&format!("a Colony Module x{}", card.module_materials_multiplier), Some("materials"), String::new()));
        extras.push(plain("and", Some("widgets"), String::new()));
    }
    if !extras.is_empty() {
        glyph_row(ui, &extras, 15.0);
    }
    ui.add_space(6.0);
    // Ticket #132: every price in the paragraphs by the one glyph rule -- `30 [cart], 2 turns,
    // 4 [bolt] upkeep`. `12 Colonists` stays words: Colonists are pieces, not the population figure.
    // Ticket #203 (version 0.08.1): the Unique Facility, which no card named until now. The
    // four arrived in version 0.08.0 (tickets #182 to #186) and none of the signature rules
    // was rewritten to mention them, so a player could build one having never been told what
    // it was. The sentence lives in `factions.toml` beside the blurb and the signature.
    ui.label(RichText::new("Unique Facility").strong());
    ui.label(&card.unique);
    ui.add_space(6.0);
    let ink = ui.visuals().text_color();
    ui.label(RichText::new("Signature rule").strong());
    draw_with_icons(ui, &card.signature, 14.0, ink, &[]);
    ui.add_space(6.0);
    ui.label(RichText::new("Victory Condition").strong());
    draw_with_icons(ui, &card.victory, 14.0, ink, &[]);
    // Ticket #84: the gate Tech it waits on.
    if let Some(gate) = session.tables.victory_gate(kind) {
        let t = session.tables.tech(gate);
        ui.label(RichText::new(format!("Waits on {}, a rung-{} Tech ({} Research): {}.", t.name, t.rung, t.cost, t.effect)).weak());
    }
}

/// One Faction's card on the setup screen: its symbol and name, the rulebook, the button that plays
/// it, and the tutorial tick.
/// Ticket #217 (version 0.08.2): `want` is the tallest card's CONTENT height, measured last frame;
/// the card pads its own content out to it and then places its Play button, so the four buttons sit
/// on two clean lines instead of wherever each rulebook happened to end. Returns this card's content
/// height, so the caller can take the maximum for the next frame.
fn faction_card(ui: &mut Ui, session: &Session, kind: FactionKind, want: f32, actions: &mut Vec<Action>) -> f32 {
    let card = session.tables.faction(kind);
    let r = egui::Frame::group(ui.style()).inner_margin(12.0).show(ui, |ui| {
        faction_heading(ui, session, kind, 28.0);
        faction_rulebook(ui, session, kind);
        // What the foot of the card costs: the button, the space above it, and on the Custodians'
        // card the tutorial tick beneath. Everything above is pushed up by padding out to `want`.
        // The tick's height is reserved on ALL FOUR cards, not only the Custodians' -- otherwise the
        // one card carrying it would seat its button that much higher than the other three and the
        // buttons would not line up, which is the whole point of pinning them. Three cards end with
        // that much empty air, which is the price of a straight row and is invisible.
        let used: f32 = ui.min_rect().height();
        // What is measured and matched across the four cards is the CONTENT height -- everything
        // above the foot -- not the finished card's. Matching finished heights looked right and was
        // not: the tallest card in a row hits the minimum-space clamp, `ui.columns` then stretches
        // the shorter card's frame to match, and its button has already been placed off the stale
        // figure. Padding the content to a common height puts every button at the same offset from
        // the top of its card, which is what pinning them means.
        // The 10 is added to EVERY card rather than used as a floor. As a floor it applied only to
        // the tallest card -- the one card whose content already equals `want` -- seating its button
        // ten pixels below the other three, which is precisely the raggedness being removed.
        ui.add_space((want - used).max(0.0) + 10.0);
        if ui.add(egui::Button::new(RichText::new(format!("Play the {}", card.name)).size(17.0)).min_size(egui::vec2(190.0, 36.0))).clicked() {
            actions.push(Action::ChooseFaction(kind));
        }
        // Ticket #174 (version 0.07.6): the tutorial is asked for here, at the foot of the one card
        // it belongs to, and nowhere else. The designer: *"move tutorial choice to a radio box on
        // custodian card during faction selection"*, placed at the bottom of the card and reading
        // `Play Tutorial`. The other three cards stay quiet: a line about a thing this card cannot
        // give you is clutter. The tick is remembered while the screen is open and read once the
        // start has been chosen, since a ticked card still picks its own Region.
        if kind == FactionKind::Custodians {
            ui.add_space(6.0);
            let mut on = session.tutorial_ticked;
            // Ticket #217 (version 0.08.2): a fifth larger at the designer's word. The checkbox took
            // egui's default size, so there was no number to multiply and one had to be named; this
            // follows how ticket #211 handled the command cluster's 1.15. Its placement is unchanged
            // -- ticket #174 put it at the foot of this card deliberately.
            if ui
                .checkbox(&mut on, RichText::new("Play Tutorial").size(TUTORIAL_TICK))
                // Ticket #290 (version 0.08.6): the count is the table's, since it moved from five
                // to six and a literal would have gone stale a second time.
                .on_hover_text(format!("A note at the head of each of the first {} turns, saying what that turn is for. Nothing is forced, and it stops after the last one.", session.tables.tutorial.note.len()))
                .changed()
            {
                actions.push(Action::SetTutorialTick(on));
            }
        }
        used
    });
    r.inner
}

#[allow(clippy::too_many_arguments)]
/// Ticket #126 (version 0.07.2): the start is chosen on the map. The designer: *"Starting location
/// selection screen should display the regions and allow the player to choose by clicking map;
/// retire the clickable list."* Every Region wears its own colour with its borders drawn, its name
/// is painted on the globe where the game view paints it, the Region under the pointer lights up
/// and its card appears in the panel, a click chooses it, and a Begin button starts the game -- the
/// list of fourteen buttons is gone. A click chooses rather than starts because a globe of real
/// borders has small Regions beside large ones and a mis-click on Japan should cost nothing.
fn start_screen(
    root: &mut Ui,
    session: &Session,
    faction: FactionKind,
    view: &mut ViewState,
    cam: Option<(&Camera, &GlobalTransform)>,
    globes: &Query<(&Globe, &GlobalTransform)>,
    textures: &Textures,
    actions: &mut Vec<Action>,
) {
    // Ticket #100 (version 0.07.0): open the globe on this Faction's home, once. Aiming every frame
    // would undo a drag as fast as the player made it.
    if view.start_aimed != Some(faction) {
        let (lon, lat) = geo::state_lonlat(session.tables.faction(faction).opens_on);
        view.spin = geo::yaw_facing(lon, lat);
        view.yaw = view.spin;
        // Tilt to the home's latitude as well, or a northern continent sits on the limb.
        view.pitch = (lat.to_radians()).clamp(-1.3, 1.3);
        view.zoom = 1.0;
        view.start_grabbed = false;
        view.start_aimed = Some(faction);
        view.start_selected = None;
        view.start_hover = None;
    }
    egui::Panel::right("start_panel").default_size(340.0).show(root, |ui| {
        ui.add_space(10.0);
        ui.label(RichText::new("Choose your starting Region").size(22.0).strong());
        ui.label(format!("You play the {}.", faction.name()));
        // Ticket #50: the three Factions not picked are played by the computer, in their own colours.
        ui.horizontal_wrapped(|ui| {
            ui.label("Played by the computer:");
            for k in FactionKind::ALL.into_iter().filter(|k| *k != faction) {
                ui.label(RichText::new(k.name()).strong().color(rgb(session.tables.faction(k).colour)));
            }
        });
        ui.label("Each computer Faction takes the uncontrolled Region with the highest Industry Level.");
        ui.add_space(14.0);
        // Ticket #126: the card of the Region chosen, else of the one under the pointer, with the
        // Nation's flag at the size the Region card uses; and Begin once one is chosen.
        let shown = view.start_selected.or(view.start_hover);
        match shown {
            Some(sid) => {
                let c = session.tables.state(sid);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 10.0;
                    if let Some(flag) = Icons::flag_from_ctx(ui.ctx(), &c.flag, 32.0) {
                        ui.add(flag);
                    }
                    ui.label(RichText::new(&c.name).size(32.0).strong());
                });
                // Ticket #132 (version 0.07.3): the Region's lean as a glyph, and a second line of
                // the three figures a start is chosen on -- Influence value, Ducats a turn and
                // Emissions -- read from the cards, since no game exists yet.
                let lean = format!("{:?}", c.resource_lean).to_lowercase();
                let lean_key: &'static str = match lean.as_str() {
                    "materials" => "materials",
                    "fuel" => "fuel",
                    _ => "energy",
                };
                glyph_row(
                    ui,
                    &[
                        RowPart { before: String::new(), icon: Some("population"), after: format!("Region population {}", Game::population_text(c.population)), hover: Some("The whole Region's people, not its Nation's alone, in units of five million.".to_string()) },
                        RowPart { before: format!("Industry Level {}", c.industry_level), icon: None, after: String::new(), hover: None },
                        RowPart { before: "leans".to_string(), icon: Some(lean_key), after: String::new(), hover: Some(format!("Leans {:?}: the resource this Region is naturally good at producing.", c.resource_lean)) },
                    ],
                    15.0,
                );
                let ducats = session.tables.start_ducats(sid, faction);
                glyph_row(
                    ui,
                    &[
                        RowPart { before: String::new(), icon: Some("influence"), after: format!("{}", c.influence), hover: Some(format!("Influence value {}: what it adds to its controller's Allotment each turn.", c.influence)) },
                        RowPart { before: String::new(), icon: Some("ducats"), after: format!("{ducats} a turn"), hover: Some(format!("GDP {}: its economy pays {} Ducats a turn (GDP x Industry Level / 5, never below 1).", c.gdp, ducats)) },
                        RowPart { before: String::new(), icon: Some("emissions"), after: format!("{:.1}", session.tables.start_emissions(sid, faction)), hover: Some("Emissions a turn as the game opens: its industry, its people and its start Facilities.".to_string()) },
                        RowPart { before: format!("Education Level {}", c.education_level), icon: None, after: String::new(), hover: None },
                    ],
                    15.0,
                );
                ui.label(RichText::new(if view.start_selected == Some(sid) { "Chosen. Begin, or click another Region." } else { "Click it to choose." }).weak());
            }
            None => {
                ui.label(RichText::new("Turn the globe and point at a Region to read it; click one to choose it.").weak());
            }
        }
        ui.add_space(14.0);
        let begin = egui::Button::new(RichText::new("Begin").size(18.0).strong()).min_size(egui::vec2(160.0, 36.0));
        if ui.add_enabled(view.start_selected.is_some(), begin).on_disabled_hover_text("Choose a Region on the globe first.").clicked()
            && let Some(sid) = view.start_selected
        {
            actions.push(Action::NewGame(faction, sid));
        }
        ui.add_space(20.0);
        ui.label(RichText::new("Antarctica has no people to govern: it is three Colony Slots, founded from a Colony Ship at Earth.").weak());
    });
    egui::CentralPanel::default().frame(egui::Frame::NONE).show(root, |ui| {
        let (_, resp) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
        // Ticket #100: dragging turns the globe, the wheel zooms it, and a click picks the continent
        // under the pointer. The first drag stops the spin for good: a spin that carried the
        // Faction's home back out of view would undo the point of opening on it.
        let d = resp.drag_motion();
        if d != egui::Vec2::ZERO {
            if !view.start_grabbed {
                // Take the angle the spin had reached, so the globe does not jump when grabbed.
                view.yaw = view.spin;
                view.start_grabbed = true;
            }
            view.yaw += d.x * 0.008;
            view.pitch = (view.pitch + d.y * 0.008).clamp(-1.3, 1.3);
        }
        if resp.hovered() {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll.abs() > 0.0 {
                view.zoom = (view.zoom * (1.0 - scroll * 0.002)).clamp(0.45, 2.2);
            }
        }
        // Ticket #126: the Region under the pointer lights up; a click chooses it; Begin starts.
        view.start_hover = match (resp.hover_pos(), cam) {
            (Some(pos), Some((camera, cam_gt))) if resp.hovered() => start_pick(pos, camera, cam_gt, globes, textures),
            _ => None,
        };
        if resp.clicked()
            && let Some(pos) = resp.interact_pointer_pos()
            && let Some((camera, cam_gt)) = cam
            && let Some(sid) = start_pick(pos, camera, cam_gt, globes, textures)
        {
            view.start_selected = Some(sid);
            // Ticket #152 (version 0.07.4): a click stops the spin for good as a drag does, so the
            // Region chosen stays where it was chosen.
            if !view.start_grabbed {
                view.yaw = view.spin;
                view.start_grabbed = true;
            }
        }
        // The names, painted where the game view paints them: on the globe, lifted, over a plate.
        if let (Some((camera, cam_gt)), Some((_, globe_gt))) = (cam, globes.iter().find(|(g, _)| g.0 == BodyId::Earth)) {
            let painter = ui.painter();
            let center = globe_gt.translation();
            let cam_pos = cam_gt.translation();
            for sid in StateId::ALL {
                let (lon, lat) = geo::state_lonlat(sid);
                let world = globe_gt.transform_point(geo::local_from_lonlat(lon, lat) * 1.01);
                if (world - center).dot(cam_pos - center) < 0.25 * GLOBE_RADIUS * (cam_pos - center).length() {
                    continue;
                }
                let Some(p) = camera.world_to_viewport(cam_gt, world).ok().map(|v| Pos2::new(v.x, v.y)) else { continue };
                let card = session.tables.state(sid);
                let lit = view.start_selected == Some(sid) || view.start_hover == Some(sid);
                label_kind_at(painter, p, Some(Kind::Region), &card.name, if lit { Color32::WHITE } else { rgb(card.colour) }, if lit { 15.0 } else { 13.0 });
            }
        }
    });
}

/// Ticket #100 (version 0.07.0): which Region the pointer is over on the start screen's
/// globe. The playing screen's picker reads a `Game`, and on this screen no game exists yet, so
/// this walks the twelve cards' own longitudes and latitudes instead and takes the nearest.
fn start_pick(pos: Pos2, camera: &Camera, cam_gt: &GlobalTransform, globes: &Query<(&Globe, &GlobalTransform)>, textures: &Textures) -> Option<StateId> {
    let ray = camera.viewport_to_world(cam_gt, Vec2::new(pos.x, pos.y)).ok()?;
    let (origin, dir) = (ray.origin, Vec3::from(ray.direction));
    let (_, globe_gt) = globes.iter().find(|(g, _)| g.0 == BodyId::Earth)?;
    let t = geo::ray_sphere(origin, dir, globe_gt.translation(), GLOBE_RADIUS)?;
    let local = globe_gt.affine().inverse().transform_point3(origin + dir * t);
    let (lon, lat) = geo::lonlat_from_local(local);
    // Ticket #126 (version 0.07.2): the Region under the point, by the mask. Until now this took the
    // nearest label, which on a board of real borders gave Kazakhstan to Russia; the border a
    // player sees is the border that answers.
    textures.state_at_lonlat(lon, lat)
}

// ------------------------------------------------------------------ the game

#[allow(clippy::too_many_arguments)]
fn game_screen(
    root: &mut Ui,
    ctx: &egui::Context,
    session: &Session,
    view: &mut ViewState,
    cam: Option<(&Camera, &GlobalTransform)>,
    globes: &Query<(&Globe, &GlobalTransform)>,
    textures: &Textures,
    icons: &Icons,
    actions: &mut Vec<Action>,
) {
    let Some(game) = session.game.as_ref() else { return };
    // Ticket #104 (version 0.07.0): NOTHING opens itself. The Tech Tree used to throw itself up
    // whenever a pick was owed, which at turn 1 is always, so a new game began behind two panels.
    // The nudge that replaces it is the yellow line in the top bar, and the turn cannot be ended
    // with a pick outstanding (ticket #105), so a panel that opens itself buys nothing.
    top_bar(root, session, game, view, icons, actions);
    side_panel(root, session, game, view, actions);
    // The 3D area: drag turns, wheel zooms, click picks.
    let mut hotspots: Vec<Hotspot> = Vec::new();
    // Ticket #64: the globe fills the window and the panels are drawn over it, so the map's own
    // labels are clipped to what is left between them; otherwise a label for a place behind a panel
    // is painted across the panel's text.
    let painter = ctx.layer_painter(egui::LayerId::background()).with_clip_rect(root.available_rect_before_wrap());
    if let Some((camera, cam_gt)) = cam {
        overlays(&painter, session, game, view, camera, cam_gt, globes, &mut hotspots);
    }
    egui::CentralPanel::default().frame(egui::Frame::NONE).show(root, |ui| {
        let (rect, resp) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
        if view.popup == Popup::None && session.screen == Screen::Playing {
            let d = resp.drag_motion();
            match view.view {
                View::Solar => view.solar_yaw += d.x * 0.01,
                View::Surface(_) => {
                    view.yaw += d.x * 0.008;
                    view.pitch = (view.pitch + d.y * 0.008).clamp(-1.3, 1.3);
                }
            }
            // Ticket #216 (version 0.08.2): hovering a Faction's line in an in-orbit block names
            // that Faction's hulls. The block itself is unchanged -- the designer wanted the maps to
            // look as they do -- so this is the only way a name reaches a Ship sitting at a Body.
            // Both maps use it: the Solar System Map's stack labels already carried a hotspot for
            // the click, and the Body Surface Map's block grew one above.
            //
            // Capped at five with a tail, which keeps the standing six-line limit on a tooltip: the
            // worst fleet ever measured was 27 hulls, and 27 lines over an eighteen-pixel Earth is
            // what the stress mockup showed to be unreadable. The click is untouched and still
            // selects the stack, where the full list lives.
            if let Some(p) = resp.hover_pos() {
                let mut best: Option<(f32, BodyId, Seat)> = None;
                for h in &hotspots {
                    if let Hit::Select(Selection::ShipStack(b, s)) = h.hit {
                        let d = h.pos.distance(p);
                        if d <= h.radius && best.map(|(bd, _, _)| d < bd).unwrap_or(true) {
                            best = Some((d, b, s));
                        }
                    }
                }
                if let Some((_, b, s)) = best {
                    let ids = game.ships_at(s, b);
                    let mut lines: Vec<String> = ids
                        .iter()
                        .filter_map(|id| game.ship(*id))
                        .take(5)
                        .map(|sh| format!("{} ({})", game.ship_name(sh), sh.kind.name().to_lowercase()))
                        .collect();
                    if ids.len() > 5 {
                        lines.push(format!("and {} more - click the stack", ids.len() - 5));
                    }
                    if !lines.is_empty() {
                        resp.show_tooltip_text(lines.join("\n"));
                    }
                }
                // Ticket #311 (version 0.08.7): the Battle ring's hover, read from the record.
                let mut ring: Option<(f32, usize)> = None;
                for h in &hotspots {
                    if let Hit::Battle(i) = h.hit {
                        let d = h.pos.distance(p);
                        if d <= h.radius && ring.map(|(bd, _)| d < bd).unwrap_or(true) {
                            ring = Some((d, i));
                        }
                    }
                }
                if let Some((_, i)) = ring {
                    resp.show_tooltip_text(battle_summary(game, i));
                }
            }
            if resp.hovered() {
                let scroll = ui.input(|i| i.smooth_scroll_delta.y);
                if scroll.abs() > 0.0 {
                    view.zoom = (view.zoom * (1.0 - scroll * 0.002)).clamp(0.45, 2.2);
                }
            }
            // Ticket #151 (version 0.07.4): a click lands where the mouse was PRESSED, not where it
            // was released, so a station travelling its ring is caught by the click that began on it.
            let click = if resp.clicked() { ui.input(|i| i.pointer.press_origin()).or(resp.interact_pointer_pos()).filter(|p| rect.contains(*p)) } else { None };
            if let (Some(pos), Some((camera, cam_gt))) = (click, cam) {
                pick(pos, session, game, view, camera, cam_gt, globes, textures, &hotspots);
            }
            // Ticket #323 (version 0.08.8): a right-click moves the armed stack, or the selected
            // Ship stack; it never selects.
            let right = if resp.secondary_clicked() { resp.interact_pointer_pos().filter(|p| rect.contains(*p)) } else { None };
            if let (Some(pos), Some((camera, cam_gt))) = (right, cam) {
                right_click(pos, session, game, view, camera, cam_gt, globes, textures, actions);
            }
        }
    });
    popups(ctx, session, game, view, actions);
}

/// Ticket #109 (version 0.07.0): one resource on the top bar. The icon REPLACES the word there,
/// since the bar is cramped and the glyphs are learned in a turn or two; everywhere else the icon
/// sits beside its words. Where the art did not load the word comes back, so the bar is never mute.
fn bar_resource(ui: &mut Ui, icons: &Icons, key: &str, word: &str, value: String, hover: String) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        match icons.image(key, 16.0) {
            Some(image) => {
                ui.add(image).on_hover_text(format!("{word}. {hover}"));
                ui.label(RichText::new(value).strong()).on_hover_text(format!("{word}. {hover}"));
            }
            None => {
                ui.label(RichText::new(format!("{word} {value}")).strong()).on_hover_text(hover);
            }
        }
    });
}

/// Ticket #153 (version 0.07.4): `bar_resource` for a figure whose hover draws something -- the
/// Emissions figure and its history. One tooltip on the glyph and the label together, so the chart
/// is never painted twice.
fn bar_resource_with(ui: &mut Ui, icons: &Icons, key: &str, word: &str, value: String, add: impl Fn(&mut Ui)) {
    let resp = ui
        .horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            match icons.image(key, 16.0) {
                Some(image) => {
                    ui.add(image);
                    ui.label(RichText::new(value).strong());
                }
                None => {
                    ui.label(RichText::new(format!("{word} {value}")).strong());
                }
            }
        })
        .response
        .interact(egui::Sense::hover());
    rule_tip_ui(resp, word, add);
}

/// The Influence figure's hover, lifted out of the bar when ticket #128 moved the figure.
const INFLUENCE_HOVER: &str = "The Allotment is what your places and buildings give each turn; bought Influence comes from the Trading window at 2 Ducats each.";

/// Ticket #128 (version 0.07.2): a button of the bar's second row, one step larger than egui's
/// default -- "make buttons slightly larger" -- and naming its key in parentheses as the text.
fn bar_button(text: impl Into<String>) -> egui::Button<'static> {
    egui::Button::new(RichText::new(text).size(BAR_BUTTON_TEXT))
}

const BAR_BUTTON_TEXT: f32 = 15.0;
const BAR_BUTTON_PADDING: egui::Vec2 = egui::vec2(10.0, 5.0);

fn top_bar(root: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, icons: &Icons, actions: &mut Vec<Action>) {
    let bar = egui::Panel::top("top_bar").show(root, |ui| {
        // Ticket #64: the spectator's bar names the table instead of a Faction of their own, and
        // says whose Stockpile the numbers beside it are.
        if session.spectator {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Spectating.").strong());
                for seat in Seat::ALL {
                    ui.label(RichText::new(game.seat_name(seat)).strong().color(seat_colour(session, seat)));
                }
                // Ticket #60: whose Stockpile the row below shows, said here instead of as a prefix
                // on that row: at 1280 wide the prefix pushed the Temperature onto a second line,
                // and this line has room to spare. "- the computer plays all four." went with it,
                // since four Faction names under "Spectating." say the same thing.
                ui.label(RichText::new(format!("- the figures below are the {}'.", game.seat_name(Seat(0)))).weak());
            });
        }
        ui.horizontal_wrapped(|ui| {
            let s = game.seat(Seat(0));
            let (left, influence_left) = game.remaining(Seat(0), &session.pending);
            let inc = s.income_last_turn;
            let signed = |v: i64| if v >= 0 { format!("+{v}") } else { format!("{v}") };
            // Hover a resource for last Income by source (ticket #31).
            let sources = |res: dying_earth_engine::Resource| -> String {
                let lines: Vec<String> = s.income_sources.iter().filter(|(_, r, _)| *r == res).map(|(name, _, v)| format!("{v:+}  {name}")).collect();
                if lines.is_empty() { "No income from buildings last turn.".to_string() } else { format!("Last Income:\n{}", lines.join("\n")) }
            };
            bar_resource(ui, icons, "materials", "Materials", format!("{} ({})", left.materials, signed(inc.materials)), sources(dying_earth_engine::Resource::Materials));
            ui.separator();
            // Ticket #332 (version 0.09.0): Widgets, the work half of every build, beside the
            // Materials half. The figure is MADE / APPLIED -- what every place the player directs
            // makes a turn, and what was applied at the last Resolution -- Faction-wide, at the
            // designer's word; the hover lists each place's rate, its makers and its queue. Drawn
            // by hand rather than through `bar_resource`, so the hover goes through `rule_tip` and
            // the `tip:` aid can photograph it.
            {
                let made: i64 = directed_places(game, Seat(0)).iter().map(|p| game.widgets_at(*p)).sum();
                let applied = game.widgets.applied_last[0];
                let resp = ui
                    .horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 4.0;
                        match icons.image("widgets", 16.0) {
                            Some(image) => {
                                ui.add(image);
                                ui.label(RichText::new(format!("{made} / {applied}")).strong());
                            }
                            None => {
                                ui.label(RichText::new(format!("Widgets {made} / {applied}")).strong());
                            }
                        }
                    })
                    .response
                    .interact(egui::Sense::hover());
                rule_tip(resp, widgets_bar_hover(game, made, applied));
                ui.separator();
            }
            bar_resource(ui, icons, "fuel", "Fuel", format!("{} ({})", left.fuel, signed(inc.fuel)), sources(dying_earth_engine::Resource::Fuel));
            ui.separator();
            bar_resource(ui, icons, "energy", "Energy", format!("{} ({})", left.energy, signed(inc.energy)), sources(dying_earth_engine::Resource::Energy));
            ui.separator();
            bar_resource(ui, icons, "ducats", "Ducats", format!("{} ({})", left.ducats, signed(inc.ducats)), sources(dying_earth_engine::Resource::Ducats));
            // Ticket #72: the Prospectors' Fund stood beside their Materials from version 0.05.5,
            // when the Fund held Materials. Ticket #308 (version 0.08.7): moved beside Ducats as a
            // progress bar, then CUT on the designer's seeing it -- *"let's just cut it, the
            // archivist bank is not shown on the bar"* -- so the top bar carries no Faction's fund;
            // the Victory window carries the Prospectors'.
            ui.separator();
            // Ticket #128 (version 0.07.2): Influence stands before Research, at the designer's
            // word; the race bar and the Pick a Tech button are Research's and travel with it.
            {
                // Ticket #42: the turn's Allotment and what the trading window added, shown apart.
                let bought: i64 = session.pending.iter().map(|o| if let Order::BuyInfluence { amount } = o { *amount } else { 0 }).sum();
                let influence = if bought > 0 { format!("{} of {} ({} free + {} bought)", influence_left, s.allotment + bought, s.allotment, bought) } else { format!("{} of {}", influence_left, s.allotment) };
                // Ticket #112 (version 0.07.1): Influence now has a glyph, so on the bar it follows the
                // same rule the five resources do -- the picture stands in place of the word.
                bar_resource(ui, icons, "influence", "Influence", influence, INFLUENCE_HOVER.to_string());
                ui.separator();
            }
            // Ticket #112 (version 0.07.1): the bar carries the FIGURE and the hover carries the
            // name. The Tech's title was the longest thing on the bar by a wide margin -- "39 / 40
            // toward Clean Manufacturing" against "14 (+6)" for a resource -- and it is the one
            // thing there that does not change from turn to turn, so it was paying for width with
            // nothing a player rereads. Nothing is lost: the name is on the hover, on the Tech Tree
            // button beside it, and in the race bar's own tooltip.
            let (research, research_hover) = match game.research.current {
                Some(t) => (
                    format!("{} / {}", game.research.progress, game.tables.tech(t).cost),
                    format!("Research: {} of {} toward {}.", game.research.progress, game.tables.tech(t).cost, game.tables.tech(t).name),
                ),
                None => {
                    let waiting = game.research.unallocated.iter().sum::<i64>() + game.research.unattributed;
                    (format!("- ({waiting} waiting)"), format!("Research: no Tech is chosen, and {waiting} Research is waiting for one."))
                }
            };
            // Ticket #104 (version 0.07.0): the Tech Tree no longer opens itself, so the bar has to
            // say when a pick is owed. End Turn is disabled until one is made, but that only shows
            // on a hover, and a player who does not know to look will not find it.
            if game.research.awaiting_pick == Some(Seat(0)) && game.research.current.is_none() && !game.available_techs().is_empty() && !session.spectator {
                // Ticket #163 (version 0.07.5): in the red End Turn wears, since the two are the
                // pair that gate a turn. The designer: *"'Pick a tech' button when present should be
                // red."* It was black text on the default dark fill, the least legible button on the bar.
                let pick = egui::Button::new(RichText::new("Pick a Tech").strong()).fill(TURN_RED);
                if ui.add(pick).on_hover_text("The Research Lead is yours: choose what the world researches next. The turn cannot end until you do.").clicked() {
                    view.show_tech = true;
                }
                ui.separator();
            }
            // Ticket #109: Research joins the others, its word replaced by its glyph on the bar.
            // Ticket #194 (version 0.08.0): the figure OPENS the Tech Tree, and so does the race bar
            // beside it. They sit side by side and are about the same thing, so making one live and
            // the other dead would be a distinction a player finds only by clicking. OPEN rather
            // than toggle: a player who clicks a FIGURE is asking to see what is behind it, and a
            // figure that makes a window vanish on a second click is a surprise. The `Tech Tree (T)`
            // button keeps toggling, because a button labelled "Tech Tree" reads as a switch.
            let research_hover = format!("{research_hover}\nClick to open the Tech Tree.");
            let opened = match icons.image("research", 16.0) {
                Some(image) => ui
                    .horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 4.0;
                        let glyph = ui.add(image).interact(egui::Sense::click());
                        let figure = ui.add(egui::Label::new(research).sense(egui::Sense::click()));
                        (glyph | figure).on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text(&research_hover).clicked()
                    })
                    .inner,
                None => ui
                    .add(egui::Label::new(format!("Research {research}")).sense(egui::Sense::click()))
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .on_hover_text(&research_hover)
                    .clicked(),
            };
            if opened {
                view.show_tech = true;
            }
            // Ticket #58: the Research race, as a bar of the four Factions' contributions to the
            // Tech under research, in Faction colours and in proportion.
            if research_race_bar(ui, session, game, 150.0, false) {
                view.show_tech = true;
            }
            ui.separator();
            // Ticket #57: the bar names the turn's month. Ticket #67 (version 0.05.5): a Turn is two months,
            // named by its first alone, so turn 2 reads March 2030.
            ui.label(RichText::new(format!("Turn {} / {}, {}", game.turn, game.tables.victory.turns, game.date_text())).strong());
            ui.separator();
            // Ticket #158 (version 0.07.4): the Temperature figure's hover draws its history.
            let temp = ui.label(format!("{:+.1} C, heading to {:+.1}", game.climate.temperature, game.target_temperature()));
            rule_tip_ui(temp, "Temperature history", |ui| {
                ui.set_max_width(300.0);
                ui.label(RichText::new("Temperature history").strong());
                ui.label("Where the heat stands, and where the CO2 Stock already in the air is taking it. The Climate Panel's bar shows what it has crossed and what is next.");
                temperature_history(ui, game, egui::vec2(280.0, 96.0));
            });
            ui.separator();
            // Ticket #112 (version 0.07.1): net Emissions join the bar. Until now the only way to
            // learn whether the world went over or under the Natural Sink this turn was to open the
            // Climate Panel; the Temperature beside it moves too slowly to answer that question.
            // Ticket #153 (version 0.07.4): the hover draws the Emissions history under its sentence.
            bar_resource_with(ui, icons, "emissions", "Emissions history", format!("{:+.1} ppm", game.climate.last.net()), |ui| {
                ui.set_max_width(300.0);
                ui.label(RichText::new("Emissions history").strong());
                ui.label("Net Emissions at the last Resolution: everything the world emitted less the Natural Sink and any Scrubbers. Above zero the CO2 Stock rose and the Temperature will follow; below zero it fell. The Climate Panel breaks it into its sources.");
                emissions_history(ui, game, egui::vec2(280.0, 96.0));
            });
            ui.separator();
            // Ticket #143 (version 0.07.3): Earth's people and space's, in real numbers. The
            // designer: *"Please track earth and space populations on the top bar."* Earth is the
            // Regions' figures and the Colonists in Antarctica; space is every Colonist living off
            // Earth, a station over Earth counting as off, as Off-world Presence counts it.
            let mut regions: Vec<(f64, String)> = StateId::ALL.iter().map(|s| (game.state(*s).population, game.tables.state(*s).name.clone())).collect();
            regions.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
            let mut bodies: Vec<(u32, String)> = BodyId::ALL
                .iter()
                .map(|b| (game.colonies.iter().filter(|c| c.body == *b && game.off_earth(c)).map(|c| c.colonists).sum::<u32>(), game.tables.body(*b).name.clone()))
                .filter(|(n, _)| *n > 0)
                .collect();
            bodies.sort_by_key(|(n, _)| std::cmp::Reverse(*n));
            // Ticket #166 (version 0.07.5): the four largest and a count of the rest. All fourteen
            // were listed, which buried the chart the player hovered for under a list they could
            // read off the map.
            const NAMED: usize = 4;
            let mut earth_lines: Vec<String> = regions.iter().take(NAMED).map(|(p, n)| format!("{n} {}", Game::people_text(*p))).collect();
            if regions.len() > NAMED {
                let rest: f64 = regions.iter().skip(NAMED).map(|(p, _)| *p).sum();
                earth_lines.push(format!("and {} more, {}", regions.len() - NAMED, Game::people_text(rest)));
            }
            let space_lines: Vec<String> = if bodies.is_empty() { vec!["nobody yet".to_string()] } else { bodies.iter().map(|(n, b)| format!("{} {}", b, Game::people_text(*n as f64))).collect() };
            // Ticket #166 (version 0.07.5): the figure's hover draws the population history under
            // its sentence, as the Emissions figure's does.
            let pop_sentence = format!(
                "On Earth: {}.\nOff Earth: {}.\nOne Colonist is five million people; a station over Earth is off Earth and Antarctica is on it.\nPioneers waiting on a card and Colonists aboard a Ship are in neither line.",
                earth_lines.join(", "),
                space_lines.join(", ")
            );
            bar_resource_with(
                ui,
                icons,
                "population",
                "Population history",
                format!("Earth {} · Space {}", Game::people_text(game.earth_population()), Game::people_text(game.space_population() as f64)),
                |ui| {
                    ui.set_max_width(300.0);
                    ui.label(RichText::new("Population history").strong());
                    ui.label(&pop_sentence);
                    population_history(ui, game, egui::vec2(280.0, 96.0));
                },
            );
        });
        ui.horizontal_wrapped(|ui| {
            // Ticket #128 (version 0.07.2): one step larger, and every button names its key. The
            // keys themselves are read in `keyboard`, and each does what its button does.
            ui.spacing_mut().button_padding = BAR_BUTTON_PADDING;
            if ui.add(bar_button("Tech Tree (T)")).clicked() {
                view.show_tech = !view.show_tech;
            }
            if ui.add(bar_button(if view.show_climate { "Hide Climate Panel (C)" } else { "Climate Panel (C)" })).clicked() {
                toggle_climate(view);
            }
            if ui.add(bar_button("Victory (V)")).clicked() {
                view.show_victory = !view.show_victory;
            }
            // Ticket #203 (version 0.08.1): the Faction window, beside the Victory window it took
            // Relations and Blame from.
            if ui.add(bar_button("Factions (F)")).clicked() {
                view.show_factions = !view.show_factions;
            }
            if !session.spectator && ui.add(bar_button("Trading (R)")).clicked() {
                view.show_trade = !view.show_trade;
            }
            // Ticket #59: a Save writes this turn start to a file. It is dead while an order is
            // pending, because a save captures a turn start and never half-entered orders. A
            // spectated game saves the same way.
            let can_save = dying_earth_engine::save::can_save_now(session.pending.len()) && session.screen == Screen::Playing;
            let save = ui.add_enabled(can_save, bar_button("Save (Ctrl+S)"));
            if save.on_disabled_hover_text(dying_earth_engine::save::SAVE_PENDING_HOVER).on_hover_text("Write this turn start to a file. Load it again from the title screen.").clicked() {
                actions.push(Action::Save);
            }
            if let Some((text, _)) = &session.save_notice {
                ui.label(RichText::new(text).strong().color(Color32::from_rgb(140, 210, 150)));
            }
            let swap_text = match view.view {
                View::Solar => format!("To {} (M)", game.tables.body(view.last_surface).name),
                View::Surface(_) => "Solar System Map (M)".to_string(),
            };
            if ui.add(bar_button(swap_text)).clicked() {
                view.swap();
            }
            if matches!(view.view, View::Surface(_)) && ui.add(bar_button("Back (Esc)")).clicked() {
                view.view = View::Solar;
                view.selection = Selection::None;
            }
            if session.screen == Screen::Playing {
                // Ticket #114 (version 0.07.1): End Turn moved into the command cluster at the foot
                // of the side panel, where the rest of the every-turn controls now are. A spectator
                // has no cluster -- they give no orders -- so theirs stays here beside the Auto box.
                if session.spectator {
                    let button = egui::Button::new(RichText::new("End Turn (Enter)").strong().size(16.0)).fill(TURN_RED);
                    if ui.add_enabled(view.popup == Popup::None, button).clicked() {
                        press_end_turn(session, game, view, actions);
                    }
                }
                // Ticket #64: Auto runs a turn every three seconds until it is unticked; the clock
                // stops while a Moment, the Report or the game-over popup is up. Escape unticks it.
                if session.spectator {
                    let mut auto = session.auto;
                    if ui.checkbox(&mut auto, "Auto").on_hover_text(format!("A turn every {:.0} seconds, paused while a Moment or the Report is up. Escape unticks it.", crate::app::AUTO_INTERVAL)).changed() {
                        actions.push(Action::SetAuto(auto));
                    }
                }
            }
        });
    });
    // Ticket #292 (version 0.08.6): the bar's foot, measured, for every window that opens under it.
    view.top_bar_bottom = bar.response.rect.max.y;
}

/// Ticket #58: the four Factions' shares of the Tech under research, drawn as one bar in Faction
/// colours. The unfilled part of the bar is what the Tech still needs.
/// Ticket #194 (version 0.08.0): returns true when it was clicked, which OPENS the Tech Tree. One
/// wrinkle, recorded rather than solved: when no Tech is under research the bar is not drawn at all,
/// so in that state only the Research figure is there to click -- which is also the state in which
/// the red `Pick a Tech` button is on the bar doing the same job, so nothing is lost.
fn research_race_bar(ui: &mut Ui, session: &Session, game: &Game, width: f32, in_window: bool) -> bool {
    let Some(tech) = game.research.current else { return false };
    let cost = game.tables.tech(tech).cost.max(1);
    let c = game.research.contributions;
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, 14.0), egui::Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 3.0, Color32::from_gray(45));
    let mut x = rect.min.x;
    for seat in Seat::ALL {
        let w = rect.width() * (c[seat.index()].max(0) as f32) / cost as f32;
        if w <= 0.0 {
            continue;
        }
        let seg = egui::Rect::from_min_size(egui::pos2(x, rect.min.y), egui::vec2(w.min(rect.max.x - x), rect.height()));
        painter.rect_filled(seg, 0.0, seat_colour(session, seat));
        x += w;
    }
    // Ticket #211 (version 0.08.1): the Research NOBODY produced, in grey, so the bar's total
    // always equals the progress figure printed beside it. When a Tech is picked, everything
    // banked since the last one pours in: each seat's own share lands under its name, but the
    // UNATTRIBUTED pool -- the spill past the previous Tech's cost, which ticket #105 credits to
    // nobody on purpose -- goes into progress alone. So the bar had always under-drawn by exactly
    // that amount, and a Tech opening with spill carried into it opened showing 10 of 45 beside an
    // empty bar. Found by looking at a picture; the suite had nothing to say about it. Grey is
    // this game's colour for nobody's, which is what an unheld Region wears.
    let loose = (game.research.progress - c.iter().sum::<i64>()).max(0);
    if loose > 0 {
        let w = rect.width() * (loose as f32) / cost as f32;
        let seg = egui::Rect::from_min_size(egui::pos2(x, rect.min.y), egui::vec2(w.min(rect.max.x - x), rect.height()));
        painter.rect_filled(seg, 0.0, Color32::from_gray(110));
    }
    painter.rect_stroke(rect, 3.0, egui::Stroke::new(1.0, Color32::from_gray(120)), egui::StrokeKind::Inside);
    let shares: Vec<String> = Seat::ALL.into_iter().map(|s| format!("{} {}", game.seat_name(s), c[s.index()])).collect();
    // Ticket #211 (version 0.08.1): in the Tech Tree window the bar stands where the window's first
    // SENTENCE stood, and that sentence moves onto this hover -- the designer: "Put the research bar
    // showing each factions contribution On the tech tree in place of text at top of window (make
    // the text a mouse over) used in both places". So the hover carries the Research Lead there,
    // and on the top bar it keeps the line saying what a click does. One function, two callers,
    // which is what "used in both places" asks for.
    let tail = if in_window { format!("
{}", game.research_lead_text()) } else { "
Click to open the Tech Tree.".to_string() };
    // Ticket #219 (version 0.08.2): the hover was to "trim to what the face does not say" now that
    // each Faction's name and percentage stand under the bar. Nothing needed trimming. What this
    // hover carries is the RAW COUNTS -- `Custodians 10, Prospectors 7` -- and a count is precisely
    // what a percentage cannot tell you: whether 42% is 10 Research or 100. It also carries the
    // Tech's name, the progress and the Research Lead, none of which the face says either. So the
    // line stands as ticket #211 left it, and the trim is a no-op recorded rather than a change
    // made. The top bar's copy is untouched for the same reason and one more: at 150 pixels it has
    // no room for names, so its hover is the only place they appear at all.
    let resp = ui.interact(rect, ui.id().with("race"), egui::Sense::click()).on_hover_text(format!(
        "The Research race for {}: {}{}. {} of {}.{tail}",
        game.tables.tech(tech).name,
        shares.join(", "),
        if loose > 0 { format!(", carried over {loose}") } else { String::new() },
        game.research.progress,
        cost
    ));
    // In the window a click opens nothing -- the window IS the Tech Tree -- so it wears no pointer.
    if in_window {
        false
    } else {
        resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked()
    }
}

// ------------------------------------------------------------------ overlays and picking

#[allow(clippy::too_many_arguments)]
fn overlays(painter: &egui::Painter, session: &Session, game: &Game, view: &ViewState, camera: &Camera, cam_gt: &GlobalTransform, globes: &Query<(&Globe, &GlobalTransform)>, hotspots: &mut Vec<Hotspot>) {
    let project = |p: Vec3| -> Option<Pos2> { camera.world_to_viewport(cam_gt, p).ok().map(|v| Pos2::new(v.x, v.y)) };
    let cam_pos = cam_gt.translation();
    match view.view {
        View::Solar => {
            for body in BodyId::ALL {
                let pos = geo::solar_place(game, body);
                // Ticket #155 (version 0.07.4): a label by Body -- Venus's and the satellites' hang
                // BELOW their discs where a planet's stands above -- so the words of the two inner
                // planets never meet at a conjunction, and a moon's never lie over its planet's.
                let below = matches!(body, BodyId::Venus | BodyId::Moon | BodyId::Phobos | BodyId::Deimos);
                let side = if below { -1.0 } else { 1.0 };
                let head = project(pos + Vec3::Y * side * (geo::solar_radius(body) + 0.05));
                if let Some(p) = head {
                    let name = game.tables.body(body).name.clone();
                    let slots = game.tables.body(body).colony_slots();
                    let filled = game.colonies.iter().filter(|c| c.body == body && !c.in_orbit).count();
                    let orbital = game.tables.body(body).orbital_slots;
                    let stations = game.colonies.iter().filter(|c| c.body == body && c.in_orbit).count();
                    let hovering = view.force_hover == Some(body)
                        || painter.ctx().pointer_latest_pos().map(|q| (q - p).length() < 40.0).unwrap_or(false);
                    let mut text = format!("{name}  {filled}/{slots} slots, {stations}/{orbital} stations");
                    // Ticket #136 (version 0.07.3): every Orbital Slot by name and holder. The
                    // designer: *"List orbital slots in the body card."* The list unfolds while the
                    // Body is under the pointer: always open, Earth's six lines lay over the Moon
                    // and Mars's over its moons in the first picture, since the three stand a few
                    // pixels apart on this map.
                    let mut lines = 0;
                    if hovering {
                        for slot in 0..orbital {
                            let holder = match game.colonies.iter().find(|c| c.in_orbit && c.body == body && c.slot == slot) {
                                Some(c) => c.control.director().map(|s| game.seat_name(s)).unwrap_or_else(|| "nobody's".to_string()),
                                None => "free".to_string(),
                            };
                            let blockade = warship_in_slot(game, body, slot).map(|s| format!(", a {} warship in it", game.seat_name(s.seat))).unwrap_or_default();
                            text.push_str(&format!("\n{}: {holder}{blockade}", game.station_name(body, slot)));
                            lines += 1;
                        }
                    }
                    // Ticket #56: Earth says when its Antarctic slots open, until they do.
                    if body == BodyId::Earth && !game.antarctica_open {
                        text.push_str(&format!("\nAntarctica: opens at {:+.1} C", game.tables.climate.antarctica_opens_at));
                    }
                    label_at(painter, p - egui::vec2(0.0, side * (22.0 + 7.5 * lines as f32)), &text, Color32::WHITE, 13.0);
                    // Ticket #136: one orbit per Body, the stations on it at spaced positions, dashed
                    // while nothing is in orbit. Five rings will not fit round an eighteen-pixel Earth
                    // without swallowing the Moon, so on this map the slots share one ring; each has
                    // its own on the Surface Map.
                    if orbital > 0 {
                        let r = geo::solar_radius(body) * 1.9;
                        let at = |a: f32| pos + Vec3::new(a.cos() * r, 0.0, a.sin() * r);
                        let samples = 64;
                        let points: Vec<Option<Pos2>> = (0..samples).map(|i| project(at(i as f32 / samples as f32 * std::f32::consts::TAU))).collect();
                        orbit_polyline(painter, &points, stations == 0, Color32::from_gray(140));
                        for slot in 0..orbital {
                            let Some(q) = project(at(slot as f32 / orbital as f32 * std::f32::consts::TAU + 0.3)) else { continue };
                            if let Some(c) = game.colonies.iter().find(|c| c.in_orbit && c.body == body && c.slot == slot) {
                                let colour = c.control.director().map(|s| seat_colour(session, s)).unwrap_or(Color32::LIGHT_GRAY);
                                glyph_at(painter, Kind::Station, q, 14.0, colour);
                            }
                            if let Some(s) = warship_in_slot(game, body, slot) {
                                glyph_at(painter, Kind::Warship, q + egui::vec2(12.0, 0.0), 12.0, seat_colour(session, s.seat));
                            }
                        }
                    }
                    hotspots.push(Hotspot { pos: p, radius: 40.0, hit: Hit::Enter(body) });
                    // Ticket #317 (version 0.08.8): a Battle in orbit last turn, marked beside the
                    // Body's label, to its left, at the label's own height: below the disc the
                    // moons' labels hang, and above it the Orbital Control and stack labels stack.
                    if let Some(i) = game.battle_last_turn_at(ReportPlace::Body(body)) {
                        let label_centre = p - egui::vec2(0.0, side * (22.0 + 7.5 * lines as f32));
                        let width = painter.layout_no_wrap(text.clone(), FontId::proportional(13.0), Color32::WHITE).size().x;
                        let at = label_centre - egui::vec2(width / 2.0 + 18.0, 0.0);
                        battle_mark(painter, at, battle_colour(session, game, i));
                        hotspots.push(Hotspot { pos: at, radius: 12.0, hit: Hit::Battle(i) });
                    }
                    // Ticket #57: hovering a Body across the gulf says when its launch window is and
                    // what the flight costs now against what it costs then. TO BE REVISITED: these
                    // are transits FROM EARTH. Once a Faction can launch from the Moon, or home from
                    // Mars, one line for one departure point will no longer be the whole truth, and
                    // how this is presented has to be settled again.
                    let far = game.crossing_offset(BodyId::Earth, body, game.turn).is_some();
                    if far && hovering {
                        label_on_screen(painter, p + egui::vec2(0.0, 96.0), &game.window_text(body), Color32::from_rgb(255, 220, 140), 13.0);
                    }
                    // The Orbital Control flag in the holder's Faction colour.
                    if let Some(s) = game.orbital_control(body) {
                        label_at(painter, p - egui::vec2(0.0, side * 40.0), &format!("Orbital Control: {}", game.seat_name(s)), seat_colour(session, s), 12.0);
                    }
                }
                // Ticket #50: up to four stacks at one Body. The markers sit at four fixed angles
                // round it (`geo::stack_offset`); their labels stack above it in seat order, since
                // four labels at four angles on a small Body would cover each other.
                let mut row = 0.0;
                for seat in Seat::ALL {
                    let ships = game.ships_at(seat, body);
                    if ships.is_empty() {
                        continue;
                    }
                    let Some(p) = head else { continue };
                    let text = format!("{} x{}  str {}", game.seat_name(seat), ships.len(), game.ship_stack_strength(seat, body));
                    let at = p - egui::vec2(0.0, 58.0 + row * 16.0);
                    label_kind_at(painter, at, Some(Kind::of_ships(ships.iter().filter_map(|id| game.ship(*id)))), &text, seat_colour(session, seat), 12.0);
                    hotspots.push(Hotspot { pos: at, radius: 14.0, hit: Hit::Select(Selection::ShipStack(body, seat)) });
                    row += 1.0;
                }
            }
            for s in &game.ships {
                if let ShipAt::Transit { from, to, turns_left } = s.at {
                    let a = geo::solar_place(game, from);
                    let b = geo::solar_place(game, to);
                    if let Some(p) = project(a.lerp(b, 0.5) + Vec3::Y * 0.2) {
                        // Ticket #216 (version 0.08.2): a Ship in transit is named. The designer's
                        // "the name of the ship should appear ... and the map" meant THIS label and
                        // not the per-Faction block at a Body, which is unchanged. The Faction's
                        // NAME goes: the prefix already says whose (TSV, PMV, ARK, ACV) and the
                        // label is drawn in the Faction's colour, so it was saying it three times.
                        // The type stays in words at the designer's word, beside the kind glyph.
                        let text = format!("{} ({}): {} turn(s)", game.ship_name(s), s.kind.name().to_lowercase(), turns_left);
                        label_kind_at(painter, p, Some(Kind::of_unit(s.kind)), &text, seat_colour(session, s.seat), 12.0);
                    }
                }
            }
        }
        View::Surface(body) => {
            let Some((_, globe_gt)) = globes.iter().find(|(g, _)| g.0 == body) else { return };
            let center = globe_gt.translation();
            let visible = |local: Vec3| -> Option<Pos2> {
                let world = globe_gt.transform_point(local);
                if (world - center).dot(cam_pos - center) < 0.25 * GLOBE_RADIUS * (cam_pos - center).length() {
                    return None;
                }
                project(world)
            };
            // Ticket #136 (version 0.07.3): one ring per Orbital Slot round the globe.
            orbit_rings_on_globe(painter, session, game, body, globe_gt, cam_pos, *cam_gt.right(), &project, hotspots);
            match body {
                BodyId::Earth => {
                    for sid in StateId::ALL {
                        let (lon, lat) = geo::state_lonlat(sid);
                        let Some(p) = visible(geo::local_from_lonlat(lon, lat) * 1.01) else { continue };
                        let st = game.state(sid);
                        let armies: i64 = Seat::ALL.iter().map(|s| game.army_stack_strength(*s, Place::State(sid))).sum::<i64>()
                            + game.armies.iter().filter(|a| a.at == ArmyAt::Place(Place::State(sid)) && game.army_seat(a).is_none()).map(|a| game.army_strength(a)).sum::<i64>();
                        let owner = match st.control {
                            Control::Neutral => "neutral".to_string(),
                            Control::Controlled(s) => game.seat_name(s),
                            Control::Occupied { occupier, .. } => format!("occupied by {}", game.seat_name(occupier)),
                        };
                        let colour = match st.control.director() {
                            Some(s) => seat_colour(session, s),
                            None => Color32::LIGHT_GRAY,
                        };
                        let _ = armies;
                        let text = format!("{}\n{}\n{} Facilities, {} free slot(s)", game.tables.state(sid).name, owner, st.facilities.len(), game.free_slots(sid));
                        label_kind_at(painter, p, Some(Kind::Region), &text, colour, 12.0);
                        // Ticket #52: the Unrest figure once it bites, red once Facilities run at half.
                        let army_line = game.tables.unrest.army_threshold;
                        if st.unrest >= army_line {
                            let hot = st.unrest >= game.tables.unrest.facility_threshold;
                            let tint = if hot { Color32::from_rgb(255, 90, 80) } else { Color32::from_rgb(255, 190, 90) };
                            label_at(painter, p - egui::vec2(0.0, 38.0), &format!("Unrest {}", game.unrest_text(sid)), tint, 13.0);
                        }
                        hotspots.push(Hotspot { pos: p, radius: 30.0, hit: Hit::Select(Selection::State(sid)) });
                        // Ticket #323 (version 0.08.8): the Regions an armed stack may reach are
                        // outlined, in the roster's ring colour, so a right-click knows its targets.
                        if let Some(from) = view.armed_stack
                            && game.tables.state(from).neighbours.contains(&sid)
                        {
                            painter.circle_stroke(p, 30.0, egui::Stroke::new(2.0, RING_WANTS));
                        }
                        // Ticket #311 (version 0.08.7): last turn's Battle here, for the one Orders
                        // phase the Report lives; its hover reads the record and a click opens the
                        // Report. Ticket #317 (version 0.08.8): the ring became the Battle mark,
                        // above the label in the Unrest label's row, and a row higher when that
                        // label is showing.
                        if let Some(i) = game.battle_last_turn_at(ReportPlace::State(sid)) {
                            let lift = if st.unrest >= game.tables.unrest.army_threshold { 56.0 } else { 38.0 };
                            let at = p - egui::vec2(0.0, lift);
                            battle_mark(painter, at, battle_colour(session, game, i));
                            hotspots.push(Hotspot { pos: at, radius: 12.0, hit: Hit::Battle(i) });
                        }
                        // Army shields (ticket #31): one per Faction present, grey for a neutral Standing Army.
                        // Ticket #297 (version 0.08.6): a shield whose Army is dug in carries a
                        // trench line beneath it, in its own colour.
                        let mut shields: Vec<(Option<Seat>, i64, bool, bool)> = Vec::new();
                        for seat in Seat::ALL {
                            let s = game.army_stack_strength(seat, Place::State(sid));
                            let ids = game.armies_of_seat_at(seat, Place::State(sid));
                            if s > 0 || !ids.is_empty() {
                                let dug = ids.iter().filter_map(|id| game.army(*id)).any(|a| game.army_dug_in(a));
                                let hurt = ids.iter().filter_map(|id| game.army(*id)).any(|a| a.damage > 0);
                                shields.push((Some(seat), s, dug, hurt));
                            }
                        }
                        let neutral_armies: Vec<&Army> = game.armies.iter().filter(|a| a.at == ArmyAt::Place(Place::State(sid)) && game.army_seat(a).is_none() && !game.army_stands_down(a)).collect();
                        let neutral: i64 = neutral_armies.iter().map(|a| game.army_strength(a)).sum();
                        if neutral > 0 {
                            shields.push((None, neutral, neutral_armies.iter().any(|a| game.army_dug_in(a)), neutral_armies.iter().any(|a| a.damage > 0)));
                        }
                        for (i, (seat, strength, dug, hurt)) in shields.iter().enumerate() {
                            let centre = p + egui::vec2(-38.0 + 26.0 * i as f32, 36.0);
                            let fill = seat.map(|s| seat_colour(session, s)).unwrap_or(Color32::from_gray(150));
                            shield(painter, centre, fill, &strength.to_string(), *hurt);
                            if *dug {
                                painter.line_segment([centre + egui::vec2(-10.0, 15.0), centre + egui::vec2(10.0, 15.0)], egui::Stroke::new(3.0, fill));
                            }
                            // Ticket #323 (version 0.08.8): the player's own shield arms its stack
                            // on a click and wears a ring while it is armed.
                            let own = *seat == Some(Seat(0));
                            if own && view.armed_stack == Some(sid) {
                                painter.circle_stroke(centre, 16.0, egui::Stroke::new(2.0, RING_WANTS));
                            }
                            hotspots.push(Hotspot { pos: centre, radius: 12.0, hit: if own { Hit::Shield(sid) } else { Hit::Select(Selection::State(sid)) } });
                        }
                    }
                    // Ticket #44: Antarctica's Colony Slots.
                    slot_labels(painter, session, game, body, &visible, hotspots);
                }
                _ => {
                    slot_labels(painter, session, game, body, &visible, hotspots);
                    // The band along the top: Ship stacks in orbit and Orbital Control. Ticket #50:
                    // four seats will not fit on one line, so each takes its own in its own colour.
                    // Ticket #216 (version 0.08.2): a line that belongs to a Faction's stack carries
                    // its seat, so the block can grow hotspots and a hover naming the hulls. A
                    // station's line and the Orbital Control line belong to nobody and carry None.
                    let mut band: Vec<(String, Color32, Option<Kind>, Option<Seat>)> = Vec::new();
                    for seat in Seat::ALL {
                        let ids = game.ships_at(seat, body);
                        if !ids.is_empty() {
                            let kind = Kind::of_ships(ids.iter().filter_map(|id| game.ship(*id)));
                            band.push((format!("{}: {} Ship(s), strength {}", game.seat_name(seat), ids.len(), game.ship_stack_strength(seat, body)), seat_colour(session, seat), Some(kind), Some(seat)));
                        }
                    }
                    for c in game.colonies.iter().filter(|c| c.in_orbit && c.body == body) {
                        let who = c.control.director();
                        let name = who.map(|s| game.seat_name(s)).unwrap_or_else(|| "nobody's".into());
                        band.push((format!("{} ({})", game.station_name(body, c.slot), name), who.map(|s| seat_colour(session, s)).unwrap_or(Color32::LIGHT_GRAY), Some(Kind::Station), None));
                    }
                    // Ticket #324 (version 0.08.8): a seat's working Batteries at the Body are a row
                    // of the band, and the Control line says when they are why nobody holds it.
                    for seat in Seat::ALL {
                        let n = game.batteries_at(seat, body).len();
                        if n > 0 {
                            band.push((format!("{}: {} Batter{}, strength {}", game.seat_name(seat), n, if n == 1 { "y" } else { "ies" }, game.battery_strength(seat, body)), seat_colour(session, seat), None, None));
                        }
                    }
                    let any_battery = Seat::ALL.iter().any(|s| !game.batteries_at(*s, body).is_empty());
                    band.push(match game.orbital_control(body) {
                        Some(s) => (format!("Orbital Control: {}", game.seat_name(s)), seat_colour(session, s), None, None),
                        None if any_battery => ("Orbital Control: nobody, a Battery stands".to_string(), Color32::LIGHT_GRAY, None, None),
                        None => ("Orbital Control: nobody".to_string(), Color32::LIGHT_GRAY, None, None),
                    });
                    // Ticket #317 (version 0.08.8): a Battle in orbit last turn is a row of the
                    // band, with the Battle mark's glyph, in the aggressor's colour; its hotspot
                    // reads the record and opens the Report as the mark's does.
                    let fought = game.battle_last_turn_at(ReportPlace::Body(body));
                    if let Some(i) = fought {
                        let who = game.report.battles[i].aggressor().map(|s| format!("the {} attacked", game.seat_name(s))).unwrap_or_else(|| "nobody attacked".to_string());
                        band.push((format!("A Battle here last turn: {who}"), battle_colour(session, game, i), Some(Kind::Battle), None));
                    }
                    let rect = painter.clip_rect();
                    let x = rect.center().x - 120.0;
                    label_at(painter, Pos2::new(x, rect.min.y + 50.0), &format!("In orbit around {}", game.tables.body(body).name), Color32::WHITE, 13.0);
                    for (i, (text, colour, kind, seat)) in band.iter().enumerate() {
                        let at = Pos2::new(x, rect.min.y + 70.0 + 18.0 * i as f32);
                        label_kind_at(painter, at, *kind, text, *colour, 12.0);
                        if *kind == Some(Kind::Battle) && let Some(b) = fought {
                            hotspots.push(Hotspot { pos: at, radius: 14.0, hit: Hit::Battle(b) });
                        }
                        // Ticket #216: a Faction's line gets a hit target, so hovering it can name
                        // that Faction's hulls. The Solar System Map's block already had one for the
                        // click; this block had none at all, being painted at fixed positions.
                        if let Some(s) = seat {
                            hotspots.push(Hotspot { pos: at, radius: 14.0, hit: Hit::Select(Selection::ShipStack(body, *s)) });
                        }
                    }
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
/// Colony Slot labels on a Body's surface: Antarctica's on Earth since ticket #44.
/// Ticket #113 (version 0.07.1): a Colony Slot's four yields, each figure behind the glyph of what
/// that Module makes. It replaces `M 1.22 G 0.80 R 1.86 H 1.63` -- four bare capitals standing for
/// Mine, Generator, Refinery and Habitat, which a player was never taught and could not look up on
/// the map. The mapping is exactly one to one, so no new art was needed: a Mine makes Materials, a
/// Generator Energy, a Refinery Fuel, and a Habitat people.
///
/// **Drawn at 14 points, not the 11 the rest of the label uses.** Rendered at 11 the bolt and the
/// bust read but the mine cart is a blob and the jerrycan is mud; the fault is the size, not the
/// glyph, and the size was ours to change.
///
/// The cost, measured rather than guessed: the line is about **forty per cent wider** than the
/// capitals it replaces, because a 14-pixel glyph is wider than an 11-point capital. Charting
/// expected the opposite and charting was wrong. No label on any Body overlaps another at that
/// width, so it is paid and not a problem; dropping the figures to one decimal would buy most of it
/// back if a later Body ever crowds.
///
/// Where a glyph has not loaded the whole line falls back to the capitals. A map label has no
/// tooltip behind it -- it is the whole of what is shown -- so it must never be able to go mute.
const SLOT_YIELD_SIZE: f32 = 14.0;

/// Ticket #258 (version 0.08.4): four yields as a row of glyph-and-figure pairs in a Ui -- the
/// notation `slot_yield_label` paints under every slot on the map, brought onto the panel and the
/// founding button. It replaces `slot_yield_hover` (ticket #211), a line of words in the form
/// "Materials x1.37". The line of WORDS that stood here before this ticket never became glyphs at
/// all: the one glyph rule (`draw_with_icons`) trades a word only where it follows a figure, and
/// "Materials x1.37" has the figure after the word. Ticket #218 promised glyphs on the button's
/// face and a picture on this ticket was the first to show it had none. Drawn directly, glyph then
/// figure, so it cannot fall through that rule again.
fn slot_yield_row(ui: &mut Ui, figures: [(&str, f64); 4], size: f32, tint: Color32) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 3.0;
        for (i, (key, v)) in figures.iter().enumerate() {
            if i > 0 {
                ui.add_space(6.0);
            }
            match Icons::from_ctx(ui.ctx(), key, size) {
                Some(image) => {
                    ui.add(image);
                }
                None => {
                    ui.label(RichText::new(*key).size(size).color(tint));
                }
            }
            ui.label(RichText::new(format!("x{v:.2}")).size(size).color(tint));
        }
    });
}

fn slot_yield_figures(y: &dying_earth_engine::SlotYields) -> [(&'static str, f64); 4] {
    [("materials", y.mine), ("energy", y.generator), ("fuel", y.refinery), ("research", y.research)]
}

fn slot_yield_label(painter: &egui::Painter, pos: Pos2, yields: &dying_earth_engine::SlotYields) {
    let figures = [("materials", yields.mine), ("energy", yields.generator), ("fuel", yields.refinery), ("research", yields.research)];
    let Some(glyphs) = figures.iter().map(|(key, _)| Icons::texture_from_ctx(painter.ctx(), key)).collect::<Option<Vec<_>>>() else {
        label_at(painter, pos, &yields.text(), Color32::from_gray(170), 11.0);
        return;
    };
    let font = FontId::proportional(SLOT_YIELD_SIZE);
    let ink = Color32::from_gray(205);
    let galleys: Vec<_> = figures.iter().map(|(_, v)| painter.layout_no_wrap(format!("{v:.2}"), font.clone(), ink)).collect();
    let gap = 3.0;
    let between = 9.0;
    let width: f32 = galleys.iter().map(|g| SLOT_YIELD_SIZE + gap + g.size().x + between).sum::<f32>() - between;
    let height = SLOT_YIELD_SIZE.max(galleys.iter().map(|g| g.size().y).fold(0.0, f32::max));
    let rect = egui::Rect::from_center_size(pos, egui::vec2(width + 8.0, height + 4.0));
    painter.rect_filled(rect, 3.0, Color32::from_black_alpha(170));
    let uv = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));
    let mut x = rect.min.x + 4.0;
    for ((key, _), (glyph, galley)) in figures.iter().zip(glyphs.into_iter().zip(galleys)) {
        let square = egui::Rect::from_min_size(egui::pos2(x, pos.y - SLOT_YIELD_SIZE / 2.0), egui::vec2(SLOT_YIELD_SIZE, SLOT_YIELD_SIZE));
        painter.image(glyph, square, uv, crate::icons::fill(key));
        x += SLOT_YIELD_SIZE + gap;
        let w = galley.size().x;
        painter.galley(egui::pos2(x, pos.y - galley.size().y / 2.0), galley, ink);
        x += w + between;
    }
}

fn slot_labels(painter: &egui::Painter, session: &Session, game: &Game, body: BodyId, visible: &dyn Fn(Vec3) -> Option<Pos2>, hotspots: &mut Vec<Hotspot>) {
    // Ticket #103 (version 0.07.0): Earth's three slots are Antarctica's, and nothing of them is
    // drawn until the ice opens -- no marker, no name, no yields. A player is told the ice EXISTS
    // and at what warmth it goes, on the Solar System Map and the Climate Panel, so an Antarctic
    // Colony can still be planned for; what is hidden is which of the three is the best site.
    if body == BodyId::Earth && !game.antarctica_open {
        return;
    }
            for slot in 0..game.tables.body(body).colony_slots() {
                let (lon, lat) = geo::slot_lonlat(game.tables.body(body), slot);
        let name = &game.tables.body(body).slots[slot as usize].name;
                let Some(p) = visible(geo::local_from_lonlat(lon, lat) * 1.03) else { continue };
                let (text, colour, hit, kind) = match game.colony_at(body, slot) {
                    Some(c) => {
                        let mods: Vec<String> = c.modules.iter().map(|m| format!("{}{}", m.kind.name(), if m.online { "" } else { " (offline)" })).collect();
                        let army = game.armies.iter().filter(|a| a.at == ArmyAt::Place(Place::Colony(c.id))).count();
                        let owner = c.control.director().map(|s| game.seat_name(s)).unwrap_or_default();
                        (
                            format!("{}: {}\n{} Colonists\n{}{}", name, owner, c.colonists, mods.join(", "), if army > 0 { format!("\nArmies: {army}") } else { String::new() }),
                            c.control.director().map(|s| seat_colour(session, s)).unwrap_or(Color32::LIGHT_GRAY),
                            Hit::Select(Selection::Colony(c.id)),
                            Some(Kind::of_colony(c)),
                        )
                    }
                    None => (format!("{name}: empty"), Color32::LIGHT_GRAY, Hit::Select(Selection::Slot(body, slot)), None),
                };
                label_kind_at(painter, p + egui::vec2(0.0, 24.0), kind, &text, colour, 12.0);
                // Ticket #311 (version 0.08.7): last turn's Battle at this Colony, as a Region's.
                // Ticket #317 (version 0.08.8): the Battle mark above the slot's point.
                if let Some(c) = game.colony_at(body, slot)
                    && let Some(i) = game.battle_last_turn_at(ReportPlace::Colony(c.id))
                {
                    let at = p - egui::vec2(0.0, 18.0);
                    battle_mark(painter, at, battle_colour(session, game, i));
                    hotspots.push(Hotspot { pos: at, radius: 12.0, hit: Hit::Battle(i) });
                }
                // Ticket #57: every slot carries its own four yields under its name, filled or free;
                // a free slot's figures are what a Colony founded there would get. TO BE REVISITED
                // WHEN BOARD LENSES ARRIVE: this is on the map always, and once the player can turn
                // a yield lens on and off it should live there instead of over every label at once.
                // `label_at` centres its block on the point, so the figures clear half the name
                // block above them and half their own line.
                let lines = text.lines().count() as f32;
                slot_yield_label(painter, p + egui::vec2(0.0, 24.0 + lines * 7.0 + 10.0), &game.slot_yields(body, slot));
                hotspots.push(Hotspot { pos: p, radius: 22.0, hit });
            }
}

/// Ticket #46: the stations over the Body on screen, and the orbital slots still free.
/// Ticket #283 (version 0.08.5): the planet card's Colonies block. The Body's own four figures
/// first, weak, then one row per Colony on the ground and per open site, in slot order, with the
/// slot's glyph row beneath as the map label draws it; clicking a row selects it. Every row goes
/// through `slot_yield_row`, since the one glyph rule only swaps a word that follows a figure and
/// a line written "Materials x1.37" would silently come out in words (ticket #258's lesson).
fn colonies_block(ui: &mut Ui, game: &Game, view: &mut ViewState, body: BodyId) {
    let card = game.tables.body(body);
    let (ink, weak) = (ui.visuals().text_color(), ui.visuals().weak_text_color());
    ui.horizontal(|ui| {
        ui.label(RichText::new(format!("{} as a whole:", card.name)).weak());
        slot_yield_row(ui, [("materials", card.mine_yield), ("energy", card.generator_yield), ("fuel", card.refinery_yield), ("research", card.research_yield)], 14.0, weak);
    });
    let rows: Vec<u32> = (0..card.colony_slots()).filter(|s| body != BodyId::Earth || game.colony_at(body, *s).is_some()).collect();
    if rows.is_empty() {
        return;
    }
    ui.label(RichText::new("Colonies and sites").strong()).on_hover_text(
        "Every Colony on the ground and every site still open, each with its own four yields: what a Mine, a Generator, a Refinery and an Observatory make there, against the figure for the whole Body above. A station reads the Body's figures, so its row below carries none.",
    );
    for slot in rows {
        let (text, select) = match game.colony_at(body, slot) {
            Some(c) => {
                let owner = match c.control {
                    Control::Neutral => "nobody's".to_string(),
                    Control::Controlled(s) => game.seat_name(s),
                    Control::Occupied { occupier, .. } => format!("occupied by the {}", game.seat_name(occupier)),
                };
                (format!("{}: {}, {} Colonists", game.place_name(Place::Colony(c.id)), owner, c.colonists), Selection::Colony(c.id))
            }
            None => (format!("{}: empty", card.slots[slot as usize].name), Selection::Slot(body, slot)),
        };
        if ui.button(text).clicked() {
            view.selection = select;
        }
        ui.horizontal(|ui| {
            ui.add_space(12.0);
            slot_yield_row(ui, slot_yield_figures(&game.slot_yields(body, slot)), 13.0, ink);
        });
    }
    ui.add_space(4.0);
}

fn stations_panel(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, actions: &mut Vec<Action>) {
    let View::Surface(body) = view.view else { return };
    let card = game.tables.body(body);
    if card.orbital_slots == 0 {
        return;
    }
    ui.separator();
    let count = game.colonies.iter().filter(|c| c.in_orbit && c.body == body).count();
    // Ticket #116 (version 0.07.1): what an orbital slot is for, and what a Warship in one does.
    rule_tip(
        ui.label(RichText::new(format!("In orbit: {} of {} station slots", count, card.orbital_slots)).strong()),
        format!(
            "{} has room for {} stations in orbit, and a station taken is a station gone: nobody else builds there.
A station is where Ships refuel and where a Shipyard can stand, and a Warship holding a slot blockades that slot alone, not the whole world.",
            card.name, card.orbital_slots
        ),
    );
    for c in game.colonies.iter().filter(|c| c.in_orbit && c.body == body) {
        let owner = match c.control {
            Control::Neutral => "nobody's".to_string(),
            Control::Controlled(s) => game.seat_name(s),
            Control::Occupied { occupier, .. } => format!("occupied by the {}", game.seat_name(occupier)),
        };
        // Ticket #165 (version 0.07.5): the Core Module is left out of the list. Every Colony and
        // every station has one, so naming it says nothing about this one; and a station that holds
        // only its Core Module is exactly what the words below have always called a bare core module.
        let mods: Vec<&str> = c.modules.iter().filter(|m| m.kind != ModuleKind::Core).map(|m| m.kind.name()).collect();
        // Ticket #278 (version 0.08.5): a station under a Blockade says so, and by whom.
        let starved = match game.starved_by(c.id) {
            Some(by) => format!("; blockaded by the {}: producing nothing", game.seat_name(by)),
            None => String::new(),
        };
        let text = format!("{}: {}, {} Colonists, {}{starved}", game.station_name(body, c.slot), owner, c.colonists, if mods.is_empty() { "a bare core module".to_string() } else { mods.join(", ") });
        if ui.button(text).clicked() {
            view.selection = Selection::Colony(c.id);
        }
    }
    if !session.spectator {
        for slot in game.free_orbital_slots(body) {
            cost_button(ui, game, &session.pending, Order::BuildStation { body, slot }, &format!("Build {} here", game.station_name(body, slot)), actions);
        }
    }
    ui.label(RichText::new("A station holds a Shipyard, Habitats, Observatories, Solar Arrays and a Trade Post. Ships are built only at a Shipyard.").weak());
}

/// The Colony Slot within fourteen degrees of a point on a Body, nearest first.
fn nearest_slot(game: &Game, body: BodyId, lon: f32, lat: f32) -> Option<u32> {
    let mut best: Option<(f32, u32)> = None;
    for slot in 0..game.tables.body(body).colony_slots() {
        let (slon, slat) = geo::slot_lonlat(game.tables.body(body), slot);
        let d = geo::local_from_lonlat(slon, slat).angle_between(geo::local_from_lonlat(lon, lat)).to_degrees();
        if d < 14.0 && best.map(|(bd, _)| d < bd).unwrap_or(true) {
            best = Some((d, slot));
        }
    }
    best.map(|(_, slot)| slot)
}

#[allow(clippy::too_many_arguments)]
fn pick(pos: Pos2, session: &Session, game: &Game, view: &mut ViewState, camera: &Camera, cam_gt: &GlobalTransform, globes: &Query<(&Globe, &GlobalTransform)>, textures: &Textures, hotspots: &[Hotspot]) {
    let _ = session;
    // Ticket #323 (version 0.08.8): any left-click disarms the stack; a shield's click re-arms it.
    view.armed_stack = None;
    // Labels and markers first.
    let mut best: Option<(f32, Hit)> = None;
    for h in hotspots {
        let d = h.pos.distance(pos);
        if d <= h.radius && best.map(|(bd, _)| d < bd).unwrap_or(true) {
            best = Some((d, h.hit));
        }
    }
    if let Some((_, hit)) = best {
        apply_hit(hit, view);
        return;
    }
    let Ok(ray) = camera.viewport_to_world(cam_gt, Vec2::new(pos.x, pos.y)) else { return };
    let (origin, dir) = (ray.origin, Vec3::from(ray.direction));
    match view.view {
        View::Solar => {
            let mut nearest: Option<(f32, BodyId)> = None;
            for body in BodyId::ALL {
                let hit = geo::ray_sphere(origin, dir, geo::solar_place(game, body), geo::solar_radius(body) * 1.5);
                if let Some(t) = hit.filter(|t| nearest.map(|(n, _)| *t < n).unwrap_or(true)) {
                    nearest = Some((t, body));
                }
            }
            if let Some((_, body)) = nearest {
                view.enter_surface(body);
            }
        }
        View::Surface(body) => {
            let Some((_, globe_gt)) = globes.iter().find(|(g, _)| g.0 == body) else { return };
            let center = globe_gt.translation();
            let Some(t) = geo::ray_sphere(origin, dir, center, GLOBE_RADIUS) else {
                view.selection = Selection::None;
                return;
            };
            let world = origin + dir * t;
            let local = globe_gt.affine().inverse().transform_point3(world);
            let (lon, lat) = geo::lonlat_from_local(local);
            match body {
                BodyId::Earth => {
                    // Ticket #44: a Colony Slot in Antarctica first, else the Region under the click.
                    view.selection = match nearest_slot(game, body, lon, lat) {
                        Some(slot) => match game.colony_at(body, slot) {
                            Some(c) => Selection::Colony(c.id),
                            None => Selection::Slot(body, slot),
                        },
                        None => {
                            let (x, y) = geo::pixel_for(lon, lat, textures.earth.w, textures.earth.h);
                            textures.state_at(x, y).map(Selection::State).unwrap_or(Selection::None)
                        }
                    };
                }
                _ => {
                    view.selection = match nearest_slot(game, body, lon, lat) {
                        Some(slot) => match game.colony_at(body, slot) {
                            Some(c) => Selection::Colony(c.id),
                            None => Selection::Slot(body, slot),
                        },
                        None => Selection::None,
                    };
                }
            }
        }
    }
}

fn apply_hit(hit: Hit, view: &mut ViewState) {
    match hit {
        Hit::Select(s) => {
            view.selection = s;
            view.attack_preview = false;
        }
        Hit::Enter(b) => view.enter_surface(b),
        Hit::Battle(_) => view.popup = Popup::Report,
        Hit::Shield(sid) => {
            view.selection = Selection::State(sid);
            view.attack_preview = false;
            view.armed_stack = Some(sid);
            view.armed_scroll = true;
        }
    }
}

/// Ticket #323 (version 0.08.8): **a right-click on the map moves the armed stack.** On Earth, with
/// a Region's stack armed by a click on its shield, a right-click on a neighbouring Region places
/// the stack's march there, the same orders the card's *attack X* button places; a second
/// right-click on the same Region takes them back. On the Solar System Map, with the player's Ship
/// stack selected, a right-click on another Body sends every Ship of it that can pay the leg, as
/// the card's *All that can* does, and a second right-click takes that back. A right-click never
/// selects; on anything else it does nothing, and a Region out of reach says so in a notice.
#[allow(clippy::too_many_arguments)]
fn right_click(pos: Pos2, session: &Session, game: &Game, view: &ViewState, camera: &Camera, cam_gt: &GlobalTransform, globes: &Query<(&Globe, &GlobalTransform)>, textures: &Textures, actions: &mut Vec<Action>) {
    let Ok(ray) = camera.viewport_to_world(cam_gt, Vec2::new(pos.x, pos.y)) else { return };
    let (origin, dir) = (ray.origin, Vec3::from(ray.direction));
    // The orders the target would take, and whether every one of them is already pending.
    let place_or_cancel = |orders: Vec<Order>, actions: &mut Vec<Action>| {
        if orders.is_empty() {
            return;
        }
        let pending: Vec<usize> = orders.iter().filter_map(|o| session.pending.iter().position(|p| p == o)).collect();
        if pending.len() == orders.len() {
            let mut idx = pending;
            idx.sort_unstable_by(|a, b| b.cmp(a));
            for i in idx {
                actions.push(Action::Cancel(i));
            }
        } else {
            for o in orders {
                if game.check_order(Seat(0), &session.pending, &o).is_ok() {
                    actions.push(Action::Place(o));
                }
            }
        }
    };
    match view.view {
        View::Solar => {
            let Selection::ShipStack(from, Seat(0)) = view.selection else { return };
            let mut nearest: Option<(f32, BodyId)> = None;
            for body in BodyId::ALL {
                let hit = geo::ray_sphere(origin, dir, geo::solar_place(game, body), geo::solar_radius(body) * 1.5);
                if let Some(t) = hit.filter(|t| nearest.map(|(n, _)| *t < n).unwrap_or(true)) {
                    nearest = Some((t, body));
                }
            }
            let Some((_, to)) = nearest else { return };
            if to == from {
                return;
            }
            let orders: Vec<Order> = game.ships_at(Seat(0), from).into_iter().map(|id| Order::Transit { ship: id, to, slot: None }).filter(|o| game.check_order(Seat(0), &session.pending, o).is_ok() || session.pending.contains(o)).collect();
            if orders.is_empty() {
                actions.push(Action::Notice(format!("No Ship of the stack can pay the leg to {}.", game.tables.body(to).name)));
            }
            place_or_cancel(orders, actions);
        }
        View::Surface(BodyId::Earth) => {
            let Some(from) = view.armed_stack else { return };
            let Some((_, globe_gt)) = globes.iter().find(|(g, _)| g.0 == BodyId::Earth) else { return };
            let center = globe_gt.translation();
            let Some(t) = geo::ray_sphere(origin, dir, center, GLOBE_RADIUS) else { return };
            let world = origin + dir * t;
            let local = globe_gt.affine().inverse().transform_point3(world);
            let (lon, lat) = geo::lonlat_from_local(local);
            let (x, y) = geo::pixel_for(lon, lat, textures.earth.w, textures.earth.h);
            let Some(to) = textures.state_at(x, y) else { return };
            if !game.tables.state(from).neighbours.contains(&to) {
                actions.push(Action::Notice(format!("{} is not next to {}: the stack can reach only the outlined Regions.", game.tables.state(to).name, game.tables.state(from).name)));
                return;
            }
            let orders: Vec<Order> = game
                .armies_of_seat_at(Seat(0), Place::State(from))
                .into_iter()
                .map(|id| Order::MoveArmy { army: id, to })
                .filter(|o| game.check_order(Seat(0), &session.pending, o).is_ok() || session.pending.contains(o))
                .collect();
            if orders.is_empty() {
                actions.push(Action::Notice("No Army of the stack may march this turn.".to_string()));
            }
            place_or_cancel(orders, actions);
        }
        View::Surface(_) => {}
    }
}

/// Ticket #311 (version 0.08.7): what the Battle ring's hover says, read from the record: who
/// attacked, how long it ran, and each party's line, within the six-line rule.
fn battle_summary(game: &Game, i: usize) -> String {
    let Some(b) = game.report.battles.get(i) else { return String::new() };
    let who = |s: Option<Seat>| s.map(|s| game.seat_name(s)).unwrap_or_else(|| "Neutral".to_string());
    let mut lines = vec![match b.aggressor() {
        Some(s) => format!("A Battle here last turn: the {} attacked; {}", game.seat_name(s), b.result),
        None => format!("A Battle here last turn; {}", b.result),
    }];
    for party in b.parties.iter().take(4) {
        lines.push(format!("{}: {} ({} hit(s) landed)", who(party.seat), party.units, party.hits));
    }
    lines.push("Click the ring for the Report.".to_string());
    lines.join("\n")
}

// ------------------------------------------------------------------ the command cluster

/// Ticket #211 (version 0.08.1): what the command cluster is multiplied by, the designer's own
/// figure. One constant, so the next such request is one number.
/// Ticket #294 (version 0.08.6): a tenth larger again, at the designer's word -- *"everything on
/// the command cluster 10% larger"* -- so 1.15 times 1.1.
const CLUSTER_SCALE: f32 = 1.265;

/// Ticket #312 (version 0.08.7): what a card's Armies block -- heading, stance row, rows and the
/// orders under them -- is multiplied by, at the designer's word: *"enlarge by 10%"*, *"whole
/// block"*. Nothing in the block had a size to multiply (egui's defaults throughout), so a
/// constant is named, as `TUTORIAL_TICK` and `CLUSTER_SCALE` were, and applied to the block's text
/// styles the way the cluster applies its own.
const ARMY_LIST_SCALE: f32 = 1.1;

/// Ticket #114 (version 0.07.1): **the command cluster**, a strip along the foot of the side panel
/// that never scrolls away. The designer asked for a corner like the one CK3 and other 4X games put
/// their standing controls in: *"add influence spend button to bottom right ... with a second button
/// called something like defense to automatically split your budget across all held facilities."*
///
/// Two departures from that line, both the designer's own:
///
/// **There is no dropdown of countries.** *"Let's axe the dropdown it will be to unwieldy."* Twelve
/// Regions plus every Colony and station is a long list to open for one number, and the game
/// already has a way of naming a place: click it. So Spend acts on **whatever is selected**, and
/// with nothing selected it says so instead of offering a menu.
///
/// **"Facilities" meant places.** Influence is never spent on a Facility -- that word is a building
/// inside a Region -- and the designer confirmed the split covers **every place held**: Nation
/// States, Colonies and stations alike.
///
/// End Turn moved here from the row of buttons under the top bar. It is the one control pressed
/// every single turn, and the cluster is where a hand already is.
fn command_cluster(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, actions: &mut Vec<Action>) {
    if session.spectator || session.screen != Screen::Playing {
        return;
    }
    let (_, influence_left) = game.remaining(Seat(0), &session.pending);
    let s = game.seat(Seat(0));
    // Ticket #211 (version 0.08.1): the whole strip a seventh larger, at the designer's word --
    // *"Everything in the command cluster 15% larger"*. It is done by scaling the panel's TEXT
    // STYLES and not by editing the four explicit sizes, because most of the strip has no explicit
    // size at all: the number box, `Spend on X`, `Max` and the every-turn checkbox all take egui's
    // default, and scaling only the labelled figures would leave a 25-pixel Allotment beside an
    // unchanged button. The explicit sizes are multiplied by the same constant so the strip keeps
    // its proportions, and the icon with them.
    //
    // The cost, real and accepted knowingly: the cluster is drawn BEFORE the scrolling column and
    // reserves its height (ticket #114), so the card above it loses whatever the strip gains.
    for font in ui.style_mut().text_styles.values_mut() {
        font.size *= CLUSTER_SCALE;
    }
    ui.add_space(4.0);
    // Ticket #305 (version 0.08.7): two columns, at the designer's word -- *"next turn icon has
    // its own column on the right side of the command cluster - button sits at the bottom"*. The
    // left column holds the four rows as they were (the Allotment, the rail, Spend, Max and the
    // every-turn tick) at a width fixed BEFORE the rail is drawn, since the rail takes all the
    // width it is given; the right column is the sun's own width and nothing more, with the sun
    // pushed to the bottom by measured space so its lower edge sits level with the Max row. The
    // space is measured from the left column's rect rather than laid out bottom-up, because a
    // bottom-up layout in a panel that sizes itself from its content has no bottom to sit on.
    let sun_d = 46.2 * CLUSTER_SCALE;
    let sun_w = sun_d + 8.0;
    let sun_h = sun_d + 16.0 * CLUSTER_SCALE + 4.0;
    ui.horizontal(|ui| {
        let left_w = ui.available_width() - sun_w - ui.spacing().item_spacing.x;
        let column = ui.vertical(|ui| {
            ui.set_width(left_w);
            // Text wraps inside the column: at the panel's default width the "Click a Region or a
            // Colony" line is wider than the column and would otherwise push the sun off the edge
            // (seen in the first picture).
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
            // The Allotment, at the size the designer asked for: "much higher and more prominent".
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 5.0;
                if let Some(image) = Icons::from_ctx(ui.ctx(), "influence", 20.0 * CLUSTER_SCALE) {
                    ui.add(image);
                }
                ui.label(RichText::new(format!("{influence_left}")).size(22.0 * CLUSTER_SCALE).strong())
                    .on_hover_text("Influence still unspent this turn. It is lost at End Turn: the Allotment does not carry over.");
                ui.label(RichText::new(format!("of {} left", s.allotment)).size(15.0 * CLUSTER_SCALE));
            });

            // Spend, on whatever is selected.
            let target = match view.selection {
                Selection::State(sid) => Some((Place::State(sid), game.tables.state(sid).name.clone())),
                Selection::Colony(cid) => Some((Place::Colony(cid), game.place_name(Place::Colony(cid)))),
                _ => None,
            };
            // Ticket #294 (version 0.08.6): the amount is set on the rail the Smear and the Greenwash use
            // (`influence_rail`, ticket #293), at the designer's word -- *"spending influence on the
            // command cluster is now a slider"* -- so the three Influence spends read alike: single points
            // from nought to the turn's whole Influence, what is already ordered greyed from the right.
            // The rail is drawn whether or not a place is selected, so the strip never changes shape; the
            // Spend button below it is what needs the place.
            let (whole, left) = influence_this_turn(game, session);
            let amount = influence_rail(ui, &mut view.influence_amount, whole, left, None);
            ui.horizontal(|ui| {
                match &target {
                    Some((place, name)) => {
                        let order = Order::Influence { target: *place, amount };
                        let check = game.check_order(Seat(0), &session.pending, &order);
                        let resp = ui.add_enabled(check.is_ok(), egui::Button::new(format!("Spend {amount} on {name}")));
                        if let Err(e) = &check {
                            resp.clone().on_disabled_hover_text(&e.0);
                        }
                        if resp.on_hover_text(format!("Raise your Standing on {name}. Its card shows what it would take to hold or take it.")).clicked() {
                            actions.push(Action::Place(order));
                        }
                    }
                    None => {
                        // Ticket #306 (version 0.08.7): the sentence that stood here is cut as a
                        // duplicate of the Max button's hover, at the designer's word; a greyed
                        // Spend keeps the strip's shape, since the rail is drawn either way.
                        ui.add_enabled(false, egui::Button::new(format!("Spend {amount}")));
                    }
                }
            });

            // Ticket #134 (version 0.07.3): Max, exactly where Defence stood. The designer: *"Get rid of
            // defense button replace with a max spend button that just say Max."* One press places one
            // order spending everything left this turn on the selected place; greyed with a hint when
            // nothing is selected. Ticket #294 (version 0.08.6): and the rail follows it to the bound --
            // *"it also moves the slider to max"* -- and, until ticket #305, End Turn shared this row.
            ui.horizontal(|ui| {
                let button = egui::Button::new(RichText::new("Max").strong());
                match &target {
                    Some((place, name)) if influence_left > 0 => {
                        let order = Order::Influence { target: *place, amount: influence_left };
                        let check = game.check_order(Seat(0), &session.pending, &order);
                        let resp = ui.add_enabled(check.is_ok(), button);
                        if let Err(e) = &check {
                            resp.clone().on_disabled_hover_text(&e.0);
                        }
                        if resp.on_hover_text(format!("Spend all {influence_left} left this turn on {name}.")).clicked() {
                            view.influence_amount = influence_left;
                            actions.push(Action::Place(order));
                        }
                    }
                    Some(_) => {
                        ui.add_enabled(false, button).on_disabled_hover_text("Nothing left to spend this turn.");
                    }
                    None => {
                        ui.add_enabled(false, button).on_disabled_hover_text("Click a Region or a Colony to spend on it");
                    }
                }
                // The standing order, on the shape the Archive's funding already uses: a pending order that
                // sets a seat field, so it survives a save and shows in the turn's order list like anything
                // else. It never spends by itself -- next turn it places the whole Allotment on the place
                // as a pending order, which the player can read and cancel; cancelling it ends the standing
                // order too (the designer's addition), as does the place ceasing to be theirs.
                let pending_flip = session.pending.iter().find_map(|o| match o {
                    Order::SetMaxStanding { target } => Some(*target),
                    _ => None,
                });
                let standing = pending_flip.unwrap_or(s.max_standing);
                let mut on = standing.is_some();
                let label = match standing {
                    Some(place) => format!("every turn on {}", game.place_name(place)),
                    None => "every turn".to_string(),
                };
                let can_tick = on || target.is_some();
                let resp = ui.add_enabled(can_tick, egui::Checkbox::new(&mut on, label));
                let resp = resp
                    .on_hover_text("Spend your whole Allotment on this place at the start of every turn from now on. It is placed as an ordinary order you can read and cancel before ending the turn, never spent behind your back; cancelling it, or losing the place, ends it.")
                    .on_disabled_hover_text("Click a Region or a Colony first.");
                if resp.changed() {
                    if let Some(i) = session.pending.iter().position(|o| matches!(o, Order::SetMaxStanding { .. })) {
                        actions.push(Action::Cancel(i));
                    } else {
                        let target = if on { target.as_ref().map(|(p, _)| *p) } else { None };
                        actions.push(Action::Place(Order::SetMaxStanding { target }));
                    }
                }
            });
        });
        // End Turn, where a hand already is. Ticket #128 (version 0.07.2): named for its key.
        // Ticket #294 (version 0.08.6): a sun, the words beneath it and the key on the hover, at the
        // designer's word; a tenth larger again on seeing the first picture: "make it 10% larger".
        // Ticket #305 (version 0.08.7): in its own column at the right, at the bottom, nothing above it.
        let left_h = column.response.rect.height();
        ui.vertical(|ui| {
            if left_h > sun_h {
                ui.add_space(left_h - sun_h);
            }
            let resp = sun_button(ui, can_end_turn(game, view), sun_d, "End Turn");
            if resp.on_hover_text("End the turn (Enter).").on_disabled_hover_text("Pick a Tech first").clicked() {
                press_end_turn(session, game, view, actions);
            }
        });
    });
    ui.add_space(4.0);
}

/// Ticket #294 (version 0.08.6): **End Turn as a sun** -- a shaded disc with sunspots and a
/// darkened limb, the word beneath it -- at the designer's word: *"make it resemble the sun with
/// sun spots and what not, put the words below the disk."* Drawn by hand with the painter, as
/// the roster's order ring is, because nothing round and clickable existed in the interface and
/// an SVG would have needed the icon ledger. The shading is concentric discs brightening toward a
/// point above and left of centre, which is how a sphere is drawn without a gradient; the limb is
/// a darker outer ring; the spots are a few small dark discs, fixed so the sun does not flicker.
/// Dimmed to embers while it cannot be pressed. Returns the click response.
fn sun_button(ui: &mut Ui, enabled: bool, diameter: f32, word: &str) -> egui::Response {
    let label_h = 16.0 * CLUSTER_SCALE;
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(diameter + 8.0, diameter + label_h + 4.0), if enabled { egui::Sense::click() } else { egui::Sense::hover() });
    let centre = egui::pos2(rect.center().x, rect.min.y + 4.0 + diameter / 2.0);
    let r = diameter / 2.0;
    let p = ui.painter();
    let lit = enabled && resp.hovered();
    // The palette: a sun, or its embers while End Turn is dead.
    // Softened on the designer's word at the first picture ("soften the shading"): the rings sit
    // closer in colour and drift less, so the disc reads as one lit body rather than a target.
    let (glow, limb, rings, spot, ink): (Color32, Color32, [Color32; 4], Color32, Color32) = if enabled {
        (
            Color32::from_rgba_unmultiplied(255, 170, 60, if lit { 70 } else { 40 }),
            Color32::from_rgb(215, 110, 30),
            [Color32::from_rgb(238, 150, 45), Color32::from_rgb(248, 180, 70), Color32::from_rgb(252, 205, 105), Color32::from_rgb(255, 228, 150)],
            Color32::from_rgba_unmultiplied(120, 50, 15, 170),
            Color32::from_rgb(255, 225, 160),
        )
    } else {
        (
            Color32::from_rgba_unmultiplied(120, 70, 40, 20),
            Color32::from_rgb(80, 48, 34),
            [Color32::from_rgb(92, 58, 38), Color32::from_rgb(104, 66, 43), Color32::from_rgb(116, 76, 49), Color32::from_rgb(128, 88, 56)],
            Color32::from_rgba_unmultiplied(40, 20, 10, 170),
            Color32::from_gray(120),
        )
    };
    p.circle_filled(centre, r * 1.25, glow);
    p.circle_filled(centre, r, limb);
    // Four discs, each a little smaller and brighter, drifting gently toward the light above and
    // to the left.
    for (i, colour) in rings.iter().enumerate() {
        let k = (i + 1) as f32;
        let shrink = 1.0 - 0.14 * k;
        let off = egui::vec2(-r * 0.05 * k, -r * 0.06 * k);
        p.circle_filled(centre + off, r * shrink, *colour);
    }
    // Sunspots: two pairs low on the disc where the light does not reach, and one alone.
    for (dx, dy, s) in [(0.30, 0.28, 0.12), (0.42, 0.18, 0.07), (-0.34, 0.36, 0.10), (-0.22, 0.44, 0.06), (0.05, -0.45, 0.05)] {
        p.circle_filled(centre + egui::vec2(r * dx, r * dy), r * s, spot);
    }
    p.text(egui::pos2(rect.center().x, centre.y + r + 3.0), egui::Align2::CENTER_TOP, word, FontId::proportional(13.0 * CLUSTER_SCALE), ink);
    resp
}

/// End Turn is dead while a Tech pick is owed and while a popup is up.
fn can_end_turn(game: &Game, view: &ViewState) -> bool {
    let must_pick = game.research.awaiting_pick == Some(Seat(0)) && !game.available_techs().is_empty();
    !must_pick && view.popup == Popup::None
}

/// Ticket #128 (version 0.07.2): what pressing End Turn does, whether by the button or by Enter,
/// so the key can never do more than the button. Unspent Influence raises the confirmation rather
/// than ending the turn; on that confirmation, pressing again confirms. A spectator has no
/// Influence to lose and no confirmation.
fn press_end_turn(session: &Session, game: &Game, view: &mut ViewState, actions: &mut Vec<Action>) {
    // Ticket #323 (version 0.08.8): End Turn disarms the stack.
    view.armed_stack = None;
    if view.popup == Popup::ConfirmEndTurn {
        view.popup = Popup::None;
        actions.push(Action::EndTurn);
        return;
    }
    if !can_end_turn(game, view) {
        return;
    }
    let (_, influence_left) = game.remaining(Seat(0), &session.pending);
    if !session.spectator && influence_left > 0 && game.seat(Seat(0)).allotment > 0 {
        view.popup = Popup::ConfirmEndTurn;
    } else {
        actions.push(Action::EndTurn);
    }
}

// ------------------------------------------------------------------ the side panel

fn side_panel(root: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, actions: &mut Vec<Action>) {
    // Ticket #64: a spectator gives no orders, so the panel that held the order list holds the
    // table instead -- all four Factions' boards -- and whatever has been clicked opens on the
    // left, where nothing of the spectator's own has to share the room with it.
    if session.spectator {
        egui::Panel::left("spectator_card").default_size(400.0).resizable(true).show(root, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| selection_card(ui, session, game, view, actions));
        });
        egui::Panel::right("side").default_size(360.0).resizable(true).show(root, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| roster(ui, session, game, view));
        });
        return;
    }
    egui::Panel::right("side").default_size(360.0).resizable(true).show(root, |ui| {
        // Ticket #114, kept by ticket #134 for Max: the standing order, applied here, once a turn,
        // before anything is drawn. It places an ordinary pending order rather than spending
        // anything, so the spend arrives in the turn's order list where it can be read and
        // cancelled like any other order. It is applied only on a turn the player has not already
        // placed Influence, so it never fights a hand.
        if !session.spectator
            && let Some(place) = game.seat(Seat(0)).max_standing
            && view.max_placed != Some(game.turn)
        {
            view.max_placed = Some(game.turn);
            if !session.pending.iter().any(|o| matches!(o, Order::Influence { .. })) {
                let (_, influence_left) = game.remaining(Seat(0), &session.pending);
                if influence_left > 0 {
                    actions.push(Action::Place(Order::Influence { target: place, amount: influence_left }));
                }
            }
        }
        // Ticket #114: the cluster is shown BEFORE the scrolling column, so it reserves its strip
        // at the foot of the panel and the card or the roster scrolls above it rather than pushing
        // it off the bottom. That is the whole point of a command cluster: it is always there.
        egui::Panel::bottom("command_cluster").show(ui, |ui| command_cluster(ui, session, game, view, actions));
        egui::ScrollArea::vertical().show(ui, |ui| {
            selection_card(ui, session, game, view, actions);
            if view.selection == Selection::None {
                roster(ui, session, game, view);
            }
            ui.separator();
            ui.label(RichText::new(format!("Orders this turn ({})", session.pending.len())).strong());
            if let Some(e) = session.last_error.as_ref().filter(|_| view.selection != Selection::None) {
                ui.colored_label(Color32::LIGHT_RED, e);
            }
            let mut cancel: Option<usize> = None;
            for (i, o) in session.pending.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("{} ({})", order_text(game, o), game.order_cost(Seat(0), o).text()));
                    if ui.small_button("cancel").clicked() {
                        cancel = Some(i);
                    }
                });
            }
            if let Some(i) = cancel {
                actions.push(Action::Cancel(i));
            }
        });
    });
}

/// Whatever is selected, as its card: the view's own heading and the stations over it when nothing
/// is.
fn selection_card(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, actions: &mut Vec<Action>) {
    match view.selection {
        Selection::None => {
            ui.add_space(6.0);
            ui.label(RichText::new(match view.view {
                View::Solar => "Solar System Map",
                View::Surface(BodyId::Earth) => "Earth Map",
                View::Surface(b) => game.tables.body(b).name.as_str(),
            }).size(20.0).strong());
            ui.label(match (view.view, session.spectator) {
                (View::Solar, false) => "Click a Body to enter its surface. Click a Ship stack for orders.",
                (View::Solar, true) => "Click a Body to enter its surface. Click a Ship stack to read it.",
                (View::Surface(BodyId::Earth), false) => "Click a Region for its card and orders. Drag to turn, wheel to zoom.",
                (View::Surface(BodyId::Earth), true) => "Click a Region for its card. Drag to turn, wheel to zoom.",
                (View::Surface(_), false) => "Click a Colony Slot or Colony for its card and orders.",
                (View::Surface(_), true) => "Click a Colony Slot or Colony for its card.",
            });
            if let Some(e) = &session.last_error {
                ui.colored_label(Color32::LIGHT_RED, e);
            }
            // Ticket #283 (version 0.08.5): the planet's own figure at the head of its card, moved
            // here from the founding door at the designer's word, and a block of every Colony on
            // the ground and every site still open, each with its own yields in glyphs. A station
            // reads the Body's figures, so the In orbit rows beneath carry none. On Earth only the
            // Colonies stand here: Antarctica's shut sites are the ice's business.
            if let View::Surface(b) = view.view {
                colonies_block(ui, game, view, b);
            }
            // Ticket #317 (version 0.08.8): last turn's Battles, listed on the Solar System Map's
            // page, one row per Battle in the aggressor's colour, each a way there; the Report is
            // reachable by no button once it has closed, and this is where a player who missed a
            // mark finds the fight.
            if view.view == View::Solar && !game.report.battles.is_empty() {
                ui.label(RichText::new("Battles last turn").strong());
                for (i, b) in game.report.battles.iter().enumerate() {
                    let who = b.aggressor().map(|s| format!("the {} attacked", game.seat_name(s))).unwrap_or_else(|| "nobody attacked".to_string());
                    let text = format!("{}: {who}; {}", b.place, b.result);
                    let colour = battle_colour(session, game, i);
                    let button = match Kind::Battle.image(ui.ctx(), 14.0) {
                        Some(image) => egui::Button::image_and_text(image, RichText::new(&text).color(colour)),
                        None => egui::Button::new(RichText::new(&text).color(colour)),
                    };
                    let resp = ui.add(button.frame(false));
                    match b.at {
                        Some(place) => {
                            if resp.on_hover_text("Go there").clicked() {
                                actions.push(Action::GoTo(place));
                            }
                        }
                        None => {
                            resp.on_hover_text("An older record with no place to go to.");
                        }
                    }
                }
                ui.separator();
            }
            stations_panel(ui, session, game, view, actions);
            ui.separator();
        }
        Selection::State(sid) => state_panel(ui, session, game, view, sid, actions),
        Selection::Colony(cid) => colony_panel(ui, session, game, view, cid, actions),
        Selection::Slot(body, slot) => slot_panel(ui, session, game, body, slot, actions),
        Selection::ShipStack(body, seat) => stack_panel(ui, session, game, view, body, seat, actions),
    }
}

/// The roster (#23): every Ship stack, Army, Colony and Region the player directs, each row a
/// button that selects it and jumps to its view, with a mark on anything that has no order this turn.
/// Ticket #64: a spectator directs nothing, so their roster deals all four Factions, each under its
/// own heading in its own colour, and marks nothing, since nobody owes an order.
fn roster(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState) {
    let mut jump: Option<(View, Selection)> = None;
    if session.spectator {
        ui.label(RichText::new("The table").size(18.0).strong());
        ui.label(RichText::new("Every Faction's board. Click a row to go there.").weak());
        for seat in Seat::ALL {
            ui.add_space(6.0);
            ui.label(RichText::new(game.seat_name(seat)).size(17.0).strong().color(seat_colour(session, seat)));
                roster_of(ui, session, game, seat, false, &mut jump);
        }
    } else {
        // Ticket #163 (version 0.07.5): the roster has no button on the bar, so its heading names
        // the key that brings it back, which is the rule every bar button follows.
        ui.label(RichText::new("Your roster (Tab)").size(18.0).strong());
        // Ticket #127 (version 0.07.2): the ring, in place of 0.07.1's "no order" and its filter.
        ui.label(RichText::new("Click a row to select it and go there. An open ring marks a row that still wants an order; it fills once the order is given.").weak());
        roster_of(ui, session, game, Seat(0), true, &mut jump);
    }
    if let Some((v, sel)) = jump {
        match v {
            View::Solar => {
                view.view = View::Solar;
            }
            View::Surface(b) => {
                if view.view != View::Surface(b) {
                    view.enter_surface(b);
                }
            }
        }
        view.selection = sel;
        view.attack_preview = false;
        // Ticket #162 (version 0.07.5): selecting a Colony is enough -- its card carries the
        // Module tiles now, so there is no window for the roster row to open. The clicked tile is
        // cleared, as it was when the row opened a fresh window.
        if matches!(sel, Selection::Colony(_)) {
            view.hab_tile = None;
        }
    }
}

/// One seat's Ships, Armies, Colonies and stations and Regions. `marks` writes the "no order"
/// mark, which only a seat that gives orders can owe; where it is off, four Factions share the
/// panel, so each group is named on its own rows rather than over a heading, and an empty group is
/// left out instead of saying so.
/// Ticket #115 (version 0.07.1): one row of the roster, gathered before any of it is drawn so a
/// group can be counted, sorted and filtered before it goes on screen.
struct RosterRow {
    /// Ticket #127 (version 0.07.2): what kind of thing the row names, for the glyph in front of it.
    kind: Kind,
    text: String,
    /// Ticket #116 (version 0.07.1): the rule behind the row's figures, where one governs them.
    tip: Option<String>,
    /// Ticket #127 (version 0.07.2): the ring at the row's end. `Some(true)` is a row that still
    /// wants a decision from the player this turn, drawn open; `Some(false)` a row that has its
    /// order, drawn filled; `None` a row that cannot want one -- a Ship in transit, an Army, or any
    /// row of a seat that gives no orders -- and it wears no ring at all.
    mark: Option<bool>,
    jump: Option<(View, Selection)>,
}

/// A roster group: its heading and the line shown when it is empty.
///
/// Ticket #127 (version 0.07.2): the heading carried a count of the rows that still wanted an
/// order, clickable to filter the group down to them, for one version. The designer replaced it:
/// "Rather than a button to filter I want an icon displayed to the right of the item indicating an
/// order is needed" -- and no count either, since the rings say it row by row. The heading is a
/// heading again; the filter, its state and its building aid are gone.
struct RosterGroup {
    name: &'static str,
    empty: &'static str,
}

fn roster_group(ui: &mut Ui, group: RosterGroup, rows: Vec<RosterRow>, marks: bool, jump: &mut Option<(View, Selection)>) {
    let RosterGroup { name, empty } = group;
    if marks {
        ui.label(RichText::new(name).strong());
    }
    if rows.is_empty() {
        if marks {
            ui.label(RichText::new(empty).weak());
        }
        return;
    }
    for row in &rows {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            // Ticket #127: the kind glyph rides inside the button where there is art for it.
            // Ticket #138 (version 0.07.3): the Army's shield rides inside too -- it stood beside
            // the button, a drawn shape having no way into egui's, and the designer saw it: *"every
            // other glyph is a part of the button while armies stands apart."* Its button is drawn
            // by hand, the way a priced button is.
            let mut resp = match row.kind.image(ui.ctx(), KIND_GLYPH) {
                Some(image) => ui.add(egui::Button::image_and_text(image, &row.text)),
                None => glyph_button(ui, row.kind, &row.text),
            };
            if let Some(tip) = &row.tip {
                resp = rule_tip(resp, tip.clone());
            }
            if resp.clicked() {
                *jump = row.jump;
            }
            if let Some(wants) = row.mark {
                order_ring(ui, wants);
            }
        });
    }
}

/// The ring's colours: open in the warm amber 0.07.1's count wore, since it means the same thing;
/// filled in a quiet grey, so an attended row is marked as attended and asks for nothing.
const RING_WANTS: Color32 = Color32::from_rgb(250, 210, 130);
const RING_ATTENDED: Color32 = Color32::from_rgb(150, 150, 150);

/// Ticket #127 (version 0.07.2): the ring at the end of a roster row -- open while the row still
/// wants an order this turn, filled once it has one. A mark on the row itself, at the designer's
/// word, in place of a control on the heading.
fn order_ring(ui: &mut Ui, wants: bool) {
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(12.0, 12.0), egui::Sense::hover());
    let (centre, radius) = (rect.center(), 4.5);
    if wants {
        ui.painter().circle_stroke(centre, radius, egui::Stroke::new(1.5, RING_WANTS));
    } else {
        ui.painter().circle_filled(centre, radius, RING_ATTENDED);
    }
    resp.on_hover_text(if wants { "Still wants an order from you this turn." } else { "Has its order for this turn." });
}

fn roster_of(ui: &mut Ui, session: &Session, game: &Game, seat: Seat, marks: bool, jump: &mut Option<(View, Selection)>) {
    let pending = &session.pending;
    let tag = |text: &str| if marks { String::new() } else { format!("{text}: ") };

    // Ticket #216 (version 0.08.2): ONE ROW PER SHIP, at a Body and in transit alike, replacing the
    // row-per-stack this list carried since ticket #115. Every Ship has a name since ticket #210 and
    // the designer wants to read it here; the ring that says a thing still wants an order is also
    // per-SHIP in meaning and was only per-stack by accident of the row.
    //
    // Affordable because it was measured, not assumed: over 200 games the median Faction holds ZERO
    // Ships, one or two is typical, and the largest fleet ever seen was 27 (an Arkwright yard
    // backlog at Earth, once). The rows this adds are almost always none.
    let mut ships_rows: Vec<RosterRow> = Vec::new();
    for body in BodyId::ALL {
        let ships: Vec<&Ship> = game.ships.iter().filter(|s| s.seat == seat && s.at == ShipAt::Body(body)).collect();
        for s in ships {
            let ordered = pending.iter().any(|o| {
                matches!(o, Order::Transit { ship, .. } | Order::Load { ship, .. } | Order::Unload { ship, .. } | Order::Repair { unit: UnitRef::Ship(ship), .. } if *ship == s.id)
            }) || pending.iter().any(|o| matches!(o, Order::ShipStance { body: b, .. } if *b == body));
            let tank = game.tables.unit(s.kind).tank;
            // The working figures stay ON the row: the Roster is where a player checks whether a
            // hull can move before ordering it, and the tank is the figure that says stranded.
            let mut text = format!(
                "{}{} ({}) at {} - strength {}, {}/{}",
                tag("Ship"),
                game.ship_name(s),
                s.kind.name().to_lowercase(),
                game.tables.body(body).name,
                game.ship_strength(s),
                s.fuel,
                tank
            );
            if s.colonists > 0 {
                text.push_str(&format!(", {} Colonists aboard", s.colonists));
            }
            if s.army.is_some() {
                text.push_str(", an Army aboard");
            }
            if s.damage > 0 {
                text.push_str(&format!(", damage {}/{}", s.damage, game.tables.unit(s.kind).hit_points));
            }
            // Stranded is now per-SHIP rather than the old all-or-nothing warning on the stack,
            // which is strictly more accurate: one hull can be dry while another beside it is full.
            // Ticket #313 (version 0.08.7): the stance word in brackets, as an Army's row has it,
            // and the stance's sentence on the hover; a Ship on Intercept was the case nobody could
            // read anywhere.
            text.push_str(&format!("  ({})", s.stance.name()));
            if game.stranded(s.id) {
                text.push_str(" - STRANDED: no leg affordable and no station of yours here");
            }
            // Ticket #116 (version 0.07.1): what the tank is for, and what being stranded means. A Ship
            // with no leg it can afford and no station of its own is the one piece in the game that can
            // become permanently useless, and the roster said so in four words and explained none of it.
            let tip = format!(
                "{}: {} {}\nTank {} of {}. Fuel goes on transits, and a leg costs least at a launch window.\nRefuelling needs a station or Colony of yours where the Ship sits, so a Ship is STRANDED with no leg it can afford and nowhere to fill up.",
                s.stance.name(),
                s.stance.one_liner(true),
                Stance::PERSISTS,
                s.fuel,
                tank
            );
            ships_rows.push(RosterRow { kind: Kind::of_unit(s.kind), text, tip: Some(tip), mark: marks.then_some(!ordered), jump: Some((View::Solar, Selection::ShipStack(body, seat))) });
        }
    }
    for s in game.ships.iter().filter(|s| s.seat == seat) {
        if let ShipAt::Transit { to, turns_left, .. } = s.at {
            // A Ship in transit is not waiting on anybody: it arrives when it arrives. Named the same
            // way as a Ship at a Body, so the two read as the same kind of thing -- and the same way
            // the Solar System Map labels the transit, which is the point of naming it in both.
            ships_rows.push(RosterRow {
                kind: Kind::of_unit(s.kind),
                text: format!("{}{} ({}) - in transit to {}, {} turn(s) left", tag("Ship"), game.ship_name(s), s.kind.name().to_lowercase(), game.tables.body(to).name, turns_left),
                tip: Some("A Ship in transit cannot be ordered and cannot be intercepted. It arrives at its Resolution, Holding, with whatever Fuel it has left.".to_string()),
                mark: None,
                jump: Some((View::Solar, Selection::None)),
            });
        }
    }
    roster_group(ui, RosterGroup { name: "Ships", empty: "  none; a Shipyard on a station or Colony builds them" }, ships_rows, marks, jump);

    // Armies, sorted by where they stand rather than by when they were raised.
    //
    // Ticket #115: an Army does NOT carry the "no order" mark. The designer: "let's allow an armies
    // orders to park them in that stance until otherwise moved - a army on defense should remain on
    // defense unless told otherwise." The engine already worked that way and now a test guards it,
    // so an Army standing where it was put is attended by definition, and the mark had been firing
    // on twelve rows out of twelve, which says exactly as much as firing on none. What the row
    // shows in its place is the stance it is parked in, so "until otherwise moved" is something a
    // player can SEE.
    let mut army_rows: Vec<(String, RosterRow)> = Vec::new();
    for a in game.armies.iter().filter(|a| game.army_seat(a) == Some(seat) && !game.army_stands_down(a)) {
        let (where_, target) = match a.at {
            ArmyAt::Place(Place::State(s)) => (game.tables.state(s).name.clone(), Some((View::Surface(BodyId::Earth), Selection::State(s)))),
            ArmyAt::Place(Place::Colony(c)) => (game.place_name(Place::Colony(c)), game.colony(c).map(|col| (View::Surface(col.body), Selection::Colony(c)))),
            ArmyAt::Aboard(ship) => (format!("aboard {ship}"), game.ship(ship).map(|s| match s.at { ShipAt::Body(b) => (View::Solar, Selection::ShipStack(b, seat)), _ => (View::Solar, Selection::None) })),
        };
        // Ticket #115: the heading already says "Armies", so every row repeating the word was pure
        // width, and "damage 0" is true of almost every Army almost always.
        // Ticket #270 (version 0.08.4): the Army's own name leads the row.
        let mut text = format!("{}{}, at {}: strength {}", tag("Army"), game.army_name(a), where_, game.army_strength(a));
        if a.damage > 0 {
            text.push_str(&format!(", damage {}", a.damage));
        }
        if !a.standing {
            text.push_str(", raised");
        }
        if !matches!(a.at, ArmyAt::Aboard(_)) {
            text.push_str(&format!("  ({})", a.stance.name()));
        }
        // Ticket #313 (version 0.08.7): the stance's sentence is the engine's, the same one the
        // stance row's label shows, so it lives once.
        let tip = format!(
            "An Army keeps the stance it was last given until it is moved or given another; it is never reset at a Resolution.\n{}: {} A Standing Army replenishes where it stands, unless its state's Unrest has passed {:.0}.",
            a.stance.name(),
            a.stance.one_liner(false),
            game.tables.unrest.army_threshold
        );
        army_rows.push((where_, RosterRow { kind: Kind::Army, text, tip: Some(tip), mark: None, jump: target }));
    }
    army_rows.sort_by(|a, b| a.0.cmp(&b.0));
    roster_group(ui, RosterGroup { name: "Armies", empty: "  none" }, army_rows.into_iter().map(|(_, r)| r).collect(), marks, jump);

    // Colonies and stations.
    //
    // Ticket #115: these DO carry the mark, on the designer's answer, and it counts this turn's
    // orders alone -- a Colony with a Module three turns from done is attended, not neglected.
    let mut colony_rows: Vec<RosterRow> = Vec::new();
    for c in game.colonies.iter().filter(|c| c.control.director() == Some(seat)) {
        let building = c.queue.len();
        let ordered = building > 0 || pending.iter().any(|o| roster_order_touches_colony(o, c.id));
        // Ticket #127: the spectator's word tag agrees with the glyph, so a station is not a "Colony:".
        // Ticket #165 (version 0.07.5): the count is the one the card shows, which leaves out the
        // Core Module every place has and the Archive, so the roster and the card cannot disagree.
        let text = format!(
            "{}{}: {} Colonists, {} Modules{}",
            tag(if c.in_orbit { "Station" } else { "Colony" }),
            game.place_name(Place::Colony(c.id)),
            c.colonists,
            c.modules.iter().filter(|m| m.kind != ModuleKind::Core && m.kind != ModuleKind::Archive).count(),
            if building > 0 { format!(", {building} building") } else { String::new() }
        );
        colony_rows.push(RosterRow { kind: Kind::of_colony(c), text, tip: None, mark: marks.then_some(!ordered), jump: Some((View::Surface(c.body), Selection::Colony(c.id))) });
    }
    roster_group(ui, RosterGroup { name: "Colonies and stations", empty: "  none; a Colony Ship founds one" }, colony_rows, marks, jump);

    // Regions, on the same rule as the Colonies.
    let mut state_rows: Vec<RosterRow> = Vec::new();
    for sid in game.directed_states(seat) {
        let st = game.state(sid);
        let building = st.queue.len();
        let ordered = building > 0 || pending.iter().any(|o| roster_order_touches_state(o, sid));
        let text = format!("{}{}: {} Facilities, {} free slot(s){}", tag("Region"), game.tables.state(sid).name, st.facilities.len(), game.free_slots(sid), if building > 0 { format!(", {building} building") } else { String::new() });
        state_rows.push(RosterRow { kind: Kind::Region, text, tip: None, mark: marks.then_some(!ordered), jump: Some((View::Surface(BodyId::Earth), Selection::State(sid))) });
    }
    roster_group(ui, RosterGroup { name: "Regions", empty: "  none" }, state_rows, marks, jump);
}

/// Ticket #115: does this pending order do anything to that Colony this turn? It decides only
/// whether the roster marks the row, so a near miss costs a mark and nothing else.
fn roster_order_touches_colony(o: &Order, cid: ColonyId) -> bool {
    match o {
        Order::BuildModule { colony, .. } | Order::BuildModuleWithDucats { colony, .. } | Order::BuildArchive { colony } => *colony == cid,
        Order::BuildArmy { place } | Order::Influence { target: place, .. } => *place == Place::Colony(cid),
        Order::BuildShip { site, .. } => *site == Place::Colony(cid),
        Order::Unload { into: UnloadTarget::Colony(c), .. } => *c == cid,
        Order::Change { building, .. } => matches!(building, BuildingRef::Module(c, _) if *c == cid),
        Order::ArmyStance { place, .. } => *place == Place::Colony(cid),
        _ => false,
    }
}

/// The same for a Region.
fn roster_order_touches_state(o: &Order, sid: StateId) -> bool {
    match o {
        Order::BuildFacility { state, .. } | Order::BuildFacilityWithDucats { state, .. } | Order::RaiseIndustry { state } => *state == sid,
        Order::BuildArmy { place } | Order::Influence { target: place, .. } => *place == Place::State(sid),
        Order::BuildShip { site, .. } => *site == Place::State(sid),
        Order::MoveArmy { to, .. } => *to == sid,
        Order::Change { building, .. } => matches!(building, BuildingRef::Facility(s, _) if *s == sid),
        Order::ArmyStance { place, .. } => *place == Place::State(sid),
        _ => false,
    }
}

fn order_text(game: &Game, o: &Order) -> String {
    match o {
        // Ticket #226 (version 0.08.2): the Accord orders, so a pending one reads in the order list
        // like any other and can be cancelled like any other.
        Order::ProposeAccord { to, terms } => {
            let names: Vec<&str> = terms
                .iter()
                .map(|t| match t {
                    Term::NonAggression => "non-aggression",
                    Term::Passage => "passage",
                    Term::Refuel => "refuel",
                    Term::ResearchAgreement => "a research agreement",
                })
                .collect();
            format!("Offer the {} an Accord: {}", game.seat_name(*to), names.join(", "))
        }
        Order::EndAccord { with } => format!("Declare your Accord with the {} over", game.seat_name(*with)),
        // Ticket #332 (version 0.09.0): a rival's build cancelled at a place that changed hands.
        Order::CancelBuild { place, index } => {
            let building = game.queue_at(*place).get(*index).map(|b| b.item.name()).unwrap_or_else(|| "build".to_string());
            format!("Cancel the {building} under way at {}", game.place_name(*place))
        }
        Order::Tribute { to, materials } => format!("Pay the {} a tribute in {}", game.seat_name(*to), if *materials { "Materials" } else { "Ducats" }),
        Order::BuildFacility { state, kind } => format!("Build {} in {}", kind.name(), game.tables.state(*state).name),
        Order::RaiseIndustry { state } => format!("Raise Industry Level in {}", game.tables.state(*state).name),
        Order::BuildModule { colony, kind } => format!("Build {} at {}", kind.name(), game.place_name(Place::Colony(*colony))),
        Order::BuildShip { site, kind } => format!("Build {} at {}", kind.name(), game.place_name(*site)),
        Order::BuildArmy { place } => format!("Build Army at {}", game.place_name(*place)),
        Order::Repair { unit, points } => format!("Repair {} point(s) on {}", points, unit_name(game, unit)),
        Order::Transit { ship, to, slot } => match slot {
            Some(n) => format!("Send {} to {}, into Orbital Slot {}", ship, game.tables.body(*to).name, n),
            None => format!("Send {} to {}", ship, game.tables.body(*to).name),
        },
        Order::Refuel { ship } => format!("Refuel {} ({} Fuel from the Stockpile)", ship, game.refuel_amount(Seat(0), *ship)),
        Order::ShipStance { body, stance } => format!("Ships at {}: {}", game.tables.body(*body).name, stance.name()),
        Order::Bombard { ship, colony } => format!("Bombard {} from {}", game.place_name(Place::Colony(*colony)), ship),
        Order::ArmyStance { place, stance } => format!("Armies at {}: {}", game.place_name(*place), stance.name()),
        Order::MoveArmy { army, to } => format!("{} to {}", army, game.tables.state(*to).name),
        Order::Load { ship, colonists, army, .. } => format!("Load {} onto {}", if *colonists > 0 { format!("{colonists} Colonists") } else { format!("{}", army.unwrap_or(ArmyId(0))) }, ship),
        Order::Unload { ship, colonists, army, into } => match into {
            UnloadTarget::Slot(b, s) => format!("Found a Colony at {} on {} from {}", game.tables.body(*b).slots[*s as usize].name, game.tables.body(*b).name, ship),
            UnloadTarget::Colony(c) => format!("Unload {} from {} into {}", if *colonists > 0 { format!("{colonists} Colonists") } else if *army { "the Army".into() } else { "nothing".into() }, ship, game.place_name(Place::Colony(*c))),
        },
        Order::Influence { target, amount } => format!("{} Influence on {}", amount, game.place_name(*target)),
        Order::BuyInfluence { amount } => format!("Buy {} Influence with Ducats", amount),
        Order::RepairWithDucats { unit, points } => format!("Repair {} point(s) on {} with Ducats", points, unit_name(game, unit)),
        Order::Buy { resource, amount } => format!("Buy {} {} for {} Ducats", amount, resource.name(), game.order_cost(Seat(0), o).ducats),
        Order::Sell { resource, amount } => format!("Sell {} {} for {} Ducats", amount, resource.name(), -game.order_cost(Seat(0), o).ducats),
        Order::BuildFacilityWithDucats { state, kind } => format!("Build {} in {} for Ducats", kind.name(), game.tables.state(*state).name),
        Order::BuildModuleWithDucats { colony, kind } => format!("Build {} at {} for Ducats", kind.name(), game.place_name(Place::Colony(*colony))),
        Order::BuildStation { body, slot } => format!("Build {} over {}", game.station_name(*body, *slot), game.tables.body(*body).name),
        Order::BuildArchive { colony } => format!("Build the Archive at {}", game.place_name(Place::Colony(*colony))),
        Order::SetMaxStanding { target: Some(p) } => format!("Spend your whole Allotment on {}, every turn", game.place_name(*p)),
        Order::SetMaxStanding { target: None } => "Place your Influence by hand again".to_string(),
        Order::SetResearchDirective { percent: 0 } => "Pay all your Research into the shared Tech from the next Income".to_string(),
        Order::SetResearchDirective { percent } => format!("Direct {percent}% of your Research from the next Income"),
        // Ticket #73.
        Order::BuildEmigrants { state, n } => format!("Recruit {n} Pioneers in {}", game.tables.state(*state).name),
        Order::LiftToStation { state, n, colony } => format!("Send {n} Pioneers from {} to {} by lift", game.tables.state(*state).name, game.place_name(Place::Colony(*colony))),
        Order::SendToAntarctica { state, n, into } => format!(
            "Send {n} Pioneers from {} to {} by sea",
            game.tables.state(*state).name,
            match into {
                UnloadTarget::Slot(_, slot) => game.tables.body(BodyId::Earth).slots[*slot as usize].name.clone(),
                UnloadTarget::Colony(c) => game.place_name(Place::Colony(*c)),
            }
        ),
        // Ticket #72.
        Order::SetVentureShare { share } => format!("Bank {share}% of Ducat income in the Venture Capital Fund"),
        Order::DrawVenture { amount } => format!("Withdraw {amount} Ducats from the Venture Capital Fund"),
        Order::Smear { target, amount } => format!("Smear the {} with {amount} Influence", game.seat_name(*target)),
        Order::Greenwash { amount } => format!("Greenwash with {amount} Influence and {} Ducats", amount * game.tables.influence.greenwash.ducats_per_influence),
        Order::Agitate { state } => format!("Agitate in {}", game.tables.state(*state).name),
        Order::OfferCredits { ppm } => format!("Offer {ppm} ppm of carbon credit a turn"),
        Order::BuyCredits { ppm } => format!("Request {ppm} ppm of carbon credit from the Custodians"),
        // Ticket #52.
        Order::Relief { state } => format!("Relief in {}: Unrest -1", game.tables.state(*state).name),
        Order::Resettle { state } => format!("Resettle this turn's refugees in {}", game.tables.state(*state).name),
        // Ticket #54.
        Order::Change { building, what } => format!("{} the {} at {}", what.name(), building_name(game, *building), game.place_name(building.place())),
        Order::Leapfrog { state } => format!("Leapfrog {}: its people emit {:.2} less per hundred million", game.tables.state(*state).name, game.tables.climate.population_emissions_per_level * Game::UNITS_PER_HUNDRED_MILLION),
        Order::StripPermit { state } => format!("Strip Permit in {}: three turns of double output", game.tables.state(*state).name),
        Order::ExodusCall { state } => format!("Exodus Call in {}: a doubled muster at the ordinary cost in people", game.tables.state(*state).name),
        // Ticket #192 (version 0.08.0): the Upload.
        Order::Upload { colony, n } => format!("Upload {} Colonists into the Archive at {}", n, game.place_name(Place::Colony(*colony))),
    }
}

/// Ticket #54: what a Mothball, Restart or Decommission order is aimed at, by name.
fn building_name(game: &Game, b: BuildingRef) -> String {
    match b {
        BuildingRef::Facility(sid, i) => game.state(sid).facilities.get(i).map(|f| f.kind.name().to_string()).unwrap_or_else(|| "building".into()),
        BuildingRef::Module(cid, i) => game
            .colony(cid)
            .and_then(|c| c.modules.get(i))
            .map(|m| m.kind.name().to_string())
            .unwrap_or_else(|| "building".into()),
    }
}


fn change_row(ui: &mut Ui, game: &Game, pending: &[Order], b: BuildingRef, mothballed: bool, change: Option<PendingChange>, actions: &mut Vec<Action>) {
    if let Some(c) = change {
        ui.label(RichText::new(format!("    {} ordered, lands at turn {}'s Resolution", c.what.name(), c.due_turn)).weak());
        return;
    }
    ui.horizontal(|ui| {
        ui.add_space(16.0);
        change_buttons(ui, game, pending, b, mothballed, false, actions);
    });
}

/// Ticket #138 (version 0.07.3): the two buttons a standing building carries -- Mothball (Restart
/// once it is mothballed) and Decommission. `reversed` lays them out for a right-to-left row, which
/// adds from the right, so that they still read Mothball then Decommission.
fn change_buttons(ui: &mut Ui, game: &Game, pending: &[Order], b: BuildingRef, mothballed: bool, reversed: bool, actions: &mut Vec<Action>) {
    let wanted = if mothballed { BuildingChange::Restart } else { BuildingChange::Mothball };
    let mut pair = [wanted, BuildingChange::Decommission];
    if reversed {
        pair.reverse();
    }
    for what in pair {
        cost_button(ui, game, pending, Order::Change { building: b, what }, what.name(), actions);
    }
}

/// Ticket #138 (version 0.07.3): the room a building's name line needs to its right for its two
/// buttons. With less than this the buttons go beneath the line as they did before, so a narrowed
/// panel degrades to the old shape rather than to clipped buttons.
const CHANGE_BUTTONS_WIDTH: f32 = 196.0;

/// Ticket #116 (version 0.07.1): the rule for what gets a tooltip, so the next person has a test
/// to apply rather than a list to extend. The designer: *"increase the use of mouse over tooltips."*
///
/// **Anything showing a bare number that a rule governs earns a tooltip naming the rule.** Not
/// restating the number, which is already on screen -- naming the rule behind it: what sets it, what
/// it does at its thresholds, what happens if it runs out. A figure with no rule behind it (a name,
/// a count of things you can see) earns nothing, and a rule with no figure on screen belongs in the
/// prose, not under the pointer.
///
/// **A tooltip may run to a short list and no further.** Six lines is the ceiling; the Income
/// breakdown is about that long and still works. Anything longer covers the thing it explains, which
/// makes it a worse tooltip than none, and belongs on the card.
fn rule_tip(response: egui::Response, text: String) -> egui::Response {
    // `tip:<word>` (a building aid, not part of the spec): the first tooltip whose text contains
    // that word is shown WITHOUT a hover, so a headless picture can be taken of one. A tooltip is
    // otherwise unreachable in a shot: the window sits off-screen and no pointer ever enters it,
    // which would leave every tooltip in the game unlooked-at.
    if let Some(word) = std::env::args().find_map(|a| a.strip_prefix("tip:").map(str::to_owned))
        && text.contains(&word)
    {
        // Once a frame: a card of twelve build buttons all match "Ready", and twelve tooltips at
        // once is a picture of nothing.
        let pass = response.ctx.cumulative_pass_nr();
        let fired: Option<u64> = response.ctx.data(|d| d.get_temp(egui::Id::new("tip_fired")));
        if fired != Some(pass) {
            response.ctx.data_mut(|d| d.insert_temp(egui::Id::new("tip_fired"), pass));
            response.show_tooltip_ui(|ui| hover_with_icons(ui, &text));
            return response;
        }
    }
    response.on_hover_ui(|ui| hover_with_icons(ui, &text))
}

/// Ticket #153 (version 0.07.4): `rule_tip` for a tooltip that DRAWS rather than says -- the
/// Emissions history. `word` is what the `tip:` aid matches against, so the hover can be
/// photographed headlessly like any other; the same once-a-frame guard applies.
fn rule_tip_ui(response: egui::Response, word: &str, add: impl Fn(&mut Ui)) -> egui::Response {
    if let Some(wanted) = std::env::args().find_map(|a| a.strip_prefix("tip:").map(str::to_owned))
        && word.contains(&wanted)
    {
        let pass = response.ctx.cumulative_pass_nr();
        let fired: Option<u64> = response.ctx.data(|d| d.get_temp(egui::Id::new("tip_fired")));
        if fired != Some(pass) {
            response.ctx.data_mut(|d| d.insert_temp(egui::Id::new("tip_fired"), pass));
            response.show_tooltip_ui(|ui| add(ui));
            return response;
        }
    }
    response.on_hover_ui(|ui| add(ui))
}

/// Ticket #112 (version 0.07.1): a figure's glyph BESIDE its word, which is the rule everywhere
/// except the top bar. It reaches the art through the egui context, so a call site deep in a panel
/// does not have to be handed an `Icons` to draw one. Where the art is missing the line is
/// unchanged, so nothing is ever lost -- only unillustrated.
fn icon_word(ui: &mut Ui, key: &str, text: impl Into<String>) -> egui::Response {
    let text = text.into();
    match Icons::from_ctx(ui.ctx(), key, 15.0) {
        Some(image) => {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                ui.add(image);
                ui.label(text);
            })
            .response
        }
        None => ui.label(text),
    }
}

/// Ticket #132 (version 0.07.3): one part of a `glyph_row` -- words before the glyph, the glyph,
/// words after it, and the phrase the glyph replaced on hover.
struct RowPart {
    before: String,
    icon: Option<&'static str>,
    after: String,
    hover: Option<String>,
}

/// Ticket #132 (version 0.07.3): a row of parts separated by a middle dot, each hugging its own
/// glyph and carrying its own hover: `Output x1 · [chimney] x0.75 · [flask] x1.25 · [horn] x1.2`
/// on a Faction card, `leans [cart]` on the start globe's Region panel. This is the one place the
/// glyph rule bends: here the glyph HEADS a multiplier instead of following a number (see
/// `draw_with_icons`), because four cards side by side are read by comparison, glyph under glyph,
/// and `Research x1.25` under `Research x0.75` is a word to read where a glyph is a shape to match.
/// The designer chose the compact row over the same line in words. Where the art is missing the
/// part's words stand alone, so nothing is lost -- only unillustrated.
fn glyph_row(ui: &mut Ui, parts: &[RowPart], size: f32) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 5.0;
        let font = egui::TextStyle::Body.resolve(ui.style());
        for (i, p) in parts.iter().enumerate() {
            // A part wraps as ONE piece, its glyph hugging its words. A nested `horizontal` is
            // laid out at the cursor and simply overruns the edge (the first picture had the
            // Arkwrights' last part crossing into the Archivists' card), so the part is measured
            // first and the row broken before it when it will not fit; a part that opens a new
            // line takes no separator, since a dot at a line's start reads as a bullet.
            let width = |s: &str| if s.is_empty() { 0.0 } else { ui.painter().layout_no_wrap(s.to_owned(), font.clone(), Color32::WHITE).size().x + 4.0 };
            let need = width(&p.before) + width(&p.after) + if p.icon.is_some() { size + 4.0 } else { 0.0 } + if i > 0 { width("·") + 5.0 } else { 0.0 };
            let mut separate = i > 0;
            if i > 0 && need > ui.available_size_before_wrap().x {
                ui.end_row();
                separate = false;
            }
            if separate {
                ui.label(RichText::new("·").weak());
            }
            let response = ui
                .horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    if !p.before.is_empty() {
                        ui.label(&p.before);
                    }
                    if let Some(image) = p.icon.and_then(|k| Icons::from_ctx(ui.ctx(), k, size)) {
                        ui.add(image);
                    }
                    if !p.after.is_empty() {
                        ui.label(&p.after);
                    }
                })
                .response;
            if let Some(h) = &p.hover {
                // Through `rule_tip`, so the `tip:<word>` aid can photograph it.
                rule_tip(response, h.clone());
            }
        }
    });
}

/// Ticket #106 (version 0.07.0): hover text with the resource words replaced by their glyphs. The
/// designer's rule is that the icons are used EXCLUSIVELY on mouse-overs -- the words stay
/// everywhere a player reads at a glance, and the tooltips, which are the wordiest thing in the
/// interface, trade them for pictures. Where an icon is missing the word comes straight back.
fn hover_with_icons(ui: &mut Ui, text: &str) {
    ui.set_max_width(360.0);
    text_with_icons(ui, text, 14.0, Color32::from_rgb(225, 220, 210));
}

/// The eight figures that have a glyph, in both the spellings the game's prose uses. The five
/// resources and Influence are capitalised as defined terms; population and emissions are written
/// in lower case mid-sentence, so both forms have to be looked for.
/// Ticket #332 (version 0.09.0): and Widgets, the eleventh, so `8 Widgets` wears the cog.
const ICON_WORDS: [(&str, &str); 11] = [
    ("Materials", "materials"),
    ("Widgets", "widgets"),
    ("Fuel", "fuel"),
    ("Energy", "energy"),
    ("Research", "research"),
    ("Ducats", "ducats"),
    ("Population", "population"),
    ("Influence", "influence"),
    ("Emissions", "emissions"),
    ("population", "population"),
    ("emissions", "emissions"),
];

/// Ticket #112 (version 0.07.1): a line of text with every figure's word traded for its glyph.
/// Version 0.07.0 used this on tooltips alone; the designer has now pointed it at the two densest
/// lists in the game -- a Region's Facilities and a Colony's Modules -- where `+16 Energy, 2
/// Energy upkeep, 1.9 Emissions` is the line a player actually compares two buildings across, and
/// where the words are most of the width. Where an icon is missing the word comes straight back, so
/// a failed load costs legibility and nothing else.
fn text_with_icons(ui: &mut Ui, text: &str, size: f32, tint: Color32) -> egui::Response {
    draw_with_icons(ui, text, size, tint, &[])
}

/// A dense list line. It reads the same rule every other line does -- see `draw_with_icons` -- and
/// exists only to carry the `extra` words below.
///
/// `extra` carries words that mean a figure HERE and nowhere else. The Blame block is the whole
/// reason it exists: there "ppm" is the Emissions a Faction is answerable for, while four lines
/// higher the same three letters are the CO2 Stock and a Scrubber's pull on the Natural Sink, and a
/// chimney against either of those would be a lie.
fn figures_with_icons(ui: &mut Ui, text: &str, size: f32, tint: Color32, extra: &[(&str, &str)]) -> egui::Response {
    draw_with_icons(ui, text, size, tint, extra)
}

/// **The one rule for turning a word into a glyph, everywhere in the game.** A word is traded for
/// its glyph ONLY where it names a figure, which is to say only directly after a number.
///
/// It arrived in two halves. Version 0.07.0 swapped a resource word anywhere it stood, which was
/// right for the tooltips it was written for; ticket #112 pointed that at the Facility list and it
/// ate the word out of a building's own NAME -- "Research Lab (inland): +2 Research" came out as
/// "[microscope] Lab (inland): +2 [microscope]" -- so lists took a narrower rule and prose kept the
/// old one. Ticket #116 then found the other half of the same fault: in prose where a resource is a
/// sentence's SUBJECT, "Fuel goes on transits" came out as a jerrycan and a verb. The designer's
/// answer was to narrow everything to the list rule, so there is now one rule and no flag.
fn draw_with_icons(ui: &mut Ui, text: &str, size: f32, tint: Color32, extra: &[(&str, &str)]) -> egui::Response {
    // Ticket #116 (version 0.07.1): the row's own response comes back, so a caller can hang a
    // tooltip on a whole line of glyphs and figures.
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 3.0;
        let mut previous_was_a_figure = false;
        for token in text.split(' ') {
            // Keep whatever punctuation rides on the word, so "30 Materials," still reads. Ticket
            // #132 (version 0.07.3): a closing bracket rides too, so "(44 Research)" on a Faction
            // card and "(a Colony Ship 25 Materials)" take their glyphs.
            let bare = token.trim_end_matches([',', '.', ';', ':', ')']);
            let tail = &token[bare.len()..];
            let allowed = previous_was_a_figure;
            // A figure is a token ending in a digit that does NOT end its sentence. The full stop is
            // what separates "1.9 Emissions" in a Facility list, where the glyph belongs, from "Tank
            // 9 of 30. Fuel goes on transits", where it would leave a jerrycan standing as the
            // subject of a verb.
            previous_was_a_figure = bare.ends_with(|c: char| c.is_ascii_digit()) && !token.ends_with(['.', ';', ':']);
            match ICON_WORDS
                .iter()
                .chain(extra.iter())
                .find(|(w, _)| allowed && *w == bare)
                .and_then(|(_, key)| Icons::from_ctx(ui.ctx(), key, size))
            {
                Some(image) => {
                    if tail.is_empty() {
                        ui.add(image);
                    } else {
                        // The comma belongs to the glyph and must hug it. egui applies item_spacing
                        // AFTER a widget, so the gap to close is the one the IMAGE leaves behind,
                        // not the one before the punctuation -- the first attempt set it the other
                        // way round and produced "+16 [bolt] ,1.9".
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.add(image);
                        ui.spacing_mut().item_spacing.x = 3.0;
                        ui.colored_label(tint, tail);
                    }
                }
                None => {
                    ui.colored_label(tint, token);
                }
            }
        }
    })
    .response
}

fn cost_button(ui: &mut Ui, game: &Game, pending: &[Order], order: Order, label: &str, actions: &mut Vec<Action>) {
    cost_button_with_hover(ui, game, pending, order, label, None, actions);
}

/// Ticket #309 (version 0.08.7): the hover on a button that attacks a place -- a march on a
/// Region, a landing at a Colony -- naming who defends it and what the odds figure is a chance
/// OF, at the designer's word: the place and its holder; a line per defender with its name, its
/// strength, what it defends at (dug in noted) and its damage; the first-exchange chance with the
/// two strengths it was made from; and the cost-and-stance line. Six lines at most: three
/// defenders are named, and past three, two are named and the rest counted. The presentation
/// review's finding was that a player saw "61%" with no way to learn what defended or at what.
fn attack_hover(game: &Game, place: Place, name: &str, attacker: i64, landing: bool) -> String {
    let control = match place {
        Place::State(sid) => game.state(sid).control,
        Place::Colony(cid) => game.colonies.iter().find(|c| c.id == cid).map(|c| c.control).unwrap_or(Control::Neutral),
    };
    let holder = match control {
        Control::Neutral => if matches!(place, Place::State(_)) { "neutral".to_string() } else { "nobody's".to_string() },
        Control::Controlled(Seat(0)) => "held by you".to_string(),
        Control::Controlled(s) => format!("held by the {}", game.seat_name(s)),
        Control::Occupied { occupier, .. } => format!("occupied by the {}", game.seat_name(occupier)),
    };
    let defenders: Vec<&Army> = game.defenders_at(place, Seat(0)).iter().filter_map(|id| game.army(*id)).collect();
    let arrives = if landing { "The Army attacks as it lands" } else { "The Army arrives on Attack" };
    if defenders.is_empty() {
        return format!("{name}: {holder}, undefended. {arrives} and the Occupation begins.");
    }
    let total: i64 = defenders.iter().map(|a| game.army_defended_strength(a)).sum();
    let named = if defenders.len() > 3 { 2 } else { defenders.len() };
    let mut lines = vec![format!("{name}: {holder}.")];
    for a in &defenders[..named] {
        let dug = if game.army_dug_in(a) { " (dug in)" } else { "" };
        lines.push(format!("defended by {}: strength {}, defends at {}{dug}, damage {}/{}", game.army_name(a), game.army_strength(a), game.army_defended_strength(a), a.damage, game.army_hit_points(a)));
    }
    if named < defenders.len() {
        let rest: i64 = defenders[named..].iter().map(|a| game.army_defended_strength(a)).sum();
        lines.push(format!("and {} more, defending at {rest} in all", defenders.len() - named));
    }
    lines.push(format!("{:.0}% is the chance to win the first exchange: your {attacker} against their {total}.", first_round_odds(attacker, total) * 100.0));
    lines.push(format!("{} costs nothing; {}.", if landing { "Landing" } else { "Moving" }, if landing { "the Army attacks as it lands" } else { "the Army arrives on Attack" }));
    lines.join("\n")
}

/// Ticket #121 (version 0.07.2): a button whose price is **figures and glyphs**, not words in
/// parentheses -- `Factory  25 [cart]` where it read `Factory (25 Materials)` -- and whose price is
/// simply absent when there is none, so a Mothball is a plain verb and not `Mothball (free)`. The
/// designer: *"no (25 materials) just 25 ICON"* and *"remove '(free)' in all cases that don't
/// refer to build slots."* The figure carries **no sign**: every other figure on the card is
/// unsigned and the glyph already says it is a cost.
///
/// egui's own `Button` cannot hold an image mid-text, so a priced button is a clickable group drawn
/// in the button's own visuals -- `UiBuilder::sense` gives the group a response and the style's
/// `interact` gives it the hover and press colours a button has -- and a free one is a plain
/// `Button`, which is exactly what it should look like.
///
/// Ticket #332 (version 0.09.0): `widgets` is the build's Widget figure, drawn after the price as
/// `8 [cog]` -- the second half of what a build costs, on the face beside the first -- and nought
/// for an order that builds nothing.
fn priced_button(ui: &mut Ui, enabled: bool, label: &str, cost: &dying_earth_engine::Cost, widgets: u32) -> egui::Response {
    let parts: Vec<(&str, i64)> = [("materials", cost.materials), ("fuel", cost.fuel), ("energy", cost.energy), ("influence", cost.influence), ("ducats", cost.ducats), ("widgets", widgets as i64)]
        .into_iter()
        .filter(|(_, n)| *n > 0)
        .collect();
    if parts.is_empty() {
        return ui.add_enabled(enabled, egui::Button::new(label));
    }
    ui.add_enabled_ui(enabled, |ui| {
        ui.scope_builder(egui::UiBuilder::new().sense(egui::Sense::click()), |ui| {
            let resp = ui.response();
            let visuals = *ui.style().interact(&resp);
            egui::Frame::new()
                .inner_margin(egui::Margin::symmetric(6, 3))
                .corner_radius(visuals.corner_radius)
                .fill(visuals.weak_bg_fill)
                .stroke(visuals.bg_stroke)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 4.0;
                        ui.label(RichText::new(label).color(visuals.text_color()));
                        for (key, n) in &parts {
                            ui.label(RichText::new(n.to_string()).color(visuals.text_color()));
                            match Icons::from_ctx(ui.ctx(), key, 14.0) {
                                Some(image) => {
                                    ui.add(image);
                                }
                                None => {
                                    ui.label(RichText::new(*key).color(visuals.text_color()));
                                }
                            }
                        }
                    });
                });
        })
        .response
    })
    .inner
}

/// Ticket #332 (version 0.09.0): the place and the item a build order raises -- a Facility, an
/// Industry raise, a Module, the Archive, a Ship, an Army -- and whether it is the outright buy
/// in Ducats. `None` for an order that builds nothing -- a Mothball, a Relief -- which then gets
/// no cost line and no cog on its face. (Ticket #121 read a flat turn count off the row here;
/// a build has none any more.)
fn build_item_of(order: &Order) -> Option<(Place, BuildItem, bool)> {
    match order {
        Order::BuildFacility { state, kind } => Some((Place::State(*state), BuildItem::Facility(*kind), false)),
        Order::BuildFacilityWithDucats { state, kind } => Some((Place::State(*state), BuildItem::Facility(*kind), true)),
        Order::RaiseIndustry { state } => Some((Place::State(*state), BuildItem::IndustryLevel, false)),
        Order::BuildModule { colony, kind } => Some((Place::Colony(*colony), BuildItem::Module(*kind), false)),
        Order::BuildModuleWithDucats { colony, kind } => Some((Place::Colony(*colony), BuildItem::Module(*kind), true)),
        Order::BuildArchive { colony } => Some((Place::Colony(*colony), BuildItem::Module(ModuleKind::Archive), false)),
        Order::BuildShip { site, kind } => Some((*site, BuildItem::Unit(*kind), false)),
        Order::BuildArmy { place } => Some((*place, BuildItem::Unit(UnitKind::Army), false)),
        _ => None,
    }
}

/// Ticket #332: the first line of a build button's hover, in the shape the designer set --
/// `20 Materials, 8 Widgets, about 2 turns here` -- the Materials at this seat's price, the
/// Widget figure at its discount, and the estimate at this place's Widgets a turn behind
/// everything already in its queue (`Game::turns_to_build`). An outright buy in Ducats completes
/// at the next Resolution ahead of the queue whatever its figure, and says so instead.
fn build_words(game: &Game, order: &Order) -> Option<String> {
    let (place, item, ducats) = build_item_of(order)?;
    let cost = game.order_cost(Seat(0), order);
    if ducats {
        return Some(format!("{} Ducats, ready at the next Resolution ahead of the queue", cost.ducats));
    }
    let widgets = game.build_widgets(Seat(0), item);
    let when = match game.turns_to_build(Seat(0), place, item) {
        u32::MAX => "nothing here makes Widgets, so it would never finish".to_string(),
        1 => "ready next turn here".to_string(),
        n => format!("about {n} turns here"),
    };
    Some(format!("{} Materials, {widgets} Widgets, {when}", cost.materials))
}

/// Ticket #332: an estimate in words, for a queue line, a hatched tile and the Under way block
/// alike: `about 2 turns`, `ready next turn`, or the one case a rate of nought gives.
fn estimate_words(turns: u32) -> String {
    match turns {
        u32::MAX => "nothing here makes Widgets".to_string(),
        1 => "ready next turn".to_string(),
        n => format!("about {n} turns"),
    }
}

/// Ticket #332: every place a seat directs -- its Regions, then its Colonies and stations -- in
/// the order the board lists them.
fn directed_places(game: &Game, seat: Seat) -> Vec<Place> {
    game.directed_states(seat).into_iter().map(Place::State).chain(game.directed_colonies(seat).into_iter().map(Place::Colony)).collect()
}

/// Ticket #332: the makers behind a place's Widgets a turn, each named with its figure -- the base
/// (a Region's Industry Level, a Colony's Core Module) and every working Factory -- read off the
/// same yields `Game::widgets_at` sums, so the parts on the hover always add up to the line.
fn widget_makers(game: &Game, place: Place) -> Vec<(String, i64)> {
    let mut parts: Vec<(String, i64)> = Vec::new();
    match place {
        Place::State(sid) => {
            let st = game.state(sid);
            parts.push((format!("Industry Level {}", st.industry_level), st.industry_level as i64 * game.tables.widgets.per_industry_level as i64));
            if let Some(seat) = st.control.director() {
                for f in st.facilities.iter().filter(|f| f.working()) {
                    let y = game.facility_yield(seat, sid, f.kind);
                    if y.resource == Some(dying_earth_engine::Resource::Widgets) {
                        parts.push((format!("the {}", f.kind.name()), y.amount));
                    }
                }
            }
        }
        Place::Colony(cid) => {
            let Some(col) = game.colony(cid) else { return parts };
            let Some(seat) = col.control.director() else { return parts };
            if col.grid_failed || game.starved_by(cid).is_some() {
                return parts;
            }
            for (i, m) in col.modules.iter().enumerate() {
                if !m.working() {
                    continue;
                }
                let y = game.module_yield_at(seat, cid, i);
                if y.resource == Some(dying_earth_engine::Resource::Widgets) {
                    let name = if y.doubled_by.is_some() { format!("the {}, doubled", m.kind.name()) } else { format!("the {}", m.kind.name()) };
                    parts.push((name, y.amount));
                }
            }
        }
    }
    parts
}

/// Ticket #332: one queue line, `Habitat 3 of 8, about 2 turns`, and who began it when that was
/// not the place's present director -- a build left behind when the place changed hands.
fn queue_line(game: &Game, place: Place, b: &Build, turns: u32) -> String {
    let whose = if game.place_director(place) == Some(b.seat) { String::new() } else { format!(", begun by the {}", game.seat_name(b.seat)) };
    format!("{} {} of {}, {}{whose}", b.item.name(), b.done, b.widgets, estimate_words(turns))
}

/// Ticket #332: **the Widgets block on a card**, Region, Colony and station alike, at the
/// designer's word: `Widgets 6 a turn`, the base and each maker named on its hover, and the
/// queue under it, each item `Habitat 3 of 8` with its estimate. The queue is drawn on the tiles
/// too; here it is in order, which the tiles cannot say.
fn widgets_block(ui: &mut Ui, game: &Game, place: Place) {
    let rate = game.widgets_at(place);
    let makers = widget_makers(game, place);
    let breakdown = if makers.is_empty() { "nothing here makes any".to_string() } else { makers.iter().map(|(name, n)| format!("{n} from {name}")).collect::<Vec<_>>().join(", ") };
    let hover = format!(
        "Widgets {rate} a turn here: {breakdown}.\nA Widget is one unit of work. Every build carries a Widget figure and completes at the Resolution its count reaches it; each turn this place's Widgets fill the earliest order under way here first and flow on to the next. What is not applied is lost: Widgets are never banked, traded or carried."
    );
    rule_tip(icon_word(ui, "widgets", format!("Widgets {rate} a turn")), hover);
    for (b, turns) in game.queue_at(place).iter().zip(game.queue_estimates(place)) {
        ui.label(format!("  {}", queue_line(game, place, b, turns)));
    }
}

/// Ticket #332: the top bar's Widgets hover -- the two figures explained, then a line per place
/// the player directs with its rate, its makers and its queue.
fn widgets_bar_hover(game: &Game, made: i64, applied: i64) -> String {
    let mut lines = vec![format!(
        "Widgets: {made} made a turn across the places you direct, {applied} applied at the last Resolution.\nA Widget is one unit of work, the second half of every build's price. Each place's Widgets go that same turn to the builds under way at that place, earliest order first, and what is not applied is lost: never banked, never traded."
    )];
    for place in directed_places(game, Seat(0)) {
        let rate = game.widgets_at(place);
        let makers = widget_makers(game, place);
        let breakdown = if makers.is_empty() { "nothing makes any".to_string() } else { makers.iter().map(|(name, n)| format!("{n} from {name}")).collect::<Vec<_>>().join(", ") };
        let queue: Vec<String> = game.queue_at(place).iter().zip(game.queue_estimates(place)).map(|(b, t)| queue_line(game, place, b, t)).collect();
        let queue = if queue.is_empty() { "nothing under way".to_string() } else { queue.join("; ") };
        lines.push(format!("{}: {rate} a turn ({breakdown}). {queue}.", game.place_name(place)));
    }
    lines.join("\n")
}

/// Ticket #332: the hover on a hatched tile, Region or Colony alike, and the cancel a right-click
/// on it gives. `3 of 8 Widgets, about 2 turns at this place's rate`; a build another seat began
/// at a place the player now directs -- left behind when it changed hands -- may be cancelled,
/// its Materials at the player's own price to their Stockpile (the designer: *a conquest is a
/// prize*), and the hover says so with the figure. The player's own build under way is not
/// cancellable, as it never was: only this turn's order is, from its own tile.
#[allow(clippy::too_many_arguments)]
fn building_tip(game: &Game, session: &Session, place: Place, index: usize, b: &Build, turns: u32, mine: bool, heading: &str, tail: &str) -> (String, Option<Order>) {
    let mut tip = format!("{heading}: building, {} of {} Widgets, {} at this place's rate.{tail}", b.done, b.widgets, estimate_words(turns));
    if b.seat == Seat(0) {
        return (tip, None);
    }
    tip.push_str(&format!("\nBegun by the {}", game.seat_name(b.seat)));
    if !mine {
        tip.push('.');
        return (tip, None);
    }
    let order = Order::CancelBuild { place, index };
    let refund = game.cancel_refund(Seat(0), place, index);
    if session.pending.iter().any(|o| matches!(o, Order::CancelBuild { place: p, index: i } if *p == place && *i == index)) {
        tip.push_str(&format!(". Cancelled this turn: {refund} Materials to your Stockpile at End Turn."));
        return (tip, None);
    }
    match game.check_order(Seat(0), &session.pending, &order) {
        Ok(_) => {
            tip.push_str(&format!(", who paid for it. Right-click to cancel it: {refund} Materials to your Stockpile at your own price, and its Widgets so far are lost."));
            (tip, Some(order))
        }
        Err(e) => {
            tip.push_str(&format!(". Cannot be cancelled: {}.", e.0));
            (tip, None)
        }
    }
}

/// A build button: the price on it in glyphs, and on hover how long it takes and what it would
/// make each turn (#22). Ticket #121 (version 0.07.2): the hover no longer repeats the price --
/// *"that's already stated"* -- and *"Once it stands"* became the turn count: `Ready next turn:` or
/// `Ready in 2 turns:`, from the building's own card. Eight of the ten Facilities take one turn.
/// Ticket #322 (version 0.08.8): a button that places SEVERAL orders at once -- a stack's march,
/// a stack's transit, a Repair all -- at the designer's word that Armies stack for orders. It is
/// enabled when every order it holds is legal on its own, priced at their sum, and its refusal is
/// the first order's, since a stack is refused for one reason at a time.
fn orders_button(ui: &mut Ui, game: &Game, pending: &[Order], orders: Vec<Order>, label: &str, hover: Option<String>, actions: &mut Vec<Action>) {
    let mut cost = dying_earth_engine::Cost::default();
    let mut refusal: Option<String> = None;
    for o in &orders {
        let c = game.order_cost(Seat(0), o);
        cost.materials += c.materials;
        cost.fuel += c.fuel;
        cost.energy += c.energy;
        cost.ducats += c.ducats;
        cost.influence += c.influence;
        if refusal.is_none() && let Err(e) = game.check_order(Seat(0), pending, o) {
            refusal = Some(e.0);
        }
    }
    let ok = refusal.is_none() && !orders.is_empty();
    let mut resp = priced_button(ui, ok, label, &cost, 0);
    if let Some(h) = &hover {
        resp = rule_tip(resp, h.clone()).on_disabled_hover_ui(|ui| hover_with_icons(ui, h));
    }
    if let Some(e) = &refusal {
        resp = rule_tip(resp, e.clone());
    }
    if resp.clicked() && ok {
        for o in orders {
            actions.push(Action::Place(o));
        }
    }
}

fn cost_button_with_hover(ui: &mut Ui, game: &Game, pending: &[Order], order: Order, label: &str, hover: Option<String>, actions: &mut Vec<Action>) {
    let cost = game.order_cost(Seat(0), &order);
    let check = game.check_order(Seat(0), pending, &order);
    // Ticket #332 (version 0.09.0): the Widget figure on the face after the price, and the hover's
    // first line the Materials, the Widgets and the estimate at this place's rate behind its queue.
    let widgets = build_item_of(&order).filter(|(_, _, ducats)| !ducats).map(|(_, item, _)| game.build_widgets(Seat(0), item)).unwrap_or(0);
    let mut resp = priced_button(ui, check.is_ok(), label, &cost, widgets);
    let ready = build_words(game, &order);
    let whole = match (&ready, &hover) {
        (Some(r), Some(h)) => Some(format!("{r}.\nOnce it stands: {h}")),
        (None, Some(h)) => Some(h.clone()),
        (Some(r), None) => Some(format!("{r}.")),
        (None, None) => None,
    };
    if let Some(whole) = whole {
        // Through rule_tip, so the tip:<word> aid can photograph a build hover too.
        let b = whole.clone();
        resp = rule_tip(resp, whole).on_disabled_hover_ui(move |ui| hover_with_icons(ui, &b));
    }
    if let Err(e) = &check {
        // Ticket #238 (version 0.08.3): through `rule_tip`, so a REFUSAL can be photographed like
        // any other tooltip. It could not be before -- a plain `on_disabled_hover_text` needs a
        // pointer, and the shot window never has one -- which left every refusal in the game
        // unlookable-at, this version's three-turn rule among them.
        rule_tip(resp.clone(), e.0.clone());
    }
    if resp.clicked() {
        actions.push(Action::Place(order));
    }
}

fn stance_row(ui: &mut Ui, game: &Game, pending: &[Order], current: Stance, make: impl Fn(Stance) -> Order, ships: bool, actions: &mut Vec<Action>) {
    ui.horizontal(|ui| {
        ui.label("Stance:");
        // Ticket #278 (version 0.08.5): Blockade, Ships only, beside Intercept.
        // Ticket #297 (version 0.08.6): Dig In, Armies only, after Hold.
        for st in [Stance::Attack, Stance::Hold, Stance::DigIn, Stance::Intercept, Stance::Blockade, Stance::Evade] {
            if matches!(st, Stance::Intercept | Stance::Blockade) && !ships {
                continue;
            }
            if st == Stance::DigIn && ships {
                continue;
            }
            let pending_stance = pending.iter().rev().find_map(|o| match (o, &make(st)) {
                (Order::ShipStance { body, stance }, Order::ShipStance { body: b2, .. }) if body == b2 => Some(*stance),
                (Order::ArmyStance { place, stance }, Order::ArmyStance { place: p2, .. }) if place == p2 => Some(*stance),
                _ => None,
            });
            let shown = pending_stance.unwrap_or(current);
            // Ticket #313 (version 0.08.7): each label says what it does on hover, in the one
            // sentence the engine keeps for it, and the rule every stance shares; no marker (#233).
            // Ticket #319 (version 0.08.8): the Intercept label says when the computer seats use
            // it, which is measured behaviour and not a rule, at the designer's word.
            let measured = if ships && st == Stance::Intercept { "\nThe computer seats intercept with warships when an unarmed rival hull is inbound: a Colony Ship or a Carrier." } else { "" };
            let resp = rule_tip(ui.selectable_label(shown == st, st.name()), format!("{}: {}\n{}{measured}", st.name(), st.one_liner(ships), Stance::PERSISTS));
            if resp.clicked() && shown != st {
                let order = make(st);
                if game.check_order(Seat(0), pending, &order).is_ok() {
                    actions.push(Action::Place(order));
                }
            }
        }
    });
}

/// `controls` says whether the spend box, the Spend button and the Trading-window button are drawn.
/// Ticket #121 (version 0.07.2): a Region card passes `false` -- *"remove buttons to buy/spend
/// influence from nation card"* -- since the Command Cluster spends on the selected place and the
/// card's controls had become a second copy. The Standings, the threshold and the Blame note stay:
/// they are the figures a player reads before pressing Spend in the corner, and the reason they
/// clicked the country. A Colony's card keeps its controls, that not being what was asked.
fn influence_row(ui: &mut Ui, game: &Game, session: &Session, view: &mut ViewState, target: Place, controls: bool, actions: &mut Vec<Action>) {
    // Ticket #64: a spectator reads every Faction's Standing here and spends nothing.
    if session.spectator {
        let threshold = game.influence_threshold(target);
        let margin = game.tables.influence.challenge_margin;
        let explain = match game.place_control(target).controller() {
            Some(c) => format!("Held by the {}. A rival needs their Standing plus {margin}, and at least the threshold. Decays {} a turn for the holder.", game.seat_name(c), game.tables.influence.decay_controlled),
            None => format!("First to {threshold} takes it. Decays {} a turn.", game.tables.influence.decay),
        };
        standings_row(ui, game, session, target, threshold, explain);
        return;
    }
    if controls {
        // Ticket #161 (version 0.07.5): the Colony's card follows the Nation card, and its three
        // controls -- which the Nation card no longer has -- get hovers of their own.
        ui.horizontal(|ui| {
            rule_tip(
                ui.label("Influence:"),
                format!(
                    "How much of this turn's Allotment to put here. It becomes your Standing and stays, whoever holds the place after.\nUnspent Allotment is lost at End Turn. A Standing decays {} a turn for the holder, {} for everyone else.",
                    game.tables.influence.decay_controlled, game.tables.influence.decay
                ),
            );
            ui.add(egui::DragValue::new(&mut view.influence_amount).range(1..=100));
            let order = Order::Influence { target, amount: view.influence_amount };
            let ok = game.check_order(Seat(0), &session.pending, &order);
            let spend = ui.add_enabled(ok.is_ok(), egui::Button::new("Spend"));
            let needed = game.influence_needed_for(Seat(0), target);
            let mine = game.seat(Seat(0)).influence.get(&target).copied().unwrap_or(0);
            let spend = match &ok {
                Ok(_) => rule_tip(spend, format!("Adds the figure beside it to your Standing here, now {mine}. You take this place at {needed}.")),
                Err(e) => spend.on_disabled_hover_text(e.0.clone()),
            };
            if spend.clicked() {
                actions.push(Action::Place(order));
            }
            if let Err(e) = ok {
                ui.label(RichText::new(e.0).weak());
            }
        });
        // Ticket #42: buying Influence lives in the Trading window now.
        if rule_tip(
            ui.small_button("Buy more Influence in the Trading window"),
            "Ducats buy Influence into this turn's Allotment, two a point, spent like any other. Opens the Trading window.".to_string(),
        )
        .clicked()
        {
            view.show_trade = true;
        }
    }
    // Ticket #53: the threshold shown is the player's own, since Blame raises it seat by seat.
    let threshold = game.influence_threshold_for(Seat(0), target);
    // Ticket #137 (version 0.07.3): the two sentences that explained the threshold are a hover on
    // the Standings line, one sentence and a number per case. The designer: *"replace with mouse
    // over that relays the same information in far fewer words."*
    let margin = game.tables.influence.challenge_margin;
    let explain = match game.place_control(target).controller() {
        Some(Seat(0)) => format!(
            "A rival needs {}: your Standing plus {margin}, and at least the threshold. Decays {} a turn.",
            game.influence_needed_for(Seat(0), target),
            game.tables.influence.decay_controlled
        ),
        Some(_) => format!("You need {}: their Standing plus {margin}, and at least your threshold. Decays {} a turn.", game.influence_needed_for(Seat(0), target), game.tables.influence.decay),
        None => format!("First to {threshold} takes it. Decays {} a turn.", game.tables.influence.decay),
    };
    standings_row(ui, game, session, target, threshold, explain);
    threshold_breakdown(ui, game, target);
    // Ticket #53: on every Region the player does not hold, what its Blame is costing it here.
    let blame_mult = game.blame_threshold_multiplier_on(Seat(0), target);
    if blame_mult > 1.0 {
        let note = ui.label(
            RichText::new(format!(
                "Blame: your threshold here is {}, not {} (share {:.2}, x{:.2}). Emit less, or take back what you emit, and it comes down.",
                threshold,
                game.influence_threshold(target),
                game.blame_share(Seat(0)),
                blame_mult
            ))
            .color(Color32::from_rgb(255, 170, 120)),
        );
        // Ticket #161 (version 0.07.5): four rule-governed figures on one line and no hover behind
        // any of them.
        rule_tip(
            note,
            format!(
                "Blame is the CO2 you are answerable for all game: what your places emitted, less what you took back.\nYour share is {:.0} per cent, above a fair quarter, so every place you do not hold costs more to win over -- up to half again.\nThe Climate Panel breaks it down by Faction.",
                game.blame_share(Seat(0)) * 100.0
            ),
        );
    }
}

/// Ticket #190 (version 0.08.0): the breakdown of what it takes to win this place, in the designer's
/// own three-line format:
///
/// ```text
/// Threshold 40 - 20 + 20 for size 2
/// Held by the Archivists: 58 - 38 + 20 (margin) + 5 (Constabulary)
/// Your Influence converts at 0.91
/// ```
///
/// It replaces a single sentence that named neither Blame nor Green Consensus though both already
/// moved the figure. Blame and Green Consensus join the threshold line as further terms only while
/// they are BITING, which keeps the common case to three lines: measured, Blame sits at exactly
/// x1.00 in 66% of takes. The conversion line is new with Resistance (ticket #187) and belongs
/// beside the threshold rather than inside it: it prices what your SPENDING is worth here, where the
/// other two lines price the gate.
fn threshold_breakdown(ui: &mut Ui, game: &Game, target: Place) {
    let t = &game.tables.influence;
    let threshold = game.influence_threshold_for(Seat(0), target);
    let mut line = match target {
        Place::State(s) => {
            let size = game.tables.state(s).size as i64;
            format!("Threshold {threshold} - {} + {} for size {size}", t.state_threshold_base, t.state_threshold_per_size * size)
        }
        Place::Colony(c) => {
            let people = game.colony(c).map(|x| x.colonists).unwrap_or(0) as i64;
            let station = game.colony(c).map(|x| x.in_orbit).unwrap_or(false);
            let base = if station { format!("{} for the station", t.station_threshold_base) } else { "0".to_string() };
            format!("Threshold {threshold} - {base} + {} for {people} Colonists", t.colony_threshold_per_colonist * people)
        }
    };
    // Ticket #53 and ticket #43: the two multipliers that already moved this figure and never said
    // so. Shown only while they bite, so the usual card is three lines.
    let blame = game.blame_threshold_multiplier_on(Seat(0), target);
    if blame > 1.0 {
        line.push_str(&format!(", x{blame:.2} (Blame)"));
    }
    if game.research.done.contains(&TechId::GreenConsensus) {
        line.push_str(", cut by Green Consensus");
    }
    ui.label(RichText::new(line).weak());

    // The second line: what the holder's Standing and the margin make of it.
    if let Some(c) = game.place_control(target).controller() {
        let standing = game.seat(c).influence.get(&target).copied().unwrap_or(0);
        let margin = game.challenge_margin_at(target);
        let base = t.challenge_margin;
        let guard = margin - base;
        let held = if c == Seat(0) { "Held by you".to_string() } else { format!("Held by the {}", game.seat_name(c)) };
        let needed = game.influence_needed_for(Seat(0), target);
        let mut second = format!("{held}: {needed} - {standing} + {base} (margin)");
        if guard > 0 {
            second.push_str(&format!(" + {guard} (Constabulary)"));
        }
        // A holder with little Standing is protected by the THRESHOLD rather than by the margin, and
        // the line has to say so or it reads as bad arithmetic: "80 - 0 + 20" is not 80.
        if standing + margin < needed {
            second.push_str(&format!(" comes to {}, under the threshold, so the threshold stands", standing + margin));
        }
        ui.label(RichText::new(second).weak());
        // Ticket #262 (version 0.08.4): **the challenger line**, on a place the player holds: the
        // rival nearest to taking it -- nearest its OWN price, since Blame and Relations move one
        // rival's price and not another's -- and how far off it stands. The designer's sentence,
        // kept: "The Prospectors stand at 31; they take this at 54." The arithmetic rides on the
        // hover, within the six-line rule; the line carries the two figures. Held places only.
        if c == Seat(0) {
            match game.nearest_challenger(target) {
                Some((who, theirs, price)) => {
                    let name = game.seat_name(who);
                    let line = format!("The {name} stand at {theirs}; they take this at {price}.");
                    let their_threshold = game.influence_threshold_for(who, target);
                    let their_margin = game.challenge_margin_for(Some(who), target);
                    let their_blame = game.blame_threshold_multiplier_on(who, target);
                    let resistance = game.resistance(target);
                    let gap = (price - theirs).max(0);
                    let spend = ((gap as f64) * resistance).ceil() as i64;
                    // Ticket #75's warning, folded in: within two steps of the holder's Standing the
                    // line turns amber and says what to do about it.
                    let step = game.tables.ai.thresholds.influence_step;
                    let pressing = theirs + 2 * step >= standing;
                    let line = if pressing { format!("{line} Spend here to stay ahead.") } else { line };
                    let tip = format!(
                        "The {name}'s price here is the greater of their own threshold, {their_threshold}{}, and your Standing plus the margin they face, {standing} + {their_margin}.
They are {gap} short. An outsider's Influence converts at {:.2} here, so that is about {spend} Influence spent.
Spending here raises the bar; doing nothing lowers it, yours decaying {} a turn and theirs {}{}.",
                        if their_blame > 1.0 { format!(" (x{their_blame:.2} for their Blame)") } else { String::new() },
                        1.0 / resistance.max(1e-9),
                        game.standing_decay_for(Seat(0), target),
                        game.standing_decay_for(who, target),
                        // Ticket #266 (version 0.08.4): a rival's decay on a Region reads its Blame.
                        if matches!(target, Place::State(_)) && game.standing_decay_for(who, target) != game.tables.influence.decay { " for their Blame" } else { "" }
                    );
                    let colour = if pressing { Color32::from_rgb(255, 160, 60) } else { rgb(game.tables.faction(game.kind(who)).colour) };
                    rule_tip(ui.label(RichText::new(line).color(colour)), tip);
                }
                None => {
                    ui.label(RichText::new("No rival has a Standing here.").weak());
                }
            }
            // Ticket #310 (version 0.08.7): **the threat line**, the military counterpart of the
            // challenger line, on a Region the player holds: the rival raised Army next door with
            // the best first-exchange odds against this Region's defenders. Presence and strength,
            // never its stance, at the designer's word (as a Region arms stance-blind, #282); no
            // line at all when nobody stands next door; amber when their odds reach the bar the
            // computer attacks at, which is measured behaviour read from `ai.toml`, not a rule.
            if let Place::State(sid) = target
                && let Some((id, from, odds)) = game.nearest_army_threat(sid)
                && let Some(a) = game.army(id)
                && let Some(who) = game.army_seat(a)
            {
                let strength = game.army_strength(a);
                let defence: i64 = game.defenders_at(Place::State(sid), who).iter().filter_map(|d| game.army(*d)).map(|d| game.army_defended_strength(d)).sum();
                let bar = game.tables.ai.thresholds.attack_odds;
                let pressing = odds >= bar;
                let line = format!(
                    "The {}' {}, strength {strength}, stands next door in {}.{}",
                    game.seat_name(who),
                    game.army_name(a).trim_start_matches("the "),
                    game.tables.state(from).name,
                    if pressing { " Dig In here to hold it." } else { "" }
                );
                let tip = format!(
                    "Their {strength} against the {defence} that defends here: {:.0}% is their chance to win the first exchange.\nThe computer seats attack at {:.0}% or better, so the line turns amber there.\nArmies next door whatever their stance; a Region's own Army standing at home is no threat, marched out it is.",
                    odds * 100.0,
                    bar * 100.0
                );
                let colour = if pressing { Color32::from_rgb(255, 160, 60) } else { rgb(game.tables.faction(game.kind(who)).colour) };
                rule_tip(ui.label(RichText::new(line).color(colour)), tip);
            }
        }
    }

    // The third: Resistance, which taxes the spending rather than the gate.
    let convert = 1.0 / game.resistance(target).max(1e-9);
    let own = game.place_control(target).controller() == Some(Seat(0));
    ui.label(
        RichText::new(if own {
            "Your Influence converts at 1.00 here: a controller converts in full.".to_string()
        } else {
            format!("Your Influence converts at {convert:.2}")
        })
        .weak(),
    );
}

/// Ticket #50: four seats, so the Standings are chips in Faction colours, and only where there is
/// a Standing to show. Ticket #64: the spectator's cards carry the same row. Ticket #137 (version
/// 0.07.3): the row ends with `· Threshold N` and carries the whole explanation of what it takes to
/// hold or take the place as a hover on the line -- the figure (ticket #60: the engine's own, which
/// the Resolution and the AI read too) stays in view, and the reasoning is one hover away.
fn standings_row(ui: &mut Ui, game: &Game, session: &Session, target: Place, threshold: i64, explain: String) {
    let row = ui.horizontal_wrapped(|ui| {
        // Ticket #116 (version 0.07.1): what a Standing IS, which the row shows four of and never
        // explains. The decay figures are the reason a Standing left alone slides, and the reason
        // holding a place costs less than taking one.
        rule_tip(
            ui.label("Standings:"),
            format!(
                "Standing is the Influence a Faction has built up here. It persists, and is never wiped when the place changes hands.\nLeft alone it decays {} a turn for the controller and {} a turn for everybody else, so a claim you stop paying for slides faster than the holder's.",
                game.tables.influence.decay_controlled, game.tables.influence.decay
            ),
        );
        let mut any = false;
        for s in Seat::ALL {
            let v = game.seat(s).influence.get(&target).copied().unwrap_or(0);
            if v <= 0 {
                continue;
            }
            any = true;
            let chip = ui.label(RichText::new(format!(" {} {} ", game.seat_name(s), v)).color(Color32::BLACK).background_color(seat_colour(session, s)));
            // Ticket #161 (version 0.07.5): a chip of its own says whose the Standing is and how far
            // it has to go, which the line's hover -- written for the player's own case -- cannot.
            // The designer asked that the player's own chip, on a place they hold, say instead what
            // it would take to be safe from the nearest rival, which is the figure they act on.
            let holder = game.place_control(target).controller();
            let mine = s == Seat(0) && !session.spectator;
            let whose = if mine { "Your Standing here.".to_string() } else { format!("The {}' Standing here.", game.seat_name(s)) };
            let rest = if holder == Some(s) && mine {
                let margin = game.tables.influence.challenge_margin;
                let nearest = Seat(0).others().iter().map(|r| (*r, game.seat(*r).influence.get(&target).copied().unwrap_or(0))).max_by_key(|(_, n)| *n).filter(|(_, n)| *n > 0);
                match nearest {
                    Some((r, n)) => format!("A rival takes it at {}, your Standing plus {margin}. The {} are nearest at {n}, {} short. Every point you add here adds one to that.", v + margin, game.seat_name(r), (v + margin - n).max(0)),
                    None => format!("No rival has any Standing here; one would need {}, your Standing plus {margin}.", v + margin),
                }
            } else if holder == Some(s) {
                "They hold this place.".to_string()
            } else {
                let needed = game.influence_needed_for(s, target);
                let who = if mine { "You take" } else { "They take" };
                let want = if mine { "you want" } else { "they want" };
                format!("{who} this place at {needed}, so {want} {} more.", (needed - v).max(0))
            };
            rule_tip(chip, format!("{whose} {rest}"));
        }
        if !any {
            rule_tip(
                ui.label(RichText::new("nobody has any yet").weak()),
                "No Faction has spent here. The first Standing to reach the threshold takes it; two arriving level are settled by lot.".to_string(),
            );
        }
        ui.label(RichText::new("·").weak());
        // Ticket #161: what a threshold IS and what sets it. The line's hover uses the word and
        // never defines it.
        let t = &game.tables.influence;
        let from = match target {
            Place::State(s) => format!("{}, plus {} a size step; this state's size is {}.", t.state_threshold_base, t.state_threshold_per_size, game.tables.state(s).size),
            Place::Colony(_) => format!("{} a Colonist living here{}.", t.colony_threshold_per_colonist, if game.colony(match target { Place::Colony(c) => c, _ => unreachable!() }).map(|c| c.in_orbit).unwrap_or(false) { format!(", and {} for the station itself", t.station_threshold_base) } else { String::new() }),
        };
        rule_tip(
            ui.label(format!("Threshold {threshold}")),
            format!(
                "What a Standing must reach to take this place: {from}\nGreen Consensus cuts a quarter off it; your Blame adds to the one you read, on every place you do not hold.\nA held place also wants the holder's Standing plus {}.",
                t.challenge_margin
            ),
        );
    });
    rule_tip(row.response, explain);
}


/// Ticket #146 (version 0.07.3): one Facility's line -- its figures with their glyphs, the hover
/// naming the rule, and the Mothball / Restart / Decommission buttons on the line (ticket #138).
/// It was the body of the card's Facility loop; it is now the strip under the slot boxes for the
/// box that was clicked, and the row a Facility that takes no slot (a Sea Wall, a Scrubber) keeps.
/// The figures a Facility's line carries: what it makes, its upkeep and its Emissions, or the
/// sentence a mothballed or undirected one shows instead. Read by the row and, since ticket #150
/// (version 0.07.4), by the slot box's hover.
fn facility_figures(game: &Game, sid: StateId, f: &Facility, director: Option<Seat>) -> String {
    // Ticket #54: a mothballed Facility says so rather than showing figures it is not making.
    if f.mothballed {
        // Ticket #154 (version 0.07.4): a Scrubber or Sea Wall has no slot to keep.
        return format!("mothballed: making nothing, paying no upkeep, emitting nothing{}", if game.takes_slot(f.kind) { ", keeping its slot" } else { "" });
    }
    // Ticket #69: a Lab in a state nobody holds, or under Occupation, works for the world.
    let world_lab = f.kind == FacilityKind::ResearchLab && f.working() && !f.offline_until_resolution && matches!(game.state(sid).control, Control::Neutral | Control::Occupied { .. });
    // Ticket #257 (version 0.08.4): a Sea Wall says what it has held back and what that costs.
    if f.kind == FacilityKind::SeaWall {
        let yield_text = director.map(|d| game.facility_yield(d, sid, f.kind).text()).unwrap_or_else(|| "idle, nobody directs this state".to_string());
        let keep = f.rises_held as f64 * game.tables.sea_wall.upkeep_per_rise;
        let held = match f.rises_held {
            0 => "has held back no rise yet".to_string(),
            1 => format!("has held back 1 rise: {keep:.1} Materials a turn to keep"),
            n => format!("has held back {n} rises: {keep:.1} Materials a turn to keep"),
        };
        let unkept = if !f.online && !f.mothballed { "; unkept this turn, holding nothing" } else { "" };
        return format!("{yield_text}; {held}{unkept}");
    }
    match director {
        Some(d) if world_lab => format!("{} (the Lab works for the world: {} Research a turn to the Tech under research)", game.facility_yield(d, sid, f.kind).text(), game.world_lab_yield(sid) / 2),
        Some(d) => game.facility_yield(d, sid, f.kind).text(),
        None if world_lab => format!("in no one's hands: {} Research a turn to the Tech under research", game.world_lab_yield(sid) / 2),
        None => "idle, nobody directs this state".to_string(),
    }
}

/// Ticket #116 (version 0.07.1): what the two figures on a Facility's line actually DO. Upkeep
/// and Emissions are the numbers a player weighs a building by, and neither said what it cost to
/// fail to pay them. The first line is the heading the hover opens with.
fn facility_rules(heading: &str, coastal: bool) -> String {
    format!(
        "{heading}\nEnergy upkeep is paid at Income first; short of Energy, buildings go offline in order until the bill is met, and an offline one makes nothing and keeps its slot.\nIts Emissions go on the CO2 Stock every turn and on its controller's Blame.{}",
        if coastal { "\nOn the coast, the sea can take it at a threshold." } else { "" }
    )
}

#[allow(clippy::too_many_arguments)]
fn facility_row(ui: &mut Ui, session: &Session, game: &Game, sid: StateId, i: usize, f: &Facility, mine: bool, director: Option<Seat>, actions: &mut Vec<Action>) {
    let figures = facility_figures(game, sid, f, director);
    let colour = if f.mothballed { Color32::from_rgb(170, 170, 190) } else { ui.visuals().text_color() };
    // Ticket #112 (version 0.07.1): the glyphs come down into the Facility list, where the
    // figures are compared building against building and the words are most of the width.
    // Ticket #138 (version 0.07.3): the two buttons ride right-aligned on this line, after the
    // figures, and drop beneath only if the panel is too narrow. The designer: *"Mothball and
    // decommission buttons moved next to facility name not under (after yields and upkeep)."*
    let mut inline = false;
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        // Ticket #154 (version 0.07.4): coastal or inland only where the building has a slot to be
        // on; a Scrubber or Sea Wall is neither.
        let side = if !game.takes_slot(f.kind) { "" } else if f.coastal { " (coastal)" } else { " (inland)" };
        let resp = figures_with_icons(
            ui,
            // Ticket #306 (version 0.08.7): the offline suffix is the box's hover's alone now.
            &format!("{}{}: {}", f.kind.name(), side, figures),
            14.0,
            colour,
            &[],
        );
        rule_tip(resp, facility_rules(f.kind.name(), f.coastal));
        if mine && f.change.is_none() && ui.available_width() >= CHANGE_BUTTONS_WIDTH {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                change_buttons(ui, game, &session.pending, BuildingRef::Facility(sid, i), f.mothballed, true, actions);
            });
            inline = true;
        }
    });
    if mine && !inline {
        change_row(ui, game, &session.pending, BuildingRef::Facility(sid, i), f.mothballed, f.change, actions);
    }
}

/// Ticket #146 (version 0.07.3): the Facility build buttons a Region's card offers, drawn in its
/// Build section and in the strip under the slot boxes for a free box alike.
fn facility_build_buttons(ui: &mut Ui, session: &Session, game: &Game, sid: StateId, actions: &mut Vec<Action>) {
    // Ticket #181 (version 0.08.0): a Unique Facility REPLACES the common building on its Faction's
    // list, so a Prospector sees the Investment Bank where everyone else sees the Bank -- and nobody
    // sees another Faction's, a captured Region full of them included.
    let me = game.kind(Seat(0));
    for fk in FacilityKind::ALL {
        if fk.built_by(me) != fk || fk.unique_to().is_some_and(|f| f != me) {
            continue;
        }
        // Ticket #54: the Scrubber has its own button, with the state's cap on it. Ticket #154
        // (version 0.07.4): so does the Sea Wall -- neither takes a slot, so neither is offered
        // for a free box; both stand under the boxes in `no_slot_section`.
        if !game.takes_slot(fk) {
            continue;
        }
        // Ticket #56: a Facility that waits on a Tech is not offered until the Tech is in.
        if game.tables.facility(fk).needs_tech.map(|t| !game.has_tech(t)).unwrap_or(false) {
            continue;
        }
        let hover = game.facility_yield(Seat(0), sid, fk).text();
        ui.horizontal(|ui| {
            cost_button_with_hover(ui, game, &session.pending, Order::BuildFacility { state: sid, kind: fk }, fk.name(), Some(hover), actions);
            // Ticket #42: the same building bought outright for Ducats.
            cost_button(ui, game, &session.pending, Order::BuildFacilityWithDucats { state: sid, kind: fk }, "or", actions);
        });
    }
}

/// Ticket #154 (version 0.07.4): **the Facilities that take no slot** -- the Scrubber and the Sea
/// Wall -- under the boxes, in the Facilities section: each a row when it stands, a line while it
/// builds, and a build button pair when it may be built here. The designer: *"scrubber sea wall
/// need to stay but put them in the same section as the tiles just below them."* The Scrubber's
/// pair carries the state's cap; the Sea Wall's appears once Coastal Engineering is in and while
/// none stands or builds, one being the most a state may hold.
#[allow(clippy::too_many_arguments)]
fn no_slot_section(ui: &mut Ui, session: &Session, game: &Game, sid: StateId, mine: bool, director: Option<Seat>, actions: &mut Vec<Action>) {
    let st = game.state(sid);
    for (i, f) in st.facilities.iter().enumerate() {
        if !game.takes_slot(f.kind) {
            facility_row(ui, session, game, sid, i, f, mine, director, actions);
        }
    }
    // Ticket #332 (version 0.09.0): a Scrubber or Sea Wall under way is on the card's queue, under
    // its Widgets line, with every other build; the line that stood here said it twice.
    if !mine {
        return;
    }
    if game.kind(Seat(0)) == FactionKind::Custodians {
        // Ticket #280 (version 0.08.5): the hover is the row's own sentence, from the data. The one
        // written here by hand said 4 Energy upkeep for two versions while the data said 3.
        ui.horizontal(|ui| {
            cost_button_with_hover(
                ui,
                game,
                &session.pending,
                Order::BuildFacility { state: sid, kind: FacilityKind::Scrubber },
                "Scrubber",
                Some(game.facility_yield(Seat(0), sid, FacilityKind::Scrubber).text()),
                actions,
            );
            cost_button(ui, game, &session.pending, Order::BuildFacilityWithDucats { state: sid, kind: FacilityKind::Scrubber }, "or", actions);
            ui.label(RichText::new(format!("{} of {} this state may hold", game.scrubbers_committed(sid), game.scrubber_cap(sid))).weak());
        });
    }
    if game.has_tech(TechId::CoastalEngineering) {
        let standing = st.facilities.iter().any(|f| f.kind == FacilityKind::SeaWall);
        let building = st.queue.iter().any(|b| matches!(b.item, BuildItem::Facility(FacilityKind::SeaWall)));
        if !standing && !building {
            let hover = game.facility_yield(Seat(0), sid, FacilityKind::SeaWall).text();
            ui.horizontal(|ui| {
                cost_button_with_hover(
                    ui,
                    game,
                    &session.pending,
                    Order::BuildFacility { state: sid, kind: FacilityKind::SeaWall },
                    "Sea Wall",
                    // Ticket #257 (version 0.08.4): the wall stands and holds every threshold; each
                    // rise held adds to its keep; a Storm Surge it holds cuts the coast's output.
                    // Ticket #280 (version 0.08.5): all of it said by the row's own sentence now.
                    Some(hover),
                    actions,
                );
                cost_button(ui, game, &session.pending, Order::BuildFacilityWithDucats { state: sid, kind: FacilityKind::SeaWall }, "or", actions);
            });
        }
    }
}

/// Ticket #163 (version 0.07.5): the red the game puts on a button that gates the turn. End Turn
/// has worn it since the first playable; the Pick a Tech button joins it, since the turn cannot end
/// until the pick is made. Written twice as a literal before this; named once now.
const TURN_RED: Color32 = Color32::from_rgb(120, 40, 30);

/// The columns of slot boxes on a Region's card: six, since the card is a step wider than the Hab View.
const SLOT_COLS: usize = 6;
/// Ticket #152 (version 0.07.4): how long the start globe takes to turn once on its own.
const START_GLOBE_PERIOD_SECS: f32 = 75.0;
/// The coast's blue, a slot box's edge where the sea can reach it.
const COAST_EDGE: Color32 = Color32::from_rgb(90, 150, 230);

/// Ticket #146 (version 0.07.3): what one slot box on a Region's card shows.
enum SlotBoxKind {
    Standing(usize),
    /// A build under way, by its index in the Region's queue. Since ticket #332 (version 0.09.0)
    /// its kind, its count and its figure are read off the queue, its face says `3 of 8`, and a
    /// right-click on a rival's build cancels it.
    Building(usize),
    /// Ticket #291 (version 0.08.6): the kind ORDERED this turn and not yet committed, with its
    /// index in the pending list, so a right-click on the box can cancel it.
    Ordered(FacilityKind, usize),
    Free,
    Flooded(Option<FacilityKind>),
}

/// Ticket #146 (version 0.07.3): **a Region's build slots as boxes**, in the Hab View's language.
/// The designer: *"represent them as boxes inland and costal differ in line used for the box …
/// completed building will have art representing them populate the empty boxes. adding an
/// industrial level will add them. flooded ones greyed out."* Coastal boxes first, edged in the
/// coast's blue -- standing, building, free, then the ones the sea has taken, under water -- and
/// the inland boxes after, edged in grey. A click on a box puts that Facility's line in the strip
/// beneath; a click on a free box puts the build buttons there.
#[allow(clippy::too_many_arguments)]
fn slot_boxes(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, sid: StateId, mine: bool, director: Option<Seat>, actions: &mut Vec<Action>) {
    let st = game.state(sid);
    // Ticket #291 (version 0.08.6): the builds ORDERED this turn, each with the side its slot will
    // take -- decided the way the rule decides it at End Turn (`next_slot_is_coastal`, the pending
    // orders ahead of it taking theirs first), and shown as the Faction's own kind, which is what
    // the order raises (#181). Until this ticket the boxes read the queue alone, so an ordered
    // building was invisible on the card until the turn ended while the rule already counted its
    // slot as taken.
    let mut ordered: Vec<(FacilityKind, usize, bool)> = Vec::new();
    if mine {
        let (mut taken_coastal, mut taken_inland) = (0u32, 0u32);
        for (i, o) in session.pending.iter().enumerate() {
            if o.build_state() != Some(sid) {
                continue;
            }
            let Some(k) = o.build_facility().map(|k| k.built_by(game.kind(Seat(0)))) else { continue };
            if !game.takes_slot(k) {
                continue;
            }
            match game.next_slot_is_coastal(sid, k, taken_coastal, taken_inland) {
                Some(true) => {
                    taken_coastal += 1;
                    ordered.push((k, i, true));
                }
                Some(false) => {
                    taken_inland += 1;
                    ordered.push((k, i, false));
                }
                None => {}
            }
        }
    }
    let mut boxes: Vec<(SlotBoxKind, bool)> = Vec::new();
    for coastal in [true, false] {
        for (i, f) in st.facilities.iter().enumerate() {
            if f.coastal == coastal && game.takes_slot(f.kind) {
                boxes.push((SlotBoxKind::Standing(i), coastal));
            }
        }
        for (qi, b) in st.queue.iter().enumerate() {
            if let BuildItem::Facility(k) = b.item
                && b.coastal == coastal
                && game.takes_slot(k)
            {
                boxes.push((SlotBoxKind::Building(qi), coastal));
            }
        }
        let ordered_here = ordered.iter().filter(|(_, _, c)| *c == coastal).count() as u32;
        for (k, i, _) in ordered.iter().filter(|(_, _, c)| *c == coastal) {
            boxes.push((SlotBoxKind::Ordered(*k, *i), coastal));
        }
        let free = if coastal { game.coastal_slots(sid).saturating_sub(game.coastal_used(sid)) } else { game.inland_slots(sid).saturating_sub(game.inland_used(sid)) };
        for _ in 0..free.saturating_sub(ordered_here) {
            boxes.push((SlotBoxKind::Free, coastal));
        }
        if coastal {
            for k in 0..st.lost_slots as usize {
                boxes.push((SlotBoxKind::Flooded(st.drowned.get(k).copied()), true));
            }
        }
    }
    let rows = boxes.len().div_ceil(SLOT_COLS).max(1);
    let grid_size = egui::vec2(SLOT_COLS as f32 * HAB_TILE + (SLOT_COLS as f32 - 1.0) * HAB_GAP, rows as f32 * (HAB_TILE + HAB_LABEL + HAB_GAP));
    let (grid, _) = ui.allocate_exact_size(grid_size, egui::Sense::hover());
    // Ticket #332 (version 0.09.0): the estimate for each build in the queue, at this Region's
    // Widgets a turn behind everything ahead of it.
    let estimates = game.queue_estimates(Place::State(sid));
    for (n, (kind, coastal)) in boxes.iter().enumerate() {
        let (c, r) = (n % SLOT_COLS, n / SLOT_COLS);
        let rect = egui::Rect::from_min_size(grid.min + egui::vec2(c as f32 * (HAB_TILE + HAB_GAP), r as f32 * (HAB_TILE + HAB_LABEL + HAB_GAP)), egui::vec2(HAB_TILE, HAB_TILE));
        let edge = if *coastal { Some(COAST_EDGE) } else { None };
        let id = ui.id().with(("slot-box", n));
        // Ticket #150 (version 0.07.4): every box says on hover what its row said -- the figures and
        // the rules -- and the other states say what they are. The designer: *"mouse over
        // information on buildings didn't xfer to icon grid."*
        let side = if *coastal { "coastal" } else { "inland" };
        match kind {
            SlotBoxKind::Standing(i) => {
                let f = &st.facilities[*i];
                let state = if f.mothballed { TileState::Mothballed } else if !f.online { TileState::Offline } else { TileState::Standing };
                let heading = format!("{} ({side}): {}{}", f.kind.name(), facility_figures(game, sid, f, director), facility_offline_words(f));
                let tip = facility_rules(&heading, f.coastal);
                if hab_tile(ui, rect, id, Some(crate::icons::facility_icon(f.kind)), f.kind.name(), state, view.slot_box == Some(SlotBox::Facility(*i)), edge, tip).clicked() {
                    view.slot_box = Some(SlotBox::Facility(*i));
                }
            }
            SlotBoxKind::Building(qi) => {
                let b = &st.queue[*qi];
                let BuildItem::Facility(k) = b.item else { continue };
                let turns = estimates.get(*qi).copied().unwrap_or(u32::MAX);
                let coast = if *coastal { "\nOn the coast, the sea can take it at a threshold." } else { "" };
                let (tip, cancel) = building_tip(game, session, Place::State(sid), *qi, b, turns, mine, &format!("{} ({side})", k.name()), coast);
                let resp = hab_tile(ui, rect, id, Some(crate::icons::facility_icon(k)), k.name(), TileState::Building { ordered: false, done: b.done, widgets: b.widgets }, false, edge, tip);
                // Ticket #332: a rival's build, left behind when this Region changed hands, is
                // cancelled by a right-click, as this turn's own order is on the box below.
                if let Some(order) = cancel
                    && resp.secondary_clicked()
                {
                    actions.push(Action::Place(order));
                }
            }
            SlotBoxKind::Ordered(k, i) => {
                // Ticket #291: ordered this turn. Right-click takes the order back, the same
                // cancel the orders list's button does. Ticket #332: the face reads `0 of 8`,
                // nothing being done on it until the turn ends, and the hover the estimate.
                let widgets = game.build_widgets(Seat(0), BuildItem::Facility(*k));
                let turns = game.turns_to_build(Seat(0), Place::State(sid), BuildItem::Facility(*k));
                let tip = format!("{} ({side}): ordered this turn, {widgets} Widgets, {} once the turn ends.\nRight-click to cancel the order.{}", k.name(), estimate_words(turns), if *coastal { "\nOn the coast, the sea can take it at a threshold." } else { "" });
                if hab_tile(ui, rect, id, Some(crate::icons::facility_icon(*k)), k.name(), TileState::Building { ordered: true, done: 0, widgets }, false, edge, tip).secondary_clicked() {
                    actions.push(Action::Cancel(*i));
                }
            }
            SlotBoxKind::Free => {
                let first_free = boxes.iter().position(|(k, _)| matches!(k, SlotBoxKind::Free)) == Some(n);
                let tip = format!("Free {side} slot: click it to build here.{}", if *coastal { "\nOn the coast, the sea can take what stands here at a threshold." } else { "" });
                if hab_tile(ui, rect, id, None, "", TileState::Free(mine), first_free && view.slot_box == Some(SlotBox::Free), edge, tip).clicked() {
                    view.slot_box = Some(SlotBox::Free);
                }
            }
            SlotBoxKind::Flooded(k) => {
                let tip = format!(
                    "{}lost to the sea: a Sea Level threshold took this coastal slot.\nA working Sea Wall holds every threshold off, at most one to a state.",
                    k.map(|k| format!("{}, ", k.name())).unwrap_or_else(|| "A slot ".to_string())
                );
                hab_tile(ui, rect, id, k.map(crate::icons::facility_icon), k.map(|k| k.name()).unwrap_or(""), TileState::Flooded, false, edge, tip);
            }
        }
    }
    ui.add_space(4.0);
    // The strip: the clicked box's line, or the build buttons for a free one.
    match view.slot_box {
        Some(SlotBox::Facility(i)) if i < st.facilities.len() && game.takes_slot(st.facilities[i].kind) => {
            facility_row(ui, session, game, sid, i, &st.facilities[i], mine, director, actions);
        }
        Some(SlotBox::Free) if mine => {
            ui.label(RichText::new("Build here").strong());
            facility_build_buttons(ui, session, game, sid, actions);
        }
        _ => {
            ui.label(RichText::new("Click a box for its figures and controls.").weak());
        }
    }
}

fn state_panel(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, sid: StateId, actions: &mut Vec<Action>) {
    let card = game.tables.state(sid);
    let st = game.state(sid);
    // Ticket #122 (version 0.07.2): the Nation's flag beside the Region's name, both at thirty-two
    // pixels -- the size the flag research found brings an emblem back, and the designer asked for
    // the name to match it. Where there is no flag the name stands alone at the same size.
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 10.0;
        if let Some(flag) = Icons::flag_from_ctx(ui.ctx(), &card.flag, 32.0) {
            ui.add(flag);
        }
        ui.label(RichText::new(&card.name).size(32.0).strong());
    });
    let owner = match st.control {
        Control::Neutral => "Neutral".to_string(),
        Control::Controlled(s) => format!("Controlled by the {}", game.seat_name(s)),
        Control::Occupied { occupier, turns, .. } => format!("Occupied by the {} (turn {} of {})", game.seat_name(occupier), turns, game.tables.influence.occupation_turns),
    };
    // Ticket #161 (version 0.07.5): who holds a Region, and by what rule they keep or lose it.
    let owner_tip = match st.control {
        Control::Neutral => "Nobody holds it. The first Standing to reach the threshold takes it; two arriving level are settled by lot.".to_string(),
        Control::Controlled(s) => format!(
            "The {} hold it, and take its Influence value into their Allotment each turn.\nA rival needs their Standing plus {}, and at least its own threshold.",
            game.seat_name(s),
            game.tables.influence.challenge_margin
        ),
        Control::Occupied { occupier, .. } => format!(
            "An Army of the {} beat its defenders: they choose what is built here but do not direct its Armies.\nControl passes after {} turns, or sooner once the occupier's Influence, gained each turn and halved while Unrest is high, passes the threshold.",
            game.seat_name(occupier),
            game.tables.influence.occupation_turns
        ),
    };
    rule_tip(ui.label(owner), owner_tip);
    let mult = st.control.director().map(|s| game.tables.faction(game.kind(s)).emissions_multiplier).unwrap_or(1.0);
    let industry_em = card.baseline_emissions * st.industry_level as f64 * mult;
    let fac_em: f64 = st.facilities.iter().filter(|f| f.working()).map(|f| game.tables.facility(f.kind).emissions * mult).sum();
    // Ticket #143 (version 0.07.3): the figure in units of five million with the real number beside
    // it, and the word Region, since the figure is the territory's and the Nation's name on the card
    // read it as the Nation's. The designer: *"Population 12.2 (hundreds of millions) should say
    // something like Population 12.2 (339M)."*
    icon_word(ui, "population", format!("Region population {}, Industry Level {}, leans {:?}", Game::population_text(st.population), st.industry_level, card.resource_lean));
    // Ticket #161 (version 0.07.5): what an Allotment is, which this line names and never explains.
    rule_tip(
        icon_word(ui, "influence", format!("Influence value {}: what it adds to its controller's Allotment each turn (+1 per Industry Level raised)", game.state_influence_value(sid))),
        format!(
            "The Allotment is what you receive each turn: {}, plus the Influence value of every Region you hold.\nIt does not carry over; what is unspent at End Turn is lost. Ducats buy more, two a point, in the Trading window.",
            game.tables.influence.allotment_base
        ),
    );
    ui.label(format!("GDP {}: its economy pays its controller {} Ducats a turn (GDP x Industry Level / 5, never below 1); a Bank here would add {}", card.gdp, game.state_ducats(sid), (game.tables.facility(FacilityKind::Bank).produces.as_ref().map(|p| p.amount).unwrap_or(0) * card.gdp) / 10));
    icon_word(ui, "emissions", format!("Emissions this turn: industry {:.1}, Facilities {:.1}, people {:.1}", industry_em, fac_em, game.population_coefficient(sid) * st.population * mult));
    // Ticket #54: the per-person line, its formula, and what Leapfrog has taken off it.
    {
        let c = &game.tables.climate;
        let n = game.leapfrogs(sid);
        let leaps = match n {
            0 => String::new(),
            1 => ", Leapfrogged once".to_string(),
            2 => ", Leapfrogged twice".to_string(),
            n => format!(", Leapfrogged {n} times"),
        };
        ui.label(
            RichText::new(format!(
                "Its people emit {:.2} per hundred million ({:.2} base + {:.2} x Industry Level {}{})",
                // Ticket #143: the rate is kept per hundred million, which is twenty units now.
                game.population_coefficient(sid) * Game::UNITS_PER_HUNDRED_MILLION,
                c.population_emissions_base * Game::UNITS_PER_HUNDRED_MILLION,
                c.population_emissions_per_level * Game::UNITS_PER_HUNDRED_MILLION,
                st.industry_level,
                leaps
            ))
            .weak(),
        );
    }
    // Ticket #54: a Strip Permit running here, and what it will cost when it ends.
    if game.strip_permit_running(sid) {
        let left = game.strip_permit_turns_left(sid);
        let t = &game.tables.strip_permit;
        ui.colored_label(
            Color32::from_rgb(255, 170, 120),
            match left {
                0 => format!(
                    "Strip Permit: the last doubled turn. At this Resolution its Baseline Emissions rise {:.1} for good and its Unrest by {}.",
                    t.baseline_rise, Game::unrest_figure(t.unrest)
                ),
                n => format!("Strip Permit: {} turn(s) left of double output, then +{:.1} Baseline Emissions for good and +{} Unrest.", n, t.baseline_rise, Game::unrest_figure(t.unrest)),
            },
        );
    } else if st.baseline_rise > 0.0 {
        ui.label(RichText::new(format!("A spent Strip Permit left its Baseline Emissions {:.1} higher, for good.", st.baseline_rise)).weak());
    }
    // Ticket #154 (version 0.07.4): the slot count is said once, on the Facilities header.
    // Ticket #208 (version 0.08.1): the LIVE figure, which a School moves, where the card's own
    // unchanging row stood. Since ticket #185 built the School this line had printed
    // `card.education_level` -- the static table value -- so a player could raise a School and
    // watch the number it exists to lift sit there not moving, its only visible effect being a
    // Lab's Research creeping up. It is the figure four rules now read: a Lab twice over, a
    // Region's resistance to Influence, and what Colonists carry away with them.
    let live = game.education_level(sid);
    let schooled = live - card.education_level;
    let hover = if schooled > 0.005 {
        format!("{:.2} on the card, and {:+.2} from a School. It multiplies a Research Lab twice over, stiffens this Region against an outsider's Influence, and goes with any Colonist recruited here.", card.education_level, schooled)
    } else {
        format!("{:.2} on the card, and no School standing. It multiplies a Research Lab twice over, stiffens this Region against an outsider's Influence, and goes with any Colonist recruited here.", card.education_level)
    };
    rule_tip(ui.label(format!("Education Level {live:.2}")), hover);
    // Ticket #52: Unrest, and what it is doing here in words.
    {
        let u = &game.tables.unrest;
        let n = st.unrest;
        let colour = if n >= u.facility_threshold {
            Color32::from_rgb(255, 90, 80)
        } else if n >= u.army_threshold {
            Color32::from_rgb(255, 190, 90)
        } else {
            Color32::LIGHT_GREEN
        };
        let u = &game.tables.unrest;
        rule_tip(
            ui.colored_label(colour, format!("Unrest {}: {}", game.unrest_text(sid), game.unrest_note(sid))),
            format!(
                "Unrest runs 0 to {:.0}, in halves, with three thresholds:\nat {:.0} the Standing Army stops replenishing,\nat {:.0} every Facility here runs at half,\nat {:.0} the state throws its controller off.\nIt falls {:.1} a turn on its own, except the turn the state changed hands.",
                u.max, u.army_threshold, u.facility_threshold, u.max, u.natural_fall
            ),
        );
        // Ticket #269 (version 0.08.4): Agitate, on a Region a rival holds -- Relief's mirror, at
        // the top of the card where the holder's Relief would be on their own.
        if let Some(holder) = game.place_control(Place::State(sid)).controller().filter(|h| *h != Seat(0))
            && !session.spectator
        {
            let ag = &game.tables.unrest;
            ui.horizontal(|ui| {
                cost_button_with_hover(
                    ui,
                    game,
                    &session.pending,
                    Order::Agitate { state: sid },
                    "Agitate: Unrest +1",
                    Some(format!(
                        "Turn its people against the {}: Unrest rises by {} at End Turn, halved by a working Constabulary. Once a turn here. They will know who paid: it is an offence.",
                        game.seat_name(holder), Game::unrest_figure(ag.agitate_points)
                    )),
                    actions,
                );
            });
        }
        // Ticket #75's warning line -- a rival within two steps, at the top of the card -- stood here
        // until ticket #262 (version 0.08.4) folded it into the challenger line in the Standings block
        // below, which reads the engine's own price (ticket #60) where this one added the margin to the
        // holder's Standing and could disagree with the Resolution. One line, one arithmetic.
        // Ticket #114 (version 0.07.1): Influence is the card's FIRST business, not its last. The
        // designer: "Influence spend should be much higher and more prominent in the side bar when
        // countries are selected as well." It used to sit under the Facility list, the Army orders
        // and two paragraphs of help -- below the fold on every card with more than a few buildings,
        // which is every card by the middle of a game. It is the reason a player clicked the
        // country; it goes where their eye lands.
        ui.separator();
        // Ticket #161 (version 0.07.5): the heading is the one place that can say what Influence IS,
        // which nothing on the card says today. The designer: *"influence mouse over on all words in
        // the influence portion of the nation card."*
        rule_tip(
            icon_word(ui, "influence", "Influence"),
            format!(
                "Your claim here: spend from the corner's Allotment and it becomes your Standing, which survives any change of hands.\nHighest Standing at the threshold takes a free place; a held one wants the holder's plus {}.\nDecays {} a turn for the holder, {} for everyone else.",
                game.tables.influence.challenge_margin,
                game.tables.influence.decay_controlled,
                game.tables.influence.decay
            ),
        );
        influence_row(ui, game, session, view, Place::State(sid), false, actions);
        ui.separator();
        if game.constabulary_online(sid) {
            ui.label(RichText::new("A Constabulary here takes 1 off every turn and damps what the climate and the refugees add.").weak());
        }
        // Ticket #54: a Scrubber calms its state as well as the air.
        if game.scrubbers_online(sid) > 0 {
            ui.label(
                RichText::new(format!(
                    "{} Scrubber(s) here take {} off the Sink and {} off the Unrest every turn.",
                    game.scrubbers_online(sid),
                    format_args!("{:.1} ppm", game.tables.facility(FacilityKind::Scrubber).sink_per_turn * game.scrubbers_online(sid) as f64),
                    Game::unrest_figure(game.tables.unrest.scrubber_fall)
                ))
                .weak(),
            );
        }
    }
    // Ticket #332 (version 0.09.0): what this Region makes in Widgets a turn, and its queue.
    widgets_block(ui, game, Place::State(sid));
    // Ticket #146 (version 0.07.3): the slots the sea took are drawn under water among the boxes
    // below, so the sea-blue count that stood here is gone.
    // Ticket #56: the two rows of slots, with what stands in each and what the sea has taken.
    // Ticket #291 (version 0.08.6): a slot an order placed this turn will take is not free, and the
    // boxes below no longer draw it as one, so the count agrees with them and with the rule.
    let ordered_slots = session.pending.iter().filter(|o| o.build_state() == Some(sid) && o.build_facility().is_some_and(|k| game.takes_slot(k.built_by(game.kind(Seat(0)))))).count() as u32;
    let free_now = game.free_slots(sid).saturating_sub(ordered_slots);
    rule_tip(
        ui.label(RichText::new(format!("Facilities ({free_now} of {} slots free{})", game.build_slots(sid), if ordered_slots > 0 { format!(", {ordered_slots} ordered this turn") } else { String::new() })).strong()),
        format!(
            "Slots: Size {} plus {} plus the Industry Level {} it started at, and one more for every raise since, always inland.\n{} are coastal: the sea takes those at a threshold, oldest Facility with them, and turns one inland slot coastal every time, wall or no wall. A Sea Wall holds the taking off, not the turning.\nMothballed and building each keep a slot.",
            game.tables.state(sid).size,
            game.tables.base_slots,
            game.tables.state(sid).industry_level,
            game.coastal_slots(sid)
        ),
    );
    let director = st.control.director();
    // Ticket #64: a spectator reads every card and orders on none of them.
    let mine = !session.spectator && st.control.director() == Some(Seat(0));
    // Ticket #146 (version 0.07.3): the slots as boxes, with the clicked box's line beneath them.
    slot_boxes(ui, session, game, view, sid, mine, director, actions);
    no_slot_section(ui, session, game, sid, mine, director, actions);
    // Ticket #312 (version 0.08.7): the Armies block holds the Army orders too, at the designer's
    // word -- *"Move the Army orders block up the card and into the Armies list"*: the stance row
    // under the heading (it is per place, so it belongs to the list and not to any row), and under
    // each of the player's raised Armies its march buttons and repairs, indented. The block that
    // stood at the foot of the card, below the fold at 1080 in the presentation review's picture,
    // is gone. The whole block is a tenth larger, at `ARMY_LIST_SCALE`.
    let armies: Vec<&Army> = game.armies.iter().filter(|a| a.at == ArmyAt::Place(Place::State(sid))).collect();
    ui.scope(|ui| {
        for font in ui.style_mut().text_styles.values_mut() {
            font.size *= ARMY_LIST_SCALE;
        }
        let heading = ui.label(RichText::new("Armies").strong());
        // Ticket #323 (version 0.08.8): a click on the shield brings the card to its Armies block.
        if view.armed_stack == Some(sid) && view.armed_scroll {
            heading.scroll_to_me(Some(egui::Align::Min));
            view.armed_scroll = false;
        }
        let my_armies: Vec<&Army> = armies.iter().copied().filter(|a| mine && game.army_seat(a) == Some(Seat(0)) && !game.army_stands_down(a)).collect();
        if !my_armies.is_empty() {
            stance_row(ui, game, &session.pending, my_armies[0].stance, |s| Order::ArmyStance { place: Place::State(sid), stance: s }, false, actions);
        }
        // Ticket #322 (version 0.08.8): **the stack**: every Army of the player's at the place. With
        // more than one, a row of neighbour buttons moves them all, priced at nothing, the odds
        // read from the stack's summed strength; the per-Army rows keep their buttons for a split.
        // With one, the Army's own row is the stack and no second row is drawn.
        let stack: Vec<&Army> = my_armies.iter().copied().filter(|a| a.stance != Stance::DigIn).collect();
        let stacked = stack.len() > 1;
        if stacked {
            let strength: i64 = stack.iter().map(|a| game.army_strength(a)).sum();
            ui.horizontal_wrapped(|ui| {
                ui.label(format!("All {} ({}):", stack.len(), strength));
                for n in &card.neighbours {
                    let ctrl = game.state(*n).control;
                    let own = ctrl == Control::Controlled(Seat(0));
                    let passage = matches!(ctrl, Control::Controlled(h) if h != Seat(0) && game.accord_has(Seat(0), h, Term::Passage));
                    let name = &game.tables.state(*n).name;
                    let label = format!("{} {name}", if own || passage { "move to" } else { "attack" });
                    let hover = if own {
                        format!("{name}: held by you. Moving costs nothing; the stack keeps its stance.")
                    } else if let (true, Control::Controlled(h)) = (passage, ctrl) {
                        format!("{name}: held by the {}, a partner under Passage. Moving costs nothing; the stack arrives on Hold.", game.seat_name(h))
                    } else {
                        attack_hover(game, Place::State(*n), name, strength, false)
                    };
                    let orders: Vec<Order> = stack.iter().map(|a| Order::MoveArmy { army: a.id, to: *n }).collect();
                    orders_button(ui, game, &session.pending, orders, &label, Some(hover), actions);
                }
            });
            let hurt: Vec<&Army> = stack.iter().copied().filter(|a| a.damage > 0).collect();
            if hurt.len() > 1 {
                ui.horizontal_wrapped(|ui| {
                    let repairs: Vec<Order> = hurt.iter().map(|a| Order::Repair { unit: UnitRef::Army(a.id), points: a.damage }).collect();
                    orders_button(ui, game, &session.pending, repairs, "Repair all", Some("Every damaged Army of the stack repaired fully, for Materials.".to_string()), actions);
                    let repairs: Vec<Order> = hurt.iter().map(|a| Order::RepairWithDucats { unit: UnitRef::Army(a.id), points: a.damage }).collect();
                    orders_button(ui, game, &session.pending, repairs, "Repair all with Ducats", Some("Every damaged Army of the stack repaired fully, for Ducats.".to_string()), actions);
                });
            }
        }
        for a in &armies {
            let who = match game.army_seat(a) {
                Some(s) => game.seat_name(s),
                None => "neutral".to_string(),
            };
            // Ticket #270 (version 0.08.4): named, the Standing Army included.
            // Ticket #282 (version 0.08.5): a Levy says so, and a standing row's hover names the rule.
            // Ticket #297 (version 0.08.6): a dug-in Army says so. Ticket #302: and every Army says
            // what it defends at -- its strength, its Region's people, Dig In -- beside its strength.
            let defended = game.army_defended_strength(a);
            let defends = if defended != game.army_strength(a) { format!(", defends at {defended}") } else { String::new() };
            let dug = if game.army_dug_in(a) { ", dug in" } else { "" };
            let row = ui.label(format!(
                "  {} ({}{}): strength {}, damage {}/{}{defends}{dug}",
                game.army_name(a),
                who,
                if a.standing { ", standing" } else { "" },
                game.army_strength(a),
                a.damage,
                game.army_hit_points(a)
            ));
            // Ticket #302 (version 0.08.6): the hover names every term of the one Army system.
            let t = &game.tables.standing_army;
            let tip = if a.standing {
                let armed = game.state(sid).armed;
                let police = if game.constabulary_online(sid) { format!(" +{} for the working Constabulary", t.constabulary) } else { format!(" +{} if a Constabulary were working here", t.constabulary) };
                let calm = if game.army_replenishes(sid) { format!(", +{} while Unrest is under {:.0}", t.calm, game.tables.unrest.army_threshold) } else { format!(", +{} lost to Unrest at {:.0} or more", t.calm, game.tables.unrest.army_threshold) };
                format!(
                    "A Region's own Army. Its strength and hit points are Industry Level + 1{}; it may march, and away from home it is an Army like any other. Defending at home, it fights at that{police}{calm}{}. It heals 1 a turn while Unrest is under {:.0}; at its strength in damage it is destroyed, and returns at strength 1 two Incomes later. A neutral Region arms for good, +{} when a threat appears next door and +{} for every attack it holds against, with no ceiling.",
                    if armed > 0 { format!(" and +{armed} armed") } else { String::new() },
                    if game.army_dug_in(a) { format!(", +{} dug in", game.tables.dig_in.defence) } else { String::new() },
                    game.tables.unrest.army_threshold,
                    t.threat_steps,
                    t.held_step
                )
            } else {
                format!(
                    "A raised Army: its strength and hit points were its home's Industry Level + 1 when it was raised, fixed since. It belongs to its home Region and changes hands with it; it marches, and it may Dig In for +{} while defending.",
                    game.tables.dig_in.defence
                )
            };
            row.on_hover_text(tip);
            // Ticket #302 (version 0.08.6): a Region's own Army stays at home, so only a raised Army
            // has march buttons. Ticket #309 (version 0.08.7): the odds are on a hover that names the
            // defender, not on the button face. Ticket #312: under the Army's own row.
            // Ticket #321 (version 0.08.8): a Region's own Army marches too, so every Army of the
        // player's has march buttons.
        if my_armies.iter().any(|m| m.id == a.id) {
                ui.horizontal_wrapped(|ui| {
                    ui.add_space(16.0);
                    for n in &card.neighbours {
                        let ctrl = game.state(*n).control;
                        let own = ctrl == Control::Controlled(Seat(0));
                        // Ticket #320 (version 0.08.8): a partner's Region under Passage is moved
                        // into, not attacked; the Army arrives on Hold.
                        let passage = matches!(ctrl, Control::Controlled(h) if h != Seat(0) && game.accord_has(Seat(0), h, Term::Passage));
                        let name = &game.tables.state(*n).name;
                        let label = format!("{} {name}", if own || passage { "move to" } else { "attack" });
                        let hover = if own {
                            format!("{name}: held by you. Moving costs nothing; the Army keeps its stance.")
                        } else if let (true, Control::Controlled(h)) = (passage, ctrl) {
                            format!("{name}: held by the {}, a partner under Passage. Moving costs nothing; the Army arrives on Hold and fights nobody while the Accord stands.", game.seat_name(h))
                        } else {
                            attack_hover(game, Place::State(*n), name, game.army_strength(a), false)
                        };
                        cost_button_with_hover(ui, game, &session.pending, Order::MoveArmy { army: a.id, to: *n }, &label, Some(hover), actions);
                    }
                    if a.damage > 0 {
                        cost_button(ui, game, &session.pending, Order::Repair { unit: UnitRef::Army(a.id), points: a.damage }, "Repair fully", actions);
                        cost_button(ui, game, &session.pending, Order::RepairWithDucats { unit: UnitRef::Army(a.id), points: a.damage }, "Repair fully with Ducats", actions);
                    }
                });
            }
        }
    });
    ui.separator();
    if mine {
        // Ticket #154 (version 0.07.4): the per-kind build list is gone from here -- a Facility is
        // built by clicking a free box -- and the Scrubber and Sea Wall buttons stand under the
        // boxes; what is left is orders, and the header says so. The designer: *"remove redundant
        // build list from the region cards."*
        ui.label(RichText::new("Orders").strong());
        // Ticket #54: the Custodians' Leapfrog, and the Prospectors' Strip Permit.
        if game.kind(Seat(0)) == FactionKind::Custodians {
            ui.horizontal(|ui| {
                cost_button(ui, game, &session.pending, Order::Leapfrog { state: sid }, "Leapfrog", actions);
                ui.label(RichText::new(format!("lowers its people to {:.2} per hundred million, for good", (game.population_coefficient(sid) - game.tables.climate.population_emissions_per_level).max(game.tables.climate.population_emissions_base) * Game::UNITS_PER_HUNDRED_MILLION)).weak());
            });
        }
        // Ticket #237 (version 0.08.3): the Arkwrights' own order, beside the Custodians' Leapfrog
        // and the Prospectors' Strip Permit, and guarded the same way -- once per state, ever.
        if game.kind(Seat(0)) == FactionKind::Arkwrights && !st.exodus_call_used {
            let t = &game.tables.exodus_call;
            ui.horizontal(|ui| {
                cost_button(ui, game, &session.pending, Order::ExodusCall { state: sid }, "Exodus Call", actions);
                ui.label(
                    RichText::new(format!(
                        "{} turns recruiting {} Pioneers here instead of {}, and each costs this Region the ordinary population rather than your double. Once per Region, ever.",
                        t.turns,
                        game.emigrants_per_turn(Seat(0)) * t.muster_multiplier,
                        game.emigrants_per_turn(Seat(0))
                    ))
                    .weak(),
                );
            });
        }
        if game.kind(Seat(0)) == FactionKind::Prospectors && !st.strip_permit_used {
            let t = &game.tables.strip_permit;
            ui.horizontal(|ui| {
                cost_button(ui, game, &session.pending, Order::StripPermit { state: sid }, "Strip Permit", actions);
                ui.label(RichText::new(format!("{} turns of double output here, then +{:.1} Baseline Emissions and +{} Unrest, for good", t.turns, t.baseline_rise, Game::unrest_figure(t.unrest))).weak());
            });
        }
        cost_button(ui, game, &session.pending, Order::RaiseIndustry { state: sid }, "Raise Industry Level", actions);
        ui.label(RichText::new("Raising the Industry Level adds an inland slot.").weak());
        cost_button(ui, game, &session.pending, Order::BuildArmy { place: Place::State(sid) }, "Build Army", actions);
        // Ticket #73: muster Emigrants here, and send them to Antarctica by sea once the ice is open.
        ui.label(RichText::new("Pioneers").strong());
        // Ticket #211 (version 0.08.1): the figure stands at the HEAD of this block, above the
        // button that changes it, and is shown at every value including nought. It existed before
        // -- in the Influence block, some way up the card, and only while it was above zero -- so a
        // player who mustered and then looked for the result found the line had simply not been
        // there a moment ago. At nought it now says so, which is the answer to "did that work?".
        ui.label(format!("Pioneers waiting: {}", st.emigrants)).on_hover_text(
            "Recruited here and not yet lifted or sent: a working Launch Site lifts them onto a Ship or straight to a station of yours over Earth, and once the ice is open the sea takes them to Antarctica.",
        );
        // Ticket #196 (version 0.08.0): as many as this state's people can pay for, where the button
        // always asked for the whole batch. Coach Class costs the Arkwrights twice the population for
        // twice the batch -- 16.0 people -- and Australia carries 10.1 to 12.6, so the button was dead
        // there with nothing on screen to say why. Where the state cannot pay for even one, it still
        // offers one, so the refusal a player reads is "not enough people there" rather than silence.
        let per = game.emigrants_affordable(Seat(0), sid).max(1);
        cost_button_with_hover(
            ui,
            game,
            &session.pending,
            Order::BuildEmigrants { state: sid, n: per },
            &format!("Recruit {per} Pioneers"),
            Some(format!(
                "{} people, on the card at End Turn, and {} off this state's Unrest. A working Launch Site lifts them onto a Ship or straight to a station of yours over Earth; once the ice is open the sea takes them to Antarctica.",
                Game::people_text(game.lift_population(Seat(0), per)),
                Game::unrest_figure(game.tables.emigrants.unrest_fall)
            )),
            actions,
        );
        if game.antarctica_open && st.emigrants > 0 {
            let n = st.emigrants;
            for slot in game.free_slots_on(BodyId::Earth) {
                // Ticket #283 (version 0.08.5): the third founding door wears the same face as the
                // two Ship doors, the site's yields in glyphs, at the designer's word.
                let order = Order::SendToAntarctica { state: sid, n, into: UnloadTarget::Slot(BodyId::Earth, slot) };
                let label = format!("Send {n} to {} by sea", game.tables.body(BodyId::Earth).slots[slot as usize].name);
                if found_button(ui, &game.slot_yields(BodyId::Earth, slot), &label).clicked() {
                    actions.push(Action::Place(order));
                }
            }
            // Ticket #204 (version 0.08.1): capped at the room there. A sea crossing checks no room
            // at the order -- it lands `min(n, room)` a turn later and sends the surplus home with a
            // Report line -- so this button offered to put twelve people on a round trip that costs
            // a turn and achieves nothing, and said nothing about it first. The engine rule is
            // unchanged; only the button stops offering it. Founding a NEW Colony from a free slot,
            // above, still offers everyone waiting: there is no room limit where nothing stands yet.
            for c in game.colonies.iter().filter(|c| c.body == BodyId::Earth && !c.in_orbit && c.control.director() == Some(Seat(0))) {
                let k = n.min(game.habitat_room(c).saturating_sub(c.colonists));
                if k == 0 {
                    continue;
                }
                cost_button(ui, game, &session.pending, Order::SendToAntarctica { state: sid, n: k, into: UnloadTarget::Colony(c.id) }, &format!("Send {k} to {} by sea", game.place_name(Place::Colony(c.id))), actions);
            }
        }
        // Ticket #141 (version 0.07.3): waiting Emigrants lift straight to a station of yours over
        // Earth, as many as it has room for, by the Launch Site here. A launch, no Ship.
        if st.emigrants > 0 && st.facilities.iter().any(|f| f.kind.does_the_job_of(FacilityKind::LaunchSite) && f.working()) {
            for c in game.colonies.iter().filter(|c| c.body == BodyId::Earth && c.in_orbit && c.control.director() == Some(Seat(0))) {
                let room = game.habitat_room(c).saturating_sub(c.colonists);
                let n = st.emigrants.min(room);
                if n == 0 {
                    continue;
                }
                cost_button_with_hover(
                    ui,
                    game,
                    &session.pending,
                    Order::LiftToStation { state: sid, n, colony: c.id },
                    &format!("Send {n} to {} by lift", game.place_name(Place::Colony(c.id))),
                    Some(format!("Aboard at this turn's Resolution. A launch: it emits like any lift. {} has room for {room} more.", game.place_name(Place::Colony(c.id)))),
                    actions,
                );
            }
        }
        // Ticket #193 (version 0.08.0): and straight onto a Colony Ship of yours at Earth with room
        // left, whether it sits in an Orbital Slot or at the Body at large -- that distinction is
        // about blockades and has nothing to do with loading people, so a button for one and not the
        // other would read as a defect. A Carrier takes an Army and no Colonists, so it never
        // appears. The rule already worked; only the door was missing, exactly as ticket #141
        // answered for stations. Both doors write the same Load order, so either cancels the other.
        //
        // It never offers the CROWDED places: above +1.8 a Ship lifting at Earth may take Colonists
        // beyond its capacity, and each of those may die on arrival. A risk that drowns people wants
        // the sentence explaining it beside the button, and that sentence lives on the Ship's card --
        // so a player who means to crowd a ship goes there deliberately.
        if st.emigrants > 0 && st.facilities.iter().any(|f| f.kind.does_the_job_of(FacilityKind::LaunchSite) && f.working()) {
            let capacity = game.colony_ship_capacity(Seat(0));
            for s in game.ships.iter().filter(|s| s.seat == Seat(0) && s.kind == UnitKind::ColonyShip && s.at == ShipAt::Body(BodyId::Earth)) {
                let room = capacity.saturating_sub(s.colonists);
                let n = st.emigrants.min(room);
                if n == 0 {
                    continue;
                }
                cost_button_with_hover(
                    ui,
                    game,
                    &session.pending,
                    Order::Load { ship: s.id, colonists: n, from: LoadSource::State(sid), army: None },
                    &format!("Send {n} to {} in orbit", game.ship_name(s)),
                    Some(format!("A launch, aboard at this turn's Resolution. This Ship carries {capacity} and has {} aboard. To crowd it past its capacity, load it from its own card.", s.colonists)),
                    actions,
                );
            }
        }
        // Ticket #52: Relief and Resettle, with their prices on the buttons.
        ui.label(RichText::new("Unrest").strong());
        ui.horizontal(|ui| {
            cost_button(ui, game, &session.pending, Order::Relief { state: sid }, "Relief: Unrest -1", actions);
            cost_button(ui, game, &session.pending, Order::Resettle { state: sid }, "Resettle here", actions);
        });
        ui.label(
            RichText::new(
                "Relief may be paid any number of times a turn. Resettle sends every refugee leaving your states here this turn, once a turn, and raises your Standing here by 5.",
            )
            .weak(),
        );
        // Ticket #46: Ships come from Shipyards; a Launch Site lifts people to orbit.
        ui.label(
            RichText::new(if st.facilities.iter().any(|f| f.kind.does_the_job_of(FacilityKind::LaunchSite) && f.working()) {
                "Launch Site: Colonists and Armies lift to orbit from here. Ships are built at a Shipyard on a station or Colony."
            } else {
                "No working Launch Site: nothing lifts to orbit from here."
            })
            .weak(),
        );
        // Ticket #312 (version 0.08.7): the Army orders that stood here are in the Armies block.
    }
}

/// Ticket #204 (version 0.08.1): which Region a loader's dropdown opens on.
///
/// It was the most POPULOUS Region the player directs, chosen with no regard for whether that
/// Region could lift at all -- so a Ship's door could open already dead: a greyed button, and
/// nothing to suggest that another Region in the same list would work. It now opens on the first
/// Region that can ACT, and falls back to the most populous when none can, so the refusal is still
/// reachable and still explains itself. A sea crossing wants no Launch Site, which is what
/// `needs_launch_site` is for.
fn default_emigrant_source(game: &Game, states: &[StateId], needs_launch_site: bool) -> Option<StateId> {
    let ready = states.iter().copied().find(|st| {
        let s = game.state(*st);
        s.emigrants > 0 && (!needs_launch_site || s.facilities.iter().any(|f| f.kind.does_the_job_of(FacilityKind::LaunchSite) && f.working()))
    });
    ready.or_else(|| states.iter().copied().max_by(|a, b| game.state(*a).population.partial_cmp(&game.state(*b).population).unwrap()))
}

/// Ticket #204 (version 0.08.1): **the receiver's door**. A Region's card has been able to send
/// people to a station since ticket #141 and to a Colony Ship since ticket #193, but the place
/// receiving them had no way to ask: a player standing on the ISS wondering how to get anybody up
/// there had to go back down to a Region and find the button. This is the mirror of the job ticket
/// #193 did for Ships, in the other direction. The rule does not change; only the door was missing.
///
/// It sits directly under the card's `Colonists N of M room`, at the designer's word, because that
/// line is exactly the figure a lift changes -- cause and effect read as one thing. The `Orders`
/// block below is a long list of build and mothball buttons and a loader would be lost in it.
///
/// The dropdown lists EVERY Region the player directs, with the button greying and naming the
/// reason, since hiding a Region hides the answer to "why can't Brazil lift?".
///
/// **The two routes disagreed about "too many", and this door takes the stricter reading.** A lift
/// refuses at the order (`LiftToStation` checks Habitat room); a sea crossing checks no room at
/// all, lands `min(n, room)` a turn later and sends the surplus home with a Report line. So the
/// Region card's sea button could dispatch twelve people on a round trip that costs a turn and
/// achieves nothing, in silence. Both sea doors are now capped at `min(waiting, room)` -- the
/// sender's included -- while **the engine rule is untouched**, so saves, the computer players and
/// any other route to the same order behave exactly as before. Only the button stops offering it.
fn emigrant_loader(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, col: &dying_earth_engine::Colony, actions: &mut Vec<Action>) {
    // A lift is a launch from a surface, and there is no surface under a station over Mars; a
    // Colony on a surface beyond Earth is loaded from a Ship, which has had its door all along.
    if session.spectator || col.body != BodyId::Earth || col.control.director() != Some(Seat(0)) {
        return;
    }
    let by_sea = !col.in_orbit;
    if by_sea && !game.antarctica_open {
        return;
    }
    let states = game.directed_states(Seat(0));
    let Some(chosen) = view.lift_state.filter(|x| states.contains(x)).or_else(|| default_emigrant_source(game, &states, !by_sea)) else {
        return;
    };
    let room = game.habitat_room(col).saturating_sub(col.colonists);
    let waiting = game.state(chosen).emigrants;
    let n = room.min(waiting);
    ui.horizontal(|ui| {
        ui.label("from");
        egui::ComboBox::from_id_salt(("lift", col.id.0)).selected_text(game.tables.state(chosen).name.clone()).show_ui(ui, |ui| {
            for st in &states {
                if ui.selectable_label(*st == chosen, game.tables.state(*st).name.clone()).clicked() {
                    view.lift_state = Some(*st);
                }
            }
        });
        // With nothing to move the door STAYS, disabled and saying why, rather than vanishing and
        // leaving a player to wonder -- the shape ticket #196 settled for the muster button. It
        // carries a refusal of its own rather than the engine's, because **the engine has none to
        // give on the sea route**: `SendToAntarctica` checks no room, so a live button here would
        // hand a player the very round trip this ticket set out to stop. Found by looking at the
        // picture: at `Colonists 4 of 4 room` the sea button came up white and clickable.
        if n == 0 {
            let why = if waiting == 0 {
                format!("Nobody is waiting in {}. Recruit Pioneers there first.", game.tables.state(chosen).name)
            } else {
                format!("{} is full: {} Colonists in {} of room.", game.place_name(Place::Colony(col.id)), col.colonists, game.habitat_room(col))
            };
            let label = if by_sea { "Bring Pioneers by sea" } else { "Lift Pioneers" };
            ui.add_enabled(false, egui::Button::new(label)).on_disabled_hover_text(why);
            return;
        }
        let (order, label, hover) = if by_sea {
            (
                Order::SendToAntarctica { state: chosen, n, into: UnloadTarget::Colony(col.id) },
                format!("Bring {n} Pioneers by sea"),
                format!("They land here at NEXT turn's Resolution: a sea crossing takes a turn. {} has room for {room} more.", game.place_name(Place::Colony(col.id))),
            )
        } else {
            (
                Order::LiftToStation { state: chosen, n, colony: col.id },
                format!("Lift {n} Pioneers"),
                format!("Aboard at this turn's Resolution. A launch: it emits like any lift. {} has room for {room} more.", game.place_name(Place::Colony(col.id))),
            )
        };
        cost_button_with_hover(ui, game, &session.pending, order, &label, Some(hover), actions);
    });
}

fn colony_panel(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, cid: ColonyId, actions: &mut Vec<Action>) {
    let Some(col) = game.colony(cid) else { return };
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;
        // Ticket #216 (version 0.08.2): the holder's symbol ALONE, where a Region card's flag sits.
        // Ticket #210 put the kind glyph beside it and ticket #127 put it on the heading before that;
        // both are undone here at the designer's word. On a CARD the kind is already said by the name
        // and by what the card contains -- you are looking at a Colony -- where in the Roster the
        // glyph is the only thing separating a Colony row from a Region row at a glance, so the
        // Roster's own glyphs stay. A neutral place therefore wears no mark at all, which is the
        // standing rule that a colour says whose and nobody's place says nothing.
        faction_glyph(ui, session, game, col.control.controller(), 22.0);
        ui.label(RichText::new(game.place_name(Place::Colony(cid))).size(22.0).strong());
    });
    // Ticket #283 (version 0.08.5): what the ground is worth, under the heading, in glyphs. A
    // station reads the Body's figures, which the planet card shows, so it carries no row.
    if !col.in_orbit {
        let ink = ui.visuals().text_color();
        ui.horizontal(|ui| {
            ui.label(RichText::new("Yields here:").weak());
            slot_yield_row(ui, slot_yield_figures(&game.slot_yields(col.body, col.slot)), 13.0, ink);
        });
    }
    let owner = match col.control {
        Control::Neutral => "Nobody's".to_string(),
        Control::Controlled(s) => format!("Held by the {}", game.seat_name(s)),
        Control::Occupied { occupier, turns, .. } => format!("Occupied by the {} (turn {})", game.seat_name(occupier), turns),
    };
    ui.label(owner);
    // Ticket #278 (version 0.08.5): a starved Colony says so, in the designer's words, and how.
    if let Some(by) = game.starved_by(cid) {
        let how = if col.in_orbit { format!("Blockaded by the {}", game.seat_name(by)) } else { format!("Under the {}' Orbital Control", game.seat_name(by)) };
        ui.label(RichText::new(format!("{how}: producing nothing, upkeep still paid.")).color(Color32::from_rgb(230, 110, 90))).on_hover_text(
            "A warship stack ordered to Blockade the slot of a station starves it; a Colony on the ground starves while one rival holds Orbital Control outright and has a stack there on Blockade. Every Module makes nothing and pays its upkeep; nobody dies and nothing is destroyed. Each turn of it is an offence against you.",
        );
    }
    // Ticket #164 (version 0.07.5): the room is the Core Module's four and the Habitats' eight
    // each, so the line no longer names Habitats alone.
    ui.label(format!("Colonists {} of {} room", col.colonists, game.habitat_room(col)));
    // Ticket #204 (version 0.08.1): the receiver's door, against the figure it changes.
    emigrant_loader(ui, session, game, view, col, actions);
    // Ticket #97 (version 0.07.0): the Module cap, shown beside the Colonists that buy it, so a
    // player meets it on the card rather than as a refusal.
    let (used, cap) = (game.module_slots_used(col), game.module_slots(col));
    // Ticket #164 (version 0.07.5): with the base allowance at nothing, the cap is exactly the
    // number of people living here, so the line says that rather than naming a base of zero.
    let line = if game.tables.slots.base == 0 {
        format!("Modules {used} of {cap} (one for each Colonist)")
    } else {
        format!("Modules {used} of {cap} ({} free, one for each Colonist)", game.tables.slots.base)
    };
    let resp = if used >= cap {
        ui.colored_label(Color32::YELLOW, format!("{line} - no room for another until more Colonists live here"))
    } else {
        ui.label(line)
    };
    // Ticket #116 (version 0.07.1): the rule behind the cap, which ticket #97 put in the engine and
    // nowhere on the card.
    rule_tip(
        resp,
        format!(
            "One slot for every {} Colonist living here, and none before: the Core Module a founding gives is the whole of what a founding gives, so a place grows only as its people arrive.\nIts Core Module holds {}, which is how the first of them get here. Mothballed keeps a slot and building reserves one; the Core Module and the Archive count on neither side.",
            game.tables.slots.per_colonist,
            game.tables.module(ModuleKind::Core).holds_colonists
        ),
    );
    // Ticket #332 (version 0.09.0): what this place makes in Widgets a turn, and its queue in
    // order -- the Archive and a Ship on it too, which have no tile.
    widgets_block(ui, game, Place::Colony(cid));
    let research = game.tables.archive.research;
    // Ticket #162 (version 0.07.5): the Modules are tiles on the card, where ticket #145 had put
    // them in a window. The summary line above says the cap; the tiles say the rest, so the
    // heading, the list of names and the button that opened the window are gone.
    // Ticket #88: build it where you dig. Ticket #162 (version 0.07.5): the note stands above the
    // tiles, where a free one is clicked to build, rather than under a Build header that no longer
    // offers a Module.
    match game.working_mines(col) {
        0 => {}
        1 => {
            ui.label(RichText::new(format!("One working Mine here: Modules cost x{} (never under half the row).", game.tables.in_situ.one_mine)).weak());
        }
        n => {
            ui.label(RichText::new(format!("{n} working Mines here: Modules cost x{} (never under half the row).", game.tables.in_situ.two_mines)).weak());
        }
    }
    let mine_here = !session.spectator && col.control.director() == Some(Seat(0));
    module_boxes(ui, session, game, view, cid, mine_here, col.control.director(), actions);
    // Ticket #51's Archive-on-order line and ticket #306's lines for the builds with no tile
    // stood here; since ticket #332 the queue is in full under the Widgets line above the tiles.
    // Ticket #312 (version 0.08.7): the Colony's Armies block, as the Region card's: a heading, the
    // stance row under it, the rows, and the repairs under the player's own; a tenth larger.
    let armies: Vec<&Army> = game.armies.iter().filter(|a| a.at == ArmyAt::Place(Place::Colony(cid))).collect();
    if !armies.is_empty() {
        ui.scope(|ui| {
            for font in ui.style_mut().text_styles.values_mut() {
                font.size *= ARMY_LIST_SCALE;
            }
            ui.label(RichText::new("Armies").strong());
            let my_armies: Vec<&Army> = armies.iter().copied().filter(|a| !session.spectator && game.army_seat(a) == Some(Seat(0))).collect();
            if !my_armies.is_empty() {
                stance_row(ui, game, &session.pending, my_armies[0].stance, |s| Order::ArmyStance { place: Place::Colony(cid), stance: s }, false, actions);
            }
            for a in &armies {
                let who = game.army_seat(a).map(|s| game.seat_name(s)).unwrap_or_else(|| "nobody's".into());
                ui.label(format!("  {} ({}): strength {}, damage {}", game.army_name(a), who, game.army_strength(a), a.damage));
                if my_armies.iter().any(|m| m.id == a.id) && a.damage > 0 {
                    ui.horizontal_wrapped(|ui| {
                        ui.add_space(16.0);
                        cost_button(ui, game, &session.pending, Order::Repair { unit: UnitRef::Army(a.id), points: a.damage }, "Repair fully", actions);
                        cost_button(ui, game, &session.pending, Order::RepairWithDucats { unit: UnitRef::Army(a.id), points: a.damage }, "Repair fully with Ducats", actions);
                    });
                }
            }
        });
    }
    ui.separator();
    let mine = !session.spectator && col.control.director() == Some(Seat(0));
    if mine {
        // Ticket #51: the Archive has its own orders on an Archivist's card. Ticket #68: one Module
        // from its own button, and a fund that holds a quarter of the Research until it stands.
        if game.kind(Seat(0)) == FactionKind::Archivists {
            let fund = game.seat(Seat(0)).archive_fund;
            let cap = game.archive_fund_cap(Seat(0));
            ui.separator();
            ui.label(RichText::new("The Archive").strong());
            let built = game.archive_built(Seat(0));
            let fund_line = if built {
                format!("Archive fund {fund} of {research}")
            } else {
                format!("Archive fund {fund} of {cap} (a quarter of the {research} until the Archive stands)")
            };
            ui.label(fund_line);
            // Ticket #235 (version 0.08.3): the switch that stood here is a SLIDER now, and it
            // lives in the Tech Tree window with the other three Factions' -- at the designer's
            // word, *"yeah tech tree - put the archive's slider there too"*. One control, one
            // place, for a decision every Faction now makes about the same thing.
            let directive = game.seat(Seat(0)).research_directive;
            ui.label(
                egui::RichText::new(if directive == 0 {
                    "Your Labs pay the shared Tech. Set a Research Directive in the Tech Tree window to pay this fund instead.".to_string()
                } else {
                    format!("Your Research Directive sends {directive}% of your Research to this fund, from the next Income. It is set in the Tech Tree window.")
                })
                .weak(),
            );
            if !built && !game.archive_ordered(Seat(0)) {
                let order = Order::BuildArchive { colony: cid };
                let materials = game.order_cost(Seat(0), &order).materials;
                let check = game.check_order(Seat(0), &session.pending, &order);
                let label = format!("Build the Archive ({materials} Materials)");
                let resp = ui
                    .add_enabled(check.is_ok(), egui::Button::new(label))
                    .on_hover_text(format!("{}; then the fund opens to the full {research} Research", build_words(game, &order).unwrap_or_default()));
                if let Err(e) = &check {
                    resp.clone().on_disabled_hover_text(&e.0);
                }
                if resp.clicked() {
                    actions.push(Action::Place(order));
                }
            }
            // Ticket #192 (version 0.08.0): the Upload. Free, and irreversible -- the people leave
            // the board -- so it is an order like any other and the turn may be thought better of
            // before it ends. It draws only from the people at the Archive's own place, and it may
            // be done in batches: four at a time from a Core Module alone is enough, three times
            // over, which is what keeps the Habitat off the Archivists' critical path.
            if game.archive_complete(Seat(0)) && game.archive_colony(Seat(0)) == Some(cid) {
                let bar = game.tables.faction(game.kind(Seat(0))).victory_second.bar as u32;
                let uploaded = game.seat(Seat(0)).uploaded;
                let already: u32 = session.pending.iter().map(|o| if let Order::Upload { colony: c, n } = o { if *c == cid { *n } else { 0 } } else { 0 }).sum();
                let room = game.uploadable_at(cid, already);
                ui.label(format!("Uploaded: {uploaded} of {bar}"));
                if room > 0 {
                    let n = room.min(bar.saturating_sub(uploaded).max(1));
                    cost_button_with_hover(
                        ui,
                        game,
                        &session.pending,
                        Order::Upload { colony: cid, n },
                        &format!("Upload {n} Colonists"),
                        Some(format!(
                            "They are read into the Archive and leave the living population here: {} live at this place now. It cannot be undone once the turn ends, and the uploaded can never be lost -- not to a raid, a crowding death or a handover.",
                            game.colony(cid).map(|c| c.colonists).unwrap_or(0)
                        )),
                        actions,
                    );
                } else {
                    ui.label(RichText::new("Nobody is left here to upload; the Archive draws only from the people where it stands.").weak());
                }
            }
            ui.separator();
        }
        // Ticket #162 (version 0.07.5): the header matches the Nation card's, which ticket #154
        // renamed when its own build list folded into the boxes. A Module is ordered from a free
        // tile; what is left here is orders.
        ui.label(RichText::new("Orders").strong());
        if !col.in_orbit {
            cost_button(ui, game, &session.pending, Order::BuildArmy { place: Place::Colony(cid) }, "Build Army (Barracks)", actions);
        }
        if col.modules.iter().any(|m| m.kind == ModuleKind::Shipyard) {
            ui.label(RichText::new("Ships (Shipyard)").strong());
            for uk in UnitKind::SHIPS {
                cost_button(ui, game, &session.pending, Order::BuildShip { site: Place::Colony(cid), kind: uk }, uk.name(), actions);
            }
        }
    }
    // Ticket #312 (version 0.08.7): the stance row and the repairs that stood here are in the
    // Armies block above.
    influence_row(ui, game, session, view, Place::Colony(cid), true, actions);
}

fn slot_panel(ui: &mut Ui, session: &Session, game: &Game, body: BodyId, slot: u32, actions: &mut Vec<Action>) {
    ui.label(RichText::new(format!("{}, Colony Slot {} on {}", game.tables.body(body).slots[slot as usize].name, slot + 1, game.tables.body(body).name)).size(22.0).strong());
    // Ticket #56: Earth's three slots are Antarctica's, and they open at +1.6 C.
    if body == BodyId::Earth && !game.antarctica_open {
        ui.colored_label(
            Color32::from_rgb(150, 195, 235),
            format!("Under the Antarctic ice. It opens the first Climate phase the Temperature stands at {:+.1} C, and stays open.", game.tables.climate.antarctica_opens_at),
        );
    }
    ui.label("Empty. A Colony Ship carrying Colonists founds a Colony here; a Habitat comes with it.");
    // Ticket #57: the slot's own four yields, drawn when the game started, beside its Body's.
    let y = game.slot_yields(body, slot);
    // Ticket #258 (version 0.08.4): the site's line in the glyph-and-number row the founding button
    // and the map labels already use, at the designer's word. Ticket #283 (version 0.08.5): the
    // Body's line, kept here on #258, has moved to the head of the planet card -- "when founding a
    // colony from a ship we don't need to know the yields of the planet as a whole".
    let ink = ui.visuals().text_color();
    ui.horizontal(|ui| {
        ui.label("Yields here:");
        slot_yield_row(ui, slot_yield_figures(&y), 14.0, ink);
    });
    for s in game.ships.iter().filter(|s| !session.spectator && s.seat == Seat(0) && s.at == ShipAt::Body(body) && s.kind == UnitKind::ColonyShip && s.colonists > 0) {
        let order = Order::Unload { ship: s.id, colonists: s.colonists, army: s.army.is_some(), into: UnloadTarget::Slot(body, slot) };
        // Ticket #211 (version 0.08.1): what the site is worth, at the moment of choosing it. The
        // four yields are drawn under every slot on the Body view already, but the moment of the
        // DECISION said nothing about them. In glyphs, at the designer's word -- each figure's word
        // heads its multiplier, which is the form the one glyph rule reads (ticket #132).
        if found_button(ui, &game.slot_yields(body, slot), &format!("Found a Colony here with the {} Colonists aboard {}", s.colonists, game.ship_name(s))).clicked() {
            actions.push(Action::Place(order));
        }
    }
}

fn stack_panel(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, body: BodyId, seat: Seat, actions: &mut Vec<Action>) {
    let ships: Vec<&Ship> = game.ships.iter().filter(|s| s.seat == seat && s.at == ShipAt::Body(body)).collect();
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;
        // Ticket #216 (version 0.08.2): the Faction's symbol alone, as on a Colony card's heading.
        // The heading already names the Faction and the Body, so the kind glyph said nothing a
        // reader did not have.
        faction_glyph(ui, session, game, Some(seat), 22.0);
        ui.label(RichText::new(format!("{} Ships at {}", game.seat_name(seat), game.tables.body(body).name)).size(22.0).strong());
    });
    for s in &ships {
        let card = game.tables.unit(s.kind);
        let mut extra = Vec::new();
        if s.colonists > 0 {
            extra.push(format!("{} Colonists", s.colonists));
        }
        if s.army.is_some() {
            extra.push("an Army".into());
        }
        ui.horizontal(|ui| {
            faction_glyph(ui, session, game, Some(s.seat), 16.0);
            ui.label(format!("{}: strength {}, damage {}/{}{}", game.ship_name(s), game.ship_strength(s), s.damage, card.hit_points, if extra.is_empty() { String::new() } else { format!(", carrying {}", extra.join(" and ")) }))
                .on_hover_text(format!("{} {}", s.kind.name(), s.id.0));
        });
    }
    if session.spectator {
        let enemy = game.enemy_ship_strength(seat, body);
        if enemy > 0 {
            let rivals = rivals_at(game, seat, body);
            ui.label(format!("Against {} ({} in all) if it attacked here.", rivals_text(game, &rivals), enemy));
        }
        return;
    }
    if seat != Seat(0) {
        let mine = game.ship_stack_strength(Seat(0), body);
        // Ticket #50: a Battle at a Body is a melee, so the odds run against everyone else present.
        let theirs = game.enemy_ship_strength(Seat(0), body);
        let rivals = rivals_at(game, Seat(0), body);
        ui.label(format!(
            "A Battle here is a melee against every Faction present. Odds of winning the first round if you attack: {:.0}% (your strength {} against {}{})",
            first_round_odds(mine, theirs) * 100.0,
            mine,
            rivals_text(game, &rivals),
            if rivals.len() > 1 { format!(", {theirs} in all") } else { String::new() }
        ));
        return;
    }
    if ships.is_empty() {
        return;
    }
    ui.separator();
    stance_row(ui, game, &session.pending, ships[0].stance, |s| Order::ShipStance { body, stance: s }, true, actions);
    let enemy = game.enemy_ship_strength(seat, body);
    let enemy_ships: usize = seat.others().iter().map(|s| game.ships_at(*s, body).len()).sum();
    if enemy > 0 || enemy_ships > 0 {
        let mine = game.ship_stack_strength(Seat(0), body);
        // Ticket #50: name every Faction with Ships here; the attack is against all of them at once.
        let rivals = rivals_at(game, seat, body);
        ui.label(format!("Against {} ({} in all). Attack odds (first round): {:.0}%", rivals_text(game, &rivals), enemy, first_round_odds(mine, enemy) * 100.0));
        if ui.button("Attack this turn").clicked() {
            view.attack_preview = true;
        }
        if view.attack_preview {
            ui.label(format!("Your {} Ship(s) (strength {}) against {} Ship(s) of {} (strength {} in all). Confirm?", ships.len(), mine, enemy_ships, rivals_text(game, &rivals), enemy));
            if ui.button("Confirm Attack").clicked() {
                actions.push(Action::Place(Order::ShipStance { body, stance: Stance::Attack }));
                view.attack_preview = false;
            }
        }
    }
    // Ticket #328 (version 0.08.8): Bombard, one button per Battleship per rival Colony at the
    // Body, never over Earth; the hover carries the odds and the offence, and the button greys
    // with the reason when the orbit is not held outright.
    if body != BodyId::Earth {
        let targets: Vec<&Colony> = game.colonies.iter().filter(|c| c.body == body && c.control.director().is_some_and(|d| d != Seat(0))).collect();
        let battleships: Vec<&Ship> = ships.iter().copied().filter(|s| s.kind == UnitKind::Battleship).collect();
        if !targets.is_empty() && !battleships.is_empty() {
            ui.label(RichText::new("Bombard").strong());
            let p = game.tables.influence.destruction_chance * 100.0;
            for s in battleships {
                for c in targets.iter() {
                    let n = c.modules.iter().filter(|m| !matches!(m.kind, ModuleKind::Core | ModuleKind::Archive)).count();
                    let place = game.place_name(Place::Colony(c.id));
                    let holder = c.control.director().map(|d| game.seat_name(d)).unwrap_or_default();
                    let hover = format!("One Module of {n} at {place}, drawn at random, rolls a {p:.0}% chance to burn; when a Habitat burns, the Colonists beyond the room left die with it. A rung 3 offence against the {holder}, breaking a non-aggression Accord if one stands. Needs Orbital Control here held outright; never over Earth.");
                    cost_button_with_hover(ui, game, &session.pending, Order::Bombard { ship: s.id, colony: c.id }, &format!("Bombard {} from {}", place, game.ship_name(s)), Some(hover), actions);
                }
            }
        }
    }
    ui.label(RichText::new("Transits (whole stack)").strong());
    for to in BodyId::ALL {
        if to == body {
            continue;
        }
        // Ticket #92: the player's own figure, with the Faction's and the Tech's multipliers and a
        // Mass Driver's cut on it.
        let (turns, fuel) = game.transit_cost_for(Seat(0), body, to);
        ui.horizontal_wrapped(|ui| {
            ui.label(format!("To {}: {} turn(s), {} Fuel each from the tank", game.tables.body(to).name, turns, fuel));
            // Ticket #322 (version 0.08.8): the heading's promise kept: one button moves every
            // Ship of the stack that can pay the leg, the per-Ship buttons staying for a split.
            if ships.len() > 1 {
                let able: Vec<Order> = ships
                    .iter()
                    .map(|s| Order::Transit { ship: s.id, to, slot: None })
                    .filter(|o| game.check_order(Seat(0), &session.pending, o).is_ok())
                    .collect();
                let n = able.len();
                orders_button(ui, game, &session.pending, able, &format!("All {n} that can"), Some(format!("Every Ship of the stack whose tank pays the leg, {n} of {}, sent together.", ships.len())), actions);
            }
            for s in &ships {
                // Ticket #87: the button reads the tank against the leg.
                cost_button(ui, game, &session.pending, Order::Transit { ship: s.id, to, slot: None }, &format!("{} ({}/{} in the tank)", game.ship_name(s), s.fuel, game.tables.unit(s.kind).tank), actions);
            }
        });
    }
    // Ticket #87: a Refuel button per Ship at a Body with a station of yours, and a word for a
    // Ship that is stranded.
    ui.label(RichText::new("Tanks").strong());
    for s in &ships {
        let tank = game.tables.unit(s.kind).tank;
        ui.horizontal_wrapped(|ui| {
            // Ticket #210 (version 0.08.1): the name, with the Faction's symbol in front of it and
            // the kind and id kept on the hover -- a save, a log line and a Report all speak in ids.
            faction_glyph(ui, session, game, Some(s.seat), 16.0);
            ui.label(format!("{}: {}/{} Fuel", game.ship_name(s), s.fuel, tank))
                .on_hover_text(format!("{} {}", s.kind.name(), s.id.0));
            // Ticket #325 (version 0.08.8): or a partner's station under a Refuel Accord, named on
            // the hover; the Fuel is still the player's own Stockpile's.
            if game.refuel_station_at(Seat(0), body) {
                let partner = if game.own_station_at(Seat(0), body) {
                    None
                } else {
                    game.colonies.iter().filter(|c| c.body == body && game.fuels_for(c, Seat(0))).find_map(|c| c.control.director()).map(|d| format!("At the station of the {}, under your Refuel Accord: the Fuel is your own Stockpile's, drawn there.", game.seat_name(d)))
                };
                cost_button_with_hover(ui, game, &session.pending, Order::Refuel { ship: s.id }, "Refuel from the Stockpile", partner, actions);
            } else if game.stranded(s.id) {
                ui.colored_label(Color32::from_rgb(230, 120, 90), "stranded: no leg it can pay, and no station of yours or of a Refuel partner's here to refuel at; a station built in orbit here, or a Refuel Accord with one who holds a station here, rescues it");
            } else {
                ui.label("no station of yours, or of a Refuel partner's, here to refuel at");
            }
        });
    }
    ui.label(RichText::new("Load and unload").strong());
    for s in &ships {
        let card = game.tables.unit(s.kind);
        // Ticket #86: at Earth a warming world crowds a Colony Ship beyond its safe capacity.
        let safe = if s.kind == UnitKind::ColonyShip { game.colony_ship_capacity(Seat(0)) } else { card.carries_colonists };
        let crowd = if s.kind == UnitKind::ColonyShip && body == BodyId::Earth { game.crowd_extra() } else { 0 };
        let capacity = safe + crowd;
        if capacity == 0 && !card.carries_army {
            continue;
        }
        ui.horizontal(|ui| {
            faction_glyph(ui, session, game, Some(s.seat), 16.0);
            ui.label(format!("{}:", game.ship_name(s))).on_hover_text(format!("{} {}", s.kind.name(), s.id.0));
        });
        if crowd > 0 {
            let p = game.tables.crowding.death_chance_per_extra * 100.0;
            ui.colored_label(
                Color32::from_rgb(230, 170, 90),
                format!("+{:.1} C: {safe} ride safely, up to {capacity} may board; each one beyond {safe} risks {:.0}% per extra aboard on arrival.", game.climate.temperature, p),
            );
        }
        if capacity > s.colonists {
            let n = capacity - s.colonists;
            match body {
                BodyId::Earth => {
                    let states = game.directed_states(Seat(0));
                    // Ticket #204 (version 0.08.1): opens on a Region that can actually lift, where
                    // it opened on the most populous one whether or not it had a Launch Site or
                    // anybody waiting. Same defect, same fix, both doors. The `Some` also stands in
                    // for the emptiness check this replaced: with no directed Region there is
                    // nothing to draw from and nothing to draw.
                    if let Some(chosen) = view.load_state.filter(|x| states.contains(x)).or_else(|| default_emigrant_source(game, &states, true)) {
                        ui.horizontal(|ui| {
                            ui.label("from");
                            egui::ComboBox::from_id_salt(("load", s.id.0)).selected_text(game.tables.state(chosen).name.clone()).show_ui(ui, |ui| {
                                for st in &states {
                                    if ui.selectable_label(*st == chosen, game.tables.state(*st).name.clone()).clicked() {
                                        view.load_state = Some(*st);
                                    }
                                }
                            });
                            // Ticket #73: a Launch Site lifts the Emigrants waiting there, no more.
                            let lift = n.min(game.state(chosen).emigrants).max(1);
                            cost_button(ui, game, &session.pending, Order::Load { ship: s.id, colonists: lift, from: LoadSource::State(chosen), army: None }, &format!("Load {lift} Pioneers"), actions);
                        });
                    }
                }
                _ => {
                    for c in game.colonies.iter().filter(|c| c.body == body && c.control.director() == Some(Seat(0)) && c.colonists > 0) {
                        let k = n.min(c.colonists);
                        cost_button(ui, game, &session.pending, Order::Load { ship: s.id, colonists: k, from: LoadSource::Colony(c.id), army: None }, &format!("Load {} Colonists from {}", k, game.tables.body(c.body).slots[c.slot as usize].name), actions);
                    }
                }
            }
        }
        if card.carries_army && s.army.is_none() {
            let armies: Vec<&Army> = game
                .armies
                .iter()
                .filter(|a| game.army_seat(a) == Some(Seat(0)) && !matches!(a.home, ArmyHome::Colony(_)) && !game.army_stands_down(a))
                .filter(|a| match a.at {
                    ArmyAt::Place(Place::State(_)) => body == BodyId::Earth,
                    ArmyAt::Place(Place::Colony(c)) => game.colony(c).map(|c| c.body == body).unwrap_or(false),
                    _ => false,
                })
                .collect();
            for a in armies {
                let from = match a.at {
                    ArmyAt::Place(p) => game.place_name(p),
                    _ => String::new(),
                };
                cost_button(ui, game, &session.pending, Order::Load { ship: s.id, colonists: 0, from: LoadSource::State(StateId::EastAsia), army: Some(a.id) }, &format!("Load the Army from {from}"), actions);
            }
        }
        if s.colonists > 0 || s.army.is_some() {
            for c in game.colonies.iter().filter(|c| c.body == body) {
                let own = c.control.director() == Some(Seat(0));
                if s.colonists > 0 && own {
                    let room = game.habitat_room(c).saturating_sub(c.colonists);
                    let k = s.colonists.min(room);
                    if k > 0 {
                        cost_button(ui, game, &session.pending, Order::Unload { ship: s.id, colonists: k, army: false, into: UnloadTarget::Colony(c.id) }, &format!("Unload {} Colonists into {}", k, game.tables.body(c.body).slots[c.slot as usize].name), actions);
                    }
                }
                if let Some(aid) = s.army {
                    // Ticket #300 (version 0.08.6): the attack happens the turn it lands, so the
                    // button carries the first-round odds as the march buttons do. Ticket #309
                    // (version 0.08.7): on a hover that names the defender, not on the face.
                    let slot_name = &game.tables.body(c.body).slots[c.slot as usize].name;
                    let (label, hover) = if own {
                        (format!("Land the Army at {slot_name}"), format!("{slot_name}: held by you. Landing costs nothing; the Army lands on Hold."))
                    } else {
                        let mine = game.army(aid).map(|a| game.army_strength(a)).unwrap_or(0);
                        (format!("Land the Army to attack {slot_name}"), attack_hover(game, Place::Colony(c.id), slot_name, mine, true))
                    };
                    cost_button_with_hover(ui, game, &session.pending, Order::Unload { ship: s.id, colonists: 0, army: true, into: UnloadTarget::Colony(c.id) }, &label, Some(hover), actions);
                }
            }
            if s.kind == UnitKind::ColonyShip && s.colonists > 0 && body != BodyId::Earth {
                // Ticket #56: Antarctica's slots are shut until the ice opens.
                if body == BodyId::Earth && !game.antarctica_open {
                    ui.label(
                        RichText::new(format!("Antarctica is under the ice: its Colony Slots open at {:+.1} C.", game.tables.climate.antarctica_opens_at))
                            .color(Color32::from_rgb(150, 195, 235)),
                    );
                }
                for slot in game.free_slots_on(body).into_iter().filter(|_| body != BodyId::Earth || game.antarctica_open) {
                    // Both founding doors read the same, at the designer's word: the same decision
                    // reached two ways should not want learning twice.
                    let order = Order::Unload { ship: s.id, colonists: s.colonists, army: s.army.is_some(), into: UnloadTarget::Slot(body, slot) };
                    let label = format!("Found a Colony at {}", game.tables.body(body).slots[slot as usize].name);
                    if found_button(ui, &game.slot_yields(body, slot), &label).clicked() {
                        actions.push(Action::Place(order));
                    }
                }
            }
        }
        if s.damage > 0 {
            cost_button(ui, game, &session.pending, Order::Repair { unit: UnitRef::Ship(s.id), points: s.damage }, "Repair fully", actions);
            cost_button(ui, game, &session.pending, Order::RepairWithDucats { unit: UnitRef::Ship(s.id), points: s.damage }, "Repair fully with Ducats", actions);
        }
    }
    if body != BodyId::Earth {
        icon_word(ui, "influence", "Influence on Colonies here");
        for c in game.colonies.iter().filter(|c| c.body == body) {
            ui.label(game.place_name(Place::Colony(c.id)));
            influence_row(ui, game, session, view, Place::Colony(c.id), true, actions);
        }
    }
}

// ------------------------------------------------------------------ popups

/// Ticket #42: the trading window. Ducats buy Influence, Materials, Fuel and Energy at the table
/// prices, spendable in this turn's orders; Materials and Fuel sell back at half; buildings are
/// bought for Ducats from their own build buttons.
fn trading_window(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, actions: &mut Vec<Action>) {
    let (left, _) = game.remaining(Seat(0), &session.pending);
    ui.label(RichText::new(format!("Ducats {} to spend this turn (+{} last Income).", left.ducats, game.seat(Seat(0)).income_last_turn.ducats)).strong());
    ui.label("What you buy is yours at once, for this turn's orders. Ducats come from your Regions' economies, Banks and Trade Posts.");
    ui.separator();
    let lines: [(usize, Option<dying_earth_engine::Resource>, &str); 4] = [(0, None, "Influence"), (1, Some(dying_earth_engine::Resource::Materials), "Materials"), (2, Some(dying_earth_engine::Resource::Fuel), "Fuel"), (3, Some(dying_earth_engine::Resource::Energy), "Energy")];
    egui::Grid::new("trade_grid").num_columns(5).spacing((12.0, 6.0)).show(ui, |ui| {
        ui.label(RichText::new("Line").strong());
        ui.label(RichText::new("Price").strong());
        ui.label(RichText::new("Quantity").strong());
        ui.label(RichText::new("Buy").strong());
        ui.label(RichText::new("Sell").strong());
        ui.end_row();
        for (i, res, name) in lines {
            let per = match res {
                None => game.tables.ducats.per_influence,
                Some(r) => game.trade_price(r).unwrap_or(0),
            };
            ui.label(name);
            let sells = matches!(res, Some(dying_earth_engine::Resource::Materials) | Some(dying_earth_engine::Resource::Fuel));
            // Ticket #83: the Prospectors' 15% off is taken over the lot, so the button's figure is
            // the price; the line says so.
            let off = game.tables.faction(game.kind(Seat(0))).market_multiplier;
            let discount = if res.is_some() && off != 1.0 { format!(" (x{off} for you, over the lot)") } else { String::new() };
            ui.label(if sells { format!("{per} Ducats each; sells for {:.1}{discount}", per as f64 / game.tables.ducats.sell_divisor.max(1) as f64) } else { format!("{per} Ducats each{discount}") });
            ui.add(egui::DragValue::new(&mut view.trade_amounts[i]).range(1..=999).speed(1.0));
            let n = view.trade_amounts[i].max(1);
            let buy = match res {
                None => Order::BuyInfluence { amount: n },
                Some(r) => Order::Buy { resource: r, amount: n },
            };
            let cost = game.order_cost(Seat(0), &buy).ducats;
            let ok = game.check_order(Seat(0), &session.pending, &buy);
            let mut resp = ui.add_enabled(ok.is_ok(), egui::Button::new(format!("Buy for {cost} Ducats")));
            if let Err(e) = &ok {
                resp = resp.on_disabled_hover_text(&e.0);
            }
            if resp.clicked() {
                actions.push(Action::Place(buy));
            }
            if let Some(r) = res.filter(|_| sells) {
                let sell = Order::Sell { resource: r, amount: n };
                let gain = -game.order_cost(Seat(0), &sell).ducats;
                let ok = game.check_order(Seat(0), &session.pending, &sell);
                let mut resp = ui.add_enabled(ok.is_ok(), egui::Button::new(format!("Sell for {gain} Ducats")));
                if let Err(e) = &ok {
                    resp = resp.on_disabled_hover_text(&e.0);
                }
                if resp.clicked() {
                    actions.push(Action::Place(sell));
                }
            } else {
                ui.label(RichText::new("not bought back").weak());
            }
            ui.end_row();
        }
    });
    ui.separator();
    // Ticket #285 (version 0.08.5): the carbon-credit line is gone from here; the Custodians offer
    // from their own Faction page and everyone else requests from theirs.
    ui.separator();
    ui.label(format!("Buildings: every build button on a Region or Colony card has an \"or\" beside it that buys the building outright for Ducats, at {} times its Materials cost.", game.tables.ducats.per_building_material));
    let trades: Vec<String> = session.pending.iter().filter(|o| matches!(o, Order::Buy { .. } | Order::Sell { .. } | Order::BuyInfluence { .. } | Order::BuildFacilityWithDucats { .. } | Order::BuildModuleWithDucats { .. } | Order::BuyCredits { .. } | Order::OfferCredits { .. })).map(|o| order_text(game, o)).collect();
    if !trades.is_empty() {
        ui.separator();
        ui.label(RichText::new("Trades this turn (undo them in the orders list)").strong());
        for t in trades {
            ui.label(t);
        }
    }
}

/// Ticket #268 (version 0.08.4): **carbon credits**. Ticket #285 (version 0.08.5): off the Trading
/// window and onto the Faction window, at the designer's word -- the Custodians OFFER from their own
/// page, beside the Blame line whose credit they sell and the Greenwash that is its neighbour, and
/// everyone else REQUESTS from the Custodians' page, under the Accords and the Smear, the things one
/// asks of that Faction. The rule did not move: a request is filled at End Turn from the standing
/// offer, or refunded where the offer ran out.
fn credits_hover(game: &Game) -> String {
    let c = &game.tables.carbon_credits;
    format!(
        "A ppm of carbon credit bought comes off your Blame for good. The Custodians sell it at {} Ducats a ppm, times how they think of you -- Friendly x{}, Cordial x{}, Neutral x{}, Wary x{}, Cold x{}; Hostile refuses -- up to {} ppm a turn.\nA purchase is an act of friendship both ways.",
        c.price_per_ppm, c.friendly, c.cordial, c.neutral, c.wary, c.cold, c.cap_per_turn
    )
}

/// The seller's side, on the Custodians' own page: the ppm they offer a turn, standing until changed.
fn credits_offer_block(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, actions: &mut Vec<Action>) {
    let me = Seat(0);
    ui.label(RichText::new("Carbon credits").strong()).on_hover_text(credits_hover(game));
    let credit = game.blame_credit(me);
    let standing = game.seat(me).credits_offered;
    let pending = session.pending.iter().find_map(|o| if let Order::OfferCredits { ppm } = o { Some(*ppm) } else { None });
    ui.label(format!(
        "You hold {credit:.0} ppm in credit. You offer {standing} ppm a turn{}; what you sell past your credit goes onto your own Blame.",
        pending.map(|p| format!(" ({p} from next turn)")).unwrap_or_default()
    ));
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut view.credits_offer).range(0..=999));
        let order = Order::OfferCredits { ppm: view.credits_offer };
        let check = game.check_order(me, &session.pending, &order);
        let resp = ui.add_enabled(check.is_ok(), egui::Button::new(format!("Offer {} ppm a turn", view.credits_offer)));
        if let Err(e) = &check {
            resp.clone().on_disabled_hover_text(&e.0);
        }
        if resp.on_hover_text("Stands from next turn until you set it again; nought refuses everyone.").clicked() {
            actions.push(Action::Place(order));
        }
    });
}

/// The buyer's side, on the Custodians' page: what they offer this turn, how they think of you and
/// the price that makes, and a Request button up to the cap. Their view of you can refuse outright.
fn credits_request_block(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, seller: Seat, actions: &mut Vec<Action>) {
    let me = Seat(0);
    let c = game.tables.carbon_credits.clone();
    ui.label(RichText::new("Carbon credits").strong()).on_hover_text(credits_hover(game));
    let offer = game.seat(seller).credits_offered;
    let level = game.relations_level(seller, me);
    match game.credit_price_multiplier(me) {
        None => {
            ui.label(RichText::new(format!("The Custodians will not sell to you: they are {level} toward you.")).weak());
        }
        Some(m) if offer <= 0 => {
            ui.label(RichText::new(format!("The Custodians are not selling this turn. They are {level} toward you (x{m}).")).weak());
        }
        Some(m) => {
            ui.label(format!("The Custodians offer {offer} ppm this turn. They are {level} toward you, so a ppm costs {} Ducats (x{m}).", game.credit_cost(me, 1).unwrap_or(0)));
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut view.credits_amount).range(1..=c.cap_per_turn.max(1)));
                let order = Order::BuyCredits { ppm: view.credits_amount };
                let cost = game.order_cost(me, &order).ducats;
                let check = game.check_order(me, &session.pending, &order);
                let resp = ui.add_enabled(check.is_ok(), egui::Button::new(format!("Request {} ppm for {cost} Ducats", view.credits_amount)));
                if let Err(e) = &check {
                    resp.clone().on_disabled_hover_text(&e.0);
                }
                if resp.on_hover_text("Off your Blame at End Turn. If others request first and the offer runs out, the Ducats for what you did not get come back.").clicked() {
                    actions.push(Action::Place(order));
                }
            });
        }
    }
}

/// Ticket #41: the Tech Tree drawn as a tree, a line from every Tech to each Tech that needs it,
/// each box coloured by its state. Ticket #133 (version 0.07.3) transposed it: **one row per
/// branch, one column per rung**, so time runs left to right the way a tree is read, the branch
/// names as row headings down the left edge. The designer's line: *"Transpose tech tree."*
fn tech_tree(ui: &mut Ui, game: &Game, available: &[TechId], must_pick: bool, actions: &mut Vec<Action>) {
    // Ticket #242 (version 0.08.3): every figure here is a TENTH SMALLER than it was, at the
    // designer's word -- "let's reduce the size of the tech boxes by 10%". The spacing went with
    // the boxes deliberately: the tree's height is `rows x ROW`, so shrinking the boxes alone would
    // have put more air around smaller boxes and saved not one pixel, which is not what the change
    // was for. Ticket #232's two new Techs had taken the tree from 672 pixels to 864; at nine
    // tenths it is 774, which clears 1080 with room and leaves less to scroll at 800.
    //
    // The two font sizes below came down with them, 13 to 12 and 11 to 10, and the text offsets
    // inside a box with them. Box text is drawn centred and is NOT clipped, so a name that no
    // longer fits spills over the box edge rather than being cut -- "Closed-Loop Colonies" and
    // "The Extraction Charter" are the two that would have shown it.
    const COL: f32 = 144.0;
    const ROW: f32 = 86.0;
    const BOX_W: f32 = 122.0;
    const BOX_H: f32 = 58.0;
    /// The row-heading column on the left, wide enough for "Off-world Living".
    const HEAD_W: f32 = 128.0;
    // Ticket #250 (version 0.08.3): the order the BANDS are drawn in, settled over five turns of
    // the designer looking at the tree -- #247 gave Extraction its place under Industry, #248
    // lifted Off-world Living to the top, #249 set the middle three, and this is the last word:
    // *"now swap industry and extraction and move society to the top"*.
    //
    // It happens to undo the damage #249 counted. Society and Off-world Living are ADJACENT again,
    // and those are the two bands the Closed-Loop Colonies -> The Upload edge runs between, so the
    // edge is a hop between neighbours rather than a line down the whole tree behind four boxes.
    //
    // A branch not named here keeps its first-appearance place, after the named ones, so a new
    // branch cannot vanish by being forgotten.
    const BAND_ORDER: [&str; 5] = ["Society", "Off-world Living", "Extraction", "Industry", "Propulsion"];
    let mut branches: Vec<String> = Vec::new();
    for t in TechId::ALL {
        let b = &game.tables.tech(t).branch;
        if !branches.contains(b) {
            branches.push(b.clone());
        }
    }
    branches.sort_by_key(|b| BAND_ORDER.iter().position(|x| x == b).unwrap_or(BAND_ORDER.len()));
    let rungs = TechId::ALL.iter().map(|t| game.tables.tech(*t).rung).max().unwrap_or(1).max(1) as usize;
    // Ticket #56: a branch may hold more than one Tech on a rung (Efficient Grids and Coastal
    // Engineering both sit on Industry 1). Ticket #133: they sat SIDE BY SIDE. Ticket #156
    // (version 0.07.4): they sit side by side only on the LAST rung, where nothing leaves them
    // (Planetary Stewardship beside The Upload); on any earlier rung they are STACKED in a taller
    // branch row, so every column is one box wide, every first-rung box lines up, and a line out
    // of Efficient Grids leaves its right edge instead of running beneath Coastal Engineering.
    // The designer: *"adjust costal engineering in the tech tree."*
    let cell: Vec<Vec<TechId>> = (0..branches.len() * rungs)
        .map(|i| {
            let (b, r) = (i % branches.len(), i / branches.len());
            TechId::ALL
                .into_iter()
                .filter(|t| {
                    let card = game.tables.tech(*t);
                    branches.iter().position(|x| *x == card.branch) == Some(b) && card.rung.max(1) as usize - 1 == r
                })
                .collect()
        })
        .collect();
    // Ticket #250 (version 0.08.3): EVERY rung stacks, the last one included. It did not before --
    // `r + 1 < rungs` left the final rung laying its boxes side by side -- and the consequence was
    // that a band with two rung-3 Techs split the column between them, so Society's Planetary
    // Stewardship and The Upload sat at two x positions that no other band's rung-3 box shared.
    //
    // The designer: *"there is no reason upload needs to be on the same line as society ... and I
    // want all level three techs to appear on the same column"*. Stacking gives both at once: The
    // Upload takes a row of its own inside the Society band, and every rung-3 box in the tree now
    // sits at `left[2] + COL / 2`, one column.
    //
    // It costs nothing in height -- a band is already as tall as its fullest stacked cell, and
    // Society's rung 2 already held two -- and it makes the tree one COL narrower, since the last
    // rung no longer claims width for the widest cell in it.
    let stacked = |_r: usize| true;
    let span: Vec<f32> = (0..rungs).map(|r| if stacked(r) { 1.0 } else { (0..branches.len()).map(|b| cell[r * branches.len() + b].len()).max().unwrap_or(1).max(1) as f32 }).collect();
    let left: Vec<f32> = (0..rungs).map(|r| HEAD_W + span[..r].iter().sum::<f32>() * COL).collect();
    let width: f32 = HEAD_W + span.iter().sum::<f32>() * COL;
    // A branch's band is as many rows tall as its tallest stacked cell.
    let rows: Vec<f32> = (0..branches.len()).map(|b| (0..rungs).filter(|r| stacked(*r)).map(|r| cell[r * branches.len() + b].len()).max().unwrap_or(1).max(1) as f32).collect();
    let top: Vec<f32> = (0..branches.len()).map(|b| rows[..b].iter().sum::<f32>() * ROW).collect();
    let height: f32 = rows.iter().sum::<f32>() * ROW;
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    let painter = ui.painter_at(rect);
    let box_of = |t: TechId| -> egui::Rect {
        let card = game.tables.tech(t);
        let b = branches.iter().position(|x| *x == card.branch).unwrap_or(0);
        let r = card.rung.max(1) as usize - 1;
        let here = &cell[r * branches.len() + b];
        let i = here.iter().position(|x| *x == t).unwrap_or(0) as f32;
        // A stack fills its band row by row; a lone box, on any rung, stands at the band's middle.
        let (centre, y) = if stacked(r) && here.len() > 1 {
            (left[r] + COL / 2.0, top[b] + i * ROW + (ROW - BOX_H) / 2.0)
        } else {
            let each = span[r] * COL / here.len().max(1) as f32;
            (left[r] + each * (i + 0.5), top[b] + (rows[b] * ROW - BOX_H) / 2.0)
        };
        egui::Rect::from_min_size(rect.min + egui::vec2(centre - BOX_W / 2.0, y), egui::vec2(BOX_W, BOX_H))
    };
    for (i, b) in branches.iter().enumerate() {
        painter.text(rect.min + egui::vec2(HEAD_W - 12.0, top[i] + rows[i] * ROW / 2.0), egui::Align2::RIGHT_CENTER, b, FontId::proportional(14.0), Color32::WHITE);
    }
    // Lines first, so the boxes sit on top of them. A line is green once the Tech it comes from is done.
    // Ticket #133: a line is ELBOWED -- it leaves the needed box, runs along the gap to the left of
    // the needing box's column, and enters the needing box's left edge -- so it never crosses a
    // box. A Tech that needs one on its own rung (Closed-Loop Colonies needs Clean Power) is
    // reached the same way: out of the needed box's LEFT edge, down that same gap, and in.
    for t in TechId::ALL {
        for n in &game.tables.tech(t).needs {
            let to_box = box_of(t);
            let from_box = box_of(*n);
            let to = to_box.left_center();
            // Each source row takes its own lane in the gap, or every line into a column merges
            // into one trunk and nobody can tell which Tech feeds which (the first picture).
            let lane = branches.iter().position(|x| *x == game.tables.tech(*n).branch).unwrap_or(0) as f32;
            let gap_x = to_box.min.x - 4.0 - lane * 3.5;
            let colour = if game.research.done.contains(n) { Color32::from_rgb(120, 200, 120) } else { Color32::from_gray(150) };
            let stroke = egui::Stroke::new(2.0, colour);
            // A box standing between the needed box and the lane (Coastal Engineering beside
            // Efficient Grids) would have the line run behind it and seem to feed the target
            // itself; so the line leaves that box's bottom instead, runs along the row gap, and
            // only then climbs the lane.
            let between = TechId::ALL.iter().any(|o| {
                let ob = box_of(*o);
                *o != *n && ob.min.x >= from_box.max.x && ob.max.x <= gap_x && (ob.center().y - from_box.center().y).abs() < 1.0
            });
            let from = if between {
                let start = from_box.center_bottom();
                let row_gap_y = from_box.max.y + (ROW - BOX_H) / 4.0;
                painter.line_segment([start, Pos2::new(start.x, row_gap_y)], stroke);
                painter.line_segment([Pos2::new(start.x, row_gap_y), Pos2::new(gap_x, row_gap_y)], stroke);
                Pos2::new(gap_x, row_gap_y)
            } else if from_box.max.x < gap_x {
                from_box.right_center()
            } else {
                from_box.left_center()
            };
            painter.line_segment([from, Pos2::new(gap_x, from.y)], stroke);
            painter.line_segment([Pos2::new(gap_x, from.y), Pos2::new(gap_x, to.y)], stroke);
            painter.line_segment([Pos2::new(gap_x, to.y), to], stroke);
            painter.circle_filled(to, 3.5, colour);
        }
    }
    for t in TechId::ALL {
        let card = game.tables.tech(t);
        let r = box_of(t);
        let (fill, status) = if game.research.done.contains(&t) {
            (Color32::from_rgb(50, 120, 60), "done")
        } else if game.research.current == Some(t) && !game.research.pick_committed {
            // Ticket #173 (version 0.07.6): picked this turn, and still changeable until the turn
            // ends. A paler amber than the settled one. The caption stays one word because the box
            // is only as wide as "cost 45 - locked"; the prompt above the tree carries the rest.
            (Color32::from_rgb(120, 95, 35), "chosen")
        } else if game.research.current == Some(t) {
            (Color32::from_rgb(170, 130, 30), "under research")
        } else if available.contains(&t) {
            (Color32::from_rgb(40, 90, 160), "available")
        } else {
            (Color32::from_gray(60), "locked")
        };
        // Ticket #84: a Victory gate wears its Faction's colour as a thick border.
        let stroke = match card.gate_for {
            Some(k) => egui::Stroke::new(3.0, rgb(game.tables.faction(k).colour)),
            None => egui::Stroke::new(1.0, Color32::from_gray(200)),
        };
        painter.rect(r, 6.0, fill, stroke, egui::StrokeKind::Inside);
        painter.text(r.center_top() + egui::vec2(0.0, 13.0), egui::Align2::CENTER_CENTER, &card.name, FontId::proportional(12.0), Color32::WHITE);
        painter.text(r.center_top() + egui::vec2(0.0, 29.0), egui::Align2::CENTER_CENTER, format!("cost {} - {}", card.cost, status), FontId::proportional(10.0), Color32::from_gray(230));
        let needs = if card.needs.is_empty() { "nothing".to_string() } else { card.needs.iter().map(|n| game.tables.tech(*n).name.clone()).collect::<Vec<_>>().join(" and ") };
        ui.interact(r, ui.id().with(format!("tech-{t:?}")), egui::Sense::hover()).on_hover_text(format!("{} (rung {}, cost {} Research)\n{}\nNeeds: {}", card.name, card.rung, card.cost, card.effect, needs));
        if must_pick && available.contains(&t) && game.research.current != Some(t) {
            let b = egui::Rect::from_center_size(r.center_bottom() - egui::vec2(0.0, 10.0), egui::vec2(50.0, 16.0));
            if ui.put(b, egui::Button::new(RichText::new("Pick").size(10.0))).clicked() {
                actions.push(Action::PickTech(t));
            }
        }
    }
    // Ticket #194 (version 0.08.0): put the layout cursor back where the tree's ALLOCATION left it
    // before the legend is drawn. The tree paints every box with `painter_at` at absolute
    // coordinates, but each Pick button is placed with `ui.put()`, which advances the Ui's cursor --
    // so the legend below used to start from wherever the last Pick button happened to be, which is
    // INSIDE the tree, about a third of the way down and lying across the connector lines. The
    // painted tree never noticed. That is also why it read as wonky *when picking a Tech*: the
    // legend's position was decided by which boxes currently carry a Pick button, and picking,
    // unpicking or changing a pick is exactly what changes that set. Diagnosed from a picture; the
    // reading of this code made while the ticket was charted was wrong.
    ui.advance_cursor_after_rect(rect);
}

/// Ticket #219 (version 0.08.2): the legend, lifted out of the tree's tail so it can be drawn
/// DIRECTLY BENEATH the race bar at the top of the window. The designer asked for the bar "below the
/// legend"; the literal reading was ruled out by measurement, since this window is `resizable(false)`
/// around a fixed-size tree and does not scroll, so anything below the legend makes the WINDOW
/// taller -- and at the default 1280x800 it already reaches the bottom of the screen. Moving the
/// legend up satisfies "the bar, then the legend" and costs no height at all.
fn tech_legend(ui: &mut Ui) {
    ui.horizontal(|ui| {
        // Ticket #173 (version 0.07.6): the paler amber of a pick that can still change earns its own
        // swatch, next to the settled amber it must be told apart from.
        for (colour, label) in [(Color32::from_rgb(50, 120, 60), "done"), (Color32::from_rgb(170, 130, 30), "under research"), (Color32::from_rgb(120, 95, 35), "chosen this turn"), (Color32::from_rgb(40, 90, 160), "available"), (Color32::from_gray(60), "locked")] {
            let (sw, _) = ui.allocate_exact_size(egui::vec2(14.0, 14.0), egui::Sense::hover());
            ui.painter().rect_filled(sw, 3.0, colour);
            ui.label(label);
        }
    });
}

/// Ticket #219 (version 0.08.2): under the race bar, one entry per Faction that has put Research
/// into the Tech under research -- its symbol in its own colour, its name, and its SHARE OF WHAT HAS
/// BEEN CONTRIBUTED SO FAR, the entries adding to 100%. That is the figure the bar's segment widths
/// already draw; a percentage disagreeing with the width above it would be worse than none, which is
/// why this is not a share of the Tech's whole cost. The `N of M` progress is unchanged.
///
/// A Faction that has contributed NOTHING is not shown at all, at the designer's word: its absence
/// from this line and the grey remainder on the bar say the same thing twice. The symbol rather than
/// a plain swatch, since 0.08.1 wears it wherever something belongs to somebody -- which also keeps
/// the legend's swatches beneath meaning STATES rather than OWNERS.
fn research_shares(ui: &mut Ui, session: &Session, game: &Game) {
    let c = game.research.contributions;
    let total: i64 = c.iter().map(|v| (*v).max(0)).sum();
    if total <= 0 {
        return;
    }
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        for seat in Seat::ALL {
            let v = c[seat.index()].max(0);
            if v == 0 {
                continue;
            }
            faction_glyph(ui, session, game, Some(seat), 15.0);
            let pct = (v as f64 * 100.0 / total as f64).round() as i64;
            ui.label(RichText::new(format!("{} {}%", game.seat_name(seat), pct)).size(13.0).color(seat_colour(session, seat)).strong());
            ui.add_space(10.0);
        }
    });
}

/// Ticket #58: the Moments corner at the foot of the Report. A checkbox per kind, remembered for
/// the session; `report.toml` holds the defaults.
fn moments_corner(ui: &mut Ui, session: &Session, view: &mut ViewState) {
    egui::CollapsingHeader::new("Moments").id_salt("moments_corner").show(ui, |ui| {
        ui.label(RichText::new("A Moment stops the turn for one sentence and one number before this Report. At most two a turn, the most serious first.").weak());
        let mut on: [bool; MomentKind::ALL.len()] =
            std::array::from_fn(|i| view.moment_on(&session.tables, MomentKind::ALL[i]));
        let before = on;
        for (i, kind) in MomentKind::ALL.into_iter().enumerate() {
            ui.checkbox(&mut on[i], kind.name());
        }
        if on != before || view.moments_on.is_some() {
            view.moments_on = Some(on);
        }
    });
}

// ------------------------------------------------------------------ Ticket #145: the Hab View

/// The side of a Module or build-slot tile and the gap between tiles. Ticket #162 (version 0.07.5)
/// retired `HAB_COLS`, the Hab View window's five: both cards lay their tiles out in `SLOT_COLS`
/// columns now.
/// Ticket #217 (version 0.08.2): the `Play Tutorial` tick's text size on the Faction screen, a fifth
/// larger than egui's 14-pixel body default at the designer's word. Named rather than written as
/// 16.8 in place, so the next such request is one number -- as ticket #211's command cluster was.
const TUTORIAL_TICK: f32 = 14.0 * 1.2;

const HAB_TILE: f32 = 84.0;
const HAB_GAP: f32 = 10.0;
/// Room under a tile for its name.
const HAB_LABEL: f32 = 18.0;

/// What a tile shows.
#[derive(Clone, Copy, PartialEq, Eq)]
enum TileState {
    Standing,
    Mothballed,
    /// Ticket #307 (version 0.08.7): a building that stands and is not mothballed but is not
    /// online -- short of Energy, struck by a card, in a grid-failed Colony, or an Occupied
    /// Colony's Archive. Drawn with the mothballed tile's dimming and the word *offline* in a
    /// warmer colour than mothballed's grey, at the designer's word; the cause is on the hover.
    /// Until this ticket such a building was drawn exactly like a working one.
    Offline,
    /// Ticket #291 (version 0.08.6): a building ordered this turn (`ordered`, before End Turn) or
    /// under way (after it). Both are drawn hatched AND dimmed, with a count on the face, at the
    /// designer's word: *"hatch stays but greyed out with turns to complete indicated."* Until
    /// that ticket an ordered building did not show in its box at all, and a building one was
    /// drawn bright under the hatch with no count.
    /// Ticket #332 (version 0.09.0): the count is `3 of 8` -- the Widgets done of the figure --
    /// in place of the turns, at the designer's word; an ordered one reads `0 of 8`. The estimate
    /// in turns is on the hover.
    Building { ordered: bool, done: u32, widgets: u32 },
    /// Ticket #218 (version 0.08.2): the flag says whether the PLAYER could actually build here --
    /// their own place, and not a slot the sea has taken. A tile they can use invites the click;
    /// one they cannot keeps the old word, because telling somebody to click a thing that will do
    /// nothing is worse than the word it replaced.
    Free(bool),
    /// Ticket #146: a coastal slot the sea has taken, drawn under water.
    Flooded,
}

/// Ticket #145 (version 0.07.3): one tile of the Hab View -- a picture on a dark tile with its name
/// beneath: dimmed while mothballed, hatched while building, dashed and empty for a free slot. Returns
/// the click response so the window can open the strip for it. Since ticket #150 (version 0.07.4)
/// the tile carries a hover, `tip`, through `rule_tip` like every other tooltip in the game.
#[allow(clippy::too_many_arguments)]
fn hab_tile(ui: &mut Ui, rect: egui::Rect, id: egui::Id, key: Option<&str>, name: &str, state: TileState, selected: bool, edge: Option<Color32>, tip: String) -> egui::Response {
    let resp = ui.interact(rect, id, egui::Sense::click());
    let painter = ui.painter();
    // Ticket #146: a slot box on a Region's card carries the coast's blue as its edge; a tile in
    // the Hab View has none of its own. The selection and the hover still win over it.
    let outline = if selected {
        Color32::from_rgb(250, 210, 130)
    } else if resp.hovered() {
        Color32::from_gray(200)
    } else {
        edge.unwrap_or(Color32::from_gray(120))
    };
    match state {
        TileState::Free(yours) => {
            // A dashed border, four sides of short strokes, and the word in the middle.
            let dash = 5.0;
            let step = 9.0;
            let stroke = egui::Stroke::new(1.0, outline);
            let mut x = rect.min.x;
            while x < rect.max.x {
                let x2 = (x + dash).min(rect.max.x);
                painter.line_segment([Pos2::new(x, rect.min.y), Pos2::new(x2, rect.min.y)], stroke);
                painter.line_segment([Pos2::new(x, rect.max.y), Pos2::new(x2, rect.max.y)], stroke);
                x += step;
            }
            let mut y = rect.min.y;
            while y < rect.max.y {
                let y2 = (y + dash).min(rect.max.y);
                painter.line_segment([Pos2::new(rect.min.x, y), Pos2::new(rect.min.x, y2)], stroke);
                painter.line_segment([Pos2::new(rect.max.x, y), Pos2::new(rect.max.x, y2)], stroke);
                y += step;
            }
            // Ticket #218 (version 0.08.2): a tile the player can build in says so. Two lines: the
            // tile is 84 square and `Click to Build` is about 78 wide at 12pt, so one line would
            // leave three pixels of air and break if the font ever moved.
            let (word, ink) = if yours { ("Click to
Build", Color32::from_gray(165)) } else { ("free", Color32::from_gray(130)) };
            painter.text(rect.center(), egui::Align2::CENTER_CENTER, word, FontId::proportional(12.0), ink);
        }
        _ => {
            // Ticket #291 (version 0.08.6): a building ordered or under way takes the mothballed
            // tile's darker fill and dimmed picture as well as its hatch.
            let dim = matches!(state, TileState::Mothballed | TileState::Offline | TileState::Flooded | TileState::Building { .. });
            let fill = if matches!(state, TileState::Mothballed | TileState::Offline | TileState::Building { .. }) { Color32::from_rgb(36, 36, 42) } else { Color32::from_rgb(48, 48, 58) };
            painter.rect(rect, 6.0, fill, egui::Stroke::new(if edge.is_some() { 2.0 } else { 1.0 }, outline), egui::StrokeKind::Inside);
            if let Some(image) = key.and_then(|k| Icons::from_ctx(ui.ctx(), k, 48.0)) {
                let tint = if dim { crate::icons::kind_fill().gamma_multiply(0.4) } else { crate::icons::kind_fill() };
                let art = egui::Rect::from_center_size(rect.center() - egui::vec2(0.0, 4.0), egui::vec2(48.0, 48.0));
                image.tint(tint).paint_at(ui, art);
            }
            if let TileState::Building { ordered, done, widgets } = state {
                // Hatched, clipped to the tile, with the word in the bottom-left corner and the
                // count in the top-right, where neither crosses the picture: the picture spans
                // the tile's middle 48 pixels and each corner word is one 11pt line.
                let clipped = ui.painter().with_clip_rect(rect);
                let stroke = egui::Stroke::new(2.0, Color32::from_rgba_unmultiplied(200, 170, 90, 110));
                let mut k = -rect.width();
                while k < rect.width() {
                    clipped.line_segment([Pos2::new(rect.min.x + k, rect.max.y), Pos2::new(rect.min.x + k + rect.width(), rect.min.y)], stroke);
                    k += 8.0;
                }
                let gold = Color32::from_rgb(250, 210, 130);
                clipped.text(rect.left_bottom() + egui::vec2(4.0, -4.0), egui::Align2::LEFT_BOTTOM, if ordered { "ordered" } else { "building" }, FontId::proportional(11.0), gold);
                clipped.text(rect.right_top() + egui::vec2(-4.0, 4.0), egui::Align2::RIGHT_TOP, format!("{done} of {widgets}"), FontId::proportional(11.0), gold);
            }
            if state == TileState::Mothballed {
                ui.painter().text(rect.left_bottom() + egui::vec2(4.0, -4.0), egui::Align2::LEFT_BOTTOM, "mothballed", FontId::proportional(11.0), Color32::from_rgb(170, 170, 190));
            }
            if state == TileState::Offline {
                ui.painter().text(rect.left_bottom() + egui::vec2(4.0, -4.0), egui::Align2::LEFT_BOTTOM, "offline", FontId::proportional(11.0), Color32::from_rgb(225, 165, 115));
            }
            if state == TileState::Flooded {
                // Ticket #146 (version 0.07.3): the sea. The designer: *"flooded tiles to filled 3/4th
                // with a blue transparency and wave like billows along the top the transparency
                // overlay such that it looks like water."* Three quarters of the box under
                // translucent blue, the top edge a sine of two billows with a bright crest and a
                // fainter one a little below it, the drowned building dimmed beneath.
                let inner = rect.shrink(1.0);
                let top = inner.min.y + inner.height() * 0.25;
                let amp = 3.0;
                let water = Color32::from_rgba_unmultiplied(60, 130, 230, 120);
                let p = ui.painter().with_clip_rect(inner);
                p.rect_filled(egui::Rect::from_min_max(Pos2::new(inner.min.x, top + amp), inner.max), 0.0, water);
                let n = inner.width().round() as usize;
                let wave = |px: usize, phase: f32, a: f32, base: f32| Pos2::new(inner.min.x + px as f32, base + a * (px as f32 / inner.width() * std::f32::consts::TAU * 2.0 + phase).sin());
                let crest: Vec<Pos2> = (0..=n).map(|px| wave(px, 0.0, amp, top)).collect();
                for c in &crest {
                    p.line_segment([*c, Pos2::new(c.x, top + amp + 0.5)], egui::Stroke::new(1.0, water));
                }
                p.add(egui::Shape::line(crest, egui::Stroke::new(2.0, Color32::from_rgba_unmultiplied(150, 200, 255, 200))));
                let second: Vec<Pos2> = (0..=n).map(|px| wave(px, 1.2, 2.5, top + 9.0)).collect();
                p.add(egui::Shape::line(second, egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(120, 180, 255, 90))));
                p.text(inner.left_bottom() + egui::vec2(4.0, -4.0), egui::Align2::LEFT_BOTTOM, "lost to the sea", FontId::proportional(11.0), Color32::from_rgb(180, 210, 255));
            }
        }
    }
    if !name.is_empty() {
        ui.painter().text(rect.center_bottom() + egui::vec2(0.0, 3.0), egui::Align2::CENTER_TOP, name, FontId::proportional(12.0), Color32::from_gray(225));
    }
    if tip.is_empty() { resp } else { rule_tip(resp, tip) }
}

/// Ticket #145 (version 0.07.3): **the Hab View**, a station's or Colony's Modules as a grid of
/// tiles. The designer: *"I would like a window popup showing the modules of the space stations and
/// colonies similar the ones in terra invicta."* Five columns; one tile per Module standing
/// (dimmed while mothballed), one hatched tile per Module building, one dashed tile per free place
/// under the cap, and the Archive on a tile of its own outside the count. A click on a tile puts
/// that Module's figures and its Mothball, Restart and Decommission buttons in the strip under the
/// grid; a click on a free tile puts the build buttons there. The card keeps its summary line and
/// its own build buttons; the text rows that stood there have moved in here.
/// A Module's line: its name and figures, or the sentence a mothballed, idle or offline one shows
/// instead, and the Archive's fund. Read by the Hab View's strip and, since ticket #150 (version
/// 0.07.4), by its tiles' hovers.
fn module_line(game: &Game, col: &Colony, cid: ColonyId, mi: usize, director: Option<Seat>) -> String {
    let m = &col.modules[mi];
    let figures = if m.kind == ModuleKind::Archive {
        let research = game.tables.archive.research;
        let fund = director.map(|d| game.seat(d).archive_fund).unwrap_or(0);
        if fund >= research {
            format!("complete, {}", if m.online && !col.control.is_occupied() { "online" } else { "offline" })
        } else {
            format!("standing, {fund} of {research} Research paid")
        }
    } else if m.mothballed {
        "mothballed: making nothing and paying no upkeep".to_string()
    } else if m.kind == ModuleKind::Battery {
        // Ticket #324 (version 0.08.8): a Battery's figures are a unit's, not a yield.
        let card = game.tables.module(ModuleKind::Battery);
        format!("strength {}, {} of {} hit points, {} Energy upkeep", card.strength, card.hit_points.saturating_sub(m.damage), card.hit_points, card.energy_upkeep)
    } else {
        match director {
            Some(d) => game.module_yield_at(d, cid, mi).text(),
            None => "idle".to_string(),
        }
    };
    format!("{}: {}", m.kind.name(), figures)
}

/// Ticket #324 (version 0.08.8): what a Repair order's line calls the thing it repairs.
fn unit_name(game: &Game, unit: &UnitRef) -> String {
    match unit {
        UnitRef::Ship(s) => s.to_string(),
        UnitRef::Army(a) => a.to_string(),
        UnitRef::Battery { colony, .. } => format!("the Battery at {}", game.place_name(Place::Colony(*colony))),
    }
}

/// Ticket #324 (version 0.08.8): the Battery's rules, under its tile's figures on the hover.
fn battery_rules(game: &Game) -> String {
    let card = game.tables.module(ModuleKind::Battery);
    format!("\nA Battery stands in the line of any Battle fought in this orbit, on Hold, and never disengages. While it stands and works, no rival holds Orbital Control here: none may land, and no Blockade shuts its owner's station; its owner gains no Control by it. Repaired with Materials here, as a Ship is; at {} hits it is destroyed.", card.hit_points)
}

/// The words a box's hover adds to an offline building's line, and nothing for a working or a
/// mothballed one. Ticket #306 (version 0.08.7): the hover's alone; the row under the grid does
/// not carry them, since the row and the hover are about the same building. Ticket #307: they
/// name the cause. A Facility goes offline two ways: struck by a card until the next Resolution,
/// or shut at Income for want of Energy.
fn facility_offline_words(f: &Facility) -> &'static str {
    if f.online || f.mothballed {
        ""
    } else if f.offline_until_resolution {
        " (offline until the next Resolution, struck by a card; making nothing)"
    } else {
        " (offline, short of Energy; making nothing)"
    }
}

/// A Module's offline words, the counterpart of `facility_offline_words`: a card, the Colony's
/// grid down, an Occupied Colony's Archive, or want of Energy.
fn module_offline_words(col: &Colony, m: &Module) -> &'static str {
    if m.online || m.mothballed {
        ""
    } else if m.offline_until_resolution {
        " (offline until the next Resolution, struck by a card; making nothing)"
    } else if col.grid_failed {
        " (offline, the grid is down; making nothing)"
    } else if m.kind == ModuleKind::Archive && col.control.is_occupied() {
        " (offline while the Colony is Occupied)"
    } else {
        " (offline, short of Energy; making nothing)"
    }
}

/// Ticket #150 (version 0.07.4): the Module rules under a tile, the counterpart of `facility_rules`.
fn module_rules(heading: &str) -> String {
    format!("{heading}\nEnergy upkeep is paid at Income first; short of Energy, Modules go offline in order until the bill is met, and an offline one makes nothing and keeps its place.\nMothballed, it makes nothing and pays nothing until it is restarted.")
}

/// Ticket #162 (version 0.07.5): a Colony's Module tiles are laid out in the SAME grid as a
/// Region's build slots, at the designer's word -- *"the grid should match the one used by
/// nations"* -- so the two cards read as one thing and neither can drift from the other.
const MODULE_COLS: usize = SLOT_COLS;

/// Ticket #162 (version 0.07.5): **a Colony's or a station's Modules as boxes on its card**, in the
/// language the Region card's build slots already speak. The designer: *"Move the station module
/// grid to its in the card like nation states and retire the window."* One tile per Module standing
/// (dimmed while mothballed), one hatched tile per Module building, one dashed tile per free place
/// under the cap, and the Archive on a row of its own outside the count. A click puts that Module's
/// figures and its Mothball, Restart and Decommission buttons in the strip beneath; a click on a
/// free tile puts the build buttons there, which is the only place a Module is ordered -- as a
/// Facility that takes a slot is ordered only from a free box on a Region's card (#154).
#[allow(clippy::too_many_arguments)]
fn module_boxes(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, cid: ColonyId, mine: bool, director: Option<Seat>, actions: &mut Vec<Action>) {
    let Some(col) = game.colony(cid) else { return };
    let (used, cap) = (game.module_slots_used(col), game.module_slots(col));
    // The tiles, in the order the cap counts them: standing (the Archive apart), building, free.
    let standing: Vec<usize> = (0..col.modules.len()).filter(|i| col.modules[*i].kind != ModuleKind::Archive).collect();
    // Ticket #332 (version 0.09.0): a building tile is a queue index; its face reads the count
    // off the build and its hover the estimate at this place's Widgets behind everything ahead.
    let building: Vec<usize> = col.queue.iter().enumerate().filter(|(_, b)| matches!(b.item, BuildItem::Module(k) if k != ModuleKind::Archive)).map(|(qi, _)| qi).collect();
    let estimates = game.queue_estimates(Place::Colony(cid));
    // Ticket #291 (version 0.08.6): the Modules ORDERED this turn and not yet committed, as the
    // Faction's own kind (#186), each with its index in the pending list for the right-click that
    // cancels it. They take their places from the free count, as the rule already did at the
    // order (`orders.rs`, the Module cap), so the grid and the refusal agree.
    let ordered: Vec<(ModuleKind, usize)> = if mine {
        session.pending.iter().enumerate().filter_map(|(i, o)| o.build_module().filter(|(c, _)| *c == cid).map(|(_, k)| (k.built_by(game.kind(Seat(0))), i))).collect()
    } else {
        Vec::new()
    };
    let free = (cap.saturating_sub(used) as usize).saturating_sub(ordered.len());
    let total = standing.len() + building.len() + ordered.len() + free;
    let rows = total.div_ceil(MODULE_COLS).max(1);
    let archive = col.modules.iter().position(|m| m.kind == ModuleKind::Archive);
    let archive_rows = if archive.is_some() { 1 } else { 0 };
    let grid_size = egui::vec2(MODULE_COLS as f32 * HAB_TILE + (MODULE_COLS as f32 - 1.0) * HAB_GAP, (rows + archive_rows) as f32 * (HAB_TILE + HAB_LABEL + HAB_GAP));
    let (grid, _) = ui.allocate_exact_size(grid_size, egui::Sense::hover());
    let tile_rect = |i: usize| {
        let (c, r) = (i % MODULE_COLS, i / MODULE_COLS);
        egui::Rect::from_min_size(grid.min + egui::vec2(c as f32 * (HAB_TILE + HAB_GAP), r as f32 * (HAB_TILE + HAB_LABEL + HAB_GAP)), egui::vec2(HAB_TILE, HAB_TILE))
    };
    let mut i = 0usize;
    for mi in standing {
        let m = &col.modules[mi];
        let state = if m.mothballed { TileState::Mothballed } else if !m.online { TileState::Offline } else { TileState::Standing };
        let selected = view.hab_tile == Some(HabTile::Module(mi));
        // Ticket #150 (version 0.07.4): the tile's hover -- the figures its strip line carries and
        // the Module rules, which the old rows never had.
        let mut tip = module_rules(&format!("{}{}", module_line(game, col, cid, mi, director), module_offline_words(col, m)));
        // Ticket #324 (version 0.08.8): a Battery's hover carries its rules; a damaged one wears its
        // hit points on its label, as a shield wears an Army's.
        let mut label = m.kind.name().to_string();
        if m.kind == ModuleKind::Battery {
            tip.push_str(&battery_rules(game));
            if m.damage > 0 {
                let hp = game.tables.module(ModuleKind::Battery).hit_points;
                label = format!("Battery {}/{}", hp.saturating_sub(m.damage), hp);
            }
        }
        if hab_tile(ui, tile_rect(i), ui.id().with(("hab", mi)), Some(crate::icons::module_icon(m.kind)), &label, state, selected, None, tip).clicked() {
            view.hab_tile = Some(HabTile::Module(mi));
        }
        i += 1;
    }
    for (bi, qi) in building.iter().enumerate() {
        let b = &col.queue[*qi];
        let BuildItem::Module(kind) = b.item else { continue };
        let turns = estimates.get(*qi).copied().unwrap_or(u32::MAX);
        let (tip, cancel) = building_tip(game, session, Place::Colony(cid), *qi, b, turns, mine, kind.name(), "");
        let resp = hab_tile(ui, tile_rect(i), ui.id().with(("hab-building", bi)), Some(crate::icons::module_icon(kind)), kind.name(), TileState::Building { ordered: false, done: b.done, widgets: b.widgets }, false, None, tip);
        // Ticket #332: a rival's build, left behind when this place changed hands, is cancelled
        // by a right-click, as this turn's own order is on the tile below.
        if let Some(order) = cancel
            && resp.secondary_clicked()
        {
            actions.push(Action::Place(order));
        }
        i += 1;
    }
    for (oi, (kind, pi)) in ordered.iter().enumerate() {
        // Ticket #291: ordered this turn; right-click takes the order back. Ticket #332: the face
        // reads `0 of 8`, nothing being done on it until the turn ends; the hover the estimate.
        let widgets = game.build_widgets(Seat(0), BuildItem::Module(*kind));
        let turns = game.turns_to_build(Seat(0), Place::Colony(cid), BuildItem::Module(*kind));
        let tip = format!("{}: ordered this turn, {widgets} Widgets, {} once the turn ends.\nRight-click to cancel the order.", kind.name(), estimate_words(turns));
        if hab_tile(ui, tile_rect(i), ui.id().with(("hab-ordered", oi)), Some(crate::icons::module_icon(*kind)), kind.name(), TileState::Building { ordered: true, done: 0, widgets }, false, None, tip).secondary_clicked() {
            actions.push(Action::Cancel(*pi));
        }
        i += 1;
    }
    for fi in 0..free {
        // One free place is as good as another, so the first stands for the click.
        let selected = fi == 0 && view.hab_tile == Some(HabTile::Free);
        let tip = format!("Room for another Module: click it to build here.\n{} places are free from the start and one more for every {} Colonist.", game.tables.slots.base, game.tables.slots.per_colonist);
        if hab_tile(ui, tile_rect(i), ui.id().with(("hab-free", fi)), None, "", TileState::Free(mine), selected, None, tip).clicked() {
            view.hab_tile = Some(HabTile::Free);
        }
        i += 1;
    }
    if let Some(ai) = archive {
        // The Archive stands apart: a row of its own, outside the count.
        let rect = egui::Rect::from_min_size(grid.min + egui::vec2(0.0, rows as f32 * (HAB_TILE + HAB_LABEL + HAB_GAP)), egui::vec2(HAB_TILE, HAB_TILE));
        let state = if col.modules[ai].mothballed { TileState::Mothballed } else if !col.modules[ai].online { TileState::Offline } else { TileState::Standing };
        let tip = format!("{}{}\nOutside the Module count. Its Research is paid into the Archive fund at any pace; complete, it takes a great deal of Energy to keep running. Destroyed outright if this Colony changes hands; the fund is kept.", module_line(game, col, cid, ai, director), module_offline_words(col, &col.modules[ai]));
        if hab_tile(ui, rect, ui.id().with("hab-archive"), Some(crate::icons::module_icon(ModuleKind::Archive)), "The Archive", state, view.hab_tile == Some(HabTile::Module(ai)), None, tip).clicked() {
            view.hab_tile = Some(HabTile::Module(ai));
        }
    }
    ui.add_space(4.0);
    // The strip: the clicked tile's figures and controls, or the build buttons for a free one.
    match view.hab_tile {
        Some(HabTile::Module(mi)) if mi < col.modules.len() => {
            let m = &col.modules[mi];
            let colour = if m.mothballed { Color32::from_rgb(170, 170, 190) } else { ui.visuals().text_color() };
            figures_with_icons(ui, &module_line(game, col, cid, mi, director), 14.0, colour, &[]);
            if mine && m.kind != ModuleKind::Archive {
                change_row(ui, game, &session.pending, BuildingRef::Module(cid, mi), m.mothballed, m.change, actions);
            }
            // Ticket #324 (version 0.08.8): a damaged Battery repairs here, as a Ship does at a yard.
            if mine && m.kind == ModuleKind::Battery && m.damage > 0 {
                ui.horizontal(|ui| {
                    cost_button(ui, game, &session.pending, Order::Repair { unit: UnitRef::Battery { colony: cid, index: mi }, points: m.damage }, "Repair fully", actions);
                    cost_button(ui, game, &session.pending, Order::RepairWithDucats { unit: UnitRef::Battery { colony: cid, index: mi }, points: m.damage }, "Repair fully with Ducats", actions);
                });
            }
        }
        Some(HabTile::Free) if mine => {
            ui.label(RichText::new("Build here").strong());
            module_build_buttons(ui, session, game, cid, actions);
        }
        Some(HabTile::Free) => {
            ui.label(RichText::new("Room for another Module.").weak());
        }
        _ => {
            ui.label(RichText::new("Click a tile for its figures and controls.").weak());
        }
    }
}

/// Ticket #145 (version 0.07.3): the Module build buttons a Colony's card offers, drawn on the card
/// and in the Hab View's strip for a free tile alike, so the two never differ.
fn module_build_buttons(ui: &mut Ui, session: &Session, game: &Game, cid: ColonyId, actions: &mut Vec<Action>) {
    let Some(col) = game.colony(cid) else { return };
    // Ticket #186 (version 0.08.0): as on Earth -- the Custodians see their Academy where everyone
    // else sees the Institute.
    let me = game.kind(Seat(0));
    for mk in ModuleKind::BUILDABLE {
        if mk.built_by(me) != mk || mk.unique_to().is_some_and(|f| f != me) {
            continue;
        }
        // Ticket #80: a station holds a Shipyard, Habitats and Observatories; ticket #89: and
        // Solar Arrays, which stand nowhere else.
        // Ticket #185 (version 0.08.0): and an Institute, since a station carries Observatories
        // and the Institute is what multiplies them.
        //
        // Ticket #239 (version 0.08.3): the list names the JOB, never the kind. Written with the
        // kinds it had to name the Academy too, and the next three Unique Modules broke it
        // silently -- the Prospectors lost the Trade Post row from every station without gaining
        // the Exchange, and the Archivists lost the Solar Array without gaining the Heliostat,
        // because `built_by` above swaps the common kind out and this list then threw the Unique
        // away. A picture of the build list found it; no test did.
        if col.in_orbit && !mk.stands_on_a_station() {
            continue;
        }
        if !col.in_orbit && game.tables.module(mk).station_only {
            continue;
        }
        let hover = game.module_yield(Seat(0), cid, mk).text();
        ui.horizontal(|ui| {
            cost_button_with_hover(ui, game, &session.pending, Order::BuildModule { colony: cid, kind: mk }, mk.name(), Some(hover), actions);
            cost_button(ui, game, &session.pending, Order::BuildModuleWithDucats { colony: cid, kind: mk }, "or", actions);
        });
    }
}


/// Ticket #203 (version 0.08.1): a Faction's name drawn as a LINK -- clicking it opens that
/// Faction's page in the Faction window.
///
/// It is used in exactly two places, and the ticket names them: the Victory window's four per-seat
/// blocks, and the Relations rows in the Faction window itself. Those are the two places where a
/// player is already comparing Factions and the next thought is "tell me more about that one". A
/// Region card's `Held by the Archivists` and the roster are deliberately NOT links: there a
/// Faction's name is describing a place, and a click must go on selecting the place.
fn faction_link(ui: &mut Ui, view: &mut ViewState, seat: Seat, text: RichText) {
    let resp = ui.add(egui::Label::new(text).sense(egui::Sense::click()));
    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    if resp.on_hover_text("Open this Faction's page.").clicked() {
        view.faction_seat = seat;
        view.show_factions = true;
    }
}

/// The Faction's name as the dropdown and its closed box say it: seat 0's carries `(you)` in a game
/// the player sits at, and nothing in a spectated one, where no seat is theirs.
fn faction_page_name(session: &Session, game: &Game, seat: Seat) -> String {
    if seat == Seat(0) && !session.spectator { format!("{} (you)", game.seat_name(seat)) } else { game.seat_name(seat) }
}

/// Ticket #203: one of the Faction window's two Relations rows. `outward` is what this seat thinks
/// of the other three; otherwise it is what each of the other three thinks of this seat. Each name
/// is a link to that Faction's page.
fn relations_row(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, seat: Seat, outward: bool) {
    ui.horizontal_wrapped(|ui| {
        for other in Seat::ALL {
            if other == seat {
                continue;
            }
            let (viewer, subject) = if outward { (seat, other) } else { (other, seat) };
            let v = game.relations_score(viewer, subject);
            faction_link(ui, view, other, RichText::new(game.seat_name(other)).color(seat_colour(session, other)));
            // Ticket #221 (version 0.08.2): the LEVEL is what a player reasons with and the number is
            // the audit trail, so the level is on the row and the figure rides on its hover -- with
            // the Blame part broken out, because a standing penalty nobody can see the cause of is
            // the one thing that would make this grid unreadable.
            let colour = if v < 0 { Color32::from_rgb(230, 120, 100) } else { Color32::from_gray(190) };
            let blame = game.blame_relations_term(viewer, subject);
            let deeds = game.relations_deeds(viewer, subject);
            let floor = game.relations.floor[viewer.index()][subject.index()];
            let mut tip = format!("{:+} in all: {:+} from what they have done, {:+} from their Blame.", v, deeds, blame);
            if floor < 0 {
                tip.push_str(&format!("
Scarred: this pair can never recover above {floor:+}."));
            }
            // Ticket #233 (version 0.08.3): and what STANDS between the pair -- never what they
            // could strike. The designer chose the narrower line: the hover answers the glance,
            // and the Accords block a few rows below is where a player acts, already listing every
            // term with its own explanation and greying out the research agreement when the pair
            // is not Friendly. Saying it twice, a scroll apart, is how a figure drifts.
            //
            // The SAME rule covers a pair the player is not in, which this grid shows on every
            // Faction's page: what stands between two rivals is visible on the board once it bites,
            // where what they COULD strike is intelligence. It is the line the Faction window's
            // disclosure rule already draws.
            if let Some(acc) = game.accords.iter().find(|a| (a.a == viewer && a.b == subject) || (a.a == subject && a.b == viewer)) {
                let terms: Vec<&str> = acc
                    .terms
                    .iter()
                    .map(|t| match t {
                        Term::NonAggression => "non-aggression",
                        Term::Passage => "passage",
                        Term::Refuel => "refuel",
                        Term::ResearchAgreement => "a research agreement",
                    })
                    .collect();
                if acc.ending {
                    tip.push_str(&format!("
An Accord between them is over: it lapses at the next turn ({}).", terms.join(", ")));
                } else {
                    tip.push_str(&format!("
An Accord stands: {}.", terms.join(", ")));
                }
            }
            ui.label(RichText::new(game.relations_level(viewer, subject)).color(colour)).on_hover_text(tip);
            ui.add_space(10.0);
        }
    });
}

/// Ticket #235 (version 0.08.3): the **Research Directive** -- the share of a Faction's Research
/// that goes somewhere other than the shared Tech, chosen as a percentage and standing until it is
/// changed.
///
/// A PERCENTAGE rather than a count of points, because Research grows all game: a setting made on
/// turn 5 in points is meaningless by turn 25, where a share keeps its meaning and reads directly
/// against the shared-pot rule -- the player sees what they are contributing, not just what they
/// are taking.
///
/// The Archivists' cap is 100 and everyone else's 50. Theirs was a switch until this version and
/// that switch always sent ALL of it, so the slider keeps the reach.
fn research_directive_control(ui: &mut Ui, session: &Session, game: &Game, actions: &mut Vec<Action>) {
    let me = Seat(0);
    let cap = game.research_directive_cap(me);
    // Ticket #235 gave this control the DIRECTIVE -- the share diverted. Ticket #251 turns it the
    // other way up, at the designer's word: the slider carries the **contribution to the shared
    // Tech**, which is what "will not extend below 50%" and "the selected contribution in the
    // header" both describe, and what the shared-pot rule of ticket #236 is written in. A player
    // reading "78%" can compare it to that rule's 85 without doing the subtraction in their head.
    let floor = 100 - cap;
    let standing = 100 - game.seat(me).research_directive;
    let pending_set = session.pending.iter().find_map(|o| match o {
        Order::SetResearchDirective { percent } => Some(100 - *percent),
        _ => None,
    });
    let mut contribution = pending_set.unwrap_or(standing);

    ui.label(RichText::new(format!("Research Directive: {contribution}%")).strong()).on_hover_text(
        "The share of your Research that goes to the shared Tech, from the next Income until you set it again. What you keep back never reaches the Tech, so it counts nothing toward the Research Lead -- and the Lead is the only seat that picks what the table researches next.",
    );

    // The scale is 0 to 100 for EVERY Faction, so the four controls read alike and the Archivists'
    // extra reach is visible rather than implied: their slider runs the whole way, and everyone
    // else's is stopped at half with the unreachable part dimmed behind it.
    // A fifth larger than the default, at the designer's word. Both figures matter: `slider_width`
    // is the rail's length and `slider_rail_height` its thickness, and raising only the first
    // makes a long thin bar rather than a bigger control.
    let full = ui.available_width();
    let (was_width, was_rail, was_interact) = (ui.spacing().slider_width, ui.spacing().slider_rail_height, ui.spacing().interact_size);
    ui.spacing_mut().slider_width = full;
    ui.spacing_mut().slider_rail_height = was_rail * 1.2;
    ui.spacing_mut().interact_size.y = was_interact.y * 1.2;
    let resp = ui.add(egui::Slider::new(&mut contribution, 0..=100).show_value(false));
    ui.spacing_mut().slider_width = was_width;
    ui.spacing_mut().slider_rail_height = was_rail;
    ui.spacing_mut().interact_size = was_interact;
    if floor > 0 {
        // The share no Faction but the Archivists may reach, painted OVER the rail so the scale
        // still reads 0 to 100 for everyone and their extra reach is visible rather than implied.
        // The colour took three goes and only a picture could settle it. A translucent black over
        // an already dark rail was invisible, and so was a grey close to the rail's own; a
        // diagnostic pass in bright red proved the geometry had been right all along, so the fault
        // was never the rectangle. A dark grey read as a hole in the control rather than a bound,
        // so at the designer's word it is now the grey this game already uses for NOBODY'S --
        // `from_gray(110)`, the unattributed segment of the race bar a few lines above -- leaned a
        // little red to say "not yours to take" rather than "nothing here".
        let r = resp.rect;
        let dim = egui::Rect::from_min_max(
            egui::pos2(r.min.x, r.center().y - ui.spacing().slider_rail_height * 0.6),
            egui::pos2(r.min.x + r.width() * floor as f32 / 100.0, r.center().y + ui.spacing().slider_rail_height * 0.6),
        );
        ui.painter().rect_filled(dim, 2.0, Color32::from_rgb(124, 104, 104));
        ui.painter().line_segment(
            [egui::pos2(dim.max.x, r.center().y - 9.0), egui::pos2(dim.max.x, r.center().y + 9.0)],
            egui::Stroke::new(1.5, Color32::from_gray(120)),
        );
    }
    contribution = contribution.max(floor);

    let (what, rate) = match game.kind(me) {
        FactionKind::Custodians => ("the Natural Sink", format!("{} ppm for good, per point", game.tables.research_directive.custodians_ppm_per_point)),
        FactionKind::Prospectors => ("your coffers", format!("{} Ducats per point", game.tables.research_directive.prospectors_ducats_per_point)),
        FactionKind::Arkwrights => ("propellant", format!("{} Fuel per point", game.tables.research_directive.arkwrights_fuel_per_point)),
        FactionKind::Archivists => ("the Archive fund", "one for one, to the fund's cap".to_string()),
    };
    let made = game.seat(me).research_last_turn;
    let directive = 100 - contribution;
    let taken = made * directive as i64 / 100;
    ui.label(
        RichText::new(if directive == 0 {
            format!("All of it to the shared Tech. {made} Research last turn.")
        } else {
            format!("{contribution}% to the shared Tech; {directive}% to {what} ({rate}). On last turn's {made} Research that is {taken} directed.")
        })
        .weak(),
    );
    // Ticket #236 (version 0.08.3): what the table makes of this setting, said on the control that
    // sets it. The rule is a term read afresh every settle, so this line is always the truth about
    // right now rather than a forecast -- and it is the reason the slider carries the CONTRIBUTION
    // rather than the share taken: the player compares one number to one line.
    let pot = &game.tables.relations;
    let (pot_text, pot_colour) = if contribution >= 100 {
        ("Every rival thinks a little better of you for giving all of it.".to_string(), Color32::from_rgb(140, 200, 140))
    } else if contribution < pot.directive_min_contribution {
        (format!("Below {}%, every rival thinks a little worse of you while it lasts.", pot.directive_min_contribution), Color32::from_rgb(230, 150, 130))
    } else {
        (format!("Above the {}% line: nobody minds, and nobody is grateful.", pot.directive_min_contribution), ui.visuals().weak_text_color())
    };
    ui.label(RichText::new(pot_text).color(pot_colour));

    if contribution != pending_set.unwrap_or(standing) {
        if let Some(i) = session.pending.iter().position(|o| matches!(o, Order::SetResearchDirective { .. })) {
            actions.push(Action::Cancel(i));
        }
        if contribution != standing {
            let order = Order::SetResearchDirective { percent: directive };
            if game.check_order(me, &session.pending, &order).is_ok() {
                actions.push(Action::Place(order));
            }
        }
    }
}

/// Ticket #256 (version 0.08.4): **the Venture Capital Fund's controls** on the Victory window, the
/// Prospectors only -- the share as a slider, and a withdrawal.
///
/// The share was a row of nine labels, 0% to 80% in tenths (ticket #72). The designer asked for a
/// slider *"to allow finer control"*, mirroring the Research Directive's: the same full-width rail
/// on a 0-to-100 scale in whole percents, with the part of the scale the rule does not allow --
/// the top fifth, since `max_share` is 0.8 -- painted over in the grey the game uses for nobody's,
/// so the bound is visible rather than implied. `share_step` in `factions.toml` is a hundredth now
/// and the order takes whole percents; the AI's smallest-share-that-reaches-the-bar loop walks the
/// same step.
///
/// The draw was one button that took ten, every time. It is **Withdraw** now, with a field for the
/// amount as the Influence cluster has one, at the designer's word. The tenth lost on the way out is
/// unchanged and the hover says what comes back.
fn venture_fund_control(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, actions: &mut Vec<Action>) {
    let me = Seat(0);
    let v = game.tables.venture.clone();
    let cap = (v.max_share * 100.0).round() as u32;
    let standing = (game.seat(me).venture_share * 100.0).round() as u32;
    let pending_set = session.pending.iter().find_map(|o| if let Order::SetVentureShare { share } = o { Some(*share) } else { None });
    let mut share = pending_set.unwrap_or(standing);

    ui.label(RichText::new(format!("Venture Capital Fund: banking {share}% of Ducat income")).strong()).on_hover_text(
        "The share of each turn's Ducat income that goes into the Fund at Income, before you can spend a coin of it, from the next Income until you set it again. Ducats got by selling are not income and never reach it.",
    );
    // The same rail the Research Directive draws, a fifth larger than egui's default in both
    // dimensions; see `research_directive_control` for why both figures matter.
    let full = ui.available_width();
    let (was_width, was_rail, was_interact) = (ui.spacing().slider_width, ui.spacing().slider_rail_height, ui.spacing().interact_size);
    ui.spacing_mut().slider_width = full;
    ui.spacing_mut().slider_rail_height = was_rail * 1.2;
    ui.spacing_mut().interact_size.y = was_interact.y * 1.2;
    let resp = ui.add(egui::Slider::new(&mut share, 0..=100).show_value(false));
    ui.spacing_mut().slider_width = was_width;
    ui.spacing_mut().slider_rail_height = was_rail;
    ui.spacing_mut().interact_size = was_interact;
    if cap < 100 {
        // The fifth no share may reach, painted over the rail's top end in the Directive's grey.
        let r = resp.rect;
        let dim = egui::Rect::from_min_max(
            egui::pos2(r.min.x + r.width() * cap as f32 / 100.0, r.center().y - ui.spacing().slider_rail_height * 0.6),
            egui::pos2(r.max.x, r.center().y + ui.spacing().slider_rail_height * 0.6),
        );
        ui.painter().rect_filled(dim, 2.0, Color32::from_rgb(124, 104, 104));
        ui.painter().line_segment(
            [egui::pos2(dim.min.x, r.center().y - 9.0), egui::pos2(dim.min.x, r.center().y + 9.0)],
            egui::Stroke::new(1.5, Color32::from_gray(120)),
        );
    }
    share = share.min(cap);
    let fund = game.seat(me).venture_fund;
    let banked = game.seat(me).venture_banked_last_turn;
    ui.label(
        RichText::new(format!(
            "The Fund holds {fund} Ducats; {banked} went in last turn.{}",
            pending_set.filter(|p| *p != standing).map(|p| format!(" {p}% from the next Income.")).unwrap_or_default()
        ))
        .weak(),
    );
    if share != pending_set.unwrap_or(standing) {
        if let Some(i) = session.pending.iter().position(|o| matches!(o, Order::SetVentureShare { .. })) {
            actions.push(Action::Cancel(i));
        }
        if share != standing {
            let order = Order::SetVentureShare { share };
            if game.check_order(me, &session.pending, &order).is_ok() {
                actions.push(Action::Place(order));
            }
        }
    }

    // Withdraw: a field and a button, the Influence cluster's shape.
    ui.horizontal(|ui| {
        let most = fund - session.pending.iter().map(|o| if let Order::DrawVenture { amount } = o { *amount } else { 0 }).sum::<i64>();
        ui.add(egui::DragValue::new(&mut view.venture_withdraw).range(1..=most.max(1)));
        let order = Order::DrawVenture { amount: view.venture_withdraw };
        let check = game.check_order(me, &session.pending, &order);
        let back = (view.venture_withdraw as f64 * v.draw_return).floor() as i64;
        let resp = ui.add_enabled(check.is_ok(), egui::Button::new("Withdraw from the Fund"));
        if let Err(e) = &check {
            resp.clone().on_disabled_hover_text(&e.0);
        }
        if resp.on_hover_text(format!("{back} Ducats come back to the Stockpile at End Turn; a tenth is lost on the way out.")).clicked() {
            actions.push(Action::Place(order));
        }
    });
}

/// **The Faction window** (ticket #203, version 0.08.1), opened by `Factions (F)` on the top bar and
/// by the F key. One Faction a page, chosen by the dropdown in its top right, which opens on the
/// player's own seat.
///
/// It holds two halves that existed nowhere together before. The **live figures** -- Victory
/// progress, income, Blame, Relations, holdings -- and, behind a header shut by default, the
/// **rulebook**: the setup screen's Faction card, which a player could not reach again once a game
/// began. The header is shut because the mockups measured the flat page at about 950 pixels of
/// content at 524 wide, the signature rules being the long part, which very nearly fills a
/// 1080-line screen and would push the symbol off the top of it.
///
/// **The disclosure rule.** Your own seat shows its income totals AND keeps the building-by-building
/// breakdown on the hover, as the top bar does. Any other seat shows **totals only**: the breakdown
/// names individual buildings in individual Regions, which is a targeting list, where the total is
/// only the rate of a hoard the Victory window already prints to the unit. A **spectator** gets the
/// breakdown on every seat and no disclosure line at all, having no side to keep secrets from.
/// Ticket #226 (version 0.08.2): the Accords, on a rival's page of the Faction window.
///
/// What stands between the player's seat and this one, what it would take to strike one, and the two
/// acts that raise Relations. The terms are ticked and offered together, because an Accord is one
/// bargain rather than four; the computer seat answers at the Resolution by its own weights, and a
/// refusal is not an offence.
/// Ticket #293 (version 0.08.6): **the rail a one-shot Influence spend is set on** -- the Smear's
/// and the Greenwash's, drawn as the Research Directive's rail is (`research_directive_control`):
/// full width, single points, the part of the scale the player cannot reach painted over in
/// nobody's grey so the scale holds still through the turn. The designer: *"runs from 0 to their
/// max income - width fixed."* `whole` is the rail's end, this turn's Allotment plus any
/// Influence bought; `left` is what is not yet committed to other orders, greyed from the right;
/// `second` is a tighter bound of another kind (the Greenwash's Ducats), greyed in a bluer shade
/// from where it bites to where the Influence would have. Returns the amount, clamped to the
/// tightest bound. The button that spends it stays with the caller: a spend is pressed.
fn influence_rail(ui: &mut Ui, value: &mut i64, whole: i64, left: i64, second: Option<i64>) -> i64 {
    let end = whole.max(1);
    let mut v = (*value).clamp(0, end);
    let full = ui.available_width();
    let (was_width, was_rail, was_interact) = (ui.spacing().slider_width, ui.spacing().slider_rail_height, ui.spacing().interact_size);
    ui.spacing_mut().slider_width = full;
    ui.spacing_mut().slider_rail_height = was_rail * 1.2;
    ui.spacing_mut().interact_size.y = was_interact.y * 1.2;
    let resp = ui.add(egui::Slider::new(&mut v, 0..=end).show_value(false));
    ui.spacing_mut().slider_width = was_width;
    ui.spacing_mut().slider_rail_height = was_rail;
    ui.spacing_mut().interact_size = was_interact;
    let r = resp.rect;
    let x_of = |n: i64| r.min.x + r.width() * (n.clamp(0, end) as f32 / end as f32);
    let (top, bottom) = (r.center().y - ui.spacing().slider_rail_height * 0.6, r.center().y + ui.spacing().slider_rail_height * 0.6);
    let tick = |ui: &Ui, x: f32| {
        ui.painter().line_segment([egui::pos2(x, r.center().y - 9.0), egui::pos2(x, r.center().y + 9.0)], egui::Stroke::new(1.5, Color32::from_gray(120)));
    };
    let left = left.clamp(0, end);
    if left < end {
        // Committed to other orders already: nobody's grey, from the right, as the Directive's floor
        // is from the left.
        ui.painter().rect_filled(egui::Rect::from_min_max(egui::pos2(x_of(left), top), egui::pos2(r.max.x, bottom)), 2.0, Color32::from_rgb(124, 104, 104));
        tick(ui, x_of(left));
    }
    let mut bound = left;
    if let Some(s) = second
        && s < left
    {
        let s = s.max(0);
        ui.painter().rect_filled(egui::Rect::from_min_max(egui::pos2(x_of(s), top), egui::pos2(x_of(left), bottom)), 2.0, Color32::from_rgb(104, 104, 124));
        tick(ui, x_of(s));
        bound = s;
    }
    v = v.min(bound);
    *value = v;
    v
}

/// Ticket #267 (version 0.08.4): **the Smear campaign**, on a rival's page beside the Accords:
/// a field for the Influence and a button, the Influence cluster's shape. The hover names the
/// rate, the ledger it lands on, and the offence.
/// Ticket #293 (version 0.08.6): the field is a rail (`influence_rail`), and what is left reads
/// the command cluster's own figure, so Influence bought this turn counts -- the field ignored it.
fn smear_block(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, other: Seat, actions: &mut Vec<Action>) {
    let me = Seat(0);
    let rate = game.tables.influence.smear.ppm_per_influence;
    ui.label(RichText::new("Smear campaign").strong()).on_hover_text(format!(
        "Influence spent on this Faction rather than a place: every point lays {rate} ppm on their Blame for good, and the share every rule reads moves with it.\nOne campaign a turn against each rival, from this turn's Allotment. They will know who paid: it is an offence."
    ));
    let laid = game.seat(other).blame_smeared;
    if laid > 0.0 {
        ui.label(RichText::new(format!("{laid:.0} ppm of their Blame was laid on them by rivals.")).weak());
    }
    let (whole, left) = influence_this_turn(game, session);
    let amount = influence_rail(ui, &mut view.smear_amount, whole, left, None);
    ui.label(RichText::new(format!("{amount} of your {whole} Influence this turn; {left} not yet ordered elsewhere.")).weak());
    let order = Order::Smear { target: other, amount };
    let check = game.check_order(me, &session.pending, &order);
    let resp = ui.add_enabled(check.is_ok(), egui::Button::new(format!("Smear the {} with {amount} Influence", game.seat_name(other))));
    if let Err(e) = &check {
        resp.clone().on_disabled_hover_text(&e.0);
    }
    if resp.on_hover_text(format!("{:.0} ppm on their Blame at End Turn.", amount as f64 * rate)).clicked() {
        actions.push(Action::Place(order));
    }
}

/// Ticket #293 (version 0.08.6): this turn's whole Influence (the Allotment plus what was bought
/// in the Trading window) and what is left of it once the pending orders have taken theirs -- the
/// command cluster's figure, read through `remaining`, so the two controls agree with it.
fn influence_this_turn(game: &Game, session: &Session) -> (i64, i64) {
    let me = Seat(0);
    let bought: i64 = session.pending.iter().map(|o| if let Order::BuyInfluence { amount } = o { *amount } else { 0 }).sum();
    let (_, left) = game.remaining(me, &session.pending);
    (game.seat(me).allotment + bought, left)
}

/// Ticket #277 (version 0.08.5): the Greenwash block on the player's own page: a heading with the
/// rule on hover, a field for the Influence and a button that names both prices, the Smear block's
/// shape. Public and no offence, at the designer's word, so the hover says a rival can answer it.
fn greenwash_block(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, actions: &mut Vec<Action>) {
    let me = Seat(0);
    let g = &game.tables.influence.greenwash;
    let (rate, per) = (g.ppm_per_influence, g.ducats_per_influence);
    ui.label(RichText::new("Greenwash campaign").strong()).on_hover_text(format!(
        "Influence spent on your own name, with {per} Ducat{} beside every point: every point takes {rate} ppm off your Blame for good, and the share every rule reads moves with it. Nothing leaves the air.\nOne campaign a turn, from this turn's Allotment and your Ducats. It is public and no offence: the Report says you greenwashed, and a rival can answer with a Smear.",
        if per == 1 { "" } else { "s" }
    ));
    let cleaned = game.seat(me).blame_cleaned;
    if cleaned > 0.0 {
        ui.label(RichText::new(format!("{cleaned:.0} ppm of your Blame has been greenwashed away.")).weak());
    }
    // Ticket #293 (version 0.08.6): the rail, with the Ducats as a second bound painted in a bluer
    // grey where they bite before the Influence does, and the line under it naming which bites.
    let (whole, left) = influence_this_turn(game, session);
    let (stock, _) = game.remaining(me, &session.pending);
    let by_ducats = if per > 0 { Some((stock.ducats / per).max(0)) } else { None };
    let amount = influence_rail(ui, &mut view.greenwash_amount, whole, left, by_ducats);
    let bites = match by_ducats {
        Some(d) if d < left => format!("your {} Ducats cover {d} of it, which is the bound", stock.ducats),
        _ => format!("{left} not yet ordered elsewhere, and the Ducats cover it"),
    };
    ui.label(RichText::new(format!("{amount} of your {whole} Influence this turn; {bites}.")).weak());
    let order = Order::Greenwash { amount };
    let check = game.check_order(me, &session.pending, &order);
    let resp = ui.add_enabled(check.is_ok(), egui::Button::new(format!("Greenwash with {amount} Influence for {} Ducats", amount * per)));
    if let Err(e) = &check {
        resp.clone().on_disabled_hover_text(&e.0);
    }
    if resp.on_hover_text(format!("{:.0} ppm off your Blame at End Turn.", amount as f64 * rate)).clicked() {
        actions.push(Action::Place(order));
    }
}

fn accords_block(ui: &mut Ui, session: &Session, game: &Game, other: Seat, actions: &mut Vec<Action>) {
    let me = Seat(0);
    let r = &game.tables.relations;
    ui.label(RichText::new("Accords").strong());

    let standing = game.accords.iter().find(|a| a.holds(me, other) || (a.ending && ((a.a == me && a.b == other) || (a.a == other && a.b == me))));
    if let Some(acc) = standing {
        let names: Vec<&str> = acc
            .terms
            .iter()
            .map(|t| match t {
                Term::NonAggression => "non-aggression",
                Term::Passage => "passage",
                Term::Refuel => "refuel",
                Term::ResearchAgreement => "a research agreement",
            })
            .collect();
        if acc.ending {
            ui.label(RichText::new(format!("Your Accord with the {} is over: it lapses at the next turn.", game.seat_name(other))).color(Color32::from_rgb(230, 190, 120)));
        } else {
            ui.label(format!("You hold an Accord with the {}: {}.", game.seat_name(other), names.join(", ")));
            let order = Order::EndAccord { with: other };
            let placed = session.pending.contains(&order);
            if placed {
                ui.label(RichText::new("You have declared it over this turn.").weak());
            } else if ui
                .button("Declare it over")
                .on_hover_text("Free, and it takes a turn's notice: it lapses at the start of the next turn and you may act then. Acting against a term while it still stands is another matter -- it costs 3 and ends the whole Accord at once.")
                .clicked()
            {
                actions.push(Action::Place(order));
            }
        }
    } else {
        // No Accord: the terms to offer. `view` is not threaded in here, so the ticks live in egui's
        // own memory under this pair's id -- they are a scratch choice, not game state.
        let id = egui::Id::new(("accord-terms", other.index()));
        let mut picked: Vec<Term> = ui.ctx().memory(|m| m.data.get_temp(id).unwrap_or_default());
        let friendly = game.relations_score(me, other) >= 7 && game.relations_score(other, me) >= 7;
        for (term, label, tip) in [
            (Term::NonAggression, "Non-aggression", "Neither spends Influence on a place the other holds, nor opens a Battle against them."),
            (Term::Passage, "Passage", "Either's Armies may march into the other's Regions without attacking, arriving on Hold; neither intercepts the other's Ships, and a Blockade does not shut them out of the slot."),
            (Term::Refuel, "Refuel", "Either may Refuel at the other's Space Stations."),
            (Term::ResearchAgreement, "Research agreement", "Both parties' Research rises a tenth while it stands. Wants Friendly on both sides to strike, and once struck it stands whatever the scores later do."),
        ] {
            let mut on = picked.contains(&term);
            let enabled = term != Term::ResearchAgreement || friendly;
            let resp = ui.add_enabled(enabled, egui::Checkbox::new(&mut on, label));
            let resp = if enabled { resp.on_hover_text(tip) } else { resp.on_disabled_hover_text("Both sides must be Friendly to strike a research agreement.") };
            if resp.changed() {
                if on {
                    picked.push(term);
                } else {
                    picked.retain(|t| *t != term);
                }
                ui.ctx().memory_mut(|m| m.data.insert_temp(id, picked.clone()));
            }
        }
        let order = Order::ProposeAccord { to: other, terms: picked.clone() };
        let offered = session.pending.iter().any(|o| matches!(o, Order::ProposeAccord { to, .. } if *to == other));
        if offered {
            ui.label(RichText::new("Your offer goes to them this turn.").weak());
        } else {
            let ok = game.check_order(me, &session.pending, &order);
            let resp = ui.add_enabled(ok.is_ok(), egui::Button::new(format!("Offer the {} an Accord", game.seat_name(other))));
            let resp = match &ok {
                Ok(_) => resp.on_hover_text("They answer this turn, by their own reckoning. A refusal costs you nothing: it is not an offence."),
                Err(e) => resp.on_disabled_hover_text(e.0.clone()),
            };
            if resp.clicked() {
                actions.push(Action::Place(order));
            }
        }
    }

    // Tribute: the other of the two acts that raise Relations, and the only one available to a pair
    // holding no Accord at all -- which is how a pair climbs out of Neutral in the first place.
    ui.horizontal(|ui| {
        for (materials, label) in [(false, format!("Pay {} Ducats", r.tribute_ducats)), (true, format!("Pay {} Materials", r.tribute_materials))] {
            let order = Order::Tribute { to: other, materials };
            let placed = session.pending.iter().any(|o| matches!(o, Order::Tribute { to, .. } if *to == other));
            let ok = game.check_order(me, &session.pending, &order);
            let resp = ui.add_enabled(ok.is_ok() && !placed, egui::Button::new(label));
            let resp = match &ok {
                Ok(_) if !placed => resp.on_hover_text("A tribute raises their view of you by one. A fixed gift, one a turn to a Faction: the gain is flat, so a larger one would buy no more."),
                Ok(_) => resp.on_disabled_hover_text("You have already paid them a tribute this turn."),
                Err(e) => resp.on_disabled_hover_text(e.0.clone()),
            };
            if resp.clicked() {
                actions.push(Action::Place(order));
            }
        }
    });
}

fn faction_window(ctx: &egui::Context, session: &Session, game: &Game, view: &mut ViewState, actions: &mut Vec<Action>) {
    if !view.show_factions {
        return;
    }
    let mut open = true;
    // The window opens BELOW the top bar's row of buttons. Left where egui first placed it, it came
    // up over `Tech Tree (T)` through `Trading (R)` -- the same complaint ticket #194 answered for
    // the Tech Tree, whose left third the Climate Panel was covering. The designer, seeing the first
    // capture: *"shift the faction window down so it doesn't block."* `default_pos` places it only
    // the first time, so a window the player has dragged stays where they put it.
    // Ticket #292 (version 0.08.6): under the measured bar, where 120 stood.
    egui::Window::new("Factions").open(&mut open).default_width(524.0).default_pos(egui::pos2(16.0, view.below_bar())).show(ctx, |ui| {
        // The dropdown, in the top right, at the designer's word. Under the hood it names SEATS --
        // every live figure below is a seat's -- but each seat holds one Faction, so its four rows
        // are the four Factions, each with its small glyph in its own colour.
        // A `with_layout` on its own takes the whole remaining height of the window and leaves the
        // page below it off the bottom; inside a `horizontal` it takes one row, which is the row
        // the dropdown wants.
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let chosen = view.faction_seat;
                egui::ComboBox::from_id_salt("faction_page")
                    .selected_text(RichText::new(faction_page_name(session, game, chosen)).color(seat_colour(session, chosen)))
                    .show_ui(ui, |ui| {
                        for seat in Seat::ALL {
                            let kind = game.kind(seat);
                            let label = RichText::new(faction_page_name(session, game, seat)).color(seat_colour(session, seat));
                            let clicked = match Icons::from_ctx(ui.ctx(), crate::icons::faction_symbol(kind), 18.0) {
                                Some(image) => ui.add(egui::Button::image_and_text(image.tint(rgb(game.tables.faction(kind).colour)), label).selected(seat == chosen)).clicked(),
                                None => ui.selectable_label(seat == chosen, label).clicked(),
                            };
                            if clicked {
                                view.faction_seat = seat;
                            }
                        }
                    });
                ui.label(RichText::new("Faction").weak());
            });
        });
        let seat = view.faction_seat;
        let kind = game.kind(seat);
        // The symbol at 64 pixels in the Faction's own colour, beside the name. See
        // `faction_heading` for why the CALLER picks the colour here and in only one other place.
        faction_heading(ui, session, kind, 64.0);
        ui.separator();

        // 1. Victory history. Ticket #264 (version 0.08.4): on every Faction's page, the player's
        // own included, at the population chart's size. Ticket #306 (version 0.08.7): the progress
        // lines and bars that stood above it are cut, at the designer's word, as a copy of the
        // Victory window's four blocks; the Victory window is the victory's home.
        ui.add_space(4.0);
        victory_history(ui, game, seat, egui::vec2(ui.available_width(), 90.0));
        ui.add_space(6.0);

        // 2. Income last turn, under the disclosure rule in this function's doc comment.
        let s = game.seat(seat);
        let inc = s.income_last_turn;
        let breakdown = session.spectator || seat == Seat(0);
        let signed = |v: i64| if v >= 0 { format!("+{v}") } else { format!("{v}") };
        let hover = |res: dying_earth_engine::Resource, word: &str| -> String {
            if !breakdown {
                return format!("{word}. A rival's income is shown as a total only.");
            }
            let lines: Vec<String> = s.income_sources.iter().filter(|(_, r, _)| *r == res).map(|(name, _, v)| format!("{v:+}  {name}")).collect();
            if lines.is_empty() {
                format!("{word}. No income from buildings last turn.")
            } else {
                format!("{word}. Last Income:\n{}", lines.join("\n"))
            }
        };
        // Research is not in the Stockpile -- it is spent the turn it is made -- so its total is
        // gathered from the sources. That is still a total, and gives nothing away.
        let research: i64 = s.income_sources.iter().filter(|(_, r, _)| *r == dying_earth_engine::Resource::Research).map(|(_, _, v)| v).sum();
        ui.label(RichText::new("Income last turn").strong());
        glyph_row(
            ui,
            &[
                RowPart { before: signed(inc.materials), icon: Some("materials"), after: String::new(), hover: Some(hover(dying_earth_engine::Resource::Materials, "Materials")) },
                RowPart { before: signed(inc.fuel), icon: Some("fuel"), after: String::new(), hover: Some(hover(dying_earth_engine::Resource::Fuel, "Fuel")) },
                RowPart { before: signed(inc.energy), icon: Some("energy"), after: String::new(), hover: Some(hover(dying_earth_engine::Resource::Energy, "Energy")) },
                RowPart { before: signed(inc.ducats), icon: Some("ducats"), after: String::new(), hover: Some(hover(dying_earth_engine::Resource::Ducats, "Ducats")) },
                RowPart { before: signed(research), icon: Some("research"), after: String::new(), hover: Some(hover(dying_earth_engine::Resource::Research, "Research")) },
            ],
            15.0,
        );
        // Ticket #203: a spectator has no side to keep secrets from, so they get no line at all.
        // Ticket #306 (version 0.08.7): the line that said where the breakdown is ("Hover a
        // figure...") is cut as a signpost, at the designer's word; the withholding line stays,
        // since it states a rule.
        if !session.spectator && !breakdown {
            ui.label(RichText::new("A rival's income is shown as totals only; the building-by-building breakdown is yours alone.").weak());
        }
        ui.add_space(6.0);

        // 3. Blame: this Faction's share of the CO2 the table has put up. The Climate Panel keeps
        // its own fuller four-Faction breakdown, and the two are not duplicates: that one is the
        // comparison view and this is the detail view, the same relation the Victory window's four
        // progress bars now have with the Victory progress block above.
        // Ticket #233 (version 0.08.3): the sentence that stood under this block is on the heading's
        // hover now, at the designer's word -- and with NO marker to advertise it: *"no other mouse
        // overs have any ? - the convention here is the same, mouseovers are common AF in 4x
        // games"*. It is an ordinary `on_hover_text`, the same call every other hover in the game
        // makes: *"I want this new mouseover to work exactly like all the others have been
        // working"*.
        //
        // Only THIS copy moves. The Climate Panel keeps its own version of the sentence on the page
        // (*"faction windows leave climate as is"*), being a four-Faction comparison read
        // occasionally rather than a page a player sits on, and the top bar's Influence hover keeps
        // its longer wording. Nothing is deleted.
        // Ticket #265 (version 0.08.4): the hover names what Blame is, what the credit is, and the
        // two rules that read it.
        ui.label(RichText::new("Blame").strong()).on_hover_text("Blame is the CO2 this Faction is answerable for: everything the sources it controlled emitted, less everything it removed.\nWhat it removed -- its Scrubbers, and for the Custodians what their Research Directive adds to the Natural Sink -- is its Blame credit.\nTwo rules read Blame: a share above a fair quarter raises this Faction's Influence thresholds on every Region it does not hold, up to half again;\nand every rival thinks a point worse of it for each step its share stands above that quarter, each by its own measure.");
        let share = game.blame_share(seat);
        ui.horizontal(|ui| {
            ui.add(
                egui::ProgressBar::new(share as f32)
                    .desired_width(240.0)
                    .fill(seat_colour(session, seat))
                    .text(RichText::new(format!("{:.0}%", share * 100.0)).color(Color32::BLACK)),
            );
            // Ticket #265 (version 0.08.4): answerable for, then how it got there, then the credit.
            let s = game.seat(seat);
            // Ticket #267: what rivals laid on by Smear, when any, so the line never says the seat
            // put it in the air.
            let smeared = if s.blame_smeared > 0.0 { format!(", {:.0} laid on by rivals", s.blame_smeared) } else { String::new() };
            let smeared = format!("{smeared}{}{}", if s.credits_bought > 0.0 { format!(", {:.0} bought as carbon credits", s.credits_bought) } else { String::new() }, if s.credits_sold > 0.0 { format!(", {:.0} sold as carbon credits", s.credits_sold) } else { String::new() });
            // Ticket #277 (version 0.08.5): the sixth clause, at the designer's word.
            let smeared = format!("{smeared}{}", if s.blame_cleaned > 0.0 { format!(", {:.0} cleaned by campaign", s.blame_cleaned) } else { String::new() });
            let line = format!(
                "Answerable for {:.0} ppm (emitted {:.0}, removed {:.0} in credit{smeared}), thresholds x{:.2}",
                game.blame(seat),
                s.blame_emitted,
                game.blame_credit(seat),
                game.blame_threshold_multiplier(seat)
            );
            figures_with_icons(ui, &line, 14.0, ui.visuals().weak_text_color(), &[("ppm", "emissions")]);
        });
        ui.add_space(6.0);

        // 4. Relations, as TWO ROWS rather than the twelve-pair grid this window took off the
        // Victory window. Twelve ordered pairs as a grid made a player find the right cell; two
        // rows tell them the answer, and a figure kept in two places drifts.
        // Ticket #233 (version 0.08.3): the two paragraphs that stood under this block are one
        // hover on the heading now. Five or six lines of standing prose came off a page the 0.08.1
        // mockups measured at about 950 pixels of content in a 524-wide panel.
        let r = &game.tables.relations;
        let relations_note = format!(
            "{:+} to {:+} from a neutral {}, read as six levels from Friendly to Hostile. A score is what the pair have DONE to each other plus what this Faction makes of the other's Blame -- hover a level for the two figures. Offences differ in weight and a turn charges every one, to {} at most; quiet mends {} every {} turns below neutral and lapses half as fast above it. A pair crossed on {} turns can never fully recover again.
A rival that holds you at less than neutral defends its places against you a little harder, and an Accord wants a level it will not strike below.",
            r.best, r.worst, r.start, r.turn_cap, r.recover, r.quiet_turns, r.scar_turns
        );
        ui.label(RichText::new("Relations").strong()).on_hover_text(relations_note);
        let name = game.seat_name(seat);
        ui.label(RichText::new(format!("What the {name} think of the others")).weak());
        relations_row(ui, session, game, view, seat, true);
        ui.label(RichText::new(format!("What the others think of the {name}")).weak());
        relations_row(ui, session, game, view, seat, false);
        ui.add_space(6.0);

        // 4b. Ticket #226 (version 0.08.2): the Accords, where the designer put them -- "add
        // necessary UI to faction screen". No new screen: this window already has a Faction selector
        // and already shows the two Relations rows a player consults before offering anything, so
        // the controls belong beside them.
        //
        // Only on a RIVAL's page, and never for a spectator, since an Accord is struck between the
        // player's seat and somebody else. Your own page has nobody to strike one with.
        if !session.spectator && seat != Seat(0) {
            accords_block(ui, session, game, seat, actions);
            ui.add_space(6.0);
            smear_block(ui, session, game, view, seat, actions);
            ui.add_space(6.0);
            // Ticket #285 (version 0.08.5): and a request for carbon credits, on the Custodians' page.
            if game.credit_seller() == Some(seat) {
                credits_request_block(ui, session, game, view, seat, actions);
                ui.add_space(6.0);
            }
        }
        // Ticket #277 (version 0.08.5): the Greenwash, on the player's OWN page, the Smear's mirror
        // in place as well as in rule, beside the Blame line its term shows on.
        if !session.spectator && seat == Seat(0) {
            greenwash_block(ui, session, game, view, actions);
            ui.add_space(6.0);
            // Ticket #285 (version 0.08.5): the Custodians offer their credit from their own page.
            if game.credit_seller() == Some(Seat(0)) {
                credits_offer_block(ui, session, game, view, actions);
                ui.add_space(6.0);
            }
        }

        // 5. Holdings, which no window counted for anybody before this one.
        ui.label(RichText::new("Holdings").strong());
        let regions = game.directed_states(seat).len();
        let colonies = game.colonies.iter().filter(|c| c.control.director() == Some(seat) && !c.in_orbit).count();
        let stations = game.colonies.iter().filter(|c| c.control.director() == Some(seat) && c.in_orbit).count();
        let ships = game.ships.iter().filter(|s| s.seat == seat).count();
        let armies = game.armies.iter().filter(|a| game.army_seat(a) == Some(seat) && !game.army_stands_down(a)).count();
        let plural = |n: usize, one: &str, many: &str| format!("{n} {}", if n == 1 { one } else { many });
        ui.label(format!(
            "{}, {}, {}, {}, {}",
            plural(regions, "Region", "Regions"),
            plural(colonies, "Colony", "Colonies"),
            plural(stations, "station", "stations"),
            plural(ships, "Ship", "Ships"),
            plural(armies, "Army", "Armies")
        ));
        ui.add_space(6.0);

        // 6. Ticket #263 (version 0.08.4): **Under way** -- what this Faction has begun and not yet
        // finished: builds with the turns until they land, Ships in transit with their names and
        // their roads, soonest first. The list in full on every page, at the designer's word: a
        // build stands hatched on its card and a transit is drawn on the Solar System Map for
        // anyone to see, so the disclosure rule hides nothing here; it only saves the clicks.
        ui.label(RichText::new("Under way").strong());
        let u = game.under_way(seat);
        // Ticket #332 (version 0.09.0): each build with its count and figure and the estimate,
        // `3 of 8, about 2 turns`, read off the queues here since `under_way` carries the estimate
        // alone; soonest first as before, the name breaking a tie.
        let mut builds: Vec<(String, Place, u32, u32, u32)> = Vec::new();
        for place in StateId::ALL.iter().map(|s| Place::State(*s)).chain(game.colonies.iter().map(|c| Place::Colony(c.id))) {
            for (b, t) in game.queue_at(place).iter().zip(game.queue_estimates(place)).filter(|(b, _)| b.seat == seat) {
                builds.push((b.item.name(), place, b.done, b.widgets, t));
            }
        }
        builds.sort_by(|a, b| a.4.cmp(&b.4).then_with(|| a.0.cmp(&b.0)));
        if builds.is_empty() && u.transits.is_empty() {
            ui.label(RichText::new("Nothing under way.").weak());
        } else {
            let turns = |n: u32| if n == 1 { "1 turn".to_string() } else { format!("{n} turns") };
            if !builds.is_empty() {
                // "in China", "at Tycho on the Moon": a Region is a country, a Colony a place.
                let items: Vec<String> = builds.iter().map(|(what, place, done, widgets, n)| format!("{} {} {} ({done} of {widgets}, {})", with_article(what), if matches!(place, Place::State(_)) { "in" } else { "at" }, game.place_name(*place), estimate_words(*n))).collect();
                ui.label(format!("Building: {}", items.join(", ")));
            }
            if !u.transits.is_empty() {
                // "Earth to Mars" in words: the interface font has no arrow and drew a box for one.
                let items: Vec<String> = u.transits.iter().map(|(name, from, to, n)| format!("{name}, {from} to {to}, {}", turns(*n))).collect();
                ui.label(format!("In transit: {}", items.join("; ")));
            }
        }
        ui.add_space(8.0);

        // The rulebook, SHUT by default: the setup screen's Faction card, the same code, brought
        // in-game. `faction_rulebook_open` is a building aid -- nothing but a `rulebook:1` shot
        // ever sets it, and in play the header opens only when a player opens it.
        egui::CollapsingHeader::new(RichText::new("Rulebook").strong()).id_salt("faction_rulebook").default_open(view.faction_rulebook_open).show(ui, |ui| {
            faction_rulebook(ui, session, kind);
        });
    });
    view.show_factions = open;
}

fn popups(ctx: &egui::Context, session: &Session, game: &Game, view: &mut ViewState, actions: &mut Vec<Action>) {
    if view.show_trade && !session.spectator {
        let mut open = true;
        // Ticket #292 (version 0.08.6): under the bar and to the right of the Faction window's
        // home, at the designer's word. With no position it took egui's fallback, sixteen pixels
        // from the corner, over the bar's figures row.
        let home = view.beside_faction_window();
        egui::Window::new("Trading").open(&mut open).default_width(470.0).default_pos(home).show(ctx, |ui| trading_window(ui, session, game, view, actions));
        view.show_trade = open;
    }
    if view.show_tech {
        let mut open = true;
        // Ticket #194 (version 0.08.0): a default position clear of the Climate Panel, which sits at
        // x 10 (or 420 spectating) and is 400 wide. Found while taking the pictures for this ticket:
        // on the Earth view the Panel covered the Tech Tree's left third, hiding every branch name
        // and four of the five legend swatches, and the first two captures were useless because of
        // it. `default_pos` only places it the first time, so a window the player has dragged stays
        // where they put it.
        // Ticket #242 (version 0.08.3): the window is bounded by the SCREEN and its tree scrolls
        // inside it. The bound is on the window rather than on the ScrollArea because that is what
        // egui actually constrains: a `max_height` on the ScrollArea alone left the window 535
        // pixels tall on an 800-pixel screen, showing two branches where four had fitted before.
        // Ticket #292 (version 0.08.6): under the measured bar, where 120 stood. The tree's own
        // bound is measured inside the window, at the line the tree starts on, below.
        let top = view.below_bar();
        let screen_bottom = ctx.content_rect().bottom();
        egui::Window::new("Tech Tree").open(&mut open).resizable(false).default_pos(egui::pos2(if session.spectator { 840.0 } else { 430.0 }, top)).show(ctx, |ui| {
            // Ticket #211 (version 0.08.1): the race bar stands where this window's first SENTENCE
            // stood, at the designer's word, and the sentence moves onto its hover. One wrinkle,
            // handled rather than lived with: when NO Tech is under research the bar is not drawn at
            // all -- and that is exactly the state this window is open in, with a pick owed -- so
            // the line that says so takes its place and the top of the window is never empty.
            // Ticket #219 (version 0.08.2): the bar, its shares and the legend are ONE block at the
            // top of the window now. With no Tech under research -- the state this window is open in
            // when a pick is owed -- a single sentence stands in place of the whole block, and the
            // legend follows it, so the top never goes empty and never half-empties either.
            match game.research.current {
                Some(_) => {
                    // Ticket #242 (version 0.08.3): the bar says what it is. Ticket #219 put it at
                    // the top of this window with its shares and legend beneath, and nothing there
                    // named the quantity -- a reader saw a coloured bar and four percentages and
                    // had to infer that the subject was Research. The designer: "let's add a
                    // subheading at the top of the tech tree indicating the bar is research share".
                    ui.label(RichText::new("Research share").strong());
                    research_race_bar(ui, session, game, ui.available_width(), true);
                    research_shares(ui, session, game);
                }
                None => {
                    ui.label(format!("No Tech under research. {} Research waiting.", game.research.unallocated.iter().sum::<i64>() + game.research.unattributed));
                }
            }
            tech_legend(ui);
            ui.separator();
            // Ticket #235 (version 0.08.3): the Research Directive, for every Faction, in the one
            // window whose subject is Research.
            if !session.spectator {
                research_directive_control(ui, session, game, actions);
                ui.separator();
            }
            // Ticket #51: an Archivist player is told whether Provisional Findings is in force.
            if game.kind(Seat(0)) == FactionKind::Archivists {
                let on = game.provisional_findings(Seat(0));
                ui.colored_label(
                    if on { Color32::LIGHT_GREEN } else { Color32::GRAY },
                    format!(
                        "Provisional Findings: {}. {}",
                        if on { "on" } else { "off" },
                        if on {
                            "You already have half the effect of the Tech under research."
                        } else {
                            "You funded the Archive last turn, so this turn you have none of it."
                        }
                    ),
                );
                if game.seat(Seat(0)).research_directive > 0 || session.pending.iter().any(|o| matches!(o, Order::SetResearchDirective { percent } if *percent > 0)) {
                    ui.colored_label(Color32::YELLOW, "Your Labs pay the Archive fund: the turn after they next pay it, Provisional Findings is off.");
                }
            }
            // Ticket #173 (version 0.07.6): the tree keeps offering its Pick buttons while the
            // choice can still be changed -- a Tech picked this turn is not locked in until the turn
            // ends -- not only while none has been picked at all.
            let must_pick = game.research.awaiting_pick == Some(Seat(0)) || !game.research.pick_committed;
            if game.research.current.is_none() && must_pick {
                ui.colored_label(Color32::YELLOW, "You pick the next Tech: choose one below.");
            } else if !game.research.pick_committed {
                // Ticket #173: the pick is made but not final; say so where the player is looking.
                ui.colored_label(Color32::from_rgb(210, 190, 120), "Chosen for this turn. Press another box to change it; it is locked in when the turn ends.");
            }
            // Ticket #98: the Lead chooses from the drawn shortlist, so that is what the tree offers.
            let available = game.pickable_techs();
            // Ticket #242 (version 0.08.3): the tree SCROLLS when it does not fit, the scrollbar
            // showing only when it is needed -- the decision ticket #217 already took for the
            // Faction selection screen, so the game answers this problem the same way twice.
            //
            // It became necessary here because ticket #232's two new Techs gave Off-world Living
            // and Extraction a second row each: the tree was 672 pixels tall and is now 864. A
            // headless capture at 1280x800 showed the WHOLE SOCIETY BRANCH off the bottom of the
            // screen -- Public Science, Green Consensus, Civil Defense and two Victory gates -- with
            // no scrollbar and the window fixed at `resizable(false)`. At 1920x1080, which is what
            // the game opens maximised into, it fitted with about thirty pixels to spare.
            //
            // The height is set from the screen rather than left to egui's default, and the first
            // attempt is why: a bare `ScrollArea::vertical()` took a default height and showed TWO
            // branches where four had fitted before, which is a worse window than the one it
            // replaced. It takes everything between the cursor and the bottom of the screen, less
            // a margin for the window's own frame, and `auto_shrink` upward so a tree that fits is
            // drawn whole with no scrollbar at all.
            // The bound is computed from the SCREEN and the window's own top, never from
            // `ui.cursor()`: the first attempt used the cursor and left the window 535 pixels tall
            // on an 800-pixel screen, showing two branches where four had fitted before. A
            // `max_height` on the Window does not help either -- it is a cap, and a window sizes
            // itself to its content, so the ScrollArea is what has to be told.
            // Ticket #292 (version 0.08.6): the bound is the screen's foot less the line the tree
            // starts on, read from the cursor now that the window's top is measured rather than
            // guessed; with the guess retired the old sum (top plus forty) pushed the window past
            // the foot of an 800-pixel screen and egui shoved it up over the bar.
            let room = (screen_bottom - ui.cursor().top() - 40.0).max(240.0);
            egui::ScrollArea::vertical().auto_shrink([false, true]).max_height(room).min_scrolled_height(room).show(ui, |ui| {
                tech_tree(ui, game, &available, must_pick, actions);
            });
        });
        view.show_tech = open;
    }
    if view.show_climate {
        let mut open = true;
        let bottom = ctx.viewport_rect().max.y;
        // Ticket #64: the spectator's card sits on the left, so the panel's home is clear of it.
        // `climate:top` (a building aid, not part of the spec): the panel opens at the top of the
        // window instead of 400 rows off the bottom, so a tall window can photograph the whole of
        // it. The panel is otherwise always about 400 rows tall and scrolls, and a scroll cannot be
        // driven headlessly -- which also means the Blame block at its foot is below the fold for a
        // player at 1280x800 until they scroll.
        let top = std::env::args().any(|a| a == "climate:top");
        // Ticket #165 (version 0.07.5): the panel opens at the TOP, under the top bar, and as tall
        // as the window leaves room for. The designer: *"climate panel should start long enough to
        // show all the information in the panel."* It held about a thousand rows of content and
        // opened four hundred tall, four hundred rows off the bottom, so two thirds of it -- the
        // whole Blame block among them -- were below the fold before the player touched it. Opened
        // this way a maximised window shows the lot and a small one still scrolls.
        // Ticket #292 (version 0.08.6): under the measured bar, where 104 stood.
        let bar = view.below_bar();
        let home = (if session.spectator { 420.0 } else { 10.0 }, if top { 10.0 } else { bar });
        let tall = (bottom - home.1 - 12.0).max(300.0);
        // Ticket #104 (version 0.07.0): the panel could be dragged larger but not back down. Its
        // content is not what held it: rendered at 230 wide everything wraps and nothing overflows.
        // So the width is made authoritative -- an explicit floor it may be dragged to, and a scroll
        // rather than unbounded growth, since the panel is taller than the screen on a small window.
        let mut window = egui::Window::new("Climate Panel")
            .open(&mut open)
            .default_pos(home)
            .default_width(400.0)
            // The aid still forces its own height: with a scroll area inside, the window settles at
            // whatever it is given, and a picture wants more rows than the screen has.
            .default_height(if top { 1300.0 } else { tall })
            .resizable(true)
            .min_width(230.0)
            .min_height(140.0)
            .vscroll(true);
        if view.climate_reopen {
            window = window.current_pos(home);
            view.climate_reopen = false;
        }
        window.show(ctx, |ui| {
            let c = &game.climate;
            let e = &c.last;
            ui.label(RichText::new(format!("CO2 Stock {:.1} ppm", c.co2)).strong());
            ui.label(format!("Temperature {:+.1} C, heading to {:+.1}", c.temperature, game.target_temperature()));
            // Ticket #55: the Temperature bar, with every notch the game turns on.
            temperature_bar(ui, game);
            ui.separator();
            icon_word(ui, "emissions", "Emissions this turn, by source");
            ui.label(format!("Region industry {:.1}", e.state_industry));
            ui.label(format!("Factories {:.1}", e.factories));
            ui.label(format!("Power Plants {:.1}", e.power_plants));
            ui.label(format!("Refineries {:.1}", e.refineries));
            ui.label(format!("Launches {:.1}", e.launches));
            {
                let c = &game.tables.climate;
                ui.label(format!("Population {:.1}", e.population)).on_hover_text(format!(
                    "Each state's people emit {:.2} + {:.2} x its Industry Level per hundred million, halved by Green Consensus, times its controller's Emissions multiplier. The Custodians' Leapfrog lowers a state's own figure by {:.2} for good, never below {:.2}.",
                    c.population_emissions_base * Game::UNITS_PER_HUNDRED_MILLION,
                    c.population_emissions_per_level * Game::UNITS_PER_HUNDRED_MILLION,
                    c.population_emissions_per_level * Game::UNITS_PER_HUNDRED_MILLION,
                    c.population_emissions_base * Game::UNITS_PER_HUNDRED_MILLION
                ));
            }
            if e.cards > 0.0 {
                ui.label(format!("Event cards {:.1}", e.cards));
            }
            // Ticket #279 (version 0.08.5): war's own line, when there was one.
            if e.war > 0.0 {
                ui.label(format!("War {:.1}", e.war)).on_hover_text(format!(
                    "Last turn's Battles on Earth and in Earth orbit: {} ppm for every hit landed, worn as Blame by whoever landed it, and {} for every building burned when a place is taken by an Occupation that ran its three turns, worn by the taker. A Battle itself burns nothing, and a place Pacified is taken whole. A neutral Region's Army's hits are nobody's. It counts against a Stabilization run: a war a Faction chose is not the weather.",
                    game.tables.climate.war_ppm_per_hit, game.tables.climate.war_ppm_per_building
                ));
            }
            // Ticket #55: the Permafrost Thaw Break's own line, once it has fired. It is the world's
            // carbon: nobody's Blame, and it never counts against a Stabilization run.
            if e.permafrost > 0.0 {
                ui.label(format!("Permafrost {:.1}", e.permafrost))
                    .on_hover_text("The permafrost has thawed. This much CO2 comes out of the ground every turn now, whatever anybody does. It is nobody's Blame and it does not count against a Stabilization run.");
            }
            // Ticket #54: the Scrubbers stand beside the Natural Sink in the same line.
            ui.label(format!("Natural Sink -{:.1}{}", e.sink, if e.scrubbers > 0.0 { format!(" and Scrubbers -{:.1}", e.scrubbers) } else { String::new() }));
            ui.label(RichText::new(format!("Net {:+.1} ppm", e.net())).strong());
            // Ticket #153 (version 0.07.4): the same history, at the panel's width.
            ui.add_space(4.0);
            emissions_history(ui, game, egui::vec2(ui.available_width(), 110.0));
            // Ticket #175 (version 0.07.6): the population history stood here, above the growth rate
            // that drives it, until the designer took it off: *"remove pop graph from climate
            // window."* It keeps the top bar's Population hover, where it now has the picture to
            // itself. **The Emissions history stays**, at the designer's word and for the reason the
            // ticket gave: this is the page about emissions, and that chart is the page's own
            // subject over time, where the population chart was a guest. Nothing fills the space and
            // the Emissions chart keeps the height it had; a chart that grows because a neighbour
            // left is a chart sized by accident.
            ui.separator();
            let growth = game.population_growth_rate() * 100.0;
            ui.label(format!(
                "Penalties in force: population growth {:+.2}% per turn; a card comes {:.0}% of turns at this Temperature ({} cards left in the deck, {} of them Climate).",
                growth,
                game.draw_chance() * 100.0,
                game.deck.cards.len(),
                game.deck.climate_cards_left()
            ));
            // Ticket #55: what the Stock already commits the world to, and how long cutting can
            // still avoid the Collapse Line.
            ui.label(
                RichText::new(format!("Already locked in: {:+.1} C even if net Emissions stopped today", game.target_temperature()))
                    .size(15.0)
                    .color(Color32::from_rgb(240, 180, 140)),
            )
            .on_hover_text("The Temperature the CO2 Stock as it stands will deliver once the lag has caught up. Nothing anybody builds or stops building takes it back.");
            let (last_turn, last_colour) = match game.last_turn_to_act() {
                LastTurn::Turn(t) => (format!("Last turn to act: {t}"), Color32::from_rgb(240, 210, 120)),
                LastTurn::TooLate => ("Cuts alone no longer avoid Collapse.".to_string(), Color32::from_rgb(240, 110, 100)),
                LastTurn::NoCollapse => ("On this path Collapse is not reached.".to_string(), Color32::from_rgb(150, 220, 160)),
            };
            ui.label(RichText::new(last_turn).size(15.0).strong().color(last_colour)).on_hover_text(
                "The latest turn on which cutting net Emissions to zero from that turn onward still keeps the Temperature under the Collapse Line by the last turn, counting every Break the world would cross on the way.",
            );
            let p = game.projection();
            let line = match p.collapse_turn {
                Some(t) => format!("At this rate, {:+.1} C by turn {}; Collapse at +{:.1} around turn {}.", p.temperature_at_last_turn, game.tables.victory.turns, game.tables.climate.collapse_line, t),
                None => format!("At this rate, {:+.1} C by turn {}; Collapse at +{:.1} not reached.", p.temperature_at_last_turn, game.tables.victory.turns, game.tables.climate.collapse_line),
            };
            ui.label(RichText::new(line).size(16.0).strong().color(Color32::from_rgb(255, 200, 120)));
            ui.separator();
            ui.label(format!("Stabilization run: {} consecutive turn(s) under the Sink.", game.seat(Seat(0)).stabilization_run));
            // Ticket #53: Blame, Faction by Faction, in the panel that attributes the Emissions.
            ui.separator();
            ui.label(RichText::new("Blame: the CO2 each Faction is answerable for").strong()).on_hover_text("Blame is the CO2 this Faction is answerable for: everything the sources it controlled emitted, less everything it removed.\nWhat it removed -- its Scrubbers, and for the Custodians what their Research Directive adds to the Natural Sink -- is its Blame credit.\nTwo rules read Blame: a share above a fair quarter raises this Faction's Influence thresholds on every Region it does not hold, up to half again;\nand every rival thinks a point worse of it for each step its share stands above that quarter, each by its own measure.");
            for seat in Seat::ALL {
                let s = game.seat(seat);
                // Ticket #265 (version 0.08.4): one form for every seat -- answerable for, how it
                // got there, and what it holds in credit -- at the designer's word.
                let smeared = if s.blame_smeared > 0.0 { format!(", {:.0} laid on by rivals", s.blame_smeared) } else { String::new() };
                let smeared = format!("{smeared}{}{}", if s.credits_bought > 0.0 { format!(", {:.0} bought as carbon credits", s.credits_bought) } else { String::new() }, if s.credits_sold > 0.0 { format!(", {:.0} sold as carbon credits", s.credits_sold) } else { String::new() });
            // Ticket #277 (version 0.08.5): the sixth clause, at the designer's word.
            let smeared = format!("{smeared}{}", if s.blame_cleaned > 0.0 { format!(", {:.0} cleaned by campaign", s.blame_cleaned) } else { String::new() });
                let line = format!(
                    "{}: answerable for {:.0} ppm (emitted {:.0}, removed {:.0} in credit{smeared}); share {:.2}, thresholds x{:.2}",
                    game.seat_name(seat),
                    game.blame(seat),
                    s.blame_emitted,
                    game.blame_credit(seat),
                    game.blame_share(seat),
                    game.blame_threshold_multiplier(seat)
                );
                // Ticket #112 (version 0.07.1): each Faction's ppm figures wear the Emissions
                // glyph, so the figure a Faction is answerable for is marked as the same thing the
                // Facility lists and the top bar count. "share" and "thresholds" are not ppm and
                // keep their words.
                figures_with_icons(ui, &line, 14.0, seat_colour(session, seat), &[("ppm", "emissions")]);
            }
            // Ticket #306 (version 0.08.7): the sentence that stood here, a clause of the heading's
            // hover word for word, is cut at the designer's word.
        });
        view.show_climate = open;
    }
    if view.show_victory {
        let mut open = true;
        // Ticket #292 (version 0.08.6): the same home as Trading, for the same reason.
        let home = view.beside_faction_window();
        egui::Window::new("Victory").open(&mut open).default_width(470.0).default_pos(home).show(ctx, |ui| {
            // Ticket #50: a row per seat, in seat order, each headed by its Faction in its colour.
            for seat in Seat::ALL {
                let p = game.progress(seat);
                // Ticket #203 (version 0.08.1): the Faction's name here OPENS ITS PAGE in the Faction
                // window. One of exactly two places that do -- the other is the Relations rows in that
                // window itself -- because these are the two places where a player is already comparing
                // Factions and the next thought is "tell me more about that one". A Region card's
                // `Held by the Archivists` and the roster are deliberately left alone: there a Faction's
                // name describes a PLACE, and a click there must go on selecting the place.
                faction_link(ui, view, seat, RichText::new(format!("{} - {:.0}% of the way there", game.seat_name(seat), p.score() * 100.0)).strong().color(seat_colour(session, seat)));
                // Ticket #306 (version 0.08.7): the Victory Condition sentence that stood here is
                // cut, at the designer's word, as a restatement of the two labelled bars beneath
                // it; the Rulebook keeps its copy.
                ui.label(match &p.first_held_back {
                    Some(why) => format!("{}: {:.0} of {:.0} - {}", p.first_name, p.first_value, p.first_bar, why),
                    None => format!("{}: {:.0} of {:.0}", p.first_name, p.first_value, p.first_bar),
                });
                ui.add(egui::ProgressBar::new(p.first_fraction() as f32));
                // Ticket #51: the second part in the words its own card uses.
                ui.label(format!("{}: {}", p.second_name, p.second_text));
                ui.add(egui::ProgressBar::new(p.second_fraction() as f32));
                // Ticket #72: the Prospectors set their Venture Capital Fund's share here, and draw.
                // Ticket #256 (version 0.08.4): a slider and a Withdraw field, in their own function.
                if seat == Seat(0) && !session.spectator && game.kind(Seat(0)) == FactionKind::Prospectors {
                    venture_fund_control(ui, session, game, view, actions);
                }
                ui.add_space(8.0);
            }
            ui.label(format!("Collapse Line +{:.1} C; the Temperature is {:+.1}.", game.tables.climate.collapse_line, game.climate.temperature));
        });
        view.show_victory = open;
    }
    // Ticket #203 (version 0.08.1): the Faction window, which took Relations and Blame off the
    // window above and brought the setup screen's Faction card in-game behind them.
    faction_window(ctx, session, game, view, actions);
    match view.popup {
        // Ticket #105 (version 0.07.0): the engine refused to end the turn, and says why. The rule
        // is worth nothing if the player is left wondering why the button did nothing.
        Popup::Refused => {
            let why = session.refusal.clone().unwrap_or_else(|| "The turn cannot end yet.".to_string());
            egui::Modal::new("refused".into()).show(ctx, |ui| {
                ui.set_width(460.0);
                ui.label(RichText::new("The turn did not end").size(20.0).strong());
                ui.label(why);
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    // Ticket #163: the same words and the same job as the bar's, so the same red.
                    if ui.add(egui::Button::new(RichText::new("Pick a Tech").strong()).fill(TURN_RED)).clicked() {
                        view.popup = Popup::None;
                        view.show_tech = true;
                    }
                    if ui.button("Back").clicked() {
                        view.popup = Popup::None;
                    }
                });
            });
        }
        Popup::Event => {
            let text = game.last_event.as_ref().map(|e| e.text.clone()).unwrap_or_default();
            egui::Modal::new("event".into()).show(ctx, |ui| {
                ui.set_width(460.0);
                ui.label(RichText::new("Event drawn").size(20.0).strong());
                ui.label(text);
                // Only Climate cards scale with the Temperature (ticket #31); the rest say so.
                let climate = game.last_event.as_ref().map(|e| matches!(e.card, Card::Event(id) if game.tables.event(id).kind == EventKind::Climate)).unwrap_or(false);
                if climate {
                    ui.label(RichText::new(format!("A Climate card: x{:.2} at this Temperature.", game.last_event.as_ref().map(|e| e.scale).unwrap_or(1.0))).weak());
                } else {
                    ui.label(RichText::new("Not a Climate card: the Temperature does not change it.").weak());
                }
                if ui.button("Continue").clicked() {
                    let moments = view.moments_of(&session.tables, &game.report).len();
                    advance_popup(view, moments, false);
                }
            });
        }
        Popup::ConfirmEndTurn => {
            let (_, influence_left) = game.remaining(Seat(0), &session.pending);
            egui::Modal::new("confirm_end".into()).show(ctx, |ui| {
                ui.set_width(420.0);
                ui.label(RichText::new("Influence unspent").size(20.0).strong());
                ui.label(format!("{influence_left} Influence is unspent; it is lost at End Turn. End the turn anyway?"));
                ui.horizontal(|ui| {
                    if ui.button("End Turn (Enter)").clicked() {
                        view.popup = Popup::None;
                        actions.push(Action::EndTurn);
                    }
                    if ui.button("Back").clicked() {
                        view.popup = Popup::None;
                    }
                });
            });
        }
        Popup::Report => {
            egui::Modal::new("report".into()).show(ctx, |ui| {
                ui.set_width(620.0);
                // Ticket #58: a dated bulletin.
                ui.label(RichText::new(format!("Report, {}", game.date(game.report.turn).text())).size(20.0).strong());
                // Ticket #50: who is at this table, and under which seed.
                // Ticket #64: a spectator has no seat, so the line names all four.
                ui.horizontal_wrapped(|ui| {
                    if session.spectator {
                        ui.label(RichText::new(format!("Seed {}. Spectating.", game.seed)).weak());
                        for seat in Seat::ALL {
                            ui.label(RichText::new(game.seat_name(seat)).color(seat_colour(session, seat)));
                        }
                    } else {
                        ui.label(RichText::new(format!("Seed {}. You:", game.seed)).weak());
                        ui.label(RichText::new(game.seat_name(Seat(0))).strong().color(seat_colour(session, Seat(0))));
                        ui.label(RichText::new("Computer:").weak());
                        for seat in Seat::ALL.into_iter().skip(1) {
                            ui.label(RichText::new(game.seat_name(seat)).color(seat_colour(session, seat)));
                        }
                    }
                });
                // The headline: the most severe thing that happened, in its own size.
                if let Some(head) = game.report.headline() {
                    ui.add_space(4.0);
                    let label = ui.label(RichText::new(&head.text).size(17.0).strong().color(Color32::from_rgb(255, 220, 150)));
                    if let Some(place) = head.place
                        && label.interact(egui::Sense::click()).on_hover_text("Go there").clicked()
                    {
                        actions.push(Action::GoTo(place));
                    }
                }
                ui.separator();
                egui::ScrollArea::vertical().max_height(520.0).show(ui, |ui| {
                    // The four headings, empty ones left out; every line with a place is a way there.
                    for (section, lines) in game.report.sections() {
                        ui.label(RichText::new(section.name_for(session.spectator)).strong());
                        for l in lines {
                            match l.place {
                                Some(place) => {
                                    // Ticket #127 (version 0.07.2): a line that points somewhere wears
                                    // the glyph of what it points to. A Body is no one kind of thing.
                                    let kind = match place {
                                        dying_earth_engine::report::ReportPlace::State(_) => Some(Kind::Region),
                                        dying_earth_engine::report::ReportPlace::Colony(c) => game.colony(c).map(Kind::of_colony),
                                        dying_earth_engine::report::ReportPlace::Body(_) => None,
                                    };
                                    let button = match kind.and_then(|k| k.image(ui.ctx(), 14.0)) {
                                        Some(image) => egui::Button::image_and_text(image, &l.text),
                                        None => egui::Button::new(&l.text),
                                    };
                                    if ui.add(button.frame(false)).on_hover_text("Go there").clicked() {
                                        actions.push(Action::GoTo(place));
                                    }
                                }
                                None => {
                                    ui.label(&l.text);
                                }
                            }
                        }
                        ui.add_space(4.0);
                    }
                    if !game.report.battles.is_empty() {
                        ui.label(RichText::new("Battle Report").strong());
                        // Ticket #50: a Battle is a melee, so every party present takes its own line.
                        for b in &game.report.battles {
                            ui.label(RichText::new(&b.place).strong());
                            for party in &b.parties {
                                let who = party.seat.map(|s| game.seat_name(s)).unwrap_or_else(|| "Neutral".to_string());
                                let colour = party.seat.map(|s| seat_colour(session, s)).unwrap_or(Color32::LIGHT_GRAY);
                                // Ticket #281 (version 0.08.5): every unit by name and what it took,
                                // and the odds the attacker faced, labelled for what they are.
                                let attacking = match (party.aggressor, party.odds) {
                                    (true, Some(o)) => format!(", attacking at {:.0}% first-round odds", o * 100.0),
                                    (true, None) => ", attacking".to_string(),
                                    _ => String::new(),
                                };
                                ui.label(RichText::new(format!("   {who}{attacking}: {} (strength {}, {} hit(s) landed)", party.units, party.strength, party.hits)).color(colour));
                            }
                            ui.label(format!("   {}", b.result));
                        }
                        ui.add_space(4.0);
                    }
                    // Ticket #58: what each rival Faction did, one paragraph each, in seat order.
                    // Ticket #64: a spectator has no rivals, so all four Factions are told.
                    let paragraphs = game.faction_paragraphs();
                    if !paragraphs.is_empty() {
                        ui.label(RichText::new(if session.spectator { "What the Factions did" } else { "What the rival Factions did" }).strong());
                        for (seat, text) in paragraphs {
                            ui.label(RichText::new(text).color(seat_colour(session, seat)));
                        }
                    }
                });
                ui.separator();
                moments_corner(ui, session, view);
                if ui.button("Close").clicked() {
                    let moments = view.moments_of(&session.tables, &game.report).len();
                    advance_popup(view, moments, false);
                }
            });
        }
        Popup::Tutorial => {
            // Ticket #169 (version 0.07.5): the tutorial's note for this turn. Drawn as a Moment is
            // drawn, because it is the same thing to the player: the turn stops for one short
            // thought. Nothing is forced and nothing is checked -- the note says where to look.
            let Some(note) = tutorial_note(&session.tables, game.turn).cloned() else {
                advance_popup(view, view.moments_of(&session.tables, &game.report).len(), game.last_event.is_some());
                return;
            };
            let last = session.tables.tutorial.note.iter().map(|n| n.turn).max() == Some(game.turn);
            egui::Modal::new("tutorial".into()).show(ctx, |ui| {
                ui.set_width(460.0);
                ui.label(RichText::new(&note.figure).size(26.0).strong().color(Color32::from_rgb(255, 220, 150)));
                ui.label(RichText::new(&note.text).size(17.0));
                if let Some(line) = &note.note {
                    ui.label(RichText::new(line).size(15.0).color(Color32::from_rgb(200, 220, 255)));
                }
                ui.add_space(8.0);
                if ui.button(if last { "Play on" } else { "Go on" }).clicked() {
                    actions.push(Action::TutorialNoteRead);
                }
            });
        }
        Popup::Moment(i) => {
            let shown = view.moments_of(&session.tables, &game.report);
            let Some(m) = shown.get(i).copied().cloned() else {
                view.popup = Popup::Report;
                return;
            };
            let count = shown.len();
            egui::Modal::new("moment".into()).show(ctx, |ui| {
                ui.set_width(if m.tech.is_some() { 780.0 } else { 460.0 });
                // Ticket #261 (version 0.08.4): a rival's Moment wears that Faction's colour on its figure.
                let figure_colour = m.seat.map(|s| seat_colour(session, s)).unwrap_or(Color32::from_rgb(255, 220, 150));
                ui.label(RichText::new(&m.figure).size(30.0).strong().color(figure_colour));
                ui.label(RichText::new(&m.text).size(17.0));
                if let Some(note) = &m.note {
                    ui.label(RichText::new(note).size(15.0).color(Color32::from_rgb(200, 220, 255)));
                }
                // Ticket #58: a completed Tech shows the tree with its new box lit, and the Pick
                // buttons when the player is the Research Lead.
                if m.tech.is_some() {
                    ui.separator();
                    // Ticket #173: the same, in the Moment that a completed Tech opens.
                    let must_pick = game.research.awaiting_pick == Some(Seat(0)) || !game.research.pick_committed;
                    let available = game.pickable_techs();
                    tech_tree(ui, game, &available, must_pick, actions);
                }
                ui.horizontal(|ui| {
                    if ui.button("Close").clicked() {
                        advance_popup(view, count, false);
                    }
                    ui.label(RichText::new(format!("{} of {}", i + 1, count)).weak());
                });
            });
        }
        Popup::None => {}
    }
    if session.screen == Screen::GameOver && view.popup == Popup::None {
        egui::Modal::new("gameover".into()).show(ctx, |ui| {
            ui.set_width(520.0);
            ui.label(RichText::new("Game over").size(26.0).strong());
            ui.label(RichText::new(game.outcome_text()).size(18.0));
            ui.label(format!("Turn {}. Seed {}.", game.turn, game.seed));
            for seat in Seat::ALL {
                let p = game.progress(seat);
                ui.label(
                    RichText::new(format!(
                        "{}: {} {:.0} of {:.0}; {} ({:.0}% of its Victory Condition).",
                        game.seat_name(seat),
                        p.first_name,
                        p.first_value,
                        p.first_bar,
                        p.second_text,
                        p.score() * 100.0
                    ))
                    .color(seat_colour(session, seat)),
                );
            }
            ui.horizontal(|ui| {
                if ui.button("Title screen").clicked() {
                    actions.push(Action::ToTitle);
                }
                if ui.button("Quit").clicked() {
                    actions.push(Action::Quit);
                }
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ticket #205 (version 0.08.1): a tutorial note must hand on to the turn's Event when there is
    /// one. It did so when the note was dismissed by its button, because `Action::TutorialNoteRead`
    /// checked for itself, and NOT when it was dismissed with Escape, which came through
    /// `advance_popup` where no Event arm existed. A new player -- the only player a tutorial has --
    /// could therefore be hit by an Event and never told.
    #[test]
    fn a_tutorial_note_hands_on_to_the_turns_event() {
        let next = |popup: Popup, moments: usize, has_event: bool| {
            let mut view = ViewState { popup, ..Default::default() };
            advance_popup(&mut view, moments, has_event);
            view.popup
        };

        assert_eq!(next(Popup::Tutorial, 0, true), Popup::Event, "a note with an Event behind it hands on to the Event");
        // With no Event the older order stands: the Moments, then the Report.
        assert_eq!(next(Popup::Tutorial, 2, false), Popup::Moment(0), "no Event, but Moments to show");
        assert_eq!(next(Popup::Tutorial, 0, false), Popup::Report, "no Event and no Moments: straight to the Report");
        // An Event already on screen never hands on to itself, whatever the flag says.
        assert_eq!(next(Popup::Event, 0, true), Popup::Report, "the Event is the thing being dismissed");
    }
}

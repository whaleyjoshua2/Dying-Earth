//! The interface (spec 17): views, the top bar, panels, popups, the Battle Report, and picking.

use crate::app::*;
use crate::geo;
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
    EndTurn,
    PickTech(TechId),
    /// Ticket #58: a Report line was clicked; go where it points.
    GoTo(ReportPlace),
    ChooseFaction(FactionKind),
    NewGame(FactionKind, StateId),
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
}

pub fn keyboard(keys: Res<ButtonInput<KeyCode>>, mut view: ResMut<ViewState>, mut session: ResMut<Session>, contexts: Option<Res<bevy_egui::input::EguiWantsInput>>) {
    if session.screen != Screen::Playing {
        return;
    }
    if contexts.map(|c| c.wants_keyboard_input()).unwrap_or(false) {
        return;
    }
    if keys.just_pressed(KeyCode::Tab) {
        view.swap();
    }
    // Ticket #41: C toggles the Climate Panel, a second way back once it is closed.
    if keys.just_pressed(KeyCode::KeyC) {
        toggle_climate(&mut view);
    }
    if keys.just_pressed(KeyCode::Escape) {
        // Ticket #64: Escape unticks Auto, so a spectator can always stop the clock.
        if session.auto {
            session.auto = false;
            session.auto_elapsed = 0.0;
        }
        if view.popup != Popup::None {
            let moments = session.game.as_ref().map(|g| view.moments_of(&session.tables, &g.report).len()).unwrap_or(0);
            advance_popup(&mut view, moments);
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

/// Ticket #58: Event, then the turn's Moments one after another, then the Report.
fn advance_popup(view: &mut ViewState, moments: usize) {
    view.popup = match view.popup {
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
    textures: Res<Textures>,
    handles: Option<Res<SceneHandles>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !session.earth_dirty {
        return;
    }
    let Some(handles) = handles else { return };
    let rgba = match &session.game {
        Some(game) => textures.compose_earth(game, &session.colours()),
        None => crate::textures::Rgba { w: textures.earth.w, h: textures.earth.h, data: textures.earth.data.clone() },
    };
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

/// A shield with a number on it: the Army icon of the Earth Map.
fn shield(painter: &egui::Painter, centre: Pos2, fill: Color32, text: &str) {
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
}

fn label_at(painter: &egui::Painter, pos: Pos2, text: &str, colour: Color32, size: f32) {
    let galley = painter.layout_no_wrap(text.to_string(), FontId::proportional(size), colour);
    let rect = egui::Rect::from_center_size(pos, galley.size() + egui::vec2(8.0, 4.0));
    painter.rect_filled(rect, 3.0, Color32::from_black_alpha(170));
    painter.galley(rect.min + egui::vec2(4.0, 2.0), galley, colour);
}

/// The same, slid sideways so a long line stays on screen (ticket #57: the launch-window tooltip is
/// wider than a Body's other labels, and Mars can stand at the edge of its ring).
fn label_on_screen(painter: &egui::Painter, pos: Pos2, text: &str, colour: Color32, size: f32) {
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
    time: Res<Time>,
    mut exit: MessageWriter<AppExit>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let mut root = Ui::new(ctx.clone(), "viewport".into(), egui::UiBuilder::new().layer_id(egui::LayerId::background()).max_rect(ctx.viewport_rect()));
    let mut actions: Vec<Action> = Vec::new();
    view.spin += time.delta_secs() * 0.25;
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
    match session.screen.clone() {
        Screen::Title => title_screen(&mut root, &mut session, &mut actions),
        Screen::Load => load_screen(&mut root, &session, &mut actions),
        Screen::ChooseFaction => faction_screen(&mut root, &session, &mut actions),
        Screen::ChooseStart { faction } => start_screen(&mut root, &session, faction, &mut actions),
        Screen::Playing | Screen::GameOver => {
            let cam = camera.single().ok();
            game_screen(&mut root, ctx, &session, &mut view, cam, &globes, &textures, &mut actions);
        }
    }
    for a in actions {
        match a {
            Action::Place(o) => {
                session.place(o);
            }
            Action::Cancel(i) => {
                if i < session.pending.len() {
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
                view.selection = Selection::None;
                let moments = session.game.as_ref().map(|g| view.moments_of(&session.tables, &g.report).len()).unwrap_or(0);
                view.popup = if session.game.as_ref().and_then(|g| g.last_event.as_ref()).is_some() {
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
                session.new_game(f, s);
                *view = ViewState::default();
                view.popup = Popup::Report;
                let (lon, lat) = geo::state_lonlat(s);
                view.yaw = geo::yaw_facing(lon, lat);
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
            if ui.add(egui::Button::new(RichText::new("New Game").size(22.0)).min_size(egui::vec2(220.0, 44.0))).clicked() {
                session.screen = Screen::ChooseFaction;
            }
            ui.add_space(10.0);
            // Ticket #59: every save this machine holds, newest first.
            if ui.add(egui::Button::new(RichText::new("Load").size(22.0)).min_size(egui::vec2(220.0, 44.0))).clicked() {
                actions.push(Action::OpenLoad);
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
        egui::ScrollArea::vertical().show(ui, |ui| {
            for row in FactionKind::ALL.chunks(2) {
                ui.columns(2, |cols| {
                    for (i, kind) in row.iter().enumerate() {
                        faction_card(&mut cols[i], session, *kind, actions);
                    }
                });
                ui.add_space(6.0);
            }
        });
    });
}

/// One Faction's card: its colour swatch and name, its blurb, its multipliers, its signature rule
/// and its Victory Condition in plain words.
fn faction_card(ui: &mut Ui, session: &Session, kind: FactionKind, actions: &mut Vec<Action>) {
    let card = session.tables.faction(kind);
    egui::Frame::group(ui.style()).inner_margin(12.0).show(ui, |ui| {
        ui.horizontal(|ui| {
            let (swatch, _) = ui.allocate_exact_size(egui::vec2(24.0, 24.0), egui::Sense::hover());
            ui.painter().rect_filled(swatch, 4.0, rgb(card.colour));
            ui.label(RichText::new(&card.name).size(24.0).strong().color(rgb(card.colour)));
        });
        ui.label(&card.blurb);
        ui.add_space(6.0);
        ui.label(RichText::new("Multipliers").strong());
        ui.label(format!(
            "Facility and Module output x{}; Emissions from Earth sources it controls x{}; Research x{}; Influence Allotment x{}",
            card.output_multiplier, card.emissions_multiplier, card.research_multiplier, card.influence_multiplier
        ));
        // Ticket #51: a card may carry figures of its own beyond the four; list only the ones it moved.
        let mut extras: Vec<String> = Vec::new();
        if card.habitat_capacity_multiplier != 1.0 {
            extras.push(format!("Habitat capacity x{}", card.habitat_capacity_multiplier));
        }
        if card.transit_fuel_multiplier != 1.0 {
            extras.push(format!("transit Fuel x{}", card.transit_fuel_multiplier));
        }
        if card.colony_ship_capacity_multiplier != 1.0 {
            extras.push(format!("Colony Ship capacity x{}", card.colony_ship_capacity_multiplier));
        }
        if card.lift_population_multiplier != 1.0 {
            extras.push(format!("population per lifted Colonist x{}", card.lift_population_multiplier));
        }
        if let Some(m) = card.colony_ship_materials {
            extras.push(format!("a Colony Ship {m} Materials"));
        }
        // Ticket #83: the Arkwrights' Ships, the Prospectors' Ducats and market.
        if card.ship_materials_multiplier != 1.0 {
            extras.push(format!("every Ship x{} Materials", card.ship_materials_multiplier));
        }
        if card.ducats_multiplier != 1.0 {
            extras.push(format!("a state's Ducats x{}", card.ducats_multiplier));
        }
        if card.market_multiplier != 1.0 {
            extras.push(format!("the Trading window's prices x{}", card.market_multiplier));
        }
        if card.station_materials_multiplier != 1.0 {
            extras.push(format!("a Space Station x{} Materials", card.station_materials_multiplier));
        }
        if card.module_materials_multiplier != 1.0 {
            extras.push(format!("a Colony Module x{} Materials", card.module_materials_multiplier));
        }
        if !extras.is_empty() {
            ui.label(extras.join("; "));
        }
        ui.add_space(6.0);
        ui.label(RichText::new("Signature rule").strong());
        ui.label(&card.signature);
        ui.add_space(6.0);
        ui.label(RichText::new("Victory Condition").strong());
        ui.label(&card.victory);
        // Ticket #84: the gate Tech it waits on.
        if let Some(gate) = session.tables.victory_gate(kind) {
            let t = session.tables.tech(gate);
            ui.label(RichText::new(format!("Waits on {}, a rung-{} Tech ({} Research): {}.", t.name, t.rung, t.cost, t.effect)).weak());
        }
        ui.add_space(10.0);
        if ui.add(egui::Button::new(RichText::new(format!("Play the {}", card.name)).size(17.0)).min_size(egui::vec2(190.0, 36.0))).clicked() {
            actions.push(Action::ChooseFaction(kind));
        }
    });
}

fn start_screen(root: &mut Ui, session: &Session, faction: FactionKind, actions: &mut Vec<Action>) {
    egui::Panel::right("start_panel").default_size(320.0).show(root, |ui| {
        ui.add_space(10.0);
        ui.label(RichText::new("Choose your starting continent").size(22.0).strong());
        ui.label(format!("You play the {}.", faction.name()));
        // Ticket #50: the three Factions not picked are played by the computer, in their own colours.
        ui.horizontal_wrapped(|ui| {
            ui.label("Played by the computer:");
            for k in FactionKind::ALL.into_iter().filter(|k| *k != faction) {
                ui.label(RichText::new(k.name()).strong().color(rgb(session.tables.faction(k).colour)));
            }
        });
        ui.label("Each computer Faction takes the uncontrolled continent with the highest Industry Level.");
        ui.add_space(10.0);
        for sid in StateId::ALL {
            let c = session.tables.state(sid);
            let text = format!("{}  (population {:.1}, Industry {}, leans {:?}, education {})", c.name, c.population, c.industry_level, c.resource_lean, c.education_level);
            if ui.add(egui::Button::new(text).min_size(egui::vec2(300.0, 32.0))).clicked() {
                actions.push(Action::NewGame(faction, sid));
            }
        }
        ui.add_space(20.0);
        ui.label(RichText::new("Antarctica has no people to govern: it is three Colony Slots, founded from a Colony Ship at Earth.").weak());
    });
    egui::CentralPanel::default().frame(egui::Frame::NONE).show(root, |ui| {
        ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
    });
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
    actions: &mut Vec<Action>,
) {
    let Some(game) = session.game.as_ref() else { return };
    // Open the Tech Tree once when the player must pick.
    let must_pick = game.research.awaiting_pick == Some(Seat(0)) && !game.available_techs().is_empty();
    if must_pick && !view.tech_prompted {
        view.show_tech = true;
        view.tech_prompted = true;
    }
    if !must_pick {
        view.tech_prompted = false;
    }
    top_bar(root, session, game, view, actions);
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
            if resp.hovered() {
                let scroll = ui.input(|i| i.smooth_scroll_delta.y);
                if scroll.abs() > 0.0 {
                    view.zoom = (view.zoom * (1.0 - scroll * 0.002)).clamp(0.45, 2.2);
                }
            }
            let click = if resp.clicked() { resp.interact_pointer_pos().filter(|p| rect.contains(*p)) } else { None };
            if let (Some(pos), Some((camera, cam_gt))) = (click, cam) {
                pick(pos, session, game, view, camera, cam_gt, globes, textures, &hotspots);
            }
        }
    });
    popups(ctx, session, game, view, actions);
}

fn top_bar(root: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, actions: &mut Vec<Action>) {
    egui::Panel::top("top_bar").show(root, |ui| {
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
            ui.label(RichText::new(format!("Materials {} ({})", left.materials, signed(inc.materials))).strong()).on_hover_text(sources(dying_earth_engine::Resource::Materials));
            // Ticket #72: the Prospectors' Fund beside their Materials.
            if game.kind(Seat(0)) == FactionKind::Prospectors {
                let s = game.seat(Seat(0));
                ui.label(RichText::new(format!("Fund {} ({}%)", s.venture_fund, (s.venture_share * 100.0).round() as u32)).strong())
                    .on_hover_text("The Venture Capital Fund: Materials banked toward the 750 your Victory Condition asks for, and the share of your Factories' and Mines' output going in each turn. Set it on the Victory panel.");
            }
            ui.separator();
            ui.label(RichText::new(format!("Fuel {} ({})", left.fuel, signed(inc.fuel))).strong()).on_hover_text(sources(dying_earth_engine::Resource::Fuel));
            ui.separator();
            ui.label(RichText::new(format!("Energy {} ({})", left.energy, signed(inc.energy))).strong()).on_hover_text(sources(dying_earth_engine::Resource::Energy));
            ui.separator();
            ui.label(RichText::new(format!("Ducats {} ({})", left.ducats, signed(inc.ducats))).strong()).on_hover_text(sources(dying_earth_engine::Resource::Ducats));
            ui.separator();
            let research = match game.research.current {
                Some(t) => format!("Research {} / {} toward {}", game.research.progress, game.tables.tech(t).cost, game.tables.tech(t).name),
                None => format!("Research: no Tech chosen ({} waiting)", game.research.unallocated),
            };
            ui.label(research);
            // Ticket #58: the Research race, as a bar of the four Factions' contributions to the
            // Tech under research, in Faction colours and in proportion.
            research_race_bar(ui, session, game);
            ui.separator();
            // Ticket #42: the turn's Allotment and what the trading window added, shown apart.
            let bought: i64 = session.pending.iter().map(|o| if let Order::BuyInfluence { amount } = o { *amount } else { 0 }).sum();
            let influence = if bought > 0 { format!("Influence {} of {} ({} free + {} bought)", influence_left, s.allotment + bought, s.allotment, bought) } else { format!("Influence {} of {}", influence_left, s.allotment) };
            ui.label(influence).on_hover_text("The Allotment is what your places and buildings give each turn; bought Influence comes from the Trading window at 2 Ducats each.");
            ui.separator();
            // Ticket #57: the bar names the turn's month. Ticket #67 (version 0.05.5): a Turn is two months,
            // named by its first alone, so turn 2 reads March 2030.
            ui.label(RichText::new(format!("Turn {} / {}, {}", game.turn, game.tables.victory.turns, game.date_text())).strong());
            ui.separator();
            ui.label(format!("{:+.1} C, heading to {:+.1}", game.climate.temperature, game.target_temperature()));
        });
        ui.horizontal_wrapped(|ui| {
            if ui.button("Tech Tree").clicked() {
                view.show_tech = !view.show_tech;
            }
            if ui.button(if view.show_climate { "Hide Climate Panel (C)" } else { "Climate Panel (C)" }).clicked() {
                toggle_climate(view);
            }
            if ui.button("Victory").clicked() {
                view.show_victory = !view.show_victory;
            }
            if !session.spectator && ui.button("Trading").clicked() {
                view.show_trade = !view.show_trade;
            }
            // Ticket #59: a Save writes this turn start to a file. It is dead while an order is
            // pending, because a save captures a turn start and never half-entered orders. A
            // spectated game saves the same way.
            let can_save = dying_earth_engine::save::can_save_now(session.pending.len()) && session.screen == Screen::Playing;
            let save = ui.add_enabled(can_save, egui::Button::new("Save"));
            if save.on_disabled_hover_text(dying_earth_engine::save::SAVE_PENDING_HOVER).on_hover_text("Write this turn start to a file. Load it again from the title screen.").clicked() {
                actions.push(Action::Save);
            }
            if let Some((text, _)) = &session.save_notice {
                ui.label(RichText::new(text).strong().color(Color32::from_rgb(140, 210, 150)));
            }
            let swap_text = match view.view {
                View::Solar => format!("To {} (Tab)", game.tables.body(view.last_surface).name),
                View::Surface(_) => "Solar System Map (Tab)".to_string(),
            };
            if ui.button(swap_text).clicked() {
                view.swap();
            }
            if matches!(view.view, View::Surface(_)) && ui.button("Back (Esc)").clicked() {
                view.view = View::Solar;
                view.selection = Selection::None;
            }
            if session.screen == Screen::Playing {
                let must_pick = !session.spectator && game.research.awaiting_pick == Some(Seat(0)) && !game.available_techs().is_empty();
                let button = egui::Button::new(RichText::new("End Turn").strong().size(16.0)).fill(Color32::from_rgb(120, 40, 30));
                if ui.add_enabled(!must_pick && view.popup == Popup::None, button).on_disabled_hover_text("Pick a Tech first").clicked() {
                    let (_, influence_left) = game.remaining(Seat(0), &session.pending);
                    if !session.spectator && influence_left > 0 && game.seat(Seat(0)).allotment > 0 {
                        view.popup = Popup::ConfirmEndTurn;
                    } else {
                        actions.push(Action::EndTurn);
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
}

/// Ticket #58: the four Factions' shares of the Tech under research, drawn as one bar in Faction
/// colours. The unfilled part of the bar is what the Tech still needs.
fn research_race_bar(ui: &mut Ui, session: &Session, game: &Game) {
    let Some(tech) = game.research.current else { return };
    let cost = game.tables.tech(tech).cost.max(1);
    let c = game.research.contributions;
    let (rect, _) = ui.allocate_exact_size(egui::vec2(150.0, 14.0), egui::Sense::hover());
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
    painter.rect_stroke(rect, 3.0, egui::Stroke::new(1.0, Color32::from_gray(120)), egui::StrokeKind::Inside);
    let shares: Vec<String> = Seat::ALL.into_iter().map(|s| format!("{} {}", game.seat_name(s), c[s.index()])).collect();
    ui.interact(rect, ui.id().with("race"), egui::Sense::hover())
        .on_hover_text(format!("The Research race for {}: {}. {} of {}.", game.tables.tech(tech).name, shares.join(", "), game.research.progress, cost));
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
                let head = project(pos + Vec3::Y * (geo::solar_radius(body) + 0.05));
                if let Some(p) = head {
                    let name = game.tables.body(body).name.clone();
                    let slots = game.tables.body(body).colony_slots();
                    let filled = game.colonies.iter().filter(|c| c.body == body && !c.in_orbit).count();
                    let orbital = game.tables.body(body).orbital_slots;
                    let stations = game.colonies.iter().filter(|c| c.body == body && c.in_orbit).count();
                    let mut text = format!("{name}  {filled}/{slots} slots, {stations}/{orbital} stations");
                    // Ticket #56: Earth says when its Antarctic slots open, until they do.
                    if body == BodyId::Earth && !game.antarctica_open {
                        text.push_str(&format!("\nAntarctica: opens at {:+.1} C", game.tables.climate.antarctica_opens_at));
                    }
                    label_at(painter, p - egui::vec2(0.0, 22.0), &text, Color32::WHITE, 13.0);
                    hotspots.push(Hotspot { pos: p, radius: 40.0, hit: Hit::Enter(body) });
                    // Ticket #57: hovering a Body across the gulf says when its launch window is and
                    // what the flight costs now against what it costs then. TO BE REVISITED: these
                    // are transits FROM EARTH. Once a Faction can launch from the Moon, or home from
                    // Mars, one line for one departure point will no longer be the whole truth, and
                    // how this is presented has to be settled again.
                    let far = game.crossing_offset(BodyId::Earth, body, game.turn).is_some();
                    let hovering = view.force_hover == Some(body)
                        || painter.ctx().pointer_latest_pos().map(|q| (q - p).length() < 40.0).unwrap_or(false);
                    if far && hovering {
                        label_on_screen(painter, p + egui::vec2(0.0, 96.0), &game.window_text(body), Color32::from_rgb(255, 220, 140), 13.0);
                    }
                    // The Orbital Control flag in the holder's Faction colour.
                    if let Some(s) = game.orbital_control(body) {
                        label_at(painter, p - egui::vec2(0.0, 40.0), &format!("Orbital Control: {}", game.seat_name(s)), seat_colour(session, s), 12.0);
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
                    label_at(painter, at, &text, seat_colour(session, seat), 12.0);
                    hotspots.push(Hotspot { pos: at, radius: 14.0, hit: Hit::Select(Selection::ShipStack(body, seat)) });
                    row += 1.0;
                }
            }
            for s in &game.ships {
                if let ShipAt::Transit { from, to, turns_left } = s.at {
                    let a = geo::solar_place(game, from);
                    let b = geo::solar_place(game, to);
                    if let Some(p) = project(a.lerp(b, 0.5) + Vec3::Y * 0.2) {
                        label_at(painter, p, &format!("{} {}: {} turn(s)", game.seat_name(s.seat), s.kind.name(), turns_left), seat_colour(session, s.seat), 12.0);
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
                        label_at(painter, p, &text, colour, 12.0);
                        // Ticket #52: the Unrest figure once it bites, red once Facilities run at half.
                        let army_line = game.tables.unrest.army_threshold;
                        if st.unrest >= army_line {
                            let hot = st.unrest >= game.tables.unrest.facility_threshold;
                            let tint = if hot { Color32::from_rgb(255, 90, 80) } else { Color32::from_rgb(255, 190, 90) };
                            label_at(painter, p - egui::vec2(0.0, 38.0), &format!("Unrest {}", game.unrest_text(sid)), tint, 13.0);
                        }
                        hotspots.push(Hotspot { pos: p, radius: 30.0, hit: Hit::Select(Selection::State(sid)) });
                        // Army shields (ticket #31): one per Faction present, grey for a neutral Standing Army.
                        let mut shields: Vec<(Option<Seat>, i64)> = Vec::new();
                        for seat in Seat::ALL {
                            let s = game.army_stack_strength(seat, Place::State(sid));
                            if s > 0 || !game.armies_of_seat_at(seat, Place::State(sid)).is_empty() {
                                shields.push((Some(seat), s));
                            }
                        }
                        let neutral: i64 = game.armies.iter().filter(|a| a.at == ArmyAt::Place(Place::State(sid)) && game.army_seat(a).is_none() && !game.army_stands_down(a)).map(|a| game.army_strength(a)).sum();
                        if neutral > 0 {
                            shields.push((None, neutral));
                        }
                        for (i, (seat, strength)) in shields.iter().enumerate() {
                            let centre = p + egui::vec2(-38.0 + 26.0 * i as f32, 36.0);
                            let fill = seat.map(|s| seat_colour(session, s)).unwrap_or(Color32::from_gray(150));
                            shield(painter, centre, fill, &strength.to_string());
                            hotspots.push(Hotspot { pos: centre, radius: 12.0, hit: Hit::Select(Selection::State(sid)) });
                        }
                    }
                    // Ticket #44: Antarctica's Colony Slots.
                    slot_labels(painter, session, game, body, &visible, hotspots);
                }
                _ => {
                    slot_labels(painter, session, game, body, &visible, hotspots);
                    // The band along the top: Ship stacks in orbit and Orbital Control. Ticket #50:
                    // four seats will not fit on one line, so each takes its own in its own colour.
                    let mut band: Vec<(String, Color32)> = Vec::new();
                    for seat in Seat::ALL {
                        let n = game.ships_at(seat, body).len();
                        if n > 0 {
                            band.push((format!("{}: {} Ship(s), strength {}", game.seat_name(seat), n, game.ship_stack_strength(seat, body)), seat_colour(session, seat)));
                        }
                    }
                    for c in game.colonies.iter().filter(|c| c.in_orbit && c.body == body) {
                        let who = c.control.director();
                        let name = who.map(|s| game.seat_name(s)).unwrap_or_else(|| "nobody's".into());
                        band.push((format!("{} ({})", game.station_name(body, c.slot), name), who.map(|s| seat_colour(session, s)).unwrap_or(Color32::LIGHT_GRAY)));
                    }
                    band.push(match game.orbital_control(body) {
                        Some(s) => (format!("Orbital Control: {}", game.seat_name(s)), seat_colour(session, s)),
                        None => ("Orbital Control: nobody".to_string(), Color32::LIGHT_GRAY),
                    });
                    let rect = painter.clip_rect();
                    let x = rect.center().x - 120.0;
                    label_at(painter, Pos2::new(x, rect.min.y + 50.0), &format!("In orbit around {}", game.tables.body(body).name), Color32::WHITE, 13.0);
                    for (i, (text, colour)) in band.iter().enumerate() {
                        label_at(painter, Pos2::new(x, rect.min.y + 70.0 + 18.0 * i as f32), text, *colour, 12.0);
                    }
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
/// Colony Slot labels on a Body's surface: Antarctica's on Earth since ticket #44.
fn slot_labels(painter: &egui::Painter, session: &Session, game: &Game, body: BodyId, visible: &dyn Fn(Vec3) -> Option<Pos2>, hotspots: &mut Vec<Hotspot>) {
            for slot in 0..game.tables.body(body).colony_slots() {
                let (lon, lat) = geo::slot_lonlat(game.tables.body(body), slot);
        let name = &game.tables.body(body).slots[slot as usize].name;
                let Some(p) = visible(geo::local_from_lonlat(lon, lat) * 1.03) else { continue };
                let (text, colour, hit) = match game.colony_at(body, slot) {
                    Some(c) => {
                        let mods: Vec<String> = c.modules.iter().map(|m| format!("{}{}", m.kind.name(), if m.online { "" } else { " (offline)" })).collect();
                        let army = game.armies.iter().filter(|a| a.at == ArmyAt::Place(Place::Colony(c.id))).count();
                        let owner = c.control.director().map(|s| game.seat_name(s)).unwrap_or_default();
                        (
                            format!("{}: {}\n{} Colonists\n{}{}", name, owner, c.colonists, mods.join(", "), if army > 0 { format!("\nArmies: {army}") } else { String::new() }),
                            c.control.director().map(|s| seat_colour(session, s)).unwrap_or(Color32::LIGHT_GRAY),
                            Hit::Select(Selection::Colony(c.id)),
                        )
                    }
                    // Ticket #56: Antarctica's slots lie under the ice until the world is warm enough.
                    None if body == BodyId::Earth && !game.antarctica_open => (
                        format!("{name}: under the ice\nopens at {:+.1} C", game.tables.climate.antarctica_opens_at),
                        Color32::from_rgb(150, 195, 235),
                        Hit::Select(Selection::Slot(body, slot)),
                    ),
                    None => (format!("{name}: empty"), Color32::LIGHT_GRAY, Hit::Select(Selection::Slot(body, slot))),
                };
                label_at(painter, p + egui::vec2(0.0, 24.0), &text, colour, 12.0);
                // Ticket #57: every slot carries its own four yields under its name, filled or free;
                // a free slot's figures are what a Colony founded there would get. TO BE REVISITED
                // WHEN BOARD LENSES ARRIVE: this is on the map always, and once the player can turn
                // a yield lens on and off it should live there instead of over every label at once.
                // `label_at` centres its block on the point, so the figures clear half the name
                // block above them and half their own line.
                let lines = text.lines().count() as f32;
                label_at(
                    painter,
                    p + egui::vec2(0.0, 24.0 + lines * 7.0 + 8.0),
                    &game.slot_yields(body, slot).text(),
                    Color32::from_gray(170),
                    11.0,
                );
                hotspots.push(Hotspot { pos: p, radius: 22.0, hit });
            }
}

/// Ticket #46: the stations over the Body on screen, and the orbital slots still free.
fn stations_panel(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, actions: &mut Vec<Action>) {
    let View::Surface(body) = view.view else { return };
    let card = game.tables.body(body);
    if card.orbital_slots == 0 {
        return;
    }
    ui.separator();
    let count = game.colonies.iter().filter(|c| c.in_orbit && c.body == body).count();
    ui.label(RichText::new(format!("In orbit: {} of {} station slots", count, card.orbital_slots)).strong());
    for c in game.colonies.iter().filter(|c| c.in_orbit && c.body == body) {
        let owner = match c.control {
            Control::Neutral => "nobody's".to_string(),
            Control::Controlled(s) => game.seat_name(s),
            Control::Occupied { occupier, .. } => format!("occupied by the {}", game.seat_name(occupier)),
        };
        let mods: Vec<&str> = c.modules.iter().map(|m| m.kind.name()).collect();
        let text = format!("{}: {}, {} Colonists, {}", game.station_name(body, c.slot), owner, c.colonists, if mods.is_empty() { "a bare core module".to_string() } else { mods.join(", ") });
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
                    // Ticket #44: a Colony Slot in Antarctica first, else the Nation State under the click.
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
                (View::Surface(BodyId::Earth), false) => "Click a Nation State for its card and orders. Drag to turn, wheel to zoom.",
                (View::Surface(BodyId::Earth), true) => "Click a Nation State for its card. Drag to turn, wheel to zoom.",
                (View::Surface(_), false) => "Click a Colony Slot or Colony for its card and orders.",
                (View::Surface(_), true) => "Click a Colony Slot or Colony for its card.",
            });
            if let Some(e) = &session.last_error {
                ui.colored_label(Color32::LIGHT_RED, e);
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

/// The roster (#23): every Ship stack, Army, Colony and Nation State the player directs, each row a
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
        ui.label(RichText::new("Your roster").size(18.0).strong());
        ui.label(RichText::new("Click a row to select it and go there. \"no order\" marks what still waits.").weak());
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
    }
}

/// One seat's Ships, Armies, Colonies and stations and Nation States. `marks` writes the "no order"
/// mark, which only a seat that gives orders can owe; where it is off, four Factions share the
/// panel, so each group is named on its own rows rather than over a heading, and an empty group is
/// left out instead of saying so.
fn roster_of(ui: &mut Ui, session: &Session, game: &Game, seat: Seat, marks: bool, jump: &mut Option<(View, Selection)>) {
    let pending = &session.pending;
    let heading = |ui: &mut Ui, text: &str| {
        if marks {
            ui.label(RichText::new(text).strong());
        }
    };
    let tag = |text: &str| if marks { String::new() } else { format!("{text}: ") };
    // Ships, one row per stack, then those in transit.
    heading(ui, "Ships");
    let mut any_ship = false;
    for body in BodyId::ALL {
        let ships: Vec<&Ship> = game.ships.iter().filter(|s| s.seat == seat && s.at == ShipAt::Body(body)).collect();
        if ships.is_empty() {
            continue;
        }
        any_ship = true;
        let ordered = ships.iter().all(|s| {
            pending.iter().any(|o| matches!(o, Order::Transit { ship, .. } | Order::Load { ship, .. } | Order::Unload { ship, .. } | Order::Repair { unit: UnitRef::Ship(ship), .. } if *ship == s.id))
        }) || pending.iter().any(|o| matches!(o, Order::ShipStance { body: b, .. } if *b == body));
        let mut kinds: Vec<String> = ships.iter().map(|s| s.kind.name().to_string()).collect();
        kinds.sort();
        kinds.dedup();
        let cargo: u32 = ships.iter().map(|s| s.colonists).sum();
        let armies = ships.iter().filter(|s| s.army.is_some()).count();
        let mut text = format!("{}{} at {}: {} (strength {})", tag("Ships"), ships.len(), game.tables.body(body).name, kinds.join(", "), game.ship_stack_strength(seat, body));
        if cargo > 0 {
            text.push_str(&format!(", {cargo} Colonists aboard"));
        }
        if armies > 0 {
            text.push_str(&format!(", {armies} Army aboard"));
        }
        // Ticket #87: the tanks, and a stack that cannot leave.
        let fuel: i64 = ships.iter().map(|s| s.fuel).sum();
        let tanks: i64 = ships.iter().map(|s| game.tables.unit(s.kind).tank).sum();
        text.push_str(&format!(", tank {fuel}/{tanks}"));
        if ships.iter().all(|s| game.stranded(s.id)) {
            text.push_str(" - STRANDED: no leg affordable and no station of yours here");
        }
        if marks && !ordered {
            text.push_str("  - no order");
        }
        if ui.button(text).clicked() {
            *jump = Some((View::Solar, Selection::ShipStack(body, seat)));
        }
    }
    for s in game.ships.iter().filter(|s| s.seat == seat) {
        if let ShipAt::Transit { to, turns_left, .. } = s.at {
            any_ship = true;
            let text = format!("{}{} in transit to {}, {} turn(s) left", tag("Ship"), s.kind.name(), game.tables.body(to).name, turns_left);
            if ui.button(text).clicked() {
                *jump = Some((View::Solar, Selection::None));
            }
        }
    }
    if !any_ship && marks {
        ui.label(RichText::new("  none; a Shipyard on a station or Colony builds them").weak());
    }
    // Armies.
    heading(ui, "Armies");
    let mut any_army = false;
    for a in game.armies.iter().filter(|a| game.army_seat(a) == Some(seat) && !game.army_stands_down(a)) {
        any_army = true;
        let ordered = pending.iter().any(|o| match o {
            Order::MoveArmy { army, .. } | Order::Repair { unit: UnitRef::Army(army), .. } | Order::Load { army: Some(army), .. } => *army == a.id,
            Order::ArmyStance { place, .. } => a.at == ArmyAt::Place(*place),
            _ => false,
        });
        let (where_, target) = match a.at {
            ArmyAt::Place(Place::State(s)) => (game.tables.state(s).name.clone(), Some((View::Surface(BodyId::Earth), Selection::State(s)))),
            ArmyAt::Place(Place::Colony(c)) => (game.place_name(Place::Colony(c)), game.colony(c).map(|col| (View::Surface(col.body), Selection::Colony(c)))),
            ArmyAt::Aboard(ship) => (format!("aboard {ship}"), game.ship(ship).map(|s| match s.at { ShipAt::Body(b) => (View::Solar, Selection::ShipStack(b, seat)), _ => (View::Solar, Selection::None) })),
        };
        let mut text = format!("{}{} at {}: strength {}, damage {}", tag("Army"), if a.standing { "Standing Army" } else { "Army" }, where_, game.army_strength(a), a.damage);
        if marks && !ordered && !matches!(a.at, ArmyAt::Aboard(_)) {
            text.push_str("  - no order");
        }
        if ui.button(text).clicked() {
            *jump = target;
        }
    }
    if !any_army && marks {
        ui.label(RichText::new("  none").weak());
    }
    // Colonies.
    heading(ui, "Colonies and stations");
    let mut any_colony = false;
    for c in game.colonies.iter().filter(|c| c.control.director() == Some(seat)) {
        any_colony = true;
        let building = c.queue.len();
        let text = format!("{}{}: {} Colonists, {} Modules{}", tag("Colony"), game.place_name(Place::Colony(c.id)), c.colonists, c.modules.len(), if building > 0 { format!(", {building} building") } else { String::new() });
        if ui.button(text).clicked() {
            *jump = Some((View::Surface(c.body), Selection::Colony(c.id)));
        }
    }
    if !any_colony && marks {
        ui.label(RichText::new("  none; a Colony Ship founds one").weak());
    }
    // Nation States.
    heading(ui, "Nation States");
    let states = game.directed_states(seat);
    if states.is_empty() && marks {
        ui.label(RichText::new("  none").weak());
    }
    for sid in states {
        let st = game.state(sid);
        let building = st.queue.len();
        let text = format!("{}{}: {} Facilities, {} free slot(s){}", tag("State"), game.tables.state(sid).name, st.facilities.len(), game.free_slots(sid), if building > 0 { format!(", {building} building") } else { String::new() });
        if ui.button(text).clicked() {
            *jump = Some((View::Surface(BodyId::Earth), Selection::State(sid)));
        }
    }
}


fn order_text(game: &Game, o: &Order) -> String {
    match o {
        Order::BuildFacility { state, kind } => format!("Build {} in {}", kind.name(), game.tables.state(*state).name),
        Order::RaiseIndustry { state } => format!("Raise Industry Level in {}", game.tables.state(*state).name),
        Order::BuildModule { colony, kind } => format!("Build {} at {}", kind.name(), game.place_name(Place::Colony(*colony))),
        Order::BuildShip { site, kind } => format!("Build {} at {}", kind.name(), game.place_name(*site)),
        Order::BuildArmy { place } => format!("Build Army at {}", game.place_name(*place)),
        Order::Repair { unit, points } => format!("Repair {} point(s) on {}", points, match unit { UnitRef::Ship(s) => s.to_string(), UnitRef::Army(a) => a.to_string() }),
        Order::Transit { ship, to, slot } => match slot {
            Some(n) => format!("Send {} to {}, into Orbital Slot {}", ship, game.tables.body(*to).name, n),
            None => format!("Send {} to {}", ship, game.tables.body(*to).name),
        },
        Order::Refuel { ship } => format!("Refuel {} ({} Fuel from the Stockpile)", ship, game.refuel_amount(Seat(0), *ship)),
        Order::ShipStance { body, stance } => format!("Ships at {}: {}", game.tables.body(*body).name, stance.name()),
        Order::ArmyStance { place, stance } => format!("Armies at {}: {}", game.place_name(*place), stance.name()),
        Order::MoveArmy { army, to } => format!("{} to {}", army, game.tables.state(*to).name),
        Order::Load { ship, colonists, army, .. } => format!("Load {} onto {}", if *colonists > 0 { format!("{colonists} Colonists") } else { format!("{}", army.unwrap_or(ArmyId(0))) }, ship),
        Order::Unload { ship, colonists, army, into } => match into {
            UnloadTarget::Slot(b, s) => format!("Found a Colony at {} on {} from {}", game.tables.body(*b).slots[*s as usize].name, game.tables.body(*b).name, ship),
            UnloadTarget::Colony(c) => format!("Unload {} from {} into {}", if *colonists > 0 { format!("{colonists} Colonists") } else if *army { "the Army".into() } else { "nothing".into() }, ship, game.place_name(Place::Colony(*c))),
        },
        Order::Influence { target, amount } => format!("{} Influence on {}", amount, game.place_name(*target)),
        Order::BuyInfluence { amount } => format!("Buy {} Influence with Ducats", amount),
        Order::RepairWithDucats { unit, points } => format!("Repair {} point(s) on {} with Ducats", points, match unit { UnitRef::Ship(s) => s.to_string(), UnitRef::Army(a) => a.to_string() }),
        Order::Buy { resource, amount } => format!("Buy {} {} for {} Ducats", amount, resource.name(), game.order_cost(Seat(0), o).ducats),
        Order::Sell { resource, amount } => format!("Sell {} {} for {} Ducats", amount, resource.name(), -game.order_cost(Seat(0), o).ducats),
        Order::BuildFacilityWithDucats { state, kind } => format!("Build {} in {} for Ducats", kind.name(), game.tables.state(*state).name),
        Order::BuildModuleWithDucats { colony, kind } => format!("Build {} at {} for Ducats", kind.name(), game.place_name(Place::Colony(*colony))),
        Order::BuildStation { body, slot } => format!("Build {} over {}", game.station_name(*body, *slot), game.tables.body(*body).name),
        Order::BuildArchive { colony } => format!("Build the Archive at {}", game.place_name(Place::Colony(*colony))),
        Order::SetArchiveFunding { on: true } => "Pay your Labs into the Archive fund from the next Income".to_string(),
        Order::SetArchiveFunding { on: false } => "Pay your Labs into the shared Tech from the next Income".to_string(),
        // Ticket #73.
        Order::BuildEmigrants { state, n } => format!("Muster {n} Emigrants in {}", game.tables.state(*state).name),
        Order::SendToAntarctica { state, n, into } => format!(
            "Send {n} Emigrants from {} to {} by sea",
            game.tables.state(*state).name,
            match into {
                UnloadTarget::Slot(_, slot) => game.tables.body(BodyId::Earth).slots[*slot as usize].name.clone(),
                UnloadTarget::Colony(c) => game.place_name(Place::Colony(*c)),
            }
        ),
        // Ticket #72.
        Order::SetVentureShare { share } => format!("Bank {share}% of Materials output in the Venture Capital Fund"),
        Order::DrawVenture { amount } => format!("Draw {amount} Materials from the Venture Capital Fund"),
        // Ticket #52.
        Order::Relief { state } => format!("Relief in {}: Unrest -1", game.tables.state(*state).name),
        Order::Resettle { state } => format!("Resettle this turn's refugees in {}", game.tables.state(*state).name),
        // Ticket #54.
        Order::Change { building, what } => format!("{} the {} at {}", what.name(), building_name(game, *building), game.place_name(building.place())),
        Order::Leapfrog { state } => format!("Leapfrog {}: its people emit 0.03 less per hundred million", game.tables.state(*state).name),
        Order::StripPermit { state } => format!("Strip Permit in {}: three turns of double output", game.tables.state(*state).name),
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

/// Ticket #54: the Mothball / Restart / Decommission row under one standing building.
/// Ticket #56: the two rows of a Nation State's build slots, Coastal and Inland, each slot named by
/// what stands or builds in it, or "free". The coastal slots the sea has taken stand at the end of
/// the coastal row, struck through in the sea's own blue.
fn slot_rows(ui: &mut Ui, game: &Game, sid: StateId) {
    let st = game.state(sid);
    let occupants = |coastal: bool| -> Vec<String> {
        let mut v: Vec<String> = st
            .facilities
            .iter()
            .filter(|f| f.coastal == coastal && game.takes_slot(f.kind))
            .map(|f| f.kind.name().to_string())
            .collect();
        v.extend(
            st.queue
                .iter()
                .filter(|b| b.coastal == coastal && matches!(b.item, BuildItem::Facility(k) if game.takes_slot(k)))
                .map(|b| format!("{} building", b.item.name())),
        );
        v
    };
    for (coastal, label, total) in [(true, "Coastal", game.coastal_slots(sid)), (false, "Inland", game.inland_slots(sid))] {
        let mut cells = occupants(coastal);
        while (cells.len() as u32) < total {
            cells.push("free".to_string());
        }
        let lost = if coastal { st.lost_slots } else { 0 };
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new(format!("{label}:")).strong());
            for c in &cells {
                let weak = c == "free";
                ui.label(if weak { RichText::new(c).weak() } else { RichText::new(c) });
            }
            for i in 0..lost {
                let what = match st.drowned.get(i as usize) {
                    Some(k) => format!("{}, lost to the sea", k.name()),
                    None => "lost to the sea".to_string(),
                };
                ui.label(RichText::new(what).color(Color32::from_rgb(110, 160, 220)).strikethrough());
            }
            if cells.is_empty() && lost == 0 {
                ui.label(RichText::new("none").weak());
            }
        });
    }
}

fn change_row(ui: &mut Ui, game: &Game, pending: &[Order], b: BuildingRef, mothballed: bool, change: Option<PendingChange>, actions: &mut Vec<Action>) {
    if let Some(c) = change {
        ui.label(RichText::new(format!("    {} ordered, lands at turn {}'s Resolution", c.what.name(), c.due_turn)).weak());
        return;
    }
    ui.horizontal(|ui| {
        ui.add_space(16.0);
        let wanted = if mothballed { BuildingChange::Restart } else { BuildingChange::Mothball };
        for what in [wanted, BuildingChange::Decommission] {
            cost_button(ui, game, pending, Order::Change { building: b, what }, what.name(), actions);
        }
    });
}

fn cost_button(ui: &mut Ui, game: &Game, pending: &[Order], order: Order, label: &str, actions: &mut Vec<Action>) {
    cost_button_with_hover(ui, game, pending, order, label, None, actions);
}

/// A build button: cost in the label, and on hover what the building would make each turn (#22).
fn cost_button_with_hover(ui: &mut Ui, game: &Game, pending: &[Order], order: Order, label: &str, hover: Option<String>, actions: &mut Vec<Action>) {
    let cost = game.order_cost(Seat(0), &order);
    let check = game.check_order(Seat(0), pending, &order);
    let text = format!("{} ({})", label, cost.text());
    let button = egui::Button::new(text);
    let mut resp = ui.add_enabled(check.is_ok(), button);
    if let Some(h) = &hover {
        resp = resp.on_hover_text(format!("Once it stands: {h}")).on_disabled_hover_text(format!("Once it stands: {h}"));
    }
    if let Err(e) = &check {
        resp.clone().on_disabled_hover_text(&e.0);
    }
    if resp.clicked() {
        actions.push(Action::Place(order));
    }
}

fn stance_row(ui: &mut Ui, game: &Game, pending: &[Order], current: Stance, make: impl Fn(Stance) -> Order, ships: bool, actions: &mut Vec<Action>) {
    ui.horizontal(|ui| {
        ui.label("Stance:");
        for st in [Stance::Attack, Stance::Hold, Stance::Intercept, Stance::Evade] {
            if st == Stance::Intercept && !ships {
                continue;
            }
            let pending_stance = pending.iter().rev().find_map(|o| match (o, &make(st)) {
                (Order::ShipStance { body, stance }, Order::ShipStance { body: b2, .. }) if body == b2 => Some(*stance),
                (Order::ArmyStance { place, stance }, Order::ArmyStance { place: p2, .. }) if place == p2 => Some(*stance),
                _ => None,
            });
            let shown = pending_stance.unwrap_or(current);
            if ui.selectable_label(shown == st, st.name()).clicked() && shown != st {
                let order = make(st);
                if game.check_order(Seat(0), pending, &order).is_ok() {
                    actions.push(Action::Place(order));
                }
            }
        }
    });
}

fn influence_row(ui: &mut Ui, game: &Game, session: &Session, view: &mut ViewState, target: Place, actions: &mut Vec<Action>) {
    // Ticket #64: a spectator reads every Faction's Standing here and spends nothing.
    if session.spectator {
        standings_row(ui, game, session, target);
        ui.label(format!(
            "Threshold {}; a place already held changes hands only at the holder's Standing plus the challenge margin of {}.",
            game.influence_threshold(target),
            game.tables.influence.challenge_margin
        ));
        match game.place_control(target).controller() {
            Some(c) => ui.label(RichText::new(format!("Held by the {}.", game.seat_name(c))).weak()),
            None => ui.label(RichText::new("Neutral. The first Standing at the threshold takes it; Standings decay 2 a turn when nothing is spent.").weak()),
        };
        return;
    }
    ui.horizontal(|ui| {
        ui.label("Influence:");
        ui.add(egui::DragValue::new(&mut view.influence_amount).range(1..=100));
        let order = Order::Influence { target, amount: view.influence_amount };
        let ok = game.check_order(Seat(0), &session.pending, &order);
        if ui.add_enabled(ok.is_ok(), egui::Button::new("Spend")).clicked() {
            actions.push(Action::Place(order));
        }
        if let Err(e) = ok {
            ui.label(RichText::new(e.0).weak());
        }
    });
    // Ticket #42: buying Influence lives in the Trading window now.
    if ui.small_button("Buy more Influence in the Trading window").clicked() {
        view.show_trade = true;
    }
    // Ticket #53: the threshold shown is the player's own, since Blame raises it seat by seat.
    let threshold = game.influence_threshold_for(Seat(0), target);
    standings_row(ui, game, session, target);
    ui.label(format!("Threshold {}; a place already held changes hands only at the holder's Standing plus the challenge margin of {}.", threshold, game.tables.influence.challenge_margin));
    // Ticket #53: on every Nation State the player does not hold, what its Blame is costing it here.
    let blame_mult = game.blame_threshold_multiplier_on(Seat(0), target);
    if blame_mult > 1.0 {
        ui.label(
            RichText::new(format!(
                "Blame: your threshold here is {}, not {} (share {:.2}, x{:.2}). Emit less, or take back what you emit, and it comes down.",
                threshold,
                game.influence_threshold(target),
                game.blame_share(Seat(0)),
                blame_mult
            ))
            .color(Color32::from_rgb(255, 170, 120)),
        );
    }
    match game.place_control(target).controller() {
        Some(c) => {
            // Ticket #60: the engine's own figure, which the Resolution and the AI read too. It
            // was `threshold.max(standing + 1)` here, which ignored the challenge margin ticket #41
            // put on a held place, so the card printed a figure the Resolution would not honour.
            let need = game.influence_needed_for(Seat(0), target);
            if c == Seat(0) {
                ui.label(RichText::new(format!("Yours. A rival takes it with a standing above yours and at least the threshold: {need} now. Spending here raises your standing; it decays 1 a turn.")).weak());
            } else {
                ui.label(RichText::new(format!("Theirs. You take it with a standing above theirs and at least the threshold: {need} now.")).weak());
            }
        }
        None => {
            ui.label(RichText::new(format!("Neutral. The first standing at the threshold ({threshold}) takes it; standings decay 2 a turn when nothing is spent.")).weak());
        }
    }
}

/// Ticket #50: four seats, so the Standings are chips in Faction colours, and only where there is
/// a Standing to show. Ticket #64: the spectator's cards carry the same row.
fn standings_row(ui: &mut Ui, game: &Game, session: &Session, target: Place) {
    ui.horizontal_wrapped(|ui| {
        ui.label("Standings:");
        let mut any = false;
        for s in Seat::ALL {
            let v = game.seat(s).influence.get(&target).copied().unwrap_or(0);
            if v <= 0 {
                continue;
            }
            any = true;
            ui.label(RichText::new(format!(" {} {} ", game.seat_name(s), v)).color(Color32::BLACK).background_color(seat_colour(session, s)));
        }
        if !any {
            ui.label(RichText::new("nobody has any yet").weak());
        }
    });
}

fn state_panel(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, sid: StateId, actions: &mut Vec<Action>) {
    let card = game.tables.state(sid);
    let st = game.state(sid);
    ui.label(RichText::new(&card.name).size(22.0).strong());
    let owner = match st.control {
        Control::Neutral => "Neutral".to_string(),
        Control::Controlled(s) => format!("Controlled by the {}", game.seat_name(s)),
        Control::Occupied { occupier, turns, .. } => format!("Occupied by the {} (turn {} of {})", game.seat_name(occupier), turns, game.tables.influence.occupation_turns),
    };
    ui.label(owner);
    let mult = st.control.director().map(|s| game.tables.faction(game.kind(s)).emissions_multiplier).unwrap_or(1.0);
    let industry_em = card.baseline_emissions * st.industry_level as f64 * mult;
    let fac_em: f64 = st.facilities.iter().filter(|f| f.working()).map(|f| game.tables.facility(f.kind).emissions * mult).sum();
    ui.label(format!("Population {:.1} (hundreds of millions), Industry Level {}, leans {:?}", st.population, st.industry_level, card.resource_lean));
    ui.label(format!("Influence value {}: what it adds to its controller's Allotment each turn (+1 per Industry Level raised)", game.state_influence_value(sid)));
    ui.label(format!("GDP {}: its economy pays its controller {} Ducats a turn (GDP x Industry Level / 10); a Bank here would add {}", card.gdp, game.state_ducats(sid), (game.tables.facility(FacilityKind::Bank).produces.as_ref().map(|p| p.amount).unwrap_or(0) * card.gdp) / 10));
    ui.label(format!("Emissions this turn: industry {:.1}, Facilities {:.1}, people {:.1}", industry_em, fac_em, game.population_coefficient(sid) * st.population * mult));
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
                game.population_coefficient(sid),
                c.population_emissions_base,
                c.population_emissions_per_level,
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
    ui.label(format!("Build slots: {} used of {} ({} free); Education Level {}", game.slots_used(sid), game.build_slots(sid), game.free_slots(sid), card.education_level));
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
        ui.colored_label(colour, format!("Unrest {}: {}", game.unrest_text(sid), game.unrest_note(sid)));
        // Ticket #75: a rival's Standing within two steps of the player's own, at the top of the
        // card where it is seen, not in the Influence section below the fold.
        if game.place_control(Place::State(sid)).controller() == Some(Seat(0)) {
            let target = Place::State(sid);
            let mine = game.seat(Seat(0)).influence.get(&target).copied().unwrap_or(0);
            let step = game.tables.ai.thresholds.influence_step;
            let pressing = Seat(0).others().iter().map(|s| (*s, game.seat(*s).influence.get(&target).copied().unwrap_or(0))).max_by_key(|(_, n)| *n).filter(|(_, n)| *n > 0 && *n + 2 * step >= mine);
            if let Some((rival, standing)) = pressing {
                ui.label(
                    RichText::new(format!(
                        "The {} stand at {} here against your {}: they take it at {}. Spend here to stay ahead.",
                        game.seat_name(rival),
                        standing,
                        mine,
                        mine + game.tables.influence.challenge_margin
                    ))
                    .color(Color32::from_rgb(255, 160, 60)),
                );
            }
        }
        // Ticket #73: Emigrants waiting here for a lift or the sea.
        if st.emigrants > 0 {
            ui.label(format!("Emigrants waiting: {}", st.emigrants)).on_hover_text("Mustered here and not yet lifted or sent: a working Launch Site lifts them onto a Ship, or, once the ice is open, the sea takes them to Antarctica.");
        }
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
    if st.lost_slots > 0 {
        ui.colored_label(Color32::LIGHT_BLUE, format!("{} coastal slot(s) lost to the sea", st.lost_slots));
    }
    // Ticket #56: the two rows of slots, with what stands in each and what the sea has taken.
    slot_rows(ui, game, sid);
    ui.label(RichText::new(format!("Facilities ({} of {} slots free)", game.free_slots(sid), game.build_slots(sid))).strong());
    let director = st.control.director();
    // Ticket #64: a spectator reads every card and orders on none of them.
    let mine = !session.spectator && st.control.director() == Some(Seat(0));
    for (i, f) in st.facilities.iter().enumerate() {
        // Ticket #54: a mothballed Facility says so rather than showing figures it is not making.
        let figures = if f.mothballed {
            "mothballed: making nothing, paying no upkeep, emitting nothing, keeping its slot".to_string()
        } else {
            // Ticket #69: a Lab in a state nobody holds, or under Occupation, works for the world.
            let world_lab = f.kind == FacilityKind::ResearchLab && f.working() && !f.offline_until_resolution && matches!(game.state(sid).control, Control::Neutral | Control::Occupied { .. });
            match director {
                Some(d) if world_lab => format!("{} (the Lab works for the world: {} Research a turn to the Tech under research)", game.facility_yield(d, sid, f.kind).text(), game.world_lab_yield(sid) / 2),
                Some(d) => game.facility_yield(d, sid, f.kind).text(),
                None if world_lab => format!("in no one's hands: {} Research a turn to the Tech under research", game.world_lab_yield(sid) / 2),
                None => "idle, nobody directs this state".to_string(),
            }
        };
        let colour = if f.mothballed { Color32::from_rgb(170, 170, 190) } else { ui.visuals().text_color() };
        ui.colored_label(
            colour,
            format!(
                "  {} ({}): {}{}",
                f.kind.name(),
                if f.coastal { "coastal" } else { "inland" },
                figures,
                if f.online || f.mothballed { "" } else { " (offline, making nothing)" }
            ),
        );
        if mine {
            change_row(ui, game, &session.pending, BuildingRef::Facility(sid, i), f.mothballed, f.change, actions);
        }
    }
    for b in &st.queue {
        ui.label(format!("  {} under construction, ready turn {}", b.item.name(), b.due_turn + 1));
    }
    ui.label(RichText::new("Armies").strong());
    let armies: Vec<&Army> = game.armies.iter().filter(|a| a.at == ArmyAt::Place(Place::State(sid))).collect();
    for a in &armies {
        let who = match game.army_seat(a) {
            Some(s) => game.seat_name(s),
            None => "neutral".to_string(),
        };
        ui.label(format!("  {} {} strength {}, damage {}/{}", who, if a.standing { "Standing Army" } else { "Army" }, game.army_strength(a), a.damage, game.tables.unit(UnitKind::Army).hit_points));
    }
    ui.separator();
    if mine {
        ui.label(RichText::new("Build (hover a button for what it makes)").strong());
        // Ticket #54: the Custodians' Scrubber and Leapfrog, and the Prospectors' Strip Permit.
        if game.kind(Seat(0)) == FactionKind::Custodians {
            ui.label(RichText::new(format!("Scrubbers {} of {}", game.scrubbers_committed(sid), game.scrubber_cap(sid))).strong());
            let sink = game.tables.facility(FacilityKind::Scrubber).sink_per_turn;
            ui.horizontal(|ui| {
                cost_button_with_hover(
                    ui,
                    game,
                    &session.pending,
                    Order::BuildFacility { state: sid, kind: FacilityKind::Scrubber },
                    "Scrubber",
                    Some(format!("+{sink:.1} ppm on the Natural Sink and 1 off this state's Unrest a turn, no build slot, 4 Energy upkeep")),
                    actions,
                );
                cost_button(ui, game, &session.pending, Order::BuildFacilityWithDucats { state: sid, kind: FacilityKind::Scrubber }, "or", actions);
            });
            ui.label(RichText::new("A Scrubber takes no build slot and is destroyed if this state changes hands.").weak());
            ui.horizontal(|ui| {
                cost_button(ui, game, &session.pending, Order::Leapfrog { state: sid }, "Leapfrog", actions);
                ui.label(RichText::new(format!("lowers its people to {:.2} per hundred million, for good", (game.population_coefficient(sid) - game.tables.climate.population_emissions_per_level).max(game.tables.climate.population_emissions_base))).weak());
            });
        }
        if game.kind(Seat(0)) == FactionKind::Prospectors && !st.strip_permit_used {
            let t = &game.tables.strip_permit;
            ui.horizontal(|ui| {
                cost_button(ui, game, &session.pending, Order::StripPermit { state: sid }, "Strip Permit", actions);
                ui.label(RichText::new(format!("{} turns of double output here, then +{:.1} Baseline Emissions and +{} Unrest, for good", t.turns, t.baseline_rise, Game::unrest_figure(t.unrest))).weak());
            });
        }
        for fk in FacilityKind::ALL {
            // Ticket #54: the Scrubber has its own button, with the state's cap on it.
            if fk == FacilityKind::Scrubber {
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
        if game.has_tech(TechId::CoastalEngineering) {
            ui.label(
                RichText::new("A Sea Wall takes no build slot, as a Scrubber does, and takes this state's next Sea Level threshold whole; it is destroyed doing it.")
                    .weak(),
            );
        }
        cost_button(ui, game, &session.pending, Order::RaiseIndustry { state: sid }, "Raise Industry Level", actions);
        ui.label(RichText::new("Raising the Industry Level adds an inland slot, which the sea never reaches.").weak());
        cost_button(ui, game, &session.pending, Order::BuildArmy { place: Place::State(sid) }, "Build Army", actions);
        // Ticket #73: muster Emigrants here, and send them to Antarctica by sea once the ice is open.
        ui.label(RichText::new("Emigrants").strong());
        let per = game.emigrants_per_turn(Seat(0));
        cost_button_with_hover(
            ui,
            game,
            &session.pending,
            Order::BuildEmigrants { state: sid, n: per },
            &format!("Muster {per} Emigrants"),
            Some(format!(
                "{:.1} population, on the card at End Turn, and {} off this state's Unrest. A working Launch Site lifts them onto a Ship; once the ice is open the sea takes them to Antarctica.",
                game.lift_population(Seat(0), per),
                Game::unrest_figure(game.tables.emigrants.unrest_fall)
            )),
            actions,
        );
        if game.antarctica_open && st.emigrants > 0 {
            let n = st.emigrants;
            for slot in game.free_slots_on(BodyId::Earth) {
                cost_button(ui, game, &session.pending, Order::SendToAntarctica { state: sid, n, into: UnloadTarget::Slot(BodyId::Earth, slot) }, &format!("Send {n} to {} by sea", game.tables.body(BodyId::Earth).slots[slot as usize].name), actions);
            }
            for c in game.colonies.iter().filter(|c| c.body == BodyId::Earth && !c.in_orbit && c.control.director() == Some(Seat(0))) {
                cost_button(ui, game, &session.pending, Order::SendToAntarctica { state: sid, n, into: UnloadTarget::Colony(c.id) }, &format!("Send {n} to {} by sea", game.place_name(Place::Colony(c.id))), actions);
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
            RichText::new(if st.facilities.iter().any(|f| f.kind == FacilityKind::LaunchSite && f.working()) {
                "Launch Site: Colonists and Armies lift to orbit from here. Ships are built at a Shipyard on a station or Colony."
            } else {
                "No working Launch Site: nothing lifts to orbit from here."
            })
            .weak(),
        );
        let my_armies: Vec<&Army> = armies.iter().copied().filter(|a| game.army_seat(a) == Some(Seat(0)) && !game.army_stands_down(a)).collect();
        if !my_armies.is_empty() {
            ui.label(RichText::new("Army orders").strong());
            stance_row(ui, game, &session.pending, my_armies[0].stance, |s| Order::ArmyStance { place: Place::State(sid), stance: s }, false, actions);
            for a in &my_armies {
                ui.label(format!("{} (strength {}):", if a.standing { "Standing Army" } else { "Army" }, game.army_strength(a)));
                ui.horizontal_wrapped(|ui| {
                    for n in &card.neighbours {
                        let ctrl = game.state(*n).control;
                        let verb = if ctrl == Control::Controlled(Seat(0)) { "move to" } else { "attack" };
                        let def: i64 = game.defenders_at(Place::State(*n), Seat(0)).iter().filter_map(|id| game.army(*id)).map(|x| game.army_strength(x)).sum();
                        let odds = first_round_odds(game.army_strength(a), def);
                        let label = if verb == "attack" { format!("{} {} ({:.0}%)", verb, game.tables.state(*n).name, odds * 100.0) } else { format!("{} {}", verb, game.tables.state(*n).name) };
                        cost_button(ui, game, &session.pending, Order::MoveArmy { army: a.id, to: *n }, &label, actions);
                    }
                });
                if a.damage > 0 {
                    cost_button(ui, game, &session.pending, Order::Repair { unit: UnitRef::Army(a.id), points: a.damage }, "Repair fully", actions);
                    cost_button(ui, game, &session.pending, Order::RepairWithDucats { unit: UnitRef::Army(a.id), points: a.damage }, "Repair fully with Ducats", actions);
                }
            }
        }
    } else {
        ui.label(RichText::new("Influence").strong());
    }
    influence_row(ui, game, session, view, Place::State(sid), actions);
}

fn colony_panel(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, cid: ColonyId, actions: &mut Vec<Action>) {
    let Some(col) = game.colony(cid) else { return };
    ui.label(RichText::new(game.place_name(Place::Colony(cid))).size(22.0).strong());
    let owner = match col.control {
        Control::Neutral => "Nobody's".to_string(),
        Control::Controlled(s) => format!("Held by the {}", game.seat_name(s)),
        Control::Occupied { occupier, turns, .. } => format!("Occupied by the {} (turn {})", game.seat_name(occupier), turns),
    };
    ui.label(owner);
    ui.label(format!("Colonists {} of {} Habitat room", col.colonists, game.habitat_room(col)));
    // Ticket #97 (version 0.07.0): the Module cap, shown beside the Colonists that buy it, so a
    // player meets it on the card rather than as a refusal.
    let (used, cap) = (game.module_slots_used(col), game.module_slots(col));
    let line = format!("Modules {used} of {cap} ({} free, one for each Colonist)", game.tables.slots.base);
    if used >= cap {
        ui.colored_label(Color32::YELLOW, format!("{line} - no room for another until more Colonists live here"));
    } else {
        ui.label(line);
    }
    ui.label(RichText::new("Modules").strong());
    let director = col.control.director();
    let colony_mine = !session.spectator && col.control.director() == Some(Seat(0));
    let research = game.tables.archive.research;
    for (mi, m) in col.modules.iter().enumerate() {
        // Ticket #51: the Archive reads by its Research paid, not as a yield (ticket #68: one Module).
        if m.kind == ModuleKind::Archive {
            let fund = director.map(|d| game.seat(d).archive_fund).unwrap_or(0);
            let state = if fund >= research {
                let running = m.online && !col.control.is_occupied();
                format!("complete, {}", if running { "online" } else { "offline" })
            } else {
                format!("standing, {fund} of {research} Research paid")
            };
            ui.label(format!("  The Archive: {state}"));
            continue;
        }
        // Ticket #54: a mothballed Module says so, and carries the same three buttons.
        let figures = if m.mothballed {
            "mothballed: making nothing and paying no upkeep".to_string()
        } else {
            match director {
                // Ticket #82: this Module's own figure, its doubling included.
                Some(d) => game.module_yield_at(d, cid, mi).text(),
                None => "idle".to_string(),
            }
        };
        let colour = if m.mothballed { Color32::from_rgb(170, 170, 190) } else { ui.visuals().text_color() };
        ui.colored_label(colour, format!("  {}: {}{}", m.kind.name(), figures, if m.online || m.mothballed { "" } else { " (offline, making nothing)" }));
        if colony_mine {
            change_row(ui, game, &session.pending, BuildingRef::Module(cid, mi), m.mothballed, m.change, actions);
        }
    }
    // Ticket #51: the Archive on order shows before its Module does.
    if !col.modules.iter().any(|m| m.kind == ModuleKind::Archive)
        && let Some(b) = col.queue.iter().find(|b| b.item == BuildItem::Module(ModuleKind::Archive))
    {
        ui.label(format!("  The Archive: building, {} turn(s) left", (b.due_turn + 1).saturating_sub(game.turn)));
    }
    for b in col.queue.iter().filter(|b| b.item != BuildItem::Module(ModuleKind::Archive)) {
        ui.label(format!("  {} under construction, ready turn {}", b.item.name(), b.due_turn + 1));
    }
    let armies: Vec<&Army> = game.armies.iter().filter(|a| a.at == ArmyAt::Place(Place::Colony(cid))).collect();
    for a in &armies {
        let who = game.army_seat(a).map(|s| game.seat_name(s)).unwrap_or_else(|| "nobody's".into());
        ui.label(format!("  {} Army strength {}, damage {}", who, game.army_strength(a), a.damage));
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
            // Version 0.07.0: a standing declaration, read at the next Income, not a per-turn order.
            let declared = game.seat(Seat(0)).archive_funding;
            let pending_set = session.pending.iter().find_map(|o| match o {
                Order::SetArchiveFunding { on } => Some(*on),
                _ => None,
            });
            let mut on = pending_set.unwrap_or(declared);
            let flip = Order::SetArchiveFunding { on: !on };
            let may_flip = pending_set.is_some() || game.check_order(Seat(0), &session.pending, &flip).is_ok();
            let box_ = ui.add_enabled(
                may_flip,
                egui::Checkbox::new(&mut on, "Pay your Labs into the Archive fund (from the next Income, until you set it back)"),
            );
            if !may_flip {
                box_.clone().on_disabled_hover_text("The fund is at its cap; your Labs' Research goes to the shared Tech.");
            }
            if box_.changed() {
                if let Some(i) = session.pending.iter().position(|o| matches!(o, Order::SetArchiveFunding { .. })) {
                    actions.push(Action::Cancel(i));
                } else {
                    actions.push(Action::Place(Order::SetArchiveFunding { on }));
                }
            }
            ui.label(
                egui::RichText::new(if on {
                    "Your Labs pay the fund from the next Income."
                } else {
                    "Your Labs pay the shared Tech."
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
                    .on_hover_text(format!("{} turns to raise; then the fund opens to the full {research} Research", game.tables.module(ModuleKind::Archive).build_turns));
                if let Err(e) = &check {
                    resp.clone().on_disabled_hover_text(&e.0);
                }
                if resp.clicked() {
                    actions.push(Action::Place(order));
                }
            }
            ui.separator();
        }
        ui.label(RichText::new("Build (hover a button for what it makes)").strong());
        // Ticket #88: build it where you dig.
        match game.working_mines(col) {
            0 => {}
            1 => {
                ui.label(RichText::new(format!("One working Mine here: Modules cost x{} (never under half the row).", game.tables.in_situ.one_mine)).weak());
            }
            n => {
                ui.label(RichText::new(format!("{n} working Mines here: Modules cost x{} (never under half the row).", game.tables.in_situ.two_mines)).weak());
            }
        }
        for mk in ModuleKind::BUILDABLE {
            // Ticket #80: a station holds a Shipyard, Habitats and Observatories; ticket #89: and
            // Solar Arrays, which stand nowhere else.
            if col.in_orbit && !matches!(mk, ModuleKind::Shipyard | ModuleKind::Habitat | ModuleKind::Observatory | ModuleKind::SolarArray | ModuleKind::TradePost) {
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
    let my_armies: Vec<&Army> = armies.iter().copied().filter(|a| !session.spectator && game.army_seat(a) == Some(Seat(0))).collect();
    if !my_armies.is_empty() {
        stance_row(ui, game, &session.pending, my_armies[0].stance, |s| Order::ArmyStance { place: Place::Colony(cid), stance: s }, false, actions);
        for a in &my_armies {
            if a.damage > 0 {
                cost_button(ui, game, &session.pending, Order::Repair { unit: UnitRef::Army(a.id), points: a.damage }, "Repair Army fully", actions);
                cost_button(ui, game, &session.pending, Order::RepairWithDucats { unit: UnitRef::Army(a.id), points: a.damage }, "Repair Army fully with Ducats", actions);
            }
        }
    }
    influence_row(ui, game, session, view, Place::Colony(cid), actions);
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
    let card = game.tables.body(body);
    let y = game.slot_yields(body, slot);
    ui.label(format!("Yields here: Mine x{:.2}, Generator x{:.2}, Refinery x{:.2}, Habitat x{:.2}", y.mine, y.generator, y.refinery, y.habitat));
    ui.label(
        RichText::new(format!(
            "{} as a whole: Mine x{}, Generator x{}, Refinery x{}, Habitat x{}",
            card.name, card.mine_yield, card.generator_yield, card.refinery_yield, card.habitat_yield
        ))
        .weak(),
    );
    for s in game.ships.iter().filter(|s| !session.spectator && s.seat == Seat(0) && s.at == ShipAt::Body(body) && s.kind == UnitKind::ColonyShip && s.colonists > 0) {
        let order = Order::Unload { ship: s.id, colonists: s.colonists, army: s.army.is_some(), into: UnloadTarget::Slot(body, slot) };
        if ui.button(format!("Found a Colony here with the {} Colonists aboard {}", s.colonists, s.id)).clicked() {
            actions.push(Action::Place(order));
        }
    }
}

fn stack_panel(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState, body: BodyId, seat: Seat, actions: &mut Vec<Action>) {
    let ships: Vec<&Ship> = game.ships.iter().filter(|s| s.seat == seat && s.at == ShipAt::Body(body)).collect();
    ui.label(RichText::new(format!("{} Ships at {}", game.seat_name(seat), game.tables.body(body).name)).size(22.0).strong());
    for s in &ships {
        let card = game.tables.unit(s.kind);
        let mut extra = Vec::new();
        if s.colonists > 0 {
            extra.push(format!("{} Colonists", s.colonists));
        }
        if s.army.is_some() {
            extra.push("an Army".into());
        }
        ui.label(format!("  {} {}: strength {}, damage {}/{}{}", s.kind.name(), s.id.0, game.ship_strength(s), s.damage, card.hit_points, if extra.is_empty() { String::new() } else { format!(", carrying {}", extra.join(" and ")) }));
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
            for s in &ships {
                // Ticket #87: the button reads the tank against the leg.
                cost_button(ui, game, &session.pending, Order::Transit { ship: s.id, to, slot: None }, &format!("{} {} ({}/{} in the tank)", s.kind.name(), s.id.0, s.fuel, game.tables.unit(s.kind).tank), actions);
            }
        });
    }
    // Ticket #87: a Refuel button per Ship at a Body with a station of yours, and a word for a
    // Ship that is stranded.
    ui.label(RichText::new("Tanks").strong());
    for s in &ships {
        let tank = game.tables.unit(s.kind).tank;
        ui.horizontal_wrapped(|ui| {
            ui.label(format!("{} {}: {}/{} Fuel", s.kind.name(), s.id.0, s.fuel, tank));
            if game.own_station_at(Seat(0), body) {
                cost_button(ui, game, &session.pending, Order::Refuel { ship: s.id }, "Refuel from the Stockpile", actions);
            } else if game.stranded(s.id) {
                ui.colored_label(Color32::from_rgb(230, 120, 90), "stranded: no leg it can pay, and no station of yours here to refuel at; a station built in orbit here rescues it");
            } else {
                ui.label("no station of yours here to refuel at");
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
        ui.label(format!("{} {}:", s.kind.name(), s.id.0));
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
                    if !states.is_empty() {
                        let chosen = view.load_state.filter(|x| states.contains(x)).unwrap_or_else(|| *states.iter().max_by(|a, b| game.state(**a).population.partial_cmp(&game.state(**b).population).unwrap()).unwrap());
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
                            cost_button(ui, game, &session.pending, Order::Load { ship: s.id, colonists: lift, from: LoadSource::State(chosen), army: None }, &format!("Load {lift} Emigrants"), actions);
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
                if s.army.is_some() {
                    let label = if own { format!("Land the Army at {}", game.tables.body(c.body).slots[c.slot as usize].name) } else { format!("Land the Army to attack {}", game.tables.body(c.body).slots[c.slot as usize].name) };
                    cost_button(ui, game, &session.pending, Order::Unload { ship: s.id, colonists: 0, army: true, into: UnloadTarget::Colony(c.id) }, &label, actions);
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
                    cost_button(ui, game, &session.pending, Order::Unload { ship: s.id, colonists: s.colonists, army: s.army.is_some(), into: UnloadTarget::Slot(body, slot) }, &format!("Found a Colony at {}", game.tables.body(body).slots[slot as usize].name), actions);
                }
            }
        }
        if s.damage > 0 {
            cost_button(ui, game, &session.pending, Order::Repair { unit: UnitRef::Ship(s.id), points: s.damage }, "Repair fully", actions);
            cost_button(ui, game, &session.pending, Order::RepairWithDucats { unit: UnitRef::Ship(s.id), points: s.damage }, "Repair fully with Ducats", actions);
        }
    }
    if body != BodyId::Earth {
        ui.label(RichText::new("Influence on Colonies here").strong());
        for c in game.colonies.iter().filter(|c| c.body == body) {
            ui.label(game.place_name(Place::Colony(c.id)));
            influence_row(ui, game, session, view, Place::Colony(c.id), actions);
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
    ui.label("What you buy is yours at once, for this turn's orders. Ducats come from your Nation States' economies, Banks and Trade Posts.");
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
    ui.label(format!("Buildings: every build button on a Nation State or Colony card has an \"or\" beside it that buys the building outright for Ducats, at {} times its Materials cost.", game.tables.ducats.per_building_material));
    let trades: Vec<String> = session.pending.iter().filter(|o| matches!(o, Order::Buy { .. } | Order::Sell { .. } | Order::BuyInfluence { .. } | Order::BuildFacilityWithDucats { .. } | Order::BuildModuleWithDucats { .. })).map(|o| order_text(game, o)).collect();
    if !trades.is_empty() {
        ui.separator();
        ui.label(RichText::new("Trades this turn (undo them in the orders list)").strong());
        for t in trades {
            ui.label(t);
        }
    }
}

/// Ticket #41: the Tech Tree drawn as a tree. One column per branch, one row per rung, a line from
/// every Tech to each Tech that needs it, each box coloured by its state.
fn tech_tree(ui: &mut Ui, game: &Game, available: &[TechId], must_pick: bool, actions: &mut Vec<Action>) {
    const COL: f32 = 156.0;
    const ROW: f32 = 96.0;
    const BOX_W: f32 = 140.0;
    const BOX_H: f32 = 64.0;
    const HEAD: f32 = 26.0;
    let mut branches: Vec<String> = Vec::new();
    for t in TechId::ALL {
        let b = &game.tables.tech(t).branch;
        if !branches.contains(b) {
            branches.push(b.clone());
        }
    }
    let rungs = TechId::ALL.iter().map(|t| game.tables.tech(*t).rung).max().unwrap_or(1).max(1);
    // Ticket #56: a branch may hold more than one Tech on a rung (Clean Power and Coastal
    // Engineering both sit on Industry 2), so a branch's column is as many columns wide as its
    // busiest rung, and the Techs on a rung share that width between them.
    let cell: Vec<Vec<TechId>> = (0..branches.len() * rungs as usize)
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
    let span: Vec<f32> = (0..branches.len())
        .map(|b| (0..rungs as usize).map(|r| cell[r * branches.len() + b].len()).max().unwrap_or(1).max(1) as f32)
        .collect();
    let left: Vec<f32> = (0..branches.len()).map(|b| span[..b].iter().sum::<f32>() * COL).collect();
    let width: f32 = span.iter().sum::<f32>() * COL;
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, HEAD + ROW * rungs as f32), egui::Sense::hover());
    let painter = ui.painter_at(rect);
    let box_of = |t: TechId| -> egui::Rect {
        let card = game.tables.tech(t);
        let b = branches.iter().position(|x| *x == card.branch).unwrap_or(0);
        let r = card.rung.max(1) as usize - 1;
        let here = &cell[r * branches.len() + b];
        let i = here.iter().position(|x| *x == t).unwrap_or(0) as f32;
        let each = span[b] * COL / here.len().max(1) as f32;
        let centre = left[b] + each * (i + 0.5);
        let min = rect.min + egui::vec2(centre - BOX_W / 2.0, HEAD + r as f32 * ROW + (ROW - BOX_H) / 2.0);
        egui::Rect::from_min_size(min, egui::vec2(BOX_W, BOX_H))
    };
    for (i, b) in branches.iter().enumerate() {
        painter.text(rect.min + egui::vec2(left[i] + span[i] * COL / 2.0, HEAD / 2.0), egui::Align2::CENTER_CENTER, b, FontId::proportional(14.0), Color32::WHITE);
    }
    // Lines first, so the boxes sit on top of them. A line is green once the Tech it comes from is done.
    for t in TechId::ALL {
        for n in &game.tables.tech(t).needs {
            let from = box_of(*n).center_bottom();
            let to = box_of(t).center_top();
            let colour = if game.research.done.contains(n) { Color32::from_rgb(120, 200, 120) } else { Color32::from_gray(150) };
            painter.line_segment([from, to], egui::Stroke::new(2.0, colour));
            painter.circle_filled(to, 3.5, colour);
        }
    }
    for t in TechId::ALL {
        let card = game.tables.tech(t);
        let r = box_of(t);
        let (fill, status) = if game.research.done.contains(&t) {
            (Color32::from_rgb(50, 120, 60), "done")
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
        painter.text(r.center_top() + egui::vec2(0.0, 14.0), egui::Align2::CENTER_CENTER, &card.name, FontId::proportional(13.0), Color32::WHITE);
        painter.text(r.center_top() + egui::vec2(0.0, 32.0), egui::Align2::CENTER_CENTER, format!("cost {} - {}", card.cost, status), FontId::proportional(11.0), Color32::from_gray(230));
        let needs = if card.needs.is_empty() { "nothing".to_string() } else { card.needs.iter().map(|n| game.tables.tech(*n).name.clone()).collect::<Vec<_>>().join(" and ") };
        ui.interact(r, ui.id().with(format!("tech-{t:?}")), egui::Sense::hover()).on_hover_text(format!("{} (rung {}, cost {} Research)\n{}\nNeeds: {}", card.name, card.rung, card.cost, card.effect, needs));
        if must_pick && available.contains(&t) {
            let b = egui::Rect::from_center_size(r.center_bottom() - egui::vec2(0.0, 11.0), egui::vec2(56.0, 18.0));
            if ui.put(b, egui::Button::new(RichText::new("Pick").size(11.0))).clicked() {
                actions.push(Action::PickTech(t));
            }
        }
    }
    ui.horizontal(|ui| {
        for (colour, label) in [(Color32::from_rgb(50, 120, 60), "done"), (Color32::from_rgb(170, 130, 30), "under research"), (Color32::from_rgb(40, 90, 160), "available"), (Color32::from_gray(60), "locked")] {
            let (sw, _) = ui.allocate_exact_size(egui::vec2(14.0, 14.0), egui::Sense::hover());
            ui.painter().rect_filled(sw, 3.0, colour);
            ui.label(label);
        }
        ui.label("Hover a box for its effect.");
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

fn popups(ctx: &egui::Context, session: &Session, game: &Game, view: &mut ViewState, actions: &mut Vec<Action>) {
    if view.show_trade && !session.spectator {
        let mut open = true;
        egui::Window::new("Trading").open(&mut open).default_width(470.0).show(ctx, |ui| trading_window(ui, session, game, view, actions));
        view.show_trade = open;
    }
    if view.show_tech {
        let mut open = true;
        egui::Window::new("Tech Tree").open(&mut open).resizable(false).show(ctx, |ui| {
            ui.label(match game.research.current {
                Some(t) => format!("Under research: {} ({} of {}). {}", game.tables.tech(t).name, game.research.progress, game.tables.tech(t).cost, game.research_lead_text()),
                None => format!("No Tech under research. {} Research waiting.", game.research.unallocated),
            });
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
                if game.seat(Seat(0)).archive_funding || session.pending.iter().any(|o| matches!(o, Order::SetArchiveFunding { on: true })) {
                    ui.colored_label(Color32::YELLOW, "Your Labs pay the Archive fund: the turn after they next pay it, Provisional Findings is off.");
                }
            }
            let must_pick = game.research.awaiting_pick == Some(Seat(0)) && game.research.current.is_none();
            if must_pick {
                ui.colored_label(Color32::YELLOW, "You pick the next Tech: choose one below.");
            }
            // Ticket #98: the Lead chooses from the drawn shortlist, so that is what the tree offers.
            let available = game.pickable_techs();
            tech_tree(ui, game, &available, must_pick, actions);
        });
        view.show_tech = open;
    }
    if view.show_climate {
        let mut open = true;
        let bottom = ctx.viewport_rect().max.y;
        // Ticket #64: the spectator's card sits on the left, so the panel's home is clear of it.
        let home = (if session.spectator { 420.0 } else { 10.0 }, bottom - 400.0);
        let mut window = egui::Window::new("Climate Panel").open(&mut open).default_pos(home).default_width(400.0);
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
            ui.label(RichText::new("Emissions this turn, by source").strong());
            ui.label(format!("Nation State industry {:.1}", e.state_industry));
            ui.label(format!("Factories {:.1}", e.factories));
            ui.label(format!("Power Plants {:.1}", e.power_plants));
            ui.label(format!("Refineries {:.1}", e.refineries));
            ui.label(format!("Launches {:.1}", e.launches));
            {
                let c = &game.tables.climate;
                ui.label(format!("Population {:.1}", e.population)).on_hover_text(format!(
                    "Each state's people emit {:.2} + {:.2} x its Industry Level per hundred million, halved by Green Consensus, times its controller's Emissions multiplier. The Custodians' Leapfrog lowers a state's own figure by {:.2} for good, never below {:.2}.",
                    c.population_emissions_base, c.population_emissions_per_level, c.population_emissions_per_level, c.population_emissions_base
                ));
            }
            if e.cards > 0.0 {
                ui.label(format!("Event cards {:.1}", e.cards));
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
                RichText::new(format!("Committed: {:+.1} C even if net Emissions stopped today", game.target_temperature()))
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
            ui.label(RichText::new("Blame: the CO2 each Faction is answerable for").strong());
            for seat in Seat::ALL {
                let s = game.seat(seat);
                let credit = game.blame_credit(seat);
                let line = if credit > 0.0 {
                    format!(
                        "{}: emitted {:.0} ppm, removed {:.0}, Blame 0, credit {:.0} ppm, share {:.2}, thresholds x{:.2}",
                        game.seat_name(seat),
                        s.blame_emitted,
                        s.blame_removed,
                        credit,
                        game.blame_share(seat),
                        game.blame_threshold_multiplier(seat)
                    )
                } else {
                    format!(
                        "{}: emitted {:.0} ppm, removed {:.0}, Blame {:.0}, share {:.2}, thresholds x{:.2}",
                        game.seat_name(seat),
                        s.blame_emitted,
                        s.blame_removed,
                        game.blame(seat),
                        game.blame_share(seat),
                        game.blame_threshold_multiplier(seat)
                    )
                };
                ui.label(RichText::new(line).color(seat_colour(session, seat)));
            }
            ui.label(RichText::new("A share above a fair quarter raises that Faction's Influence thresholds on every Nation State it does not hold, up to half again.").weak());
        });
        view.show_climate = open;
    }
    if view.show_victory {
        let mut open = true;
        egui::Window::new("Victory").open(&mut open).default_width(470.0).show(ctx, |ui| {
            // Ticket #50: a row per seat, in seat order, each headed by its Faction in its colour.
            for seat in Seat::ALL {
                let p = game.progress(seat);
                ui.label(RichText::new(format!("{} - {:.0}% of the way there", game.seat_name(seat), p.score() * 100.0)).strong().color(seat_colour(session, seat)));
                ui.label(RichText::new(&game.tables.faction(game.kind(seat)).victory).weak());
                ui.label(match &p.first_held_back {
                    Some(why) => format!("{}: {:.0} of {:.0} - {}", p.first_name, p.first_value, p.first_bar, why),
                    None => format!("{}: {:.0} of {:.0}", p.first_name, p.first_value, p.first_bar),
                });
                ui.add(egui::ProgressBar::new(p.first_fraction() as f32));
                // Ticket #51: the second part in the words its own card uses.
                ui.label(format!("{}: {}", p.second_name, p.second_text));
                ui.add(egui::ProgressBar::new(p.second_fraction() as f32));
                // Ticket #72: the Prospectors set their Venture Capital Fund's share here, and draw.
                if seat == Seat(0) && !session.spectator && game.kind(Seat(0)) == FactionKind::Prospectors {
                    let v = game.tables.venture.clone();
                    let now = (game.seat(Seat(0)).venture_share * 100.0).round() as u32;
                    let pending_share = session.pending.iter().find_map(|o| if let Order::SetVentureShare { share } = o { Some(*share) } else { None });
                    ui.horizontal_wrapped(|ui| {
                        ui.label(format!("Banking {now}% of Materials output{}:", pending_share.map(|p| format!(" ({p}% from next turn)")).unwrap_or_default()));
                        let step = (v.share_step * 100.0).round().max(1.0) as u32;
                        let max = (v.max_share * 100.0).round() as u32;
                        let mut pct = 0u32;
                        while pct <= max {
                            if ui.selectable_label(pending_share.unwrap_or(now) == pct, format!("{pct}%")).clicked() {
                                if let Some(i) = session.pending.iter().position(|o| matches!(o, Order::SetVentureShare { .. })) {
                                    actions.push(Action::Cancel(i));
                                }
                                if pct != now {
                                    actions.push(Action::Place(Order::SetVentureShare { share: pct }));
                                }
                            }
                            pct += step;
                        }
                    });
                    let draw = Order::DrawVenture { amount: 10 };
                    let ok = game.check_order(Seat(0), &session.pending, &draw).is_ok();
                    let back = (10.0 * v.draw_return).floor() as i64;
                    if ui.add_enabled(ok, egui::Button::new("Draw 10 from the Fund")).on_hover_text(format!("{back} Materials come back to the Stockpile; a tenth is lost.")).clicked() {
                        actions.push(Action::Place(draw));
                    }
                }
                ui.add_space(8.0);
            }
            ui.label(format!("Collapse Line +{:.1} C; the Temperature is {:+.1}.", game.tables.climate.collapse_line, game.climate.temperature));
            // Ticket #53: who is doing this to the world, as one strip of four bars.
            ui.separator();
            ui.label(RichText::new("Blame: each Faction's share of the CO2 the table has put up").strong());
            for seat in Seat::ALL {
                let share = game.blame_share(seat);
                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("{:>12}", game.seat_name(seat))).color(seat_colour(session, seat)));
                    ui.add(
                        egui::ProgressBar::new(share as f32)
                            .desired_width(240.0)
                            .fill(seat_colour(session, seat))
                            .text(RichText::new(format!("{:.0}%", share * 100.0)).color(Color32::BLACK)),
                    );
                    let credit = game.blame_credit(seat);
                    if credit > 0.0 {
                        ui.label(RichText::new(format!("Blame 0, credit {credit:.0} ppm")).weak());
                    } else {
                        ui.label(RichText::new(format!("Blame {:.0} ppm, thresholds x{:.2}", game.blame(seat), game.blame_threshold_multiplier(seat))).weak());
                    }
                });
            }
        });
        view.show_victory = open;
    }
    match view.popup {
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
                    advance_popup(view, moments);
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
                    if ui.button("End Turn").clicked() {
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
                                    if ui.add(egui::Button::new(&l.text).frame(false)).on_hover_text("Go there").clicked() {
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
                                ui.label(
                                    RichText::new(format!(
                                        "   {}{}: {}, strength {}, {} hit(s) landed; destroyed: {}; escaped: {}",
                                        who,
                                        if party.aggressor { ", attacking" } else { "" },
                                        party.units,
                                        party.strength,
                                        party.hits,
                                        if party.destroyed.is_empty() { "none".to_string() } else { party.destroyed.join(", ") },
                                        if party.escaped.is_empty() { "none".to_string() } else { party.escaped.join(", ") },
                                    ))
                                    .color(colour),
                                );
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
                    advance_popup(view, moments);
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
                ui.label(RichText::new(&m.figure).size(30.0).strong().color(Color32::from_rgb(255, 220, 150)));
                ui.label(RichText::new(&m.text).size(17.0));
                if let Some(note) = &m.note {
                    ui.label(RichText::new(note).size(15.0).color(Color32::from_rgb(200, 220, 255)));
                }
                // Ticket #58: a completed Tech shows the tree with its new box lit, and the Pick
                // buttons when the player is the Research Lead.
                if m.tech.is_some() {
                    ui.separator();
                    let must_pick = game.research.awaiting_pick == Some(Seat(0)) && game.research.current.is_none();
                    let available = game.pickable_techs();
                    tech_tree(ui, game, &available, must_pick, actions);
                }
                ui.horizontal(|ui| {
                    if ui.button("Close").clicked() {
                        advance_popup(view, count);
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

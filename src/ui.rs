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

/// What the drawn interface asks the session to do, applied after drawing.
enum Action {
    Place(Order),
    Cancel(usize),
    EndTurn,
    PickTech(TechId),
    ChooseFaction(FactionKind),
    NewGame(FactionKind, StateId),
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

pub fn keyboard(keys: Res<ButtonInput<KeyCode>>, mut view: ResMut<ViewState>, session: Res<Session>, contexts: Option<Res<bevy_egui::input::EguiWantsInput>>) {
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
        if view.popup != Popup::None {
            advance_popup(&mut view);
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

fn advance_popup(view: &mut ViewState) {
    view.popup = match view.popup {
        Popup::Event => Popup::Report,
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

fn seat_colour(session: &Session, seat: Seat) -> Color32 {
    let c = session.colours()[seat.index()];
    Color32::from_rgb((c[0] * 255.0) as u8, (c[1] * 255.0) as u8, (c[2] * 255.0) as u8)
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
    match session.screen.clone() {
        Screen::Title => title_screen(&mut root, &mut session, &mut actions),
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
                view.popup = if session.game.as_ref().and_then(|g| g.last_event.as_ref()).is_some() { Popup::Event } else { Popup::Report };
                view.attack_preview = false;
            }
            Action::PickTech(t) => {
                let result = session.game.as_mut().map(|g| g.pick_tech(Seat(0), t));
                if let Some(Err(e)) = result {
                    session.last_error = Some(e);
                }
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
            if ui.add(egui::Button::new(RichText::new("Quit").size(22.0)).min_size(egui::vec2(220.0, 44.0))).clicked() {
                actions.push(Action::Quit);
            }
            ui.add_space(30.0);
            ui.label(RichText::new(format!("Seed {}", session.seed)).weak());
        });
    });
}

fn faction_screen(root: &mut Ui, session: &Session, actions: &mut Vec<Action>) {
    egui::CentralPanel::default().show(root, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.label(RichText::new("Choose your Faction").size(30.0).strong());
            ui.add_space(20.0);
        });
        ui.columns(2, |cols| {
            for (i, kind) in [FactionKind::Custodians, FactionKind::Prospectors].into_iter().enumerate() {
                let card = session.tables.faction(kind);
                let ui = &mut cols[i];
                egui::Frame::group(ui.style()).inner_margin(14.0).show(ui, |ui| {
                    let c = card.colour;
                    ui.label(RichText::new(&card.name).size(26.0).strong().color(Color32::from_rgb((c[0] * 255.0) as u8, (c[1] * 255.0) as u8, (c[2] * 255.0) as u8)));
                    ui.label(&card.blurb);
                    ui.add_space(8.0);
                    ui.label(RichText::new("Multipliers").strong());
                    ui.label(format!("Facility and Module output x{}", card.output_multiplier));
                    ui.label(format!("Emissions from Earth sources it controls x{}", card.emissions_multiplier));
                    ui.label(format!("Research x{}", card.research_multiplier));
                    ui.label(format!("Influence Allotment x{}", card.influence_multiplier));
                    ui.add_space(8.0);
                    ui.label(RichText::new("Signature rule").strong());
                    ui.label(&card.signature);
                    ui.add_space(8.0);
                    ui.label(RichText::new("Victory Condition").strong());
                    ui.label(&card.victory);
                    ui.add_space(12.0);
                    if ui.add(egui::Button::new(RichText::new(format!("Play the {}", card.name)).size(18.0)).min_size(egui::vec2(200.0, 40.0))).clicked() {
                        actions.push(Action::ChooseFaction(kind));
                    }
                });
            }
        });
    });
}

fn start_screen(root: &mut Ui, session: &Session, faction: FactionKind, actions: &mut Vec<Action>) {
    egui::Panel::right("start_panel").default_size(320.0).show(root, |ui| {
        ui.add_space(10.0);
        ui.label(RichText::new("Choose your starting continent").size(22.0).strong());
        ui.label(format!("You play the {}. The AI takes the uncontrolled continent with the highest Industry Level.", faction.name()));
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
    let painter = ctx.layer_painter(egui::LayerId::background());
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
            ui.separator();
            // Ticket #42: the turn's Allotment and what the trading window added, shown apart.
            let bought: i64 = session.pending.iter().map(|o| if let Order::BuyInfluence { amount } = o { *amount } else { 0 }).sum();
            let influence = if bought > 0 { format!("Influence {} of {} ({} free + {} bought)", influence_left, s.allotment + bought, s.allotment, bought) } else { format!("Influence {} of {}", influence_left, s.allotment) };
            ui.label(influence).on_hover_text("The Allotment is what your places and buildings give each turn; bought Influence comes from the Trading window at 2 Ducats each.");
            ui.separator();
            ui.label(RichText::new(format!("Turn {} / {}", game.turn, game.tables.victory.turns)).strong());
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
            if ui.button("Trading").clicked() {
                view.show_trade = !view.show_trade;
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
                let must_pick = game.research.awaiting_pick == Some(Seat(0)) && !game.available_techs().is_empty();
                let button = egui::Button::new(RichText::new("End Turn").strong().size(16.0)).fill(Color32::from_rgb(120, 40, 30));
                if ui.add_enabled(!must_pick && view.popup == Popup::None, button).on_disabled_hover_text("Pick a Tech first").clicked() {
                    let (_, influence_left) = game.remaining(Seat(0), &session.pending);
                    if influence_left > 0 && game.seat(Seat(0)).allotment > 0 {
                        view.popup = Popup::ConfirmEndTurn;
                    } else {
                        actions.push(Action::EndTurn);
                    }
                }
            }
        });
    });
}

// ------------------------------------------------------------------ overlays and picking

#[allow(clippy::too_many_arguments)]
fn overlays(painter: &egui::Painter, session: &Session, game: &Game, view: &ViewState, camera: &Camera, cam_gt: &GlobalTransform, globes: &Query<(&Globe, &GlobalTransform)>, hotspots: &mut Vec<Hotspot>) {
    let project = |p: Vec3| -> Option<Pos2> { camera.world_to_viewport(cam_gt, p).ok().map(|v| Pos2::new(v.x, v.y)) };
    let cam_pos = cam_gt.translation();
    match view.view {
        View::Solar => {
            for body in BodyId::ALL {
                let pos = geo::solar_position(body, game.turn);
                if let Some(p) = project(pos + Vec3::Y * (geo::solar_radius(body) + 0.05)) {
                    let name = game.tables.body(body).name.clone();
                    let slots = game.tables.body(body).colony_slots();
                    let filled = game.colonies.iter().filter(|c| c.body == body && !c.in_orbit).count();
                    let orbital = game.tables.body(body).orbital_slots;
                    let stations = game.colonies.iter().filter(|c| c.body == body && c.in_orbit).count();
                    let text = format!("{name}  {filled}/{slots} slots, {stations}/{orbital} stations");
                    label_at(painter, p - egui::vec2(0.0, 22.0), &text, Color32::WHITE, 13.0);
                    hotspots.push(Hotspot { pos: p, radius: 40.0, hit: Hit::Enter(body) });
                    if let Some(s) = game.orbital_control(body) {
                        label_at(painter, p - egui::vec2(0.0, 40.0), &format!("Orbital Control: {}", game.seat_name(s)), seat_colour(session, s), 12.0);
                    }
                }
                for seat in Seat::ALL {
                    let ships = game.ships_at(seat, body);
                    if ships.is_empty() {
                        continue;
                    }
                    let side = if seat == Seat(0) { -1.0 } else { 1.0 };
                    let world = pos + Vec3::new(side * geo::solar_radius(body) * 1.4, geo::solar_radius(body) + 0.25, 0.0);
                    if let Some(p) = project(world) {
                        let text = format!("{} x{}  str {}", game.seat_name(seat), ships.len(), game.ship_stack_strength(seat, body));
                        label_at(painter, p + egui::vec2(0.0, -16.0), &text, seat_colour(session, seat), 12.0);
                        hotspots.push(Hotspot { pos: p, radius: 26.0, hit: Hit::Select(Selection::ShipStack(body, seat)) });
                    }
                }
            }
            for s in &game.ships {
                if let ShipAt::Transit { from, to, turns_left } = s.at {
                    let a = geo::solar_position(from, game.turn);
                    let b = geo::solar_position(to, game.turn);
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
                    // The band along the top: Ship stacks in orbit and Orbital Control.
                    let mut band = Vec::new();
                    for seat in Seat::ALL {
                        let n = game.ships_at(seat, body).len();
                        if n > 0 {
                            band.push(format!("{}: {} Ship(s), strength {}", game.seat_name(seat), n, game.ship_stack_strength(seat, body)));
                        }
                    }
                    for c in game.colonies.iter().filter(|c| c.in_orbit && c.body == body) {
                        let who = c.control.director().map(|s| game.seat_name(s)).unwrap_or_else(|| "nobody's".into());
                        band.push(format!("{} ({})", game.station_name(body, c.slot), who));
                    }
                    band.push(match game.orbital_control(body) {
                        Some(s) => format!("Orbital Control: {}", game.seat_name(s)),
                        None => "Orbital Control: nobody".to_string(),
                    });
                    let rect = painter.clip_rect();
                    label_at(painter, Pos2::new(rect.center().x - 120.0, rect.min.y + 60.0), &format!("In orbit around {}: {}", game.tables.body(body).name, band.join(" | ")), Color32::WHITE, 13.0);
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
                    None => (format!("{name}: empty"), Color32::LIGHT_GRAY, Hit::Select(Selection::Slot(body, slot))),
                };
                label_at(painter, p + egui::vec2(0.0, 24.0), &text, colour, 12.0);
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
    for slot in game.free_orbital_slots(body) {
        cost_button(ui, game, &session.pending, Order::BuildStation { body, slot }, &format!("Build {} here", game.station_name(body, slot)), actions);
    }
    ui.label(RichText::new("A station holds a Shipyard and Habitats. Ships are built only at a Shipyard.").weak());
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
                let hit = geo::ray_sphere(origin, dir, geo::solar_position(body, game.turn), geo::solar_radius(body) * 1.5);
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
    egui::Panel::right("side").default_size(360.0).resizable(true).show(root, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            match view.selection {
                Selection::None => {
                    ui.add_space(6.0);
                    ui.label(RichText::new(match view.view {
                        View::Solar => "Solar System Map",
                        View::Surface(BodyId::Earth) => "Earth Map",
                        View::Surface(b) => game.tables.body(b).name.as_str(),
                    }).size(20.0).strong());
                    ui.label(match view.view {
                        View::Solar => "Click a Body to enter its surface. Click a Ship stack for orders.",
                        View::Surface(BodyId::Earth) => "Click a Nation State for its card and orders. Drag to turn, wheel to zoom.",
                        View::Surface(_) => "Click a Colony Slot or Colony for its card and orders.",
                    });
                    if let Some(e) = &session.last_error {
                        ui.colored_label(Color32::LIGHT_RED, e);
                    }
                    stations_panel(ui, session, game, view, actions);
                    ui.separator();
                    roster(ui, session, game, view);
                }
                Selection::State(sid) => state_panel(ui, session, game, view, sid, actions),
                Selection::Colony(cid) => colony_panel(ui, session, game, view, cid, actions),
                Selection::Slot(body, slot) => slot_panel(ui, game, body, slot, actions),
                Selection::ShipStack(body, seat) => stack_panel(ui, session, game, view, body, seat, actions),
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

/// The roster (#23): every Ship stack, Army, Colony and Nation State the player directs, each row a
/// button that selects it and jumps to its view, with a mark on anything that has no order this turn.
fn roster(ui: &mut Ui, session: &Session, game: &Game, view: &mut ViewState) {
    ui.label(RichText::new("Your roster").size(18.0).strong());
    ui.label(RichText::new("Click a row to select it and go there. \"no order\" marks what still waits.").weak());
    let pending = &session.pending;
    let mut jump: Option<(View, Selection)> = None;
    // Ships, one row per stack, then those in transit.
    ui.label(RichText::new("Ships").strong());
    let mut any_ship = false;
    for body in BodyId::ALL {
        let ships: Vec<&Ship> = game.ships.iter().filter(|s| s.seat == Seat(0) && s.at == ShipAt::Body(body)).collect();
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
        let mut text = format!("{} at {}: {} (strength {})", ships.len(), game.tables.body(body).name, kinds.join(", "), game.ship_stack_strength(Seat(0), body));
        if cargo > 0 {
            text.push_str(&format!(", {cargo} Colonists aboard"));
        }
        if armies > 0 {
            text.push_str(&format!(", {armies} Army aboard"));
        }
        if !ordered {
            text.push_str("  - no order");
        }
        if ui.button(text).clicked() {
            jump = Some((View::Solar, Selection::ShipStack(body, Seat(0))));
        }
    }
    for s in game.ships.iter().filter(|s| s.seat == Seat(0)) {
        if let ShipAt::Transit { to, turns_left, .. } = s.at {
            any_ship = true;
            let text = format!("{} in transit to {}, {} turn(s) left", s.kind.name(), game.tables.body(to).name, turns_left);
            if ui.button(text).clicked() {
                jump = Some((View::Solar, Selection::None));
            }
        }
    }
    if !any_ship {
        ui.label(RichText::new("  none; a Shipyard on a station or Colony builds them").weak());
    }
    // Armies.
    ui.label(RichText::new("Armies").strong());
    let mut any_army = false;
    for a in game.armies.iter().filter(|a| game.army_seat(a) == Some(Seat(0)) && !game.army_stands_down(a)) {
        any_army = true;
        let ordered = pending.iter().any(|o| match o {
            Order::MoveArmy { army, .. } | Order::Repair { unit: UnitRef::Army(army), .. } | Order::Load { army: Some(army), .. } => *army == a.id,
            Order::ArmyStance { place, .. } => a.at == ArmyAt::Place(*place),
            _ => false,
        });
        let (where_, target) = match a.at {
            ArmyAt::Place(Place::State(s)) => (game.tables.state(s).name.clone(), Some((View::Surface(BodyId::Earth), Selection::State(s)))),
            ArmyAt::Place(Place::Colony(c)) => (game.place_name(Place::Colony(c)), game.colony(c).map(|col| (View::Surface(col.body), Selection::Colony(c)))),
            ArmyAt::Aboard(ship) => (format!("aboard {ship}"), game.ship(ship).map(|s| match s.at { ShipAt::Body(b) => (View::Solar, Selection::ShipStack(b, Seat(0))), _ => (View::Solar, Selection::None) })),
        };
        let mut text = format!("{} at {}: strength {}, damage {}", if a.standing { "Standing Army" } else { "Army" }, where_, game.army_strength(a), a.damage);
        if !ordered && !matches!(a.at, ArmyAt::Aboard(_)) {
            text.push_str("  - no order");
        }
        if ui.button(text).clicked() {
            jump = target;
        }
    }
    if !any_army {
        ui.label(RichText::new("  none").weak());
    }
    // Colonies.
    ui.label(RichText::new("Colonies and stations").strong());
    let mut any_colony = false;
    for c in game.colonies.iter().filter(|c| c.control.director() == Some(Seat(0))) {
        any_colony = true;
        let building = c.queue.len();
        let text = format!("{}: {} Colonists, {} Modules{}", game.place_name(Place::Colony(c.id)), c.colonists, c.modules.len(), if building > 0 { format!(", {building} building") } else { String::new() });
        if ui.button(text).clicked() {
            jump = Some((View::Surface(c.body), Selection::Colony(c.id)));
        }
    }
    if !any_colony {
        ui.label(RichText::new("  none; a Colony Ship founds one").weak());
    }
    // Nation States.
    ui.label(RichText::new("Nation States").strong());
    for sid in game.directed_states(Seat(0)) {
        let st = game.state(sid);
        let building = st.queue.len();
        let text = format!("{}: {} Facilities, {} free slot(s){}", game.tables.state(sid).name, st.facilities.len(), game.free_slots(sid), if building > 0 { format!(", {building} building") } else { String::new() });
        if ui.button(text).clicked() {
            jump = Some((View::Surface(BodyId::Earth), Selection::State(sid)));
        }
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

fn order_text(game: &Game, o: &Order) -> String {
    match o {
        Order::BuildFacility { state, kind } => format!("Build {} in {}", kind.name(), game.tables.state(*state).name),
        Order::RaiseIndustry { state } => format!("Raise Industry Level in {}", game.tables.state(*state).name),
        Order::BuildModule { colony, kind } => format!("Build {} at {}", kind.name(), game.place_name(Place::Colony(*colony))),
        Order::BuildShip { site, kind } => format!("Build {} at {}", kind.name(), game.place_name(*site)),
        Order::BuildArmy { place } => format!("Build Army at {}", game.place_name(*place)),
        Order::Repair { unit, points } => format!("Repair {} point(s) on {}", points, match unit { UnitRef::Ship(s) => s.to_string(), UnitRef::Army(a) => a.to_string() }),
        Order::Transit { ship, to } => format!("Send {} to {}", ship, game.tables.body(*to).name),
        Order::ShipStance { body, stance } => format!("Ships at {}: {}", game.tables.body(*body).name, stance.name()),
        Order::ArmyStance { place, stance } => format!("Armies at {}: {}", game.place_name(*place), stance.name()),
        Order::MoveArmy { army, to } => format!("{} to {}", army, game.tables.state(*to).name),
        Order::Load { ship, colonists, army, .. } => format!("Load {} onto {}", if *colonists > 0 { format!("{colonists} Colonists") } else { format!("{}", army.unwrap_or(ArmyId(0))) }, ship),
        Order::Unload { ship, colonists, army, into } => match into {
            UnloadTarget::Slot(b, s) => format!("Found a Colony at {} on {} from {}", game.tables.body(*b).slots[*s as usize].name, game.tables.body(*b).name, ship),
            UnloadTarget::Colony(c) => format!("Unload {} from {} into {}", if *colonists > 0 { format!("{colonists} Colonists") } else if *army { "the Army".into() } else { "nothing".into() }, ship, game.place_name(Place::Colony(*c))),
        },
        Order::Influence { target, amount } => format!("{} Influence on {}", amount, game.place_name(*target)),
        Order::Restoration { steps } => format!("Restoration: {} Energy", steps * 10),
        Order::BuyInfluence { amount } => format!("Buy {} Influence with Ducats", amount),
        Order::RestorationWithDucats { steps } => format!("Restoration: {} step(s) paid in Ducats", steps),
        Order::RepairWithDucats { unit, points } => format!("Repair {} point(s) on {} with Ducats", points, match unit { UnitRef::Ship(s) => s.to_string(), UnitRef::Army(a) => a.to_string() }),
        Order::Buy { resource, amount } => format!("Buy {} {} for {} Ducats", amount, resource.name(), game.order_cost(Seat(0), o).ducats),
        Order::Sell { resource, amount } => format!("Sell {} {} for {} Ducats", amount, resource.name(), -game.order_cost(Seat(0), o).ducats),
        Order::BuildFacilityWithDucats { state, kind } => format!("Build {} in {} for Ducats", kind.name(), game.tables.state(*state).name),
        Order::BuildModuleWithDucats { colony, kind } => format!("Build {} at {} for Ducats", kind.name(), game.place_name(Place::Colony(*colony))),
        Order::BuildStation { body, slot } => format!("Build {} over {}", game.station_name(*body, *slot), game.tables.body(*body).name),
    }
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
    let threshold = game.influence_threshold(target);
    for seat in Seat::ALL {
        let _ = seat;
    }
    let standing = |s: Seat| game.seat(s).influence.get(&target).copied().unwrap_or(0);
    ui.label(format!("Standings: {} {}, {} {}; threshold {}", game.seat_name(Seat(0)), standing(Seat(0)), game.seat_name(Seat(1)), standing(Seat(1)), threshold));
    match game.place_control(target).controller() {
        Some(c) => {
            let need = threshold.max(standing(c) + 1);
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
    let fac_em: f64 = st.facilities.iter().filter(|f| f.online).map(|f| game.tables.facility(f.kind).emissions * mult).sum();
    ui.label(format!("Population {:.1} (hundreds of millions), Industry Level {}, leans {:?}", st.population, st.industry_level, card.resource_lean));
    ui.label(format!("Influence value {}: what it adds to its controller's Allotment each turn (+1 per Industry Level raised)", game.state_influence_value(sid)));
    ui.label(format!("GDP {}: its economy pays its controller {} Ducats a turn (GDP x Industry Level / 10); a Bank here would add {}", card.gdp, game.state_ducats(sid), (game.tables.facility(FacilityKind::Bank).produces.as_ref().map(|p| p.amount).unwrap_or(0) * card.gdp) / 10));
    ui.label(format!("Emissions this turn: industry {:.1}, Facilities {:.1}, people {:.1}", industry_em, fac_em, game.tables.climate.population_emissions_per_hundred_million * st.population * mult));
    ui.label(format!("Build slots: {} used of {} ({} free); Education Level {}", game.slots_used(sid), game.build_slots(sid), game.free_slots(sid), card.education_level));
    if st.lost_slots > 0 {
        ui.colored_label(Color32::LIGHT_BLUE, format!("{} slot(s) lost to the sea", st.lost_slots));
    }
    ui.label(RichText::new(format!("Facilities ({} of {} slots free)", game.free_slots(sid), game.build_slots(sid))).strong());
    let director = st.control.director();
    for f in &st.facilities {
        let figures = match director {
            Some(d) => game.facility_yield(d, sid, f.kind).text(),
            None => "idle, nobody directs this state".to_string(),
        };
        ui.label(format!("  {}: {}{}", f.kind.name(), figures, if f.online { "" } else { " (offline, making nothing)" }));
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
    let mine = st.control.director() == Some(Seat(0));
    if mine {
        ui.label(RichText::new("Build (hover a button for what it makes)").strong());
        for fk in FacilityKind::ALL {
            let hover = game.facility_yield(Seat(0), sid, fk).text();
            ui.horizontal(|ui| {
                cost_button_with_hover(ui, game, &session.pending, Order::BuildFacility { state: sid, kind: fk }, fk.name(), Some(hover), actions);
                // Ticket #42: the same building bought outright for Ducats.
                cost_button(ui, game, &session.pending, Order::BuildFacilityWithDucats { state: sid, kind: fk }, "or", actions);
            });
        }
        cost_button(ui, game, &session.pending, Order::RaiseIndustry { state: sid }, "Raise Industry Level", actions);
        cost_button(ui, game, &session.pending, Order::BuildArmy { place: Place::State(sid) }, "Build Army", actions);
        // Ticket #46: Ships come from Shipyards; a Launch Site lifts people to orbit.
        ui.label(
            RichText::new(if st.facilities.iter().any(|f| f.kind == FacilityKind::LaunchSite && f.online) {
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
    if game.kind(Seat(0)) == FactionKind::Custodians && mine {
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Restoration steps:");
            ui.add(egui::DragValue::new(&mut view.restoration_steps).range(1..=20));
            let order = Order::Restoration { steps: view.restoration_steps };
            if ui.add_enabled(game.check_order(Seat(0), &session.pending, &order).is_ok(), egui::Button::new(format!("Buy for {} Energy", game.order_cost(Seat(0), &order).energy))).clicked() {
                actions.push(Action::Place(order));
            }
            let with_ducats = Order::RestorationWithDucats { steps: view.restoration_steps };
            if ui.add_enabled(game.check_order(Seat(0), &session.pending, &with_ducats).is_ok(), egui::Button::new(format!("Buy for {} Ducats", game.order_cost(Seat(0), &with_ducats).ducats))).clicked() {
                actions.push(Action::Place(with_ducats));
            }
        });
    }
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
    ui.label(RichText::new("Modules").strong());
    let director = col.control.director();
    for m in &col.modules {
        let figures = match director {
            Some(d) => game.module_yield(d, cid, m.kind).text(),
            None => "idle".to_string(),
        };
        ui.label(format!("  {}: {}{}", m.kind.name(), figures, if m.online { "" } else { " (offline, making nothing)" }));
    }
    for b in &col.queue {
        ui.label(format!("  {} under construction, ready turn {}", b.item.name(), b.due_turn + 1));
    }
    let armies: Vec<&Army> = game.armies.iter().filter(|a| a.at == ArmyAt::Place(Place::Colony(cid))).collect();
    for a in &armies {
        let who = game.army_seat(a).map(|s| game.seat_name(s)).unwrap_or_else(|| "nobody's".into());
        ui.label(format!("  {} Army strength {}, damage {}", who, game.army_strength(a), a.damage));
    }
    ui.separator();
    let mine = col.control.director() == Some(Seat(0));
    if mine {
        ui.label(RichText::new("Build (hover a button for what it makes)").strong());
        for mk in ModuleKind::ALL {
            if col.in_orbit && !matches!(mk, ModuleKind::Shipyard | ModuleKind::Habitat) {
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
    let my_armies: Vec<&Army> = armies.iter().copied().filter(|a| game.army_seat(a) == Some(Seat(0))).collect();
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

fn slot_panel(ui: &mut Ui, game: &Game, body: BodyId, slot: u32, actions: &mut Vec<Action>) {
    ui.label(RichText::new(format!("{}, Colony Slot {} on {}", game.tables.body(body).slots[slot as usize].name, slot + 1, game.tables.body(body).name)).size(22.0).strong());
    ui.label("Empty. A Colony Ship carrying Colonists founds a Colony here; a Habitat comes with it.");
    let card = game.tables.body(body);
    ui.label(format!("Yields here: Mine x{}, Generator x{}, Refinery x{}, Habitat x{}", card.mine_yield, card.generator_yield, card.refinery_yield, card.habitat_yield));
    for s in game.ships.iter().filter(|s| s.seat == Seat(0) && s.at == ShipAt::Body(body) && s.kind == UnitKind::ColonyShip && s.colonists > 0) {
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
    if seat != Seat(0) {
        let mine = game.ship_stack_strength(Seat(0), body);
        let theirs = game.ship_stack_strength(seat, body);
        ui.label(format!("Odds of winning the first round if you attack: {:.0}% (your strength {} against {})", first_round_odds(mine, theirs) * 100.0, mine, theirs));
        return;
    }
    if ships.is_empty() {
        return;
    }
    ui.separator();
    stance_row(ui, game, &session.pending, ships[0].stance, |s| Order::ShipStance { body, stance: s }, true, actions);
    let enemy = game.ship_stack_strength(seat.other(), body);
    if enemy > 0 || !game.ships_at(seat.other(), body).is_empty() {
        let mine = game.ship_stack_strength(Seat(0), body);
        ui.label(format!("Enemy stack strength {}. Attack odds (first round): {:.0}%", enemy, first_round_odds(mine, enemy) * 100.0));
        if ui.button("Attack this turn").clicked() {
            view.attack_preview = true;
        }
        if view.attack_preview {
            ui.label(format!("Your {} (strength {}) against their {} (strength {}). Confirm?", ships.len(), mine, game.ships_at(seat.other(), body).len(), enemy));
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
        let (turns, fuel) = game.transit_cost(body, to);
        ui.horizontal_wrapped(|ui| {
            ui.label(format!("To {}: {} turn(s), {} Fuel each", game.tables.body(to).name, turns, fuel));
            for s in &ships {
                cost_button(ui, game, &session.pending, Order::Transit { ship: s.id, to }, &format!("{} {}", s.kind.name(), s.id.0), actions);
            }
        });
    }
    ui.label(RichText::new("Load and unload").strong());
    for s in &ships {
        let card = game.tables.unit(s.kind);
        if card.carries_colonists == 0 && !card.carries_army {
            continue;
        }
        ui.label(format!("{} {}:", s.kind.name(), s.id.0));
        if card.carries_colonists > s.colonists {
            let n = card.carries_colonists - s.colonists;
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
                            cost_button(ui, game, &session.pending, Order::Load { ship: s.id, colonists: n, from: LoadSource::State(chosen), army: None }, &format!("Load {n} Colonists"), actions);
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
                cost_button(ui, game, &session.pending, Order::Load { ship: s.id, colonists: 0, from: LoadSource::State(StateId::Asia), army: Some(a.id) }, &format!("Load the Army from {from}"), actions);
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
                for slot in game.free_slots_on(body) {
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
            ui.label(if sells { format!("{per} Ducats each; sells for {:.1}", per as f64 / game.tables.ducats.sell_divisor.max(1) as f64) } else { format!("{per} Ducats each") });
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
    let (rect, _) = ui.allocate_exact_size(egui::vec2(COL * branches.len() as f32, HEAD + ROW * rungs as f32), egui::Sense::hover());
    let painter = ui.painter_at(rect);
    let box_of = |t: TechId| -> egui::Rect {
        let card = game.tables.tech(t);
        let c = branches.iter().position(|b| *b == card.branch).unwrap_or(0) as f32;
        let r = card.rung.max(1) as f32 - 1.0;
        let min = rect.min + egui::vec2(c * COL + (COL - BOX_W) / 2.0, HEAD + r * ROW + (ROW - BOX_H) / 2.0);
        egui::Rect::from_min_size(min, egui::vec2(BOX_W, BOX_H))
    };
    for (i, b) in branches.iter().enumerate() {
        painter.text(rect.min + egui::vec2(i as f32 * COL + COL / 2.0, HEAD / 2.0), egui::Align2::CENTER_CENTER, b, FontId::proportional(14.0), Color32::WHITE);
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
        painter.rect(r, 6.0, fill, egui::Stroke::new(1.0, Color32::from_gray(200)), egui::StrokeKind::Inside);
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

fn popups(ctx: &egui::Context, session: &Session, game: &Game, view: &mut ViewState, actions: &mut Vec<Action>) {
    if view.show_trade {
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
            let must_pick = game.research.awaiting_pick == Some(Seat(0)) && game.research.current.is_none();
            if must_pick {
                ui.colored_label(Color32::YELLOW, "You pick the next Tech: choose one below.");
            }
            let available = game.available_techs();
            tech_tree(ui, game, &available, must_pick, actions);
        });
        view.show_tech = open;
    }
    if view.show_climate {
        let mut open = true;
        let bottom = ctx.viewport_rect().max.y;
        let home = (10.0, bottom - 400.0);
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
            ui.separator();
            ui.label(RichText::new("Emissions this turn, by source").strong());
            ui.label(format!("Nation State industry {:.1}", e.state_industry));
            ui.label(format!("Factories {:.1}", e.factories));
            ui.label(format!("Power Plants {:.1}", e.power_plants));
            ui.label(format!("Refineries {:.1}", e.refineries));
            ui.label(format!("Launches {:.1}", e.launches));
            ui.label(format!("Population {:.1}", e.population));
            if e.cards > 0.0 {
                ui.label(format!("Event cards {:.1}", e.cards));
            }
            ui.label(format!("Natural Sink -{:.1}{}", e.sink, if e.restoration > 0.0 { format!(" and Restoration -{:.1}", e.restoration) } else { String::new() }));
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
            let p = game.projection();
            let line = match p.collapse_turn {
                Some(t) => format!("At this rate, {:+.1} C by turn {}; Collapse at +{:.1} around turn {}.", p.temperature_at_last_turn, game.tables.victory.turns, game.tables.climate.collapse_line, t),
                None => format!("At this rate, {:+.1} C by turn {}; Collapse at +{:.1} not reached.", p.temperature_at_last_turn, game.tables.victory.turns, game.tables.climate.collapse_line),
            };
            ui.label(RichText::new(line).size(16.0).strong().color(Color32::from_rgb(255, 200, 120)));
            ui.separator();
            ui.label(format!("Stabilization run: {} consecutive turn(s) under the Sink.", game.seat(Seat(0)).stabilization_run));
        });
        view.show_climate = open;
    }
    if view.show_victory {
        let mut open = true;
        egui::Window::new("Victory").open(&mut open).default_width(420.0).show(ctx, |ui| {
            for seat in Seat::ALL {
                let p = game.progress(seat);
                ui.label(RichText::new(game.seat_name(seat)).strong().color(seat_colour(session, seat)));
                ui.label(format!("{}: {:.0} of {:.0}", p.first_name, p.first_value, p.first_bar));
                ui.add(egui::ProgressBar::new(p.first_fraction() as f32));
                ui.label(format!("Off-world Presence: {} of {} Colonists", p.presence, p.presence_bar));
                ui.add(egui::ProgressBar::new(p.presence_fraction() as f32));
                ui.add_space(8.0);
            }
            ui.label(format!("Collapse Line +{:.1} C; the Temperature is {:+.1}.", game.tables.climate.collapse_line, game.climate.temperature));
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
                    advance_popup(view);
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
                ui.label(RichText::new(format!("Report, turn {}", game.report.turn)).size(20.0).strong());
                egui::ScrollArea::vertical().max_height(420.0).show(ui, |ui| {
                    if !game.report.battles.is_empty() {
                        ui.label(RichText::new("Battle Report").strong());
                        for b in &game.report.battles {
                            ui.label(b.text(&|s| game.seat_name(s), "neutral"));
                        }
                    }
                    if let Some(e) = &game.report.event {
                        ui.label(RichText::new("Last turn's Event").strong());
                        ui.label(e);
                    }
                    ui.label(RichText::new("News").strong());
                    for l in &game.report.lines {
                        ui.label(l);
                    }
                    if !game.report.ai_lines.is_empty() {
                        ui.label(RichText::new(format!("What the {} did", game.seat_name(Seat(1)))).strong());
                        for l in game.report.ai_lines.iter().filter(|l| l.contains("take")) {
                            ui.label(l.trim().trim_start_matches("take").trim());
                        }
                    }
                });
                if ui.button("Close").clicked() {
                    advance_popup(view);
                }
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
                ui.label(format!("{}: {} {:.0} of {:.0}; {} of {} Colonists off Earth.", game.seat_name(seat), p.first_name, p.first_value, p.first_bar, p.presence, p.presence_bar));
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

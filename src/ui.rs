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
    if keys.just_pressed(KeyCode::Escape) {
        if view.popup != Popup::None {
            advance_popup(&mut view);
        } else if matches!(view.view, View::Surface(_)) {
            view.view = View::Solar;
            view.selection = Selection::None;
        }
    }
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
            if sid == StateId::Antarctica {
                continue;
            }
            let c = session.tables.state(sid);
            let text = format!("{}  (population {:.1}, Industry {}, leans {:?}, education {})", c.name, c.population, c.industry_level, c.resource_lean, c.education_level);
            if ui.add(egui::Button::new(text).min_size(egui::vec2(300.0, 32.0))).clicked() {
                actions.push(Action::NewGame(faction, sid));
            }
        }
        ui.add_space(20.0);
        ui.label(RichText::new("Antarctica is not offered.").weak());
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
            ui.label(RichText::new(format!("Materials {} ({})", left.materials, signed(inc.materials))).strong());
            ui.separator();
            ui.label(RichText::new(format!("Fuel {} ({})", left.fuel, signed(inc.fuel))).strong());
            ui.separator();
            ui.label(RichText::new(format!("Energy {} ({})", left.energy, signed(inc.energy))).strong());
            ui.separator();
            let research = match game.research.current {
                Some(t) => format!("Research {} / {} toward {}", game.research.progress, game.tables.tech(t).cost, game.tables.tech(t).name),
                None => format!("Research: no Tech chosen ({} waiting)", game.research.unallocated),
            };
            ui.label(research);
            ui.separator();
            ui.label(format!("Influence {} of {}", influence_left, s.allotment));
            ui.separator();
            ui.label(RichText::new(format!("Turn {} / {}", game.turn, game.tables.victory.turns)).strong());
            ui.separator();
            ui.label(format!("{:+.1} C, heading to {:+.1}", game.climate.temperature, game.target_temperature()));
        });
        ui.horizontal_wrapped(|ui| {
            if ui.button("Tech Tree").clicked() {
                view.show_tech = !view.show_tech;
            }
            if ui.button("Climate Panel").clicked() {
                view.show_climate = !view.show_climate;
            }
            if ui.button("Victory").clicked() {
                view.show_victory = !view.show_victory;
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
                    actions.push(Action::EndTurn);
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
                    let slots = game.tables.body(body).colony_slots;
                    let filled = game.colonies.iter().filter(|c| c.body == body).count();
                    let text = if slots > 0 { format!("{name}  {filled}/{slots} slots") } else { name };
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
                        let text = format!("{}\n{}\n{} Facilities, Army {}", game.tables.state(sid).name, owner, st.facilities.len(), armies);
                        label_at(painter, p, &text, colour, 12.0);
                        hotspots.push(Hotspot { pos: p, radius: 30.0, hit: Hit::Select(Selection::State(sid)) });
                    }
                }
                _ => {
                    for slot in 0..game.tables.body(body).colony_slots {
                        let (lon, lat) = geo::slot_lonlat(body, slot);
                        let Some(p) = visible(geo::local_from_lonlat(lon, lat) * 1.03) else { continue };
                        let (text, colour, hit) = match game.colony_at(body, slot) {
                            Some(c) => {
                                let mods: Vec<String> = c.modules.iter().map(|m| format!("{}{}", m.kind.name(), if m.online { "" } else { " (offline)" })).collect();
                                let army = game.armies.iter().filter(|a| a.at == ArmyAt::Place(Place::Colony(c.id))).count();
                                let owner = c.control.director().map(|s| game.seat_name(s)).unwrap_or_default();
                                (
                                    format!("Slot {}: {}\n{} Colonists\n{}{}", slot + 1, owner, c.colonists, mods.join(", "), if army > 0 { format!("\nArmies: {army}") } else { String::new() }),
                                    c.control.director().map(|s| seat_colour(session, s)).unwrap_or(Color32::LIGHT_GRAY),
                                    Hit::Select(Selection::Colony(c.id)),
                                )
                            }
                            None => (format!("Slot {}: empty", slot + 1), Color32::LIGHT_GRAY, Hit::Select(Selection::Slot(body, slot))),
                        };
                        label_at(painter, p + egui::vec2(0.0, 24.0), &text, colour, 12.0);
                        hotspots.push(Hotspot { pos: p, radius: 22.0, hit });
                    }
                    // The band along the top: Ship stacks in orbit and Orbital Control.
                    let mut band = Vec::new();
                    for seat in Seat::ALL {
                        let n = game.ships_at(seat, body).len();
                        if n > 0 {
                            band.push(format!("{}: {} Ship(s), strength {}", game.seat_name(seat), n, game.ship_stack_strength(seat, body)));
                        }
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
                    let (x, y) = geo::pixel_for(lon, lat, textures.earth.w, textures.earth.h);
                    view.selection = match textures.state_at(x, y) {
                        Some(s) => Selection::State(s),
                        None => Selection::None,
                    };
                }
                _ => {
                    let mut best: Option<(f32, u32)> = None;
                    for slot in 0..game.tables.body(body).colony_slots {
                        let (slon, slat) = geo::slot_lonlat(body, slot);
                        let d = geo::local_from_lonlat(slon, slat).angle_between(geo::local_from_lonlat(lon, lat)).to_degrees();
                        if d < 14.0 && best.map(|(bd, _)| d < bd).unwrap_or(true) {
                            best = Some((d, slot));
                        }
                    }
                    view.selection = match best {
                        Some((_, slot)) => match game.colony_at(body, slot) {
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
                        View::Surface(b) => if b == BodyId::Moon { "The Moon" } else { "Mars" },
                    }).size(20.0).strong());
                    ui.label(match view.view {
                        View::Solar => "Click a Body to enter its surface. Click a Ship stack for orders.",
                        View::Surface(BodyId::Earth) => "Click a Nation State for its card and orders. Drag to turn, wheel to zoom.",
                        View::Surface(_) => "Click a Colony Slot or Colony for its card and orders.",
                    });
                    if let Some(e) = &session.last_error {
                        ui.colored_label(Color32::LIGHT_RED, e);
                    }
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
            UnloadTarget::Slot(b, s) => format!("Found a Colony in slot {} on {} from {}", s + 1, game.tables.body(*b).name, ship),
            UnloadTarget::Colony(c) => format!("Unload {} from {} into {}", if *colonists > 0 { format!("{colonists} Colonists") } else if *army { "the Army".into() } else { "nothing".into() }, ship, game.place_name(Place::Colony(*c))),
        },
        Order::Influence { target, amount } => format!("{} Influence on {}", amount, game.place_name(*target)),
        Order::Restoration { steps } => format!("Restoration: {} Energy", steps * 10),
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
    let threshold = game.influence_threshold(target);
    for seat in Seat::ALL {
        let v = game.seat(seat).influence.get(&target).copied().unwrap_or(0);
        ui.label(format!("{} Influence here: {} of {}", game.seat_name(seat), v, threshold));
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
    ui.label(format!("Emissions this turn: industry {:.1}, Facilities {:.1}, people {:.1}", industry_em, fac_em, game.tables.climate.population_emissions_per_hundred_million * st.population * mult));
    ui.label(format!("Build slots: {} used of {} ({} free); Education Level {}", game.slots_used(sid), game.build_slots(sid), game.free_slots(sid), card.education_level));
    if st.lost_slots > 0 {
        ui.colored_label(Color32::LIGHT_BLUE, format!("{} slot(s) lost to the sea", st.lost_slots));
    }
    ui.label(RichText::new("Facilities").strong());
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
            cost_button_with_hover(ui, game, &session.pending, Order::BuildFacility { state: sid, kind: fk }, fk.name(), Some(hover), actions);
        }
        cost_button(ui, game, &session.pending, Order::RaiseIndustry { state: sid }, "Raise Industry Level", actions);
        cost_button(ui, game, &session.pending, Order::BuildArmy { place: Place::State(sid) }, "Build Army", actions);
        if st.facilities.iter().any(|f| f.kind == FacilityKind::LaunchSite) {
            ui.label(RichText::new("Ships (Launch Site)").strong());
            for uk in UnitKind::SHIPS {
                cost_button(ui, game, &session.pending, Order::BuildShip { site: Place::State(sid), kind: uk }, uk.name(), actions);
            }
        }
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
            ui.label("Restoration steps (10 Energy each):");
            ui.add(egui::DragValue::new(&mut view.restoration_steps).range(1..=20));
            let order = Order::Restoration { steps: view.restoration_steps };
            if ui.add_enabled(game.check_order(Seat(0), &session.pending, &order).is_ok(), egui::Button::new("Buy")).clicked() {
                actions.push(Action::Place(order));
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
            let hover = game.module_yield(Seat(0), cid, mk).text();
            cost_button_with_hover(ui, game, &session.pending, Order::BuildModule { colony: cid, kind: mk }, mk.name(), Some(hover), actions);
        }
        cost_button(ui, game, &session.pending, Order::BuildArmy { place: Place::Colony(cid) }, "Build Army (Barracks)", actions);
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
            }
        }
    }
    influence_row(ui, game, session, view, Place::Colony(cid), actions);
}

fn slot_panel(ui: &mut Ui, game: &Game, body: BodyId, slot: u32, actions: &mut Vec<Action>) {
    ui.label(RichText::new(format!("Colony Slot {} on {}", slot + 1, game.tables.body(body).name)).size(22.0).strong());
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
                        cost_button(ui, game, &session.pending, Order::Load { ship: s.id, colonists: k, from: LoadSource::Colony(c.id), army: None }, &format!("Load {} Colonists from slot {}", k, c.slot + 1), actions);
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
                        cost_button(ui, game, &session.pending, Order::Unload { ship: s.id, colonists: k, army: false, into: UnloadTarget::Colony(c.id) }, &format!("Unload {} Colonists into slot {}", k, c.slot + 1), actions);
                    }
                }
                if s.army.is_some() {
                    let label = if own { format!("Land the Army at slot {}", c.slot + 1) } else { format!("Land the Army to attack slot {}", c.slot + 1) };
                    cost_button(ui, game, &session.pending, Order::Unload { ship: s.id, colonists: 0, army: true, into: UnloadTarget::Colony(c.id) }, &label, actions);
                }
            }
            if s.kind == UnitKind::ColonyShip && s.colonists > 0 && body != BodyId::Earth {
                for slot in game.free_slots_on(body) {
                    cost_button(ui, game, &session.pending, Order::Unload { ship: s.id, colonists: s.colonists, army: s.army.is_some(), into: UnloadTarget::Slot(body, slot) }, &format!("Found a Colony in slot {}", slot + 1), actions);
                }
            }
        }
        if s.damage > 0 {
            cost_button(ui, game, &session.pending, Order::Repair { unit: UnitRef::Ship(s.id), points: s.damage }, "Repair fully", actions);
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

fn popups(ctx: &egui::Context, session: &Session, game: &Game, view: &mut ViewState, actions: &mut Vec<Action>) {
    if view.show_tech {
        let mut open = true;
        egui::Window::new("Tech Tree").open(&mut open).default_width(560.0).show(ctx, |ui| {
            ui.label(match game.research.current {
                Some(t) => format!("Under research: {} ({} of {}). {}", game.tables.tech(t).name, game.research.progress, game.tables.tech(t).cost, game.research_lead_text()),
                None => format!("No Tech under research. {} Research waiting.", game.research.unallocated),
            });
            let must_pick = game.research.awaiting_pick == Some(Seat(0)) && game.research.current.is_none();
            if must_pick {
                ui.colored_label(Color32::YELLOW, "You pick the next Tech: choose one below.");
            }
            let available = game.available_techs();
            let mut branch = String::new();
            for t in TechId::ALL {
                let card = game.tables.tech(t);
                if card.branch != branch {
                    branch = card.branch.clone();
                    ui.separator();
                    ui.label(RichText::new(&branch).strong());
                }
                let status = if game.research.done.contains(&t) {
                    "done".to_string()
                } else if game.research.current == Some(t) {
                    "under research".to_string()
                } else if available.contains(&t) {
                    "available".to_string()
                } else {
                    format!("needs {}", card.needs.iter().map(|n| game.tables.tech(*n).name.clone()).collect::<Vec<_>>().join(" and "))
                };
                ui.horizontal(|ui| {
                    ui.label(format!("{} (rung {}, cost {}): {} [{}]", card.name, card.rung, card.cost, card.effect, status));
                    if must_pick && available.contains(&t) && ui.button("Pick").clicked() {
                        actions.push(Action::PickTech(t));
                    }
                });
            }
        });
        view.show_tech = open;
    }
    if view.show_climate {
        let mut open = true;
        let bottom = ctx.viewport_rect().max.y;
        egui::Window::new("Climate Panel").open(&mut open).default_pos((10.0, bottom - 400.0)).default_width(400.0).show(ctx, |ui| {
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
            if e.wildfire > 0.0 {
                ui.label(format!("Wildfire {:.1}", e.wildfire));
            }
            ui.label(format!("Natural Sink -{:.1}{}", e.sink, if e.restoration > 0.0 { format!(" and Restoration -{:.1}", e.restoration) } else { String::new() }));
            ui.label(RichText::new(format!("Net {:+.1} ppm", e.net())).strong());
            ui.separator();
            let growth = game.population_growth_rate() * 100.0;
            ui.label(format!("Penalties in force: population growth {:+.2}% per turn, {} Climate cards in the deck ({} Calm left).", growth, game.deck.climate_cards_left(), game.deck.calm_left()));
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
                ui.label(RichText::new(format!("Scale at this Temperature: x{:.2}", game.last_event.as_ref().map(|e| e.scale).unwrap_or(1.0))).weak());
                if ui.button("Continue").clicked() {
                    advance_popup(view);
                }
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

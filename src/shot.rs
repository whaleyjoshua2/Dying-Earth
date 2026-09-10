//! Headless screenshot mode (spec 2.5): `dying-earth.exe shot:<prefix>` puts the window off-screen,
//! starts a game, captures the four views to `<prefix>-<view>.png`, and exits 0.

use crate::app::*;
use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};
use dying_earth_engine::*;

#[derive(Resource, Default)]
pub struct ShotPlan {
    pub step: usize,
    pub next_at: f32,
    pub captured: bool,
    pub done_at: Option<f32>,
    /// `menus:1` (a building aid): also capture the title, Faction and start screens and the Report.
    pub menus: bool,
    pub menu_step: usize,
    pub select: Option<String>,
    /// `tech:1` (a building aid): the Tech Tree window is open in every picture.
    pub tech: bool,
    /// `climate:toggle` (a building aid): the Earth picture closes the Climate Panel and brings it
    /// back through `toggle_climate`, the path the button and the C key use.
    pub climate_toggle: bool,
    pub toggled: bool,
    /// `trade:1` (a building aid): the trading window is open in every picture.
    pub trade: bool,
    /// `look:<lon>,<lat>` (a building aid): every surface picture faces that point.
    pub look: Option<(f32, f32)>,
}

fn apply_aids(plan: &mut ShotPlan, view: &mut ViewState) {
    if plan.tech {
        view.show_tech = true;
    }
    if plan.trade {
        view.show_trade = true;
    }
    if let (Some((lon, lat)), View::Surface(_)) = (plan.look, view.view) {
        view.yaw = crate::geo::yaw_facing(lon, lat);
        view.pitch = lat.to_radians().clamp(-1.3, 1.3);
    }
    if plan.climate_toggle && view.view == View::Surface(BodyId::Earth) {
        view.show_climate = false;
        plan.toggled = false;
    }
}

const VIEWS: [(&str, View); 4] = [
    ("solar", View::Solar),
    ("earth", View::Surface(BodyId::Earth)),
    ("moon", View::Surface(BodyId::Moon)),
    ("mars", View::Surface(BodyId::Mars)),
];

const MENUS: [&str; 4] = ["title", "faction", "start", "report"];

fn show_view(view: &mut ViewState, v: View) {
    match v {
        View::Solar => {
            view.view = View::Solar;
            view.selection = Selection::None;
        }
        View::Surface(b) => view.enter_surface(b),
    }
    // The Climate Panel belongs to the Earth Map; the other pictures show their globe unobstructed.
    view.show_climate = v == View::Surface(BodyId::Earth);
    view.show_tech = false;
}

pub fn shot_system(time: Res<Time>, mut plan: ResMut<ShotPlan>, mut session: ResMut<Session>, mut view: ResMut<ViewState>, mut commands: Commands, mut exit: MessageWriter<AppExit>) {
    if session.mode != Mode::Shot {
        return;
    }
    let t = time.elapsed_secs();
    if session.game.is_none() && !plan.menus {
        plan.menus = std::env::args().any(|a| a == "menus:1");
        if plan.menus {
            session.screen = Screen::Title;
            plan.next_at = t + 3.0;
            return;
        }
    }
    if plan.menus && plan.menu_step < MENUS.len() {
        if t < plan.next_at {
            return;
        }
        if !plan.captured {
            let path = format!("{}-{}.png", session.shot_prefix, MENUS[plan.menu_step]);
            commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
            plan.captured = true;
            plan.next_at = t + 1.0;
            return;
        }
        plan.captured = false;
        plan.menu_step += 1;
        match plan.menu_step {
            1 => session.screen = Screen::ChooseFaction,
            2 => {
                session.screen = Screen::ChooseStart { faction: FactionKind::Custodians };
                session.earth_dirty = true;
            }
            3 => {
                session.new_game(FactionKind::Custodians, StateId::Asia);
                view.popup = Popup::Report;
                view.show_climate = false;
            }
            _ => {
                // Fall through to the four views with the game already made.
                view.popup = Popup::None;
                if let Some(g) = &mut session.game {
                    let first = g.available_techs().first().copied();
                    if let Some(first) = first {
                        g.pick_tech(Seat(0), first).ok();
                    }
                }
                view.tech_prompted = true;
                show_view(&mut view, VIEWS[0].1);
                plan.next_at = t + 2.5;
                return;
            }
        }
        plan.next_at = t + 2.0;
        return;
    }
    if session.game.is_none() {
        session.new_game(FactionKind::Custodians, StateId::Asia);
        // `turns:<n>` (a building aid, not part of the spec) lets both AIs play n turns first so the
        // pictures show Colonies, transits and tinted states rather than an empty board.
        let turns: u32 = std::env::args().find_map(|a| a.strip_prefix("turns:").and_then(|v| v.parse().ok())).unwrap_or(0);
        if let Some(g) = &mut session.game {
            if let Some(first) = g.available_techs().first().copied() {
                g.pick_tech(Seat(0), first).ok();
            }
            if turns > 0 {
                g.seats[0].ai = true;
                for _ in 0..turns {
                    if g.is_over() {
                        break;
                    }
                    g.end_turn([Vec::new(), Vec::new()]);
                }
                g.seats[0].ai = false;
            }
        }
        session.earth_dirty = true;
        view.popup = Popup::None;
        view.tech_prompted = true;
        show_view(&mut view, VIEWS[0].1);
        // `select:<state id>` (a building aid) opens that Nation State's card in the Earth picture.
        plan.select = std::env::args().find_map(|a| a.strip_prefix("select:").map(str::to_owned));
        plan.tech = std::env::args().any(|a| a == "tech:1");
        plan.trade = std::env::args().any(|a| a == "trade:1");
        plan.look = std::env::args().find_map(|a| {
            let (lon, lat) = a.strip_prefix("look:")?.split_once(',')?;
            Some((lon.parse().ok()?, lat.parse().ok()?))
        });
        plan.climate_toggle = std::env::args().any(|a| a == "climate:toggle");
        apply_aids(&mut plan, &mut view);
        plan.next_at = t + 4.0;
        return;
    }
    if let Some(done) = plan.done_at {
        if t > done {
            exit.write(AppExit::Success);
        }
        return;
    }
    if t < plan.next_at {
        return;
    }
    if !plan.captured {
        if plan.climate_toggle && !plan.toggled && view.view == View::Surface(BodyId::Earth) {
            plan.toggled = true;
            crate::ui::toggle_climate(&mut view);
            plan.next_at = t + 1.0;
            return;
        }
        let (name, _) = VIEWS[plan.step];
        let path = format!("{}-{}.png", session.shot_prefix, name);
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
        plan.captured = true;
        // Give the capture a moment before the view changes under it.
        plan.next_at = t + 1.0;
        return;
    }
    plan.step += 1;
    plan.captured = false;
    if plan.step >= VIEWS.len() {
        plan.done_at = Some(t + 1.5);
    } else {
        let (_, v) = VIEWS[plan.step];
        show_view(&mut view, v);
        apply_aids(&mut plan, &mut view);
        let wanted = plan.select.as_ref().filter(|_| v == View::Surface(BodyId::Earth)).and_then(|name| StateId::ALL.into_iter().find(|s| format!("{s:?}").eq_ignore_ascii_case(name)));
        if let Some(s) = wanted {
            view.selection = Selection::State(s);
            view.show_climate = false;
        }
        plan.next_at = t + 2.5;
    }
}

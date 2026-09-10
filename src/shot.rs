//! Headless screenshot mode (spec 2.5): `dying-earth.exe shot:<prefix>` puts the window off-screen,
//! starts a game, captures the four views to `<prefix>-<view>.png`, and exits 0.

use crate::app::*;
use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};
use dying_earth_engine::*;

// `build_board` runs before the plan is next borrowed, so the Colony it plants is parked here.
thread_local! {
    static ARCHIVE_COLONY: std::cell::Cell<Option<ColonyId>> = const { std::cell::Cell::new(None) };
}

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
    /// `victory:1` (a building aid): the Victory panel is open in every picture.
    pub victory: bool,
    /// `stack:1` (a building aid): the player's Ship stack at Mars is selected, so its card and the
    /// attack odds preview are in the picture.
    pub stack: bool,
    /// `look:<lon>,<lat>` (a building aid): every surface picture faces that point.
    pub look: Option<(f32, f32)>,
    /// Ticket #50: 0 the Faction choice screen is not up yet, 1 it is up, 2 it has been captured.
    pub factions_step: u8,
    /// Ticket #51: `archive:<stage>` planted an Archive at this Colony, so the Body picture opens
    /// its card rather than the globe alone.
    pub archive_colony: Option<ColonyId>,
}

fn apply_aids(plan: &mut ShotPlan, view: &mut ViewState) {
    if plan.tech {
        view.show_tech = true;
    }
    if plan.trade {
        view.show_trade = true;
    }
    if plan.victory {
        view.show_victory = true;
    }
    if plan.stack {
        view.selection = Selection::ShipStack(BodyId::Mars, Seat(0));
    }
    // Ticket #51: `archive:<stage>` opens the Archive's Colony card in that Body's picture.
    if let (Some(cid), View::Surface(_)) = (plan.archive_colony, view.view) {
        view.selection = Selection::Colony(cid);
        view.show_climate = false;
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

const VIEWS: [(&str, View); 6] = [
    ("solar", View::Solar),
    ("earth", View::Surface(BodyId::Earth)),
    ("moon", View::Surface(BodyId::Moon)),
    ("mars", View::Surface(BodyId::Mars)),
    ("phobos", View::Surface(BodyId::Phobos)),
    ("deimos", View::Surface(BodyId::Deimos)),
];

const MENUS: [&str; 4] = ["title", "faction", "start", "report"];

/// The board every picture is taken of: a new game, the first Tech picked, the `turns:<n>` aid
/// played out, and (ticket #50) a Ship stack for every seat at Mars so the four-angle stack markers
/// and the four-Faction band are visible. Building aids, not part of the spec.
fn build_board(session: &mut Session) {
    // `player:<faction id>` (a building aid): the Faction in seat 0, so a picture can be taken of a
    // Faction other than the Custodians' seat.
    let player = std::env::args()
        .find_map(|a| a.strip_prefix("player:").and_then(FactionKind::from_id))
        .unwrap_or(FactionKind::Custodians);
    session.new_game(player, StateId::EastAsia);
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
                g.end_turn(std::array::from_fn(|_| Vec::new()));
            }
            g.seats[0].ai = false;
        }
        for seat in Seat::ALL {
            if !g.ships_at(seat, BodyId::Mars).is_empty() {
                continue;
            }
            let kind = match seat.index() {
                0 => UnitKind::Frigate,
                2 => UnitKind::Carrier,
                _ => UnitKind::ColonyShip,
            };
            let id = ShipId(g.fresh_id());
            let built_turn = g.turn;
            g.ships.push(Ship { id, kind, seat, damage: 0, at: ShipAt::Body(BodyId::Mars), colonists: 0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn });
        }
        // `battle:1` (a building aid): three seats bring a Frigate to Mars with Attack stances and
        // one more turn runs, so the Report carries a three-party Battle (ticket #50).
        if std::env::args().any(|a| a == "battle:1") {
            for seat in [Seat(0), Seat(1), Seat(2)] {
                let id = ShipId(g.fresh_id());
                let built_turn = g.turn;
                g.ships.push(Ship { id, kind: UnitKind::Frigate, seat, damage: 0, at: ShipAt::Body(BodyId::Mars), colonists: 0, army: None, stance: Stance::Attack, escaped: false, arrived_this_turn: false, built_turn });
            }
            for s in g.ships.iter_mut().filter(|s| s.at == ShipAt::Body(BodyId::Mars)) {
                s.stance = Stance::Attack;
            }
            // The rivals sit still for this one turn, so their stacks are all at Mars when it runs.
            for seat in Seat::ALL.into_iter().skip(1) {
                g.seats[seat.index()].ai = false;
            }
            let mut orders: [Vec<Order>; SEAT_COUNT] = std::array::from_fn(|_| Vec::new());
            orders[0] = vec![Order::ShipStance { body: BodyId::Mars, stance: Stance::Attack }];
            g.end_turn(orders);
            for seat in Seat::ALL.into_iter().skip(1) {
                g.seats[seat.index()].ai = true;
            }
        }
        // `archive:<stage>` (a building aid, ticket #51): seat 0 gets a Colony on Mars with the
        // Archive at that stage, the next stage building, a part-filled fund and Colonists in its
        // Habitats, since an AI Archivist rarely has all of that in six turns.
        if let Some(stage) = std::env::args().find_map(|a| a.strip_prefix("archive:").and_then(|v| v.parse::<u32>().ok())) {
            let stages = g.tables.archive.stages;
            let slot = g.free_slots_on(BodyId::Mars).first().copied().unwrap_or(0);
            let id = ColonyId(g.fresh_id());
            let mut modules = vec![Module::new(ModuleKind::Habitat), Module::new(ModuleKind::Habitat), Module::new(ModuleKind::Generator), Module::new(ModuleKind::Mine)];
            let mut archive = Module::new(ModuleKind::Archive);
            archive.stage = stage.min(stages);
            modules.push(archive);
            let turn = g.turn;
            let mut queue = Vec::new();
            if stage < stages {
                queue.push(Build { item: BuildItem::Module(ModuleKind::Archive), seat: Seat(0), due_turn: turn });
            }
            g.colonies.push(Colony { id, body: BodyId::Mars, slot, control: Control::Controlled(Seat(0)), modules, colonists: 8, queue, grid_failed: false, founded_turn: 1, in_orbit: false });
            g.seats[0].archive_fund = 14;
            g.seats[0].stockpile.materials = 120;
            g.seats[0].stockpile.energy = 60;
            ARCHIVE_COLONY.with(|c| c.set(Some(id)));
        }
        // `unrest:<n>` (a building aid, ticket #52): a spread of Unrest over three states on the
        // face the Earth picture shows, so one card, the map labels and the thresholds are all
        // visible at once. The AI seldom leaves a state of the player's this restive.
        if let Some(n) = std::env::args().find_map(|a| a.strip_prefix("unrest:").and_then(|v| v.parse::<f64>().ok())) {
            for (sid, off) in [(StateId::EastAsia, 0.0), (StateId::Europe, 1.0), (StateId::NorthAfrica, 3.0)] {
                let v = (n - off).clamp(0.0, 10.0);
                let st = g.state_mut(sid);
                st.unrest = v;
                st.unrest_reported = v;
            }
            // Room and money, so the card shows the Constabulary and the Relief buttons live.
            g.state_mut(StateId::EastAsia).industry_level += 3;
            g.seats[0].stockpile.materials = 200;
            g.seats[0].stockpile.ducats = 200;
        }
        // `blame:1` (a building aid, ticket #53, rebuilt on #54 now Restoration is retired): the
        // Custodian in seat 0 takes every Nation State and fills each with its cap of Scrubbers, so
        // one Climate phase takes back more CO2 than it has emitted all game and the Climate Panel's
        // Blame section shows a removal credit beside three Factions carrying Blame.
        if std::env::args().any(|a| a == "blame:1") && g.kind(Seat(0)) == FactionKind::Custodians {
            for sid in StateId::ALL {
                g.take_control(sid, Seat(0));
                fill_with_scrubbers(g, sid);
            }
            g.seats[0].stockpile.energy = 4000;
            run_one_quiet_turn(g);
        }
        // `scrub:<n>` (a building aid, ticket #54): the Custodian in seat 0 holds East Asia with n
        // Scrubbers standing (up to its cap), one Facility mothballed, two Leapfrogs bought and
        // Ducats left to buy a third, and a turn is run so the Climate Panel's Sink line carries
        // the Scrubbers. An AI Custodian builds one at a time and mothballs it again for Energy.
        if let Some(n) = std::env::args().find_map(|a| a.strip_prefix("scrub:").and_then(|v| v.parse::<u32>().ok()))
            && g.kind(Seat(0)) == FactionKind::Custodians
        {
            let sid = StateId::EastAsia;
            g.take_control(sid, Seat(0));
            while g.scrubbers_committed(sid) < n.min(g.scrubber_cap(sid)) {
                g.state_mut(sid).facilities.push(Facility::new(FacilityKind::Scrubber));
            }
            let per = g.tables.climate.population_emissions_per_level;
            g.state_mut(sid).leapfrog += per * 2.0;
            // A Factory stood down: the card shows what a mothballed Facility reads like.
            if let Some(f) = g.state_mut(sid).facilities.iter_mut().find(|f| f.kind == FacilityKind::Factory) {
                f.mothballed = true;
                f.online = false;
            }
            g.seats[0].stockpile.energy = 400;
            g.seats[0].stockpile.materials = 300;
            g.seats[0].stockpile.ducats = 300;
            run_one_quiet_turn(g);
            g.seats[0].stockpile.ducats = 300;
        }
        // `strip:1` (a building aid, ticket #54): the Prospector in seat 0 holds East Asia under a
        // Strip Permit with a turn already run, so the card reads "2 turns left". The AI Prospector
        // is never behind its Extraction pace, so it never issues one.
        if std::env::args().any(|a| a == "strip:1") && g.kind(Seat(0)) == FactionKind::Prospectors {
            let sid = StateId::EastAsia;
            g.take_control(sid, Seat(0));
            g.seats[0].stockpile.energy = 400;
            g.seats[0].ai = false;
            let mut orders: [Vec<Order>; SEAT_COUNT] = std::array::from_fn(|_| Vec::new());
            orders[0] = vec![Order::StripPermit { state: sid }];
            g.end_turn(orders);
            g.seats[0].ai = true;
        }
        // `temp:<now>[,<committed>]` (a building aid, ticket #55): the Temperature is put at `now`
        // and the CO2 Stock at the figure that commits the world to `committed` (the same, if it is
        // left off), then a quiet turn runs so the Climate phase fires every Break at or under the
        // Temperature and the Report and the Climate Panel read a real board. An AI game reaches a
        // given Temperature at a turn the aid cannot choose, and the last-turn picture needs a
        // Stock well ahead of the Temperature, which no ordinary board offers on demand.
        if let Some(arg) = std::env::args().find_map(|a| a.strip_prefix("temp:").map(str::to_owned)) {
            let (now, committed) = match arg.split_once(',') {
                Some((a, b)) => (a.parse::<f64>().ok(), b.parse::<f64>().ok()),
                None => (arg.parse::<f64>().ok(), None),
            };
            if let Some(now) = now {
                let c = g.tables.climate.clone();
                let target = committed.unwrap_or(now);
                g.climate.temperature = now;
                g.climate.co2 = c.starting_co2 + (target - c.base_temperature) * c.ppm_step / c.degrees_per_ppm_step;
                g.seats[0].stockpile.energy = 400;
                g.seats[0].stockpile.materials = 300;
                g.seats[0].stockpile.ducats = 300;
                run_one_quiet_turn(g);
            }
        }
        // `tints:1` (a building aid): one Nation State per seat on the face the Earth picture shows,
        // so all four Faction tints are in one picture. The AI seldom leaves four controllers alive.
        if std::env::args().any(|a| a == "tints:1") {
            for (sid, seat) in [(StateId::SouthAmerica, Seat(0)), (StateId::Europe, Seat(1)), (StateId::MiddleEast, Seat(2)), (StateId::SubSaharanAfrica, Seat(3))] {
                g.state_mut(sid).control = Control::Controlled(seat);
            }
        }
    }
    // A game played to its end shows the game-over modal, as it would in play.
    if session.game.as_ref().map(|g| g.is_over()).unwrap_or(false) {
        session.screen = Screen::GameOver;
    }
    session.earth_dirty = true;
}

/// A building aid (ticket #54): as many Scrubbers as the state's cap allows, standing and online.
fn fill_with_scrubbers(g: &mut Game, sid: StateId) {
    let cap = g.scrubber_cap(sid);
    while g.scrubbers_committed(sid) < cap {
        g.state_mut(sid).facilities.push(Facility::new(FacilityKind::Scrubber));
    }
}

/// A building aid: run one turn with seat 0 giving no orders, so Income and the Climate phase read
/// the board the aid just built.
fn run_one_quiet_turn(g: &mut Game) {
    g.seats[0].ai = false;
    g.end_turn(std::array::from_fn(|_| Vec::new()));
    g.seats[0].ai = true;
}

fn show_view(view: &mut ViewState, v: View) {
    match v {
        View::Solar => {
            view.view = View::Solar;
            view.selection = Selection::None;
            // A building aid: stand back a little, so Mars is in frame wherever its orbit has it.
            view.zoom = 1.35;
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
                build_board(&mut session);
                plan.archive_colony = ARCHIVE_COLONY.with(|c| c.get());
                view.popup = Popup::Report;
                view.show_climate = false;
            }
            _ => {
                // Fall through to the four views with the game already made.
                view.popup = Popup::None;
                view.tech_prompted = true;
                show_view(&mut view, VIEWS[0].1);
                plan.next_at = t + 2.5;
                return;
            }
        }
        plan.next_at = t + 2.0;
        return;
    }
    // Ticket #50: a picture of the Faction choice screen, where all four cards are dealt.
    if session.game.is_none() && plan.factions_step < 2 {
        if plan.factions_step == 0 {
            session.screen = Screen::ChooseFaction;
            plan.factions_step = 1;
            plan.next_at = t + 2.0;
            return;
        }
        if t < plan.next_at {
            return;
        }
        let path = format!("{}-factions.png", session.shot_prefix);
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
        plan.factions_step = 2;
        plan.next_at = t + 1.5;
        return;
    }
    if session.game.is_none() {
        if t < plan.next_at {
            return;
        }
        // `turns:<n>` (a building aid, not part of the spec) lets the four AIs play n turns first so
        // the pictures show Colonies, transits and tinted states rather than an empty board.
        build_board(&mut session);
        plan.archive_colony = ARCHIVE_COLONY.with(|c| c.get());
        view.popup = Popup::None;
        view.tech_prompted = true;
        show_view(&mut view, VIEWS[0].1);
        // `select:<state id>` (a building aid) opens that Nation State's card in the Earth picture.
        plan.select = std::env::args().find_map(|a| a.strip_prefix("select:").map(str::to_owned));
        plan.tech = std::env::args().any(|a| a == "tech:1");
        plan.trade = std::env::args().any(|a| a == "trade:1");
        plan.victory = std::env::args().any(|a| a == "victory:1");
        plan.stack = std::env::args().any(|a| a == "stack:1");
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

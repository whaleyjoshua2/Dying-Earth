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
    /// `panel:0` (a building aid, ticket #56): the Earth picture shows the globe with no Climate
    /// Panel over it, for a picture of the map itself.
    pub no_panel: bool,
    pub toggled: bool,
    /// `trade:1` (a building aid): the trading window is open in every picture.
    pub trade: bool,
    /// `victory:1` (a building aid): the Victory panel is open in every picture.
    pub victory: bool,
    /// `stack:1` (a building aid): the player's Ship stack at Mars is selected, so its card and the
    /// attack odds preview are in the picture.
    pub stack: bool,
    /// `hover:<body id>` (a building aid, ticket #57): the Solar System Map draws that Body's launch
    /// window tooltip as though the pointer were on it. Nothing hovers in a headless capture.
    pub hover: Option<BodyId>,
    /// `look:<lon>,<lat>` (a building aid): every surface picture faces that point.
    pub look: Option<(f32, f32)>,
    /// Ticket #50: 0 the Faction choice screen is not up yet, 1 it is up, 2 it has been captured.
    pub factions_step: u8,
    /// Ticket #51: `archive:<stage>` planted an Archive at this Colony, so the Body picture opens
    /// its card rather than the globe alone.
    pub archive_colony: Option<ColonyId>,
    /// Ticket #58, a building aid (`moment:colony`, `moment:tech`): the Report picture of a
    /// `menus:1` run opens that Moment instead of the Report itself.
    pub moment: Option<MomentKind>,
    /// Ticket #59, a building aid (`saved:1`): the top bar carries the notice a Save leaves, held
    /// up for the whole run so a capture cannot miss it.
    pub notice: Option<String>,
    /// Ticket #59, a building aid (`load:1`): 0 nothing yet, 1 the Load screen is up, 2 captured.
    pub load_step: u8,
}

/// Ticket #58: the Moment a `moment:` aid names.
fn moment_from_id(name: &str) -> Option<MomentKind> {
    match name {
        "colony" => Some(MomentKind::ColonyFounded),
        "tech" => Some(MomentKind::TechComplete),
        "control" => Some(MomentKind::ControlChanged),
        "climate" => Some(MomentKind::ClimateThreshold),
        "battle" => Some(MomentKind::DecisiveBattle),
        "antarctica" => Some(MomentKind::Antarctica),
        "archive" => Some(MomentKind::ArchiveComplete),
        "lost" => Some(MomentKind::LostInTransit),
        _ => None,
    }
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
    view.force_hover = plan.hover.filter(|_| view.view == View::Solar);
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
    if plan.no_panel {
        view.show_climate = false;
    }
}

const VIEWS: [(&str, View); 7] = [
    ("solar", View::Solar),
    ("earth", View::Surface(BodyId::Earth)),
    ("moon", View::Surface(BodyId::Moon)),
    ("mars", View::Surface(BodyId::Mars)),
    ("phobos", View::Surface(BodyId::Phobos)),
    ("deimos", View::Surface(BodyId::Deimos)),
    ("venus", View::Surface(BodyId::Venus)),
];

// Ticket #109: the credits picture sits between the Faction cards and the start screen, so the
// attribution the icons' licence requires is photographed with every other menu.
const MENUS: [&str; 5] = ["title", "faction", "credits", "start", "report"];

/// Ticket #57: the Body a `hover:` aid names, by the id its data row carries.
fn body_from_id(name: &str) -> Option<BodyId> {
    BodyId::ALL.into_iter().find(|b| format!("{b:?}").eq_ignore_ascii_case(name))
}

/// The board every picture is taken of: a new game, the first Tech picked, the `turns:<n>` aid
/// played out, and (ticket #50) a Ship stack for every seat at Mars so the four-angle stack markers
/// and the four-Faction band are visible. Building aids, not part of the spec.
fn build_board(session: &mut Session) {
    // `player:<faction id>` (a building aid): the Faction in seat 0, so a picture can be taken of a
    // Faction other than the Custodians' seat.
    let player = std::env::args()
        .find_map(|a| a.strip_prefix("player:").and_then(FactionKind::from_id))
        .unwrap_or(FactionKind::Custodians);
    // `spectate:1` (a building aid, ticket #64): the game nobody sits at, as the Spectate button
    // makes it, so a picture can be taken of the spectator's board and its dispatch.
    let spectate = std::env::args().any(|a| a == "spectate:1");
    if spectate {
        session.spectate();
    } else {
        session.new_game(player, StateId::EastAsia);
    }
    let turns: u32 = std::env::args().find_map(|a| a.strip_prefix("turns:").and_then(|v| v.parse().ok())).unwrap_or(0);
    if let Some(g) = &mut session.game {
        if !spectate && let Some(first) = g.available_techs().first().copied() {
            g.pick_tech(Seat(0), first).ok();
        }
        if turns > 0 {
            g.seats[0].ai = true;
            for _ in 0..turns {
                if g.is_over() {
                    break;
                }
                g.end_turn(std::array::from_fn(|_| Vec::new())).expect("the screenshot harness picks a Tech before it drives turns");
            }
            g.seats[0].ai = spectate;
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
            g.ships.push(Ship { id, kind, seat, damage: 0, at: ShipAt::Body(BodyId::Mars), colonists: 0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn, fuel: 30, slot: None });
        }
        // `battle:1` (a building aid): three seats bring a Frigate to Mars with Attack stances and
        // one more turn runs, so the Report carries a three-party Battle (ticket #50).
        if std::env::args().any(|a| a == "battle:1") {
            for seat in [Seat(0), Seat(1), Seat(2)] {
                let id = ShipId(g.fresh_id());
                let built_turn = g.turn;
                g.ships.push(Ship { id, kind: UnitKind::Frigate, seat, damage: 0, at: ShipAt::Body(BodyId::Mars), colonists: 0, army: None, stance: Stance::Attack, escaped: false, arrived_this_turn: false, built_turn, fuel: 30, slot: None });
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
            g.end_turn(orders).expect("the screenshot harness picks a Tech before it drives turns");
            for seat in Seat::ALL.into_iter().skip(1) {
                g.seats[seat.index()].ai = true;
            }
        }
        // `archive:<n>` (a building aid, ticket #51, reshaped by #68): seat 0 gets a Colony on Mars
        // with Colonists in its Habitats and the Archive at one of three points: 0, the Module on
        // order with the fund at its quarter; 1, the Module standing and the fund half paid; 2,
        // complete. An AI Archivist rarely has any of that in six turns.
        if let Some(point) = std::env::args().find_map(|a| a.strip_prefix("archive:").and_then(|v| v.parse::<u32>().ok())) {
            let research = g.tables.archive.research;
            let slot = g.free_slots_on(BodyId::Mars).first().copied().unwrap_or(0);
            let id = ColonyId(g.fresh_id());
            let mut modules = vec![Module::new(ModuleKind::Habitat), Module::new(ModuleKind::Habitat), Module::new(ModuleKind::Generator), Module::new(ModuleKind::Mine)];
            let turn = g.turn;
            let mut queue = Vec::new();
            if point == 0 {
                queue.push(Build { item: BuildItem::Module(ModuleKind::Archive), seat: Seat(0), due_turn: turn + 1, coastal: false });
            } else {
                modules.push(Module::new(ModuleKind::Archive));
            }
            g.colonies.push(Colony { id, body: BodyId::Mars, slot, control: Control::Controlled(Seat(0)), modules, colonists: 8, queue, grid_failed: false, founded_turn: 1, in_orbit: false });
            g.seats[0].archive_fund = match point {
                0 => (research as f64 * g.tables.archive.banked_before_built) as i64,
                1 => research / 2,
                _ => research,
            };
            g.seats[0].stockpile.materials = 120;
            g.seats[0].stockpile.energy = 60;
            ARCHIVE_COLONY.with(|c| c.set(Some(id)));
        }
        // `observatory:<n>` (a building aid, ticket #80): seat 0 gets a Colony on Mars with three
        // Habitats, a Generator, a Mine and an Observatory, n Colonists living there, and its card
        // opens in the Mars picture so the Observatory's line and the build button can be seen.
        if let Some(n) = std::env::args().find_map(|a| a.strip_prefix("observatory:").and_then(|v| v.parse::<u32>().ok())) {
            let slot = g.free_slots_on(BodyId::Mars).first().copied().unwrap_or(0);
            let id = ColonyId(g.fresh_id());
            let mut modules = vec![
                Module::new(ModuleKind::Habitat),
                Module::new(ModuleKind::Habitat),
                Module::new(ModuleKind::Habitat),
                Module::new(ModuleKind::Generator),
                Module::new(ModuleKind::Mine),
                Module::new(ModuleKind::Observatory),
            ];
            // `post:1` (ticket #90): a Trade Post there too, so its network line can be pictured.
            if std::env::args().any(|a| a == "post:1") {
                modules.push(Module::new(ModuleKind::TradePost));
            }
            g.colonies.push(Colony { id, body: BodyId::Mars, slot, control: Control::Controlled(Seat(0)), modules, colonists: n, queue: Vec::new(), grid_failed: false, founded_turn: 1, in_orbit: false });
            g.seats[0].stockpile.materials = 120;
            g.seats[0].stockpile.energy = 60;
            ARCHIVE_COLONY.with(|c| c.set(Some(id)));
        }
        // `idle:1` (a building aid, ticket #82): a Factory and a Research Lab stand mothballed in
        // seat 0's start state, so a Custodian's Mars card (with `observatory:<n>`) shows its Mine
        // and Observatory doubled by Production Moved.
        if std::env::args().any(|a| a == "idle:1")
            && let Some(sid) = g.controlled_states(Seat(0)).first().copied()
        {
            for k in [FacilityKind::Factory, FacilityKind::ResearchLab] {
                let mut f = Facility::new(k);
                f.mothballed = true;
                g.state_mut(sid).facilities.push(f);
            }
        }
        // `antarctic:2` (a building aid, ticket #85): the ice open, seat 0's start state sends four
        // Emigrants to each of the first two Antarctic slots on consecutive turns, so the last
        // Resolution's founding Moment reads "their second Colony in Antarctica" (with `moment:colony`).
        if let Some(n) = std::env::args().find_map(|a| a.strip_prefix("antarctic:").and_then(|v| v.parse::<usize>().ok()))
            && let Some(sid) = g.controlled_states(Seat(0)).first().copied()
        {
            g.antarctica_open = true;
            g.state_mut(sid).emigrants = 4 * n as u32;
            let slots = g.free_slots_on(BodyId::Earth);
            for slot in slots.into_iter().take(n) {
                let send = Order::SendToAntarctica { state: sid, n: 4, into: UnloadTarget::Slot(BodyId::Earth, slot) };
                g.commit_orders(Seat(0), std::slice::from_ref(&send));
                g.resolution_phase();
                g.turn += 1;
            }
            g.report.moments.clear();
            g.resolution_phase();
        }
        // `crowded:1` (a building aid, ticket #86): the world at +2.6 C, and seat 0's Colony Ship
        // arrives at the Moon with eight aboard, four beyond its capacity; the turn is rerun on the
        // game's own dice until someone dies, so `moment:lost` has a Moment to show.
        if std::env::args().any(|a| a == "crowded:1") {
            g.climate.temperature = 2.6;
            let id = ShipId(g.fresh_id());
            let built_turn = g.turn;
            g.ships.push(Ship {
                id,
                kind: UnitKind::ColonyShip,
                seat: Seat(0),
                damage: 0,
                at: ShipAt::Transit { from: BodyId::Earth, to: BodyId::Moon, turns_left: 1 },
                colonists: 8,
                army: None,
                stance: Stance::Hold,
                escaped: false,
                arrived_this_turn: false,
                built_turn,
                fuel: 30, slot: None,
            });
            for _ in 0..40 {
                let before = g.clone();
                g.report.moments.clear();
                g.resolution_phase();
                if g.report.moments.iter().any(|m| m.kind == MomentKind::LostInTransit) {
                    break;
                }
                *g = before;
                // Advance the dice one roll and try the same turn again.
                dying_earth_engine::combat::Dice::chance(&mut g.rng, 0.5);
            }
        }
        // `array:1` (a building aid, ticket #89): a Solar Array stands on seat 0's station over
        // Earth and its card opens in the Earth picture, so the array's line and the button show.
        if std::env::args().any(|a| a == "array:1")
            && let Some(id) = g.colonies.iter().find(|c| c.in_orbit && c.body == BodyId::Earth && c.control.director() == Some(Seat(0))).map(|c| c.id)
        {
            g.colony_mut(id).unwrap().modules.push(Module::new(ModuleKind::SolarArray));
            g.seats[0].stockpile.materials = 120;
            ARCHIVE_COLONY.with(|c| c.set(Some(id)));
        }
        // `driver:1` (a building aid, ticket #92): Efficient Transit stands, and seat 0 holds a
        // Colony on the Moon with a Mine and a Mass Driver, its card open in the Moon picture.
        if std::env::args().any(|a| a == "driver:1") {
            g.research.done.push(TechId::EfficientTransit);
            let slot = g.free_slots_on(BodyId::Moon).first().copied().unwrap_or(0);
            let id = ColonyId(g.fresh_id());
            let modules = vec![Module::new(ModuleKind::Habitat), Module::new(ModuleKind::Generator), Module::new(ModuleKind::Mine), Module::new(ModuleKind::MassDriver)];
            g.colonies.push(Colony { id, body: BodyId::Moon, slot, control: Control::Controlled(Seat(0)), modules, colonists: 4, queue: Vec::new(), grid_failed: false, founded_turn: 1, in_orbit: false });
            g.seats[0].stockpile.materials = 120;
            g.seats[0].stockpile.energy = 60;
            ARCHIVE_COLONY.with(|c| c.set(Some(id)));
        }
        // `dry:1` (a building aid, ticket #87): seat 0's Ships at Mars have one Fuel in the tank and
        // no station of theirs overhead, so the stack reads stranded and its panel says why.
        if std::env::args().any(|a| a == "dry:1") {
            for s in g.ships.iter_mut().filter(|s| s.seat == Seat(0) && s.at == ShipAt::Body(BodyId::Mars)) {
                s.fuel = 1;
            }
        }
        // `pressed:<n>` (a building aid, ticket #75): seat 0 holds North Africa (a short card) with a
        // Standing of n there, and seat 1 stands at n too, so the card's warning line shows.
        if let Some(n) = std::env::args().find_map(|a| a.strip_prefix("pressed:").and_then(|v| v.parse::<i64>().ok())) {
            let sid = StateId::NorthAfrica;
            g.take_control(sid, Seat(0));
            g.seats[0].influence.insert(Place::State(sid), n);
            g.seats[1].influence.insert(Place::State(sid), n);
        }
        // `emigrants:<n>` (a building aid, ticket #73): n Emigrants wait in seat 0's start state.
        if let Some(n) = std::env::args().find_map(|a| a.strip_prefix("emigrants:").and_then(|v| v.parse::<u32>().ok())) {
            let start = g.controlled_states(Seat(0)).first().copied();
            if let Some(sid) = start {
                g.state_mut(sid).emigrants = n;
            }
        }
        // `venture:<n>` (a building aid, ticket #72): seat 0 as the Prospectors holds n Materials in
        // the Venture Capital Fund and banks half its output.
        if let Some(n) = std::env::args().find_map(|a| a.strip_prefix("venture:").and_then(|v| v.parse::<i64>().ok()))
            && g.kind(Seat(0)) == FactionKind::Prospectors
        {
            g.seats[0].venture_fund = n;
            g.seats[0].venture_share = 0.5;
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
            g.end_turn(orders).expect("the screenshot harness picks a Tech before it drives turns");
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
        // `walls:1` (a building aid, ticket #56): East Asia with the sea already through two of its
        // coastal slots, a Factory drowned with them, a Sea Wall standing in a coastal slot and two
        // Facilities inland, so one card carries both rows and everything the ticket changed. An AI
        // game reaches that board on a turn nobody can choose, and never with a wall.
        if std::env::args().any(|a| a == "walls:1") {
            let sid = StateId::EastAsia;
            g.take_control(sid, Seat(0));
            if !g.has_tech(TechId::CoastalEngineering) {
                g.research.done.push(TechId::CoastalEngineering);
            }
            g.state_mut(sid).facilities = vec![
                Facility::in_coastal_slot(FacilityKind::Factory),
                Facility::in_coastal_slot(FacilityKind::PowerPlant),
                Facility::in_coastal_slot(FacilityKind::Refinery),
                Facility::in_coastal_slot(FacilityKind::LaunchSite),
                Facility::in_coastal_slot(FacilityKind::Bank),
                Facility::new(FacilityKind::ResearchLab),
            ];
            // The first threshold: two coastal slots gone and the oldest coastal Facility with them.
            g.apply_sea_threshold(sid, 0);
            // The state stood the Bank down to make room for the wall, as a player would.
            if let Some(i) = g.state(sid).facilities.iter().position(|f| f.kind == FacilityKind::Bank) {
                g.state_mut(sid).facilities.remove(i);
            }
            g.state_mut(sid).facilities.push(Facility::new(FacilityKind::SeaWall));
            g.seats[0].stockpile.materials = 300;
            g.seats[0].stockpile.energy = 400;
            g.seats[0].stockpile.ducats = 300;
        }
        // `found:1` (a building aid, ticket #58): seat 0 lands a loaded Colony Ship at the Moon and
        // the turn runs, so the Report carries a real founding, its headline and its Moment. An AI
        // game founds one on a turn nobody can choose. `moment:colony` implies it.
        // Ticket #85: unless `antarctic:<n>` staged a founding of its own for the Moment.
        if std::env::args().any(|a| a == "found:1" || a == "moment:colony") && !std::env::args().any(|a| a.starts_with("antarctic:")) {
            let id = ShipId(g.fresh_id());
            let built_turn = g.turn;
            g.ships.push(Ship {
                id,
                kind: UnitKind::ColonyShip,
                seat: Seat(0),
                damage: 0,
                at: ShipAt::Body(BodyId::Moon),
                colonists: 4,
                army: None,
                stance: Stance::Hold,
                escaped: false,
                arrived_this_turn: false,
                built_turn,
                fuel: 30, slot: None,
            });
            if let Some(slot) = g.free_slots_on(BodyId::Moon).first().copied() {
                let mut orders: [Vec<Order>; SEAT_COUNT] = std::array::from_fn(|_| Vec::new());
                orders[0] = vec![Order::Unload { ship: id, colonists: 4, army: false, into: UnloadTarget::Slot(BodyId::Moon, slot) }];
                g.seats[0].ai = false;
                g.end_turn(orders).expect("the screenshot harness picks a Tech before it drives turns");
                g.seats[0].ai = true;
            }
        }
        // `moment:tech` (a building aid, ticket #58): a rival seat pushes the Tech under research
        // over the line, so the Report carries a Tech Moment with the Lead, the margin and the AI's
        // pick. The turn a Tech completes is not one an aid can choose.
        if std::env::args().any(|a| a == "moment:tech") {
            if g.research.current.is_none()
                && let Some(first) = g.available_techs().first().copied()
            {
                g.pick_tech(Seat(0), first).ok();
            }
            if let Some(tech) = g.research.current {
                let cost = g.tables.tech(tech).cost;
                // A real race, then the Prospectors take it by a margin.
                g.research.progress = 0;
                g.research.contributions = [0; SEAT_COUNT];
                for (seat, share) in [(Seat(0), cost / 5), (Seat(2), cost / 5), (Seat(3), cost / 5)] {
                    g.accrue_research(seat, share);
                }
                let left = cost - g.research.progress;
                g.accrue_research(Seat(1), left);
            }
            // And a race under way again, so the top bar's four-colour bar is in the picture.
            race_spread(g);
        }
        // `race:1` (a building aid, ticket #58): the four seats each hold a share of the Tech under
        // research, so the top bar's Research race bar shows all four colours at once.
        if std::env::args().any(|a| a == "race:1") {
            race_spread(g);
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
    // `save:1` (a building aid, ticket #59): the release binary's own check that a save written to
    // disk comes back the same game. It says what it found on stdout and in `<prefix>-save.txt`,
    // because a release build has no console of its own.
    if std::env::args().any(|a| a == "save:1") {
        let line = save_round_trip(session);
        println!("{line}");
        let _ = std::fs::write(format!("{}-save.txt", session.shot_prefix), format!("{line}
"));
        if !line.starts_with("save round trip ok") {
            eprintln!("{line}");
            std::process::exit(3);
        }
    }
    // `saved:1` (a building aid, ticket #59): a real Save is taken, so the top bar carries the
    // notice it leaves and the picture shows the button as a player would have just used it.
    if std::env::args().any(|a| a == "saved:1") {
        session.save_now();
    }
    session.earth_dirty = true;
}

/// Ticket #59, a building aid: write the board to the saves folder, read it back, and say whether
/// the two are the same game.
fn save_round_trip(session: &mut Session) -> String {
    use dying_earth_engine::save::{self, SaveKind};
    let Some(game) = session.game.as_ref() else { return "save round trip FAILED: there is no game".to_string() };
    let dir = match &session.saves {
        Ok(d) => d.clone(),
        Err(e) => return format!("save round trip FAILED: {e}"),
    };
    let path = match save::save_to(&dir, game, SaveKind::Manual) {
        Ok(p) => p,
        Err(e) => return format!("save round trip FAILED: {e}"),
    };
    let loaded = match save::load_from(&path, session.tables.clone()) {
        Ok(g) => g,
        Err(e) => return format!("save round trip FAILED: {e}"),
    };
    let (a, b) = (save::to_text(game, SaveKind::Manual), save::to_text(&loaded, SaveKind::Manual));
    match (a, b) {
        (Ok(a), Ok(b)) if a == b => {
            let bytes = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
            format!("save round trip ok: turn {}, {} bytes, {}", game.turn, bytes, path.display())
        }
        (Ok(_), Ok(_)) => "save round trip FAILED: the loaded game is not the game that was saved".to_string(),
        _ => "save round trip FAILED: the board could not be written".to_string(),
    }
}

/// A building aid (ticket #58): a four-way share of the Tech under research, so the top bar's
/// Research race bar carries all four Faction colours.
fn race_spread(g: &mut Game) {
    if g.research.current.is_none()
        && let Some(first) = g.available_techs().first().copied()
    {
        g.pick_tech(Seat(0), first).ok();
    }
    let Some(tech) = g.research.current else { return };
    let cost = g.tables.tech(tech).cost;
    g.research.progress = 0;
    g.research.contributions = [0; SEAT_COUNT];
    let shares = [cost * 4 / 10, cost * 3 / 10, cost * 2 / 10, cost / 10];
    for (i, share) in shares.into_iter().enumerate() {
        g.research.contributions[i] = share;
        g.research.progress += share;
    }
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
    g.end_turn(std::array::from_fn(|_| Vec::new())).expect("the screenshot harness picks a Tech before it drives turns");
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

/// Ticket #59, a building aid (`load:1`): three saves in the folder, so a picture of the Load list
/// has something to show — a manual save of the player's own game, an autosave of it two turns on,
/// and an autosave of a game nobody sits at. The AI never takes a manual save on its own.
fn plant_saves(session: &mut Session) {
    use dying_earth_engine::save::{self, SaveKind};
    build_board(session);
    let Ok(dir) = session.saves.clone() else { return };
    if let Some(g) = &mut session.game {
        save::save_to(&dir, g, SaveKind::Manual).ok();
        g.seats[0].ai = true;
        for _ in 0..2 {
            if g.is_over() {
                break;
            }
            g.end_turn(std::array::from_fn(|_| Vec::new())).expect("the screenshot harness picks a Tech before it drives turns");
        }
        save::save_to(&dir, g, SaveKind::Autosave).ok();
    }
    let seed = session.seed.wrapping_add(1);
    let mut watched = Game::spectate(session.tables.clone(), seed);
    watched.start();
    for _ in 0..2 {
        if watched.is_over() {
            break;
        }
        watched.end_turn(std::array::from_fn(|_| Vec::new())).expect("the screenshot harness picks a Tech before it drives turns");
    }
    save::save_to(&dir, &watched, SaveKind::Autosave).ok();
}

pub fn shot_system(time: Res<Time>, mut plan: ResMut<ShotPlan>, mut session: ResMut<Session>, mut view: ResMut<ViewState>, mut commands: Commands, mut exit: MessageWriter<AppExit>) {
    if session.mode != Mode::Shot {
        return;
    }
    let t = time.elapsed_secs();
    // `saved:1` (a building aid, ticket #59): the Save notice is held up for the whole run, so the
    // picture cannot be taken in the second after it has faded.
    if let Some(text) = &plan.notice {
        session.save_notice = Some((text.clone(), crate::app::SAVE_NOTICE_SECONDS));
    }
    // `load:1` (a building aid, ticket #59): a folder with three saves in it, one of them a manual
    // save of the game in hand and two autosaves, and the title screen's Load list open over it.
    if std::env::args().any(|a| a == "load:1") && plan.load_step < 2 {
        if plan.load_step == 0 {
            plant_saves(&mut session);
            session.game = None;
            session.refresh_saves();
            session.screen = Screen::Load;
            plan.load_step = 1;
            plan.next_at = t + 2.0;
            return;
        }
        if t < plan.next_at {
            return;
        }
        let path = format!("{}-load.png", session.shot_prefix);
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
        plan.load_step = 2;
        plan.done_at = Some(t + 1.5);
        return;
    }
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
            // Ticket #109: the credits, so the picture that proves the CC BY attribution is
            // standing gets taken with every other menu picture.
            2 => session.screen = Screen::Credits,
            3 => {
                session.screen = Screen::ChooseStart { faction: FactionKind::Custodians };
                session.earth_dirty = true;
            }
            4 => {
                build_board(&mut session);
                plan.archive_colony = ARCHIVE_COLONY.with(|c| c.get());
                plan.moment = std::env::args().find_map(|a| a.strip_prefix("moment:").and_then(moment_from_id));
                // Ticket #58: `moment:<kind>` opens that Moment in the Report picture's place. Every
                // other kind is switched off for the picture, the way the Moments corner would, so
                // the named Moment is the one the turn stops for whatever else happened.
                if let Some(k) = plan.moment {
                    view.moments_on = Some(std::array::from_fn(|i| MomentKind::ALL[i] == k));
                }
                let at = plan.moment.and_then(|k| {
                    let game = session.game.as_ref()?;
                    view.moments_of(&session.tables, &game.report).iter().position(|m| m.kind == k)
                });
                view.popup = match at {
                    Some(i) => Popup::Moment(i),
                    None => Popup::Report,
                };
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
        // `roster:filter` (a building aid, not part of the spec): every roster group filtered down
        // to the rows that still want an order, which is otherwise a click and so unreachable in a
        // headless picture.
        if std::env::args().any(|a| a == "roster:filter") {
            view.roster_filter = [true; 4];
        }
        plan.tech = std::env::args().any(|a| a == "tech:1");
        plan.trade = std::env::args().any(|a| a == "trade:1");
        plan.victory = std::env::args().any(|a| a == "victory:1");
        plan.stack = std::env::args().any(|a| a == "stack:1");
        plan.hover = std::env::args().find_map(|a| a.strip_prefix("hover:").and_then(body_from_id));
        plan.look = std::env::args().find_map(|a| {
            let (lon, lat) = a.strip_prefix("look:")?.split_once(',')?;
            Some((lon.parse().ok()?, lat.parse().ok()?))
        });
        plan.climate_toggle = std::env::args().any(|a| a == "climate:toggle");
        // Ticket #59: whatever the Save left in the top bar, held up for the rest of the run.
        plan.notice = session.save_notice.as_ref().map(|(text, _)| text.clone());
        plan.no_panel = std::env::args().any(|a| a == "panel:0");
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

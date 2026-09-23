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
    /// Ticket #323 (version 0.08.8): `arm:1`, the Region whose stack is armed for the picture.
    pub arm: Option<StateId>,
    /// `hab:1` (a building aid, ticket #145; ticket #162 in version 0.07.5): seat 0's first station
    /// or Colony is SELECTED, so its card -- which carries the Module tiles since the Hab View
    /// window retired -- is in every picture.
    pub hab: bool,
    pub hab_colony: Option<ColonyId>,
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
    /// `factions:1` or `factions:<faction id>` (a building aid, ticket #203): the Faction window is
    /// open in every picture, on seat 0's page or on the page of the Faction named. A rival's page
    /// is the only way to photograph the totals-only disclosure line.
    pub faction_window: bool,
    pub faction_seat: Option<Seat>,
    /// `rulebook:1` (a building aid, ticket #203): the Faction window's rulebook header starts
    /// OPEN. It is shut by default in play, so the picture of it open has to be asked for.
    pub rulebook_open: bool,
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
        "rival" => Some(MomentKind::RivalProgress),
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
    if plan.faction_window {
        view.show_factions = true;
        view.faction_rulebook_open = plan.rulebook_open;
        if let Some(seat) = plan.faction_seat {
            view.faction_seat = seat;
        }
    }
    if plan.stack {
        view.selection = Selection::ShipStack(BodyId::Mars, Seat(0));
    }
    // Ticket #162 (version 0.07.5): `hab:1` SELECTS seat 0's first station or Colony (the ISS on a
    // fresh board), so its card and its Module tiles are in the picture; the window it used to open
    // is gone.
    if plan.hab && let Some(cid) = plan.hab_colony {
        view.selection = Selection::Colony(cid);
    }
    // `habtile:<n>` or `habtile:free` (a building aid): that tile is clicked, so the strip under the
    // grid can be photographed with a Module's figures or the build buttons in it. It stands on its
    // own now, as `slotbox:` does on a selected Region.
    if let Some(v) = std::env::args().find_map(|a| a.strip_prefix("habtile:").map(str::to_owned)) {
        view.hab_tile = if v == "free" { Some(HabTile::Free) } else { v.parse::<usize>().ok().map(HabTile::Module) };
    }
    // `arm:1` (a building aid, ticket #323): seat 0's start state has its stack armed, as a click on
    // its shield would, so the shield's ring and the outlined neighbours can be photographed; the
    // right-click itself cannot be, headless.
    if let Some(sid) = plan.arm {
        view.armed_stack = Some(sid);
        view.armed_scroll = true;
    }
    // `slotbox:<n>` or `slotbox:free` (a building aid, ticket #146): that slot box on the selected
    // Region's card is clicked, so the strip beneath the boxes can be photographed.
    view.slot_box = std::env::args().find_map(|a| a.strip_prefix("slotbox:").map(str::to_owned)).and_then(|v| if v == "free" { Some(SlotBox::Free) } else { v.parse::<usize>().ok().map(SlotBox::Facility) });
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
        // `pick:0` (a building aid, ticket #163): the opening Tech pick is LEFT UNMADE, so the top
        // bar carries its Pick a Tech button and the button can be photographed. It only makes
        // sense with no turns driven: a turn cannot end while the pick is owed.
        let leave_pick = std::env::args().any(|a| a == "pick:0");
        if !spectate && !leave_pick && let Some(first) = g.available_techs().first().copied() {
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
            // Ticket #210 (version 0.08.1): a planted Ship is named as a built one is, so a picture
            // shows what a game shows.
            let name = g.next_ship_name(kind);
            g.ships.push(Ship { id, name, kind, seat, damage: 0, at: ShipAt::Body(BodyId::Mars), colonists: 0, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn, fuel: 30, slot: None });
        }
        // `levy:1` (a building aid, ticket #282, version 0.08.5): seat 0 raises a built Army in
        // China, so the neutral neighbours -- India among them -- are threatened, and a quiet turn
        // runs so their Levies stand. With `select:southasia` India's card shows two Armies.
        if std::env::args().any(|a| a == "levy:1") {
            g.raise_army(Place::State(StateId::EastAsia), false);
            run_one_quiet_turn(g);
        }
        // `blockade:1` (a building aid, ticket #278, version 0.08.5): a Prospector Frigate sits in
        // the slot of seat 0's station over Earth on Blockade, so the station's button and card say
        // it is starved. With `hab:1` the station's card is open in the Earth picture.
        if std::env::args().any(|a| a == "blockade:1")
            && let Some(station) = g.colonies.iter().find(|c| c.in_orbit && c.body == BodyId::Earth && c.control.director() == Some(Seat(0))).cloned()
            && let Some(seat) = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Prospectors && *s != Seat(0))
        {
            let id = ShipId(g.fresh_id());
            let built_turn = g.turn;
            let name = g.next_ship_name(UnitKind::Frigate);
            g.ships.push(Ship { id, name, kind: UnitKind::Frigate, seat, damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, colonists_education: 1.0, army: None, stance: Stance::Blockade, escaped: false, arrived_this_turn: false, built_turn, fuel: 30, slot: Some(station.slot) });
        }
        // `battle:1` (a building aid): three seats bring a Frigate to Mars with Attack stances and
        // one more turn runs, so the Report carries a three-party Battle (ticket #50).
        // Ticket #279 (version 0.08.5): `battle:earth` fights it in Earth orbit instead and runs a
        // second quiet turn, so the Climate Panel's War line has last turn's Battle to show.
        let earth_battle = std::env::args().any(|a| a == "battle:earth");
        if std::env::args().any(|a| a == "battle:1") || earth_battle {
            let body = if earth_battle { BodyId::Earth } else { BodyId::Mars };
            for seat in [Seat(0), Seat(1), Seat(2)] {
                let id = ShipId(g.fresh_id());
                let built_turn = g.turn;
                let name = g.next_ship_name(UnitKind::Frigate);
                g.ships.push(Ship { id, name, kind: UnitKind::Frigate, seat, damage: 0, at: ShipAt::Body(body), colonists: 0, colonists_education: 1.0, army: None, stance: Stance::Attack, escaped: false, arrived_this_turn: false, built_turn, fuel: 30, slot: None });
            }
            for s in g.ships.iter_mut().filter(|s| s.at == ShipAt::Body(body)) {
                s.stance = Stance::Attack;
            }
            // The rivals sit still for this one turn, so their stacks are all at the Body when it runs.
            for seat in Seat::ALL.into_iter().skip(1) {
                g.seats[seat.index()].ai = false;
            }
            let mut orders: [Vec<Order>; SEAT_COUNT] = std::array::from_fn(|_| Vec::new());
            orders[0] = vec![Order::ShipStance { body, stance: Stance::Attack }];
            g.end_turn(orders).expect("the screenshot harness picks a Tech before it drives turns");
            if earth_battle {
                run_one_quiet_turn(g);
            }
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
            g.colonies.push(Colony { id, body: BodyId::Mars, slot, control: Control::Controlled(Seat(0)), modules, colonists: 8, education: 1.0, settler_education: 1.0, queue, grid_failed: false, founded_turn: 1, in_orbit: false });
            g.seats[0].archive_fund = match point {
                0 => (research as f64 * g.tables.archive.banked_before_built) as i64,
                1 => research / 2,
                _ => research,
            };
            g.seats[0].stockpile.materials = 120;
            g.seats[0].stockpile.energy = 60;
            ARCHIVE_COLONY.with(|c| c.set(Some(id)));
        }
        // `rival:1` (a building aid, ticket #261, version 0.08.4): seat 1 stands three quarters of
        // the way to its Victory Condition -- nine Colonists on the Moon of twelve, and, for the
        // Prospectors it usually is, the Fund at 2000 of 2500 -- and a quiet turn runs so the
        // rival's Moment fires and `moment:rival` can open it in the Report picture.
        if std::env::args().any(|a| a == "rival:1") {
            let rival = Seat(1);
            let slot = g.free_slots_on(BodyId::Moon).first().copied().unwrap_or(0);
            let id = ColonyId(g.fresh_id());
            let modules = vec![Module::new(ModuleKind::Habitat), Module::new(ModuleKind::Habitat), Module::new(ModuleKind::Habitat)];
            g.colonies.push(Colony { id, body: BodyId::Moon, slot, control: Control::Controlled(rival), modules, colonists: 9, education: 1.0, settler_education: 1.0, queue: Vec::new(), grid_failed: false, founded_turn: 1, in_orbit: false });
            g.seats[1].venture_fund = 2000;
            g.seats[1].stabilization_run = 3;
            run_one_quiet_turn(g);
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
            g.colonies.push(Colony { id, body: BodyId::Mars, slot, control: Control::Controlled(Seat(0)), modules, colonists: n, education: 1.0, settler_education: 1.0, queue: Vec::new(), grid_failed: false, founded_turn: 1, in_orbit: false });
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
            let name = g.next_ship_name(UnitKind::ColonyShip);
            g.ships.push(Ship {
                id,
                name,
                kind: UnitKind::ColonyShip,
                seat: Seat(0),
                damage: 0,
                at: ShipAt::Transit { from: BodyId::Earth, to: BodyId::Moon, turns_left: 1 },
                colonists: 8,
                colonists_education: 1.0,
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
            g.colonies.push(Colony { id, body: BodyId::Moon, slot, control: Control::Controlled(Seat(0)), modules, colonists: 4, education: 1.0, settler_education: 1.0, queue: Vec::new(), grid_failed: false, founded_turn: 1, in_orbit: false });
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
        // `room:1` (a building aid, ticket #141): seat 0's station over Earth has a Habitat, so the
        // lift button can be photographed on turn 1, when the station is still a bare core.
        if std::env::args().any(|a| a == "room:1")
            && let Some(id) = g.colonies.iter().find(|c| c.in_orbit && c.body == BodyId::Earth && c.control.director() == Some(Seat(0))).map(|c| c.id)
            && !g.colony(id).unwrap().modules.iter().any(|m| m.kind == ModuleKind::Habitat)
        {
            g.colony_mut(id).unwrap().modules.push(Module::new(ModuleKind::Habitat));
        }
        // `ship:1` (a building aid, ticket #193, version 0.08.0): a Colony Ship of seat 0's sits at
        // Earth, so the Region card's "Send N to Colony Ship" button can be photographed. The
        // computer almost never has one parked at Earth with Emigrants waiting -- about 1.8 Colony
        // Ships are completed a game -- which is exactly why the missing button showed up as a
        // player's complaint rather than as a figure in a sweep.
        if std::env::args().any(|a| a == "ship:1") {
            let id = ShipId(g.fresh_id());
            let turn = g.turn;
            let name = g.next_ship_name(UnitKind::ColonyShip);
            g.ships.push(Ship {
                id,
                name,
                slot: None,
                kind: UnitKind::ColonyShip,
                seat: Seat(0),
                damage: 0,
                at: ShipAt::Body(BodyId::Earth),
                colonists: 0,
                colonists_education: 1.0,
                army: None,
                stance: Stance::Hold,
                escaped: false,
                arrived_this_turn: false,
                built_turn: turn,
                fuel: g.tables.unit(UnitKind::ColonyShip).tank,
            });
        }
        // `settler:<body id>` (a building aid, ticket #258, version 0.08.4): a Colony Ship of seat 0's
        // with eight Colonists aboard sits at that Body, and that Body's picture selects the stack,
        // so the Ship card's founding buttons -- one per free slot, yields on their faces -- can be
        // photographed. Nothing else composes a loaded Colony Ship at a world with free slots.
        if let Some(body) = std::env::args().find_map(|a| a.strip_prefix("settler:").and_then(body_from_id)) {
            let id = ShipId(g.fresh_id());
            let turn = g.turn;
            let name = g.next_ship_name(UnitKind::ColonyShip);
            g.ships.push(Ship {
                id,
                name,
                slot: None,
                kind: UnitKind::ColonyShip,
                seat: Seat(0),
                damage: 0,
                at: ShipAt::Body(body),
                colonists: 8,
                colonists_education: 1.0,
                army: None,
                stance: Stance::Hold,
                escaped: false,
                arrived_this_turn: false,
                built_turn: turn,
                fuel: g.tables.unit(UnitKind::ColonyShip).tank,
            });
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
        // `threat:1` (a building aid, not part of the spec): a rival is stood within reach of every
        // Region seat 0 holds, so the Defence button in the command cluster has work to do and
        // can be photographed doing it. In an ordinary headless run all four seats are the computer
        // and the computer now defends its own holdings, so nothing is ever under threat to look at.
        if std::env::args().any(|a| a == "threat:1") {
            let margin = g.tables.influence.challenge_margin;
            let held: Vec<StateId> = g.directed_states(Seat(0));
            for (i, sid) in held.iter().enumerate() {
                let place = Place::State(*sid);
                let mine = g.seat(Seat(0)).influence.get(&place).copied().unwrap_or(0);
                let threshold = g.influence_threshold_for(Seat(1), place);
                // Each one a little further gone than the last, so the split has an order to find.
                let theirs = threshold.max(mine + margin + 5 + 4 * i as i64);
                g.seat_mut(Seat(1)).influence.insert(place, theirs);
            }
        }
        // `underway:1` (a building aid, ticket #263, version 0.08.4): seat 0 has a Factory on order
        // in its start state, a Module on order at a Colony on the Moon, and a Frigate three turns
        // out on the road to Mars, so the Faction window's Under way block has both lines to show.
        if std::env::args().any(|a| a == "underway:1") {
            let turn = g.turn;
            if let Some(sid) = g.directed_states(Seat(0)).first().copied() {
                g.state_mut(sid).queue.push(Build { item: BuildItem::Facility(FacilityKind::Factory), seat: Seat(0), due_turn: turn + 2, coastal: false });
            }
            let slot = g.free_slots_on(BodyId::Moon).first().copied().unwrap_or(0);
            let id = ColonyId(g.fresh_id());
            let queue = vec![Build { item: BuildItem::Module(ModuleKind::Mine), seat: Seat(0), due_turn: turn, coastal: false }];
            g.colonies.push(Colony { id, body: BodyId::Moon, slot, control: Control::Controlled(Seat(0)), modules: vec![Module::new(ModuleKind::Habitat)], colonists: 4, education: 1.0, settler_education: 1.0, queue, grid_failed: false, founded_turn: 1, in_orbit: false });
            let sid = ShipId(g.fresh_id());
            let name = g.next_ship_name(UnitKind::Frigate);
            g.ships.push(Ship {
                id: sid,
                name,
                slot: None,
                kind: UnitKind::Frigate,
                seat: Seat(0),
                damage: 0,
                at: ShipAt::Transit { from: BodyId::Earth, to: BodyId::Mars, turns_left: 3 },
                colonists: 0,
                colonists_education: 1.0,
                army: None,
                stance: Stance::Hold,
                escaped: false,
                arrived_this_turn: false,
                built_turn: turn,
                fuel: g.tables.unit(UnitKind::Frigate).tank,
            });
        }
        // `challenger:1` (a building aid, ticket #262, version 0.08.4): a rival stands on seat 0's
        // start state, well short of its price, so the challenger line on the held card has a name
        // and two figures to show. `threat:1` puts a rival OVER the price; this one keeps it under.
        if std::env::args().any(|a| a == "challenger:1")
            && let Some(sid) = g.directed_states(Seat(0)).first().copied()
        {
            let place = Place::State(sid);
            let price = g.influence_needed_for(Seat(1), place);
            g.seat_mut(Seat(1)).influence.insert(place, (price - 23).max(1));
        }
        // Ticket #127 (version 0.07.2): `attend:1` (a building aid, not part of the spec) turns the
        // standing order on for seat 0, so the side panel places an Influence order and a roster
        // ring can be photographed FILLED. Since ticket #134 (version 0.07.3) the standing order is
        // Max, on seat 0's start state.
        if std::env::args().any(|a| a == "attend:1")
            && let Some(home) = g.controlled_states(Seat(0)).first().copied()
        {
            g.seat_mut(Seat(0)).max_standing = Some(Place::State(home));
        }
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
        // Custodian in seat 0 takes every Region and fills each with its cap of Scrubbers, so
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
        // Ticket #276 (version 0.08.5): `walls:2` is the same board one rise on, the wall standing
        // through the +2.3 threshold and the coast reaching one slot further in behind it.
        if std::env::args().any(|a| a == "walls:1" || a == "walls:2") {
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
            // Ticket #257 (version 0.08.4): the wall stands through a rise now, so the aid's wall
            // has held one, and its row says what that costs.
            let mut wall = Facility::new(FacilityKind::SeaWall);
            wall.rises_held = 1;
            g.state_mut(sid).facilities.push(wall);
            if std::env::args().any(|a| a == "walls:2") {
                g.apply_sea_threshold(sid, 1);
            }
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
            let name = g.next_ship_name(UnitKind::ColonyShip);
            g.ships.push(Ship {
                id,
                name,
                kind: UnitKind::ColonyShip,
                seat: Seat(0),
                damage: 0,
                at: ShipAt::Body(BodyId::Moon),
                colonists: 4, colonists_education: 1.0,
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
        // `loaded:1` (a building aid, ticket #218 in version 0.08.2): seat 0 has a LOADED Colony
        // Ship sitting at the Moon and the turn does NOT run, so the founding buttons are on screen
        // to be photographed. `found:1` cannot serve: it unloads and ends the turn, so by the time a
        // picture is taken the Colony already stands and the buttons have gone.
        if std::env::args().any(|a| a == "loaded:1") {
            let id = ShipId(g.fresh_id());
            let built_turn = g.turn;
            let name = g.next_ship_name(UnitKind::ColonyShip);
            g.ships.push(Ship {
                id,
                name,
                kind: UnitKind::ColonyShip,
                seat: Seat(0),
                damage: 0,
                at: ShipAt::Body(BodyId::Moon),
                colonists: 4,
                colonists_education: 1.0,
                army: None,
                stance: Stance::Hold,
                escaped: false,
                arrived_this_turn: false,
                built_turn,
                fuel: 30,
                slot: None,
            });
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
        // `tints:1` (a building aid): one Region per seat on the face the Earth picture shows,
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
    // `order:<facility kind>` and `morder:<module kind>` (building aids, ticket #291, version
    // 0.08.6): an order is PLACED and left pending -- a Facility in seat 0's start state, a Module
    // on seat 0's first station over Earth -- so the box that shows an ordered building before End
    // Turn can be photographed. The kind is the enum name, case-insensitive, as `select:` takes it.
    if let Some(name) = std::env::args().find_map(|a| a.strip_prefix("order:").map(str::to_owned))
        && let Some(kind) = FacilityKind::ALL.into_iter().find(|k| format!("{k:?}").eq_ignore_ascii_case(&name))
        && let Some(sid) = session.game.as_ref().and_then(|g| g.directed_states(Seat(0)).first().copied())
        && !session.place(Order::BuildFacility { state: sid, kind })
    {
        eprintln!("order:{name} was refused");
        std::process::exit(3);
    }
    if let Some(name) = std::env::args().find_map(|a| a.strip_prefix("morder:").map(str::to_owned))
        && let Some(kind) = ModuleKind::ALL.into_iter().find(|k| format!("{k:?}").eq_ignore_ascii_case(&name))
        && let Some(cid) = session.game.as_ref().and_then(|g| g.colonies.iter().find(|c| c.in_orbit && c.body == BodyId::Earth && c.control.director() == Some(Seat(0))).map(|c| c.id))
        && !session.place(Order::BuildModule { colony: cid, kind })
    {
        eprintln!("morder:{name} was refused");
        std::process::exit(3);
    }
    // `commit:1` (a building aid, ticket #291): the turn is ended WITH the orders the aids above
    // placed, the way the End Turn button ends it, so a building under way -- the turn after the
    // order, with its turns to go -- can be photographed. `turns:<n>` cannot do this: it hands
    // seat 0 to the computer, which throws the placed orders away.
    if std::env::args().any(|a| a == "commit:1") {
        session.end_turn();
    }
    // `army:1` (a building aid, ticket #309): a raised Army of seat 0's stands in its start state,
    // so the march buttons and their hovers can be photographed; since ticket #302 a Region's own
    // Army did not march until ticket #321, so a fresh board had no march buttons at all.
    if std::env::args().any(|a| a == "army:1")
        && let Some(g) = session.game.as_mut()
        && let Some(sid) = g.directed_states(Seat(0)).first().copied()
    {
        g.raise_army(Place::State(sid), false);
    }
    // `passage:1` (a building aid, ticket #320): a Passage Accord stands between seat 0 and seat 1,
    // so a partner's Region reads "move to" on seat 0's card and its hover says why.
    if std::env::args().any(|a| a == "passage:1")
        && let Some(g) = session.game.as_mut()
    {
        let _ = g.strike_accord(Seat(0), Seat(1), vec![Term::Passage]);
    }
    // `battle:region` (a building aid, ticket #311): seat 1 takes the first neighbour of seat 0's
    // start state and raises an Army there; seat 0 raises one at home and marches on it; the turn
    // runs, so the Orders phase that follows carries a ground Battle in the Report, a ring on the
    // map, and outlined shields with damage. The rivals sit still for the one turn, as `battle:1`
    // has them do.
    if std::env::args().any(|a| a == "battle:region")
        && let Some(g) = session.game.as_mut()
        && let Some(sid) = g.directed_states(Seat(0)).first().copied()
        && let Some(n) = g.tables.state(sid).neighbours.first().copied()
    {
        g.take_control(n, Seat(1));
        g.raise_army(Place::State(n), false);
        let mine = g.raise_army(Place::State(sid), false);
        for seat in Seat::ALL.into_iter().skip(1) {
            g.seats[seat.index()].ai = false;
        }
        let mut orders: [Vec<Order>; SEAT_COUNT] = std::array::from_fn(|_| Vec::new());
        orders[0] = vec![Order::MoveArmy { army: mine, to: n }];
        g.end_turn(orders).expect("the screenshot harness picks a Tech before it drives turns");
        for seat in Seat::ALL.into_iter().skip(1) {
            g.seats[seat.index()].ai = true;
        }
    }
    // `threat:1` (a building aid, ticket #310): seat 1 takes the first neighbour of seat 0's start
    // state and raises an Army there, so the threat line on seat 0's card can be photographed.
    if std::env::args().any(|a| a == "threat:1")
        && let Some(g) = session.game.as_mut()
        && let Some(sid) = g.directed_states(Seat(0)).first().copied()
        && let Some(n) = g.tables.state(sid).neighbours.first().copied()
    {
        g.take_control(n, Seat(1));
        g.raise_army(Place::State(n), false);
    }
    // `offline:1` (a building aid, ticket #307): the first Facility of seat 0's start state and the
    // first Module beyond the Core on seat 0's first station over Earth are put offline as an
    // Energy shortfall would put them, so the offline tile can be photographed. After `commit:1`,
    // so a Module built by `morder:` is standing to be switched off.
    if std::env::args().any(|a| a == "offline:1")
        && let Some(g) = session.game.as_mut()
    {
        if let Some(sid) = g.directed_states(Seat(0)).first().copied()
            && let Some(f) = g.state_mut(sid).facilities.first_mut()
        {
            f.online = false;
        }
        if let Some(c) = g.colonies.iter_mut().find(|c| c.in_orbit && c.body == BodyId::Earth && c.control.director() == Some(Seat(0)))
            && let Some(m) = c.modules.iter_mut().find(|m| m.kind != ModuleKind::Core)
        {
            m.online = false;
        }
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

/// `tutorialtick:1` (a building aid, ticket #174, not part of the spec): the `Play Tutorial` tick at
/// the foot of the Custodians' card stands ticked in the picture, since a headless run cannot click
/// it. Off by default, as the screen opens for a player.
fn tutorial_tick() -> bool {
    std::env::args().any(|a| a == "tutorialtick:1")
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
            1 => {
                session.screen = Screen::ChooseFaction;
                session.tutorial_ticked = tutorial_tick();
            }
            // Ticket #109: the credits, so the picture that proves the CC BY attribution is
            // standing gets taken with every other menu picture.
            2 => session.screen = Screen::Credits,
            3 => {
                session.screen = Screen::ChooseStart { faction: FactionKind::Custodians };
                session.earth_dirty = true;
                // `start:<state>` (a building aid, not part of the spec): the start screen with that
                // Region already chosen, since a click cannot be made in a headless picture. The
                // enum name, case-insensitive, as `select:` takes it.
                if let Some(sid) = std::env::args().find_map(|a| a.strip_prefix("start:").map(str::to_owned)).and_then(|name| StateId::ALL.into_iter().find(|s| format!("{s:?}").eq_ignore_ascii_case(&name))) {
                    view.start_selected = Some(sid);
                    view.start_aimed = Some(FactionKind::Custodians);
                    let (lon, lat) = crate::geo::state_lonlat(sid);
                    view.yaw = crate::geo::yaw_facing(lon, lat);
                    view.spin = view.yaw;
                    view.pitch = lat.to_radians().clamp(-1.3, 1.3);
                    view.zoom = 1.0;
                    view.start_grabbed = true;
                }
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
                // `tutorial:<turn>` (a building aid, ticket #169): the tutorial's note for that turn
                // stands in the Report picture's place, so a note can be photographed. A tutorial
                // game is an ordinary game otherwise, so nothing else about the board changes.
                if let Some(turn) = std::env::args().find_map(|a| a.strip_prefix("tutorial:").and_then(|v| v.parse::<u32>().ok())) {
                    session.tutorial = true;
                    if let Some(g) = session.game.as_mut() {
                        g.turn = turn;
                    }
                    view.popup = Popup::Tutorial;
                }
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
            session.tutorial_ticked = tutorial_tick();
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
        // `select:<state id>` (a building aid) opens that Region's card in the Earth picture.
        plan.select = std::env::args().find_map(|a| a.strip_prefix("select:").map(str::to_owned));
        plan.arm = std::env::args().any(|a| a == "arm:1").then(|| session.game.as_ref().and_then(|g| g.directed_states(Seat(0)).first().copied())).flatten();
        plan.tech = std::env::args().any(|a| a == "tech:1");
        plan.hab = std::env::args().any(|a| a == "hab:1");
        // Ticket #204 (version 0.08.1): `hab:ground` picks seat 0's first Colony ON a surface
        // rather than its first of any kind, which on every board so far is the station over Earth.
        // It is how the sea half of the receiver's door gets photographed.
        let ground = std::env::args().any(|a| a == "hab:ground");
        if ground {
            plan.hab = true;
        }
        plan.hab_colony = session
            .game
            .as_ref()
            .and_then(|g| g.colonies.iter().find(|c| c.control.director() == Some(Seat(0)) && (!ground || !c.in_orbit)).map(|c| c.id));
        plan.trade = std::env::args().any(|a| a == "trade:1");
        plan.victory = std::env::args().any(|a| a == "victory:1");
        // Ticket #203: `factions:1` for seat 0's page, `factions:archivists` for that Faction's.
        if let Some(v) = std::env::args().find_map(|a| a.strip_prefix("factions:").map(str::to_owned)) {
            plan.faction_window = true;
            plan.rulebook_open = std::env::args().any(|a| a == "rulebook:1");
            if let Some(kind) = FactionKind::from_id(&v) {
                plan.faction_seat = session.game.as_ref().and_then(|g| Seat::ALL.into_iter().find(|s| g.kind(*s) == kind));
            }
        }
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
        // `site:<body id>,<slot>` (a building aid, ticket #258): that Body's picture opens the empty
        // Colony Slot's panel, the one place the yields were still in words. `settler:<body id>`
        // (the same ticket): that Body's picture selects seat 0's Ship stack there.
        if let View::Surface(body) = v {
            if let Some((b, slot)) = std::env::args().find_map(|a| a.strip_prefix("site:").and_then(|v| v.split_once(',')).and_then(|(b, n)| Some((body_from_id(b)?, n.parse::<u32>().ok()?))))
                && b == body
            {
                view.selection = Selection::Slot(body, slot);
            }
            if std::env::args().any(|a| a.strip_prefix("settler:").and_then(body_from_id) == Some(body)) {
                view.selection = Selection::ShipStack(body, Seat(0));
            }
        }
        plan.next_at = t + 2.5;
    }
}

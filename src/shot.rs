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
    /// Ticket #343 (version 0.09.1): or `stack:<body id>` -- `stack:earth` -- for the stack at
    /// another Body, since the Launch door is photographed over EARTH, which is the one Body a
    /// Bombard could never be given over and so the one this aid had never needed to reach.
    pub stack: Option<BodyId>,
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
    /// Ticket #335 (version 0.09.0), a building aid (`scroll:transits`, `scroll:orbits`; ticket
    /// #346 added `scroll:tanks`): the Ship stack card scrolls to that block, which a headless
    /// capture cannot do with a scrollbar.
    pub stack_scroll: Option<StackBlock>,
    /// Ticket #337 (version 0.09.0), a building aid (`card:<event id>`): that Choice Card is the
    /// turn's question, so its modal stands in every picture of the run. The deck is stacked with
    /// the card and the real Question phase is run, so what is asked is asked the way a game asks
    /// it. `cardshut:1` sets the modal aside -- which only a picture may do -- so the board behind
    /// it can be photographed with End Turn greyed; the `tip:` aid forces that button hover.
    pub card: bool,
    pub card_shut: bool,
    /// Ticket #338 (version 0.09.0), a building aid (`chronicle:1`): **the chronicle page**, the
    /// page the game-over box opens, which the game-over screen had never had a name for. It fills
    /// the window, so the run takes ONE picture of it rather than seven copies of the same page.
    /// Wants `turns:<n>` enough to play the board out, or the page reports a game still on.
    pub chronicle: bool,
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
        // Ticket #345 (version 0.09.1): a Body settled for the first time.
        "first" => Some(MomentKind::FirstToABody),
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
    if let Some(body) = plan.stack {
        view.selection = Selection::ShipStack(body, Seat(0));
    }
    view.stack_scroll = plan.stack_scroll;
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
    // Ticket #337 (version 0.09.0): the turn's Choice Card stands in the picture, unless the aid
    // has set it aside for a picture of the board behind it. Applied at every view change, since
    // the modal is raised again by the interface the moment nothing else is up.
    view.card_aside = plan.card_shut;
    if plan.card && !plan.card_shut {
        view.popup = Popup::Card;
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

// Ticket #338 (version 0.09.0): `chronicle:1` photographs a PAGE and not a map, so the run takes
// one picture and stops. The View beside it is never seen -- the page covers the window -- and is
// only there so the capture loop needs no second shape.
const CHRONICLE: [(&str, View); 1] = [("chronicle", View::Surface(BodyId::Earth))];

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
            g.ships.push(Ship { id, name, kind, seat, damage: 0, at: ShipAt::Body(BodyId::Mars), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn, fuel: 30, slot: None });
        }
        // `levy:1` (a building aid, ticket #282, version 0.08.5): seat 0 raises a built Army in
        // China, so the neutral neighbours -- India among them -- are threatened, and a quiet turn
        // runs so their Levies stand. With `select:southasia` India's card shows two Armies.
        if std::env::args().any(|a| a == "levy:1") {
            g.raise_army(Place::State(StateId::EastAsia), false);
            run_one_quiet_turn(g);
        }
        // `orbits:1` (a building aid, ticket #335, version 0.09.0): **three of Mars's four orbits
        // are occupied at once.** Seat 0 holds the station in slot 0 (Mars Base Camp) and seat 1 the
        // station in slot 1 (Ares); seat 0 keeps its Frigate in LOW ORBIT and gains a Colony Ship at
        // its own station's ring, seat 1 gains a Frigate at seat 0's station's ring on Blockade, and
        // seat 2's Carrier stays in low orbit. So one Surface Map picture carries low orbit's ring
        // with stacks on it, two station rings with stacks on them, and an orbit band with a row per
        // stack per orbit; and the stack card's Change orbit door has three other orbits to offer.
        // It runs BEFORE `battle:1`, so that aid's Attack stances make a Battle in each orbit that
        // has two parties in it, which is the only way to photograph two Battle marks at one Body.
        if std::env::args().any(|a| a == "orbits:1") {
            for (slot, seat) in [(0u32, Seat(0)), (1, Seat(1))] {
                if g.station_at(BodyId::Mars, slot).is_none() {
                    let id = ColonyId(g.fresh_id());
                    let modules = vec![Module::new(ModuleKind::Core), Module::new(ModuleKind::Habitat)];
                    g.colonies.push(Colony { id, body: BodyId::Mars, slot, control: Control::Controlled(seat), modules, colonists: 2, education: 1.0, settler_education: 1.0, queue: Vec::new(), grid_failed: false, founded_turn: 1, in_orbit: true });
                }
            }
            for (seat, kind, slot, stance) in [(Seat(0), UnitKind::ColonyShip, Some(0u32), Stance::Hold), (Seat(1), UnitKind::Frigate, Some(0), Stance::Blockade)] {
                let id = ShipId(g.fresh_id());
                let built_turn = g.turn;
                let name = g.next_ship_name(kind);
                g.ships.push(Ship { id, name, kind, seat, damage: 0, at: ShipAt::Body(BodyId::Mars), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance, escaped: false, arrived_this_turn: false, built_turn, fuel: 30, slot });
            }
            // And, over EARTH, a Colony Ship of seat 0's at the ISS's own ring rather than in low
            // orbit. Ticket #335 photographed the Region card's lift door SHUT on it; since ticket
            // #357 (version 0.09.1) a Launch Site reaches any orbit of Earth, so the door is open.
            if let Some(slot) = g.colonies.iter().find(|c| c.in_orbit && c.body == BodyId::Earth && c.control.director() == Some(Seat(0))).map(|c| c.slot) {
                let id = ShipId(g.fresh_id());
                let built_turn = g.turn;
                let name = g.next_ship_name(UnitKind::ColonyShip);
                g.ships.push(Ship { id, name, kind: UnitKind::ColonyShip, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn, fuel: 30, slot: Some(slot) });
            }
            g.seats[0].stockpile.materials = 200;
            g.seats[0].stockpile.energy = 80;
            g.seats[0].stockpile.fuel = 60;
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
            g.ships.push(Ship { id, name, kind: UnitKind::Frigate, seat, damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Blockade, escaped: false, arrived_this_turn: false, built_turn, fuel: 30, slot: Some(station.slot) });
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
                g.ships.push(Ship { id, name, kind: UnitKind::Frigate, seat, damage: 0, at: ShipAt::Body(body), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Attack, escaped: false, arrived_this_turn: false, built_turn, fuel: 30, slot: None });
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
            refuse_any_card(g);
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
        // order with a quarter of the Research banked; 1, the Module standing and the fund half
        // paid; 2, complete. An AI Archivist rarely has any of that in six turns.
        // Ticket #347 (version 0.09.1): point 0's quarter was `banked_before_built`, the cap the
        // fund sat at until the Module stood. The field is gone and the fund now runs to the whole
        // figure from the first turn; a quarter of it is kept here only as a part-paid fund to
        // photograph, and the fund line it draws no longer names a cap.
        if let Some(point) = std::env::args().find_map(|a| a.strip_prefix("archive:").and_then(|v| v.parse::<u32>().ok())) {
            let research = g.tables.archive.research;
            let slot = g.free_slots_on(BodyId::Mars).first().copied().unwrap_or(0);
            let id = ColonyId(g.fresh_id());
            let mut modules = vec![Module::new(ModuleKind::Habitat), Module::new(ModuleKind::Habitat), Module::new(ModuleKind::Generator), Module::new(ModuleKind::Mine)];
            let mut queue = Vec::new();
            if point == 0 {
                queue.push(Build { item: BuildItem::Module(ModuleKind::Archive), seat: Seat(0), widgets: 12, done: 4, coastal: false });
            } else {
                modules.push(Module::new(ModuleKind::Archive));
            }
            g.colonies.push(Colony { id, body: BodyId::Mars, slot, control: Control::Controlled(Seat(0)), modules, colonists: 8, education: 1.0, settler_education: 1.0, queue, grid_failed: false, founded_turn: 1, in_orbit: false });
            g.seats[0].archive_fund = match point {
                0 => research / 4,
                1 => research / 2,
                _ => research,
            };
            g.seats[0].stockpile.materials = 120;
            g.seats[0].stockpile.energy = 60;
            ARCHIVE_COLONY.with(|c| c.set(Some(id)));
        }
        // `battery:1` (a building aid, ticket #324, version 0.08.8): seat 0 gets a Colony on Mars
        // with a Battery standing, two hits on it, and seat 1 a Frigate in Mars orbit on Hold; so
        // the Mars band reads the Battery's row and why nobody holds Orbital Control, and the
        // Colony's card (`hab:ground`) shows the tile, its hover and the Repair buttons.
        if std::env::args().any(|a| a == "battery:1") {
            let slot = g.free_slots_on(BodyId::Mars).first().copied().unwrap_or(0);
            let id = ColonyId(g.fresh_id());
            let mut battery = Module::new(ModuleKind::Battery);
            battery.damage = 2;
            let modules = vec![Module::new(ModuleKind::Habitat), Module::new(ModuleKind::Generator), Module::new(ModuleKind::Mine), battery];
            g.colonies.push(Colony { id, body: BodyId::Mars, slot, control: Control::Controlled(Seat(0)), modules, colonists: 4, education: 1.0, settler_education: 1.0, queue: Vec::new(), grid_failed: false, founded_turn: 1, in_orbit: false });
            let sid = ShipId(g.fresh_id());
            let name = g.next_ship_name(UnitKind::Frigate);
            let built_turn = g.turn;
            g.ships.push(Ship { id: sid, name, kind: UnitKind::Frigate, seat: Seat(1), damage: 0, at: ShipAt::Body(BodyId::Mars), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn, fuel: 30, slot: None });
            g.seats[0].stockpile.materials = 120;
            g.seats[0].stockpile.energy = 60;
        }
        // `barracks:1` (a building aid, ticket #334, version 0.09.0): seat 0 gets a Colony on the
        // Moon with a Barracks standing and four Colonists, and the Materials for a raise; so the
        // Colony's card (`hab:ground`) shows the Build Army button and its hover, which names the
        // Colonist a raise takes.
        if std::env::args().any(|a| a == "barracks:1") {
            let slot = g.free_slots_on(BodyId::Moon).first().copied().unwrap_or(0);
            let id = ColonyId(g.fresh_id());
            let modules = vec![Module::new(ModuleKind::Habitat), Module::new(ModuleKind::Generator), Module::new(ModuleKind::Mine), Module::new(ModuleKind::Barracks), Module::new(ModuleKind::Core)];
            g.colonies.push(Colony { id, body: BodyId::Moon, slot, control: Control::Controlled(Seat(0)), modules, colonists: 4, education: 1.0, settler_education: 1.0, queue: Vec::new(), grid_failed: false, founded_turn: 1, in_orbit: false });
            g.seats[0].stockpile.materials = 120;
            g.seats[0].stockpile.energy = 60;
        }
        // `refuel:1` (a building aid, ticket #325, version 0.08.8): seat 1 holds a station over
        // Mars, seat 0 a Frigate in Mars orbit with an empty tank and no station of its own there,
        // and a Refuel Accord stands between them; so the Ship card (`stack:1`) shows the Refuel
        // button at a partner's station and its hover.
        if std::env::args().any(|a| a == "refuel:1") {
            let id = ColonyId(g.fresh_id());
            let modules = vec![Module::new(ModuleKind::Habitat)];
            g.colonies.push(Colony { id, body: BodyId::Mars, slot: 0, control: Control::Controlled(Seat(1)), modules, colonists: 2, education: 1.0, settler_education: 1.0, queue: Vec::new(), grid_failed: false, founded_turn: 1, in_orbit: true });
            let sid = ShipId(g.fresh_id());
            let name = g.next_ship_name(UnitKind::Frigate);
            let built_turn = g.turn;
            g.ships.push(Ship { id: sid, name, kind: UnitKind::Frigate, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Mars), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn, fuel: 0, slot: None });
            let _ = g.strike_accord(Seat(0), Seat(1), vec![Term::NonAggression, Term::Refuel]);
            g.seats[0].stockpile.fuel = 40;
        }
        // `bombard:1` (a building aid, ticket #328, version 0.08.8): seat 1 holds a Colony on the
        // ground of Mars and seat 0 a Battleship in Mars orbit, holding the orbit; so the Ship card
        // (`stack:1`) shows the Bombard button and its hover. With `bombard:order` the Bombard is
        // placed as well, for `commit:1` to resolve.
        if std::env::args().any(|a| a == "bombard:1") {
            let slot = g.free_slots_on(BodyId::Mars).first().copied().unwrap_or(0);
            let id = ColonyId(g.fresh_id());
            let modules = vec![Module::new(ModuleKind::Habitat), Module::new(ModuleKind::Habitat), Module::new(ModuleKind::Mine), Module::new(ModuleKind::Generator)];
            g.colonies.push(Colony { id, body: BodyId::Mars, slot, control: Control::Controlled(Seat(1)), modules, colonists: 8, education: 1.0, settler_education: 1.0, queue: Vec::new(), grid_failed: false, founded_turn: 1, in_orbit: false });
            g.ships.retain(|s| !(s.at == ShipAt::Body(BodyId::Mars) && s.kind.is_warship() && s.seat != Seat(0)));
            let sid = ShipId(g.fresh_id());
            let name = g.next_ship_name(UnitKind::Battleship);
            let built_turn = g.turn;
            g.ships.push(Ship { id: sid, name, kind: UnitKind::Battleship, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Mars), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn, fuel: 30, slot: None });
        }
        // `nuke:1` (a building aid, ticket #343, version 0.09.1): **the board the Missile Carrier is
        // photographed on**, and it is EARTH's, because Earth is the one Body a Bombard is refused
        // over and a Launch is not. Seat 1 takes the first neighbour of seat 0's start Region, so a
        // Region a rival directs is a lawful target; every rival warship at Earth is cleared, as
        // `bombard:1` clears Mars's, so seat 0 holds Orbital Control of low orbit outright.
        //
        // Three hulls of seat 0's go up: a Battleship, because a Missile Carrier is NO WARSHIP and
        // holds no Orbital Control of its own -- the escort is what opens the door, which is the
        // whole shape of ticket #326's counter -- an ARMED carrier beside it in low orbit, where
        // the Launch is lawful, and a SPENT one at seat 0's own station's ring, where the Rearm is.
        // So one picture carries the live Launch, the refusal a spent hull is given, and the Rearm
        // that answers it. The Tech is granted, since a planted Ship never passed the build gate.
        if std::env::args().any(|a| a == "nuke:1") {
            if !g.has_tech(TechId::MissileTechnology) {
                g.research.done.push(TechId::MissileTechnology);
            }
            if let Some(home) = g.directed_states(Seat(0)).first().copied()
                && let Some(theirs) = g.tables.state(home).neighbours.first().copied()
            {
                g.take_control(theirs, Seat(1));
            }
            g.ships.retain(|s| !(s.at == ShipAt::Body(BodyId::Earth) && s.kind.is_warship() && s.seat != Seat(0)));
            let station = g.colonies.iter().find(|c| c.in_orbit && c.body == BodyId::Earth && c.control.director() == Some(Seat(0))).map(|c| (c.id, c.slot));
            if let Some((cid, _)) = station
                && let Some(col) = g.colony_mut(cid)
                && !col.modules.iter().any(|m| m.kind == ModuleKind::Shipyard)
            {
                col.modules.push(Module::new(ModuleKind::Shipyard));
            }
            for (kind, warhead, slot) in [(UnitKind::Battleship, false, None), (UnitKind::MissileCarrier, true, None), (UnitKind::MissileCarrier, false, station.map(|(_, s)| s))] {
                let id = ShipId(g.fresh_id());
                let built_turn = g.turn;
                let name = g.next_ship_name(kind);
                g.ships.push(Ship { id, name, kind, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, warhead, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn, fuel: 30, slot });
            }
            // Enough of each to pay for a hull and a Warhead, so the Shipyard's own Missile Carrier
            // button and the Rearm are both LIVE in the picture rather than greyed for want of
            // Fuel -- a fresh board holds 23 Fuel and every Ship in the game costs 30.
            g.seats[0].stockpile.materials = 220;
            g.seats[0].stockpile.energy = 80;
            g.seats[0].stockpile.fuel = 120;
        }
        // `first:1` and `first:lost` (building aids, ticket #345, version 0.09.1): **the board the
        // first-to-a-Body rule is photographed on.** Seat 0 lands on the MOON and takes its first,
        // and Mars, Phobos and Deimos are left with theirs unclaimed -- so one picture of the Solar
        // System Map carries a world that has been taken beside three that are still worth the
        // crossing, which is the whole argument of the rule in one frame.
        //
        // The landing is driven through `end_turn` with a real Unload order rather than by pushing a
        // Colony onto the board, because everything worth photographing here is made by the engine
        // at the founding: the record, the windfall, the Report line and the Moment. A planted
        // Colony would have none of them, and `moment:first` would open on nothing. The rivals sit
        // still for the one turn, as `battle:1` has them do, so the board the aids built stays put.
        //
        // `first:lost` then hands the Colony to seat 1. The +1 is the FOUNDER'S and sleeps while
        // somebody else directs the place, and that second reading is a picture of its own: no game
        // gives both boards at once, since a Colony has one holder.
        if let Some(mode) = std::env::args().find_map(|a| a.strip_prefix("first:").map(str::to_owned)) {
            let body = BodyId::Moon;
            let slot = g.free_slots_on(body).first().copied().unwrap_or(0);
            let id = ShipId(g.fresh_id());
            let built_turn = g.turn;
            let name = g.next_ship_name(UnitKind::ColonyShip);
            g.ships.push(Ship { id, name, kind: UnitKind::ColonyShip, seat: Seat(0), damage: 0, at: ShipAt::Body(body), colonists: 8, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn, fuel: 30, slot: None });
            for seat in Seat::ALL.into_iter().skip(1) {
                g.seats[seat.index()].ai = false;
            }
            let mut orders: [Vec<Order>; SEAT_COUNT] = std::array::from_fn(|_| Vec::new());
            orders[0] = vec![Order::Unload { ship: id, colonists: 8, army: false, into: UnloadTarget::Slot(body, slot) }];
            refuse_any_card(g);
            g.end_turn(orders).expect("the screenshot harness picks a Tech before it drives turns");
            for seat in Seat::ALL.into_iter().skip(1) {
                g.seats[seat.index()].ai = true;
            }
            if g.first_at(body).is_none() {
                eprintln!("first:{mode} founded nothing on {body:?}, so no first was claimed");
                std::process::exit(3);
            }
            if mode == "lost"
                && let Some((_, cid)) = g.first_at(body)
                && let Some(col) = g.colony_mut(cid)
            {
                col.control = Control::Controlled(Seat(1));
            }
            g.seats[0].stockpile.materials = 120;
            g.seats[0].stockpile.energy = 60;
        }
        // `drybar:1` (a building aid, ticket #346, version 0.09.1): **the board the Battle bar is
        // photographed on.** Seat 0 keeps a DRY Frigate -- 1 Fuel, under the 2 a Battle charges --
        // and a full Battleship in the SAME orbit over Mars, so one picture carries both readings
        // of a Ship row side by side: the hull that has lost the orbit, the blockade, the intercept
        // and half its strength, and the hull beside it that has lost none of them. A Frigate of
        // seat 1's sits in that orbit with a full tank, so the Attack door, the odds line and the
        // Fuel-cost line all have a rival to speak of, and seat 0's Blockade and Intercept are
        // live rather than greyed -- the Battleship holds the bar for the whole stack.
        //
        // Nothing an AI game plays composes this. The computer does not fight in orbit at all
        // (ticket #355 is what would change that), and the measured sweep behind this ticket found
        // ONE orbital Battle in eighty games.
        //
        // `drybar:all` dries the Battleship too, so NOTHING of seat 0's at the Body holds the bar.
        // That is the board the two NEW refusals stand on -- a Blockade and an Intercept greyed for
        // a reason that lives in a tank -- and no other aid can reach them.
        if let Some(mode) = std::env::args().find_map(|a| a.strip_prefix("drybar:").map(str::to_owned)) {
            let escort = if mode == "all" { 1i64 } else { 30 };
            g.ships.retain(|s| s.at != ShipAt::Body(BodyId::Mars));
            for (seat, kind, fuel) in [(Seat(0), UnitKind::Frigate, 1i64), (Seat(0), UnitKind::Battleship, escort), (Seat(1), UnitKind::Frigate, 30)] {
                let id = ShipId(g.fresh_id());
                let built_turn = g.turn;
                let name = g.next_ship_name(kind);
                g.ships.push(Ship { id, name, kind, seat, damage: 0, at: ShipAt::Body(BodyId::Mars), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn, fuel, slot: None });
            }
            g.seats[0].stockpile.fuel = 60;
        }
        // `eye:1` and `eye:0` (building aids, ticket #339, version 0.09.0): **the pair of boards the
        // eye is photographed on.** Seat 1 takes the first neighbour of seat 0's start Region and
        // runs two Facilities in it, and holds a Colony on the Moon with three Modules; with
        // `eye:1` seat 0 also has a working Embassy at home and a Colony of its own at the Moon
        // with a working Relay, which are the two eyes. `eye:0` builds neither, so the same two
        // rival cards can be photographed with nothing watching them and the difference is the
        // block and nothing else. An AI game gives no board where one Faction has an Embassy and a
        // rival a built-up Region next door on a turn anybody can choose.
        if let Some(watching) = std::env::args().find_map(|a| match a.strip_prefix("eye:") {
            Some("1") => Some(true),
            Some("0") => Some(false),
            _ => None,
        }) {
            if let Some(home) = g.directed_states(Seat(0)).first().copied()
                && let Some(theirs) = g.tables.state(home).neighbours.first().copied()
            {
                g.take_control(theirs, Seat(1));
                for kind in [FacilityKind::Factory, FacilityKind::ResearchLab] {
                    g.state_mut(theirs).facilities.push(Facility::new(kind));
                }
                if watching {
                    g.state_mut(home).facilities.push(Facility::new(FacilityKind::Embassy));
                }
            }
            let mut slots = g.free_slots_on(BodyId::Moon).into_iter();
            if let Some(slot) = slots.next() {
                let id = ColonyId(g.fresh_id());
                let modules = vec![Module::new(ModuleKind::Core), Module::new(ModuleKind::Habitat), Module::new(ModuleKind::Mine), Module::new(ModuleKind::Generator)];
                g.colonies.push(Colony { id, body: BodyId::Moon, slot, control: Control::Controlled(Seat(1)), modules, colonists: 4, education: 1.0, settler_education: 1.0, queue: Vec::new(), grid_failed: false, founded_turn: 1, in_orbit: false });
            }
            if watching && let Some(slot) = slots.next() {
                let id = ColonyId(g.fresh_id());
                let modules = vec![Module::new(ModuleKind::Core), Module::new(ModuleKind::Habitat), Module::new(ModuleKind::Relay)];
                g.colonies.push(Colony { id, body: BodyId::Moon, slot, control: Control::Controlled(Seat(0)), modules, colonists: 4, education: 1.0, settler_education: 1.0, queue: Vec::new(), grid_failed: false, founded_turn: 1, in_orbit: false });
            }
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
                warhead: false,
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
        // Ticket #349 (version 0.09.1): `pressed:<n>,<theirs>` puts seat 1 at `theirs` instead, so a
        // held card NOT Pressed can be photographed beside one that is.
        if let Some((n, theirs)) = std::env::args().find_map(|a| {
            let v = a.strip_prefix("pressed:")?;
            let mut it = v.split(',').map(|x| x.parse::<i64>().ok());
            let n = it.next()??;
            Some((n, it.next().flatten().unwrap_or(n)))
        }) {
            let sid = StateId::NorthAfrica;
            g.take_control(sid, Seat(0));
            g.seats[0].influence.insert(Place::State(sid), n);
            g.seats[1].influence.insert(Place::State(sid), theirs);
        }
        // `short:1` (a building aid, ticket #351): seat 0's Energy store is emptied and two Scrubbers
        // and a Research Lab stand in its start Region, so the next Income is short and the top bar's
        // Energy figure is red; `tip:Next` photographs its hover.
        if std::env::args().any(|a| a == "short:1")
            && let Some(home) = g.controlled_states(Seat(0)).first().copied()
        {
            g.seats[0].stockpile.energy = 0;
            for kind in [FacilityKind::Scrubber, FacilityKind::Scrubber, FacilityKind::ResearchLab] {
                g.state_mut(home).facilities.push(Facility::new(kind));
            }
        }
        // `pressedat:<k>` (a building aid, ticket #349): seat 0 holds k more Regions nobody held, each
        // at 50 with seat 1 at 45, so the Command Cluster's Pressed list runs past its three lines.
        if let Some(k) = std::env::args().find_map(|a| a.strip_prefix("pressedat:").and_then(|v| v.parse::<usize>().ok())) {
            let free: Vec<StateId> = g.states.iter().filter(|s| s.control == Control::Neutral).map(|s| s.id).take(k).collect();
            for sid in free {
                g.take_control(sid, Seat(0));
                g.seats[0].influence.insert(Place::State(sid), 50);
                g.seats[1].influence.insert(Place::State(sid), 45);
            }
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
        // `shut:1` (a building aid, ticket #359, version 0.09.1), given with `barracks:1`: that Moon
        // Colony's Habitat and Barracks are mothballed and six live there, two more than the Core
        // alone holds -- so its card (`hab:ground`) shows the half line and the Build Army door
        // greyed with the shut Barracks' refusal. Nothing else mothballs before the first Resolution.
        if std::env::args().any(|a| a == "shut:1")
            && let Some(id) = g.colonies.iter().find(|c| !c.in_orbit && c.body == BodyId::Moon && c.control.director() == Some(Seat(0))).map(|c| c.id)
        {
            let col = g.colony_mut(id).unwrap();
            for m in col.modules.iter_mut().filter(|m| matches!(m.kind, ModuleKind::Habitat | ModuleKind::Barracks)) {
                m.mothballed = true;
            }
            col.colonists = 6;
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
                warhead: false,
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
                warhead: false,
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
                g.state_mut(sid).queue.push(Build { item: BuildItem::Facility(FacilityKind::Factory), seat: Seat(0), widgets: 4, done: 0, coastal: false });
            }
            let slot = g.free_slots_on(BodyId::Moon).first().copied().unwrap_or(0);
            let id = ColonyId(g.fresh_id());
            let queue = vec![Build { item: BuildItem::Module(ModuleKind::Mine), seat: Seat(0), widgets: 4, done: 3, coastal: false }];
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
                warhead: false,
                army: None,
                stance: Stance::Hold,
                escaped: false,
                arrived_this_turn: false,
                built_turn: turn,
                fuel: g.tables.unit(UnitKind::Frigate).tank,
            });
        }
        // `queue:1` (a building aid, ticket #332, version 0.09.0): seat 0's start Region has three
        // builds under way -- a Power Plant 3 of 8, a Research Lab 0 of 4, and a Bank 2 of 8 that
        // seat 1 began, as if the Region had lately changed hands -- and seat 0 holds a Colony on
        // the Moon with a Factory Module standing and three builds in its queue, a Habitat 2 of 4,
        // a Mine 0 of 4 and seat 1's Refinery 1 of 8, with room for one Module more; so the
        // Widgets line, the queue, the `3 of 8` tiles, the rival's cancel hover and the build
        // buttons' estimates can all be photographed. Nothing the computer plays in six turns
        // composes a queue this deep, and nothing composes a rival's build at a place of one's own.
        if std::env::args().any(|a| a == "queue:1") {
            if let Some(sid) = g.directed_states(Seat(0)).first().copied() {
                for (kind, seat, widgets, done) in [(FacilityKind::PowerPlant, Seat(0), 8, 3), (FacilityKind::ResearchLab, Seat(0), 4, 0), (FacilityKind::Bank, Seat(1), 8, 2)] {
                    let coastal = g.next_slot_is_coastal(sid, kind, 0, 0).unwrap_or(false);
                    g.state_mut(sid).queue.push(Build { item: BuildItem::Facility(kind), seat, widgets, done, coastal });
                }
            }
            let slot = g.free_slots_on(BodyId::Moon).first().copied().unwrap_or(0);
            let id = ColonyId(g.fresh_id());
            let modules = vec![Module::new(ModuleKind::Core), Module::new(ModuleKind::Habitat), Module::new(ModuleKind::Habitat), Module::new(ModuleKind::Generator), Module::new(ModuleKind::Factory)];
            let queue = vec![
                Build { item: BuildItem::Module(ModuleKind::Habitat), seat: Seat(0), widgets: 4, done: 2, coastal: false },
                Build { item: BuildItem::Module(ModuleKind::Mine), seat: Seat(0), widgets: 4, done: 0, coastal: false },
                Build { item: BuildItem::Module(ModuleKind::Refinery), seat: Seat(1), widgets: 8, done: 1, coastal: false },
            ];
            g.colonies.push(Colony { id, body: BodyId::Moon, slot, control: Control::Controlled(Seat(0)), modules, colonists: 8, education: 1.0, settler_education: 1.0, queue, grid_failed: false, founded_turn: 1, in_orbit: false });
            g.seats[0].stockpile.materials = 200;
            g.seats[0].stockpile.energy = 80;
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
            refuse_any_card(g);
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
                colonists: 4, warhead: false, colonists_education: 1.0,
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
                refuse_any_card(g);
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
                warhead: false,
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
    // `bombard:order` (ticket #328): the Bombard of `bombard:1`'s Battleship at the rival Colony
    // is placed, for `commit:1` to resolve.
    if std::env::args().any(|a| a == "bombard:order")
        && let Some((ship, colony)) = session.game.as_ref().and_then(|g| {
            let ship = g.ships.iter().find(|s| s.seat == Seat(0) && s.kind == UnitKind::Battleship && s.at == ShipAt::Body(BodyId::Mars))?.id;
            let colony = g.colonies.iter().find(|c| c.body == BodyId::Mars && c.control.director() == Some(Seat(1)))?.id;
            Some((ship, colony))
        })
        && !session.place(Order::Bombard { ship, colony })
    {
        eprintln!("bombard:order was refused");
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
        refuse_any_card(g);
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
    // Ticket #337 (version 0.09.0): `ducats:<n>` (a building aid): seat 0 holds exactly n Ducats.
    // A card whose offer costs more than a seat holds greys its take button, and that is the state
    // a third of the table is in when a card is drawn; a fresh board is never poor enough to show
    // it.
    if let Some(n) = std::env::args().find_map(|a| a.strip_prefix("ducats:").and_then(|v| v.parse::<i64>().ok()))
        && let Some(g) = session.game.as_mut()
    {
        g.seats[0].stockpile.ducats = n;
    }
    // `card:<event id>` (a building aid, ticket #337): that Choice Card is the turn's question, so
    // the modal that asks it stands in the picture. `cardanswer:take` or `cardanswer:refuse` then
    // answers it for seat 0 and runs the turn, so the Report that follows carries all four seats'
    // answers -- the computer seats' among them, each answered by the card's own rule.
    if let Some(id) = std::env::args().find_map(|a| a.strip_prefix("card:").and_then(card_from_id))
        && let Some(g) = session.game.as_mut()
    {
        if !ask_the_card(g, id) {
            eprintln!("card:{id:?} never came up in two hundred rolls");
            std::process::exit(3);
        }
        if let Some(taken) = std::env::args().find_map(|a| match a.strip_prefix("cardanswer:") {
            Some("take") => Some(true),
            Some("refuse") => Some(false),
            _ => None,
        }) {
            if let Err(e) = g.answer_card(Seat(0), taken) {
                eprintln!("cardanswer: {e}");
                std::process::exit(3);
            }
            run_one_quiet_turn(g);
        }
    }
    session.earth_dirty = true;
}

/// Ticket #337 (version 0.09.0): the Choice Card a `card:` aid names, by the id its data row
/// carries (`the_hard_winter`) or by the name of its variant.
fn card_from_id(name: &str) -> Option<EventId> {
    let want = name.replace('_', "");
    EventId::ALL.into_iter().find(|id| format!("{id:?}").eq_ignore_ascii_case(&want))
}

/// Ticket #337, a building aid: make that card the turn's question, by stacking the deck with it
/// and running the game's OWN Question phase until a roll brings a card -- so the question is asked
/// exactly as a game asks one, every seat included or passed over by the rule rather than by this
/// aid. The deck is put back as it would stand with that card drawn, so the Climate Panel's count
/// of what is left reads a real deck.
fn ask_the_card(g: &mut Game, id: EventId) -> bool {
    // Ticket #367 (version 0.09.2): no card comes before `first_draw_turn`, so a fresh board (turn
    // 1) would never draw and the aid would report the card never came. It stands on the first
    // turn that can draw, which is what the picture then honestly reads.
    g.turn = g.turn.max(g.tables.events.first_draw_turn);
    let deck = g.deck.clone();
    for _ in 0..200 {
        g.deck.cards = vec![Card::Event(id)];
        g.deck.drawn.clear();
        g.question_phase();
        if g.pending_question().is_some() {
            g.deck = deck;
            g.deck.cards.retain(|c| *c != Card::Event(id));
            g.deck.drawn.push(Card::Event(id));
            return true;
        }
    }
    false
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
    refuse_any_card(g);
    g.end_turn(std::array::from_fn(|_| Vec::new())).expect("the screenshot harness picks a Tech before it drives turns");
    g.seats[0].ai = true;
}

/// Ticket #339 (version 0.09.0), a building aid: **refuse this turn's Choice Card for every human
/// seat**, since ticket #337 made `end_turn` refuse while one is unanswered and an aid that drives
/// a turn by hand has no player to answer it. It REFUSES, which is the answer that buys nothing and
/// leaves the board the aid built where the aid put it. Without this, an aid that drives a turn
/// panicked on any seed whose first turn happened to draw a card: `battle:region seed:7` did, which
/// is how it was found.
fn refuse_any_card(g: &mut Game) {
    for seat in Seat::ALL {
        if !g.seat(seat).ai && g.pending_question().map(|q| q.answer_of(seat).is_none()).unwrap_or(false) {
            g.answer_card(seat, false).ok();
        }
    }
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
    // Ticket #338 (version 0.09.0): which pictures this run takes -- the four maps and the three
    // moons, or the one chronicle page, which fills the window and would otherwise be photographed
    // seven times over.
    let views: &[(&str, View)] = if plan.chronicle { &CHRONICLE } else { &VIEWS };
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
                show_view(&mut view, views[0].1);
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
        show_view(&mut view, views[0].1);
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
        // Ticket #339 (version 0.09.0): `hab:rival` picks the first Colony a RIVAL directs instead,
        // since the eye's whole point is what it reads on somebody else's card and no aid could
        // open one before.
        let rival = std::env::args().any(|a| a == "hab:rival");
        if rival {
            plan.hab = true;
        }
        plan.hab_colony = session.game.as_ref().and_then(|g| {
            g.colonies
                .iter()
                .find(|c| match c.control.director() {
                    // `hab:ground` narrows either to a Colony on a surface, which is how the Moon's
                    // picture is kept off the station standing over Earth.
                    Some(d) if rival => d != Seat(0) && (!ground || !c.in_orbit),
                    Some(d) => d == Seat(0) && (!ground || !c.in_orbit),
                    None => false,
                })
                .map(|c| c.id)
        });
        plan.trade = std::env::args().any(|a| a == "trade:1");
        plan.victory = std::env::args().any(|a| a == "victory:1");
        // Ticket #338 (version 0.09.0): `chronicle:1` opens the chronicle page, which in play is
        // reached by the Chronicle button on the game-over box -- a click a headless run cannot
        // make. The board is built above; `turns:<n>` is what plays it out to an ending.
        plan.chronicle = std::env::args().any(|a| a == "chronicle:1");
        if plan.chronicle {
            session.screen = Screen::Chronicle;
        }
        // Ticket #203: `factions:1` for seat 0's page, `factions:archivists` for that Faction's.
        if let Some(v) = std::env::args().find_map(|a| a.strip_prefix("factions:").map(str::to_owned)) {
            plan.faction_window = true;
            plan.rulebook_open = std::env::args().any(|a| a == "rulebook:1");
            if let Some(kind) = FactionKind::from_id(&v) {
                plan.faction_seat = session.game.as_ref().and_then(|g| Seat::ALL.into_iter().find(|s| g.kind(*s) == kind));
            }
        }
        // Ticket #343 (version 0.09.1): `stack:1` still means Mars; anything else is read as a
        // Body id, so `stack:earth` opens the stack over Earth.
        plan.stack = std::env::args().find_map(|a| match a.strip_prefix("stack:") {
            Some("1") => Some(BodyId::Mars),
            Some(v) => body_from_id(v),
            None => None,
        });
        // Ticket #335 (version 0.09.0): `scroll:transits` or `scroll:orbits`, the block of the Ship
        // stack's card the picture is of.
        plan.stack_scroll = std::env::args().find_map(|a| match a.strip_prefix("scroll:") {
            Some("transits") => Some(StackBlock::Transits),
            Some("orbits") => Some(StackBlock::ChangeOrbit),
            // Ticket #346 (version 0.09.1): the Tanks block, which carries the dry warning.
            Some("tanks") => Some(StackBlock::Tanks),
            _ => None,
        });
        // Ticket #337 (version 0.09.0): the turn's Choice Card stands in every picture of the run,
        // unless `cardshut:1` sets it aside for a picture of the board behind it.
        plan.card = std::env::args().any(|a| a.starts_with("card:"));
        plan.card_shut = std::env::args().any(|a| a == "cardshut:1");
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
        let (name, _) = views[plan.step];
        let path = format!("{}-{}.png", session.shot_prefix, name);
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
        plan.captured = true;
        // Give the capture a moment before the view changes under it.
        plan.next_at = t + 1.0;
        return;
    }
    plan.step += 1;
    plan.captured = false;
    if plan.step >= views.len() {
        plan.done_at = Some(t + 1.5);
    } else {
        let (_, v) = views[plan.step];
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
        // Ticket #339 (version 0.09.0): `site:` is applied AFTER `settler:`, so the two can be given
        // together -- the Colony Ship `settler:` plants is what puts a founding button on the SLOT
        // panel `site:` opens, and before this the stack selection won and the slot panel could
        // never be photographed with a founding door on it. The more specific aid wins.
        if let View::Surface(body) = v {
            if std::env::args().any(|a| a.strip_prefix("settler:").and_then(body_from_id) == Some(body)) {
                view.selection = Selection::ShipStack(body, Seat(0));
            }
            if let Some((b, slot)) = std::env::args().find_map(|a| a.strip_prefix("site:").and_then(|v| v.split_once(',')).and_then(|(b, n)| Some((body_from_id(b)?, n.parse::<u32>().ok()?))))
                && b == body
            {
                view.selection = Selection::Slot(body, slot);
            }
        }
        plan.next_at = t + 2.5;
    }
}

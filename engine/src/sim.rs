//! Simulate mode (spec 19.3): an AI-versus-AI game, headless, with a full log.
//! Ticket #50: four seats, so every Faction plays every game.

use crate::climate::LastTurn;
use crate::data::Tables;
use crate::ids::*;
use crate::state::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct SimResult {
    pub seed: u64,
    /// The Faction in seat 0.
    pub player: FactionKind,
    pub outcome: Option<Outcome>,
    pub last_turn: u32,
    pub first_colony_turn: Option<u32>,
    pub buildings: [u32; SEAT_COUNT],
    pub colonists_off_earth: [u32; SEAT_COUNT],
    pub temperature: f64,
    pub collapse_projected_turn: Option<u32>,
    pub colony_changed_hands: Vec<(u32, u32)>,
    /// Ticket #41: places that changed hands by Influence over the game.
    pub influence_transfers: u32,
    /// Ticket #41: Banks, Trade Posts, Embassies and Relays completed by any seat.
    pub new_buildings: [u32; 4],
    /// Ticket #52: states that threw off a controller, the highest Unrest any state reached, the
    /// Constabularies raised, the Relief orders paid, and the population refugees carried.
    pub throw_offs: u32,
    pub peak_unrest: f64,
    pub constabularies: u32,
    pub relief_orders: u32,
    pub population_moved: f64,
    /// Ticket #53: each seat's Blame at the end, its share of the table's, and the multiplier its
    /// share puts on its Influence thresholds; and how many neutral states developed themselves.
    pub blame: [f64; SEAT_COUNT],
    pub blame_share: [f64; SEAT_COUNT],
    pub threshold_multiplier: [f64; SEAT_COUNT],
    pub developments: u32,
    /// Ticket #54: Scrubbers completed, Mothballs, Restarts and Decommissions landed, Leapfrogs
    /// bought and Strip Permits issued over the game, by any seat.
    pub scrubbers: u32,
    pub mothballs: u32,
    pub restarts: u32,
    pub decommissions: u32,
    pub leapfrogs: u32,
    pub strip_permits: u32,
    /// Ticket #54: the longest Stabilization run each seat held at any point in the game.
    pub longest_stabilization: [u32; SEAT_COUNT],
    /// Ticket #54: the world's net Emissions at the Climate phase of turn 12, and at the last turn.
    pub net_at_twelve: Option<f64>,
    pub net_at_end: f64,
    /// Ticket #55: the turn each Break fired, by index into `climate.toml`'s list, None if it never
    /// did; and the Last Turn the Climate Panel showed at turn 1 and at turn 12.
    pub break_turns: Vec<Option<u32>>,
    pub last_turn_at_one: LastTurn,
    pub last_turn_at_twelve: Option<LastTurn>,
    /// Ticket #56: Sea Walls completed and Sea Walls spent absorbing a threshold; coastal slots the
    /// sea took over the game and Facilities it destroyed with them; the turn Antarctica opened and
    /// how many Colonies were founded there.
    pub sea_walls_built: u32,
    pub sea_walls_spent: u32,
    pub coastal_slots_lost: u32,
    pub facilities_drowned: u32,
    pub antarctica_turn: Option<u32>,
    pub antarctic_colonies: u32,
    /// Ticket #57: the turn the first Colony in the Mars system was founded, and the turn the Mars
    /// launch window falls on, which the ephemeris fixes and no seed moves.
    pub first_mars_colony_turn: Option<u32>,
    pub window_turn: u32,
    /// Ticket #68: the turn the Archive Module first stood, the turn the Archive was complete, and
    /// the Archivists' fund at the end.
    pub archive_built_turn: Option<u32>,
    pub archive_complete_turn: Option<u32>,
    pub archive_fund_at_end: i64,
    /// Ticket #69: what the neutral Labs paid into the shared Tech over the game, and the turn
    /// Coastal Engineering completed.
    pub neutral_research: i64,
    pub coastal_engineering_turn: Option<u32>,
    /// Ticket #80 (version 0.06.0): Observatories standing at the end, per seat, and the Research
    /// each seat's Observatories made away from Earth over the game.
    pub observatories: [u32; 4],
    pub research_off_earth: [i64; 4],
    /// Ticket #82 (version 0.06.0): Module-turns doubled by an idle Facility on Earth, per seat.
    pub doubled_module_turns: [i64; 4],
    /// Ticket #84: the turn each seat's Victory gate completed, if it did.
    pub gate_turn: [Option<u32>; 4],
    /// Ticket #86: Colonists each seat lost in transit to crowding.
    pub lost_in_transit: [i64; 4],
    /// Ticket #87: Ships stranded at the end (no leg their tank can pay, no station of their own
    /// there), per seat; Refuel orders committed over the game; stations standing off Earth at
    /// the end (a station over Earth is not one), all seats.
    pub stranded_at_end: [u32; 4],
    pub refuels: u32,
    pub stations_off_earth: u32,
    /// Ticket #72: the Prospectors' Venture Capital Fund at the end.
    pub venture_fund_at_end: i64,
    /// Ticket #76: cards drawn over the game, and whether the deck ran dry.
    pub cards_drawn: u32,
    pub deck_empty: bool,
    /// Ticket #73: Emigrant batches mustered over the game, and Antarctic Colonies founded by sea.
    pub emigrant_batches: u32,
    pub antarctic_by_sea: u32,
    /// Ticket #75: the turn seat 0 first lost the Nation State it started in, None if never.
    pub start_state_lost_turn: Option<u32>,
    /// Ticket #58: how many Moments the turns of this game earned, how many the cap of two and the
    /// defaults in `report.toml` actually showed, how many turns stopped for at least one, and the
    /// most any one turn showed.
    pub moments_earned: u32,
    pub moments_shown: u32,
    pub turns_with_moment: u32,
    pub most_moments_in_a_turn: u32,
    /// Ticket #60: the Techs the world finished over the game and the highest rung any of them
    /// stood on, which is what says whether a rung-2 Tech (Coastal Engineering, and so the Sea
    /// Wall) was ever reachable; and whether any seat ever met its Victory Condition outright,
    /// rather than winning on the last turn's score.
    pub techs_completed: u32,
    pub highest_rung: u32,
    pub victory_met: Option<(Seat, FactionKind)>,
    pub log: Vec<String>,
}

impl SimResult {
    /// The Faction each seat held, in seat order.
    pub fn seat_kinds(&self) -> [FactionKind; SEAT_COUNT] {
        let mut kinds: Vec<FactionKind> = vec![self.player];
        kinds.extend(FactionKind::ALL.into_iter().filter(|k| *k != self.player));
        std::array::from_fn(|i| kinds[i])
    }
}

/// Play one whole game with all four seats on the AI. `player` is the Faction in seat 0.
pub fn run(tables: Arc<Tables>, seed: u64, player: FactionKind) -> SimResult {
    run_from(tables, seed, player, StateId::EastAsia)
}

/// As `run`, with seat 0 starting in `start` (the sweep uses this to try other seats at the table).
pub fn run_from(tables: Arc<Tables>, seed: u64, player: FactionKind, start: StateId) -> SimResult {
    let mut game = Game::new(tables.clone(), NewGame { seed, player, player_is_ai: true, player_start: start });
    game.start();
    let mut first_colony_turn = None;
    let mut projected_collapse: Option<u32> = None;
    let mut founded: Vec<(ColonyId, u32, Option<Seat>)> = Vec::new();
    let mut changed: Vec<(u32, u32)> = Vec::new();
    let mut peak_unrest = 0.0f64;
    // Ticket #54: the longest Stabilization run each seat reached, and the net at turn 12.
    let mut longest_stabilization = [0u32; SEAT_COUNT];
    let mut net_at_twelve: Option<f64> = None;
    // Ticket #55: when each Break fired, and what the Last Turn line said at turn 1 and turn 12.
    let mut break_turns: Vec<Option<u32>> = vec![None; tables.climate.breaks.len()];
    let note_breaks = |g: &Game, turns: &mut Vec<Option<u32>>| {
        for (i, fired) in g.climate.breaks_fired.iter().enumerate() {
            if *fired && turns[i].is_none() {
                turns[i] = Some(g.turn);
            }
        }
    };
    note_breaks(&game, &mut break_turns);
    let last_turn_at_one = game.last_turn_to_act();
    let mut last_turn_at_twelve: Option<LastTurn> = None;
    // Ticket #56: the turn the ice opened, and the most Antarctic Colonies standing at once.
    let mut antarctica_turn: Option<u32> = None;
    let mut antarctic_colonies = 0u32;
    // Ticket #57: the first Colony anywhere in the Mars system, and the turn the window falls on.
    let mut first_mars_colony_turn: Option<u32> = None;
    let archivist = Seat::ALL.into_iter().find(|s| game.kind(*s) == FactionKind::Archivists);
    let mut archive_built_turn: Option<u32> = None;
    let mut archive_complete_turn: Option<u32> = None;
    let mut coastal_engineering_turn: Option<u32> = None;
    // Ticket #84: the turn each seat's Victory gate completed, if it did.
    let mut gate_turn: [Option<u32>; 4] = [None; 4];
    let home = game.controlled_states(Seat(0)).first().copied();
    let mut start_state_lost_turn: Option<u32> = None;
    let window_turn = game.next_window_turn(1);
    let max_turns = tables.victory.turns;
    let mut guard = 0;
    // Ticket #58: what the Moments did over the game.
    let (mut moments_earned, mut moments_shown, mut turns_with_moment, mut most_moments_in_a_turn) = (0u32, 0u32, 0u32, 0u32);
    // Ticket #60: the first seat to meet its Victory Condition outright, at any point in the game.
    let mut victory_met: Option<(Seat, FactionKind)> = None;
    while !game.is_over() && guard < max_turns + 2 {
        guard += 1;
        game.end_turn(std::array::from_fn(|_| Vec::new()));
        moments_earned += game.report.moments.len() as u32;
        let shown = game.report.moments_shown(&|k| tables.report.moment_on(k)).len() as u32;
        moments_shown += shown;
        most_moments_in_a_turn = most_moments_in_a_turn.max(shown);
        if shown > 0 {
            turns_with_moment += 1;
        }
        if first_colony_turn.is_none() && game.colonies.iter().any(|c| !c.in_orbit) {
            first_colony_turn = game.colonies.iter().filter(|c| !c.in_orbit).map(|c| c.founded_turn).min();
        }
        for c in &game.colonies {
            match founded.iter_mut().find(|(id, _, _)| *id == c.id) {
                None => founded.push((c.id, c.founded_turn, c.control.controller())),
                Some((_, t, owner)) => {
                    if *owner != c.control.controller() && c.control.controller().is_some() {
                        changed.push((*t, game.turn));
                        *owner = c.control.controller();
                        *t = game.turn;
                    }
                }
            }
        }
        if projected_collapse.is_none() {
            projected_collapse = game.projection().collapse_turn;
        }
        peak_unrest = game.states.iter().map(|s| s.unrest).fold(peak_unrest, f64::max);
        for s in Seat::ALL {
            longest_stabilization[s.index()] = longest_stabilization[s.index()].max(game.seat(s).stabilization_run);
        }
        if game.turn >= 12 && net_at_twelve.is_none() {
            net_at_twelve = Some(game.climate.last.net());
            last_turn_at_twelve = Some(game.last_turn_to_act());
        }
        note_breaks(&game, &mut break_turns);
        if victory_met.is_none() {
            victory_met = Seat::ALL.into_iter().find(|s| game.progress(*s).met()).map(|s| (s, game.kind(s)));
        }
        if antarctica_turn.is_none() && game.antarctica_open {
            antarctica_turn = Some(game.turn);
        }
        antarctic_colonies = antarctic_colonies.max(game.colonies.iter().filter(|c| c.body == BodyId::Earth && !c.in_orbit).count() as u32);
        if let Some(a) = archivist {
            if archive_built_turn.is_none() && game.archive_built(a) {
                archive_built_turn = Some(game.turn);
            }
            if archive_complete_turn.is_none() && game.archive_complete(a) {
                archive_complete_turn = Some(game.turn);
            }
        }
        if start_state_lost_turn.is_none() && home.map(|h| game.state(h).control.controller() != Some(Seat(0))).unwrap_or(false) {
            start_state_lost_turn = Some(game.turn);
        }
        if coastal_engineering_turn.is_none() && game.has_tech(crate::ids::TechId::CoastalEngineering) {
            coastal_engineering_turn = Some(game.turn);
        }
        for s in Seat::ALL {
            if gate_turn[s.index()].is_none()
                && let Some(gate) = game.tables.victory_gate(game.kind(s))
                && game.has_tech(gate)
            {
                gate_turn[s.index()] = Some(game.turn);
            }
        }
        if first_mars_colony_turn.is_none() {
            first_mars_colony_turn = game
                .colonies
                .iter()
                .filter(|c| !c.in_orbit && matches!(c.body, BodyId::Mars | BodyId::Phobos | BodyId::Deimos))
                .map(|c| c.founded_turn)
                .min();
        }
    }
    let buildings = Seat::ALL.map(|s| {
        let f: u32 = game.directed_states(s).iter().map(|st| game.state(*st).facilities.len() as u32).sum();
        let m: u32 = game.directed_colonies(s).iter().map(|c| game.colony(*c).unwrap().modules.len() as u32).sum();
        f + m
    });
    let colonists = Seat::ALL.map(|s| game.off_world_colonists(s));
    // Ticket #80 (version 0.06.0): Observatories standing at the end, and the Research they made
    // away from Earth over the game, per seat.
    let observatories = Seat::ALL.map(|s| {
        game.directed_colonies(s).iter().map(|c| game.colony(*c).unwrap().modules.iter().filter(|m| m.kind == ModuleKind::Observatory).count() as u32).sum::<u32>()
    });
    let research_off_earth = Seat::ALL.map(|s| game.seat(s).research_off_earth_total);
    game.log(format!(
        "Summary: {} | last turn {} | first Colony {:?} | buildings {:?} | Colonists off Earth {:?} | temperature {:+.2} | collapse projected {:?} | Observatories {:?} | Research off Earth {:?}",
        game.outcome_text(),
        game.turn,
        first_colony_turn,
        buildings,
        colonists,
        game.climate.temperature,
        projected_collapse,
        observatories,
        research_off_earth
    ));
    let influence_transfers = game.log.iter().filter(|l| !l.starts_with(' ') && l.ends_with("(Influence).")).count() as u32;
    // Ticket #52, read off the log the same way: throw-offs, Constabularies, Relief orders and the
    // population the refugee flows carried (to the tenth the line prints).
    let throw_offs = game.log.iter().filter(|l| l.contains("threw off the")).count() as u32;
    let constabularies = game.log.iter().filter(|l| l.contains("completed Constabulary at")).count() as u32;
    let relief_orders = game.log.iter().filter(|l| l.trim_start().starts_with("take") && l.contains("pay Relief in")).count() as u32;
    let population_moved: f64 = game
        .log
        .iter()
        .filter(|l| l.contains(" population left "))
        .filter_map(|l| l.trim_start().split(' ').next().and_then(|n| n.parse::<f64>().ok()))
        .sum();
    let new_buildings = ["Bank", "Trade Post", "Embassy", "Relay"].map(|b| game.log.iter().filter(|l| l.contains(&format!("completed {b} at"))).count() as u32);
    // Ticket #53: Blame as it stands at the end, and the neutral states that developed themselves.
    let blame = Seat::ALL.map(|s| game.blame(s));
    let blame_share = Seat::ALL.map(|s| game.blame_share(s));
    let threshold_multiplier = Seat::ALL.map(|s| game.blame_threshold_multiplier(s));
    let developments = game.log.iter().filter(|l| l.contains(" raised its Industry Level to ")).count() as u32;
    // Ticket #54, read off the log the same way the #52 and #53 figures are.
    let scrubbers = game.log.iter().filter(|l| l.contains("completed Scrubber at")).count() as u32;
    let mothballs = game.log.iter().filter(|l| l.contains(" mothballed the ")).count() as u32;
    let restarts = game.log.iter().filter(|l| l.contains(" restarted the ")).count() as u32;
    let decommissions = game.log.iter().filter(|l| l.contains(" decommissioned the ")).count() as u32;
    let leapfrogs = game.log.iter().filter(|l| l.contains(" Leapfrogged ")).count() as u32;
    let strip_permits = game.log.iter().filter(|l| l.contains(" issued a Strip Permit in ")).count() as u32;
    let net_at_end = game.climate.last.net();
    // Ticket #60: the Techs the world finished and the highest rung it reached.
    let techs_completed = game.research.done.len() as u32;
    let highest_rung = game.research.done.iter().map(|t| tables.tech(*t).rung).max().unwrap_or(0);
    // Ticket #56, read off the log as the #52 to #55 figures are.
    let sea_walls_built = game.log.iter().filter(|l| l.contains("completed Sea Wall at")).count() as u32;
    let sea_walls_spent = game.log.iter().filter(|l| l.contains("the Sea Wall in") && l.contains("was destroyed")).count() as u32;
    let mut coastal_slots_lost = 0u32;
    let mut facilities_drowned = 0u32;
    for l in game.log.iter().filter(|l| l.starts_with("The sea took ")) {
        if let Some(n) = l.trim_start_matches("The sea took ").split(' ').next().and_then(|n| n.parse::<u32>().ok()) {
            coastal_slots_lost += n;
        }
        if let Some((_, rest)) = l.split_once(" C: ") {
            let list = rest.split('.').next().unwrap_or("");
            facilities_drowned += list.split(" and ").flat_map(|p| p.split(", ")).filter(|p| !p.trim().is_empty()).count() as u32;
        }
    }
    SimResult {
        seed,
        player,
        outcome: game.outcome.clone(),
        last_turn: game.turn,
        first_colony_turn,
        buildings,
        colonists_off_earth: colonists,
        temperature: game.climate.temperature,
        collapse_projected_turn: projected_collapse,
        colony_changed_hands: changed,
        influence_transfers,
        new_buildings,
        throw_offs,
        peak_unrest,
        constabularies,
        relief_orders,
        population_moved,
        blame,
        blame_share,
        threshold_multiplier,
        developments,
        scrubbers,
        mothballs,
        restarts,
        decommissions,
        leapfrogs,
        strip_permits,
        longest_stabilization,
        net_at_twelve,
        net_at_end,
        break_turns,
        last_turn_at_one,
        last_turn_at_twelve,
        sea_walls_built,
        sea_walls_spent,
        coastal_slots_lost,
        facilities_drowned,
        antarctica_turn,
        antarctic_colonies,
        first_mars_colony_turn,
        window_turn,
        archive_built_turn,
        archive_complete_turn,
        archive_fund_at_end: archivist.map(|a| game.seat(a).archive_fund).unwrap_or(0),
        neutral_research: game.research.neutral_total,
        coastal_engineering_turn,
        observatories: Seat::ALL.map(|s| {
            game.directed_colonies(s).iter().map(|c| game.colony(*c).unwrap().modules.iter().filter(|m| m.kind == ModuleKind::Observatory).count() as u32).sum::<u32>()
        }),
        research_off_earth: Seat::ALL.map(|s| game.seat(s).research_off_earth_total),
        doubled_module_turns: Seat::ALL.map(|s| game.seat(s).doubled_module_turns),
        gate_turn,
        lost_in_transit: Seat::ALL.map(|s| game.seat(s).lost_in_transit),
        stranded_at_end: Seat::ALL.map(|s| game.ships.iter().filter(|sh| sh.seat == s && game.stranded(sh.id)).count() as u32),
        refuels: game.log.iter().filter(|l| l.contains(" refuels ")).count() as u32,
        stations_off_earth: game.colonies.iter().filter(|c| c.in_orbit && c.body != BodyId::Earth).count() as u32,
        venture_fund_at_end: Seat::ALL.into_iter().find(|s| game.kind(*s) == FactionKind::Prospectors).map(|s| game.seat(s).venture_fund).unwrap_or(0),
        cards_drawn: game.deck.drawn.len() as u32,
        deck_empty: game.deck.cards.is_empty(),
        start_state_lost_turn,
        emigrant_batches: game.log.iter().filter(|l| l.contains("Emigrants mustered in")).count() as u32,
        antarctic_by_sea: game.log.iter().filter(|l| l.contains("in Antarctica with")).count() as u32,
        moments_earned,
        moments_shown,
        turns_with_moment,
        most_moments_in_a_turn,
        techs_completed,
        highest_rung,
        victory_met,
        log: game.log,
    }
}

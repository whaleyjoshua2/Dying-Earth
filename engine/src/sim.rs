//! Simulate mode (spec 19.3): an AI-versus-AI game, headless, with a full log.
//! Ticket #50: four seats, so every Faction plays every game.

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
    let max_turns = tables.victory.turns;
    let mut guard = 0;
    while !game.is_over() && guard < max_turns + 2 {
        guard += 1;
        game.end_turn(std::array::from_fn(|_| Vec::new()));
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
        }
    }
    let buildings = Seat::ALL.map(|s| {
        let f: u32 = game.directed_states(s).iter().map(|st| game.state(*st).facilities.len() as u32).sum();
        let m: u32 = game.directed_colonies(s).iter().map(|c| game.colony(*c).unwrap().modules.len() as u32).sum();
        f + m
    });
    let colonists = Seat::ALL.map(|s| game.off_world_colonists(s));
    game.log(format!(
        "Summary: {} | last turn {} | first Colony {:?} | buildings {:?} | Colonists off Earth {:?} | temperature {:+.2} | collapse projected {:?}",
        game.outcome_text(),
        game.turn,
        first_colony_turn,
        buildings,
        colonists,
        game.climate.temperature,
        projected_collapse
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
        log: game.log,
    }
}

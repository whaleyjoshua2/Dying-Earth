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
    pub peak_unrest: i64,
    pub constabularies: u32,
    pub relief_orders: u32,
    pub population_moved: f64,
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
    run_from(tables, seed, player, StateId::Asia)
}

/// As `run`, with seat 0 starting in `start` (the sweep uses this to try other seats at the table).
pub fn run_from(tables: Arc<Tables>, seed: u64, player: FactionKind, start: StateId) -> SimResult {
    let mut game = Game::new(tables.clone(), NewGame { seed, player, player_is_ai: true, player_start: start });
    game.start();
    let mut first_colony_turn = None;
    let mut projected_collapse: Option<u32> = None;
    let mut founded: Vec<(ColonyId, u32, Option<Seat>)> = Vec::new();
    let mut changed: Vec<(u32, u32)> = Vec::new();
    let mut peak_unrest = 0i64;
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
        peak_unrest = peak_unrest.max(game.states.iter().map(|s| s.unrest).max().unwrap_or(0));
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
        log: game.log,
    }
}

//! Simulate mode (spec 19.3): an AI-versus-AI game, headless, with a full log.

use crate::data::Tables;
use crate::ids::*;
use crate::state::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct SimResult {
    pub seed: u64,
    pub outcome: Option<Outcome>,
    pub last_turn: u32,
    pub first_colony_turn: Option<u32>,
    pub buildings: [u32; 2],
    pub colonists_off_earth: [u32; 2],
    pub temperature: f64,
    pub collapse_projected_turn: Option<u32>,
    pub colony_changed_hands: Vec<(u32, u32)>,
    pub log: Vec<String>,
}

/// Play one whole game between two AIs. `kinds` are the two seats' Factions.
pub fn run(tables: Arc<Tables>, seed: u64, kinds: [FactionKind; 2]) -> SimResult {
    let mut game = Game::new(tables.clone(), NewGame { seed, seats: [(kinds[0], true), (kinds[1], true)], player_start: StateId::Asia });
    game.start();
    let mut first_colony_turn = None;
    let mut projected_collapse: Option<u32> = None;
    let mut founded: Vec<(ColonyId, u32, Option<Seat>)> = Vec::new();
    let mut changed: Vec<(u32, u32)> = Vec::new();
    let max_turns = tables.victory.turns;
    let mut guard = 0;
    while !game.is_over() && guard < max_turns + 2 {
        guard += 1;
        game.end_turn([Vec::new(), Vec::new()]);
        if first_colony_turn.is_none() && !game.colonies.is_empty() {
            first_colony_turn = Some(game.colonies.iter().map(|c| c.founded_turn).min().unwrap());
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
    }
    let buildings = [Seat(0), Seat(1)].map(|s| {
        let f: u32 = game.directed_states(s).iter().map(|st| game.state(*st).facilities.len() as u32).sum();
        let m: u32 = game.directed_colonies(s).iter().map(|c| game.colony(*c).unwrap().modules.len() as u32).sum();
        f + m
    });
    let colonists = [Seat(0), Seat(1)].map(|s| game.off_world_colonists(s));
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
    SimResult {
        seed,
        outcome: game.outcome.clone(),
        last_turn: game.turn,
        first_colony_turn,
        buildings,
        colonists_off_earth: colonists,
        temperature: game.climate.temperature,
        collapse_projected_turn: projected_collapse,
        colony_changed_hands: changed,
        log: game.log,
    }
}

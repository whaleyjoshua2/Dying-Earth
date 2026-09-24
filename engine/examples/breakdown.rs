//! Ticket #332 probe: the emissions breakdown of an all-computer game, every fifth turn.
use dying_earth_engine::data::{default_data_dir, Tables};
use dying_earth_engine::*;
use std::sync::Arc;

fn main() {
    let seed: u64 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(7);
    let tables = Arc::new(Tables::load(&default_data_dir()).expect("tables"));
    let mut g = Game::new(tables, NewGame { seed, player: FactionKind::Custodians, player_is_ai: true, player_start: StateId::EastAsia });
    let mut guard = 0;
    while !g.is_over() && guard < 40 {
        guard += 1;
        if g.end_turn(std::array::from_fn(|_| Vec::new())).is_err() { break; }
    }
    println!("seed {seed}: over at turn {} outcome {:?}", g.turn, g.outcome.is_some());
    println!("turn  industry factories power refin launch people cards permaf war | sink scrub | temp");
    for r in g.climate.history.iter().filter(|r| r.turn % 5 == 0 || r.turn == 1) {
        let b = &r.breakdown;
        println!("{:>4}  {:>8.1} {:>9.1} {:>5.1} {:>5.1} {:>6.1} {:>6.1} {:>5.1} {:>6.1} {:>4.1} | {:>4.1} {:>5.1} | {:+.2}", r.turn, b.state_industry, b.factories, b.power_plants, b.refineries, b.launches, b.population, b.cards, b.permafrost, b.war, b.sink, b.scrubbers, r.temperature);
    }
}

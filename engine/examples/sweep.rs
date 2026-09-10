//! Ticket #27: sweep the two Climate knobs and report when a four-Faction game collapses (ticket #50).
//! `cargo run -p dying-earth-engine --example sweep -- [seeds]`

use dying_earth_engine::data::{default_data_dir, Tables};
use dying_earth_engine::ids::FactionKind;
use dying_earth_engine::state::Outcome;
use std::sync::Arc;

fn main() {
    let seeds: u64 = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(20);
    let base = Tables::load(&default_data_dir()).expect("tables");
    println!("sink  step | collapses/{} | collapse turns (median, min..max) | temp at end (median)", seeds);
    for sink in [6.0] {
        for step in [90.0, 100.0, 110.0, 120.0, 130.0, 140.0] {
            let mut t = base.clone();
            t.climate.natural_sink = sink;
            t.climate.ppm_step = step;
            let tables = Arc::new(t);
            let mut turns = Vec::new();
            let mut temps = Vec::new();
            for seed in 1..=seeds {
                let r = dying_earth_engine::sim::run(tables.clone(), seed, FactionKind::Custodians);
                if r.outcome == Some(Outcome::Collapse) {
                    turns.push(r.last_turn);
                }
                temps.push(r.temperature);
            }
            turns.sort();
            temps.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let median = turns.get(turns.len() / 2).map(|t| t.to_string()).unwrap_or("-".into());
            let range = if turns.is_empty() { "-".to_string() } else { format!("{}..{}", turns[0], turns[turns.len() - 1]) };
            println!("{sink:4.0}  {step:4.0} | {:2}/{} | {median:>3} ({range}) | {:+.2}", turns.len(), seeds, temps[temps.len() / 2]);
        }
    }
}

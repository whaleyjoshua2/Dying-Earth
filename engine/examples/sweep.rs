//! Ticket #27: sweep the two Climate knobs and report when a four-Faction game collapses (ticket #50).
//! `cargo run -p dying-earth-engine --example sweep -- [seeds] [--player=<faction>] [--sinks=6,7] [--steps=90,120]`

use dying_earth_engine::data::{default_data_dir, Tables};
use dying_earth_engine::ids::{FactionKind, StateId};
use dying_earth_engine::state::Outcome;
use std::sync::Arc;

fn list(flag: &str, default: &[f64]) -> Vec<f64> {
    std::env::args()
        .find_map(|a| a.strip_prefix(flag).map(|s| s.split(',').filter_map(|x| x.parse().ok()).collect::<Vec<f64>>()))
        .unwrap_or_else(|| default.to_vec())
}

fn main() {
    let seeds: u64 = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(20);
    let player = std::env::args()
        .find_map(|a| a.strip_prefix("--player=").map(|s| s.to_string()))
        .and_then(|s| FactionKind::ALL.into_iter().find(|k| k.id() == s))
        .unwrap_or(FactionKind::Custodians);
    let start = std::env::args()
        .find_map(|a| a.strip_prefix("--start=").map(|s| s.to_string()))
        .and_then(|s| StateId::ALL.into_iter().find(|k| format!("{k:?}").to_lowercase().starts_with(&s)))
        .unwrap_or(StateId::EastAsia);
    let sinks = list("--sinks=", &[6.0]);
    let steps = list("--steps=", &[90.0, 100.0, 110.0, 120.0, 130.0, 140.0]);
    let base = Tables::load(&default_data_dir()).expect("tables");
    println!("seat 0: {} starting in {}", player.name(), format!("{start:?}"));
    println!("sink  step | collapses/{} | collapse turns (median, min..max) | temp at end (median) | wins by seat", seeds);
    for &sink in &sinks {
        for &step in &steps {
            let mut t = base.clone();
            t.climate.natural_sink = sink;
            t.climate.ppm_step = step;
            let tables = Arc::new(t);
            let mut turns = Vec::new();
            let mut temps = Vec::new();
            let mut wins = [0u32; 4];
            for seed in 1..=seeds {
                let r = dying_earth_engine::sim::run_from(tables.clone(), seed, player, start);
                match r.outcome {
                    Some(Outcome::Collapse) => turns.push(r.last_turn),
                    Some(Outcome::Win { seat, .. }) => wins[seat.index()] += 1,
                    _ => {}
                }
                temps.push(r.temperature);
            }
            turns.sort();
            temps.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let median = turns.get(turns.len() / 2).map(|t| t.to_string()).unwrap_or("-".into());
            let range = if turns.is_empty() { "-".to_string() } else { format!("{}..{}", turns[0], turns[turns.len() - 1]) };
            println!("{sink:4.0}  {step:4.0} | {:2}/{} | {median:>3} ({range}) | {:+.2} | {:?}", turns.len(), seeds, temps[temps.len() / 2], wins);
        }
    }
}

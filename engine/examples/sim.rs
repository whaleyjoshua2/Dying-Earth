//! Quick headless runner: `cargo run -p dying-earth-engine --example sim -- <seed> [prospectors|custodians] [prospectors|custodians] [--log]`

use dying_earth_engine::data::{default_data_dir, Tables};
use dying_earth_engine::ids::FactionKind;
use std::sync::Arc;

fn kind(s: &str) -> FactionKind {
    if s.starts_with('c') { FactionKind::Custodians } else { FactionKind::Prospectors }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let seed: u64 = args.first().and_then(|a| a.parse().ok()).unwrap_or(1);
    let a = args.get(1).map(|s| kind(s)).unwrap_or(FactionKind::Custodians);
    let b = args.get(2).map(|s| kind(s)).unwrap_or(FactionKind::Prospectors);
    let log = args.iter().any(|a| a == "--log");
    let count: u64 = args.iter().find_map(|a| a.strip_prefix("--count=")).and_then(|v| v.parse().ok()).unwrap_or(1);
    if a == b {
        eprintln!("Factions must be unique (ticket #27): one Custodian seat against one Prospector seat.");
        std::process::exit(2);
    }
    let tables = Arc::new(Tables::load(&default_data_dir()).expect("tables"));
    for s in seed..seed + count {
        let r = dying_earth_engine::sim::run(tables.clone(), s, [a, b]);
        if log {
            for l in &r.log {
                println!("{l}");
            }
        }
        println!(
            "seed {:3} | {:?} | turn {:2} | first colony {:?} | buildings {:?} | colonists {:?} | temp {:+.2} | collapse proj {:?} | colony hands {:?} | influence transfers {:2} | bank/post/embassy/relay {:?}",
            r.seed, r.outcome, r.last_turn, r.first_colony_turn, r.buildings, r.colonists_off_earth, r.temperature, r.collapse_projected_turn, r.colony_changed_hands, r.influence_transfers, r.new_buildings
        );
    }
}

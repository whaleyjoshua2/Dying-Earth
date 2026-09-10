//! Quick headless runner (ticket #50: four seats, all AI):
//! `cargo run -p dying-earth-engine --example sim -- <seed> [--player=<faction>] [--count=N] [--log]`
//! Faction ids are the full names: custodians, prospectors, arkwrights, archivists.

use dying_earth_engine::data::{default_data_dir, Tables};
use dying_earth_engine::ids::{FactionKind, Seat, SEAT_COUNT};
use dying_earth_engine::state::Outcome;
use std::sync::Arc;

fn median(v: &mut [u32]) -> String {
    if v.is_empty() {
        return "-".to_string();
    }
    v.sort_unstable();
    v[v.len() / 2].to_string()
}

/// Ticket #53: the middle value of a run of figures, for the Blame tables.
fn median_f(v: &mut [f64]) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    v[v.len() / 2]
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let seed: u64 = args.first().and_then(|a| a.parse().ok()).unwrap_or(1);
    let log = args.iter().any(|a| a == "--log");
    let count: u64 = args.iter().find_map(|a| a.strip_prefix("--count=")).and_then(|v| v.parse().ok()).unwrap_or(1);
    let player = match args.iter().find_map(|a| a.strip_prefix("--player=")) {
        None => FactionKind::Custodians,
        Some(name) => match FactionKind::from_id(name) {
            Some(k) => k,
            None => {
                eprintln!("unknown Faction {name:?}: use custodians, prospectors, arkwrights or archivists");
                std::process::exit(2);
            }
        },
    };
    let tables = Arc::new(Tables::load(&default_data_dir()).expect("tables"));
    let mut wins = [0u32; SEAT_COUNT];
    let mut draws = 0u32;
    let mut collapses = 0u32;
    let mut collapse_turns: Vec<u32> = Vec::new();
    let mut first_colony: Vec<u32> = Vec::new();
    // Ticket #52.
    let mut throw_offs = 0u32;
    let mut peaks: Vec<f64> = Vec::new();
    let mut constabularies = 0u32;
    let mut relief = 0u32;
    let mut moved = 0.0f64;
    // Ticket #53.
    let mut blame: [Vec<f64>; SEAT_COUNT] = std::array::from_fn(|_| Vec::new());
    let mut shares: [Vec<f64>; SEAT_COUNT] = std::array::from_fn(|_| Vec::new());
    let mut mults: [Vec<f64>; SEAT_COUNT] = std::array::from_fn(|_| Vec::new());
    let mut developments: Vec<u32> = Vec::new();
    let mut kinds = [FactionKind::Custodians; SEAT_COUNT];
    for s in seed..seed + count {
        let r = dying_earth_engine::sim::run(tables.clone(), s, player);
        kinds = r.seat_kinds();
        if log {
            for l in &r.log {
                println!("{l}");
            }
        }
        match &r.outcome {
            Some(Outcome::Win { seat, .. }) => wins[seat.index()] += 1,
            Some(Outcome::Draw { .. }) => draws += 1,
            Some(Outcome::Collapse) => {
                collapses += 1;
                collapse_turns.push(r.last_turn);
            }
            None => {}
        }
        if let Some(t) = r.first_colony_turn {
            first_colony.push(t);
        }
        throw_offs += r.throw_offs;
        peaks.push(r.peak_unrest.max(0.0));
        constabularies += r.constabularies;
        relief += r.relief_orders;
        moved += r.population_moved;
        for seat in Seat::ALL {
            blame[seat.index()].push(r.blame[seat.index()]);
            shares[seat.index()].push(r.blame_share[seat.index()]);
            mults[seat.index()].push(r.threshold_multiplier[seat.index()]);
        }
        developments.push(r.developments);
        println!(
            "seed {:3} | {:?} | turn {:2} | first colony {:?} | buildings {:?} | colonists {:?} | temp {:+.2} | collapse proj {:?} | colony hands {:?} | influence transfers {:2} | bank/post/embassy/relay {:?}",
            r.seed, r.outcome, r.last_turn, r.first_colony_turn, r.buildings, r.colonists_off_earth, r.temperature, r.collapse_projected_turn, r.colony_changed_hands, r.influence_transfers, r.new_buildings
        );
        println!(
            "         | unrest: threw off {} | peak {:.1} | constabularies {} | relief orders {} | population moved {:.1}",
            r.throw_offs, r.peak_unrest, r.constabularies, r.relief_orders, r.population_moved
        );
        println!(
            "         | blame {:?} | share {:?} | thresholds {:?} | neutral developments {}",
            r.blame.map(|b| format!("{b:.0}")),
            r.blame_share.map(|b| format!("{b:.2}")),
            r.threshold_multiplier.map(|b| format!("x{b:.2}")),
            r.developments
        );
    }
    if count > 1 {
        println!();
        println!("--- {count} seeds from {seed}, seat 0 the {} ---", player.name());
        for seat in Seat::ALL {
            println!("{:>12} (seat {}): {:2} win(s)", kinds[seat.index()].name(), seat.0, wins[seat.index()]);
        }
        println!("{:>12}         : {:2}", "draws", draws);
        println!("{:>12}         : {:2}", "collapses", collapses);
        println!("{:>12}         : {}", "median collapse turn", median(&mut collapse_turns));
        println!("{:>12}         : {}", "median turn of first Colony", median(&mut first_colony));
        println!("{:>12}         : {}", "states that threw off a controller", throw_offs);
        peaks.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        println!("{:>12}         : {:.1}", "median peak Unrest", peaks.get(peaks.len() / 2).copied().unwrap_or(0.0));
        println!("{:>12}         : {}", "Constabularies built by the AIs", constabularies);
        println!("{:>12}         : {}", "Relief orders paid by the AIs", relief);
        println!("{:>12}         : {:.1}", "population moved by refugees", moved);
        // Ticket #53: the Blame game, per seat, and how much the world developed on its own.
        println!();
        println!("{:>12} | {:>12} | {:>11} | {:>10}", "seat", "Faction", "median Blame", "share");
        for seat in Seat::ALL {
            let i = seat.index();
            println!(
                "{:>12} | {:>12} | {:>12.0} | {:>9.2} | thresholds x{:.2}",
                seat.0,
                kinds[i].name(),
                median_f(&mut blame[i]),
                median_f(&mut shares[i]),
                median_f(&mut mults[i])
            );
        }
        println!("{:>12}         : {}", "median neutral developments a game", median(&mut developments));
    }
}

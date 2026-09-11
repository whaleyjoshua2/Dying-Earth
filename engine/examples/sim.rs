//! Quick headless runner (ticket #50: four seats, all AI):
//! `cargo run -p dying-earth-engine --example sim -- <seed> [--player=<faction>] [--count=N] [--log]`
//! Faction ids are the full names: custodians, prospectors, arkwrights, archivists.

use dying_earth_engine::climate::LastTurn;
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
    // Ticket #54.
    let mut scrubbers = 0u32;
    let mut mothballs = 0u32;
    let mut restarts = 0u32;
    let mut decommissions = 0u32;
    let mut leapfrogs = 0u32;
    let mut strip_permits = 0u32;
    let mut runs: [Vec<u32>; SEAT_COUNT] = std::array::from_fn(|_| Vec::new());
    let mut net_twelve: Vec<f64> = Vec::new();
    let mut net_end: Vec<f64> = Vec::new();
    // Ticket #55: how many seeds each Break fired in and on what turn, and the Last Turn shown at
    // turn 1 and at turn 12.
    let mut break_turns: Vec<Vec<u32>> = tables.climate.breaks.iter().map(|_| Vec::new()).collect();
    let mut last_turn_one: Vec<u32> = Vec::new();
    let mut last_turn_twelve: Vec<u32> = Vec::new();
    let (mut gone_one, mut gone_twelve) = (0u32, 0u32);
    let (mut safe_one, mut safe_twelve) = (0u32, 0u32);
    // Ticket #56: the sea and the ice.
    let (mut sea_walls_built, mut sea_walls_spent) = (0u32, 0u32);
    let mut coastal_lost: Vec<u32> = Vec::new();
    let mut drowned: Vec<u32> = Vec::new();
    let mut ice_turns: Vec<u32> = Vec::new();
    let mut antarctic_colonies = 0u32;
    // Ticket #57: the first Colony in the Mars system, the Colonists off Earth at the end, and the
    // turn the Mars launch window falls on.
    let mut first_mars: Vec<u32> = Vec::new();
    let mut off_earth_at_end: Vec<u32> = Vec::new();
    let mut window_turn = 0u32;
    // Ticket #58: what the Moments did, over every seed.
    let (mut moments_earned, mut moments_shown, mut turns_with_moment, mut report_turns) = (0u32, 0u32, 0u32, 0u32);
    let mut most_in_a_turn = 0u32;
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
        scrubbers += r.scrubbers;
        mothballs += r.mothballs;
        restarts += r.restarts;
        decommissions += r.decommissions;
        leapfrogs += r.leapfrogs;
        strip_permits += r.strip_permits;
        for seat in Seat::ALL {
            runs[seat.index()].push(r.longest_stabilization[seat.index()]);
        }
        if let Some(n) = r.net_at_twelve {
            net_twelve.push(n);
        }
        net_end.push(r.net_at_end);
        sea_walls_built += r.sea_walls_built;
        sea_walls_spent += r.sea_walls_spent;
        coastal_lost.push(r.coastal_slots_lost);
        drowned.push(r.facilities_drowned);
        if let Some(t) = r.antarctica_turn {
            ice_turns.push(t);
        }
        antarctic_colonies += r.antarctic_colonies;
        if let Some(t) = r.first_mars_colony_turn {
            first_mars.push(t);
        }
        off_earth_at_end.push(r.colonists_off_earth.iter().sum());
        window_turn = r.window_turn;
        moments_earned += r.moments_earned;
        moments_shown += r.moments_shown;
        turns_with_moment += r.turns_with_moment;
        most_in_a_turn = most_in_a_turn.max(r.most_moments_in_a_turn);
        report_turns += r.last_turn;
        for (i, t) in r.break_turns.iter().enumerate() {
            if let Some(t) = t {
                break_turns[i].push(*t);
            }
        }
        for (shown, turns, gone, safe) in [
            (Some(r.last_turn_at_one), &mut last_turn_one, &mut gone_one, &mut safe_one),
            (r.last_turn_at_twelve, &mut last_turn_twelve, &mut gone_twelve, &mut safe_twelve),
        ] {
            match shown {
                Some(LastTurn::Turn(t)) => turns.push(t),
                Some(LastTurn::TooLate) => *gone += 1,
                Some(LastTurn::NoCollapse) => *safe += 1,
                None => {}
            }
        }
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
        println!(
            "         | scrubbers {} | mothballs {} | restarts {} | decommissions {} | leapfrogs {} | strip permits {} | longest stabilization {:?} | net at 12 {:?} | net at end {:+.1}",
            r.scrubbers,
            r.mothballs,
            r.restarts,
            r.decommissions,
            r.leapfrogs,
            r.strip_permits,
            r.longest_stabilization,
            r.net_at_twelve.map(|n| format!("{n:+.1}")),
            r.net_at_end
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
        // Ticket #54: what the new orders did over the batch.
        println!();
        println!("{:>12}         : {}", "Scrubbers built", scrubbers);
        println!("{:>12}         : {}", "Mothballs", mothballs);
        println!("{:>12}         : {}", "Restarts", restarts);
        println!("{:>12}         : {}", "Decommissions", decommissions);
        println!("{:>12}         : {}", "Leapfrogs", leapfrogs);
        println!("{:>12}         : {}", "Strip Permits", strip_permits);
        for seat in Seat::ALL {
            let i = seat.index();
            let max = runs[i].iter().copied().max().unwrap_or(0);
            let label = format!("{} longest Stabilization run", kinds[i].name());
            println!("{:>12}         : median {}, max {}", label, median(&mut runs[i]), max);
        }
        println!("{:>12}         : {:+.1}", "median world net Emissions at turn 12", median_f(&mut net_twelve));
        println!("{:>12}         : {:+.1}", "median world net Emissions at the end", median_f(&mut net_end));
        // Ticket #55: the Breaks, and the Last Turn the panel showed.
        println!();
        println!("{:>24} | {:>6} | {:>5} | {:>12}", "Break", "at", "seeds", "median turn");
        for (i, b) in tables.climate.breaks.iter().enumerate() {
            println!(
                "{:>24} | {:>+6.1} | {:>5} | {:>12}",
                b.name,
                b.temperature,
                break_turns[i].len(),
                median(&mut break_turns[i])
            );
        }
        println!(
            "{:>12}         : median {} ({} seeds), cuts gone in {}, no Collapse on the path in {}",
            "Last Turn at turn 1",
            median(&mut last_turn_one),
            last_turn_one.len(),
            gone_one,
            safe_one
        );
        println!(
            "{:>12}         : {} built, {} spent absorbing a threshold",
            "Sea Walls", sea_walls_built, sea_walls_spent
        );
        println!("{:>12}         : {}", "median coastal slots lost a game", median(&mut coastal_lost));
        println!("{:>12}         : {}", "median Facilities destroyed by the sea", median(&mut drowned));
        println!(
            "{:>12}         : median {} ({} of {} seeds opened it)",
            "turn Antarctica opened",
            median(&mut ice_turns),
            ice_turns.len(),
            count
        );
        println!("{:>12}         : {}", "Antarctic Colonies founded", antarctic_colonies);
        // Ticket #57.
        println!(
            "{:>12}         : median {} ({} of {} seeds founded one)",
            "turn of the first Mars-system Colony",
            median(&mut first_mars),
            first_mars.len(),
            count
        );
        println!("{:>12}         : median {}", "Colonists off Earth at the end, all seats", median(&mut off_earth_at_end));
        println!("{:>12}         : turn {}", "the Mars launch window", window_turn);
        // Ticket #58.
        println!(
            "{:>12}         : {} earned, {} shown ({:.2} a turn over {} turns), {} of those turns stopped for one, most in a turn {}",
            "Moments", moments_earned, moments_shown, moments_shown as f64 / report_turns.max(1) as f64, report_turns, turns_with_moment, most_in_a_turn
        );
        println!(
            "{:>12}         : median {} ({} seeds), cuts gone in {}, no Collapse on the path in {}",
            "Last Turn at turn 12",
            median(&mut last_turn_twelve),
            last_turn_twelve.len(),
            gone_twelve,
            safe_twelve
        );
    }
}

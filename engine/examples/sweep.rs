//! Ticket #27: sweep the Climate knobs and report when a four-Faction game collapses (ticket #50).
//! Ticket #60: the Breaks moved the clock as much as the knobs did, so the two figures that do the
//! moving are overridable too -- the Permafrost Thaw's `emissions_per_turn` and the Sink Weakens'
//! `sink_after` -- and a single cell can print the four-way balance block with `--balance`.
//!
//! `cargo run -p dying-earth-engine --example sweep -- [seeds] [--player=<faction>]
//!   [--start=<state>] [--sinks=6,8] [--steps=150,180] [--permafrost=4.0,2.5]
//!   [--sink-after=4.0,5.0] [--balance]`
//!
//! THE TARGET this sweep is read against, as ticket #60 restated it from #46 and #53: every seating
//! stays hot to the end -- a median end Temperature of +2.5 to +2.9 C where it does not collapse --
//! and Collapse is a real threat but not a certainty, roughly half to three quarters of seeds
//! collapsing with a median Collapse turn of 19 or later. The five seatings it is measured over are
//! the Custodians, the Prospectors and the Arkwrights in East Asia, and the Custodians and the
//! Archivists from Europe. #60's chosen cell (step 180, sink 6, the Breaks untouched) meets it for
//! one of those five and comes close on a second; the three that miss, and why, are in the 0.05 dev
//! diary and in `climate.toml`'s comments.

use dying_earth_engine::data::BreakEffect;
use dying_earth_engine::data::{default_data_dir, Tables};
use dying_earth_engine::ids::{FactionKind, Seat, StateId};
use dying_earth_engine::state::Outcome;
use std::sync::Arc;

fn list(flag: &str, default: &[f64]) -> Vec<f64> {
    std::env::args()
        .find_map(|a| a.strip_prefix(flag).map(|s| s.split(',').filter_map(|x| x.parse().ok()).collect::<Vec<f64>>()))
        .unwrap_or_else(|| default.to_vec())
}

fn median_u(v: &mut [u32]) -> String {
    if v.is_empty() {
        return "-".to_string();
    }
    v.sort_unstable();
    v[v.len() / 2].to_string()
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
    // Ticket #60: the Breaks' own two figures. Every other Break figure is left where `climate.toml`
    // has it -- the Coral Die-off, Ice Sheets and the Amazon pulse are not swept here.
    let permafrosts = list("--permafrost=", &[f64::NAN]);
    let sink_afters = list("--sink-after=", &[f64::NAN]);
    let balance = std::env::args().any(|a| a == "--balance");
    let base = Tables::load(&default_data_dir()).expect("tables");
    println!("seat 0: {} starting in {start:?}", player.name());
    println!("sink  step  perm  after | collapses/{seeds} | collapse turns (median, min..max) | temp at end (median) | wins by seat");
    for &sink in &sinks {
        for &step in &steps {
            for &permafrost in &permafrosts {
                for &sink_after in &sink_afters {
                    let mut t = base.clone();
                    t.climate.natural_sink = sink;
                    t.climate.ppm_step = step;
                    for b in &mut t.climate.breaks {
                        match b.effect {
                            BreakEffect::EmissionsPerTurn if permafrost.is_finite() => b.emissions = permafrost,
                            BreakEffect::WeakenSink if sink_after.is_finite() => b.sink_after = sink_after,
                            _ => {}
                        }
                    }
                    let shown = |v: f64, from: f64| if v.is_finite() { v } else { from };
                    let perm_shown = shown(permafrost, t.climate.breaks.iter().find(|b| b.effect == BreakEffect::EmissionsPerTurn).map(|b| b.emissions).unwrap_or(0.0));
                    let after_shown = shown(sink_after, t.climate.breaks.iter().find(|b| b.effect == BreakEffect::WeakenSink).map(|b| b.sink_after).unwrap_or(0.0));
                    let tables = Arc::new(t);
                    let mut turns = Vec::new();
                    let mut temps = Vec::new();
                    let mut wins = [0u32; 4];
                    // Ticket #60: the balance counters, for the one-cell runs of the balance report.
                    let (mut draws, mut scrubbers, mut leapfrogs, mut constabularies, mut sea_walls) = (0u32, 0u32, 0u32, 0u32, 0u32);
                    let (mut first_colony, mut off_earth, mut techs) = (Vec::new(), Vec::new(), Vec::new());
                    let mut breaks_fired = vec![0u32; tables.climate.breaks.len()];
                    let mut highest_rung = 0u32;
                    let mut victory_met: Vec<String> = Vec::new();
                    // Ticket #67 (version 0.05.5): whether the Mars system is reached now that the
                    // game holds three windows, and how many Antarctic Colonies are founded.
                    let (mut mars_turns, mut antarctic) = (Vec::new(), 0u32);
                    // Ticket #68: how far the Archivists' Archive gets.
                    let (mut archive_built, mut archive_complete, mut archive_funds) = (Vec::new(), Vec::new(), Vec::new());
                    // Ticket #69: the neutral Labs' Research and the Sea Wall's Tech.
                    let (mut neutral_research, mut coastal_engineering) = (Vec::new(), Vec::new());
                    // Ticket #70: what the sea took.
                    let (mut slots_lost, mut drowned) = (Vec::new(), Vec::new());
                    // Ticket #72: the Prospectors' Fund.
                    let mut venture = Vec::new();
                    // Ticket #76: the deck.
                    let (mut cards_drawn, mut deck_empty) = (Vec::new(), 0u32);
                    // Ticket #73: Emigrants.
                    let (mut emigrant_batches, mut by_sea) = (0u32, 0u32);
                    // Ticket #80: Observatories and Research off Earth, per seat.
                    let mut observatories = [0u32; 4];
                    let mut research_off_earth: [Vec<u32>; 4] = Default::default();
                    // Ticket #82: Module-turns doubled by an idle Facility on Earth, per seat.
                    let mut doubled_turns: [Vec<u32>; 4] = Default::default();
                    // Ticket #84: the turn each seat's Victory gate completed, over the seeds it did.
                    let mut gate_turns: [Vec<u32>; 4] = Default::default();
                    // Ticket #75: seat 0's start state.
                    let mut home_lost = Vec::new();
                    for seed in 1..=seeds {
                        let r = dying_earth_engine::sim::run_from(tables.clone(), seed, player, start);
                        match r.outcome {
                            Some(Outcome::Collapse) => turns.push(r.last_turn),
                            Some(Outcome::Win { seat, .. }) => wins[seat.index()] += 1,
                            Some(Outcome::Draw { .. }) => draws += 1,
                            _ => {}
                        }
                        temps.push(r.temperature);
                        scrubbers += r.scrubbers;
                        leapfrogs += r.leapfrogs;
                        constabularies += r.constabularies;
                        sea_walls += r.sea_walls_built;
                        if let Some(t) = r.first_colony_turn {
                            first_colony.push(t);
                        }
                        off_earth.push(r.colonists_off_earth.iter().sum::<u32>());
                        techs.push(r.techs_completed);
                        highest_rung = highest_rung.max(r.highest_rung);
                        // Ticket #80: Observatories at the end and Research made off Earth, per seat.
                        for s in 0..4 {
                            observatories[s] += r.observatories[s];
                            research_off_earth[s].push(r.research_off_earth[s].max(0) as u32);
                            doubled_turns[s].push(r.doubled_module_turns[s].max(0) as u32);
                            if let Some(t) = r.gate_turn[s] {
                                gate_turns[s].push(t);
                            }
                        }
                        if let Some(t) = r.first_mars_colony_turn {
                            mars_turns.push(t);
                        }
                        antarctic += r.antarctic_colonies;
                        if let Some(t) = r.archive_built_turn {
                            archive_built.push(t);
                        }
                        if let Some(t) = r.archive_complete_turn {
                            archive_complete.push(t);
                        }
                        archive_funds.push(r.archive_fund_at_end.max(0) as u32);
                        neutral_research.push(r.neutral_research.max(0) as u32);
                        slots_lost.push(r.coastal_slots_lost);
                        drowned.push(r.facilities_drowned);
                        venture.push(r.venture_fund_at_end.max(0) as u32);
                        cards_drawn.push(r.cards_drawn);
                        if r.deck_empty {
                            deck_empty += 1;
                        }
                        emigrant_batches += r.emigrant_batches;
                        by_sea += r.antarctic_by_sea;
                        if let Some(t) = r.start_state_lost_turn {
                            home_lost.push(t);
                        }
                        if let Some(t) = r.coastal_engineering_turn {
                            coastal_engineering.push(t);
                        }
                        if let Some((seat, kind)) = r.victory_met {
                            victory_met.push(format!("seed {seed} {} (seat {})", kind.name(), seat.0));
                        }
                        for (i, t) in r.break_turns.iter().enumerate() {
                            if t.is_some() {
                                breaks_fired[i] += 1;
                            }
                        }
                    }
                    turns.sort_unstable();
                    temps.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    let median = turns.get(turns.len() / 2).map(|t| t.to_string()).unwrap_or("-".into());
                    let range = if turns.is_empty() { "-".to_string() } else { format!("{}..{}", turns[0], turns[turns.len() - 1]) };
                    println!(
                        "{sink:4.0}  {step:4.0}  {perm_shown:4.1}  {after_shown:5.1} | {:2}/{seeds} | {median:>3} ({range}) | {:+.2} | {:?}",
                        turns.len(),
                        temps[temps.len() / 2],
                        wins
                    );
                    if balance {
                        let kinds: Vec<String> = {
                            let mut ks: Vec<FactionKind> = vec![player];
                            ks.extend(FactionKind::ALL.into_iter().filter(|k| *k != player));
                            ks.iter().map(|k| k.name().to_string()).collect()
                        };
                        for s in Seat::ALL {
                            println!("      {:>12} (seat {}): {:2} win(s)", kinds[s.index()], s.0, wins[s.index()]);
                        }
                        println!("      draws {draws}, collapses {}/{seeds}, median collapse turn {median}", turns.len());
                        println!("      median turn of first Colony {}", median_u(&mut first_colony));
                        println!("      median Colonists off Earth at the end, all seats {}", median_u(&mut off_earth));
                        println!("      Scrubbers {scrubbers}, Leapfrogs {leapfrogs}, Constabularies {constabularies}, Sea Walls {sea_walls}");
                        println!("      Techs: median {} completed, highest rung reached {highest_rung}", median_u(&mut techs));
                        println!(
                            "      Observatories standing at the end, all seeds, by seat {observatories:?}; median Research made off Earth a game, by seat {:?}",
                            research_off_earth.iter_mut().map(|v| median_u(v)).collect::<Vec<_>>()
                        );
                        println!(
                            "      Production Moved: median Module-turns doubled by an idle Facility a game, by seat {:?}",
                            doubled_turns.iter_mut().map(|v| median_u(v)).collect::<Vec<_>>()
                        );
                        println!(
                            "      Victory gates: completed in {:?} seeds by seat, median turn {:?}",
                            gate_turns.iter().map(|v| v.len()).collect::<Vec<_>>(),
                            gate_turns.iter_mut().map(|v| median_u(v)).collect::<Vec<_>>()
                        );
                        println!(
                            "      Mars system: a Colony founded in {}/{seeds} seeds, median first turn {}; Antarctic Colonies founded {antarctic}",
                            mars_turns.len(),
                            median_u(&mut mars_turns)
                        );
                        println!(
                            "      The Archive: standing in {}/{seeds} seeds (median turn {}), complete in {}/{seeds} (median turn {}), median fund at the end {}",
                            archive_built.len(),
                            median_u(&mut archive_built),
                            archive_complete.len(),
                            median_u(&mut archive_complete),
                            median_u(&mut archive_funds)
                        );
                        println!(
                            "      Neutral Labs paid a median {} Research a game; Coastal Engineering complete in {}/{seeds} seeds (median turn {})",
                            median_u(&mut neutral_research),
                            coastal_engineering.len(),
                            median_u(&mut coastal_engineering)
                        );
                        println!("      The sea: median {} coastal slots lost a game, {} Facilities drowned", median_u(&mut slots_lost), median_u(&mut drowned));
                        println!("      The Prospectors' Venture Capital Fund at the end: median {}", median_u(&mut venture));
                        println!("      The deck: median {} cards drawn a game, empty at the end in {deck_empty}/{seeds} seeds", median_u(&mut cards_drawn));
                        println!("      Emigrants: {emigrant_batches} batches mustered, {by_sea} Antarctic Colonies founded by sea");
                        println!("      Seat 0 lost its start state in {}/{seeds} seeds (median turn {})", home_lost.len(), median_u(&mut home_lost));
                        println!(
                            "      Victory Conditions met outright: {}",
                            if victory_met.is_empty() { "none in any seed".to_string() } else { victory_met.join(", ") }
                        );
                        let fired: Vec<String> = tables.climate.breaks.iter().zip(&breaks_fired).map(|(b, n)| format!("{} {n}/{seeds}", b.name)).collect();
                        println!("      Breaks fired: {}", fired.join(", "));
                    }
                }
            }
        }
    }
}

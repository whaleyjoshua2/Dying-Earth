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
    // Ticket #241 (version 0.08.3): `--seatings` runs all four Factions as seat 0 in one
    // process and prints ONE win column of `4 x seeds` at the end. Version 0.08.2 ended by
    // naming this as missing: its figures were per seating, and a win column of 80 had to be
    // added up by hand from four of 20, which is how a baseline gets misquoted.
    let all_seatings = std::env::args().any(|a| a == "--seatings");
    let players: Vec<FactionKind> = if all_seatings { FactionKind::ALL.to_vec() } else { vec![player] };
    // Indexed by the Faction's place in `FactionKind::ALL`, never by seat: seat 0 is a
    // different Faction in every seating, which is the whole point of running four.
    let mut all_wins = [0u32; 4];
    let (mut all_games, mut all_collapses) = (0u32, 0u32);
    for player in players {
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
                        let (mut slots_lost, mut drowned, mut converted) = (Vec::new(), Vec::new(), Vec::new());
                        // Ticket #72: the Prospectors' Fund.
                        let mut venture = Vec::new();
                        // Ticket #227 (version 0.08.2): the six figures the version's rules depend on.
                        let mut blame_shares: [Vec<f64>; 4] = Default::default();
                        // Ticket #272 (version 0.08.4): the figures this version's tickets want read.
                        let mut blame_ppm: [Vec<f64>; 4] = Default::default();
                        let mut credit_ppm: [Vec<f64>; 4] = Default::default();
                        let mut smeared_ppm: [Vec<f64>; 4] = Default::default();
                        let mut cleaned_ppm: [Vec<f64>; 4] = Default::default();
                        let mut credits_bought = [0.0f64; 4];
                        let mut credits_sold = [0.0f64; 4];
                        let mut agitates = [0u32; 4];
                        let (mut blockade_suffered, mut blockade_imposed) = ([0u32; 4], [0u32; 4]);
                        let (mut levies, mut neutral_holds) = (0u32, 0u32);
                        let mut warc = dying_earth_engine::state::WarCounters::default();
                        let mut war_ppm: [Vec<f64>; 4] = Default::default();
                        let mut war_nobody: Vec<f64> = Vec::new();
                        let mut walls_standing = 0u32;
                        let mut walls_held = 0u32;
                        let mut no_target = 0u32;
                        let mut fund_met = 0u32;
                        let mut rel_end: Vec<i64> = Vec::new();
                        let mut rel_floored = 0u32;
                        let mut accords = 0u32;
                        let mut accord_terms = [0u32; 4];
                        let mut bought: [i64; 4] = [0; 4];
                        let mut sold: [i64; 4] = [0; 4];
                        let mut takes = 0u32;
                        let mut tree_turns: Vec<u32> = Vec::new();
                        // Ticket #76: the deck.
                        let (mut cards_drawn, mut deck_empty) = (Vec::new(), 0u32);
                        // Ticket #73: Emigrants.
                        let (mut emigrant_batches, mut by_sea) = (0u32, 0u32);
                        // Ticket #80: Observatories and Research off Earth, per seat.
                        let mut observatories = [0u32; 4];
                        let mut research_off_earth: [Vec<u32>; 4] = Default::default();
                        // Ticket #82: Module-turns doubled by an idle Facility on Earth, per seat.
                        let mut doubled_turns: [Vec<u32>; 4] = Default::default();
                        // Ticket #332 (version 0.09.0): Widgets made, applied and lost a game, per
                        // seat, and each game's median queue depth at Resolution.
                        let mut widgets_made: [Vec<u32>; 4] = Default::default();
                        let mut widgets_applied: [Vec<u32>; 4] = Default::default();
                        let mut widgets_lost: [Vec<u32>; 4] = Default::default();
                        let mut queue_depths: Vec<u32> = Vec::new();
                        // Ticket #84: the turn each seat's Victory gate completed, over the seeds it did.
                        let mut gate_turns: [Vec<u32>; 4] = Default::default();
                        // Ticket #86: Colonists lost in transit to crowding, per seat, over the batch.
                        let mut lost_in_transit = [0i64; 4];
                        // Ticket #87: stranded Ships at the end, Refuel orders and stations off Earth.
                        let mut stranded = [0u32; 4];
                        let (mut refuels, mut partner_refuels, mut stations_off_earth) = (0u32, 0u32, 0u32);
                        // Ticket #290 (version 0.08.6): Modules beyond the Core on a starting
                        // station at the end of turn three, per seat, summed over the batch.
                        let mut opening_modules = [0u32; 4];
                        // Ticket #88: Colonies with two or more working Mines, and Modules per ground Colony.
                        let (mut deep_colonies, mut ground_modules, mut ground_colonies) = (0u32, 0u32, 0u32);
                        // Ticket #241 (version 0.08.3): the figures 0.08.2 named as missing, and this
                        // version's own.
                        let (mut d_made, mut d_spent): (Vec<[i64; 4]>, Vec<[i64; 4]>) = (vec![], vec![]);
                        let mut m_made: Vec<[i64; 4]> = vec![];
                        let (mut mines_done, mut factories_done) = (0u32, 0u32);
                        let mut d_mean: Vec<[f64; 4]> = vec![];
                        let mut d_below = [0u32; 4];
                        let (mut ex_calls, mut ex_pioneers, mut acc_struck) = (0u32, 0u32, 0u32);
                        let mut uniq = [0u32; 4];
                        // Ticket #324 (version 0.08.8): Batteries standing at the end over the batch, by seat.
                        let mut batteries = [0u32; 4];
                        // Ticket #89: Solar Arrays standing at the end over the batch.
                        let mut solar_arrays = 0u32;
                        // Ticket #90: Trade Posts standing at the end over the batch.
                        let mut trade_posts = 0u32;
                        // Ticket #92: Mass Drivers at the end, and Colonies on Phobos or Deimos.
                        let (mut mass_drivers, mut martian_moon_colonies) = (0u32, 0u32);
                        // Ticket #93: stations at Venus at the end, and Colonists living there.
                        let (mut venus_stations, mut venus_colonists) = (0u32, 0u32);
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
                            for i in 0..4 {
                                blame_shares[i].push(r.blame_share[i]);
                                blame_ppm[i].push(r.blame[i]);
                                credit_ppm[i].push(r.blame_credit[i]);
                                smeared_ppm[i].push(r.blame_smeared[i]);
                                cleaned_ppm[i].push(r.blame_cleaned[i]);
                                credits_bought[i] += r.credits_bought[i];
                                credits_sold[i] += r.credits_sold[i];
                                agitates[i] += r.agitates[i];
                                blockade_suffered[i] += r.blockade_suffered[i];
                                blockade_imposed[i] += r.blockade_imposed[i];
                                war_ppm[i].push(r.war_ppm[i]);
                                bought[i] += r.bought[i];
                                sold[i] += r.sold[i];
                            }
                            rel_end.extend(r.relations_end.iter().copied());
                            rel_floored += r.relations_floored;
                            accords += r.accords_end;
                            for (i, a) in accord_terms.iter_mut().enumerate() {
                                *a += r.accord_terms[i];
                            }
                            takes += r.influence_transfers;
                            if let Some(t) = r.tree_done_turn {
                                tree_turns.push(t);
                            }
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
                                widgets_made[s].push(r.widgets_made[s].max(0) as u32);
                                widgets_applied[s].push(r.widgets_applied[s].max(0) as u32);
                                widgets_lost[s].push(r.widgets_lost[s].max(0) as u32);
                                if let Some(t) = r.gate_turn[s] {
                                    gate_turns[s].push(t);
                                }
                                lost_in_transit[s] += r.lost_in_transit[s];
                                stranded[s] += r.stranded_at_end[s];
                            }
                            queue_depths.push(r.queue_depth_median);
                            refuels += r.refuels;
                            partner_refuels += r.partner_refuels;
                            stations_off_earth += r.stations_off_earth;
                            for (i, n) in opening_modules.iter_mut().enumerate() {
                                *n += r.opening_modules[i];
                            }
                            deep_colonies += r.deep_colonies;
                            ground_modules += r.ground_modules;
                            ground_colonies += r.ground_colonies;
                            solar_arrays += r.solar_arrays;
                            d_made.push(r.ducats_made);
                            m_made.push(r.materials_made);
                            mines_done += r.mines_completed;
                            factories_done += r.factories_completed;
                            d_spent.push(r.ducats_spent);
                            d_mean.push(r.directive_mean);
                            ex_calls += r.exodus_calls;
                            ex_pioneers += r.exodus_pioneers;
                            acc_struck += r.accords_struck;
                            for i in 0..4 {
                                d_below[i] += r.directive_turns_below[i];
                                uniq[i] += r.uniques[i];
                                batteries[i] += r.batteries[i];
                            }
                            trade_posts += r.trade_posts;
                            mass_drivers += r.mass_drivers;
                            martian_moon_colonies += r.martian_moon_colonies;
                            venus_stations += r.venus_stations;
                            venus_colonists += r.venus_colonists;
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
                            converted.push(r.slots_converted);
                            venture.push(r.venture_fund_at_end.max(0) as u32);
                            if r.venture_fund_at_end as f64 >= base.faction(FactionKind::Prospectors).victory_first.bar {
                                fund_met += 1;
                            }
                            walls_standing += r.sea_walls_standing;
                            walls_held += r.sea_walls_spent;
                            no_target += r.events_no_target;
                            cards_drawn.push(r.cards_drawn);
                            war_nobody.push(r.war_ppm_nobody);
                            levies += r.levies_raised;
                            neutral_holds += r.neutral_holds;
                            warc.add(&r.war);
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
                        // Ticket #241 (version 0.08.3): fold this cell into the all-seatings
                        // tally, keyed by the Faction rather than by the seat it happens to sit in.
                        {
                            let order: Vec<FactionKind> = {
                                let mut ks: Vec<FactionKind> = vec![player];
                                ks.extend(FactionKind::ALL.into_iter().filter(|k| *k != player));
                                ks
                            };
                            for s in Seat::ALL {
                                let at = FactionKind::ALL.into_iter().position(|k| k == order[s.index()]).unwrap_or(0);
                                all_wins[at] += wins[s.index()];
                            }
                            all_games += seeds as u32;
                            all_collapses += turns.len() as u32;
                        }
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
                            // Ticket #332 (version 0.09.0): Widgets, the work half of every build.
                            println!(
                                "      Widgets a game (median), by seat: made {:?}, applied {:?}, lost {:?}; median queue depth at Resolution {}",
                                widgets_made.iter_mut().map(|v| median_u(v)).collect::<Vec<_>>(),
                                widgets_applied.iter_mut().map(|v| median_u(v)).collect::<Vec<_>>(),
                                widgets_lost.iter_mut().map(|v| median_u(v)).collect::<Vec<_>>(),
                                median_u(&mut queue_depths)
                            );
                            println!(
                                "      Victory gates: completed in {:?} seeds by seat, median turn {:?}",
                                gate_turns.iter().map(|v| v.len()).collect::<Vec<_>>(),
                                gate_turns.iter_mut().map(|v| median_u(v)).collect::<Vec<_>>()
                            );
                            println!("      Crowded ships: Colonists lost in transit over the batch, by seat {lost_in_transit:?}");
                            println!("      Tanks: Ships stranded at the end over the batch, by seat {stranded:?}; {refuels} Refuel orders ({partner_refuels} at a partner's station); {stations_off_earth} stations standing off Earth at the end");
                            println!("      The opening: Modules beyond the Core on a starting station at the end of turn 3 over the batch, by seat {opening_modules:?}");
                            println!(
                                "      Build it where you dig: {deep_colonies} ground Colonies with two or more working Mines at the end over the batch; {:.1} Modules per ground Colony",
                                if ground_colonies > 0 { ground_modules as f64 / ground_colonies as f64 } else { 0.0 }
                            );
                            println!("      Solar Arrays standing at the end over the batch: {solar_arrays}; Trade Posts {trade_posts}; Mass Drivers {mass_drivers}; Colonies on Phobos or Deimos {martian_moon_colonies}");
                            // Ticket #241 (version 0.08.3): the three figures version 0.08.2 ended by
                            // naming as unreported, and the three this version's own rules need.
                            let med_i = |v: &Vec<[i64; 4]>, seat: usize| { let mut c: Vec<i64> = v.iter().map(|x| x[seat]).collect(); c.sort(); if c.is_empty() { 0 } else { c[c.len() / 2] } };
                            let q_i = |v: &Vec<[i64; 4]>| [med_i(v, 0), med_i(v, 1), med_i(v, 2), med_i(v, 3)];
                            let mean_f = |v: &Vec<[f64; 4]>, seat: usize| if v.is_empty() { 0.0 } else { v.iter().map(|x| x[seat]).sum::<f64>() / v.len() as f64 };
                            println!("      Ducats over a game (median), by seat -- made {:?}, spent {:?}", q_i(&d_made), q_i(&d_spent));
                            println!("      Materials income over a game (median), by seat {:?}; Mines completed over the batch {mines_done}, Factories {factories_done}", q_i(&m_made));
                            println!("      Research Directive: mean % KEPT BACK from the shared pot, by seat [{:.0}, {:.0}, {:.0}, {:.0}]; turns under an 85% contribution over the batch {:?}",
                                mean_f(&d_mean, 0), mean_f(&d_mean, 1), mean_f(&d_mean, 2), mean_f(&d_mean, 3), d_below);
                            println!("      Accords STRUCK over the batch: {acc_struck} (against {accords} standing at the end)");
                            println!("      Exodus Calls sounded over the batch: {ex_calls}, worth {ex_pioneers} Pioneers");
                            println!("      Unique Modules standing at the end over the batch: Academy {}, Heliostat {}, Exchange {}, Chorus {}", uniq[0], uniq[1], uniq[2], uniq[3]);
                            println!("      Batteries standing at the end over the batch, by seat {batteries:?}");
                            println!("      Venus: {venus_stations} stations at the end over the batch, {venus_colonists} Colonists living there");
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
                            println!(
                                "      The sea: median {} coastal slots lost a game, {} Facilities drowned, {} inland slots turned coastal",
                                median_u(&mut slots_lost),
                                median_u(&mut drowned),
                                median_u(&mut converted)
                            );
                            println!("      The Prospectors' Venture Capital Fund at the end: median {} Ducats of the {} their Victory Condition asks", median_u(&mut venture), base.faction(FactionKind::Prospectors).victory_first.bar);
                            println!("      The deck: median {} cards drawn a game, empty at the end in {deck_empty}/{seeds} seeds", median_u(&mut cards_drawn));
                            println!("      Pioneers: {emigrant_batches} batches recruited, {by_sea} Antarctic Colonies founded by sea");
                            println!("      Seat 0 lost its start state in {}/{seeds} seeds (median turn {})", home_lost.len(), median_u(&mut home_lost));
                            println!(
                                "      Victory Conditions met outright: {}",
                                if victory_met.is_empty() { "none in any seed".to_string() } else { victory_met.join(", ") }
                            );
                            let fired: Vec<String> = tables.climate.breaks.iter().zip(&breaks_fired).map(|(b, n)| format!("{} {n}/{seeds}", b.name)).collect();
                            // Ticket #227 (version 0.08.2): the six the spec asks for, in one block.
                            let med = |v: &mut Vec<f64>| {
                                if v.is_empty() {
                                    return "-".to_string();
                                }
                                v.sort_by(|a, b| a.partial_cmp(b).unwrap());
                                format!("{:.2}", v[v.len() / 2])
                            };
                            let shares: Vec<String> = (0..4).map(|i| med(&mut blame_shares[i])).collect();
                            println!("      Blame share at the end, by seat (median): [{}]", shares.join(", "));
                            // Ticket #272 (version 0.08.4): the figures this version's tickets asked the sweep to say.
                            let med0 = |v: &mut Vec<f64>| if v.is_empty() { "-".to_string() } else { v.sort_by(|a, b| a.partial_cmp(b).unwrap()); format!("{:.0}", v[v.len() / 2]) };
                            let ppm: Vec<String> = (0..4).map(|i| med0(&mut blame_ppm[i])).collect();
                            let cred: Vec<String> = (0..4).map(|i| med0(&mut credit_ppm[i])).collect();
                            let smear: Vec<String> = (0..4).map(|i| med0(&mut smeared_ppm[i])).collect();
                            let cleaned: Vec<String> = (0..4).map(|i| med0(&mut cleaned_ppm[i])).collect();
                            println!(
                                "      Blame in ppm at the end, by seat (median): [{}]; in credit [{}]; laid on by Smear [{}]; taken off by Greenwash [{}]",
                                ppm.join(", "),
                                cred.join(", "),
                                smear.join(", "),
                                cleaned.join(", ")
                            );
                            println!(
                                "      Carbon credits over the batch: bought by seat [{}], sold by seat [{}]; Agitates landed by seat {agitates:?}",
                                credits_bought.iter().map(|v| format!("{v:.0}")).collect::<Vec<_>>().join(", "),
                                credits_sold.iter().map(|v| format!("{v:.0}")).collect::<Vec<_>>().join(", ")
                            );
                            // Ticket #278 (version 0.08.5): Colony-turns starved under a Blockade, suffered and imposed.
                            println!("      Blockade-turns over the batch: suffered by seat {blockade_suffered:?}, imposed by seat {blockade_imposed:?}");
                            // Ticket #279 (version 0.08.5): what war put in the air, by seat and nobody's.
                            let war: Vec<String> = (0..4).map(|i| med0(&mut war_ppm[i])).collect();
                            println!("      War in ppm a game, by seat (median): [{}]; nobody's (median) {}", war.join(", "), med0(&mut war_nobody));
                            // Ticket #282 (version 0.08.5): neutral states arming.
                            println!("      Neutral states: {levies} threat episodes armed for over the batch, {neutral_holds} attacks held against");
                            println!("      Sea Walls: {sea_walls} built over the batch, {walls_standing} standing at the end, {walls_held} thresholds held");
                            println!("      Events drawn with nowhere to land over the batch: {no_target}; the Fund at or past its bar in {fund_met}/{seeds} seeds");
                            let floored_pct = if rel_end.is_empty() { 0.0 } else { rel_floored as f64 * 100.0 / rel_end.len() as f64 };
                            let mut sorted = rel_end.clone();
                            sorted.sort_unstable();
                            let rel_med = if sorted.is_empty() { 0 } else { sorted[sorted.len() / 2] };
                            let hostile = rel_end.iter().filter(|v| **v <= -6).count();
                            println!(
                                "      Relations at the end over {} ordered pairs: median {rel_med}, {hostile} at Cold or worse, {:.0}% carrying a scar floor",
                                rel_end.len(),
                                floored_pct
                            );
                            println!(
                                "      Accords standing at the end: {accords} (non-aggression {}, passage {}, refuel {}, research {})",
                                accord_terms[0], accord_terms[1], accord_terms[2], accord_terms[3]
                            );
                            println!("      Trading window units, by seat: bought {bought:?}, sold {sold:?}");
                            println!("      Places taken by Influence over the batch: {takes}");
                            // Ticket #286 (version 0.08.5): the war, over the batch, by seat; printed as the military block after the Influence line.
                            // Blockade-turns are printed above under ticket #278.
                            println!(
                                "      War over the batch: Battles opened by seat {:?}, {} against neutrals; attacks in orbit {:?}; marches on neutrals {:?}, on held Regions {:?}",
                                warc.battles, warc.battles_vs_neutral, warc.orbit_attacks, warc.marches_neutral, warc.marches_held
                            );
                            println!(
                                "      Armies built {:?}, lost {:?}, Standing Armies lost {}; warships built {:?}, lost {:?}; Occupations begun {:?}, broken {:?}; places taken by force {:?}",
                                warc.armies_built, warc.armies_lost, warc.standing_armies_lost, warc.warships_built, warc.warships_lost, warc.occupations_begun, warc.occupations_broken, warc.takes_by_force
                            );
                            // Ticket #334 (version 0.09.0): Armies are raised from people, and the
                            // computer's raises refused for want of them are counted.
                            println!("      Army raises refused for want of people, by seat {:?}", warc.army_raises_refused_people);
                            // Ticket #295 (version 0.08.6): the escapes, never counted before the
                            // disengage figure was nudged.
                            println!(
                                "      Escapes over the batch: units escaped by seat {:?}, neutral {}; Battles with an escape {} of {}",
                                warc.escapes, warc.escapes_neutral, warc.battles_with_escape, warc.battles.iter().sum::<u32>()
                            );
                            // Ticket #297 (version 0.08.6): Dig In orders committed, by seat.
                            println!("      Dig In orders over the batch, by seat {:?}", warc.dig_ins);
                            // Ticket #300 (version 0.08.6): Armies landed from a Carrier, by seat.
                            println!("      Armies landed at a Colony over the batch, by seat {:?}", warc.armies_landed);
                            // Ticket #319 (version 0.08.8): interceptions fought, by the intercepting seat.
                            println!("      Interceptions over the batch, by seat {:?}", warc.interceptions);
                            // Ticket #324 (version 0.08.8): Batteries lost in a Battle, by the seat that held them.
                            println!("      Batteries lost over the batch, by seat {:?}", warc.batteries_lost);
                            // Ticket #328 (version 0.08.8): Bombards and what they burned, by the bombarder.
                            println!("      Bombards over the batch, by seat {:?}; Modules burned {:?}", warc.bombards, warc.modules_burned);
                            // Ticket #335 (version 0.09.0): orbit changes, and Blockades -- every
                            // one of which a human could now give by the same path, where before
                            // this ticket no human player could give one at all.
                            println!("      Orbit changes over the batch, by seat {:?}; Blockades ordered {:?}", warc.orbit_changes, warc.blockades_ordered);
                            println!("      The whole Tech Tree completed in {}/{seeds} seeds (median turn {})", tree_turns.len(), median_u(&mut tree_turns));
                            println!("      Breaks fired: {}", fired.join(", "));
                        }
                    }
                }
            }
        }
    }
    if all_seatings {
        println!();
        println!("ACROSS ALL FOUR SEATINGS, {all_games} games:");
        for (i, k) in FactionKind::ALL.into_iter().enumerate() {
            println!("  {:>12}: {:2} win(s) of {all_games}", k.name(), all_wins[i]);
        }
        println!("  collapses {all_collapses} of {all_games}");
    }
}

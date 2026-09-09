//! The formula tests spec 19.4 asks for, one per pinned rule.

use dying_earth_engine::combat::{self, Combatant, Dice};
use dying_earth_engine::data::{default_data_dir, Tables};
use dying_earth_engine::*;
use std::collections::VecDeque;
use std::sync::Arc;

fn tables() -> Arc<Tables> {
    Arc::new(Tables::load(&default_data_dir()).expect("tables load"))
}

/// A player Custodian in Asia against an AI Prospector, before the first turn runs, on a bare board:
/// the start Facilities of ticket #24 are stripped (Launch Sites stay) so each test places exactly
/// the buildings it reasons about. `fresh()` keeps the real start.
fn game() -> Game {
    let mut g = fresh();
    for s in &mut g.states {
        s.facilities.retain(|f| f.kind == FacilityKind::LaunchSite);
    }
    g
}

fn fresh() -> Game {
    Game::new(tables(), NewGame { seed: 7, seats: [(FactionKind::Custodians, false), (FactionKind::Prospectors, true)], player_start: StateId::Asia })
}

fn facility(kind: FacilityKind) -> Facility {
    Facility { kind, online: true, offline_until_resolution: false }
}

fn colony(g: &mut Game, seat: Seat, body: BodyId, modules: &[ModuleKind], colonists: u32) -> ColonyId {
    let id = ColonyId(g.fresh_id());
    let slot = g.free_slots_on(body)[0];
    g.colonies.push(Colony {
        id,
        body,
        slot,
        control: Control::Controlled(seat),
        modules: modules.iter().map(|k| Module::new(*k)).collect(),
        colonists,
        queue: Vec::new(),
        grid_failed: false,
        founded_turn: 1,
    });
    id
}

// ---------------------------------------------------------------- 7.2 Income shortfall order

#[test]
fn income_shortfall_shuts_highest_upkeep_first_modules_before_facilities_then_alphabetical() {
    let mut g = game();
    g.seats[0].stockpile.energy = 0;
    let st = g.state_mut(StateId::Asia);
    st.facilities.clear();
    st.facilities.push(facility(FacilityKind::Factory)); // upkeep 2
    st.facilities.push(facility(FacilityKind::ResearchLab)); // upkeep 3
    st.facilities.push(facility(FacilityKind::Refinery)); // upkeep 3
    colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Mine], 0); // upkeep 3, a Module
    let order = g.shortfall_order(Seat(0));
    assert_eq!(order, vec!["Mine", "Refinery", "Research Lab", "Factory"]);
}

#[test]
fn income_shortfall_stops_once_the_balance_is_met() {
    let mut g = game();
    g.seats[0].stockpile.energy = 4;
    let st = g.state_mut(StateId::Asia);
    st.facilities.clear();
    st.facilities.push(facility(FacilityKind::Factory)); // 2
    st.facilities.push(facility(FacilityKind::Refinery)); // 3
    st.facilities.push(facility(FacilityKind::ResearchLab)); // 3
    // 4 - 8 = -4: shut Refinery (3) -> -1, then Research Lab (3) -> 2. Factory stays on.
    assert_eq!(g.shortfall_order(Seat(0)), vec!["Refinery", "Research Lab"]);
    g.income_phase();
    let st = g.state(StateId::Asia);
    assert!(st.facilities.iter().find(|f| f.kind == FacilityKind::Factory).unwrap().online);
    assert!(!st.facilities.iter().find(|f| f.kind == FacilityKind::Refinery).unwrap().online);
    assert_eq!(g.seats[0].stockpile.energy, 2);
}

// ---------------------------------------------------------------- 11.1 Temperature lag

#[test]
fn temperature_reaches_within_a_tenth_of_target_in_two_climate_phases() {
    let mut g = game();
    // One turn of real Emissions: a quarter of a step moves the target by +0.125.
    g.climate.co2 = 420.0 + 0.25 * g.tables.climate.ppm_step;
    let target = g.target_temperature();
    assert!((target - 1.325).abs() < 1e-9);
    // Freeze the stock so only the lag acts: no producers, so the phase adds population minus sink.
    for s in &mut g.states {
        s.population = 0.0;
        s.industry_level = 0;
    }
    let before = g.climate.co2;
    g.climate_phase();
    g.climate.co2 = before;
    g.climate_phase();
    g.climate.co2 = before;
    assert!((g.climate.temperature - target).abs() <= 0.1, "temperature {} target {}", g.climate.temperature, target);
    assert!(g.climate.temperature < target, "the lag leaves it still short of the target");
}

#[test]
fn temperature_halves_the_remaining_distance_each_phase() {
    let mut g = game();
    for s in &mut g.states {
        s.population = 0.0;
        s.industry_level = 0;
    }
    // Nothing emits, so each phase takes the Sink off; the stock lands two steps above 420, target 2.2.
    let (step, sink) = (g.tables.climate.ppm_step, g.tables.climate.natural_sink);
    g.climate.co2 = 420.0 + 2.0 * step + sink;
    g.climate_phase();
    assert!((g.climate.co2 - (420.0 + 2.0 * step)).abs() < 1e-9);
    assert!((g.climate.temperature - 1.7).abs() < 1e-6, "{}", g.climate.temperature);
    g.climate.co2 = 420.0 + 2.0 * step + sink;
    g.climate_phase();
    assert!((g.climate.temperature - 1.95).abs() < 1e-6, "{}", g.climate.temperature);
}

// ---------------------------------------------------------------- 11.4 Sea level

#[test]
fn sea_level_thresholds_fire_once_per_state() {
    let mut g = game();
    for s in &mut g.states {
        s.population = 0.0;
        s.industry_level = 3;
    }
    g.climate.temperature = 1.85;
    g.climate.co2 = 420.0 + 1.5 * g.tables.climate.ppm_step; // target 1.95 keeps it above 1.8
    let asia_before = g.build_slots(StateId::Asia);
    g.climate_phase();
    assert_eq!(g.build_slots(StateId::Asia), asia_before - 2, "Asia has Coastal Exposure 2");
    assert_eq!(g.build_slots(StateId::Antarctica), 5, "Antarctica has no coast to lose");
    g.climate_phase();
    assert_eq!(g.build_slots(StateId::Asia), asia_before - 2, "the same threshold never fires twice");
}

#[test]
fn sea_level_destroys_facilities_beyond_the_slots_highest_upkeep_first() {
    let mut g = game();
    for s in &mut g.states {
        s.population = 0.0;
    }
    // Australia: size 2, industry 2 -> 4 slots, exposure 2. Fill all four.
    let st = g.state_mut(StateId::Australia);
    st.control = Control::Controlled(Seat(0));
    st.facilities = vec![facility(FacilityKind::Factory), facility(FacilityKind::Refinery), facility(FacilityKind::PowerPlant), facility(FacilityKind::LaunchSite)];
    g.climate.temperature = 1.85;
    g.climate.co2 = 420.0 + 1.5 * g.tables.climate.ppm_step;
    g.climate_phase();
    let kinds: Vec<FacilityKind> = g.state(StateId::Australia).facilities.iter().map(|f| f.kind).collect();
    assert_eq!(kinds.len(), 2);
    assert!(!kinds.contains(&FacilityKind::Refinery), "Refinery (upkeep 3) went first");
    assert!(kinds.contains(&FacilityKind::PowerPlant), "Power Plant (upkeep 0) stayed");
}

// ---------------------------------------------------------------- 10.2 The battle round with fixed dice

struct Script {
    chances: VecDeque<bool>,
    d6s: VecDeque<u32>,
    picks: VecDeque<usize>,
}

impl Dice for Script {
    fn chance(&mut self, _p: f64) -> bool {
        self.chances.pop_front().expect("scripted chance")
    }
    fn d6(&mut self) -> u32 {
        self.d6s.pop_front().expect("scripted d6")
    }
    fn pick(&mut self, _n: usize) -> usize {
        self.picks.pop_front().unwrap_or(0)
    }
}

fn frigate(id: u32) -> Combatant {
    Combatant::new(UnitRef::Ship(ShipId(id)), format!("Frigate {id}"), 3, 4, 0, 4, false)
}
fn colony_ship(id: u32) -> Combatant {
    Combatant::new(UnitRef::Ship(ShipId(id)), format!("Colony Ship {id}"), 0, 3, 0, 0, false)
}

#[test]
fn battle_round_three_hit_rolls_then_disengage_then_pursuit() {
    // A Frigate attacks a Colony Ship: every hit-roll goes to the attacker (scripted true), no disengage
    // roll is needed for the undamaged Frigate, and the Colony Ship (3 HP) dies on the third hit.
    let mut a = vec![frigate(1)];
    let mut d = vec![colony_ship(2)];
    let mut dice = Script { chances: VecDeque::from(vec![true, true, true]), d6s: VecDeque::new(), picks: VecDeque::new() };
    let stats = combat::fight(&mut a, &mut d, &mut dice);
    assert_eq!(stats.rounds, 1);
    assert_eq!(stats.hits_by_attacker, 3);
    assert_eq!(stats.hits_by_defender, 0);
    assert!(d[0].destroyed());
    assert_eq!(stats.destroyed, vec!["Colony Ship 2".to_string()]);
}

#[test]
fn battle_defender_hits_land_on_the_attacker_and_a_damaged_unit_may_disengage() {
    // Frigate vs Frigate. Scripted: hit-rolls go defender, defender, attacker; then disengage rolls:
    // the attacker (2 damage of 4 -> 0.25) rolls true and leaves; the defender (1 damage) rolls false.
    // Pursuit: the defending Frigate has Pursuit 4; d6 = 2 catches; the pursuit hit-roll is true -> 3 damage.
    let mut a = vec![frigate(1)];
    let mut d = vec![frigate(2)];
    let mut dice = Script {
        chances: VecDeque::from(vec![false, false, true, true, false, true]),
        d6s: VecDeque::from(vec![2]),
        picks: VecDeque::new(),
    };
    let stats = combat::fight(&mut a, &mut d, &mut dice);
    assert_eq!(stats.rounds, 1, "the attacker escaped, so the battle ended");
    assert_eq!(a[0].damage, 3, "2 from the round and 1 from the pursuit");
    assert!(a[0].escaped && !a[0].engaged);
    assert_eq!(d[0].damage, 1);
    assert_eq!(stats.escaped, vec!["Frigate 1".to_string()]);
}

// ---------------------------------------------------------------- 10.2 Disengage and Pursuit probabilities

#[test]
fn disengage_probability_is_damage_over_hit_points_halved_and_evade_is_half() {
    let mut c = frigate(1);
    assert_eq!(combat::disengage_chance(&c), 0.0);
    c.damage = 2;
    assert!((combat::disengage_chance(&c) - 0.25).abs() < 1e-12);
    c.damage = 3;
    assert!((combat::disengage_chance(&c) - 0.375).abs() < 1e-12);
    c.evade = true;
    c.damage = 0;
    assert_eq!(combat::disengage_chance(&c), 0.5);
}

#[test]
fn pursuit_catches_on_a_d6_at_or_under_pursuit_and_not_above() {
    // Colony Ship (evade) flees a Frigate (Pursuit 4). d6 = 4 catches; d6 = 5 does not.
    for (roll, caught) in [(4u32, true), (5u32, false)] {
        let mut a = vec![frigate(1)];
        let mut d = vec![Combatant::new(UnitRef::Ship(ShipId(2)), "Colony Ship 2", 0, 3, 0, 0, true)];
        // Evade roll true at the start; pursuit hit-roll true if caught.
        let mut dice = Script { chances: VecDeque::from(vec![true, true]), d6s: VecDeque::from(vec![roll]), picks: VecDeque::new() };
        let stats = combat::fight(&mut a, &mut d, &mut dice);
        assert_eq!(stats.rounds, 0, "nobody was left engaged to fight a round");
        assert_eq!(d[0].damage, if caught { 1 } else { 0 }, "d6 {roll}");
        assert!(d[0].escaped);
    }
}

#[test]
fn first_round_odds_formula() {
    // p = 0.5: 0.125 + 3 * 0.25 * 0.5 = 0.5
    assert!((combat::first_round_odds(3, 3) - 0.5).abs() < 1e-12);
    // p = 0.75: 0.421875 + 3 * 0.5625 * 0.25 = 0.84375
    assert!((combat::first_round_odds(3, 1) - 0.84375).abs() < 1e-12);
    assert_eq!(combat::first_round_odds(0, 0), 0.0);
}

// ---------------------------------------------------------------- 8.3 Influence threshold and decay

#[test]
fn influence_threshold_takes_control_and_decay_takes_two_from_untouched_targets() {
    let mut g = game();
    // Africa: 20 + 10 * 3 = 50.
    assert_eq!(g.influence_threshold(Place::State(StateId::Africa)), 50);
    g.seats[0].allotment = 100;
    g.pending.influence.push((Seat(0), Place::State(StateId::Africa), 49));
    g.pending.influence.push((Seat(0), Place::State(StateId::Europe), 10));
    g.resolution_phase();
    assert_eq!(g.state(StateId::Africa).control, Control::Neutral, "49 is under the threshold");
    assert_eq!(g.seats[0].influence[&Place::State(StateId::Africa)], 49);
    // Next turn: 1 more on Africa flips it; Europe, untouched, decays by 2.
    g.pending.influence.push((Seat(0), Place::State(StateId::Africa), 1));
    g.resolution_phase();
    assert_eq!(g.state(StateId::Africa).control, Control::Controlled(Seat(0)));
    assert_eq!(g.seats[0].influence[&Place::State(StateId::Africa)], 50, "the standing persists through the transfer (#33)");
    assert_eq!(g.seats[0].influence[&Place::State(StateId::Europe)], 8);
}

// ---------------------------------------------------------------- #33 standings that persist

#[test]
fn spending_on_your_own_place_raises_your_standing_and_it_decays_one_a_turn() {
    let mut g = game();
    let c = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Habitat], 4);
    g.seats[1].influence.insert(Place::Colony(c), 30);
    g.seats[1].influenced_this_turn.push(Place::Colony(c));
    g.pending.influence.push((Seat(0), Place::Colony(c), 12));
    g.resolution_phase();
    assert_eq!(g.seats[1].influence[&Place::Colony(c)], 30, "the rival's standing is untouched");
    assert_eq!(g.seats[0].influence[&Place::Colony(c)], 12, "your own standing rose");
    g.resolution_phase();
    assert_eq!(g.seats[0].influence[&Place::Colony(c)], 11, "decay 1 on a place you control");
    assert_eq!(g.seats[1].influence[&Place::Colony(c)], 28, "decay 2 elsewhere");
}

#[test]
fn a_challenger_needs_a_standing_above_the_controllers_and_at_least_the_threshold() {
    let mut g = game();
    // Africa (threshold 50) is taken by seat 0 with a standing of 60.
    g.seats[0].influence.insert(Place::State(StateId::Africa), 60);
    g.seats[0].influenced_this_turn.push(Place::State(StateId::Africa));
    g.resolution_phase();
    assert_eq!(g.state(StateId::Africa).control, Control::Controlled(Seat(0)));
    // Seat 1 reaches the threshold but not the controller's standing: no change.
    g.seats[1].influence.insert(Place::State(StateId::Africa), 55);
    g.seats[1].influenced_this_turn.push(Place::State(StateId::Africa));
    g.seats[0].influenced_this_turn.push(Place::State(StateId::Africa));
    g.resolution_phase();
    assert_eq!(g.state(StateId::Africa).control, Control::Controlled(Seat(0)), "55 is not above 60");
    // Above the controller's standing: it flips, and seat 0 keeps its 60 to contest it back.
    g.seats[1].influence.insert(Place::State(StateId::Africa), 61);
    g.seats[1].influenced_this_turn.push(Place::State(StateId::Africa));
    g.seats[0].influenced_this_turn.push(Place::State(StateId::Africa));
    g.resolution_phase();
    assert_eq!(g.state(StateId::Africa).control, Control::Controlled(Seat(1)));
    assert_eq!(g.seats[0].influence[&Place::State(StateId::Africa)], 60);
    // Above the controller but under the threshold: a Colony with 8 Colonists (threshold 80) held at 20.
    let c = colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Habitat, ModuleKind::Habitat], 8);
    g.seats[0].influence.insert(Place::Colony(c), 20);
    g.seats[1].influence.insert(Place::Colony(c), 40);
    g.seats[0].influenced_this_turn.push(Place::Colony(c));
    g.seats[1].influenced_this_turn.push(Place::Colony(c));
    g.resolution_phase();
    assert_eq!(g.colony(c).unwrap().control, Control::Controlled(Seat(0)), "40 is above 20 but under the threshold of 80");
}

#[test]
fn occupation_transfer_keeps_the_old_controllers_standing() {
    let mut g = game();
    // Europe is the AI's; seat 1 holds it at 40. Seat 0 occupies it with its defenders gone.
    g.seats[1].influence.insert(Place::State(StateId::Europe), 40);
    g.armies.retain(|a| a.home != ArmyHome::State(StateId::Europe));
    occupier_in(&mut g, StateId::Asia, StateId::Europe);
    for _ in 0..3 {
        g.seats[1].influenced_this_turn.push(Place::State(StateId::Europe));
        g.resolution_phase();
    }
    assert_eq!(g.state(StateId::Europe).control, Control::Controlled(Seat(0)));
    assert_eq!(g.seats[1].influence[&Place::State(StateId::Europe)], 40, "the old controller keeps its standing");
    let threshold = g.influence_threshold(Place::State(StateId::Europe));
    assert!(g.seats[0].influence[&Place::State(StateId::Europe)] >= threshold, "the occupier's gains are its standing");
}

// ---------------------------------------------------------------- #34 per-state Influence values

#[test]
fn the_allotment_is_the_base_plus_each_controlled_states_value_times_the_faction_multiplier() {
    let mut g = game();
    // Custodians hold Asia (7): (10 + 7) x 1.3 = 22.1 -> 22. Prospectors hold Europe (5): 15.
    assert_eq!(g.influence_allotment(Seat(0)), 22);
    assert_eq!(g.influence_allotment(Seat(1)), 15);
    g.state_mut(StateId::NorthAmerica).control = Control::Controlled(Seat(1));
    assert_eq!(g.influence_allotment(Seat(1)), 23, "North America adds 8");
    // Raising Asia's Industry Level adds one to its value.
    g.state_mut(StateId::Asia).industry_level += 1;
    assert_eq!(g.state_influence_value(StateId::Asia), 8);
    assert_eq!(g.influence_allotment(Seat(0)), 23, "(10 + 8) x 1.3 = 23.4");
    // The card figures, as decided: Antarctica counts for nothing.
    assert_eq!(g.state_influence_value(StateId::Antarctica), 0);
    let total: i64 = StateId::ALL.iter().map(|s| g.tables.state(*s).influence).sum();
    assert_eq!(total, 34, "8 + 7 + 5 + 4 + 4 + 2 + 2 + 2 + 0");
}

// ---------------------------------------------------------------- #35 Ducats

#[test]
fn a_controlled_state_pays_ducats_from_gdp_times_industry_and_a_bank_adds_more() {
    let mut g = game();
    // Asia: gdp 30 x Industry 3 / 10 = 9 a turn for the Custodians; Europe 20 x 3 / 10 = 6 for the Prospectors.
    assert_eq!(g.state_ducats(StateId::Asia), 9);
    assert_eq!(g.state_ducats(StateId::Europe), 6);
    assert_eq!(g.state_ducats(StateId::Antarctica), 0);
    let paid = income_of(&mut g, Seat(0));
    assert_eq!(paid.ducats, 9);
    // A Bank in Asia adds 4 x 30 / 10 = 12 (Custodian output x1.0); in Africa (gdp 3) it would add 1.
    g.state_mut(StateId::Asia).facilities.push(facility(FacilityKind::Bank));
    assert_eq!(g.facility_yield(Seat(0), StateId::Asia, FacilityKind::Bank).amount, 12);
    assert_eq!(g.facility_yield(Seat(0), StateId::Africa, FacilityKind::Bank).amount, 1);
    assert_eq!(income_of(&mut g, Seat(0)).ducats, 21);
    // A Trade Post follows the Habitat yield: 3 on the Moon, 4 on Mars (3 x 1.5 rounded down).
    let moon = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::TradePost], 0);
    let mars = colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::TradePost], 0);
    assert_eq!(g.module_yield(Seat(0), moon, ModuleKind::TradePost).amount, 3);
    assert_eq!(g.module_yield(Seat(0), mars, ModuleKind::TradePost).amount, 4);
    // Ducats never count toward the Extraction Total.
    let before = g.seats[1].extraction_total;
    g.state_mut(StateId::Europe).facilities = vec![facility(FacilityKind::Bank)];
    income_of(&mut g, Seat(1));
    assert_eq!(g.seats[1].extraction_total, before);
}

#[test]
fn ducats_buy_influence_two_for_one_and_the_bought_influence_is_spendable_at_once() {
    let mut g = game();
    g.seats[0].stockpile.ducats = 20;
    g.seats[0].allotment = 22;
    let buy = Order::BuyInfluence { amount: 10 };
    assert_eq!(g.order_cost(Seat(0), &buy).ducats, 20);
    assert!(g.check_order(Seat(0), &[], &Order::BuyInfluence { amount: 11 }).is_err(), "22 Ducats needed, 20 held");
    let pending = vec![buy.clone()];
    let (_, left) = g.remaining(Seat(0), &pending);
    assert_eq!(left, 32, "the Allotment plus the bought 10");
    let spend = Order::Influence { target: Place::State(StateId::Africa), amount: 30 };
    assert!(g.check_order(Seat(0), &pending, &spend).is_ok());
    g.commit_orders(Seat(0), &[buy, spend]);
    assert_eq!(g.seats[0].stockpile.ducats, 0);
    assert_eq!(g.seats[0].allotment, 2);
}

#[test]
fn ducats_pay_for_restoration_and_repairs_at_the_table_rates() {
    let mut g = game();
    g.seats[0].stockpile.ducats = 25;
    g.seats[0].stockpile.energy = 0;
    let r = Order::RestorationWithDucats { steps: 2 };
    assert_eq!(g.order_cost(Seat(0), &r).ducats, 20);
    g.commit_orders(Seat(0), &[r]);
    assert!((g.climate.restoration_next - 6.0).abs() < 1e-9, "two steps of 3.0 ppm");
    assert_eq!(g.seats[0].stockpile.ducats, 5);
    // A repair: 5 Ducats a point, same legality as a Materials repair.
    g.ships.push(Ship { id: ShipId(1), kind: UnitKind::Frigate, seat: Seat(0), damage: 1, at: ShipAt::Body(BodyId::Earth), colonists: 0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1 });
    let fix = Order::RepairWithDucats { unit: UnitRef::Ship(ShipId(1)), points: 1 };
    assert_eq!(g.order_cost(Seat(0), &fix).ducats, 5);
    assert!(g.check_order(Seat(0), &[], &fix).is_ok());
    g.commit_orders(Seat(0), &[fix]);
    g.resolution_phase();
    assert_eq!(g.ship(ShipId(1)).unwrap().damage, 0);
    assert_eq!(g.seats[0].stockpile.ducats, 0);
    assert!(g.check_order(Seat(0), &[], &Order::RepairWithDucats { unit: UnitRef::Ship(ShipId(1)), points: 1 }).is_err(), "nothing to repair");
}

// ---------------------------------------------------------------- #36 Embassies and Relays

#[test]
fn embassies_and_relays_add_to_the_allotment_and_raise_their_places_standing_each_turn() {
    let mut g = game();
    // Custodians in Asia: (10 + 7) x 1.3 = 22. Two Embassies (they stack) add 4: (10 + 7 + 4) x 1.3 = 27.
    assert_eq!(g.influence_allotment(Seat(0)), 22);
    g.state_mut(StateId::Asia).facilities.push(facility(FacilityKind::Embassy));
    g.state_mut(StateId::Asia).facilities.push(facility(FacilityKind::Embassy));
    assert_eq!(g.building_allotment(Seat(0)), 4);
    assert_eq!(g.influence_allotment(Seat(0)), 27);
    // A Relay in a Colony adds 1 more.
    let c = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Habitat, ModuleKind::Relay], 4);
    assert_eq!(g.influence_allotment(Seat(0)), 28, "(10 + 7 + 5) x 1.3 = 28.6");
    // Each Resolution the standing rises by the buildings' figures and does not decay.
    g.resolution_phase();
    assert_eq!(g.seats[0].influence[&Place::State(StateId::Asia)], 4, "two Embassies, 2 each");
    assert_eq!(g.seats[0].influence[&Place::Colony(c)], 2, "one Relay");
    g.resolution_phase();
    assert_eq!(g.seats[0].influence[&Place::State(StateId::Asia)], 8);
    // An offline Embassy adds nothing.
    for f in g.state_mut(StateId::Asia).facilities.iter_mut().filter(|f| f.kind == FacilityKind::Embassy) {
        f.online = false;
    }
    assert_eq!(g.building_allotment(Seat(0)), 1, "only the Relay");
    g.resolution_phase();
    assert_eq!(g.seats[0].influence[&Place::State(StateId::Asia)], 7, "no rise, and decay 1 on your own place");
    // The card says what they do.
    let y = g.facility_yield(Seat(0), StateId::Asia, FacilityKind::Embassy);
    assert_eq!(y.text(), "+2 Influence Allotment, standing here +2 a turn, 2 Energy upkeep");
}

// ---------------------------------------------------------------- 8.5 Occupation

fn occupier_in(g: &mut Game, seat_home: StateId, target: StateId) -> ArmyId {
    let id = ArmyId(g.fresh_id());
    g.armies.push(Army { id, home: ArmyHome::State(seat_home), at: ArmyAt::Place(Place::State(target)), damage: 0, standing: false, stance: Stance::Attack, escaped: false, move_to: None });
    id
}

#[test]
fn occupation_transfers_control_at_the_end_of_the_third_turn() {
    let mut g = game();
    // Seat 0 (Asia) has an Army in Europe, whose Standing Army has been removed.
    g.armies.retain(|a| a.home != ArmyHome::State(StateId::Europe));
    occupier_in(&mut g, StateId::Asia, StateId::Europe);
    g.resolution_phase();
    assert!(matches!(g.state(StateId::Europe).control, Control::Occupied { occupier: Seat(0), turns: 1, .. }));
    g.resolution_phase();
    assert!(matches!(g.state(StateId::Europe).control, Control::Occupied { occupier: Seat(0), turns: 2, .. }));
    g.resolution_phase();
    assert_eq!(g.state(StateId::Europe).control, Control::Controlled(Seat(0)));
}

#[test]
fn occupation_transfers_at_once_when_pacified() {
    let mut g = game();
    g.armies.retain(|a| a.home != ArmyHome::State(StateId::Europe));
    occupier_in(&mut g, StateId::Asia, StateId::Europe);
    // Threshold 50; 40 already accumulated; one turn of Occupation adds ceil(50/3) = 17.
    g.seats[0].influence.insert(Place::State(StateId::Europe), 40);
    g.resolution_phase();
    assert_eq!(g.state(StateId::Europe).control, Control::Controlled(Seat(0)), "Pacified in the first turn");
}

#[test]
fn occupation_ends_when_the_occupier_leaves() {
    let mut g = game();
    // Africa is neutral (Europe is the AI's start state).
    g.armies.retain(|a| a.home != ArmyHome::State(StateId::Africa));
    let a = occupier_in(&mut g, StateId::Asia, StateId::Africa);
    g.resolution_phase();
    assert!(matches!(g.state(StateId::Africa).control, Control::Occupied { occupier: Seat(0), previous: None, turns: 1 }));
    g.armies.retain(|x| x.id != a);
    g.resolution_phase();
    assert_eq!(g.state(StateId::Africa).control, Control::Neutral);
}

#[test]
fn occupation_of_a_controlled_state_returns_it_to_its_owner_when_broken() {
    let mut g = game();
    g.armies.retain(|a| a.home != ArmyHome::State(StateId::Europe));
    let a = occupier_in(&mut g, StateId::Asia, StateId::Europe);
    g.resolution_phase();
    assert!(matches!(g.state(StateId::Europe).control, Control::Occupied { occupier: Seat(0), previous: Some(Seat(1)), turns: 1 }));
    g.armies.retain(|x| x.id != a);
    g.resolution_phase();
    assert_eq!(g.state(StateId::Europe).control, Control::Controlled(Seat(1)));
}

// ---------------------------------------------------------------- 15 Victory checks in order

fn meet_first(g: &mut Game, seat: Seat) {
    match g.kind(seat) {
        FactionKind::Prospectors => g.seats[seat.index()].extraction_total = 500,
        FactionKind::Custodians => g.seats[seat.index()].stabilization_run = 3,
    }
}

#[test]
fn a_faction_meeting_both_parts_wins_before_collapse_is_checked() {
    let mut g = game();
    colony(&mut g, Seat(1), BodyId::Mars, &[ModuleKind::Habitat, ModuleKind::Habitat], 12);
    meet_first(&mut g, Seat(1));
    g.climate.temperature = 3.2;
    g.end_phase();
    assert!(matches!(g.outcome, Some(Outcome::Win { seat: Seat(1), .. })));
}

#[test]
fn both_met_the_larger_margin_wins() {
    let mut g = game();
    colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Habitat, ModuleKind::Habitat], 12);
    colony(&mut g, Seat(1), BodyId::Mars, &[ModuleKind::Habitat, ModuleKind::Habitat, ModuleKind::Habitat], 15);
    g.seats[0].stabilization_run = 3; // parts 1.0 and 1.0 -> margin 1.0
    g.seats[1].extraction_total = 600; // parts 1.2 and 1.25 -> margin 1.2
    g.end_phase();
    assert!(matches!(g.outcome, Some(Outcome::Win { seat: Seat(1), .. })), "{:?}", g.outcome);
}

#[test]
fn both_met_by_the_same_margin_is_a_draw() {
    let mut g = game();
    colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Habitat, ModuleKind::Habitat], 12);
    colony(&mut g, Seat(1), BodyId::Mars, &[ModuleKind::Habitat, ModuleKind::Habitat], 12);
    g.seats[0].stabilization_run = 3;
    g.seats[1].extraction_total = 600; // the lower fraction is the presence, 1.0, on both sides
    g.end_phase();
    assert!(matches!(g.outcome, Some(Outcome::Draw { .. })), "{:?}", g.outcome);
}

#[test]
fn collapse_ends_the_game_with_nobody_winning_when_no_condition_is_met() {
    let mut g = game();
    g.climate.temperature = 3.0;
    g.end_phase();
    assert_eq!(g.outcome, Some(Outcome::Collapse));
}

#[test]
fn the_last_turn_scores_the_lower_fraction_of_the_two_parts() {
    let mut g = game();
    g.turn = g.tables.victory.turns;
    colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Habitat], 6); // presence 0.5, run 0 -> score 0
    g.seats[0].stabilization_run = 3;
    colony(&mut g, Seat(1), BodyId::Moon, &[ModuleKind::Habitat], 3); // presence 0.25
    g.seats[1].extraction_total = 500; // first 1.0 -> score 0.25
    g.end_phase();
    assert!(matches!(g.outcome, Some(Outcome::Win { seat: Seat(0), .. })), "0.5 beats 0.25: {:?}", g.outcome);
}

#[test]
fn nothing_ends_before_the_last_turn_without_a_condition_or_collapse() {
    let mut g = game();
    g.turn = g.tables.victory.turns - 1;
    g.end_phase();
    assert_eq!(g.outcome, None);
}

// ---------------------------------------------------------------- 13.1 as amended by #25: the draw chance and the deck

#[test]
fn the_draw_chance_is_half_at_base_and_rises_per_full_fifth_of_a_degree() {
    let mut g = game();
    g.climate.temperature = 1.2;
    assert!((g.draw_chance() - 0.5).abs() < 1e-12);
    g.climate.temperature = 1.39;
    assert!((g.draw_chance() - 0.5).abs() < 1e-12, "a part step counts for nothing");
    g.climate.temperature = 1.4;
    assert!((g.draw_chance() - 0.525).abs() < 1e-12);
    g.climate.temperature = 3.0;
    assert!((g.draw_chance() - 0.725).abs() < 1e-12);
}

#[test]
fn a_card_comes_on_about_half_the_turns_at_the_start_and_more_when_warm() {
    let mut g = game();
    g.climate.temperature = 1.2;
    let draws = (0..4000).filter(|_| g.rolls_a_card()).count();
    assert!((1800..=2200).contains(&draws), "{draws} of 4000 at +1.2");
    g.climate.temperature = 3.0;
    let draws = (0..4000).filter(|_| g.rolls_a_card()).count();
    assert!((2750..=3050).contains(&draws), "{draws} of 4000 at +3.0");
}

#[test]
fn the_deck_is_twenty_six_cards_as_the_table_deals_them_and_no_calm() {
    let g = game();
    assert_eq!(g.deck.cards.len(), 28, "ten first-playable Events twice, eight later ones once (#25, #32)");
    for e in &g.tables.events.event {
        assert_eq!(g.deck.count(e.id), e.copies as usize, "{}", e.name);
    }
    assert!(!g.tables.events.event.iter().any(|e| e.target == "faction"), "no card singles out a Faction (#32)");
}

// ---------------------------------------------------------------- #32 the two replacement cards

#[test]
fn launch_pad_fire_delays_the_ships_due_at_that_state_unless_clean_propellant() {
    let mut g = game();
    g.turn = 3;
    g.state_mut(StateId::Asia).queue.push(Build { item: BuildItem::Unit(UnitKind::Frigate), seat: Seat(0), due_turn: 3 });
    g.state_mut(StateId::Asia).queue.push(Build { item: BuildItem::Facility(FacilityKind::Factory), seat: Seat(0), due_turn: 3 });
    drawn(&mut g, EventId::LaunchPadFire, EventTarget::State(StateId::Asia));
    g.resolution_phase();
    assert!(g.ships.is_empty(), "the Frigate did not appear");
    assert_eq!(g.state(StateId::Asia).queue.len(), 1, "it is back in the queue");
    assert_eq!(g.state(StateId::Asia).queue[0].due_turn, 4);
    assert!(g.state(StateId::Asia).facilities.iter().any(|f| f.kind == FacilityKind::Factory), "the Factory was not delayed");
    assert!(!g.state(StateId::Asia).facilities.iter().find(|f| f.kind == FacilityKind::LaunchSite).unwrap().online, "the Launch Site is offline");
    // With Clean Propellant nothing is delayed.
    let mut g = game();
    g.turn = 3;
    with_tech(&mut g, TechId::CleanPropellant);
    g.state_mut(StateId::Asia).queue.push(Build { item: BuildItem::Unit(UnitKind::Frigate), seat: Seat(0), due_turn: 3 });
    drawn(&mut g, EventId::LaunchPadFire, EventTarget::State(StateId::Asia));
    g.resolution_phase();
    assert_eq!(g.ships.len(), 1);
}

#[test]
fn labour_dispute_idles_a_states_facilities_at_the_next_income_and_public_science_spares_all_but_one() {
    let mut g = game();
    g.state_mut(StateId::Asia).facilities = vec![facility(FacilityKind::Factory), facility(FacilityKind::Refinery), facility(FacilityKind::PowerPlant)];
    drawn(&mut g, EventId::LabourDispute, EventTarget::State(StateId::Asia));
    g.apply_event_now();
    let paid = income_of(&mut g, Seat(0));
    assert_eq!((paid.materials, paid.fuel), (0, 0), "nothing made: {paid:?}");
    g.last_event = None;
    g.resolution_phase();
    let paid = income_of(&mut g, Seat(0));
    assert!(paid.materials > 0 && paid.fuel > 0, "back at work after Resolution: {paid:?}");
    with_tech(&mut g, TechId::PublicScience);
    drawn(&mut g, EventId::LabourDispute, EventTarget::State(StateId::Asia));
    g.apply_event_now();
    let idle = g.state(StateId::Asia).facilities.iter().filter(|f| f.offline_until_resolution).count();
    assert_eq!(idle, 1, "one Facility only");
}

#[test]
fn no_card_drawn_leaves_the_deck_alone_and_says_so() {
    let mut g = game();
    g.climate.temperature = 1.2;
    // Seed 7's first roll at +1.2 draws nothing; if the generator changes, the assertion says which way.
    let before = g.deck.cards.len();
    g.event_phase();
    match &g.last_event {
        None => {
            assert_eq!(g.deck.cards.len(), before);
            assert!(g.report.event.as_deref().unwrap_or("").starts_with("No Event this turn"));
        }
        Some(_) => assert_eq!(g.deck.cards.len(), before - 1),
    }
}

// ---------------------------------------------------------------- #25 the six new Events

fn drawn(g: &mut Game, id: EventId, target: EventTarget) {
    let scale = g.climate_scale();
    g.last_event = Some(DrawnEvent { card: Card::Event(id), target, scale, text: String::new() });
}

#[test]
fn solar_maximum_boosts_power_plants_and_generators_at_the_next_income_once() {
    let mut g = game();
    g.state_mut(StateId::Asia).facilities = vec![facility(FacilityKind::PowerPlant)];
    colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Generator], 0);
    drawn(&mut g, EventId::SolarMaximum, EventTarget::Everyone);
    g.apply_event_now();
    // Power Plant 6 x 1.5 = 9; Generator 5 x 1.25 (Moon) x 1.5 = 9.
    assert_eq!(income_of(&mut g, Seat(0)).energy, 18);
    assert_eq!(income_of(&mut g, Seat(0)).energy, 12, "the boost lasts one Income");
    with_tech(&mut g, TechId::EfficientGrids);
    drawn(&mut g, EventId::SolarMaximum, EventTarget::Everyone);
    g.apply_event_now();
    // Power Plant 6 x 1.5 x 2 = 18; Generator 5 x 1.25 x 1.5 x 2 = 18.
    assert_eq!(income_of(&mut g, Seat(0)).energy, 36, "Efficient Grids makes it x2");
}

#[test]
fn permafrost_thaw_adds_scaled_emissions_next_turn_that_do_not_count_against_stabilization() {
    let mut g = game();
    g.climate.temperature = 2.2; // scale 1.5
    drawn(&mut g, EventId::PermafrostThaw, EventTarget::Everyone);
    g.apply_event_now();
    let e = g.emissions_now();
    assert!((e.cards - 4.5).abs() < 1e-9, "3.0 x 1.5: {}", e.cards);
    assert!((e.total() - e.counted() - 4.5).abs() < 1e-9);
    with_tech(&mut g, TechId::GreenConsensus);
    g.climate.card_emissions_next = 0.0;
    drawn(&mut g, EventId::PermafrostThaw, EventTarget::Everyone);
    g.apply_event_now();
    assert!((g.emissions_now().cards - 2.25).abs() < 1e-9, "halved");
}

#[test]
fn meteor_shower_hits_ships_in_orbit_not_in_transit_and_hardened_hulls_shrug() {
    let mut g = game();
    let mk = |id: u32, at: ShipAt| Ship { id: ShipId(id), kind: UnitKind::Frigate, seat: Seat(0), damage: 0, at, colonists: 0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1 };
    g.ships.push(mk(1, ShipAt::Body(BodyId::Earth)));
    g.ships.push(mk(2, ShipAt::Transit { from: BodyId::Earth, to: BodyId::Mars, turns_left: 2 }));
    drawn(&mut g, EventId::MeteorShower, EventTarget::Everyone);
    g.apply_event_now();
    assert_eq!(g.ship(ShipId(1)).unwrap().damage, 1);
    assert_eq!(g.ship(ShipId(2)).unwrap().damage, 0);
    with_tech(&mut g, TechId::HardenedHulls);
    drawn(&mut g, EventId::MeteorShower, EventTarget::Everyone);
    g.apply_event_now();
    assert_eq!(g.ship(ShipId(1)).unwrap().damage, 1);
}

#[test]
fn dust_storm_knocks_every_module_on_mars_offline_until_the_next_resolution() {
    let mut g = game();
    let mars = colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Mine], 0);
    let moon = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Mine], 0);
    drawn(&mut g, EventId::DustStorm, EventTarget::Body(BodyId::Mars));
    g.apply_event_now();
    assert!(!g.colony(mars).unwrap().modules[0].online);
    assert!(g.colony(moon).unwrap().modules[0].online);
    g.seats[0].stockpile.energy = 100;
    g.income_phase();
    assert!(!g.colony(mars).unwrap().modules[0].online, "still off at the next Income");
    g.last_event = None; // in a real turn a fresh draw precedes Resolution
    g.resolution_phase();
    assert!(g.colony(mars).unwrap().modules[0].online, "back on after Resolution");
}

#[test]
fn unrest_damages_the_standing_army_and_cuts_every_factions_influence_there() {
    let mut g = game();
    g.seats[0].influence.insert(Place::State(StateId::Africa), 12);
    g.seats[1].influence.insert(Place::State(StateId::Africa), 3);
    drawn(&mut g, EventId::Unrest, EventTarget::State(StateId::Africa));
    g.apply_event_now();
    let army = g.armies.iter().find(|a| a.standing && a.home == ArmyHome::State(StateId::Africa)).unwrap();
    assert_eq!(army.damage, 2);
    assert_eq!(g.seats[0].influence[&Place::State(StateId::Africa)], 7);
    assert_eq!(g.seats[1].influence[&Place::State(StateId::Africa)], 0);
}

#[test]
fn reactor_leak_stops_generators_until_resolution_and_costs_five_energy() {
    let mut g = game();
    let c = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Generator, ModuleKind::Mine], 0);
    g.seats[0].stockpile.energy = 20;
    drawn(&mut g, EventId::ReactorLeak, EventTarget::Colony(c));
    g.apply_event_now();
    let col = g.colony(c).unwrap();
    assert!(!col.modules[0].online, "the Generator is off");
    assert!(col.modules[1].online, "the Mine is not");
    assert_eq!(g.seats[0].stockpile.energy, 15);
    g.income_phase();
    assert!(!g.colony(c).unwrap().modules[0].online, "still off at Income");
    g.last_event = None;
    g.resolution_phase();
    assert!(g.colony(c).unwrap().modules[0].online);
}

// ---------------------------------------------------------------- 12.3 Every Tech effect

fn with_tech(g: &mut Game, t: TechId) {
    g.research.done.push(t);
}

fn income_of(g: &mut Game, seat: Seat) -> Stockpile {
    let before = g.seat(seat).stockpile;
    g.seats[seat.index()].stockpile.energy = 1000;
    g.income_phase();
    let after = g.seat(seat).stockpile;
    Stockpile { materials: after.materials - before.materials, fuel: after.fuel - before.fuel, energy: after.energy - 1000, ducats: after.ducats - before.ducats }
}

#[test]
fn tech_efficient_grids_raises_power_plant_and_generator_output() {
    let mut g = game();
    g.state_mut(StateId::Asia).facilities = vec![facility(FacilityKind::PowerPlant)];
    let plain = income_of(&mut g, Seat(0)).energy;
    with_tech(&mut g, TechId::EfficientGrids);
    let boosted = income_of(&mut g, Seat(0)).energy;
    assert_eq!(plain, 6);
    assert_eq!(boosted, 9);
}

#[test]
fn tech_clean_power_cuts_power_plant_emissions() {
    let mut g = game();
    g.state_mut(StateId::Asia).facilities = vec![facility(FacilityKind::PowerPlant)];
    let plain = g.emissions_now().power_plants;
    with_tech(&mut g, TechId::CleanPower);
    let clean = g.emissions_now().power_plants;
    assert!((plain - 1.5 * 0.75).abs() < 1e-9, "Custodians x0.75: {plain}");
    assert!((clean - 1.5 * 0.4 * 0.75).abs() < 1e-9);
}

#[test]
fn tech_clean_manufacturing_cuts_factory_and_refinery_emissions() {
    let mut g = game();
    g.state_mut(StateId::Asia).facilities = vec![facility(FacilityKind::Factory), facility(FacilityKind::Refinery)];
    with_tech(&mut g, TechId::CleanManufacturing);
    let e = g.emissions_now();
    assert!((e.factories - 1.0 * 0.4 * 0.75).abs() < 1e-9);
    assert!((e.refineries - 1.5 * 0.4 * 0.75).abs() < 1e-9);
}

#[test]
fn tech_clean_propellant_makes_a_launch_emit_less() {
    let mut g = game();
    g.climate.launches_pending = [1, 0];
    let plain = g.emissions_now().launches;
    with_tech(&mut g, TechId::CleanPropellant);
    let clean = g.emissions_now().launches;
    assert!((plain - 2.0 * 0.75).abs() < 1e-9);
    assert!((clean - 0.8 * 0.75).abs() < 1e-9);
}

#[test]
fn tech_efficient_transit_cuts_fuel() {
    let mut g = game();
    assert_eq!(g.transit_cost(BodyId::Earth, BodyId::Mars).1, 20);
    with_tech(&mut g, TechId::EfficientTransit);
    assert_eq!(g.transit_cost(BodyId::Earth, BodyId::Mars).1, 12);
    assert_eq!(g.transit_cost(BodyId::Moon, BodyId::Earth).1, 3, "6 x 0.6 rounded down");
}

#[test]
fn tech_hardened_hulls_adds_two_strength_to_every_ship_but_a_colony_ship_stays_at_zero() {
    let mut g = game();
    let f = Ship { id: ShipId(1), kind: UnitKind::Frigate, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1 };
    let c = Ship { kind: UnitKind::ColonyShip, ..f.clone() };
    assert_eq!(g.ship_strength(&f), 3);
    with_tech(&mut g, TechId::HardenedHulls);
    assert_eq!(g.ship_strength(&f), 5);
    assert_eq!(g.ship_strength(&c), 0);
}

#[test]
fn tech_expanded_habitats_holds_two_more() {
    let mut g = game();
    let c = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Habitat], 0);
    assert_eq!(g.habitat_room(g.colony(c).unwrap()), 4);
    with_tech(&mut g, TechId::ExpandedHabitats);
    assert_eq!(g.habitat_room(g.colony(c).unwrap()), 6);
}

#[test]
fn tech_closed_loop_colonies_halves_module_upkeep() {
    let mut g = game();
    colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Mine], 0); // upkeep 3
    g.state_mut(StateId::Asia).facilities.clear();
    let plain = income_of(&mut g, Seat(0)).energy;
    with_tech(&mut g, TechId::ClosedLoopColonies);
    let halved = income_of(&mut g, Seat(0)).energy;
    assert_eq!(plain, -3);
    assert_eq!(halved, -1, "3 x 0.5 rounded down");
}

#[test]
fn tech_deep_mining_raises_mine_and_factory_output() {
    let mut g = game();
    g.state_mut(StateId::Asia).facilities = vec![facility(FacilityKind::Factory)];
    colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Mine], 0);
    // Factory: 4 x 1.5 (Asia leans Materials) = 6. Mine: 4 x 1.5 (Moon) = 6.
    assert_eq!(income_of(&mut g, Seat(0)).materials, 12);
    with_tech(&mut g, TechId::DeepMining);
    // Factory 9, Mine 9.
    assert_eq!(income_of(&mut g, Seat(0)).materials, 18);
}

#[test]
fn tech_automated_refining_raises_refinery_output() {
    let mut g = game();
    g.state_mut(StateId::Asia).facilities = vec![facility(FacilityKind::Refinery)];
    assert_eq!(income_of(&mut g, Seat(0)).fuel, 3);
    with_tech(&mut g, TechId::AutomatedRefining);
    assert_eq!(income_of(&mut g, Seat(0)).fuel, 4, "4.5 rounded down");
}

#[test]
fn tech_public_science_raises_research_lab_output() {
    let mut g = game();
    g.state_mut(StateId::Asia).facilities = vec![facility(FacilityKind::ResearchLab)];
    g.research.current = Some(TechId::CleanPower);
    g.research.done.push(TechId::EfficientGrids);
    g.seats[0].stockpile.energy = 1000;
    g.income_phase();
    // 2 x (1 + 47/50) x 0.9 x 1.25 = 4.23 -> 4
    assert_eq!(g.seats[0].research_last_turn, 4);
    with_tech(&mut g, TechId::PublicScience);
    g.income_phase();
    assert_eq!(g.seats[0].research_last_turn, 6, "6.345 rounded down");
}

#[test]
fn tech_green_consensus_halves_per_person_emissions_and_lowers_thresholds() {
    let mut g = game();
    let before = g.emissions_now().population;
    assert_eq!(g.influence_threshold(Place::State(StateId::Africa)), 50);
    with_tech(&mut g, TechId::GreenConsensus);
    let after = g.emissions_now().population;
    assert!((after - before / 2.0).abs() < 1e-9);
    assert_eq!(g.influence_threshold(Place::State(StateId::Africa)), 37, "50 x 0.75 rounded down");
}

// ---------------------------------------------------------------- 19.3 a defended Colony changes hands

/// The fifth anchor, scripted: a Battleship lands an Army on a rival Colony defended by a Barracks Army.
/// Counted from the turn the Army lands: land (turn 1), battle and Occupation (2), Occupation (3), transfer (4).
#[test]
fn a_defended_colony_changes_hands_in_about_four_turns() {
    // Battles are dice, so the scenario runs over twelve seeds: most attacks must take the Colony,
    // and every transfer must land in the three-to-five-turn window.
    let mut taken = Vec::new();
    for seed in 1..=12u64 {
        if let Some(t) = colony_attack_turns(seed) {
            taken.push(t);
        }
    }
    assert!(taken.len() >= 6, "only {} of 12 attacks took the Colony", taken.len());
    taken.sort();
    let median = taken[taken.len() / 2];
    assert!((3..=5).contains(&median), "median {median} turns: {taken:?}");
    for t in &taken {
        assert!(*t >= 3, "changed hands after only {t} turns, faster than the rules allow: {taken:?}");
    }
}

#[test]
fn a_zero_threshold_is_not_met_by_zero_influence() {
    let mut g = game();
    // A Colony with no Colonists has a threshold of 10 x 0 = 0; nobody has spent anything on it.
    let c = colony(&mut g, Seat(1), BodyId::Moon, &[ModuleKind::Barracks], 0);
    assert_eq!(g.influence_threshold(Place::Colony(c)), 0);
    g.resolution_phase();
    assert_eq!(g.colony(c).unwrap().control, Control::Controlled(Seat(1)), "control does not move for free");
}

fn colony_attack_turns(seed: u64) -> Option<u32> {
    let mut g = Game::new(tables(), NewGame { seed, seats: [(FactionKind::Custodians, false), (FactionKind::Prospectors, true)], player_start: StateId::Asia });
    // The AI Prospectors hold a Colony on the Moon with a Barracks and its Army.
    let cid = colony(&mut g, Seat(1), BodyId::Moon, &[ModuleKind::Habitat, ModuleKind::Barracks], 4);
    let defender = ArmyId(g.fresh_id());
    g.armies.push(Army { id: defender, home: ArmyHome::Colony(cid), at: ArmyAt::Place(Place::Colony(cid)), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None });
    // The player's Battleship and a Colony Ship arrive at the Moon, each carrying an Army: strength 8 against 4.
    let mut attackers = Vec::new();
    let mut ships = Vec::new();
    for kind in [UnitKind::Battleship, UnitKind::ColonyShip] {
        let attacker = ArmyId(g.fresh_id());
        let ship = ShipId(g.fresh_id());
        g.armies.push(Army { id: attacker, home: ArmyHome::State(StateId::Asia), at: ArmyAt::Aboard(ship), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None });
        g.ships.push(Ship { id: ship, kind, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Moon), colonists: 0, army: Some(attacker), stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1 });
        attackers.push(attacker);
        ships.push(ship);
    }
    g.seats[1].ai = false; // keep the rival still so the count is clean
    for turn in 1..=8 {
        let mut orders = Vec::new();
        if turn == 1 {
            for ship in &ships {
                orders.push(Order::Unload { ship: *ship, colonists: 0, army: true, into: UnloadTarget::Colony(cid) });
            }
        } else if !g.armies_of_seat_at(Seat(0), Place::Colony(cid)).is_empty() {
            orders.push(Order::ArmyStance { place: Place::Colony(cid), stance: Stance::Attack });
        }
        g.end_turn([orders, Vec::new()]);
        if g.colony(cid).map(|c| c.control == Control::Controlled(Seat(0))).unwrap_or(false) {
            return Some(turn);
        }
        if attackers.iter().all(|a| g.army(*a).is_none()) {
            return None; // the attack failed this time; the dice decide battles
        }
    }
    None
}

// ---------------------------------------------------------------- #22 the card's figures are the Income phase's figures

#[test]
fn building_yields_on_the_card_equal_what_income_pays() {
    let mut g = game();
    g.state_mut(StateId::Asia).facilities = vec![facility(FacilityKind::Factory), facility(FacilityKind::Refinery), facility(FacilityKind::ResearchLab), facility(FacilityKind::PowerPlant)];
    g.state_mut(StateId::Africa).control = Control::Controlled(Seat(0));
    g.state_mut(StateId::Africa).facilities = vec![facility(FacilityKind::Factory)];
    let c = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Mine, ModuleKind::Generator, ModuleKind::Refinery], 0);
    g.research.done.push(TechId::DeepMining);
    g.research.current = Some(TechId::CleanPower);
    let mut expect = Stockpile::default();
    let mut research = 0;
    for sid in [StateId::Asia, StateId::Africa] {
        for f in &g.state(sid).facilities {
            let y = g.facility_yield(Seat(0), sid, f.kind);
            match y.resource {
                Some(Resource::Materials) => expect.materials += y.amount,
                Some(Resource::Fuel) => expect.fuel += y.amount,
                Some(Resource::Energy) => expect.energy += y.amount,
                _ => {}
            }
            research += y.research;
            expect.energy -= y.upkeep;
        }
    }
    for m in &g.colony(c).unwrap().modules {
        let y = g.module_yield(Seat(0), c, m.kind);
        match y.resource {
            Some(Resource::Materials) => expect.materials += y.amount,
            Some(Resource::Fuel) => expect.fuel += y.amount,
            Some(Resource::Energy) => expect.energy += y.amount,
            _ => {}
        }
        expect.energy -= y.upkeep;
    }
    assert!(expect.materials > 0 && expect.fuel > 0 && research > 0, "the scenario produces something: {expect:?} research {research}");
    let paid = income_of(&mut g, Seat(0));
    assert_eq!((paid.materials, paid.fuel, paid.energy), (expect.materials, expect.fuel, expect.energy));
    assert_eq!(g.seats[0].research_last_turn, research);
    // And the Emissions figure on the card is the Climate phase's figure for that building.
    let card: f64 = g.state(StateId::Asia).facilities.iter().map(|f| g.facility_yield(Seat(0), StateId::Asia, f.kind).emissions).sum();
    let e = g.emissions_now();
    let asia_share = e.factories + e.power_plants + e.refineries - g.facility_yield(Seat(0), StateId::Africa, FacilityKind::Factory).emissions;
    assert!((card - asia_share).abs() < 1e-9, "card {card} climate {asia_share}");
}

// ---------------------------------------------------------------- #24 start buildings

#[test]
fn every_state_starts_with_its_start_facilities_and_the_faction_states_add_a_launch_site() {
    let g = fresh();
    for sid in StateId::ALL {
        let card = g.tables.state(sid);
        let have: Vec<FacilityKind> = g.state(sid).facilities.iter().map(|f| f.kind).collect();
        let mut want = card.start_facilities.clone();
        if g.state(sid).control.controller().is_some() {
            want.push(FacilityKind::LaunchSite);
        }
        assert_eq!(have, want, "{}", card.name);
        assert_eq!(card.start_facilities.len() as u32, card.industry_level, "{}: as many as the Industry Level", card.name);
    }
    assert_eq!(g.seats[0].stockpile, Stockpile { materials: 80, fuel: 20, energy: 20, ducats: 0 });
}

#[test]
fn idle_facilities_in_a_neutral_state_make_nothing_and_emit_nothing() {
    let mut g = fresh();
    // Africa is neutral and starts with a Factory.
    assert_eq!(g.state(StateId::Africa).control, Control::Neutral);
    assert!(g.state(StateId::Africa).facilities.iter().any(|f| f.kind == FacilityKind::Factory));
    let before = g.emissions_now().factories;
    g.state_mut(StateId::Africa).control = Control::Controlled(Seat(0));
    let after = g.emissions_now().factories;
    assert!(after > before, "the Factory emits once somebody directs it: {before} -> {after}");
}

#[test]
fn start_income_flows_from_turn_one() {
    let mut g = fresh();
    g.start();
    let s = g.seat(Seat(0));
    assert!(s.income_last_turn.materials > 0, "Materials income on turn one: {:?}", s.income_last_turn);
    assert!(s.income_last_turn.fuel > 0, "Fuel income on turn one: {:?}", s.income_last_turn);
    // Asia's start (Factory, Power Plant, Refinery and the Launch Site) pays 7 Energy against 6 made.
    assert_eq!(s.income_last_turn.energy, -1, "{:?}", s.income_last_turn);
    assert!(s.stockpile.energy >= 15, "no Energy starvation at the start: {:?}", s.stockpile);
}

// ---------------------------------------------------------------- #31 housekeeping rules

#[test]
fn only_climate_cards_scale_with_the_temperature() {
    let mut g = game();
    g.climate.temperature = 3.0; // scale 1.9 for a Climate card
    let mut seen = 0;
    for id in [EventId::MeteorShower, EventId::SolarMaximum, EventId::RichSeam, EventId::Heatwave] {
        // Force the next draw to be this card; roll until the Draw Chance lets it through.
        let mut drawn = None;
        for _ in 0..50 {
            g.deck.cards = vec![Card::Event(id)];
            g.last_event = None;
            g.event_phase();
            if let Some(e) = &g.last_event {
                drawn = Some(e.clone());
                break;
            }
        }
        let e = drawn.expect("the card came within fifty rolls");
        let climate = g.tables.event(id).kind == EventKind::Climate;
        if climate {
            assert!((e.scale - 1.9).abs() < 1e-9, "{id:?} scale {}", e.scale);
        } else {
            assert_eq!(e.scale, 1.0, "{id:?} must not scale");
        }
        seen += 1;
    }
    assert_eq!(seen, 4);
    // And a Meteor Shower does one damage at +3.0 as at +1.2.
    let mk = |id: u32| Ship { id: ShipId(id), kind: UnitKind::Frigate, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1 };
    g.ships.push(mk(1));
    drawn(&mut g, EventId::MeteorShower, EventTarget::Everyone);
    g.apply_event_now();
    assert_eq!(g.ship(ShipId(1)).unwrap().damage, 1);
}

#[test]
fn a_place_taken_by_influence_keeps_every_facility() {
    for seed in 1..=5u64 {
        let mut g = Game::new(tables(), NewGame { seed, seats: [(FactionKind::Custodians, false), (FactionKind::Prospectors, true)], player_start: StateId::Asia });
        g.state_mut(StateId::Africa).facilities = (0..8).map(|_| facility(FacilityKind::Factory)).collect();
        g.seats[0].influence.insert(Place::State(StateId::Africa), 60);
        g.seats[0].influenced_this_turn.push(Place::State(StateId::Africa));
        g.resolution_phase();
        assert_eq!(g.state(StateId::Africa).control, Control::Controlled(Seat(0)), "seed {seed}");
        assert_eq!(g.state(StateId::Africa).facilities.len(), 8, "seed {seed}: nothing destroyed by Influence");
    }
}

// ---------------------------------------------------------------- the whole loop holds together

#[test]
fn an_ai_versus_ai_game_runs_to_an_outcome() {
    let t = tables();
    let r = dying_earth_engine::sim::run(t.clone(), 3, [FactionKind::Custodians, FactionKind::Prospectors]);
    assert!(r.outcome.is_some());
    assert!(r.last_turn <= t.victory.turns);
}

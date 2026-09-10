//! The formula tests spec 19.4 asks for, one per pinned rule.

use dying_earth_engine::combat::{self, Combatant, Dice};
use dying_earth_engine::data::{default_data_dir, Tables};
use dying_earth_engine::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::VecDeque;
use std::sync::Arc;

fn tables() -> Arc<Tables> {
    Arc::new(Tables::load(&default_data_dir()).expect("tables load"))
}

/// A player Custodian in Asia against the three AI Factions, before the first turn runs, on a bare
/// board: the start Facilities of ticket #24 are stripped (Launch Sites stay) so each test places
/// exactly the buildings it reasons about. `fresh()` keeps the real start.
/// Ticket #50: seat 1 is the Prospectors, seat 2 the Arkwrights, seat 3 the Archivists.
fn game() -> Game {
    let mut g = fresh();
    for s in &mut g.states {
        s.facilities.retain(|f| f.kind == FacilityKind::LaunchSite);
    }
    g
}

fn fresh() -> Game {
    with_seed(7)
}

fn with_seed(seed: u64) -> Game {
    Game::new(tables(), NewGame { seed, player: FactionKind::Custodians, player_is_ai: false, player_start: StateId::EastAsia })
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
        in_orbit: false,
    });
    id
}

/// The seat's station over a Body (ticket #46): the one it starts with over Earth, or None.
fn station_of(g: &Game, seat: Seat, body: BodyId) -> Option<ColonyId> {
    g.colonies.iter().find(|c| c.in_orbit && c.body == body && c.control == Control::Controlled(seat)).map(|c| c.id)
}

// ---------------------------------------------------------------- 7.2 Income shortfall order

#[test]
fn income_shortfall_shuts_highest_upkeep_first_modules_before_facilities_then_alphabetical() {
    let mut g = game();
    g.seats[0].stockpile.energy = 0;
    let st = g.state_mut(StateId::EastAsia);
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
    let st = g.state_mut(StateId::EastAsia);
    st.facilities.clear();
    st.facilities.push(facility(FacilityKind::Factory)); // 2
    st.facilities.push(facility(FacilityKind::Refinery)); // 3
    st.facilities.push(facility(FacilityKind::ResearchLab)); // 3
    // 4 - 8 = -4: shut Refinery (3) -> -1, then Research Lab (3) -> 2. Factory stays on.
    assert_eq!(g.shortfall_order(Seat(0)), vec!["Refinery", "Research Lab"]);
    g.income_phase();
    let st = g.state(StateId::EastAsia);
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
    let asia_before = g.build_slots(StateId::EastAsia);
    g.climate_phase();
    assert_eq!(g.build_slots(StateId::EastAsia), asia_before - 2, "Asia has Coastal Exposure 2");
    g.climate_phase();
    assert_eq!(g.build_slots(StateId::EastAsia), asia_before - 2, "the same threshold never fires twice");
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
    assert_eq!(stats.hits_of(0), 3);
    assert_eq!(stats.hits_of(1), 0);
    assert!(d[0].destroyed());
    assert_eq!(stats.all_destroyed(), vec!["Colony Ship 2".to_string()]);
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
    assert_eq!(stats.all_escaped(), vec!["Frigate 1".to_string()]);
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
    // Ticket #53: North Africa is Size 2, so 20 + 10 * 2 = 40.
    assert_eq!(g.influence_threshold(Place::State(StateId::NorthAfrica)), 40);
    g.seats[0].allotment = 100;
    g.pending.influence.push((Seat(0), Place::State(StateId::NorthAfrica), 39));
    g.pending.influence.push((Seat(0), Place::State(StateId::Europe), 10));
    g.resolution_phase();
    assert_eq!(g.state(StateId::NorthAfrica).control, Control::Neutral, "39 is under the threshold");
    assert_eq!(g.seats[0].influence[&Place::State(StateId::NorthAfrica)], 39);
    // Next turn: 1 more on North Africa flips it; Europe, untouched, decays by 2.
    g.pending.influence.push((Seat(0), Place::State(StateId::NorthAfrica), 1));
    g.resolution_phase();
    assert_eq!(g.state(StateId::NorthAfrica).control, Control::Controlled(Seat(0)));
    assert_eq!(g.seats[0].influence[&Place::State(StateId::NorthAfrica)], 40, "the standing persists through the transfer (#33)");
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
    g.seats[0].influence.insert(Place::State(StateId::NorthAfrica), 60);
    g.seats[0].influenced_this_turn.push(Place::State(StateId::NorthAfrica));
    g.resolution_phase();
    assert_eq!(g.state(StateId::NorthAfrica).control, Control::Controlled(Seat(0)));
    // Seat 1 reaches the threshold but not the controller's standing: no change.
    g.seats[1].influence.insert(Place::State(StateId::NorthAfrica), 55);
    g.seats[1].influenced_this_turn.push(Place::State(StateId::NorthAfrica));
    g.seats[0].influenced_this_turn.push(Place::State(StateId::NorthAfrica));
    g.resolution_phase();
    assert_eq!(g.state(StateId::NorthAfrica).control, Control::Controlled(Seat(0)), "55 is not above 60");
    // Version 0.04 (ticket #41): above the controller's standing but inside the challenge margin
    // of 10: still no change. That is what stops a place flipping back and forth every turn.
    g.seats[1].influence.insert(Place::State(StateId::NorthAfrica), 69);
    g.seats[1].influenced_this_turn.push(Place::State(StateId::NorthAfrica));
    g.seats[0].influenced_this_turn.push(Place::State(StateId::NorthAfrica));
    g.resolution_phase();
    assert_eq!(g.state(StateId::NorthAfrica).control, Control::Controlled(Seat(0)), "69 is not 60 plus the margin of 10");
    // The controller's standing plus the margin: it flips, and seat 0 keeps its 60 to contest it back.
    g.seats[1].influence.insert(Place::State(StateId::NorthAfrica), 70);
    g.seats[1].influenced_this_turn.push(Place::State(StateId::NorthAfrica));
    g.seats[0].influenced_this_turn.push(Place::State(StateId::NorthAfrica));
    g.resolution_phase();
    assert_eq!(g.state(StateId::NorthAfrica).control, Control::Controlled(Seat(1)));
    assert_eq!(g.seats[0].influence[&Place::State(StateId::NorthAfrica)], 60);
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
    // Ticket #52: three turns of Occupation leave the state at Unrest 5, and from 4 the occupier's
    // Pacification gain is halved, so it needs a standing of its own to end up holding the place.
    g.seats[0].influence.insert(Place::State(StateId::Europe), 20);
    g.armies.retain(|a| a.home != ArmyHome::State(StateId::Europe));
    occupier_in(&mut g, StateId::EastAsia, StateId::Europe);
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
    // Ticket #53: Custodians hold East Asia (4): (10 + 4) x 1.3 = 18.2 -> 18. Europe is still 5.
    assert_eq!(g.influence_allotment(Seat(0)), 18);
    assert_eq!(g.influence_allotment(Seat(1)), 15);
    g.state_mut(StateId::NorthAmerica).control = Control::Controlled(Seat(1));
    assert_eq!(g.influence_allotment(Seat(1)), 22, "North America adds 7");
    // Raising East Asia's Industry Level adds one to its value.
    g.state_mut(StateId::EastAsia).industry_level += 1;
    assert_eq!(g.state_influence_value(StateId::EastAsia), 5);
    assert_eq!(g.influence_allotment(Seat(0)), 19, "(10 + 5) x 1.3 = 19.5");
    // Ticket #53: twelve states share out the eight states' figures exactly, so the total stands.
    let total: i64 = StateId::ALL.iter().map(|s| g.tables.state(*s).influence).sum();
    assert_eq!(total, 34, "7 + 5 + 4 + 4 + 4 + 2 + 2 + 2 + 1 + 1 + 1 + 1, as the eight totalled 34");
}

// ---------------------------------------------------------------- #35 Ducats

#[test]
fn a_controlled_state_pays_ducats_from_gdp_times_industry_and_a_bank_adds_more() {
    let mut g = game();
    // Ticket #53: East Asia gdp 23 x Industry 3 / 10 = 6 a turn; Europe 20 x 3 / 10 = 6.
    assert_eq!(g.state_ducats(StateId::EastAsia), 6);
    assert_eq!(g.state_ducats(StateId::Europe), 6);
    let paid = income_of(&mut g, Seat(0));
    assert_eq!(paid.ducats, 6);
    // A Bank in East Asia adds 4 x 23 / 10 = 9; in North Africa (gdp 1) it would add nothing.
    g.state_mut(StateId::EastAsia).facilities.push(facility(FacilityKind::Bank));
    assert_eq!(g.facility_yield(Seat(0), StateId::EastAsia, FacilityKind::Bank).amount, 9);
    assert_eq!(g.facility_yield(Seat(0), StateId::NorthAfrica, FacilityKind::Bank).amount, 0);
    assert_eq!(income_of(&mut g, Seat(0)).ducats, 15);
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
    let spend = Order::Influence { target: Place::State(StateId::NorthAfrica), amount: 30 };
    assert!(g.check_order(Seat(0), &pending, &spend).is_ok());
    g.commit_orders(Seat(0), &[buy, spend]);
    assert_eq!(g.seats[0].stockpile.ducats, 0);
    assert_eq!(g.seats[0].allotment, 2);
}

#[test]
fn ducats_pay_for_restoration_and_repairs_at_the_table_rates() {
    let mut g = game();
    // Version 0.04 (ticket #41): two Ducats for one of the thing bought. A Restoration step is
    // 10 Energy, so 20 Ducats; a repair point is 5 Materials, so 10 Ducats.
    g.seats[0].stockpile.ducats = 50;
    g.seats[0].stockpile.energy = 0;
    let r = Order::RestorationWithDucats { steps: 2 };
    assert_eq!(g.order_cost(Seat(0), &r).ducats, 40);
    g.commit_orders(Seat(0), &[r]);
    assert!((g.climate.restoration_next - 6.0).abs() < 1e-9, "two steps of 3.0 ppm");
    assert_eq!(g.seats[0].stockpile.ducats, 10);
    // A repair: 10 Ducats a point, same legality as a Materials repair.
    g.ships.push(Ship { id: ShipId(1), kind: UnitKind::Frigate, seat: Seat(0), damage: 1, at: ShipAt::Body(BodyId::Earth), colonists: 0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1 });
    let fix = Order::RepairWithDucats { unit: UnitRef::Ship(ShipId(1)), points: 1 };
    assert_eq!(g.order_cost(Seat(0), &fix).ducats, 10);
    assert!(g.check_order(Seat(0), &[], &fix).is_ok());
    g.commit_orders(Seat(0), &[fix]);
    g.resolution_phase();
    assert_eq!(g.ship(ShipId(1)).unwrap().damage, 0);
    assert_eq!(g.seats[0].stockpile.ducats, 0);
    assert!(g.check_order(Seat(0), &[], &Order::RepairWithDucats { unit: UnitRef::Ship(ShipId(1)), points: 1 }).is_err(), "nothing to repair");
}

// ---------------------------------------------------------------- #46 orbital slots and stations

#[test]
fn a_station_is_built_for_materials_in_an_orbital_slot_and_holds_only_a_shipyard_and_habitats() {
    let mut g = game();
    let slots: Vec<u32> = BodyId::ALL.iter().map(|b| g.tables.body(*b).orbital_slots).collect();
    assert_eq!(slots, vec![5, 2, 3, 1, 1], "ticket #50: five orbital slots over Earth");
    // The start (ticket #50): the Custodians' ISS, the Prospectors' Tiangong and the Archivists'
    // Axiom over Earth, bare; the Arkwrights start with no station, so two slots stand free.
    let iss = station_of(&g, Seat(0), BodyId::Earth).expect("the Custodians start with a station");
    let tiangong = station_of(&g, Seat(1), BodyId::Earth).expect("the Prospectors start with a station");
    let axiom = station_of(&g, Seat(3), BodyId::Earth).expect("the Archivists start with a station");
    assert_eq!(g.place_name(Place::Colony(iss)), "ISS over Earth");
    assert_eq!(g.place_name(Place::Colony(tiangong)), "Tiangong over Earth");
    assert_eq!(g.place_name(Place::Colony(axiom)), "Axiom over Earth");
    assert!(station_of(&g, Seat(2), BodyId::Earth).is_none(), "the Arkwrights start with no station");
    assert!(g.colony(iss).unwrap().modules.is_empty(), "no Shipyard at the start");
    assert!(g.colony(iss).unwrap().in_orbit);
    assert_eq!(g.free_orbital_slots(BodyId::Earth), vec![3, 4]);
    assert_eq!(g.free_slots_on(BodyId::Earth).len(), 3, "stations take no surface slot");
    // Built for 40 Materials from a state with a Launch Site (Earth) or a Colony (elsewhere); no crew.
    let build = Order::BuildStation { body: BodyId::Earth, slot: 3 };
    assert_eq!(g.order_cost(Seat(0), &build).materials, 40);
    assert!(g.check_order(Seat(0), &[], &build).is_ok(), "Asia has a Launch Site");
    assert!(g.check_order(Seat(0), &[], &Order::BuildStation { body: BodyId::Mars, slot: 0 }).is_err(), "nothing of the Custodians' at Mars");
    assert!(g.check_order(Seat(0), &[], &Order::BuildStation { body: BodyId::Earth, slot: 0 }).is_err(), "the ISS is there");
    g.commit_orders(Seat(0), &[build]);
    assert_eq!(g.seats[0].stockpile.materials, 40);
    g.resolution_phase();
    let reef = g.station_at(BodyId::Earth, 3).expect("built at the Resolution");
    assert_eq!(g.place_name(Place::Colony(reef.id)), "Orbital Reef over Earth");
    assert_eq!(reef.control, Control::Controlled(Seat(0)));
    assert_eq!(reef.colonists, 0);
    // Only a Shipyard and Habitats stand on a station.
    assert!(g.check_order(Seat(0), &[], &Order::BuildModule { colony: iss, kind: ModuleKind::Mine }).is_err(), "nothing to dig in orbit");
    assert!(g.check_order(Seat(0), &[], &Order::BuildModule { colony: iss, kind: ModuleKind::Shipyard }).is_ok());
    assert!(g.check_order(Seat(0), &[], &Order::BuildModule { colony: iss, kind: ModuleKind::Habitat }).is_ok());
    // A bare station is not free to take: its threshold starts at the station base.
    assert_eq!(g.influence_threshold(Place::Colony(iss)), 20);
    // A Colony on Mars lets the seat build a station over Mars.
    colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Mine], 0);
    assert!(g.check_order(Seat(0), &[], &Order::BuildStation { body: BodyId::Mars, slot: 0 }).is_ok());
}

#[test]
fn ships_are_built_only_at_shipyards_and_lifts_need_a_launch_site() {
    let mut g = game();
    let iss = station_of(&g, Seat(0), BodyId::Earth).unwrap();
    let frigate = |site: Place| Order::BuildShip { site, kind: UnitKind::Frigate };
    assert!(g.check_order(Seat(0), &[], &frigate(Place::State(StateId::EastAsia))).is_err(), "a Launch Site builds no Ship now");
    assert!(g.check_order(Seat(0), &[], &frigate(Place::Colony(iss))).is_err(), "no Shipyard on the ISS yet");
    g.colony_mut(iss).unwrap().modules.push(Module::new(ModuleKind::Shipyard));
    assert!(g.check_order(Seat(0), &[], &frigate(Place::Colony(iss))).is_ok());
    // Lifts: a Ship at Earth loads Colonists only from a state with a Launch Site, and each lift is a launch.
    let ship = ShipId(g.fresh_id());
    g.ships.push(Ship { id: ship, kind: UnitKind::ColonyShip, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1 });
    g.state_mut(StateId::NorthAfrica).control = Control::Controlled(Seat(0));
    g.state_mut(StateId::NorthAfrica).facilities.retain(|f| f.kind != FacilityKind::LaunchSite);
    let from_africa = Order::Load { ship, colonists: 2, from: LoadSource::State(StateId::NorthAfrica), army: None };
    assert!(g.check_order(Seat(0), &[], &from_africa).is_err(), "no Launch Site in Africa");
    let from_asia = Order::Load { ship, colonists: 2, from: LoadSource::State(StateId::EastAsia), army: None };
    assert!(g.check_order(Seat(0), &[], &from_asia).is_ok());
    g.commit_orders(Seat(0), &[from_asia]);
    assert_eq!(g.climate.launches_pending[0], 1, "a lift is a launch");
    // Leaving orbit is no launch: the Ship is already up.
    g.commit_orders(Seat(0), &[Order::Transit { ship, to: BodyId::Moon }]);
    assert_eq!(g.climate.launches_pending[0], 1);
}

// ---------------------------------------------------------------- #45 Phobos and Deimos

#[test]
fn phobos_and_deimos_are_small_different_bodies_one_hop_past_mars() {
    let g = game();
    assert_eq!(BodyId::ALL.len(), 5);
    let ph = g.tables.body(BodyId::Phobos).clone();
    let de = g.tables.body(BodyId::Deimos).clone();
    assert_eq!(ph.name, "Phobos");
    assert_eq!(de.name, "Deimos");
    assert_eq!((ph.colony_slots(), de.colony_slots()), (2, 1));
    assert_eq!((ph.mine_yield, ph.generator_yield, ph.refinery_yield, ph.habitat_yield), (1.75, 0.75, 0.5, 0.5));
    assert_eq!((de.mine_yield, de.generator_yield, de.refinery_yield, de.habitat_yield), (1.0, 1.0, 0.25, 0.5));
    // Reach: five turns and 24 Fuel from Earth (and from the Moon, which counts as Earth), one turn
    // and 2 Fuel from Mars, one turn and 1 Fuel between the two moons; Earth to Mars stays 4 and 20.
    assert_eq!(g.transit_cost(BodyId::Earth, BodyId::Phobos), (5, 24));
    assert_eq!(g.transit_cost(BodyId::Deimos, BodyId::Earth), (5, 24));
    assert_eq!(g.transit_cost(BodyId::Moon, BodyId::Deimos), (5, 24));
    assert_eq!(g.transit_cost(BodyId::Mars, BodyId::Phobos), (1, 2));
    assert_eq!(g.transit_cost(BodyId::Deimos, BodyId::Mars), (1, 2));
    assert_eq!(g.transit_cost(BodyId::Phobos, BodyId::Deimos), (1, 1));
    assert_eq!(g.transit_cost(BodyId::Earth, BodyId::Mars), (4, 20));
    assert_eq!(g.transit_cost(BodyId::Moon, BodyId::Mars), (4, 20));
    assert_eq!(g.transit_cost(BodyId::Earth, BodyId::Moon), (1, 6));
    // Their Colonists are off Earth.
    let mut g = g;
    colony(&mut g, Seat(0), BodyId::Phobos, &[ModuleKind::Habitat], 4);
    assert_eq!(g.off_world_colonists(Seat(0)), 4);
}

#[test]
fn every_colony_slot_is_a_named_place_on_its_body() {
    let g = game();
    for b in BodyId::ALL {
        let card = g.tables.body(b);
        assert_eq!(card.slots.len() as u32, card.colony_slots(), "{}", card.name);
        for sl in &card.slots {
            assert!(!sl.name.is_empty(), "{}: a slot without a name", card.name);
            assert!((-180.0..=180.0).contains(&sl.lon) && (-90.0..=90.0).contains(&sl.lat), "{}: {} off the globe", card.name, sl.name);
        }
    }
    assert_eq!(g.tables.body(BodyId::Moon).slots[0].name, "Mare Tranquillitatis");
    assert_eq!(g.tables.body(BodyId::Mars).slots[0].name, "Olympus Mons");
    assert_eq!(g.tables.body(BodyId::Earth).slots[0].name, "Antarctic Peninsula");
    assert_eq!(g.tables.body(BodyId::Phobos).slots[0].name, "Stickney");
    assert_eq!(g.tables.body(BodyId::Deimos).slots[0].name, "Swift");
    // A Colony is named for its slot.
    let mut g = g;
    let c = colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Habitat], 4);
    assert_eq!(g.place_name(Place::Colony(c)), "Olympus Mons on Mars");
}

// ---------------------------------------------------------------- #44 Antarctica

#[test]
fn antarctica_is_three_colony_slots_on_earth_whose_colonists_stay_on_earth_and_whose_modules_emit() {
    let mut g = game();
    let earth = g.tables.body(BodyId::Earth).clone();
    assert_eq!(earth.colony_slots(), 3, "three Colony Slots on Earth, in Antarctica");
    assert_eq!((earth.mine_yield, earth.generator_yield, earth.refinery_yield, earth.habitat_yield), (1.0, 0.75, 1.5, 0.75));
    assert_eq!(g.free_slots_on(BodyId::Earth).len(), 3);
    assert_eq!(StateId::ALL.len(), 12, "twelve Nation States since ticket #53, and Antarctica is none of them");
    let m = g.tables.faction(FactionKind::Custodians).emissions_multiplier;
    let before = g.emissions_now();
    colony(&mut g, Seat(0), BodyId::Earth, &[ModuleKind::Habitat, ModuleKind::Mine, ModuleKind::Refinery, ModuleKind::Generator], 4);
    assert_eq!(g.off_world_colonists(Seat(0)), 0, "Antarctic Colonists live on Earth");
    let after = g.emissions_now();
    assert!((after.factories - before.factories - 1.0 * m).abs() < 1e-9, "a Mine on Earth emits as a Factory does");
    assert!((after.refineries - before.refineries - 1.5 * m).abs() < 1e-9, "a Refinery on Earth emits as one on a state does");
    assert!((after.power_plants - before.power_plants - 1.5 * m).abs() < 1e-9, "a Generator on Earth emits as a Power Plant does");
    // Off Earth a Mine emits nothing, as before.
    colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Mine], 0);
    let off = g.emissions_now();
    assert!((off.factories - after.factories).abs() < 1e-9);
}

// ---------------------------------------------------------------- #43 Colony Ship and Carrier

#[test]
fn only_a_carrier_carries_an_army_and_a_colony_ship_carries_only_colonists() {
    let mut g = game();
    let army = ArmyId(g.fresh_id());
    g.armies.push(Army { id: army, home: ArmyHome::State(StateId::EastAsia), at: ArmyAt::Place(Place::State(StateId::EastAsia)), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None });
    let ship = |id: u32, kind: UnitKind| Ship { id: ShipId(id), kind, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1 };
    g.ships.extend([ship(101, UnitKind::ColonyShip), ship(102, UnitKind::Battleship), ship(103, UnitKind::Carrier)]);
    let load_army = |s: u32| Order::Load { ship: ShipId(s), colonists: 0, from: LoadSource::State(StateId::EastAsia), army: Some(army) };
    assert!(g.check_order(Seat(0), &[], &load_army(101)).is_err(), "a Colony Ship carries Colonists only");
    assert!(g.check_order(Seat(0), &[], &load_army(102)).is_err(), "a Battleship fights; it carries no Army");
    assert!(g.check_order(Seat(0), &[], &load_army(103)).is_ok(), "a Carrier carries one Army");
    assert!(g.check_order(Seat(0), &[], &Order::Load { ship: ShipId(103), colonists: 1, from: LoadSource::State(StateId::EastAsia), army: None }).is_err(), "a Carrier carries no Colonists");
    assert!(g.check_order(Seat(0), &[], &Order::Load { ship: ShipId(101), colonists: 4, from: LoadSource::State(StateId::EastAsia), army: None }).is_ok());
    let card = g.tables.unit(UnitKind::Carrier);
    assert_eq!(card.materials, 30);
    assert_eq!(card.build_turns, 1);
    assert_eq!(card.energy_upkeep, 2);
    assert_eq!(card.strength, 0);
    assert_eq!(card.hit_points, 4);
    assert_eq!(card.pursuit, 0);
    assert!(!UnitKind::Carrier.is_warship());
}

// ---------------------------------------------------------------- #42 the trading window

#[test]
fn the_trading_window_sells_materials_fuel_and_energy_at_the_table_prices() {
    let mut g = game();
    g.seats[0].stockpile = Stockpile { materials: 0, fuel: 0, energy: 0, ducats: 100 };
    let m = Order::Buy { resource: Resource::Materials, amount: 10 };
    assert_eq!(g.order_cost(Seat(0), &m).ducats, 20, "Materials are 2 Ducats each");
    // Bought Materials are spendable at once: a Factory (20 Materials) is affordable with the buy pending.
    let factory = Order::BuildFacility { state: StateId::EastAsia, kind: FacilityKind::Factory };
    assert!(g.check_order(Seat(0), &[], &factory).is_err(), "no Materials yet");
    let pending = vec![Order::Buy { resource: Resource::Materials, amount: 20 }];
    let (left, _) = g.remaining(Seat(0), &pending);
    assert_eq!((left.materials, left.ducats), (20, 60));
    assert!(g.check_order(Seat(0), &pending, &factory).is_ok());
    let f = Order::Buy { resource: Resource::Fuel, amount: 2 };
    assert_eq!(g.order_cost(Seat(0), &f).ducats, 6, "Fuel is 3 Ducats each");
    let e = Order::Buy { resource: Resource::Energy, amount: 5 };
    assert_eq!(g.order_cost(Seat(0), &e).ducats, 5, "Energy is 1 Ducat each");
    assert!(g.check_order(Seat(0), &[], &Order::Buy { resource: Resource::Materials, amount: 0 }).is_err(), "a positive amount");
    assert!(g.check_order(Seat(0), &[], &Order::Buy { resource: Resource::Ducats, amount: 5 }).is_err(), "Ducats are not for sale");
    assert!(g.check_order(Seat(0), &[], &Order::Buy { resource: Resource::Materials, amount: 51 }).is_err(), "102 Ducats needed, 100 held");
    g.commit_orders(Seat(0), &[m, f, e]);
    assert_eq!(g.seats[0].stockpile, Stockpile { materials: 10, fuel: 2, energy: 5, ducats: 69 });
}

#[test]
fn a_building_bought_for_ducats_costs_twice_its_materials_and_queues_like_a_materials_build() {
    let mut g = game();
    g.seats[0].stockpile = Stockpile { materials: 0, fuel: 0, energy: 50, ducats: 40 };
    let order = Order::BuildFacilityWithDucats { state: StateId::EastAsia, kind: FacilityKind::Factory };
    let cost = g.order_cost(Seat(0), &order);
    assert_eq!((cost.materials, cost.ducats), (0, 40), "a 20-Materials Factory is 40 Ducats");
    assert!(g.check_order(Seat(0), &[], &order).is_ok());
    // It takes a build slot like any build: Asia has one free slot on the bare board, so a second is refused.
    let free = g.free_slots(StateId::EastAsia);
    let mut pending = Vec::new();
    for _ in 0..free {
        pending.push(order.clone());
    }
    assert!(g.check_order_legality(Seat(0), &pending, &Order::BuildFacility { state: StateId::EastAsia, kind: FacilityKind::Factory }).is_err(), "no free build slot");
    g.commit_orders(Seat(0), &[order]);
    assert_eq!(g.seats[0].stockpile.ducats, 0);
    assert_eq!(g.state(StateId::EastAsia).queue.len(), 1);
    assert_eq!(g.state(StateId::EastAsia).queue[0].item, BuildItem::Facility(FacilityKind::Factory));
    // A Module too: a Mine on a Colony.
    let c = colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Habitat], 4);
    let mine = Order::BuildModuleWithDucats { colony: c, kind: ModuleKind::Mine };
    assert_eq!(g.order_cost(Seat(0), &mine).ducats, 2 * g.tables.module(ModuleKind::Mine).materials);
    assert!(g.check_order(Seat(0), &[], &mine).is_err(), "no Ducats left");
}

#[test]
fn selling_materials_or_fuel_returns_half_the_buying_price() {
    let mut g = game();
    g.seats[0].stockpile = Stockpile { materials: 10, fuel: 2, energy: 20, ducats: 0 };
    let m = Order::Sell { resource: Resource::Materials, amount: 10 };
    assert_eq!(g.order_cost(Seat(0), &m).ducats, -10, "half of 2 Ducats each");
    let f = Order::Sell { resource: Resource::Fuel, amount: 2 };
    assert_eq!(g.order_cost(Seat(0), &f).ducats, -3, "half of 3 Ducats each, rounded down over the lot");
    assert!(g.check_order(Seat(0), &[], &Order::Sell { resource: Resource::Materials, amount: 11 }).is_err(), "10 held");
    assert!(g.check_order(Seat(0), &[], &Order::Sell { resource: Resource::Energy, amount: 5 }).is_err(), "Energy is not bought back");
    // The Ducats from a sale are spendable at once.
    let pending = vec![m.clone()];
    let (left, _) = g.remaining(Seat(0), &pending);
    assert_eq!((left.materials, left.ducats), (0, 10));
    assert!(g.check_order(Seat(0), &pending, &Order::BuyInfluence { amount: 5 }).is_ok());
    g.commit_orders(Seat(0), &[m, f]);
    assert_eq!(g.seats[0].stockpile, Stockpile { materials: 0, fuel: 0, energy: 20, ducats: 13 });
}

// ---------------------------------------------------------------- #36 Embassies and Relays

#[test]
fn embassies_and_relays_add_to_the_allotment_and_raise_their_places_standing_each_turn() {
    let mut g = game();
    // Custodians in East Asia: (10 + 4) x 1.3 = 18. Two Embassies (they stack) add 4: (10 + 4 + 4) x 1.3 = 23.
    assert_eq!(g.influence_allotment(Seat(0)), 18);
    g.state_mut(StateId::EastAsia).facilities.push(facility(FacilityKind::Embassy));
    g.state_mut(StateId::EastAsia).facilities.push(facility(FacilityKind::Embassy));
    assert_eq!(g.building_allotment(Seat(0)), 4);
    assert_eq!(g.influence_allotment(Seat(0)), 23);
    // A Relay in a Colony adds 1 more.
    let c = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Habitat, ModuleKind::Relay], 4);
    assert_eq!(g.influence_allotment(Seat(0)), 24, "(10 + 4 + 5) x 1.3 = 24.7");
    // Each Resolution the standing rises by the buildings' figures and does not decay.
    g.resolution_phase();
    assert_eq!(g.seats[0].influence[&Place::State(StateId::EastAsia)], 4, "two Embassies, 2 each");
    assert_eq!(g.seats[0].influence[&Place::Colony(c)], 2, "one Relay");
    g.resolution_phase();
    assert_eq!(g.seats[0].influence[&Place::State(StateId::EastAsia)], 8);
    // An offline Embassy adds nothing.
    for f in g.state_mut(StateId::EastAsia).facilities.iter_mut().filter(|f| f.kind == FacilityKind::Embassy) {
        f.online = false;
    }
    assert_eq!(g.building_allotment(Seat(0)), 1, "only the Relay");
    g.resolution_phase();
    assert_eq!(g.seats[0].influence[&Place::State(StateId::EastAsia)], 7, "no rise, and decay 1 on your own place");
    // The card says what they do.
    let y = g.facility_yield(Seat(0), StateId::EastAsia, FacilityKind::Embassy);
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
    occupier_in(&mut g, StateId::EastAsia, StateId::Europe);
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
    occupier_in(&mut g, StateId::EastAsia, StateId::Europe);
    // Threshold 50; 40 already accumulated; one turn of Occupation adds ceil(50/3) = 17.
    g.seats[0].influence.insert(Place::State(StateId::Europe), 40);
    g.resolution_phase();
    assert_eq!(g.state(StateId::Europe).control, Control::Controlled(Seat(0)), "Pacified in the first turn");
}

#[test]
fn occupation_ends_when_the_occupier_leaves() {
    let mut g = game();
    // Africa is neutral (Europe is the AI's start state).
    g.armies.retain(|a| a.home != ArmyHome::State(StateId::NorthAfrica));
    let a = occupier_in(&mut g, StateId::EastAsia, StateId::NorthAfrica);
    g.resolution_phase();
    assert!(matches!(g.state(StateId::NorthAfrica).control, Control::Occupied { occupier: Seat(0), previous: None, turns: 1 }));
    g.armies.retain(|x| x.id != a);
    g.resolution_phase();
    assert_eq!(g.state(StateId::NorthAfrica).control, Control::Neutral);
}

#[test]
fn occupation_of_a_controlled_state_returns_it_to_its_owner_when_broken() {
    let mut g = game();
    g.armies.retain(|a| a.home != ArmyHome::State(StateId::Europe));
    let a = occupier_in(&mut g, StateId::EastAsia, StateId::Europe);
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
        FactionKind::Arkwrights => {}
        FactionKind::Archivists => g.seats[seat.index()].research_total = 150,
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
fn launch_pad_fire_closes_a_launch_site_unless_clean_propellant_is_known() {
    let mut g = game();
    g.turn = 3;
    drawn(&mut g, EventId::LaunchPadFire, EventTarget::State(StateId::EastAsia));
    g.resolution_phase();
    assert!(!g.state(StateId::EastAsia).facilities.iter().find(|f| f.kind == FacilityKind::LaunchSite).unwrap().online, "the Launch Site is offline");
    let ship = ShipId(g.fresh_id());
    g.ships.push(Ship { id: ship, kind: UnitKind::ColonyShip, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1 });
    assert!(g.check_order(Seat(0), &[], &Order::Load { ship, colonists: 2, from: LoadSource::State(StateId::EastAsia), army: None }).is_err(), "nothing lifts from a closed Launch Site");
    // With Clean Propellant the Launch Site stays open.
    let mut g = game();
    g.turn = 3;
    with_tech(&mut g, TechId::CleanPropellant);
    drawn(&mut g, EventId::LaunchPadFire, EventTarget::State(StateId::EastAsia));
    g.resolution_phase();
    assert!(g.state(StateId::EastAsia).facilities.iter().find(|f| f.kind == FacilityKind::LaunchSite).unwrap().online);
}

#[test]
fn labour_dispute_idles_a_states_facilities_at_the_next_income_and_public_science_spares_all_but_one() {
    let mut g = game();
    g.state_mut(StateId::EastAsia).facilities = vec![facility(FacilityKind::Factory), facility(FacilityKind::Refinery), facility(FacilityKind::PowerPlant)];
    drawn(&mut g, EventId::LabourDispute, EventTarget::State(StateId::EastAsia));
    g.apply_event_now();
    let paid = income_of(&mut g, Seat(0));
    assert_eq!((paid.materials, paid.fuel), (0, 0), "nothing made: {paid:?}");
    g.last_event = None;
    g.resolution_phase();
    let paid = income_of(&mut g, Seat(0));
    assert!(paid.materials > 0 && paid.fuel > 0, "back at work after Resolution: {paid:?}");
    with_tech(&mut g, TechId::PublicScience);
    drawn(&mut g, EventId::LabourDispute, EventTarget::State(StateId::EastAsia));
    g.apply_event_now();
    let idle = g.state(StateId::EastAsia).facilities.iter().filter(|f| f.offline_until_resolution).count();
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
    g.state_mut(StateId::EastAsia).facilities = vec![facility(FacilityKind::PowerPlant)];
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

// The Unrest card's Army damage and Standing loss were retired on ticket #52: it is a flat +3
// Unrest now, pinned by `j_the_unrest_card_adds_three` below.

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
    g.state_mut(StateId::EastAsia).facilities = vec![facility(FacilityKind::PowerPlant)];
    let plain = income_of(&mut g, Seat(0)).energy;
    with_tech(&mut g, TechId::EfficientGrids);
    let boosted = income_of(&mut g, Seat(0)).energy;
    assert_eq!(plain, 6);
    assert_eq!(boosted, 9);
}

#[test]
fn tech_clean_power_cuts_power_plant_emissions() {
    let mut g = game();
    g.state_mut(StateId::EastAsia).facilities = vec![facility(FacilityKind::PowerPlant)];
    let plain = g.emissions_now().power_plants;
    with_tech(&mut g, TechId::CleanPower);
    let clean = g.emissions_now().power_plants;
    assert!((plain - 1.5 * 0.75).abs() < 1e-9, "Custodians x0.75: {plain}");
    assert!((clean - 1.5 * 0.4 * 0.75).abs() < 1e-9);
}

#[test]
fn tech_clean_manufacturing_cuts_factory_and_refinery_emissions() {
    let mut g = game();
    g.state_mut(StateId::EastAsia).facilities = vec![facility(FacilityKind::Factory), facility(FacilityKind::Refinery)];
    with_tech(&mut g, TechId::CleanManufacturing);
    let e = g.emissions_now();
    assert!((e.factories - 1.0 * 0.4 * 0.75).abs() < 1e-9);
    assert!((e.refineries - 1.5 * 0.4 * 0.75).abs() < 1e-9);
}

#[test]
fn tech_clean_propellant_makes_a_launch_emit_less() {
    let mut g = game();
    g.climate.launches_pending = [1, 0, 0, 0];
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
    g.state_mut(StateId::EastAsia).facilities.clear();
    let plain = income_of(&mut g, Seat(0)).energy;
    with_tech(&mut g, TechId::ClosedLoopColonies);
    let halved = income_of(&mut g, Seat(0)).energy;
    assert_eq!(plain, -3);
    assert_eq!(halved, -1, "3 x 0.5 rounded down");
}

#[test]
fn tech_deep_mining_raises_mine_and_factory_output() {
    let mut g = game();
    g.state_mut(StateId::EastAsia).facilities = vec![facility(FacilityKind::Factory)];
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
    g.state_mut(StateId::EastAsia).facilities = vec![facility(FacilityKind::Refinery)];
    assert_eq!(income_of(&mut g, Seat(0)).fuel, 3);
    with_tech(&mut g, TechId::AutomatedRefining);
    assert_eq!(income_of(&mut g, Seat(0)).fuel, 4, "4.5 rounded down");
}

#[test]
fn tech_public_science_raises_research_lab_output() {
    let mut g = game();
    g.state_mut(StateId::EastAsia).facilities = vec![facility(FacilityKind::ResearchLab)];
    g.research.current = Some(TechId::CleanPower);
    g.research.done.push(TechId::EfficientGrids);
    g.seats[0].stockpile.energy = 1000;
    g.income_phase();
    // Ticket #53, East Asia: 2 x (1 + 16.4/50) x 1.1 x 1.25 = 3.65 -> 3
    assert_eq!(g.seats[0].research_last_turn, 3);
    with_tech(&mut g, TechId::PublicScience);
    g.income_phase();
    assert_eq!(g.seats[0].research_last_turn, 5, "5.48 rounded down");
}

#[test]
fn tech_green_consensus_halves_per_person_emissions_and_lowers_thresholds() {
    let mut g = game();
    let before = g.emissions_now().population;
    assert_eq!(g.influence_threshold(Place::State(StateId::NorthAfrica)), 40);
    with_tech(&mut g, TechId::GreenConsensus);
    let after = g.emissions_now().population;
    assert!((after - before / 2.0).abs() < 1e-9);
    assert_eq!(g.influence_threshold(Place::State(StateId::NorthAfrica)), 30, "40 x 0.75");
}

// ---------------------------------------------------------------- 19.3 a defended Colony changes hands

/// The fifth anchor, scripted: two Carriers land Armies on a rival Colony defended by a Barracks Army.
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
    let mut g = with_seed(seed);
    // The AI Prospectors hold a Colony on the Moon with a Barracks and its Army.
    let cid = colony(&mut g, Seat(1), BodyId::Moon, &[ModuleKind::Habitat, ModuleKind::Barracks], 4);
    let defender = ArmyId(g.fresh_id());
    g.armies.push(Army { id: defender, home: ArmyHome::Colony(cid), at: ArmyAt::Place(Place::Colony(cid)), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None });
    // The player's two Carriers arrive at the Moon, each carrying an Army: strength 8 against 4.
    let mut attackers = Vec::new();
    let mut ships = Vec::new();
    for kind in [UnitKind::Carrier, UnitKind::Carrier] {
        let attacker = ArmyId(g.fresh_id());
        let ship = ShipId(g.fresh_id());
        g.armies.push(Army { id: attacker, home: ArmyHome::State(StateId::EastAsia), at: ArmyAt::Aboard(ship), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None });
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
        g.end_turn([orders, Vec::new(), Vec::new(), Vec::new()]);
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
    g.state_mut(StateId::EastAsia).facilities = vec![facility(FacilityKind::Factory), facility(FacilityKind::Refinery), facility(FacilityKind::ResearchLab), facility(FacilityKind::PowerPlant)];
    g.state_mut(StateId::NorthAfrica).control = Control::Controlled(Seat(0));
    g.state_mut(StateId::NorthAfrica).facilities = vec![facility(FacilityKind::Factory)];
    let c = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Mine, ModuleKind::Generator, ModuleKind::Refinery], 0);
    g.research.done.push(TechId::DeepMining);
    g.research.current = Some(TechId::CleanPower);
    let mut expect = Stockpile::default();
    let mut research = 0;
    for sid in [StateId::EastAsia, StateId::NorthAfrica] {
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
    let card: f64 = g.state(StateId::EastAsia).facilities.iter().map(|f| g.facility_yield(Seat(0), StateId::EastAsia, f.kind).emissions).sum();
    let e = g.emissions_now();
    let asia_share = e.factories + e.power_plants + e.refineries - g.facility_yield(Seat(0), StateId::NorthAfrica, FacilityKind::Factory).emissions;
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
    // Ticket #53: North Africa is neutral and, leaning Fuel, starts with a Refinery.
    assert_eq!(g.state(StateId::NorthAfrica).control, Control::Neutral);
    assert!(g.state(StateId::NorthAfrica).facilities.iter().any(|f| f.kind == FacilityKind::Refinery));
    let before = g.emissions_now().refineries;
    g.state_mut(StateId::NorthAfrica).control = Control::Controlled(Seat(0));
    let after = g.emissions_now().refineries;
    assert!(after > before, "the Refinery emits once somebody directs it: {before} -> {after}");
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
        let mut g = with_seed(seed);
        g.state_mut(StateId::NorthAfrica).facilities = (0..8).map(|_| facility(FacilityKind::Factory)).collect();
        g.seats[0].influence.insert(Place::State(StateId::NorthAfrica), 60);
        g.seats[0].influenced_this_turn.push(Place::State(StateId::NorthAfrica));
        g.resolution_phase();
        assert_eq!(g.state(StateId::NorthAfrica).control, Control::Controlled(Seat(0)), "seed {seed}");
        assert_eq!(g.state(StateId::NorthAfrica).facilities.len(), 8, "seed {seed}: nothing destroyed by Influence");
    }
}

// ---------------------------------------------------------------- the whole loop holds together

#[test]
fn an_ai_versus_ai_game_runs_to_an_outcome() {
    let t = tables();
    let r = dying_earth_engine::sim::run(t.clone(), 3, FactionKind::Custodians);
    assert!(r.outcome.is_some());
    assert!(r.last_turn <= t.victory.turns);
}


// ================================================================ #50 four Factions in every game

// ---------------------------------------------------------------- 10.2 the melee

/// A party of one unit with the strength, hit points and pursuit given.
fn party(id: u32, name: &str, strength: i64, hp: u32, pursuit: u32) -> Vec<Combatant> {
    vec![Combatant::new(UnitRef::Ship(ShipId(id)), name.to_string(), strength, hp, 0, pursuit, false)]
}

#[test]
fn a_partys_chance_to_land_a_hit_is_its_share_of_the_total_strength_present() {
    // Two parties: exactly the old p = A / (A + D).
    assert!((combat::hit_share(&[3, 1], 0) - 0.75).abs() < 1e-12);
    assert!((combat::hit_share(&[3, 1], 1) - 0.25).abs() < 1e-12);
    assert!((combat::hit_share(&[3, 3], 0) - 0.5).abs() < 1e-12);
    // Three parties: each party's share of the whole strength present.
    assert!((combat::hit_share(&[6, 3, 1], 0) - 0.6).abs() < 1e-12);
    assert!((combat::hit_share(&[6, 3, 1], 1) - 0.3).abs() < 1e-12);
    assert!((combat::hit_share(&[6, 3, 1], 2) - 0.1).abs() < 1e-12);
    assert_eq!(combat::hit_share(&[0, 0, 0], 0), 0.0);
    // And the melee itself lands hits in those proportions.
    let mut rng = ChaCha8Rng::seed_from_u64(11);
    let mut landed = [0u32; 3];
    for _ in 0..400 {
        let mut a = party(1, "A Frigate", 6, 10_000, 0);
        let mut b = party(2, "B Frigate", 3, 10_000, 0);
        let mut c = party(3, "C Frigate", 1, 10_000, 0);
        let stats = combat::melee(&mut [&mut a, &mut b, &mut c], &mut rng as &mut dyn Dice);
        for (i, l) in landed.iter_mut().enumerate() {
            *l += stats.hits_of(i);
        }
    }
    let total: u32 = landed.iter().sum();
    let share = |i: usize| landed[i] as f64 / total as f64;
    assert!((share(0) - 0.6).abs() < 0.05, "party A landed {:.3} of the hits, expected 0.60: {landed:?}", share(0));
    assert!((share(1) - 0.3).abs() < 0.05, "party B landed {:.3} of the hits, expected 0.30: {landed:?}", share(1));
    assert!((share(2) - 0.1).abs() < 0.05, "party C landed {:.3} of the hits, expected 0.10: {landed:?}", share(2));
}

#[test]
fn a_partys_hits_are_spread_across_the_enemy_parties_in_proportion_to_their_strength() {
    // The rule in figures: the attacker's own share is nothing, the enemies' shares are their
    // strengths over the enemy total.
    let shares = combat::target_shares(&[6, 3, 1], 0);
    assert_eq!(shares[0], 0.0);
    assert!((shares[1] - 0.75).abs() < 1e-12);
    assert!((shares[2] - 0.25).abs() < 1e-12);
    // With one enemy it is a certainty.
    assert_eq!(combat::target_shares(&[3, 1], 0), vec![0.0, 1.0]);
    // And the melee spreads the damage that way: B has three times C's strength, so it takes about
    // three times the damage from A.
    let mut rng = ChaCha8Rng::seed_from_u64(23);
    let (mut b_damage, mut c_damage) = (0u32, 0u32);
    for _ in 0..400 {
        let mut a = party(1, "A Frigate", 6, 10_000, 0);
        let mut b = party(2, "B Frigate", 3, 10_000, 0);
        let mut c = party(3, "C Frigate", 1, 10_000, 0);
        combat::melee(&mut [&mut a, &mut b, &mut c], &mut rng as &mut dyn Dice);
        b_damage += b[0].damage;
        c_damage += c[0].damage;
    }
    assert!(c_damage > 0, "the weaker party was never hit at all: {b_damage} to {c_damage}");
    let ratio = b_damage as f64 / c_damage as f64;
    assert!((2.0..4.5).contains(&ratio), "B took {b_damage} and C {c_damage}, a ratio of {ratio:.2}; strength says about 3");
}

// ---------------------------------------------------------------- 9.3 Orbital Control in a melee

#[test]
fn orbital_control_needs_the_only_engaged_warship_at_the_body() {
    let mut g = game();
    let warship = |id: u32, seat: Seat| Ship {
        id: ShipId(id),
        kind: UnitKind::Frigate,
        seat,
        damage: 0,
        at: ShipAt::Body(BodyId::Mars),
        colonists: 0,
        army: None,
        stance: Stance::Hold,
        escaped: false,
        arrived_this_turn: false,
        built_turn: 1,
    };
    assert_eq!(g.orbital_control(BodyId::Mars), None, "nobody is there");
    g.ships.push(warship(201, Seat(0)));
    assert_eq!(g.orbital_control(BodyId::Mars), Some(Seat(0)), "one seat's warship, engaged, alone");
    g.ships.push(warship(202, Seat(1)));
    assert_eq!(g.orbital_control(BodyId::Mars), None, "two seats have engaged warships");
    g.ships.push(warship(203, Seat(2)));
    assert_eq!(g.orbital_control(BodyId::Mars), None, "three seats, still nobody");
    // A warship that escaped is no longer engaged and contests nothing.
    for s in g.ships.iter_mut().filter(|s| s.seat != Seat(2) && s.at == ShipAt::Body(BodyId::Mars)) {
        s.escaped = true;
    }
    assert_eq!(g.orbital_control(BodyId::Mars), Some(Seat(2)), "only the Arkwrights are still engaged");
    // Landing follows control.
    assert!(g.may_land(Seat(2), BodyId::Mars));
    assert!(!g.may_land(Seat(0), BodyId::Mars));
}

// ---------------------------------------------------------------- 15 victory over four seats

#[test]
fn more_than_one_seat_meeting_its_condition_gives_the_game_to_the_larger_margin() {
    let mut g = game();
    // The Custodians and the Prospectors both meet theirs; the Prospectors by the larger margin.
    colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Habitat, ModuleKind::Habitat], 12);
    colony(&mut g, Seat(1), BodyId::Mars, &[ModuleKind::Habitat, ModuleKind::Habitat, ModuleKind::Habitat], 15);
    g.seats[0].stabilization_run = 3; // parts 1.0 and 1.0 -> margin 1.0
    g.seats[1].extraction_total = 600; // parts 1.2 and 1.25 -> margin 1.2
    g.end_phase();
    assert!(matches!(g.outcome, Some(Outcome::Win { seat: Seat(1), .. })), "{:?}", g.outcome);
}

#[test]
fn the_last_turn_ranks_every_seat_by_score() {
    let mut g = game();
    g.turn = g.tables.victory.turns;
    // Each seat is scored on its own two parts (ticket #51). Custodians: 6 Colonists off Earth of
    // 12 but no Stabilization run, score 0. Prospectors: 9 of 12 and the full Extraction Total,
    // score 0.75. Arkwrights: 3 Colonists of 30 and no Body with 4 on it, score 0. Archivists:
    // nothing, score 0.
    colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Habitat, ModuleKind::Habitat], 6);
    let p = colony(&mut g, Seat(1), BodyId::Moon, &[ModuleKind::Habitat, ModuleKind::Habitat, ModuleKind::Habitat], 9);
    let _ = p;
    g.seats[1].extraction_total = 500;
    colony(&mut g, Seat(2), BodyId::Phobos, &[ModuleKind::Habitat], 3);
    assert!((g.progress(Seat(1)).score() - 0.75).abs() < 1e-9, "{:?}", g.progress(Seat(1)).score());
    g.end_phase();
    assert!(matches!(g.outcome, Some(Outcome::Win { seat: Seat(1), .. })), "the highest score of four: {:?}", g.outcome);
    // With the leader's score removed, the next seat down takes it. The Arkwrights hold 4 Colonists
    // on Phobos: 4 of 30 off Earth and one Body of three, score 4/30. The Archivists hold 4 on the
    // Moon but have no Archive at all, so their first part is 0 and their score with it.
    let mut g = game();
    g.turn = g.tables.victory.turns;
    colony(&mut g, Seat(2), BodyId::Phobos, &[ModuleKind::Habitat, ModuleKind::Habitat], 4);
    colony(&mut g, Seat(3), BodyId::Moon, &[ModuleKind::Habitat], 4);
    assert!((g.progress(Seat(2)).score() - 4.0 / 30.0).abs() < 1e-9, "{:?}", g.progress(Seat(2)).score());
    assert_eq!(g.progress(Seat(3)).score(), 0.0);
    g.end_phase();
    assert!(matches!(g.outcome, Some(Outcome::Win { seat: Seat(2), .. })), "{:?}", g.outcome);
}

// ---------------------------------------------------------------- 14.3 the AI seats spread out

#[test]
fn the_ai_seats_take_start_states_not_adjacent_to_any_taken_one() {
    let g = Game::new(tables(), NewGame { seed: 7, player: FactionKind::Prospectors, player_is_ai: false, player_start: StateId::Europe });
    let held = |seat: Seat| g.controlled_states(seat);
    assert_eq!(held(Seat(0)), vec![StateId::Europe]);
    // Europe touches North America, North Africa, Russia and the Middle East, so the first AI seat
    // takes the highest Industry Level among what is left untouched: East Asia at 3.
    assert_eq!(held(Seat(1)), vec![StateId::EastAsia]);
    // East Asia adds Russia, South Asia and South-East Asia to the adjacent set; Australia is the
    // highest Industry Level still untouched, at 2.
    assert_eq!(held(Seat(2)), vec![StateId::Australia]);
    // Then the untouched states are all at Industry 1, so the tie goes to the most populous:
    // Sub-Saharan Africa at 11.4.
    assert_eq!(held(Seat(3)), vec![StateId::SubSaharanAfrica]);
    // The fallback, when every free state touches a taken one: the highest Industry Level free
    // state, ties by population. With everything above taken, North America at 3 wins.
    let taken = [StateId::Europe, StateId::EastAsia, StateId::Australia, StateId::SubSaharanAfrica, StateId::SouthAmerica, StateId::CentralAmerica];
    assert_eq!(g.ai_start_state(&taken), StateId::NorthAmerica, "the fallback picks the best free state");
    // Every seat's start carries a Launch Site.
    for seat in Seat::ALL {
        let sid = held(seat)[0];
        assert!(g.state(sid).facilities.iter().any(|f| f.kind == FacilityKind::LaunchSite), "{seat:?} has no Launch Site");
    }
}

// ---------------------------------------------------------------- 8.3 a same-turn Influence tie

#[test]
fn two_challengers_at_the_same_standing_leave_the_place_where_it_was() {
    let mut g = game();
    // Africa's threshold is 50. Two seats reach it in the same Resolution at the same Standing.
    for seat in [Seat(0), Seat(1)] {
        g.seats[seat.index()].influence.insert(Place::State(StateId::NorthAfrica), 50);
        g.seats[seat.index()].influenced_this_turn.push(Place::State(StateId::NorthAfrica));
    }
    g.resolution_phase();
    assert_eq!(g.state(StateId::NorthAfrica).control, Control::Neutral, "an exact tie goes to nobody");
    // One more point and the higher Standing takes it.
    g.seats[0].influence.insert(Place::State(StateId::NorthAfrica), 51);
    for seat in [Seat(0), Seat(1)] {
        g.seats[seat.index()].influenced_this_turn.push(Place::State(StateId::NorthAfrica));
    }
    g.resolution_phase();
    assert_eq!(g.state(StateId::NorthAfrica).control, Control::Controlled(Seat(0)), "the higher Standing takes it");
}

// ---------------------------------------------------------------- 12.2 the Research Lead tie

#[test]
fn a_tied_research_lead_goes_to_the_seat_that_picked_least_recently() {
    let mut g = game();
    g.research.contributions = [10, 10, 0, 0];
    g.research.last_picked_turn = [Some(5), Some(2), None, None];
    assert_eq!(g.research_lead(), Seat(1), "seat 1 picked on turn 2, seat 0 on turn 5");
    g.research.last_picked_turn = [Some(1), Some(3), None, None];
    assert_eq!(g.research_lead(), Seat(0), "seat 0 picked on turn 1, seat 1 on turn 3");
    // A seat that has never picked counts as longest ago.
    g.research.contributions = [10, 0, 10, 0];
    g.research.last_picked_turn = [Some(1), None, None, None];
    assert_eq!(g.research_lead(), Seat(2), "the Arkwrights have never picked");
    // The highest contributor still wins outright when there is no tie.
    g.research.contributions = [4, 9, 2, 1];
    g.research.last_picked_turn = [None, Some(9), None, None];
    assert_eq!(g.research_lead(), Seat(1));
    // And picking records the turn.
    g.turn = 7;
    g.research.current = None;
    g.pick_tech(Seat(3), TechId::EfficientGrids).expect("a free pick");
    assert_eq!(g.research.last_picked_turn[3], Some(7));
}

// ---------------------------------------------------------------- #50 ties are drawn, not ordered

#[test]
fn a_tie_between_seats_is_drawn_from_the_seed_and_is_the_same_every_replay() {
    // Two seats want the same orbital slot with no Ships anywhere: strength decides nothing, so the
    // draw does. The same seed draws the same seat twice; over many seeds both seats come up, which
    // is what tells a draw from "seat 0 wins".
    let winner = |seed: u64| {
        let mut g = with_seed(seed);
        g.tiebreak_at_body(BodyId::Mars, &[Seat(0), Seat(1)])
    };
    for seed in 1..=20u64 {
        assert_eq!(winner(seed), winner(seed), "seed {seed} drew differently on a replay");
    }
    let firsts = (1..=40u64).filter(|s| winner(*s) == Seat(0)).count();
    assert!((8..=32).contains(&firsts), "seat 0 took {firsts} of 40 draws; a draw should be near half");
    // Greater strength still decides before the draw.
    let mut g = with_seed(3);
    g.ships.push(Ship {
        id: ShipId(301),
        kind: UnitKind::Frigate,
        seat: Seat(1),
        damage: 0,
        at: ShipAt::Body(BodyId::Mars),
        colonists: 0,
        army: None,
        stance: Stance::Hold,
        escaped: false,
        arrived_this_turn: false,
        built_turn: 1,
    });
    for _ in 0..8 {
        assert_eq!(g.tiebreak_at_body(BodyId::Mars, &[Seat(0), Seat(1)]), Seat(1), "the stronger stack takes it");
    }
}

// ---------------------------------------------------------------- #51 the Arkwrights and the Archivists

/// A Colony Ship of one seat, sitting at a Body.
fn a_colony_ship(g: &mut Game, seat: Seat, body: BodyId) -> ShipId {
    let id = ShipId(g.fresh_id());
    g.ships.push(Ship {
        id,
        kind: UnitKind::ColonyShip,
        seat,
        damage: 0,
        at: ShipAt::Body(body),
        colonists: 0,
        army: None,
        stance: Stance::Hold,
        escaped: false,
        arrived_this_turn: false,
        built_turn: 1,
    });
    id
}

/// A Colony off Earth holding the seat's Archive at `stage`, with Habitats and Colonists.
fn archive_at(g: &mut Game, seat: Seat, body: BodyId, stage: u32, colonists: u32) -> ColonyId {
    let cid = colony(g, seat, body, &[ModuleKind::Habitat, ModuleKind::Habitat, ModuleKind::Habitat], colonists);
    let col = g.colony_mut(cid).unwrap();
    let mut m = Module::new(ModuleKind::Archive);
    m.stage = stage;
    col.modules.push(m);
    cid
}

#[test]
fn steerage_doubles_an_arkwright_colony_ships_load_and_cuts_its_price() {
    let mut g = game();
    // Capacity: the card figure for everyone else, twice it for the Arkwrights, and Expanded
    // Habitats adds its two before the doubling.
    assert_eq!(g.colony_ship_capacity(Seat(0)), 4);
    assert_eq!(g.colony_ship_capacity(Seat(2)), 8, "Steerage carries twice");
    g.research.done.push(TechId::ExpandedHabitats);
    assert_eq!(g.colony_ship_capacity(Seat(0)), 6);
    assert_eq!(g.colony_ship_capacity(Seat(2)), 12, "(4 + 2) doubled");
    // Price: 30 Materials on the units.toml row, 20 on the Arkwrights' card.
    let build = Order::BuildShip { site: Place::State(StateId::EastAsia), kind: UnitKind::ColonyShip };
    assert_eq!(g.order_cost(Seat(0), &build).materials, 30);
    assert_eq!(g.order_cost(Seat(2), &build).materials, 20);
    // And the Load order holds them to it.
    let sid = g.controlled_states(Seat(2))[0];
    // Ticket #53: a lift of twelve costs an Arkwright state 2.4 population, more than some of the
    // twelve states hold, so the test gives its start state people to spare.
    g.state_mut(sid).population = 10.0;
    let ship = a_colony_ship(&mut g, Seat(2), BodyId::Earth);
    let load = |n: u32| Order::Load { ship, colonists: n, from: LoadSource::State(sid), army: None };
    assert!(g.check_order(Seat(2), &[], &load(12)).is_ok());
    let err = g.check_order(Seat(2), &[], &load(13)).unwrap_err();
    assert_eq!(err.0, "this Ship carries at most 12 Colonists");
}

#[test]
fn an_arkwright_lift_takes_twice_the_population_out_of_its_state() {
    let mut g = game();
    g.state_mut(StateId::NorthAfrica).control = Control::Controlled(Seat(2));
    g.state_mut(StateId::NorthAfrica).facilities.retain(|f| f.kind != FacilityKind::LaunchSite);
    g.state_mut(StateId::NorthAfrica).facilities.push(facility(FacilityKind::LaunchSite));
    assert!((g.lift_population(Seat(0), 4) - 0.4).abs() < 1e-9);
    assert!((g.lift_population(Seat(2), 4) - 0.8).abs() < 1e-9, "Steerage costs the state twice");
    let before = g.state(StateId::NorthAfrica).population;
    let ship = a_colony_ship(&mut g, Seat(2), BodyId::Earth);
    g.commit_orders(Seat(2), &[Order::Load { ship, colonists: 4, from: LoadSource::State(StateId::NorthAfrica), army: None }]);
    g.resolution_phase();
    let taken = before - g.state(StateId::NorthAfrica).population;
    assert!((taken - 0.8).abs() < 1e-9, "the lift took {taken}, not 0.8");
    assert_eq!(g.ship(ship).unwrap().colonists, 4);
}

#[test]
fn an_arkwright_habitat_holds_six() {
    let mut g = game();
    // The Moon's Habitat yield is 1.0, so the Faction figure is all that separates them.
    let theirs = colony(&mut g, Seat(2), BodyId::Moon, &[ModuleKind::Habitat], 0);
    let mine = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Habitat], 0);
    assert_eq!(g.habitat_room(g.colony(mine).unwrap()), 4);
    assert_eq!(g.habitat_room(g.colony(theirs).unwrap()), 6, "half again for the Arkwrights");
    g.research.done.push(TechId::ExpandedHabitats);
    assert_eq!(g.habitat_room(g.colony(mine).unwrap()), 6);
    assert_eq!(g.habitat_room(g.colony(theirs).unwrap()), 9, "(4 + 2) x 1.5");
}

#[test]
fn an_arkwright_pays_half_for_a_station_and_three_quarters_for_a_module() {
    let mut g = game();
    let station = Order::BuildStation { body: BodyId::Earth, slot: 3 };
    assert_eq!(g.order_cost(Seat(0), &station).materials, 40);
    assert_eq!(g.order_cost(Seat(2), &station).materials, 20, "they start with no station");
    let cid = colony(&mut g, Seat(2), BodyId::Moon, &[], 0);
    let mine = colony(&mut g, Seat(0), BodyId::Moon, &[], 0);
    for (kind, full, theirs) in [(ModuleKind::Habitat, 25, 18), (ModuleKind::Mine, 20, 15), (ModuleKind::Shipyard, 35, 26)] {
        assert_eq!(g.order_cost(Seat(0), &Order::BuildModule { colony: mine, kind }).materials, full);
        assert_eq!(g.order_cost(Seat(2), &Order::BuildModule { colony: cid, kind }).materials, theirs, "{} x 0.75 rounded down", kind.name());
    }
    // Transit Fuel too: three quarters, then Efficient Transit on top of that.
    assert_eq!(g.transit_cost_for(Seat(0), BodyId::Earth, BodyId::Mars).1, 20);
    assert_eq!(g.transit_cost_for(Seat(2), BodyId::Earth, BodyId::Mars).1, 15);
    g.research.done.push(TechId::EfficientTransit);
    assert_eq!(g.transit_cost_for(Seat(2), BodyId::Earth, BodyId::Mars).1, 9, "20 x 0.75 x 0.6");
}

#[test]
fn diaspora_wants_three_bodies_with_four_colonists_each_and_counts_no_antarctic_one() {
    let mut g = game();
    let p = g.progress(Seat(2));
    assert_eq!(p.first_bar, 30.0, "30 Colonists off Earth");
    assert_eq!(p.second_bar, 3.0, "on at least three Bodies");
    colony(&mut g, Seat(2), BodyId::Mars, &[ModuleKind::Habitat], 4);
    colony(&mut g, Seat(2), BodyId::Moon, &[ModuleKind::Habitat], 4);
    // Antarctica is Earth's, so a Colony there settles no Body.
    colony(&mut g, Seat(2), BodyId::Earth, &[ModuleKind::Habitat], 4);
    assert_eq!(g.progress(Seat(2)).second_value, 2.0, "Antarctica is not a third Body");
    // Three Colonists on Phobos are not four.
    let ph = colony(&mut g, Seat(2), BodyId::Phobos, &[ModuleKind::Habitat], 3);
    assert_eq!(g.progress(Seat(2)).second_value, 2.0);
    g.colony_mut(ph).unwrap().colonists = 4;
    assert_eq!(g.progress(Seat(2)).second_value, 3.0);
    // A station over another Body counts for that Body.
    let sid = ColonyId(g.fresh_id());
    g.colonies.push(Colony {
        id: sid,
        body: BodyId::Deimos,
        slot: 0,
        control: Control::Controlled(Seat(2)),
        modules: vec![Module::new(ModuleKind::Habitat)],
        colonists: 4,
        queue: Vec::new(),
        grid_failed: false,
        founded_turn: 1,
        in_orbit: true,
    });
    assert_eq!(g.progress(Seat(2)).second_value, 4.0);
    // Both parts together: 30 off Earth as well.
    assert!(!g.progress(Seat(2)).met(), "16 Colonists off Earth is not 30");
    g.colony_mut(ph).unwrap().colonists = 18;
    assert_eq!(g.progress(Seat(2)).first_value, 30.0);
    assert!(g.progress(Seat(2)).met());
}

#[test]
fn funding_the_archive_banks_this_turns_research_and_contributes_nothing_to_the_lead() {
    let mut g = game();
    g.state_mut(StateId::Europe).control = Control::Controlled(Seat(3));
    g.state_mut(StateId::Europe).facilities.push(facility(FacilityKind::ResearchLab));
    g.pick_tech(Seat(0), TechId::PublicScience).unwrap();
    g.income_phase();
    let made = g.seats[3].research_last_turn;
    assert!(made > 0 && made < g.tables.tech(TechId::PublicScience).cost, "one Lab makes {made}");
    assert_eq!(g.research.contributions[3], made, "Income paid it into the shared Tech");
    let before = g.research.progress;
    g.commit_orders(Seat(3), &[Order::FundArchive]);
    assert_eq!(g.seats[3].archive_fund, made, "the whole turn's Research is banked");
    assert_eq!(g.research.contributions[3], 0, "and counts nothing toward the Research Lead");
    assert_eq!(g.research.progress, before - made, "the shared Tech gives it back");
    assert!(g.funding_archive(Seat(3)));
    assert!(g.report.lines.iter().any(|l| l.contains("Archivists are funding the Archive")), "{:?}", g.report.lines);
    // Nobody else may.
    assert_eq!(g.check_order(Seat(0), &[], &Order::FundArchive).unwrap_err().0, "only the Archivists fund the Archive");
    // The fund never holds more than the remaining stages need.
    assert_eq!(g.archive_fund_cap(Seat(3)), 80);
    g.seats[3].archive_fund = 80;
    g.seats[3].research_last_turn = 40;
    g.commit_orders(Seat(3), &[Order::FundArchive]);
    assert_eq!(g.seats[3].archive_fund, 80, "Research past what the stages need is wasted");
}

#[test]
fn a_stage_of_the_archive_needs_its_research_banked_and_a_colony_off_earth() {
    let mut g = game();
    g.seats[3].stockpile.materials = 200;
    let mars = colony(&mut g, Seat(3), BodyId::Mars, &[ModuleKind::Habitat], 4);
    let order = Order::BuildArchiveStage { colony: mars };
    assert_eq!(g.order_cost(Seat(3), &order).materials, 30);
    let err = g.check_order(Seat(3), &[], &order).unwrap_err();
    assert_eq!(err.0, "stage 1 needs 20 Research banked in the Archive fund, 0 there");
    g.seats[3].archive_fund = 20;
    assert!(g.check_order(Seat(3), &[], &order).is_ok());
    // Antarctica will not do, and neither will a station over Earth.
    let ant = colony(&mut g, Seat(3), BodyId::Earth, &[ModuleKind::Habitat], 4);
    assert!(g.check_order(Seat(3), &[], &Order::BuildArchiveStage { colony: ant }).unwrap_err().0.contains("off Earth"));
    let axiom = station_of(&g, Seat(3), BodyId::Earth).unwrap();
    assert!(g.check_order(Seat(3), &[], &Order::BuildArchiveStage { colony: axiom }).unwrap_err().0.contains("off Earth"));
    // Nobody else builds one, and the ordinary Module button never places it.
    let mine = colony(&mut g, Seat(0), BodyId::Mars, &[], 0);
    assert_eq!(g.check_order(Seat(0), &[], &Order::BuildArchiveStage { colony: mine }).unwrap_err().0, "only the Archivists build the Archive");
    assert!(g.check_order(Seat(3), &[], &Order::BuildModule { colony: mars, kind: ModuleKind::Archive }).is_err());
    // Ordering spends the banked Research, and the next stage wants its own twenty.
    g.commit_orders(Seat(3), &[order.clone()]);
    assert_eq!(g.seats[3].archive_fund, 0);
    // One stage at a time: while stage 1 is building, stage 2 cannot be ordered even with the Research banked.
    g.seats[3].archive_fund = 20;
    assert_eq!(g.check_order(Seat(3), &[], &order).unwrap_err().0, "a stage of the Archive is already building; one stage at a time");
    g.seats[3].archive_fund = 0;
    // Two turns later the stage stands.
    g.resolution_phase();
    assert_eq!(g.archive_stage(Seat(3)), 0, "two turns to raise");
    g.turn += 1;
    g.resolution_phase();
    assert_eq!(g.archive_stage(Seat(3)), 1);
    assert_eq!(g.check_order(Seat(3), &[], &order).unwrap_err().0, "stage 2 needs 20 Research banked in the Archive fund, 0 there");
    assert_eq!(g.archive_colony(Seat(3)), Some(mars));
    // At most one per Faction.
    let deimos = colony(&mut g, Seat(3), BodyId::Deimos, &[], 0);
    g.seats[3].archive_fund = 20;
    assert!(g.check_order(Seat(3), &[], &Order::BuildArchiveStage { colony: deimos }).unwrap_err().0.contains("already stands"));
}

#[test]
fn provisional_findings_halves_the_tech_under_research_and_goes_off_the_turn_after_funding() {
    let mut g = game();
    g.state_mut(StateId::Europe).control = Control::Controlled(Seat(3));
    g.state_mut(StateId::Europe).facilities.push(facility(FacilityKind::ResearchLab));
    g.pick_tech(Seat(0), TechId::PublicScience).unwrap();
    // A multiplier of 1.5 reads 1.25; nobody else reads an unfinished Tech at all.
    assert!(g.provisional_findings(Seat(3)), "on at the start: nobody has funded yet");
    assert!((g.tech_multiplier(Seat(3), TechId::PublicScience) - 1.25).abs() < 1e-9);
    assert_eq!(g.tech_multiplier(Seat(0), TechId::PublicScience), 1.0);
    // An addition of +2 reads +1, and an immunity does not carry at all.
    assert_eq!(g.tech_addition(Seat(3), TechId::ExpandedHabitats), 0, "only the Tech under research");
    let with = g.facility_yield(Seat(3), StateId::Europe, FacilityKind::ResearchLab).research;
    // A turn of funding switches it off for the turn after.
    g.commit_orders(Seat(3), &[Order::FundArchive]);
    g.income_phase();
    assert!(!g.provisional_findings(Seat(3)), "they funded last turn");
    assert_eq!(g.tech_multiplier(Seat(3), TechId::PublicScience), 1.0);
    let without = g.facility_yield(Seat(3), StateId::Europe, FacilityKind::ResearchLab).research;
    assert!(without < with, "the Lab made {with} with Provisional Findings and {without} without");
    // A turn of contributing switches it back on.
    g.income_phase();
    assert!(g.provisional_findings(Seat(3)));
    // Once the Tech is done everybody reads it whole.
    g.research.done.push(TechId::ExpandedHabitats);
    assert_eq!(g.tech_addition(Seat(3), TechId::ExpandedHabitats), 2);
}

#[test]
fn a_complete_archive_goes_offline_when_energy_runs_short_and_wins_nothing_that_end_phase() {
    let mut g = game();
    let cid = archive_at(&mut g, Seat(3), BodyId::Mars, 4, 12);
    g.seats[3].stockpile.energy = 0;
    assert_eq!(g.module_yield(Seat(3), cid, ModuleKind::Archive).upkeep, 12, "a complete Archive draws 12");
    assert_eq!(g.shortfall_order(Seat(3))[0], "The Archive", "the highest upkeep goes first");
    g.income_phase();
    assert!(!g.colony(cid).unwrap().modules.iter().any(|m| m.kind == ModuleKind::Archive && m.online), "shut down");
    assert!(!g.archive_online(Seat(3)));
    let p = g.progress(Seat(3));
    assert_eq!(p.first_value, 4.0, "every stage stands");
    assert_eq!(p.second_value, 12.0, "and the Colonists are there");
    assert!(p.first_held_back.is_some() && !p.met(), "but it is not running");
    g.end_phase();
    assert!(g.outcome.is_none(), "no win with the Archive dark: {:?}", g.outcome);
}

#[test]
fn the_archive_is_destroyed_when_its_colony_changes_hands_and_the_fund_is_kept() {
    let mut g = game();
    let cid = archive_at(&mut g, Seat(3), BodyId::Mars, 3, 6);
    g.seats[3].archive_fund = 40;
    assert_eq!(g.archive_colony(Seat(3)), Some(cid));
    g.transfer_control(Place::Colony(cid), Seat(1), "Influence");
    assert_eq!(g.archive_colony(Seat(3)), None, "the Archive went with the Colony");
    assert!(!g.colony(cid).unwrap().modules.iter().any(|m| m.kind == ModuleKind::Archive));
    assert_eq!(g.seats[3].archive_fund, 40, "the fund is kept");
    assert!(g.report.lines.iter().any(|l| l.contains("Archive at") && l.contains("destroyed")), "{:?}", g.report.lines);
    // An Occupied Colony's Archive is dark while the Occupation lasts.
    let again = archive_at(&mut g, Seat(3), BodyId::Moon, 4, 12);
    g.income_phase();
    assert!(g.archive_online(Seat(3)));
    g.colony_mut(again).unwrap().control = Control::Occupied { occupier: Seat(1), previous: Some(Seat(3)), turns: 1 };
    g.income_phase();
    assert!(!g.archive_online(Seat(3)), "an Occupied Colony's Archive is offline");
}

#[test]
fn the_archivists_win_with_the_archive_running_and_twelve_colonists_at_its_colony() {
    let mut g = game();
    let cid = archive_at(&mut g, Seat(3), BodyId::Mars, 4, 12);
    let _ = cid;
    g.seats[3].stockpile.energy = 200;
    g.income_phase();
    assert!(g.archive_online(Seat(3)));
    let p = g.progress(Seat(3));
    assert_eq!((p.first_value, p.first_bar), (4.0, 4.0));
    assert_eq!((p.second_value, p.second_bar), (12.0, 12.0));
    assert!(p.met());
    g.end_phase();
    assert!(matches!(g.outcome, Some(Outcome::Win { seat: Seat(3), .. })), "{:?}", g.outcome);
    // One Colonist short and it is no win.
    let mut g = game();
    let cid = archive_at(&mut g, Seat(3), BodyId::Mars, 4, 11);
    let _ = cid;
    g.seats[3].stockpile.energy = 200;
    g.income_phase();
    g.end_phase();
    assert!(g.outcome.is_none(), "{:?}", g.outcome);
}

#[test]
fn an_ai_with_no_station_over_earth_orders_one_from_its_launch_site() {
    // Ticket #51: the Arkwrights start with no station, so the AI must be able to build its first.
    let mut g = game();
    let ark = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Arkwrights).unwrap();
    g.seats[ark.index()].stockpile.materials = 200;
    let orders = g.ai_orders(ark);
    assert!(
        orders.iter().any(|o| matches!(o, Order::BuildStation { body: BodyId::Earth, .. })),
        "the Arkwright AI never orders a station over Earth: {orders:?}"
    );
}

#[test]
fn the_archivist_ai_funds_the_archive_before_it_holds_a_colony() {
    // Ticket #51: the fund can start on turn one; only the stage needs a Colony off Earth.
    let mut g = game();
    let arc = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Archivists).unwrap();
    assert!(g.colonies.iter().all(|c| c.in_orbit || c.control.director() != Some(arc)));
    g.seats[arc.index()].research_last_turn = 6;
    let orders = g.ai_orders(arc);
    assert!(orders.iter().any(|o| matches!(o, Order::FundArchive)), "no funding order: {orders:?}");
}

// ---------------------------------------------------------------- Ticket #52: Unrest, Occupation and refugees

/// Every state calm, with every sea-level threshold already fired, so a Climate-phase test sees
/// only what it set up.
fn calm(g: &mut Game) {
    let n = g.tables.climate.sea_level_thresholds.len();
    for s in &mut g.states {
        s.unrest = 0.0;
        s.changed_hands = false;
        s.unrest_reported = 0.0;
        s.refugees_in = 0.0;
        s.thresholds_fired = vec![true; n];
    }
}

/// Hold the Temperature still: the CO2 Stock that makes `temp` its own target.
fn hold_temperature(g: &mut Game, temp: f64) {
    let c = &g.tables.climate;
    g.climate.co2 = c.starting_co2 + (temp - c.base_temperature) * c.ppm_step / c.degrees_per_ppm_step;
    g.climate.temperature = temp;
}

fn constabulary_in(g: &mut Game, sid: StateId) {
    g.state_mut(sid).facilities.push(facility(FacilityKind::Constabulary));
}

/// (a) A Climate phase in which a state's population fell raises its Unrest by 1, and by 2 when it
/// fell by more than one per cent.
#[test]
fn a_population_fall_raises_unrest_by_one_and_a_fall_over_one_percent_by_two() {
    let mut g = game();
    calm(&mut g);
    hold_temperature(&mut g, 2.0);
    let rate = g.population_growth_rate();
    assert!(rate < 0.0 && rate > -0.01, "a small fall: {rate}");
    g.climate_phase();
    assert_eq!(g.unrest(StateId::EastAsia), 1.5, "a fall of less than one per cent raises Unrest by one and a half");

    let mut g = game();
    calm(&mut g);
    hold_temperature(&mut g, 2.6);
    let rate = g.population_growth_rate();
    assert!(rate < -0.01, "a fall of more than one per cent: {rate}");
    g.climate_phase();
    assert_eq!(g.unrest(StateId::EastAsia), 2.0, "a fall of more than one per cent raises Unrest by two");
}

/// (b) A Sea Level threshold raises Unrest by 2 per build slot lost and drives 5% of the people out
/// per point of Coastal Exposure, half of them to the neighbours in proportion to Industry Level.
#[test]
fn b_a_sea_level_threshold_raises_two_a_slot_and_displaces_five_percent_an_exposure() {
    let mut g = game();
    calm(&mut g);
    // East Asia: Coastal Exposure 2; neighbours Russia, South Asia and South-East Asia.
    let pop = 40.0;
    g.state_mut(StateId::EastAsia).population = pop;
    for s in [StateId::Russia, StateId::SouthAsia, StateId::SouthEastAsia] {
        g.state_mut(s).population = 0.0;
    }
    g.state_mut(StateId::Russia).industry_level = 3;
    g.state_mut(StateId::SouthAsia).industry_level = 1;
    g.state_mut(StateId::SouthEastAsia).industry_level = 0;
    g.apply_sea_threshold(StateId::EastAsia, 0);
    assert_eq!(g.unrest(StateId::EastAsia), 2.0, "one per build slot, and Asia is exposed 2");
    let displaced = pop * 0.05 * 2.0;
    assert!((g.state(StateId::EastAsia).population - (pop - displaced)).abs() < 1e-9, "{}", g.state(StateId::EastAsia).population);
    let moved = displaced * 0.5;
    assert!((g.state(StateId::Russia).population - moved * 0.75).abs() < 1e-9, "Russia {}", g.state(StateId::Russia).population);
    assert!((g.state(StateId::SouthAsia).population - moved * 0.25).abs() < 1e-9, "South Asia {}", g.state(StateId::SouthAsia).population);
    assert_eq!(g.state(StateId::SouthEastAsia).population, 0.0, "an Industry Level of 0 takes none while another has some");
    assert!(g.report.lines.iter().any(|l| l.contains("Unrest there rose")), "the Report says so: {:?}", g.report.lines);
}

/// (c) Heat refugees: half of what a state lost arrives at its neighbours and raises their Unrest
/// by one per half a person, at most three in a turn.
#[test]
fn c_heat_refugees_arrive_at_the_neighbours_and_raise_unrest_per_half_a_person() {
    let mut g = game();
    calm(&mut g);
    for s in &mut g.states {
        s.population = 0.0;
    }
    let before = 100.0;
    g.state_mut(StateId::EastAsia).population = before;
    // Russia is the only neighbour of East Asia's with any Industry Level, so it takes the whole flow.
    g.state_mut(StateId::Russia).industry_level = 2;
    g.state_mut(StateId::SouthAsia).industry_level = 0;
    g.state_mut(StateId::SouthEastAsia).industry_level = 0;
    hold_temperature(&mut g, 3.0);
    assert!(g.population_growth_rate() < 0.0);
    g.climate_phase();
    let lost = before - g.state(StateId::EastAsia).population;
    let arrived = lost * 0.5;
    assert!(arrived > 0.5 && arrived < 1.0, "the flow is worth exactly one point of Unrest: {arrived}");
    assert!((g.state(StateId::Russia).population - arrived).abs() < 1e-6, "Russia took the flow: {}", g.state(StateId::Russia).population);
    assert!(
        g.report.lines.iter().any(|l| l.contains("left East Asia for") && l.contains("Russia")),
        "a refugee line naming where they went: {:?}",
        g.report.lines
    );
    // Russia changed nothing and holds nobody, so the turn's fall of 1.5 nets against the rise.
    g.state_mut(StateId::Russia).changed_hands = true;
    g.resolve_unrest();
    let want = (arrived / 0.5).floor();
    assert_eq!(g.unrest(StateId::Russia), want.min(2.0), "one Unrest per half a person arriving");

    // The cap: eight people arriving in a turn is still only two, where #52 allowed three.
    let mut g = game();
    calm(&mut g);
    g.state_mut(StateId::Russia).refugees_in = 8.0;
    g.state_mut(StateId::Russia).changed_hands = true;
    g.resolve_unrest();
    assert_eq!(g.unrest(StateId::Russia), 2.0, "at most two from refugees in a turn");
}

/// (d) Occupation: +3 when it begins and +1 a turn after; the figure carries over when control
/// transfers by force; Pacification is halved from Unrest 4.
#[test]
fn d_occupation_raises_three_then_one_a_turn_carries_over_and_halves_pacification() {
    let mut g = game();
    calm(&mut g);
    g.armies.retain(|a| a.home != ArmyHome::State(StateId::Europe));
    occupier_in(&mut g, StateId::EastAsia, StateId::Europe);
    g.resolution_phase();
    assert!(g.state(StateId::Europe).control.is_occupied(), "{:?}", g.state(StateId::Europe).control);
    // The turn a place changes hands it goes without its natural fall (ticket #53), so the whole
    // +3 stands; the turns after, the fall of 1.5 nets against the +1 Occupation adds.
    assert_eq!(g.unrest(StateId::Europe), 3.0, "Occupation begins at +3");
    g.resolution_phase();
    assert_eq!(g.unrest(StateId::Europe), 2.5, "a turn of Occupation adds one against a fall of 1.5");
    g.resolution_phase();
    assert_eq!(g.state(StateId::Europe).control, Control::Controlled(Seat(0)), "the third turn transfers it");
    assert_eq!(g.unrest(StateId::Europe), 3.5, "the figure carries over, and a transfer takes no fall");

    // Pacification: one third of the threshold rounded up while calm, one sixth from Unrest 4.
    let mut g = game();
    calm(&mut g);
    let threshold = g.influence_threshold(Place::State(StateId::NorthAfrica));
    assert_eq!(g.pacification_gain(Place::State(StateId::NorthAfrica)), (threshold + 2) / 3, "the calm gain");
    g.state_mut(StateId::NorthAfrica).unrest = 4.0;
    assert_eq!(g.pacification_gain(Place::State(StateId::NorthAfrica)), (threshold + 5) / 6, "halved from Unrest 4");
}

/// (e) The thresholds: 4 stops the Standing Army replenishing, 7 halves output and Emissions, 10
/// throws the controller off with every Standing kept and Unrest back at 5.
#[test]
fn e_four_stops_replenishment_seven_halves_output_ten_throws_the_controller_off() {
    let mut g = game();
    calm(&mut g);
    g.take_control(StateId::NorthAfrica, Seat(0));
    let damage = |g: &Game| g.armies.iter().find(|a| a.standing && a.home == ArmyHome::State(StateId::NorthAfrica)).unwrap().damage;
    g.armies.iter_mut().find(|a| a.standing && a.home == ArmyHome::State(StateId::NorthAfrica)).unwrap().damage = 1;
    g.state_mut(StateId::NorthAfrica).unrest = 3.0;
    g.income_phase();
    assert_eq!(damage(&g), 0, "below Unrest 4 the Standing Army replenishes");
    g.armies.iter_mut().find(|a| a.standing && a.home == ArmyHome::State(StateId::NorthAfrica)).unwrap().damage = 1;
    g.state_mut(StateId::NorthAfrica).unrest = 4.0;
    g.income_phase();
    assert_eq!(damage(&g), 1, "at Unrest 4 it does not");

    // 7: half the output and half the Emissions.
    let mut g = game();
    calm(&mut g);
    g.take_control(StateId::EastAsia, Seat(0));
    g.state_mut(StateId::EastAsia).facilities.push(facility(FacilityKind::Factory));
    let full = g.facility_yield(Seat(0), StateId::EastAsia, FacilityKind::Factory);
    let all_full = g.emissions_now().factories;
    g.state_mut(StateId::EastAsia).unrest = 7.0;
    let half = g.facility_yield(Seat(0), StateId::EastAsia, FacilityKind::Factory);
    assert!(full.amount > 0);
    assert_eq!(half.amount, full.amount / 2, "output at half, rounded down (full {})", full.amount);
    assert!((half.emissions - full.emissions / 2.0).abs() < 1e-9, "Emissions at half: {} of {}", half.emissions, full.emissions);
    assert!((g.emissions_now().factories - all_full / 2.0).abs() < 1e-9, "and the Climate Panel charges the half");

    // 10: the state throws its controller off.
    let mut g = game();
    calm(&mut g);
    g.take_control(StateId::NorthAfrica, Seat(0));
    g.take_control(StateId::Europe, Seat(0));
    g.seats[0].influence.insert(Place::State(StateId::NorthAfrica), 42);
    g.seats[1].influence.insert(Place::State(StateId::NorthAfrica), 17);
    g.state_mut(StateId::NorthAfrica).queue.push(Build { item: BuildItem::Facility(FacilityKind::Bank), seat: Seat(0), due_turn: 99 });
    let id = ArmyId(g.fresh_id());
    g.armies.push(Army { id, home: ArmyHome::State(StateId::Europe), at: ArmyAt::Place(Place::State(StateId::NorthAfrica)), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None });
    g.raise_unrest(StateId::NorthAfrica, 10.0, UnrestSource::Plain);
    assert_eq!(g.unrest(StateId::NorthAfrica), 10.0);
    // Ticket #53: the falls run first, so a state at 10 that did not change hands is pulled back
    // to 8.5 and keeps its controller. Only a state with no fall to take crosses.
    g.resolve_unrest();
    assert_eq!(g.state(StateId::NorthAfrica).control, Control::Controlled(Seat(0)), "the fall saved it");
    assert_eq!(g.unrest(StateId::NorthAfrica), 8.5, "pulled back by the turn's fall");
    g.raise_unrest(StateId::NorthAfrica, 10.0, UnrestSource::Plain);
    g.state_mut(StateId::NorthAfrica).changed_hands = true;
    g.resolve_unrest();
    assert_eq!(g.state(StateId::NorthAfrica).control, Control::Neutral, "it throws its controller off");
    assert_eq!(g.unrest(StateId::NorthAfrica), 5.0, "and settles back to 5");
    assert_eq!(g.seats[0].influence.get(&Place::State(StateId::NorthAfrica)).copied(), Some(42), "every Faction's Standing stays");
    assert_eq!(g.seats[1].influence.get(&Place::State(StateId::NorthAfrica)).copied(), Some(17));
    assert_eq!(g.state(StateId::NorthAfrica).queue.len(), 1, "the build queue is kept");
    assert_eq!(g.army(id).unwrap().home, ArmyHome::State(StateId::NorthAfrica), "the Armies there become the state's own");
    assert!(g.army_seat(g.army(id).unwrap()).is_none(), "so they fight for nobody");
    assert!(g.report.lines.iter().any(|l| l.contains("threw off")), "a Report line names it: {:?}", g.report.lines);
}

/// (f) A neutral state's Unrest is capped at 9, and a Faction taking it by Influence inherits it.
#[test]
fn f_a_neutral_state_caps_at_nine_and_the_faction_that_takes_it_inherits_its_unrest() {
    let mut g = game();
    calm(&mut g);
    assert_eq!(g.state(StateId::NorthAfrica).control, Control::Neutral);
    g.raise_unrest(StateId::NorthAfrica, 20.0, UnrestSource::Plain);
    assert_eq!(g.unrest(StateId::NorthAfrica), 9.0, "a neutral state stops at 9");
    g.resolve_unrest();
    assert_eq!(g.state(StateId::NorthAfrica).control, Control::Neutral, "and throws nobody off, since it holds nobody");
    assert_eq!(g.unrest(StateId::NorthAfrica), 7.5, "and the turn's fall lands whatever raised it");
    g.state_mut(StateId::NorthAfrica).unrest = 9.0;
    g.transfer_control(Place::State(StateId::NorthAfrica), Seat(1), "Influence");
    assert_eq!(g.unrest(StateId::NorthAfrica), 9.0, "the Faction that takes it inherits its Unrest");
    g.raise_unrest(StateId::NorthAfrica, 5.0, UnrestSource::Plain);
    assert_eq!(g.unrest(StateId::NorthAfrica), 10.0, "and now the ceiling is 10");
}

/// (g) Relief costs 10 Ducats a point; a Constabulary lowers Unrest by 1 a turn and damps a
/// climate rise by 1; at most one Constabulary stands in a state.
#[test]
fn g_relief_costs_ten_ducats_a_point_and_a_constabulary_calms_and_damps() {
    let mut g = game();
    calm(&mut g);
    g.take_control(StateId::NorthAfrica, Seat(0));
    g.state_mut(StateId::SouthAmerica).control = Control::Neutral;
    g.state_mut(StateId::NorthAfrica).unrest = 6.0;
    g.seats[0].stockpile.ducats = 30;
    let order = Order::Relief { state: StateId::NorthAfrica };
    assert_eq!(g.order_cost(Seat(0), &order).ducats, 10, "10 Ducats a point");
    assert!(g.check_order(Seat(0), &[], &order).is_ok());
    assert!(g.check_order(Seat(0), &[], &Order::Relief { state: StateId::SouthAmerica }).is_err(), "only on a state you direct");
    g.commit_orders(Seat(0), &[order.clone(), order.clone()]);
    assert_eq!(g.seats[0].stockpile.ducats, 10, "two orders, twenty Ducats");
    g.resolve_unrest();
    assert_eq!(g.unrest(StateId::NorthAfrica), 2.5, "two points of Relief and the turn's fall of 1.5 off six");

    let mut g = game();
    calm(&mut g);
    g.take_control(StateId::EastAsia, Seat(0));
    constabulary_in(&mut g, StateId::EastAsia);
    g.state_mut(StateId::EastAsia).unrest = 5.0;
    assert_eq!(g.unrest_damping(StateId::EastAsia, UnrestSource::Climate), 0.5);
    assert_eq!(g.raise_unrest(StateId::EastAsia, 2.0, UnrestSource::Climate), 1.5, "a climate rise of two arrives as one and a half");
    assert_eq!(g.raise_unrest(StateId::EastAsia, 2.0, UnrestSource::Plain), 2.0, "nothing damps Occupation, a mothball or the card");
    let before = g.unrest(StateId::EastAsia);
    g.resolve_unrest();
    assert_eq!(g.unrest(StateId::EastAsia), before - 2.5, "the Constabulary's 1 and the turn's own 1.5");
    g.seats[0].stockpile.materials = 200;
    assert!(
        g.check_order(Seat(0), &[], &Order::BuildFacility { state: StateId::EastAsia, kind: FacilityKind::Constabulary }).is_err(),
        "at most one Constabulary per Nation State"
    );
}

/// (h) Two green Techs damp a climate rise by 1, four by 2, and nothing falls below 0. With the
/// hook the neutral-development rule of a later ticket will read.
#[test]
fn h_two_green_techs_damp_a_climate_rise_by_one_and_four_by_two() {
    let mut g = game();
    calm(&mut g);
    assert_eq!(g.raise_unrest(StateId::NorthAfrica, 2.0, UnrestSource::Climate), 2.0, "no Techs, no damping");
    g.state_mut(StateId::NorthAfrica).unrest = 0.0;
    g.research.done.push(TechId::CleanPropellant);
    g.research.done.push(TechId::CleanPower);
    assert_eq!(g.green_techs_done(), 2);
    assert_eq!(g.raise_unrest(StateId::NorthAfrica, 2.0, UnrestSource::Climate), 1.5, "two green Techs take a half off");
    g.state_mut(StateId::NorthAfrica).unrest = 0.0;
    g.research.done.push(TechId::CleanManufacturing);
    g.research.done.push(TechId::GreenConsensus);
    assert_eq!(g.green_techs_done(), 4);
    assert_eq!(g.raise_unrest(StateId::NorthAfrica, 2.0, UnrestSource::Climate), 1.0, "four green Techs take one off");
    assert_eq!(g.raise_unrest(StateId::NorthAfrica, 1.0, UnrestSource::Climate), 0.0, "and it never goes below zero");
    assert_eq!(g.unrest(StateId::NorthAfrica), 1.0, "damping never lowers Unrest by itself");
    // Ticket #53: the green Techs moderate arriving refugees too, which they did not on #52.
    assert_eq!(g.raise_unrest(StateId::NorthAfrica, 3.0, UnrestSource::Refugees), 2.0, "four green Techs damp refugees as well");
    assert_eq!(g.raise_unrest(StateId::NorthAfrica, 3.0, UnrestSource::Plain), 3.0, "but never Occupation or the card");
    let mut g = game();
    calm(&mut g);
    assert!(g.may_develop(StateId::NorthAfrica), "a calm state develops itself");
    g.state_mut(StateId::NorthAfrica).unrest = 7.0;
    assert!(!g.may_develop(StateId::NorthAfrica), "a state at 7 or more does not");
}

/// (i) Resettle routes this turn's flows to the chosen state and adds 5 Standing there.
#[test]
fn i_resettle_routes_the_flow_and_adds_five_standing() {
    let mut g = game();
    calm(&mut g);
    for s in &mut g.states {
        s.population = 0.0;
    }
    g.take_control(StateId::EastAsia, Seat(0));
    g.take_control(StateId::SouthAmerica, Seat(0));
    let before = 100.0;
    g.state_mut(StateId::EastAsia).population = before;
    g.seats[0].stockpile.ducats = 40;
    let order = Order::Resettle { state: StateId::SouthAmerica };
    assert_eq!(g.order_cost(Seat(0), &order).ducats, 20, "20 Ducats");
    assert!(g.check_order(Seat(0), &[], &order).is_ok());
    assert!(g.check_order(Seat(0), &[order.clone()], &order).is_err(), "once a turn per Faction");
    g.commit_orders(Seat(0), &[order]);
    g.resolve_unrest();
    assert_eq!(g.seats[0].influence.get(&Place::State(StateId::SouthAmerica)).copied(), Some(5), "+5 Standing on the chosen state");
    hold_temperature(&mut g, 3.0);
    g.climate_phase();
    let arrived = (before - g.state(StateId::EastAsia).population) * 0.5;
    assert!(arrived > 0.0);
    assert!(
        (g.state(StateId::SouthAmerica).population - arrived).abs() < 1e-6,
        "South America took the whole flow: {}",
        g.state(StateId::SouthAmerica).population
    );
    assert_eq!(g.state(StateId::Russia).population, 0.0, "and Asia's own neighbours took none");
}

/// (j) The Unrest card is a flat +3 there, damped by nothing, with no Army damage and no Standing loss.
#[test]
fn j_the_unrest_card_adds_three() {
    let mut g = game();
    calm(&mut g);
    g.take_control(StateId::EastAsia, Seat(0));
    constabulary_in(&mut g, StateId::EastAsia);
    for t in Game::GREEN_TECHS {
        g.research.done.push(t);
    }
    let damage = |g: &Game| g.armies.iter().find(|a| a.standing && a.home == ArmyHome::State(StateId::EastAsia)).unwrap().damage;
    let before = damage(&g);
    g.seats[0].influence.insert(Place::State(StateId::EastAsia), 30);
    g.last_event = Some(DrawnEvent { card: Card::Event(EventId::Unrest), target: EventTarget::State(StateId::EastAsia), scale: 1.0, text: String::new() });
    g.apply_event_now();
    assert_eq!(g.unrest(StateId::EastAsia), 3.0, "a flat three, damped by nothing");
    assert_eq!(damage(&g), before, "no Army damage any more");
    assert_eq!(g.seats[0].influence.get(&Place::State(StateId::EastAsia)).copied(), Some(30), "and no Standing loss");
    // A Heatwave, by contrast, is one of the three Climate cards, so the Techs and the Constabulary damp it.
    g.state_mut(StateId::EastAsia).unrest = 0.0;
    g.last_event = Some(DrawnEvent { card: Card::Event(EventId::Heatwave), target: EventTarget::State(StateId::EastAsia), scale: 1.0, text: String::new() });
    g.apply_event_now();
    assert_eq!(g.unrest(StateId::EastAsia), 0.0, "a Climate card of 1.5, damped by 1.5, raises nothing");
    let mut g = game();
    calm(&mut g);
    g.last_event = Some(DrawnEvent { card: Card::Event(EventId::Wildfire), target: EventTarget::State(StateId::NorthAfrica), scale: 1.0, text: String::new() });
    g.apply_event_now();
    assert_eq!(g.unrest(StateId::NorthAfrica), 1.5, "an undamped Climate card is one and a half");
}

/// (k) Unrest falls by 1 on its own only in a turn nothing raised it.
#[test]
fn k_the_natural_fall_lands_only_in_a_turn_nothing_raised_it() {
    let mut g = game();
    calm(&mut g);
    g.take_control(StateId::NorthAfrica, Seat(0));
    g.state_mut(StateId::NorthAfrica).unrest = 5.0;
    g.resolve_unrest();
    assert_eq!(g.unrest(StateId::NorthAfrica), 3.5, "a quiet turn takes 1.5 off");
    // Ticket #53: the fall lands even in a turn something raised it, by subtraction.
    g.raise_unrest(StateId::NorthAfrica, 1.0, UnrestSource::Plain);
    assert_eq!(g.unrest(StateId::NorthAfrica), 4.5);
    g.resolve_unrest();
    assert_eq!(g.unrest(StateId::NorthAfrica), 3.0, "a rise of one against a fall of 1.5 is a net half off");
    // The one turn it does not fall is the turn the state changed hands.
    g.state_mut(StateId::NorthAfrica).unrest = 5.0;
    g.transfer_control(Place::State(StateId::NorthAfrica), Seat(1), "Influence");
    assert!(g.state(StateId::NorthAfrica).changed_hands, "the transfer marked it");
    g.resolve_unrest();
    assert_eq!(g.unrest(StateId::NorthAfrica), 5.0, "a state that changed hands this turn takes no fall");
    g.resolve_unrest();
    assert_eq!(g.unrest(StateId::NorthAfrica), 3.5, "and the turn after, it falls again");
    g.state_mut(StateId::NorthAfrica).unrest = 0.0;
    g.resolve_unrest();
    assert_eq!(g.unrest(StateId::NorthAfrica), 0.0, "never below zero");
}

/// The AI of ticket #52: Relief where Unrest has taken hold, a Constabulary where it is worse, and
/// Resettle into a calm state of its own.
#[test]
fn the_ai_pays_relief_and_raises_a_constabulary_where_unrest_has_taken_hold() {
    let mut g = game();
    calm(&mut g);
    // A state the seat has just Occupied, restive enough that one more turn would throw it off:
    // Relief takes the opportunity multiplier at 9, a Constabulary the threat multiplier here.
    g.state_mut(StateId::NorthAfrica).control = Control::Occupied { occupier: Seat(1), previous: None, turns: 1 };
    g.seats[1].stockpile.ducats = 200;
    g.seats[1].stockpile.materials = 200;
    // On pace, so the victory gap is not multiplying every producer past everything else.
    g.seats[1].extraction_total = 1000;
    g.state_mut(StateId::NorthAfrica).unrest = 9.0;
    let orders = g.ai_orders(Seat(1));
    assert!(orders.iter().any(|o| matches!(o, Order::Relief { state: StateId::NorthAfrica })), "no Relief: {orders:?}");
    assert!(
        orders.iter().any(|o| matches!(o, Order::BuildFacility { state: StateId::NorthAfrica, kind: FacilityKind::Constabulary })),
        "no Constabulary: {orders:?}"
    );
    // A calm state gets neither.
    let mut g = game();
    calm(&mut g);
    g.take_control(StateId::NorthAfrica, Seat(1));
    g.seats[1].stockpile.ducats = 200;
    g.seats[1].stockpile.materials = 200;
    g.seats[1].extraction_total = 1000;
    let orders = g.ai_orders(Seat(1));
    assert!(!orders.iter().any(|o| matches!(o, Order::Relief { .. })), "Relief in a calm state: {orders:?}");
    assert!(
        !orders.iter().any(|o| matches!(o, Order::BuildFacility { kind: FacilityKind::Constabulary, .. })),
        "a Constabulary in a calm state: {orders:?}"
    );
}

/// Ticket #53: twelve Nation States, each split sharing out its parent's real-world figures rather
/// than inventing more, and every neighbour edge listed on both states.
#[test]
fn twelve_nation_states_share_out_the_eight_they_came_from() {
    let g = game();
    let t = &g.tables;
    assert_eq!(StateId::ALL.len(), 12);
    let card = |s: StateId| t.state(s);
    // Asia's 30 GDP and 7 Influence go to East Asia, South Asia and South-East Asia.
    let asia = [StateId::EastAsia, StateId::SouthAsia, StateId::SouthEastAsia];
    assert_eq!(asia.iter().map(|s| card(*s).gdp).sum::<i64>(), 30, "Asia's GDP share");
    assert_eq!(asia.iter().map(|s| card(*s).influence).sum::<i64>(), 7, "Asia's Influence value");
    // Africa's 3 and 2 split at the Sahara.
    let africa = [StateId::SubSaharanAfrica, StateId::NorthAfrica];
    assert_eq!(africa.iter().map(|s| card(*s).gdp).sum::<i64>(), 3);
    assert_eq!(africa.iter().map(|s| card(*s).influence).sum::<i64>(), 2);
    // North America's 25 and 8 split with Central America and the Caribbean.
    let america = [StateId::NorthAmerica, StateId::CentralAmerica];
    assert_eq!(america.iter().map(|s| card(*s).gdp).sum::<i64>(), 25);
    assert_eq!(america.iter().map(|s| card(*s).influence).sum::<i64>(), 8);
    // The world still holds about 7.9 billion people, as the eight states did.
    let people: f64 = StateId::ALL.iter().map(|s| card(*s).population).sum();
    assert!((people - 78.6).abs() < 0.1, "population {people}");
    // Every edge is listed on both states, and nothing neighbours itself.
    for s in StateId::ALL {
        assert!(!card(s).neighbours.contains(&s), "{s:?} neighbours itself");
        assert!(!card(s).neighbours.is_empty(), "{s:?} is an island with no neighbour");
        for n in &card(s).neighbours {
            assert!(card(*n).neighbours.contains(&s), "{s:?} lists {n:?} but not the other way");
        }
    }
    // Every state's start Facilities fit its slots with a Launch Site on top, and follow its Lean.
    for s in StateId::ALL {
        let c = card(s);
        assert_eq!(c.start_facilities.len() as u32, c.industry_level, "{s:?} starts with as many Facilities as its Industry Level");
        assert!(c.start_facilities.len() as u32 + 1 <= c.size + c.industry_level, "{s:?} has no room for a Launch Site");
        assert!(c.unrest == 0.0, "{s:?} starts calm");
    }
}


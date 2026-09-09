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
        modules: modules.iter().map(|k| Module { kind: *k, online: true }).collect(),
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
    // One turn of real Emissions: about +10 ppm net moves the target by +0.125.
    g.climate.co2 = 430.0;
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
    // Nothing emits, so each phase takes the Sink (6 ppm) off: 506 becomes 500, and the target is 2.2.
    g.climate.co2 = 506.0;
    g.climate_phase();
    assert!((g.climate.co2 - 500.0).abs() < 1e-9);
    assert!((g.climate.temperature - 1.7).abs() < 1e-6, "{}", g.climate.temperature);
    g.climate.co2 = 506.0;
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
    g.climate.co2 = 480.0; // keeps the target above 1.8 so the temperature stays there
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
    g.climate.co2 = 480.0;
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
    assert!(g.seats[0].influence.get(&Place::State(StateId::Africa)).is_none(), "wiped on transfer");
    assert_eq!(g.seats[0].influence[&Place::State(StateId::Europe)], 8);
}

#[test]
fn influence_on_an_owned_colony_pushes_the_rival_back_one_for_one() {
    let mut g = game();
    let c = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Habitat], 4);
    g.seats[1].influence.insert(Place::Colony(c), 30);
    g.seats[1].influenced_this_turn.push(Place::Colony(c));
    g.pending.influence.push((Seat(0), Place::Colony(c), 12));
    g.resolution_phase();
    assert_eq!(g.seats[1].influence[&Place::Colony(c)], 18);
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
fn turn_twelve_scores_the_lower_fraction_of_the_two_parts() {
    let mut g = game();
    g.turn = 12;
    colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Habitat], 6); // presence 0.5, run 0 -> score 0
    g.seats[0].stabilization_run = 3;
    colony(&mut g, Seat(1), BodyId::Moon, &[ModuleKind::Habitat], 3); // presence 0.25
    g.seats[1].extraction_total = 500; // first 1.0 -> score 0.25
    g.end_phase();
    assert!(matches!(g.outcome, Some(Outcome::Win { seat: Seat(0), .. })), "0.5 beats 0.25: {:?}", g.outcome);
}

#[test]
fn nothing_ends_before_turn_twelve_without_a_condition_or_collapse() {
    let mut g = game();
    g.turn = 5;
    g.end_phase();
    assert_eq!(g.outcome, None);
}

// ---------------------------------------------------------------- 13.1 Deck swap rate

#[test]
fn one_calm_card_becomes_a_climate_card_per_full_fifth_of_a_degree() {
    let mut g = game();
    for s in &mut g.states {
        s.population = 0.0;
        s.industry_level = 0;
    }
    let calm = g.deck.calm_left();
    let climate = g.deck.climate_cards_left();
    g.climate.temperature = 1.65; // two full 0.2 steps above 1.2
    g.climate.co2 = 460.0; // target 1.7 keeps it there
    g.climate_phase();
    assert_eq!(g.deck.calm_left(), calm - 2);
    assert_eq!(g.deck.climate_cards_left(), climate + 2);
    assert_eq!(g.deck.cards.len(), 20, "a swap replaces, it does not add");
    g.climate_phase();
    assert_eq!(g.deck.calm_left(), calm - 2, "steps already counted do not swap again");
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
    Stockpile { materials: after.materials - before.materials, fuel: after.fuel - before.fuel, energy: after.energy - 1000 }
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
    assert_eq!(g.seats[0].stockpile, Stockpile { materials: 80, fuel: 20, energy: 20 });
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

// ---------------------------------------------------------------- the whole loop holds together

#[test]
fn an_ai_versus_ai_game_runs_to_an_outcome() {
    let r = dying_earth_engine::sim::run(tables(), 3, [FactionKind::Custodians, FactionKind::Prospectors]);
    assert!(r.outcome.is_some());
    assert!(r.last_turn <= 12);
}

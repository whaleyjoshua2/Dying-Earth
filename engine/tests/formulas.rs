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
    Facility::new(kind)
}

/// Ticket #57: stand the game on the turn the Mars launch window falls on, where a crossing costs
/// the Hohmann flight and the card's Fuel. Off the window both rise, so a test about anything else
/// -- Efficient Transit, the Arkwrights' Fuel multiplier -- reads the card figures here.
fn at_window(g: &mut Game) {
    g.turn = g.next_window_turn(1);
}

/// The same for the flight home, which wants a different phase angle and so falls on a different
/// turn: the turn in the game's span whose return offset is smallest.
fn at_return_window(g: &mut Game) {
    g.turn = (1..=g.tables.victory.turns)
        .min_by(|a, b| g.return_window_offset(*a).abs().partial_cmp(&g.return_window_offset(*b).abs()).unwrap())
        .unwrap();
}

/// Ticket #57: every Colony Slot draws its own four yields at the start, so a test that reasons
/// about a Faction or a Tech would otherwise be reading a random draw. This helper founds into the
/// first free slot and pins that slot's yields back to its Body's card figures; the tests that are
/// ABOUT the per-slot draw read the drawn figures instead.
fn colony(g: &mut Game, seat: Seat, body: BodyId, modules: &[ModuleKind], colonists: u32) -> ColonyId {
    let id = ColonyId(g.fresh_id());
    let slot = g.free_slots_on(body)[0];
    g.slot_yields.insert((body, slot), SlotYields::of_body(g.tables.body(body)));
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
    // Ticket #55: and no Break fires on the way, so this reads the lag alone.
    breaks_held(&mut g);
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

// Ticket #56 replaced the highest-upkeep-first rule with an oldest-first one that reaches only the
// coastal slots: it is pinned by `d_the_sea_takes_coastal_slots_only_oldest_first_and_then_nothing`.

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
    // Ticket #54 (g): the Custodians' multiplier is 1.25, not 1.3. They hold East Asia (4):
    // (10 + 4) x 1.25 = 17.5 -> 17. Europe is still 5, and the Prospectors are still x1.0.
    assert_eq!(g.tables.faction(FactionKind::Custodians).influence_multiplier, 1.25);
    assert_eq!(g.influence_allotment(Seat(0)), 17);
    assert_eq!(g.influence_allotment(Seat(1)), 15);
    g.state_mut(StateId::NorthAmerica).control = Control::Controlled(Seat(1));
    assert_eq!(g.influence_allotment(Seat(1)), 22, "North America adds 7");
    // Raising East Asia's Industry Level adds one to its value.
    g.state_mut(StateId::EastAsia).industry_level += 1;
    assert_eq!(g.state_influence_value(StateId::EastAsia), 5);
    assert_eq!(g.influence_allotment(Seat(0)), 18, "(10 + 5) x 1.25 = 18.75");
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

/// Ticket #54 (h) adjusted this test: Restoration and its Ducat price are retired, so what the
/// window used to buy in Restoration steps it now buys in Leapfrogs, and the repair rate stands.
#[test]
fn ducats_pay_for_a_leapfrog_and_repairs_at_the_table_rates() {
    let mut g = game();
    // Version 0.04 (ticket #41): two Ducats for one of the thing bought; a repair point is 5
    // Materials, so 10 Ducats. Ticket #54: a Leapfrog is 50 Ducats.
    g.seats[0].stockpile.ducats = 60;
    g.seats[0].stockpile.energy = 0;
    let r = Order::Leapfrog { state: StateId::EastAsia };
    assert_eq!(g.order_cost(Seat(0), &r).ducats, 50);
    g.commit_orders(Seat(0), &[r]);
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
    // Reach. Ticket #57 replaced the fixed card turns for a crossing between the Earth system and
    // the Mars system with the real flight: at the window it is the Hohmann 259 days, nine turns,
    // for the card's Fuel. The hops inside a system are untouched by it.
    let mut g = g;
    at_window(&mut g);
    assert_eq!(g.transit_cost(BodyId::Earth, BodyId::Phobos), (9, 24));
    assert_eq!(g.transit_cost(BodyId::Moon, BodyId::Deimos), (9, 24));
    assert_eq!(g.transit_cost(BodyId::Mars, BodyId::Phobos), (1, 2));
    assert_eq!(g.transit_cost(BodyId::Deimos, BodyId::Mars), (1, 2));
    assert_eq!(g.transit_cost(BodyId::Phobos, BodyId::Deimos), (1, 1));
    assert_eq!(g.transit_cost(BodyId::Earth, BodyId::Mars), (9, 20));
    assert_eq!(g.transit_cost(BodyId::Moon, BodyId::Mars), (9, 20));
    assert_eq!(g.transit_cost(BodyId::Earth, BodyId::Moon), (1, 6));
    // The flight home reads the same cards, at its own window, which is not the same turn: the
    // moons' 24 against Mars's 20, both stretched by however far off that window the best turn in
    // the game's span falls. The window arithmetic itself is pinned by the ticket #57 tests.
    at_return_window(&mut g);
    let (moon_turns, from_deimos) = g.transit_cost(BodyId::Deimos, BodyId::Earth);
    let (mars_turns, from_mars) = g.transit_cost(BodyId::Mars, BodyId::Moon);
    assert_eq!(moon_turns, mars_turns, "one flight home, whichever rock it leaves from");
    assert!(from_deimos > from_mars, "Deimos reads the dearer card: {from_deimos} Fuel against {from_mars}");
    // Their Colonists are off Earth.
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
    // Ticket #56 re-cut them: abundant ore and Fuel under the ice.
    assert_eq!((earth.mine_yield, earth.generator_yield, earth.refinery_yield, earth.habitat_yield), (1.75, 0.75, 2.0, 1.0));
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
    // Ticket #54: the Custodians' multiplier is 1.25. In East Asia: (10 + 4) x 1.25 = 17. Two
    // Embassies (they stack) add 4: (10 + 4 + 4) x 1.25 = 22.
    assert_eq!(g.influence_allotment(Seat(0)), 17);
    g.state_mut(StateId::EastAsia).facilities.push(facility(FacilityKind::Embassy));
    g.state_mut(StateId::EastAsia).facilities.push(facility(FacilityKind::Embassy));
    assert_eq!(g.building_allotment(Seat(0)), 4);
    assert_eq!(g.influence_allotment(Seat(0)), 22);
    // A Relay in a Colony adds 1 more.
    let c = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Habitat, ModuleKind::Relay], 4);
    assert_eq!(g.influence_allotment(Seat(0)), 23, "(10 + 4 + 5) x 1.25 = 23.75");
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
fn the_methane_burst_adds_scaled_emissions_next_turn_that_do_not_count_against_stabilization() {
    let mut g = game();
    g.climate.temperature = 2.2; // scale 1.5
    drawn(&mut g, EventId::MethaneBurst, EventTarget::Everyone);
    g.apply_event_now();
    let e = g.emissions_now();
    assert!((e.cards - 3.75).abs() < 1e-9, "2.5 x 1.5: {}", e.cards);
    assert!((e.total() - e.counted() - 3.75).abs() < 1e-9);
    with_tech(&mut g, TechId::GreenConsensus);
    g.climate.card_emissions_next = 0.0;
    drawn(&mut g, EventId::MethaneBurst, EventTarget::Everyone);
    g.apply_event_now();
    assert!((g.emissions_now().cards - 1.875).abs() < 1e-9, "halved");
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
    // Ticket #57: on the window a crossing pays the card's Fuel, and Efficient Transit comes after.
    at_window(&mut g);
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
    // Transit Fuel too: three quarters, then Efficient Transit on top of that. Ticket #57: on the
    // window the crossing pays the card's Fuel, and both multipliers apply after the window factor.
    at_window(&mut g);
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
    assert!(g.report.lines.iter().any(|l| l.text.contains("Archivists are funding the Archive")), "{:?}", g.report.lines);
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
    g.commit_orders(Seat(3), std::slice::from_ref(&order));
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
    assert!(g.report.lines.iter().any(|l| l.text.contains("Archive at") && l.text.contains("destroyed")), "{:?}", g.report.lines);
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
    // Ticket #55: and every Break already fired, so a Climate-phase test at a high Temperature sees
    // only what it set up. `breaks_ahead` puts them back for the tests that are about them.
    breaks_held(g);
}

/// Ticket #55: every Break already fired, so none of them fires in the phase under test.
fn breaks_held(g: &mut Game) {
    g.climate.breaks_fired = vec![true; g.tables.climate.breaks.len()];
}

/// Ticket #55: every Break still ahead of the game, whatever `calm` did.
fn breaks_ahead(g: &mut Game) {
    g.climate.breaks_fired = vec![false; g.tables.climate.breaks.len()];
}

/// The index of a Break in `climate.toml`'s list, by its id.
fn break_at(g: &Game, id: &str) -> usize {
    g.tables.climate.breaks.iter().position(|b| b.id == id).unwrap_or_else(|| panic!("no [[break]] {id} in climate.toml"))
}

/// A board with nothing on it that emits: every state empty of people, industry and buildings, so a
/// Climate-phase test reads only the Break it fired.
fn bare_world(g: &mut Game) {
    for s in &mut g.states {
        s.population = 0.0;
        s.industry_level = 0;
        s.facilities.clear();
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
    assert!(g.report.lines.iter().any(|l| l.text.contains("Unrest there rose")), "the Report says so: {:?}", g.report.lines);
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
        g.report.lines.iter().any(|l| l.text.contains("left East Asia for") && l.text.contains("Russia")),
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
    g.state_mut(StateId::NorthAfrica).queue.push(Build { item: BuildItem::Facility(FacilityKind::Bank), seat: Seat(0), coastal: false, due_turn: 99 });
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
    assert!(g.report.lines.iter().any(|l| l.text.contains("threw off")), "a Report line names it: {:?}", g.report.lines);
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
    assert!(g.check_order(Seat(0), std::slice::from_ref(&order), &order).is_err(), "once a turn per Faction");
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
        // Ticket #56: Size + Industry Level + base_slots, and the coastal row must leave one inland.
        assert!((c.start_facilities.len() as u32) < c.size + c.industry_level + g.tables.base_slots, "{s:?} has no room for a Launch Site");
        assert!(c.unrest == 0.0, "{s:?} starts calm");
    }
}


// ================================================================ #53 Blame and neutral development

/// A board with nothing emitting but what the test puts on it: every state's people gone, every
/// Facility stripped, the sea already done with, and every state calm and neutral.
fn quiet_world(g: &mut Game) {
    calm(g);
    for s in &mut g.states {
        s.population = 0.0;
        s.facilities.clear();
        s.industry_level = 0;
        s.control = Control::Neutral;
        s.neutral_since = Some(1);
    }
}

/// (a) A Climate phase adds a controlled state's attributable Emissions to its controller's emitted
/// total, and a neutral state's to nobody's.
#[test]
fn a_blame_follows_control_and_a_neutral_state_belongs_to_nobody() {
    let mut g = game();
    quiet_world(&mut g);
    // North Africa is the Prospectors': baseline 0.4 at Industry Level 2, with 3.0 hundred million
    // people. Sub-Saharan Africa stays neutral carrying exactly the same weight.
    g.take_control(StateId::NorthAfrica, Seat(1));
    g.state_mut(StateId::NorthAfrica).industry_level = 2;
    g.state_mut(StateId::NorthAfrica).population = 3.0;
    g.state_mut(StateId::SubSaharanAfrica).industry_level = 2;
    g.state_mut(StateId::SubSaharanAfrica).population = 3.0;
    let card = g.tables.state(StateId::NorthAfrica).baseline_emissions;
    let mult = g.tables.faction(FactionKind::Prospectors).emissions_multiplier;
    // Ticket #54: the per-person figure follows the state's Industry Level, which is 2 here.
    let per_hundred = g.population_coefficient(StateId::NorthAfrica);
    let expected = card * 2.0 * mult + per_hundred * 3.0 * mult;
    g.climate_phase();
    assert!(
        (g.seats[1].blame_emitted - expected).abs() < 1e-9,
        "the Prospectors wear North Africa's industry and its people: emitted {}, expected {expected}",
        g.seats[1].blame_emitted
    );
    for seat in [Seat(0), Seat(2), Seat(3)] {
        assert_eq!(g.seats[seat.index()].blame_emitted, 0.0, "a neutral state's Emissions are nobody's");
    }
    assert_eq!(g.blame(Seat(1)), g.seats[1].blame_emitted, "nothing removed, so Blame is what was emitted");
}

/// (b) A Scrubber's removal is added to its controller's removed total, and Blame floors at zero: a
/// Faction that takes back more than it put out shows Blame 0 and a credit. Ticket #54 rewrote this
/// test, which ran on Restoration until the Scrubber replaced it.
#[test]
fn b_a_scrubbers_removal_is_credited_and_blame_floors_at_zero() {
    let mut g = game();
    quiet_world(&mut g);
    g.take_control(StateId::EastAsia, Seat(0));
    g.state_mut(StateId::EastAsia).industry_level = 1;
    for _ in 0..4 {
        g.state_mut(StateId::EastAsia).facilities.push(facility(FacilityKind::Scrubber));
    }
    let removed = g.tables.facility(FacilityKind::Scrubber).sink_per_turn * 4.0;
    assert_eq!(g.scrubber_removal_by_seat()[0], removed, "the Custodians' Scrubbers are theirs, not the table's");
    g.climate_phase();
    assert_eq!(g.seats[0].blame_removed, removed, "the Climate phase banks what was removed");
    assert!(g.seats[0].blame_emitted > 0.0, "East Asia's industry is still theirs");
    assert_eq!(g.blame(Seat(0)), 0.0, "removing more than you emitted floors Blame at zero");
    assert!(
        (g.blame_credit(Seat(0)) - (removed - g.seats[0].blame_emitted)).abs() < 1e-9,
        "and shows as a credit of {:.1} ppm",
        g.blame_credit(Seat(0))
    );
}

/// (c) The share and the multiplier: an even quarter each is x1.0, half the table x1.25, the whole
/// of it x1.5 (the cap), and none of it x1.0 (the floor).
#[test]
fn c_blame_share_sets_the_threshold_multiplier() {
    let mut g = game();
    for s in &mut g.seats {
        s.blame_emitted = 40.0;
        s.blame_removed = 0.0;
    }
    for seat in Seat::ALL {
        assert_eq!(g.blame_share(seat), 0.25, "four Factions at equal Blame each hold a quarter");
        assert_eq!(g.blame_threshold_multiplier(seat), 1.0, "a fair share costs nothing");
    }
    // One Faction with half the table's Blame: 60 against 20 each.
    g.seats[1].blame_emitted = 60.0;
    for i in [0usize, 2, 3] {
        g.seats[i].blame_emitted = 20.0;
    }
    assert_eq!(g.blame_share(Seat(1)), 0.5, "60 of 120");
    assert_eq!(g.blame_threshold_multiplier(Seat(1)), 1.25, "1 + (0.5 - 0.25)");
    // The whole of it: the cap.
    for i in [0usize, 2, 3] {
        g.seats[i].blame_emitted = 0.0;
    }
    assert_eq!(g.blame_share(Seat(1)), 1.0);
    assert_eq!(g.blame_threshold_multiplier(Seat(1)), 1.5, "1.75 is capped at x1.5");
    assert_eq!(g.blame_threshold_multiplier(Seat(0)), 1.0, "a Faction with no Blame is floored at x1.0");
    // Nobody has any: no share, no multiplier, and no division by zero.
    for s in &mut g.seats {
        s.blame_emitted = 0.0;
    }
    assert_eq!(g.blame_total(), 0.0);
    assert_eq!(g.blame_share(Seat(1)), 0.0, "no Blame at the table is no share");
    assert_eq!(g.blame_threshold_multiplier(Seat(1)), 1.0);
}

/// (d) The per-seat threshold applies to a neutral state and to a rival's state, and to nothing
/// else: not the seat's own state, not a Colony, not a Space Station.
#[test]
fn d_the_raised_threshold_lands_only_on_states_the_seat_does_not_hold() {
    let mut g = game();
    // Seat 1 wears the whole table's Blame: x1.5.
    for s in &mut g.seats {
        s.blame_emitted = 0.0;
    }
    g.seats[1].blame_emitted = 100.0;
    assert_eq!(g.blame_threshold_multiplier(Seat(1)), 1.5);
    let neutral = Place::State(StateId::NorthAfrica);
    let plain = g.influence_threshold(neutral);
    assert_eq!(plain, 40, "North Africa is Size 2: 20 + 10 x 2");
    assert_eq!(g.influence_threshold_for(Seat(1), neutral), 60, "x1.5 on a neutral state");
    assert_eq!(g.influence_threshold_for(Seat(0), neutral), 40, "and nothing on a Faction with no Blame");
    // Another Faction's state.
    g.take_control(StateId::NorthAfrica, Seat(0));
    assert_eq!(g.influence_threshold_for(Seat(1), neutral), 60, "x1.5 on another Faction's state");
    // The seat's own state.
    g.take_control(StateId::SouthAmerica, Seat(1));
    let own = Place::State(StateId::SouthAmerica);
    assert_eq!(g.influence_threshold_for(Seat(1), own), g.influence_threshold(own), "never on a state it holds");
    // A Colony and a Space Station.
    let col = colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Habitat], 3);
    let station = station_of(&g, Seat(0), BodyId::Earth).expect("the ISS");
    for place in [Place::Colony(col), Place::Colony(station)] {
        assert_eq!(
            g.influence_threshold_for(Seat(1), place),
            g.influence_threshold(place),
            "Blame never touches a Colony or a Space Station: {place:?}"
        );
    }
}

/// (e) An Influence transfer reads the challenger's own threshold: a Faction sitting under its
/// raised threshold does not take the place, and the seat beside it, with no Blame, does.
#[test]
fn e_a_transfer_uses_the_challengers_own_threshold() {
    let mut g = game();
    for s in &mut g.seats {
        s.blame_emitted = 0.0;
    }
    g.seats[1].blame_emitted = 100.0;
    let target = Place::State(StateId::NorthAfrica);
    assert_eq!(g.influence_threshold(target), 40);
    assert_eq!(g.influence_threshold_for(Seat(1), target), 60);
    // 45 clears the plain threshold of 40 and not the Prospectors' own 60.
    g.pending.influence.push((Seat(1), target, 45));
    g.resolution_phase();
    assert_eq!(g.state(StateId::NorthAfrica).control, Control::Neutral, "45 does not reach a Blamed Faction's 60");
    assert_eq!(g.seats[1].influence[&target], 45, "the Standing is there; it is the threshold that moved");
    // The Arkwrights, with no Blame at all, take it on 40.
    g.pending.influence.push((Seat(2), target, 40));
    g.resolution_phase();
    assert_eq!(g.state(StateId::NorthAfrica).control, Control::Controlled(Seat(2)), "40 is enough for a Faction with no Blame");
}

/// (f) A neutral state develops on its sixth neutral turn, and does not at Industry Level 4, at
/// +2.5 C, at Unrest 7, or while a Faction holds it.
#[test]
fn f_a_neutral_state_raises_its_industry_level_every_sixth_turn() {
    // North Africa is neutral when the game opens; Sub-Saharan Africa is one of the four start states.
    let sid = StateId::NorthAfrica;
    // The plain case: six neutral turns, one level, one idle Facility woken.
    let mut g = game();
    calm(&mut g);
    hold_temperature(&mut g, 1.2);
    g.state_mut(sid).facilities.push(Facility { online: false, ..Facility::new(FacilityKind::Factory) });
    let start = g.state(sid).industry_level;
    let before = g.emissions_now();
    let n = g.tables.development.turns;
    g.state_mut(sid).neutral_since = Some(1);
    for turn in 1..=n {
        g.turn = turn;
        g.neutral_development();
        if turn < n {
            assert_eq!(g.state(sid).industry_level, start, "nothing on neutral turn {turn}");
        }
    }
    assert_eq!(g.state(sid).industry_level, start + 1, "the {n}th neutral turn raises the Industry Level");
    assert!(g.state(sid).facilities.iter().all(|f| f.online), "and brings the idle Factory online");
    // The woken Factory is run by the state itself: while nobody directs the state it emits at
    // x1.0 (a Facility nobody directs otherwise emits nothing), to nobody's Blame.
    assert!(g.state(sid).facilities.iter().any(|f| f.kind == FacilityKind::Factory && f.self_run), "the state runs the Factory itself");
    let b = g.emissions_now();
    assert!(b.factories > before.factories, "a self-run Factory in a neutral state emits: {b:?}");
    assert_eq!(b.by_seat, before.by_seat, "and it is nobody's Blame");
    assert!(
        g.report.lines.iter().any(|l| l.text.contains("raised its Industry Level to") && l.text.contains("Factory")),
        "the Report names it: {:?}",
        g.report.lines
    );
    // The clock starts again, so nothing happens until the period has run a second time.
    for turn in n + 1..2 * n {
        g.turn = turn;
        g.neutral_development();
    }
    assert_eq!(g.state(sid).industry_level, start + 1, "and not again until turn {}", 2 * n);
    g.turn = 2 * n;
    g.neutral_development();
    assert_eq!(g.state(sid).industry_level, start + 2, "every {n} turns");

    // At the ceiling.
    let mut g = game();
    calm(&mut g);
    hold_temperature(&mut g, 1.2);
    let ceiling = g.tables.development.max_level;
    g.state_mut(sid).industry_level = ceiling;
    g.state_mut(sid).neutral_since = Some(1);
    g.turn = g.tables.development.turns;
    g.neutral_development();
    assert_eq!(g.state(sid).industry_level, ceiling, "Industry Level 4 develops no further");

    // Too hot.
    let mut g = game();
    calm(&mut g);
    let stops = g.tables.development.stops_at_temperature;
    hold_temperature(&mut g, stops);
    g.state_mut(sid).neutral_since = Some(1);
    g.turn = g.tables.development.turns;
    g.neutral_development();
    assert_eq!(g.state(sid).industry_level, start, "nothing develops at +2.5 C");

    // Too restive.
    let mut g = game();
    calm(&mut g);
    hold_temperature(&mut g, 1.2);
    g.state_mut(sid).unrest = g.tables.unrest.no_development_at;
    g.state_mut(sid).neutral_since = Some(1);
    g.turn = g.tables.development.turns;
    g.neutral_development();
    assert_eq!(g.state(sid).industry_level, start, "a state at Unrest 7 does not develop");

    // Held.
    let mut g = game();
    calm(&mut g);
    hold_temperature(&mut g, 1.2);
    g.take_control(sid, Seat(1));
    g.state_mut(sid).neutral_since = Some(1);
    g.turn = g.tables.development.turns;
    g.neutral_development();
    assert_eq!(g.state(sid).industry_level, start, "a controlled state is developed by its controller, not by itself");
}

/// (g) The clock is per state: a state taken and then freed counts six fresh turns.
#[test]
fn g_the_neutrality_clock_restarts_when_a_state_goes_neutral_again() {
    let sid = StateId::NorthAfrica;
    let mut g = game();
    calm(&mut g);
    hold_temperature(&mut g, 1.2);
    assert!(g.state(sid).neutral_since.is_some(), "every state is neutral when the game opens (clocks staggered by the seed)");
    g.turn = 3;
    g.take_control(sid, Seat(1));
    assert_eq!(g.state(sid).neutral_since, None, "a state a Faction holds has no clock");
    // Thrown off on turn 5: the count starts again from turn 6, so nothing until the period has
    // run from there, and the raise on turn 5 + period.
    g.turn = 5;
    g.state_mut(sid).unrest = g.tables.unrest.throw_off_threshold;
    // It changed hands this turn, so the natural fall is withheld and 10 is still 10 at the throw-off.
    g.state_mut(sid).changed_hands = true;
    g.resolve_unrest();
    assert_eq!(g.state(sid).control, Control::Neutral, "it threw its controller off");
    assert_eq!(g.state(sid).neutral_since, Some(6), "the clock starts on the turn after the change");
    let start = g.state(sid).industry_level;
    g.state_mut(sid).unrest = 0.0;
    let n = g.tables.development.turns;
    for turn in 6..5 + n {
        g.turn = turn;
        g.neutral_development();
    }
    assert_eq!(g.state(sid).industry_level, start, "the old clock is gone: nothing by turn {}", 4 + n);
    g.turn = 5 + n;
    g.neutral_development();
    assert_eq!(g.state(sid).industry_level, start + 1, "six fresh turns from turn 6");
}

/// Ticket #53: the states that are neutral when the game opens do not all develop on the same
/// turn; their clocks are staggered by the seed, so the first developments spread over several turns.
#[test]
fn the_opening_neutral_states_do_not_all_develop_on_the_same_turn() {
    let g = game();
    let clocks: Vec<u32> = StateId::ALL.into_iter().filter(|s| g.state(*s).control == Control::Neutral).filter_map(|s| g.state(s).neutral_since).collect();
    assert!(clocks.len() >= 6, "most states are neutral at the start: {clocks:?}");
    let distinct: std::collections::BTreeSet<u32> = clocks.iter().copied().collect();
    assert!(distinct.len() >= 3, "the clocks are staggered, not all {:?}", clocks);
    let turns = g.tables.development.turns;
    assert!(clocks.iter().all(|c| *c >= 1 && *c < 1 + turns), "each clock starts within the first development period: {clocks:?}");
    // The same seed gives the same stagger.
    let h = game();
    let again: Vec<u32> = StateId::ALL.into_iter().filter_map(|s| h.state(s).neutral_since).collect();
    let first: Vec<u32> = StateId::ALL.into_iter().filter_map(|s| g.state(s).neutral_since).collect();
    assert_eq!(first, again, "seed-stable");
}


// ================================================================ #54 Emissions you can lower

/// The position of the first Facility of this kind in the state's list.
fn facility_at(g: &Game, sid: StateId, kind: FacilityKind) -> usize {
    g.state(sid).facilities.iter().position(|f| f.kind == kind).expect("that Facility stands there")
}

/// Order one Mothball, Restart or Decommission and let this turn's Resolution land it.
fn change_now(g: &mut Game, seat: Seat, b: BuildingRef, what: BuildingChange) {
    let o = Order::Change { building: b, what };
    g.check_order(seat, &[], &o).unwrap_or_else(|e| panic!("{what:?} refused: {e}"));
    g.commit_orders(seat, &[o]);
    g.resolution_phase();
}

/// (a) A mothballed Facility makes nothing, pays no Energy upkeep, emits nothing and keeps its
/// slot; a Restart costs 5 Materials and a turn; and a mothballed Launch Site lifts nobody.
#[test]
fn a_a_mothballed_facility_makes_nothing_costs_nothing_and_keeps_its_slot() {
    let mut g = game();
    calm(&mut g);
    let sid = StateId::EastAsia;
    g.state_mut(sid).facilities.push(facility(FacilityKind::Factory));
    let idx = facility_at(&g, sid, FacilityKind::Factory);
    let slots = g.slots_used(sid);
    let emissions_before = g.emissions_now().factories;
    assert!(emissions_before > 0.0, "the Factory emits while it works");

    // What it makes and what it costs to run, before and after.
    g.seats[0].stockpile.energy = 20;
    let before = g.seats[0].stockpile;
    g.income_phase();
    let made = g.seats[0].stockpile.materials - before.materials;
    let spent = 20 - g.seats[0].stockpile.energy;
    assert!(made > 0, "a working Factory makes Materials");

    let o = Order::Change { building: BuildingRef::Facility(sid, idx), what: BuildingChange::Mothball };
    assert_eq!(g.order_cost(Seat(0), &o), Cost::default(), "a Mothball is free");
    change_now(&mut g, Seat(0), BuildingRef::Facility(sid, idx), BuildingChange::Mothball);
    assert!(g.state(sid).facilities[idx].mothballed, "it is mothballed at the Resolution");
    assert_eq!(g.slots_used(sid), slots, "a mothballed Facility keeps its slot");
    assert_eq!(g.emissions_now().factories, emissions_before - g.facility_yield(Seat(0), sid, FacilityKind::Factory).emissions, "it emits nothing");

    g.seats[0].stockpile.energy = 20;
    let before = g.seats[0].stockpile;
    g.income_phase();
    assert_eq!(g.seats[0].stockpile.materials - before.materials, 0, "a mothballed Factory makes nothing");
    let spent_now = 20 - g.seats[0].stockpile.energy;
    assert_eq!(spent_now, spent - g.tables.facility(FacilityKind::Factory).energy_upkeep, "it pays no Energy upkeep");

    // A Restart: 5 Materials and a turn.
    let r = Order::Change { building: BuildingRef::Facility(sid, idx), what: BuildingChange::Restart };
    assert_eq!(g.order_cost(Seat(0), &r).materials, 5, "a Restart is 5 Materials");
    g.seats[0].stockpile.materials = 50;
    g.commit_orders(Seat(0), &[r]);
    assert_eq!(g.seats[0].stockpile.materials, 45, "paid at once");
    assert!(g.state(sid).facilities[idx].mothballed, "still mothballed until its turn lands");
    g.resolution_phase();
    assert!(!g.state(sid).facilities[idx].mothballed, "one turn, and it works again");

    // A mothballed Launch Site lifts nobody.
    let ship = a_colony_ship(&mut g, Seat(0), BodyId::Earth);
    let lift = Order::Load { ship, colonists: 1, from: LoadSource::State(sid), army: None };
    assert!(g.check_order(Seat(0), &[], &lift).is_ok(), "a working Launch Site lifts");
    let ls = facility_at(&g, sid, FacilityKind::LaunchSite);
    change_now(&mut g, Seat(0), BuildingRef::Facility(sid, ls), BuildingChange::Mothball);
    assert!(g.check_order(Seat(0), &[], &lift).is_err(), "a mothballed Launch Site lifts nobody");
}

/// (b) A Decommission refunds half the building's Materials rounded down, frees its slot, and adds
/// 2 Unrest to a Nation State (nothing to a Colony).
#[test]
fn b_decommission_refunds_half_frees_the_slot_and_adds_two_unrest() {
    let mut g = game();
    calm(&mut g);
    let sid = StateId::EastAsia;
    g.state_mut(sid).facilities.push(facility(FacilityKind::Factory));
    let idx = facility_at(&g, sid, FacilityKind::Factory);
    let slots = g.slots_used(sid);
    g.seats[0].stockpile.materials = 0;
    g.state_mut(sid).unrest = 3.0;
    g.state_mut(sid).unrest_reported = 3.0;
    let o = Order::Change { building: BuildingRef::Facility(sid, idx), what: BuildingChange::Decommission };
    assert_eq!(g.order_cost(Seat(0), &o), Cost::default(), "a Decommission is paid in the refund");
    g.commit_orders(Seat(0), &[o]);
    assert!(g.state(sid).facilities.iter().any(|f| f.kind == FacilityKind::Factory), "it takes a turn");
    g.resolution_phase();
    assert!(!g.state(sid).facilities.iter().any(|f| f.kind == FacilityKind::Factory), "and then it is gone");
    assert_eq!(g.slots_used(sid), slots - 1, "its slot is free");
    assert_eq!(g.seats[0].stockpile.materials, g.tables.facility(FacilityKind::Factory).materials / 2, "half its Materials, rounded down");
    // The rise is 2; the turn's own fall of 1.5 comes off it in the same Resolution.
    assert_eq!(g.tables.unrest.per_decommission, 2.0);
    let expected = 3.0 + 2.0 - g.tables.unrest.natural_fall;
    assert!((g.unrest(sid) - expected).abs() < 1e-9, "Unrest {} where {expected} was wanted", g.unrest(sid));

    // A Module in a Colony: the refund and the freeing, and no Unrest anywhere.
    let cid = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Mine], 0);
    g.seats[0].stockpile.materials = 0;
    change_now(&mut g, Seat(0), BuildingRef::Module(cid, 0), BuildingChange::Decommission);
    assert!(g.colony(cid).unwrap().modules.is_empty(), "the Mine is gone");
    assert_eq!(g.seats[0].stockpile.materials, g.tables.module(ModuleKind::Mine).materials / 2);
}

/// (c) Population Emissions are `base + per_level x Industry Level` per hundred million, and the
/// world's opening total is within a tenth of the old flat 0.1.
#[test]
fn c_population_emissions_follow_the_industry_level() {
    let g = game();
    let c = &g.tables.climate;
    assert_eq!(c.population_emissions_base, 0.04);
    assert_eq!(c.population_emissions_per_level, 0.03);
    for sid in StateId::ALL {
        let want = c.population_emissions_base + c.population_emissions_per_level * g.state(sid).industry_level as f64;
        assert!((g.population_coefficient(sid) - want).abs() < 1e-9, "{sid:?}: {} where {want} was wanted", g.population_coefficient(sid));
    }
    // Sub-Saharan Africa at Industry Level 1 emits 0.07; East Asia at 3 emits 0.13.
    assert!((g.population_coefficient(StateId::SubSaharanAfrica) - 0.07).abs() < 1e-9);
    assert!((g.population_coefficient(StateId::EastAsia) - 0.13).abs() < 1e-9);
    let new: f64 = StateId::ALL.iter().map(|s| g.population_coefficient(*s) * g.state(*s).population).sum();
    let old: f64 = StateId::ALL.iter().map(|s| 0.1 * g.state(*s).population).sum();
    assert!((new - old).abs() / old < 0.10, "the world's people emit {new:.2} where the flat 0.1 gave {old:.2}");
    // And the Climate phase reads it: one more Industry Level in East Asia is 0.03 x 16.4 more.
    let mut g = g;
    let before = g.emissions_now().population;
    let mult = g.tables.faction(FactionKind::Custodians).emissions_multiplier;
    g.state_mut(StateId::EastAsia).industry_level += 1;
    let rise = g.emissions_now().population - before;
    assert!((rise - 0.03 * 16.4 * mult).abs() < 1e-9, "the population line rose {rise:.3}");
}

/// (d) Leapfrog is the Custodians' alone, costs 50 Ducats, takes one level's worth off the state's
/// per-person coefficient, and never takes it below the base.
#[test]
fn d_leapfrog_is_custodian_only_and_never_goes_below_the_base() {
    let mut g = game();
    let sid = StateId::EastAsia;
    let base = g.tables.climate.population_emissions_base;
    let per = g.tables.climate.population_emissions_per_level;
    g.seats[0].stockpile.ducats = 500;
    g.seats[1].stockpile.ducats = 500;
    // The Prospectors have no Leapfrog, on their own state or anyone's.
    g.take_control(StateId::SouthAmerica, Seat(1));
    assert!(g.check_order(Seat(1), &[], &Order::Leapfrog { state: StateId::SouthAmerica }).is_err(), "only the Custodians Leapfrog");
    // And the Custodians need to control the state.
    assert!(g.check_order(Seat(0), &[], &Order::Leapfrog { state: StateId::SouthAmerica }).is_err(), "and only on a state they control");
    let o = Order::Leapfrog { state: sid };
    assert_eq!(g.order_cost(Seat(0), &o).ducats, 50, "50 Ducats a Leapfrog");
    let before = g.population_coefficient(sid);
    g.commit_orders(Seat(0), std::slice::from_ref(&o));
    assert_eq!(g.seats[0].stockpile.ducats, 450);
    assert!((g.population_coefficient(sid) - (before - per)).abs() < 1e-9, "one level's worth off");
    assert_eq!(g.leapfrogs(sid), 1, "Leapfrogged once");
    // East Asia at Industry Level 3 starts at 0.13, so three Leapfrogs reach the base and a fourth
    // is refused rather than taking 50 Ducats for nothing.
    g.commit_orders(Seat(0), std::slice::from_ref(&o));
    g.commit_orders(Seat(0), std::slice::from_ref(&o));
    assert!((g.population_coefficient(sid) - base).abs() < 1e-9, "the base, and no lower");
    assert!(g.check_order(Seat(0), &[], &o).is_err(), "a fourth Leapfrog buys nothing and is refused");
}

/// (e) A Scrubber takes no build slot, adds 3.0 ppm to the Sink while it is online, is capped by
/// its state's population, is destroyed when the state changes hands, counts as removal for its
/// controller's Blame, and takes 1 off its state's Unrest a turn.
#[test]
fn e_a_scrubber_enlarges_the_sink_and_is_capped_destroyed_and_calming() {
    let mut g = game();
    calm(&mut g);
    let sid = StateId::EastAsia;
    let card = g.tables.facility(FacilityKind::Scrubber);
    assert_eq!((card.materials, card.build_turns, card.energy_upkeep, card.emissions), (30, 2, 4, 0.0));
    assert!(card.no_slot, "a Scrubber takes no build slot");
    // The cap: half the population in hundreds of millions, between 2 and 10.
    assert_eq!(g.scrubber_cap(StateId::Russia), 2, "Russia at 1.5 takes the floor");
    assert_eq!(g.scrubber_cap(StateId::SouthAsia), 10, "South Asia at 19.4 takes the ceiling");
    // Only the Custodians, and only on a state they control. Every seat that is not the Custodians
    // is refused, on its own state and on anyone else's, by BOTH build paths: the Materials one and
    // the Ducat one of ticket #42, which a Faction with money could otherwise walk in through.
    g.take_control(StateId::SouthAmerica, Seat(1));
    g.take_control(StateId::SouthEastAsia, Seat(2));
    g.take_control(StateId::MiddleEast, Seat(3));
    for seat in [Seat(1), Seat(2), Seat(3)] {
        g.seats[seat.index()].stockpile.materials = 900;
        g.seats[seat.index()].stockpile.ducats = 900;
        for state in [StateId::SouthAmerica, StateId::SouthEastAsia, StateId::MiddleEast, StateId::EastAsia] {
            for o in [
                Order::BuildFacility { state, kind: FacilityKind::Scrubber },
                Order::BuildFacilityWithDucats { state, kind: FacilityKind::Scrubber },
            ] {
                assert!(
                    g.check_order(seat, &[], &o).is_err(),
                    "only the Custodians build a Scrubber: {:?} was allowed one in {state:?} ({o:?})",
                    g.kind(seat)
                );
            }
        }
    }
    // And a Custodian may not build one in a state it merely occupies, or does not hold at all.
    assert!(
        g.check_order(Seat(0), &[], &Order::BuildFacility { state: StateId::SouthAmerica, kind: FacilityKind::Scrubber }).is_err(),
        "a Scrubber needs a Nation State the Custodians control"
    );
    // It is built without a slot: fill the state and build one anyway.
    g.seats[0].stockpile.materials = 900;
    while g.free_slots(sid) > 0 {
        g.state_mut(sid).facilities.push(facility(FacilityKind::Bank));
    }
    let slots = g.slots_used(sid);
    let o = Order::BuildFacility { state: sid, kind: FacilityKind::Scrubber };
    g.check_order(Seat(0), &[], &o).expect("a Scrubber needs no free slot");
    g.commit_orders(Seat(0), &[o]);
    assert_eq!(g.slots_used(sid), slots, "and it uses none either");
    g.resolution_phase();
    assert_eq!(g.scrubbers_online(sid), 0, "two turns to build");
    g.turn += 1;
    g.resolution_phase();
    assert_eq!(g.scrubbers_online(sid), 1, "and then it stands");

    // The Sink.
    let e = g.emissions_now();
    assert!((e.scrubbers - 3.0).abs() < 1e-9, "one Scrubber is 3.0 ppm");
    assert!((e.total_sink() - (g.tables.climate.natural_sink + 3.0)).abs() < 1e-9, "beside the Natural Sink");

    // The cap, read against what stands and what is on order.
    let cap = g.scrubber_cap(sid);
    while g.scrubbers_committed(sid) < cap {
        g.state_mut(sid).facilities.push(facility(FacilityKind::Scrubber));
    }
    assert!(g.check_order(Seat(0), &[], &Order::BuildFacility { state: sid, kind: FacilityKind::Scrubber }).is_err(), "the cap holds at {cap}");

    // Removal for Blame, and the Unrest it takes off.
    g.state_mut(sid).unrest = 5.0;
    let removed = g.scrubber_removal_by_seat()[0];
    assert!((removed - 3.0 * cap as f64).abs() < 1e-9);
    let before = g.seats[0].blame_removed;
    g.climate_phase();
    assert!((g.seats[0].blame_removed - before - removed).abs() < 1e-9, "the Climate phase credits it as removal");
    assert!((g.calming_fall(sid) - g.tables.unrest.scrubber_fall).abs() < 1e-9, "a Scrubber calms its state by 1 a turn");

    // Destroyed when the state changes hands.
    g.transfer_control(Place::State(sid), Seat(1), "Influence");
    assert_eq!(g.scrubbers_online(sid), 0, "the Scrubbers do not pass to whoever takes the state");
    assert!(g.report.lines.iter().any(|l| l.text.contains("Scrubber(s) in East Asia were destroyed")), "and the Report says so");
}

/// (f) A Strip Permit doubles a state's Facility output for three turns, then raises its Baseline
/// Emissions by 0.2 and its Unrest by 3, for good. Once per state, ever, and the Prospectors only.
#[test]
fn f_a_strip_permit_doubles_output_for_three_turns_then_charges_its_price() {
    let mut g = game();
    calm(&mut g);
    let sid = StateId::SouthAmerica;
    g.take_control(sid, Seat(1));
    g.state_mut(sid).facilities.push(facility(FacilityKind::Factory));
    let t = g.tables.strip_permit.clone();
    assert_eq!((t.turns, t.multiplier, t.baseline_rise, t.unrest), (3, 2.0, 0.2, 3.0));
    // The Custodians have no Strip Permit.
    assert!(g.check_order(Seat(0), &[], &Order::StripPermit { state: StateId::EastAsia }).is_err(), "only the Prospectors issue one");
    let o = Order::StripPermit { state: sid };
    assert_eq!(g.order_cost(Seat(1), &o), Cost::default(), "a Strip Permit is free");
    let normal = g.facility_yield(Seat(1), sid, FacilityKind::Factory).amount;
    assert!(normal > 0);
    let baseline = g.baseline_emissions(sid);
    g.state_mut(sid).unrest = 2.0;
    g.state_mut(sid).unrest_reported = 2.0;
    g.commit_orders(Seat(1), std::slice::from_ref(&o));
    assert!(g.state(sid).strip_permit_used, "one per state, ever");
    assert!(g.check_order(Seat(1), &[], &o).is_err(), "and never a second");
    // Three turns of Income at double output, then the price.
    for turn in 1..=3 {
        g.turn += 1;
        assert_eq!(g.facility_yield(Seat(1), sid, FacilityKind::Factory).amount, normal * 2, "doubled on turn {turn} of the permit");
        // The Unrest is set afresh before the last Resolution, since every Resolution takes the
        // turn's own fall off whatever stands.
        if turn == 3 {
            g.state_mut(sid).unrest = 2.0;
        }
        g.resolution_phase();
    }
    assert_eq!(g.facility_yield(Seat(1), sid, FacilityKind::Factory).amount, normal, "and back to normal afterwards");
    assert!((g.baseline_emissions(sid) - (baseline + 0.2)).abs() < 1e-9, "its Baseline Emissions rose 0.2 for good");
    // The rise is 3; the turn's own fall of 1.5 comes off it in the same Resolution.
    let expected = 2.0 + 3.0 - g.tables.unrest.natural_fall;
    assert!((g.unrest(sid) - expected).abs() < 1e-9, "Unrest {} where {expected} was wanted", g.unrest(sid));
}

/// (h) Restoration is retired: its table, its Ducat price and its AI weight are gone, and the
/// Custodians' card names the Scrubber and Leapfrog instead. The two orders themselves no longer
/// exist in `Order`, so no test can name them; the two tests that did were rewritten above
/// (`ducats_pay_for_a_leapfrog_and_repairs_at_the_table_rates` and
/// `b_a_scrubbers_removal_is_credited_and_blame_floors_at_zero`).
#[test]
fn h_restoration_is_retired_for_the_scrubber() {
    let g = game();
    let text = std::fs::read_to_string(default_data_dir().join("factions.toml")).expect("factions.toml");
    assert!(!text.contains("\n[restoration]"), "the [restoration] table is gone");
    assert!(!text.contains("
per_restoration_step ="), "and so is its Ducat price");
    assert_eq!(g.tables.ducats.per_leapfrog, 50, "a Leapfrog took its place in [ducats]");
    let ai = std::fs::read_to_string(default_data_dir().join("ai.toml")).expect("ai.toml");
    assert!(!ai.contains("\nrestoration ="), "and its AI weight is gone");
    for k in FactionKind::ALL {
        assert_eq!(g.tables.ai_weights(k).build_scrubber, if k == FactionKind::Custodians { 8.0 } else { 0.0 });
    }
    let custodians = &g.tables.faction(FactionKind::Custodians).signature;
    assert!(custodians.contains("Scrubber") && custodians.contains("Leapfrog"), "the Custodian card names them: {custodians}");
    assert!(!custodians.contains("Restoration"), "and no longer names Restoration");
    let prospectors = &g.tables.faction(FactionKind::Prospectors).signature;
    assert!(prospectors.contains("Cheap Industry") && prospectors.contains("Strip Permit"), "the Prospector card names both: {prospectors}");
}

/// Ticket #54: Leapfrog lowers Emissions, so a Custodian AI behind on Stabilization reaches for it
/// as it reaches for a Scrubber, instead of spending every Ducat on Influence first.
#[test]
fn a_custodian_ai_behind_on_stabilization_leapfrogs_when_it_has_the_ducats() {
    let mut g = game();
    let cust = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Custodians).unwrap();
    g.turn = 14; // well past the pace at which the Custodians should be under the Sink
    g.seats[cust.index()].stockpile.ducats = 120;
    g.seats[cust.index()].stockpile.materials = 0;
    let orders = g.ai_orders(cust);
    let scored: Vec<String> = g.log.iter().filter(|&l| l.contains("Leapfrog") || l.contains("Scrubber") || l.contains("Ducats")).cloned().collect();
    let probe = (g.controlled_states(cust), g.leapfrog_would_bite(StateId::EastAsia), g.population_coefficient(StateId::EastAsia), g.seats[cust.index()].stockpile.ducats, g.kind(cust));
    assert!(orders.iter().any(|o| matches!(o, Order::Leapfrog { .. })), "no Leapfrog among: {orders:?}
scored: {scored:#?}
probe: {probe:?}");
}


// ---------------------------------------------------------------- Ticket #55: Breaks, Committed
// Warming and the Last Turn

/// (a) A Break fires in the first Climate phase whose Temperature stands at or above its figure,
/// and never again.
#[test]
fn a_a_break_fires_once_at_its_temperature_and_never_again() {
    let mut g = game();
    calm(&mut g);
    breaks_ahead(&mut g);
    let i = break_at(&g, "permafrost_thaw");
    let at = g.tables.climate.breaks[i].temperature;
    hold_temperature(&mut g, at - 0.05);
    g.climate_phase();
    assert!(!g.climate.breaks_fired[i], "under its Temperature it has not fired");
    assert_eq!(g.climate.permafrost, 0.0, "and nothing of it is in the world yet");

    hold_temperature(&mut g, at);
    g.climate_phase();
    assert!(g.climate.breaks_fired[i], "at its Temperature it fires");
    assert_eq!(g.climate.permafrost, 4.0, "and its effect is in the world");
    assert!(
        g.report.lines.iter().any(|l| l.text.contains("Permafrost Thaw. The permafrost thaws.")),
        "the Report says it happened: {:?}",
        g.report.lines
    );

    hold_temperature(&mut g, at + 0.5);
    g.climate_phase();
    assert_eq!(g.climate.permafrost, 4.0, "a Break never fires twice, however far the Temperature goes past it");
}

/// (b) Coral Die-off: +1 Unrest and -2% population on the Coastal Exposure 2 states, and on no others.
#[test]
fn b_coral_die_off_costs_the_exposed_coasts_their_people_and_their_calm() {
    let mut g = game();
    calm(&mut g);
    breaks_ahead(&mut g);
    bare_world(&mut g);
    // Central America is Coastal Exposure 2 and both its neighbours are 1, so nothing flows back
    // into it. Europe is Coastal Exposure 1 and stands as the control.
    g.state_mut(StateId::CentralAmerica).population = 10.0;
    g.state_mut(StateId::Europe).population = 10.0;
    // A shade above the Break's figure: with nothing emitting, the phase takes the Sink off the
    // Stock and the Temperature settles just under where it was held.
    let at = g.tables.climate.breaks[break_at(&g, "coral_die_off")].temperature;
    hold_temperature(&mut g, at + 0.05);
    g.climate_phase();
    assert!(g.climate.temperature >= at, "the phase settled at or above the Break: {}", g.climate.temperature);
    let rate = g.population_growth_rate();
    assert!((g.state(StateId::CentralAmerica).population - 9.8 * (1.0 + rate)).abs() < 1e-9, "-2%: {}", g.state(StateId::CentralAmerica).population);
    assert!((g.state(StateId::Europe).population - 10.0 * (1.0 + rate)).abs() < 1e-9, "Coastal Exposure 1 loses nothing: {}", g.state(StateId::Europe).population);
    assert_eq!(g.unrest(StateId::CentralAmerica), 1.0, "one Unrest on an exposed coast");
    assert_eq!(g.unrest(StateId::EastAsia), 1.0, "every Coastal Exposure 2 state, not one");
    assert_eq!(g.unrest(StateId::Europe), 0.0, "and no other");
    // The 2% that left flowed on as refugees, by the rule for a population that falls.
    assert!(g.state(StateId::NorthAmerica).population > 0.0 || g.state(StateId::SouthAmerica).population > 0.0, "the people who left went to the neighbours");
}

/// (c) Permafrost Thaw: 4.0 ppm every Climate phase after it, on its own line, nobody's Blame and
/// never counted against a Stabilization run.
#[test]
fn c_the_permafrost_break_adds_four_ppm_a_turn_on_its_own_line_outside_blame_and_stabilization() {
    let mut g = game();
    calm(&mut g);
    breaks_ahead(&mut g);
    bare_world(&mut g);
    g.take_control(StateId::EastAsia, Seat(0));
    hold_temperature(&mut g, 1.7);
    g.climate_phase();
    assert_eq!(g.climate.permafrost, 4.0, "the Break has fired");
    let e = g.emissions_now();
    assert_eq!(e.permafrost, 4.0, "its own line on the panel");
    assert_eq!(e.counted(), 0.0, "and not among the counted sources");
    assert!((e.total() - 4.0).abs() < 1e-9, "but in the total that reaches the air: {}", e.total());

    // A Sink of 2.0: 0 counted is under it, 4.0 of Permafrost would not be.
    g.climate.natural_sink = 2.0;
    let blame_before: f64 = Seat::ALL.into_iter().map(|s| g.seat(s).blame_emitted).sum();
    let co2_before = g.climate.co2;
    let run_before = g.seat(Seat(0)).stabilization_run;
    g.climate_phase();
    assert!((g.climate.co2 - (co2_before + 4.0 - 2.0)).abs() < 1e-9, "4.0 ppm again, every Climate phase from then on: {}", g.climate.co2);
    let blame_after: f64 = Seat::ALL.into_iter().map(|s| g.seat(s).blame_emitted).sum();
    assert_eq!(blame_after, blame_before, "nobody's Blame");
    assert_eq!(g.seat(Seat(0)).stabilization_run, run_before + 1, "exempt from Stabilization: the run goes on");
}

/// (d) The Sink Weakens: the Natural Sink falls to 4.0 for good, and Stabilization is measured
/// against 4.0 from then on.
#[test]
fn d_the_sink_weakens_lowers_the_sink_and_the_stabilization_bar_with_it() {
    let mut g = game();
    calm(&mut g);
    breaks_ahead(&mut g);
    bare_world(&mut g);
    assert_eq!(g.climate.natural_sink, 6.0, "it opens at the table's figure");
    hold_temperature(&mut g, 2.1);
    g.climate_phase();
    assert_eq!(g.climate.natural_sink, 4.0, "the Sink Weakens took it to 4.0");
    assert_eq!(g.emissions_now().sink, 4.0, "and the panel's Sink line with it");
    assert!(g.seat(Seat(0)).stabilization_run > 0, "0 counted was under the Sink");

    // 5.0 counted Emissions: under the old Sink of 6.0, over the new one of 4.0. A neutral state,
    // so no Faction's Emissions multiplier stands between the card and the sum.
    g.state_mut(StateId::MiddleEast).control = Control::Neutral;
    g.state_mut(StateId::MiddleEast).industry_level = 10; // Baseline 0.5 x 10
    let e = g.emissions_now();
    assert!((e.counted() - 5.0).abs() < 1e-9, "5.0 counted: {}", e.counted());
    g.climate_phase();
    assert_eq!(g.seat(Seat(0)).stabilization_run, 0, "5.0 breaks a run against a Sink of 4.0, where it would have held against 6.0");
    // The Custodian AI steers by this same figure (`victory_gap` reads `climate.last.total_sink()`),
    // so a weakened Sink moves its pace with it.
    assert_eq!(g.climate.last.total_sink(), 4.0, "the Sink the AI reads is the weakened one");
}

/// (e) Ice Sheets Committed fires a Sea Level threshold out of sequence, and the scheduled ones
/// still fire on their own turns: a state loses build slots twice between +2.2 and +2.3.
#[test]
fn e_ice_sheets_committed_fires_a_threshold_out_of_sequence_and_the_scheduled_ones_still_fire() {
    let mut g = game();
    breaks_ahead(&mut g);
    for s in &mut g.states {
        s.population = 0.0;
    }
    // The first scheduled threshold, +1.8, goes first.
    hold_temperature(&mut g, 1.85);
    g.climate_phase();
    let after_first = g.build_slots(StateId::EastAsia);
    assert!(g.state(StateId::EastAsia).thresholds_fired[0], "+1.8 has fired");

    // +2.2: the Break, with +2.3 still ahead.
    hold_temperature(&mut g, 2.25);
    g.climate_phase();
    let after_break = g.build_slots(StateId::EastAsia);
    assert_eq!(after_break, after_first - 2, "the Break took a threshold's worth of slots out of sequence");
    assert!(!g.state(StateId::EastAsia).thresholds_fired[1], "and it did not use up the scheduled +2.3");

    // +2.3: the scheduled threshold, on its own turn.
    hold_temperature(&mut g, 2.35);
    g.climate_phase();
    assert_eq!(g.build_slots(StateId::EastAsia), after_break - 2, "so the state loses slots twice between +2.2 and +2.3");
}

/// (f) Amazon Dieback: 20 ppm into the CO2 Stock once, and South America's Baseline Emissions up by
/// 1.0 for good.
#[test]
fn f_amazon_dieback_pulses_twenty_ppm_once_and_leaves_south_america_dirtier_for_good() {
    let mut g = game();
    calm(&mut g);
    breaks_ahead(&mut g);
    bare_world(&mut g);
    let base = g.baseline_emissions(StateId::SouthAmerica);
    hold_temperature(&mut g, 2.7);
    let co2_before = g.climate.co2;
    g.climate_phase();
    // Nothing counted, no Permafrost yet in the sum, a Sink of 6.0, then the 20 ppm pulse.
    assert!((g.climate.co2 - (co2_before - 6.0 + 20.0)).abs() < 1e-9, "a 20 ppm pulse: {}", g.climate.co2);
    assert!((g.baseline_emissions(StateId::SouthAmerica) - (base + 1.0)).abs() < 1e-9, "Baseline Emissions up by 1.0");
    // And it bites: the state's industry emits at the raised figure.
    g.state_mut(StateId::SouthAmerica).industry_level = 2;
    assert!((g.emissions_now().state_industry - (base + 1.0) * 2.0).abs() < 1e-9, "{}", g.emissions_now().state_industry);
    g.state_mut(StateId::SouthAmerica).industry_level = 0;

    // A second phase: Permafrost 4.0 against the weakened Sink of 4.0, and no second pulse.
    let co2_again = g.climate.co2;
    g.climate_phase();
    assert!((g.climate.co2 - co2_again).abs() < 1e-9, "the pulse does not come twice: {}", g.climate.co2);
    assert!((g.baseline_emissions(StateId::SouthAmerica) - (base + 1.0)).abs() < 1e-9, "and the rise does not come twice either");
}

/// (g) The Event card that was Permafrost Thaw is the Methane Burst, at 2.5.
#[test]
fn g_the_methane_burst_card_is_two_and_a_half() {
    let g = game();
    assert_eq!(g.tables.event(EventId::MethaneBurst).name, "Methane Burst");
    assert!((g.tables.events.methane_emissions - 2.5).abs() < 1e-9, "{}", g.tables.events.methane_emissions);
    assert!(g.tables.event(EventId::MethaneBurst).effect.contains("2.5"), "the card text says so: {}", g.tables.event(EventId::MethaneBurst).effect);
}

/// (h) Committed Warming is the target Temperature: what the CO2 Stock as it stands delivers once
/// the lag has caught up.
#[test]
fn h_committed_warming_is_the_temperature_the_stock_delivers_once_the_lag_catches_up() {
    let mut g = game();
    let c = g.tables.climate.clone();
    g.climate.co2 = c.starting_co2 + 2.0 * c.ppm_step;
    assert!((g.target_temperature() - (c.base_temperature + 2.0 * c.degrees_per_ppm_step)).abs() < 1e-9, "{}", g.target_temperature());
    // Hold the stock still and let the lag run: the Temperature arrives at exactly that figure.
    g.climate.temperature = c.base_temperature;
    for _ in 0..40 {
        let t = g.climate.temperature;
        g.climate.temperature = t + (g.target_temperature() - t) * c.temperature_lag_fraction;
    }
    assert!((g.climate.temperature - g.target_temperature()).abs() < 1e-9, "{} against {}", g.climate.temperature, g.target_temperature());
}

/// Ticket #60: the (i) paths below are worked examples in ppm, and every ppm figure in them is read
/// against `ppm_step`, which is re-swept on every ticket that changes the board (150 on #53, 180 on
/// #60). Scaling a figure with the step keeps the Temperature arithmetic -- and so the answer the
/// example was written for -- exactly as it was, whatever the step becomes next.
fn at_the_step(g: &Game, at_150: f64) -> f64 {
    at_150 * g.tables.climate.ppm_step / 150.0
}

/// A constructed path: `gross` ppm of Emissions a turn against the Sink as it stands, at `temp`
/// with the CO2 Stock `above` ppm over its starting figure.
fn path(g: &mut Game, gross: f64, temp: f64, above: f64) {
    bare_world(g);
    g.climate.co2 = g.tables.climate.starting_co2 + above;
    g.climate.temperature = temp;
    g.climate.last = EmissionsBreakdown { state_industry: gross, sink: g.climate.natural_sink, ..Default::default() };
}

/// (i) The Last Turn, on a path whose answer can be worked out by hand. Turn 1 of 24, the Stock 405
/// ppm above its start at a step of 150 (target +2.55 C), the Temperature +1.5, and 20 ppm NET a
/// turn going in. Cutting net Emissions to zero at turn k freezes the Stock at 405 + (k - 2) x 20
/// ppm above the start, and the Temperature then arrives at 1.2 + 0.5 x that / 150. At k = 8 that is
/// +2.95 C, under the Collapse Line; at k = 9 it is +3.02 C, over it. So the answer is 8. Both ppm
/// figures are taken at the step in force (`at_the_step`), so every ratio in that arithmetic, and
/// so the answer, is the same whatever the step is.
#[test]
fn i_a_the_last_turn_is_the_latest_turn_a_cut_still_avoids_collapse() {
    let mut g = game();
    calm(&mut g);
    let (net, above, sink) = (at_the_step(&g, 20.0), at_the_step(&g, 405.0), g.climate.natural_sink);
    path(&mut g, net + sink, 1.5, above);
    assert_eq!(g.turn, 1);
    assert_eq!(g.last_turn_to_act(), LastTurn::Turn(8));
}

/// (i) A path that never collapses: net Emissions already at the Sink and the Stock at its start.
#[test]
fn i_b_the_last_turn_says_so_when_the_path_never_collapses() {
    let mut g = game();
    breaks_ahead(&mut g);
    path(&mut g, 6.0, 1.2, 0.0);
    assert_eq!(g.last_turn_to_act(), LastTurn::NoCollapse);
}

/// (i) A path already past saving: the Stock as it stands is committed to +3.2 C, so no cut of any
/// kind keeps the Temperature under the line.
#[test]
fn i_c_the_last_turn_says_so_when_cuts_alone_no_longer_avoid_collapse() {
    let mut g = game();
    calm(&mut g);
    let (sink, above) = (g.climate.natural_sink, at_the_step(&g, 600.0));
    path(&mut g, sink, 1.5, above);
    assert!((g.target_temperature() - 3.2).abs() < 1e-9, "committed to {:+.2}", g.target_temperature());
    assert_eq!(g.last_turn_to_act(), LastTurn::TooLate);
}

/// (i) A Break on the path brings the Last Turn forward: the same path, with the Permafrost Thaw
/// still ahead of it, leaves less room than one where it has already fired. Permafrost is the world's
/// carbon, not anybody's, so a cut of net Emissions does not touch it: it goes on adding 4.0 ppm a
/// turn past the cut, and the Temperature the Stock is committed to goes on rising with it.
#[test]
fn i_d_a_break_on_the_path_brings_the_last_turn_forward() {
    let mut clear = game();
    calm(&mut clear);
    let (net, above, sink) = (at_the_step(&clear, 20.0), at_the_step(&clear, 405.0), clear.climate.natural_sink);
    path(&mut clear, net + sink, 1.5, above);
    let LastTurn::Turn(without) = clear.last_turn_to_act() else { panic!("the clear path has a Last Turn") };

    let mut ahead = game();
    calm(&mut ahead);
    path(&mut ahead, net + sink, 1.5, above);
    let i = break_at(&ahead, "permafrost_thaw");
    ahead.climate.breaks_fired[i] = false;
    let LastTurn::Turn(with) = ahead.last_turn_to_act() else { panic!("the path with the Break has a Last Turn") };

    assert!(with < without, "the Permafrost Thaw on the path brings the Last Turn forward: {with} against {without}");
}

// ---------------------------------------------------------------- Ticket #56: sea level and the ice

/// A Nation State this seat directs, with room and money, so a build order is legal.
fn directed(g: &mut Game, sid: StateId) {
    g.take_control(sid, Seat(0));
    g.seats[0].stockpile.materials = 500;
    g.seats[0].stockpile.energy = 500;
    g.seats[0].stockpile.ducats = 500;
}

/// The kinds standing in a state's coastal slots, or in its inland ones, in the order they stand.
fn standing(g: &Game, sid: StateId, coastal: bool) -> Vec<FacilityKind> {
    g.state(sid).facilities.iter().filter(|f| f.coastal == coastal).map(|f| f.kind).collect()
}

/// Every sea-level threshold still ahead of every state.
fn sea_ahead(g: &mut Game) {
    let n = g.tables.climate.sea_level_thresholds.len();
    for s in &mut g.states {
        s.thresholds_fired = vec![false; n];
    }
}

/// (a) Build slots are Size + Industry Level + `base_slots`, and `base_slots` is 3.
#[test]
fn a_every_state_has_three_more_build_slots() {
    let g = fresh();
    assert_eq!(g.tables.base_slots, 3, "nation_states.toml gives every state three slots on top");
    for sid in StateId::ALL {
        let card = g.tables.state(sid);
        let want = card.size + g.state(sid).industry_level + 3;
        assert_eq!(g.build_slots(sid), want, "{}: Size {} + Industry Level {} + 3", card.name, card.size, g.state(sid).industry_level);
    }
    // And a raise adds one more.
    let mut g = fresh();
    let before = g.build_slots(StateId::Europe);
    g.state_mut(StateId::Europe).industry_level += 1;
    assert_eq!(g.build_slots(StateId::Europe), before + 1, "a raise of the Industry Level adds a slot");
}

/// (b) Coastal slots are 3 x Coastal Exposure, capped at the start slots less one; the rest of the
/// start slots are inland, and every slot a raise adds is inland.
#[test]
fn b_coastal_slots_are_three_an_exposure_capped_and_a_raise_is_inland() {
    let mut g = fresh();
    assert_eq!(g.tables.coastal_per_exposure, 3, "three coastal slots per point of Coastal Exposure");
    for sid in StateId::ALL {
        let card = g.tables.state(sid);
        let start = card.size + card.industry_level + 3;
        let want = (3 * card.coastal_exposure).min(start - 1);
        assert_eq!(g.start_slots(sid), start, "{}: start slots", card.name);
        assert_eq!(g.coastal_slots(sid), want, "{}: 3 x Exposure {} capped at {} start slots less one", card.name, card.coastal_exposure, start);
        assert_eq!(g.inland_slots(sid), start - want, "{}: the rest of the start slots are inland", card.name);
        assert_eq!(g.coastal_slots(sid) + g.inland_slots(sid), g.build_slots(sid), "{}: the two rows are the whole card", card.name);
    }
    // Central America and the Caribbean is where the cap bites: Size 1, Industry Level 1, Coastal
    // Exposure 2 -> 1 + 3 + 1 = 5 start slots, 3 x 2 = 6 coastal wanted, capped at 4.
    let ca = StateId::CentralAmerica;
    assert_eq!(g.start_slots(ca), 5, "Central America starts with five slots");
    assert_eq!(g.coastal_slots(ca), 4, "six coastal wanted, capped at the start slots less one");
    assert_eq!(g.inland_slots(ca), 1, "and one inland");

    // Every slot a raise adds is inland.
    let coastal_before = g.coastal_slots(ca);
    g.state_mut(ca).industry_level += 2;
    assert_eq!(g.coastal_slots(ca), coastal_before, "a raise adds no coastal slot");
    assert_eq!(g.inland_slots(ca), 3, "it adds inland slots");
}

/// (c) Start Facilities take coastal slots first, in the table's order; a new build fills an inland
/// slot while one is free.
#[test]
fn c_start_facilities_are_coastal_first_and_a_new_build_is_inland_first() {
    let mut g = fresh();
    // East Asia: three start Facilities, six coastal slots, three inland.
    let sid = StateId::EastAsia;
    // East Asia is the player's start state, so its Launch Site is a start Facility too and takes
    // the next coastal slot after the three on the card.
    assert_eq!(
        standing(&g, sid, true),
        vec![FacilityKind::Factory, FacilityKind::PowerPlant, FacilityKind::Refinery, FacilityKind::LaunchSite],
        "the start Facilities stand on the coast, in the table's order"
    );
    assert!(standing(&g, sid, false).is_empty(), "and nothing stands inland");

    // A new build takes an inland slot while one is free.
    directed(&mut g, sid);
    let orders = vec![Order::BuildFacility { state: sid, kind: FacilityKind::Bank }];
    g.end_turn([orders, Vec::new(), Vec::new(), Vec::new()]);
    g.end_turn(std::array::from_fn(|_| Vec::new()));
    assert!(standing(&g, sid, false).contains(&FacilityKind::Bank), "the Bank went inland: {:?}", standing(&g, sid, false));
}

/// (d) A threshold takes coastal slots only, destroys the oldest Facility standing in them first,
/// and takes nothing once the coastal slots are gone.
#[test]
fn d_the_sea_takes_coastal_slots_only_oldest_first_and_then_nothing() {
    let mut g = game();
    calm(&mut g);
    sea_ahead(&mut g);
    for s in &mut g.states {
        s.population = 0.0;
    }
    // Australia and Oceania: Size 2, Industry Level 2, Exposure 2 -> 7 slots, 6 coastal, 1 inland.
    let sid = StateId::Australia;
    let st = g.state_mut(sid);
    st.control = Control::Controlled(Seat(0));
    st.facilities = vec![
        Facility::in_coastal_slot(FacilityKind::Factory),
        Facility::in_coastal_slot(FacilityKind::Refinery),
        Facility::in_coastal_slot(FacilityKind::PowerPlant),
        Facility::new(FacilityKind::ResearchLab),
    ];
    assert_eq!(g.coastal_slots(sid), 6);
    assert_eq!(g.inland_slots(sid), 1);

    g.apply_sea_threshold(sid, 0);
    assert_eq!(g.coastal_slots(sid), 4, "Exposure 2 takes two coastal slots");
    assert_eq!(g.inland_slots(sid), 1, "and no inland slot");
    assert_eq!(
        standing(&g, sid, true),
        vec![FacilityKind::Factory, FacilityKind::Refinery, FacilityKind::PowerPlant],
        "three coastal Facilities still fit four coastal slots"
    );

    g.apply_sea_threshold(sid, 1);
    assert_eq!(g.coastal_slots(sid), 2, "two more");
    assert_eq!(standing(&g, sid, true), vec![FacilityKind::Refinery, FacilityKind::PowerPlant], "the oldest coastal Facility went first: the Factory");
    g.apply_sea_threshold(sid, 2);
    assert_eq!(g.coastal_slots(sid), 0, "the coast is gone");
    assert!(standing(&g, sid, true).is_empty(), "and everything that stood on it with it");
    assert_eq!(standing(&g, sid, false), vec![FacilityKind::ResearchLab], "the inland Research Lab never moved");
    assert!(
        g.report.lines.iter().any(|l| l.text.contains("The sea took 2 coastal slots from Australia and Oceania")),
        "the Report names what the sea took: {:?}",
        g.report.lines
    );

    // Nothing more to take.
    let slots = g.build_slots(sid);
    g.apply_sea_loss(sid, 2.9);
    assert_eq!(g.build_slots(sid), slots, "once a state's coastal slots are gone it loses nothing more");
    assert!(g.report.lines.iter().any(|l| l.text.contains("no coastal slots left")), "and the Report says so: {:?}", g.report.lines);
}

/// (e) The Sea Wall: it needs Coastal Engineering and a free coastal slot, one per state, and it
/// absorbs the state's next threshold of any kind and is destroyed doing it. A mothballed one does not.
#[test]
fn e_the_sea_wall_needs_its_tech_and_a_coastal_slot_and_takes_one_threshold() {
    let sid = StateId::Australia;
    let mut g = game();
    directed(&mut g, sid);
    let order = Order::BuildFacility { state: sid, kind: FacilityKind::SeaWall };
    assert!(g.check_order(Seat(0), &[], &order).is_err(), "no Coastal Engineering, no Sea Wall");
    g.research.done.push(TechId::CoastalEngineering);
    assert!(g.check_order(Seat(0), &[], &order).is_ok(), "with the Tech in, it is legal");

    // One per state.
    g.state_mut(sid).facilities.push(Facility::in_coastal_slot(FacilityKind::SeaWall));
    assert!(g.check_order(Seat(0), &[], &order).is_err(), "at most one Sea Wall stands in a state");

    // No free coastal slot: illegal even in a state with inland room.
    let mut g2 = game();
    let s2 = StateId::CentralAmerica;
    directed(&mut g2, s2);
    g2.research.done.push(TechId::CoastalEngineering);
    for _ in 0..g2.coastal_slots(s2) {
        g2.state_mut(s2).facilities.push(Facility::in_coastal_slot(FacilityKind::Factory));
    }
    assert!(g2.free_inland(s2) > 0, "there is inland room");
    assert!(
        g2.check_order(Seat(0), &[], &Order::BuildFacility { state: s2, kind: FacilityKind::SeaWall }).is_err(),
        "but a Sea Wall wants a coastal slot"
    );

    // It absorbs a scheduled threshold and is destroyed doing it.
    let mut g = game();
    calm(&mut g);
    sea_ahead(&mut g);
    for s in &mut g.states {
        s.population = 0.0;
    }
    g.state_mut(sid).facilities = vec![Facility::in_coastal_slot(FacilityKind::SeaWall)];
    let before = g.coastal_slots(sid);
    g.apply_sea_threshold(sid, 0);
    assert_eq!(g.coastal_slots(sid), before, "the wall took the sea: no coastal slot lost");
    assert!(!g.state(sid).facilities.iter().any(|f| f.kind == FacilityKind::SeaWall), "and it was destroyed doing it");
    assert!(g.report.lines.iter().any(|l| l.text.contains("Sea Wall") && l.text.contains("destroyed")), "the Report says so: {:?}", g.report.lines);
    // The next one lands as normal.
    g.apply_sea_threshold(sid, 1);
    assert_eq!(g.coastal_slots(sid), before - 2, "the wall absorbed one threshold, not two");

    // The Ice Sheets Break is a threshold of a kind too.
    let mut g = game();
    calm(&mut g);
    breaks_ahead(&mut g);
    bare_world(&mut g);
    g.state_mut(sid).facilities = vec![Facility::in_coastal_slot(FacilityKind::SeaWall)];
    let before = g.coastal_slots(sid);
    hold_temperature(&mut g, 2.25);
    g.climate_phase();
    assert_eq!(g.coastal_slots(sid), before, "the wall took the Ice Sheets Break");
    assert!(!g.state(sid).facilities.iter().any(|f| f.kind == FacilityKind::SeaWall), "and went with it");

    // A mothballed wall absorbs nothing.
    let mut g = game();
    calm(&mut g);
    sea_ahead(&mut g);
    for s in &mut g.states {
        s.population = 0.0;
    }
    let mut wall = Facility::in_coastal_slot(FacilityKind::SeaWall);
    wall.mothballed = true;
    wall.online = false;
    g.state_mut(sid).facilities = vec![wall];
    let before = g.coastal_slots(sid);
    g.apply_sea_threshold(sid, 0);
    assert_eq!(g.coastal_slots(sid), before - 2, "a mothballed Sea Wall absorbs nothing");
}

/// (f) Coastal Engineering is Industry rung 2, needs Efficient Grids, and the tree holds thirteen.
#[test]
fn f_coastal_engineering_is_the_thirteenth_tech() {
    let g = fresh();
    assert_eq!(TechId::ALL.len(), 13, "thirteen Techs");
    assert_eq!(g.tables.techs.len(), 13, "and thirteen rows in techs.toml");
    let c = g.tables.tech(TechId::CoastalEngineering);
    assert_eq!(c.name, "Coastal Engineering");
    assert_eq!(c.branch, "Industry");
    assert_eq!(c.rung, 2, "rung 2, beside Clean Power");
    assert_eq!(c.cost, 25);
    assert_eq!(c.needs, vec![TechId::EfficientGrids], "it needs Efficient Grids");
    assert!(c.effect.contains("Sea Wall"), "its effect names the Sea Wall: {}", c.effect);
    // Two boxes on Industry rung 2.
    let rung_two: Vec<&str> = TechId::ALL
        .into_iter()
        .map(|t| g.tables.tech(t))
        .filter(|t| t.branch == "Industry" && t.rung == 2)
        .map(|t| t.name.as_str())
        .collect();
    assert_eq!(rung_two, vec!["Clean Power", "Coastal Engineering"], "two boxes on Industry rung 2");
    // The Sea Wall's card names it as its unlock, and there are ten Facilities.
    assert_eq!(g.tables.facility(FacilityKind::SeaWall).needs_tech, Some(TechId::CoastalEngineering));
    assert_eq!(FacilityKind::ALL.len(), 10, "ten Facilities");
}

/// (g) Antarctica opens the first Climate phase the Temperature stands at +1.6, stays open, and its
/// yields are the card's.
#[test]
fn g_antarctica_opens_at_one_point_six_and_stays_open() {
    let mut g = game();
    calm(&mut g);
    let opens = g.tables.climate.antarctica_opens_at;
    assert!((opens - 1.6).abs() < 1e-9, "+1.6 C: {opens}");
    assert!(!g.antarctica_open, "the ice is shut when the game opens");

    // A Colony Ship at Earth cannot found there yet.
    let id = ShipId(g.fresh_id());
    let turn = g.turn;
    g.ships.push(Ship {
        id,
        kind: UnitKind::ColonyShip,
        seat: Seat(0),
        damage: 0,
        at: ShipAt::Body(BodyId::Earth),
        colonists: 4,
        army: None,
        stance: Stance::Hold,
        escaped: false,
        arrived_this_turn: false,
        built_turn: turn,
    });
    let found = Order::Unload { ship: id, colonists: 4, army: false, into: UnloadTarget::Slot(BodyId::Earth, 0) };
    let refused = g.check_order(Seat(0), &[], &found).unwrap_err();
    assert!(format!("{refused}").contains("+1.6"), "the refusal says when it opens: {refused}");

    // Below the line, a Climate phase leaves it shut.
    hold_temperature(&mut g, 1.55);
    g.climate_phase();
    assert!(!g.antarctica_open, "still shut at +1.55");
    // At the line it opens, and says so.
    hold_temperature(&mut g, 1.6);
    g.climate_phase();
    assert!(g.antarctica_open, "open at +1.6");
    assert!(g.report.lines.iter().any(|l| l.text.contains("The Antarctic ice opens")), "the Report says so: {:?}", g.report.lines);
    assert!(g.check_order(Seat(0), &[], &found).is_ok(), "and the Colony Ship may found there");
    // It stays open when the world cools.
    hold_temperature(&mut g, 1.2);
    g.climate_phase();
    assert!(g.antarctica_open, "once open it stays open");

    // The yields on the card.
    let e = g.tables.body(BodyId::Earth);
    assert!((e.mine_yield - 1.75).abs() < 1e-9, "Mine 1.75: {}", e.mine_yield);
    assert!((e.refinery_yield - 2.0).abs() < 1e-9, "Refinery 2.0: {}", e.refinery_yield);
    assert!((e.generator_yield - 0.75).abs() < 1e-9, "Generator 0.75: {}", e.generator_yield);
    assert!((e.habitat_yield - 1.0).abs() < 1e-9, "Habitat 1.0: {}", e.habitat_yield);
}

/// (h) The AI enumerates a Sea Wall once Coastal Engineering is in and a threshold is near.
#[test]
fn h_the_ai_raises_a_sea_wall_when_the_sea_is_close() {
    let sid = StateId::Australia;
    let mut g = game();
    calm(&mut g);
    sea_ahead(&mut g);
    directed(&mut g, sid);
    g.research.done.push(TechId::CoastalEngineering);
    // The first threshold is +1.8; stand within 0.2 C of it.
    hold_temperature(&mut g, 1.65);
    let orders = g.ai_orders(Seat(0));
    assert!(
        orders.iter().any(|o| matches!(o, Order::BuildFacility { state, kind: FacilityKind::SeaWall } if *state == sid)),
        "the AI walls the coast with the sea 0.15 C away: {orders:?}"
    );

    // Without the Tech it enumerates none, however close the sea.
    let mut g = game();
    calm(&mut g);
    sea_ahead(&mut g);
    directed(&mut g, sid);
    hold_temperature(&mut g, 1.65);
    let orders = g.ai_orders(Seat(0));
    assert!(
        !orders.iter().any(|o| matches!(o, Order::BuildFacility { kind: FacilityKind::SeaWall, .. })),
        "no Coastal Engineering, no Sea Wall: {orders:?}"
    );
}

/// Ticket #56: the AI holds Materials for a dearer, higher-scored build within four turns of
/// income, so a Colony Ship is not starved by a Factory bought every turn once the states have
/// slots to spare.
#[test]
fn the_ai_holds_materials_four_turns_for_a_colony_ship_it_wants_more_than_a_factory() {
    let mut g = game();
    let cust = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Custodians).unwrap();
    let iss = station_of(&g, cust, BodyId::Earth).unwrap();
    g.colony_mut(iss).unwrap().modules.push(Module::new(ModuleKind::Shipyard));
    g.turn = 8;
    // Every build slot in its state is full, so the only Materials sinks are Ships: the Colony
    // Ship it wants (30) and a Frigate it wants less (25), which it must not buy meanwhile.
    for sid in g.controlled_states(cust) {
        while g.free_slots(sid) > 0 {
            g.state_mut(sid).facilities.push(facility(FacilityKind::Bank));
        }
    }
    g.seats[cust.index()].stockpile.materials = 8;
    g.seats[cust.index()].stockpile.energy = 200;
    g.seats[cust.index()].income_last_turn.materials = 8;
    g.seats[cust.index()].income_last_turn.energy = 20;
    let orders = g.ai_orders(cust);
    let spent: Vec<&Order> = orders.iter().filter(|o| g.order_cost(cust, o).materials > 0).collect();
    let lines: Vec<String> = g.log.iter().filter(|&l| l.contains("Colony Ship") || l.starts_with("  take")).cloned().collect();
    assert!(lines.iter().any(|l| l.contains("wait") && l.contains("Colony Ship")), "the Colony Ship is waited for: {lines:#?}");
    assert!(spent.is_empty(), "and nothing cheaper takes the Materials meanwhile: {spent:?}");
}

/// Ticket #56: whatever the Energy, the AI never mothballs a Shipyard or a Launch Site: they are
/// the only way off Earth, and a mothballed Shipyard starved the Custodian AI of every Colony Ship.
#[test]
fn the_ai_never_mothballs_a_shipyard_or_a_launch_site() {
    let mut g = game();
    let cust = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Custodians).unwrap();
    let iss = station_of(&g, cust, BodyId::Earth).unwrap();
    g.colony_mut(iss).unwrap().modules.push(Module::new(ModuleKind::Shipyard));
    // An Embassy (2 Energy, makes no resource) is the thing it should mothball instead.
    let home = g.controlled_states(cust)[0];
    g.state_mut(home).facilities.push(facility(FacilityKind::Embassy));
    g.turn = 6;
    g.seats[cust.index()].stockpile.energy = 1;
    g.seats[cust.index()].stockpile.materials = 0;
    g.seats[cust.index()].income_last_turn.energy = -6;
    let orders = g.ai_orders(cust);
    let mothballed: Vec<String> = orders
        .iter()
        .filter_map(|o| match o {
            Order::Change { building: BuildingRef::Facility(sid, i), .. } => Some(format!("{:?}", g.state(*sid).facilities[*i].kind)),
            Order::Change { building: BuildingRef::Module(cid, i), .. } => Some(format!("{:?}", g.colony(*cid).unwrap().modules[*i].kind)),
            _ => None,
        })
        .collect();
    assert!(!mothballed.is_empty(), "with Energy a turn from short the AI mothballs something: {orders:?}");
    assert!(!mothballed.iter().any(|m| m.contains("Shipyard") || m.contains("LaunchSite")), "never the Shipyard or the Launch Site: {mothballed:?}");
}

// ---------------------------------------------------------------- #57 real yields, a real sky

/// Ticket #57 (a): every Colony Slot draws its own four yields when the game starts, never more
/// than a quarter either side of its Body's, no two slots on a Body alike, and the same seed always
/// deals the same board.
#[test]
fn every_colony_slot_draws_its_own_four_yields_within_a_quarter_of_its_bodys() {
    let g = with_seed(11);
    let spread = g.tables.slot_yield_spread;
    assert_eq!(spread, 0.25, "bodies.toml sets the spread");
    for body in BodyId::ALL {
        let card = g.tables.body(body).clone();
        for slot in 0..card.colony_slots() {
            let y = g.slot_yields(body, slot);
            for (what, drawn, base) in [
                ("Mine", y.mine, card.mine_yield),
                ("Generator", y.generator, card.generator_yield),
                ("Refinery", y.refinery, card.refinery_yield),
                ("Habitat", y.habitat, card.habitat_yield),
            ] {
                let factor = drawn / base;
                assert!(
                    factor >= 1.0 - spread - 1e-9 && factor <= 1.0 + spread + 1e-9,
                    "{} slot {} {what}: {drawn} is x{factor:.3} of the Body's {base}, outside +/-{spread}",
                    card.name,
                    slot
                );
                assert_eq!(drawn, (drawn * 100.0).round() / 100.0, "{} slot {slot} {what} is rounded to two decimals", card.name);
            }
        }
        // No two slots on a Body carry the same four figures.
        let all: Vec<String> = (0..card.colony_slots()).map(|s| g.slot_yields(body, s).text()).collect();
        let mut unique = all.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(unique.len(), all.len(), "{} deals every slot its own figures: {all:?}", card.name);
    }
    // Seed-stable, and a different seed deals a different board.
    let again = with_seed(11);
    let other = with_seed(12);
    for body in BodyId::ALL {
        for slot in 0..g.tables.body(body).colony_slots() {
            assert_eq!(g.slot_yields(body, slot), again.slot_yields(body, slot), "seed 11 deals {body:?} slot {slot} the same twice");
        }
    }
    let same: bool = BodyId::ALL
        .into_iter()
        .all(|b| (0..g.tables.body(b).colony_slots()).all(|s| g.slot_yields(b, s) == other.slot_yields(b, s)));
    assert!(!same, "another seed deals another board");
}

/// Ticket #57 (b): a Module's output is its Colony Slot's yield, not its Body's average. A station
/// in orbit, which stands in no Colony Slot, keeps the Body's figures.
#[test]
fn a_modules_output_uses_its_own_slots_yield() {
    let mut g = with_seed(11);
    let card = g.tables.body(BodyId::Mars).clone();
    // Two Colonies on Mars, in the two slots whose Mine yields are furthest apart.
    let mut slots: Vec<u32> = (0..card.colony_slots()).collect();
    slots.sort_by(|a, b| g.slot_yields(BodyId::Mars, *a).mine.partial_cmp(&g.slot_yields(BodyId::Mars, *b).mine).unwrap());
    let (poor, rich) = (slots[0], *slots.last().unwrap());
    let mine_amount = g.tables.module(ModuleKind::Mine).produces.as_ref().unwrap().amount as f64;
    for slot in [poor, rich] {
        let id = ColonyId(g.fresh_id());
        g.colonies.push(Colony {
            id,
            body: BodyId::Mars,
            slot,
            control: Control::Controlled(Seat(0)),
            modules: vec![Module::new(ModuleKind::Mine)],
            colonists: 0,
            queue: Vec::new(),
            grid_failed: false,
            founded_turn: 1,
            in_orbit: false,
        });
        let want = (mine_amount * g.slot_yields(BodyId::Mars, slot).mine).floor() as i64;
        assert_eq!(
            g.module_yield(Seat(0), id, ModuleKind::Mine).amount,
            want,
            "the Mine at {} makes what its slot's Mine yield x{} says, not the Body's x{}",
            card.slots[slot as usize].name,
            g.slot_yields(BodyId::Mars, slot).mine,
            card.mine_yield
        );
    }
    let by_body = (mine_amount * card.mine_yield).floor() as i64;
    let poor_id = g.colonies[g.colonies.len() - 2].id;
    let rich_id = g.colonies[g.colonies.len() - 1].id;
    let poor_out = g.module_yield(Seat(0), poor_id, ModuleKind::Mine).amount;
    let rich_out = g.module_yield(Seat(0), rich_id, ModuleKind::Mine).amount;
    assert!(poor_out != rich_out || by_body != poor_out, "the two slots do not both read the Body's {by_body}: {poor_out} and {rich_out}");
    // A Habitat's room follows the slot too, and a station over Earth still takes no Body yield.
    g.colony_mut(rich_id).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    let per = g.tables.module(ModuleKind::Habitat).holds_colonists as f64;
    let want = (per * g.slot_yields(BodyId::Mars, rich).habitat).floor() as u32;
    assert_eq!(g.habitat_room(g.colony(rich_id).unwrap()), want, "the Habitat holds what its slot's Habitat yield says");
    let iss = station_of(&g, Seat(0), BodyId::Earth).unwrap();
    g.colony_mut(iss).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    assert_eq!(g.habitat_room(g.colony(iss).unwrap()), per as u32, "a station's Habitats take no Body yield and no slot's either");
}

/// Ticket #57 (c): the game begins on 1 January 2030 and a Turn is a calendar month.
#[test]
fn a_turn_is_a_calendar_month_from_january_2030() {
    let g = game();
    assert_eq!(g.date(1), Date { year: 2030, month: 1 });
    assert_eq!(g.date(1).text(), "January 2030");
    assert_eq!(g.date(7).text(), "July 2030");
    assert_eq!(g.date(12).text(), "December 2030");
    assert_eq!(g.date(13).text(), "January 2031");
    assert_eq!(g.date(24), Date { year: 2031, month: 12 }, "the last turn is December 2031");
    assert_eq!(g.date(24).text(), "December 2031");
    // Turn 1 is exactly 2030-01-01 00:00 UTC, the moment the game begins.
    assert_eq!(g.julian_day(1), 2462502.5);
}

/// Ticket #57 (d): the sky the game draws is the real one. Earth's and Mars's heliocentric ecliptic
/// longitudes on 2030-01-01 against JPL Horizons (DE441, heliocentric J2000 ecliptic), recorded in
/// `docs/research/earth-mars-ephemeris.md` section 2.3.
#[test]
fn earth_and_mars_stand_where_jpl_horizons_puts_them_on_the_first_of_january_2030() {
    let g = game();
    // Horizons, 2030-01-01 00:00:00 TDB = JD 2462502.5.
    const EARTH: f64 = 100.1845;
    const MARS: f64 = 337.8203;
    let earth = g.heliocentric_longitude(BodyId::Earth, 1);
    let mars = g.heliocentric_longitude(BodyId::Mars, 1);
    assert!((earth - EARTH).abs() < 1.0, "Earth at {earth:.4} against Horizons' {EARTH}");
    assert!((mars - MARS).abs() < 1.0, "Mars at {mars:.4} against Horizons' {MARS}");
    // The phase angle the whole launch-window rule reads: Mars trails Earth by about 122 degrees.
    let phase = g.phase_angle(1);
    assert!((phase + 122.36).abs() < 1.0, "the phase angle at the start is {phase:.4}, not about -122.36");
    // The Moon stands where Earth stands and the moons of Mars where Mars stands.
    assert_eq!(g.heliocentric_longitude(BodyId::Moon, 1), earth);
    assert_eq!(g.heliocentric_longitude(BodyId::Phobos, 1), mars);
}

/// Ticket #57 (e): the first Mars launch window after January 2030 falls where the real one does.
/// The research file (section 3.3) puts the 2031 Type I optimum departure at 28 January 2031, which
/// is turn 13; the game's own window turn must be within one turn of it.
#[test]
fn the_first_mars_window_falls_where_the_real_one_of_early_2031_does() {
    let g = game();
    let window = g.next_window_turn(1);
    assert!((12..=14).contains(&window), "the window is turn {window} ({}), not within one turn of January 2031", g.date(window).text());
    assert_eq!(g.date(window).year, 2031, "and it is in 2031");
    // It really is the smallest offset in the span, and no other turn is nearer.
    let offset = g.window_offset(window).abs();
    assert!(offset < 15.0, "the window turn stands {offset:.1} degrees off the Hohmann angle");
    for t in 1..=g.tables.victory.turns {
        if t != window {
            assert!(g.window_offset(t).abs() >= offset, "turn {t} is no nearer the window than turn {window}");
        }
    }
    // One window in the whole game: the next is a synodic period away, past the last turn.
    let after = g.next_window_turn(window + 1);
    assert!(after > g.tables.victory.turns, "the second window is turn {after}, inside the game's {} turns", g.tables.victory.turns);
}

/// Ticket #57 (f): what a crossing between the Earth system and the Mars system costs. At the
/// window it is the Hohmann flight and the card's Fuel; away from it both rise; the flight is never
/// longer than the cap; and Efficient Transit applies after the window factor, not before.
#[test]
fn a_crossing_costs_the_hohmann_flight_at_the_window_and_more_away_from_it() {
    let mut g = game();
    let tr = g.tables.transit.clone();
    let card_fuel = g.tables.body(BodyId::Mars).transit_fuel;
    let window = g.next_window_turn(1);
    let hohmann = (tr.days_at_window / tr.days_per_turn).ceil() as u32;
    assert_eq!(hohmann, 9, "259 days is nine turns of thirty");
    assert_eq!(
        g.transit_cost_at(BodyId::Earth, BodyId::Mars, window),
        (hohmann, card_fuel),
        "at the window: the Hohmann flight for the card's Fuel"
    );
    // Far from it, dearer in both.
    let cycle = (tr.synodic_days / tr.days_per_turn).ceil() as u32 + 1;
    let far = (1..=cycle).max_by(|a, b| g.window_offset(*a).abs().partial_cmp(&g.window_offset(*b).abs()).unwrap()).unwrap();
    let (far_turns, far_fuel) = g.transit_cost_at(BodyId::Earth, BodyId::Mars, far);
    assert!(far_turns > hohmann, "the worst turn takes {far_turns} turns, no more than the window's {hohmann}");
    assert!(far_fuel > card_fuel, "and costs {far_fuel} Fuel, no more than the card's {card_fuel}");
    assert_eq!(far_turns, tr.max_turns, "the worst turn reaches the cap of {} turns", tr.max_turns);
    // The cap holds over a whole synodic cycle, and nothing ever falls under the card's Fuel.
    for t in 1..=cycle {
        let (turns, fuel) = g.transit_cost_at(BodyId::Earth, BodyId::Mars, t);
        assert!(turns <= tr.max_turns, "turn {t} takes {turns} turns, past the cap of {}", tr.max_turns);
        assert!(fuel >= card_fuel, "turn {t} costs {fuel} Fuel, under the card's {card_fuel}");
    }
    // Efficient Transit multiplies what the window has already made of the Fuel, not the card.
    g.turn = far;
    let plain = g.transit_cost(BodyId::Earth, BodyId::Mars).1;
    with_tech(&mut g, TechId::EfficientTransit);
    let cut = g.transit_cost(BodyId::Earth, BodyId::Mars).1;
    let value = g.tables.tech(TechId::EfficientTransit).value;
    assert_eq!(cut, (plain as f64 * value).floor() as i64, "{plain} x {value} rounded down, not the card's {card_fuel} x {value}");
    assert!(cut > (card_fuel as f64 * value).floor() as i64, "the window factor came first: {cut}");
}

/// Ticket #57 (g): the launch window touches only a crossing between the two systems. A hop inside
/// the Earth system or inside the Mars system costs what its card always said, on every turn.
#[test]
fn earth_moon_and_mars_system_hops_are_untouched_by_the_window() {
    let mut g = game();
    let cycle = (g.tables.transit.synodic_days / g.tables.transit.days_per_turn).ceil() as u32 + 1;
    let hops = [
        ((BodyId::Earth, BodyId::Moon), (1u32, 6i64)),
        ((BodyId::Moon, BodyId::Earth), (1, 6)),
        ((BodyId::Mars, BodyId::Phobos), (1, 2)),
        ((BodyId::Phobos, BodyId::Mars), (1, 2)),
        ((BodyId::Phobos, BodyId::Deimos), (1, 1)),
        ((BodyId::Deimos, BodyId::Phobos), (1, 1)),
    ];
    for t in 1..=cycle {
        g.turn = t;
        for ((from, to), want) in hops {
            assert!(g.crossing_offset(from, to, t).is_none(), "{from:?} to {to:?} crosses nothing");
            assert_eq!(g.transit_cost(from, to), want, "{from:?} to {to:?} on turn {t}");
        }
    }
}

/// Ticket #57 (h): with the Mars launch window two turns away or less, the AI banks Fuel for the
/// crossing it wants and spends none on anything else, as it banks Materials for a build.
#[test]
fn the_ai_banks_fuel_when_the_mars_window_is_within_two_turns() {
    // A Colony Ship loaded at Earth wanting Mars (every Moon slot is taken, so Mars is the only
    // Body with room), and a Frigate at Earth that would otherwise hop to the Moon: only one of
    // the two may burn Fuel while the window is near.
    let board = |turn: u32| {
        let mut g = game();
        g.turn = turn;
        let seat = Seat(0);
        while !g.free_slots_on(BodyId::Moon).is_empty() {
            colony(&mut g, seat, BodyId::Moon, &[ModuleKind::Habitat], 2);
        }
        g.seats[seat.index()].stockpile.fuel = 200;
        g.seats[seat.index()].stockpile.materials = 0;
        g.seats[seat.index()].stockpile.energy = 200;
        for kind in [UnitKind::ColonyShip, UnitKind::Frigate] {
            let id = ShipId(g.fresh_id());
            let colonists = if kind == UnitKind::ColonyShip { 4 } else { 0 };
            g.ships.push(Ship {
                id,
                kind,
                seat,
                damage: 0,
                at: ShipAt::Body(BodyId::Earth),
                colonists,
                army: None,
                stance: Stance::Hold,
                escaped: false,
                arrived_this_turn: false,
                built_turn: 1,
            });
        }
        g
    };
    let window = game().next_window_turn(1);
    // Two turns out: the bank is on.
    let mut near = board(window - 2);
    let orders = near.ai_orders(Seat(0));
    let lines: Vec<String> = near.log.to_vec();
    assert!(lines.iter().any(|l| l.contains("banking Fuel for")), "the window is two turns off, so Fuel is banked: {lines:#?}");
    let crossings = orders
        .iter()
        .filter(|o| matches!(o, Order::Transit { to, .. } if near.crossing_offset(BodyId::Earth, *to, near.turn).is_some()))
        .count();
    let spends: Vec<&Order> = orders.iter().filter(|o| near.order_cost(Seat(0), o).fuel > 0).collect();
    assert_eq!(spends.len(), crossings, "nothing but the crossing it is banking for burns Fuel: {spends:?}");
    // Three turns out, and the AI spends Fuel as it always did.
    let mut off = board(window - 3);
    off.ai_orders(Seat(0));
    let lines: Vec<String> = off.log.to_vec();
    assert!(!lines.iter().any(|l| l.contains("banking Fuel for")), "three turns out the bank is off: {lines:#?}");
}

/// Ticket #57: a loaded Colony Ship weighs a Body by what its slot is worth less the share of the
/// game the flight would eat, so off the window the Moon, one turn away, beats a Mars that is
/// seventeen turns away; before this the AI was only ever offered the single best Body.
#[test]
fn a_loaded_colony_ship_goes_to_the_moon_when_mars_is_a_year_away() {
    let mut g = game();
    let cust = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Custodians).unwrap();
    g.turn = 4; // ten turns short of the window: the Mars flight is seventeen turns
    let (mars_turns, _) = g.transit_cost_for(cust, BodyId::Earth, BodyId::Mars);
    assert!(mars_turns >= 12, "off the window Mars is far: {mars_turns} turns");
    let ship = ShipId(900);
    g.ships.push(Ship { id: ship, kind: UnitKind::ColonyShip, seat: cust, damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 4, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1 });
    g.seats[cust.index()].stockpile.fuel = 100;
    g.seats[cust.index()].stockpile.energy = 200;
    let orders = g.ai_orders(cust);
    let dest = orders.iter().find_map(|o| match o {
        Order::Transit { ship: s, to } if *s == ship => Some(*to),
        _ => None,
    });
    assert_eq!(dest, Some(BodyId::Moon), "the Moon, not a year-long flight: {orders:?}");
}


// ---------------------------------------------------------------- Ticket #58: the turn as a story

/// A Colony Ship of seat 0's standing at `body` with `colonists` aboard, and the Unload order that
/// founds a Colony into that Body's first free slot.
fn colony_ship_ready(g: &mut Game, body: BodyId) -> (ShipId, Order) {
    let id = ShipId(g.fresh_id());
    let turn = g.turn;
    g.ships.push(Ship {
        id,
        kind: UnitKind::ColonyShip,
        seat: Seat(0),
        damage: 0,
        at: ShipAt::Body(body),
        colonists: 4,
        army: None,
        stance: Stance::Hold,
        escaped: false,
        arrived_this_turn: false,
        built_turn: turn,
    });
    let slot = g.free_slots_on(body)[0];
    (id, Order::Unload { ship: id, colonists: 4, army: false, into: UnloadTarget::Slot(body, slot) })
}

/// (a) The headline follows the severity order: a turn that completed a Tech and founded a Colony
/// headlines the Colony, whichever of the two the engine wrote down first.
#[test]
fn the_headline_takes_the_most_severe_line_whatever_order_it_came_in() {
    let mut g = game();
    calm(&mut g);
    let (_, found) = colony_ship_ready(&mut g, BodyId::Moon);
    let mut orders: [Vec<Order>; SEAT_COUNT] = std::array::from_fn(|_| Vec::new());
    orders[0] = vec![found];
    g.end_turn(orders);
    let founded = g.report.lines.iter().position(|l| l.kind == LineKind::ColonyFounded).expect("a Colony was founded");
    // The Tech completes after the founding, so only the severity order can put the Colony first.
    g.research.current = Some(TechId::EfficientGrids);
    let cost = g.tables.tech(TechId::EfficientGrids).cost;
    g.accrue_research(Seat(0), cost);
    let tech = g.report.lines.iter().position(|l| l.kind == LineKind::TechComplete).expect("a Tech completed");
    assert!(tech > founded, "the Tech line was written after the founding: {tech} > {founded}");
    let head = g.report.headline().expect("a headline");
    assert_eq!(head.kind, LineKind::ColonyFounded, "the Colony headlines over the Tech: {}", head.text);
    // And the ranks are the order the ticket set, all eight of them.
    let ranks: Vec<Option<u8>> = [
        LineKind::ColonyFounded,
        LineKind::ControlChanged,
        LineKind::Break,
        LineKind::SeaLevel,
        LineKind::DecisiveBattle,
        LineKind::Occupation,
        LineKind::TechComplete,
        LineKind::Event,
        LineKind::BuildComplete,
    ]
    .iter()
    .map(|k| k.headline_rank())
    .collect();
    assert_eq!(ranks, vec![Some(1), Some(2), Some(3), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8)]);
}

/// (b) Every line carries its kind and its place, and the four headings take them by both.
#[test]
fn every_report_line_carries_its_kind_and_place_and_falls_under_the_right_heading() {
    let mut g = game();
    calm(&mut g);
    // The player's own Factory completes this turn, a Colony is founded at the Moon, and the
    // Climate phase that opens the next turn fires every Break at the Temperature.
    let turn = g.turn;
    g.state_mut(StateId::EastAsia).queue.push(Build {
        item: BuildItem::Facility(FacilityKind::Factory),
        seat: Seat(0),
        due_turn: turn,
        coastal: false,
    });
    let (_, found) = colony_ship_ready(&mut g, BodyId::Moon);
    hold_temperature(&mut g, 1.7);
    breaks_ahead(&mut g);
    let mut orders: [Vec<Order>; SEAT_COUNT] = std::array::from_fn(|_| Vec::new());
    orders[0] = vec![found];
    g.end_turn(orders);

    let founded = g.report.lines.iter().find(|l| l.kind == LineKind::ColonyFounded).expect("a Colony was founded");
    assert!(matches!(founded.place, Some(ReportPlace::Colony(_))), "the founding points at its Colony: {:?}", founded.place);
    assert_eq!(founded.section(), Section::InSpace, "a founding is In space");

    let broke = g.report.lines.iter().find(|l| l.kind == LineKind::Break).expect("a Break fired at +1.7 C");
    assert_eq!(broke.section(), Section::TheClimate, "a Break is The climate");

    let mine = g
        .report
        .lines
        .iter()
        .find(|l| l.kind == LineKind::YourBuild && l.text.contains("Factory"))
        .expect("the player's Factory completed");
    assert_eq!(mine.section(), Section::YourWorks, "the player's build is Your works");
    assert!(matches!(mine.place, Some(ReportPlace::State(StateId::EastAsia))), "and points at East Asia: {:?}", mine.place);

    // A rival's build at a Nation State is On Earth, not Your works.
    assert_eq!(LineKind::BuildComplete.section(Some(ReportPlace::State(StateId::Europe))), Section::OnEarth);
    assert_eq!(LineKind::BuildComplete.section(Some(ReportPlace::Body(BodyId::Mars))), Section::InSpace);
    assert_eq!(LineKind::Unrest.section(Some(ReportPlace::State(StateId::Europe))), Section::OnEarth);

    // The headings partition the report: every line but the headline appears exactly once.
    let grouped: usize = g.report.sections().iter().map(|(_, l)| l.len()).sum();
    assert_eq!(grouped, g.report.lines.len() - 1, "every line but the headline is under a heading");
    let names: Vec<&str> = g.report.sections().iter().map(|(s, _)| s.name()).collect();
    let wanted: Vec<&str> = ["In space", "On Earth", "The climate", "Your works"].into_iter().filter(|n| names.contains(n)).collect();
    assert_eq!(names, wanted, "the headings come in their fixed order and empty ones are left out");
}

/// (c) A rival's paragraph names every visible order it committed, and carries none of the scores,
/// waits and skips the AI chose from.
#[test]
fn a_rivals_paragraph_names_its_visible_orders_and_none_of_its_scores() {
    let mut g = game();
    let seat = Seat(1);
    let colony = colony(&mut g, seat, BodyId::Mars, &[ModuleKind::Habitat, ModuleKind::Shipyard], 4);
    let ship = ShipId(g.fresh_id());
    let turn = g.turn;
    g.ships.push(Ship {
        id: ship,
        kind: UnitKind::ColonyShip,
        seat,
        damage: 0,
        at: ShipAt::Body(BodyId::Earth),
        colonists: 0,
        army: None,
        stance: Stance::Hold,
        escaped: false,
        arrived_this_turn: false,
        built_turn: turn,
    });
    // One order of every visible kind: each must have its own sentence.
    let every: Vec<Order> = vec![
        Order::BuildFacility { state: StateId::EastAsia, kind: FacilityKind::Factory },
        Order::BuildFacilityWithDucats { state: StateId::EastAsia, kind: FacilityKind::Bank },
        Order::RaiseIndustry { state: StateId::EastAsia },
        Order::BuildModule { colony, kind: ModuleKind::Mine },
        Order::BuildModuleWithDucats { colony, kind: ModuleKind::Relay },
        Order::BuildShip { site: Place::Colony(colony), kind: UnitKind::Frigate },
        Order::BuildArmy { place: Place::State(StateId::EastAsia) },
        Order::BuildStation { body: BodyId::Mars, slot: 0 },
        Order::BuildArchiveStage { colony },
        Order::FundArchive,
        Order::Repair { unit: UnitRef::Ship(ship), points: 1 },
        Order::RepairWithDucats { unit: UnitRef::Ship(ship), points: 1 },
        Order::Transit { ship, to: BodyId::Moon },
        Order::ShipStance { body: BodyId::Earth, stance: Stance::Hold },
        Order::ArmyStance { place: Place::State(StateId::EastAsia), stance: Stance::Hold },
        Order::MoveArmy { army: ArmyId(0), to: StateId::Europe },
        Order::Load { ship, colonists: 2, from: LoadSource::State(StateId::EastAsia), army: None },
        Order::Load { ship, colonists: 0, from: LoadSource::State(StateId::EastAsia), army: Some(ArmyId(0)) },
        Order::Unload { ship, colonists: 2, army: false, into: UnloadTarget::Slot(BodyId::Moon, 0) },
        Order::Influence { target: Place::State(StateId::Europe), amount: 5 },
        Order::BuyInfluence { amount: 3 },
        Order::Buy { resource: Resource::Materials, amount: 4 },
        Order::Sell { resource: Resource::Fuel, amount: 4 },
        Order::Relief { state: StateId::EastAsia },
        Order::Resettle { state: StateId::EastAsia },
        Order::Change { building: BuildingRef::Facility(StateId::EastAsia, 0), what: BuildingChange::Mothball },
        Order::Change { building: BuildingRef::Facility(StateId::EastAsia, 0), what: BuildingChange::Restart },
        Order::Change { building: BuildingRef::Module(colony, 0), what: BuildingChange::Decommission },
        Order::Leapfrog { state: StateId::EastAsia },
        Order::StripPermit { state: StateId::EastAsia },
    ];
    for o in &every {
        let deed = g.rival_deed(seat, o);
        assert!(deed.is_some(), "no sentence for {o:?}");
        let text = deed.unwrap();
        assert!(!text.starts_with('['), "{o:?} fell through to a missing template: {text}");
    }

    // And a real AI turn's paragraph says what it did, with none of the scored list in it.
    let mut g = game();
    g.end_turn(std::array::from_fn(|_| Vec::new()));
    let entry = g.report.ai_lines.iter().find(|e| e.seat == Seat(1)).expect("the Prospectors ordered");
    assert!(!entry.deeds.is_empty(), "and the Report keeps what they did");
    let para = g.rival_paragraph(Seat(1)).expect("a paragraph");
    for deed in &entry.deeds {
        assert!(para.contains(deed.as_str()), "the paragraph names {deed:?}: {para}");
    }
    for marker in ["  take", "  skip", "  wait", "  save", "[", "Category"] {
        assert!(!para.contains(marker), "no {marker:?} from the AI's scored list: {para}");
    }
    // The scored list is still in the log, where the simulate run reads it.
    assert!(g.log.iter().any(|l| l.trim_start().starts_with("take")), "the scored list stays in the log");
}

/// (d) At most two Moments a turn, the most severe first, and a kind switched off is skipped.
#[test]
fn moments_are_capped_at_two_a_turn_most_severe_first_and_a_switched_off_kind_is_skipped() {
    let mut g = game();
    g.report = Report::default();
    // Written down in the least severe order, so only the severity order can sort them.
    g.moment(MomentKind::ArchiveComplete, &[("faction", "the Archivists".into()), ("place", "Tycho".into()), ("stages", "3".into())], None);
    g.moment(MomentKind::TechComplete, &[("tech", "Deep Mining".into()), ("faction", "the Prospectors".into()), ("lead", "9".into()), ("cost", "15".into())], None);
    g.moment(MomentKind::ControlChanged, &[("place", "Europe".into()), ("faction", "the Prospectors".into())], None);
    g.moment(MomentKind::ColonyFounded, &[("faction", "the Custodians".into()), ("colony", "Tycho".into()), ("note", "their first".into()), ("n", "4".into())], None);
    assert_eq!(g.report.moments.len(), 4, "every Moment the turn earned is kept");

    let all_on = |_: MomentKind| true;
    let shown = g.report.moments_shown(&all_on);
    assert_eq!(shown.len(), 2, "at most two a turn");
    assert_eq!(shown[0].kind, MomentKind::ColonyFounded, "the founding is the most severe");
    assert_eq!(shown[1].kind, MomentKind::ControlChanged, "then the change of control");

    let without_founding = |k: MomentKind| k != MomentKind::ColonyFounded;
    let shown = g.report.moments_shown(&without_founding);
    assert_eq!(shown.len(), 2);
    assert_eq!(shown[0].kind, MomentKind::ControlChanged, "a switched-off kind is skipped, not shown");
    assert_eq!(shown[1].kind, MomentKind::TechComplete);

    let none = |_: MomentKind| false;
    assert!(g.report.moments_shown(&none).is_empty(), "every kind off is no Moments at all");
    // Every kind has a default in report.toml.
    for k in MomentKind::ALL {
        assert!(g.tables.report.moment(k).is_some(), "{k:?} has a [moments] table");
    }
}

/// (e) The Tech Moment names the Lead and the margin, and says what the AI picked and why.
#[test]
fn the_tech_moment_names_the_lead_the_margin_and_the_ai_pick() {
    let mut g = game();
    g.report = Report::default();
    let tech = TechId::EfficientGrids;
    let cost = g.tables.tech(tech).cost;
    g.research.current = Some(tech);
    g.research.progress = 0;
    g.research.contributions = [0; SEAT_COUNT];
    g.accrue_research(Seat(1), cost);
    let m = g.report.moments.iter().find(|m| m.kind == MomentKind::TechComplete).expect("a Tech Moment");
    assert!(m.text.contains("Efficient Grids"), "it names the Tech: {}", m.text);
    assert!(m.text.contains(&g.seat_name(Seat(1))), "it names the Lead: {}", m.text);
    assert!(m.text.contains(&format!("{cost} of {cost}")), "it gives the margin: {}", m.text);
    assert_eq!(m.figure, format!("{cost} of {cost}"), "the number is the margin");
    assert_eq!(m.tech, Some(tech), "and the Moment knows which boxes to light");
    let note = m.note.as_ref().expect("the AI's pick line");
    let picked = g.research.current.expect("the AI picked at once");
    assert!(note.contains(&g.tables.tech(picked).name), "the note names the pick: {note}");
    assert!(
        note.contains("first choice") || note.contains("cheapest left") || note.contains("until last"),
        "and why it was picked: {note}"
    );

    // When the player leads, the Moment hands the pick back instead.
    let mut g = game();
    g.report = Report::default();
    g.research.current = Some(TechId::CleanPropellant);
    g.research.progress = 0;
    g.research.contributions = [0; SEAT_COUNT];
    g.accrue_research(Seat(0), g.tables.tech(TechId::CleanPropellant).cost);
    let m = g.report.moments.iter().find(|m| m.kind == MomentKind::TechComplete).expect("a Tech Moment");
    assert_eq!(g.research.awaiting_pick, Some(Seat(0)), "the player picks");
    assert_eq!(m.note.as_deref(), Some("You led. Pick the next Tech."));
}

/// (f) Every template in report.toml has its placeholders satisfied: every key the engine asks for
/// is there, nothing else is, and no template uses a placeholder the engine does not supply.
#[test]
fn every_template_in_report_toml_has_its_placeholders_satisfied() {
    let t = tables();
    assert!(t.report.check().is_ok(), "the shipped file passes: {:?}", t.report.check());

    // A placeholder the engine never supplies is refused, by name.
    let mut bad = t.report.clone();
    bad.line.insert("colony_founded".into(), "The {faction} founded {nonesuch}.".into());
    let e = bad.check().unwrap_err();
    assert!(e.contains("nonesuch") && e.contains("colony_founded"), "it names the placeholder and the key: {e}");

    // A missing key is refused.
    let mut bad = t.report.clone();
    bad.line.remove("colony_founded");
    assert!(bad.check().unwrap_err().contains("colony_founded"), "a missing sentence is named");

    // A key the engine never asks for is refused, so a template cannot rot unread.
    let mut bad = t.report.clone();
    bad.rival.insert("nobody_asks".into(), "did something".into());
    assert!(bad.check().unwrap_err().contains("nobody_asks"), "an unread sentence is named");

    // The same for a Moment's figure.
    let mut bad = t.report.clone();
    let mut card = bad.moments.get("antarctica").unwrap().clone();
    card.figure = "{slots} Colony Slots".into();
    bad.moments.insert("antarctica".into(), card);
    assert!(bad.check().unwrap_err().contains("slots"), "a Moment's figure is checked too");

    // And rendering leaves nothing standing: every placeholder in every [line] template is filled
    // when the engine supplies the arguments it declares for that key.
    for (key, args) in dying_earth_engine::report::LINE_ARGS {
        let filled: Vec<(&str, String)> = args.iter().map(|a| (*a, format!("<{a}>"))).collect();
        let text = t.report.line(key, &filled);
        assert!(!text.contains('{'), "{key} still has a placeholder standing: {text}");
    }
}

/// (g) The first Report keeps its explanation as the headline.
#[test]
fn the_first_reports_headline_is_the_seating_explanation() {
    let mut g = with_seed(7);
    g.start();
    let head = g.report.headline().expect("a headline on turn 1");
    assert_eq!(head.kind, LineKind::Seating);
    assert_eq!(
        head.text,
        "January 2030. You play the Custodians from East Asia; the computer plays the Prospectors, the Arkwrights and the Archivists."
    );
    // It headlines over everything else the first turn wrote down.
    assert!(g.report.lines.len() > 1, "and there are other lines under it");
    assert_eq!(g.report.headline_index(), Some(0));
}

// ---------------------------------------------------------------- Ticket #64: spectator mode

/// (a) A game nobody sits at: every seat is the computer's, End Turn with no orders runs all four
/// and advances, and seat 0's start is the spreading rule's first pick rather than a continent
/// anybody handed in.
#[test]
fn a_spectated_game_seats_four_computers_and_deals_seat_zero_by_the_spreading_rule() {
    let t = tables();
    let mut g = Game::spectate(t.clone(), 7);
    g.start();
    assert!(g.spectator, "the game knows nobody is sitting at it");
    // Seat 0 holds the Custodians, so the seating reads as it does in simulate mode.
    assert_eq!(g.kind(Seat(0)), FactionKind::Custodians);
    for seat in Seat::ALL {
        assert!(g.seat(seat).ai, "{seat:?} is played by the computer");
    }
    // The four starts are the spreading rule's own four picks, taken from an empty table.
    let mut taken: Vec<StateId> = Vec::new();
    for _ in Seat::ALL {
        let pick = t.ai_start_state(&taken);
        taken.push(pick);
    }
    assert_eq!(taken, vec![StateId::EastAsia, StateId::Europe, StateId::Australia, StateId::SubSaharanAfrica]);
    for seat in Seat::ALL {
        assert_eq!(g.controlled_states(seat), vec![taken[seat.index()]], "{seat:?} starts where the rule put it");
    }
    // And not where a handed-in start would have put it: the same seed with a player start of South
    // America seats the Custodians there, where the spectated game seats them in East Asia.
    let handed = Game::new(t.clone(), NewGame { seed: 7, player: FactionKind::Custodians, player_is_ai: true, player_start: StateId::SouthAmerica });
    assert_eq!(handed.controlled_states(Seat(0)), vec![StateId::SouthAmerica], "a handed-in start is obeyed");
    assert_ne!(g.controlled_states(Seat(0)), handed.controlled_states(Seat(0)), "the spectated game took no handed-in start");
    // End Turn with empty orders runs all four seats and advances the turn.
    let turn = g.turn;
    g.end_turn(std::array::from_fn(|_| Vec::new()));
    assert_eq!(g.turn, turn + 1, "the turn advanced");
    assert_eq!(g.report.ai_lines.len(), SEAT_COUNT, "all four seats ordered");
    for (i, entry) in g.report.ai_lines.iter().enumerate() {
        assert_eq!(entry.seat, Seat(i as u8), "in seat order");
    }
}

/// (b) The spectator's dispatch: the heading that carried one seat's works is named "Builds and
/// works" and carries every seat's, and the Report ends on four paragraphs, seat 0 included.
#[test]
fn the_spectators_dispatch_carries_every_factions_works_and_all_four_paragraphs() {
    // The heading, and the routing rule under it.
    assert_eq!(Section::YourWorks.name_for(false), "Your works");
    assert_eq!(Section::YourWorks.name_for(true), "Builds and works");
    for seat in Seat::ALL {
        assert_eq!(dying_earth_engine::report::line_kind_of(seat, LineKind::YourBuild, LineKind::BuildComplete, true), LineKind::YourBuild, "{seat:?}'s build is the spectator's business");
    }
    assert_eq!(dying_earth_engine::report::line_kind_of(Seat(0), LineKind::YourBuild, LineKind::BuildComplete, false), LineKind::YourBuild);
    assert_eq!(dying_earth_engine::report::line_kind_of(Seat(2), LineKind::YourBuild, LineKind::BuildComplete, false), LineKind::BuildComplete);
    assert_eq!(LineKind::YourBuild.section(Some(ReportPlace::State(StateId::Europe))), Section::YourWorks, "and it lands under that heading");

    // On a real board: run the game until a seat other than 0 completes a build, and find its line
    // under the works heading rather than out on the board.
    let t = tables();
    let mut g = Game::spectate(t.clone(), 7);
    g.start();
    let mut found: Option<(Seat, String)> = None;
    for _ in 0..12 {
        g.end_turn(std::array::from_fn(|_| Vec::new()));
        let works: Vec<&ReportLine> = g.report.sections().into_iter().find(|(s, _)| *s == Section::YourWorks).map(|(_, l)| l).unwrap_or_default();
        for seat in Seat::ALL.into_iter().skip(1) {
            let name = g.seat_name(seat);
            if let Some(l) = works.iter().find(|l| l.text.contains(&name)) {
                found = Some((seat, l.text.clone()));
            }
        }
        if found.is_some() {
            break;
        }
    }
    let (seat, text) = found.expect("a rival seat's works under the spectator's works heading");
    assert!(seat != Seat(0) && text.contains(&g.seat_name(seat)), "it names the Faction whose work it is: {text}");

    // And the paragraphs: four of them, in seat order, seat 0 among them.
    let paragraphs = g.faction_paragraphs();
    assert_eq!(paragraphs.len(), SEAT_COUNT, "four paragraphs");
    for (i, (s, text)) in paragraphs.iter().enumerate() {
        assert_eq!(*s, Seat(i as u8), "in seat order");
        assert!(text.contains(&g.seat_name(*s)), "each names its Faction: {text}");
    }
    // A player's game still tells only the three rivals.
    let mut p = with_seed(7);
    p.start();
    p.end_turn(std::array::from_fn(|_| Vec::new()));
    assert!(p.faction_paragraphs().iter().all(|(s, _)| *s != Seat(0)), "a player's Report keeps seat 0 out of the rivals");
}
// ---------------------------------------------------------------- Ticket #60: four small fixes

/// A Wildfire is the world's carbon wherever it burns, so the Emissions it leaves stand on the
/// cards line whether or not a Faction directs the state. Until this ticket the line sat inside the
/// directed branch of `emissions_now` -- an accident of where ticket #24's `continue` landed -- so
/// a Wildfire on a state nobody holds charged nothing at all.
#[test]
fn a_wildfire_on_a_neutral_nation_state_charges_its_emissions_to_the_cards_line() {
    let mut g = game();
    calm(&mut g);
    let sid = StateId::NorthAfrica;
    assert!(g.state(sid).control.director().is_none(), "North Africa is neutral at the start");
    let before = g.emissions_now();
    g.last_event = Some(DrawnEvent { card: Card::Event(EventId::Wildfire), target: EventTarget::State(sid), scale: 1.0, text: String::new() });
    g.apply_event_now();
    let after = g.emissions_now();
    assert_eq!(after.cards - before.cards, g.tables.events.wildfire_emissions, "a Wildfire on a neutral state adds its Emissions to the cards line");
    // And it stays the world's doing: nobody's Blame.
    assert_eq!(after.by_seat, before.by_seat, "an Event card is nobody's Blame, neutral state or not");
    // A Wildfire on a directed state charges the same line, as it always has.
    let mut g = game();
    calm(&mut g);
    g.take_control(sid, Seat(1));
    let before = g.emissions_now().cards;
    g.last_event = Some(DrawnEvent { card: Card::Event(EventId::Wildfire), target: EventTarget::State(sid), scale: 1.0, text: String::new() });
    g.apply_event_now();
    assert_eq!(g.emissions_now().cards - before, g.tables.events.wildfire_emissions, "and a directed state is unchanged");
}

/// Ticket #41 put the challenge margin on a held place; the figure the state card printed for
/// "N now" kept ticket #33's `controller + 1`, so the card asked for less than the Resolution
/// would honour. `influence_needed_for` is the one computation all three read.
#[test]
fn the_figure_for_taking_a_held_place_includes_the_challenge_margin() {
    let mut g = game();
    let target = Place::State(StateId::NorthAfrica);
    let margin = g.tables.influence.challenge_margin;
    g.take_control(StateId::NorthAfrica, Seat(1));
    let threshold = g.influence_threshold_for(Seat(0), target);
    // The holder standing at the threshold: a challenger needs its Standing plus the margin.
    g.seats[1].influence.insert(target, threshold);
    assert_eq!(g.influence_needed_for(Seat(0), target), threshold + margin, "the holder's Standing plus the challenge margin");
    // A holder with almost nothing: the threshold is the greater, and the figure is the threshold.
    g.seats[1].influence.insert(target, 1);
    assert_eq!(g.influence_needed_for(Seat(0), target), threshold, "the threshold, where it is the greater");
    // A neutral place asks the threshold and nothing else.
    let neutral = Place::State(StateId::SouthAsia);
    assert_eq!(g.influence_needed_for(Seat(0), neutral), g.influence_threshold_for(Seat(0), neutral), "a neutral place asks the threshold");
    // And it is the figure the Resolution actually applies: one short takes nothing.
    let mut g = game();
    g.take_control(StateId::NorthAfrica, Seat(1));
    g.seats[1].influence.insert(target, g.influence_threshold_for(Seat(0), target));
    let need = g.influence_needed_for(Seat(0), target);
    let hold = |g: &mut Game| {
        for s in Seat::ALL {
            g.seats[s.index()].influenced_this_turn.push(target);
        }
    };
    g.seats[0].influence.insert(target, need - 1);
    hold(&mut g);
    g.resolution_phase();
    assert_eq!(g.state(StateId::NorthAfrica).control.controller(), Some(Seat(1)), "one short of the figure takes nothing");
    g.seats[0].influence.insert(target, need);
    hold(&mut g);
    g.resolution_phase();
    assert_eq!(g.state(StateId::NorthAfrica).control.controller(), Some(Seat(0)), "the figure itself takes the place");
}

/// Tickets #52 and #53 measured no Constabulary in any AI game: a building that fixes nothing
/// economic never beat a producer under the victory-gap multiplier, so it never won a build slot.
/// From Unrest 5 it takes that multiplier too, because a state at 7 halves every Facility there.
#[test]
fn a_custodian_ai_behind_on_pace_builds_a_constabulary_where_unrest_has_reached_seven() {
    let mut g = game();
    calm(&mut g);
    let seat = Seat(0);
    assert_eq!(g.kind(seat), FactionKind::Custodians);
    assert_eq!(g.state(StateId::EastAsia).control.controller(), Some(seat), "seat 0 starts in East Asia");
    // Behind on its pace: turn 14 with no Stabilization run at all, which is every game's shape.
    g.turn = 14;
    g.seats[seat.index()].stabilization_run = 0;
    // Materials for three builds and no more, so the scored list has to RANK the Constabulary above
    // the cheap economic answers rather than reach it once everything else is bought: at its bare
    // weight of 6 it sits under the Research Lab's 8 and is never reached.
    g.seats[seat.index()].stockpile.materials = 70;
    g.seats[seat.index()].stockpile.energy = 100;
    g.state_mut(StateId::EastAsia).unrest = 7.0;
    assert!(g.free_slots(StateId::EastAsia) > 0, "a free slot to build it in");
    let orders = g.ai_orders(seat);
    let scored: Vec<String> = g.log.iter().filter(|&l| l.contains("take") || l.contains("skip")).take(12).cloned().collect();
    assert!(
        orders.iter().any(|o| matches!(o, Order::BuildFacility { state: StateId::EastAsia, kind: FacilityKind::Constabulary })),
        "no Constabulary where Unrest has reached 7: {orders:?}\nscored: {scored:#?}"
    );
    // And a calm state still gets none.
    let mut g = game();
    calm(&mut g);
    g.turn = 14;
    g.seats[seat.index()].stockpile.materials = 400;
    g.seats[seat.index()].stockpile.energy = 100;
    let orders = g.ai_orders(seat);
    assert!(
        !orders.iter().any(|o| matches!(o, Order::BuildFacility { kind: FacilityKind::Constabulary, .. })),
        "a Constabulary in a calm state: {orders:?}"
    );
}

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
        s.facilities.retain(|f| f.kind.does_the_job_of(FacilityKind::LaunchSite));
    }
    g
}

fn fresh() -> Game {
    with_seed(7)
}

fn with_seed(seed: u64) -> Game {
    Game::new(tables(), NewGame { seed, player: FactionKind::Custodians, player_is_ai: false, player_start: StateId::EastAsia })
}

/// Ticket #105 (version 0.07.0): `end_turn` now refuses while a human Research Lead owes a Tech, so
/// a test that drives turns has to pick one first, exactly as a player does.
fn pick_a_tech(g: &mut Game) {
    if g.research.current.is_none()
        && let Some(t) = g.pickable_techs().first().copied()
    {
        g.pick_tech(Seat(0), t).ok();
    }
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
        // Ticket #164 (version 0.07.5): every founding gives a Core Module, so a Colony without one
        // is not a board the game can produce. It is appended AFTER the kinds asked for: the order
        // of a Colony's Modules carries no rule, and this way an index into the list still means
        // the nth kind the test asked for.
        modules: modules.iter().map(|k| Module::new(*k)).chain(std::iter::once(Module::new(ModuleKind::Core))).collect(),
        colonists,
        // Ticket #189 (version 0.08.0): a test Colony's people know the neutral figure unless the
        // test settles somebody with a figure of their own.
        education: 1.0,
        settler_education: 1.0,
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
    // Ticket #164 (version 0.07.5): one less, for the Core Module on the seat's station.
    assert_eq!(g.seats[0].stockpile.energy, 1);
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
    // Ticket #187 (version 0.08.0): this test is about the threshold and the decay, not about
    // schooling, so the state is set to Resistance's pivot where an outsider's Influence converts
    // one for one. At its own card of 0.85 it converts 39 into 41 and the place changes hands.
    let pivot = g.tables.influence.resistance.pivot;
    for s in [StateId::NorthAfrica, StateId::Europe] {
        g.state_mut(s).schooling = pivot - g.tables.state(s).education_level;
    }
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
    // Ticket #266 (version 0.08.4): 1, not 2. With no Blame on the table yet every seat's share is
    // nought, at or below the eighth where a Standing on a Region you do not hold decays slowly;
    // from the first Climate phase on, shares exist and the plain 2 returns for everyone between.
    assert_eq!(g.seats[0].influence[&Place::State(StateId::Europe)], 9, "decay 1 while nobody has any Blame (#266)");
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
    // (20 since ticket #75; 10 before): still no change. That is what stops a place flipping back
    // and forth every turn.
    g.seats[1].influence.insert(Place::State(StateId::NorthAfrica), 79);
    g.seats[1].influenced_this_turn.push(Place::State(StateId::NorthAfrica));
    g.seats[0].influenced_this_turn.push(Place::State(StateId::NorthAfrica));
    g.resolution_phase();
    assert_eq!(g.state(StateId::NorthAfrica).control, Control::Controlled(Seat(0)), "79 is not 60 plus the margin of 20");
    // The controller's standing plus the margin: it flips, and seat 0 keeps its 60 to contest it back.
    g.seats[1].influence.insert(Place::State(StateId::NorthAfrica), 80);
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
    // Ticket #54 (g): the Custodians' multiplier is 1.25, not 1.3; ticket #82 (version 0.06.0):
    // 1.2. They hold East Asia -- China since ticket #122 -- whose value went 4 to 3 when ticket
    // #125 (version 0.07.2) cut Japan and Korea out of it: (10 + 3) x 1.2 = 15.6 -> 15. Europe
    // is still 5, and the Prospectors are still x1.0.
    assert_eq!(g.tables.faction(FactionKind::Custodians).influence_multiplier, 1.2);
    assert_eq!(g.influence_allotment(Seat(0)), 15);
    assert_eq!(g.influence_allotment(Seat(1)), 15);
    g.state_mut(StateId::NorthAmerica).control = Control::Controlled(Seat(1));
    assert_eq!(g.influence_allotment(Seat(1)), 22, "North America adds 7");
    // Raising East Asia's Industry Level adds one to its value.
    g.state_mut(StateId::EastAsia).industry_level += 1;
    assert_eq!(g.state_influence_value(StateId::EastAsia), 4);
    assert_eq!(g.influence_allotment(Seat(0)), 16, "(10 + 4) x 1.2 = 16.8");
    // Ticket #53: twelve states share out the eight states' figures exactly, so the total stands.
    let total: i64 = StateId::ALL.iter().map(|s| g.tables.state(*s).influence).sum();
    assert_eq!(total, 34, "7 + 5 + 4 + 4 + 4 + 2 + 2 + 2 + 1 + 1 + 1 + 1, as the eight totalled 34");
}

// ---------------------------------------------------------------- #35 Ducats

#[test]
fn a_controlled_state_pays_ducats_from_gdp_times_industry_and_a_bank_adds_more() {
    let mut g = game();
    // Ticket #53: East Asia gdp 23 x Industry 3 / 10 = 6 a turn; ticket #125 (version 0.07.2)
    // took 6 of that gdp to Japan and Korea, so 17 x 3 / 10 = 5. Ticket #139 (version 0.07.3): the
    // divisor is 5 and nothing pays under 1, so 17 x 3 / 5 = 10; Europe 20 x 3 / 5 = 12, and ticket
    // #83 (version 0.06.0): x1.2 for the Prospectors who hold it, 14. North Africa, gdp 1 at
    // Industry 1, would have paid nothing under the old rule and pays the floor of 1.
    assert_eq!(g.state_ducats(StateId::EastAsia), 10);
    assert_eq!(g.state_ducats(StateId::Europe), 14);
    assert_eq!(g.tables.base_ducats(StateId::NorthAfrica, 1), 1, "the floor: no Region pays nothing");
    assert_eq!(g.tables.base_ducats(StateId::ArabianPeninsula, 2), 1, "Saudi Arabia, 2 x 2 / 5, pays the floor");
    let paid = income_of(&mut g, Seat(0));
    assert_eq!(paid.ducats, 10);
    // A Bank in East Asia adds 4 x 17 / 10 = 6 (it was 4 x 23 / 10 = 9 before ticket #125 took 6 of
    // the gdp to Japan and Korea); in North Africa (gdp 1) it would add nothing. A Bank's own
    // figure did not move on ticket #139.
    g.state_mut(StateId::EastAsia).facilities.push(facility(FacilityKind::Bank));
    assert_eq!(g.facility_yield(Seat(0), StateId::EastAsia, FacilityKind::Bank).amount, 6);
    assert_eq!(g.facility_yield(Seat(0), StateId::NorthAfrica, FacilityKind::Bank).amount, 0);
    assert_eq!(income_of(&mut g, Seat(0)).ducats, 16);
    // A Trade Post followed the Habitat yield; ticket #90 (version 0.06.0): it pays 2 per Colonist
    // at its Body plus 3 per other Body held. Empty Colonies on the Moon and Mars, with Earth held:
    // each sees two other Bodies, so 6.
    let moon = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::TradePost], 0);
    let mars = colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::TradePost], 0);
    assert_eq!(g.module_yield(Seat(0), moon, ModuleKind::TradePost).amount, 6);
    assert_eq!(g.module_yield(Seat(0), mars, ModuleKind::TradePost).amount, 6);
    // Ticket #72 had it that Ducats are not Materials output, so a Bank banked nothing in the
    // Venture Capital Fund however high the share was set. Ticket #240 (version 0.08.3) turns that
    // exactly around: the Fund banks DUCAT INCOME, so a Bank is now one of the things filling it.
    g.seats[1].venture_share = 0.8;
    let before = g.seats[1].venture_fund;
    g.state_mut(StateId::Europe).facilities = vec![facility(FacilityKind::Bank)];
    income_of(&mut g, Seat(1));
    assert!(g.seats[1].venture_fund > before, "a Bank's Ducats reach the Fund now: {} -> {}", before, g.seats[1].venture_fund);
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
    g.ships.push(Ship { name: String::new(), id: ShipId(1), kind: UnitKind::Frigate, seat: Seat(0), damage: 1, at: ShipAt::Body(BodyId::Earth), colonists: 0, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
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
    // Ticket #93 (version 0.06.0): and three over Venus.
    assert_eq!(slots, vec![5, 2, 3, 1, 1, 3], "ticket #50: five orbital slots over Earth");
    // The start (ticket #50): the Custodians' ISS, the Prospectors' Tiangong and the Archivists'
    // Axiom over Earth, bare; the Arkwrights start with no station, so two slots stand free.
    let iss = station_of(&g, Seat(0), BodyId::Earth).expect("the Custodians start with a station");
    let tiangong = station_of(&g, Seat(1), BodyId::Earth).expect("the Prospectors start with a station");
    let axiom = station_of(&g, Seat(3), BodyId::Earth).expect("the Archivists start with a station");
    assert_eq!(g.place_name(Place::Colony(iss)), "ISS over Earth");
    assert_eq!(g.place_name(Place::Colony(tiangong)), "Tiangong over Earth");
    assert_eq!(g.place_name(Place::Colony(axiom)), "Axiom over Earth");
    assert!(station_of(&g, Seat(2), BodyId::Earth).is_none(), "the Arkwrights start with no station");
    // Ticket #164 (version 0.07.5): a station stands with its Core Module and nothing else.
    assert_eq!(g.colony(iss).unwrap().modules.len(), 1, "its Core Module, and no Shipyard at the start");
    assert_eq!(g.colony(iss).unwrap().modules[0].kind, ModuleKind::Core);
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
    // A bare station is not free to take: its threshold starts at the station base.
    assert_eq!(g.influence_threshold(Place::Colony(iss)), 20);
    // Ticket #164 (version 0.07.5): a station nobody lives on has no slots, and the lines below are
    // about which kinds stand in orbit, so give it somebody first.
    g.colony_mut(iss).unwrap().colonists = 2;
    assert!(g.check_order(Seat(0), &[], &Order::BuildModule { colony: iss, kind: ModuleKind::Mine }).is_err(), "nothing to dig in orbit");
    assert!(g.check_order(Seat(0), &[], &Order::BuildModule { colony: iss, kind: ModuleKind::Shipyard }).is_ok());
    assert!(g.check_order(Seat(0), &[], &Order::BuildModule { colony: iss, kind: ModuleKind::Habitat }).is_ok());
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
    // Ticket #87 (version 0.06.0): a Ship is built with a full tank of 30, so the Stockpile must hold it.
    g.seats[0].stockpile.fuel = 50;
    assert!(g.check_order(Seat(0), &[], &frigate(Place::Colony(iss))).is_ok());
    // Lifts: a Ship at Earth loads Colonists only from a state with a Launch Site, and each lift is a launch.
    let ship = ShipId(g.fresh_id());
    g.ships.push(Ship { name: String::new(), id: ship, kind: UnitKind::ColonyShip, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
    g.state_mut(StateId::NorthAfrica).control = Control::Controlled(Seat(0));
    g.state_mut(StateId::NorthAfrica).facilities.retain(|f| f.kind != FacilityKind::LaunchSite);
    // Ticket #73: a lift takes Emigrants already mustered, so both states hold some.
    g.state_mut(StateId::NorthAfrica).emigrants = 2;
    g.state_mut(StateId::EastAsia).emigrants = 2;
    let from_africa = Order::Load { ship, colonists: 2, from: LoadSource::State(StateId::NorthAfrica), army: None };
    assert!(g.check_order(Seat(0), &[], &from_africa).is_err(), "no Launch Site in Africa");
    let from_asia = Order::Load { ship, colonists: 2, from: LoadSource::State(StateId::EastAsia), army: None };
    assert!(g.check_order(Seat(0), &[], &from_asia).is_ok());
    g.commit_orders(Seat(0), &[from_asia]);
    assert_eq!(g.climate.launches_pending[0], 1, "a lift is a launch");
    // Leaving orbit is no launch: the Ship is already up.
    g.commit_orders(Seat(0), &[Order::Transit { ship, to: BodyId::Moon, slot: None }]);
    assert_eq!(g.climate.launches_pending[0], 1);
}

// ---------------------------------------------------------------- #45 Phobos and Deimos

#[test]
fn phobos_and_deimos_are_small_different_bodies_one_hop_past_mars() {
    let g = game();
    // Ticket #93 (version 0.06.0): Venus is the sixth.
    assert_eq!(BodyId::ALL.len(), 6);
    let ph = g.tables.body(BodyId::Phobos).clone();
    let de = g.tables.body(BodyId::Deimos).clone();
    assert_eq!(ph.name, "Phobos");
    assert_eq!(de.name, "Deimos");
    assert_eq!((ph.colony_slots(), de.colony_slots()), (2, 1));
    assert_eq!((ph.mine_yield, ph.generator_yield, ph.refinery_yield, ph.research_yield), (1.75, 0.75, 0.5, 0.8));
    assert_eq!((de.mine_yield, de.generator_yield, de.refinery_yield, de.research_yield), (1.0, 1.0, 0.25, 0.8));
    // Reach. Ticket #57 replaced the fixed card turns for a crossing between the Earth system and
    // the Mars system with the real flight: at the window it is the Hohmann 259 days, nine turns,
    // for the card's Fuel. The hops inside a system are untouched by it.
    let mut g = g;
    at_window(&mut g);
    // Ticket #67 (version 0.05.5): the Hohmann flight is five turns of sixty days.
    assert_eq!(g.transit_cost(BodyId::Earth, BodyId::Phobos), (5, 24));
    assert_eq!(g.transit_cost(BodyId::Moon, BodyId::Deimos), (5, 24));
    assert_eq!(g.transit_cost(BodyId::Mars, BodyId::Phobos), (1, 2));
    assert_eq!(g.transit_cost(BodyId::Deimos, BodyId::Mars), (1, 2));
    assert_eq!(g.transit_cost(BodyId::Phobos, BodyId::Deimos), (1, 1));
    assert_eq!(g.transit_cost(BodyId::Earth, BodyId::Mars), (5, 20));
    assert_eq!(g.transit_cost(BodyId::Moon, BodyId::Mars), (5, 20));
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
    assert_eq!((earth.mine_yield, earth.generator_yield, earth.refinery_yield, earth.research_yield), (1.75, 0.75, 2.0, 1.0));
    assert_eq!(g.free_slots_on(BodyId::Earth).len(), 3);
    assert_eq!(StateId::ALL.len(), 14, "fourteen Regions since ticket #125, and Antarctica is none of them");
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
    g.armies.push(Army { name: String::new(), id: army, home: ArmyHome::State(StateId::EastAsia), at: ArmyAt::Place(Place::State(StateId::EastAsia)), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None, levy: false });
    let ship = |id: u32, kind: UnitKind| Ship { name: String::new(), id: ShipId(id), kind, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None };
    g.ships.extend([ship(101, UnitKind::ColonyShip), ship(102, UnitKind::Battleship), ship(103, UnitKind::Carrier)]);
    let load_army = |s: u32| Order::Load { ship: ShipId(s), colonists: 0, from: LoadSource::State(StateId::EastAsia), army: Some(army) };
    assert!(g.check_order(Seat(0), &[], &load_army(101)).is_err(), "a Colony Ship carries Colonists only");
    assert!(g.check_order(Seat(0), &[], &load_army(102)).is_err(), "a Battleship fights; it carries no Army");
    assert!(g.check_order(Seat(0), &[], &load_army(103)).is_ok(), "a Carrier carries one Army");
    g.state_mut(StateId::EastAsia).emigrants = 4;
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
    assert_eq!(g.order_cost(Seat(0), &m).ducats, 30, "Materials are 3 Ducats each since ticket #220");
    // Bought Materials are spendable at once: a Factory (20 Materials) is affordable with the buy pending.
    let factory = Order::BuildFacility { state: StateId::EastAsia, kind: FacilityKind::Factory };
    assert!(g.check_order(Seat(0), &[], &factory).is_err(), "no Materials yet");
    let pending = vec![Order::Buy { resource: Resource::Materials, amount: 20 }];
    let (left, _) = g.remaining(Seat(0), &pending);
    assert_eq!((left.materials, left.ducats), (20, 40));
    assert!(g.check_order(Seat(0), &pending, &factory).is_ok());
    let f = Order::Buy { resource: Resource::Fuel, amount: 2 };
    assert_eq!(g.order_cost(Seat(0), &f).ducats, 8, "Fuel is 4 Ducats each since ticket #220");
    let e = Order::Buy { resource: Resource::Energy, amount: 5 };
    assert_eq!(g.order_cost(Seat(0), &e).ducats, 10, "Energy is 2 Ducats each since ticket #220");
    assert!(g.check_order(Seat(0), &[], &Order::Buy { resource: Resource::Materials, amount: 0 }).is_err(), "a positive amount");
    assert!(g.check_order(Seat(0), &[], &Order::Buy { resource: Resource::Ducats, amount: 5 }).is_err(), "Ducats are not for sale");
    assert!(g.check_order(Seat(0), &[], &Order::Buy { resource: Resource::Materials, amount: 34 }).is_err(), "102 Ducats needed, 100 held");
    g.commit_orders(Seat(0), &[m, f, e]);
    assert_eq!(g.seats[0].stockpile, Stockpile { materials: 10, fuel: 2, energy: 5, ducats: 52 });
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
    assert_eq!(g.order_cost(Seat(0), &m).ducats, -15, "half of 3 Ducats each since ticket #220");
    let f = Order::Sell { resource: Resource::Fuel, amount: 2 };
    assert_eq!(g.order_cost(Seat(0), &f).ducats, -4, "half of 4 Ducats each, rounded down over the lot");
    assert!(g.check_order(Seat(0), &[], &Order::Sell { resource: Resource::Materials, amount: 11 }).is_err(), "10 held");
    assert!(g.check_order(Seat(0), &[], &Order::Sell { resource: Resource::Energy, amount: 5 }).is_err(), "Energy is not bought back");
    // The Ducats from a sale are spendable at once.
    let pending = vec![m.clone()];
    let (left, _) = g.remaining(Seat(0), &pending);
    assert_eq!((left.materials, left.ducats), (0, 15));
    assert!(g.check_order(Seat(0), &pending, &Order::BuyInfluence { amount: 5 }).is_ok());
    g.commit_orders(Seat(0), &[m, f]);
    assert_eq!(g.seats[0].stockpile, Stockpile { materials: 0, fuel: 0, energy: 20, ducats: 19 });
}

// ---------------------------------------------------------------- #36 Embassies and Relays

#[test]
fn embassies_and_relays_add_to_the_allotment_and_raise_their_places_standing_each_turn() {
    let mut g = game();
    // Ticket #54: the Custodians' multiplier is 1.25; ticket #82 (version 0.06.0): 1.2. In East
    // Asia, value 3 since ticket #125: (10 + 3) x 1.2 = 15. Two Embassies (they stack) add 4:
    // (10 + 3 + 4) x 1.2 = 20.4 -> 20.
    assert_eq!(g.influence_allotment(Seat(0)), 15);
    g.state_mut(StateId::EastAsia).facilities.push(facility(FacilityKind::Embassy));
    g.state_mut(StateId::EastAsia).facilities.push(facility(FacilityKind::Embassy));
    assert_eq!(g.building_allotment(Seat(0)), 4);
    assert_eq!(g.influence_allotment(Seat(0)), 20);
    // A Relay in a Colony adds 1 more.
    let c = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Habitat, ModuleKind::Relay], 4);
    assert_eq!(g.influence_allotment(Seat(0)), 21, "(10 + 3 + 5) x 1.2 = 21.6");
    // Each Resolution the standing rises by the buildings' figures and does not decay. Ticket #75:
    // the start state begins at its threshold, so the rises are counted from there.
    let claim = g.seats[0].influence[&Place::State(StateId::EastAsia)];
    assert_eq!(claim, g.influence_threshold(Place::State(StateId::EastAsia)), "a claim on the home state from turn 1");
    g.resolution_phase();
    assert_eq!(g.seats[0].influence[&Place::State(StateId::EastAsia)], claim + 4, "two Embassies, 2 each");
    assert_eq!(g.seats[0].influence[&Place::Colony(c)], 2, "one Relay");
    g.resolution_phase();
    assert_eq!(g.seats[0].influence[&Place::State(StateId::EastAsia)], claim + 8);
    // An offline Embassy adds nothing.
    for f in g.state_mut(StateId::EastAsia).facilities.iter_mut().filter(|f| f.kind == FacilityKind::Embassy) {
        f.online = false;
    }
    assert_eq!(g.building_allotment(Seat(0)), 1, "only the Relay");
    g.resolution_phase();
    assert_eq!(g.seats[0].influence[&Place::State(StateId::EastAsia)], claim + 7, "no rise, and decay 1 on your own place");
    // The card says what they do.
    let y = g.facility_yield(Seat(0), StateId::EastAsia, FacilityKind::Embassy);
    assert_eq!(y.text(), "+2 Influence Allotment, standing here +2 a turn, 2 Energy upkeep");
}

// ---------------------------------------------------------------- 8.5 Occupation

fn occupier_in(g: &mut Game, seat_home: StateId, target: StateId) -> ArmyId {
    let id = ArmyId(g.fresh_id());
    g.armies.push(Army { name: String::new(), id, home: ArmyHome::State(seat_home), at: ArmyAt::Place(Place::State(target)), damage: 0, standing: false, stance: Stance::Attack, escaped: false, move_to: None, levy: false });
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
        FactionKind::Prospectors => g.seats[seat.index()].venture_fund = 2500,
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
    open_gates(&mut g);
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
    g.seats[1].venture_fund = 3000; // parts 1.2 and 1.25 -> margin 1.2 (ticket #256: the bar is 2500)
    open_gates(&mut g);
    g.end_phase();
    assert!(matches!(g.outcome, Some(Outcome::Win { seat: Seat(1), .. })), "{:?}", g.outcome);
}

#[test]
fn both_met_by_the_same_margin_is_a_draw() {
    let mut g = game();
    colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Habitat, ModuleKind::Habitat], 12);
    colony(&mut g, Seat(1), BodyId::Mars, &[ModuleKind::Habitat, ModuleKind::Habitat], 12);
    g.seats[0].stabilization_run = 3;
    g.seats[1].venture_fund = 3000; // the lower fraction is the presence, 1.0, on both sides (ticket #256: the bar is 2500)
    open_gates(&mut g);
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
    g.seats[1].venture_fund = 1500; // first 1.0 -> score 0.25
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
    let mut g = game();
    // Ticket #76 (version 0.05.5): forty cards for thirty-six turns. The 28 of #25 and #32, a third
    // copy of Heatwave, Wildfire, Rich Seam and Solar Storm, a second of Unrest, Methane Burst,
    // Labour Dispute and Dust Storm, and four new Events once each.
    // Ticket #259 (version 0.08.4): the twelve cards that can only land off Earth are not dealt at
    // the start; they join on turn 12. So the deck begins at 28 and is 40 only once they are in.
    assert_eq!(g.deck.cards.len(), 28, "twenty-eight at the start: the twelve off-Earth cards join on turn 12 (#259)");
    assert!(!g.deck.off_earth_joined);
    for id in [EventId::GridFailure, EventId::ReactorLeak, EventId::DustStorm, EventId::Moonquake, EventId::HeliumVein, EventId::RichSeam, EventId::IceDeposit] {
        assert!(g.tables.events.event.iter().find(|e| e.id == id).unwrap().off_earth, "{id:?} is flagged off Earth");
        assert_eq!(g.deck.count(id), 0, "{id:?} not dealt at the start");
    }
    assert_eq!(g.tables.events.off_earth_join_turn, 12);
    // A card may or may not be drawn on any turn (the chance is never nought), so the deck and its
    // drawn pile are counted together.
    let dealt = |g: &Game| g.deck.cards.len() + g.deck.drawn.len();
    g.turn = 11;
    g.event_phase();
    assert_eq!(dealt(&g), 28, "turn 11: not yet");
    g.turn = 12;
    g.event_phase();
    assert!(g.deck.off_earth_joined);
    assert_eq!(dealt(&g), 40, "turn 12: the twelve join, and the deck is the forty of #76");
    assert!(g.report.lines.iter().any(|l| l.text.contains("join the deck")), "the Report says so: {:?}", g.report.lines);
    g.event_phase();
    assert_eq!(dealt(&g), 40, "and they join once");
    // The rest of this test reads the deck as dealt, so a fresh one -- with the off-Earth cards
    // in -- is what the copy counts below are checked against.
    let mut g = game();
    g.turn = 12;
    g.deck.cards.append(&mut g.deck.drawn);
    g.event_phase();
    g.deck.cards.append(&mut g.deck.drawn);
    let copies = |id: EventId| g.deck.cards.iter().filter(|c| **c == Card::Event(id)).count();
    for id in [EventId::Heatwave, EventId::Wildfire, EventId::RichSeam, EventId::SolarStorm] {
        assert_eq!(copies(id), 3, "{id:?} three times");
    }
    for id in [EventId::Unrest, EventId::MethaneBurst, EventId::LabourDispute, EventId::DustStorm] {
        assert_eq!(copies(id), 2, "{id:?} twice");
    }
    for id in [EventId::RadiationSurge, EventId::CommsBlackout, EventId::GridFailure, EventId::IceDeposit, EventId::Breakthrough, EventId::StormSurge] {
        assert_eq!(copies(id), 2, "{id:?} still twice");
    }
    for id in [EventId::LaunchPadFire, EventId::SolarMaximum, EventId::MeteorShower, EventId::ReactorLeak] {
        assert_eq!(copies(id), 1, "{id:?} still once");
    }
    assert_eq!(EventId::ALL.len(), 22, "eighteen Events and the four of #76");
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
    g.ships.push(Ship { name: String::new(), id: ship, kind: UnitKind::ColonyShip, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
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
    // Power Plant 6 x 1.5 = 9; Generator 5 x 1.375 (the Moon since ticket #72) x 1.5 = 10.
    // Ticket #164 (version 0.07.5): less the Colony's Core Module, 1 Energy a turn.
    // Ticket #164 (version 0.07.5): less two Core Modules, the Moon Colony's and the station's.
    assert_eq!(income_of(&mut g, Seat(0)).energy, 19 - 2);
    assert_eq!(income_of(&mut g, Seat(0)).energy, 12 - 2, "the boost lasts one Income");
    with_tech(&mut g, TechId::EfficientGrids);
    drawn(&mut g, EventId::SolarMaximum, EventTarget::Everyone);
    g.apply_event_now();
    // Power Plant 6 x 1.5 x 2 = 18; Generator 5 x 1.375 x 1.5 x 2 = 20 (rounded down).
    // Ticket #164 (version 0.07.5): less the two Core Modules.
    assert_eq!(income_of(&mut g, Seat(0)).energy, 38 - 2, "Efficient Grids makes it x2");
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
    let mk = |id: u32, at: ShipAt| Ship { name: String::new(), id: ShipId(id), kind: UnitKind::Frigate, seat: Seat(0), damage: 0, at, colonists: 0, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None };
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

/// Ticket #238 (version 0.08.3): the Strip Permit, the Leapfrog and the Exodus Call all want the
/// Region held three whole turns. A test that issues one on turn 1 is testing the clock, not the
/// order, so it backdates the hold instead.
fn held_long_enough(g: &mut Game, s: StateId) {
    g.turn = g.turn.max(g.tables.faction_orders.min_turns_held + 1);
    g.state_mut(s).held_since = Some(0);
}

fn with_tech(g: &mut Game, t: TechId) {
    g.research.done.push(t);
}

/// Ticket #84 (version 0.06.0): every Victory Condition waits on its gate Tech, so a test about
/// winning opens all four first.
/// Ticket #199 (version 0.08.0): the Archivists' gate Tech, which the Archive now waits on to be
/// ORDERED as well as to win. Every test that builds an Archive needs it in.
fn the_upload(g: &mut Game) {
    if !g.research.done.contains(&TechId::TheUpload) {
        g.research.done.push(TechId::TheUpload);
    }
}

fn open_gates(g: &mut Game) {
    for t in [TechId::PlanetaryStewardship, TechId::ExtractionCharter, TechId::GenerationShips, TechId::TheUpload] {
        g.research.done.push(t);
    }
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
    // Ticket #164 (version 0.07.5): less the ISS's Core Module, 1 Energy a turn.
    assert_eq!(plain, 6 - 1);
    assert_eq!(boosted, 9 - 1);
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
    let f = Ship { name: String::new(), id: ShipId(1), kind: UnitKind::Frigate, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None };
    let c = Ship { name: String::new(), kind: UnitKind::ColonyShip, ..f.clone() };
    assert_eq!(g.ship_strength(&f), 3);
    with_tech(&mut g, TechId::HardenedHulls);
    assert_eq!(g.ship_strength(&f), 5);
    assert_eq!(g.ship_strength(&c), 0);
}

#[test]
fn tech_expanded_habitats_holds_four_more() {
    let mut g = game();
    // Ticket #80 (version 0.06.0): a Habitat holds 8. The Moon's Habitat yield of 1.1 made that
    // 8.8 and 11.0 until ticket #140 (version 0.07.3) traded the yield for a Research one: flat 8
    // and 10 everywhere now.
    let c = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Habitat], 0);
    // Ticket #164 (version 0.07.5): plus the Core Module's flat four, which the Tech does not reach.
    let core = g.tables.module(ModuleKind::Core).holds_colonists;
    // Ticket #207 (version 0.08.1): 4, with the Tech raised to +4 in its place, so a RESEARCHED
    // Habitat holds the 8 an unresearched one held before. The Tech's Colony Ship clause stays +2
    // and is read from `value`; this one is read from `habitat_colonists`.
    assert_eq!(g.habitat_room(g.colony(c).unwrap()), 4 + core);
    with_tech(&mut g, TechId::ExpandedHabitats);
    assert_eq!(g.habitat_room(g.colony(c).unwrap()), 8 + core);
}

#[test]
fn tech_closed_loop_colonies_halves_module_upkeep() {
    let mut g = game();
    colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Mine], 0); // upkeep 3
    g.state_mut(StateId::EastAsia).facilities.clear();
    let plain = income_of(&mut g, Seat(0)).energy;
    with_tech(&mut g, TechId::ClosedLoopColonies);
    let halved = income_of(&mut g, Seat(0)).energy;
    // Ticket #164 (version 0.07.5): the Mine's 3 and two Core Modules, the Colony's and the
    // station's, at 1 each.
    assert_eq!(plain, -5);
    assert_eq!(halved, -1, "the Mine's 3 halved to 1, and each Core Module's 1 halved to nothing");
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
    g.armies.push(Army { name: String::new(), id: defender, home: ArmyHome::Colony(cid), at: ArmyAt::Place(Place::Colony(cid)), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None, levy: false });
    // The player's two Carriers arrive at the Moon, each carrying an Army: strength 8 against 4.
    let mut attackers = Vec::new();
    let mut ships = Vec::new();
    for kind in [UnitKind::Carrier, UnitKind::Carrier] {
        let attacker = ArmyId(g.fresh_id());
        let ship = ShipId(g.fresh_id());
        g.armies.push(Army { name: String::new(), id: attacker, home: ArmyHome::State(StateId::EastAsia), at: ArmyAt::Aboard(ship), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None, levy: false });
        g.ships.push(Ship { name: String::new(), id: ship, kind, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Moon), colonists: 0, colonists_education: 1.0, army: Some(attacker), stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
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
        pick_a_tech(&mut g);
        g.end_turn([orders, Vec::new(), Vec::new(), Vec::new()]).expect("the turn should end");
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
    // Ticket #164 (version 0.07.5): the seat's station over Earth carries a Core Module of its own,
    // whose Energy the loops above never walk, so account for every Core Module the seat holds.
    for col in g.colonies.iter().filter(|x| x.control.director() == Some(Seat(0)) && x.id != c) {
        for m in col.modules.iter().filter(|m| m.kind == ModuleKind::Core) {
            expect.energy -= g.tables.module(m.kind).energy_upkeep;
        }
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
        // Ticket #181 (version 0.08.0): a Faction's start Region's Facilities come up as that
        // Faction's own versions, the added Launch Site included -- so the Arkwrights' start Region
        // carries a Spaceport and Australia's Power Plant is the Archivists' Reactor.
        if let Some(seat) = g.state(sid).control.controller() {
            want.push(FacilityKind::LaunchSite);
            let faction = g.kind(seat);
            for k in want.iter_mut() {
                *k = k.built_by(faction);
            }
        }
        assert_eq!(have, want, "{}", card.name);
        // Ticket #69: North America and South-East Asia carry a Research Lab on top of the count.
        let labs = card.start_facilities.iter().filter(|k| **k == FacilityKind::ResearchLab).count() as u32;
        assert_eq!(card.start_facilities.len() as u32 - labs, card.industry_level, "{}: as many as the Industry Level, plus a start Lab", card.name);
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
    // Asia's start (Factory, Power Plant, Refinery and the Launch Site) pays 7 Energy against 6 made,
    // and since ticket #164 the ISS's Core Module pays 1 more.
    assert_eq!(s.income_last_turn.energy, -2, "{:?}", s.income_last_turn);
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
    let mk = |id: u32| Ship { name: String::new(), id: ShipId(id), kind: UnitKind::Frigate, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None };
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
        name: String::new(), id: ShipId(id),
        kind: UnitKind::Frigate,
        seat,
        damage: 0,
        at: ShipAt::Body(BodyId::Mars),
        colonists: 0, colonists_education: 1.0,
        army: None,
        stance: Stance::Hold,
        escaped: false,
        arrived_this_turn: false,
        built_turn: 1,
        fuel: 30, slot: None,
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
    g.seats[1].venture_fund = 3000; // parts 1.2 and 1.25 -> margin 1.2 (ticket #256: the bar is 2500)
    open_gates(&mut g);
    g.end_phase();
    assert!(matches!(g.outcome, Some(Outcome::Win { seat: Seat(1), .. })), "{:?}", g.outcome);
}

#[test]
fn the_last_turn_ranks_every_seat_by_score() {
    let mut g = game();
    g.turn = g.tables.victory.turns;
    // Each seat is scored on its own two parts (ticket #51). Custodians: 6 Colonists off Earth of
    // 12 but no Stabilization run, score 0. Prospectors: 9 of 12 and the full Venture Capital Fund,
    // score 0.75. Arkwrights: 3 Colonists of 30 and no Body with 4 on it, score 0. Archivists:
    // nothing, score 0.
    colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Habitat, ModuleKind::Habitat], 6);
    let p = colony(&mut g, Seat(1), BodyId::Moon, &[ModuleKind::Habitat, ModuleKind::Habitat, ModuleKind::Habitat], 9);
    let _ = p;
    g.seats[1].venture_fund = 1875; // three quarters of the 2500 bar (ticket #256)
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
    // East Asia adds Russia, South Asia, South-East Asia and Japan to the adjacent set. Two
    // untouched Regions stand at Industry 2, Australia and -- since ticket #125 (version 0.07.2)
    // -- the Arabian Peninsula, and the tie goes to the more populous: the peninsula at 1.0
    // against Australia at 0.5.
    assert_eq!(held(Seat(2)), vec![StateId::ArabianPeninsula]);
    // Then the untouched states are all at Industry 1, so the tie goes to the most populous:
    // Sub-Saharan Africa at 11.4.
    // The peninsula's neighbours join the adjacent set; Australia is the last at Industry 2.
    assert_eq!(held(Seat(3)), vec![StateId::Australia]);
    // The fallback, when every free state touches a taken one: the highest Industry Level free
    // state, ties by population. With everything above taken, North America at 3 wins.
    let taken = [StateId::Europe, StateId::EastAsia, StateId::Australia, StateId::SubSaharanAfrica, StateId::SouthAmerica, StateId::CentralAmerica];
    assert_eq!(g.ai_start_state(&taken), StateId::NorthAmerica, "the fallback picks the best free state");
    // Every seat's start carries a Launch Site.
    for seat in Seat::ALL {
        let sid = held(seat)[0];
        assert!(g.state(sid).facilities.iter().any(|f| f.kind.does_the_job_of(FacilityKind::LaunchSite)), "{seat:?} has no Launch Site");
    }
}

// ---------------------------------------------------------------- 8.3 a same-turn Influence tie

/// Ticket #70 (version 0.05.5): two challengers at the same Standing on a NEUTRAL place draw lots,
/// from the game's own generator so a seed replays the same draw; on a held place the holder keeps
/// it, since a challenger is never tied with a holder and two tied challengers cancel out.
#[test]
fn two_challengers_at_the_same_standing_draw_lots_for_a_neutral_place_and_a_held_one_stays() {
    let claim = |g: &mut Game, seats: &[Seat], standing: i64| {
        for seat in seats {
            g.seats[seat.index()].influence.insert(Place::State(StateId::NorthAfrica), standing);
            g.seats[seat.index()].influenced_this_turn.push(Place::State(StateId::NorthAfrica));
        }
    };
    // North Africa's threshold is 50. Two seats reach it in the same Resolution at the same Standing.
    let mut g = game();
    claim(&mut g, &[Seat(0), Seat(1)], 50);
    g.resolution_phase();
    let winner = g.state(StateId::NorthAfrica).control.controller().expect("the lot fell to one of them");
    assert!(matches!(winner, Seat(0) | Seat(1)), "one of the two claimants: {winner:?}");
    assert!(g.log.to_vec().iter().any(|l| l.contains("the lot falls to")), "the log names the lot: {:?}", g.log.to_vec());
    assert!(g.report.lines.iter().any(|l| l.text.contains("the lot falls to")), "and so does the Report: {:?}", g.report.lines);
    // The same seed draws the same lot.
    let mut again = game();
    claim(&mut again, &[Seat(0), Seat(1)], 50);
    again.resolution_phase();
    assert_eq!(again.state(StateId::NorthAfrica).control, Control::Controlled(winner), "the draw comes from the game's own generator");
    // A held place: two rivals both clear the holder's Standing plus the margin at the same figure,
    // and it stays where it was.
    let mut g = game();
    g.state_mut(StateId::NorthAfrica).control = Control::Controlled(Seat(2));
    g.seats[2].influence.insert(Place::State(StateId::NorthAfrica), 30);
    claim(&mut g, &[Seat(0), Seat(1)], 60);
    g.resolution_phase();
    assert_eq!(g.state(StateId::NorthAfrica).control, Control::Controlled(Seat(2)), "the holder keeps a place two rivals tie for");
    assert!(g.log.to_vec().iter().any(|l| l.contains("it stays as it is")), "{:?}", g.log.to_vec());
    // One more point and the higher Standing takes it.
    g.seats[0].influence.insert(Place::State(StateId::NorthAfrica), 61);
    claim(&mut g, &[Seat(0), Seat(1)], 0);
    g.seats[1].influence.insert(Place::State(StateId::NorthAfrica), 60);
    g.seats[0].influence.insert(Place::State(StateId::NorthAfrica), 61);
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
        name: String::new(), id: ShipId(301),
        kind: UnitKind::Frigate,
        seat: Seat(1),
        damage: 0,
        at: ShipAt::Body(BodyId::Mars),
        colonists: 0, colonists_education: 1.0,
        army: None,
        stance: Stance::Hold,
        escaped: false,
        arrived_this_turn: false,
        built_turn: 1,
        fuel: 30, slot: None,
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
        name: String::new(),
        kind: UnitKind::ColonyShip,
        seat,
        damage: 0,
        at: ShipAt::Body(body),
        colonists: 0, colonists_education: 1.0,
        army: None,
        stance: Stance::Hold,
        escaped: false,
        arrived_this_turn: false,
        built_turn: 1,
        fuel: 30, slot: None,
    });
    id
}

/// A Colony off Earth holding the seat's Archive Module with `paid` Research in the fund, and
/// Habitats and Colonists. Ticket #68 (version 0.05.5): one Module, no stages.
/// Ticket #192 (version 0.08.0): `colonists` is now what has been UPLOADED as well as who lives
/// there, because the Archivists' second Victory part counts the uploaded rather than whoever
/// happens to be standing beside the Module. Every caller of this helper means "the second part
/// stands at N", and a test that wants the two apart sets `uploaded` itself.
fn archive_at(g: &mut Game, seat: Seat, body: BodyId, paid: i64, colonists: u32) -> ColonyId {
    let cid = colony(g, seat, body, &[ModuleKind::Habitat, ModuleKind::Habitat, ModuleKind::Habitat], colonists);
    g.colony_mut(cid).unwrap().modules.push(Module::new(ModuleKind::Archive));
    g.seats[seat.index()].archive_fund = paid;
    g.seats[seat.index()].uploaded = colonists;
    cid
}

#[test]
fn coach_class_doubles_an_arkwright_colony_ships_load_and_cuts_its_price() {
    let mut g = game();
    // Capacity: the card figure for everyone else, twice it for the Arkwrights, and Expanded
    // Habitats adds its two before the doubling.
    assert_eq!(g.colony_ship_capacity(Seat(0)), 4);
    assert_eq!(g.colony_ship_capacity(Seat(2)), 8, "Coach Class carries twice");
    g.research.done.push(TechId::ExpandedHabitats);
    assert_eq!(g.colony_ship_capacity(Seat(0)), 6);
    assert_eq!(g.colony_ship_capacity(Seat(2)), 12, "(4 + 2) doubled");
    // Price: 30 Materials on the units.toml row; ticket #83 (version 0.06.0): the Arkwrights' 20
    // is retired and every Ship costs them 15% less, so 25.
    let build = Order::BuildShip { site: Place::State(StateId::EastAsia), kind: UnitKind::ColonyShip };
    assert_eq!(g.order_cost(Seat(0), &build).materials, 30);
    assert_eq!(g.order_cost(Seat(2), &build).materials, 25);
    // And the Load order holds them to it.
    let sid = g.controlled_states(Seat(2))[0];
    // Ticket #53: a lift of twelve costs an Arkwright state 2.4 population, more than some of the
    // twelve states hold, so the test gives its start state people to spare.
    g.state_mut(sid).population = 10.0;
    g.state_mut(sid).emigrants = 13;
    let ship = a_colony_ship(&mut g, Seat(2), BodyId::Earth);
    let load = |n: u32| Order::Load { ship, colonists: n, from: LoadSource::State(sid), army: None };
    assert!(g.check_order(Seat(2), &[], &load(12)).is_ok());
    let err = g.check_order(Seat(2), &[], &load(13)).unwrap_err();
    assert_eq!(err.0, "this Ship carries at most 12 Colonists");
}

#[test]
fn an_arkwright_muster_takes_twice_the_population_out_of_its_state() {
    let mut g = game();
    g.state_mut(StateId::NorthAfrica).control = Control::Controlled(Seat(2));
    g.state_mut(StateId::NorthAfrica).facilities.retain(|f| f.kind != FacilityKind::LaunchSite);
    g.state_mut(StateId::NorthAfrica).facilities.push(facility(FacilityKind::LaunchSite));
    assert!((g.lift_population(Seat(0), 4) - 4.0).abs() < 1e-9, "one unit of five million each since ticket #143 (version 0.07.3)");
    assert!((g.lift_population(Seat(2), 4) - 8.0).abs() < 1e-9, "Coach Class costs the state twice");
    // Ticket #73: the population is paid when the Emigrants muster, and the lift takes none.
    let before = g.state(StateId::NorthAfrica).population;
    g.commit_orders(Seat(2), &[Order::BuildEmigrants { state: StateId::NorthAfrica, n: 4 }]);
    let taken = before - g.state(StateId::NorthAfrica).population;
    assert!((taken - 8.0).abs() < 1e-9, "the recruit took {taken}, not 8.0 (two units of five million per Pioneer under Coach Class)");
    let after_muster = g.state(StateId::NorthAfrica).population;
    let ship = a_colony_ship(&mut g, Seat(2), BodyId::Earth);
    g.commit_orders(Seat(2), &[Order::Load { ship, colonists: 4, from: LoadSource::State(StateId::NorthAfrica), army: None }]);
    g.resolution_phase();
    assert!((g.state(StateId::NorthAfrica).population - after_muster).abs() < 1e-9, "the lift itself takes nobody");
    assert_eq!(g.ship(ship).unwrap().colonists, 4);
}

#[test]
fn an_arkwright_habitat_holds_twelve() {
    let mut g = game();
    // Ticket #80 (version 0.06.0): a Habitat holds 8, the Arkwrights' 12. The Moon's Habitat yield
    // of 1.1 made those 8 and 13 until ticket #140 (version 0.07.3) traded the yield away: a
    // Habitat holds the same everywhere, so 8 and 12, and with Expanded Habitats 10 and 15.
    let theirs = colony(&mut g, Seat(2), BodyId::Moon, &[ModuleKind::Habitat], 0);
    let mine = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Habitat], 0);
    // Ticket #164 (version 0.07.5): and the Core Module every founding gives holds four more, flat,
    // which neither the Arkwrights' multiplier nor Expanded Habitats reaches.
    let core = g.tables.module(ModuleKind::Core).holds_colonists;
    // Ticket #207 (version 0.08.1): 4 and 6, and with Expanded Habitats 8 and 12.
    assert_eq!(g.habitat_room(g.colony(mine).unwrap()), 4 + core);
    assert_eq!(g.habitat_room(g.colony(theirs).unwrap()), 6 + core, "half again for the Arkwrights");
    g.research.done.push(TechId::ExpandedHabitats);
    assert_eq!(g.habitat_room(g.colony(mine).unwrap()), 8 + core);
    assert_eq!(g.habitat_room(g.colony(theirs).unwrap()), 12 + core, "(4 + 4) x 1.5, and the Core Module still four");
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
        colonists: 4, education: 1.0, settler_education: 1.0,
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
    open_gates(&mut g);
    assert!(g.progress(Seat(2)).met());
}

#[test]
fn funding_the_archive_banks_this_turns_research_and_contributes_nothing_to_the_lead() {
    let mut g = game();
    g.state_mut(StateId::Europe).control = Control::Controlled(Seat(3));
    g.state_mut(StateId::Europe).facilities.push(facility(FacilityKind::ResearchLab));
    g.pick_tech(Seat(0), TechId::PublicScience).unwrap();
    // Version 0.07.0: the declaration is made before Income and read by the next one. Nothing is
    // taken back out of the shared Tech, because nothing of the Archivists' ever goes in.
    g.commit_orders(Seat(3), &[Order::SetResearchDirective { percent: 100 }]);
    assert_eq!(g.seats[3].research_directive, 100, "the directive stands at all of it");
    assert_eq!(g.seats[3].archive_fund, 0, "and banks nothing until Income");
    g.income_phase();
    let made = g.seats[3].research_last_turn;
    assert!(made > 0 && made < g.tables.tech(TechId::PublicScience).cost, "one Lab makes {made}");
    assert_eq!(g.seats[3].archive_fund, made, "the whole turn's Research is banked");
    assert_eq!(g.research.contributions[3], 0, "and none of it reached the shared Tech");
    assert!(g.funding_archive(Seat(3)));
    assert!(g.report.lines.iter().any(|l| l.text.contains("Archivists are funding the Archive")), "{:?}", g.report.lines);
    // Ticket #235 (version 0.08.3): "nobody else may" is gone -- every Faction directs Research
    // now. What is still the Archivists' alone is the REACH: theirs runs to 100, everyone else's
    // stops at half, because their switch always sent all of it and the slider keeps that.
    assert_eq!(g.check_order(Seat(0), &[], &Order::SetResearchDirective { percent: 100 }).unwrap_err().0, "a Research Directive may not pass 50 per cent for this Faction");
    assert!(g.check_order(Seat(0), &[], &Order::SetResearchDirective { percent: 50 }).is_ok(), "a Custodian may direct half");
    assert_eq!(g.research_directive_cap(Seat(0)), 50);
    assert_eq!(g.research_directive_cap(Seat(3)), 100, "the Archivists alone reach all of it");
    // Ticket #68: until the Module stands the fund holds a quarter of the 80, and what it has no
    // room for goes on to the shared Tech rather than being wasted.
    assert_eq!(g.archive_fund_cap(Seat(3)), 20, "a quarter of 80 before the Archive stands");
    g.seats[3].archive_fund = 17;
    g.research.contributions = [0; 4];
    g.income_phase();
    let made = g.seats[3].research_last_turn;
    assert!(made > 3, "the Lab makes more than the three the fund still has room for: {made}");
    assert_eq!(g.seats[3].archive_fund, 20, "only the room under the cap is banked");
    assert_eq!(g.research.contributions[3], made - 3, "the rest counts toward the Lead as usual");
    // At the cap the declaration is refused outright.
    g.seats[3].research_directive = 0;
    assert_eq!(
        g.check_order(Seat(3), &[], &Order::SetResearchDirective { percent: 100 }).unwrap_err().0,
        "the Archive fund holds its quarter (20) until the Archive stands at a Colony off Earth"
    );
    // Once the Module stands the fund opens to the whole 80, and is refused again only when full.
    let mars = colony(&mut g, Seat(3), BodyId::Mars, &[ModuleKind::Habitat], 4);
    g.colony_mut(mars).unwrap().modules.push(Module::new(ModuleKind::Archive));
    assert_eq!(g.archive_fund_cap(Seat(3)), 80);
    assert!(g.check_order(Seat(3), &[], &Order::SetResearchDirective { percent: 100 }).is_ok());
    g.seats[3].archive_fund = 80;
    assert_eq!(g.check_order(Seat(3), &[], &Order::SetResearchDirective { percent: 100 }).unwrap_err().0, "the Archive's Research is paid in full");
}

/// Ticket #68 (version 0.05.5): the Archive is one Module of 50 Materials and three turns, built
/// once from its own button at a Colony off Earth, with no Research banked first; standing, it
/// draws no Energy until its 80 Research is paid, and the payment that fills the fund completes it.
#[test]
fn the_archive_is_one_module_of_fifty_materials_and_three_turns_built_once_off_earth() {
    let mut g = game();
    g.seats[3].stockpile.materials = 200;
    the_upload(&mut g);
    let mars = colony(&mut g, Seat(3), BodyId::Mars, &[ModuleKind::Habitat], 4);
    let order = Order::BuildArchive { colony: mars };
    assert_eq!(g.order_cost(Seat(3), &order).materials, 50);
    assert!(g.check_order(Seat(3), &[], &order).is_ok(), "no Research needs banking first");
    // Antarctica will not do, and ticket #209 (version 0.08.1): neither will a station over Earth,
    // which ticket #81 had allowed and which was the only place the Archive ever stood.
    let ant = colony(&mut g, Seat(3), BodyId::Earth, &[ModuleKind::Habitat], 4);
    assert!(g.check_order(Seat(3), &[], &Order::BuildArchive { colony: ant }).unwrap_err().0.contains("another Body"));
    let axiom = station_of(&g, Seat(3), BodyId::Earth).unwrap();
    // Ticket #209: the PLACE is refused before the four-Colonist gate of ticket #192 is reached, so
    // filling Axiom does not help -- the refusal names the Body, not the population.
    g.colony_mut(axiom).unwrap().modules.push(Module::new(ModuleKind::Core));
    g.settle_people(axiom, 4, 1.0);
    assert_eq!(
        g.check_order(Seat(3), &[], &Order::BuildArchive { colony: axiom }).unwrap_err().0,
        "the Archive stands at a Colony on another Body; neither Antarctica nor a station over Earth will do"
    );
    // Nobody else builds one, and the ordinary Module button never places it.
    let mine = colony(&mut g, Seat(0), BodyId::Mars, &[], 0);
    assert_eq!(g.check_order(Seat(0), &[], &Order::BuildArchive { colony: mine }).unwrap_err().0, "only the Archivists build the Archive");
    assert!(g.check_order(Seat(3), &[], &Order::BuildModule { colony: mars, kind: ModuleKind::Archive }).is_err());
    // Ordered once is ordered: a second order is refused while it builds.
    g.commit_orders(Seat(3), std::slice::from_ref(&order));
    assert!(g.archive_ordered(Seat(3)));
    assert_eq!(g.check_order(Seat(3), &[], &order).unwrap_err().0, "the Archive is already building");
    assert!(g.log.to_vec().iter().any(|l| l.contains("began the Archive at")), "{:?}", g.log.to_vec());
    // Three turns to raise: it stands after the third Resolution.
    g.resolution_phase();
    assert!(!g.archive_built(Seat(3)), "one turn");
    g.turn += 1;
    g.resolution_phase();
    assert!(!g.archive_built(Seat(3)), "two turns");
    g.turn += 1;
    g.resolution_phase();
    assert!(g.archive_built(Seat(3)), "three turns");
    assert_eq!(g.archive_colony(Seat(3)), Some(mars));
    assert!(!g.archive_complete(Seat(3)), "standing is not complete: the Research is still owed");
    assert!(g.log.to_vec().iter().any(|l| l.contains("raised the Archive at") && l.contains("80 more Research")), "{:?}", g.log.to_vec());
    // At most one per Faction.
    let deimos = colony(&mut g, Seat(3), BodyId::Deimos, &[], 0);
    assert!(g.check_order(Seat(3), &[], &Order::BuildArchive { colony: deimos }).unwrap_err().0.contains("already stands at"));
    // No upkeep until it is complete; the payment that fills the fund completes it, with its Moment.
    assert_eq!(g.module_yield(Seat(3), mars, ModuleKind::Archive).upkeep, 0);
    g.seats[3].archive_fund = 76;
    g.seats[3].research_directive = 100;
    let banked = g.bank_archive_research(Seat(3), 10);
    assert_eq!(banked, 4, "only the four still owed are banked");
    assert_eq!(g.seats[3].archive_fund, 80);
    assert!(g.archive_complete(Seat(3)));
    assert_eq!(g.module_yield(Seat(3), mars, ModuleKind::Archive).upkeep, 12);
    assert!(g.log.to_vec().iter().any(|l| l.contains("completed the Archive at")), "{:?}", g.log.to_vec());
}

#[test]
fn provisional_findings_halves_the_tech_under_research_and_goes_off_the_turn_after_funding() {
    let mut g = game();
    g.state_mut(StateId::Europe).control = Control::Controlled(Seat(3));
    g.state_mut(StateId::Europe).facilities.push(facility(FacilityKind::ResearchLab));
    g.pick_tech(Seat(0), TechId::PublicScience).unwrap();
    // Ticket #173 (version 0.07.6): the half-effect reads the Tech FROZEN at the turn's head, not
    // whatever was picked a moment ago, so that a Lead changing its mind cannot re-price orders
    // already placed. A pick made during this turn is read from the next one; the test stands the
    // frozen Tech up by hand rather than running a turn, which would move everything else it reads.
    g.research.findings_tech = g.research.current;
    // A multiplier of 1.5 reads 1.25; nobody else reads an unfinished Tech at all.
    assert!(g.provisional_findings(Seat(3)), "on at the start: nobody has funded yet");
    assert!((g.tech_multiplier(Seat(3), TechId::PublicScience) - 1.25).abs() < 1e-9);
    assert_eq!(g.tech_multiplier(Seat(0), TechId::PublicScience), 1.0);
    // An addition of +2 reads +1, and an immunity does not carry at all.
    assert_eq!(g.tech_addition(Seat(3), TechId::ExpandedHabitats), 0, "only the Tech under research");
    let with = g.facility_yield(Seat(3), StateId::Europe, FacilityKind::ResearchLab).research;
    // Version 0.07.0: the declaration is made in one turn and paid at the next Income, so it is
    // that Income which funds, and the Income after it that finds Provisional Findings gone.
    g.commit_orders(Seat(3), &[Order::SetResearchDirective { percent: 100 }]);
    g.income_phase();
    assert!(g.funding_archive(Seat(3)), "this Income paid the fund");
    assert!(g.provisional_findings(Seat(3)), "and the turn that funds still has it");
    // Back to the shared Tech, so the Income after this one restores it.
    g.commit_orders(Seat(3), &[Order::SetResearchDirective { percent: 0 }]);
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
    let cid = archive_at(&mut g, Seat(3), BodyId::Mars, 80, 12);
    g.seats[3].stockpile.energy = 0;
    assert_eq!(g.module_yield(Seat(3), cid, ModuleKind::Archive).upkeep, 12, "a complete Archive draws 12");
    assert_eq!(g.shortfall_order(Seat(3))[0], "The Archive", "the highest upkeep goes first");
    g.income_phase();
    assert!(!g.colony(cid).unwrap().modules.iter().any(|m| m.kind == ModuleKind::Archive && m.online), "shut down");
    assert!(!g.archive_online(Seat(3)));
    let p = g.progress(Seat(3));
    assert_eq!(p.first_value, 80.0, "every point of Research is paid");
    assert_eq!(p.second_value, 12.0, "and the Colonists are uploaded");
    assert!(p.first_held_back.is_some() && !p.met(), "but it is not running");
    g.end_phase();
    assert!(g.outcome.is_none(), "no win with the Archive dark: {:?}", g.outcome);
}

#[test]
fn the_archive_is_destroyed_when_its_colony_changes_hands_and_the_fund_is_kept() {
    let mut g = game();
    let cid = archive_at(&mut g, Seat(3), BodyId::Mars, 40, 6);
    assert_eq!(g.archive_colony(Seat(3)), Some(cid));
    g.transfer_control(Place::Colony(cid), Seat(1), "Influence");
    assert_eq!(g.archive_colony(Seat(3)), None, "the Archive went with the Colony");
    assert!(!g.colony(cid).unwrap().modules.iter().any(|m| m.kind == ModuleKind::Archive));
    assert_eq!(g.seats[3].archive_fund, 40, "the fund is kept");
    assert!(g.report.lines.iter().any(|l| l.text.contains("Archive at") && l.text.contains("destroyed")), "{:?}", g.report.lines);
    // An Occupied Colony's Archive is dark while the Occupation lasts.
    let again = archive_at(&mut g, Seat(3), BodyId::Moon, 80, 12);
    // Ticket #164 (version 0.07.5): every Colony now draws 1 Energy for its Core Module, and an
    // Archive is 12 on its own; this test is about Occupation, not about the Energy bill.
    g.seats[3].stockpile.energy = 400;
    g.income_phase();
    assert!(g.archive_online(Seat(3)));
    g.colony_mut(again).unwrap().control = Control::Occupied { occupier: Seat(1), previous: Some(Seat(3)), turns: 1 };
    g.income_phase();
    assert!(!g.archive_online(Seat(3)), "an Occupied Colony's Archive is offline");
}

#[test]
fn the_archivists_win_with_the_archive_running_and_twelve_colonists_uploaded() {
    let mut g = game();
    let cid = archive_at(&mut g, Seat(3), BodyId::Mars, 80, 12);
    let _ = cid;
    open_gates(&mut g);
    g.seats[3].stockpile.energy = 200;
    g.income_phase();
    assert!(g.archive_online(Seat(3)));
    let p = g.progress(Seat(3));
    assert_eq!((p.first_value, p.first_bar), (80.0, 80.0));
    assert_eq!((p.second_value, p.second_bar), (12.0, 12.0));
    assert!(p.met());
    g.end_phase();
    assert!(matches!(g.outcome, Some(Outcome::Win { seat: Seat(3), .. })), "{:?}", g.outcome);
    // One Colonist short and it is no win.
    let mut g = game();
    let cid = archive_at(&mut g, Seat(3), BodyId::Mars, 80, 11);
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
    assert!(orders.iter().any(|o| matches!(o, Order::SetResearchDirective { percent: 100 })), "no funding order: {orders:?}");
}

/// Ticket #68 (version 0.05.5): the Archivist AI builds its way off Earth before the Archive. With
/// no Colony, no Launch Site and a bare station, its first steps are the Launch Site and the
/// Shipyard, which #51 never counted as advancing the Archive (so it never left Earth); with a
/// Colony off Earth and the Materials, it orders the Module there.
#[test]
fn the_archivist_ai_builds_its_way_off_earth_and_then_the_archive() {
    let mut g = game();
    let arc = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Archivists).unwrap();
    g.seats[arc.index()].stockpile.materials = 200;
    g.seats[arc.index()].stockpile.energy = 200;
    the_upload(&mut g);
    // Ticket #192 (version 0.08.0): the station is bare, so the Archive is not orderable there, and
    // the computer must not spend the turn on an order that would be refused. Measured, it ordered
    // the Archive on turn 1 at an empty station in 80 of 80 games before this gate.
    let axiom = station_of(&g, arc, BodyId::Earth).unwrap();
    assert_eq!(g.colony(axiom).unwrap().colonists, 0, "the premise: Axiom is founded bare");
    let orders = g.ai_orders(arc);
    assert!(!orders.iter().any(|o| matches!(o, Order::BuildArchive { .. })), "it should not order what would be refused: {orders:?}");

    // With the Core Module's four living there, it orders the Module at once.
    g.colony_mut(axiom).unwrap().modules.push(Module::new(ModuleKind::Core));
    g.settle_people(axiom, 4, 1.0);
    let orders = g.ai_orders(arc);
    assert!(
        orders.iter().any(|o| matches!(
            o,
            Order::BuildFacility { kind: FacilityKind::LaunchSite, .. } | Order::BuildModule { kind: ModuleKind::Shipyard, .. } | Order::BuildArchive { .. }
        )),
        "nothing on the way off Earth: {orders:?}"
    );
    let mut g = game();
    let arc = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Archivists).unwrap();
    g.seats[arc.index()].stockpile.materials = 200;
    g.seats[arc.index()].stockpile.energy = 200;
    let mars = colony(&mut g, arc, BodyId::Mars, &[ModuleKind::Habitat, ModuleKind::Mine, ModuleKind::Generator], 4);
    // Ticket #199 (version 0.08.0): without the gate Tech the Archive is refused, and the computer
    // must neither order it nor FREEZE ITS MATERIALS waiting for it -- a candidate it cannot afford
    // yet makes the seat hold its Materials and build nothing else, which over the fifteen turns the
    // Tech takes would be worse than the gate itself. Both are guaranteed by the validator rather
    // than by a guard in the AI: `check_order` drops the candidate and `check_order_legality` stops
    // it reserving. The place has its four Colonists here, so the Tech is all that is left.
    assert!(!g.has_tech(TechId::TheUpload), "the premise: the world has not researched it yet");
    let orders = g.ai_orders(arc);
    assert!(!orders.iter().any(|o| matches!(o, Order::BuildArchive { .. })), "it should not order what the Tech refuses: {orders:?}");
    assert!(
        orders.iter().any(|o| matches!(o, Order::BuildFacility { .. } | Order::BuildModule { .. })),
        "and it should get on with something else rather than hold its Materials for an Archive it cannot order: {orders:?}"
    );
    the_upload(&mut g);
    let axiom = station_of(&g, arc, BodyId::Earth).unwrap();
    // Ticket #192 (version 0.08.0): the Archive goes to the oldest place that can take it, and since
    // this ticket "can take it" means four people live there. Axiom is bare here, so Mars wins --
    // which is the gate doing its job rather than a change of preference.
    let orders = g.ai_orders(arc);
    assert!(orders.iter().any(|o| matches!(o, Order::BuildArchive { colony } if *colony == mars)), "no Archive order at the one place that can take it: {orders:?}");

    // Ticket #209 (version 0.08.1): peopling Axiom no longer takes the older place back. A station
    // over Earth may not hold the Archive at all now, whoever lives on it, so Mars keeps the order
    // -- which is the whole of this change, since Earth orbit is where the Archive stood in 80 of
    // 80 measured games.
    g.colony_mut(axiom).unwrap().modules.push(Module::new(ModuleKind::Core));
    g.settle_people(axiom, 4, 1.0);
    let orders = g.ai_orders(arc);
    assert!(orders.iter().any(|o| matches!(o, Order::BuildArchive { colony } if *colony == mars)), "a peopled Axiom must not win the Archive back: {orders:?}");
    assert!(!orders.iter().any(|o| matches!(o, Order::BuildArchive { colony } if *colony == axiom)), "Earth orbit is barred: {orders:?}");
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
    // Ticket #125 (version 0.07.2): Japan and Korea border East Asia too, and take none here.
    g.state_mut(StateId::Japan).industry_level = 0;
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
    // Ticket #125 (version 0.07.2): Japan and Korea border East Asia too, and take none here.
    g.state_mut(StateId::Japan).industry_level = 0;
    hold_temperature(&mut g, 3.0);
    assert!(g.population_growth_rate() < 0.0);
    g.climate_phase();
    let lost = before - g.state(StateId::EastAsia).population;
    let arrived = lost * 0.5;
    assert!(arrived > 0.5 && arrived < 1.0, "the flow is worth exactly one point of Unrest: {arrived}");
    assert!((g.state(StateId::Russia).population - arrived).abs() < 1e-6, "Russia took the flow: {}", g.state(StateId::Russia).population);
    // Ticket #176 (version 0.07.6): the Report no longer speaks per flow. The old line here read
    // "0.8 left China for Russia (the heat)" and was written the moment the people moved; a Region
    // losing people to two causes said it twice, and a hot turn spent up to 39 of a 74-line Report
    // on refugees. The Report now says one NET line per Region once every flow of the turn is in,
    // which is in `resolve_unrest`, so nothing is said yet.
    assert!(
        !g.report.lines.iter().any(|l| l.kind == LineKind::Refugees),
        "the moving itself says nothing now: {:?}",
        g.report.lines
    );
    // The LOG keeps the whole record, one line per flow, naming where they went and why.
    assert!(g.log.iter().any(|l| l.contains("left China for") && l.contains("Russia")), "the log still names the flow: {:?}", g.log);
    // Russia changed nothing and holds nobody, so the turn's fall of 1.5 nets against the rise.
    g.state_mut(StateId::Russia).changed_hands = true;
    g.resolve_unrest();
    let want = (arrived / 0.5).floor();
    assert_eq!(g.unrest(StateId::Russia), want.min(2.0), "one Unrest per half a person arriving");
    // And now the two net lines: China lost them, naming the cause that drove them out, and Russia
    // took them in, with the Unrest clause that is the one place the Report explains an Unrest rise.
    let said = |s: &str| g.report.lines.iter().any(|l| l.kind == LineKind::Refugees && l.text.contains(s));
    assert!(said("China lost"), "China's net loss: {:?}", g.report.lines);
    assert!(said("the heat"), "the largest cause survives into the line: {:?}", g.report.lines);
    assert!(said("Russia took in"), "Russia's net gain: {:?}", g.report.lines);
    assert!(said("Unrest rose by 1"), "and why Russia's Unrest rose: {:?}", g.report.lines);
    assert_eq!(g.report.lines.iter().filter(|l| l.kind == LineKind::Refugees).count(), 2, "one line each, and no more");

    // The cap: eight people arriving in a turn is still only two, where #52 allowed three.
    let mut g = game();
    calm(&mut g);
    g.state_mut(StateId::Russia).refugees_in = 8.0;
    g.state_mut(StateId::Russia).changed_hands = true;
    g.resolve_unrest();
    assert_eq!(g.unrest(StateId::Russia), 2.0, "at most two from refugees in a turn");
}

/// Ticket #176 (version 0.07.6): the Report says **one net migration line per Region**, and only
/// when the net is worth at least half a person. The designer: *"reduce report clutter by reporting
/// only net migration from refugees and only when migration occurs."* Measured before the change,
/// over ten computer-played games: the worst turn spent 39 of its 74 Report lines on refugees, the
/// median turn 12, and refugees were 30% of the median Report -- because a Region spoke once per
/// cause that drove people out and once more for arrivals, so a Region that took ten people and
/// sent ten away spoke twice while netting nothing.
#[test]
fn the_report_says_one_net_migration_line_per_region_and_only_when_it_is_worth_saying() {
    let refugee_lines = |g: &Game| g.report.lines.iter().filter(|l| l.kind == LineKind::Refugees).map(|l| l.text.clone()).collect::<Vec<_>>();

    // Flows that cancel say nothing at all: this is the case the designer's line is about.
    let mut g = game();
    calm(&mut g);
    g.state_mut(StateId::Russia).refugees_in = 6.0;
    g.state_mut(StateId::Russia).refugees_out = vec![("the heat".to_string(), 6.0)];
    // Unrest falls by 1.5 in a turn that changed nothing, and this line stops that fall so the rise
    // charged on the arrivals can be read on its own.
    g.state_mut(StateId::Russia).changed_hands = true;
    g.resolve_unrest();
    assert!(refugee_lines(&g).is_empty(), "ten in and ten out is no migration to report: {:?}", refugee_lines(&g));
    // Unrest is still charged on the GROSS arrivals, which is the rule and is unchanged: a Region
    // that took six people absorbed six people's worth of grievance, whatever left afterwards.
    assert_eq!(g.unrest(StateId::Russia), 2.0, "charged on everyone who arrived, up to the cap");

    // Under half a person, in either direction: silence.
    let mut g = game();
    calm(&mut g);
    g.state_mut(StateId::Russia).refugees_in = 0.4;
    g.state_mut(StateId::SouthAsia).refugees_out = vec![("the sea".to_string(), 0.4)];
    g.resolve_unrest();
    assert!(refugee_lines(&g).is_empty(), "under the floor in both directions: {:?}", refugee_lines(&g));

    // A net gain, with some of what arrived cancelled by what left: the line names both figures,
    // because the Unrest clause is charged on the gross and would otherwise be unexplained.
    let mut g = game();
    calm(&mut g);
    g.state_mut(StateId::Russia).refugees_in = 7.0;
    g.state_mut(StateId::Russia).refugees_out = vec![("the heat".to_string(), 4.0)];
    g.resolve_unrest();
    let lines = refugee_lines(&g);
    assert_eq!(lines.len(), 1, "one line for the Region, not one per flow: {lines:?}");
    assert!(lines[0].contains("took in 3.0 people of 7.0 arriving"), "the net and the gross: {lines:?}");
    assert!(lines[0].contains("Unrest rose by"), "and why its Unrest rose: {lines:?}");

    // A net loss to two causes: the largest survives, with `mostly`.
    let mut g = game();
    calm(&mut g);
    g.state_mut(StateId::Russia).refugees_out = vec![("the sea".to_string(), 2.0), ("the heat".to_string(), 5.0)];
    g.resolve_unrest();
    let lines = refugee_lines(&g);
    assert_eq!(lines.len(), 1, "one line, two causes: {lines:?}");
    assert_eq!(lines[0], "Russia lost 7.0 people to its neighbours: mostly the heat.", "the largest cause, and only it");

    // A net loss to one cause says it plainly, without `mostly`.
    let mut g = game();
    calm(&mut g);
    g.state_mut(StateId::Russia).refugees_out = vec![("the reefs".to_string(), 3.0)];
    g.resolve_unrest();
    assert_eq!(refugee_lines(&g)[0], "Russia lost 3.0 people to its neighbours: the reefs.");
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
    g.armies.push(Army { name: String::new(), id, home: ArmyHome::State(StateId::Europe), at: ArmyAt::Place(Place::State(StateId::NorthAfrica)), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None, levy: false });
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
    g.seats[1].venture_fund = 1500;
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
    g.seats[1].venture_fund = 1500;
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
    assert_eq!(StateId::ALL.len(), 14);
    let card = |s: StateId| t.state(s);
    // Asia's 30 GDP and 7 Influence go to East Asia, South Asia and South-East Asia -- and since
    // ticket #125 (version 0.07.2) to Japan and Korea, cut out of East Asia with 6 and 1 of them.
    let asia = [StateId::EastAsia, StateId::SouthAsia, StateId::SouthEastAsia, StateId::Japan];
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
    // The world still holds about 7.9 billion people, as the eight states did: 1,572 units of five
    // million since ticket #143 (version 0.07.3), 78.6 hundred-million before.
    let people: f64 = StateId::ALL.iter().map(|s| card(*s).population).sum();
    assert!((people - 1572.0).abs() < 0.1, "population {people}");
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
        // Ticket #69: less the start Lab of North America and South-East Asia.
        let labs = c.start_facilities.iter().filter(|k| **k == FacilityKind::ResearchLab).count() as u32;
        assert_eq!(c.start_facilities.len() as u32 - labs, c.industry_level, "{s:?} starts with as many Facilities as its Industry Level, plus a start Lab");
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
    // Ticket #265 (version 0.08.4): the credit is what was REMOVED, not the surplus past what was emitted.
    assert!((g.blame_credit(Seat(0)) - removed).abs() < 1e-9, "and shows what was removed as a credit of {:.1} ppm", g.blame_credit(Seat(0)));
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
    // Ticket #187 (version 0.08.0): the pivot, so Resistance converts one for one and this test
    // measures only whose threshold applies.
    let pivot = g.tables.influence.resistance.pivot;
    g.state_mut(StateId::NorthAfrica).schooling = pivot - g.tables.state(StateId::NorthAfrica).education_level;
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
    g.state_mut(sid).emigrants = 1;
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
    // Ticket #164 (version 0.07.5): the Core Module stays; it is never decommissioned.
    assert!(!g.colony(cid).unwrap().modules.iter().any(|m| m.kind == ModuleKind::Mine), "the Mine is gone");
    assert!(g.colony(cid).unwrap().modules.iter().any(|m| m.kind == ModuleKind::Core), "and the Core Module remains");
    assert_eq!(g.seats[0].stockpile.materials, g.tables.module(ModuleKind::Mine).materials / 2);
}

/// (c) Population Emissions are `base + per_level x Industry Level` per hundred million, and the
/// world's opening total is within a tenth of the old flat 0.1.
#[test]
fn c_population_emissions_follow_the_industry_level() {
    let g = game();
    let c = &g.tables.climate;
    // Ticket #143 (version 0.07.3): per unit of five million, a twentieth of the per-hundred-million 0.04 and 0.03.
    assert_eq!(c.population_emissions_base, 0.002);
    assert_eq!(c.population_emissions_per_level, 0.0015);
    for sid in StateId::ALL {
        let want = c.population_emissions_base + c.population_emissions_per_level * g.state(sid).industry_level as f64;
        assert!((g.population_coefficient(sid) - want).abs() < 1e-9, "{sid:?}: {} where {want} was wanted", g.population_coefficient(sid));
    }
    // Sub-Saharan Africa at Industry Level 1 emits 0.07 per hundred million; East Asia at 3 emits
    // 0.13. Ticket #143: the coefficient is per unit of five million, twenty to the hundred million.
    let per_hundred_million = Game::UNITS_PER_HUNDRED_MILLION;
    assert!((g.population_coefficient(StateId::SubSaharanAfrica) * per_hundred_million - 0.07).abs() < 1e-9);
    assert!((g.population_coefficient(StateId::EastAsia) * per_hundred_million - 0.13).abs() < 1e-9);
    let new: f64 = StateId::ALL.iter().map(|s| g.population_coefficient(*s) * g.state(*s).population).sum();
    let old: f64 = StateId::ALL.iter().map(|s| 0.1 / per_hundred_million * g.state(*s).population).sum();
    assert!((new - old).abs() / old < 0.10, "the world's people emit {new:.2} where the flat 0.1 gave {old:.2}");
    // And the Climate phase reads it: one more Industry Level in East Asia is 0.03 x 14.4 more,
    // 14.4 being what ticket #125 (version 0.07.2) left it after Japan and Korea took 2.0.
    let mut g = g;
    let before = g.emissions_now().population;
    let mult = g.tables.faction(FactionKind::Custodians).emissions_multiplier;
    g.state_mut(StateId::EastAsia).industry_level += 1;
    let rise = g.emissions_now().population - before;
    // Ticket #143: 288 units of five million, the 14.4 hundred-million of before; the same 0.432.
    let per_level = g.tables.climate.population_emissions_per_level;
    assert!((rise - per_level * 288.0 * mult).abs() < 1e-9, "the population line rose {rise:.3}");
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
    // Ticket #238 (version 0.08.3): three whole turns in hand before a Faction may remake a
    // country, so this test ages the hold rather than measuring the clock by accident.
    held_long_enough(&mut g, sid);
    let o = Order::Leapfrog { state: sid };
    assert_eq!(g.order_cost(Seat(0), &o).ducats, 50, "50 Ducats a Leapfrog");
    let before = g.population_coefficient(sid);
    g.commit_orders(Seat(0), std::slice::from_ref(&o));
    assert_eq!(g.seats[0].stockpile.ducats, 450);
    assert!((g.population_coefficient(sid) - (before - per)).abs() < 1e-9, "one level's worth off");
    assert_eq!(g.leapfrogs(sid), 1, "Leapfrogged once");
    // East Asia at Industry Level 3 starts at 0.13, so three Leapfrogs reach the base coefficient.
    g.commit_orders(Seat(0), std::slice::from_ref(&o));
    g.commit_orders(Seat(0), std::slice::from_ref(&o));
    assert!((g.population_coefficient(sid) - base).abs() < 1e-9, "the base, and no lower");
    // Ticket #108 (version 0.07.0): a Leapfrog buys two things now, so a fourth is still worth
    // taking -- the coefficient is spent, but East Asia's Baseline of 0.5 is not.
    assert!(g.check_order(Seat(0), &[], &o).is_ok(), "a fourth still buys Baseline");
    let cut = g.tables.climate.leapfrog_baseline_cut;
    assert!((g.baseline_emissions(sid) - (0.5 - 3.0 * cut)).abs() < 1e-9, "three Leapfrogs have taken 0.3 off it");
    // Five in all take the Baseline to nothing, and only then is a Leapfrog refused.
    g.commit_orders(Seat(0), std::slice::from_ref(&o));
    g.commit_orders(Seat(0), std::slice::from_ref(&o));
    assert_eq!(g.baseline_emissions(sid), 0.0, "the Baseline is spent as well");
    assert!(g.check_order(Seat(0), &[], &o).is_err(), "now it buys nothing and is refused");
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
    // Version 0.07.0: the Scrubber runs on 3 Energy, down from 4.
    assert_eq!((card.materials, card.build_turns, card.energy_upkeep, card.emissions), (30, 2, 3, 0.0));
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
    assert!(g.report.lines.iter().any(|l| l.text.contains("Scrubber(s) in China were destroyed")), "and the Report says so");
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

    // +2.3: the scheduled threshold, on its own turn. Ticket #70 (version 0.05.5): East Asia has
    // four coastal slots now, not six, so the Break and +1.8 took all four -- and since ticket #276
    // (version 0.08.5) each of those rises turned one inland slot coastal after the taking, so +2.3
    // finds the two turned slots and takes them: the coast never runs out.
    hold_temperature(&mut g, 2.35);
    g.climate_phase();
    assert!(g.state(StateId::EastAsia).thresholds_fired[1], "the scheduled +2.3 still fires on its own turn");
    assert_eq!(g.state(StateId::EastAsia).lost_slots, 6, "and takes the two slots the first two rises turned coastal");
    assert_eq!(g.build_slots(StateId::EastAsia), after_break - 2, "so the state is two slots smaller again");
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

/// (b) Coastal slots are 2 x Coastal Exposure (3 until ticket #70 of version 0.05.5, which moved
/// fifteen of the world's 49 inland), capped at the start slots less one; the rest of the start
/// slots are inland, and every slot a raise adds is inland.
#[test]
fn b_coastal_slots_are_two_an_exposure_capped_and_a_raise_is_inland() {
    let mut g = fresh();
    assert_eq!(g.tables.coastal_per_exposure, 2, "two coastal slots per point of Coastal Exposure");
    let mut world = 0;
    for sid in StateId::ALL {
        let card = g.tables.state(sid);
        let start = card.size + card.industry_level + 3;
        let want = (2 * card.coastal_exposure).min(start - 1);
        world += want;
        assert_eq!(g.start_slots(sid), start, "{}: start slots", card.name);
        assert_eq!(g.coastal_slots(sid), want, "{}: 2 x Exposure {} capped at {} start slots less one", card.name, card.coastal_exposure, start);
        assert_eq!(g.inland_slots(sid), start - want, "{}: the rest of the start slots are inland", card.name);
        assert_eq!(g.coastal_slots(sid) + g.inland_slots(sid), g.build_slots(sid), "{}: the two rows are the whole card", card.name);
    }
    // Ticket #125 (version 0.07.2): Japan and Korea (Exposure 2, four slots) and the Arabian
    // Peninsula (Exposure 1, two) bring six to the 34 of before.
    assert_eq!(world, 40, "40 coastal slots in the world: the 34 of version 0.05.5 and six on the two new Regions");
    // Ticket #70: Europe's Refinery and North America's Factory, third on their cards with an
    // Exposure of 1, now stand inland from the first turn.
    assert!(g.state(StateId::Europe).facilities.iter().any(|f| f.kind == FacilityKind::Refinery && !f.coastal), "Europe's Refinery went inland");
    assert!(g.state(StateId::NorthAmerica).facilities.iter().any(|f| f.kind == FacilityKind::Factory && !f.coastal), "North America's Factory went inland");
    // Central America and the Caribbean: Size 1, Industry Level 1, Coastal Exposure 2 -> 1 + 3 + 1
    // = 5 start slots, 2 x 2 = 4 coastal, which the cap of four just lets stand.
    let ca = StateId::CentralAmerica;
    assert_eq!(g.start_slots(ca), 5, "Central America starts with five slots");
    assert_eq!(g.coastal_slots(ca), 4, "four coastal, at the cap of the start slots less one");
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
    pick_a_tech(&mut g);
    g.end_turn([orders, Vec::new(), Vec::new(), Vec::new()]).expect("the turn should end");
    g.end_turn(std::array::from_fn(|_| Vec::new())).expect("the turn should end");
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
    // Australia and Oceania: Size 2, Industry Level 2, Exposure 2 -> 7 slots, 4 coastal (ticket
    // #70: two an Exposure, where it was six of the seven), 3 inland.
    let sid = StateId::Australia;
    let st = g.state_mut(sid);
    st.control = Control::Controlled(Seat(0));
    st.facilities = vec![
        Facility::in_coastal_slot(FacilityKind::Factory),
        Facility::in_coastal_slot(FacilityKind::Refinery),
        Facility::in_coastal_slot(FacilityKind::PowerPlant),
        Facility::new(FacilityKind::ResearchLab),
    ];
    assert_eq!(g.coastal_slots(sid), 4);
    assert_eq!(g.inland_slots(sid), 3);

    // Ticket #276 (version 0.08.5): every rise also turns one inland slot coastal, AFTER the taking,
    // so the coast is two taken and one turned each time and never runs out; the "then nothing" of
    // this test's old name went with it.
    g.apply_sea_threshold(sid, 0);
    assert_eq!(g.state(sid).lost_slots, 2, "Exposure 2 takes two coastal slots");
    assert_eq!(g.coastal_slots(sid), 3, "two taken, one turned: three coastal");
    assert_eq!(g.inland_slots(sid), 2, "and one inland slot fewer");
    assert_eq!(standing(&g, sid, true), vec![FacilityKind::Refinery, FacilityKind::PowerPlant], "the oldest coastal Facility went first: the Factory");

    g.apply_sea_threshold(sid, 1);
    assert_eq!(g.state(sid).lost_slots, 4, "two more taken");
    assert_eq!(g.coastal_slots(sid), 2, "the Power Plant's slot and the one just turned");
    assert_eq!(standing(&g, sid, true), vec![FacilityKind::PowerPlant], "the Refinery went with the second rise");
    assert_eq!(standing(&g, sid, false), vec![FacilityKind::ResearchLab], "the inland Research Lab has not moved: an empty slot turned each time");
    assert!(
        g.report.lines.iter().any(|l| l.text.contains("The sea took 2 coastal slots from Australia")),
        "the Report names what the sea took: {:?}",
        g.report.lines
    );

    // A third rise takes the two coastal slots that are left, and with no empty inland slot left
    // the Research Lab's slot turns, Lab and all: the coast has reached the last of the state.
    g.apply_sea_loss(sid, 2.9);
    assert!(g.report.lines.iter().any(|l| l.text.contains("The sea took 2 coastal slots from Australia") && l.text.contains("Power Plant")), "the Power Plant drowned: {:?}", g.report.lines);
    assert_eq!((g.coastal_slots(sid), g.inland_slots(sid)), (1, 0), "one coastal slot, the turned one, and nothing inland");
    assert_eq!(standing(&g, sid, true), vec![FacilityKind::ResearchLab], "the Research Lab stands on the coast now");
    assert_eq!(g.build_slots(sid), 1, "seven slots less six taken");

    // And a fourth takes that too, and turns nothing, since nothing is left inland.
    g.apply_sea_loss(sid, 3.0);
    assert_eq!(g.build_slots(sid), 0, "the state is all sea");
    assert!(g.state(sid).facilities.is_empty(), "the Research Lab drowned with the last slot");
    g.apply_sea_loss(sid, 3.1);
    assert!(g.report.lines.iter().any(|l| l.text.contains("no coastal slots left")), "with nothing left the Report says so: {:?}", g.report.lines);
}

/// Ticket #276 (version 0.08.5): the sea reaches inland. Every rise turns one inland slot coastal
/// AFTER it has taken what it takes, wall or no wall; an empty inland slot first, else the oldest
/// inland Facility with its slot; a state with no inland slot left turns nothing.
#[test]
fn d2_every_rise_turns_one_inland_slot_coastal_wall_or_no_wall() {
    let mut g = game();
    calm(&mut g);
    sea_ahead(&mut g);
    for s in &mut g.states {
        s.population = 0.0;
    }
    // Australia again: 7 slots, 4 coastal, 3 inland. Three coastal Facilities, one inland, so two
    // inland slots are empty.
    let sid = StateId::Australia;
    let st = g.state_mut(sid);
    st.control = Control::Controlled(Seat(0));
    st.facilities = vec![
        Facility::in_coastal_slot(FacilityKind::Factory),
        Facility::in_coastal_slot(FacilityKind::Refinery),
        Facility::in_coastal_slot(FacilityKind::PowerPlant),
        Facility::new(FacilityKind::ResearchLab),
    ];
    assert_eq!((g.coastal_slots(sid), g.inland_slots(sid)), (4, 3));

    // No wall: the rise takes two coastal slots, then turns one EMPTY inland slot coastal.
    g.apply_sea_threshold(sid, 0);
    assert_eq!(g.coastal_slots(sid), 3, "Exposure 2 took two, and one inland slot turned coastal after the taking");
    assert_eq!(g.inland_slots(sid), 2, "one inland slot fewer");
    assert_eq!(g.build_slots(sid), 5, "the total is the two taken, and nothing else");
    assert_eq!(standing(&g, sid, true), vec![FacilityKind::Refinery, FacilityKind::PowerPlant], "the Factory drowned, oldest first, as before");
    assert_eq!(standing(&g, sid, false), vec![FacilityKind::ResearchLab], "the empty slot turned, not the Research Lab's");
    assert_eq!(g.free_coastal(sid), 1, "the turned slot is coastal and empty, waiting for the next rise");
    assert!(g.report.lines.iter().any(|l| l.text.contains("The coast now reaches one slot further in.")), "the Report says the coast moved: {:?}", g.report.lines);

    // A wall: the rise takes nothing, and STILL turns one inland slot -- the last empty one.
    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::SeaWall));
    let before = g.state(sid).facilities.len();
    g.apply_sea_threshold(sid, 1);
    assert_eq!(g.state(sid).facilities.len(), before, "the wall held: nothing drowned");
    assert_eq!((g.coastal_slots(sid), g.inland_slots(sid)), (4, 1), "and one more inland slot turned coastal behind it");
    assert!(g.report.lines.iter().any(|l| l.text.contains("took the sea") && l.text.contains("one slot further in")), "the held-rise line says both: {:?}", g.report.lines);

    // No empty inland slot left: the oldest inland Facility turns with its slot.
    g.apply_sea_threshold(sid, 2);
    assert_eq!((g.coastal_slots(sid), g.inland_slots(sid)), (5, 0), "the last inland slot turned");
    assert_eq!(standing(&g, sid, false), vec![FacilityKind::SeaWall], "nothing but the wall, which takes no slot, stands inland any more");
    assert!(standing(&g, sid, true).contains(&FacilityKind::ResearchLab), "the Research Lab stands on the coast now");
    assert!(g.report.lines.iter().any(|l| l.text.contains("a Research Lab stands on it now")), "the Report names what turned: {:?}", g.report.lines);

    // Nothing inland left to turn: a further rise turns nothing.
    g.apply_sea_loss(sid, 2.9);
    assert_eq!((g.coastal_slots(sid), g.inland_slots(sid)), (5, 0), "a state that is all coast turns nothing more");

    // A raised slot turns like any other.
    g.state_mut(sid).industry_level += 1;
    assert_eq!(g.inland_slots(sid), 1, "a raise adds an inland slot");
    g.apply_sea_loss(sid, 3.0);
    assert_eq!((g.coastal_slots(sid), g.inland_slots(sid)), (6, 0), "and the sea reaches that one too");
}

/// (e) The Sea Wall: it needs Coastal Engineering and a free coastal slot, one per state, and it
/// absorbs the state's next threshold of any kind and is destroyed doing it. A mothballed one does not.
#[test]
fn e_the_sea_wall_needs_its_tech_takes_no_slot_and_takes_one_threshold() {
    let sid = StateId::Australia;
    let mut g = game();
    directed(&mut g, sid);
    let order = Order::BuildFacility { state: sid, kind: FacilityKind::SeaWall };
    assert!(g.check_order(Seat(0), &[], &order).is_err(), "no Coastal Engineering, no Sea Wall");
    g.research.done.push(TechId::CoastalEngineering);
    assert!(g.check_order(Seat(0), &[], &order).is_ok(), "with the Tech in, it is legal");

    // One per state.
    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::SeaWall));
    assert!(g.check_order(Seat(0), &[], &order).is_err(), "at most one Sea Wall stands in a state");

    // Ticket #77 (version 0.05.5): it takes no build slot, as the Scrubber does: legal with every
    // slot full, counting against none once it stands, and 20 Materials.
    let mut g2 = game();
    let s2 = StateId::CentralAmerica;
    directed(&mut g2, s2);
    g2.research.done.push(TechId::CoastalEngineering);
    while g2.free_slots(s2) > 0 {
        g2.state_mut(s2).facilities.push(facility(FacilityKind::Factory));
    }
    assert_eq!(g2.free_slots(s2), 0, "every slot full");
    assert!(g2.tables.facility(FacilityKind::SeaWall).no_slot, "the Sea Wall takes no slot");
    assert!(g2.check_order(Seat(0), &[], &Order::BuildFacility { state: s2, kind: FacilityKind::SeaWall }).is_ok(), "a Sea Wall needs no slot");
    g2.state_mut(s2).facilities.push(Facility::new(FacilityKind::SeaWall));
    assert_eq!(g2.free_slots(s2), 0, "and takes none once it stands");
    assert_eq!(g2.tables.facility(FacilityKind::SeaWall).materials, 20, "20 Materials since ticket #77");

    // Ticket #257 (version 0.08.4): it absorbs a scheduled threshold and STANDS -- it was destroyed
    // doing it from ticket #56 to here -- counting the rise it held, and the next one is held too.
    let mut g = game();
    calm(&mut g);
    sea_ahead(&mut g);
    for s in &mut g.states {
        s.population = 0.0;
    }
    g.state_mut(sid).facilities = vec![Facility::new(FacilityKind::SeaWall)];
    let before = g.coastal_slots(sid);
    g.apply_sea_threshold(sid, 0);
    assert_eq!(g.state(sid).lost_slots, 0, "the wall took the sea: no coastal slot lost");
    // Ticket #276 (version 0.08.5): the rise still turns one inland slot coastal behind the wall.
    assert_eq!(g.coastal_slots(sid), before + 1, "and one inland slot turned coastal behind it, wall or no wall");
    let wall = g.state(sid).facilities.iter().find(|f| f.kind == FacilityKind::SeaWall).expect("the wall stands");
    assert_eq!(wall.rises_held, 1, "and counts the rise it held");
    assert!(g.report.lines.iter().any(|l| l.text.contains("Sea Wall") && l.text.contains("dearer to keep")), "the Report says so: {:?}", g.report.lines);
    // The next one is held too.
    g.apply_sea_threshold(sid, 1);
    assert_eq!(g.state(sid).lost_slots, 0, "the wall holds every threshold, not one");
    assert_eq!(g.coastal_slots(sid), before + 2, "and the coast moved in again");
    assert_eq!(g.state(sid).facilities.iter().find(|f| f.kind == FacilityKind::SeaWall).unwrap().rises_held, 2);

    // The Ice Sheets Break is a threshold of a kind too.
    let mut g = game();
    calm(&mut g);
    breaks_ahead(&mut g);
    bare_world(&mut g);
    g.state_mut(sid).facilities = vec![Facility::new(FacilityKind::SeaWall)];
    let before = g.coastal_slots(sid);
    hold_temperature(&mut g, 2.25);
    g.climate_phase();
    assert_eq!(g.state(sid).lost_slots, 0, "the wall took the Ice Sheets Break");
    assert_eq!(g.coastal_slots(sid), before + 1, "which turned one inland slot coastal like any rise (ticket #276)");
    assert!(g.state(sid).facilities.iter().any(|f| f.kind == FacilityKind::SeaWall && f.rises_held >= 1), "and stands, counting it (ticket #257)");

    // A mothballed wall absorbs nothing.
    let mut g = game();
    calm(&mut g);
    sea_ahead(&mut g);
    for s in &mut g.states {
        s.population = 0.0;
    }
    let mut wall = Facility::new(FacilityKind::SeaWall);
    wall.mothballed = true;
    wall.online = false;
    g.state_mut(sid).facilities = vec![wall];
    g.apply_sea_threshold(sid, 0);
    assert_eq!(g.state(sid).lost_slots, 2, "a mothballed Sea Wall absorbs nothing");
}

/// (f) Coastal Engineering is Industry rung 2, needs Efficient Grids, and the tree holds thirteen;
/// ticket #84 (version 0.06.0) adds the four Victory gates, so seventeen.
#[test]
fn f_coastal_engineering_is_the_thirteenth_tech() {
    let g = fresh();
    // Ticket #201 (version 0.08.1): eighteen, with Civil Defense on Society rung 2.
    assert_eq!(TechId::ALL.len(), 20, "thirteen Techs, the four gates, Civil Defense, and ticket #232's two");
    assert_eq!(g.tables.techs.len(), 20, "and twenty rows in techs.toml");
    let c = g.tables.tech(TechId::CoastalEngineering);
    assert_eq!(c.name, "Coastal Engineering");
    assert_eq!(c.branch, "Industry");
    // Ticket #69 (version 0.05.5): moved from rung 2 at 25 to rung 1 at 10 with no prerequisite.
    // Ticket #117 (version 0.07.1): 10 to 11, with every other cost, a tenth rounded to the nearest.
    assert_eq!(c.rung, 1, "rung 1, beside Efficient Grids");
    assert_eq!(c.cost, 15, "15 since ticket #231 (version 0.08.3); 14 from #201, 12 from #142, 11 from #117, 10 before");
    assert!(c.needs.is_empty(), "it needs nothing");
    assert!(c.effect.contains("Sea Wall"), "its effect names the Sea Wall: {}", c.effect);
    // Two boxes on Industry rung 1, and Clean Power alone on rung 2.
    let on_rung = |r: u32| -> Vec<&str> { TechId::ALL.into_iter().map(|t| g.tables.tech(t)).filter(|t| t.branch == "Industry" && t.rung == r).map(|t| t.name.as_str()).collect() };
    assert_eq!(on_rung(1), vec!["Efficient Grids", "Coastal Engineering"], "two boxes on Industry rung 1");
    assert_eq!(on_rung(2), vec!["Clean Power"]);
    // The Sea Wall's card names it as its unlock, and there are fifteen Facilities in version
    // 0.08.0: ten through version 0.07, the School on ticket #185, and the four Unique Facilities on
    // tickets #182 to #186 -- the Investment Bank, the Spaceport, the Reactor and the Academy.
    assert_eq!(g.tables.facility(FacilityKind::SeaWall).needs_tech, Some(TechId::CoastalEngineering));
    assert_eq!(FacilityKind::ALL.len(), 15, "fifteen Facilities");
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
        name: String::new(),
        kind: UnitKind::ColonyShip,
        seat: Seat(0),
        damage: 0,
        at: ShipAt::Body(BodyId::Earth),
        colonists: 4, colonists_education: 1.0,
        army: None,
        stance: Stance::Hold,
        escaped: false,
        arrived_this_turn: false,
        built_turn: turn,
        fuel: 30, slot: None,
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
    assert!((e.research_yield - 1.0).abs() < 1e-9, "Research 1.0 (the Habitat yield until ticket #140): {}", e.research_yield);
}

/// (h) The AI enumerates a Sea Wall once Coastal Engineering is in and a threshold is near.
/// Ticket #257 (version 0.08.4): each rise a Sea Wall has held adds half a Material a turn to its
/// keep, paid at Income; the half is carried, so two rises pay one a turn and one rise pays one
/// every other turn. Short of Materials, the wall stands unkept that turn and holds nothing.
#[test]
fn a_sea_wall_costs_half_a_material_a_turn_for_every_rise_it_has_held() {
    let sid = StateId::Australia;
    let mut g = game();
    calm(&mut g);
    directed(&mut g, sid);
    let mut wall = Facility::new(FacilityKind::SeaWall);
    wall.rises_held = 1;
    g.state_mut(sid).facilities = vec![wall];
    assert_eq!(income_of(&mut g, Seat(0)).materials, 0, "half a Material owed, none paid yet");
    assert_eq!(income_of(&mut g, Seat(0)).materials, -1, "the second half makes one");
    assert!(g.seat(Seat(0)).income_sources.iter().any(|(n, _, v)| n.contains("Sea Wall") && *v == -1), "a source line while it pays: {:?}", g.seat(Seat(0)).income_sources);
    assert_eq!(income_of(&mut g, Seat(0)).materials, 0, "and the count starts again");
    g.state_mut(sid).facilities[0].rises_held = 3;
    g.seats[0].sea_wall_upkeep_owed = 0.0;
    assert_eq!(income_of(&mut g, Seat(0)).materials, -1, "three rises: 1.5 a turn, 1 paid");
    assert_eq!(income_of(&mut g, Seat(0)).materials, -2, "then 2, the half carried");
    assert!(g.state(sid).facilities[0].working(), "kept, so it works");
    // A mothballed wall pays nothing.
    g.state_mut(sid).facilities[0].mothballed = true;
    g.seats[0].sea_wall_upkeep_owed = 0.0;
    assert_eq!(income_of(&mut g, Seat(0)).materials, 0, "mothballed: no keep");
    g.state_mut(sid).facilities[0].mothballed = false;
    // Short of Materials: the wall stands unkept this turn and holds nothing.
    g.seats[0].stockpile.materials = 0;
    g.seats[0].sea_wall_upkeep_owed = 0.0;
    g.state_mut(sid).facilities[0].rises_held = 2;
    assert_eq!(income_of(&mut g, Seat(0)).materials, 0, "nothing to pay with, nothing paid");
    assert_eq!(g.seat(Seat(0)).stockpile.materials, 0, "never below nothing");
    assert!(!g.state(sid).facilities[0].working(), "unkept: it holds nothing this turn");
    assert!(g.report.lines.iter().any(|l| l.text.contains("Sea Wall") && l.text.contains("unkept")), "the Report says so: {:?}", g.report.lines);
}

/// Ticket #257 (version 0.08.4): a Storm Surge that breaks on a standing Sea Wall no longer brings
/// the next threshold forward; the coastal Facilities make 30% less at the next Income, once. An
/// unwalled state takes the threshold early as it always has.
#[test]
fn a_storm_surge_on_a_walled_state_cuts_its_coastal_facilities_by_a_third_for_one_income() {
    let sid = StateId::Europe;
    let mut g = game();
    calm(&mut g);
    sea_ahead(&mut g);
    for s in &mut g.states {
        s.population = 0.0;
    }
    g.take_control(sid, Seat(0));
    g.seats[0].stockpile.materials = 500;
    g.state_mut(sid).facilities = vec![Facility::in_coastal_slot(FacilityKind::Factory), facility(FacilityKind::Factory), Facility::new(FacilityKind::SeaWall)];
    assert_eq!(income_of(&mut g, Seat(0)).materials, 8, "two Custodian Factories make 4 each");
    let slots = g.coastal_slots(sid);
    drawn(&mut g, EventId::StormSurge, EventTarget::State(sid));
    g.apply_event_now();
    assert!(g.state(sid).thresholds_fired.iter().all(|f| !f), "no threshold brought forward: the wall held");
    assert_eq!(g.coastal_slots(sid), slots, "no slot lost");
    assert!(g.state(sid).storm_surge, "the surge is on the state");
    assert_eq!(income_of(&mut g, Seat(0)).materials, 6, "the coastal Factory makes floor(4 x 0.7) = 2; the inland one 4");
    assert!(!g.state(sid).storm_surge, "spent");
    assert_eq!(income_of(&mut g, Seat(0)).materials, 8, "one Income only");
    assert!(g.report.lines.iter().any(|l| l.text.contains("Sea Wall held")), "the Report says so: {:?}", g.report.lines);
    // Without a wall, the threshold comes forward as before.
    g.state_mut(sid).facilities.retain(|f| f.kind != FacilityKind::SeaWall);
    drawn(&mut g, EventId::StormSurge, EventTarget::State(sid));
    g.apply_event_now();
    assert!(g.state(sid).thresholds_fired[0], "unwalled: the next threshold applies now");
    assert!(!g.state(sid).storm_surge);
}

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

    // Ticket #77 (version 0.05.5): the wall takes no slot, so it is offered with every slot full.
    let mut g = game();
    calm(&mut g);
    sea_ahead(&mut g);
    directed(&mut g, sid);
    g.research.done.push(TechId::CoastalEngineering);
    hold_temperature(&mut g, 1.65);
    while g.free_slots(sid) > 0 {
        g.state_mut(sid).facilities.push(facility(FacilityKind::Factory));
    }
    g.seats[0].stockpile.materials = 200;
    g.seats[0].stockpile.energy = 200;
    let orders = g.ai_orders(Seat(0));
    assert!(
        orders.iter().any(|o| matches!(o, Order::BuildFacility { state, kind: FacilityKind::SeaWall } if *state == sid)),
        "every slot full and the AI still walls the coast, since the wall takes none: {orders:?}"
    );

    // Ticket #70 (version 0.05.5): with the sea close the wall takes the victory-gap and threat
    // multipliers, so a Custodian with Materials for one building or the other walls the coast
    // rather than holding everything for a Scrubber, which is what 0.05.5's Research ticket found
    // it doing in every seed.
    let mut g = game();
    calm(&mut g);
    sea_ahead(&mut g);
    directed(&mut g, sid);
    assert_eq!(g.kind(Seat(0)), FactionKind::Custodians, "seat 0 is the Custodians in this fixture");
    g.research.done.push(TechId::CoastalEngineering);
    hold_temperature(&mut g, 1.65);
    g.seats[0].stockpile.materials = 40;
    g.seats[0].stockpile.energy = 200;
    let orders = g.ai_orders(Seat(0));
    assert!(
        orders.iter().any(|o| matches!(o, Order::BuildFacility { kind: FacilityKind::SeaWall, .. })),
        "with 40 Materials, the sea 0.15 C away and a Scrubber on offer, a wall comes first: {orders:?}; the AI's list: {:#?}",
        g.log.to_vec().iter().filter(|l| l.contains("Sea Wall") || l.contains("Scrubber")).collect::<Vec<_>>()
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
    // Ticket #164 (version 0.07.5): a station holds four from the day it stands, so lifting
    // Emigrants to it is a live use of Materials that would outbid the saving. Fill it: this test
    // is about waiting for a Colony Ship.
    let room = g.habitat_room(g.colony(iss).unwrap());
    g.colony_mut(iss).unwrap().colonists = room;
    g.seats[cust.index()].stockpile.materials = 8;
    g.seats[cust.index()].stockpile.energy = 200;
    g.seats[cust.index()].income_last_turn.materials = 8;
    g.seats[cust.index()].income_last_turn.energy = 20;
    let orders = g.ai_orders(cust);
    let spent: Vec<&Order> = orders.iter().filter(|o| g.order_cost(cust, o).materials > 0).collect();
    let lines: Vec<String> = g.log.iter().filter(|&l| l.contains("Colony Ship") || l.starts_with("  take")).cloned().collect();
    // Ticket #164 (version 0.07.5): the AI logs `wait` for something affordable within a few turns
    // and `save` for something it is holding Materials against a cheaper want; both mean it is
    // holding rather than buying, which is what this test guards, and the assertion below is the
    // hard half of it.
    assert!(
        lines.iter().any(|l| (l.contains("wait") || l.contains("save")) && l.contains("Colony Ship")),
        "the Colony Ship is held for: {lines:#?}"
    );
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
                ("Research", y.research, card.research_yield),
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
            colonists: 0, education: 1.0, settler_education: 1.0,
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
    // Ticket #140 (version 0.07.3): a Habitat holds the same everywhere -- the slot's fourth yield
    // is Research now, and it is the Observatory that follows the slot, on the ground; a station's
    // Observatory reads its Body's figure, the first Body yield a station has read.
    g.colony_mut(rich_id).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    let per = g.tables.module(ModuleKind::Habitat).holds_colonists as f64;
    // Ticket #164 (version 0.07.5): a place also holds four for every Core Module standing on it,
    // so the reading is the Habitat's figure plus whatever this place was founded with.
    let core_room = |g: &Game, c| {
        g.colony(c).unwrap().modules.iter().filter(|m| m.kind == ModuleKind::Core).count() as u32 * g.tables.module(ModuleKind::Core).holds_colonists
    };
    assert_eq!(g.habitat_room(g.colony(rich_id).unwrap()), per as u32 + core_room(&g, rich_id), "a Habitat holds the same on every slot");
    let iss = station_of(&g, Seat(0), BodyId::Earth).unwrap();
    g.colony_mut(iss).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    assert_eq!(g.habitat_room(g.colony(iss).unwrap()), per as u32 + core_room(&g, iss), "and the same in orbit");
    let science = g.slot_yields(BodyId::Mars, rich).research;
    assert!((g.research_yield_at(g.colony(rich_id).unwrap()) - science).abs() < 1e-9, "an Observatory on the ground reads its slot's Research yield");
    assert!((g.research_yield_at(g.colony(iss).unwrap()) - g.tables.body(BodyId::Earth).research_yield).abs() < 1e-9, "a station's Observatory reads its Body's");
    g.colony_mut(rich_id).unwrap().modules.push(Module::new(ModuleKind::Observatory));
    let base = g.tables.module(ModuleKind::Observatory).produces.as_ref().map(|p| p.amount).unwrap_or(0) as f64;
    let colonists = g.colony(rich_id).unwrap().colonists as f64;
    let per_colonist = g.tables.observatory.research_per_colonist;
    let want = (base * science * (1.0 + colonists * per_colonist) * g.tables.faction(g.kind(Seat(0))).research_multiplier_off_earth.unwrap_or(g.tables.faction(g.kind(Seat(0))).research_multiplier)).floor() as i64;
    assert_eq!(g.module_yield(Seat(0), rich_id, ModuleKind::Observatory).research, want, "the Observatory's Research carries the slot's yield");
}

/// Ticket #57 (c), amended by ticket #67 (version 0.05.5): the game begins on 1 January 2030, runs
/// thirty-six turns, and a Turn is TWO calendar months, named by its first month alone.
#[test]
fn a_turn_is_two_calendar_months_from_january_2030() {
    let g = game();
    assert_eq!(g.tables.victory.turns, 36, "thirty-six turns");
    assert_eq!(g.tables.victory.months_per_turn, 2, "of two months each");
    assert_eq!(g.date(1), Date { year: 2030, month: 1 });
    assert_eq!(g.date(1).text(), "January 2030");
    assert_eq!(g.date(2).text(), "March 2030", "turn 2 is named by its first month, March, not February");
    assert_eq!(g.date(6).text(), "November 2030");
    assert_eq!(g.date(7).text(), "January 2031");
    assert_eq!(g.date(13).text(), "January 2032");
    assert_eq!(g.date(36), Date { year: 2035, month: 11 }, "the last turn is November 2035");
    assert_eq!(g.date(36).text(), "November 2035");
    // Turn 1 is exactly 2030-01-01 00:00 UTC, the moment the game begins, and turn 2's sky is read
    // at the first instant of March 2030 (59 days on; 2030 is no leap year).
    assert_eq!(g.julian_day(1), 2462502.5);
    assert_eq!(g.julian_day(2), 2462502.5 + 59.0);
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

/// Ticket #57 (e), amended by ticket #67 (version 0.05.5): the first Mars launch window after
/// January 2030 falls where the real one does. The research file (section 3.3) puts the 2031 Type I
/// optimum departure at 28 January 2031, which at two months a turn is turn 7; the game's own window
/// turn must be within one turn of it. With thirty-six turns the game now holds THREE windows: the
/// real ones of early 2031, spring 2033 and spring 2035, a synodic period (13 turns) apart.
#[test]
fn the_first_mars_window_falls_where_the_real_one_of_early_2031_does() {
    let g = game();
    let window = g.next_window_turn(1);
    assert!((6..=8).contains(&window), "the window is turn {window} ({}), not within one turn of January 2031", g.date(window).text());
    assert_eq!(g.date(window).year, 2031, "and it is in 2031");
    // It really is the smallest offset in the whole first cycle, and no other turn of it is nearer.
    let offset = g.window_offset(window).abs();
    assert!(offset < 15.0, "the window turn stands {offset:.1} degrees off the Hohmann angle");
    let cycle = (g.tables.transit.synodic_days / g.tables.transit.days_per_turn).ceil() as u32;
    assert_eq!(cycle, 13, "780 days is thirteen turns of sixty");
    for t in 1..=cycle {
        if t != window {
            assert!(g.window_offset(t).abs() >= offset, "turn {t} is no nearer the window than turn {window}");
        }
    }
    // The second and third windows are inside the game, a synodic period apart; a fourth is not.
    let second = g.next_window_turn(window + 1);
    assert!((19..=21).contains(&second), "the second window is turn {second} ({}), not spring 2033", g.date(second).text());
    assert_eq!(g.date(second).year, 2033);
    let third = g.next_window_turn(second + 1);
    assert!((32..=34).contains(&third), "the third window is turn {third} ({}), not spring 2035", g.date(third).text());
    assert_eq!(g.date(third).year, 2035);
    assert!(g.next_window_turn(third + 1) > g.tables.victory.turns, "a fourth window lies past the last turn");
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
    // Ticket #67 (version 0.05.5): a turn is sixty days, so the same flight is five turns, and the
    // cap of a year and a half is nine turns rather than eighteen.
    assert_eq!(hohmann, 5, "259 days is five turns of sixty");
    assert_eq!(tr.max_turns, 9, "the cap is nine turns of sixty, the same year and a half");
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
                name: String::new(),
                kind,
                seat,
                damage: 0,
                at: ShipAt::Body(BodyId::Earth),
                colonists,
                colonists_education: 1.0,
                army: None,
                stance: Stance::Hold,
                escaped: false,
                arrived_this_turn: false,
                built_turn: 1,
                fuel: 30, slot: None,
            });
        }
        g
    };
    let window = game().next_window_turn(1);
    // Ticket #87 (version 0.06.0): a crossing spends the tank, not the Stockpile, so the bank has
    // nothing to hold against; what the AI does two turns out is fill a short tank at its station,
    // the one Fuel spend the bank never blocks, and fly no leg the tank cannot pay.
    let mut near = board(window - 2);
    for s in near.ships.iter_mut().filter(|s| s.kind == UnitKind::ColonyShip) {
        s.fuel = 5;
    }
    let orders = near.ai_orders(Seat(0));
    let refuels = orders.iter().filter(|o| matches!(o, Order::Refuel { .. })).count();
    assert!(refuels >= 1, "a short tank at the ISS is refuelled: {orders:?}");
    let spends: Vec<&Order> = orders.iter().filter(|o| near.order_cost(Seat(0), o).fuel > 0 && !matches!(o, Order::Refuel { .. })).collect();
    assert!(spends.is_empty(), "nothing but a Refuel takes Fuel from the Stockpile: {spends:?}");
    assert!(
        !orders.iter().any(|o| matches!(o, Order::Transit { ship, .. } if near.ship(*ship).map(|s| s.kind == UnitKind::ColonyShip).unwrap_or(false))),
        "five in the tank flies nowhere: {orders:?}"
    );
    // A full tank at the window, and the crossing is ordered as before.
    let mut at = board(window);
    let orders = at.ai_orders(Seat(0));
    assert!(orders.iter().any(|o| matches!(o, Order::Transit { to: BodyId::Mars, .. })), "a full tank on the window crosses: {orders:?}");
}

/// Ticket #57: a loaded Colony Ship weighs a Body by what its slot is worth less the share of the
/// game the flight would eat, so off the window the Moon, one turn away, beats a Mars that is a
/// year and a half away; before this the AI was only ever offered the single best Body.
/// Ticket #67 (version 0.05.5): with thirty-six turns a nine-turn flight is a quarter of the game,
/// and early on the AI rightly takes it; so the board stands on the turn farthest from the SECOND
/// window, in the last third of the game, where the flight at its cap eats nearly all that is left.
#[test]
fn a_loaded_colony_ship_goes_to_the_moon_when_mars_is_a_year_away() {
    let mut g = game();
    let cust = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Custodians).unwrap();
    let second = g.next_window_turn(g.next_window_turn(1) + 1);
    let last = g.tables.victory.turns;
    g.turn = (second..=last).max_by(|a, b| g.window_offset(*a).abs().partial_cmp(&g.window_offset(*b).abs()).unwrap()).unwrap();
    let (mars_turns, _) = g.transit_cost_for(cust, BodyId::Earth, BodyId::Mars);
    assert!(mars_turns >= 8, "off the window Mars is far: {mars_turns} turns");
    let ship = ShipId(900);
    g.ships.push(Ship { name: String::new(), id: ship, kind: UnitKind::ColonyShip, seat: cust, damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 4, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
    g.seats[cust.index()].stockpile.fuel = 100;
    g.seats[cust.index()].stockpile.energy = 200;
    let orders = g.ai_orders(cust);
    let dest = orders.iter().find_map(|o| match o {
        Order::Transit { ship: s, to, .. } if *s == ship => Some(*to),
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
        name: String::new(),
        kind: UnitKind::ColonyShip,
        seat: Seat(0),
        damage: 0,
        at: ShipAt::Body(body),
        colonists: 4, colonists_education: 1.0,
        army: None,
        stance: Stance::Hold,
        escaped: false,
        arrived_this_turn: false,
        built_turn: turn,
        fuel: 30, slot: None,
    });
    // Ticket #93: a Body with no Colony Slots (Venus) gets slot 0, an order the check will refuse.
    let slot = g.free_slots_on(body).first().copied().unwrap_or(0);
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
    pick_a_tech(&mut g);
    g.end_turn(orders).expect("the turn should end");
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
    pick_a_tech(&mut g);
    g.end_turn(orders).expect("the turn should end");

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
        name: String::new(), id: ship,
        kind: UnitKind::ColonyShip,
        seat,
        damage: 0,
        at: ShipAt::Body(BodyId::Earth),
        colonists: 0, colonists_education: 1.0,
        army: None,
        stance: Stance::Hold,
        escaped: false,
        arrived_this_turn: false,
        built_turn: turn,
        fuel: 30, slot: None,
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
        Order::BuildArchive { colony },
        Order::SetResearchDirective { percent: 100 },
        Order::Repair { unit: UnitRef::Ship(ship), points: 1 },
        Order::RepairWithDucats { unit: UnitRef::Ship(ship), points: 1 },
        Order::Transit { ship, to: BodyId::Moon, slot: None },
        // Ticket #106 (version 0.07.0): a stance that is NOT the default still earns a sentence.
        Order::ShipStance { body: BodyId::Earth, stance: Stance::Attack },
        Order::ArmyStance { place: Place::State(StateId::EastAsia), stance: Stance::Attack },
        Order::MoveArmy { army: ArmyId(0), to: StateId::Europe },
        Order::Load { ship, colonists: 2, from: LoadSource::State(StateId::EastAsia), army: None },
        Order::Load { ship, colonists: 0, from: LoadSource::State(StateId::EastAsia), army: Some(ArmyId(0)) },
        Order::BuildEmigrants { state: StateId::EastAsia, n: 4 },
        Order::SendToAntarctica { state: StateId::EastAsia, n: 2, into: UnloadTarget::Slot(BodyId::Earth, 0) },
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
    // Ticket #106 (version 0.07.0): a rival's paragraph does not report defaults. Hold is what a
    // stack does when nobody tells it otherwise, so it earns no sentence at all -- five of the nine
    // clauses in one sampled paragraph were "set its Armies at X to Hold", and cutting them took
    // 38% off the rival paragraphs of a whole game and 13% off the Report entire.
    assert!(g.rival_deed(seat, &Order::ShipStance { body: BodyId::Earth, stance: Stance::Hold }).is_none());
    assert!(g.rival_deed(seat, &Order::ArmyStance { place: Place::State(StateId::EastAsia), stance: Stance::Hold }).is_none());

    // And a real AI turn's paragraph says what it did, with none of the scored list in it.
    let mut g = game();
    pick_a_tech(&mut g);
    g.end_turn(std::array::from_fn(|_| Vec::new())).expect("the turn should end");
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
        "January 2030. You play the Custodians from China; the computer plays the Prospectors, the Arkwrights and the Archivists."
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
    // Ticket #125 (version 0.07.2): the Arabian Peninsula, untouched at Industry 2 and more populous
    // than Australia, is the third pick now; Australia is the fourth.
    assert_eq!(taken, vec![StateId::EastAsia, StateId::Europe, StateId::ArabianPeninsula, StateId::Australia]);
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
    g.end_turn(std::array::from_fn(|_| Vec::new())).expect("the turn should end");
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
        pick_a_tech(&mut g);
        g.end_turn(std::array::from_fn(|_| Vec::new())).expect("the turn should end");
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
    pick_a_tech(&mut p);
    p.end_turn(std::array::from_fn(|_| Vec::new())).expect("the turn should end");
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
    // Materials for four builds and no more, so the scored list has to RANK the Constabulary above
    // the cheap economic answers rather than reach it once everything else is bought: at its bare
    // weight of 6 it sits under the Research Lab's 8 and is never reached. Ticket #81 (version
    // 0.06.0): a Habitat on the ISS now advances Off-world Presence and ranks above the
    // Constabulary too, so the fourth build is the one this test reads (was three builds, 70).
    g.seats[seat.index()].stockpile.materials = 95;
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

// ---------------------------------------------------------------- Ticket #69 (version 0.05.5): two start Labs, the neutral half, the Sea Wall's Tech on rung 1

/// Ticket #69 (a): North America and South-East Asia begin with a Research Lab ADDED to their start
/// Facilities, and a start Lab stands inland so the sea never takes the world's Research.
#[test]
fn north_america_and_south_east_asia_start_with_an_inland_research_lab() {
    let g = fresh();
    for sid in [StateId::NorthAmerica, StateId::SouthEastAsia] {
        let st = g.state(sid);
        let labs: Vec<&Facility> = st.facilities.iter().filter(|f| f.kind == FacilityKind::ResearchLab).collect();
        assert_eq!(labs.len(), 1, "{sid:?} starts with one Lab: {:?}", st.facilities.iter().map(|f| f.kind).collect::<Vec<_>>());
        assert!(!labs[0].coastal, "{sid:?}'s start Lab stands inland");
    }
    assert_eq!(g.state(StateId::NorthAmerica).facilities.iter().filter(|f| f.kind != FacilityKind::LaunchSite).count(), 4, "added to North America's three");
    assert_eq!(g.state(StateId::SouthEastAsia).facilities.iter().filter(|f| f.kind != FacilityKind::LaunchSite).count(), 3, "added to South-East Asia's two");
    for sid in StateId::ALL {
        if !matches!(sid, StateId::NorthAmerica | StateId::SouthEastAsia) {
            assert!(!g.state(sid).facilities.iter().any(|f| f.kind == FacilityKind::ResearchLab), "{sid:?} starts with no Lab");
        }
    }
}

/// Ticket #69 (b): a Lab in a state nobody holds runs itself, pays no Energy, and pays half its
/// yield (rounded down) into the Tech under research, counting toward nobody's Research Lead; under
/// Occupation the half still flows and the occupier pays the upkeep but draws no Research; held, the
/// Lab is the holder's, whole.
#[test]
fn a_neutral_states_lab_pays_half_its_yield_into_the_tech_and_nobodys_lead() {
    let mut g = game();
    g.pick_tech(Seat(0), TechId::PublicScience).unwrap();
    // The only Lab in the world stands in a neutral North America.
    for st in &mut g.states {
        st.facilities.retain(|f| f.kind != FacilityKind::ResearchLab);
    }
    g.state_mut(StateId::NorthAmerica).control = Control::Neutral;
    g.state_mut(StateId::NorthAmerica).facilities.push(facility(FacilityKind::ResearchLab));
    // A Faction's Lab there makes 3 (2 x 1.076 x 1.5, rounded down); the world gets half of that: 1.
    assert_eq!(g.facility_yield(Seat(2), StateId::NorthAmerica, FacilityKind::ResearchLab).research, 3, "the Arkwrights, at Research x1.0");
    let before = g.research.progress;
    g.income_phase();
    assert_eq!(g.research.progress - before, 1, "half of one Lab, rounded down");
    assert_eq!(g.research.contributions, [0; 4], "and nobody's Lead");
    assert!(g.report.lines.iter().any(|l| l.text.contains("The United States") && l.text.contains("Research")), "the Report says so: {:?}", g.report.lines);
    // Occupied: the half still flows; the occupier pays the 3 Energy and draws nothing from it.
    g.state_mut(StateId::NorthAmerica).control = Control::Occupied { occupier: Seat(2), previous: None, turns: 1 };
    let y = g.facility_yield(Seat(2), StateId::NorthAmerica, FacilityKind::ResearchLab);
    assert_eq!((y.research, y.upkeep), (0, 3), "the occupier pays for a Lab that works for the world");
    let before = g.research.progress;
    g.income_phase();
    assert_eq!(g.research.progress - before, 1);
    assert_eq!(g.research.contributions[2], 0);
    // Held: whole, and the holder's.
    g.state_mut(StateId::NorthAmerica).control = Control::Controlled(Seat(2));
    let before = g.research.progress;
    g.income_phase();
    assert_eq!(g.research.progress - before, 3);
    assert_eq!(g.research.contributions[2], 3, "a held Lab counts toward the Lead as it always did");
}

/// Ticket #69 (c): Coastal Engineering on Industry rung 1, cheaper than the rung it stands on and
/// with no prerequisite, so the Sea Wall can be reached in time; Clean Power and the Sea Wall's own
/// row are untouched. Ticket #117 (version 0.07.1) raised every cost a tenth, 10 to 11, and the
/// figure is pinned beside the relationship that is the actual point of it.
#[test]
fn coastal_engineering_sits_on_rung_one_below_its_rungs_cost_with_no_prerequisite() {
    let g = game();
    let t = g.tables.tech(TechId::CoastalEngineering);
    assert_eq!((t.rung, t.cost), (1, 15), "15 since ticket #231, still below the rung it shares");
    assert!(t.cost < g.tables.tech(TechId::EfficientGrids).cost, "cheaper than the rung it shares, or the Sea Wall arrives too late");
    assert!(t.needs.is_empty(), "no prerequisite: {:?}", t.needs);
    assert!(g.available_techs().contains(&TechId::CoastalEngineering), "pickable from the first turn");
    assert_eq!(g.tables.tech(TechId::CleanPower).needs, vec![TechId::EfficientGrids]);
    // Ticket #242 (version 0.08.3): Green Consensus dropped Clean Power, and with it the whole
    // foot of the Industry branch, at the designer's word -- "Efficient grids is no longer
    // required for green consensus". Pinned because it moves the Custodians' Victory gate:
    // Planetary Stewardship now hangs off Society alone.
    assert_eq!(g.tables.tech(TechId::GreenConsensus).needs, vec![TechId::PublicScience], "Society alone since ticket #242");
    assert_eq!(g.tables.tech(TechId::PlanetaryStewardship).needs, vec![TechId::GreenConsensus], "and the gate above it is unchanged");
    let w = g.tables.facility(FacilityKind::SeaWall);
    assert_eq!((w.materials, w.build_turns), (20, 2), "20 Materials since ticket #77");
}

// ---------------------------------------------------------------- Ticket #72 (version 0.05.5): the Venture Capital Fund, the 15% discount, the Moon's yields

/// Ticket #72 (a): the Prospectors' Facilities and Colony Modules cost 15% less, rounded down, and
/// a building bought for Ducats follows; Space Stations, Ships and Industry Level do not.
#[test]
fn the_prospectors_pay_fifteen_percent_less_for_facilities_and_modules_and_nothing_less_for_the_rest() {
    let mut g = game();
    let pro = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Prospectors).unwrap();
    let cus = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Custodians).unwrap();
    g.take_control(StateId::Europe, pro);
    assert_eq!(g.order_cost(pro, &Order::BuildFacility { state: StateId::Europe, kind: FacilityKind::Factory }).materials, 17, "20 x 0.85 rounded down");
    assert_eq!(g.order_cost(cus, &Order::BuildFacility { state: StateId::EastAsia, kind: FacilityKind::Factory }).materials, 20, "everyone else pays the row");
    // Ticket #83 (version 0.06.0): the Ducat price then takes their 15% off the market too.
    assert_eq!(g.order_cost(pro, &Order::BuildFacilityWithDucats { state: StateId::Europe, kind: FacilityKind::Factory }).ducats, 28, "the Ducat price follows: 17 x 2 = 34, x 0.85 = 28.9");
    let moon = colony(&mut g, pro, BodyId::Moon, &[], 0);
    assert_eq!(g.order_cost(pro, &Order::BuildModule { colony: moon, kind: ModuleKind::Habitat }).materials, 21, "25 x 0.85 rounded down");
    assert_eq!(g.order_cost(pro, &Order::BuildModuleWithDucats { colony: moon, kind: ModuleKind::Habitat }).ducats, 35, "21 x 2 = 42, x 0.85 = 35.7");
    assert_eq!(g.order_cost(pro, &Order::BuildStation { body: BodyId::Moon, slot: 0 }).materials, 40, "a Space Station is not a building of theirs to discount");
    assert_eq!(g.order_cost(pro, &Order::RaiseIndustry { state: StateId::Europe }).materials, 15, "Cheap Industry is its own clause");
}

/// Ticket #72 (b), rewritten on ticket #240 (version 0.08.3): the Venture Capital Fund banks a
/// share of the Prospectors' **Ducat income**, set on any turn in steps of 10% from 0 to 80; a draw
/// returns nine tenths of it in Ducats, rounded down; nobody else has one.
///
/// It banked a share of Materials OUTPUT until version 0.08.3. The designer moved the hoard to
/// Ducats — *"now require duckets rather than materials"* — and chose income over a relabelled
/// share of output, because that makes the Condition a decision the seat takes every turn
/// (bank it or spend it) rather than a consequence of digging.
#[test]
fn the_venture_capital_fund_banks_a_share_of_ducat_income_and_a_draw_returns_nine_tenths() {
    let mut g = game();
    let pro = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Prospectors).unwrap();
    g.take_control(StateId::Europe, pro);
    g.state_mut(StateId::Europe).facilities = vec![facility(FacilityKind::Factory)];
    let moon = colony(&mut g, pro, BodyId::Moon, &[ModuleKind::Mine], 0);
    let _ = moon;
    // Europe's economy pays 14 Ducats a turn to this seat. Materials output is 13 and is now
    // beside the point: a Factory at 4 x 1.25 = 5 and a Mine at 4 x 1.65 x 1.25 = 8.
    let inc = income_of(&mut g, pro);
    assert_eq!((inc.ducats, inc.materials), (14, 13), "at 0% nothing is banked and every Ducat lands");
    assert_eq!(g.seats[pro.index()].venture_fund, 0);
    // The share is an order, refused off the steps and to anyone else.
    assert_eq!(g.check_order(Seat(0), &[], &Order::SetVentureShare { share: 50 }).unwrap_err().0, "only the Prospectors have a Venture Capital Fund");
    // Ticket #256 (version 0.08.4): whole percents, so 15 is a share now; the cap is still 80.
    assert!(g.check_order(pro, &[], &Order::SetVentureShare { share: 15 }).is_ok(), "a whole percent is a step");
    assert!(g.check_order(pro, &[], &Order::SetVentureShare { share: 81 }).is_err(), "over 80");
    assert!(g.check_order(pro, &[], &Order::SetVentureShare { share: 90 }).is_err(), "over 80");
    assert!(g.check_order(pro, &[], &Order::SetVentureShare { share: 0 }).is_ok());
    assert!(g.check_order(pro, &[], &Order::SetVentureShare { share: 80 }).is_ok());
    g.commit_orders(pro, &[Order::SetVentureShare { share: 50 }]);
    assert!((g.seats[pro.index()].venture_share - 0.5).abs() < 1e-9);
    // Half of 14 is 7 to the Fund and 7 to the Stockpile. Materials are untouched by the share now.
    let inc = income_of(&mut g, pro);
    assert_eq!((inc.ducats, inc.materials), (7, 13));
    assert_eq!(g.seats[pro.index()].venture_fund, 7);
    assert!(g.seat(pro).income_sources.iter().any(|(name, r, n)| name.contains("Venture Capital Fund") && *r == Resource::Ducats && *n == -7), "{:?}", g.seat(pro).income_sources);
    // Ducats taken by SELLING are not income, so the Fund never sees them. This is the clause that
    // kept a Materials fund honest -- bought Materials were not output -- pointed at the new
    // resource: a hoard you can fill by trading is not a hoard.
    let fund = g.seats[pro.index()].venture_fund;
    g.seats[pro.index()].stockpile.materials = 100;
    g.commit_orders(pro, &[Order::Sell { resource: Resource::Materials, amount: 10 }]);
    assert_eq!(g.seats[pro.index()].venture_fund, fund, "a sale banks nothing");
    // 80% of 14 is 11, rounded down: 3 lands, 11 banks, taking the Fund to 18.
    g.commit_orders(pro, &[Order::SetVentureShare { share: 80 }]);
    let inc = income_of(&mut g, pro);
    assert_eq!(inc.ducats, 3);
    assert_eq!(g.seats[pro.index()].venture_fund, 18);
    // A draw: 10 out, 9 back IN DUCATS; never more than the Fund holds.
    let before = g.seats[pro.index()].stockpile.ducats;
    assert!(g.check_order(pro, &[], &Order::DrawVenture { amount: 19 }).is_err(), "the Fund holds 18");
    g.commit_orders(pro, &[Order::DrawVenture { amount: 10 }]);
    assert_eq!(g.seats[pro.index()].venture_fund, 8);
    assert_eq!(g.seats[pro.index()].stockpile.ducats, before + 9, "nine tenths come back, in Ducats");
    assert_eq!(g.check_order(Seat(0), &[], &Order::DrawVenture { amount: 1 }).unwrap_err().0, "only the Prospectors have a Venture Capital Fund");
}

/// Ticket #72 (c): the Fund is the first part of the Prospectors' condition, the running Extraction
/// Total being retired, and the ranking reads the Fund over the bar. Ticket #182 (version 0.08.0)
/// moved that bar from 750 to 1000, because the Investment Bank pays uncapped interest into the Fund
/// and, measured over 40 games, nobody had ever reached 750 at all.
#[test]
fn twenty_five_hundred_ducats_in_the_fund_is_the_prospectors_first_part() {
    let mut g = game();
    let pro = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Prospectors).unwrap();
    let card = g.tables.faction(FactionKind::Prospectors).victory_first;
    assert_eq!((card.kind, card.bar), (dying_earth_engine::data::VictoryFirstKind::VentureFund, 2500.0));
    assert_eq!(card.kind.name(), "Venture Capital Fund");
    g.seats[pro.index()].venture_fund = 1000;
    let p = g.progress(pro);
    assert_eq!((p.first_value, p.first_bar), (1000.0, 2500.0));
    assert!((p.first_fraction() - 0.4).abs() < 1e-9);
    colony(&mut g, pro, BodyId::Moon, &[ModuleKind::Habitat, ModuleKind::Habitat, ModuleKind::Habitat], 12);
    g.seats[pro.index()].venture_fund = 2500;
    open_gates(&mut g);
    assert!(g.progress(pro).met());
    g.end_phase();
    assert!(matches!(g.outcome, Some(Outcome::Win { seat, .. }) if seat == pro), "{:?}", g.outcome);
}

/// Ticket #72 (d): the Moon's four yields up a tenth.
#[test]
fn the_moons_four_yields_are_up_a_tenth() {
    let g = game();
    let m = g.tables.body(BodyId::Moon);
    assert!((m.mine_yield - 1.65).abs() < 1e-9 && (m.generator_yield - 1.375).abs() < 1e-9 && (m.refinery_yield - 0.55).abs() < 1e-9 && (m.research_yield - 1.0).abs() < 1e-9, "{:?}", (m.mine_yield, m.generator_yield, m.refinery_yield, m.research_yield));
}

/// Ticket #72 (e): the Prospector AI plays the share as a strategy: nothing before the pace's first
/// waypoint (turn 9: it builds first), then the smallest step that reaches 750 by turn 34 at its
/// current output, and 80 when nothing less will do.
#[test]
fn the_prospector_ai_sets_its_share_to_reach_the_fund_in_time_and_maxes_it_when_behind() {
    let mut g = game();
    let pro = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Prospectors).unwrap();
    g.take_control(StateId::Europe, pro);
    // Every slot full, so nothing is being saved for; a fat output, so a small share suffices.
    while g.free_slots(StateId::Europe) > 0 {
        g.state_mut(StateId::Europe).facilities.push(facility(FacilityKind::Factory));
    }
    // Ticket #240 (version 0.08.3): DUCAT income is what the share is weighed against now.
    // Ticket #256 (version 0.08.4): 120 to 200. At the bar of 2500, 120 a turn over the 24 turns left
    // wants 87% and the seat maxes -- which is the AI READING the new bar, watched here when the
    // bar moved and this line had not.
    g.seats[pro.index()].income_last_turn.ducats = 200;
    g.seats[pro.index()].stockpile.energy = 500;
    g.turn = 4;
    let orders = g.ai_orders(pro);
    assert!(!orders.iter().any(|o| matches!(o, Order::SetVentureShare { .. })), "before turn 9 it builds and banks nothing: {orders:?}");
    g.turn = 10;
    let orders = g.ai_orders(pro);
    let share = orders.iter().find_map(|o| if let Order::SetVentureShare { share } = o { Some(*share) } else { None });
    assert!(matches!(share, Some(s) if s > 0 && s < 80), "a modest share from turn 9 on a fat output: {share:?} in {orders:?}");
    // Late and far behind, everything it may: 80.
    g.turn = 30;
    g.seats[pro.index()].venture_share = 0.0;
    g.seats[pro.index()].income_last_turn.ducats = 10;
    let orders = g.ai_orders(pro);
    let share = orders.iter().find_map(|o| if let Order::SetVentureShare { share } = o { Some(*share) } else { None });
    assert_eq!(share, Some(80), "{orders:?}");
}

// ---------------------------------------------------------------- Ticket #76 (version 0.05.5): the four new Events

/// Ticket #76: a Drought halves a state's Facilities at the next Income, once, and raises its
/// Unrest by 1 as a climate source; Green Consensus blunts it.
#[test]
fn a_drought_halves_a_states_facilities_at_the_next_income_and_raises_its_unrest_by_one() {
    let mut g = game();
    calm(&mut g);
    g.take_control(StateId::Europe, Seat(0));
    g.state_mut(StateId::Europe).facilities = vec![facility(FacilityKind::Factory)];
    assert_eq!(income_of(&mut g, Seat(0)).materials, 4, "a Custodian Factory makes 4");
    drawn(&mut g, EventId::Drought, EventTarget::State(StateId::Europe));
    g.apply_event_now();
    assert_eq!(g.state(StateId::Europe).unrest, 1.0, "Unrest rose by 1");
    assert!(g.state(StateId::Europe).drought);
    assert_eq!(income_of(&mut g, Seat(0)).materials, 2, "half of 4 at the next Income");
    assert!(!g.state(StateId::Europe).drought, "and the Drought is spent");
    assert_eq!(income_of(&mut g, Seat(0)).materials, 4, "one Income only");
    // Green Consensus blunts it entirely.
    with_tech(&mut g, TechId::GreenConsensus);
    drawn(&mut g, EventId::Drought, EventTarget::State(StateId::Europe));
    g.apply_event_now();
    assert!(!g.state(StateId::Europe).drought);
    assert_eq!(g.state(StateId::Europe).unrest, 1.0, "no further rise");
}

/// Ticket #76: a Volcanic Eruption takes 5 ppm from the CO2 Stock at once, scaled like every
/// Climate card (x1.00 at +1.2 C).
#[test]
fn a_volcanic_eruption_takes_five_ppm_from_the_co2_stock_at_once() {
    let mut g = game();
    let before = g.climate.co2;
    drawn(&mut g, EventId::VolcanicEruption, EventTarget::Everyone);
    g.apply_event_now();
    assert!((before - g.climate.co2 - 5.0).abs() < 1e-9, "{} to {}", before, g.climate.co2);
    // Hotter, it scales: at +2.2 the scale is x1.50, so 7.5 ppm.
    hold_temperature(&mut g, 2.2);
    let before = g.climate.co2;
    drawn(&mut g, EventId::VolcanicEruption, EventTarget::Everyone);
    g.apply_event_now();
    assert!((before - g.climate.co2 - 7.5).abs() < 1e-9, "{} to {}", before, g.climate.co2);
}

/// Ticket #76: a Moonquake idles every Module on the Moon until the next Resolution and touches
/// nothing on Mars; Closed-Loop Colonies makes it nothing.
#[test]
fn a_moonquake_idles_every_module_on_the_moon_and_nothing_on_mars() {
    let mut g = game();
    let moon = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Mine, ModuleKind::Generator], 0);
    let mars = colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Mine], 0);
    drawn(&mut g, EventId::Moonquake, EventTarget::Body(BodyId::Moon));
    g.apply_event_now();
    assert!(g.colony(moon).unwrap().modules.iter().all(|m| !m.online), "the Moon is dark");
    assert!(g.colony(mars).unwrap().modules.iter().all(|m| m.online), "Mars is untouched");
    // Closed-Loop Colonies: nothing.
    for m in &mut g.colony_mut(moon).unwrap().modules {
        m.online = true;
    }
    g.colony_mut(moon).unwrap().grid_failed = false;
    with_tech(&mut g, TechId::ClosedLoopColonies);
    drawn(&mut g, EventId::Moonquake, EventTarget::Body(BodyId::Moon));
    g.apply_event_now();
    assert!(g.colony(moon).unwrap().modules.iter().all(|m| m.online), "immune");
}

/// Ticket #76: a Helium-3 Vein doubles the Moon's Generators for two turns, as Rich Seam does for a
/// Body's Mines, and Efficient Grids makes it x3.
#[test]
fn a_helium_three_vein_doubles_the_moons_generators_for_two_turns_and_triples_with_efficient_grids() {
    let mut g = game();
    let moon = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Generator], 0);
    let _ = moon;
    let plain = income_of(&mut g, Seat(0)).energy;
    drawn(&mut g, EventId::HeliumVein, EventTarget::Body(BodyId::Moon));
    g.apply_event_now();
    assert_eq!(g.discoveries.len(), 1);
    assert_eq!((g.discoveries[0].body, g.discoveries[0].kind, g.discoveries[0].multiplier), (BodyId::Moon, ModuleKind::Generator, 2.0));
    // A Moon Generator makes 5 x 1.375 = 6 (rounded down); doubled, 13: seven more Energy.
    let boosted = income_of(&mut g, Seat(0)).energy;
    assert_eq!(boosted - plain, 7, "{plain} to {boosted}");
    income_of(&mut g, Seat(0));
    assert!(g.discoveries.is_empty(), "two Incomes and it is spent");
    with_tech(&mut g, TechId::EfficientGrids);
    drawn(&mut g, EventId::HeliumVein, EventTarget::Body(BodyId::Moon));
    g.apply_event_now();
    assert_eq!(g.discoveries[0].multiplier, 3.0, "x3 with Efficient Grids");
}

// ---------------------------------------------------------------- Ticket #73 (version 0.05.5): Emigrants

/// Ticket #73 (a): Colonists are built. Up to four Emigrants a turn per Faction muster in one state
/// it directs, at 0.1 population each, landing on the card at End Turn (a turn to muster: nothing
/// lifts them the turn they are ordered), and the batch takes 0.5 off the state's Unrest. Coach Class:
/// eight a turn at twice the population.
#[test]
fn emigrants_muster_four_a_turn_per_faction_in_one_state_at_one_unit_of_population_each_and_calm_it() {
    let mut g = game();
    calm(&mut g);
    g.state_mut(StateId::EastAsia).unrest = 3.0;
    let pop = g.state(StateId::EastAsia).population;
    let build = Order::BuildEmigrants { state: StateId::EastAsia, n: 4 };
    assert!(g.check_order(Seat(0), &[], &build).is_ok());
    assert!(g.check_order(Seat(0), &[], &Order::BuildEmigrants { state: StateId::EastAsia, n: 5 }).is_err(), "four a turn");
    assert!(g.check_order(Seat(0), std::slice::from_ref(&build), &Order::BuildEmigrants { state: StateId::Europe, n: 1 }).is_err(), "one state a turn");
    assert!(g.check_order(Seat(1), &[], &Order::BuildEmigrants { state: StateId::EastAsia, n: 1 }).is_err(), "not your state");
    assert!(g.check_order(Seat(0), &[], &Order::Load { ship: ShipId(999), colonists: 1, from: LoadSource::State(StateId::EastAsia), army: None }).is_err(), "nothing waits yet");
    g.commit_orders(Seat(0), &[build]);
    assert_eq!(g.state(StateId::EastAsia).emigrants, 4, "on the card at End Turn");
    assert!((pop - g.state(StateId::EastAsia).population - 4.0).abs() < 1e-9, "one unit of five million each (ticket #143)");
    assert_eq!(g.state(StateId::EastAsia).unrest, 2.5, "the batch took 0.5 off");
    assert!(g.log.to_vec().iter().any(|l| l.contains("Pioneers recruited in China")), "{:?}", g.log.to_vec());
    // Coach Class: eight a turn at twice the population.
    assert_eq!(g.emigrants_per_turn(Seat(0)), 4);
    assert_eq!(g.emigrants_per_turn(Seat(2)), 8, "the Arkwrights recruit eight");
    assert!((g.lift_population(Seat(2), 8) - 16.0).abs() < 1e-9, "at twice the population");
}

/// Ticket #73 (b): a Launch Site lifts only the Emigrants waiting in its state; the population was
/// paid when they mustered, and a lift is still a launch.
#[test]
fn a_launch_site_lifts_only_the_emigrants_waiting_in_its_state() {
    let mut g = game();
    let ship = a_colony_ship(&mut g, Seat(0), BodyId::Earth);
    let load = Order::Load { ship, colonists: 2, from: LoadSource::State(StateId::EastAsia), army: None };
    assert_eq!(g.check_order(Seat(0), &[], &load).unwrap_err().0, "only 0 Pioneers are waiting there");
    g.state_mut(StateId::EastAsia).emigrants = 3;
    let pop = g.state(StateId::EastAsia).population;
    assert!(g.check_order(Seat(0), &[], &load).is_ok());
    g.commit_orders(Seat(0), std::slice::from_ref(&load));
    assert_eq!(g.climate.launches_pending[0], 1, "a lift is still a launch");
    g.resolution_phase();
    assert_eq!(g.state(StateId::EastAsia).emigrants, 1, "the lift took two of the three");
    assert_eq!(g.ship(ship).unwrap().colonists, 2);
    assert!((g.state(StateId::EastAsia).population - pop).abs() < 1e-9, "the lift takes no population");
}

/// Ticket #73 (c): Emigrants go to Antarctica by sea from any state the Faction directs, a turn to
/// arrive, no launch, founding a Colony in an open slot or joining one of the Faction's own; the ice
/// Ticket #141 (version 0.07.3): waiting Emigrants lift straight onto the seat's own station over
/// Earth, from a state with a working Launch Site, as many as the station has Habitat room for.
/// It is a launch, they are aboard at this Resolution, a rival's station and a blockaded slot both
/// refuse it, and the same Emigrants cannot be sent twice.
#[test]
fn emigrants_lift_straight_to_a_station_over_earth_by_a_launch_site() {
    let mut g = game();
    calm(&mut g);
    let iss = station_of(&g, Seat(0), BodyId::Earth).expect("the Custodians start with a station");
    if !g.colony(iss).unwrap().modules.iter().any(|m| m.kind == ModuleKind::Habitat) {
        g.colony_mut(iss).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    }
    let room = g.habitat_room(g.colony(iss).unwrap());
    assert!(room >= 8, "a Habitat's room: {room}");
    let home = g.controlled_states(Seat(0))[0];
    assert!(g.state(home).facilities.iter().any(|f| f.kind == FacilityKind::LaunchSite && f.working()), "the start state has a Launch Site");
    g.state_mut(home).emigrants = 6;
    let lift = Order::LiftToStation { state: home, n: 4, colony: iss };
    assert!(g.check_order(Seat(0), &[], &lift).is_ok(), "{:?}", g.check_order(Seat(0), &[], &lift));
    assert!(g.check_order(Seat(0), &[], &Order::LiftToStation { state: home, n: 7, colony: iss }).unwrap_err().0.contains("waiting"), "six waiting");
    assert!(g.check_order(Seat(0), &[], &Order::LiftToStation { state: home, n: room + 1, colony: iss }).is_err(), "no more than the room");
    let tiangong = station_of(&g, Seat(1), BodyId::Earth).expect("the Prospectors start with a station");
    assert!(g.check_order(Seat(0), &[], &Order::LiftToStation { state: home, n: 1, colony: tiangong }).unwrap_err().0.contains("not your station"), "own stations only");
    let pending = [lift.clone()];
    assert!(g.check_order(Seat(0), &pending, &Order::LiftToStation { state: home, n: 3, colony: iss }).is_err(), "four of the six are already bound: two remain");
    // Without a Launch Site nothing lifts.
    let sites: Vec<_> = g.state(home).facilities.iter().filter(|f| f.kind == FacilityKind::LaunchSite).cloned().collect();
    g.state_mut(home).facilities.retain(|f| f.kind != FacilityKind::LaunchSite);
    assert!(g.check_order(Seat(0), &[], &lift).unwrap_err().0.contains("Launch Site"), "a lift wants a rocket");
    g.state_mut(home).facilities.extend(sites);
    // It is a launch, and they are aboard at this Resolution.
    let launches = g.climate.launches_pending[0];
    let before = g.colony(iss).unwrap().colonists;
    g.commit_orders(Seat(0), std::slice::from_ref(&lift));
    assert_eq!(g.state(home).emigrants, 2, "they have left");
    assert_eq!(g.climate.launches_pending[0], launches + 1, "a lift is a launch");
    assert_eq!(g.colony(iss).unwrap().colonists, before + 4, "aboard now");
    assert!(g.log.to_vec().iter().any(|l| l.contains("4 Pioneers lifted from")), "{:?}", g.log.to_vec());
}

/// must be open.
#[test]
fn emigrants_go_to_antarctica_by_sea_from_any_state_and_arrive_a_turn_later() {
    let mut g = game();
    calm(&mut g);
    g.take_control(StateId::Europe, Seat(0));
    g.state_mut(StateId::Europe).facilities.retain(|f| f.kind != FacilityKind::LaunchSite);
    g.state_mut(StateId::Europe).emigrants = 6;
    let slot = g.free_slots_on(BodyId::Earth)[0];
    let send = Order::SendToAntarctica { state: StateId::Europe, n: 4, into: UnloadTarget::Slot(BodyId::Earth, slot) };
    assert!(g.check_order(Seat(0), &[], &send).unwrap_err().0.contains("has not opened"), "shut ice");
    g.antarctica_open = true;
    assert!(g.check_order(Seat(0), &[], &send).is_ok(), "no Launch Site needed: they go by sea");
    assert!(g.check_order(Seat(0), &[], &Order::SendToAntarctica { state: StateId::Europe, n: 7, into: UnloadTarget::Slot(BodyId::Earth, slot) }).is_err(), "six waiting");
    assert!(g.check_order(Seat(1), &[], &send).is_err(), "not your state");
    let launches = g.climate.launches_pending[0];
    g.commit_orders(Seat(0), std::slice::from_ref(&send));
    assert_eq!(g.state(StateId::Europe).emigrants, 2, "they have left");
    assert_eq!(g.climate.launches_pending[0], launches, "no launch: the sea");
    g.resolution_phase();
    assert!(g.colonies.iter().all(|c| c.body != BodyId::Earth || c.in_orbit), "a turn to arrive");
    g.turn += 1;
    g.resolution_phase();
    let col = g.colonies.iter().find(|c| c.body == BodyId::Earth && !c.in_orbit).expect("founded").clone();
    assert_eq!((col.slot, col.colonists, col.control), (slot, 4, Control::Controlled(Seat(0))));
    assert!(g.log.to_vec().iter().any(|l| l.contains("in Antarctica with 4 Pioneers from The European Union")), "{:?}", g.log.to_vec());
    // The last two join it, once it has room.
    g.colony_mut(col.id).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    g.commit_orders(Seat(0), &[Order::SendToAntarctica { state: StateId::Europe, n: 2, into: UnloadTarget::Colony(col.id) }]);
    g.resolution_phase();
    assert_eq!(g.colony(col.id).unwrap().colonists, 4, "not yet");
    g.turn += 1;
    g.resolution_phase();
    assert_eq!(g.colony(col.id).unwrap().colonists, 6, "joined");
    assert_eq!(g.state(StateId::Europe).emigrants, 0);
}

/// Ticket #73 (d): the AI musters before it can load, loads what waits, and with the ice open sends
/// what waits to Antarctica by sea.
#[test]
fn the_ai_musters_emigrants_then_lifts_them_or_sends_them_to_antarctica() {
    let mut g = game();
    let ship = a_colony_ship(&mut g, Seat(0), BodyId::Earth);
    let _ = ship;
    g.seats[0].stockpile.energy = 200;
    let orders = g.ai_orders(Seat(0));
    assert!(orders.iter().any(|o| matches!(o, Order::BuildEmigrants { state: StateId::EastAsia, .. })), "an empty Colony Ship at Earth and nobody waiting: it recruits: {orders:?}");
    assert!(!orders.iter().any(|o| matches!(o, Order::Load { .. })), "and cannot load yet: {orders:?}");
    g.state_mut(StateId::EastAsia).emigrants = 4;
    // Ticket #164 (version 0.07.5): with a station that holds four from the day it stands, the lift
    // straight to orbit is the cheaper way and the computer takes it first.
    let orders = g.ai_orders(Seat(0));
    let iss = station_of(&g, Seat(0), BodyId::Earth).unwrap();
    assert!(orders.iter().any(|o| matches!(o, Order::LiftToStation { state: StateId::EastAsia, n: 4, colony } if *colony == iss)), "it lifts them to the station: {orders:?}");
    // With the station full there is nowhere to lift them, and the Colony Ship takes them instead.
    let room = g.habitat_room(g.colony(iss).unwrap());
    g.colony_mut(iss).unwrap().colonists = room;
    let orders = g.ai_orders(Seat(0));
    assert!(orders.iter().any(|o| matches!(o, Order::Load { colonists: 4, from: LoadSource::State(StateId::EastAsia), .. })), "it loads the four: {orders:?}");
    // The ice open and Emigrants waiting with no Ship to take them: by sea.
    let mut g = game();
    g.antarctica_open = true;
    g.state_mut(StateId::EastAsia).emigrants = 4;
    g.seats[0].stockpile.energy = 200;
    // Ticket #164 (version 0.07.5): the station over Earth holds four from the day it stands and
    // would take them first, so fill it; this reading is about the sea.
    let iss = station_of(&g, Seat(0), BodyId::Earth).unwrap();
    let room = g.habitat_room(g.colony(iss).unwrap());
    g.colony_mut(iss).unwrap().colonists = room;
    let orders = g.ai_orders(Seat(0));
    assert!(orders.iter().any(|o| matches!(o, Order::SendToAntarctica { state: StateId::EastAsia, n: 4, .. })), "{orders:?}");
}

// ---------------------------------------------------------------- Ticket #75 (version 0.05.5): the undefended home state

/// Ticket #75, rewritten on ticket #114 (version 0.07.1) to the shared Defence rule, and **put back
/// on ticket #134 (version 0.07.3)**, which retired Defence for the computer players as well as the
/// button: *"computer players lose it too."* The AI is on its own arithmetic again: once a rival's
/// Standing comes within two steps of its own it pushes as many holds as it takes to stand two steps
/// clear of the rival plus the challenge margin, as many as its Allotment allows; a rival far below
/// gets no answer. The rule it had for two versions asked the sharper question of whether the rival
/// could actually take the place; this one does not, and the sweep on the ticket measures the cost.
#[test]
fn an_ai_holder_pushes_as_many_holds_as_it_takes_when_a_rival_comes_within_reach() {
    let mut g = game();
    let here = Place::State(StateId::Europe);
    g.take_control(StateId::Europe, Seat(1));
    let margin = g.tables.influence.challenge_margin;
    g.seats[1].influence.insert(here, 30);
    g.seats[0].influence.insert(here, 25);
    g.seats[1].allotment = 40;
    g.seats[1].stockpile.energy = 200;
    let held = |orders: &[Order]| -> i64 { orders.iter().map(|o| match o { Order::Influence { target: Place::State(StateId::Europe), amount } => *amount, _ => 0 }).sum() };
    let orders = g.ai_orders(Seat(1));
    // 25 + the margin + two steps of 5 - 30: as many holds as that takes, within an Allotment of 40.
    let need = (25 + margin + 10 - 30).min(40);
    assert!(held(&orders) >= need, "it held Europe with {} of the {need} it wanted: {orders:?}", held(&orders));
    // A rival far below needs no answer.
    g.seats[0].influence.insert(here, 5);
    let orders = g.ai_orders(Seat(1));
    assert_eq!(held(&orders), 0, "{orders:?}");
}

/// Ticket #75, second round: a Faction begins with a Standing on its start state equal to that
/// state's threshold, a claim on its home from turn 1, and a challenger needs the holder's Standing
/// plus a margin of 20 (10 from version 0.04 until now).
#[test]
fn a_faction_starts_with_a_standing_on_its_home_at_the_threshold_and_the_margin_is_twenty() {
    let g = fresh();
    assert_eq!(g.tables.influence.challenge_margin, 20, "20 since ticket #75; 10 from version 0.04");
    for seat in Seat::ALL {
        let home = g.controlled_states(seat)[0];
        let standing = g.seat(seat).influence.get(&Place::State(home)).copied().unwrap_or(0);
        assert_eq!(standing, g.influence_threshold(Place::State(home)), "{seat:?} starts with a claim on {home:?} at its threshold");
        assert!(standing > 0);
        let rival = seat.others()[0];
        assert_eq!(g.influence_needed_for(rival, Place::State(home)), standing + 20, "a rival needs the holder's Standing plus 20 from turn 1");
    }
}

/// Ticket #75, second round: a state another Faction holds counts 0.3 of a neutral one on the AI's
/// Influence target list, so on the opening board every AI's Influence goes to neutral states.
#[test]
fn the_ai_spends_its_influence_on_neutral_states_while_any_are_worth_having() {
    let mut g = fresh();
    assert!((g.tables.ai.thresholds.held_state_weight - 0.3).abs() < 1e-9, "0.3 since ticket #75; 0.6 before");
    for seat in Seat::ALL.into_iter().skip(1) {
        g.seats[seat.index()].allotment = 20;
        g.seats[seat.index()].stockpile.energy = 200;
        let orders = g.ai_orders(seat);
        let on_held: Vec<&Order> = orders
            .iter()
            .filter(|o| matches!(o, Order::Influence { target: Place::State(s), .. } if g.state(*s).control.controller().map(|c| c != seat).unwrap_or(false)))
            .collect();
        assert!(on_held.is_empty(), "{seat:?} spent Influence on a held state with neutral ones on the board: {on_held:?}");
        assert!(orders.iter().any(|o| matches!(o, Order::Influence { target: Place::State(s), .. } if g.state(*s).control == Control::Neutral)), "{seat:?} spent nothing on a neutral state: {orders:?}");
    }
}

// ---------------------------------------------------------------- 0.06.0 ticket #80: the Observatory

/// Ticket #80: an Observatory makes 2 Research, plus one per cent for every Colonist at its Colony,
/// times the Faction's Research multiplier and Public Science, rounded down; no output multiplier
/// touches it. Ticket #140 (version 0.07.3): times its slot's Research yield too, which on Mars is
/// drawn near 1.3, so the expected figures carry the drawn yield rather than a constant.
#[test]
fn observatory_makes_two_research_plus_a_per_cent_per_colonist() {
    let mut g = game();
    let ark = Seat(3);
    let mars = colony(&mut g, ark, BodyId::Mars, &[ModuleKind::Observatory, ModuleKind::Habitat, ModuleKind::Habitat, ModuleKind::Habitat], 0);
    let science = g.research_yield_at(g.colony(mars).unwrap());
    assert!(science > 1.0, "Mars is where the planetary science is: {science}");
    let off = g.tables.faction(g.kind(ark)).research_multiplier_off_earth.unwrap_or(g.tables.faction(g.kind(ark)).research_multiplier);
    let y = g.module_yield(ark, mars, ModuleKind::Observatory);
    assert_eq!(y.resource, None, "Research is not a Stockpile resource");
    assert_eq!(y.research, (2.0 * science * off).floor() as i64, "2 x the slot's yield x the off-Earth multiplier with no Colonists");
    assert_eq!(y.upkeep, 3);
    g.colony_mut(mars).unwrap().colonists = 25;
    assert_eq!(g.module_yield(ark, mars, ModuleKind::Observatory).research, (2.0 * science * 1.25 * off).floor() as i64, "x1.25 with 25 Colonists");
    with_tech(&mut g, TechId::PublicScience);
    let public = g.tables.tech(TechId::PublicScience).value;
    assert_eq!(g.module_yield(ark, mars, ModuleKind::Observatory).research, (2.0 * science * 1.25 * off * public).floor() as i64, "and Public Science on top");
}

/// Ticket #80: the Observatory's Research reaches the seat's Income, named by its Colony.
#[test]
fn observatory_research_flows_into_the_income() {
    let mut g = game();
    let ark = Seat(3);
    g.seats[ark.index()].stockpile.energy = 100;
    let mars = colony(&mut g, ark, BodyId::Mars, &[ModuleKind::Observatory, ModuleKind::Generator], 0);
    // Ticket #140 (version 0.07.3): the figure carries the slot's drawn Research yield.
    let want = g.module_yield(ark, mars, ModuleKind::Observatory).research;
    assert!(want >= 3, "at least 2 x 1.6 on Mars: {want}");
    let before = g.seat(ark).research_last_turn;
    g.income_phase();
    let s = g.seat(ark);
    assert!(s.research_last_turn >= before + want, "{want} Research from the Observatory, got {}", s.research_last_turn);
    let name = g.place_name(Place::Colony(mars));
    assert!(s.income_sources.iter().any(|(src, r, n)| src == &format!("Observatory in {name}") && *r == Resource::Research && *n == want), "{:?}", s.income_sources);
}

/// Ticket #80: a Space Station holds a Shipyard, Habitats and Observatories; still no Barracks.
#[test]
fn observatory_stands_on_a_station_and_a_barracks_does_not() {
    let mut g = game();
    g.seats[0].stockpile.materials = 200;
    let iss = station_of(&g, Seat(0), BodyId::Earth).expect("the Custodians start with a station");
    // Ticket #164 (version 0.07.5): somebody must live there before anything may be built.
    g.colony_mut(iss).unwrap().colonists = 2;
    assert!(g.check_order(Seat(0), &[], &Order::BuildModule { colony: iss, kind: ModuleKind::Observatory }).is_ok());
    assert!(g.check_order(Seat(0), &[], &Order::BuildModule { colony: iss, kind: ModuleKind::Barracks }).is_err());
}

/// Ticket #80: a Habitat holds 8, 12 for the Arkwrights, +2 with Expanded Habitats; read on a
/// station, where no Body yield applies.
///
/// Ticket #164 (version 0.07.5): the station's own Core Module holds four besides, and holds a FLAT
/// four -- neither the Arkwrights' multiplier nor Expanded Habitats reaches it. So every figure here
/// is the Habitat's, plus the same four.
#[test]
fn a_habitat_holds_four_and_six_for_the_arkwrights_and_the_core_module_four_flat() {
    let mut g = game();
    let iss = station_of(&g, Seat(0), BodyId::Earth).unwrap();
    let core = g.tables.module(ModuleKind::Core).holds_colonists;
    assert_eq!(core, 4);
    assert_eq!(g.habitat_room(g.colony(iss).unwrap()), core, "the Core Module alone, before any Habitat");
    g.colony_mut(iss).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    assert_eq!(g.habitat_room(g.colony(iss).unwrap()), 4 + core);
    // The Arkwrights (seat 2) start with no station: hand them the ISS for the reading.
    g.colony_mut(iss).unwrap().control = Control::Controlled(Seat(2));
    assert_eq!(g.habitat_room(g.colony(iss).unwrap()), 6 + core, "half again for the Arkwrights, and the Core Module flat");
    with_tech(&mut g, TechId::ExpandedHabitats);
    assert_eq!(g.habitat_room(g.colony(iss).unwrap()), 12 + core, "(4 + 4) x 1.5, and the Core Module still four");
}

// ---------------------------------------------------------------- 0.06.0 ticket #81: the Archivists' card

/// Ticket #81: the output nerf is lifted whole (x1.0), Research is x1.5 on Earth and x1.75 off it,
/// and a station over Earth is off Earth while Antarctica is not. Fifty Colonists at each, so the
/// old 1.6 (4.8, so 4) reads apart from 1.75 (5.25, so 5): Axiom over Earth and Mars make 5; with
/// Public Science Antarctica makes 2 x 1.5 x 1.5 x 1.5 = 6.75, so 6, where 1.6 gave 7.2, so 7.
/// A Mars Mine makes 4 x 1.25 x 1.0 = 5, not 4.
#[test]
fn archivists_research_is_one_and_a_half_on_earth_and_one_and_three_quarters_off() {
    let mut g = game();
    let ark = Seat(3);
    let axiom = station_of(&g, ark, BodyId::Earth).expect("the Archivists start with Axiom");
    g.colony_mut(axiom).unwrap().modules.push(Module::new(ModuleKind::Observatory));
    g.colony_mut(axiom).unwrap().colonists = 50;
    // Ticket #140 (version 0.07.3): a station's Observatory reads its Body's Research yield, Earth's
    // 1.0, so Axiom's figure is unchanged; a ground Observatory reads its slot's drawn yield.
    assert_eq!(g.module_yield(ark, axiom, ModuleKind::Observatory).research, 5, "a station over Earth is off Earth: 2 x 1.0 x 1.5 x 1.75 = 5.25");
    let mars = colony(&mut g, ark, BodyId::Mars, &[ModuleKind::Observatory, ModuleKind::Mine], 50);
    let mars_science = g.research_yield_at(g.colony(mars).unwrap());
    assert_eq!(g.module_yield(ark, mars, ModuleKind::Observatory).research, (2.0 * mars_science * 1.5 * 1.75).floor() as i64, "2 x the slot's yield x 1.5 x 1.75");
    assert_eq!(g.module_yield(ark, mars, ModuleKind::Mine).amount, 5, "the output nerf is gone: 4 x 1.25 x 1.0");
    let vostok = colony(&mut g, ark, BodyId::Earth, &[ModuleKind::Observatory], 50);
    let vostok_science = g.research_yield_at(g.colony(vostok).unwrap());
    with_tech(&mut g, TechId::PublicScience);
    let public = g.tables.tech(TechId::PublicScience).value;
    assert_eq!(g.module_yield(ark, vostok, ModuleKind::Observatory).research, (2.0 * vostok_science * 1.5 * 1.5 * public).floor() as i64, "Antarctica is Earth: 2 x the slot's yield x 1.5 x 1.5 x Public Science");
}

/// Ticket #81: Colonists on a station over Earth count as off Earth for Off-world Presence and the
/// Arkwrights' Diaspora count; Antarctica's still do not; Earth is still not one of Diaspora's Bodies.
#[test]
fn a_station_over_earth_is_off_earth_and_antarctica_is_not() {
    let mut g = game();
    let iss = station_of(&g, Seat(0), BodyId::Earth).unwrap();
    g.colony_mut(iss).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    g.colony_mut(iss).unwrap().colonists = 6;
    colony(&mut g, Seat(0), BodyId::Earth, &[ModuleKind::Habitat], 5);
    assert_eq!(g.off_world_colonists(Seat(0)), 6, "the ISS's six count, Antarctica's five do not");
    assert_eq!(g.bodies_settled(Seat(0), 4), 0, "Earth is not a Body for Diaspora, in orbit or on the ice");
}

/// Ticket #81 let the Archive stand on a station over Earth. Ticket #209 (version 0.08.1) takes
/// that back: it must stand on another Body, the Moon included. Earth's orbit and Earth's ice are
/// both refused, and Earth orbit is the only place it had ever stood in 80 measured games.
#[test]
fn the_archive_may_not_stand_over_earth_nor_on_it() {
    let mut g = game();
    let axiom = station_of(&g, Seat(3), BodyId::Earth).unwrap();
    assert!(!g.may_hold_archive(g.colony(axiom).unwrap()), "a station over Earth will no longer do");
    let vostok = colony(&mut g, Seat(3), BodyId::Earth, &[ModuleKind::Habitat], 0);
    assert!(!g.may_hold_archive(g.colony(vostok).unwrap()), "nor will Antarctica");
    let luna = colony(&mut g, Seat(3), BodyId::Moon, &[ModuleKind::Habitat], 0);
    assert!(g.may_hold_archive(g.colony(luna).unwrap()), "the Moon counts: it is another Body");
    let mars = colony(&mut g, Seat(3), BodyId::Mars, &[ModuleKind::Habitat], 0);
    assert!(g.may_hold_archive(g.colony(mars).unwrap()), "and so does Mars");
}

// ---------------------------------------------------------------- 0.06.0 ticket #82: the Custodians' card

/// Ticket #82: a mothballed Custodian Factory on Earth doubles their most productive undoubled
/// Mine off Earth, one for one; Antarctica is not off Earth; a Restart ends it. The Moon's Mine
/// makes 4 x 1.65 = 6, Mars's 4 x 1.25 = 5, Lake Vostok's 4 x 1.75 = 7 but on Earth.
#[test]
fn a_custodian_mothballed_factory_doubles_their_best_mine_off_earth_one_for_one() {
    let mut g = game();
    let cus = Seat(0);
    assert_eq!(g.kind(cus), FactionKind::Custodians);
    let moon = colony(&mut g, cus, BodyId::Moon, &[ModuleKind::Mine], 0);
    let mars = colony(&mut g, cus, BodyId::Mars, &[ModuleKind::Mine], 0);
    let vostok = colony(&mut g, cus, BodyId::Earth, &[ModuleKind::Mine], 0);
    let st = g.state_mut(StateId::EastAsia);
    st.facilities.push(facility(FacilityKind::Factory));
    st.facilities.push(facility(FacilityKind::Factory));
    assert!(g.doubled_modules(cus).is_empty(), "no idle Factory, no bonus");
    assert_eq!(g.module_yield_at(cus, moon, 0).amount, 6);
    let i = g.state(StateId::EastAsia).facilities.len() - 1;
    g.state_mut(StateId::EastAsia).facilities[i].mothballed = true;
    assert_eq!(g.doubled_modules(cus), vec![(moon, 0)], "one idle Factory, the best Mine off Earth");
    let y = g.module_yield_at(cus, moon, 0);
    assert_eq!(y.amount, 12, "6 doubled");
    assert_eq!(y.doubled_by, Some("Factory"));
    assert_eq!(g.module_yield_at(cus, mars, 0).amount, 5, "the second Mine is not doubled");
    assert_eq!(g.module_yield_at(cus, vostok, 0).amount, 7, "Antarctica is Earth");
    g.state_mut(StateId::EastAsia).facilities[i - 1].mothballed = true;
    assert_eq!(g.doubled_modules(cus), vec![(moon, 0), (mars, 0)], "two idle Factories, two Mines, still not Antarctica");
    g.state_mut(StateId::EastAsia).facilities[i].mothballed = false;
    g.state_mut(StateId::EastAsia).facilities[i - 1].mothballed = false;
    assert!(g.doubled_modules(cus).is_empty(), "a Restart ends it");
}

/// Ticket #82: the pairs are Factory/Mine, Power Plant/Generator, Refinery/Refinery and Research
/// Lab/Observatory, and the doubling lands on the final figure; the Prospectors get none of it.
#[test]
fn the_custodian_pairs_and_nobody_elses() {
    let mut g = game();
    let cus = Seat(0);
    let moon = colony(&mut g, cus, BodyId::Moon, &[ModuleKind::Generator, ModuleKind::Refinery, ModuleKind::Observatory], 20);
    let st = g.state_mut(StateId::EastAsia);
    for k in [FacilityKind::PowerPlant, FacilityKind::Refinery, FacilityKind::ResearchLab, FacilityKind::Bank] {
        let mut f = facility(k);
        f.mothballed = true;
        st.facilities.push(f);
    }
    let mut d = g.doubled_modules(cus);
    d.sort();
    assert_eq!(d, vec![(moon, 0), (moon, 1), (moon, 2)]);
    with_tech(&mut g, TechId::EfficientGrids);
    // Generator 5 x 1.375 x 1.5 = 10.3, so 10, doubled 20; Refinery 3 x 0.55 = 1.65, so 1, doubled 2;
    // Observatory 2 x 1.2 x 1.25 = 3, doubled 6.
    assert_eq!(g.module_yield_at(cus, moon, 0).amount, 20);
    assert_eq!(g.module_yield_at(cus, moon, 1).amount, 2);
    assert_eq!(g.module_yield_at(cus, moon, 2).research, 6);
    let pro = Seat(1);
    let theirs = colony(&mut g, pro, BodyId::Moon, &[ModuleKind::Mine], 0);
    let sid = g.controlled_states(pro)[0];
    let mut f = facility(FacilityKind::Factory);
    f.mothballed = true;
    g.state_mut(sid).facilities.push(f);
    assert!(g.doubled_modules(pro).is_empty(), "only the Custodians");
    assert_eq!(g.module_yield_at(pro, theirs, 0).doubled_by, None);
}

/// Ticket #82: the doubled figure is what the Income pays, named for what doubled it.
#[test]
fn the_income_pays_the_doubled_mine() {
    let mut g = game();
    let cus = Seat(0);
    g.seats[0].stockpile.energy = 100;
    let moon = colony(&mut g, cus, BodyId::Moon, &[ModuleKind::Mine, ModuleKind::Generator], 0);
    let mut f = facility(FacilityKind::Factory);
    f.mothballed = true;
    g.state_mut(StateId::EastAsia).facilities.push(f);
    g.income_phase();
    let name = g.place_name(Place::Colony(moon));
    assert!(g.seat(cus).income_sources.iter().any(|(src, r, n)| src.starts_with(&format!("Mine in {name}")) && src.contains("doubled") && *r == Resource::Materials && *n == 12), "{:?}", g.seat(cus).income_sources);
    assert_eq!(g.seat(cus).doubled_module_turns, 1);
}

/// Ticket #82: the Custodians' Influence multiplier is 1.2 (1.25 before).
#[test]
fn the_custodians_influence_multiplier_is_one_point_two() {
    let g = game();
    assert!((g.tables.faction(FactionKind::Custodians).influence_multiplier - 1.2).abs() < 1e-9);
}

/// Ticket #82: the Custodian AI idles a Factory once an undoubled Mine off Earth outproduces it,
/// and does not restart a Factory whose doubling stands, even with Energy to spare. East Asia
/// leans Materials, so its Factory makes 4 x 1.5 = 6; a Phobos Mine makes 4 x 1.75 = 7.
#[test]
fn the_custodian_ai_idles_a_factory_a_phobos_mine_outproduces_and_keeps_it_idle() {
    let mut g = game();
    calm(&mut g);
    let cus = Seat(0);
    g.seats[0].stockpile.energy = 500;
    g.seats[0].stockpile.materials = 10;
    colony(&mut g, cus, BodyId::Phobos, &[ModuleKind::Mine, ModuleKind::Generator], 0);
    g.state_mut(StateId::EastAsia).facilities.push(facility(FacilityKind::Factory));
    let i = g.state(StateId::EastAsia).facilities.len() - 1;
    let orders = g.ai_orders(cus);
    assert!(
        orders.iter().any(|o| matches!(o, Order::Change { building: BuildingRef::Facility(StateId::EastAsia, j), what: BuildingChange::Mothball } if *j == i)),
        "no mothball of the Factory a Phobos Mine (7) outproduces (6): {orders:?}"
    );
    g.state_mut(StateId::EastAsia).facilities[i].mothballed = true;
    let orders = g.ai_orders(cus);
    assert!(
        !orders.iter().any(|o| matches!(o, Order::Change { building: BuildingRef::Facility(StateId::EastAsia, j), what: BuildingChange::Restart } if *j == i)),
        "the doubling stands, so no restart: {orders:?}"
    );
}

// ---------------------------------------------------------------- 0.06.0 ticket #83: the Prospectors' Ducats and market; the Arkwrights' Ships

/// Ticket #83: a Nation State the Prospectors control pays its GDP income x1.2, rounded down;
/// everyone else's is unchanged, and so are Banks and Trade Posts (they keep the general x1.25).
#[test]
fn the_prospectors_states_pay_their_ducats_at_one_point_two() {
    let mut g = game();
    let pro = Seat(1);
    assert_eq!(g.kind(pro), FactionKind::Prospectors);
    let sid = StateId::NorthAmerica;
    g.state_mut(sid).industry_level = 10;
    let gdp = g.tables.state(sid).gdp;
    g.state_mut(sid).control = Control::Controlled(Seat(0));
    // Ticket #139 (version 0.07.3): the divisor is 5 now, so Industry 10 pays twice the gdp.
    assert_eq!(g.state_ducats(sid), 2 * gdp, "the Custodians: gdp x 10 / 5");
    g.state_mut(sid).control = Control::Controlled(pro);
    assert_eq!(g.state_ducats(sid), (2.0 * gdp as f64 * 1.2).floor() as i64, "the Prospectors: x1.2");
}

/// Ticket #83: the Prospectors buy Materials, Fuel, Energy and outright buildings at 15% off,
/// rounded down over the lot; Influence and selling are untouched; nobody else gets it.
#[test]
fn the_prospectors_buy_at_fifteen_per_cent_off() {
    let g = game();
    let (cus, pro) = (Seat(0), Seat(1));
    let buy = |r, n| Order::Buy { resource: r, amount: n };
    assert_eq!(g.order_cost(cus, &buy(Resource::Materials, 10)).ducats, 30);
    assert_eq!(g.order_cost(pro, &buy(Resource::Materials, 10)).ducats, 25, "30 x 0.85 = 25.5");
    assert_eq!(g.order_cost(pro, &buy(Resource::Fuel, 5)).ducats, 17, "20 x 0.85 = 17");
    assert_eq!(g.order_cost(pro, &buy(Resource::Energy, 10)).ducats, 17, "20 x 0.85 = 17");
    assert_eq!(g.order_cost(pro, &Order::BuyInfluence { amount: 5 }).ducats, 10, "Influence is not a commodity");
    assert_eq!(g.order_cost(pro, &Order::Sell { resource: Resource::Materials, amount: 10 }).ducats, -15, "selling is not discounted: half of 3 each, undiscounted");
    let sid = g.controlled_states(pro)[0];
    let outright = Order::BuildFacilityWithDucats { state: sid, kind: FacilityKind::Factory };
    assert_eq!(g.order_cost(pro, &outright).ducats, 28, "17 Materials x 2 = 34, x 0.85 = 28.9");
}

/// Ticket #83: the Arkwrights' Ships cost 15% less, rounded down, the Colony Ship from the common
/// 30 (their 20 retired): 25, 21, 25, 42; everyone else pays the card.
#[test]
fn the_arkwrights_ships_cost_fifteen_per_cent_less() {
    let g = game();
    let ark = Seat(2);
    assert_eq!(g.kind(ark), FactionKind::Arkwrights);
    assert_eq!(g.ship_materials(ark, UnitKind::ColonyShip), 25);
    assert_eq!(g.ship_materials(ark, UnitKind::Frigate), 21);
    assert_eq!(g.ship_materials(ark, UnitKind::Carrier), 25);
    assert_eq!(g.ship_materials(ark, UnitKind::Battleship), 42);
    assert_eq!(g.ship_materials(Seat(0), UnitKind::ColonyShip), 30);
    assert_eq!(g.ship_materials(Seat(0), UnitKind::Battleship), 50);
}

// ---------------------------------------------------------------- 0.06.0 ticket #84: every Victory Condition waits on a Tech

/// Ticket #84: four new Techs on rung 3 at 40, each after its Faction's themed rung-2 Tech, each
/// the gate for one Faction's Victory Condition; seventeen Techs in all.
#[test]
fn the_four_gates_stand_on_rung_three_at_one_price_with_their_prerequisites() {
    let g = game();
    let gates = [
        (FactionKind::Custodians, TechId::PlanetaryStewardship, vec![TechId::GreenConsensus]),
        // Ticket #242 (version 0.08.3): Beneficiation joined, so the new Tech sits in the
        // branch's spine rather than being a leaf nobody has to take.
        (FactionKind::Prospectors, TechId::ExtractionCharter, vec![TechId::AutomatedRefining, TechId::Beneficiation]),
        (FactionKind::Arkwrights, TechId::GenerationShips, vec![TechId::ClosedLoopColonies]),
        // Ticket #245 (version 0.08.3): Expanded Habitats dropped, and with it the edge that read
        // on screen as an unrelated line into Generation Ships. Ticket #246: and Public Science
        // dropped too, for Closed-Loop Colonies -- the Archive stands at a Colony off Earth, so the
        // Tech that makes such a Colony liveable is what opens its door.
        (FactionKind::Archivists, TechId::TheUpload, vec![TechId::ClosedLoopColonies]),
    ];
    // Ticket #117 (version 0.07.1): rung 3 went 40 to 44, a tenth rounded to the nearest. What the
    // ticket guards is that no Faction's gate is dearer than another's, so the figure is checked
    // against the rung rather than against a literal repeated four times.
    let rung_three = g.tables.tech(TechId::PlanetaryStewardship).cost;
    assert_eq!(rung_three, 48, "rung 3 costs 48 since ticket #231 (version 0.08.3); 45 from #142, 44 from #117, 40 before");
    for (kind, t, needs) in gates {
        let card = g.tables.tech(t);
        assert_eq!(card.rung, 3, "{t:?}");
        assert_eq!(card.cost, rung_three, "every gate costs the same: {t:?}");
        assert_eq!(card.gate_for, Some(kind), "{t:?}");
        assert_eq!(card.needs, needs, "{t:?}");
        assert_eq!(g.tables.victory_gate(kind), Some(t));
    }
    assert_eq!(TechId::ALL.len(), 20, "eighteen, and Beneficiation and Relay Networks since ticket #232");
    // Version 0.08.3 moved three of the four gates' prerequisites in three separate tickets, and
    // nothing watched how deep each gate ended up. Counted as Techs that must stand before the
    // gate is reachable, the gate excluded.
    let depth = |t: TechId| -> usize {
        let mut seen = std::collections::BTreeSet::new();
        let mut stack: Vec<TechId> = g.tables.tech(t).needs.clone();
        while let Some(n) = stack.pop() {
            if seen.insert(n) {
                stack.extend(g.tables.tech(n).needs.iter().copied());
            }
        }
        seen.len()
    };
    // Ticket #246: four, and the deepest tier -- Closed-Loop Colonies waits on Expanded Habitats
    // and Clean Power, and Clean Power on Efficient Grids. This gate also bars the Archive ORDER,
    // so the whole Archive chain sits behind those four.
    assert_eq!(depth(TechId::TheUpload), 4, "the Archivists' gate, four deep since ticket #246");
    assert_eq!(depth(TechId::PlanetaryStewardship), 2, "the Custodians', two since ticket #242 freed Green Consensus from Industry");
    assert_eq!(depth(TechId::GenerationShips), 4, "the Arkwrights', through Closed-Loop Colonies, which itself pulls in Clean Power and Efficient Grids");
    assert_eq!(depth(TechId::ExtractionCharter), 4, "the Prospectors', deepest since Beneficiation joined on ticket #232");
}

/// Ticket #84: with both parts at their bars the Custodians still do not win until Planetary
/// Stewardship stands; the panel says why; progress accrues regardless.
#[test]
fn a_victory_condition_waits_on_its_gate() {
    let mut g = game();
    let cus = Seat(0);
    colony(&mut g, cus, BodyId::Moon, &[ModuleKind::Habitat, ModuleKind::Habitat], 12);
    g.seats[0].stabilization_run = 3;
    let p = g.progress(cus);
    assert!(p.first_value >= p.first_bar && p.second_value >= p.second_bar, "both parts stand: {p:?}");
    assert!(!p.met(), "the gate is not researched");
    assert!(p.first_held_back.as_deref().unwrap_or("").contains("Planetary Stewardship"), "{:?}", p.first_held_back);
    with_tech(&mut g, TechId::PlanetaryStewardship);
    assert!(g.progress(cus).met());
}

/// Ticket #84: each gate is a Tech for everyone. The Charter: Mine x1.25 (4 x 1.65 x 1.25 = 8.25);
/// the Upload: Observatory and Lab x1.25 (2 x 1.25 x 1.25 = 3.125 for a Custodian Observatory);
/// Generation Ships: a Colony Ship carries 2 more (12 for the Arkwrights); Stewardship: the Sink
/// grows by 1.0.
#[test]
fn the_gates_general_bonuses_are_for_everyone() {
    let mut g = game();
    let cus = Seat(0);
    let moon = colony(&mut g, cus, BodyId::Moon, &[ModuleKind::Mine, ModuleKind::Observatory], 0);
    g.state_mut(StateId::EastAsia).facilities.push(facility(FacilityKind::ResearchLab));
    assert_eq!(g.module_yield(cus, moon, ModuleKind::Mine).amount, 6);
    with_tech(&mut g, TechId::ExtractionCharter);
    assert_eq!(g.module_yield(cus, moon, ModuleKind::Mine).amount, 8, "4 x 1.65 x 1.25 = 8.25");
    assert_eq!(g.module_yield(cus, moon, ModuleKind::Observatory).research, 2, "2 x 1.25 = 2.5");
    let lab_before = g.facility_yield(cus, StateId::EastAsia, FacilityKind::ResearchLab).research;
    with_tech(&mut g, TechId::TheUpload);
    assert_eq!(g.module_yield(cus, moon, ModuleKind::Observatory).research, 3, "2 x 1.25 x 1.25 = 3.125");
    assert!(g.facility_yield(cus, StateId::EastAsia, FacilityKind::ResearchLab).research > lab_before, "a Lab makes more too");
    assert_eq!(g.colony_ship_capacity(cus), 4);
    with_tech(&mut g, TechId::GenerationShips);
    assert_eq!(g.colony_ship_capacity(cus), 6);
    assert_eq!(g.colony_ship_capacity(Seat(2)), 12, "(4 + 2) x 2 for the Arkwrights");
    let sink = g.emissions_now().sink;
    with_tech(&mut g, TechId::PlanetaryStewardship);
    assert!((g.emissions_now().sink - sink - 1.0).abs() < 1e-9, "the Sink grows by 1.0: {} then {}", sink, g.emissions_now().sink);
}

/// Ticket #84: the Custodian AI, as Research Lead with the road to its gate open, picks Planetary
/// Stewardship once its first part is past half or from turn 24, and its own list before that.
#[test]
fn the_ai_picks_its_gate_as_lead_once_past_half_or_from_turn_24() {
    let mut g = game();
    let cus = Seat(0);
    for t in [TechId::PublicScience, TechId::CoastalEngineering, TechId::EfficientGrids, TechId::CleanPower, TechId::GreenConsensus] {
        with_tech(&mut g, t);
    }
    g.turn = 5;
    g.seats[0].stabilization_run = 0;
    assert_ne!(g.ai_tech_pick(cus), TechId::PlanetaryStewardship, "too early: its list first");
    g.seats[0].stabilization_run = 2;
    assert_eq!(g.ai_tech_pick(cus), TechId::PlanetaryStewardship, "past half of its first part");
    g.seats[0].stabilization_run = 0;
    g.turn = 24;
    assert_eq!(g.ai_tech_pick(cus), TechId::PlanetaryStewardship, "from turn 24");
}

// ---------------------------------------------------------------- 0.06.0 ticket #85: the Antarctic founding Moment

/// Ticket #85: a Colony founded in Antarctica is announced as "their first Colony in Antarctica",
/// then "their second Colony in Antarctica"; one founded off Earth as "their first Colony off
/// Earth", then "their second Colony off Earth", the count an ordinal in words up to twelfth.
#[test]
fn the_founding_moment_names_antarctica_and_counts_in_ordinals() {
    let mut g = game();
    calm(&mut g);
    g.antarctica_open = true;
    g.take_control(StateId::Europe, Seat(0));
    g.state_mut(StateId::Europe).emigrants = 8;
    let slots = g.free_slots_on(BodyId::Earth);
    for (i, expected) in [(0usize, "their first Colony in Antarctica"), (1, "their second Colony in Antarctica")] {
        let send = Order::SendToAntarctica { state: StateId::Europe, n: 4, into: UnloadTarget::Slot(BodyId::Earth, slots[i]) };
        g.commit_orders(Seat(0), std::slice::from_ref(&send));
        g.resolution_phase();
        g.turn += 1;
        g.report.moments.clear();
        g.resolution_phase();
        let m = g.report.moments.iter().find(|m| m.kind == MomentKind::ColonyFounded).expect("a founding Moment");
        assert!(m.text.contains(expected), "{}: {:?}", expected, m.text);
        assert!(!m.text.contains("off Earth"), "Antarctica is on Earth: {:?}", m.text);
    }
    for expected in ["their first Colony off Earth", "their second Colony off Earth"] {
        let (_, found) = colony_ship_ready(&mut g, BodyId::Moon);
        g.commit_orders(Seat(0), std::slice::from_ref(&found));
        g.report.moments.clear();
        g.resolution_phase();
        let m = g.report.moments.iter().find(|m| m.kind == MomentKind::ColonyFounded).expect("a founding Moment");
        assert!(m.text.contains(expected), "{}: {:?}", expected, m.text);
    }
    assert_eq!(dying_earth_engine::report::ordinal(12), "twelfth");
    assert_eq!(dying_earth_engine::report::ordinal(13), "13th");
    assert_eq!(dying_earth_engine::report::ordinal(22), "22nd");
}

// ---------------------------------------------------------------- 0.06.0 ticket #86: a warming Earth fills the Colony Ships

/// Ticket #86: one Colonist beyond capacity for every full 0.2 C above +1.8, capped at +4, the
/// same for every Faction; the Load order accepts up to the crowded figure and no more.
#[test]
fn a_warming_earth_lets_a_colony_ship_lift_beyond_its_capacity() {
    let mut g = game();
    calm(&mut g);
    for (t, extra) in [(1.2, 0), (1.8, 0), (1.99, 0), (2.0, 1), (2.39, 2), (2.6, 4), (3.2, 4)] {
        g.climate.temperature = t;
        assert_eq!(g.crowd_extra(), extra, "at +{t}");
    }
    g.climate.temperature = 2.6;
    assert_eq!(g.colony_ship_crowded_capacity(Seat(0)), 8, "4 + 4");
    assert_eq!(g.colony_ship_crowded_capacity(Seat(2)), 12, "the Arkwrights' 8 + 4, not + 8");
    g.state_mut(StateId::EastAsia).emigrants = 12;
    let (ship, _) = colony_ship_ready(&mut g, BodyId::Earth);
    g.ship_mut(ship).unwrap().colonists = 0;
    let load = |n| Order::Load { ship, colonists: n, from: LoadSource::State(StateId::EastAsia), army: None };
    assert!(g.check_order(Seat(0), &[], &load(8)).is_ok(), "the crowded load");
    assert!(g.check_order(Seat(0), &[], &load(9)).is_err(), "and no more");
    g.climate.temperature = 1.2;
    assert!(g.check_order(Seat(0), &[], &load(5)).is_err(), "no crowd in a cool world");
}

/// Ticket #86: at arrival each crowded Colonist dies with a chance of 5% times the extras, once,
/// from the game's own generator; a Report line and a Moment say so; nothing else counts them.
/// Forty seeds of a ship with four extras (each at 20%) lose about 0.8 a flight: between 10 and
/// 60 in all, and never more than the four extras; with no extras, nobody dies.
#[test]
fn crowded_colonists_may_die_on_arrival_once_and_only_the_extras() {
    let (mut deaths, mut moments) = (0u32, 0u32);
    for seed in 1..=40u64 {
        let mut g = with_seed(seed);
        calm(&mut g);
        g.climate.temperature = 2.6;
        let (ship, _) = colony_ship_ready(&mut g, BodyId::Earth);
        g.ship_mut(ship).unwrap().colonists = 8;
        g.ship_mut(ship).unwrap().at = ShipAt::Transit { from: BodyId::Earth, to: BodyId::Moon, turns_left: 1 };
        g.report.moments.clear();
        g.resolution_phase();
        let aboard = g.ship(ship).unwrap().colonists;
        assert!((4..=8).contains(&aboard), "only the extras are at risk: {aboard}");
        let lost = 8 - aboard;
        deaths += lost;
        assert_eq!(g.seat(Seat(0)).lost_in_transit, lost as i64);
        let m = g.report.moments.iter().filter(|m| m.kind == MomentKind::LostInTransit).count() as u32;
        assert_eq!(m, u32::from(lost > 0), "a Moment when and only when someone died");
        moments += m;
        assert!(g.state(StateId::EastAsia).unrest < 0.5, "deaths move no Unrest");
    }
    assert!((10..=60).contains(&deaths), "about 0.8 a flight over forty: {deaths}");
    assert!(moments > 0);
    let mut g = with_seed(3);
    calm(&mut g);
    g.climate.temperature = 2.6;
    let (ship, _) = colony_ship_ready(&mut g, BodyId::Earth);
    g.ship_mut(ship).unwrap().colonists = 4;
    g.ship_mut(ship).unwrap().at = ShipAt::Transit { from: BodyId::Earth, to: BodyId::Moon, turns_left: 1 };
    g.resolution_phase();
    assert_eq!(g.ship(ship).unwrap().colonists, 4, "a safe load loses nobody");
}

/// Ticket #86: Emigrants sent to Antarctica by sea carry no crowd and lose nobody, hot or not.
#[test]
fn the_sea_crossing_to_antarctica_carries_no_crowd_and_loses_nobody() {
    let mut g = game();
    calm(&mut g);
    g.antarctica_open = true;
    g.climate.temperature = 3.0;
    g.take_control(StateId::Europe, Seat(0));
    g.state_mut(StateId::Europe).emigrants = 8;
    let slot = g.free_slots_on(BodyId::Earth)[0];
    let send = Order::SendToAntarctica { state: StateId::Europe, n: 8, into: UnloadTarget::Slot(BodyId::Earth, slot) };
    g.commit_orders(Seat(0), std::slice::from_ref(&send));
    g.resolution_phase();
    g.turn += 1;
    g.resolution_phase();
    let col = g.colonies.iter().find(|c| c.body == BodyId::Earth && !c.in_orbit && c.control.director() == Some(Seat(0))).expect("founded");
    // Ticket #125 (version 0.07.2): a fresh Colony takes min(n, room), and room is the slot's own
    // Habitat yield, drawn from the seed at the start -- fourteen Regions shift that stream, so
    // the first free slot's room is 7 on this seed where it was 8. The claim under test is that
    // NOBODY IS LOST, not that eight land: whoever finds no room comes home, and the sum is eight.
    assert!(col.colonists >= 1);
    assert_eq!(col.colonists + g.state(StateId::Europe).emigrants, 8, "everyone who sailed either landed or came home");
    assert_eq!(g.seat(Seat(0)).lost_in_transit, 0);
}

/// Ticket #86: the AI lifts the crowded load when behind on Off-world Presence, the safe one
/// otherwise.
#[test]
fn the_ai_lifts_a_crowded_load_only_when_behind_on_presence() {
    let mut g = game();
    calm(&mut g);
    g.climate.temperature = 2.6;
    g.state_mut(StateId::EastAsia).emigrants = 12;
    let (ship, _) = colony_ship_ready(&mut g, BodyId::Earth);
    g.ship_mut(ship).unwrap().colonists = 0;
    let orders = g.ai_orders(Seat(0));
    let lifted = orders.iter().find_map(|o| match o {
        Order::Load { ship: s, colonists, .. } if *s == ship => Some(*colonists),
        _ => None,
    });
    assert_eq!(lifted, Some(8), "behind on Presence with 0 off Earth: the crowded load: {orders:?}");
    colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Habitat, ModuleKind::Habitat], 12);
    let orders = g.ai_orders(Seat(0));
    let lifted = orders.iter().find_map(|o| match o {
        Order::Load { ship: s, colonists, .. } if *s == ship => Some(*colonists),
        _ => None,
    });
    assert_eq!(lifted, Some(4), "Presence met: the safe load: {orders:?}");
}

// ---------------------------------------------------------------- 0.06.0 ticket #87: every Ship carries its own tank

/// Ticket #87: every Ship type has a tank of 30 on its row (an Army none); a Ship is built with it
/// full, the Fuel paid from the Stockpile at the yard, and a build the Stockpile cannot fuel is
/// refused.
#[test]
fn a_ship_is_built_with_a_full_tank_paid_from_the_stockpile() {
    let mut g = game();
    for k in UnitKind::SHIPS {
        assert_eq!(g.tables.unit(k).tank, 30, "{k:?}");
    }
    assert_eq!(g.tables.unit(UnitKind::Army).tank, 0);
    let iss = station_of(&g, Seat(0), BodyId::Earth).unwrap();
    g.colony_mut(iss).unwrap().modules.push(Module::new(ModuleKind::Shipyard));
    g.seats[0].stockpile.materials = 100;
    g.seats[0].stockpile.energy = 200;
    g.seats[0].stockpile.fuel = 20;
    let build = Order::BuildShip { site: Place::Colony(iss), kind: UnitKind::Frigate };
    assert_eq!(g.order_cost(Seat(0), &build).fuel, 30, "the tank is filled at the yard");
    assert!(g.check_order(Seat(0), &[], &build).is_err(), "20 Fuel cannot fill a tank of 30");
    g.seats[0].stockpile.fuel = 50;
    g.commit_orders(Seat(0), std::slice::from_ref(&build));
    assert_eq!(g.seats[0].stockpile.fuel, 20, "30 paid at the build");
    for _ in 0..3 {
        g.resolution_phase();
        if g.ships.iter().any(|s| s.kind == UnitKind::Frigate && s.seat == Seat(0)) {
            break;
        }
        g.turn += 1;
    }
    let ship = g.ships.iter().find(|s| s.kind == UnitKind::Frigate && s.seat == Seat(0)).expect("built");
    assert_eq!(ship.fuel, 30);
}

/// Ticket #87: a transit spends the tank, not the Stockpile, and is refused when the tank cannot
/// pay the leg.
#[test]
fn a_transit_spends_the_tank_and_is_refused_when_the_tank_cannot_pay() {
    let mut g = game();
    at_window(&mut g);
    let (ship, _) = colony_ship_ready(&mut g, BodyId::Earth);
    g.ship_mut(ship).unwrap().fuel = 30;
    g.seats[0].stockpile.fuel = 0;
    let to_mars = Order::Transit { ship, to: BodyId::Mars, slot: None };
    assert_eq!(g.order_cost(Seat(0), &to_mars).fuel, 0, "the Stockpile pays nothing");
    assert!(g.check_order(Seat(0), &[], &to_mars).is_ok(), "20 of the 30 in the tank");
    g.commit_orders(Seat(0), std::slice::from_ref(&to_mars));
    assert_eq!(g.ship(ship).unwrap().fuel, 10, "30 - 20");
    assert_eq!(g.seats[0].stockpile.fuel, 0);
    let (poor, _) = colony_ship_ready(&mut g, BodyId::Earth);
    g.ship_mut(poor).unwrap().fuel = 5;
    let err = g.check_order(Seat(0), &[], &Order::Transit { ship: poor, to: BodyId::Mars, slot: None }).unwrap_err().0;
    assert!(err.contains("tank"), "{err}");
    assert!(g.check_order(Seat(0), &[], &Order::Transit { ship: poor, to: BodyId::Moon, slot: None }).is_err(), "6 needed, 5 in the tank");
}

/// Ticket #87: Refuel is an order at a Body where the Ship's Faction holds a station, filling from
/// the Stockpile as far as it can pay; nowhere else. A Ship whose tank cannot pay any leg, with no
/// station of its own there, is stranded, and a station built in orbit rescues it.
#[test]
fn refuel_is_an_order_at_a_station_of_your_own_and_a_station_rescues_a_stranded_ship() {
    let mut g = game();
    let (ship, _) = colony_ship_ready(&mut g, BodyId::Earth);
    g.ship_mut(ship).unwrap().fuel = 4;
    g.seats[0].stockpile.fuel = 10;
    let refuel = Order::Refuel { ship };
    assert!(g.check_order(Seat(0), &[], &refuel).is_ok(), "the ISS stands over Earth");
    assert_eq!(g.order_cost(Seat(0), &refuel).fuel, 10, "26 wanted, 10 held: what the Stockpile can pay");
    g.commit_orders(Seat(0), std::slice::from_ref(&refuel));
    assert_eq!(g.ship(ship).unwrap().fuel, 14);
    assert_eq!(g.seats[0].stockpile.fuel, 0);
    assert!(g.check_order(Seat(0), &[], &refuel).is_err(), "nothing to pay with");
    g.seats[0].stockpile.fuel = 50;
    let (far, _) = colony_ship_ready(&mut g, BodyId::Mars);
    g.ship_mut(far).unwrap().fuel = 1;
    assert!(g.check_order(Seat(0), &[], &Order::Refuel { ship: far }).unwrap_err().0.contains("station"));
    assert!(g.stranded(far), "1 in the tank, the cheapest leg (Phobos, 2) beyond it, no station of ours");
    assert!(!g.stranded(ship), "14 in the tank at Earth flies to the Moon");
    let id = ColonyId(g.fresh_id());
    g.colonies.push(Colony { id, body: BodyId::Mars, slot: 0, control: Control::Controlled(Seat(0)), modules: Vec::new(), colonists: 0, education: 1.0, settler_education: 1.0, queue: Vec::new(), grid_failed: false, founded_turn: 1, in_orbit: true });
    assert!(!g.stranded(far), "a station of ours in orbit rescues it");
    assert!(g.check_order(Seat(0), &[], &Order::Refuel { ship: far }).is_ok());
    let (full, _) = colony_ship_ready(&mut g, BodyId::Earth);
    g.ship_mut(full).unwrap().fuel = 30;
    assert!(g.check_order(Seat(0), &[], &Order::Refuel { ship: full }).unwrap_err().0.contains("full"));
}

/// Ticket #87: the AI refuels at its own station and never orders a leg its tank cannot pay.
#[test]
fn the_ai_refuels_at_its_station_and_orders_no_leg_its_tank_cannot_pay() {
    let mut g = game();
    calm(&mut g);
    at_window(&mut g);
    let (ship, _) = colony_ship_ready(&mut g, BodyId::Earth);
    g.ship_mut(ship).unwrap().fuel = 3;
    g.seats[0].stockpile.fuel = 100;
    g.seats[0].stockpile.energy = 200;
    let orders = g.ai_orders(Seat(0));
    assert!(orders.iter().any(|o| matches!(o, Order::Refuel { ship: s } if *s == ship)), "no Refuel at the ISS: {orders:?}");
    assert!(!orders.iter().any(|o| matches!(o, Order::Transit { ship: s, .. } if *s == ship)), "3 in the tank flies nowhere: {orders:?}");
}

// ---------------------------------------------------------------- 0.06.0 ticket #88: build it where you dig

/// Ticket #88: a Module at a Colony with one working Mine costs x0.75, with two or more x0.6,
/// multiplicative with the Faction's discount, rounded down, floored at half the row; the
/// Archive too; a mothballed or unfinished Mine counts for nothing; Antarctica counts; Ships and
/// stations are untouched.
#[test]
fn modules_cost_less_at_a_colony_with_working_mines() {
    let mut g = game();
    let cus = Seat(0);
    let moon = colony(&mut g, cus, BodyId::Moon, &[], 0);
    let habitat = |c| Order::BuildModule { colony: c, kind: ModuleKind::Habitat };
    assert_eq!(g.order_cost(cus, &habitat(moon)).materials, 25, "no Mine, the row");
    g.colony_mut(moon).unwrap().modules.push(Module::new(ModuleKind::Mine));
    assert_eq!(g.order_cost(cus, &habitat(moon)).materials, 18, "one working Mine: 25 x 0.75 = 18.75");
    g.colony_mut(moon).unwrap().modules.push(Module::new(ModuleKind::Mine));
    assert_eq!(g.order_cost(cus, &habitat(moon)).materials, 15, "two: 25 x 0.6");
    assert_eq!(g.order_cost(cus, &Order::BuildModuleWithDucats { colony: moon, kind: ModuleKind::Habitat }).ducats, 30, "the Ducat price follows: 15 x 2");
    g.colony_mut(moon).unwrap().modules[1].mothballed = true;
    assert_eq!(g.order_cost(cus, &habitat(moon)).materials, 18, "a mothballed Mine counts for nothing");
    g.colony_mut(moon).unwrap().modules[1].mothballed = false;
    // The Prospectors' 0.85 stacks; the Arkwrights' 0.75 x 0.6 would be 11.25, floored at half, 12.
    let pro = Seat(1);
    let theirs = colony(&mut g, pro, BodyId::Moon, &[ModuleKind::Mine, ModuleKind::Mine], 0);
    assert_eq!(g.order_cost(pro, &habitat(theirs)).materials, 12, "25 x 0.85 x 0.6 = 12.75");
    let ark = Seat(2);
    let deep = colony(&mut g, ark, BodyId::Mars, &[ModuleKind::Mine, ModuleKind::Mine], 0);
    assert_eq!(g.order_cost(ark, &habitat(deep)).materials, 12, "25 x 0.75 x 0.6 = 11.25, floored at 12.5");
    // Antarctica counts; a Ship and a station do not take it.
    let vostok = colony(&mut g, cus, BodyId::Earth, &[ModuleKind::Mine, ModuleKind::Shipyard], 0);
    assert_eq!(g.order_cost(cus, &habitat(vostok)).materials, 18);
    assert_eq!(g.order_cost(cus, &Order::BuildShip { site: Place::Colony(vostok), kind: UnitKind::Frigate }).materials, 25, "Ships untouched");
    assert_eq!(g.order_cost(cus, &Order::BuildStation { body: BodyId::Moon, slot: 0 }).materials, 40, "stations untouched");
    // The Archive at a Colony with two Mines: 50 x 0.6 = 30.
    let arc = Seat(3);
    let site = colony(&mut g, arc, BodyId::Mars, &[ModuleKind::Mine, ModuleKind::Mine, ModuleKind::Habitat], 4);
    assert_eq!(g.order_cost(arc, &Order::BuildArchive { colony: site }).materials, 30);
}

// ---------------------------------------------------------------- 0.06.0 ticket #89: the Solar Array

/// A station of seat 0's over `body`, bare, for the tests that need one.
fn station_at(g: &mut Game, seat: Seat, body: BodyId) -> ColonyId {
    let id = ColonyId(g.fresh_id());
    g.colonies.push(Colony { id, body, slot: 0, control: Control::Controlled(seat), modules: Vec::new(), colonists: 0, education: 1.0, settler_education: 1.0, queue: Vec::new(), grid_failed: false, founded_turn: 1, in_orbit: true });
    id
}

/// Ticket #89: a Solar Array makes 6 Energy at Earth's distance, scaled by the inverse square of
/// its Body's mean distance from the Sun (a satellite reads its parent's), rounded to the nearest
/// whole: the Moon 6, Mars 6 x 0.43 = 2.6, so 3; Efficient Grids lifts it (9 and 3.9, so 4); a
/// Solar Storm turn silences it.
#[test]
fn a_solar_array_scales_with_the_inverse_square_of_the_distance_from_the_sun() {
    let mut g = game();
    let cus = Seat(0);
    assert!((g.sun_factor(BodyId::Earth) - 1.0).abs() < 1e-3);
    assert!((g.sun_factor(BodyId::Moon) - 1.0).abs() < 1e-3);
    assert!((g.sun_factor(BodyId::Mars) - 0.4307).abs() < 1e-3, "{}", g.sun_factor(BodyId::Mars));
    assert!((g.sun_factor(BodyId::Phobos) - 0.4307).abs() < 1e-3);
    let iss = station_of(&g, cus, BodyId::Earth).unwrap();
    let moon = station_at(&mut g, cus, BodyId::Moon);
    let mars = station_at(&mut g, cus, BodyId::Mars);
    for c in [iss, moon, mars] {
        g.colony_mut(c).unwrap().modules.push(Module::new(ModuleKind::SolarArray));
    }
    let y = g.module_yield(cus, iss, ModuleKind::SolarArray);
    assert_eq!((y.resource, y.amount, y.upkeep), (Some(Resource::Energy), 6, 0));
    assert_eq!(g.module_yield(cus, moon, ModuleKind::SolarArray).amount, 6, "the Moon reads Earth's distance");
    assert_eq!(g.module_yield(cus, mars, ModuleKind::SolarArray).amount, 3, "6 x 0.43 = 2.6, nearest 3");
    with_tech(&mut g, TechId::EfficientGrids);
    assert_eq!(g.module_yield(cus, iss, ModuleKind::SolarArray).amount, 9);
    assert_eq!(g.module_yield(cus, mars, ModuleKind::SolarArray).amount, 4, "2.6 x 1.5 = 3.9, nearest 4");
    g.last_event = Some(DrawnEvent { card: Card::Event(EventId::SolarStorm), target: EventTarget::Everyone, scale: 1.0, text: String::new() });
    assert_eq!(g.module_yield(cus, iss, ModuleKind::SolarArray).amount, 0, "a Solar Storm turn silences it");
}

/// Ticket #89: a Solar Array stands only on a Space Station; a ground Colony refuses it.
#[test]
fn a_solar_array_stands_only_on_a_station() {
    let mut g = game();
    g.seats[0].stockpile.materials = 200;
    let iss = station_of(&g, Seat(0), BodyId::Earth).unwrap();
    // Ticket #164 (version 0.07.5): a place nobody lives on has no Module slots, so give both
    // somebody to live there; this test is about which kinds stand where, not about the cap.
    g.colony_mut(iss).unwrap().colonists = 2;
    let ground = colony(&mut g, Seat(0), BodyId::Moon, &[], 2);
    assert!(g.check_order(Seat(0), &[], &Order::BuildModule { colony: iss, kind: ModuleKind::SolarArray }).is_ok());
    let err = g.check_order(Seat(0), &[], &Order::BuildModule { colony: ground, kind: ModuleKind::SolarArray }).unwrap_err().0;
    assert!(err.contains("station"), "{err}");
    assert_eq!(g.tables.module(ModuleKind::SolarArray).materials, 25);
    assert_eq!(g.tables.module(ModuleKind::SolarArray).build_turns, 2);
}

/// Ticket #89: the AI raises a Solar Array on a station of its own when Energy is within a turn's
/// upkeep of nothing.
#[test]
fn the_ai_raises_a_solar_array_on_its_station_when_energy_is_tight() {
    let mut g = game();
    calm(&mut g);
    g.seats[0].stockpile.materials = 200;
    g.seats[0].stockpile.energy = 0;
    let iss = station_of(&g, Seat(0), BodyId::Earth).unwrap();
    // Ticket #164 (version 0.07.5): a station nobody lives on has no Module slots, so nothing can
    // be raised on it at all. Its Core Module holds four, so put people there first.
    g.colony_mut(iss).unwrap().colonists = 2;
    let orders = g.ai_orders(Seat(0));
    assert!(orders.iter().any(|o| matches!(o, Order::BuildModule { colony, kind: ModuleKind::SolarArray } if *colony == iss)), "no Solar Array on the ISS: {orders:?}");
}

// ---------------------------------------------------------------- 0.06.0 ticket #90: the Trade Post pays for the shape of the empire

/// Ticket #90: a Trade Post pays 2 Ducats per Colonist of its Faction at its Body (stations
/// overhead included) plus 3 for every other Body where the Faction holds a Colony, a station or,
/// for Earth, a Nation State; the Habitat yield is gone; the Prospectors' x1.25 applies.
#[test]
fn a_trade_post_pays_for_colonists_at_its_body_and_every_other_body_held() {
    let mut g = game();
    let cus = Seat(0);
    let mars = colony(&mut g, cus, BodyId::Mars, &[ModuleKind::TradePost, ModuleKind::Habitat, ModuleKind::Habitat], 12);
    // Earth counts: the Custodians direct East Asia and hold the ISS. Mars is its own Body.
    let y = g.module_yield(cus, mars, ModuleKind::TradePost);
    assert_eq!((y.resource, y.amount), (Some(Resource::Ducats), 2 * 12 + 3), "12 Colonists at Mars, one other Body (Earth)");
    assert!(y.detail.as_deref().unwrap_or("").contains("2 x 12"), "{:?}", y.detail);
    colony(&mut g, cus, BodyId::Moon, &[], 0);
    assert_eq!(g.module_yield(cus, mars, ModuleKind::TradePost).amount, 24 + 6, "the Moon is a second other Body");
    let over_mars = station_at(&mut g, cus, BodyId::Mars);
    g.colony_mut(over_mars).unwrap().colonists = 3;
    assert_eq!(g.module_yield(cus, mars, ModuleKind::TradePost).amount, 30 + 6, "three more Colonists at the Body, on the station");
    let pro = Seat(1);
    let theirs = colony(&mut g, pro, BodyId::Mars, &[ModuleKind::TradePost, ModuleKind::Habitat], 8);
    assert_eq!(g.module_yield(pro, theirs, ModuleKind::TradePost).amount, ((16 + 3) as f64 * 1.25).floor() as i64, "the Prospectors' x1.25");
}

/// Ticket #90: one Trade Post per Faction per Body, on a station or on the ground.
#[test]
fn one_trade_post_per_faction_per_body_on_the_ground_or_in_orbit() {
    let mut g = game();
    g.seats[0].stockpile.materials = 300;
    let cus = Seat(0);
    // Ticket #164 (version 0.07.5): people, or there are no slots to argue about.
    let a = colony(&mut g, cus, BodyId::Mars, &[ModuleKind::TradePost], 2);
    let b = colony(&mut g, cus, BodyId::Mars, &[], 2);
    let post = |c| Order::BuildModule { colony: c, kind: ModuleKind::TradePost };
    assert!(g.check_order(cus, &[], &post(a)).unwrap_err().0.contains("Trade Post"), "a second at the same Colony");
    assert!(g.check_order(cus, &[], &post(b)).unwrap_err().0.contains("Trade Post"), "a second on the same Body");
    let moon = colony(&mut g, cus, BodyId::Moon, &[], 2);
    assert!(g.check_order(cus, &[], &post(moon)).is_ok(), "another Body");
    let iss = station_of(&g, cus, BodyId::Earth).unwrap();
    g.colony_mut(iss).unwrap().colonists = 2;
    assert!(g.check_order(cus, &[], &post(iss)).is_ok(), "a station may hold one");
    let pro = Seat(1);
    g.seats[1].stockpile.materials = 300;
    let theirs = colony(&mut g, pro, BodyId::Mars, &[], 2);
    assert!(g.check_order(pro, &[], &post(theirs)).is_ok(), "the Prospectors' first on Mars");
}

/// Ticket #90: the AI offers a Trade Post at a Body it holds and has none on, and values it more
/// once it holds a second Body.
#[test]
fn the_ai_offers_a_trade_post_at_each_body_it_holds() {
    let mut g = game();
    calm(&mut g);
    g.seats[0].stockpile.materials = 300;
    g.seats[0].stockpile.energy = 300;
    let mars = colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Mine, ModuleKind::Generator, ModuleKind::Habitat], 8);
    g.ai_orders(Seat(0));
    let name = g.place_name(Place::Colony(mars));
    assert!(g.log.iter().any(|l| l.contains(&format!("build Trade Post at {name}"))), "no Trade Post offered at Mars");
    g.colony_mut(mars).unwrap().modules.push(Module::new(ModuleKind::TradePost));
    g.log.clear();
    g.ai_orders(Seat(0));
    assert!(!g.log.iter().any(|l| l.contains(&format!("build Trade Post at {name}"))), "one per Body: not offered again");
}

// ---------------------------------------------------------------- 0.06.0 ticket #92: the Mass Driver

/// Ticket #92: the Moon, Phobos and Deimos are low-gravity; the Mass Driver is 35 Materials, two
/// turns, 4 Energy, behind Efficient Transit, on a ground Colony of a low-gravity Body, one per
/// Colony.
#[test]
fn the_mass_driver_stands_on_a_low_gravity_colony_behind_efficient_transit_one_per_colony() {
    let mut g = game();
    for b in BodyId::ALL {
        assert_eq!(g.tables.body(b).low_gravity, matches!(b, BodyId::Moon | BodyId::Phobos | BodyId::Deimos), "{b:?}");
    }
    let card = g.tables.module(ModuleKind::MassDriver);
    assert_eq!((card.materials, card.build_turns, card.energy_upkeep), (35, 2, 4));
    assert_eq!(card.needs_tech, Some(TechId::EfficientTransit));
    g.seats[0].stockpile.materials = 300;
    // Ticket #164 (version 0.07.5): people, or there are no slots to build a Mass Driver into.
    let moon = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Mine], 3);
    let mars = colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Mine], 3);
    let build = |c| Order::BuildModule { colony: c, kind: ModuleKind::MassDriver };
    assert!(g.check_order(Seat(0), &[], &build(moon)).unwrap_err().0.contains("Efficient Transit"), "the Tech first");
    with_tech(&mut g, TechId::EfficientTransit);
    assert!(g.check_order(Seat(0), &[], &build(moon)).is_ok());
    assert!(g.check_order(Seat(0), &[], &build(mars)).unwrap_err().0.contains("low"), "Mars is not a small world");
    let iss = station_of(&g, Seat(0), BodyId::Earth).unwrap();
    assert!(g.check_order(Seat(0), &[], &build(iss)).is_err(), "not on a station");
    g.colony_mut(moon).unwrap().modules.push(Module::new(ModuleKind::MassDriver));
    assert!(g.check_order(Seat(0), &[], &build(moon)).unwrap_err().0.contains("Mass Driver"), "one per Colony");
}

/// Ticket #92: a working Mass Driver takes a flat 4 Fuel off every leg the owner's Ships fly from
/// its Body, after the multipliers, to a minimum of 1; a rival pays the full leg; a mothballed one
/// does nothing; and each Mine at its Colony makes +1 Materials.
#[test]
fn a_mass_driver_cuts_the_owners_departures_by_four_and_gives_its_mines_one_more() {
    let mut g = game();
    at_window(&mut g);
    let cus = Seat(0);
    let moon = colony(&mut g, cus, BodyId::Moon, &[ModuleKind::Mine, ModuleKind::MassDriver], 0);
    assert_eq!(g.transit_cost_for(cus, BodyId::Moon, BodyId::Earth).1, 2, "6 - 4");
    assert_eq!(g.transit_cost_for(cus, BodyId::Moon, BodyId::Mars).1, 16, "20 - 4");
    assert_eq!(g.transit_cost_for(cus, BodyId::Earth, BodyId::Moon).1, 6, "arriving is not departing");
    assert_eq!(g.transit_cost_for(Seat(1), BodyId::Moon, BodyId::Earth).1, 6, "a rival pays the leg");
    let ark = Seat(2);
    colony(&mut g, ark, BodyId::Moon, &[ModuleKind::MassDriver], 0);
    assert_eq!(g.transit_cost_for(ark, BodyId::Moon, BodyId::Earth).1, 1, "6 x 0.75 = 4, - 4, minimum 1");
    assert_eq!(g.module_yield(cus, moon, ModuleKind::Mine).amount, 7, "4 x 1.65 = 6, +1");
    g.colony_mut(moon).unwrap().modules[1].mothballed = true;
    assert_eq!(g.transit_cost_for(cus, BodyId::Moon, BodyId::Earth).1, 6, "mothballed, it throws nothing");
    assert_eq!(g.module_yield(cus, moon, ModuleKind::Mine).amount, 6);
}

/// Ticket #92: the AI offers a Mass Driver at a low-gravity Colony with a Mine once the Tech
/// stands, and weighs a Mine higher where one stands.
#[test]
fn the_ai_offers_a_mass_driver_and_weighs_a_mine_higher_beside_one() {
    let mut g = game();
    calm(&mut g);
    with_tech(&mut g, TechId::EfficientTransit);
    g.seats[0].stockpile.materials = 300;
    g.seats[0].stockpile.energy = 300;
    let moon = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Mine, ModuleKind::Generator], 4);
    let plain = colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Mine, ModuleKind::Generator], 4);
    g.ai_orders(Seat(0));
    let moon_name = g.place_name(Place::Colony(moon));
    assert!(g.log.iter().any(|l| l.contains(&format!("build Mass Driver at {moon_name}"))), "no Mass Driver offered on the Moon");
    let score = |g: &Game, c: ColonyId| -> f64 {
        let name = g.place_name(Place::Colony(c));
        let needle = format!("build Mine at {name}");
        g.log.iter().find(|l| l.contains(&needle)).and_then(|l| l.split_whitespace().nth(1).and_then(|s| s.parse().ok())).unwrap_or(0.0)
    };
    let before = score(&g, moon);
    g.colony_mut(moon).unwrap().modules.push(Module::new(ModuleKind::MassDriver));
    g.log.clear();
    g.ai_orders(Seat(0));
    let after = score(&g, moon);
    let mars = score(&g, plain);
    assert!(after > before && after > mars, "a Mine beside a Mass Driver weighs more: {before} then {after}, Mars {mars}");
}

// ---------------------------------------------------------------- 0.06.0 ticket #93: Venus

/// Ticket #93: Venus is the sixth Body, with no Colony Slots and three named Orbital Slots, three
/// turns and 16 Fuel from Earth on its own window, its row on the sky; its windows fall at turns
/// 9, 18 and 28; no leg runs between it and the Mars system.
#[test]
fn venus_is_a_body_of_orbits_only_with_its_own_window() {
    let g = game();
    assert_eq!(BodyId::ALL.len(), 6);
    let card = g.tables.body(BodyId::Venus);
    assert_eq!(card.colony_slots(), 0);
    assert_eq!(card.orbital_slots, 3);
    assert_eq!(card.stations, vec!["Ishtar", "Aphrodite", "Lada"]);
    assert!(!card.low_gravity);
    assert!((g.tables.planet(BodyId::Venus).a - 0.7233).abs() < 1e-3);
    assert!((g.sun_factor(BodyId::Venus) - 1.911).abs() < 1e-2);
    let w1 = g.next_venus_window_turn(1);
    let w2 = g.next_venus_window_turn(w1 + 1);
    let w3 = g.next_venus_window_turn(w2 + 1);
    assert_eq!((w1, w2, w3), (9, 18, 28), "the research note's windows");
    assert_eq!(g.transit_cost_at(BodyId::Earth, BodyId::Venus, w1), (3, 16), "three turns and the card's Fuel on the window");
    let (turns_off, fuel_off) = g.transit_cost_at(BodyId::Earth, BodyId::Venus, w1 + 4);
    assert!(turns_off > 3 && fuel_off > 16, "off the window both rise: {turns_off} turns, {fuel_off} Fuel");
    assert!(g.crossing_offset(BodyId::Earth, BodyId::Venus, w1).is_some());
    assert!(g.crossing_offset(BodyId::Venus, BodyId::Earth, w1).is_some());
    assert!(g.crossing_offset(BodyId::Earth, BodyId::Mars, w1).is_some(), "Mars's window is untouched");
    assert!(Game::leg_allowed(BodyId::Earth, BodyId::Venus));
    assert!(!Game::leg_allowed(BodyId::Mars, BodyId::Venus));
    assert!(!Game::leg_allowed(BodyId::Venus, BodyId::Phobos));
}

/// Ticket #93: a station at Venus is built from a Ship of the seat's in orbit there; a Colony Ship
/// lands only into it; its Colonists count for Off-world Presence and it is Venus for Diaspora;
/// no leg to Mars is accepted.
#[test]
fn a_venus_station_is_built_from_a_ship_in_orbit_and_its_colonists_are_off_earth() {
    let mut g = game();
    g.seats[0].stockpile.materials = 300;
    let build = Order::BuildStation { body: BodyId::Venus, slot: 0 };
    assert!(g.check_order(Seat(0), &[], &build).unwrap_err().0.contains("Ship"), "nothing of ours at Venus");
    let (ship, found) = colony_ship_ready(&mut g, BodyId::Venus);
    assert!(g.check_order(Seat(0), &[], &found).is_err(), "no ground to found on");
    assert!(g.check_order(Seat(0), &[], &build).is_ok(), "a Ship in orbit is the foothold");
    g.commit_orders(Seat(0), std::slice::from_ref(&build));
    g.resolution_phase();
    let station = g.colonies.iter().find(|c| c.body == BodyId::Venus && c.in_orbit).expect("Ishtar stands").id;
    g.colony_mut(station).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    let land = Order::Unload { ship, colonists: 4, army: false, into: UnloadTarget::Colony(station) };
    assert!(g.check_order(Seat(0), &[], &land).is_ok());
    g.commit_orders(Seat(0), std::slice::from_ref(&land));
    g.resolution_phase();
    assert_eq!(g.colony(station).unwrap().colonists, 4);
    assert_eq!(g.off_world_colonists(Seat(0)), 4, "Venus is off Earth");
    assert_eq!(g.bodies_settled(Seat(0), 4), 1, "and a Body for Diaspora");
    assert!(g.check_order(Seat(0), &[], &Order::Transit { ship, to: BodyId::Mars, slot: None }).unwrap_err().0.contains("Venus"));
}

/// Ticket #93: the AI raises a station at Venus when a Ship of its own stands there.
#[test]
fn the_ai_raises_a_station_at_venus_when_it_has_a_ship_there() {
    let mut g = game();
    calm(&mut g);
    g.seats[0].stockpile.materials = 300;
    g.seats[0].stockpile.energy = 300;
    let _ = colony_ship_ready(&mut g, BodyId::Venus);
    let orders = g.ai_orders(Seat(0));
    assert!(orders.iter().any(|o| matches!(o, Order::BuildStation { body: BodyId::Venus, .. })), "no station at Venus: {orders:?}");
}

// ---------------------------------------------------------------- 0.06.0 ticket #94: the AI sweep

/// The AI sweep: a loaded Colony Ship disembarks into a Colony or station of its own with room at
/// the Body it stands at: a station at Venus with a Habitat, and the ISS over Earth (off Earth
/// since ticket #81). Until the sweep this branch could never run. Over Earth the landing is a
/// foothold at half weight, so a flight to the Moon outscores it while one is on offer.
#[test]
fn the_ai_disembarks_into_its_own_station_with_room_at_venus_and_over_earth() {
    let mut g = game();
    calm(&mut g);
    g.seats[0].stockpile.materials = 300;
    g.seats[0].stockpile.energy = 300;
    let venus = station_at(&mut g, Seat(0), BodyId::Venus);
    g.colony_mut(venus).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    let (ship, _) = colony_ship_ready(&mut g, BodyId::Venus);
    let orders = g.ai_orders(Seat(0));
    assert!(orders.iter().any(|o| matches!(o, Order::Unload { ship: s, into: UnloadTarget::Colony(c), .. } if *s == ship && *c == venus)), "no landing into the Venus station: {orders:?}");
    let mut g = game();
    calm(&mut g);
    g.seats[0].stockpile.materials = 300;
    g.seats[0].stockpile.energy = 300;
    let iss = station_of(&g, Seat(0), BodyId::Earth).unwrap();
    g.colony_mut(iss).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    let (ship, _) = colony_ship_ready(&mut g, BodyId::Earth);
    g.ai_orders(Seat(0));
    let score = |needle: &str| -> f64 {
        let l = g.log.iter().find(|l| l.contains(needle)).unwrap_or_else(|| panic!("no scored line {needle:?}: {:#?}", g.log.iter().filter(|l| l.starts_with("  ")).collect::<Vec<_>>()));
        l.split_whitespace().nth(1).and_then(|n| n.parse().ok()).unwrap_or_else(|| panic!("no score on {l:?}"))
    };
    let park = score("disembark 4 Colonists into ISS");
    let fly = score("to the Moon");
    assert!(park > 0.0 && park < fly, "parking on the ISS ({park}) should be a foothold below the flight to the Moon ({fly})");
    let _ = ship;
}

/// The AI sweep: Venus weighs as a slot with the Body's own yields when the seat holds a station
/// there with room, so a loaded Colony Ship at Earth is offered the flight to Venus.
#[test]
fn the_ai_offers_a_loaded_colony_ship_the_flight_to_a_venus_station_with_room() {
    let mut g = game();
    calm(&mut g);
    g.turn = g.next_venus_window_turn(1);
    g.seats[0].stockpile.materials = 300;
    g.seats[0].stockpile.energy = 300;
    g.seats[0].stockpile.fuel = 100;
    let venus = station_at(&mut g, Seat(0), BodyId::Venus);
    g.colony_mut(venus).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    let (_, _) = colony_ship_ready(&mut g, BodyId::Earth);
    g.ai_orders(Seat(0));
    assert!(g.log.iter().any(|l| l.contains("to Venus")), "Venus never offered as a destination: {:#?}", g.log.iter().filter(|l| l.contains("send")).collect::<Vec<_>>());
}

/// The AI sweep: a Custodian Mine off Earth is worth twice its base while a Factory of theirs on
/// Earth could be idled to double it (Production Moved, ticket #82); with no Factory to idle it is
/// worth its base, as every other seat's Mine is. Until the sweep the Custodian AI never built a
/// Module off Earth in eight batches of twenty seeds: Earth's Facilities outscored them at the same
/// base and the Materials reserve starved the rest.
#[test]
fn the_custodian_ai_weighs_a_mine_off_earth_by_the_doubling_an_idle_factory_would_give() {
    let score_of_moon_mine = |factory: bool| -> f64 {
        let mut g = game();
        calm(&mut g);
        g.seats[0].stockpile.energy = 500;
        g.seats[0].stockpile.materials = 500;
        colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Generator], 4);
        for st in &mut g.states {
            st.facilities.retain(|f| f.kind != FacilityKind::Factory);
        }
        if factory {
            g.state_mut(StateId::EastAsia).facilities.push(facility(FacilityKind::Factory));
        }
        g.ai_orders(Seat(0));
        let l = g.log.iter().find(|l| l.contains("build Mine at") && l.contains("on the Moon")).unwrap_or_else(|| panic!("no Moon Mine scored: {:#?}", g.log.iter().filter(|l| l.contains("Moon")).collect::<Vec<_>>())).clone();
        l.split_whitespace().nth(1).and_then(|n| n.parse().ok()).unwrap()
    };
    let (with, without) = (score_of_moon_mine(true), score_of_moon_mine(false));
    assert!(without > 0.0 && (with - 2.0 * without).abs() < 1e-6, "a Moon Mine with a Factory to idle should score twice one without: {with} vs {without}");
}

/// The AI sweep: the Custodian AI idles a Factory for an even trade too, since the Emissions leave
/// Earth with the output. East Asia's Factory makes 6 (a Materials lean); a Moon Mine makes 6.
#[test]
fn the_custodian_ai_idles_a_factory_for_an_even_trade() {
    let mut g = game();
    calm(&mut g);
    g.seats[0].stockpile.energy = 500;
    g.seats[0].stockpile.materials = 10;
    colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Mine, ModuleKind::Generator], 0);
    g.state_mut(StateId::EastAsia).facilities.push(facility(FacilityKind::Factory));
    let i = g.state(StateId::EastAsia).facilities.len() - 1;
    let orders = g.ai_orders(Seat(0));
    assert!(
        orders.iter().any(|o| matches!(o, Order::Change { building: BuildingRef::Facility(StateId::EastAsia, j), what: BuildingChange::Mothball } if *j == i)),
        "an even trade (6 for 6) is taken: {orders:?}"
    );
}

/// Version 0.07.0: Fund the Archive is decided BEFORE Income, so a turn whose Research completes
/// the shared Tech still pays the fund. Under 0.06.0 the order ran in the Orders phase and clawed
/// the Research back out of `research.contributions`, which a completion had already zeroed, so the
/// Archivists banked nothing and the Report said they had funded the Archive.
#[test]
fn funding_the_archive_pays_even_when_the_turn_completes_a_tech() {
    let mut g = game();
    g.state_mut(StateId::Europe).control = Control::Controlled(Seat(3));
    g.state_mut(StateId::Europe).facilities.push(facility(FacilityKind::ResearchLab));
    g.pick_tech(Seat(0), TechId::CoastalEngineering).unwrap();
    // One point short, so this turn's Research would finish the Tech during Income.
    g.research.progress = g.tables.tech(TechId::CoastalEngineering).cost - 1;
    g.commit_orders(Seat(3), &[Order::SetResearchDirective { percent: 100 }]);
    g.income_phase();
    let made = g.seats[3].research_last_turn;
    assert!(made > 0, "the Lab made {made} Research");
    assert_eq!(g.seats[3].archive_fund, made, "every point the Labs made is in the fund");
}

/// Version 0.07.0 (ticket #97): a Colony or a Space Station holds `base` Modules free and one more
/// for every Colonist living there. Before this rule a Colony had no ceiling at all, and Build Where
/// You Dig made each further Module cheaper than the last: one playtested Moon Colony reached 51
/// Mines and 28 Generators on about twelve Colonists, extracting 775 Materials a turn by turn 13.
///
/// Ticket #164 (version 0.07.5): `base` is now 0 -- the Core Module a founding gives is the whole of
/// what a founding gives, and every slot after it is bought with a Colonist. So the cap is exactly
/// the number of people living there, and a place nobody lives in builds nothing.
#[test]
fn a_colony_holds_one_module_for_each_colonist_and_none_without() {
    let mut g = game();
    g.seats[0].stockpile.materials = 2000;
    let c = colony(&mut g, Seat(0), BodyId::Moon, &[], 2);
    // One for each of two Colonists, and nothing free on top.
    assert_eq!(g.module_slots(g.colony(c).unwrap()), 2);
    assert_eq!(g.free_module_slots(g.colony(c).unwrap()), 2);
    for _ in 0..2 {
        g.colony_mut(c).unwrap().modules.push(Module::new(ModuleKind::Mine));
    }
    let order = Order::BuildModule { colony: c, kind: ModuleKind::Mine };
    assert!(g.check_order(Seat(0), &[], &order).is_err(), "a Colony of two Colonists holds two Modules, not three");
    // A Colonist buys exactly one more.
    g.colony_mut(c).unwrap().colonists = 3;
    assert_eq!(g.module_slots(g.colony(c).unwrap()), 3);
    assert!(g.check_order(Seat(0), &[], &order).is_ok(), "the third Colonist bought a third");
    // An order already given this turn takes its room, so a cap of six cannot be filled twice over.
    assert!(g.check_order(Seat(0), std::slice::from_ref(&order), &order).is_err(), "the pending order took the last slot");
    // A mothballed Module keeps its slot, as a mothballed Facility does in a Nation State.
    g.colony_mut(c).unwrap().modules[0].mothballed = true;
    assert!(g.check_order(Seat(0), &[], &order).is_ok(), "mothballing frees Energy, never room");
    assert_eq!(g.module_slots_used(g.colony(c).unwrap()), 2);
    // One under construction reserves its slot.
    let due = g.turn + 1;
    g.colony_mut(c).unwrap().queue.push(Build { item: BuildItem::Module(ModuleKind::Mine), seat: Seat(0), due_turn: due, coastal: false });
    assert_eq!(g.module_slots_used(g.colony(c).unwrap()), 3);
    assert!(g.check_order(Seat(0), &[], &order).is_err(), "the one building holds the last slot");
    // The Archive is exempt, and counted on neither side of the sum.
    g.colony_mut(c).unwrap().modules.push(Module::new(ModuleKind::Archive));
    assert_eq!(g.module_slots_used(g.colony(c).unwrap()), 3, "the Archive is not counted");
    // Ticket #164: and so is the Core Module every founding gives.
    g.colony_mut(c).unwrap().modules.push(Module::new(ModuleKind::Core));
    assert_eq!(g.module_slots_used(g.colony(c).unwrap()), 3, "the Core Module is not counted either");
}

/// Ticket #97: a Space Station reads the same rule, so it grows only as its people arrive.
///
/// Ticket #164 (version 0.07.5): and with `base` at 0 a station nobody lives on builds **nothing**.
/// That is not the deadlock it would once have been, because the Core Module it is founded with
/// holds four: people can arrive the turn it is built, and each one buys a slot.
#[test]
fn a_station_builds_nothing_until_someone_lives_on_it_and_its_core_module_holds_the_first_four() {
    let mut g = game();
    g.seats[0].stockpile.materials = 2000;
    let station = g.colonies.iter().find(|c| c.in_orbit && c.control.director() == Some(Seat(0))).map(|c| c.id).expect("a station over Earth");
    assert_eq!(g.colony(station).unwrap().colonists, 0, "it starts with nobody on it");
    assert_eq!(g.colony(station).unwrap().modules.len(), 1, "and with its Core Module");
    assert_eq!(g.colony(station).unwrap().modules[0].kind, ModuleKind::Core);
    assert_eq!(g.module_slots(g.colony(station).unwrap()), 0, "no slots, and no people yet");
    let habitat = Order::BuildModule { colony: station, kind: ModuleKind::Habitat };
    assert!(g.check_order(Seat(0), &[], &habitat).is_err(), "a station nobody lives on builds nothing");
    // The Core Module is what breaks the circle: it holds four before anything is built.
    assert_eq!(g.habitat_room(g.colony(station).unwrap()), 4, "the Core Module holds four");
    // People arriving buy the room, one slot each.
    g.colony_mut(station).unwrap().colonists = 2;
    assert_eq!(g.module_slots(g.colony(station).unwrap()), 2);
    assert!(g.check_order(Seat(0), &[], &habitat).is_ok());
}

/// Ticket #97: the designer's constraint on the ratio -- a filled Habitat must always hand back at
/// least two slots after paying for its own -- holds on the poorest Colony Slot in the game. Phobos
/// and Deimos have a Habitat yield of 0.5, so a Habitat there holds three.
#[test]
fn a_filled_habitat_always_hands_back_at_least_two_slots() {
    let mut g = game();
    for body in BodyId::ALL {
        for slot in g.free_slots_on(body) {
            let c = Colony {
                id: ColonyId(g.fresh_id()),
                body,
                slot,
                control: Control::Controlled(Seat(0)),
                modules: vec![Module::new(ModuleKind::Habitat)],
                colonists: 0, education: 1.0, settler_education: 1.0,
                queue: Vec::new(),
                grid_failed: false,
                founded_turn: 1,
                in_orbit: false,
            };
            let holds = g.habitat_room(&c);
            let filled = Colony { colonists: holds, education: 1.0, settler_education: 1.0, ..c.clone() };
            // What the Habitat earns, less the one slot the Habitat itself takes.
            let net = g.module_slots(&filled) as i64 - g.module_slots(&c) as i64 - 1;
            assert!(
                net >= 2,
                "a full Habitat at {} slot {slot} holds {holds} and hands back {net} slots, not two",
                body.name()
            );
        }
    }
}

/// Ticket #98 (version 0.07.0): the Research Lead picks from a drawn shortlist, not from everything
/// available. Before this a seat holding the Lead chose the whole tree and had no reason ever to
/// pick a rival's Victory gate: one playtested seat led 9 of 9 Techs, and a Prospector AI finished
/// on 0.99 of its own Victory Condition unable to win, the Extraction Charter never researched.
#[test]
fn the_research_lead_picks_from_a_shortlist_of_three() {
    let mut g = game();
    // The opening is a free choice of the whole of rung 1: nothing is drawn for it.
    assert!(g.research.shortlist.is_empty(), "the first Tech of the game is not drawn for");
    assert_eq!(g.pickable_techs().len(), 6, "all six of rung 1");
    g.pick_tech(Seat(0), TechId::PublicScience).unwrap();
    // Seat 0 is the human here, and the only contributor, so it leads and is asked to pick.
    let cost = g.tables.tech(TechId::PublicScience).cost;
    g.accrue_research(Seat(0), cost);
    assert!(g.research.done.contains(&TechId::PublicScience));
    assert_eq!(g.research.awaiting_pick, Some(Seat(0)));
    let list = g.research.shortlist.clone();
    assert_eq!(list.len(), 3, "three, as techs.toml says: {list:?}");
    assert_eq!(g.pickable_techs(), list, "the panel offers the list and nothing else");
    for t in &list {
        assert!(g.available_techs().contains(t), "{t:?} was drawn but is not available");
    }
    // A Tech that is available but off the list is refused.
    let off = g.available_techs().into_iter().find(|t| !list.contains(t)).expect("something off the list");
    assert!(g.pick_tech(Seat(0), off).is_err(), "{off:?} is off the shortlist {list:?}");
    // One on it is taken. Ticket #173 (version 0.07.6): a human Lead's pick is provisional until
    // the turn ends, so the list is KEPT -- the Lead may change its mind, and redrawing the three
    // on every change would be a free reroll -- and only the commit throws it away.
    assert!(g.pick_tech(Seat(0), list[0]).is_ok());
    assert_eq!(g.research.shortlist, list, "the list is kept while the pick can still change");
    assert!(!g.research.pick_committed);
    // And the Lead may pick again, off the same three.
    assert!(g.pick_tech(Seat(0), list[1]).is_ok(), "a provisional pick can be changed");
    assert_eq!(g.research.current, Some(list[1]));
    g.commit_pick();
    assert!(g.research.shortlist.is_empty(), "committing clears the list");
}

/// Ticket #173 (version 0.07.6): a human Lead's Tech pick is not locked in until the turn ends.
/// The designer: *"tech choice is not locked in until the turn is ended."* Until then the pick can
/// be changed as often as the player likes, nothing is spent, and the shortlist is kept; the turn
/// ending is what makes it final. A computer seat's pick still commits in the same breath.
#[test]
fn a_tech_pick_can_be_changed_until_the_turn_ends() {
    let mut g = game();
    g.research.shortlist = Vec::new();
    g.accrue_research(Seat(2), 6);
    assert_eq!(g.research.unallocated[2], 6, "banked while nothing is under research");

    g.pick_tech(Seat(0), TechId::PublicScience).unwrap();
    assert_eq!(g.research.current, Some(TechId::PublicScience));
    assert!(!g.research.pick_committed, "a human pick is provisional");
    assert_eq!(g.research.unallocated[2], 6, "and spends nothing");
    assert!(g.end_turn_refusal().is_none(), "the turn is no longer owed a Tech");

    // Changed, twice, for nothing.
    g.pick_tech(Seat(0), TechId::DeepMining).unwrap();
    g.pick_tech(Seat(0), TechId::CleanPropellant).unwrap();
    assert_eq!(g.research.current, Some(TechId::CleanPropellant));
    assert_eq!(g.research.unallocated[2], 6, "still nothing spent");

    // Ending the turn makes it final: the bank pours in under its owner's name.
    let orders: [Vec<Order>; SEAT_COUNT] = std::array::from_fn(|_| Vec::new());
    g.end_turn(orders).expect("the turn ends");
    assert!(g.research.pick_committed, "the turn ending locks it in");
    assert_eq!(g.research.unallocated[2], 0, "the bank is spent now");
    // Version 0.08.0: the world makes enough Research on turn 1 that this pick can now COMPLETE in
    // the same breath, and a completed Tech clears the contributions. Either way the bank left seat
    // 2's hands under seat 2's name, which is what this test is about; the assertion says both.
    assert!(
        g.research.contributions[2] >= 6 || g.research.done.contains(&TechId::CleanPropellant),
        "the bank was neither credited to seat 2 nor spent finishing the Tech: contributions {:?}, done {:?}",
        g.research.contributions,
        g.research.done
    );

    // And a committed Tech is settled: it cannot be swapped for another.
    assert!(g.pick_tech(Seat(0), TechId::DeepMining).is_err(), "a committed pick is final");
}

/// Ticket #98: a Faction can be denied a rival's gate but never its own, so the Lead's own Victory
/// gate is always drawn once its prerequisites are met.
#[test]
fn the_shortlist_always_carries_the_leads_own_victory_gate() {
    let mut g = game();
    let gate = g.tables.victory_gate(FactionKind::Custodians).expect("the Custodians have a gate");
    // Open the gate's prerequisites so it is available to be drawn.
    for need in g.tables.tech(gate).needs.clone() {
        g.research.done.push(need);
    }
    assert!(g.available_techs().contains(&gate), "the gate is available");
    assert!(g.available_techs().len() > 3, "and there is more available than the list holds");
    // Seat 0 is the Custodians. Drawn many times over, the gate is on every list.
    for _ in 0..25 {
        g.draw_shortlist(Seat(0));
        assert!(g.research.shortlist.contains(&gate), "the Lead's own gate is always drawn: {:?}", g.research.shortlist);
    }
    // A rival's gate is not owed the same courtesy.
    let rival = g.tables.victory_gate(FactionKind::Prospectors).expect("the Prospectors have a gate");
    for need in g.tables.tech(rival).needs.clone() {
        g.research.done.push(need);
    }
    let mut seen_without = false;
    for _ in 0..25 {
        g.draw_shortlist(Seat(0));
        if !g.research.shortlist.contains(&rival) {
            seen_without = true;
        }
    }
    assert!(seen_without, "a rival's gate can be left off");
}

/// Ticket #108 (version 0.07.0): a Leapfrog now takes a bite out of the state's Baseline Emissions
/// as well as its people's coefficient. Leapfrog measured at about ten times a Scrubber's cost per
/// ppm and was bought zero times in twelve playtested games; `baseline x Industry Level` was a floor
/// nothing in the game could lower. This answers both.
#[test]
fn a_leapfrog_lowers_the_states_baseline_emissions_as_well() {
    let mut g = game();
    let sid = StateId::EastAsia;
    g.state_mut(sid).control = Control::Controlled(Seat(0));
    g.seats[0].stockpile.ducats = 500;
    let before = g.baseline_emissions(sid);
    assert!(before > 0.0, "East Asia has a Baseline of {before}");
    g.commit_orders(Seat(0), &[Order::Leapfrog { state: sid }]);
    let cut = g.tables.climate.leapfrog_baseline_cut;
    assert!((g.baseline_emissions(sid) - (before - cut)).abs() < 1e-9, "one Leapfrog takes {cut} off the Baseline");
    // It never goes below nothing, however many are bought.
    for _ in 0..20 {
        g.state_mut(sid).baseline_cut += cut;
    }
    assert_eq!(g.baseline_emissions(sid), 0.0, "a Baseline never falls below nothing");
}


/// Ticket #99 (version 0.07.0): a blockade is the business of ONE Orbital Slot. A playtested
/// Archivist had the Archive complete at 80 of 80 on turn 19 and could not put Colonists into their
/// OWN Space Station for nine turns, because a single rival Frigate sat in Earth orbit and the old
/// rule shut the whole Body to everyone but its holder.
#[test]
fn a_warship_blockades_the_orbital_slot_it_sits_in_and_nothing_more() {
    let mut g = game();
    let station = g.colonies.iter().find(|c| c.in_orbit && c.control.director() == Some(Seat(0))).map(|c| c.id).expect("a station over Earth");
    let (body, slot) = { let c = g.colony(station).unwrap(); (c.body, c.slot) };
    g.colony_mut(station).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    // A rival Frigate arrives at the Body at large: it blockades nothing.
    let rival = ShipId(g.fresh_id());
    g.ships.push(Ship {
        name: String::new(), id: rival, kind: UnitKind::Frigate, seat: Seat(1), damage: 0, at: ShipAt::Body(body), colonists: 0, colonists_education: 1.0, army: None,
        stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None,
    });
    // A warship of ours contests the orbit, so nobody holds Orbital Control and the ground is open:
    // that isolates the slot rule from the ground rule.
    let mine = ShipId(g.fresh_id());
    g.ships.push(Ship {
        name: String::new(), id: mine, kind: UnitKind::Frigate, seat: Seat(0), damage: 0, at: ShipAt::Body(body), colonists: 0, colonists_education: 1.0, army: None,
        stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None,
    });
    assert_eq!(g.orbital_control(body), None, "contested, so nobody holds it");
    assert!(!g.slot_blockaded_against(Seat(0), body, slot), "a Ship at the Body at large blockades nothing");
    assert!(g.may_unload_into(Seat(0), station), "your own station is reachable");
    assert!(g.may_land(Seat(0), body), "and so is the ground");
    // It moves into the station's own slot: now that one slot is shut, and only that one.
    g.ship_mut(rival).unwrap().slot = Some(slot);
    // Ticket #278 (version 0.08.5): only once it is ORDERED to blockade. Sitting there on Hold it
    // blockades nothing: "the blockade needs to be positively chosen, not just the presence of a ship".
    assert!(!g.slot_blockaded_against(Seat(0), body, slot), "a warship in the slot on Hold blockades nothing");
    assert!(g.may_unload_into(Seat(0), station), "and the station is open");
    g.ship_mut(rival).unwrap().stance = Stance::Blockade;
    assert!(g.slot_blockaded_against(Seat(0), body, slot));
    assert!(!g.may_unload_into(Seat(0), station), "the station in the blockaded slot is shut");
    assert!(g.may_land(Seat(0), body), "the ground is untouched by a slot blockade");
    assert_eq!(g.slot_blockaders(body, slot), vec![Seat(1)]);
    // Another station of another slot at the same Body is untouched.
    let other = g.colonies.iter().find(|c| c.in_orbit && c.body == body && c.slot != slot).map(|c| c.id).expect("a second station");
    let other_seat = g.colony(other).unwrap().control.controller().unwrap();
    assert!(g.may_unload_into(other_seat, other), "a blockade reaches no further than its slot");
    // And the blockader itself may still use the slot it holds.
    assert!(!g.slot_blockaded_against(Seat(1), body, slot), "it does not blockade itself");
}

/// Ticket #99: the ground answers to Orbital Control alone, and only a RIVAL holding it outright
/// shuts the surface. Two rivals' warships present used to deny everybody, punishing the bystander.
#[test]
fn only_a_rival_holding_orbital_control_shuts_the_ground() {
    let mut g = game();
    let body = BodyId::Moon;
    let push = |g: &mut Game, seat: Seat| {
        let id = ShipId(g.fresh_id());
        g.ships.push(Ship {
            id,
            name: String::new(), kind: UnitKind::Frigate, seat, damage: 0, at: ShipAt::Body(body), colonists: 0, colonists_education: 1.0, army: None,
            stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None,
        });
    };
    assert!(g.may_land(Seat(0), body), "an empty orbit lands anyone");
    push(&mut g, Seat(1));
    assert_eq!(g.orbital_control(body), Some(Seat(1)));
    assert!(!g.may_land(Seat(0), body), "a rival holding the orbit outright shuts the ground");
    assert!(g.may_land(Seat(1), body), "its holder lands freely");
    // A second rival contests it: nobody holds it, so nobody is shut out.
    push(&mut g, Seat(2));
    assert_eq!(g.orbital_control(body), None);
    assert!(g.may_land(Seat(0), body), "a contested orbit no longer punishes the bystander");
}

/// Ticket #282 (version 0.08.5): neutral states arm when threatened. A neutral Region with a built
/// Army of any Faction next door raises a Levy at Industry + 2 at Income, a second standing Army
/// that fights for it and never marches; when the threat passes the Levy stands down at the next
/// Income; a neutral that is attacked and holds gains +1 to its Standing Army for good, to
/// Industry + 4; and a destroyed Standing Army returns two Incomes later, not the next.
#[test]
fn a_threatened_neutral_raises_a_levy_and_stands_it_down_and_holding_arms_it_for_good() {
    let mut g = game();
    calm(&mut g);
    // Egypt is neutral and borders Saudi Arabia; a built Army of seat 0's stands there.
    let (egypt, next_door) = (StateId::NorthAfrica, StateId::ArabianPeninsula);
    assert_eq!(g.state(egypt).control, Control::Neutral);
    assert!(g.tables.state(egypt).neighbours.contains(&next_door), "adjacent");
    assert!(!g.neutral_threatened(egypt), "nobody next door yet");
    let army = ArmyId(g.fresh_id());
    g.armies.push(Army { name: String::new(), id: army, home: ArmyHome::State(StateId::EastAsia), at: ArmyAt::Place(Place::State(next_door)), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None, levy: false });
    assert!(g.neutral_threatened(egypt), "a foreign built Army next door threatens it, whatever its stance");
    g.income_phase();
    let levy = g.levy_at(egypt).expect("a Levy raised at Income");
    let l = g.armies.iter().find(|a| a.id == levy).unwrap().clone();
    assert!(l.standing && l.levy && l.at == ArmyAt::Place(Place::State(egypt)));
    assert_eq!(g.army_strength(&l), (g.state(egypt).industry_level + 2) as i64, "Industry + 2");
    assert!(g.army_name(&l).contains("Egyptian"), "named from its home: {}", g.army_name(&l));
    assert_eq!(g.armies.iter().filter(|a| a.standing && !a.levy && a.home == ArmyHome::State(egypt)).count(), 1, "the Standing Army is still one");
    assert!(g.report.lines.iter().any(|l| l.text.contains("Egypt arms")), "the Report says so: {:?}", g.report.lines);
    assert!(g.levies_raised >= 1, "counted for the sweep; every neutral bordering that Army arms, so more than Egypt may have");
    g.income_phase();
    assert_eq!(g.levy_at(egypt), Some(levy), "one Levy, not one a turn");
    // The threat leaves: the Levy stands down at the next Income.
    g.armies.retain(|a| a.id != army);
    g.income_phase();
    assert!(g.levy_at(egypt).is_none(), "stood down");
    assert!(g.report.lines.iter().any(|l| l.text.contains("Egypt stands down")), "{:?}", g.report.lines);
    // A Region that changes hands stands its Levy down at once.
    g.armies.push(Army { name: String::new(), id: army, home: ArmyHome::State(StateId::EastAsia), at: ArmyAt::Place(Place::State(next_door)), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None, levy: false });
    g.income_phase();
    assert!(g.levy_at(egypt).is_some());
    g.transfer_control(Place::State(egypt), Seat(1), "Influence");
    assert!(g.levy_at(egypt).is_none(), "a held Region has no Levy");

    // Holding: +1 for good, to Industry + 4.
    let mut g = game();
    calm(&mut g);
    let cap = g.standing_army_cap(egypt);
    g.neutral_held(egypt);
    assert_eq!(g.standing_army_cap(egypt), cap + 1, "one step earned");
    assert!(g.report.lines.iter().any(|l| l.text.contains("Egypt held against the attack")), "{:?}", g.report.lines);
    for _ in 0..5 {
        g.neutral_held(egypt);
    }
    assert_eq!(g.standing_army_cap(egypt), g.state(egypt).industry_level + 4, "never past Industry + 4");
    assert_eq!(g.neutral_holds, 6, "every hold is counted for the sweep");

    // The respawn: a destroyed Standing Army returns two Incomes later, not the next.
    let mut g = game();
    calm(&mut g);
    let standing = g.armies.iter().find(|a| a.standing && a.home == ArmyHome::State(egypt)).map(|a| a.id).unwrap();
    g.destroy_army(standing, "battle", Some(ReportPlace::State(egypt)));
    g.income_phase();
    assert!(!g.armies.iter().any(|a| a.standing && !a.levy && a.home == ArmyHome::State(egypt)), "not the next Income");
    g.income_phase();
    let back = g.armies.iter().find(|a| a.standing && !a.levy && a.home == ArmyHome::State(egypt)).expect("but the one after");
    assert_eq!(g.army_strength(back), 1, "at strength 1");
}

/// Ticket #281 (version 0.08.5): a Battle is a line of the Report at its real place, its parties
/// name every unit and what it took, an aggressor carries its first-round odds; when a unit died
/// the line ranks with a Ship destroyed and a Moment names the loss, and when nobody lost one the
/// line is unranked and no Moment fires; an Army destroyed is a line by name.
#[test]
fn a_battle_is_a_report_line_at_its_place_by_name_with_odds_and_a_moment_when_a_unit_dies() {
    let mut g = game();
    calm(&mut g);
    let target = StateId::NorthAfrica;
    let defender = g.armies.iter().find(|a| a.standing && a.home == ArmyHome::State(target)).cloned().expect("Egypt's Standing Army");
    let defender_name = g.army_name(&defender);
    assert!(defender_name.contains("Egyptian"), "named from its home: {defender_name}");
    occupier_in(&mut g, StateId::EastAsia, target);
    g.resolution_phase();
    let line = g.report.battles.iter().find(|b| b.at == Some(ReportPlace::State(target))).expect("a Battle at Egypt, with its real place");
    let agg = line.parties.iter().find(|p| p.aggressor).expect("an aggressor");
    assert_eq!(agg.seat, Some(Seat(0)));
    let odds = agg.odds.expect("the odds the aggressor faced");
    assert!(odds > 0.0 && odds < 1.0, "first-round odds: {odds}");
    let neutral = line.parties.iter().find(|p| p.seat.is_none()).expect("the neutral party");
    assert!(neutral.odds.is_none(), "a defender carries no odds");
    assert!(neutral.units.starts_with(&defender_name), "the party text names the unit: {}", neutral.units);
    assert!(neutral.units.contains("took") || neutral.units.contains("escaped") || neutral.units.contains("destroyed"), "and says what it took: {}", neutral.units);
    let lost: Vec<&String> = line.parties.iter().flat_map(|p| p.destroyed.iter()).collect();
    let report = g.report.lines.iter().find(|l| l.text.starts_with("Battle at Egypt")).expect("a Report line for the Battle");
    assert_eq!(report.place, Some(ReportPlace::State(target)), "the line jumps to the place");
    assert!(report.text.contains(&format!("{:.0}% first-round odds", odds * 100.0)), "and says the odds, labelled: {}", report.text);
    let moments = g.report.moments.iter().filter(|m| m.kind == MomentKind::DecisiveBattle).count();
    if lost.is_empty() {
        assert_eq!(report.kind, LineKind::Battle, "a bloodless Battle is unranked");
        assert!(report.text.contains("nobody lost a unit"), "{}", report.text);
        assert_eq!(moments, 0, "and stops nobody's turn");
    } else {
        assert_eq!(report.kind, LineKind::DecisiveBattle, "a Battle that cost a unit ranks with a Ship destroyed");
        assert!(report.text.contains(lost[0].as_str()), "and names the loss: {}", report.text);
        assert_eq!(moments, 1, "and is a Moment");
        assert!(g.report.moments.iter().any(|m| m.kind == MomentKind::DecisiveBattle && m.text.contains(lost[0].as_str())), "the Moment names it: {:?}", g.report.moments);
    }
    assert!(LineKind::Battle.headline_rank().is_none() && LineKind::DecisiveBattle.headline_rank() == Some(4));
    assert_eq!(LineKind::Battle.section(Some(ReportPlace::Body(BodyId::Mars))), Section::InSpace, "a Battle in orbit files under In space");

    // An Army destroyed is a line by name, wherever it dies.
    let mut g = game();
    let a = g.armies.iter().find(|a| a.standing && a.home == ArmyHome::State(StateId::EastAsia)).cloned().unwrap();
    let name = g.army_name(&a);
    g.destroy_army(a.id, "battle", Some(ReportPlace::State(StateId::EastAsia)));
    assert!(g.armies.iter().all(|x| x.id != a.id));
    let line = g.report.lines.iter().find(|l| l.kind == LineKind::DecisiveBattle && l.text.contains(&name)).expect("a line naming the Army");
    assert!(line.text.contains("destroyed (battle)") && line.place == Some(ReportPlace::State(StateId::EastAsia)), "{}", line.text);
}

/// Ticket #280 (version 0.08.5): a building that makes no resource says what it does, in the
/// sentence on its row in the data, where the row said "no output"; a Unique that makes a resource
/// says its clause beside it; the Scrubber's row reads the data's upkeep, not a hand-written one;
/// and the School's sentence carries the `[school]` figures, so the two cannot drift apart.
#[test]
fn a_building_with_no_output_says_what_it_does_in_the_datas_words() {
    let g = game();
    let sid = StateId::EastAsia;
    let school = g.facility_yield(Seat(0), sid, FacilityKind::School).text();
    assert!(!school.contains("no output"), "no longer 'no output': {school}");
    let s = &g.tables.school;
    assert!(school.contains(&format!("{:.2} a turn", s.per_turn)) && school.contains(&format!("to {:.1}", s.ceiling)), "the School's sentence carries the [school] figures: {school}");
    assert!(school.ends_with("2 Energy upkeep"), "and the upkeep still closes the line: {school}");
    let constabulary = g.facility_yield(Seat(0), sid, FacilityKind::Constabulary).text();
    assert!(constabulary.contains("Unrest") && !constabulary.contains("no output"), "{constabulary}");
    let scrubber = g.facility_yield(Seat(0), sid, FacilityKind::Scrubber).text();
    assert!(scrubber.contains(&format!("{} Energy upkeep", g.tables.facility(FacilityKind::Scrubber).energy_upkeep)) && scrubber.contains("Natural Sink"), "{scrubber}");
    let bank = g.facility_yield(Seat(0), sid, FacilityKind::InvestmentBank).text();
    assert!(bank.contains(" Ducats, banks 1%"), "a Unique says its clause beside its resource: {bank}");
    // Modules the same way, at a station over Earth.
    let station = g.colonies.iter().find(|c| c.in_orbit && c.control.director() == Some(Seat(0))).map(|c| c.id).expect("a station");
    for (kind, word) in [(ModuleKind::Habitat, "Colonists"), (ModuleKind::Shipyard, "Ship"), (ModuleKind::Barracks, "Army"), (ModuleKind::Institute, "Education")] {
        let text = g.module_yield(Seat(0), station, kind).text();
        assert!(text.contains(word) && !text.contains("no output"), "{}: {text}", kind.name());
    }
    // A Mine still reads as it did: a resource, no sentence.
    let mine = g.module_yield(Seat(0), station, ModuleKind::Mine).text();
    assert!(mine.starts_with("+") && !mine.contains("no output"), "{mine}");
}

/// Ticket #279 (version 0.08.5): Battles pollute. Every hit landed in a Battle on Earth puts the
/// table's ppm per hit into next Climate phase's war bucket, worn by the seat that landed it and
/// by nobody for a neutral Region's own Army; every building burned in the rolls after it puts the
/// table's ppm per building on the aggressors; the Climate phase adds it to the stock as its own
/// source, counted against Stabilization, and to the seat's Blame as emitted. Mars orbit charges nothing.
#[test]
fn a_battle_on_earth_pollutes_by_hits_and_buildings_burned_and_one_at_mars_does_not() {
    let mut g = game();
    calm(&mut g);
    let (per_hit, per_building) = (g.tables.climate.war_ppm_per_hit, g.tables.climate.war_ppm_per_building);
    assert!((per_hit - 0.5).abs() < 1e-9 && (per_building - 2.0).abs() < 1e-9, "half a ppm a hit, two a building");
    // Seat 0's Army attacks neutral Europe, whose Standing Army defends: hits on both sides.
    let target = StateId::NorthAfrica;
    assert_eq!(g.state(target).control, Control::Neutral, "Egypt is neutral at the start");
    let before = g.state(target).facilities.len();
    occupier_in(&mut g, StateId::EastAsia, target);
    g.resolution_phase();
    let line = g.report.battles.iter().find(|b| b.place == g.tables.state(target).name).expect("a Battle in Europe");
    let mine: u32 = line.parties.iter().filter(|p| p.seat == Some(Seat(0))).map(|p| p.hits).sum();
    let theirs: u32 = line.parties.iter().filter(|p| p.seat.is_none()).map(|p| p.hits).sum();
    assert!(mine + theirs > 0, "three rolls a round land somewhere: {line:?}");
    let burned = (before - g.state(target).facilities.len()) as f64;
    let want_mine = mine as f64 * per_hit + burned * per_building;
    assert!((g.climate.war_next[0] - want_mine).abs() < 1e-9, "seat 0 wears its {mine} hits and the {burned} buildings it burned: {} against {want_mine}", g.climate.war_next[0]);
    assert!((g.climate.war_next_nobody - theirs as f64 * per_hit).abs() < 1e-9, "the neutral Army's {theirs} hits are nobody's: {}", g.climate.war_next_nobody);
    assert!(g.climate.war_next[1..].iter().all(|v| *v == 0.0), "nobody else fought");

    // The next Climate phase: its own source, on the stock, counted, and seat 0's Blame.
    let want_total = want_mine + theirs as f64 * per_hit;
    let (co2_before, emitted_before) = (g.climate.co2, g.seats[0].blame_emitted);
    g.climate_phase();
    let b = &g.climate.last;
    assert!((b.war - want_total).abs() < 1e-9, "the War source: {} against {want_total}", b.war);
    let rest = b.state_industry + b.factories + b.power_plants + b.refineries + b.launches + b.population;
    assert!((b.counted() - rest - b.war).abs() < 1e-9, "counted against Stabilization, beside the buildings, launches and people");
    assert!((g.climate.co2 - co2_before - b.net()).abs() < 1e-9, "and on the stock");
    assert!((g.seats[0].blame_emitted - emitted_before - b.by_seat[0]).abs() < 1e-9 && b.by_seat[0] >= want_mine - 1e-9, "seat 0's share is its Blame, emitted");
    assert!((g.seats[0].war_ppm - want_mine).abs() < 1e-9 && (g.climate.war_nobody_total - theirs as f64 * per_hit).abs() < 1e-9, "kept for the sweep");
    assert!(g.climate.war_next.iter().all(|v| *v == 0.0) && g.climate.war_next_nobody == 0.0, "the bucket is drained");

    // A Battle in Mars orbit fouls nobody's air.
    let push = |g: &mut Game, seat: Seat| {
        let id = ShipId(g.fresh_id());
        g.ships.push(Ship {
            id,
            name: String::new(), kind: UnitKind::Frigate, seat, damage: 0, at: ShipAt::Body(BodyId::Mars), colonists: 0, colonists_education: 1.0, army: None,
            stance: Stance::Attack, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None,
        });
    };
    push(&mut g, Seat(0));
    push(&mut g, Seat(1));
    g.armies.retain(|a| a.standing);
    g.resolution_phase();
    assert!(g.report.battles.iter().any(|b| b.place.contains("Mars")), "a Battle in Mars orbit: {:?}", g.report.battles);
    assert!(g.climate.war_next.iter().all(|v| *v == 0.0) && g.climate.war_next_nobody == 0.0, "and nothing in the war bucket");
}

/// Ticket #278 (version 0.08.5): a Blockade is a stance a warship stack is ordered into, refused
/// where no warship of the seat's sits in a slot to blockade; a station under one makes nothing and
/// still pays its upkeep, its holder is offended at weight 1 each Income, the Report says so, and
/// the sweep's counters move.
#[test]
fn a_blockade_is_ordered_and_starves_the_station_in_its_slot_upkeep_still_paid() {
    let mut g = game();
    calm(&mut g);
    let station = g.colonies.iter().find(|c| c.in_orbit && c.control.director() == Some(Seat(0))).map(|c| c.id).expect("a station over Earth");
    let (body, slot) = { let c = g.colony(station).unwrap(); (c.body, c.slot) };
    // Something to starve: a Solar Array making Energy for 0 upkeep would hide the upkeep, so an
    // Observatory (Research, 2 Energy upkeep) and a Solar Array (Energy) both.
    g.colony_mut(station).unwrap().modules.push(Module::new(ModuleKind::SolarArray));
    g.colony_mut(station).unwrap().modules.push(Module::new(ModuleKind::Observatory));
    let rival = ShipId(g.fresh_id());
    g.ships.push(Ship {
        name: String::new(), id: rival, kind: UnitKind::Frigate, seat: Seat(1), damage: 0, at: ShipAt::Body(body), colonists: 0, colonists_education: 1.0, army: None,
        stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None,
    });
    // The order wants a warship in a slot: at the Body at large it is refused.
    let err = g.check_order(Seat(1), &[], &Order::ShipStance { body, stance: Stance::Blockade }).unwrap_err().0;
    assert!(err.contains("no warship of yours sits in a slot"), "{err}");
    g.ship_mut(rival).unwrap().slot = Some(slot);
    assert!(g.check_order(Seat(1), &[], &Order::ShipStance { body, stance: Stance::Blockade }).is_ok(), "in the station's slot it may be ordered");
    assert_eq!(g.starved_by(station), None, "on Hold it starves nothing");

    // Two futures from the same board: the Frigate on Hold, and ordered to Blockade.
    let mut held = g.clone();
    let mut blockaded = g.clone();
    blockaded.commit_orders(Seat(1), &[Order::ShipStance { body, stance: Stance::Blockade }]);
    assert_eq!(blockaded.ship(rival).unwrap().stance, Stance::Blockade);
    assert_eq!(blockaded.starved_by(station), Some(Seat(1)), "ordered, it starves the station");
    let before = g.seats[0].stockpile.energy;
    held.income_phase();
    blockaded.income_phase();
    let (gain_held, gain_blockaded) = (held.seats[0].stockpile.energy - before, blockaded.seats[0].stockpile.energy - before);
    let array = g.module_yield_at(Seat(0), station, 1).amount;
    assert!(array > 0, "the Solar Array makes Energy: {array}");
    assert_eq!(gain_held - gain_blockaded, array, "starved, the station made nothing and paid the same upkeep: {gain_held} against {gain_blockaded}");
    assert!(blockaded.relations.offended[0][1], "each Income under a Blockade is an offence against the holder");
    assert!(!held.relations.offended[0][1], "and a Frigate on Hold offends nobody");
    assert_eq!((blockaded.seats[0].blockade_turns_suffered, blockaded.seats[1].blockade_turns_imposed), (1, 1), "counted for the sweep");
    assert!(blockaded.report.lines.iter().any(|l| l.text.contains("is blockaded by the") && l.text.contains("made nothing")), "the Report says so: {:?}", blockaded.report.lines);
    // Nothing died and nothing burned.
    assert_eq!(blockaded.colony(station).unwrap().modules.len(), g.colony(station).unwrap().modules.len());
}

/// Ticket #278 (version 0.08.5): a Colony on the ground starves only while one rival holds Orbital
/// Control of its Body outright AND has a stack there on Blockade; a contested orbit starves nobody.
#[test]
fn a_ground_colony_starves_under_outright_orbital_control_with_a_blockading_stack_and_not_under_a_contested_orbit() {
    let mut g = game();
    calm(&mut g);
    let body = BodyId::Moon;
    let cid = colony(&mut g, Seat(0), body, &[ModuleKind::Mine], 4);
    let push = |g: &mut Game, seat: Seat, stance: Stance| {
        let id = ShipId(g.fresh_id());
        g.ships.push(Ship {
            id,
            name: String::new(), kind: UnitKind::Frigate, seat, damage: 0, at: ShipAt::Body(body), colonists: 0, colonists_education: 1.0, army: None,
            stance, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None,
        });
        id
    };
    let rival = push(&mut g, Seat(1), Stance::Hold);
    assert_eq!(g.orbital_control(body), Some(Seat(1)), "held outright");
    assert_eq!(g.starved_by(cid), None, "held outright on Hold: the ground is shut to landings and nothing more");
    g.ship_mut(rival).unwrap().stance = Stance::Blockade;
    assert_eq!(g.starved_by(cid), Some(Seat(1)), "ordered to Blockade, the Colony on the ground starves");
    push(&mut g, Seat(2), Stance::Hold);
    assert_eq!(g.orbital_control(body), None, "contested");
    assert_eq!(g.starved_by(cid), None, "a contested orbit starves nobody, as it lands nobody");
}

/// Ticket #278 (version 0.08.5): a computer seat whose warship sits in a rival station's slot is
/// offered the Blockade stance; one whose Colony is starved wants a warship there more.
#[test]
fn the_computer_orders_a_blockade_where_its_warship_sits_in_a_rival_stations_slot() {
    let mut g = game();
    calm(&mut g);
    let station = g.colonies.iter().find(|c| c.in_orbit && c.control.director() == Some(Seat(0))).map(|c| c.id).expect("a station over Earth");
    let (body, slot) = { let c = g.colony(station).unwrap(); (c.body, c.slot) };
    let id = ShipId(g.fresh_id());
    g.ships.push(Ship {
        name: String::new(), id, kind: UnitKind::Frigate, seat: Seat(1), damage: 0, at: ShipAt::Body(body), colonists: 0, colonists_education: 1.0, army: None,
        stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: Some(slot),
    });
    let orders = g.ai_orders(Seat(1));
    assert!(orders.iter().any(|o| matches!(o, Order::ShipStance { body: b, stance: Stance::Blockade } if *b == body)), "a Blockade of the station whose slot it sits in: {orders:?}");
}

/// Ticket #99: a Refuel needs a station of yours that is not blockaded, and a rival warship sitting
/// in an empty Orbital Slot denies that slot to a builder.
#[test]
fn a_blockade_stops_refuelling_and_holds_an_empty_slot_against_a_builder() {
    let mut g = game();
    let station = g.colonies.iter().find(|c| c.in_orbit && c.control.director() == Some(Seat(0))).map(|c| c.id).expect("a station");
    let (body, slot) = { let c = g.colony(station).unwrap(); (c.body, c.slot) };
    g.seats[0].stockpile.fuel = 100;
    g.seats[0].stockpile.materials = 500;
    let mine = ShipId(g.fresh_id());
    g.ships.push(Ship {
        name: String::new(), id: mine, kind: UnitKind::Frigate, seat: Seat(0), damage: 0, at: ShipAt::Body(body), colonists: 0, colonists_education: 1.0, army: None,
        stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 5, slot: None,
    });
    assert!(g.check_order(Seat(0), &[], &Order::Refuel { ship: mine }).is_ok(), "an unblockaded station fuels it");
    let rival = ShipId(g.fresh_id());
    g.ships.push(Ship {
        name: String::new(), id: rival, kind: UnitKind::Frigate, seat: Seat(1), damage: 0, at: ShipAt::Body(body), colonists: 0, colonists_education: 1.0, army: None,
        stance: Stance::Blockade, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: Some(slot),
    });
    let err = g.check_order(Seat(0), &[], &Order::Refuel { ship: mine }).unwrap_err().0;
    assert!(err.contains("blockaded"), "a blockaded station fuels nothing: {err}");
    // An empty slot a rival warship sits in cannot be built into.
    let free = g.free_orbital_slots(body).first().copied().expect("a free orbital slot over Earth");
    g.ship_mut(rival).unwrap().slot = Some(free);
    let err = g.check_order(Seat(0), &[], &Order::BuildStation { body, slot: free }).unwrap_err().0;
    assert!(err.contains("rival warship"), "a warship in an empty slot denies it: {err}");
}

/// Ticket #99: a transit names the Orbital Slot it arrives into, and refuses a slot the Body has not
/// got. The choice is made with the leg, so it is made before the Ship can see who will be there.
#[test]
fn a_transit_names_the_slot_it_arrives_into() {
    let mut g = game();
    let ship = ShipId(g.fresh_id());
    g.ships.push(Ship {
        name: String::new(), id: ship, kind: UnitKind::Frigate, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, colonists_education: 1.0, army: None,
        stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None,
    });
    let slots = g.tables.body(BodyId::Moon).orbital_slots;
    let err = g.check_order(Seat(0), &[], &Order::Transit { ship, to: BodyId::Moon, slot: Some(slots) }).unwrap_err().0;
    assert!(err.contains("Orbital Slots"), "a slot the Body has not got is refused: {err}");
    let o = Order::Transit { ship, to: BodyId::Moon, slot: Some(0) };
    assert!(g.check_order(Seat(0), &[], &o).is_ok());
    g.commit_orders(Seat(0), std::slice::from_ref(&o));
    assert_eq!(g.ship(ship).unwrap().slot, Some(0), "the leg carries the choice");
}

/// Ticket #105 (version 0.07.0): the turn will not end while a human Research Lead owes the table a
/// Tech, and the rule lives in the ENGINE so every caller is bound by it. It used to live in one
/// `add_enabled` in the interface, and the headless driver added in this same version did not know
/// about it: twelve playtest games were played in which declining to pick froze the tech tree for
/// good, and that was written up as "the strongest strategy in the game". It was the harness.
#[test]
fn the_turn_will_not_end_while_a_human_lead_owes_a_tech() {
    let mut g = game();
    assert_eq!(g.research.awaiting_pick, Some(Seat(0)), "the first Tech of the game is seat 0's to pick");
    let why = g.end_turn_refusal().expect("a pick is owed, so the turn is refused");
    assert!(why.contains("Research Lead"), "and it says why: {why}");
    let before = g.turn;
    assert_eq!(g.end_turn(std::array::from_fn(|_| Vec::new())), Err(why), "the turn refuses with the same words");
    assert_eq!(g.turn, before, "and nothing advanced");
    // Picking clears it.
    pick_a_tech(&mut g);
    assert!(g.end_turn_refusal().is_none());
    assert!(g.end_turn(std::array::from_fn(|_| Vec::new())).is_ok());
    assert_eq!(g.turn, before + 1);
    // An AI Lead is never asked: it picks the moment it leads, so simulate mode is untouched.
    let mut s = Game::spectate(tables(), 7);
    s.start();
    assert!(s.end_turn_refusal().is_none(), "every seat is an AI here");
}

/// Ticket #105 (version 0.07.0): Research banked while no Tech is under research keeps its owner,
/// so it counts toward the Research Lead when it lands. Before this it was one unattributed pool,
/// which produced Lead lines like "the Prospectors led (Archivists 0, Custodians 0, Prospectors 0,
/// Arkwrights 0)": arithmetically right, and unreadable as anything but a bug.
#[test]
fn research_banked_between_techs_keeps_its_owner() {
    let mut g = game();
    g.research.current = None;
    g.research.contributions = [0; 4];
    // Two seats bank while nothing is under research, and a Breakthrough adds what is nobody's.
    g.accrue_research(Seat(2), 5);
    g.accrue_research(Seat(3), 2);
    g.add_research_unattributed(1);
    assert_eq!(g.research.unallocated[2], 5);
    assert_eq!(g.research.unallocated[3], 2);
    assert_eq!(g.research.unattributed, 1, "a Breakthrough belongs to nobody");
    // Choosing a Tech pours it all in, each share under its own name. Public Science costs 15, so
    // the eight banked points land without completing it and resetting what we are measuring.
    g.research.shortlist = Vec::new();
    g.pick_tech(Seat(0), TechId::PublicScience).unwrap();
    // Ticket #173 (version 0.07.6): the pick alone spends nothing -- it can still be changed --
    // and the bank pours when the pick is committed at the end of the turn.
    assert_eq!(g.research.unallocated[2], 5, "the bank is untouched while the pick can change");
    assert_eq!(g.research.progress, 0);
    g.commit_pick();
    assert_eq!(g.research.contributions[2], 5, "seat 2's banked Research is still seat 2's");
    assert_eq!(g.research.contributions[3], 2);
    assert_eq!(g.research.contributions[0], 0, "picking a Tech earns nothing");
    assert_eq!(g.research.progress, 8, "and every point arrived, the Breakthrough included");
    assert_eq!(g.research.unallocated, [0; 4], "the bank is emptied");
    assert_eq!(g.research.unattributed, 0);
    // So the Lead is the seat that actually did the work, not a draw among four zeroes.
    assert_eq!(g.research_lead_candidates(), vec![Seat(2)]);
}

// ---------------------------------------------------------------- Stances park

/// Ticket #115 (version 0.07.1). The designer, looking at a roster where every Army was marked "no
/// order" every turn: *"let's allow an armies orders to park them in that stance until otherwise
/// moved - a army on defense should remain on defense unless told otherwise."*
///
/// The engine already did this and nothing guarded it, which is how a rule quietly becomes a bug.
/// A stance is set once and survives every Resolution after it; only four things take it away, and
/// each of them is a thing that happened TO the unit: a Comms Blackout, a Ship arriving out of
/// transit, an Army landing from a Ship, and a state throwing off its controller.
#[test]
fn an_army_keeps_its_stance_through_resolution_until_something_happens_to_it() {
    let mut g = game();
    let id = ArmyId(g.fresh_id());
    g.armies.push(Army {
        name: String::new(),
        id,
        home: ArmyHome::State(StateId::EastAsia),
        at: ArmyAt::Place(Place::State(StateId::EastAsia)),
        damage: 0,
        standing: false,
        stance: Stance::Evade,
        escaped: false,
        move_to: None, levy: false,
    });
    for turn in 1..=3 {
        g.resolution_phase();
        assert_eq!(g.army(id).unwrap().stance, Stance::Evade, "the stance was given once and should still hold after Resolution {turn}");
    }
    // And it is not that the field is frozen: a new stance takes, and then parks in its turn.
    g.army_mut(id).unwrap().stance = Stance::Intercept;
    g.resolution_phase();
    assert_eq!(g.army(id).unwrap().stance, Stance::Intercept, "a stance given later parks the same way");
}



// ---------------------------------------------- Defects found while charting version 0.08.0

/// Ticket #178: the LAST Report of every game carried turn 0, so a game ending in 2035 opened its
/// final dispatch as *Report, January 2030* while the top bar beside it read the true date.
/// `end_turn` clears the Report for the turn to come (leaving `turn` at 0) and stamps the real turn
/// later, in `report_phase`; on game over it returns before both, so the stamp never happened.
#[test]
fn the_last_report_carries_the_turn_the_game_finished_on() {
    let mut g = with_seed(3);
    for s in Seat::ALL {
        g.seats[s.index()].ai = true;
    }
    for _ in 0..200 {
        if g.is_over() {
            break;
        }
        g.end_turn(std::array::from_fn(|_| Vec::new())).expect("every seat is AI, so no pick is ever owed");
    }
    assert!(g.is_over(), "the game should have ended inside 200 turns");
    assert_eq!(g.report.turn, g.turn, "the final Report should carry the turn the game finished on ({}), not {}", g.turn, g.report.turn);
}

/// Ticket #197: Colonists were deleted with no log line and no Report line when a station changed
/// hands. `resolve_changes` clamped EVERY Colony to its `habitat_room` -- not only one where a
/// Mothball or Decommission had just resolved -- and `habitat_room` reads the CURRENT holder's
/// Habitat capacity multiplier, which only the Arkwrights have (x1.5). So an Arkwright station
/// passing to another Faction shrank in the same Resolution and lost its people.
#[test]
fn a_station_changing_hands_does_not_delete_its_colonists() {
    let mut g = fresh();
    let arkwrights = Seat(2);
    assert_eq!(g.kind(arkwrights), FactionKind::Arkwrights, "seat 2 should be the Arkwrights");
    let id = station_at(&mut g, arkwrights, BodyId::Earth);
    for _ in 0..2 {
        g.colony_mut(id).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    }
    let arkwright_room = g.habitat_room(g.colony(id).unwrap());
    g.colony_mut(id).unwrap().colonists = arkwright_room;
    // It passes to the Custodians, who have no capacity multiplier, so the room falls.
    g.colony_mut(id).unwrap().control = Control::Controlled(Seat(0));
    let custodian_room = g.habitat_room(g.colony(id).unwrap());
    assert!(custodian_room < arkwright_room, "the premise: Arkwright room {arkwright_room} should exceed Custodian room {custodian_room}");
    g.resolution_phase();
    assert_eq!(
        g.colony(id).unwrap().colonists,
        arkwright_room,
        "nobody living at the station should be deleted by a change of hands; {arkwright_room} lived there and the new holder's room is {custodian_room}"
    );
}

/// Ticket #197, the other half: narrowing the clamp to Colonies where something actually came down
/// must NOT disable the rule it was written for. Decommissioning a Habitat still loses the people
/// it held -- and now says so, where before it went without a word.
#[test]
fn decommissioning_a_habitat_still_loses_the_people_it_held_and_says_so() {
    let mut g = fresh();
    let id = station_at(&mut g, Seat(0), BodyId::Earth);
    for _ in 0..2 {
        g.colony_mut(id).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    }
    let full = g.habitat_room(g.colony(id).unwrap());
    g.colony_mut(id).unwrap().colonists = full;
    change_now(&mut g, Seat(0), BuildingRef::Module(id, 0), BuildingChange::Decommission);
    let room = g.habitat_room(g.colony(id).unwrap());
    assert!(room < full, "the premise: one Habitat fewer should hold fewer people than {full}");
    assert_eq!(g.colony(id).unwrap().colonists, room, "the people the decommissioned Habitat held are lost with it");
    let lost = full - room;
    assert!(
        g.report.lines.iter().any(|l| l.text.contains(&format!("{lost} Colonists")) && l.text.contains("nowhere to live")),
        "the Report should say the {lost} Colonists were lost; it said: {:?}",
        g.report.lines.iter().map(|l| l.text.clone()).collect::<Vec<_>>()
    );
}

// ------------------------------------------------ 0.08.0 ticket #196: the muster gate and partial batches

/// Ticket #196: the AI's muster gate wanted a Colony Ship or a Shipyard it could never get. A bare
/// station has zero Module slots (`base = 0`, one per Colonist), so it cannot raise the Shipyard
/// that would let it muster the Colonists that earn the slots; ticket #164's Core Module ended that
/// deadlock in the rules and the gate was never updated. Measured over 320 seat-games, NO seat ever
/// held a ship or a yard while the ice was still shut, so Antarctica opening was the only door into
/// the Colonist economy for everybody. Room to put people opens it too.
#[test]
fn a_seat_with_room_on_a_station_musters_before_antarctica_opens() {
    let mut g = game();
    let seat = Seat(0);
    assert!(!g.antarctica_open, "the premise: the ice is still shut");
    assert!(!g.ships.iter().any(|s| s.seat == seat && s.kind == UnitKind::ColonyShip), "the premise: no Colony Ship");
    // Its station over Earth stands with a Core Module, which holds four people from the day it is built.
    let station = g.colonies.iter().find(|c| c.in_orbit && c.control.director() == Some(seat)).map(|c| c.id).expect("seat 0 starts with a station");
    assert!(g.habitat_room(g.colony(station).unwrap()) > 0, "the premise: the Core Module holds somebody");
    assert!(!g.colony(station).unwrap().modules.iter().any(|m| m.kind == ModuleKind::Shipyard), "the premise: no Shipyard");

    let orders = g.ai_orders(seat);
    assert!(
        orders.iter().any(|o| matches!(o, Order::BuildEmigrants { .. })),
        "with room on a station and the ice shut, the AI should still recruit: {orders:?}"
    );
}

/// Ticket #196: a Coach Class batch costs 8 x 2.0 = 16.0 population, and Australia carries 10.1 to
/// 12.6 -- the only one of the fourteen Regions below 16 -- so the Arkwright AI was refused every
/// turn it held it, 243 times across twenty measured games, and mustered nothing at all. A muster
/// now takes as many as the Region can pay for.
#[test]
fn a_muster_takes_as_many_as_the_region_can_pay_for() {
    let mut g = game();
    let ark = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Arkwrights).unwrap();
    let per = g.emigrants_per_turn(ark);
    // The Arkwrights start with no station, and ticket #196's gate wants somewhere to put people,
    // so give them one: this test is about the SIZE of a batch, not about the gate.
    let st = station_at(&mut g, ark, BodyId::Earth);
    // `station_at` predates ticket #164 and makes a BARE station; the real game founds one with a
    // Core Module, which is what holds the first four people and so gives the gate its room.
    g.colony_mut(st).unwrap().modules.push(Module::new(ModuleKind::Core));
    assert!(g.habitat_room(g.colony(st).unwrap()) > 0, "the premise: the Core Module holds somebody");
    // A Region too small to pay for a whole batch, but big enough for some of it.
    let small = g.directed_states(ark)[0];
    g.states[small as usize].population = g.lift_population(ark, per) / 2.0;
    let want = g.emigrants_affordable(ark, small);
    assert!(want > 0 && want < per, "the premise: {want} should be a partial batch out of {per}");
    assert!(
        g.check_order(ark, &[], &Order::BuildEmigrants { state: small, n: want }).is_ok(),
        "a batch the Region can pay for should be legal"
    );
    assert!(
        g.check_order(ark, &[], &Order::BuildEmigrants { state: small, n: per }).is_err(),
        "the whole batch should still be refused: the Region cannot pay for it"
    );

    // And the AI asks for what it can afford rather than for nothing.
    let orders = g.ai_orders(ark);
    match orders.iter().find_map(|o| if let Order::BuildEmigrants { n, .. } = o { Some(*n) } else { None }) {
        Some(n) => assert!(n <= want, "the AI recruited {n}, more than the {want} the Region can pay for"),
        None => panic!("the Arkwright AI recruited nothing in a Region that can pay for {want}: {orders:?}"),
    }
}

// ------------------------------------------- 0.08.0 ticket #185: the School and a moving Education Level

/// Ticket #185: the Education Level was a fixed figure on a Region's card that nothing in the game
/// moved. A School raises it a step a turn while it stands and is online, to a ceiling, and it
/// decays back at the same rate when the School stops -- so what took five turns to build takes
/// five turns to lose, and it stops at the figure the card carries.
///
/// Driven through `run_schools` rather than a whole Income phase, so the arithmetic under test is
/// not also measuring the Energy economy: over forty Income phases with no production the School
/// is shut for want of Energy, which is correct and is a different rule.
#[test]
fn a_school_raises_its_regions_education_level_a_step_a_turn_to_the_ceiling() {
    let mut g = game();
    let sid = g.directed_states(Seat(0))[0];
    let card = g.tables.state(sid).education_level;
    assert!((g.education_level(sid) - card).abs() < 1e-9, "it starts at the card figure {card}");

    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::School));
    let step = g.tables.school.per_turn;
    let ceiling = g.tables.school.ceiling;
    for turn in 1..=3 {
        g.run_schools();
        let want = (card + step * turn as f64).min(ceiling);
        assert!((g.education_level(sid) - want).abs() < 1e-9, "after {turn} turn(s) it should read {want}, not {}", g.education_level(sid));
    }
    for _ in 0..40 {
        g.run_schools();
    }
    assert!((g.education_level(sid) - ceiling).abs() < 1e-9, "it should climb to the ceiling {ceiling} and stop, not {}", g.education_level(sid));

    // Mothballed, it falls back at the same rate and stops at the card figure.
    let idx = g.state(sid).facilities.iter().position(|f| f.kind == FacilityKind::School).unwrap();
    g.state_mut(sid).facilities[idx].mothballed = true;
    g.state_mut(sid).facilities[idx].online = false;
    g.run_schools();
    assert!((g.education_level(sid) - (ceiling - step)).abs() < 1e-9, "it should fall a step to {}, not {}", ceiling - step, g.education_level(sid));
    for _ in 0..40 {
        g.run_schools();
    }
    assert!((g.education_level(sid) - card).abs() < 1e-9, "it should stop falling at the card figure {card}, not {}", g.education_level(sid));
}

/// Ticket #185: and the Schools run at Income, once a turn, without anybody calling them by hand.
#[test]
fn the_schools_run_at_income() {
    let mut g = game();
    let sid = g.directed_states(Seat(0))[0];
    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::School));
    let before = g.education_level(sid);
    g.income_phase();
    assert!(g.education_level(sid) > before, "an Income phase should move it: it read {before} and still reads {}", g.education_level(sid));
}

/// Ticket #185: one School to a Region, as the Constabulary and the Sea Wall are.
#[test]
fn a_region_holds_one_school() {
    let mut g = game();
    let sid = g.directed_states(Seat(0))[0];
    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::School));
    let o = Order::BuildFacility { state: sid, kind: FacilityKind::School };
    assert!(g.check_order(Seat(0), &[], &o).is_err(), "a second School in one Region should be refused");
}

/// Ticket #185: a Research Lab reads the LIVE figure, not the card. That is the whole point of the
/// building -- the Education Level multiplies what a Lab makes, and now it moves.
#[test]
fn a_research_lab_reads_the_education_level_the_school_has_raised() {
    let mut g = game();
    let sid = g.directed_states(Seat(0))[0];
    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::ResearchLab));
    let before = g.facility_yield(Seat(0), sid, FacilityKind::ResearchLab).research;
    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::School));
    for _ in 0..8 {
        g.run_schools();
    }
    let after = g.facility_yield(Seat(0), sid, FacilityKind::ResearchLab).research;
    assert!(after > before, "a Lab should make more once the School has run: {before} then {after}");
}

// ------------------------------------ 0.08.0 ticket #189: Colonists carry the schooling of their Region

/// Ticket #189: Emigrants take their Region's Education Level at the moment they MUSTER, and the
/// pile waiting on the card carries one mean and a count. A batch mustered after a School has run
/// knows more than one mustered before it, and the two average by head count.
#[test]
fn emigrants_take_their_regions_schooling_at_the_moment_they_muster() {
    let mut g = game();
    let sid = g.directed_states(Seat(0))[0];
    let card = g.tables.state(sid).education_level;
    g.state_mut(sid).population = 10_000.0;

    g.muster_emigrants(sid, 4);
    assert!((g.state(sid).emigrants_education - card).abs() < 1e-9, "the first batch knows the card figure {card}");

    // A School runs, then a second batch musters: it knows more, and the pile averages.
    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::School));
    for _ in 0..4 {
        g.run_schools();
    }
    let taught = g.education_level(sid);
    assert!(taught > card, "the premise: the School raised it from {card} to {taught}");
    g.muster_emigrants(sid, 4);
    let want = (card + taught) / 2.0;
    assert!((g.state(sid).emigrants_education - want).abs() < 1e-9, "four at {card} and four at {taught} should average {want}, not {}", g.state(sid).emigrants_education);
    assert_eq!(g.state(sid).emigrants, 8, "and there are eight of them");
}

/// Ticket #189: a Colony's Education Level is the weighted average of the people who settled it,
/// and arrivals average in by head count -- so a Colony founded out of a well-schooled Region and
/// then joined by people from a poorly-schooled one sits between the two.
#[test]
fn a_colony_reads_the_weighted_average_of_the_people_who_settled_it() {
    let mut g = game();
    let id = station_at(&mut g, Seat(0), BodyId::Earth);
    g.colony_mut(id).unwrap().modules.push(Module::new(ModuleKind::Core));
    for _ in 0..3 {
        g.colony_mut(id).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    }

    g.settle_people(id, 4, 1.5);
    assert!((g.colony(id).unwrap().education - 1.5).abs() < 1e-9, "the founders' figure is the Colony's");
    g.settle_people(id, 4, 0.7);
    let want = (1.5 * 4.0 + 0.7 * 4.0) / 8.0;
    assert!((g.colony(id).unwrap().education - want).abs() < 1e-9, "four at 1.5 and four at 0.7 should average {want}, not {}", g.colony(id).unwrap().education);
    assert_eq!(g.colony(id).unwrap().colonists, 8);
}

/// Ticket #189: deaths do not move a mean. The dead are drawn evenly from the people there, so
/// losing half a Colony leaves the survivors knowing exactly what everyone knew.
#[test]
fn deaths_do_not_move_what_the_survivors_know() {
    let mut g = game();
    let id = station_at(&mut g, Seat(0), BodyId::Earth);
    g.colony_mut(id).unwrap().modules.push(Module::new(ModuleKind::Core));
    for _ in 0..3 {
        g.colony_mut(id).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    }
    g.settle_people(id, 4, 1.5);
    g.settle_people(id, 4, 0.7);
    let before = g.colony(id).unwrap().education;
    g.take_colonists(id, 4);
    assert_eq!(g.colony(id).unwrap().colonists, 4, "four are gone");
    assert!((g.colony(id).unwrap().education - before).abs() < 1e-9, "the mean should not move: {before} then {}", g.colony(id).unwrap().education);
}

/// Ticket #189: and it survives the whole journey -- mustered in a Region, lifted to a station,
/// through the order layer rather than through the helpers by hand.
#[test]
fn schooling_travels_from_the_region_to_the_station_it_is_lifted_to() {
    let mut g = game();
    let sid = g.directed_states(Seat(0))[0];
    let card = g.tables.state(sid).education_level;
    g.state_mut(sid).population = 10_000.0;
    let station = g.colonies.iter().find(|c| c.in_orbit && c.control.director() == Some(Seat(0))).map(|c| c.id).expect("seat 0 starts with a station");
    g.state_mut(sid).facilities.push(Facility { online: true, ..Facility::new(FacilityKind::LaunchSite) });

    g.commit_orders(Seat(0), &[Order::BuildEmigrants { state: sid, n: 4 }]);
    assert_eq!(g.state(sid).emigrants, 4, "four are waiting");
    g.commit_orders(Seat(0), &[Order::LiftToStation { state: sid, n: 4, colony: station }]);

    assert_eq!(g.colony(station).unwrap().colonists, 4, "four reached the station");
    assert!(
        (g.colony(station).unwrap().education - card).abs() < 1e-9,
        "the station should know what {} knows ({card}), not {}",
        g.tables.state(sid).name,
        g.colony(station).unwrap().education
    );
}

// ------------------------------------------------- 0.08.0 ticket #185: the Institute, off Earth

/// Ticket #185: the Institute is the School's form off Earth, on the same step and ceiling. What it
/// decays BACK to is the difference that matters: not zero, and not a card figure a Colony does not
/// have, but the settlers' own average -- what its people know without a school. A Colony whose
/// Institute goes dark has not become less educated; its school shut.
#[test]
fn an_institute_raises_a_colony_and_decays_back_to_what_its_settlers_knew() {
    let mut g = game();
    let id = station_at(&mut g, Seat(0), BodyId::Earth);
    g.colony_mut(id).unwrap().modules.push(Module::new(ModuleKind::Core));
    g.settle_people(id, 4, 0.8);
    let settlers = g.colony(id).unwrap().settler_education;
    assert!((settlers - 0.8).abs() < 1e-9, "the settlers brought 0.8");

    g.colony_mut(id).unwrap().modules.push(Module::new(ModuleKind::Institute));
    let step = g.tables.school.per_turn;
    let ceiling = g.tables.school.ceiling;
    g.run_schools();
    assert!((g.colony(id).unwrap().education - (0.8 + step)).abs() < 1e-9, "one turn should add a step");
    for _ in 0..40 {
        g.run_schools();
    }
    assert!((g.colony(id).unwrap().education - ceiling).abs() < 1e-9, "it climbs to the ceiling {ceiling}, not {}", g.colony(id).unwrap().education);

    // Mothballed, it falls back -- and stops at what the settlers knew.
    let idx = g.colony(id).unwrap().modules.iter().position(|m| m.kind == ModuleKind::Institute).unwrap();
    g.colony_mut(id).unwrap().modules[idx].mothballed = true;
    g.colony_mut(id).unwrap().modules[idx].online = false;
    for _ in 0..60 {
        g.run_schools();
    }
    assert!(
        (g.colony(id).unwrap().education - settlers).abs() < 1e-9,
        "it should stop at the settlers' own {settlers}, not {}",
        g.colony(id).unwrap().education
    );
}

/// Ticket #185: one Institute to a Colony, as one School to a Nation State.
#[test]
fn a_colony_holds_one_institute() {
    let mut g = game();
    let id = station_at(&mut g, Seat(0), BodyId::Earth);
    g.colony_mut(id).unwrap().modules.push(Module::new(ModuleKind::Core));
    g.settle_people(id, 8, 1.0);
    g.seats[0].stockpile.materials = 500;
    let o = Order::BuildModule { colony: id, kind: ModuleKind::Institute };
    assert!(g.check_order(Seat(0), &[], &o).is_ok(), "the first Institute is legal");
    g.colony_mut(id).unwrap().modules.push(Module::new(ModuleKind::Institute));
    assert!(g.check_order(Seat(0), &[], &o).is_err(), "a second Institute at one Colony should be refused");
}

/// Ticket #189: an Observatory reads its Colony's Education Level, exactly as a Research Lab reads
/// its Region's. Without this the Colony's figure would be bookkeeping.
#[test]
fn an_observatory_reads_its_colonys_education_level() {
    let mut g = game();
    let id = station_at(&mut g, Seat(0), BodyId::Earth);
    g.colony_mut(id).unwrap().modules.push(Module::new(ModuleKind::Core));
    g.colony_mut(id).unwrap().modules.push(Module::new(ModuleKind::Observatory));
    g.settle_people(id, 4, 0.5);
    let idx = g.colony(id).unwrap().modules.iter().position(|m| m.kind == ModuleKind::Observatory).unwrap();
    let poor = g.module_yield_at(Seat(0), id, idx).research;

    let mut h = game();
    let id2 = station_at(&mut h, Seat(0), BodyId::Earth);
    h.colony_mut(id2).unwrap().modules.push(Module::new(ModuleKind::Core));
    h.colony_mut(id2).unwrap().modules.push(Module::new(ModuleKind::Observatory));
    h.settle_people(id2, 4, 2.0);
    let rich = h.module_yield_at(Seat(0), id2, idx).research;

    assert!(rich > poor, "a well-schooled Colony's Observatory should out-produce a poorly-schooled one: {poor} then {rich}");
}

// ------------------------------------------------------- 0.08.0 ticket #187: Resistance

/// Ticket #187: a place's schooling bends what an outsider's Influence buys there. The pivot has no
/// effect; the worst-schooled Region on the board gives back MORE than was spent, the ceiling gives
/// back less, and each side reaches its own end of the band.
#[test]
fn resistance_runs_from_the_boards_worst_schooling_to_the_schools_ceiling() {
    let g = game();
    let r = &g.tables.influence.resistance;
    let sid = g.directed_states(Seat(0))[0];
    let place = Place::State(sid);

    // The anchors, read off a Region whose figure is moved to each in turn.
    let mut h = game();
    let at = |h: &mut Game, e: f64| {
        let card = h.tables.state(sid).education_level;
        h.state_mut(sid).schooling = e - card;
        h.resistance(place)
    };
    assert!((at(&mut h, r.pivot) - 1.0).abs() < 1e-9, "the pivot has no effect");
    assert!((at(&mut h, r.low) - (1.0 - r.band)).abs() < 1e-9, "the lowest card reaches 1 - band");
    assert!((at(&mut h, r.high) - (1.0 + r.band)).abs() < 1e-9, "the ceiling reaches 1 + band");
    // And it is monotonic between them, not a step.
    let mid_low = at(&mut h, (r.low + r.pivot) / 2.0);
    assert!(mid_low > 1.0 - r.band && mid_low < 1.0, "halfway down sits inside the band, not at an end: {mid_low}");
}

/// Ticket #187: the designer's worked example -- 30 Influence spent on a Region schooled to the
/// ceiling moves the needle by 27 -- and the other end, where a badly-schooled place gives back more.
#[test]
fn thirty_influence_buys_twenty_seven_standing_at_the_ceiling() {
    let mut g = game();
    let sid = g.directed_states(Seat(0))[0];
    let place = Place::State(sid);
    let card = g.tables.state(sid).education_level;

    g.state_mut(sid).schooling = g.tables.influence.resistance.high - card;
    assert_eq!(g.standing_from(place, 30), 27, "30 spent at the ceiling should become 27");

    g.state_mut(sid).schooling = g.tables.influence.resistance.low - card;
    assert_eq!(g.standing_from(place, 30), 33, "30 spent at the worst schooling should become 33");
}

/// Ticket #187: it bites an OUTSIDER and never the controller, through the Resolution rather than
/// through the helper by hand. Reinforcing a place you hold is never taxed.
#[test]
fn resistance_bites_an_outsider_and_never_the_controller() {
    let mut g = game();
    let sid = g.directed_states(Seat(0))[0];
    let place = Place::State(sid);
    let card = g.tables.state(sid).education_level;
    g.state_mut(sid).schooling = g.tables.influence.resistance.high - card;
    assert_eq!(g.place_control(place).controller(), Some(Seat(0)), "the premise: seat 0 holds it");

    // Seat 1 is an outsider here, and pays the tax.
    g.seats[1].allotment = 100;
    g.commit_orders(Seat(1), &[Order::Influence { target: place, amount: 30 }]);
    g.resolution_phase();
    assert_eq!(g.seat(Seat(1)).influence.get(&place).copied().unwrap_or(0), 27, "an outsider's 30 becomes 27");

    // Seat 0 holds it, and converts in full.
    let before = g.seat(Seat(0)).influence.get(&place).copied().unwrap_or(0);
    g.seats[0].allotment = 100;
    g.commit_orders(Seat(0), &[Order::Influence { target: place, amount: 30 }]);
    g.resolution_phase();
    let after = g.seat(Seat(0)).influence.get(&place).copied().unwrap_or(0);
    assert_eq!(after - before, 30, "the controller's own 30 is worth 30");
}

// ---------------------------------- 0.08.0 ticket #188: schooling moderates what a population is worth

/// Ticket #188: the population BONUS -- the part above 1 -- is scaled by the state's schooling, so a
/// great many badly-schooled people are worth less than a great many well-schooled ones. The base 1
/// stays, so no Region is ever worth less than one with no people at all.
#[test]
fn schooling_moderates_the_population_bonus() {
    let mut g = game();
    let sid = g.directed_states(Seat(0))[0];
    let card = g.tables.state(sid).education_level;
    let pop = g.state(sid).population;
    let bonus = pop / 1000.0;

    assert!((g.population_factor(sid) - (1.0 + bonus * card)).abs() < 1e-9, "the card figure scales the bonus");

    for taught in [0.7, 1.0, 2.0] {
        g.state_mut(sid).schooling = taught - card;
        let want = 1.0 + bonus * taught;
        assert!((g.population_factor(sid) - want).abs() < 1e-9, "at schooling {taught} the factor should be {want}, not {}", g.population_factor(sid));
    }

    // Uncapped in both directions: schooling below the pivot shrinks the bonus, above it enlarges.
    g.state_mut(sid).schooling = 0.5 - card;
    let low = g.population_factor(sid);
    g.state_mut(sid).schooling = 2.0 - card;
    let high = g.population_factor(sid);
    assert!(low < 1.0 + bonus && high > 1.0 + bonus, "it moves both ways around the old factor: {low} then {high}");
    assert!(low > 1.0, "and never below 1: {low}");
}

/// Ticket #188: and schooling therefore applies TWICE to a Lab -- once inside the population factor
/// and once as the outright multiplier it has always been. The compounding is the point: it is what
/// makes a School in a big, badly-schooled Region transformative rather than marginal.
#[test]
fn schooling_applies_twice_to_a_research_lab() {
    let mut g = game();
    let sid = g.directed_states(Seat(0))[0];
    let card = g.tables.state(sid).education_level;
    let pop = g.state(sid).population;
    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::ResearchLab));

    let taught = 2.0;
    g.state_mut(sid).schooling = taught - card;
    let base = g.tables.facility(FacilityKind::ResearchLab).produces.as_ref().unwrap().amount as f64;
    let mult = g.tables.faction(g.kind(Seat(0))).research_multiplier;
    let want = (base * (1.0 + pop / 1000.0 * taught) * taught * mult).floor() as i64;
    assert_eq!(g.facility_yield(Seat(0), sid, FacilityKind::ResearchLab).research, want, "the factor and the multiplier both carry the schooling");
}

/// Ticket #188: the Observatory's per-Colonist bonus is moderated the same way, so the rule reads
/// the same in both halves of the game.
#[test]
fn schooling_moderates_the_observatorys_per_colonist_bonus() {
    let mut g = game();
    let id = station_at(&mut g, Seat(0), BodyId::Earth);
    g.colony_mut(id).unwrap().modules.push(Module::new(ModuleKind::Core));
    for _ in 0..6 {
        g.colony_mut(id).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    }
    g.colony_mut(id).unwrap().modules.push(Module::new(ModuleKind::Observatory));
    g.settle_people(id, 40, 1.0);
    let idx = g.colony(id).unwrap().modules.iter().position(|m| m.kind == ModuleKind::Observatory).unwrap();

    let per = g.tables.observatory.research_per_colonist;
    let n = g.colony(id).unwrap().colonists as f64;
    let science = g.research_yield_at(g.colony(id).unwrap());
    let base = g.tables.module(ModuleKind::Observatory).produces.as_ref().unwrap().amount as f64;
    let mult = g.tables.faction(g.kind(Seat(0))).research_multiplier;
    for taught in [0.5, 2.0] {
        g.colony_mut(id).unwrap().education = taught;
        let want = (base * science * taught * (1.0 + n * per * taught) * mult).floor() as i64;
        assert_eq!(g.module_yield_at(Seat(0), id, idx).research, want, "at {taught} the per-Colonist bonus should be moderated too");
    }
}

// ------------------------------------------- 0.08.0 tickets #181 to #186: the Unique Facilities

fn seat_of(g: &Game, kind: FactionKind) -> Seat {
    Seat::ALL.into_iter().find(|s| g.kind(*s) == kind).expect("every game seats all four Factions")
}

/// Ticket #181: a Unique Facility is a building of its own and it REPLACES the common one on its
/// Faction's build list. A Prospector who orders a Bank raises an Investment Bank; every other
/// Faction raises a Bank; and the name is not a back door -- nobody else may order one, a Faction
/// that has captured a Region full of them included, because a captured building is never a licence
/// to build more.
#[test]
fn a_faction_builds_its_own_version_of_the_common_building() {
    let mut g = game();
    let pro = seat_of(&g, FactionKind::Prospectors);
    let cus = seat_of(&g, FactionKind::Custodians);
    let ps = g.controlled_states(pro)[0];
    let cs = g.controlled_states(cus)[0];
    g.seats[pro.index()].stockpile.materials = 500;
    g.seats[cus.index()].stockpile.materials = 500;

    g.commit_orders(pro, &[Order::BuildFacility { state: ps, kind: FacilityKind::Bank }]);
    g.commit_orders(cus, &[Order::BuildFacility { state: cs, kind: FacilityKind::Bank }]);
    let queued = |g: &Game, sid: StateId| -> Vec<BuildItem> { g.state(sid).queue.iter().map(|b| b.item).collect() };
    assert_eq!(queued(&g, ps), vec![BuildItem::Facility(FacilityKind::InvestmentBank)], "a Prospector never raises a plain Bank");
    assert_eq!(queued(&g, cs), vec![BuildItem::Facility(FacilityKind::Bank)], "and everybody else raises one");

    let err = g.check_order(cus, &[], &Order::BuildFacility { state: cs, kind: FacilityKind::InvestmentBank }).unwrap_err();
    assert_eq!(err.0, "only the Prospectors build the Investment Bank");
    assert!(g.check_order(pro, &[], &Order::BuildFacility { state: ps, kind: FacilityKind::InvestmentBank }).is_ok(), "by its own name too");

    // The cap reads the JOB, not the name: an Academy standing blocks a School order and back.
    let cus_state = g.controlled_states(cus)[0];
    g.state_mut(cus_state).queue.clear();
    g.state_mut(cus_state).facilities.push(Facility::new(FacilityKind::Academy));
    let err = g.check_order(cus, &[], &Order::BuildFacility { state: cus_state, kind: FacilityKind::School }).unwrap_err();
    assert_eq!(err.0, "this Nation State already has a School");
}

/// Ticket #181: the price is the common building's, exactly -- the same Materials, the same build
/// turns, the same upkeep, the same output, the same slot. A Unique Facility costs nothing extra,
/// and quietly paying less would be a cost by another name.
#[test]
fn a_unique_facility_costs_and_makes_what_the_common_one_does() {
    let g = game();
    for (unique, common) in [
        (FacilityKind::InvestmentBank, FacilityKind::Bank),
        (FacilityKind::Spaceport, FacilityKind::LaunchSite),
        (FacilityKind::Reactor, FacilityKind::PowerPlant),
        (FacilityKind::Academy, FacilityKind::School),
    ] {
        assert_eq!(unique.common(), Some(common), "{} replaces {}", unique.name(), common.name());
        let u = g.tables.facility(unique);
        let c = g.tables.facility(common);
        assert_eq!((u.materials, u.build_turns, u.energy_upkeep, u.no_slot), (c.materials, c.build_turns, c.energy_upkeep, c.no_slot), "{}", unique.name());
        assert!((u.emissions - c.emissions).abs() < 1e-9, "{}", unique.name());
        assert_eq!(u.produces.as_ref().map(|p| (p.resource, p.amount)), c.produces.as_ref().map(|p| (p.resource, p.amount)), "{}", unique.name());
    }
    // And off Earth, the Academy against the Institute.
    let a = g.tables.module(ModuleKind::Academy);
    let i = g.tables.module(ModuleKind::Institute);
    assert_eq!((a.materials, a.build_turns, a.energy_upkeep), (i.materials, i.build_turns, i.energy_upkeep));
    assert_eq!(ModuleKind::Academy.common(), Some(ModuleKind::Institute));
    // Ticket #239 (version 0.08.3): and the three that complete the set, each against its sibling.
    // The whole point of a Unique is that it is the common building at the common price with one
    // clause, so a row that drifts is the defect this catches.
    for (unique, common) in [
        (ModuleKind::Heliostat, ModuleKind::SolarArray),
        (ModuleKind::Exchange, ModuleKind::TradePost),
        (ModuleKind::Chorus, ModuleKind::Relay),
    ] {
        let u = g.tables.module(unique);
        let c = g.tables.module(common);
        assert_eq!(unique.common(), Some(common), "{}", unique.name());
        assert_eq!((u.materials, u.build_turns, u.energy_upkeep), (c.materials, c.build_turns, c.energy_upkeep), "{} is priced as its sibling", unique.name());
        assert_eq!(u.station_only, c.station_only, "{}", unique.name());
        assert_eq!(u.sun_scaled, c.sun_scaled, "{}", unique.name());
        assert_eq!(u.influence_allotment, c.influence_allotment, "{}", unique.name());
        assert_eq!(u.standing_per_turn, c.standing_per_turn, "{}", unique.name());
        assert_eq!(u.produces.as_ref().map(|p| (p.resource, p.amount)), c.produces.as_ref().map(|p| (p.resource, p.amount)), "{}", unique.name());
    }
    // Every Faction now has one Unique Module as well as one Unique Facility, which is what this
    // ticket was for; the Custodians' Academy is deliberately both, and wears one name.
    for faction in FactionKind::ALL {
        assert!(ModuleKind::ALL.into_iter().any(|k| k.unique_to() == Some(faction)), "{faction:?} has no Unique Module");
        assert!(FacilityKind::ALL.into_iter().any(|k| k.unique_to() == Some(faction)), "{faction:?} has no Unique Facility");
    }
}

/// Ticket #239 (version 0.08.3): the three new Unique Modules' clauses, in EXACT figures.
///
/// Written first against `g.tables.unique.*` and witnessed to prove nothing: with all three
/// figures zeroed in the data it still passed, because both sides of the assertion read the same
/// table. The numbers below are therefore literals, and a clause that stops paying fails here.
#[test]
fn the_three_unique_modules_each_pay_their_one_clause() {
    let mut g = game();
    let arch = seat_of(&g, FactionKind::Archivists);
    let pros = seat_of(&g, FactionKind::Prospectors);
    let ark = seat_of(&g, FactionKind::Arkwrights);

    // The Heliostat: one more Energy than a Solar Array, AFTER the inverse square scaling. Mars
    // is the case that discriminates -- its sun factor is 0.43, so a Solar Array's 6 rounds to 3,
    // and the extra point added BEFORE the scaling would give (6 + 1) x 0.43 = 3 as well. Only
    // adding it after yields 4. The Moon, at full sunlight, cannot tell the two apart.
    for (body, array, heliostat) in [(BodyId::Mars, 3, 4), (BodyId::Moon, 6, 7)] {
        let cid = colony(&mut g, arch, body, &[ModuleKind::SolarArray], 0);
        assert_eq!(g.module_yield(arch, cid, ModuleKind::SolarArray).amount, array, "a Solar Array at {body:?}");
        assert_eq!(g.module_yield(arch, cid, ModuleKind::Heliostat).amount, heliostat, "a Heliostat at {body:?} is a Solar Array and one more, after the scaling");
    }

    // The Exchange: one more Ducat than a Trade Post at the same Colony, flat and AFTER the
    // Prospectors' x1.25 -- which is the whole reason it is flat, since 1 through x1.25 floors
    // back to 1 and a captured Exchange pays its captor what it paid its builder.
    let cid = colony(&mut g, pros, BodyId::Mars, &[ModuleKind::TradePost], 6);
    assert_eq!(g.module_yield(pros, cid, ModuleKind::TradePost).amount, 18, "a Trade Post at a Colony of 6");
    assert_eq!(g.module_yield(pros, cid, ModuleKind::Exchange).amount, 19, "an Exchange is a Trade Post and one more Ducat");

    // The Chorus: one more Influence in the Allotment for every 6 Colonists at its OWN Colony,
    // rounded down. Three populations across the boundary pin the rounding: one short pays
    // nothing, the step itself pays one, and a figure well past two steps pays two.
    let cid = colony(&mut g, ark, BodyId::Moon, &[ModuleKind::Relay], 0);
    for (people, relay, chorus) in [(5u32, 1, 1), (6, 1, 2), (13, 1, 3)] {
        g.colony_mut(cid).expect("the Colony just made").colonists = people;
        assert_eq!(g.module_yield(ark, cid, ModuleKind::Relay).allotment, relay, "a plain Relay is unmoved by {people} Colonists");
        assert_eq!(g.module_yield(ark, cid, ModuleKind::Chorus).allotment, chorus, "a Chorus at a Colony of {people}");
        // Standing is untouched: "+1 Influence" has meant the Allotment since ticket #232, which
        // is the Faction's budget everywhere rather than a hold on one place.
        assert_eq!(g.module_yield(ark, cid, ModuleKind::Chorus).standing, 2, "a Chorus holds its place no harder than a Relay");
    }
}

/// Ticket #239 (version 0.08.3): a Unique Module stands wherever its common sibling stands.
///
/// This is the guard for a bug a PICTURE found and no test did. The rule for what may stand on a
/// Space Station was written out twice, in the interface's build list and the computer's, as a
/// list of KINDS -- and a list of kinds cannot know about a Unique. With three new Uniques the
/// Prospectors lost the Trade Post row from every station without gaining the Exchange, and the
/// Archivists lost the Solar Array without gaining the Heliostat.
#[test]
fn a_unique_module_stands_where_its_sibling_stands() {
    for unique in ModuleKind::ALL {
        let Some(common) = unique.common() else { continue };
        assert_eq!(
            unique.stands_on_a_station(),
            common.stands_on_a_station(),
            "{} must stand exactly where a {} stands",
            unique.name(),
            common.name()
        );
    }
    // And the four that a station takes, named, so the set itself cannot drift unnoticed.
    assert!(ModuleKind::Exchange.stands_on_a_station(), "a station takes a Trade Post, so it takes an Exchange");
    assert!(ModuleKind::Heliostat.stands_on_a_station(), "a Solar Array stands nowhere else, so a Heliostat must");
    assert!(!ModuleKind::Chorus.stands_on_a_station(), "a Relay is a ground Module, so a Chorus is too");
    assert!(!ModuleKind::Mine.stands_on_a_station(), "nobody digs in orbit");
}

/// Ticket #239 (version 0.08.3): a Unique Module does its COMMON sibling's job, so everything
/// keyed by kind reaches it. The Chorus is the case that would have broken silently: Relay
/// Networks takes a Relay's Allotment from 1 to 2, and written against the wrong kind the
/// Arkwrights' own Relay would have been the one Relay in the game the Tech never reached.
#[test]
fn relay_networks_reaches_the_arkwrights_chorus() {
    let mut g = game();
    let ark = seat_of(&g, FactionKind::Arkwrights);
    let cid = colony(&mut g, ark, BodyId::Moon, &[ModuleKind::Relay], 0);
    let before = g.module_yield(ark, cid, ModuleKind::Chorus).allotment;
    with_tech(&mut g, TechId::RelayNetworks);
    let after = g.module_yield(ark, cid, ModuleKind::Chorus).allotment;
    assert!(after > before, "Relay Networks must reach a Chorus as it reaches a Relay: {before} -> {after}");
    assert_eq!(after, g.module_yield(ark, cid, ModuleKind::Relay).allotment, "and reach it by exactly as much");
}

/// Ticket #181: a Faction's start Region's Facilities come up as that Faction's own versions, the
/// Launch Site every start Region is handed included. The consequence is asymmetric and was accepted
/// knowingly: the Arkwrights hold their Spaceport from turn 1, the Archivists usually hold a Reactor
/// because ten of the fourteen Regions start with a Power Plant, and neither the Bank nor the School
/// is in any Region's start Facilities, so the Prospectors and the Custodians must build for theirs.
#[test]
fn a_start_region_comes_up_with_its_factions_own_versions() {
    let g = fresh();
    let ark = seat_of(&g, FactionKind::Arkwrights);
    let sid = g.controlled_states(ark)[0];
    assert!(g.state(sid).facilities.iter().any(|f| f.kind == FacilityKind::Spaceport), "the Arkwrights launch from turn 1");
    assert!(!g.state(sid).facilities.iter().any(|f| f.kind == FacilityKind::LaunchSite), "and never from a common Launch Site");

    // Nobody starts with another Faction's version, and no start Region carries a Bank or a School,
    // so the Prospectors and the Custodians begin with nothing of theirs.
    for seat in Seat::ALL {
        let faction = g.kind(seat);
        for sid in g.controlled_states(seat) {
            for f in &g.state(sid).facilities {
                assert!(f.kind.unique_to().map(|o| o == faction).unwrap_or(true), "{:?} holds another Faction's {}", seat, f.kind.name());
            }
        }
    }
    for seat in [seat_of(&g, FactionKind::Prospectors), seat_of(&g, FactionKind::Custodians)] {
        let holds = g.controlled_states(seat).iter().any(|s| g.state(*s).facilities.iter().any(|f| f.kind.unique_to().is_some()));
        assert!(!holds, "{seat:?} should start with none of its own");
    }
}

/// Ticket #181: a Unique Facility is never destroyed when its place changes hands. It keeps standing
/// and pays its new holder the same clause it paid its builder -- the opposite of the Scrubber and
/// the Archive, which are destroyed outright. And the mirror does NOT hold: a common building
/// already standing does not convert when the Faction whose Unique version it is takes the Region.
/// The bricks are the bricks, and the rule cuts both ways or it is not a rule.
#[test]
fn a_unique_facility_changes_sides_and_a_common_building_never_converts() {
    let mut g = game();
    let pro = seat_of(&g, FactionKind::Prospectors);
    let cus = seat_of(&g, FactionKind::Custodians);
    let sid = g.controlled_states(pro)[0];
    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::InvestmentBank));

    g.take_control(sid, cus);
    assert!(g.state(sid).facilities.iter().any(|f| f.kind == FacilityKind::InvestmentBank), "it stands, and in Custodian hands");

    // The other way: three plain Banks taken by the Prospectors stay three plain Banks.
    let other = g.controlled_states(cus)[0];
    for _ in 0..3 {
        g.state_mut(other).facilities.push(Facility::new(FacilityKind::Bank));
    }
    g.take_control(other, pro);
    assert_eq!(g.state(other).facilities.iter().filter(|f| f.kind == FacilityKind::Bank).count(), 3, "capture is not conversion");
    assert!(!g.state(other).facilities.iter().any(|f| f.kind == FacilityKind::InvestmentBank));
}

/// Ticket #182: the Investment Bank banks 1% of the Venture Capital Fund's balance back INTO the
/// Fund, read before this turn's banking is added, rounded down, and the Materials are created
/// rather than drawn from the Stockpile, so they compound from next turn. ONE per Region pays,
/// however many stand there, and a floor of one applies to a single building Faction-wide.
#[test]
fn an_investment_bank_banks_a_hundredth_of_the_fund_one_per_region() {
    let mut g = game();
    let pro = seat_of(&g, FactionKind::Prospectors);
    g.seats[pro.index()].stockpile.energy = 500;
    let a = g.controlled_states(pro)[0];
    let b = StateId::ALL.into_iter().find(|s| *s != a).unwrap();
    g.take_control(b, pro);
    // Two in one Region and one in the other: three buildings, two paying Regions.
    g.state_mut(a).facilities.push(Facility::new(FacilityKind::InvestmentBank));
    g.state_mut(a).facilities.push(Facility::new(FacilityKind::InvestmentBank));
    g.state_mut(b).facilities.push(Facility::new(FacilityKind::InvestmentBank));

    g.seats[pro.index()].venture_fund = 500;
    let before = g.seats[pro.index()].stockpile.materials;
    g.income_phase();
    assert_eq!(g.seats[pro.index()].venture_fund, 500 + 10, "1% of 500 is 5, once per Region, twice");
    assert_eq!(g.seats[pro.index()].stockpile.materials - before, g.seats[pro.index()].income_last_turn.materials, "the interest is created, never taken from the Stockpile");

    // And it compounds: next turn's interest reads the larger balance.
    g.income_phase();
    assert_eq!(g.seats[pro.index()].venture_fund, 510 + 10, "1% of 510 is 5 again, and the balance keeps climbing");

    // The floor: below 100 in the Fund the percentage floors to nothing, and one building still pays.
    g.seats[pro.index()].venture_fund = 50;
    g.income_phase();
    assert_eq!(g.seats[pro.index()].venture_fund, 51, "one Material, Faction-wide, not one per Region");
}

/// Ticket #182: in a non-Prospector's hands the same share applies to that Faction's DUCAT income
/// instead, at a minimum of one Ducat, because no other Faction has a Fund for it to pay into. The
/// cap travels with the clause, so capturing a Prospector Region is never better than being the
/// Prospectors there.
#[test]
fn a_captured_investment_bank_pays_its_captor_ducats() {
    let mut g = game();
    let cus = seat_of(&g, FactionKind::Custodians);
    g.seats[cus.index()].stockpile.energy = 500;
    let sid = g.controlled_states(cus)[0];
    g.income_phase();
    let plain = g.seats[cus.index()].income_last_turn.ducats;

    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::InvestmentBank));
    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::InvestmentBank));
    let fund_before = g.seats[cus.index()].venture_fund;
    g.income_phase();
    let with = g.seats[cus.index()].income_last_turn.ducats;
    let bank_ducats = g.facility_yield(cus, sid, FacilityKind::InvestmentBank).amount * 2;
    assert_eq!(with - plain - bank_ducats, 1, "one Region pays once, at the floor of one Ducat");
    assert_eq!(g.seats[cus.index()].venture_fund, fund_before, "and nothing reaches a Fund they do not have");
}

/// Ticket #183: the Spaceport pays +1 Influence for every Emigrant it lifts OFF EARTH -- onto a Ship
/// in orbit, or onto a station of the seat's over Earth. It lands as free Allotment paid into NEXT
/// turn's, at face value OUTSIDE the Allotment formula, so the Arkwrights' x0.8 Influence multiplier
/// never touches it. The sea to Antarctica pays nothing: it is not a launch, and Antarctica is on
/// Earth.
#[test]
fn a_spaceport_pays_an_influence_for_every_emigrant_it_launches() {
    let mut g = game();
    let ark = seat_of(&g, FactionKind::Arkwrights);
    let sid = g.controlled_states(ark)[0];
    assert!(g.state(sid).facilities.iter().any(|f| f.kind == FacilityKind::Spaceport), "the premise: their start Region carries one");
    let st = station_at(&mut g, ark, BodyId::Earth);
    g.colony_mut(st).unwrap().modules.push(Module::new(ModuleKind::Core));

    let base = g.influence_allotment(ark);
    g.state_mut(sid).emigrants = 3;
    g.commit_orders(ark, &[Order::LiftToStation { state: sid, n: 3, colony: st }]);
    assert_eq!(g.seats[ark.index()].spaceport_influence, 3, "one for each Pioneer lifted");
    assert_eq!(g.influence_allotment(ark), base + 3, "at face value, never through the x0.8");

    // The sea is not a launch.
    g.state_mut(sid).emigrants = 4;
    g.commit_orders(ark, &[Order::SendToAntarctica { state: sid, n: 4, into: UnloadTarget::Slot(BodyId::Earth, 0) }]);
    assert_eq!(g.seats[ark.index()].spaceport_influence, 3, "Antarctica is on Earth");

    // A common Launch Site earns nothing, and the tally is cleared once it has been paid.
    g.state_mut(sid).facilities.retain(|f| f.kind != FacilityKind::Spaceport);
    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::LaunchSite));
    g.income_phase();
    assert_eq!(g.seats[ark.index()].spaceport_influence, 0, "paid, and the tally begins again");
    g.state_mut(sid).emigrants = 2;
    g.commit_orders(ark, &[Order::LiftToStation { state: sid, n: 2, colony: st }]);
    assert_eq!(g.seats[ark.index()].spaceport_influence, 0, "a Launch Site is not a Spaceport");
}

/// Ticket #184: while a Reactor stands and is online, its holder pays 75% of the total Energy upkeep
/// of everything it owns -- off the TOTAL and floored once, never per building, because at 2 and 3
/// Energy a building per-building rounding turns a 75% rule into a 50% and a 33% cut. The Archive is
/// the one exception and pays its figure in full. No stacking: a second Reactor is an ordinary
/// power station.
#[test]
fn a_reactor_takes_a_quarter_off_the_whole_energy_bill_but_never_off_the_archive() {
    let mut g = game();
    let arc = seat_of(&g, FactionKind::Archivists);
    let sid = g.controlled_states(arc)[0];
    g.state_mut(sid).facilities.clear();
    for _ in 0..2 {
        g.state_mut(sid).facilities.push(Facility::new(FacilityKind::Factory));
    }
    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::ResearchLab));
    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::PowerPlant));
    g.seats[arc.index()].stockpile.energy = 500;

    // A Power Plant and a Reactor make the same Energy, so swapping one for the other isolates the
    // clause exactly: the whole difference is the relief.
    g.income_phase();
    let plain = g.seats[arc.index()].income_last_turn.energy;
    let bill: i64 = g.state(sid).facilities.iter().map(|f| g.tables.facility(f.kind).energy_upkeep).sum::<i64>() + g.unit_upkeep(arc);
    let want = bill - (bill as f64 * 0.75).floor() as i64;
    assert!(want > 0, "the premise: there is a bill to take a quarter off, {bill}");

    let i = g.state(sid).facilities.iter().position(|f| f.kind == FacilityKind::PowerPlant).unwrap();
    g.state_mut(sid).facilities[i].kind = FacilityKind::Reactor;
    g.income_phase();
    assert_eq!(g.seats[arc.index()].income_last_turn.energy - plain, want, "off the total, floored once");

    // No stacking: a second Reactor is worth its 6 Energy and nothing more.
    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::Reactor));
    g.income_phase();
    let two = g.seats[arc.index()].income_last_turn.energy;
    g.state_mut(sid).facilities.pop();
    g.income_phase();
    let one = g.seats[arc.index()].income_last_turn.energy;
    // What a Reactor actually makes here, not the row figure: this Region leans Energy, so its
    // power stations pay half again.
    let makes = g.facility_yield(arc, sid, FacilityKind::Reactor).amount;
    assert_eq!(two - one, makes, "the second Reactor pays its Energy and no second discount");

    // And the Archive pays in full: it is left out of the sum the relief is taken from, so adding it
    // costs its whole figure and not three quarters of it. (Ticket #51: it draws no Energy until its
    // Research is paid in full, so the fund is filled first or there is nothing to measure.)
    let cid = colony(&mut g, arc, BodyId::Mars, &[], 4);
    g.income_phase();
    let without = g.seats[arc.index()].income_last_turn.energy;
    g.colony_mut(cid).unwrap().modules.push(Module::new(ModuleKind::Archive));
    g.seats[arc.index()].archive_fund = g.tables.archive.research;
    let archive_upkeep = g.tables.module(ModuleKind::Archive).energy_upkeep;
    assert!(archive_upkeep > 0, "the premise: the Archive has a bill of its own");
    g.income_phase();
    assert_eq!(without - g.seats[arc.index()].income_last_turn.energy, archive_upkeep, "the Archive's upkeep is paid whole");
}

/// Ticket #186: the Academy does everything a School does and pays +1 Ducat a turn, flat, wherever
/// it stands -- Region, Colony or station -- while it is online. Flat rather than scaled by GDP,
/// which would make it a second Bank built where the money already is rather than where schooling is
/// wanted; and flat is why a captured Academy pays its captor exactly what it paid its builder,
/// since 1 through the largest output multiplier in the game floors back to 1.
#[test]
fn an_academy_pays_a_ducat_wherever_it_stands_and_teaches_like_a_school() {
    let mut g = game();
    let cus = seat_of(&g, FactionKind::Custodians);
    g.seats[cus.index()].stockpile.energy = 500;
    let sid = g.controlled_states(cus)[0];
    g.income_phase();
    let plain = g.seats[cus.index()].income_last_turn.ducats;

    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::Academy));
    let cid = colony(&mut g, cus, BodyId::Mars, &[ModuleKind::Academy], 4);
    g.income_phase();
    let with = g.seats[cus.index()].income_last_turn.ducats;
    // The figure is pinned as a literal rather than read back from the same table the rule reads:
    // an oracle that shares its input with the code under test agrees with it while both diverge.
    assert_eq!(g.tables.unique.academy_ducats, 1, "the card figure: a flat Ducat");
    assert_eq!(with - plain, 2, "one on Earth and one off it");

    // It teaches: the Region's schooling climbs by the School's own step, to the School's ceiling.
    let step = g.tables.school.per_turn;
    let taught = g.state(sid).schooling;
    g.run_schools();
    assert!((g.state(sid).schooling - (taught + step)).abs() < 1e-9, "an Academy is a School that pays");
    let before = g.colony(cid).unwrap().education;
    g.run_schools();
    assert!(g.colony(cid).unwrap().education > before, "and off Earth too");
}

/// Ticket #181: only a CONTROLLER collects. An occupier pays a Unique Facility's upkeep and draws
/// nothing from its clause until control transfers -- the precedent ticket #69 set for an occupied
/// Research Lab, which works for the world and not for the occupier.
#[test]
fn an_occupier_draws_nothing_from_a_unique_facilitys_clause() {
    let mut g = game();
    let cus = seat_of(&g, FactionKind::Custodians);
    let pro = seat_of(&g, FactionKind::Prospectors);
    g.seats[cus.index()].stockpile.energy = 500;
    let sid = g.controlled_states(cus)[0];
    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::Academy));
    g.income_phase();
    let controlled = g.seats[cus.index()].income_last_turn.ducats;

    let economy = g.state_ducats(sid);
    g.state_mut(sid).control = Control::Occupied { occupier: cus, previous: Some(pro), turns: 1 };
    g.income_phase();
    let occupied = g.seats[cus.index()].income_last_turn.ducats;
    assert_eq!(controlled - occupied, 1 + economy, "the clause and the economy both wait for control");
}

// ------------------------------------------- 0.08.0 ticket #190: the Constabulary's margin

/// Ticket #190: while a Constabulary stands and is online in a Region, the challenge margin there is
/// 25 instead of 20, so taking that Region from its holder wants the holder's Standing plus 25. It
/// protects WHOEVER HOLDS the Region, not the Faction that raised it -- a police force serves the
/// government of the day -- so a Faction that builds one in a Region it later loses has made its own
/// job harder. It does nothing on a neutral Region, which has no margin at all: the price there is
/// the threshold alone.
#[test]
fn a_constabulary_adds_five_to_the_challenge_margin_for_every_challenger() {
    let mut g = game();
    let holder = Seat(0);
    let challenger = Seat(1);
    let sid = g.controlled_states(holder)[0];
    let target = Place::State(sid);
    // A holder's Standing high enough that the margin, not the threshold, is what binds.
    let threshold = g.influence_threshold_for(challenger, target);
    g.seats[holder.index()].influence.insert(target, threshold + 40);

    assert_eq!(g.challenge_margin_at(target), 20, "the base margin");
    let plain = g.influence_needed_for(challenger, target);

    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::Constabulary));
    assert_eq!(g.challenge_margin_at(target), 25, "and 25 with a Constabulary standing");
    assert_eq!(g.influence_needed_for(challenger, target) - plain, 5, "the price of taking it rises by exactly five");

    // For every challenger, whoever built it: the third seat pays the same.
    assert_eq!(g.influence_needed_for(Seat(2), target), g.influence_needed_for(challenger, target));

    // It must be online.
    let i = g.state(sid).facilities.iter().position(|f| f.kind == FacilityKind::Constabulary).unwrap();
    g.state_mut(sid).facilities[i].mothballed = true;
    assert_eq!(g.challenge_margin_at(target), 20, "a mothballed Constabulary guards nothing");
    g.state_mut(sid).facilities[i].mothballed = false;

    // And nothing on a neutral Region: the price there is the threshold alone, Constabulary or not.
    g.state_mut(sid).control = Control::Neutral;
    assert_eq!(g.influence_needed_for(challenger, target), g.influence_threshold_for(challenger, target), "a neutral Region has no margin to raise");
}

// ------------------------------------------- 0.08.0 ticket #191: Relations

/// Ticket #191: one score per ORDERED PAIR, so the Arkwrights' view of the Prospectors is a
/// different number from the Prospectors' view of the Arkwrights. It starts neutral at 0, falls by
/// one for each OFFENDING TURN -- charged per turn, never per order -- and an offence is Influence
/// spent on a place the victim HOLDS. A neutral place is never an offence, however hotly contested:
/// two Factions bidding for empty ground are competing, not crossing each other.
#[test]
fn relations_fall_once_a_turn_for_spending_on_a_place_a_rival_holds() {
    let mut g = game();
    let victim = Seat(0);
    let offender = Seat(1);
    // Ticket #236 (version 0.08.3): park the shared-pot term. Every Faction starts contributing
    // ALL of its Research, which earns the reward, so without this each score below reads one
    // higher and the test measures two rules at once. A directive of 10 leaves a 90% contribution:
    // past the 85% line, short of the 100% that pays.
    for s in Seat::ALL {
        g.seats[s.index()].research_directive = 10;
    }
    let held = Place::State(g.controlled_states(victim)[0]);
    let neutral = Place::State(StateId::ALL.into_iter().find(|s| g.state(*s).control == Control::Neutral).expect("a neutral Region"));

    assert_eq!(g.relations_score(victim, offender), 0, "neutral to begin");
    assert_eq!(g.relations_score(offender, victim), 0, "and in both directions");

    // Two spends on the same victim in one turn cost one point, not two.
    g.pending.influence.push((offender, held, 10));
    g.pending.influence.push((offender, held, 10));
    g.pending.influence.push((offender, neutral, 10));
    g.resolution_phase();
    g.settle_relations();
    assert_eq!(g.relations_score(victim, offender), -1, "charged once for the turn, however many orders");
    assert_eq!(g.relations_score(offender, victim), 0, "and the ordered pair is one-directional");

    // The neutral Region's holder-to-be is nobody, so nobody was offended by that third order.
    for seat in Seat::ALL {
        if seat != victim {
            assert_eq!(g.relations_score(seat, offender), 0, "{seat:?} was not crossed");
        }
    }
}

/// Ticket #191: the score recovers one every four quiet turns and STOPS AT NEUTRAL -- it never rises
/// above it. The +10 half of the scale is reserved and nothing fills it in this version, so an
/// all-zero-or-negative grid is the rule working and not a defect.
#[test]
fn relations_recover_slowly_and_never_rise_above_neutral() {
    let mut g = game();
    let (victim, offender) = (Seat(0), Seat(1));
    // Ticket #236 (version 0.08.3): park the shared-pot term out of the way. Every Faction starts
    // contributing ALL of its Research, which earns the reward, so without this every score below
    // would read one higher and this test would be measuring two rules at once. A directive of 10
    // leaves a 90% contribution: past the 85% line, short of the 100% that pays.
    g.seats[offender.index()].research_directive = 10;
    g.relations.score[victim.index()][offender.index()] = -3;

    for _ in 0..3 {
        g.settle_relations();
    }
    assert_eq!(g.relations_score(victim, offender), -3, "three quiet turns are not enough");
    g.settle_relations();
    assert_eq!(g.relations_score(victim, offender), -2, "the fourth pays one back");

    for _ in 0..40 {
        g.settle_relations();
    }
    assert_eq!(g.relations_score(victim, offender), 0, "and it stops at neutral, however long the peace");

    // The floor holds too.
    for _ in 0..20 {
        g.offend(offender, victim);
        g.settle_relations();
    }
    assert_eq!(g.relations_score(victim, offender), -10, "and the scale has a floor");
}

/// Ticket #191: in version 0.08.0 the score does nothing mechanical. It is read, not spent -- no
/// rule reads it, and the computer players do not read it. Pinned so that a later version giving it
/// teeth has to come past this test and say so.
#[test]
fn relations_do_nothing_mechanical_in_this_version() {
    // Ticket #268 (version 0.08.4): the premise of this test -- written on ticket #191 when Relations
    // were only read -- is dead, and this is the first version where six computer turns say so on
    // the board: with every pair Hostile the Custodians refuse to sell carbon credits, so the seat
    // that bought them in the neutral game keeps its Ducats and its Blame in the hostile one. The
    // test is kept, inverted, as the record of the moment Relations became mechanical to the
    // computer seats and not only to a player reading a card.
    let mut a = with_seed(11);
    let mut b = with_seed(11);
    for viewer in Seat::ALL {
        for subject in Seat::ALL {
            if viewer != subject {
                b.relations.score[viewer.index()][subject.index()] = -10;
            }
        }
    }
    for _ in 0..6 {
        pick_a_tech(&mut a);
        pick_a_tech(&mut b);
        a.end_turn(std::array::from_fn(|_| Vec::new())).unwrap();
        b.end_turn(std::array::from_fn(|_| Vec::new())).unwrap();
    }
    let picture = |g: &Game| -> Vec<(i64, i64, i64, u32, usize)> {
        Seat::ALL
            .into_iter()
            .map(|s| {
                let st = g.seat(s);
                (st.stockpile.materials, st.stockpile.ducats, st.venture_fund, st.allotment as u32, g.controlled_states(s).len())
            })
            .collect()
    };
    // The hostile game proposes no purchase at all -- the Custodians will not sell to a Hostile
    // buyer -- so the computer seats' choices, and with them the board, part from the neutral game's.
    assert_ne!(picture(&a), picture(&b), "six turns of the worst possible blood now change the board: {:?}", picture(&a));
    assert_eq!(Seat::ALL.into_iter().map(|s| b.seat(s).credits_bought).sum::<f64>(), 0.0, "and nobody bought a credit in it");
}

// ------------------------------------------- 0.08.0 ticket #192: the Archive's gate and the Upload

/// Ticket #192: ordering the Archive wants four Colonists living at the place, checked ONCE at the
/// order. Neither the three-turn build nor the standing Module cares afterwards: a build that could
/// stall halfway would be a new state to hold in the save, draw on the card and say in the Report,
/// for a case that fires rarely, and the Victory Condition does its own checking at the end.
#[test]
fn ordering_the_archive_wants_four_colonists_at_the_place_and_only_at_the_order() {
    let mut g = game();
    let arc = seat_of(&g, FactionKind::Archivists);
    g.seats[arc.index()].stockpile.materials = 200;
    let cid = colony(&mut g, arc, BodyId::Mars, &[ModuleKind::Habitat], 0);
    let order = Order::BuildArchive { colony: cid };

    // Ticket #199 (version 0.08.0): the gate Tech is named FIRST of the two refusals, because it is
    // the one still true after the other is solved -- four Colonists arrive at a median turn 11 and
    // The Upload at a median 15.
    g.colony_mut(cid).unwrap().colonists = 4;
    assert_eq!(
        g.check_order(arc, &[], &order).unwrap_err().0,
        "the Archive waits on The Upload, which the world has not researched yet"
    );
    g.colony_mut(cid).unwrap().colonists = 0;
    assert_eq!(g.check_order(arc, &[], &order).unwrap_err().0.split(',').next().unwrap(), "the Archive waits on The Upload");
    the_upload(&mut g);

    assert_eq!(g.tables.archive.colonists_to_order, 4, "the card figure");
    for (living, expected) in [
        (0, "the Archive wants 4 Colonists living at its Colony; nobody lives at"),
        (1, "the Archive wants 4 Colonists living at its Colony; one lives at"),
        (3, "the Archive wants 4 Colonists living at its Colony; 3 live at"),
    ] {
        g.colony_mut(cid).unwrap().colonists = living;
        let err = g.check_order(arc, &[], &order).unwrap_err().0;
        assert!(err.starts_with(expected), "at {living}: {err}");
    }
    g.colony_mut(cid).unwrap().colonists = 4;
    assert!(g.check_order(arc, &[], &order).is_ok(), "four is enough");

    // Once ordered, the place may empty and the build RUNS TO COMPLETION regardless. A build that
    // could stall halfway would be a new state to hold in the save, draw on the card and say in the
    // Report, and the Victory Condition does its own checking at the end anyway.
    g.commit_orders(arc, std::slice::from_ref(&order));
    assert!(g.archive_ordered(arc));
    g.colony_mut(cid).unwrap().colonists = 0;
    for _ in 0..g.tables.module(ModuleKind::Archive).build_turns {
        g.turn += 1;
        g.resolution_phase();
    }
    assert!(g.archive_built(arc), "the Module stands though nobody lives there now");
}

/// Ticket #192: Upload. An ORDER, because it is irreversible and this game asks before anything
/// irreversible, and FREE, because the 80 Research and the Energy upkeep are already the monument's
/// price. It needs the Archive complete, it may only draw from the population of the place the
/// Archive stands at, and an uploaded Colonist leaves the living population.
#[test]
fn uploading_reads_colonists_into_the_archive_and_they_leave_the_living() {
    let mut g = game();
    let arc = seat_of(&g, FactionKind::Archivists);
    let cus = seat_of(&g, FactionKind::Custodians);
    let cid = archive_at(&mut g, arc, BodyId::Mars, 0, 8);
    g.seats[arc.index()].uploaded = 0;
    let upload = |n| Order::Upload { colony: cid, n };

    // The Research is the machine, and the upload is what the machine is for.
    assert_eq!(
        g.check_order(arc, &[], &upload(4)).unwrap_err().0,
        "the Archive is not complete: its Research is the machine, and the upload is what the machine is for"
    );
    g.seats[arc.index()].archive_fund = g.tables.archive.research;
    assert!(g.archive_complete(arc));
    assert_eq!(g.order_cost(arc, &upload(4)), Cost::default(), "free");

    // Only the Archivists, and only where the Archive stands.
    assert_eq!(g.check_order(cus, &[], &upload(4)).unwrap_err().0, "only the Archivists upload into the Archive");
    let elsewhere = colony(&mut g, arc, BodyId::Moon, &[ModuleKind::Habitat], 8);
    assert_eq!(
        g.check_order(arc, &[], &Order::Upload { colony: elsewhere, n: 4 }).unwrap_err().0,
        "the Archive does not stand here; it may only draw from the people where it is"
    );

    // In batches, and never more than live there.
    assert!(g.check_order(arc, &[], &upload(9)).unwrap_err().0.starts_with("only 8 Colonists are left to upload"));
    assert!(g.check_order(arc, &[upload(5)], &upload(4)).unwrap_err().0.starts_with("only 3 Colonists are left to upload"), "a pending order takes its people first");

    g.commit_orders(arc, &[upload(4)]);
    assert_eq!(g.seats[arc.index()].uploaded, 4, "four read in");
    assert_eq!(g.colony(cid).unwrap().colonists, 4, "and four fewer living");
    g.commit_orders(arc, &[upload(4)]);
    assert_eq!(g.seats[arc.index()].uploaded, 8);
    assert_eq!(g.colony(cid).unwrap().colonists, 0, "the place shrinks as it uploads");
    assert_eq!(g.check_order(arc, &[], &upload(1)).unwrap_err().0.split(" at ").next().unwrap(), "nobody is left to upload");
}

/// Ticket #192: the second Victory part counts the UPLOADED and not the living. The count is
/// monotonic -- uploaded people cannot be lost to a raid, a crowding death or a handover -- and it
/// closes the odd case the old wording allowed, where a Faction won by having twelve people standing
/// NEXT TO a finished Archive rather than inside it.
#[test]
fn the_archivists_second_part_counts_the_uploaded_not_the_living() {
    let mut g = game();
    let arc = seat_of(&g, FactionKind::Archivists);
    let cid = archive_at(&mut g, arc, BodyId::Mars, 80, 12);
    // Twelve standing beside a finished Archive, and nobody read in: no win.
    g.seats[arc.index()].uploaded = 0;
    open_gates(&mut g);
    g.seats[arc.index()].stockpile.energy = 200;
    g.income_phase();
    assert!(g.archive_online(arc), "the premise: the Archive is running");
    let p = g.progress(arc);
    assert_eq!((p.first_value, p.second_value), (80.0, 0.0), "standing next to it is worth nothing now");
    assert!(!p.met());

    // Read them in, and the same twelve win it -- while the Colony itself stands empty.
    g.commit_orders(arc, &[Order::Upload { colony: cid, n: 12 }]);
    assert_eq!(g.colony(cid).unwrap().colonists, 0);
    let p = g.progress(arc);
    assert_eq!((p.second_value, p.second_bar), (12.0, 12.0));
    assert!(p.met());

    // And it cannot be taken back: losing the Colony outright leaves the count where it was.
    g.colonies.retain(|c| c.id != cid);
    assert_eq!(g.progress(arc).second_value, 12.0, "the uploaded are beyond reach");
}

/// Ticket #192: the computer must be taught both halves or the Archivists win nothing at all. It
/// uploads whenever the Archive is complete and anybody lives at its place, and it asks for no more
/// than the bar still wants.
#[test]
fn the_archivist_ai_uploads_whenever_the_archive_is_complete_and_anybody_lives_there() {
    let mut g = game();
    let arc = seat_of(&g, FactionKind::Archivists);
    g.seats[arc.index()].stockpile.materials = 200;
    g.seats[arc.index()].stockpile.energy = 200;
    let cid = archive_at(&mut g, arc, BodyId::Mars, 0, 6);
    g.seats[arc.index()].uploaded = 0;

    // Not while the Research is unpaid.
    assert!(!g.ai_orders(arc).iter().any(|o| matches!(o, Order::Upload { .. })), "the Archive is not complete yet");

    g.seats[arc.index()].archive_fund = g.tables.archive.research;
    let orders = g.ai_orders(arc);
    match orders.iter().find_map(|o| if let Order::Upload { colony, n } = o { Some((*colony, *n)) } else { None }) {
        Some((c, n)) => {
            assert_eq!(c, cid, "at the Archive's own place");
            assert_eq!(n, 6, "everyone living there");
        }
        None => panic!("the Archivist AI never uploads: {orders:?}"),
    }

    // And it asks for no more than the bar still wants.
    g.seats[arc.index()].uploaded = 10;
    let orders = g.ai_orders(arc);
    let n = orders.iter().find_map(|o| if let Order::Upload { n, .. } = o { Some(*n) } else { None }).expect("an upload order");
    assert_eq!(n, 2, "two short of twelve, so two");
}

// ---------------------------------------------------------------- 0.08.1 ticket #210: Ship names

/// Ticket #210 (version 0.08.1): every Ship is named when it is built, from one of two lists. A
/// Colony Ship draws from the colony list; a Frigate, a Battleship and the Carrier from the warship
/// list -- the Carrier at the designer's word, because it sails with a fleet and an explorer's name
/// would read oddly beside a Battleship. The pick is the first UNUSED name in LIST ORDER, which is
/// deterministic and draws no randomness: a random pick would shift every later roll in a seeded
/// game and make a sweep incomparable with its baseline.
#[test]
fn a_ship_is_named_from_the_list_its_kind_draws_from() {
    let mut g = game();
    let colony_first = g.tables.ship_names.colony.names[0].clone();
    let warship_first = g.tables.ship_names.warship.names[0].clone();
    assert_ne!(colony_first, warship_first, "the two lists must not open with the same name");

    assert_eq!(g.next_ship_name(UnitKind::ColonyShip), colony_first);
    assert_eq!(g.next_ship_name(UnitKind::Frigate), warship_first);
    assert_eq!(g.next_ship_name(UnitKind::Battleship), warship_first);
    assert_eq!(g.next_ship_name(UnitKind::Carrier), warship_first, "the Carrier sails with the fleet");

    // A name in use is skipped, and uniqueness is across the WHOLE BOARD, not per Faction: the
    // Ship below is seat 0's and still blocks the name for everyone.
    let id = ShipId(g.fresh_id());
    g.ships.push(Ship {
        id,
        name: colony_first.clone(),
        kind: UnitKind::ColonyShip,
        seat: Seat(0),
        damage: 0,
        at: ShipAt::Body(BodyId::Earth),
        colonists: 0,
        colonists_education: 1.0,
        army: None,
        stance: Stance::Hold,
        escaped: false,
        arrived_this_turn: false,
        built_turn: 1,
        fuel: 30,
        slot: None,
    });
    let second = g.tables.ship_names.colony.names[1].clone();
    assert_eq!(g.next_ship_name(UnitKind::ColonyShip), second, "a name in use is passed over");

    // What a Ship is CALLED carries its holder's prefix, which lives on the Faction's card. The
    // name belongs to the hull and the prefix to whoever flies it.
    let prefix = g.tables.faction(g.kind(Seat(0))).ship_prefix.clone();
    assert_eq!(g.ship_name(g.ship(id).unwrap()), format!("{prefix} {colony_first}"));
}

/// Ticket #210: an exhausted list begins again with a numeral rather than running out.
#[test]
fn an_exhausted_ship_name_list_wraps_with_a_numeral() {
    let mut g = game();
    let names = g.tables.ship_names.colony.names.clone();
    for (i, n) in names.iter().enumerate() {
        let id = ShipId(g.fresh_id());
        g.ships.push(Ship {
            id,
            name: n.clone(),
            kind: UnitKind::ColonyShip,
            seat: Seat(0),
            damage: 0,
            at: ShipAt::Body(BodyId::Earth),
            colonists: 0,
            colonists_education: 1.0,
            army: None,
            stance: Stance::Hold,
            escaped: false,
            arrived_this_turn: false,
            built_turn: 1 + i as u32,
            fuel: 30,
            slot: None,
        });
    }
    assert_eq!(g.next_ship_name(UnitKind::ColonyShip), format!("{} II", names[0]), "the whole list is spent, so it begins again");
    // The warship list is untouched by any of that.
    assert_eq!(g.next_ship_name(UnitKind::Battleship), g.tables.ship_names.warship.names[0]);
}

// ------------------------------------------------------- 0.08.1 ticket #201: Civil Defense

/// Ticket #201 (version 0.08.1): the eighteenth Tech. It began life on its ticket as "something
/// that moderates unrest" and became something else at the designer's word, knowingly: it works
/// through the CONSTABULARY and raises the CHALLENGE MARGIN, which is the price of taking a place
/// its holder already has, and no figure Unrest ever reads. A Constabulary adds 10 where it added 5.
///
/// The Tech is the world's, as every Tech is, so it helps whoever HOLDS a garrisoned Region and
/// hinders whoever wants one -- the same asymmetry the Constabulary has carried since ticket #190.
#[test]
fn civil_defense_doubles_what_a_constabulary_is_worth_at_the_gate() {
    let mut g = game();
    let sid = StateId::EastAsia;
    let base = g.tables.influence.challenge_margin;
    let target = Place::State(sid);

    // No Constabulary: the Tech reaches nothing at all.
    assert_eq!(g.challenge_margin_at(target), base, "no garrison, no bonus");
    with_tech(&mut g, TechId::CivilDefense);
    assert_eq!(g.challenge_margin_at(target), base, "the Tech alone does nothing: it works through the building");

    // With one standing and online, the Tech's figure replaces the Constabulary's own.
    let mut g = game();
    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::Constabulary));
    assert_eq!(g.challenge_margin_at(target), base + g.tables.influence.constabulary_margin, "5 without the Tech");
    with_tech(&mut g, TechId::CivilDefense);
    assert_eq!(
        g.challenge_margin_at(target),
        base + g.tables.influence.constabulary_margin_defended,
        "10 with it, not 5 plus 10"
    );
    assert_eq!(g.tables.influence.constabulary_margin_defended, 10);
}

/// Ticket #201: the tree's costs, which this ticket set and whose header comment had been wrong
/// since version 0.07.1 -- it claimed 16, 28, 44 while the data said 15, 30, 45.
///
/// Ticket #231 (version 0.08.3): rungs 2 and 3 to 32 and 48, RUNG 1 LEFT AT 18. The total is the
/// figure the ticket was decided on -- 554 to 585, a rise of 5.6%, about 1.8 turns of late-game
/// Research -- so it is pinned here and not left to be re-derived.
#[test]
fn the_tree_costs_eighteen_thirty_two_and_forty_eight_by_rung() {
    let g = game();
    for t in TechId::ALL {
        let card = g.tables.tech(t);
        if t == TechId::CoastalEngineering {
            continue;
        }
        let want = match card.rung {
            1 => 18,
            2 => 32,
            _ => 48,
        };
        assert_eq!(card.cost, want, "rung {} costs {want}: {t:?}", card.rung);
    }
    let total: i64 = TechId::ALL.into_iter().map(|t| g.tables.tech(t).cost).sum();
    assert_eq!(total, 649, "the whole tree since ticket #232's two rung-2 Techs; 585 from #231, 554 from #201, 507 before that");
}

// ------------------------------------------------------- 0.08.1 ticket #208: the School's step

/// Ticket #208 (version 0.08.1): the School's step and decay go to 0.20, one figure for both --
/// the designer's line changed them together, and two would let a School be built cheaply and
/// mothballed slowly, which is a lever nobody asked for.
///
/// This test exists because the change was made and the whole suite stayed green: nothing pinned
/// the figure, so a rule could move and no check had anything to say about it.
#[test]
fn a_school_climbs_and_decays_by_one_fifth_a_turn() {
    let mut g = game();
    let sid = StateId::EastAsia;
    let card = g.tables.state(sid).education_level;
    let step = g.tables.school.per_turn;
    assert!((step - 0.20).abs() < 1e-9, "0.20 since ticket #208, 0.25 from #185");

    // With a School standing and online it climbs a step a turn, and the LIVE figure is the card's
    // plus what the School has done.
    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::School));
    g.run_schools();
    assert!((g.education_level(sid) - (card + step)).abs() < 1e-9, "one step up");
    g.run_schools();
    assert!((g.education_level(sid) - (card + 2.0 * step)).abs() < 1e-9, "two steps up");

    // It never climbs past the ceiling, however long it stands.
    for _ in 0..40 {
        g.run_schools();
    }
    assert!((g.education_level(sid) - g.tables.school.ceiling).abs() < 1e-9, "the ceiling holds");

    // And it falls back at the SAME rate when the School stops, never below the card's own figure.
    g.state_mut(sid).facilities.clear();
    g.run_schools();
    assert!((g.education_level(sid) - (g.tables.school.ceiling - step)).abs() < 1e-9, "one step down, at the same rate");
    for _ in 0..40 {
        g.run_schools();
    }
    assert!((g.education_level(sid) - card).abs() < 1e-9, "back to the card, and no lower");
}

/// Ticket #220 (version 0.08.2): the Trading window's prices move with what the table bought.
///
/// Watched red before it was believed: with `settle_market` made a no-op the first assertion fails
/// at `4 != 3`, and with the drift arm removed the last one fails at `4 != 3` the other way about.
#[test]
fn market_prices_move_with_the_table() {
    let mut g = game();
    let step = g.tables.ducats.price_step_units;
    let base = g.market_base(0);
    assert_eq!(g.trade_price(dying_earth_engine::Resource::Materials), Some(base), "a fresh market opens at the card figure");

    // A turn of net buying at the step moves it one up, and it sticks at the band's edge.
    g.market.net[0] = step;
    g.settle_market();
    assert_eq!(g.market_price_at(0), base + 1, "net buying of one step raises the price");
    g.market.net[0] = step * 4;
    g.settle_market();
    assert_eq!(g.market_price_at(0), base + g.tables.ducats.price_band, "the price sticks at the edge of its band");

    // Trading below the step moves nothing, and does NOT count as quiet either.
    g.market.price[0] = base;
    g.market.net[0] = step - 1;
    g.settle_market();
    assert_eq!(g.market_price_at(0), base, "trading below the step moves nothing");

    // Only a turn with NO trade in that resource brings it home.
    g.market.price[0] = base + 1;
    g.market.net[0] = 0;
    g.settle_market();
    assert_eq!(g.market_price_at(0), base, "a quiet turn brings the price one step home");

    // A resource under steady demand stays dear rather than sliding back between purchases.
    g.market.price[0] = base + 1;
    g.market.net[0] = 1;
    g.settle_market();
    assert_eq!(g.market_price_at(0), base + 1, "any trade at all leaves the price where it was");
}

/// Ticket #221 (version 0.08.2): Blame is a LEVEL on the shown score, weighted by how much the
/// RESENTING Faction minds. Watched red: with `blame_relations_term` returning 0 the Custodian
/// assertion fails at 0 against -2.
#[test]
fn blame_costs_a_faction_its_friends() {
    let mut g = game();
    let (cus, pro, ark) = (Seat(0), Seat(1), Seat(2));
    // Ticket #236 (version 0.08.3): park the shared-pot term. Every Faction starts contributing
    // ALL of its Research, which earns the reward, so without this each score below reads one
    // higher and the test measures two rules at once. A directive of 10 leaves a 90% contribution:
    // past the 85% line, short of the 100% that pays.
    for s in Seat::ALL {
        g.seats[s.index()].research_directive = 10;
    }
    // A Prospector share of about 0.40 is one step above the 0.35 gate.
    for s in Seat::ALL {
        g.seats[s.index()].blame_emitted = if s == pro { 40.0 } else { 20.0 };
    }
    assert_eq!(g.blame_relations_term(cus, pro), -2, "the Custodians mind at x2");
    assert_eq!(g.blame_relations_term(ark, pro), 0, "the Arkwrights at x0.5 round TOWARD ZERO: one step x 0.5 truncates to nothing, so they feel it only from 0.45");
    assert_eq!(g.blame_relations_term(pro, cus), 0, "the Prospectors mind nobody's Blame");
    assert_eq!(g.relations_score(cus, pro), -2, "the shown score carries it without any deed");
    assert_eq!(g.relations_deeds(cus, pro), 0, "and the deeds figure does not");
    assert_eq!(g.relations_level(cus, pro), "Neutral", "-2 is still the top of Neutral");
}

/// Ticket #222/#225 (version 0.08.2): a turn charges the sum of every instance, capped; the scar
/// counts TURNS. Watched red: with the cap removed the first assertion reads -12.
#[test]
fn the_ladder_sums_instances_and_the_scar_counts_turns() {
    let mut g = game();
    let (cus, pro) = (Seat(0), Seat(1));
    // A conquest turn: a Battle (3), an Occupation (3), a place taken (5) and a bid elsewhere (1).
    for w in [3, 3, 5, 1] {
        g.offend_by(pro, cus, w);
    }
    g.settle_relations();
    assert_eq!(g.relations_deeds(cus, pro), -g.tables.relations.turn_cap, "twelve charged, eight paid");
    assert_eq!(g.relations.offending_turns[cus.index()][pro.index()], 1, "one turn, however much was done in it");
    // Four offending turns move the floor one step, whatever each turn cost.
    for _ in 0..3 {
        g.offend_by(pro, cus, 1);
        g.settle_relations();
    }
    assert_eq!(g.relations.floor[cus.index()][pro.index()], -1, "a step every four offending turns");
    assert!(g.relations_deeds(cus, pro) <= -1, "and the deeds figure cannot climb above it");
}

/// Ticket #226 (version 0.08.2): an Accord holds until it is ended, ending is free with a turn's
/// notice, and violating a term costs +3 AND ends the whole arrangement.
///
/// Watched red: with `break_accord` made a no-op the violation assertion finds the Accord still
/// standing, and with the +3 removed the score reads -1 rather than -4.
#[test]
fn an_accord_ends_freely_and_breaks_dearly() {
    let mut g = game();
    let (cus, pro) = (Seat(0), Seat(1));

    g.strike_accord(cus, pro, vec![Term::NonAggression]).expect("struck");
    assert!(g.accord_has(cus, pro, Term::NonAggression), "it holds both ways round");
    assert!(g.accord_has(pro, cus, Term::NonAggression));

    // Ending is free and takes a turn's notice: it still holds this turn, and is gone next.
    g.end_accord(cus, pro);
    assert!(!g.accord_has(cus, pro, Term::NonAggression), "a declared-over Accord carries nothing");
    g.settle_accords();
    assert!(g.accords.is_empty(), "and lapses at the settle");
    g.settle_relations();
    assert_eq!(g.relations_deeds(pro, cus), 0, "ending cost nothing at all");

    // Breaking is another matter: the offence AND the +3, and the Accord is gone at once.
    g.strike_accord(cus, pro, vec![Term::NonAggression]).expect("struck again");
    g.offend_by(cus, pro, 1);
    assert!(g.accords.is_empty(), "violating a term ends the whole Accord");
    g.settle_relations();
    assert_eq!(g.relations_deeds(pro, cus), -4, "the act charged 1 and the betrayal 3");
}

/// Ticket #226: a research agreement wants Friendly on BOTH sides to strike, and pays a tenth once
/// struck. Watched red: with the gate removed the first strike succeeds.
#[test]
fn a_research_agreement_wants_friendship_first() {
    let mut g = game();
    let (cus, pro) = (Seat(0), Seat(1));
    assert!(g.strike_accord(cus, pro, vec![Term::ResearchAgreement]).is_err(), "Neutral is not Friendly");
    g.relations.score[cus.index()][pro.index()] = 7;
    g.relations.score[pro.index()][cus.index()] = 7;
    assert!(g.strike_accord(cus, pro, vec![Term::ResearchAgreement]).is_ok(), "Friendly on both sides");
    assert!((g.research_agreement_multiplier(cus) - 1.10).abs() < 1e-9, "a tenth more Research");
    // The gate is checked only at the STRIKE: it stands whatever the score later does.
    g.relations.score[cus.index()][pro.index()] = -9;
    assert!((g.research_agreement_multiplier(cus) - 1.10).abs() < 1e-9, "once made, it stands");
}

// ------------------------------------------- 0.08.3 ticket #232: Beneficiation and Relay Networks

/// Ticket #232 (version 0.08.3): Beneficiation gives every Mine a tenth more, stacked inside the
/// same multiplier chain as Deep Mining and the Extraction Charter so the product is floored ONCE.
///
/// The figures are pinned exactly, and the reason is a defect this test had in its first draft: it
/// asserted `after >= before`, which PASSES with the Tech's effect deleted. Watched to fail with
/// the multiplier removed, it did not fail, and that is how the weak assertion was found. A tenth
/// is small enough to vanish into a floor, so nothing less than an exact figure guards it.
///
/// Measured while deciding, with a Mine alone on the slot: a tenth ON ITS OWN is invisible on Earth
/// (7 -> 7) and Mars (5 -> 5) and shows only on the Moon (6 -> 7). That case cannot arise in play,
/// because Beneficiation NEEDS Deep Mining -- and with Deep Mining standing it is +1 Materials per
/// Mine on every Body measured. The prerequisite is load-bearing, not decoration.
#[test]
fn beneficiation_adds_one_to_every_mine_once_deep_mining_stands() {
    for (body, deep, with_ben, all_three) in [(BodyId::Earth, 10, 11, 14), (BodyId::Moon, 9, 10, 13), (BodyId::Mars, 7, 8, 10), (BodyId::Phobos, 10, 11, 14)] {
        let mut g = game();
        let seat = Seat(0);
        let c = colony(&mut g, seat, body, &[ModuleKind::Mine], 4);
        with_tech(&mut g, TechId::DeepMining);
        assert_eq!(g.module_yield(seat, c, ModuleKind::Mine).amount, deep, "Deep Mining alone at {body:?}");
        with_tech(&mut g, TechId::Beneficiation);
        assert_eq!(g.module_yield(seat, c, ModuleKind::Mine).amount, with_ben, "and Beneficiation adds one at {body:?}");
        with_tech(&mut g, TechId::ExtractionCharter);
        assert_eq!(g.module_yield(seat, c, ModuleKind::Mine).amount, all_three, "and all three chain at {body:?}");
    }
    let g = game();
    assert_eq!(g.tables.tech(TechId::Beneficiation).needs, vec![TechId::DeepMining], "the prerequisite the figures above depend on");
}

/// Ticket #232: ANTARCTICA IS INCLUDED -- the clause a reader of the designer's words ("off world
/// mines") would get wrong, and the whole value of the Tech. Measured while deciding: of sixteen
/// Mines standing at the end of twenty games, THIRTEEN were Antarctic and three were off Earth, so
/// an off-world-only clause would have reached 0.15 Mines a game. The designer, told that: "count
/// Antarctica and change the tech's name".
#[test]
fn beneficiation_reaches_an_antarctic_mine_too() {
    let mut g = game();
    let seat = Seat(0);
    let c = colony(&mut g, seat, BodyId::Earth, &[ModuleKind::Mine], 4);
    with_tech(&mut g, TechId::DeepMining);
    let before = g.module_yield(seat, c, ModuleKind::Mine).amount;
    with_tech(&mut g, TechId::Beneficiation);
    assert_eq!(g.module_yield(seat, c, ModuleKind::Mine).amount, before + 1, "an Antarctic Mine reads the Tech exactly as one off Earth does");
    assert_eq!(g.tables.tech(TechId::Beneficiation).effect, "Mine output x1.1, wherever the Mine stands", "and the effect line says so, because the name no longer can");
    assert!(!g.tables.tech(TechId::Beneficiation).name.to_lowercase().contains("orbit"), "the name carries no space word: it would be a lie");
}

/// Ticket #232: Relay Networks takes a Relay's Influence Allotment from 1 to 2 and leaves its
/// Standing alone. The designer asked for the +1 on the Habitat, then moved it here.
#[test]
fn relay_networks_takes_a_relay_from_one_influence_to_two() {
    let mut g = game();
    let seat = Seat(0);
    let c = colony(&mut g, seat, BodyId::Moon, &[ModuleKind::Relay], 4);
    let before = g.module_yield(seat, c, ModuleKind::Relay);
    assert_eq!(before.allotment, 1, "a Relay pays 1 into the Allotment without the Tech");

    with_tech(&mut g, TechId::RelayNetworks);
    let after = g.module_yield(seat, c, ModuleKind::Relay);
    assert_eq!(after.allotment, 2, "and 2 with it");
    assert_eq!(after.standing, before.standing, "its Standing is untouched: Standing holds one place, the Allotment is the budget");

    // A Habitat gets nothing: the designer moved the clause OFF the Habitat deliberately.
    let h = colony(&mut g, seat, BodyId::Mars, &[ModuleKind::Habitat], 4);
    assert_eq!(g.module_yield(seat, h, ModuleKind::Habitat).allotment, 0, "a Habitat pays no Influence");
}

/// Ticket #232: the AI's appetite. Both weights were FLAT and read nothing about what the building
/// would actually make, so no Tech in the game had ever moved a computer seat's build choice. The
/// sweep that found 26 Mines and 1 Relay standing across forty games is the evidence. These two
/// assert the yields the weights now read, which is the thing that was missing.
#[test]
fn the_yields_the_ai_weights_now_read_actually_move_with_their_techs() {
    let mut g = game();
    let seat = Seat(0);
    let mine = colony(&mut g, seat, BodyId::Moon, &[ModuleKind::Mine], 4);
    let relay = colony(&mut g, seat, BodyId::Mars, &[ModuleKind::Relay], 4);
    let mine_before = g.module_yield(seat, mine, ModuleKind::Mine).amount;
    let relay_before = g.module_yield(seat, relay, ModuleKind::Relay).allotment;

    for t in [TechId::DeepMining, TechId::Beneficiation, TechId::ExtractionCharter, TechId::RelayNetworks] {
        with_tech(&mut g, t);
    }
    let mine_after = g.module_yield(seat, mine, ModuleKind::Mine).amount;
    let relay_after = g.module_yield(seat, relay, ModuleKind::Relay).allotment;

    assert!(mine_after > mine_before, "a Mine's yield moves with its Techs: {mine_before} -> {mine_after}");
    assert!(relay_after > relay_before, "and a Relay's: {relay_before} -> {relay_after}");
    // The Mine's weight is scaled by yield-over-card, so the card figure must stay reachable.
    let card = g.tables.module(ModuleKind::Mine).produces.as_ref().map(|p| p.amount).unwrap_or(0);
    assert_eq!(card, 4, "the Mine's card figure, which the AI weight divides by");
}

/// Ticket #244 (version 0.08.3): the Arkwrights' signature rule is COACH CLASS, and the word
/// Steerage appears on no Faction card. Renamed for the reason the Emigrant became the Pioneer --
/// steerage is the cheapest class of passage on an emigrant ship, and this rule's own avoid list
/// had been warning off "cattle class" since it was written.
///
/// This test exists because the rename moved no MECHANIC: every figure Coach Class controls is
/// unchanged, so the tests that guard the rule stayed green throughout and witnessed nothing. A
/// rename with no guard is exactly how a word creeps back.
#[test]
fn the_arkwrights_signature_rule_is_coach_class_and_says_steerage_nowhere() {
    let g = game();
    let card = g.tables.faction(FactionKind::Arkwrights);
    assert!(card.signature.starts_with("Coach Class"), "the card leads with the rule's name: {}", card.signature);
    for kind in FactionKind::ALL {
        let c = g.tables.faction(kind);
        for text in [&c.signature, &c.victory, &c.blurb, &c.unique] {
            assert!(!text.to_lowercase().contains("steerage"), "{kind:?} still says Steerage: {text}");
        }
    }
}

// -------------------------------------------- 0.08.4 ticket #270: names for Armies

/// Ticket #270 (version 0.08.4): an Army is named from its home as it is raised -- an ordinal and
/// the Region's demonym, Standing Armies included, a Colony's a Garrison -- and keeps the name
/// through a change of hands.
#[test]
fn an_army_is_named_from_its_home_as_it_is_raised() {
    let mut g = game();
    assert_eq!(Game::ordinal(1), "1st");
    assert_eq!(Game::ordinal(2), "2nd");
    assert_eq!(Game::ordinal(3), "3rd");
    assert_eq!(Game::ordinal(4), "4th");
    assert_eq!(Game::ordinal(11), "11th");
    assert_eq!(Game::ordinal(12), "12th");
    assert_eq!(Game::ordinal(13), "13th");
    assert_eq!(Game::ordinal(21), "21st");
    assert_eq!(Game::ordinal(112), "112th");
    assert_eq!(g.tables.state(StateId::EastAsia).demonym, "Chinese");
    let standing = g.armies.iter().find(|a| a.standing && a.home == ArmyHome::State(StateId::EastAsia)).expect("China's Standing Army");
    assert_eq!(g.army_name(standing), "the 1st Chinese Army", "the Standing Army is the first raised");
    let raised = g.raise_army(Place::State(StateId::EastAsia), false);
    let a = g.armies.iter().find(|a| a.id == raised).unwrap();
    assert_eq!(g.army_name(a), "the 2nd Chinese Army");
    let third = g.raise_army(Place::State(StateId::EastAsia), false);
    assert_eq!(g.army_name(g.armies.iter().find(|a| a.id == third).unwrap()), "the 3rd Chinese Army");
    // The name is the Army's, not the holder's: it survives the Region changing hands.
    g.take_control(StateId::EastAsia, Seat(2));
    assert_eq!(g.army_name(g.armies.iter().find(|a| a.id == raised).unwrap()), "the 2nd Chinese Army");
    // A Colony's is a Garrison, numbered from the second.
    let c = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Habitat, ModuleKind::Barracks], 4);
    let g1 = g.raise_army(Place::Colony(c), false);
    let g2 = g.raise_army(Place::Colony(c), false);
    let place = g.tables.body(BodyId::Moon).slots[g.colony(c).unwrap().slot as usize].name.clone();
    assert_eq!(g.army_name(g.armies.iter().find(|a| a.id == g1).unwrap()), format!("the {place} Garrison"));
    assert_eq!(g.army_name(g.armies.iter().find(|a| a.id == g2).unwrap()), format!("the 2nd {place} Garrison"));
    // A save from before names reads the old form.
    let mut old = g.armies[0].clone();
    old.name.clear();
    old.standing = true;
    assert_eq!(g.army_name(&old), "the Standing Army");
}

// -------------------------------------------- 0.08.4 ticket #269: Agitate

/// Ticket #269 (version 0.08.4): Agitate raises the Unrest of a Region a rival controls by one for
/// 15 Ducats and 5 Influence, once a turn per Region per Faction; never on your own or a neutral
/// Region; an offence the holder's Report names; a working Constabulary halves it.
#[test]
fn agitate_raises_a_rivals_regions_unrest_for_ducats_and_influence_once_a_turn() {
    let sid = StateId::Europe;
    let mut g = game();
    calm(&mut g);
    let holder = Seat(1);
    g.take_control(sid, holder);
    g.seats[0].stockpile.ducats = 100;
    g.seats[0].allotment = 20;
    let u = g.tables.unrest.clone();
    assert_eq!((u.agitate_ducats, u.agitate_influence, u.agitate_points), (15, 5, 1.0));
    let order = Order::Agitate { state: sid };
    assert_eq!(g.order_cost(Seat(0), &order), Cost { ducats: 15, influence: 5, ..Default::default() });
    assert!(g.check_order(Seat(0), &[], &order).is_ok());
    assert!(g.check_order(Seat(0), std::slice::from_ref(&order), &order).is_err(), "one a turn per Region");
    let neutral = StateId::ALL.into_iter().find(|s| matches!(g.state(*s).control, Control::Neutral)).unwrap();
    assert!(g.check_order(Seat(0), &[], &Order::Agitate { state: neutral }).is_err(), "nobody to turn them against");
    assert!(g.check_order(holder, &[], &order).is_err(), "not against yourself");
    // Unrest falls 1.5 a turn on its own, after every rise, so the rise is read against a state
    // already restive: 3, plus one, less the fall, is 2.5. (From calm, one Agitate a turn nets out
    // against the fall -- see the ticket's resolution.)
    g.state_mut(sid).unrest = 3.0;
    g.commit_orders(Seat(0), &[order]);
    assert_eq!((g.seats[0].stockpile.ducats, g.seats[0].allotment), (85, 15), "paid at the Orders phase");
    g.resolution_phase();
    assert!((g.state(sid).unrest - 2.5).abs() < 1e-9, "3 + 1 - the fall of 1.5: {}", g.state(sid).unrest);
    assert!(g.relations.offended[holder.index()][0], "an offence against the holder");
    assert!(g.report.lines.iter().any(|l| l.text.contains("agitated in")), "the Report names who paid: {:?}", g.report.lines);
    // A working Constabulary halves it -- and calms a point a turn besides: 3 + 0.5 - 1.0 - 1.5.
    g.state_mut(sid).facilities.push(facility(FacilityKind::Constabulary));
    g.state_mut(sid).unrest = 3.0;
    g.seats[0].stockpile.ducats = 100;
    g.seats[0].allotment = 20;
    g.commit_orders(Seat(0), &[Order::Agitate { state: sid }]);
    g.resolution_phase();
    assert!((g.state(sid).unrest - 1.0).abs() < 1e-9, "half a point through the police, then the falls: {}", g.state(sid).unrest);
}

/// Ticket #269 (version 0.08.4): a computer seat Cold or Hostile toward a holder proposes an
/// Agitate in that holder's Region; toward nobody it resents, none.
#[test]
fn the_computer_agitates_in_the_regions_of_a_rival_it_is_cold_toward() {
    let sid = StateId::Europe;
    let mut g = game();
    calm(&mut g);
    let (agitator, holder) = (Seat(3), Seat(1));
    g.take_control(sid, holder);
    g.seats[agitator.index()].stockpile.ducats = 200;
    g.seats[agitator.index()].allotment = 60;
    // Restive past the second threshold, where one more point matters most and the appetite is
    // at its opportunity multiplier; below that the seat spends its Allotment on places first.
    g.state_mut(sid).unrest = 9.0;
    let orders = g.ai_orders(agitator);
    assert!(!orders.iter().any(|o| matches!(o, Order::Agitate { .. })), "nothing against a holder it does not resent: {orders:?}");
    g.relations.score[agitator.index()][holder.index()] = -8;
    let orders = g.ai_orders(agitator);
    assert!(orders.iter().any(|o| matches!(o, Order::Agitate { state } if *state == sid)), "an Agitate in the resented holder's Region: {orders:?}");
}

// -------------------------------------------- 0.08.4 ticket #268: carbon credits

/// Ticket #268 (version 0.08.4): a carbon credit bought comes off the buyer's Blame ledger for
/// good and off the seller's credit; the Ducats land with the Custodians, at the table price times
/// their view of the buyer -- Hostile refuses -- up to a cap a turn; and a purchase is an act of
/// friendship both ways.
#[test]
fn a_carbon_credit_bought_moves_ppm_off_the_buyers_ledger_and_pays_the_custodians() {
    let mut g = game();
    calm(&mut g);
    let cus = g.credit_seller().expect("the Custodians sit at the table");
    let buyer = Seat::ALL.into_iter().find(|s| *s != cus).unwrap();
    for s in Seat::ALL {
        g.seats[s.index()].blame_emitted = 100.0;
    }
    g.seats[cus.index()].blame_removed = 40.0;
    g.seats[buyer.index()].stockpile.ducats = 100;
    let c = g.tables.carbon_credits.clone();
    assert_eq!((c.price_per_ppm, c.cap_per_turn), (1, 10));
    // Nothing on offer: refused.
    assert!(g.check_order(buyer, &[], &Order::BuyCredits { ppm: 5 }).is_err(), "nothing on offer yet");
    g.seats[cus.index()].credits_offered = 15;
    assert!(g.check_order(buyer, &[], &Order::BuyCredits { ppm: 11 }).is_err(), "over the cap");
    assert!(g.check_order(cus, &[], &Order::BuyCredits { ppm: 5 }).is_err(), "not from oneself");
    assert_eq!(g.credit_cost(buyer, 10), Some(10), "Neutral: the table price");
    g.relations.score[cus.index()][buyer.index()] = 8;
    assert_eq!(g.credit_cost(buyer, 10), Some(5), "Friendly: half");
    // The worst deeds can be: the shared-pot reward lifts a full contributor one point, so -9 reads Cold.
    g.relations.score[cus.index()][buyer.index()] = -10;
    assert_eq!(g.relations_level(cus, buyer), "Hostile", "{}", g.relations_score(cus, buyer));
    assert!(g.check_order(buyer, &[], &Order::BuyCredits { ppm: 5 }).is_err(), "Hostile: the Custodians refuse");
    g.relations.score[cus.index()][buyer.index()] = 0;
    let order = Order::BuyCredits { ppm: 10 };
    assert!(g.check_order(buyer, &[], &order).is_ok());
    assert!(g.check_order(buyer, std::slice::from_ref(&order), &Order::BuyCredits { ppm: 1 }).is_err(), "one purchase a turn");
    let (cus_ducats, buyer_ducats) = (g.seats[cus.index()].stockpile.ducats, g.seats[buyer.index()].stockpile.ducats);
    g.commit_orders(buyer, &[order]);
    assert_eq!(g.seats[buyer.index()].stockpile.ducats, buyer_ducats - 10, "paid at the Orders phase");
    g.resolution_phase();
    assert!((g.blame(buyer) - 90.0).abs() < 1e-9, "10 ppm off the buyer's ledger: {}", g.blame(buyer));
    assert!((g.blame_credit(cus) - 30.0).abs() < 1e-9, "10 ppm off the seller's credit: {}", g.blame_credit(cus));
    assert!((g.blame(cus) - 60.0).abs() < 1e-9, "the seller's own Blame is untouched within its credit: {}", g.blame(cus));
    assert_eq!(g.seats[cus.index()].stockpile.ducats, cus_ducats + 10, "the Ducats land with the Custodians");
    assert!(g.relations.credited[cus.index()][buyer.index()] && g.relations.credited[buyer.index()][cus.index()], "an act of friendship both ways");
    assert!(g.report.lines.iter().any(|l| l.text.contains("carbon credit")), "{:?}", g.report.lines);
    assert_eq!(g.seats[cus.index()].credits_offered, 15, "the offer stands until changed");
    g.resolution_phase();
    assert!((g.blame(buyer) - 90.0).abs() < 1e-9, "for good");
}

/// Ticket #268 (version 0.08.4): the Custodians may sell more than they hold in credit -- the
/// excess goes onto their own ledger as Blame taken -- and an offer is shared first come first
/// served, a buyer left short getting its Ducats back.
#[test]
fn overselling_carbon_credits_puts_the_excess_on_the_custodians_ledger_and_a_short_buyer_is_refunded() {
    let mut g = game();
    calm(&mut g);
    let cus = g.credit_seller().unwrap();
    let others: Vec<Seat> = Seat::ALL.into_iter().filter(|s| *s != cus).collect();
    let (a, b) = (others[0], others[1]);
    for s in Seat::ALL {
        g.seats[s.index()].blame_emitted = 100.0;
        g.seats[s.index()].stockpile.ducats = 100;
    }
    g.seats[cus.index()].blame_removed = 5.0;
    g.seats[cus.index()].credits_offered = 12;
    g.commit_orders(a, &[Order::BuyCredits { ppm: 10 }]);
    g.commit_orders(b, &[Order::BuyCredits { ppm: 10 }]);
    g.resolution_phase();
    assert!((g.blame(a) - 90.0).abs() < 1e-9, "the first buyer got its ten");
    assert_eq!(g.blame_credit(cus), 0.0, "the seller's five of credit are gone");
    assert!((g.blame(b) - 98.0).abs() < 1e-9, "the second buyer got the two left: {}", g.blame(b));
    assert!((g.blame(cus) - 102.0).abs() < 1e-9, "twelve sold against five held: seven oversold are Blame taken, 100 - 5 + 7: {}", g.blame(cus));
    assert_eq!(g.seats[b.index()].stockpile.ducats, 98, "paid ten, eight back for the eight it did not get");
    assert!(g.report.lines.iter().any(|l| l.text.contains("came back")), "{:?}", g.report.lines);
}

/// Ticket #268 (version 0.08.4): the computer Custodians offer their credit while their own share
/// is under the fair quarter and refuse when it is not; a computer seat above a fair share with an
/// offer standing buys.
#[test]
fn the_computer_custodians_offer_their_credit_when_clean_and_a_dirty_seat_buys() {
    let mut g = game();
    calm(&mut g);
    let cus = g.credit_seller().unwrap();
    for s in Seat::ALL {
        g.seats[s.index()].blame_emitted = 100.0;
        g.seats[s.index()].stockpile.ducats = 100;
    }
    g.seats[cus.index()].blame_removed = 30.0;
    assert!(g.blame_share(cus) < g.tables.influence.blame.fair_share);
    let orders = g.ai_orders(cus);
    assert!(orders.iter().any(|o| matches!(o, Order::OfferCredits { ppm } if *ppm == 30)), "clean: the whole credit on offer: {orders:?}");
    g.seats[cus.index()].blame_emitted = 900.0;
    g.seats[cus.index()].credits_offered = 30;
    let orders = g.ai_orders(cus);
    assert!(orders.iter().any(|o| matches!(o, Order::OfferCredits { ppm } if *ppm == 0)), "dirty: it withdraws the offer: {orders:?}");
    // A dirty rival buys while the offer stands.
    g.seats[cus.index()].blame_emitted = 100.0;
    let dirty = Seat::ALL.into_iter().find(|s| *s != cus).unwrap();
    g.seats[dirty.index()].blame_emitted = 600.0;
    assert!(g.blame_share(dirty) > 0.5);
    let orders = g.ai_orders(dirty);
    assert!(orders.iter().any(|o| matches!(o, Order::BuyCredits { ppm } if *ppm > 0)), "a dirty seat buys: {orders:?}");
}

// -------------------------------------------- 0.08.4 ticket #267: the Smear campaign

/// Ticket #267 (version 0.08.4): a Smear lays ppm on a rival's Blame ledger for good, at the
/// table's rate per Influence, moving the share every rule reads; one a turn per target, never
/// on oneself; an offence the Report names.
#[test]
fn a_smear_campaign_lays_ppm_on_a_rivals_blame_ledger_and_is_an_offence() {
    let mut g = game();
    calm(&mut g);
    let target = Seat(1);
    for s in Seat::ALL {
        g.seats[s.index()].blame_emitted = 100.0;
        g.seats[s.index()].blame_removed = 0.0;
    }
    assert!((g.blame_share(target) - 0.25).abs() < 1e-9, "an even quarter each to begin");
    g.seats[0].allotment = 20;
    assert!(g.check_order(Seat(0), &[], &Order::Smear { target: Seat(0), amount: 5 }).is_err(), "not oneself");
    assert!(g.check_order(Seat(0), &[], &Order::Smear { target, amount: 0 }).is_err(), "a positive amount");
    assert!(g.check_order(Seat(0), &[], &Order::Smear { target, amount: 10 }).is_ok());
    let first = Order::Smear { target, amount: 10 };
    assert!(g.check_order(Seat(0), std::slice::from_ref(&first), &Order::Smear { target, amount: 5 }).is_err(), "one a turn per target");
    assert!(g.check_order(Seat(0), std::slice::from_ref(&first), &Order::Smear { target: Seat(2), amount: 5 }).is_ok(), "another target is another campaign");
    g.commit_orders(Seat(0), &[first]);
    g.resolution_phase();
    let rate = g.tables.influence.smear.ppm_per_influence;
    assert!((g.seats[target.index()].blame_smeared - 10.0 * rate).abs() < 1e-9, "20 ppm laid on: {}", g.seats[target.index()].blame_smeared);
    assert!((g.blame(target) - (100.0 + 10.0 * rate)).abs() < 1e-9, "the ledger counts it");
    assert!(g.blame_share(target) > 0.25, "and the share the rules read has moved: {}", g.blame_share(target));
    assert!(g.relations.offended[target.index()][0], "an offence against the target");
    assert!(g.report.lines.iter().any(|l| l.text.contains("smeared")), "the Report names who paid: {:?}", g.report.lines);
    // Permanent: nothing takes it back off.
    g.resolution_phase();
    assert!((g.seats[target.index()].blame_smeared - 10.0 * rate).abs() < 1e-9, "for good");
}

/// Ticket #277 (version 0.08.5): a Greenwash takes ppm off the seat's own Blame ledger for good,
/// at the table's rate per Influence, with a Ducat beside every point; one a turn; the whole
/// ledger and never below nought; public and no offence.
#[test]
fn a_greenwash_takes_ppm_off_your_own_ledger_for_influence_and_ducats_and_offends_nobody() {
    let mut g = game();
    calm(&mut g);
    for s in Seat::ALL {
        g.seats[s.index()].blame_emitted = 100.0;
        g.seats[s.index()].blame_removed = 0.0;
    }
    g.seats[0].allotment = 20;
    g.seats[0].stockpile.ducats = 12;
    let t = g.tables.influence.greenwash.clone();
    assert!((t.ppm_per_influence - 2.0).abs() < 1e-9 && t.ducats_per_influence == 1, "two ppm and one Ducat per Influence");
    assert!(g.check_order(Seat(0), &[], &Order::Greenwash { amount: 0 }).is_err(), "a positive amount");
    assert!(g.check_order(Seat(0), &[], &Order::Greenwash { amount: 15 }).is_err(), "15 Influence wants 15 Ducats and the seat has 12");
    let first = Order::Greenwash { amount: 10 };
    assert!(g.check_order(Seat(0), &[], &first).is_ok());
    assert_eq!(g.order_cost(Seat(0), &first).ducats, 10, "a Ducat beside every point");
    assert!(g.check_order(Seat(0), std::slice::from_ref(&first), &Order::Greenwash { amount: 2 }).is_err(), "one a turn");
    g.commit_orders(Seat(0), &[first]);
    g.resolution_phase();
    assert!((g.seats[0].blame_cleaned - 20.0).abs() < 1e-9, "20 ppm cleaned: {}", g.seats[0].blame_cleaned);
    assert!((g.blame(Seat(0)) - 80.0).abs() < 1e-9, "off the whole ledger: {}", g.blame(Seat(0)));
    assert_eq!(g.seats[0].stockpile.ducats, 2, "and the Ducats are spent");
    assert!(g.blame_share(Seat(0)) < 0.25, "the share the rules read has moved: {}", g.blame_share(Seat(0)));
    assert!(Seat::ALL.iter().all(|s| !g.relations.offended[s.index()][0]), "no offence against anybody");
    assert!(g.report.lines.iter().any(|l| l.text.contains("greenwashed")), "the Report says so, in public: {:?}", g.report.lines);
    // Never below nought: a campaign past the ledger clears it and no more.
    g.seats[0].blame_cleaned = 500.0;
    assert!((g.blame(Seat(0)) - 0.0).abs() < 1e-9, "floored at nought");
}

/// Ticket #277 (version 0.08.5): a computer seat whose own Blame share stands above the fair quarter
/// greenwashes when carbon credits are not to be had and it holds Ducats past the price; when the
/// Custodians are offering and will sell, it buys credits instead; clean, it does neither.
#[test]
fn the_computer_greenwashes_when_dirty_and_credits_are_not_to_be_had() {
    let mut g = game();
    calm(&mut g);
    let seat = Seat(1);
    for s in Seat::ALL {
        g.seats[s.index()].blame_emitted = 100.0;
        g.seats[s.index()].blame_removed = 0.0;
    }
    g.seats[seat.index()].blame_emitted = 400.0;
    g.seats[seat.index()].allotment = 20;
    g.seats[seat.index()].stockpile.ducats = 200;
    for s in Seat::ALL {
        g.seats[s.index()].credits_offered = 0;
    }
    let orders = g.ai_orders(seat);
    assert!(orders.iter().any(|o| matches!(o, Order::Greenwash { .. })), "dirty, rich, no credits on offer: it greenwashes: {orders:?}");
    // The Custodians offering, and Neutral toward it: it buys credits and does not greenwash.
    let seller = g.credit_seller().expect("the Custodians sit at the table");
    g.seats[seller.index()].credits_offered = 10;
    g.seats[seller.index()].blame_removed = 50.0;
    let orders = g.ai_orders(seat);
    assert!(orders.iter().any(|o| matches!(o, Order::BuyCredits { .. })), "credits to be had: it buys: {orders:?}");
    assert!(!orders.iter().any(|o| matches!(o, Order::Greenwash { .. })), "and does not greenwash beside them: {orders:?}");
    // Clean, it does neither.
    g.seats[seat.index()].blame_emitted = 10.0;
    let orders = g.ai_orders(seat);
    assert!(!orders.iter().any(|o| matches!(o, Order::Greenwash { .. } | Order::BuyCredits { .. })), "clean: nothing: {orders:?}");
}

/// Ticket #267 (version 0.08.4): a computer seat Cold or Hostile toward a rival whose Blame share
/// stands above the fair quarter proposes a Smear against it; toward nobody it hates, none.
#[test]
fn the_computer_smears_a_dirty_rival_it_is_cold_toward() {
    let mut g = game();
    calm(&mut g);
    let (smearer, dirty) = (Seat(3), Seat(1));
    for s in Seat::ALL {
        g.seats[s.index()].blame_emitted = 50.0;
    }
    g.seats[dirty.index()].blame_emitted = 500.0;
    assert!(g.blame_share(dirty) > 0.5);
    g.seats[smearer.index()].allotment = 20;
    let orders = g.ai_orders(smearer);
    assert!(!orders.iter().any(|o| matches!(o, Order::Smear { .. })), "nothing against a rival it does not resent: {orders:?}");
    g.relations.score[smearer.index()][dirty.index()] = -8;
    assert!(g.relations_score(smearer, dirty) <= -5, "Cold or worse: {}", g.relations_score(smearer, dirty));
    let orders = g.ai_orders(smearer);
    assert!(orders.iter().any(|o| matches!(o, Order::Smear { target, amount } if *target == dirty && *amount > 0)), "a Smear against the dirty rival it is Cold toward: {orders:?}");
}

// -------------------------------------------- 0.08.4 ticket #266: Blame moderates decay

/// Ticket #266 (version 0.08.4): on a Region a Faction does not hold, its Standing decays 3 a turn
/// when its Blame share stands at or above a half, 1 when at or below an eighth, and 2 between;
/// a held place keeps its 1 and a Colony its 2 whatever the share.
#[test]
fn blame_moderates_the_decay_of_a_standing_on_regions_a_faction_does_not_hold() {
    let mut g = game();
    calm(&mut g);
    let t = g.tables.influence.blame.clone();
    assert_eq!((t.decay_slow_below, t.decay_fast_from, t.decay_slow, t.decay_fast), (0.125, 0.5, 1, 3));
    let (dirty, clean, plain) = (Seat(1), Seat(2), Seat(3));
    // Shares: seat 1 at 0.6, seat 2 at 0.05, seat 3 at 0.3, seat 0 the rest.
    for (s, ppm) in [(Seat(0), 5.0), (dirty, 60.0), (clean, 5.0), (plain, 30.0)] {
        g.seats[s.index()].blame_emitted = ppm;
        g.seats[s.index()].blame_removed = 0.0;
    }
    assert!(g.blame_share(dirty) >= 0.5 && g.blame_share(clean) <= 0.125 && g.blame_share(plain) > 0.125 && g.blame_share(plain) < 0.5);
    let region = Place::State(StateId::NorthAfrica);
    let held = Place::State(StateId::Europe);
    g.take_control(StateId::Europe, Seat(0));
    g.take_control(StateId::NorthAfrica, Seat(0));
    let c = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Habitat], 4);
    let col = Place::Colony(c);
    for s in [dirty, clean, plain] {
        g.seat_mut(s).influence.insert(region, 30);
        g.seat_mut(s).influence.insert(col, 30);
        g.seat_mut(s).influenced_this_turn.clear();
    }
    g.seat_mut(Seat(0)).influence.insert(held, 30);
    g.seat_mut(Seat(0)).influenced_this_turn.clear();
    assert_eq!(g.standing_decay_for(dirty, region), 3, "dirty, elsewhere");
    assert_eq!(g.standing_decay_for(clean, region), 1, "clean, elsewhere");
    assert_eq!(g.standing_decay_for(plain, region), 2, "between, elsewhere");
    assert_eq!(g.standing_decay_for(dirty, col), 2, "a Colony is untouched");
    assert_eq!(g.standing_decay_for(Seat(0), held), 1, "a held place keeps its 1");
    g.resolution_phase();
    assert_eq!(g.seat(dirty).influence[&region], 27, "3 off a dirty seat's Standing on a Region it does not hold");
    assert_eq!(g.seat(clean).influence[&region], 29, "1 off a clean seat's");
    assert_eq!(g.seat(plain).influence[&region], 28, "2 off everyone between");
    assert_eq!(g.seat(dirty).influence[&col], 28, "2 off on a Colony, however dirty");
    assert_eq!(g.seat(Seat(0)).influence[&held], 29, "1 off on a held place");
}

// -------------------------------------------- 0.08.4 ticket #265: Blame and Blame credit

/// Ticket #265 (version 0.08.4): Blame credit is what a Faction has REMOVED, not what it removed
/// beyond everything it emitted -- a figure measured at zero in ten of ten games. A seat that
/// emitted 100 and removed 40 is answerable for 60 and holds 40 in credit.
#[test]
fn blame_credit_is_what_a_faction_has_removed() {
    let mut g = game();
    g.seats[0].blame_emitted = 100.0;
    g.seats[0].blame_removed = 40.0;
    assert!((g.blame(Seat(0)) - 60.0).abs() < 1e-9, "answerable for what it emitted less what it removed");
    assert!((g.blame_credit(Seat(0)) - 40.0).abs() < 1e-9, "and holds what it removed in credit: {}", g.blame_credit(Seat(0)));
    g.seats[0].blame_removed = 0.0;
    assert_eq!(g.blame_credit(Seat(0)), 0.0, "nothing removed, no credit");
}

/// Ticket #265 (version 0.08.4): what the Custodians' Research Directive has added to the Natural
/// Sink counts as their removal at every Climate phase -- "credit", the designer said -- so a
/// Custodian who has bought half a ppm of Sink is credited half a ppm a turn from then on.
#[test]
fn the_custodians_directive_into_the_sink_counts_as_removal_every_climate_phase() {
    let mut g = game();
    calm(&mut g);
    quiet_world(&mut g);
    let cus = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Custodians).unwrap();
    g.seats[cus.index()].directive_sink = 0.5;
    let before = g.seats[cus.index()].blame_removed;
    g.climate_phase();
    assert!((g.seats[cus.index()].blame_removed - before - 0.5).abs() < 1e-9, "half a ppm credited: {} -> {}", before, g.seats[cus.index()].blame_removed);
    g.climate_phase();
    assert!((g.seats[cus.index()].blame_removed - before - 1.0).abs() < 1e-9, "and again the next phase, for good");
    // A directive of 10 Research at the card's rate buys that much Sink and that much standing credit.
    let rate = g.tables.research_directive.custodians_ppm_per_point;
    g.seats[cus.index()].research_directive = 50;
    let sink_before = g.climate.natural_sink;
    let bought = g.spend_research_directive(cus, 20);
    assert_eq!(bought, 10, "half of 20 directed");
    assert!((g.climate.natural_sink - sink_before - 10.0 * rate).abs() < 1e-9);
    assert!((g.seats[cus.index()].directive_sink - 0.5 - 10.0 * rate).abs() < 1e-9, "the enlargement is remembered as theirs: {}", g.seats[cus.index()].directive_sink);
}

// -------------------------------------------- 0.08.4 ticket #264: the Victory history

/// Ticket #264 (version 0.08.4): after every Climate phase each seat gets a record -- its score,
/// its Blame share, and the three tick flags -- on the same turn as the Emissions record, so the
/// Victory history and the Emissions history share an axis.
#[test]
fn the_victory_history_is_written_for_every_seat_after_each_climate_phase() {
    let mut g = game();
    calm(&mut g);
    assert!(Seat::ALL.iter().all(|s| g.seat(*s).victory_history.is_empty()), "nothing before the first phase");
    g.climate_phase();
    for s in Seat::ALL {
        let h = &g.seat(s).victory_history;
        assert_eq!(h.len(), 1, "{s:?}: one record after one phase");
        let r = &h[0];
        assert_eq!(r.turn, g.climate.history[g.climate.history.len() - 1].turn, "the Emissions record's turn");
        assert!((r.score - g.progress(s).score()).abs() < 1e-9, "{s:?}: the score as the window prints it");
        assert!((r.blame_share - g.blame_share(s)).abs() < 1e-9, "{s:?}: the share the rules read");
        assert_eq!(r.antarctica_open, g.antarctica_open);
        let gate = g.tables.victory_gate(g.kind(s)).map(|t| g.has_tech(t)).unwrap_or(true);
        assert_eq!(r.gate_done, gate, "{s:?}");
        assert_eq!(r.archive_complete, g.archive_complete(s), "{s:?}");
    }
    g.climate_phase();
    assert_eq!(g.seat(Seat(0)).victory_history.len(), 2, "one a phase");
}

// -------------------------------------------- 0.08.4 ticket #263: what a Faction has under way

/// Ticket #263 (version 0.08.4): a seat's builds begun and Ships in transit, with turns, soonest
/// first; a build is the seat's that ordered it; a Ship at a Body is not in transit.
#[test]
fn what_a_faction_has_under_way_lists_its_builds_and_transits_soonest_first() {
    let sid = StateId::Europe;
    let mut g = game();
    calm(&mut g);
    directed(&mut g, sid);
    assert_eq!(g.under_way(Seat(0)), UnderWay::default(), "nothing under way at the start");
    let turn = g.turn;
    // A Factory in Europe landing in three turns, then a Module at a Colony landing next turn.
    g.state_mut(sid).queue.push(Build { item: BuildItem::Facility(FacilityKind::Factory), seat: Seat(0), due_turn: turn + 2, coastal: false });
    let cid = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Habitat], 4);
    g.colony_mut(cid).unwrap().queue.push(Build { item: BuildItem::Module(ModuleKind::Mine), seat: Seat(0), due_turn: turn, coastal: false });
    // A rival's build in a Region the player directs is the rival's, not the player's.
    g.state_mut(sid).queue.push(Build { item: BuildItem::Facility(FacilityKind::Bank), seat: Seat(1), due_turn: turn + 1, coastal: false });
    // Two Ships: one on the road to Mars with three turns left, one arriving next turn, and one at rest.
    let put = |g: &mut Game, kind: UnitKind| -> ShipId {
        let id = ShipId(g.fresh_id());
        let name = g.next_ship_name(kind);
        g.ships.push(Ship { name, id, kind, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
        id
    };
    let far = put(&mut g, UnitKind::Frigate);
    let near = put(&mut g, UnitKind::ColonyShip);
    let _rest = put(&mut g, UnitKind::Carrier);
    g.ships.iter_mut().find(|s| s.id == far).unwrap().at = ShipAt::Transit { from: BodyId::Earth, to: BodyId::Mars, turns_left: 3 };
    g.ships.iter_mut().find(|s| s.id == near).unwrap().at = ShipAt::Transit { from: BodyId::Moon, to: BodyId::Earth, turns_left: 1 };
    let u = g.under_way(Seat(0));
    assert_eq!(u.builds.len(), 2, "the rival's Bank is not the player's: {:?}", u.builds);
    assert_eq!(u.builds[0], ("Mine".to_string(), Place::Colony(cid), 1), "soonest first: {:?}", u.builds);
    assert_eq!(u.builds[1], ("Factory".to_string(), Place::State(sid), 3), "{:?}", u.builds);
    assert_eq!(u.transits.len(), 2, "a Ship at rest is not in transit: {:?}", u.transits);
    assert_eq!(u.transits[0].3, 1, "soonest first: {:?}", u.transits);
    assert_eq!((u.transits[0].1.as_str(), u.transits[0].2.as_str()), ("the Moon", "Earth"));
    assert_eq!(u.transits[1].3, 3);
    assert!(u.transits[1].0.contains("TSV "), "the name carries the flag's prefix: {}", u.transits[1].0);
    assert_eq!(g.under_way(Seat(1)).builds.len(), 1, "the rival sees its own Bank");
}

// -------------------------------------------- 0.08.4 ticket #262: the challenger line

/// Ticket #262 (version 0.08.4): the rival named on a held place's card is the one NEAREST ITS OWN
/// PRICE, not the one with the highest Standing -- they differ when Blame or Relations move one
/// rival's price and not another's, and the nearer one takes the place first. None where nobody
/// holds the place or no rival has a Standing.
#[test]
fn the_challenger_line_names_the_rival_nearest_its_own_price() {
    let sid = StateId::Europe;
    let place = Place::State(sid);
    let mut g = game();
    calm(&mut g);
    directed(&mut g, sid);
    // Every seat starts with a Standing on its own start state (ticket #75); cleared, so the place
    // begins with nobody standing on it.
    for s in Seat::ALL {
        g.seat_mut(s).influence.remove(&place);
    }
    assert_eq!(g.nearest_challenger(place), None, "no rival has a Standing yet");
    let (a, b) = (Seat(1), Seat(2));
    // Seat 1 stands higher but pays a Blame-raised threshold; seat 2 stands lower and clean.
    g.seats[a.index()].blame_emitted = 900.0;
    for s in [Seat(0), b, Seat(3)] {
        g.seats[s.index()].blame_emitted = 10.0;
    }
    assert!(g.blame_threshold_multiplier(a) > 1.2, "{}", g.blame_threshold_multiplier(a));
    g.seat_mut(Seat(0)).influence.insert(place, 10);
    let price_a = g.influence_needed_for(a, place);
    let price_b = g.influence_needed_for(b, place);
    assert!(price_a > price_b, "Blame makes seat 1's price the dearer: {price_a} against {price_b}");
    g.seat_mut(a).influence.insert(place, price_b - 5); // the higher Standing, the farther from its own price
    g.seat_mut(b).influence.insert(place, price_b - 8); // the lower Standing, the nearer
    assert!(price_a - (price_b - 5) > 8, "seat 1 is farther off than seat 2's 8");
    let (who, standing, price) = g.nearest_challenger(place).expect("a rival stands here");
    assert_eq!((who, standing, price), (b, price_b - 8, price_b), "the nearer rival, not the higher");
    // A place nobody holds has no challenger line.
    let nobody = StateId::ALL.into_iter().find(|s| matches!(g.state(*s).control, Control::Neutral)).expect("a neutral Region");
    assert_eq!(g.nearest_challenger(Place::State(nobody)), None);
}

// -------------------------------------------- 0.08.4 ticket #261: the rival's Moment

/// Ticket #261 (version 0.08.4): a rival crossing three quarters of the way to its Victory
/// Condition, or meeting one part of it with the other short, interrupts the player once a step.
/// The player's own seat never does; rank 5, between a Battle and a Tech.
#[test]
fn a_rival_closing_on_its_victory_condition_interrupts_the_player_once_a_step() {
    let mut g = game();
    calm(&mut g);
    let pro = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Prospectors).unwrap();
    assert_ne!(pro, Seat(0), "the Prospectors are a rival in this game");
    let fired = |g: &Game| g.report.moments.iter().filter(|m| m.kind == MomentKind::RivalProgress).count();
    // Presence 9 of 12 (0.75) and the Fund at 2000 of 2500 (0.8): the score is 0.75.
    colony(&mut g, pro, BodyId::Moon, &[ModuleKind::Habitat, ModuleKind::Habitat, ModuleKind::Habitat], 9);
    g.seats[pro.index()].venture_fund = 2000;
    assert!((g.progress(pro).score() - 0.75).abs() < 1e-9, "{}", g.progress(pro).score());
    g.end_phase();
    assert_eq!(fired(&g), 1, "three quarters: the Moment fires: {:?}", g.report.moments);
    let m = g.report.moments.iter().find(|m| m.kind == MomentKind::RivalProgress).unwrap();
    assert!(m.text.contains("Prospectors") && m.text.contains("three quarters"), "{}", m.text);
    assert!(m.text.contains("9 of 12"), "the part still short is the figure: {}", m.text);
    assert_eq!(m.figure, "9 of 12");
    g.end_phase();
    assert_eq!(fired(&g), 1, "once: it does not fire again while the seat sits there");
    // The Fund full but its gate shut is held back, not met.
    g.seats[pro.index()].venture_fund = 2500;
    g.end_phase();
    assert_eq!(fired(&g), 1, "a part held back by its gate Tech is not a part met");
    // The gate open: one part met, the other short.
    open_gates(&mut g);
    g.end_phase();
    assert_eq!(fired(&g), 2, "one part met: the second step fires: {:?}", g.report.moments);
    let m = g.report.moments.iter().rev().find(|m| m.kind == MomentKind::RivalProgress).unwrap();
    assert!(m.text.contains("met half") && m.text.contains("Venture Capital Fund") && m.text.contains("9 of 12"), "{}", m.text);
    g.end_phase();
    assert_eq!(fired(&g), 2, "once");
    assert!(g.outcome.is_none(), "nobody has won");
    // The player's own seat, at the same steps, never interrupts itself.
    colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Habitat, ModuleKind::Habitat, ModuleKind::Habitat], 9);
    g.seats[0].stabilization_run = 3;
    g.end_phase();
    assert_eq!(fired(&g), 2, "rivals only: {:?}", g.report.moments);
    assert_eq!(MomentKind::RivalProgress.rank(), 5, "between a Battle (4) and a Tech (6)");
    assert_eq!(MomentKind::ALL.len(), 10, "ticket #281 (version 0.08.5) added a place taken by force");
    assert!(g.tables.report.moment_on(MomentKind::RivalProgress), "on by default");
}

// -------------------------------------------- 0.08.4 ticket #260: a motto on every card

/// Ticket #260 (version 0.08.4): every Faction's card carries a motto, one line, its own. A guard,
/// since a motto moves no mechanic and no other test would notice one going missing or two cards
/// sharing one.
#[test]
fn every_faction_card_carries_a_motto_of_its_own() {
    let g = game();
    let mut seen = std::collections::BTreeSet::new();
    for kind in FactionKind::ALL {
        let c = g.tables.faction(kind);
        assert!(!c.motto.trim().is_empty(), "{kind:?} has no motto");
        assert!(c.motto.len() <= 48, "{kind:?}'s motto is a sentence, not a paragraph: {}", c.motto);
        assert_ne!(c.motto, c.blurb, "{kind:?}'s motto restates its blurb");
        assert!(seen.insert(c.motto.clone()), "{kind:?} shares its motto with another Faction");
    }
}

// -------------------------------------------- 0.08.3 ticket #235: the Research Directive

/// Ticket #235 (version 0.08.3): every Faction may send a share of its Research somewhere other
/// than the shared Tech, and each of the four goes somewhere different.
///
/// The rates were fitted against a measurement, not guessed: Research made over a whole game,
/// median by Faction over 20 seeds, is Custodians 131, Prospectors 442, Arkwrights 149,
/// Archivists 453. Half a turn's Research is therefore about 1.8 points for the Custodians and 6.2
/// for the Prospectors -- far less than the rule was first drafted against, which is why the
/// Custodians' rate came down from 0.05 ppm a point to 0.01 and why the Prospectors' "10-1" had to
/// be settled at 0.8 rather than either of the readings it could bear.
#[test]
fn a_research_directive_sends_a_share_of_the_turns_research_somewhere_else() {
    let seat_of = |g: &Game, k: FactionKind| Seat::ALL.into_iter().find(|s| g.kind(*s) == k).expect("every Faction is seated");
    // A fresh game makes no Research at all, so every arm below needs a Lab before there is
    // anything to direct. The Archive test has needed the same since version 0.07.0.
    let with_lab = |g: &mut Game, seat: Seat| {
        g.state_mut(StateId::Europe).control = Control::Controlled(seat);
        g.state_mut(StateId::Europe).facilities.push(facility(FacilityKind::ResearchLab));
        g.pick_tech(Seat(0), TechId::PublicScience).ok();
    };

    // The Custodians: the Natural Sink itself moves, and it STAYS moved.
    let mut g = game();
    let cus = seat_of(&g, FactionKind::Custodians);
    with_lab(&mut g, cus);
    let before_sink = g.climate.natural_sink;
    g.seats[cus.index()].research_directive = 50;
    g.income_phase();
    let made = g.seats[cus.index()].research_last_turn;
    let want = made * 50 / 100;
    assert!(made > 0, "the seat made Research to direct");
    let rate = g.tables.research_directive.custodians_ppm_per_point;
    assert!((g.climate.natural_sink - (before_sink + want as f64 * rate)).abs() < 1e-9, "the Sink took {want} points at {rate} a point: {} -> {}", before_sink, g.climate.natural_sink);
    let held = g.climate.natural_sink;
    g.income_phase();
    assert!(g.climate.natural_sink > held, "and it is PERMANENT -- a second turn adds again rather than replacing");

    // The Prospectors: Ducats, at a fractional rate that carries rather than flooring away.
    // Against a CONTROL run: Income pays ordinary Ducat income too, so the directive's share is
    // the difference between two identical games, one directing and one not.
    let ducats_at = |percent: u8| -> (i64, i64, f64) {
        let mut g = game();
        let pro = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Prospectors).unwrap();
        g.state_mut(StateId::Europe).control = Control::Controlled(pro);
        g.state_mut(StateId::Europe).facilities.push(facility(FacilityKind::ResearchLab));
        g.pick_tech(Seat(0), TechId::PublicScience).ok();
        g.seats[pro.index()].research_directive = percent;
        g.income_phase();
        (g.seat(pro).stockpile.ducats, g.seats[pro.index()].research_last_turn, g.seat(pro).directive_remainder)
    };
    let (plain, made, _) = ducats_at(0);
    let (directed, made2, carried) = ducats_at(50);
    assert_eq!(made, made2, "the same game either way");
    let want = made * 50 / 100;
    let earned = want as f64 * game().tables.research_directive.prospectors_ducats_per_point;
    assert!(want > 0, "{made} Research, half of it directed");
    assert_eq!(directed - plain, earned.floor() as i64, "{want} points at 0.8 is {earned}, paid whole");
    assert!((carried - (earned - earned.floor())).abs() < 1e-9, "and the fraction is carried, not lost");

    // The Arkwrights: Fuel, same carry.
    let fuel_at = |percent: u8| -> (i64, i64) {
        let mut g = game();
        let ark = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Arkwrights).unwrap();
        g.state_mut(StateId::Europe).control = Control::Controlled(ark);
        g.state_mut(StateId::Europe).facilities.push(facility(FacilityKind::ResearchLab));
        g.pick_tech(Seat(0), TechId::PublicScience).ok();
        g.seats[ark.index()].research_directive = percent;
        g.income_phase();
        (g.seat(ark).stockpile.fuel, g.seats[ark.index()].research_last_turn)
    };
    let (plain, made) = fuel_at(0);
    let (directed, _) = fuel_at(50);
    let want = made * 50 / 100;
    let earned = want as f64 * game().tables.research_directive.arkwrights_fuel_per_point;
    assert_eq!(directed - plain, earned.floor() as i64, "{want} points at 0.2 is {earned}");
}

/// Ticket #235: what is directed never reaches the shared Tech, so it counts nothing toward the
/// Research Lead. This is the cost that makes the directive a decision rather than free money, and
/// it is the same rule the Archive fund has had since version 0.07.0.
#[test]
fn directed_research_never_reaches_the_shared_tech() {
    let mut g = game();
    let cus = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Custodians).unwrap();
    g.state_mut(StateId::Europe).control = Control::Controlled(cus);
    g.state_mut(StateId::Europe).facilities.push(facility(FacilityKind::ResearchLab));
    g.pick_tech(Seat(0), TechId::PublicScience).ok();
    g.research.contributions = [0; 4];
    g.seats[cus.index()].research_directive = 50;
    g.income_phase();
    let made = g.seats[cus.index()].research_last_turn;
    let directed = made * 50 / 100;
    assert_eq!(g.research.contributions[cus.index()], made - directed, "only what was not directed reached the Tech");
}

/// Ticket #235: Provisional Findings was BINARY because its control was a switch -- any funding at
/// all turned it off. With a slider it takes a threshold, at the designer's word "make it a
/// threshold 75%": the rule holds while at least that share still goes to the shared Tech.
///
/// Both of the old positions are unchanged, which is the point of choosing a threshold over a
/// scaling rule: 0 keeps it and 100 loses it, exactly as the switch did.
#[test]
fn provisional_findings_holds_while_three_quarters_still_goes_to_the_tech() {
    let floor = game().tables.research_directive.provisional_min_contribution;
    assert_eq!(floor, 75, "the threshold the rest of this test is written against");
    for (directive, expected) in [(0u8, true), (25, true), (26, false), (50, false), (100, false)] {
        let mut g = game();
        let arc = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Archivists).unwrap();
        // Enough Research that the share is not lost to rounding, and room in the fund for it.
        g.state_mut(StateId::Europe).control = Control::Controlled(arc);
        for _ in 0..4 {
            g.state_mut(StateId::Europe).facilities.push(facility(FacilityKind::ResearchLab));
        }
        g.pick_tech(Seat(0), TechId::PublicScience).ok();
        g.seats[arc.index()].research_directive = directive;
        g.income_phase();
        g.income_phase();
        assert_eq!(
            g.provisional_findings(arc),
            expected,
            "a directive of {directive} should leave Provisional Findings {expected}"
        );
    }
}

// ------------------------------------------------- 0.08.3 ticket #236: the shared pot

/// Ticket #236 (version 0.08.3): what everyone makes of how much of its Research a Faction gives
/// the shared Tech. A TERM and not a deed, at the designer's word "plus/minus 1 but only for that
/// turn" -- read afresh every time, gone the moment they contribute again.
///
/// The shape matters and was chosen against a measurement. With the Research Directive shipped and
/// the AI going to its cap on turn 2 -- 24 to 28 turns of 36 below any threshold -- a PERMANENT
/// -1 against every rival every turn would have been about 78 points of damage a seat over a
/// game, on a scale that bottoms at -10.
#[test]
fn the_shared_pot_is_a_term_read_afresh_and_never_banked() {
    let mut g = game();
    let (viewer, subject) = (Seat(0), Seat(1));
    let c = &g.tables.relations;
    let (floor, step) = (c.directive_min_contribution, c.directive_step);
    assert_eq!((floor, step), (85, 1), "the figures the rest of this test is written against");

    // Contributing all of it: the reward. This is also the state every game OPENS in.
    g.seats[subject.index()].research_directive = 0;
    assert_eq!(g.directive_relations_term(subject), 1, "all of it pays");
    assert_eq!(g.relations_score(viewer, subject), 1);

    // Above the line but short of all of it: nothing either way.
    g.seats[subject.index()].research_directive = 15;
    assert_eq!(g.directive_relations_term(subject), 0, "90% contributed is past the line and short of the reward");
    assert_eq!(g.relations_score(viewer, subject), 0);

    // Below the line: the penalty, and it is gone again the moment they contribute.
    g.seats[subject.index()].research_directive = 16;
    assert_eq!(g.directive_relations_term(subject), -1, "84% contributed is below the line");
    g.seats[subject.index()].research_directive = 50;
    assert_eq!(g.directive_relations_term(subject), -1, "and no worse for being far below it");
    assert_eq!(g.relations_deeds(viewer, subject), 0, "NOTHING is banked: the deeds figure never moved");
    g.seats[subject.index()].research_directive = 0;
    assert_eq!(g.relations_score(viewer, subject), 1, "forgiven the same turn they contribute again");
}

/// Ticket #236: the reward may not lift a pair past the top of Cordial, the step above Neutral, at
/// the designer's word. Version 0.08.2 settled that a pair which never strikes an Accord can never
/// rise above Neutral; this bends that by one band rather than breaking it.
#[test]
fn the_shared_pots_reward_stops_at_the_top_of_cordial() {
    let mut g = game();
    let (viewer, subject) = (Seat(0), Seat(1));
    let ceiling = g.tables.relations.directive_boost_ceiling;
    assert_eq!(ceiling, 6, "the top of Cordial");
    g.seats[subject.index()].research_directive = 0;

    g.relations.score[viewer.index()][subject.index()] = ceiling;
    assert_eq!(g.relations_score(viewer, subject), ceiling, "the reward adds nothing at the ceiling");
    assert_eq!(g.relations_level(viewer, subject), "Cordial");

    g.relations.score[viewer.index()][subject.index()] = ceiling - 1;
    assert_eq!(g.relations_score(viewer, subject), ceiling, "and only up to it from below");

    // A pair already higher by DEEDS is not dragged down to the ceiling: it binds the boost only.
    g.relations.score[viewer.index()][subject.index()] = 9;
    assert_eq!(g.relations_score(viewer, subject), 9, "Friendly by deeds stays Friendly");
}

/// Ticket #236: the penalty is not an offence. It must not feed the scar ratchet, at the
/// designer's word -- "this doesn't count towards scar" -- because a Faction spending its own
/// Research on its own business has done nothing to anybody.
#[test]
fn the_shared_pots_penalty_never_scars_a_pair() {
    let mut g = game();
    let (viewer, subject) = (Seat(0), Seat(1));
    g.seats[subject.index()].research_directive = 50;
    for _ in 0..12 {
        g.settle_relations();
    }
    assert_eq!(g.directive_relations_term(subject), -1, "twelve turns of keeping it back");
    assert_eq!(g.relations.floor[viewer.index()][subject.index()], 0, "and not one step of scar");
    assert_eq!(g.relations_deeds(viewer, subject), 0, "nor a single banked point");
}

// ------------------------------------------------- 0.08.3 ticket #237: the Exodus Call

/// Ticket #237 (version 0.08.3): the Arkwrights' own order. Two turns of a doubled muster in one
/// Region, once per Region ever, for the price of a Leapfrog.
///
/// The second clause is the ticket, and it was chosen against a measurement. Their home state runs
/// from 20 units of population to 1 over a game ALREADY, because Coach Class charges them twice a
/// head; an order that doubled only the count would have burned the country twice as fast and
/// deepened the very thing that leaves them winning 3 games of 80. So a Call SUSPENDS the double
/// charge while it runs: they move twice the people at the ordinary price in population.
#[test]
fn an_exodus_call_doubles_the_muster_and_suspends_the_double_cost() {
    let mut g = game();
    let ark = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Arkwrights).unwrap();
    let sid = g.controlled_states(ark)[0];
    let plain = g.emigrants_per_turn(ark);
    assert_eq!(plain, 8, "Coach Class musters eight where others muster four");
    assert!((g.muster_population_in(ark, sid, plain) - g.lift_population(ark, plain)).abs() < 1e-9, "and pays double for them until the Call");

    g.seats[ark.index()].stockpile.ducats = 500;
    held_long_enough(&mut g, sid);
    g.commit_orders(ark, &[Order::ExodusCall { state: sid }]);
    assert!(g.exodus_call_running(sid), "it runs from the turn it is sounded");
    assert_eq!(g.emigrants_per_turn_in(ark, sid), plain * 2, "sixteen, not eight");

    // The whole point: sixteen people cost what sixteen people cost anybody else.
    let each = g.tables.emigrants.population_each;
    assert!((g.muster_population_in(ark, sid, 16) - each * 16.0).abs() < 1e-9, "the ordinary price, not their double");
    assert!(g.muster_population_in(ark, sid, 16) < g.lift_population(ark, 16), "which is strictly less than Coach Class charges");

    // Elsewhere they are unchanged: the Call is a Region's, not a Faction's.
    let other = g.controlled_states(ark).into_iter().find(|s| *s != sid);
    if let Some(other) = other {
        assert_eq!(g.emigrants_per_turn_in(ark, other), plain, "only the Region that answered");
    }
}

/// Ticket #237: once per Region, ever -- the shape the Strip Permit has had since ticket #54 --
/// and the Arkwrights alone.
#[test]
fn an_exodus_call_is_once_per_region_and_the_arkwrights_alone() {
    let mut g = game();
    let ark = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Arkwrights).unwrap();
    let sid = g.controlled_states(ark)[0];
    g.seats[ark.index()].stockpile.ducats = 500;
    held_long_enough(&mut g, sid);

    g.commit_orders(ark, &[Order::ExodusCall { state: sid }]);
    assert!(g.state(sid).exodus_call_used, "the Region is marked for good");
    assert_eq!(
        g.check_order(ark, &[], &Order::ExodusCall { state: sid }).unwrap_err().0,
        "this Region has answered an Exodus Call once already, and may not again"
    );

    let cus = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Custodians).unwrap();
    let theirs = g.controlled_states(cus)[0];
    g.seats[cus.index()].stockpile.ducats = 500;
    held_long_enough(&mut g, theirs);
    assert_eq!(
        g.check_order(cus, &[], &Order::ExodusCall { state: theirs }).unwrap_err().0,
        "only the Arkwrights sound an Exodus Call"
    );
    assert_eq!(g.order_cost(ark, &Order::ExodusCall { state: sid }).ducats, g.tables.ducats.per_leapfrog, "priced as a Leapfrog, the other Faction-only order on a state you hold");
}

// ------------------------------------------- 0.08.3 ticket #238: three turns before you remake

/// Ticket #238 (version 0.08.3): the Strip Permit, the Leapfrog and the Exodus Call all change a
/// country for good, and a Faction that has just walked in does not get to do that. Three whole
/// turns in hand, the turn of the taking not counting.
///
/// Measured before it was taken, over five whole games: the rule would have refused 31 of 31 Strip
/// Permits and 2 of 36 Leapfrogs AS ISSUED. That figure overstates the harm, and the second
/// measurement is why: every Strip Permit goes on a Region held nought or one turns, while the
/// Prospectors hold nine Regions of nine for three turns or more by mid-game. The rule delays a
/// strip onto the pile of long-held Regions the seat already sits on; it does not abolish it.
#[test]
fn three_turns_in_hand_before_a_faction_remakes_a_region() {
    let mut g = game();
    let cus = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Custodians).unwrap();
    let min = g.tables.faction_orders.min_turns_held;
    assert_eq!(min, 3, "the figure the rest of this test is written against");
    // Taken BEFORE the Region below is seized, or `controlled_states` hands back the new one.
    let home = g.controlled_states(cus)[0];

    // A Region taken this turn: refused, and the refusal names the turn it opens.
    let fresh = StateId::ALL.into_iter().find(|s| g.state(*s).control == Control::Neutral).unwrap();
    g.turn = 10;
    g.take_control(fresh, cus);
    g.seats[cus.index()].stockpile.ducats = 500;
    assert_eq!(g.turns_held(cus, fresh), Some(0), "the turn of the taking does not count");
    let refusal = g.check_order(cus, &[], &Order::Leapfrog { state: fresh }).unwrap_err().0;
    assert!(refusal.contains("from turn 13"), "the refusal says when, not just no: {refusal}");

    // Turn by turn until it opens.
    for (turn, open) in [(11, false), (12, false), (13, true)] {
        g.turn = turn;
        assert_eq!(g.may_remake(cus, fresh), open, "turn {turn}, held {:?}", g.turns_held(cus, fresh));
    }

    // A starting Region is held from turn 1, so it opens on turn 4 -- no special case.
    assert_eq!(g.state(home).held_since, Some(1), "held from the first turn");
    g.turn = 3;
    assert!(!g.may_remake(cus, home), "turn 3 is too soon");
    g.turn = 4;
    assert!(g.may_remake(cus, home), "and turn 4 opens it");
}

/// Ticket #238: losing the Region resets the clock, and a save written before this version -- which
/// has no clock at all -- counts as held long enough rather than silently losing three orders.
#[test]
fn the_hold_clock_resets_on_a_change_of_hands_and_an_old_save_passes() {
    let mut g = game();
    let (cus, pro) = (
        Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Custodians).unwrap(),
        Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Prospectors).unwrap(),
    );
    let sid = g.controlled_states(cus)[0];
    g.turn = 20;
    assert!(g.may_remake(cus, sid), "long held by its founder");

    g.take_control(sid, pro);
    assert_eq!(g.turns_held(pro, sid), Some(0), "the new holder starts from nothing");
    assert!(!g.may_remake(pro, sid));
    assert_eq!(g.turns_held(cus, sid), None, "and the old holder has no clock here at all");

    // Written back to the SAME holder, the clock does not restart: a repeated write nobody can see
    // must not deny an order forever.
    g.turn = 25;
    let before = g.state(sid).held_since;
    g.take_control(sid, pro);
    assert_eq!(g.state(sid).held_since, before, "same holder, same clock");
    assert!(g.may_remake(pro, sid));

    // An old save carries no clock.
    g.state_mut(sid).held_since = None;
    assert!(g.may_remake(pro, sid), "a missing clock counts as held long enough");
}



//! The formula tests spec 19.4 asks for, one per pinned rule.

use dying_earth_engine::combat::{self, Combatant, Dice};
use dying_earth_engine::data::{default_data_dir, CardEffect, CardRule, Tables};
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

/// Ticket #337 (version 0.09.0): `end_turn` refuses while a human seat owes this turn's choice card
/// an answer, exactly as it refuses while a human Lead owes a Tech, so a test that drives turns has
/// to answer it as a player does. It REFUSES the offer, which is the answer that buys nothing.
fn answer_the_card(g: &mut Game) {
    for seat in Seat::ALL {
        let owed = g.pending_question().map(|q| q.answer_of(seat).is_none()).unwrap_or(false);
        if owed && !g.seat(seat).ai {
            g.answer_card(seat, false).ok();
        }
    }
}

fn facility(kind: FacilityKind) -> Facility {
    Facility::new(kind)
}

/// Ticket #337 (version 0.09.0): put one named card on top of the deck and run the Question phase
/// until the draw chance lets it through, so a test about a card is not a test about the roll. The
/// Temperature is left where the game put it, because forcing the chance to one would need a
/// Temperature past the Collapse line and every turn after it would be a finished game.
fn ask_the_card(g: &mut Game, id: EventId) {
    stand_on_a_drawing_turn(g);
    for _ in 0..200 {
        g.deck.cards = vec![Card::Event(id)];
        g.deck.drawn.clear();
        g.question_phase();
        if g.draw != CardDraw::NoCard {
            return;
        }
    }
    panic!("{id:?} did not come in two hundred rolls at a draw chance of {:.2}", g.draw_chance());
}

/// Ticket #367 (version 0.09.2): nothing is drawn before `first_draw_turn`, so a test that rolls the
/// Question phase for a card stands the game on a turn that can draw one -- a fresh game is on
/// turn 1, which never rolls.
fn stand_on_a_drawing_turn(g: &mut Game) {
    g.turn = g.turn.max(g.tables.events.first_draw_turn);
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

/// Ticket #290 (version 0.08.6): every starting station opens with two Colonists aboard. A test
/// that counts people off Earth, or reasons about the board with the ISS bare, empties the
/// starting stations first and says so, rather than carrying two strangers in its arithmetic.
fn bare_stations(g: &mut Game) {
    for c in g.colonies.iter_mut().filter(|c| c.in_orbit && c.body == BodyId::Earth && c.founded_turn == 1) {
        c.colonists = 0;
    }
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
    // Ticket #326 (version 0.08.8): unarmed, as the resolution builds it.
    Combatant::new(UnitRef::Ship(ShipId(id)), format!("Colony Ship {id}"), 0, 3, 0, 0, false).armed(false)
}

#[test]
fn battle_round_three_hit_rolls_then_disengage_then_pursuit() {
    // A Frigate attacks a Colony Ship: every hit-roll goes to the attacker (scripted true), no disengage
    // roll is needed for the undamaged Frigate, and the Colony Ship (3 HP) dies on the third hit.
    let mut a = vec![frigate(1)];
    let mut d = vec![colony_ship(2)];
    let mut dice = Script { chances: VecDeque::from(vec![true, true, true]), d6s: VecDeque::new(), picks: VecDeque::new() };
    let stats = combat::fight(&mut a, &mut d, &mut dice, 2.0);
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
    let stats = combat::fight(&mut a, &mut d, &mut dice, 2.0);
    assert_eq!(stats.rounds, 1, "the attacker escaped, so the battle ended");
    assert_eq!(a[0].damage, 3, "2 from the round and 1 from the pursuit");
    assert!(a[0].escaped && !a[0].engaged);
    assert_eq!(d[0].damage, 1);
    assert_eq!(stats.all_escaped(), vec!["Frigate 1".to_string()]);
}

// ---------------------------------------------------------------- 10.2 Disengage and Pursuit probabilities

#[test]
fn disengage_probability_is_damage_over_hit_points_over_the_divisor_and_evade_is_half() {
    // The First Playable's 2, passed explicitly; the table's own figure is ticket #295's test.
    let mut c = frigate(1);
    assert_eq!(combat::disengage_chance(&c, 2.0), 0.0);
    c.damage = 2;
    assert!((combat::disengage_chance(&c, 2.0) - 0.25).abs() < 1e-12);
    c.damage = 3;
    assert!((combat::disengage_chance(&c, 2.0) - 0.375).abs() < 1e-12);
    c.evade = true;
    c.damage = 0;
    assert_eq!(combat::disengage_chance(&c, 2.0), 0.5);
}

#[test]
fn pursuit_catches_on_a_d6_at_or_under_pursuit_and_not_above() {
    // Colony Ship (evade) flees a Frigate (Pursuit 4). d6 = 4 catches; d6 = 5 does not.
    for (roll, caught) in [(4u32, true), (5u32, false)] {
        let mut a = vec![frigate(1)];
        let mut d = vec![Combatant::new(UnitRef::Ship(ShipId(2)), "Colony Ship 2", 0, 3, 0, 0, true)];
        // Evade roll true at the start; pursuit hit-roll true if caught.
        let mut dice = Script { chances: VecDeque::from(vec![true, true]), d6s: VecDeque::from(vec![roll]), picks: VecDeque::new() };
        let stats = combat::fight(&mut a, &mut d, &mut dice, 2.0);
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
    g.ships.push(Ship { name: String::new(), id: ShipId(1), kind: UnitKind::Frigate, seat: Seat(0), damage: 1, at: ShipAt::Body(BodyId::Earth), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
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
    // Axiom over Earth (bare until ticket #290, version 0.08.6, put two aboard); the Arkwrights
    // start with no station, so two slots stand free.
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
    // A station is not free to take: its threshold starts at the station base, plus the per-Colonist
    // figure -- and the ISS opens with two aboard since ticket #290 (version 0.08.6), so 80 where a
    // bare one reads 40. A consequence of the two aboard, not a rule of its own. Ticket #336
    // (version 0.09.0): 40 + 20 x 2, where it was 20 + 10 x 2.
    assert_eq!(g.influence_threshold(Place::Colony(iss)), 80);
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
    g.ships.push(Ship { name: String::new(), id: ship, kind: UnitKind::ColonyShip, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
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
    bare_stations(&mut g);
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
    bare_stations(&mut g);
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
    g.armies.push(Army { name: String::new(), id: army, home: ArmyHome::State(StateId::EastAsia), at: ArmyAt::Place(Place::State(StateId::EastAsia)), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None, levy: false, raised_strength: 0 });
    let ship = |id: u32, kind: UnitKind| Ship { name: String::new(), id: ShipId(id), kind, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None };
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
    assert_eq!(card.widgets, 4, "ticket #332: four Widgets for its one turn");
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
    g.armies.push(Army { name: String::new(), id, home: ArmyHome::State(seat_home), at: ArmyAt::Place(Place::State(target)), damage: 0, standing: false, stance: Stance::Attack, escaped: false, move_to: None, levy: false, raised_strength: 0 });
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
    assert!(matches!(g.state(StateId::NorthAfrica).control, Control::Occupied { occupier: Seat(0), previous: None, turns: 1, .. }));
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
    assert!(matches!(g.state(StateId::Europe).control, Control::Occupied { occupier: Seat(0), previous: Some(Seat(1)), turns: 1, .. }));
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
    bare_stations(&mut g);
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
    // Ticket #76 (version 0.05.5): forty cards for thirty-six turns.
    // Ticket #259 (version 0.08.4): the cards that can only land off Earth are not dealt at the
    // start; they join on turn 12, so the deck begins short and is 40 only once they are in.
    // Ticket #337 (version 0.09.0): every duplicate copy is cut and eighteen choice cards take
    // their places, so the deck is 40 DISTINCT cards and exactly SEVEN of them are off-Earth.
    // The two figures below moved with that: 28 became 33, and twelve copies became seven cards.
    assert_eq!(g.deck.cards.len(), 33, "thirty-three at the start: the seven off-Earth cards join on turn 12 (#259)");
    assert!(!g.deck.off_earth_joined);
    for id in [EventId::GridFailure, EventId::ReactorLeak, EventId::DustStorm, EventId::Moonquake, EventId::HeliumVein, EventId::RichSeam, EventId::IceDeposit] {
        assert!(g.tables.events.event.iter().find(|e| e.id == id).unwrap().off_earth, "{id:?} is flagged off Earth");
        assert_eq!(g.deck.count(id), 0, "{id:?} not dealt at the start");
    }
    assert_eq!(g.tables.events.off_earth_join_turn, 12);
    // A card may or may not be drawn on any turn (the chance is never nought), so the deck and its
    // drawn pile are counted together.
    // Ticket #337 (version 0.09.0): the DRAW is the Question phase's now, not the Event phase's.
    let dealt = |g: &Game| g.deck.cards.len() + g.deck.drawn.len();
    g.turn = 11;
    g.question_phase();
    assert_eq!(dealt(&g), 33, "turn 11: not yet");
    g.turn = 12;
    g.question_phase();
    assert!(g.deck.off_earth_joined);
    assert_eq!(dealt(&g), 40, "turn 12: the seven join, and the deck is forty");
    assert!(g.report.lines.iter().any(|l| l.text.contains("join the deck")), "the Report says so: {:?}", g.report.lines);
    g.question_phase();
    assert_eq!(dealt(&g), 40, "and they join once");
    // The rest of this test reads the deck as dealt, so a fresh one -- with the off-Earth cards
    // in -- is what the copy counts below are checked against.
    let mut g = game();
    g.turn = 12;
    g.deck.cards.append(&mut g.deck.drawn);
    g.question_phase();
    g.deck.cards.append(&mut g.deck.drawn);
    let copies = |id: EventId| g.deck.cards.iter().filter(|c| **c == Card::Event(id)).count();
    // Ticket #337: what used to be dealt twice and three times is dealt ONCE, every kind of it.
    for id in [EventId::Heatwave, EventId::Wildfire, EventId::RichSeam, EventId::SolarStorm] {
        assert_eq!(copies(id), 1, "{id:?} once now, where it was three times");
    }
    for id in [EventId::Unrest, EventId::MethaneBurst, EventId::LabourDispute, EventId::DustStorm] {
        assert_eq!(copies(id), 1, "{id:?} once now, where it was twice");
    }
    for id in [EventId::RadiationSurge, EventId::CommsBlackout, EventId::GridFailure, EventId::IceDeposit, EventId::Breakthrough, EventId::StormSurge] {
        assert_eq!(copies(id), 1, "{id:?} once now, where it was twice");
    }
    for id in [EventId::LaunchPadFire, EventId::SolarMaximum, EventId::MeteorShower, EventId::ReactorLeak] {
        assert_eq!(copies(id), 1, "{id:?} still once");
    }
    assert_eq!(EventId::ALL.len(), 40, "the 22 ordinary kinds and the eighteen that ask a question (#337)");
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
    g.ships.push(Ship { name: String::new(), id: ship, kind: UnitKind::ColonyShip, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
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
    // Ticket #332 (version 0.09.0): a Mine, since the Factory makes Widgets now and no Materials.
    g.state_mut(StateId::EastAsia).facilities = vec![facility(FacilityKind::Mine), facility(FacilityKind::Refinery), facility(FacilityKind::PowerPlant)];
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
    let mk = |id: u32, at: ShipAt| Ship { name: String::new(), id: ShipId(id), kind: UnitKind::Frigate, seat: Seat(0), damage: 0, at, colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None };
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
    // Ticket #332 (version 0.09.0): a Factory's own figure is 0.75, at the designer's word.
    assert!((e.factories - 0.75 * 0.4 * 0.75).abs() < 1e-9, "the Factory's 0.75, Clean Manufacturing's 0.4, the Custodians' 0.75: {}", e.factories);
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
    let f = Ship { name: String::new(), id: ShipId(1), kind: UnitKind::Frigate, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None };
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
fn tech_deep_mining_raises_mine_output_on_earth_and_off_it() {
    let mut g = game();
    // Ticket #332 (version 0.09.0): the Mine on Earth, where the Factory stood; Deep Mining
    // followed the Materials to it.
    g.state_mut(StateId::EastAsia).facilities = vec![facility(FacilityKind::Mine)];
    colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Mine], 0);
    // Earth Mine: 4 x 1.5 (Asia leans Materials) = 6. Moon Mine: 4 x 1.5 (Moon) = 6.
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
    // Ticket #300 (version 0.08.6): the landing turn is the first Battle turn now, so the window
    // moved a turn earlier: three-to-five was two-to-four, and nothing changes hands before the
    // third turn's Resolution, since an Occupation begun on the landing turn runs its three.
    assert!((2..=4).contains(&median), "median {median} turns: {taken:?}");
    for t in &taken {
        assert!(*t >= 2, "changed hands after only {t} turns, faster than the rules allow: {taken:?}");
    }
}

/// Ticket #336 (version 0.09.0) moved this pin and the test's name with it. It was written as
/// `a_zero_threshold_is_not_met_by_zero_influence`, against a Colony with no Colonists whose
/// threshold was 10 x 0 = 0: the rule it guarded is that a seat with no Standing at all takes
/// nothing, however low the gate. There is no zero threshold on the board any more -- every place
/// off Earth carries `colony_threshold_base` -- so the pin reads 40 and the rule is unchanged.
#[test]
fn a_seat_with_no_standing_takes_nothing() {
    let mut g = game();
    let c = colony(&mut g, Seat(1), BodyId::Moon, &[ModuleKind::Barracks], 0);
    assert_eq!(g.influence_threshold(Place::Colony(c)), 40, "the Colony base, where an empty Colony read 0");
    g.resolution_phase();
    assert_eq!(g.colony(c).unwrap().control, Control::Controlled(Seat(1)), "control does not move for free");
}

fn colony_attack_turns(seed: u64) -> Option<u32> {
    let mut g = with_seed(seed);
    // The AI Prospectors hold a Colony on the Moon with a Barracks and its Army.
    let cid = colony(&mut g, Seat(1), BodyId::Moon, &[ModuleKind::Habitat, ModuleKind::Barracks], 4);
    let defender = ArmyId(g.fresh_id());
    g.armies.push(Army { name: String::new(), id: defender, home: ArmyHome::Colony(cid), at: ArmyAt::Place(Place::Colony(cid)), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None, levy: false, raised_strength: 0 });
    // The player's two Carriers arrive at the Moon, each carrying an Army: strength 8 against 4.
    let mut attackers = Vec::new();
    let mut ships = Vec::new();
    for kind in [UnitKind::Carrier, UnitKind::Carrier] {
        let attacker = ArmyId(g.fresh_id());
        let ship = ShipId(g.fresh_id());
        g.armies.push(Army { name: String::new(), id: attacker, home: ArmyHome::State(StateId::EastAsia), at: ArmyAt::Aboard(ship), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None, levy: false, raised_strength: 0 });
        g.ships.push(Ship { name: String::new(), id: ship, kind, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Moon), colonists: 0, warhead: false, colonists_education: 1.0, army: Some(attacker), stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
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
        answer_the_card(&mut g);
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
        // Ticket #332 (version 0.09.0): and a Mine beside every start Factory, on top of the count.
        let mines = card.start_facilities.iter().filter(|k| **k == FacilityKind::Mine).count() as u32;
        assert_eq!(card.start_facilities.iter().filter(|k| **k == FacilityKind::Factory).count() as u32, mines, "{}: a Mine beside every Factory", card.name);
        assert_eq!(card.start_facilities.len() as u32 - labs - mines, card.industry_level, "{}: as many as the Industry Level, plus a start Lab and the Mines", card.name);
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
    // since ticket #164 the ISS's Core Module pays 1 more, and since ticket #332 (version 0.09.0)
    // the start Mine beside the Factory pays 2 more.
    assert_eq!(s.income_last_turn.energy, -4, "{:?}", s.income_last_turn);
    assert!(s.stockpile.energy >= 15, "no Energy starvation at the start: {:?}", s.stockpile);
}

// ---------------------------------------------------------------- #31 housekeeping rules

#[test]
fn only_climate_cards_scale_with_the_temperature() {
    let mut g = game();
    g.climate.temperature = 3.0; // scale 1.9 for a Climate card
    let mut seen = 0;
    stand_on_a_drawing_turn(&mut g);
    for id in [EventId::MeteorShower, EventId::SolarMaximum, EventId::RichSeam, EventId::Heatwave] {
        // Force the next draw to be this card; roll until the Draw Chance lets it through.
        let mut drawn = None;
        for _ in 0..50 {
            g.deck.cards = vec![Card::Event(id)];
            g.last_event = None;
            // Ticket #337 (version 0.09.0): the draw is the Question phase's; the Event phase takes
            // up what it held. Both are run, so the card still arrives where it always did.
            g.question_phase();
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
    let mk = |id: u32| Ship { name: String::new(), id: ShipId(id), kind: UnitKind::Frigate, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None };
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
        let stats = combat::melee(&mut [&mut a, &mut b, &mut c], &mut rng as &mut dyn Dice, 2.0, 3, 3);
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
        combat::melee(&mut [&mut a, &mut b, &mut c], &mut rng as &mut dyn Dice, 2.0, 3, 3);
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
        colonists: 0, warhead: false, colonists_education: 1.0,
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
        colonists: 0, warhead: false, colonists_education: 1.0,
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
        colonists: 0, warhead: false, colonists_education: 1.0,
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
    assert!((g.lift_population(Seat(0), 4) - 4.0).abs() < 1e-9, "one unit each: one million people since ticket #333 (version 0.09.0), five million from ticket #143 (version 0.07.3)");
    assert!((g.lift_population(Seat(2), 4) - 8.0).abs() < 1e-9, "Coach Class costs the state twice");
    // Ticket #73: the population is paid when the Emigrants muster, and the lift takes none.
    let before = g.state(StateId::NorthAfrica).population;
    g.commit_orders(Seat(2), &[Order::BuildEmigrants { state: StateId::NorthAfrica, n: 4 }]);
    let taken = before - g.state(StateId::NorthAfrica).population;
    assert!((taken - 8.0).abs() < 1e-9, "the recruit took {taken}, not 8.0 (two units of one million per Pioneer under Coach Class)");
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
    // Ticket #68: the fund is capped at what the Archive needs, and what it has no room for goes on
    // to the shared Tech rather than being wasted.
    // Ticket #347 (version 0.09.1): the cap is the Archive's whole figure from the first turn. It
    // used to be a quarter of it until the Module stood, and this test read 20 of 80 here.
    let research = g.tables.archive.research;
    assert_eq!(g.archive_fund_cap(Seat(3)), research, "the whole figure, with no Archive standing");
    g.seats[3].archive_fund = research - 3;
    g.research.contributions = [0; 4];
    g.income_phase();
    let made = g.seats[3].research_last_turn;
    assert!(made > 3, "the Lab makes more than the three the fund still has room for: {made}");
    assert_eq!(g.seats[3].archive_fund, research, "only the room under the cap is banked");
    assert_eq!(g.research.contributions[3], made - 3, "the rest counts toward the Lead as usual");
    // At the cap the declaration is refused outright.
    g.seats[3].research_directive = 0;
    assert_eq!(
        g.check_order(Seat(3), &[], &Order::SetResearchDirective { percent: 100 }).unwrap_err().0,
        format!("the Archive fund is full at {research}")
    );
    // The standing Module does not open the fund -- it was already open -- and a full fund is
    // refused in the Module's own words.
    let mars = colony(&mut g, Seat(3), BodyId::Mars, &[ModuleKind::Habitat], 4);
    g.colony_mut(mars).unwrap().modules.push(Module::new(ModuleKind::Archive));
    assert_eq!(g.archive_fund_cap(Seat(3)), research);
    assert_eq!(g.check_order(Seat(3), &[], &Order::SetResearchDirective { percent: 100 }).unwrap_err().0, "the Archive's Research is paid in full");
    g.seats[3].archive_fund = research - 1;
    assert!(g.check_order(Seat(3), &[], &Order::SetResearchDirective { percent: 100 }).is_ok(), "a point short and the declaration stands");
}
// ------------------------------------------------- ticket #347: the Archivists' Condition, at 125

/// Ticket #347 (version 0.09.1), R1: **one figure, 125**. The Research the Archive wants and the
/// bar the Archivists' first Victory part is scored against are the SAME number, written in two
/// files -- `[archive] research` in modules.toml and `victory_first.bar` in factions.toml. They
/// cannot be allowed to drift, because the first part reads the fund and the fund is capped at the
/// Archive's figure: a bar above it could never be met, and a bar below it would be met by a fund
/// that had not paid for the Module.
#[test]
fn the_archives_research_and_the_archivists_victory_bar_are_one_figure() {
    let g = game();
    assert_eq!(g.tables.archive.research, 125, "the designer's figure for ticket #347");
    let bar = g.tables.faction(FactionKind::Archivists).victory_first.bar;
    assert_eq!(
        bar, g.tables.archive.research as f64,
        "modules.toml [archive] research ({}) and the Archivists' victory_first.bar ({bar}) are one figure and have drifted",
        g.tables.archive.research
    );
}

/// Ticket #347, R2: **the quarter-cap goes entirely**. `archive_fund_cap` survives -- nobody may
/// bank more Research than the Archive needs -- but it is the Archive's own figure from the first
/// turn, whether or not the Module stands, where until this ticket it was a quarter of it (20 of
/// 80) until the Module physically stood. The refusal at a full fund no longer speaks of a quarter.
#[test]
fn the_archive_fund_is_capped_at_the_archives_own_figure_and_never_at_a_quarter_of_it() {
    let mut g = game();
    let research = g.tables.archive.research;
    g.state_mut(StateId::Europe).control = Control::Controlled(Seat(3));
    g.state_mut(StateId::Europe).facilities.push(facility(FacilityKind::ResearchLab));
    assert!(!g.archive_built(Seat(3)), "the premise: no Archive stands");
    assert_eq!(g.archive_fund_cap(Seat(3)), research, "the whole figure before the Module stands, not a quarter of it");
    // The banking clamp stands, now at the whole figure: what the fund has no room for still goes
    // on to the shared Tech rather than being wasted.
    g.seats[3].archive_fund = research - 3;
    g.seats[3].research_directive = 100;
    assert_eq!(g.bank_archive_research(Seat(3), 40), 3, "only the room under the cap is banked");
    assert_eq!(g.seats[3].archive_fund, research);
    // And at a full fund the declaration is refused, in words that are true of a fund full at 125
    // rather than of a quarter held back until the Module stands.
    g.seats[3].research_directive = 0;
    let refusal = g.check_order(Seat(3), &[], &Order::SetResearchDirective { percent: 100 }).unwrap_err().0;
    assert_eq!(refusal, format!("the Archive fund is full at {research}"), "the refusal no longer speaks of a quarter");
    // Once the Module stands the cap has not moved: it was never the Module that opened it.
    let mars = colony(&mut g, Seat(3), BodyId::Mars, &[ModuleKind::Habitat], 4);
    g.colony_mut(mars).unwrap().modules.push(Module::new(ModuleKind::Archive));
    assert_eq!(g.archive_fund_cap(Seat(3)), research, "the standing Module changes nothing");
    assert_eq!(g.check_order(Seat(3), &[], &Order::SetResearchDirective { percent: 100 }).unwrap_err().0, "the Archive's Research is paid in full");
}

/// Ticket #347, R3: **the Victory figure is not clamped**. This is the rule that unlocks the
/// Archivists. Their first part used to read `archive_fund.min(archive_fund_cap(seat))`, so before
/// the Module stood it could not pass a quarter however much they banked, and the sweeps measured
/// the fund sitting at exactly that quarter in every seating: 0 wins of 80. It reads the fund.
#[test]
fn an_archivists_first_victory_part_reads_the_whole_fund_with_no_archive_standing() {
    let mut g = game();
    let research = g.tables.archive.research;
    g.seats[3].archive_fund = 60;
    assert!(!g.archive_built(Seat(3)), "the premise: no Archive stands, which is where the old clamp bit");
    let p = g.progress(Seat(3));
    assert_eq!((p.first_value, p.first_bar), (60.0, research as f64), "60 of {research}, not a quarter of it");
    assert!((p.first_fraction() - 60.0 / research as f64).abs() < 1e-9, "{} should be 60/{research}", p.first_fraction());
    // The clamp is gone from the reading and not merely made redundant by the cap. This state --
    // a fund above the cap -- is one the banking rule cannot reach, which is exactly why the clamp
    // looked harmless; it is pinned here so the READING stays the fund, and a future rule that lets
    // the fund run past the Archive's figure cannot be silently truncated at the Victory panel.
    g.seats[3].archive_fund = research + 75;
    assert_eq!(g.progress(Seat(3)).first_value, (research + 75) as f64, "the first part is the fund, unclamped");
}

/// Ticket #347, R4: **the Module's other gates do not move.** This ticket changes the money and
/// nothing else. The journey -- The Upload researched, a Colony off Earth, four Colonists living
/// there at the order, 50 Materials, 12 Widgets, 12 Energy once complete -- is pinned here so it
/// cannot be quietly moved while the figures change.
#[test]
fn ticket_347_moves_the_archives_money_and_none_of_its_other_gates() {
    let mut g = game();
    let card = g.tables.module(ModuleKind::Archive);
    assert_eq!((card.materials, card.widgets, card.energy_upkeep), (50, 12, 12), "50 Materials, 12 Widgets, 12 Energy");
    assert_eq!(g.tables.archive.colonists_to_order, 4, "four Colonists must live there at the order");
    assert_eq!(g.tables.victory_gate(FactionKind::Archivists), Some(TechId::TheUpload), "The Upload is still the gate");
    // And the refusals the order still makes: the Colonists and Earth. Ticket #361 (version 0.09.1)
    // took the gate Tech off the order at the designer's word; it gates the win.
    g.seats[3].stockpile.materials = 200;
    let mars = colony(&mut g, Seat(3), BodyId::Mars, &[ModuleKind::Habitat], 4);
    assert!(g.check_order(Seat(3), &[], &Order::BuildArchive { colony: mars }).is_ok(), "four Colonists off Earth, with or without the Tech: allowed");
    g.colony_mut(mars).unwrap().colonists = 3;
    assert!(g.check_order(Seat(3), &[], &Order::BuildArchive { colony: mars }).unwrap_err().0.contains("4 Colonists"), "three is not four");
    let home = colony(&mut g, Seat(3), BodyId::Earth, &[ModuleKind::Habitat], 8);
    assert!(g.check_order(Seat(3), &[], &Order::BuildArchive { colony: home }).unwrap_err().0.contains("at a Colony on another Body"), "not on Earth");
}

/// Ticket #347, R5: **what the game says.** The Archivists' Victory sentence on their Faction card
/// is read out to the player whole, and it quotes the Research the Archive wants. It is written by
/// hand beside a figure the loader reads, so it is the one line in the data that can go on saying
/// 80 after every rule has moved to 125.
#[test]
fn the_archivists_victory_sentence_quotes_the_figure_the_rules_use() {
    let g = game();
    let card = g.tables.faction(FactionKind::Archivists);
    let research = g.tables.archive.research;
    assert!(card.victory.contains(&format!("pay {research} Research into it")), "the sentence must quote {research}: {:?}", card.victory);
    assert!(!card.victory.contains("80"), "and must not still say 80: {:?}", card.victory);
}

/// Ticket #68 (version 0.05.5): the Archive is one Module of 50 Materials and three turns, built
/// once from its own button at a Colony off Earth, with no Research banked first; standing, it
/// draws no Energy until its Research is paid, and the payment that fills the fund completes it.
/// Ticket #347 (version 0.09.1): that figure is 125, read off the table here rather than pinned,
/// because R1's own test is what pins it.
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
    // Ticket #332 (version 0.09.0): twelve Widgets to raise, four for each of its three turns, and
    // a Colony with only its Core Module makes four a turn, so it stands after the third Resolution.
    let b = g.colony(mars).unwrap().queue.iter().find(|b| b.item == BuildItem::Module(ModuleKind::Archive)).expect("queued").clone();
    assert_eq!((b.widgets, b.done), (12, 0), "the Archivists' Module discount is 1.0");
    assert_eq!(g.widgets_at(Place::Colony(mars)), 4, "the Core Module's four");
    for n in 1..3 {
        g.resolution_phase();
        assert!(!g.archive_built(Seat(3)), "{} Widgets of 12", n * 4);
        g.turn += 1;
    }
    g.resolution_phase();
    assert!(g.archive_built(Seat(3)), "twelve Widgets");
    assert_eq!(g.archive_colony(Seat(3)), Some(mars));
    assert!(!g.archive_complete(Seat(3)), "standing is not complete: the Research is still owed");
    let research = g.tables.archive.research;
    assert!(g.log.to_vec().iter().any(|l| l.contains("raised the Archive at") && l.contains(&format!("{research} more Research"))), "{:?}", g.log.to_vec());
    // At most one per Faction.
    let deimos = colony(&mut g, Seat(3), BodyId::Deimos, &[], 0);
    assert!(g.check_order(Seat(3), &[], &Order::BuildArchive { colony: deimos }).unwrap_err().0.contains("already stands at"));
    // No upkeep until it is complete; the payment that fills the fund completes it, with its Moment.
    assert_eq!(g.module_yield(Seat(3), mars, ModuleKind::Archive).upkeep, 0);
    g.seats[3].archive_fund = research - 4;
    g.seats[3].research_directive = 100;
    let banked = g.bank_archive_research(Seat(3), 10);
    assert_eq!(banked, 4, "only the four still owed are banked");
    assert_eq!(g.seats[3].archive_fund, research);
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
    let research = g.tables.archive.research;
    let cid = archive_at(&mut g, Seat(3), BodyId::Mars, research, 12);
    g.seats[3].stockpile.energy = 0;
    assert_eq!(g.module_yield(Seat(3), cid, ModuleKind::Archive).upkeep, 12, "a complete Archive draws 12");
    assert_eq!(g.shortfall_order(Seat(3))[0], "The Archive", "the highest upkeep goes first");
    g.income_phase();
    assert!(!g.colony(cid).unwrap().modules.iter().any(|m| m.kind == ModuleKind::Archive && m.online), "shut down");
    assert!(!g.archive_online(Seat(3)));
    let p = g.progress(Seat(3));
    assert_eq!(p.first_value, research as f64, "every point of Research is paid");
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
    let research = g.tables.archive.research;
    let again = archive_at(&mut g, Seat(3), BodyId::Moon, research, 12);
    // Ticket #164 (version 0.07.5): every Colony now draws 1 Energy for its Core Module, and an
    // Archive is 12 on its own; this test is about Occupation, not about the Energy bill.
    g.seats[3].stockpile.energy = 400;
    g.income_phase();
    assert!(g.archive_online(Seat(3)));
    g.colony_mut(again).unwrap().control = Control::Occupied { occupier: Seat(1), previous: Some(Seat(3)), turns: 1, banked: 0 };
    g.income_phase();
    assert!(!g.archive_online(Seat(3)), "an Occupied Colony's Archive is offline");
}

#[test]
fn the_archivists_win_with_the_archive_running_and_twelve_colonists_uploaded() {
    let mut g = game();
    let research = g.tables.archive.research;
    let cid = archive_at(&mut g, Seat(3), BodyId::Mars, research, 12);
    let _ = cid;
    open_gates(&mut g);
    g.seats[3].stockpile.energy = 200;
    g.income_phase();
    assert!(g.archive_online(Seat(3)));
    let p = g.progress(Seat(3));
    assert_eq!((p.first_value, p.first_bar), (research as f64, research as f64));
    assert_eq!((p.second_value, p.second_bar), (12.0, 12.0));
    assert!(p.met());
    g.end_phase();
    assert!(matches!(g.outcome, Some(Outcome::Win { seat: Seat(3), .. })), "{:?}", g.outcome);
    // One Colonist short and it is no win.
    let mut g = game();
    let research = g.tables.archive.research;
    let cid = archive_at(&mut g, Seat(3), BodyId::Mars, research, 11);
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
    // Ticket #192 (version 0.08.0): the Archive is not orderable there, and the computer must not
    // spend the turn on an order that would be refused. Measured, it ordered the Archive on turn 1
    // at an empty station in 80 of 80 games before this gate. Ticket #290 (version 0.08.6): Axiom
    // opens with two aboard now, and ticket #209 keeps the Archive off Earth's orbit regardless.
    let axiom = station_of(&g, arc, BodyId::Earth).unwrap();
    assert_eq!(g.colony(axiom).unwrap().colonists, 2, "the premise: Axiom opens with two aboard");
    let orders = g.ai_orders(arc);
    assert!(!orders.iter().any(|o| matches!(o, Order::BuildArchive { .. })), "it should not order what would be refused: {orders:?}");

    // With the Core Module's four more living there, it orders the Module at once.
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
    // Ticket #361 (version 0.09.1): The Upload no longer gates the ORDER, so with four Colonists
    // living on Mars the computer orders the Archive at once, Tech or no Tech.
    assert!(!g.has_tech(TechId::TheUpload), "the premise: the world has not researched it yet");
    let orders = g.ai_orders(arc);
    assert!(orders.iter().any(|o| matches!(o, Order::BuildArchive { .. })), "it orders the Archive before The Upload: {orders:?}");
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
/// by one per `refugees_per` of population -- two and a half million people: half a unit of five
/// million until ticket #333 (version 0.09.0), two and a half units of one million since -- at most
/// two in a turn.
#[test]
fn c_heat_refugees_arrive_at_the_neighbours_and_raise_unrest_per_two_and_a_half_million_people() {
    let mut g = game();
    calm(&mut g);
    for s in &mut g.states {
        s.population = 0.0;
    }
    // Ticket #333: 500 units of one million, the 100 units of five million of before -- the same
    // five hundred million people -- so the flow below is still worth exactly one point.
    let per = g.tables.unrest.refugees_per;
    assert_eq!(per, 2.5, "two and a half million people a point, as 0.5 units of five million were (ticket #333)");
    let before = 500.0;
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
    assert!(arrived > per && arrived < 2.0 * per, "the flow is worth exactly one point of Unrest: {arrived}");
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
    let want = (arrived / per).floor();
    assert_eq!(want, 1.0, "the flow of {arrived} is one point at {per} a point");
    assert_eq!(g.unrest(StateId::Russia), want.min(2.0), "one Unrest per two and a half million people arriving");
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
/// when the net is worth at least `report_net_floor` -- two and a half million people: half a unit
/// of five million when it was set, two and a half units of one million since ticket #333 (version
/// 0.09.0). The designer: *"reduce report clutter by reporting
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

    // Under the floor, in either direction: silence. Ticket #333: 2.0 units of one million, the two
    // million people that 0.4 units of five million were, under a floor of 2.5 that was 0.5.
    let mut g = game();
    calm(&mut g);
    assert_eq!(g.tables.unrest.report_net_floor, 2.5, "two and a half million people, as 0.5 units of five million were (ticket #333)");
    g.state_mut(StateId::Russia).refugees_in = 2.0;
    g.state_mut(StateId::SouthAsia).refugees_out = vec![("the sea".to_string(), 2.0)];
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
    g.state_mut(StateId::NorthAfrica).queue.push(Build { item: BuildItem::Facility(FacilityKind::Bank), seat: Seat(0), coastal: false, widgets: 99, done: 0 });
    let id = ArmyId(g.fresh_id());
    g.armies.push(Army { name: String::new(), id, home: ArmyHome::State(StateId::Europe), at: ArmyAt::Place(Place::State(StateId::NorthAfrica)), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None, levy: false, raised_strength: 0 });
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
    g.state_mut(StateId::NorthAfrica).control = Control::Occupied { occupier: Seat(1), previous: None, turns: 1, banked: 0 };
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
    // The world still holds about 7.9 billion people, as the eight states did: 7,860 units of one
    // million since ticket #333 (version 0.09.0), 1,572 units of five million from ticket #143
    // (version 0.07.3) until then, 78.6 hundred-million before.
    let people: f64 = StateId::ALL.iter().map(|s| card(*s).population).sum();
    assert!((people - 7860.0).abs() < 0.1, "population {people}");
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
        // Ticket #332 (version 0.09.0): and less the Mine beside every start Factory.
        let mines = c.start_facilities.iter().filter(|k| **k == FacilityKind::Mine).count() as u32;
        assert_eq!(c.start_facilities.len() as u32 - labs - mines, c.industry_level, "{s:?} starts with as many Facilities as its Industry Level, plus a start Lab and the Mines");
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
    // North Africa is the Prospectors': baseline 0.4 at Industry Level 2, with 3.0 units of
    // population. Sub-Saharan Africa stays neutral carrying exactly the same weight.
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
    // Ticket #332 (version 0.09.0): the Mine, since it is Materials this test reads.
    g.state_mut(sid).facilities.push(facility(FacilityKind::Mine));
    let idx = facility_at(&g, sid, FacilityKind::Mine);
    let slots = g.slots_used(sid);
    let emissions_before = g.emissions_now().factories;
    assert!(emissions_before > 0.0, "the Mine emits while it works");

    // What it makes and what it costs to run, before and after.
    g.seats[0].stockpile.energy = 20;
    let before = g.seats[0].stockpile;
    g.income_phase();
    let made = g.seats[0].stockpile.materials - before.materials;
    let spent = 20 - g.seats[0].stockpile.energy;
    assert!(made > 0, "a working Mine makes Materials");

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
    // Ticket #333 (version 0.09.0): per unit of one million, a hundredth of them; the quotation is unchanged.
    assert_eq!(c.population_emissions_base, 0.0004);
    assert_eq!(c.population_emissions_per_level, 0.0003);
    for sid in StateId::ALL {
        let want = c.population_emissions_base + c.population_emissions_per_level * g.state(sid).industry_level as f64;
        assert!((g.population_coefficient(sid) - want).abs() < 1e-9, "{sid:?}: {} where {want} was wanted", g.population_coefficient(sid));
    }
    // Sub-Saharan Africa at Industry Level 1 emits 0.07 per hundred million; East Asia at 3 emits
    // 0.13. Ticket #143: the coefficient is per unit of five million, twenty to the hundred million;
    // ticket #333: per unit of one million, a hundred to the hundred million, read off the tables.
    assert_eq!(g.tables.climate.people_per_unit, 1_000_000.0, "one unit is one million people (ticket #333)");
    let per_hundred_million = g.tables.units_per_hundred_million();
    assert_eq!(per_hundred_million, 100.0);
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
    // Ticket #333: 1,440 units of one million, the same 0.432 again.
    let per_level = g.tables.climate.population_emissions_per_level;
    assert!((rise - per_level * 1440.0 * mult).abs() < 1e-9, "the population line rose {rise:.3}");
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
    // Ticket #332 (version 0.09.0): 8 Widgets, four for each of its two turns.
    assert_eq!((card.materials, card.widgets, card.energy_upkeep, card.emissions), (30, 8, 3, 0.0));
    assert!(card.no_slot, "a Scrubber takes no build slot");
    // The cap: one per two hundred million people (ticket #333, version 0.09.0: 200 units of one
    // million; 40 units of five million before), between 2 and 10.
    assert_eq!(g.tables.scrubber.per_population, 200.0, "two hundred million people a Scrubber");
    assert_eq!(g.scrubber_cap(StateId::Russia), 2, "Russia at 150 takes the floor");
    assert_eq!(g.scrubber_cap(StateId::SouthAsia), 10, "India at 1940 takes the ceiling");
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
    // It is built without a slot: fill the state and build one anyway. Ticket #332 (version
    // 0.09.0): with no Factory the Region makes 7 Widgets a turn (a flat 4 and Industry Level 3),
    // so the Scrubber's 8 land at the second Resolution, as its two turns did.
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
    assert_eq!(g.scrubbers_online(sid), 0, "7 of 8 Widgets: not yet");
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
    // East Asia: four start Facilities (ticket #332: a Mine beside the Factory), six coastal
    // slots, three inland.
    let sid = StateId::EastAsia;
    // East Asia is the player's start state, so its Launch Site is a start Facility too and takes
    // the next coastal slot after the four on the card.
    assert_eq!(
        standing(&g, sid, true),
        vec![FacilityKind::Factory, FacilityKind::Mine, FacilityKind::PowerPlant, FacilityKind::Refinery],
        "the start Facilities stand on the coast, in the table's order"
    );
    assert_eq!(standing(&g, sid, false), vec![FacilityKind::LaunchSite], "the Launch Site, fifth, stands inland: the coast holds four");

    // A new build takes an inland slot while one is free.
    directed(&mut g, sid);
    let orders = vec![Order::BuildFacility { state: sid, kind: FacilityKind::Bank }];
    pick_a_tech(&mut g);
    answer_the_card(&mut g);
    g.end_turn([orders, Vec::new(), Vec::new(), Vec::new()]).expect("the turn should end");
    pick_a_tech(&mut g);
    answer_the_card(&mut g);
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
    // Ticket #343 (version 0.09.1): twenty-one, with Missile Technology on Propulsion rung 3.
    assert_eq!(TechId::ALL.len(), 21, "thirteen Techs, the four gates, Civil Defense, #232's two, and Missile Technology");
    assert_eq!(g.tables.techs.len(), 21, "and twenty-one rows in techs.toml");
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
    // Ticket #332 (version 0.09.0): and the Mine, sixteen.
    assert_eq!(FacilityKind::ALL.len(), 16, "sixteen Facilities");
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
        colonists: 4, warhead: false, colonists_education: 1.0,
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
    // Ticket #332 (version 0.09.0): Mines, since it is Materials the surge cuts here.
    g.state_mut(sid).facilities = vec![Facility::in_coastal_slot(FacilityKind::Mine), facility(FacilityKind::Mine), Facility::new(FacilityKind::SeaWall)];
    assert_eq!(income_of(&mut g, Seat(0)).materials, 8, "two Custodian Mines make 4 each");
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
    // Ticket #290 (version 0.08.6): with two aboard the ISS, its opening Habitat would be a third
    // contender for the forty Materials; this fixture is about the wall against the Scrubber.
    bare_stations(&mut g);
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
                warhead: false,
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
    // Ticket #335 (version 0.09.0): a station fuels only a Ship in its own orbit, so the short
    // tanks stand at the ISS's ring; from low orbit the computer would ask for the orbit change.
    let iss_slot = near.colonies.iter().find(|c| c.in_orbit && c.body == BodyId::Earth && c.control.director() == Some(Seat(0))).map(|c| c.slot).expect("the ISS");
    for s in near.ships.iter_mut().filter(|s| s.kind == UnitKind::ColonyShip) {
        s.fuel = 5;
        s.slot = Some(iss_slot);
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
    // Ticket #345 (version 0.09.1): the crossing is into the MARS SYSTEM rather than Mars itself.
    // Phobos and Deimos pay the largest first-to-a-Body windfall on the board (20 against Mars's
    // 15), so a seat with a full tank on the window now reaches past Mars for one of its moons.
    // This test is about the FUEL BANK letting the crossing happen at all, not about which of the
    // three it picks.
    assert!(
        orders.iter().any(|o| matches!(o, Order::Transit { to: BodyId::Mars | BodyId::Phobos | BodyId::Deimos, .. })),
        "a full tank on the window crosses: {orders:?}"
    );
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
    g.ships.push(Ship { name: String::new(), id: ship, kind: UnitKind::ColonyShip, seat: cust, damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 4, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
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
        colonists: 4, warhead: false, colonists_education: 1.0,
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
    answer_the_card(&mut g);
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
    // Ticket #332 (version 0.09.0): a build whose Widgets are all done completes at this Resolution.
    g.state_mut(StateId::EastAsia).queue.push(Build {
        item: BuildItem::Facility(FacilityKind::Factory),
        seat: Seat(0),
        widgets: 4,
        done: 4,
        coastal: false,
    });
    let (_, found) = colony_ship_ready(&mut g, BodyId::Moon);
    hold_temperature(&mut g, 1.7);
    breaks_ahead(&mut g);
    let mut orders: [Vec<Order>; SEAT_COUNT] = std::array::from_fn(|_| Vec::new());
    orders[0] = vec![found];
    pick_a_tech(&mut g);
    answer_the_card(&mut g);
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
        colonists: 0, warhead: false, colonists_education: 1.0,
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
    answer_the_card(&mut g);
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
        answer_the_card(&mut g);
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
    answer_the_card(&mut p);
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
    // Ticket #332 (version 0.09.0): the Mine joins the list at the Producer's weight, one more
    // cheap answer to reach past, so the fifth build is the one this test reads (was four, 95).
    g.seats[seat.index()].stockpile.materials = 115;
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
    // Ticket #332 (version 0.09.0): North America's three and the Mine beside its Factory.
    assert_eq!(g.state(StateId::NorthAmerica).facilities.iter().filter(|f| f.kind != FacilityKind::LaunchSite).count(), 5, "added to North America's three, and the Mine");
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
    g.state_mut(StateId::NorthAmerica).control = Control::Occupied { occupier: Seat(2), previous: None, turns: 1, banked: 0 };
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
    assert_eq!((w.materials, w.widgets), (20, 8), "20 Materials since ticket #77; 8 Widgets since ticket #332");
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
    // Ticket #332 (version 0.09.0): a Mine on Earth, since it is Materials this reads.
    g.state_mut(StateId::Europe).facilities = vec![facility(FacilityKind::Mine)];
    let moon = colony(&mut g, pro, BodyId::Moon, &[ModuleKind::Mine], 0);
    let _ = moon;
    // Europe's economy pays 14 Ducats a turn to this seat. Materials output is 13 and is now
    // beside the point: an Earth Mine at 4 x 1.25 = 5 and a Moon Mine at 4 x 1.65 x 1.25 = 8.
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
    // Ticket #332 (version 0.09.0): a Mine, since it is Materials the Drought halves here.
    g.state_mut(StateId::Europe).facilities = vec![facility(FacilityKind::Mine)];
    assert_eq!(income_of(&mut g, Seat(0)).materials, 4, "a Custodian Mine makes 4");
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
    assert!((pop - g.state(StateId::EastAsia).population - 4.0).abs() < 1e-9, "one unit each: one million people since ticket #333, five million from ticket #143");
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
    // Ticket #290 (version 0.08.6): the ISS opens with two aboard; this test lifts four into a
    // station with four berths free, so it starts from the bare one it was written against.
    bare_stations(&mut g);
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
fn a_custodian_mothballed_mine_doubles_their_best_mine_off_earth_one_for_one() {
    let mut g = game();
    let cus = Seat(0);
    assert_eq!(g.kind(cus), FactionKind::Custodians);
    let moon = colony(&mut g, cus, BodyId::Moon, &[ModuleKind::Mine], 0);
    let mars = colony(&mut g, cus, BodyId::Mars, &[ModuleKind::Mine], 0);
    let vostok = colony(&mut g, cus, BodyId::Earth, &[ModuleKind::Mine], 0);
    // Ticket #332 (version 0.09.0): the Mine on Earth pairs with the Mine off it; the Factory
    // pairs with the Factory Module now.
    let st = g.state_mut(StateId::EastAsia);
    st.facilities.push(facility(FacilityKind::Mine));
    st.facilities.push(facility(FacilityKind::Mine));
    assert!(g.doubled_modules(cus).is_empty(), "no idle Mine, no bonus");
    assert_eq!(g.module_yield_at(cus, moon, 0).amount, 6);
    let i = g.state(StateId::EastAsia).facilities.len() - 1;
    g.state_mut(StateId::EastAsia).facilities[i].mothballed = true;
    assert_eq!(g.doubled_modules(cus), vec![(moon, 0)], "one idle Mine, the best Mine off Earth");
    let y = g.module_yield_at(cus, moon, 0);
    assert_eq!(y.amount, 12, "6 doubled");
    assert_eq!(y.doubled_by, Some("Mine"), "named for the Mine whose mothball pays for it");
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
    // Ticket #332 (version 0.09.0): the Mine pairs with the Mine.
    let mut f = facility(FacilityKind::Mine);
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

/// Ticket #82: the Custodian AI idles a Mine on Earth once an undoubled Mine off Earth outproduces
/// it, and does not restart one whose doubling stands, even with Energy to spare. East Asia leans
/// Materials, so its Mine makes 4 x 1.5 = 6; a Phobos Mine makes 4 x 1.75 = 7. (Ticket #332,
/// version 0.09.0: the Mine, where this read the Factory; the pairs are Mine to Mine now.)
#[test]
fn the_custodian_ai_idles_a_mine_a_phobos_mine_outproduces_and_keeps_it_idle() {
    let mut g = game();
    calm(&mut g);
    let cus = Seat(0);
    g.seats[0].stockpile.energy = 500;
    g.seats[0].stockpile.materials = 10;
    colony(&mut g, cus, BodyId::Phobos, &[ModuleKind::Mine, ModuleKind::Generator], 0);
    g.state_mut(StateId::EastAsia).facilities.push(facility(FacilityKind::Mine));
    let i = g.state(StateId::EastAsia).facilities.len() - 1;
    let orders = g.ai_orders(cus);
    assert!(
        orders.iter().any(|o| matches!(o, Order::Change { building: BuildingRef::Facility(StateId::EastAsia, j), what: BuildingChange::Mothball } if *j == i)),
        "no mothball of the Earth Mine a Phobos Mine (7) outproduces (6): {orders:?}"
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
    assert_eq!(TechId::ALL.len(), 21, "eighteen, Beneficiation and Relay Networks since ticket #232, and Missile Technology since #343");
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
    // Ticket #332 (version 0.09.0): and a Factory Module, so the station makes 8 Widgets a turn
    // and the Frigate's 4 land at the next Resolution.
    g.colony_mut(iss).unwrap().modules.push(Module::new(ModuleKind::Factory));
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
    // Ticket #335 (version 0.09.0): a station fuels only a Ship in its OWN orbit, so a Ship in low
    // orbit is refused until it has changed orbit to the ring the ISS stands on.
    let iss_slot = g.colonies.iter().find(|c| c.in_orbit && c.body == BodyId::Earth && c.control.director() == Some(Seat(0))).map(|c| c.slot).expect("the ISS");
    // Ticket #357 (version 0.09.1): and the refusal says the move first.
    assert!(g.check_order(Seat(0), &[], &refuel).unwrap_err().0.starts_with("Move this Ship to Earth, at "), "in low orbit nothing fuels it");
    g.ship_mut(ship).unwrap().slot = Some(iss_slot);
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
    // Ticket #335 (version 0.09.0): the station rescues it because the 1 in the tank still pays the
    // orbit change that reaches the ring it stands on; a dry tank in the wrong orbit does not.
    assert!(!g.stranded(far), "a station of ours in orbit rescues it");
    g.ship_mut(far).unwrap().fuel = 0;
    assert!(g.stranded(far), "with nothing in the tank it cannot even change orbit to the station");
    g.ship_mut(far).unwrap().fuel = 1;
    g.ship_mut(far).unwrap().slot = Some(0);
    assert!(g.check_order(Seat(0), &[], &Order::Refuel { ship: far }).is_ok());
    let (full, _) = colony_ship_ready(&mut g, BodyId::Earth);
    g.ship_mut(full).unwrap().fuel = 30;
    g.ship_mut(full).unwrap().slot = Some(iss_slot);
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
    // Ticket #335 (version 0.09.0): the Ship stands at the ISS's own ring, which is the orbit a
    // station fuels from; in low orbit the computer asks for the orbit change first.
    let iss_slot = g.colonies.iter().find(|c| c.in_orbit && c.body == BodyId::Earth && c.control.director() == Some(Seat(0))).map(|c| c.slot).expect("the ISS");
    g.ship_mut(ship).unwrap().slot = Some(iss_slot);
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
    assert_eq!(g.tables.module(ModuleKind::SolarArray).widgets, 8, "ticket #332: four Widgets for each of its two turns");
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
    assert_eq!((card.materials, card.widgets, card.energy_upkeep), (35, 8, 4), "ticket #332: 8 Widgets for its two turns");
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
    bare_stations(&mut g);
    let station = g.colonies.iter().find(|c| c.body == BodyId::Venus && c.in_orbit).expect("Ishtar stands").id;
    g.colony_mut(station).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    let land = Order::Unload { ship, colonists: 4, army: false, into: UnloadTarget::Colony(station) };
    // Ticket #335 (version 0.09.0): a station is unloaded into from its own orbit; the Ship that
    // built it from low orbit changes orbit to its ring first.
    assert!(g.check_order(Seat(0), &[], &land).unwrap_err().0.contains("reached from"), "not from low orbit");
    let change = Order::ChangeOrbit { ship, slot: Some(0) };
    assert!(g.check_order(Seat(0), &[], &change).is_ok());
    g.commit_orders(Seat(0), std::slice::from_ref(&change));
    g.resolution_phase();
    assert_eq!(g.ship(ship).unwrap().slot, Some(0), "it rode up to Ishtar's ring");
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
    // Ticket #335 (version 0.09.0): the Ship stands at the station's own ring, which is the orbit
    // a station is unloaded into; from low orbit the computer asks for the orbit change first.
    let slot = g.colony(venus).unwrap().slot;
    g.ship_mut(ship).unwrap().slot = Some(slot);
    let orders = g.ai_orders(Seat(0));
    assert!(orders.iter().any(|o| matches!(o, Order::Unload { ship: s, into: UnloadTarget::Colony(c), .. } if *s == ship && *c == venus)), "no landing into the Venus station: {orders:?}");
    let mut g = game();
    calm(&mut g);
    g.seats[0].stockpile.materials = 300;
    g.seats[0].stockpile.energy = 300;
    let iss = station_of(&g, Seat(0), BodyId::Earth).unwrap();
    g.colony_mut(iss).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    let (ship, _) = colony_ship_ready(&mut g, BodyId::Earth);
    // Ticket #335 (version 0.09.0): at the ISS's own ring, the orbit it is unloaded into from.
    let slot = g.colony(iss).unwrap().slot;
    g.ship_mut(ship).unwrap().slot = Some(slot);
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

/// The AI sweep: a Custodian Mine off Earth is worth twice its base while a Mine of theirs on
/// Earth could be idled to double it (Production Moved, ticket #82; ticket #332 paired the Mine
/// with the Mine, where this read the Factory); with none to idle it is worth its base, as every
/// other seat's Mine is. Until the sweep the Custodian AI never built a Module off Earth in eight
/// batches of twenty seeds: Earth's Facilities outscored them at the same base and the Materials
/// reserve starved the rest.
#[test]
fn the_custodian_ai_weighs_a_mine_off_earth_by_the_doubling_an_idle_earth_mine_would_give() {
    let score_of_moon_mine = |earth_mine: bool| -> f64 {
        let mut g = game();
        calm(&mut g);
        g.seats[0].stockpile.energy = 500;
        g.seats[0].stockpile.materials = 500;
        colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Generator], 4);
        for st in &mut g.states {
            st.facilities.retain(|f| f.kind != FacilityKind::Mine);
        }
        if earth_mine {
            g.state_mut(StateId::EastAsia).facilities.push(facility(FacilityKind::Mine));
        }
        g.ai_orders(Seat(0));
        let l = g.log.iter().find(|l| l.contains("build Mine at") && l.contains("on the Moon")).unwrap_or_else(|| panic!("no Moon Mine scored: {:#?}", g.log.iter().filter(|l| l.contains("Moon")).collect::<Vec<_>>())).clone();
        l.split_whitespace().nth(1).and_then(|n| n.parse().ok()).unwrap()
    };
    let (with, without) = (score_of_moon_mine(true), score_of_moon_mine(false));
    assert!(without > 0.0 && (with - 2.0 * without).abs() < 1e-6, "a Moon Mine with an Earth Mine to idle should score twice one without: {with} vs {without}");
}

/// The AI sweep: the Custodian AI idles an Earth Mine for an even trade too, since the Emissions
/// leave Earth with the output. East Asia's Mine makes 6 (a Materials lean); a Moon Mine makes 6.
/// (Ticket #332, version 0.09.0: the Mine, where this read the Factory.)
#[test]
fn the_custodian_ai_idles_an_earth_mine_for_an_even_trade() {
    let mut g = game();
    calm(&mut g);
    g.seats[0].stockpile.energy = 500;
    g.seats[0].stockpile.materials = 10;
    colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Mine, ModuleKind::Generator], 0);
    g.state_mut(StateId::EastAsia).facilities.push(facility(FacilityKind::Mine));
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
    g.colony_mut(c).unwrap().queue.push(Build { item: BuildItem::Module(ModuleKind::Mine), seat: Seat(0), widgets: 4, done: 0, coastal: false });
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
    // Ticket #290 (version 0.08.6): it opens with two aboard now; this test is the cap rule from
    // nobody upward, so it empties the station first.
    bare_stations(&mut g);
    assert_eq!(g.colony(station).unwrap().colonists, 0, "emptied, for the rule from nought");
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

/// Ticket #348 (version 0.09.1), R1: every Faction's OWN gate chain is on its OWN pick list. Only
/// the Custodians could reach their gate before this -- three Techs and 98 Research with a complete
/// list -- while the other three needed five Techs and 148 and each was missing antecedents it
/// would only ever have taken by the cheapest-remaining fallback.
///
/// The chains are COMPUTED from the tables here rather than pinned, so a change to the tree moves
/// this test with it instead of rotting: a test that hard-coded "Beneficiation is on the
/// Prospectors' list" would say nothing the day the Extraction Charter stopped needing it.
#[test]
fn every_factions_gate_chain_is_in_its_own_pick_list() {
    let t = tables();
    for kind in FactionKind::ALL {
        let gate = t.victory_gate(kind).expect("every Faction has a Victory gate");
        let order = &t.ai_tech_picks(kind).order;
        let chain = t.gate_chain(kind);
        for need in &chain {
            assert!(
                order.contains(need),
                "the {:?} cannot reach {} without {}, and it is not on their list: {:?}",
                kind,
                t.tech(gate).name,
                t.tech(*need).name,
                order.iter().map(|x| t.tech(*x).name.clone()).collect::<Vec<_>>()
            );
        }
        // A Faction never swears off a Tech its own Victory turns on.
        assert!(!chain.contains(&t.ai_tech_picks(kind).never.unwrap_or(gate)), "the {kind:?} refuse a Tech on their own chain");
    }
}

/// Ticket #348, R2, the first of the two witnesses it owes: the drawn shortlist carries the NEXT
/// RUNG of the Lead's own chain while the gate itself is still out of reach.
///
/// Ticket #98's guarantee fired only `if available.contains(&gate)`, and a Tech is available only
/// once its prerequisites are done -- so the promise that a Faction is never denied its own gate
/// could not be kept until the chain had already been climbed by luck. This climbs each Faction's
/// chain a rung at a time and asks for the rung at every step.
#[test]
fn the_shortlist_carries_the_next_rung_of_the_leads_chain() {
    for seat in Seat::ALL {
        let mut g = game();
        let kind = g.kind(seat);
        let gate = g.tables.victory_gate(kind).expect("every Faction has a Victory gate");
        let chain = g.tables.gate_chain(kind);
        assert!(!chain.is_empty(), "the {kind:?} have a chain to climb");
        let size = g.tables.shortlist.size;
        for step in 0..chain.len() {
            let rung = g.next_gate_rung(seat).expect("a rung is owed while the chain stands unclimbed");
            assert!(!g.available_techs().contains(&gate), "the {kind:?} gate is out of reach at rung {step}");
            assert!(g.available_techs().len() > size, "more is available than the list holds");
            // Drawn many times over: the rung is on every one of them, and the gate on none.
            for _ in 0..25 {
                g.draw_shortlist(seat);
                assert!(
                    g.research.shortlist.contains(&rung),
                    "the {:?} were not offered {} at rung {step}: {:?}",
                    kind,
                    g.tables.tech(rung).name,
                    g.research.shortlist.iter().map(|t| g.tables.tech(*t).name.clone()).collect::<Vec<_>>()
                );
                assert!(!g.research.shortlist.contains(&gate), "the {kind:?} gate was drawn while unreachable");
            }
            g.research.done.push(rung);
        }
        // The chain climbed, the gate is what is owed, and nothing on the chain is left to force.
        assert_eq!(g.next_gate_rung(seat), None, "the {kind:?} chain is climbed and nothing more is owed");
        assert!(g.available_techs().contains(&gate), "the {kind:?} gate is reachable now");
        for _ in 0..25 {
            g.draw_shortlist(seat);
            assert!(g.research.shortlist.contains(&gate), "the {kind:?} gate is drawn once its chain is climbed");
        }
    }
}

/// Ticket #348, R2, the second witness: the gate and a forced antecedent are NEVER forced together,
/// which is why the shortlist can reserve ONE place of its three and stay three Techs long.
///
/// The gate is available only when every antecedent is done; an unresearched antecedent exists only
/// when some antecedent is not done. Walked here over every state the chain can actually be in --
/// every subset of the chain, the gate done or not -- skipping the states Research cannot reach,
/// which are the ones where a done Tech's own prerequisites are not done. `pick_tech` refuses
/// anything that is not available, so that is exactly the set of states a game can be in.
#[test]
fn a_draw_never_forces_both_the_gate_and_its_chain() {
    for seat in Seat::ALL {
        let probe = game();
        let kind = probe.kind(seat);
        let gate = probe.tables.victory_gate(kind).expect("every Faction has a Victory gate");
        let chain = probe.tables.gate_chain(kind);
        let size = probe.tables.shortlist.size;
        let mut reachable_states = 0;
        for mask in 0..(1u32 << (chain.len() + 1)) {
            let mut g = game();
            for (i, t) in chain.iter().enumerate() {
                if mask & (1 << i) != 0 {
                    g.research.done.push(*t);
                }
            }
            if mask & (1 << chain.len()) != 0 {
                g.research.done.push(gate);
            }
            let closed = g.research.done.iter().all(|t| g.tables.tech(*t).needs.iter().all(|n| g.research.done.contains(n)));
            if !closed {
                continue;
            }
            reachable_states += 1;
            let gate_forced = g.available_techs().contains(&gate);
            let rung = g.next_gate_rung(seat);
            assert!(
                !(gate_forced && rung.is_some()),
                "the {:?} force BOTH {} and {:?} with {:?} done: the reserved place would be two of {size}",
                kind,
                g.tables.tech(gate).name,
                rung.map(|t| g.tables.tech(t).name.clone()),
                g.research.done.iter().map(|t| g.tables.tech(*t).name.clone()).collect::<Vec<_>>()
            );
            if let Some(r) = rung {
                assert!(g.available_techs().contains(&r), "the {kind:?} are offered a rung whose own prerequisites are unmet");
                assert!(chain.contains(&r), "the {kind:?} are offered a rung that is not on their chain");
            }
            if g.available_techs().len() <= size {
                continue;
            }
            g.draw_shortlist(seat);
            assert_eq!(g.research.shortlist.len(), size, "the {kind:?} list is no longer {size} Techs long");
            if let Some(r) = rung {
                assert!(g.research.shortlist.contains(&r), "the {kind:?} rung was not drawn");
                assert!(!g.research.shortlist.contains(&gate), "the {kind:?} drew both their gate and a rung of its chain");
            }
        }
        assert!(reachable_states > 2, "the {kind:?} chain has states to walk: {reachable_states}");
    }
}

/// Ticket #348, R3: no Faction leaves a Tech until last that another Faction's Victory gate needs.
/// The tree is SHARED, so a deferral is not a private preference: the Prospectors deferred Clean
/// Power, an antecedent of BOTH the Arkwrights' and the Archivists' gates, and so stalled two
/// rivals' Victory Conditions every time they held the Research Lead without ever choosing to.
#[test]
fn no_faction_defers_a_tech_another_factions_gate_needs() {
    let t = tables();
    for kind in FactionKind::ALL {
        let picks = t.ai_tech_picks(kind);
        // Ticket #348 (version 0.09.1): BOTH levers, not `last` alone. A Faction's `last` defers a
        // Tech while that Faction holds the Research Lead, which stalls the whole table; its
        // `never` diverts that Faction's own Research out of the shared pot for as long as the
        // table researches it, which starves the Tech more slowly and just as surely.
        //
        // The first draft of this rule checked `last` alone, on a build specification that claimed
        // Green Consensus was nobody else's antecedent. It is the Custodians' ONLY rung-2
        // antecedent, on the one chain they have, and they are the weakest Faction on the board.
        // The designer, told that: drop it.
        for (lever, tech) in [("leave", picks.last), ("refuse to fund", picks.never)] {
            let Some(tech) = tech else { continue };
            for other in FactionKind::ALL.into_iter().filter(|k| *k != kind) {
                let gate = t.victory_gate(other).expect("every Faction has a Victory gate");
                assert!(
                    !t.gate_chain(other).contains(&tech),
                    "the {:?} {} {}, and the {:?} cannot reach {} without it",
                    kind,
                    lever,
                    t.tech(tech).name,
                    other,
                    t.tech(gate).name
                );
            }
        }
    }
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
        name: String::new(), id: rival, kind: UnitKind::Frigate, seat: Seat(1), damage: 0, at: ShipAt::Body(body), colonists: 0, warhead: false, colonists_education: 1.0, army: None,
        stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None,
    });
    // A warship of ours contests the orbit, so nobody holds Orbital Control and the ground is open:
    // that isolates the slot rule from the ground rule.
    let mine = ShipId(g.fresh_id());
    g.ships.push(Ship {
        name: String::new(), id: mine, kind: UnitKind::Frigate, seat: Seat(0), damage: 0, at: ShipAt::Body(body), colonists: 0, warhead: false, colonists_education: 1.0, army: None,
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
            name: String::new(), kind: UnitKind::Frigate, seat, damage: 0, at: ShipAt::Body(body), colonists: 0, warhead: false, colonists_education: 1.0, army: None,
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

/// Ticket #286 (version 0.08.5): the war is counted on the game, at the event, never scraped: a
/// Battle by its aggressor and against a neutral, an Occupation begun and a place taken by force,
/// an Army lost by the seat it fought for, a warship lost; and the counters ride through the save.
#[test]
fn the_war_is_counted_at_the_event() {
    let mut g = game();
    calm(&mut g);
    let target = StateId::NorthAfrica;
    assert_eq!(g.state(target).control, Control::Neutral);
    occupier_in(&mut g, StateId::EastAsia, target);
    g.resolution_phase();
    assert_eq!(g.war.battles[0], 1, "one Battle, opened by seat 0");
    assert_eq!(g.war.battles_vs_neutral, 1, "against a neutral Region's own Army");
    assert_eq!(g.war.battles[1..], [0, 0, 0], "nobody else opened one");
    // An Occupation begun and completed is a place taken by force.
    let mut g = game();
    calm(&mut g);
    g.armies.retain(|a| a.home != ArmyHome::State(StateId::Europe));
    occupier_in(&mut g, StateId::EastAsia, StateId::Europe);
    g.resolution_phase();
    assert_eq!(g.war.occupations_begun[0], 1);
    g.resolution_phase();
    g.resolution_phase();
    assert_eq!(g.state(StateId::Europe).control, Control::Controlled(Seat(0)));
    assert_eq!(g.war.takes_by_force[0], 1, "taken by force, once");
    assert_eq!(g.war.occupations_broken[0], 0);
    // Losses by the seat they fought for; a Standing Army under its own figure.
    let mut g = game();
    let standing = g.armies.iter().find(|a| a.standing && a.home == ArmyHome::State(StateId::EastAsia)).map(|a| a.id).unwrap();
    g.destroy_army(standing, "battle", None);
    assert_eq!((g.war.standing_armies_lost, g.war.armies_lost[0]), (1, 0));
    let built = ArmyId(g.fresh_id());
    g.armies.push(Army { name: String::new(), id: built, home: ArmyHome::State(StateId::EastAsia), at: ArmyAt::Place(Place::State(StateId::EastAsia)), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None, levy: false, raised_strength: 0 });
    g.destroy_army(built, "battle", None);
    assert_eq!(g.war.armies_lost[0], 1, "a built Army of seat 0's");
    let frigate = ShipId(g.fresh_id());
    g.ships.push(Ship { name: String::new(), id: frigate, kind: UnitKind::Frigate, seat: Seat(2), damage: 0, at: ShipAt::Body(BodyId::Mars), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
    g.destroy_ship(frigate, "battle");
    assert_eq!(g.war.warships_lost[2], 1);
    // Through the save.
    let saved = dying_earth_engine::save::SavedGame::of(&g);
    let back = saved.into_game(g.tables.clone());
    assert_eq!(back.war.armies_lost[0], 1, "the counters ride through the save");
}

/// Ticket #284 (version 0.08.5): every computer seat may attack a Region it did not lose, on the
/// Prospectors' odds, given a cause: a neutral needs none; a rival's Region only when the seat is
/// Cold or worse toward that rival; an occupier stays where it is; and the Battle line and the
/// Occupation agree that an escaped attacker is not alone at the place.
#[test]
fn a_cold_seat_marches_on_a_rivals_region_a_cordial_one_does_not_and_an_occupier_stays() {
    let mut g = game();
    calm(&mut g);
    let (home, target) = (StateId::EastAsia, StateId::Russia);
    assert!(g.tables.state(home).neighbours.contains(&target));
    assert_eq!(g.kind(Seat(0)), FactionKind::Custodians, "a seat the old rule forbade");
    // Russia held by the Prospectors, its Standing Army weak; seat 0 has a built Army in China.
    g.take_control(target, Seat(1));
    for a in g.armies.iter_mut().filter(|a| a.standing && a.home == ArmyHome::State(target)) {
        a.damage = 2;
    }
    // Ticket #302 (version 0.08.6): a raised Army worth 6 (an Industry-5 home), and the odds read
    // what the defenders FIGHT at -- their people's calm point, and Dig In for a neutral's.
    let army = ArmyId(g.fresh_id());
    g.armies.push(Army { name: String::new(), id: army, home: ArmyHome::State(home), at: ArmyAt::Place(Place::State(home)), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None, levy: false, raised_strength: 6 });
    let def: i64 = g.defenders_at(Place::State(target), Seat(0)).iter().filter_map(|id| g.army(*id)).map(|a| g.army_defended_strength(a)).sum();
    assert!(combat::first_round_odds(6, def) >= g.tables.ai.thresholds.attack_odds, "the odds clear the bar: {def}");
    let marches = |g: &mut Game| g.ai_orders(Seat(0)).iter().any(|o| matches!(o, Order::MoveArmy { army: a, to } if *a == army && *to == target));
    assert!(!marches(&mut g), "Neutral toward the Prospectors: no cause, no march");
    g.relations.score[0][1] = -8;
    assert!(g.relations_score(Seat(0), Seat(1)) <= g.tables.ai.thresholds.war_cause, "Cold or worse");
    assert!(marches(&mut g), "Cold toward the Prospectors: the Custodians march on Russia: {:?}", g.ai_orders(Seat(0)));
    // A neutral neighbour needs no cause.
    g.relations.score[0][1] = 0;
    g.take_control(target, Seat(1));
    let neutral = StateId::SouthAsia;
    assert_eq!(g.state(neutral).control, Control::Neutral);
    for a in g.armies.iter_mut().filter(|a| a.standing && a.home == ArmyHome::State(neutral)) {
        a.damage = 3;
    }
    assert!(g.ai_orders(Seat(0)).iter().any(|o| matches!(o, Order::MoveArmy { to, .. } if *to == neutral)), "a weak neutral next door is marched on with no cause: {:?}", g.ai_orders(Seat(0)));

    // An occupier stays: an Army at a place this seat occupies is offered no march.
    let mut g = game();
    calm(&mut g);
    g.armies.retain(|a| a.home != ArmyHome::State(StateId::Europe));
    let occ = occupier_in(&mut g, StateId::EastAsia, StateId::Europe);
    g.resolution_phase();
    assert!(matches!(g.state(StateId::Europe).control, Control::Occupied { occupier: Seat(0), .. }));
    assert!(!g.ai_orders(Seat(0)).iter().any(|o| matches!(o, Order::MoveArmy { army, .. } if *army == occ)), "the occupier holds what it takes: {:?}", g.ai_orders(Seat(0)));

    // Alone at the place: an escaped attacker is not.
    let mut g = game();
    calm(&mut g);
    g.armies.retain(|a| a.home != ArmyHome::State(StateId::Europe));
    let id = occupier_in(&mut g, StateId::EastAsia, StateId::Europe);
    assert!(g.alone_at(Place::State(StateId::Europe), Seat(0)), "on Attack, unescaped, nobody defending");
    g.army_mut(id).unwrap().escaped = true;
    assert!(!g.alone_at(Place::State(StateId::Europe), Seat(0)), "escaped, it occupies nothing and the line must not promise it");
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
    g.armies.push(Army { name: String::new(), id: army, home: ArmyHome::State(StateId::EastAsia), at: ArmyAt::Place(Place::State(next_door)), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None, levy: false, raised_strength: 0 });
    assert!(g.neutral_threatened(egypt), "a foreign built Army next door threatens it, whatever its stance");
    // Ticket #302 (version 0.08.6): the threat ARMS the Standing Army for good, where a Levy was raised.
    let cap = g.standing_army_cap(egypt);
    let steps = g.tables.standing_army.threat_steps;
    g.income_phase();
    assert_eq!(g.standing_army_cap(egypt), cap + steps, "two steps for good at the Income the threat appears");
    assert_eq!(g.armies.iter().filter(|a| a.standing && a.home == ArmyHome::State(egypt)).count(), 1, "one Army, not a second");
    assert!(g.report.lines.iter().any(|l| l.text.contains("Egypt arms")), "the Report says so: {:?}", g.report.lines);
    assert!(g.levies_raised >= 1, "counted for the sweep; every neutral bordering that Army arms, so more than Egypt may have");
    g.income_phase();
    assert_eq!(g.standing_army_cap(egypt), cap + steps, "once per threat episode, not once a turn");
    // The threat leaves and comes back: a second episode, two steps more; nothing stands down.
    g.armies.retain(|a| a.id != army);
    g.income_phase();
    assert_eq!(g.standing_army_cap(egypt), cap + steps, "armed for good");
    g.armies.push(Army { name: String::new(), id: army, home: ArmyHome::State(StateId::EastAsia), at: ArmyAt::Place(Place::State(next_door)), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None, levy: false, raised_strength: 0 });
    g.income_phase();
    assert_eq!(g.standing_army_cap(egypt), cap + 2 * steps, "a second episode arms it again");
    // A held Region is not a neutral: the threat arms nothing there.
    g.transfer_control(Place::State(egypt), Seat(1), "Influence");
    let held_cap = g.standing_army_cap(egypt);
    g.income_phase();
    assert_eq!(g.standing_army_cap(egypt), held_cap, "a held Region does not arm by threat");

    // Holding: +1 for good, with no ceiling.
    let mut g = game();
    calm(&mut g);
    let cap = g.standing_army_cap(egypt);
    g.neutral_held(egypt);
    assert_eq!(g.standing_army_cap(egypt), cap + g.tables.standing_army.held_step, "one step earned");
    assert!(g.report.lines.iter().any(|l| l.text.contains("Egypt held against the attack")), "{:?}", g.report.lines);
    for _ in 0..5 {
        g.neutral_held(egypt);
    }
    // Ticket #302 (version 0.08.6): no ceiling; the ceiling of Industry + 4 went with the Levy.
    assert_eq!(g.standing_army_cap(egypt), cap + 6 * g.tables.standing_army.held_step, "six holds, six steps");
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
/// name every unit and what it took, an aggressor carries its odds; when a unit died
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
    // Ticket #339 (version 0.09.0): the whole Battle's odds, in the words the attack button quotes
    // them in. The record said "first-round odds" until the button stopped quoting them.
    assert!(odds > 0.0 && odds < 1.0, "the whole Battle's odds: {odds}");
    let neutral = line.parties.iter().find(|p| p.seat.is_none()).expect("the neutral party");
    assert!(neutral.odds.is_none(), "a defender carries no odds");
    assert!(neutral.units.starts_with(&defender_name), "the party text names the unit: {}", neutral.units);
    assert!(neutral.units.contains("took") || neutral.units.contains("escaped") || neutral.units.contains("destroyed"), "and says what it took: {}", neutral.units);
    let lost: Vec<&String> = line.parties.iter().flat_map(|p| p.destroyed.iter()).collect();
    let report = g.report.lines.iter().find(|l| l.text.starts_with("Battle at Egypt")).expect("a Report line for the Battle");
    assert_eq!(report.place, Some(ReportPlace::State(target)), "the line jumps to the place");
    assert!(report.text.contains(&format!("{:.0}% odds of holding the field", odds * 100.0)), "and says the odds, labelled: {}", report.text);
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
            name: String::new(), kind: UnitKind::Frigate, seat, damage: 0, at: ShipAt::Body(BodyId::Mars), colonists: 0, warhead: false, colonists_education: 1.0, army: None,
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
        name: String::new(), id: rival, kind: UnitKind::Frigate, seat: Seat(1), damage: 0, at: ShipAt::Body(body), colonists: 0, warhead: false, colonists_education: 1.0, army: None,
        stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None,
    });
    // Ticket #335 (version 0.09.0): there is no Body at large. A Blockade may be given in LOW
    // ORBIT, which is where the ground is starved from; what is still refused is a stack sitting in
    // nothing but its own station's orbit, which would shut its holder out of nowhere.
    assert!(g.check_order(Seat(1), &[], &Order::ShipStance { body, stance: Stance::Blockade }).is_ok(), "low orbit is an orbit to blockade");
    let own = g.colonies.iter().find(|c| c.in_orbit && c.body == body && c.control.director() == Some(Seat(1))).map(|c| c.slot).expect("seat 1's own station");
    g.ship_mut(rival).unwrap().slot = Some(own);
    let err = g.check_order(Seat(1), &[], &Order::ShipStance { body, stance: Stance::Blockade }).unwrap_err().0;
    assert!(err.contains("no warship of yours sits in an orbit"), "{err}");
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
            name: String::new(), kind: UnitKind::Frigate, seat, damage: 0, at: ShipAt::Body(body), colonists: 0, warhead: false, colonists_education: 1.0, army: None,
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
        name: String::new(), id, kind: UnitKind::Frigate, seat: Seat(1), damage: 0, at: ShipAt::Body(body), colonists: 0, warhead: false, colonists_education: 1.0, army: None,
        stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: Some(slot),
    });
    // Ticket #355 (version 0.09.1): only WITH CAUSE, at the designer's word. A calm seat holds.
    let orders = g.ai_orders(Seat(1));
    assert!(!orders.iter().any(|o| matches!(o, Order::ShipStance { stance: Stance::Blockade, .. })), "no cause, no Blockade: {orders:?}");
    g.relations.score[1][0] = g.tables.ai.thresholds.war_cause - 5;
    assert!(g.relations_score(Seat(1), Seat(0)) <= g.tables.ai.thresholds.war_cause, "cause, now");
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
        name: String::new(), id: mine, kind: UnitKind::Frigate, seat: Seat(0), damage: 0, at: ShipAt::Body(body), colonists: 0, warhead: false, colonists_education: 1.0, army: None,
        stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 5, slot: Some(slot),
    });
    // Ticket #335 (version 0.09.0): at the station's own ring, which is the orbit it fuels from.
    assert!(g.check_order(Seat(0), &[], &Order::Refuel { ship: mine }).is_ok(), "an unblockaded station fuels it");
    let rival = ShipId(g.fresh_id());
    g.ships.push(Ship {
        name: String::new(), id: rival, kind: UnitKind::Frigate, seat: Seat(1), damage: 0, at: ShipAt::Body(body), colonists: 0, warhead: false, colonists_education: 1.0, army: None,
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

/// Ticket #355 (version 0.09.1): the computer reads an Attack ORBIT BY ORBIT, as it is fought. A
/// fleet with cause at a rival's station ring, strong enough for that station's Battery, attacks --
/// where the old reading, fixed on low orbit wherever the seat wanted the ground, saw no enemy at
/// the ring at all. Too weak for the Battery, it holds.
#[test]
fn the_computer_attacks_a_defended_ring_it_can_beat() {
    let mut g = game();
    calm(&mut g);
    let station = g.colonies.iter().find(|c| c.in_orbit && c.control.director() == Some(Seat(0))).map(|c| c.id).expect("a station over Earth");
    let (body, slot) = { let c = g.colony(station).unwrap(); (c.body, c.slot) };
    g.colony_mut(station).unwrap().modules.push(Module::new(ModuleKind::Battery));
    g.relations.score[1][0] = g.tables.ai.thresholds.war_cause - 5;
    // A Colony of its own on Earth's ground, so it WANTS the ground -- the case over Earth in play,
    // and the one the old low-orbit-only reading went blind in.
    colony(&mut g, Seat(1), BodyId::Earth, &[ModuleKind::Habitat], 2);
    assert!(g.ai_wants_the_ground(Seat(1), body));
    let attacks = |g: &mut Game| g.ai_orders(Seat(1)).iter().any(|o| matches!(o, Order::ShipStance { body: b, stance: Stance::Attack } if *b == body));
    ship_in(&mut g, Seat(1), UnitKind::Frigate, body, Some(slot), Stance::Hold);
    assert!(!attacks(&mut g), "one Frigate is no match for the Battery");
    for _ in 0..4 {
        ship_in(&mut g, Seat(1), UnitKind::Battleship, body, Some(slot), Stance::Hold);
    }
    assert!(attacks(&mut g), "a fleet that beats the Battery takes the ring");
}

/// Ticket #363 (version 0.09.1): a working Battery opens a Battle on a rival warship on BLOCKADE in
/// its own orbit, at the designer's word -- a defended station under Blockade is a fight, where the
/// Battery once merely voided the Blockade. Its holder opens it. A warship merely holding there, or a
/// mothballed Battery, starts nothing.
#[test]
fn a_battery_fires_on_a_blockader_in_its_orbit() {
    let board = |stance: Stance, battery_works: bool| {
        let mut g = game();
        calm(&mut g);
        let station = g.colonies.iter().find(|c| c.in_orbit && c.control.director() == Some(Seat(0))).map(|c| c.id).expect("a station over Earth");
        let slot = g.colony(station).unwrap().slot;
        let mut bat = Module::new(ModuleKind::Battery);
        bat.mothballed = !battery_works;
        g.colony_mut(station).unwrap().modules.push(bat);
        ship_in(&mut g, Seat(1), UnitKind::Frigate, BodyId::Earth, Some(slot), stance);
        g.resolution_phase();
        g
    };
    let g = board(Stance::Blockade, true);
    assert_eq!(g.war.orbit_attacks[0], 1, "the Battery's holder opened a Battle on the blockader");
    assert_eq!(board(Stance::Hold, true).war.orbit_attacks, [0; 4], "a warship holding there is not fired on");
    assert_eq!(board(Stance::Blockade, false).war.orbit_attacks, [0; 4], "a mothballed Battery fires on nobody");
}

/// Ticket #363 (version 0.09.1): **a Ship takes one order a turn, whichever is given first.** A
/// Launch refused a move given before it, but a move, a Refuel or a Transit given AFTER a Launch,
/// Rearm or Bombard was accepted -- so a carrier could fire and leave in one turn, and the move,
/// resolving first, carried it out of the orbit its Launch was given from. Traced: every computer
/// Launch over eighty games failed that way.
#[test]
fn an_order_after_a_launch_rearm_or_bombard_is_refused() {
    let mut g = game();
    let iss = g.colonies.iter().find(|c| c.in_orbit && c.body == BodyId::Earth && c.control.director() == Some(Seat(0))).map(|c| c.slot).expect("the ISS");
    let ship = carrier_in(&mut g, Seat(0), BodyId::Earth, Some(iss), true);
    g.seats[0].stockpile.fuel = 100;
    g.ship_mut(ship).unwrap().fuel = 10;
    // Each of the three is lawful on its own, so a refusal can only be the one-order rule.
    for t in [Order::ChangeOrbit { ship, slot: None }, Order::Transit { ship, to: BodyId::Moon, slot: None }, Order::Refuel { ship }] {
        assert!(g.check_order(Seat(0), &[], &t).is_ok(), "{t:?} alone is lawful");
    }
    let first = [Order::Launch { ship, target: Place::State(StateId::SouthAsia) }, Order::Rearm { ship }];
    let then = [Order::ChangeOrbit { ship, slot: None }, Order::Transit { ship, to: BodyId::Moon, slot: None }, Order::Refuel { ship }];
    for f in &first {
        for t in &then {
            let err = g.check_order(Seat(0), std::slice::from_ref(f), t).err().map(|e| e.0).unwrap_or_default();
            assert!(err.contains("already has an order"), "{t:?} after {f:?} must be refused, got {err:?}");
        }
    }
}

/// Ticket #358 (version 0.09.1): **Relay Networks is paid.** It showed a Relay's +1 on every card and
/// the Allotment never saw it, because the sum read the raw table row and not the Module's figures.
#[test]
fn relay_networks_is_paid_into_the_allotment() {
    let mut g = game();
    calm(&mut g);
    colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Relay, ModuleKind::Habitat], 4);
    let before = g.building_allotment(Seat(0));
    with_tech(&mut g, TechId::RelayNetworks);
    assert_eq!(g.building_allotment(Seat(0)), before + 1, "the Relay's second point reaches the Allotment");
}

/// Ticket #358: **the Chorus is paid, and its per-Colonist Influence sits OUTSIDE the Faction
/// multiplier**, on the Spaceport's argument (#183): the Arkwrights' x0.8 does not shave it.
#[test]
fn the_chorus_is_paid_at_face_value() {
    let mut g = game();
    calm(&mut g);
    let ark = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Arkwrights).expect("an Arkwright seat");
    let per = g.tables.unique.chorus_colonists as u32;
    let c = colony(&mut g, ark, BodyId::Moon, &[ModuleKind::Chorus, ModuleKind::Habitat, ModuleKind::Habitat], 1);
    let before = g.influence_allotment(ark);
    g.colony_mut(c).unwrap().colonists = per * 2;
    assert_eq!(g.influence_allotment(ark), before + 2, "two more Influence for twice {per} Colonists, at face value");
}

/// Ticket #358: **Climate charges a Facility's Emissions from the figure the screen shows**, so a
/// Clean Power read at half under the Archivists' Provisional Findings thins the smoke it shows
/// thinned. It charged only a Tech fully done.
#[test]
fn climate_charges_the_emissions_the_card_shows() {
    let mut g = game();
    calm(&mut g);
    let arc = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Archivists).expect("an Archivist seat");
    let sid = g.directed_states(arc)[0];
    g.state_mut(sid).facilities.push(Facility::new(FacilityKind::PowerPlant));
    assert!(g.state(sid).facilities.iter().any(|f| f.kind.common().unwrap_or(f.kind) == FacilityKind::PowerPlant && f.working()), "a Power Plant to read");
    let shown = |g: &Game| g.state(sid).facilities.iter().filter(|f| f.working()).map(|f| g.facility_yield(arc, sid, f.kind).emissions).sum::<f64>();
    let whole = g.emissions_now().power_plants;
    let shown_whole = shown(&g);
    g.seats[arc.index()].provisional_findings = true;
    g.research.findings_tech = Some(TechId::CleanPower);
    assert!(shown(&g) < shown_whole, "the card shows the half-read Tech");
    assert!(g.emissions_now().power_plants < whole, "and the air is charged it");
}

/// Ticket #362 (version 0.09.1): the Report tells the player of a pair INVOLVING THEM that fell into
/// a worse named level, both directions, folded one line each -- and nothing of a point's slide
/// within a level, nor of rivals' quarrels between themselves. The designer: *"quiet the 'has not
/// forgiven' spam."*
#[test]
fn the_report_tells_only_a_fall_into_a_worse_level_involving_the_player() {
    let mut g = game();
    calm(&mut g);
    let names: Vec<String> = Seat::ALL.iter().map(|s| g.seat_name(*s)).collect();
    // Seat 1 sits at the foot of Wary toward the player, and falls into Cold.
    g.relations.score[1][0] = -5;
    g.offend_by(Seat(0), Seat(1), 2);
    // Seat 2 sits in the middle of Cold toward the player, and slides a point within it.
    g.relations.score[2][0] = -6;
    g.offend_by(Seat(0), Seat(2), 1);
    // The player sits at Neutral's foot toward seat 3 and falls into Wary.
    g.relations.score[0][3] = -2;
    g.offend_by(Seat(3), Seat(0), 2);
    // Seats 1 and 2 quarrel between themselves, into Cold.
    g.relations.score[2][1] = -5;
    g.offend_by(Seat(1), Seat(2), 2);
    // The level a player reads carries the Blame term on top of the deeds, so each starting score is
    // walked until the LEVEL is the one this test means.
    for (v, o, want, edge) in [(1usize, 0usize, "Wary", true), (2, 0, "Cold", false), (0, 3, "Neutral", true), (2, 1, "Wary", true)] {
        while g.relations_level(Seat(v as u8), Seat(o as u8)) != want {
            g.relations.score[v][o] += if ["Hostile", "Cold", "Wary", "Neutral", "Cordial", "Friendly"].iter().position(|x| *x == g.relations_level(Seat(v as u8), Seat(o as u8))).unwrap() < ["Hostile", "Cold", "Wary", "Neutral", "Cordial", "Friendly"].iter().position(|x| *x == want).unwrap() { 1 } else { -1 };
        }
        // At a level's foot, one point takes it down; in the middle, one point does not.
        if edge {
            while g.relations_level(Seat(v as u8), Seat(o as u8)) == want {
                g.relations.score[v][o] -= 1;
            }
            g.relations.score[v][o] += 1;
        } else {
            g.relations.score[v][o] -= 1;
            assert_eq!(g.relations_level(Seat(v as u8), Seat(o as u8)), want, "still inside {want}");
            g.relations.score[v][o] += 1;
        }
    }
    g.report.lines.clear();
    g.settle_relations();
    let texts: Vec<String> = g.report.lines.iter().map(|l| l.text.clone()).collect();
    assert!(texts.iter().any(|t| t == &format!("The {} are now Cold toward you.", names[1])), "a rival's fall into Cold: {texts:?}");
    assert!(texts.iter().any(|t| t == &format!("You are now Wary of the {}.", names[3])), "the player's own fall into Wary: {texts:?}");
    assert!(!texts.iter().any(|t| t.contains(&names[2]) && t.contains("toward you")), "a slide within Cold says nothing: {texts:?}");
    assert!(!texts.iter().any(|t| t.contains("forgiven")), "the old line is retired: {texts:?}");
    assert_eq!(texts.iter().filter(|t| t.contains("are now") || t.contains("You are now")).count(), 2, "rivals' own quarrel says nothing: {texts:?}");
}

/// Ticket #361 (version 0.09.1): **the Archivists ferry.** While their uploads are short of the bar,
/// a loaded Colony Ship of theirs at Earth crosses to a Body off Earth rather than landing in
/// Antarctica or at their station over Earth, neither of which holds the Archive. Measured before: a
/// Colony on another Body stood in 19 games of 80.
#[test]
fn the_archivists_ferry_their_people_off_earth() {
    let mut g = game();
    calm(&mut g);
    g.antarctica_open = true;
    let arc = seat_of(&g, FactionKind::Archivists);
    let station = station_of(&g, arc, BodyId::Earth).expect("their station over Earth");
    g.colony_mut(station).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    g.colony_mut(station).unwrap().colonists = 0;
    let slot = g.colony(station).unwrap().slot;
    // The Archive's Colony already stands on the Moon, so the homeless lift of #68 no longer applies
    // and only the ferry keeps the Ships crossing.
    colony(&mut g, arc, BodyId::Moon, &[ModuleKind::Habitat, ModuleKind::Habitat], 4);
    let low = ship_in(&mut g, arc, UnitKind::ColonyShip, BodyId::Earth, None, Stance::Hold);
    g.ship_mut(low).unwrap().colonists = 4;
    let ring = ship_in(&mut g, arc, UnitKind::ColonyShip, BodyId::Earth, Some(slot), Stance::Hold);
    g.ship_mut(ring).unwrap().colonists = 4;
    // With a crossing on offer the old computer crossed too, so the case that parts them is the
    // one where no leg can be paid: the old computer's foothold rule then landed them on Earth
    // ("taken when the Ship cannot go anywhere better"); a ferrying Archivist holds them aboard.
    g.ship_mut(low).unwrap().fuel = 0;
    g.ship_mut(ring).unwrap().fuel = 0;
    g.seats[arc.index()].stockpile.fuel = 0;
    let orders = g.ai_orders(arc);
    let lands_on_earth = orders.iter().any(|o| match o {
        Order::Unload { ship, into: UnloadTarget::Slot(BodyId::Earth, _), .. } => *ship == low || *ship == ring,
        Order::Unload { ship, into: UnloadTarget::Colony(c), .. } => (*ship == low || *ship == ring) && *c == station,
        _ => false,
    });
    assert!(!lands_on_earth, "no landing on or over Earth while they ferry: {orders:?}");
}

/// Ticket #99: a transit names the Orbital Slot it arrives into, and refuses a slot the Body has not
/// got. The choice is made with the leg, so it is made before the Ship can see who will be there.
#[test]
fn a_transit_names_the_slot_it_arrives_into() {
    let mut g = game();
    let ship = ShipId(g.fresh_id());
    g.ships.push(Ship {
        name: String::new(), id: ship, kind: UnitKind::Frigate, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, warhead: false, colonists_education: 1.0, army: None,
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
    answer_the_card(&mut g);
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
    raised_strength: 0,
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
    // Ticket #290 (version 0.08.6): the two aboard would blend into the mean; the test is about
    // the schooling the four carry, so the station is emptied first.
    bare_stations(&mut g);
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
    // Ticket #333 (version 0.09.0): 5,000 units of one million, the five billion that 1,000 units
    // of five million were, read off facilities.toml.
    assert_eq!(g.tables.population_factor.population_per_point, 5000.0);
    let bonus = pop / 5000.0;

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
    let want = (base * (1.0 + pop / 5000.0 * taught) * taught * mult).floor() as i64;
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
        assert_eq!((u.materials, u.widgets, u.energy_upkeep, u.no_slot), (c.materials, c.widgets, c.energy_upkeep, c.no_slot), "{}", unique.name());
        assert!((u.emissions - c.emissions).abs() < 1e-9, "{}", unique.name());
        assert_eq!(u.produces.as_ref().map(|p| (p.resource, p.amount)), c.produces.as_ref().map(|p| (p.resource, p.amount)), "{}", unique.name());
    }
    // And off Earth, the Academy against the Institute.
    let a = g.tables.module(ModuleKind::Academy);
    let i = g.tables.module(ModuleKind::Institute);
    assert_eq!((a.materials, a.widgets, a.energy_upkeep), (i.materials, i.widgets, i.energy_upkeep));
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
        assert_eq!((u.materials, u.widgets, u.energy_upkeep), (c.materials, c.widgets, c.energy_upkeep), "{} is priced as its sibling", unique.name());
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
        // Ticket #358 (version 0.09.1): the base point inside the Faction multiplier, the
        // per-Colonist points outside it; together, the figure the card prints.
        let y = g.module_yield(ark, cid, ModuleKind::Chorus);
        assert_eq!((y.allotment, y.allotment + y.allotment_outside), (1, chorus), "a Chorus at a Colony of {people}");
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
    g.state_mut(sid).control = Control::Occupied { occupier: cus, previous: Some(pro), turns: 1, banked: 0 };
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
    let picture = |g: &Game| -> Vec<(i64, i64, i64, u32, usize)> {
        Seat::ALL
            .into_iter()
            .map(|s| {
                let st = g.seat(s);
                (st.stockpile.materials, st.stockpile.ducats, st.venture_fund, st.allotment as u32, g.controlled_states(s).len())
            })
            .collect()
    };
    // Ticket #332 (version 0.09.0): seven turns, where six parted the boards before. Widgets
    // slow the opening builds, so the first credit purchase of the neutral game, which is the
    // divergence, comes a turn later; measured at seven on this seed when the ticket was built.
    for _ in 0..7 {
        pick_a_tech(&mut a);
        answer_the_card(&mut a);
        pick_a_tech(&mut b);
        answer_the_card(&mut b);
        a.end_turn(std::array::from_fn(|_| Vec::new())).unwrap();
        b.end_turn(std::array::from_fn(|_| Vec::new())).unwrap();
    }
    // The hostile game proposes no purchase at all -- the Custodians will not sell to a Hostile
    // buyer -- so the computer seats' choices, and with them the board, part from the neutral game's.
    assert_ne!(picture(&a), picture(&b), "seven turns of the worst possible blood now change the board: {:?}", picture(&a));
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

    // Ticket #361 (version 0.09.1): The Upload gates the WIN, not the order, at the designer's word
    // -- as before ticket #199, whose premise reversed: measured, The Upload now lands at a median
    // turn 28 and an Archivist Colony off Earth at 22. The order stands without it.
    assert!(!g.has_tech(TechId::TheUpload), "the premise: The Upload is not researched");
    g.colony_mut(cid).unwrap().colonists = 4;
    assert!(g.check_order(arc, &[], &order).is_ok(), "the Archive is ordered before The Upload");
    assert!(g.tables.victory_gate(FactionKind::Archivists) == Some(TechId::TheUpload), "and The Upload still gates their win");

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
    // Ticket #332 (version 0.09.0): an empty station still makes its Core Module's four Widgets a
    // turn, so the Archive's twelve land in three Resolutions, well inside the twelve run here.
    for _ in 0..g.tables.module(ModuleKind::Archive).widgets {
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
    let research = g.tables.archive.research;
    let cid = archive_at(&mut g, arc, BodyId::Mars, research, 12);
    // Twelve standing beside a finished Archive, and nobody read in: no win.
    g.seats[arc.index()].uploaded = 0;
    open_gates(&mut g);
    g.seats[arc.index()].stockpile.energy = 200;
    g.income_phase();
    assert!(g.archive_online(arc), "the premise: the Archive is running");
    let p = g.progress(arc);
    assert_eq!((p.first_value, p.second_value), (research as f64, 0.0), "standing next to it is worth nothing now");
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
        warhead: false,
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
            warhead: false,
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
    assert_eq!(total, 697, "the whole tree since ticket #343's Missile Technology (48 on rung 3); 649 from #232, 585 from #231, 554 from #201, 507 before that");
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
    // Ticket #332 (version 0.09.0): the turns are an estimate at each place's Widgets. Europe at
    // Industry Level 3 makes 7 a turn with no Factory (a flat 4 and 3 for the levels), so a Power
    // Plant of 8 is two Resolutions off; the Moon Colony makes its Core Module's 4, so a Mine with
    // one Widget left lands next turn.
    assert_eq!(g.widgets_at(Place::State(sid)), 7, "a flat 4 and Europe's Industry Level");
    g.state_mut(sid).queue.push(Build { item: BuildItem::Facility(FacilityKind::PowerPlant), seat: Seat(0), widgets: 8, done: 0, coastal: false });
    let cid = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Habitat], 4);
    assert_eq!(g.widgets_at(Place::Colony(cid)), 4, "the Core Module's four");
    g.colony_mut(cid).unwrap().queue.push(Build { item: BuildItem::Module(ModuleKind::Mine), seat: Seat(0), widgets: 4, done: 3, coastal: false });
    // A rival's build in a Region the player directs is the rival's, not the player's.
    g.state_mut(sid).queue.push(Build { item: BuildItem::Facility(FacilityKind::Bank), seat: Seat(1), widgets: 4, done: 0, coastal: false });
    // Two Ships: one on the road to Mars with three turns left, one arriving next turn, and one at rest.
    let put = |g: &mut Game, kind: UnitKind| -> ShipId {
        let id = ShipId(g.fresh_id());
        let name = g.next_ship_name(kind);
        g.ships.push(Ship { name, id, kind, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
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
    assert_eq!(u.builds[1], ("Power Plant".to_string(), Place::State(sid), 2), "{:?}", u.builds);
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
    // Presence 9 of 12 (0.75) and the Fund at 2000 of 2500 (0.8): the score is 0.75. Ticket #290
    // (version 0.08.6): Tiangong's two aboard would make it 11 of 12, so it is emptied first.
    bare_stations(&mut g);
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
    assert_eq!(MomentKind::ALL.len(), 11, "ticket #281 (version 0.08.5) added a place taken by force, and #345 (0.09.1) a Body settled first");
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

// ---------------------------------------------------------------- #290 (version 0.08.6) the opening board

/// Ticket #290 (version 0.08.6): every starting station opens with two Colonists aboard, from
/// nowhere, so it has two Module slots free at once and two berths left in its Core Module; the
/// Arkwrights, who have no station, open with two Pioneers waiting in their start Region, a gift
/// outside Coach Class that takes no population. A station BUILT during the game is still bare.
#[test]
fn a_starting_station_opens_with_two_aboard_and_the_arkwrights_with_two_pioneers() {
    let g = fresh();
    for (seat, name) in [(Seat(0), "ISS over Earth"), (Seat(1), "Tiangong over Earth"), (Seat(3), "Axiom over Earth")] {
        let id = station_of(&g, seat, BodyId::Earth).expect("a starting station");
        let c = g.colony(id).unwrap();
        assert_eq!(g.place_name(Place::Colony(id)), name);
        assert_eq!(c.colonists, 2, "two aboard from the start");
        assert_eq!(g.free_module_slots(c), 2, "one slot per Colonist, so two to build in at once");
        assert_eq!(g.habitat_room(c) - c.colonists, 2, "two berths of the Core Module's four left");
    }
    let ark = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Arkwrights).unwrap();
    assert!(station_of(&g, ark, BodyId::Earth).is_none(), "the Arkwrights still start with no station");
    for sid in StateId::ALL {
        let st = g.state(sid);
        let card = g.tables.state(sid);
        assert_eq!(st.population, card.population, "no start Region is debited for anybody's people");
        if st.control == Control::Controlled(ark) {
            assert_eq!(st.emigrants, 2, "two Pioneers waiting in the Arkwrights' start Region");
        } else {
            assert_eq!(st.emigrants, 0, "nobody else starts with a Pioneer waiting");
        }
    }
}

/// Ticket #295 (version 0.08.6): the disengage roll's divisor is the table's, and the table says
/// three: a unit at half its hit points leaves one time in six, where the First Playable's two made
/// it one in four. Evade's flat half is untouched.
#[test]
fn the_disengage_roll_is_a_third_of_the_damage_fraction_from_the_table() {
    let g = fresh();
    let d = g.tables.disengage.divisor;
    assert_eq!(d, 3.0, "the designer's word: a third");
    let mut c = frigate(1);
    c.damage = 2;
    assert!((combat::disengage_chance(&c, d) - 1.0 / 6.0).abs() < 1e-12, "half its hit points: one in six");
    c.evade = true;
    c.damage = 0;
    assert_eq!(combat::disengage_chance(&c, d), 0.5, "Evade is still a flat half");
}

/// Ticket #296 (version 0.08.6), rewritten by ticket #302: a Region's defence is its people, AS
/// DEFENCE. A Standing Army's strength and hit points are Industry + 1 plus its armed steps; while
/// it defends it fights one stronger for a working Constabulary and one while Unrest is under the
/// threshold, both live and neither adding hit points; an Army whose damage reaches its strength
/// is destroyed at Income rather than sitting at strength nought. A raised Army is its home's
/// Industry + 1, fixed; a Colony's the average of its Faction's Regions + 1.
#[test]
fn a_standing_army_reads_its_industry_and_defends_with_its_people_and_dies_at_its_strength() {
    let mut g = fresh();
    calm(&mut g);
    let sid = StateId::EastAsia;
    let industry = g.state(sid).industry_level;
    assert_eq!(industry, 3, "the fixture: China at Industry 3");
    let id = g.armies.iter().find(|a| a.standing && a.home == ArmyHome::State(sid)).map(|a| a.id).unwrap();
    let army = |g: &Game| g.armies.iter().find(|a| a.id == id).unwrap().clone();
    // Calm, no Constabulary: strength and hit points Industry + 1; defends one stronger for calm.
    assert_eq!(g.army_strength(&army(&g)), (industry + 1) as i64, "Industry + 1, and nothing for calm in the strength");
    assert_eq!(g.army_hit_points(&army(&g)), industry + 1, "hit points equal the strength");
    assert_eq!(g.army_defended_strength(&army(&g)), (industry + 2) as i64, "defends one stronger for calm");
    // A working Constabulary: one more to the defence, none to the body.
    g.state_mut(sid).facilities.push(Facility { online: true, ..Facility::new(FacilityKind::Constabulary) });
    assert!(g.constabulary_online(sid));
    assert_eq!(g.army_hit_points(&army(&g)), industry + 1, "the police add no hit points");
    assert_eq!(g.army_defended_strength(&army(&g)), (industry + 3) as i64, "and one more to the defence");
    // Restive: the calm point goes, live.
    g.state_mut(sid).unrest = g.tables.unrest.army_threshold;
    assert_eq!(g.army_defended_strength(&army(&g)), (industry + 2) as i64, "Unrest at the threshold takes the calm point");
    // Damage reaching the strength destroys it at Income and starts the two-Income return.
    g.army_mut(id).unwrap().damage = industry + 1;
    assert_eq!(g.army_strength(&army(&g)), 0);
    g.income_phase();
    assert!(!g.armies.iter().any(|a| a.id == id), "at its strength in damage it is destroyed, not left at nought");
    assert_eq!(g.state(sid).respawn_wait, 1, "and returns two Incomes later, as a destroyed one does");
    // A raised Army is its home's Industry + 1, fixed at the raise; it defends with nothing of its own.
    let mut g = fresh();
    calm(&mut g);
    let built = g.raise_army(Place::State(sid), false);
    let b = g.armies.iter().find(|a| a.id == built).unwrap().clone();
    assert_eq!((g.army_strength(&b), g.army_hit_points(&b)), ((industry + 1) as i64, industry + 1), "its home's Industry + 1, both figures");
    assert_eq!(g.army_defended_strength(&b), (industry + 1) as i64, "a raised Army has no people to defend with");
    g.state_mut(sid).industry_level += 2;
    assert_eq!(g.army_strength(&b), (industry + 1) as i64, "fixed at the raise");
    // A Colony's Army: the rounded average Industry of its Faction's Regions, plus one.
    let cid = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Habitat, ModuleKind::Barracks], 4);
    let held: Vec<u32> = g.controlled_states(Seat(0)).into_iter().map(|s| g.state(s).industry_level).collect();
    let avg = (held.iter().sum::<u32>() + held.len() as u32 / 2) / held.len() as u32;
    let garrison = g.raise_army(Place::Colony(cid), false);
    let c = g.armies.iter().find(|a| a.id == garrison).unwrap().clone();
    assert_eq!((g.army_strength(&c), g.army_hit_points(&c)), ((avg + 1) as i64, avg + 1), "the average Industry of the Faction's Regions, plus one");
}

/// Ticket #300 (version 0.08.6): a landed Army may Attack on the turn it lands. Landed at a
/// rival's Colony it lands on Attack and fights in the same Resolution, after the orbit; landed
/// where nobody defends, it occupies the Colony the same turn; landed at its own, it lands on Hold.
#[test]
fn an_army_landed_at_a_rivals_colony_fights_or_occupies_the_turn_it_lands() {
    // Nobody defends: the Occupation begins the turn of the landing.
    let mut g = game();
    calm(&mut g);
    let cid = colony(&mut g, Seat(1), BodyId::Moon, &[ModuleKind::Habitat], 4);
    let army = ArmyId(g.fresh_id());
    let ship = ShipId(g.fresh_id());
    g.armies.push(Army { name: String::new(), id: army, home: ArmyHome::State(StateId::EastAsia), at: ArmyAt::Aboard(ship), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None, levy: false, raised_strength: 0 });
    g.ships.push(Ship { name: String::new(), id: ship, kind: UnitKind::Carrier, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Moon), colonists: 0, warhead: false, colonists_education: 1.0, army: Some(army), stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
    g.commit_orders(Seat(0), &[Order::Unload { ship, colonists: 0, army: true, into: UnloadTarget::Colony(cid) }]);
    g.resolution_phase();
    let a = g.army(army).unwrap();
    assert_eq!(a.at, ArmyAt::Place(Place::Colony(cid)), "landed");
    assert_eq!(a.stance, Stance::Attack, "at a rival's Colony it lands on Attack");
    assert!(matches!(g.colony(cid).unwrap().control, Control::Occupied { occupier: Seat(0), .. }), "alone at the place, it occupies the same turn: {:?}", g.colony(cid).unwrap().control);
    assert_eq!(g.war.armies_landed[0], 1, "counted");
    // Defended: the Battle is fought the turn of the landing.
    let mut g = game();
    calm(&mut g);
    let cid = colony(&mut g, Seat(1), BodyId::Moon, &[ModuleKind::Habitat, ModuleKind::Barracks], 4);
    let defender = ArmyId(g.fresh_id());
    g.armies.push(Army { name: String::new(), id: defender, home: ArmyHome::Colony(cid), at: ArmyAt::Place(Place::Colony(cid)), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None, levy: false, raised_strength: 0 });
    let army = ArmyId(g.fresh_id());
    let ship = ShipId(g.fresh_id());
    g.armies.push(Army { name: String::new(), id: army, home: ArmyHome::State(StateId::EastAsia), at: ArmyAt::Aboard(ship), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None, levy: false, raised_strength: 0 });
    g.ships.push(Ship { name: String::new(), id: ship, kind: UnitKind::Carrier, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Moon), colonists: 0, warhead: false, colonists_education: 1.0, army: Some(army), stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
    let battles = g.war.battles[0];
    g.commit_orders(Seat(0), &[Order::Unload { ship, colonists: 0, army: true, into: UnloadTarget::Colony(cid) }]);
    g.resolution_phase();
    assert_eq!(g.war.battles[0], battles + 1, "the Battle was fought the turn it landed");
    // At its own Colony it lands on Hold.
    let mut g = game();
    calm(&mut g);
    let cid = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Habitat], 4);
    let army = ArmyId(g.fresh_id());
    let ship = ShipId(g.fresh_id());
    g.armies.push(Army { name: String::new(), id: army, home: ArmyHome::State(StateId::EastAsia), at: ArmyAt::Aboard(ship), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None, levy: false, raised_strength: 0 });
    g.ships.push(Ship { name: String::new(), id: ship, kind: UnitKind::Carrier, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Moon), colonists: 0, warhead: false, colonists_education: 1.0, army: Some(army), stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
    g.commit_orders(Seat(0), &[Order::Unload { ship, colonists: 0, army: true, into: UnloadTarget::Colony(cid) }]);
    g.resolution_phase();
    assert_eq!(g.army(army).unwrap().stance, Stance::Hold, "at its own Colony it lands on Hold");
}

/// Ticket #299 (version 0.08.6): an Occupation that must be held. A break -- here the last Army
/// gone -- hands the place back at +2 Unrest, charges the occupier a rung-2 offence from the
/// previous holder, and wipes the Standing the Occupation banked; a neutral previous holder
/// charges nobody. The march stays legal.
#[test]
fn a_broken_occupation_hands_the_place_back_at_a_cost() {
    let mut g = game();
    calm(&mut g);
    // Europe is the Prospectors' (seat 1). Seat 0 held 10 Standing there before it marched.
    let europe = StateId::Europe;
    assert_eq!(g.state(europe).control, Control::Controlled(Seat(1)));
    g.armies.retain(|a| a.home != ArmyHome::State(europe));
    g.seats[0].influence.insert(Place::State(europe), 10);
    let a = occupier_in(&mut g, StateId::EastAsia, europe);
    g.resolution_phase();
    let Control::Occupied { occupier: Seat(0), banked, .. } = g.state(europe).control else { panic!("occupied: {:?}", g.state(europe).control) };
    assert!(banked > 0, "the first turn banked Standing");
    assert_eq!(g.seats[0].influence[&Place::State(europe)], 10 + banked);
    // The march is still legal for an occupier.
    let to = g.tables.state(europe).neighbours.iter().copied().find(|n| *n != StateId::EastAsia).unwrap();
    assert!(g.check_order(Seat(0), &[], &Order::MoveArmy { army: a, to }).is_ok(), "an occupier may march away");
    let unrest_before = g.state(europe).unrest;
    let owed_before = g.relations.owed[1][0];
    g.armies.retain(|x| x.id != a);
    g.resolution_phase();
    assert_eq!(g.state(europe).control, Control::Controlled(Seat(1)), "handed back");
    assert!((g.state(europe).unrest - unrest_before - g.tables.unrest.occupation_break).abs() < 1e-9 || g.state(europe).unrest >= unrest_before + g.tables.unrest.occupation_break - 1e-9, "+2 Unrest: {} from {}", g.state(europe).unrest, unrest_before);
    assert_eq!(g.relations.owed[1][0] - owed_before, g.tables.relations.occupation_broken_offence, "a rung-2 offence from the previous holder");
    let left = g.seats[0].influence[&Place::State(europe)];
    assert!((7..=10).contains(&left), "the banked Standing is wiped; what is left is the 10 from before less the turn's ordinary decay: {left}");
    assert!(g.report.lines.iter().any(|l| l.text.contains("Occupation of The European Union broke")), "{:?}", g.report.lines.iter().map(|l| l.text.clone()).collect::<Vec<_>>());
    // A neutral previous holder charges nobody.
    let mut g = game();
    calm(&mut g);
    let egypt = StateId::NorthAfrica;
    g.armies.retain(|a| a.home != ArmyHome::State(egypt));
    let a = occupier_in(&mut g, StateId::EastAsia, egypt);
    g.resolution_phase();
    let owed: i64 = (0..4).map(|v| g.relations.owed[v][0]).sum();
    g.armies.retain(|x| x.id != a);
    g.resolution_phase();
    assert_eq!(g.state(egypt).control, Control::Neutral);
    assert_eq!((0..4).map(|v| g.relations.owed[v][0]).sum::<i64>(), owed, "nobody to offend");
}

/// Ticket #298 (version 0.08.6): a place taken whole. With the roll set to a certainty: a Battle
/// burns nothing; a transfer by Pacified burns nothing and fires the Moment for a place taken by
/// force all the same; a transfer by the three-turn clock burns every building, the Unique among
/// them; a take by Influence never rolled.
#[test]
fn a_battle_burns_nothing_a_pacified_transfer_takes_the_place_whole_and_the_clock_burns() {
    let mut t = Tables::load(&default_data_dir()).expect("tables load");
    t.influence.destruction_chance = 1.0;
    let mut g = Game::new(Arc::new(t), NewGame { seed: 7, player: FactionKind::Custodians, player_is_ai: false, player_start: StateId::EastAsia });
    calm(&mut g);
    let europe = StateId::Europe;
    let standing = g.state(europe).facilities.len();
    assert!(standing >= 3, "the fixture: Europe keeps its start Facilities ({standing})");
    let taken = |g: &Game| g.report.moments.iter().filter(|m| m.kind == MomentKind::PlaceTakenByForce).count();
    // A Battle: seat 0's Army attacks Europe's Standing Army. Every building would have burned
    // at a certain roll; none does, because the Battle no longer rolls.
    occupier_in(&mut g, StateId::EastAsia, europe);
    g.resolution_phase();
    assert!(g.war.battles[0] >= 1, "a Battle was fought: {:?}", g.war.battles);
    assert_eq!(g.state(europe).facilities.len(), standing, "the Battle burned nothing");
    // A transfer by Pacified: taken whole, and the Moment fires with nothing lost.
    let before = taken(&g);
    g.transfer_control(Place::State(europe), Seat(0), "Pacified");
    assert_eq!(g.state(europe).facilities.len(), standing, "Pacified takes the place whole");
    assert_eq!(taken(&g), before + 1, "and it is still a place taken by force");
    assert!(g.report.moments.iter().any(|m| m.text.contains("taken whole")), "{:?}", g.report.moments.iter().map(|m| m.text.clone()).collect::<Vec<_>>());
    // A transfer by the clock: at a certain roll, everything burns, the Unique Facility included.
    g.state_mut(europe).facilities.push(Facility { online: true, ..Facility::new(FacilityKind::InvestmentBank) });
    assert!(g.state(europe).facilities.iter().any(|f| f.kind.unique_to().is_some()), "a Unique stands there");
    g.transfer_control(Place::State(europe), Seat(1), "Occupation complete");
    assert_eq!(g.state(europe).facilities.len(), 0, "the clock burns, the Unique like any other");
    assert_eq!(taken(&g), before + 2);
}

/// Ticket #297 (version 0.08.6): Dig In. A dug-in Army fights at +2 while defending and never
/// rolls to disengage; hit points do not follow; it cannot march or board a Carrier until its
/// stance is changed and the turn has passed; a neutral Region's own Army is always dug in; a
/// held Region's is not until ordered.
#[test]
fn a_dug_in_army_fights_two_stronger_never_disengages_and_cannot_march_until_it_digs_out() {
    // In the melee: a dug-in unit at three damage of four would roll at (3/4)/2; scripted true,
    // it would leave. Dug in, no roll is made and it stays. Hit rolls: attacker lands none.
    let mut a = vec![frigate(1)];
    let mut d = vec![Combatant::new(UnitRef::Army(ArmyId(2)), "the 1st Trench Army", 4, 4, 3, 2, false).dug_in(true)];
    let mut dice = Script { chances: VecDeque::from(vec![false, false, false, true, true, true, true, true, true]), d6s: VecDeque::from(vec![1, 1, 1]), picks: VecDeque::new() };
    let stats = combat::fight(&mut a, &mut d, &mut dice, 2.0);
    assert!(!d[0].escaped, "dug in, it never rolls to disengage");
    assert!(stats.all_escaped().is_empty());

    let mut g = fresh();
    calm(&mut g);
    let sid = StateId::EastAsia;
    let standing_id = g.armies.iter().find(|a| a.standing && a.home == ArmyHome::State(sid)).map(|a| a.id).unwrap();
    assert!(!g.army_dug_in(g.army(standing_id).unwrap()), "a held Region's Standing Army is not dug in until ordered");
    // Ticket #302 (version 0.08.6): the march is a raised Army's; a Region's own stays at home.
    let id = g.raise_army(Place::State(sid), false);
    let army = |g: &Game| g.armies.iter().find(|a| a.id == id).unwrap().clone();
    let egypt = StateId::NorthAfrica;
    let neutral = g.armies.iter().find(|a| a.standing && a.home == ArmyHome::State(egypt)).unwrap().clone();
    assert_eq!(g.state(egypt).control, Control::Neutral);
    assert!(g.army_dug_in(&neutral), "a neutral's is always dug in");
    // Ordered to Dig In: the stance lands at commit, the bonus is a defender's in the Battle.
    let before = g.army_strength(&army(&g));
    g.commit_orders(Seat(0), &[Order::ArmyStance { place: Place::State(sid), stance: Stance::DigIn }]);
    assert!(g.army_dug_in(&army(&g)));
    assert_eq!(g.army_strength(&army(&g)), before, "the card's strength does not carry the bonus");
    assert_eq!(g.army_hit_points(&army(&g)), before as u32, "and the hit points do not follow it");
    assert_eq!(g.war.dig_ins[0], 1, "counted for the sweep");
    assert!(g.report.lines.iter().any(|l| l.text.contains("dig in at China")), "{:?}", g.report.lines.iter().map(|l| l.text.clone()).collect::<Vec<_>>());
    // Dug in, it may not march, nor be loaded; a stance change this turn does not lift that.
    let to = g.tables.state(sid).neighbours[0];
    let march = Order::MoveArmy { army: id, to };
    let refused = g.check_order(Seat(0), &[], &march).unwrap_err().0;
    assert!(refused.contains("dug in"), "{refused}");
    let refused = g.check_order(Seat(0), &[Order::ArmyStance { place: Place::State(sid), stance: Stance::Hold }], &march).unwrap_err().0;
    assert!(refused.contains("dug in"), "a stance order beside it does not lift it this turn: {refused}");
    // Dug out at the Resolution, it marches the turn after.
    g.commit_orders(Seat(0), &[Order::ArmyStance { place: Place::State(sid), stance: Stance::Hold }]);
    assert!(g.check_order(Seat(0), &[], &march).is_ok(), "the stance changed and the turn passed");
    // A Ship cannot dig in.
    let ship = a_colony_ship(&mut g, Seat(0), BodyId::Earth);
    let _ = ship;
    let refused = g.check_order(Seat(0), &[], &Order::ShipStance { body: BodyId::Earth, stance: Stance::DigIn }).unwrap_err().0;
    assert!(refused.contains("cannot dig in"), "{refused}");
}

/// Ticket #290 (version 0.08.6): the computer's opening. While its starting station has a slot free
/// and under four berths empty, the Habitat is pushed at the opportunity weight, so it comes first:
/// on turn one every seat with a station orders one there.
#[test]
fn the_computer_opens_with_a_habitat_on_its_starting_station() {
    let mut g = fresh();
    for seat in Seat::ALL {
        let Some(id) = station_of(&g, seat, BodyId::Earth) else { continue };
        let orders = g.ai_orders(seat);
        assert!(
            orders.iter().any(|o| matches!(o, Order::BuildModule { colony, kind: ModuleKind::Habitat } if *colony == id)),
            "the {} should open with a Habitat on their station: {orders:?}",
            g.kind(seat).name()
        );
    }
}



// -------------------------------------------- 0.09.0 ticket #332: the Factory Module on a station

/// Ticket #332 (version 0.09.0): the designer's word was that Colonies AND stations have a Widget
/// maker of their own -- *"for colonies and stations to have a counterpart"* -- so the Factory
/// Module stands on a station as it stands on a Colony. Measured before this was true: a
/// station's Core made one Widget a turn for the whole game, a Shipyard there took eight turns,
/// no warship was built in eighty games and no station stood off Earth at the end.
#[test]
fn a_factory_module_may_stand_on_a_station() {
    let mut g = fresh();
    let iss = station_of(&g, Seat(0), BodyId::Earth).expect("the Custodians start with a station over Earth");
    g.seat_mut(Seat(0)).stockpile.materials = 200;
    let order = Order::BuildModule { colony: iss, kind: ModuleKind::Factory };
    assert!(g.check_order(Seat(0), &[], &order).is_ok(), "a Factory Module is legal on a station: {:?}", g.check_order(Seat(0), &[], &order));
    assert!(ModuleKind::Factory.stands_on_a_station(), "the computer's list agrees");
}


/// Ticket #332 (version 0.09.0): a Factory on Earth makes Widgets, not Materials, and Widgets a
/// Region does not spend are lost. So the computer wants a Factory only where its queue is
/// `factory_module_queue_depth` deep, as it wants the Factory Module, and a seat short of
/// Materials is pulled toward the Mine and never the Factory. Measured before this held: 541
/// Factories completed to 283 Mines over twenty games, and a Custodian seat's Materials income
/// over a whole game was 265.
#[test]
fn the_computer_builds_no_factory_where_nothing_is_queued() {
    let mut g = fresh();
    g.seat_mut(Seat(0)).stockpile.materials = 300;
    g.seat_mut(Seat(0)).stockpile.energy = 100;
    for turn in 0..6 {
        let orders = g.ai_orders(Seat(0));
        assert!(
            !orders.iter().any(|o| matches!(o, Order::BuildFacility { kind: FacilityKind::Factory, .. })),
            "turn {turn}: no Region of the Custodians has a two-deep queue, so no Factory is ordered: {orders:?}"
        );
        pick_a_tech(&mut g);
        answer_the_card(&mut g);
        g.end_turn(std::array::from_fn(|s| if s == 0 { orders.clone() } else { Vec::new() })).unwrap();
    }
}


// -------------------------------------------- 0.08.7 ticket #310: the threat line

/// Ticket #310 (version 0.08.7): the military threat to a held Region is the rival RAISED Army
/// next door with the best first-exchange odds against the Region's defenders. None with nobody
/// next door; a neutral neighbour's own Army is nobody's; a rival's Standing Army never marches and
/// so never counts; and a Region nobody holds has no threat line.
#[test]
fn the_nearest_army_threat_is_the_rival_raised_army_next_door_with_the_best_odds() {
    let mut g = game();
    let china = StateId::EastAsia;
    assert_eq!(g.state(china).control, Control::Controlled(Seat(0)));
    assert!(g.nearest_army_threat(china).is_none(), "nobody stands next door on a fresh board");
    let next_door = g.tables.state(china).neighbours.iter().copied().find(|n| g.state(*n).control == Control::Neutral).expect("a neutral neighbour");
    g.take_control(next_door, Seat(1));
    assert!(g.nearest_army_threat(china).is_none(), "a rival's Standing Army next door never marches, so it is no threat");
    let raised = g.raise_army(Place::State(next_door), false);
    let (id, from, odds) = g.nearest_army_threat(china).expect("a raised rival Army next door is the threat");
    assert_eq!((id, from), (raised, next_door));
    let a = g.armies.iter().find(|a| a.id == raised).unwrap();
    let defence: i64 = g.defenders_at(Place::State(china), Seat(1)).iter().filter_map(|d| g.army(*d)).map(|d| g.army_defended_strength(d)).sum();
    assert!((odds - combat::first_round_odds(g.army_strength(a), defence)).abs() < 1e-9, "the odds are the first-exchange odds against what defends here");
    // A stronger raised Army next door with better odds takes the line.
    let second = g.raise_army(Place::State(next_door), false);
    g.armies.iter_mut().find(|x| x.id == second).unwrap().raised_strength = 9;
    assert_eq!(g.nearest_army_threat(china).map(|(id, _, _)| id), Some(second), "the best odds win the line");
    // A Region nobody holds has no threat line.
    let nobody = StateId::ALL.into_iter().find(|s| g.state(*s).control == Control::Neutral).expect("a neutral Region");
    assert!(g.nearest_army_threat(nobody).is_none());
}

// -------------------------------------------- 0.08.8 ticket #319: Carriers off Earth for every seat

/// Ticket #319 (version 0.08.8): a seat other than the Prospectors wants a Carrier only with cause:
/// a rival's Colony off Earth whose holder it is Wary or worse toward. A station over Earth is no
/// target, and a rival it is Neutral toward gives no cause.
#[test]
fn a_carrier_has_somewhere_to_go_only_with_cause_against_a_colony_off_earth() {
    let mut g = game();
    assert_eq!(g.kind(Seat(0)), FactionKind::Custodians, "seat 0 is not the Prospectors here");
    assert!(!g.carrier_target_exists(Seat(0)), "a fresh board: no rival Colony off Earth");
    // A rival's station over Earth is not off Earth.
    let over_earth = ColonyId(g.fresh_id());
    g.colonies.push(Colony { id: over_earth, body: BodyId::Earth, slot: 3, control: Control::Controlled(Seat(2)), modules: Vec::new(), colonists: 0, education: 1.0, settler_education: 1.0, queue: Vec::new(), grid_failed: false, founded_turn: 1, in_orbit: true });
    g.relations.score[0][2] = -8;
    assert!(!g.carrier_target_exists(Seat(0)), "a station over Earth is no Carrier's target");
    // A rival's Colony on Mars, the rival Neutral: no cause.
    let mars = ColonyId(g.fresh_id());
    g.colonies.push(Colony { id: mars, body: BodyId::Mars, slot: 0, control: Control::Controlled(Seat(1)), modules: Vec::new(), colonists: 4, education: 1.0, settler_education: 1.0, queue: Vec::new(), grid_failed: false, founded_turn: 1, in_orbit: false });
    assert!(!g.carrier_target_exists(Seat(0)), "Neutral toward its holder: no cause, no target");
    // Cold toward its holder: cause.
    g.relations.score[0][1] = -8;
    assert!(g.relations_score(Seat(0), Seat(1)) <= g.tables.ai.thresholds.war_cause);
    assert!(g.carrier_target_exists(Seat(0)), "Cold toward the Colony's holder: a target");
}

// -------------------------------------------- 0.08.8 ticket #318: an escaped Army holds no Occupation

/// Ticket #318 (version 0.08.8): an Occupation is held by the presence that begins one -- an Army
/// of the occupier at the place that did not escape -- so an Army that ran keeps no Occupation
/// alive. Until this ticket the holding check read every Army of the seat there, escaped or not.
#[test]
fn an_army_that_escaped_holds_no_occupation() {
    let mut g = game();
    g.armies.retain(|a| a.home != ArmyHome::State(StateId::Europe));
    let id = occupier_in(&mut g, StateId::EastAsia, StateId::Europe);
    g.resolution_phase();
    assert!(matches!(g.state(StateId::Europe).control, Control::Occupied { occupier: Seat(0), turns: 1, .. }), "the Occupation begins");
    assert!(g.present_at(Place::State(StateId::Europe), Seat(0)));
    g.armies.iter_mut().find(|a| a.id == id).unwrap().escaped = true;
    assert!(!g.present_at(Place::State(StateId::Europe), Seat(0)), "escaped, the Army is not present for an Occupation");
    g.resolution_phase();
    assert!(!matches!(g.state(StateId::Europe).control, Control::Occupied { .. }), "the Occupation broke: an Army that ran holds nothing");
}

// -------------------------------------------- 0.08.8 ticket #320: Passage as a rule for Armies

/// Ticket #320 (version 0.08.8): under a Passage Accord an Army marches into a partner's held
/// Region without attacking: it arrives on Hold, no march on a held Region is counted, no Battle
/// is fought and no offence is charged. Without Passage the same march arrives on Attack, as it
/// always did. And a Blockade does not shut out a partner under Passage.
#[test]
fn passage_lets_an_army_march_into_a_partners_region_on_hold() {
    let march = |passage: bool| {
        let mut g = game();
        let (home, target) = (StateId::EastAsia, StateId::Russia);
        assert!(g.tables.state(home).neighbours.contains(&target));
        g.take_control(target, Seat(1));
        if passage {
            g.strike_accord(Seat(0), Seat(1), vec![Term::Passage]).expect("Passage struck");
            assert!(g.accord_has(Seat(0), Seat(1), Term::Passage));
        }
        let army = g.raise_army(Place::State(home), false);
        g.armies.iter_mut().find(|a| a.id == army).unwrap().move_to = Some(target);
        let before = g.relations_score(Seat(1), Seat(0));
        g.resolution_phase();
        let a = g.armies.iter().find(|a| a.id == army).expect("the Army lives");
        assert_eq!(a.at, ArmyAt::Place(Place::State(target)), "it marched");
        (a.stance, g.war.marches_held[0], g.war.battles[0], g.relations_score(Seat(1), Seat(0)) - before)
    };
    assert_eq!(march(true), (Stance::Hold, 0, 0, 0), "under Passage: on Hold, no march on a held Region, no Battle, no offence");
    let (stance, marches, _, _) = march(false);
    assert_eq!((stance, marches), (Stance::Attack, 1), "without Passage: an attack, counted");
}

#[test]
fn a_blockade_does_not_shut_out_a_partner_under_passage() {
    let mut g = game();
    let station = g.colonies.iter().find(|c| c.in_orbit && c.body == BodyId::Earth && c.control.director() == Some(Seat(0))).expect("seat 0's station").clone();
    let id = ShipId(g.fresh_id());
    let name = g.next_ship_name(UnitKind::Frigate);
    g.ships.push(Ship { id, name, kind: UnitKind::Frigate, seat: Seat(1), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Blockade, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: Some(station.slot) });
    assert!(g.slot_blockaded_against(Seat(0), BodyId::Earth, station.slot), "a rival's warship on Blockade shuts the slot");
    g.strike_accord(Seat(0), Seat(1), vec![Term::Passage]).expect("Passage struck");
    assert!(!g.slot_blockaded_against(Seat(0), BodyId::Earth, station.slot), "not against a partner under Passage");
}

// -------------------------------------------- 0.08.8 ticket #321: a Region's own Army marches again

/// Ticket #321 (version 0.08.8): the holder may march a Region's own Army, which 0.08.6 kept at
/// home. At home it defends with its people (the Constabulary and the calm); marched out it is an
/// Army like any other, with no such bonus, and a threat next door like any other.
#[test]
fn a_regions_own_army_marches_again_and_defends_only_at_home() {
    let mut g = game();
    let (home, target) = (StateId::EastAsia, StateId::Russia);
    g.take_control(target, Seat(1));
    let standing = g.armies.iter().find(|a| a.standing && a.home == ArmyHome::State(home)).map(|a| a.id).expect("China's own Army");
    let a = g.army(standing).unwrap().clone();
    assert!(g.army_at_home(&a));
    let bonus_at_home = g.army_defence(&a);
    assert!(g.check_order(Seat(0), &[], &Order::MoveArmy { army: standing, to: target }).is_ok(), "its holder may march it");
    g.armies.iter_mut().find(|a| a.id == standing).unwrap().move_to = Some(target);
    g.resolution_phase();
    let a = g.army(standing).expect("it lives").clone();
    assert_eq!(a.at, ArmyAt::Place(Place::State(target)), "it marched");
    assert!(!g.army_at_home(&a), "and is not at home");
    assert_eq!(g.army_defence(&a), 0, "away from home, no people's defence (at home it was {bonus_at_home})");
    // Marched out next to a third Region the Prospectors hold, it is a threat to it like any other.
    let third = g.tables.state(target).neighbours.iter().copied().find(|n| *n != home && g.state(*n).control == Control::Neutral).expect("a neutral neighbour of Russia");
    g.take_control(third, Seat(1));
    let (id, from, _) = g.nearest_army_threat(third).expect("China's own Army, marched out, threatens the Region next door");
    assert_eq!((id, from), (standing, target));
}

// -------------------------------------------- 0.08.8 ticket #322: Armies that stack for orders

/// Ticket #322 (version 0.08.8): where two Armies of a seat may march and neither alone clears the
/// computer's attack bar against a neighbour, the stack does, and the computer marches them both
/// in one candidate. Before this ticket each Army was weighed alone and the seat never massed.
#[test]
fn the_computer_marches_a_stack_where_one_army_alone_would_not_clear_the_bar() {
    let mut g = game();
    calm(&mut g);
    let (home, target) = (StateId::EastAsia, StateId::Russia);
    g.take_control(target, Seat(1));
    g.relations.score[0][1] = -8;
    assert!(g.relations_score(Seat(0), Seat(1)) <= g.tables.ai.thresholds.war_cause, "cause");
    // Russia's own Army fights at 5 defended; one raised Army of 4 is under the bar, two of 4 are over it.
    let def: i64 = g.defenders_at(Place::State(target), Seat(0)).iter().filter_map(|id| g.army(*id)).map(|a| g.army_defended_strength(a)).sum();
    let bar = g.tables.ai.thresholds.attack_odds;
    assert!(combat::first_round_odds(4, def) < bar, "one Army of 4 against {def}: under the bar");
    assert!(combat::first_round_odds(8, def) >= bar, "two of 4 against {def}: over it");
    // China's own Army is kept out of it, so the stack is the two raised.
    for a in g.armies.iter_mut().filter(|a| a.standing && a.home == ArmyHome::State(home)) {
        a.stance = Stance::DigIn;
    }
    let first = g.raise_army(Place::State(home), false);
    g.armies.iter_mut().find(|a| a.id == first).unwrap().raised_strength = 4;
    let alone: Vec<Order> = g.ai_orders(Seat(0));
    assert!(!alone.iter().any(|o| matches!(o, Order::MoveArmy { to, .. } if *to == target)), "one Army alone does not march on Russia: {alone:?}");
    let second = g.raise_army(Place::State(home), false);
    g.armies.iter_mut().find(|a| a.id == second).unwrap().raised_strength = 4;
    let together: Vec<Order> = g.ai_orders(Seat(0));
    let marched: Vec<ArmyId> = together.iter().filter_map(|o| match o { Order::MoveArmy { army, to } if *to == target => Some(*army), _ => None }).collect();
    assert!(marched.contains(&first) && marched.contains(&second), "the stack of two marches on Russia together: {together:?}");
}

// -------------------------------------------- 0.08.8 ticket #324: the Battery

/// Ticket #324 (version 0.08.8): a working Battery denies a rival Orbital Control at its Body and
/// the Blockade with it, grants its owner none, is the owner's to repair at its Colony, and stands
/// in the line of a Battle there, where a rival stack on Attack fights it with no Ship of the
/// owner's present; shot to its hit points it is gone. Mothballed, it neither fires nor denies.
#[test]
fn a_battery_denies_orbital_control_and_the_blockade_and_falls_in_a_battle() {
    let mut g = game();
    let station = g.colonies.iter().find(|c| c.in_orbit && c.body == BodyId::Earth && c.control.director() == Some(Seat(0))).map(|c| c.id).expect("seat 0's station");
    let slot = g.colony(station).unwrap().slot;
    g.ships.retain(|s| s.at != ShipAt::Body(BodyId::Earth));
    let id = ShipId(g.fresh_id());
    let name = g.next_ship_name(UnitKind::Frigate);
    g.ships.push(Ship { id, name, kind: UnitKind::Frigate, seat: Seat(1), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Blockade, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: Some(slot) });
    // Ticket #335 (version 0.09.0): the frigate sits in the STATION'S orbit, so it blockades that
    // station and holds nothing of low orbit, which is what Orbital Control is of now.
    assert_eq!(g.orbital_control(BodyId::Earth), None, "a warship at a station's ring holds no Control of low orbit");
    assert!(g.slot_blockaded_against(Seat(0), BodyId::Earth, slot));
    assert_eq!(g.starved_by(station), Some(Seat(1)), "and the station starves under it");
    g.colony_mut(station).unwrap().modules.push(Module::new(ModuleKind::Battery));
    let index = g.colony(station).unwrap().modules.len() - 1;
    let card = g.tables.module(ModuleKind::Battery).clone();
    assert!(!g.slot_blockaded_against(Seat(0), BodyId::Earth, slot), "a Battery lifts the Blockade of its own orbit");
    assert_eq!(g.starved_by(station), None, "nor starves");
    assert_eq!(g.battery_strength(Seat(0), BodyId::Earth, Orbit::Slot(slot)), card.strength);
    assert_eq!(g.battery_strength(Seat(0), BodyId::Earth, Orbit::Low), 0, "ticket #335: a Battery covers its own orbit alone");
    g.colony_mut(station).unwrap().modules[index].mothballed = true;
    assert!(g.slot_blockaded_against(Seat(0), BodyId::Earth, slot), "mothballed, it denies nothing");
    g.colony_mut(station).unwrap().modules[index].mothballed = false;
    // Ticket #324's Orbital Control clause, read where Control now lives: a rival warship in LOW
    // ORBIT holds it, and the station's Battery high above does not deny it -- ticket #335 narrowed
    // a Battery to the orbit it covers, at the designer's word.
    let low = ShipId(g.fresh_id());
    let name = g.next_ship_name(UnitKind::Frigate);
    g.ships.push(Ship { id: low, name, kind: UnitKind::Frigate, seat: Seat(1), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
    assert_eq!(g.orbital_control(BodyId::Earth), Some(Seat(1)), "a lone rival warship in low orbit holds Orbital Control");
    assert!(!g.may_land(Seat(0), BodyId::Earth), "so the ground is shut");
    g.ships.retain(|s| s.id != low);
    // The Repair order: the owner's, at its Colony, for no more than its damage.
    g.colony_mut(station).unwrap().modules[index].damage = 2;
    let repair = |points: u32| Order::Repair { unit: UnitRef::Battery { colony: station, index }, points };
    assert!(g.check_order(Seat(0), &[], &repair(2)).is_ok(), "the owner repairs it");
    assert!(g.check_order(Seat(1), &[], &repair(2)).is_err(), "a rival does not");
    assert!(g.check_order(Seat(0), &[], &repair(3)).is_err(), "no more than its damage");
    // The Battle: the rival's stack on Attack, no Ship of the owner's present, and the Battery one
    // hit from gone.
    g.colony_mut(station).unwrap().modules[index].damage = card.hit_points - 1;
    // Ticket #335 (version 0.09.0): the attackers sit in the station's own orbit, since that is
    // where the Battery stands and a Battle is fought within one orbit.
    for _ in 0..2 {
        let id = ShipId(g.fresh_id());
        let name = g.next_ship_name(UnitKind::Frigate);
        g.ships.push(Ship { id, name, kind: UnitKind::Frigate, seat: Seat(1), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Attack, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: Some(slot) });
    }
    for s in g.ships.iter_mut().filter(|s| s.seat == Seat(1) && s.at == ShipAt::Body(BodyId::Earth)) {
        s.stance = Stance::Attack;
    }
    g.resolution_phase();
    // Ticket #335 (version 0.09.0): the Battle was fought in the STATION'S orbit, and the record
    // is that orbit's, not the Body's.
    let line = g.report.battles.iter().find(|b| b.at == Some(ReportPlace::Orbit(BodyId::Earth, Orbit::Slot(slot)))).expect("a Battle at the station's orbit").clone();
    let mine = line.parties.iter().find(|p| p.seat == Some(Seat(0))).expect("the Battery's side");
    assert!(mine.units.contains("the Battery at"), "the Battery is named in the line: {}", mine.units);
    assert_eq!(mine.strength, card.strength, "at its card's strength");
    assert!(!g.colony(station).unwrap().modules.iter().any(|m| m.kind == ModuleKind::Battery), "shot to nothing, it is gone: {}", mine.units);
    assert_eq!(g.war.batteries_lost[0], 1, "and counted");
    // Ticket #335: Orbital Control is low orbit's, so a fight at a station's ring says nothing
    // about it, where before every orbital Battle line ended with the state of Control.
    assert!(!line.result.contains("Orbital Control"), "{}", line.result);
}

/// Ticket #324: the computer wants a Battery at a Colony where a rival's warship stands, and not
/// where none does; and a rival reads the Battery's strength in its odds in that orbit.
#[test]
fn the_computer_wants_a_battery_where_a_rival_warship_stands() {
    let mut g = game();
    calm(&mut g);
    let station = g.colonies.iter().find(|c| c.in_orbit && c.body == BodyId::Earth && c.control.director() == Some(Seat(0))).map(|c| c.id).expect("seat 0's station");
    g.seats[0].stockpile.materials = 500;
    g.seats[0].stockpile.energy = 500;
    // Room on the station, with its opening Habitat and its Trade Post already standing: on a fresh
    // station with two free places those two outrank a Battery at the Barracks' weight (measured).
    {
        let col = g.colony_mut(station).unwrap();
        col.modules.push(Module::new(ModuleKind::Habitat));
        col.modules.push(Module::new(ModuleKind::TradePost));
        col.colonists = 8;
    }
    let before: Vec<Order> = g.ai_orders(Seat(0));
    assert!(!before.iter().any(|o| matches!(o, Order::BuildModule { kind: ModuleKind::Battery, .. })), "no rival warship here, no Battery: {before:?}");
    let id = ShipId(g.fresh_id());
    let name = g.next_ship_name(UnitKind::Frigate);
    g.ships.push(Ship { id, name, kind: UnitKind::Frigate, seat: Seat(1), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
    let after: Vec<Order> = g.ai_orders(Seat(0));
    assert!(after.iter().any(|o| matches!(o, Order::BuildModule { colony, kind: ModuleKind::Battery } if *colony == station)), "a rival warship at the Body: {after:?}");
    g.colony_mut(station).unwrap().modules.push(Module::new(ModuleKind::Battery));
    assert_eq!(g.enemy_ship_strength(Seat(1), BodyId::Earth), g.ship_stack_strength(Seat(0), BodyId::Earth) + g.tables.module(ModuleKind::Battery).strength, "the rival's odds read it");
}

// -------------------------------------------- 0.08.8 ticket #325: Refuelling at a partner's station

/// Ticket #325 (version 0.08.8): under a Refuel Accord a Ship refuels at the partner's station as
/// at its own, from its own Stockpile; without one it is stranded there; a station blockaded
/// against its holder fuels the partner no more than its holder; and the computer offers Refuel
/// where the other holds a station at a Body it has Ships at and no station of its own.
#[test]
fn a_refuel_accord_opens_a_partners_station() {
    let mut g = game();
    calm(&mut g);
    let station = ColonyId(g.fresh_id());
    g.colonies.push(Colony { id: station, body: BodyId::Mars, slot: 0, control: Control::Controlled(Seat(1)), modules: Vec::new(), colonists: 2, education: 1.0, settler_education: 1.0, queue: Vec::new(), grid_failed: false, founded_turn: 1, in_orbit: true });
    let id = ShipId(g.fresh_id());
    let name = g.next_ship_name(UnitKind::Frigate);
    // Ticket #335 (version 0.09.0): the Frigate stands at the partner station's own ring, which is
    // the orbit a station fuels from.
    g.ships.push(Ship { id, name, kind: UnitKind::Frigate, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Mars), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 0, slot: Some(0) });
    g.seats[0].stockpile.fuel = 40;
    let refuel = Order::Refuel { ship: id };
    assert!(!g.own_station_at(Seat(0), BodyId::Mars));
    assert!(g.check_order(Seat(0), &[], &refuel).is_err(), "no station of its own, no Accord: no Refuel");
    assert!(g.stranded(id), "and stranded on an empty tank");
    // The computer offers Refuel here, in its non-aggression offer.
    let offers: Vec<Order> = g.ai_orders(Seat(0)).into_iter().filter(|o| matches!(o, Order::ProposeAccord { to: Seat(1), .. })).collect();
    assert!(offers.iter().any(|o| matches!(o, Order::ProposeAccord { terms, .. } if terms.contains(&Term::Refuel))), "the Prospectors hold the only station at Mars: {offers:?}");
    g.strike_accord(Seat(0), Seat(1), vec![Term::NonAggression, Term::Refuel]).expect("struck");
    assert!(g.check_order(Seat(0), &[], &refuel).is_ok(), "under the Accord it refuels at the partner's station");
    assert!(!g.stranded(id), "and is not stranded");
    let fuel_before = g.seats[0].stockpile.fuel;
    g.commit_orders(Seat(0), std::slice::from_ref(&refuel));
    g.resolution_phase();
    assert_eq!(g.ship(id).unwrap().fuel, g.tables.unit(UnitKind::Frigate).tank, "the tank is full");
    assert_eq!(g.seats[0].stockpile.fuel, fuel_before - g.tables.unit(UnitKind::Frigate).tank, "paid from the refueller's own Stockpile");
    assert!(g.log.iter().any(|l| l.contains("at a partner's station")), "said in the log");
    // A rival blockading the partner's slot shuts it to the partner as to its holder.
    g.ships.iter_mut().find(|s| s.id == id).unwrap().fuel = 0;
    let blockader = ShipId(g.fresh_id());
    let name = g.next_ship_name(UnitKind::Frigate);
    g.ships.push(Ship { id: blockader, name, kind: UnitKind::Frigate, seat: Seat(2), damage: 0, at: ShipAt::Body(BodyId::Mars), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Blockade, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: Some(0) });
    assert!(g.check_order(Seat(0), &[], &refuel).is_err(), "blockaded against its holder, it fuels nobody");
}

// -------------------------------------------- 0.08.8 ticket #326: escorts take the fire

/// Ticket #326 (version 0.08.8): while a party has a warship engaged, hits on the party land on
/// its warships; the unarmed hull is struck only once the last warship is down. And the retreat
/// is covered: an unarmed hull that runs while its escort stands is not pursued.
#[test]
fn escorts_take_the_fire_and_cover_the_retreat() {
    // A Frigate attacks a Colony Ship escorted by a Frigate; the Colony Ship stands FIRST in its
    // party, so a uniform draw with the scripted pick of 0 would strike it first. Every hitter
    // roll goes to the attacker; the escort never disengages. Round 1: three hits on the escort.
    // Round 2: the fourth destroys it, the next two land on the Colony Ship. Round 3: the third
    // hit destroys the Colony Ship.
    let mut a = vec![frigate(1)];
    let mut d = vec![colony_ship(3), frigate(2)];
    let chances = vec![true, true, true, false, true, true, true, false, true, true, true];
    let mut dice = Script { chances: VecDeque::from(chances), d6s: VecDeque::new(), picks: VecDeque::new() };
    let stats = combat::fight(&mut a, &mut d, &mut dice, 2.0);
    assert!(d[1].destroyed(), "the escort fell first, at {} hits", d[1].damage);
    assert_eq!(d[1].damage, 4, "every hit while it stood landed on it");
    assert_eq!(d[0].damage, 3, "the Colony Ship was struck only after, and died: {stats:?}");
    assert_eq!(stats.hits[0], 7);
    // The retreat: the Colony Ship on Evade escapes at the start; its escort stands engaged, so no
    // pursuit is rolled for it (an empty d6 script would panic if one were), and it takes no hit.
    let mut a = vec![frigate(1)];
    let mut d = vec![Combatant::new(UnitRef::Ship(ShipId(3)), "Colony Ship 3", 0, 3, 0, 0, true).armed(false), frigate(2)];
    let chances = vec![true, true, true, true, false, true, true, true];
    let mut dice = Script { chances: VecDeque::from(chances), d6s: VecDeque::new(), picks: VecDeque::new() };
    combat::fight(&mut a, &mut d, &mut dice, 2.0);
    assert!(d[0].escaped && d[0].damage == 0, "the Colony Ship ran under cover and was not caught");
    assert!(d[1].destroyed(), "its escort took the fight");
}

// -------------------------------------------- 0.08.8 ticket #327: rolls per engaged warship

/// Ticket #327 (version 0.08.8): a Ship melee rolls once a round for every engaged armed unit
/// present. Two Frigates and a Battery against a Battleship roll four a round, so the Battleship
/// (8 hit points) dies in two rounds where a flat three needed three.
#[test]
fn a_ship_melee_rolls_once_a_round_for_every_engaged_armed_unit() {
    let mut a = vec![frigate(1), frigate(2), Combatant::new(UnitRef::Battery { colony: ColonyId(9), index: 0 }, "the Battery", 4, 6, 0, 0, false)];
    let mut d = vec![Combatant::new(UnitRef::Ship(ShipId(3)), "Battleship 3", 7, 8, 0, 0, false)];
    let rolls = (a.len() + d.len()) as u32;
    assert_eq!(rolls, 4);
    // Every hitter roll to the attackers; the Battleship never disengages.
    let chances = vec![true, true, true, true, false, true, true, true, true];
    let mut dice = Script { chances: VecDeque::from(chances), d6s: VecDeque::new(), picks: VecDeque::new() };
    let stats = combat::melee(&mut [&mut a, &mut d], &mut dice, 3.0, 3, rolls);
    assert!(d[0].destroyed(), "eight hits in two rounds of four: {stats:?}");
    assert_eq!(stats.rounds, 2, "two rounds, not three: {stats:?}");
    assert_eq!(stats.hits[0], 8);
    // The table carries the figures, and a fresh game reads them.
    let g = game();
    assert_eq!((g.tables.melee.rounds, g.tables.melee.rolls), (3, 3));
}

// -------------------------------------------- 0.08.8 ticket #328: Bombard

/// Ticket #328 (version 0.08.8): a Battleship holding the orbit outright bombards a rival's Colony
/// at the Body: one Module burns at the destruction chance (forced to certain here), a burned
/// Habitat takes the people beyond the room left, the offence is rung 3, the Report and the Battle
/// record carry it. Refused over Earth, without the orbit held outright, and from a Frigate.
#[test]
fn a_battleship_bombards_a_rival_colony_from_an_orbit_it_holds() {
    let mut g = game();
    calm(&mut g);
    let colony = ColonyId(g.fresh_id());
    g.colonies.push(Colony { id: colony, body: BodyId::Mars, slot: 0, control: Control::Controlled(Seat(1)), modules: vec![Module::new(ModuleKind::Habitat), Module::new(ModuleKind::Habitat), Module::new(ModuleKind::Core)], colonists: 12, education: 1.0, settler_education: 1.0, queue: Vec::new(), grid_failed: false, founded_turn: 1, in_orbit: false });
    g.ships.retain(|s| s.at != ShipAt::Body(BodyId::Mars));
    let ship = ShipId(g.fresh_id());
    let name = g.next_ship_name(UnitKind::Battleship);
    g.ships.push(Ship { id: ship, name, kind: UnitKind::Battleship, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Mars), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
    let bombard = Order::Bombard { ship, colony };
    assert_eq!(g.orbital_control(BodyId::Mars), Some(Seat(0)));
    assert!(g.check_order(Seat(0), &[], &bombard).is_ok(), "held outright, a rival's Colony at the Body");
    // Not with the orbit contested.
    let rival = ShipId(g.fresh_id());
    let name = g.next_ship_name(UnitKind::Frigate);
    g.ships.push(Ship { id: rival, name, kind: UnitKind::Frigate, seat: Seat(2), damage: 0, at: ShipAt::Body(BodyId::Mars), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
    assert!(g.check_order(Seat(0), &[], &bombard).is_err(), "the orbit is contested");
    g.ships.retain(|s| s.id != rival);
    // Never over Earth, from any hull.
    let over_earth = g.colonies.iter().find(|c| c.in_orbit && c.body == BodyId::Earth && c.control.director() == Some(Seat(1))).map(|c| c.id).expect("seat 1's station");
    let earth_ship = ShipId(g.fresh_id());
    let name = g.next_ship_name(UnitKind::Battleship);
    g.ships.push(Ship { id: earth_ship, name, kind: UnitKind::Battleship, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
    assert!(g.check_order(Seat(0), &[], &Order::Bombard { ship: earth_ship, colony: over_earth }).is_err(), "no Bombard over Earth");
    g.ships.retain(|s| s.id != earth_ship);
    // The strike, at a certain chance: one Module burns, never the Core, and the people beyond the room left die.
    std::sync::Arc::make_mut(&mut g.tables).influence.destruction_chance = 1.0;
    let owed_before = g.relations.owed[1][0];
    g.commit_orders(Seat(0), std::slice::from_ref(&bombard));
    g.resolution_phase();
    let c = g.colony(colony).unwrap().clone();
    assert_eq!(c.modules.len(), 2, "one Module burned: {:?}", c.modules.iter().map(|m| m.kind).collect::<Vec<_>>());
    assert!(c.modules.iter().any(|m| m.kind == ModuleKind::Core), "never the Core Module");
    assert_eq!(c.modules.iter().filter(|m| m.kind == ModuleKind::Habitat).count(), 1, "a Habitat burned");
    assert_eq!(c.colonists, g.habitat_room(&c), "the people beyond the room left died: {} of 12 live", c.colonists);
    assert_eq!(g.relations.owed[1][0] - owed_before, 3, "rung 3 against the holder");
    assert_eq!((g.war.bombards[0], g.war.modules_burned[0]), (1, 1));
    assert!(g.log.iter().any(|l| l.contains("bombarded") && l.contains("Habitat destroyed") && l.contains("Colonists dead")), "{:?}", g.log.iter().filter(|l| l.contains("bombard")).collect::<Vec<_>>());
    // Ticket #335 (version 0.09.0): the record is the ORBIT's -- low orbit, which is where a
    // ground Colony is broken from.
    let line = g.report.battles.iter().find(|b| b.at == Some(ReportPlace::Orbit(BodyId::Mars, Orbit::Low))).expect("a Battle record in Mars low orbit for the mark");
    assert_eq!(line.aggressor(), Some(Seat(0)));
}

/// Ticket #328: the computer bombards a Colony whose holder it has cause against, from a
/// Battleship holding the orbit outright, and not without cause.
#[test]
fn the_computer_bombards_with_cause_and_the_orbit_held() {
    let mut g = game();
    calm(&mut g);
    let colony = ColonyId(g.fresh_id());
    g.colonies.push(Colony { id: colony, body: BodyId::Mars, slot: 0, control: Control::Controlled(Seat(1)), modules: vec![Module::new(ModuleKind::Habitat), Module::new(ModuleKind::Mine)], colonists: 4, education: 1.0, settler_education: 1.0, queue: Vec::new(), grid_failed: false, founded_turn: 1, in_orbit: false });
    g.ships.retain(|s| s.at != ShipAt::Body(BodyId::Mars));
    let ship = ShipId(g.fresh_id());
    let name = g.next_ship_name(UnitKind::Battleship);
    g.ships.push(Ship { id: ship, name, kind: UnitKind::Battleship, seat: Seat(0), damage: 0, at: ShipAt::Body(BodyId::Mars), colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None });
    let calm_orders: Vec<Order> = g.ai_orders(Seat(0));
    assert!(!calm_orders.iter().any(|o| matches!(o, Order::Bombard { .. })), "no cause, no Bombard: {calm_orders:?}");
    g.relations.score[0][1] = -8;
    assert!(g.relations_score(Seat(0), Seat(1)) <= g.tables.ai.thresholds.war_cause);
    let orders: Vec<Order> = g.ai_orders(Seat(0));
    assert!(orders.iter().any(|o| matches!(o, Order::Bombard { ship: s, colony: c } if *s == ship && *c == colony)), "with cause and the orbit held: {orders:?}");
}

// -------------------------------------------- 0.09.0 ticket #332: Widgets

/// Ticket #332: the Region whose name this is, read off the cards rather than guessed at.
fn region_named(g: &Game, name: &str) -> StateId {
    StateId::ALL.into_iter().find(|s| g.tables.state(*s).name == name).unwrap_or_else(|| panic!("no Region named {name}"))
}

/// Ticket #332 (version 0.09.0), R1: Widgets are a rate per place. A Region makes a flat 4 and one
/// per Industry Level with no Factory, and four more for every working one, through the Faction's
/// output multiplier and the Unrest-7 half but NOT the Materials lean or Deep Mining, which are
/// Materials rules; a Colony makes its Core Module's four and a Factory Module's four, flat; a grid
/// failure makes none. Nothing of it enters the Stockpile or the income sources.
#[test]
fn widgets_are_a_rate_per_place_from_industry_level_factories_and_core_modules() {
    let mut g = game();
    calm(&mut g);
    let sid = StateId::EastAsia;
    directed(&mut g, sid);
    assert_eq!((g.tables.widgets.region_base, g.tables.widgets.per_industry_level), (4, 1), "the card figures");
    assert_eq!(g.state(sid).industry_level, 3, "East Asia's Industry Level");
    assert_eq!(g.widgets_at(Place::State(sid)), 7, "Industry Level 3 and no Factory: a flat 4 and 3 for the levels, 7 a turn");
    g.state_mut(sid).facilities.push(facility(FacilityKind::Factory));
    assert_eq!(g.widgets_at(Place::State(sid)), 11, "and four more for a working Factory");
    // East Asia leans Materials and Deep Mining is a Materials Tech: neither lifts a Factory now.
    assert_eq!(g.facility_yield(Seat(0), sid, FacilityKind::Factory).resource, Some(Resource::Widgets));
    with_tech(&mut g, TechId::DeepMining);
    assert_eq!(g.widgets_at(Place::State(sid)), 11, "no lean, no Deep Mining, on Widgets");
    // A mothballed Factory makes none; at Unrest 7 the Factory's four halve and the base does not.
    let i = g.state(sid).facilities.len() - 1;
    g.state_mut(sid).facilities[i].mothballed = true;
    assert_eq!(g.widgets_at(Place::State(sid)), 7, "a mothballed Factory makes nothing");
    g.state_mut(sid).facilities[i].mothballed = false;
    g.state_mut(sid).unrest = g.tables.unrest.facility_threshold;
    assert_eq!(g.widgets_at(Place::State(sid)), 9, "at Unrest 7 the Factory makes 2; the base 7 stands");
    g.state_mut(sid).unrest = 0.0;
    // The Prospectors' output multiplier reaches the figure: 4 x 1.25 = 5.
    let pro = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Prospectors).unwrap();
    g.take_control(StateId::Europe, pro);
    g.state_mut(StateId::Europe).facilities.push(facility(FacilityKind::Factory));
    assert_eq!(g.widgets_at(Place::State(StateId::Europe)), 7 + 5, "Europe's 7 at Industry Level 3, and a Prospector Factory's 5");
    // A Colony: the Core Module's four, a Factory Module's four flat (the Moon's Materials yield is
    // 1.75 and reaches none of it), and nothing at all through a grid failure.
    let moon = colony(&mut g, Seat(0), BodyId::Moon, &[], 4);
    assert_eq!(g.widgets_at(Place::Colony(moon)), 4, "the Core Module's four");
    g.colony_mut(moon).unwrap().modules.insert(0, Module::new(ModuleKind::Factory));
    assert_eq!(g.module_yield_at(Seat(0), moon, 0).amount, 4, "flat: no Body yield");
    assert_eq!(g.widgets_at(Place::Colony(moon)), 8, "and the Core Module's four");
    g.colony_mut(moon).unwrap().grid_failed = true;
    assert_eq!(g.widgets_at(Place::Colony(moon)), 0, "a failed grid makes nothing");
    g.colony_mut(moon).unwrap().grid_failed = false;
    // Never a stock: the Income banks nothing of it and lists nothing of it.
    g.state_mut(sid).facilities.retain(|f| f.kind != FacilityKind::Factory);
    g.state_mut(sid).facilities.push(facility(FacilityKind::Factory));
    let paid = income_of(&mut g, Seat(0));
    assert_eq!(paid.materials, 0, "a Factory makes no Materials: {paid:?}");
    assert!(!g.seat(Seat(0)).income_sources.iter().any(|(_, r, _)| *r == Resource::Widgets), "{:?}", g.seat(Seat(0)).income_sources);
    // And whatever is not applied at Resolution is lost, and counted so.
    let (made, lost) = (g.widgets.made[0], g.widgets.lost[0]);
    g.resolution_phase();
    assert!(g.widgets.made[0] > made, "made was counted");
    assert_eq!(g.widgets.lost[0] - lost, g.widgets.made[0] - made, "with nothing under way, every Widget made was lost");
}

/// Ticket #332, R2: a build carries a Widget figure and a count. The figure is the row's, four
/// for every turn the build took, times the seat's Faction discount for the kind, floored, never
/// below 1; the count starts at nought, or at the figure for an outright buy in Ducats, which
/// therefore completes at the next Resolution ahead of everything queued before it.
#[test]
fn a_build_carries_a_widget_figure_at_four_per_former_turn_and_a_count() {
    let mut g = game();
    calm(&mut g);
    let sid = StateId::EastAsia;
    directed(&mut g, sid);
    let t = &g.tables;
    // The rows, at four per former turn.
    assert_eq!((t.facility(FacilityKind::Factory).widgets, t.facility(FacilityKind::PowerPlant).widgets, t.facility(FacilityKind::SeaWall).widgets), (4, 8, 8));
    assert_eq!((t.module(ModuleKind::Habitat).widgets, t.module(ModuleKind::Shipyard).widgets, t.module(ModuleKind::Archive).widgets), (4, 8, 12));
    assert_eq!((t.unit(UnitKind::Frigate).widgets, t.unit(UnitKind::Battleship).widgets, t.unit(UnitKind::Army).widgets), (4, 8, 4));
    assert_eq!(t.industry_level.widgets, 4, "an Industry raise");
    assert_eq!(t.module(ModuleKind::Core).widgets, 0, "nobody orders the Core Module");
    // The Faction discounts reach the figure: the Prospectors' 15% off a Facility and an Industry
    // raise, the Arkwrights' three quarters on a Module and 15% off a Ship; an Army has none.
    let pro = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Prospectors).unwrap();
    let ark = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Arkwrights).unwrap();
    assert_eq!(g.build_widgets(Seat(0), BuildItem::Facility(FacilityKind::PowerPlant)), 8, "the Custodians pay the row");
    assert_eq!(g.build_widgets(pro, BuildItem::Facility(FacilityKind::PowerPlant)), 6, "8 x 0.85 = 6.8, floored");
    assert_eq!(g.build_widgets(pro, BuildItem::IndustryLevel), 3, "4 x 0.85 = 3.4, floored");
    assert_eq!(g.build_widgets(ark, BuildItem::Module(ModuleKind::Shipyard)), 6, "8 x 0.75");
    assert_eq!(g.build_widgets(ark, BuildItem::Unit(UnitKind::Battleship)), 6, "8 x 0.85 = 6.8, floored");
    assert_eq!(g.build_widgets(pro, BuildItem::Unit(UnitKind::Army)), 4, "an Army takes no discount");
    assert_eq!(g.build_widgets(ark, BuildItem::Module(ModuleKind::Core)), 1, "a figure that floors to nought is 1");
    // At the order: the figure and a count of nought, or the figure done for a Ducat buy.
    let plant = Order::BuildFacility { state: sid, kind: FacilityKind::PowerPlant };
    let embassy = Order::BuildFacilityWithDucats { state: sid, kind: FacilityKind::Embassy };
    g.commit_orders(Seat(0), &[plant, embassy]);
    let q = g.state(sid).queue.clone();
    assert_eq!(q.len(), 2);
    assert_eq!((q[0].item, q[0].widgets, q[0].done), (BuildItem::Facility(FacilityKind::PowerPlant), 8, 0));
    assert_eq!((q[1].item, q[1].widgets, q[1].done), (BuildItem::Facility(FacilityKind::Embassy), 4, 4), "bought outright: done at the order");
    // East Asia makes 7: the Embassy, done already, stands at this Resolution ahead of the Power
    // Plant queued before it, which takes the 7 of its 8 and waits.
    g.resolution_phase();
    assert!(g.state(sid).facilities.iter().any(|f| f.kind == FacilityKind::Embassy), "the Ducat buy stands");
    assert!(!g.state(sid).facilities.iter().any(|f| f.kind == FacilityKind::PowerPlant), "the Power Plant does not");
    assert_eq!((g.state(sid).queue.len(), g.state(sid).queue[0].done), (1, 7), "7 of 8");
}

/// Ticket #332, R3: the queue is served in order, and the specification's own refutation. On a
/// fresh board Nigeria holds one Factory at Industry Level 1 and makes 9 a turn (a flat 4, 1 for
/// the level, 4 for the Factory): a single 1-turn Facility ordered on turn N stands at turn N's
/// Resolution (9 against 4), and of a Bank and a Power Plant ordered together the Power Plant
/// waits for turn N+1 with 5 of 8 done. A build never completes short of its figure; what is
/// applied never exceeds what was made. A Launch Pad Fire takes the turn's Widgets from its
/// Region, unless Clean Propellant is held.
#[test]
fn the_queue_is_served_in_order_and_a_launch_pad_fire_takes_the_turns_widgets() {
    let mut g = fresh();
    calm(&mut g);
    let sid = region_named(&g, "Nigeria");
    directed(&mut g, sid);
    assert_eq!(g.state(sid).industry_level, 1);
    assert_eq!(g.state(sid).facilities.iter().filter(|f| f.kind == FacilityKind::Factory).count(), 1, "one start Factory");
    assert_eq!(g.widgets_at(Place::State(sid)), 9, "a flat 4, Industry Level 1 and a Factory: 9 a turn");
    let bank = Order::BuildFacility { state: sid, kind: FacilityKind::Bank };
    let plant = Order::BuildFacility { state: sid, kind: FacilityKind::PowerPlant };
    g.commit_orders(Seat(0), &[bank, plant]);
    let before = (g.widgets.made[0], g.widgets.applied[0], g.widgets.lost[0]);
    g.resolution_phase();
    let banks = |g: &Game| g.state(sid).facilities.iter().filter(|f| f.kind == FacilityKind::Bank).count();
    let plants = |g: &Game| g.state(sid).facilities.iter().filter(|f| f.kind == FacilityKind::PowerPlant).count();
    assert_eq!((banks(&g), plants(&g)), (1, 0), "the first stands at turn N; the second does not");
    assert_eq!(g.state(sid).queue.len(), 1);
    assert_eq!((g.state(sid).queue[0].widgets, g.state(sid).queue[0].done), (8, 5), "the Power Plant took what flowed on: 5 of 8");
    g.turn += 1;
    g.resolution_phase();
    assert_eq!((banks(&g), plants(&g)), (1, 1), "the second stands at turn N+1");
    assert!(g.state(sid).queue.is_empty());
    // The counters: 9 made and all 9 applied at turn N, 9 made and 3 applied at N+1, so 12 applied
    // in all across the seat's places, and lost is exactly what was made and not applied.
    let (made, applied, lost) = (g.widgets.made[0] - before.0, g.widgets.applied[0] - before.1, g.widgets.lost[0] - before.2);
    assert_eq!(applied, 12, "4 + 5 at turn N, 3 at turn N+1");
    assert!(made >= 18, "at least Nigeria's two turns: {made}");
    assert_eq!(lost, made - applied);
    // A Launch Pad Fire at Nigeria: no Widgets applied there this turn, nothing completes.
    g.commit_orders(Seat(0), &[Order::BuildFacility { state: sid, kind: FacilityKind::Bank }]);
    drawn(&mut g, EventId::LaunchPadFire, EventTarget::State(sid));
    g.turn += 1;
    g.resolution_phase();
    assert_eq!(banks(&g), 1, "nothing completed under the fire");
    assert_eq!(g.state(sid).queue[0].done, 0, "and nothing was applied");
    assert!(g.log.iter().any(|l| l.contains("Launch Pad Fire") && l.contains("applies no Widgets this turn; 9 lost")), "{:?}", g.log.iter().rev().take(8).collect::<Vec<_>>());
    // With Clean Propellant the fire takes nothing.
    with_tech(&mut g, TechId::CleanPropellant);
    drawn(&mut g, EventId::LaunchPadFire, EventTarget::State(sid));
    g.turn += 1;
    g.resolution_phase();
    assert_eq!(banks(&g), 2, "Clean Propellant: the Bank stands");
}

/// Ticket #332, R4: four makers. The Mine Facility and the Factory Module are appended last to
/// their lists under one name each; the Factory makes 4 Widgets and no Materials at all; the Mine
/// makes the Materials the Factory made, through the Region's lean and Deep Mining, emitting its
/// full figure with no Clean Manufacturing; the Factory Module makes 4 Widgets flat and the Core
/// Module 4; and every Region that starts with a Factory starts with a Mine beside it.
#[test]
fn four_makers_the_factory_makes_widgets_the_mine_makes_materials_and_the_start_board_has_both() {
    let mut g = game();
    calm(&mut g);
    assert_eq!(FacilityKind::ALL.last(), Some(&FacilityKind::Mine), "appended last");
    assert_eq!(ModuleKind::ALL.last(), Some(&ModuleKind::Factory), "appended last");
    assert!(ModuleKind::BUILDABLE.contains(&ModuleKind::Factory));
    assert_eq!((FacilityKind::Mine.name(), ModuleKind::Factory.name()), ("Mine", "Factory"), "one name in both lists");
    let t = &g.tables;
    let (f, m) = (t.facility(FacilityKind::Factory), t.facility(FacilityKind::Mine));
    // The Factory and the Mine each smoke 0.75, at the designer's word.
    assert_eq!((f.materials, f.widgets, f.energy_upkeep, f.emissions), (20, 4, 2, 0.75));
    assert_eq!(f.produces.as_ref().map(|p| (p.resource, p.amount)), Some((Resource::Widgets, 4)));
    assert_eq!((m.materials, m.widgets, m.energy_upkeep, m.emissions), (20, 4, 2, 0.75));
    assert_eq!(m.produces.as_ref().map(|p| (p.resource, p.amount)), Some((Resource::Materials, 4)));
    let fm = t.module(ModuleKind::Factory);
    assert_eq!((fm.materials, fm.widgets, fm.energy_upkeep, fm.earth_emissions), (20, 4, 3, 1.0));
    assert_eq!(fm.produces.as_ref().map(|p| (p.resource, p.amount)), Some((Resource::Widgets, 4)));
    assert_eq!(t.module(ModuleKind::Core).produces.as_ref().map(|p| (p.resource, p.amount)), Some((Resource::Widgets, 4)));
    // China leans Materials: a Mine there makes 6, not 4, and 9 with Deep Mining; a Factory there
    // makes no Materials at all.
    let sid = StateId::EastAsia;
    directed(&mut g, sid);
    assert_eq!(g.tables.state(sid).name, "China");
    g.state_mut(sid).facilities = vec![facility(FacilityKind::Mine)];
    assert_eq!(income_of(&mut g, Seat(0)).materials, 6, "a Mine in China makes 6");
    with_tech(&mut g, TechId::DeepMining);
    assert_eq!(income_of(&mut g, Seat(0)).materials, 9, "and 9 with Deep Mining");
    g.state_mut(sid).facilities = vec![facility(FacilityKind::Factory)];
    assert_eq!(income_of(&mut g, Seat(0)).materials, 0, "a Factory makes no Materials");
    // Clean Manufacturing follows the Factory; the Mine's smoke is not thinned.
    let (fac_before, mine_before) = (g.facility_yield(Seat(0), sid, FacilityKind::Factory).emissions, g.facility_yield(Seat(0), sid, FacilityKind::Mine).emissions);
    with_tech(&mut g, TechId::CleanManufacturing);
    let (fac_after, mine_after) = (g.facility_yield(Seat(0), sid, FacilityKind::Factory).emissions, g.facility_yield(Seat(0), sid, FacilityKind::Mine).emissions);
    assert!(fac_after < fac_before, "Clean Manufacturing thins the Factory: {fac_before} -> {fac_after}");
    assert!((mine_after - mine_before).abs() < 1e-9, "and not the Mine: {mine_before} -> {mine_after}");
    g.state_mut(sid).facilities = vec![facility(FacilityKind::Mine)];
    assert!(g.emissions_now().factories > 0.0, "a Mine's Emissions are charged");
    // Off Earth: a Factory Module's 4 flat at Mars (Materials yield 1.65), the Core's 4.
    let mars = colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Factory], 4);
    assert_eq!(g.module_yield_at(Seat(0), mars, 0).amount, 4, "flat");
    assert_eq!(g.module_yield_at(Seat(0), mars, 0).resource, Some(Resource::Widgets));
    let core = g.colony(mars).unwrap().modules.iter().position(|m| m.kind == ModuleKind::Core).unwrap();
    assert_eq!(g.module_yield_at(Seat(0), mars, core).amount, 4);
    // The starting board: a Mine beside every start Factory, standing from turn one.
    let fresh = fresh();
    for sid in StateId::ALL {
        let card = fresh.tables.state(sid);
        let factories = card.start_facilities.iter().filter(|k| **k == FacilityKind::Factory).count();
        let mines = card.start_facilities.iter().filter(|k| **k == FacilityKind::Mine).count();
        assert_eq!(factories, mines, "{}: a Mine beside every Factory", card.name);
        assert_eq!(fresh.state(sid).facilities.iter().filter(|f| f.kind == FacilityKind::Mine).count(), mines, "{}: standing", card.name);
    }
}

/// Ticket #332, R5: Production Moved pairs the Factory with the Factory Module and the Mine with
/// the Mine, so a mothballed Earth Factory doubles a Factory Module's Widgets off Earth, and the
/// place's Widgets a turn read the doubled figure.
#[test]
fn production_moved_pairs_factory_to_factory_and_doubles_a_factory_modules_widgets() {
    let mut g = game();
    calm(&mut g);
    let cus = Seat(0);
    assert_eq!(g.kind(cus), FactionKind::Custodians);
    let pairs = &g.tables.faction(FactionKind::Custodians).mothball_pairs;
    assert_eq!(pairs.get(&FacilityKind::Factory), Some(&ModuleKind::Factory));
    assert_eq!(pairs.get(&FacilityKind::Mine), Some(&ModuleKind::Mine));
    assert_eq!(pairs.get(&FacilityKind::PowerPlant), Some(&ModuleKind::Generator));
    assert_eq!(pairs.get(&FacilityKind::Refinery), Some(&ModuleKind::Refinery));
    assert_eq!(pairs.get(&FacilityKind::ResearchLab), Some(&ModuleKind::Observatory));
    assert_eq!(pairs.len(), 5);
    assert!(g.tables.faction(FactionKind::Custodians).signature.contains("while a Factory, Mine, Power Plant, Refinery or Research Lab"), "the signature names the pairs");
    let moon = colony(&mut g, cus, BodyId::Moon, &[ModuleKind::Factory], 4);
    assert_eq!(g.module_yield_at(cus, moon, 0).amount, 4);
    assert_eq!(g.widgets_at(Place::Colony(moon)), 8, "4 and the Core Module's 4");
    let mut f = facility(FacilityKind::Factory);
    f.mothballed = true;
    g.state_mut(StateId::EastAsia).facilities.push(f);
    let y = g.module_yield_at(cus, moon, 0);
    assert_eq!((y.amount, y.doubled_by), (8, Some("Factory")), "doubled by the idle Factory on Earth");
    assert_eq!(g.widgets_at(Place::Colony(moon)), 12, "8 and the Core Module's 4");
    // A mothballed Mine on Earth does not reach it: the Mine pairs with the Mine.
    g.state_mut(StateId::EastAsia).facilities.clear();
    let mut m = facility(FacilityKind::Mine);
    m.mothballed = true;
    g.state_mut(StateId::EastAsia).facilities.push(m);
    assert_eq!(g.module_yield_at(cus, moon, 0).amount, 4, "an idle Mine doubles no Factory Module");
}

/// Ticket #332, R6: the outright buy in Ducats is twice the Materials, as today, and completes
/// at the next Resolution even at a place making nothing; a Space Station founding costs
/// Materials only and enters no queue.
#[test]
fn an_outright_buy_completes_next_resolution_and_a_station_founding_carries_no_widgets() {
    let mut g = game();
    calm(&mut g);
    let sid = StateId::EastAsia;
    directed(&mut g, sid);
    let moon = colony(&mut g, Seat(0), BodyId::Moon, &[], 4);
    let buy = Order::BuildModuleWithDucats { colony: moon, kind: ModuleKind::Habitat };
    let cost = g.order_cost(Seat(0), &buy);
    assert_eq!((cost.materials, cost.ducats), (0, g.module_materials_at(Seat(0), moon, ModuleKind::Habitat) * g.tables.ducats.per_building_material));
    assert_eq!(g.tables.ducats.per_building_material, 2, "twice the Materials");
    g.check_order(Seat(0), &[], &buy).expect("legal");
    g.commit_orders(Seat(0), std::slice::from_ref(&buy));
    // The Core Module idled: the Colony makes nothing, and the buy stands regardless.
    let core = g.colony(moon).unwrap().modules.iter().position(|m| m.kind == ModuleKind::Core).unwrap();
    g.colony_mut(moon).unwrap().modules[core].mothballed = true;
    assert_eq!(g.widgets_at(Place::Colony(moon)), 0);
    g.resolution_phase();
    assert!(g.colony(moon).unwrap().modules.iter().any(|m| m.kind == ModuleKind::Habitat), "the Habitat stands at the next Resolution");
    assert!(g.colony(moon).unwrap().queue.is_empty());
    // A station founding: Materials only, no Widget figure, no queue entry anywhere.
    let station = Order::BuildStation { body: BodyId::Earth, slot: g.free_orbital_slots(BodyId::Earth)[0] };
    let cost = g.order_cost(Seat(0), &station);
    assert_eq!((cost.materials, cost.ducats), (g.station_materials(Seat(0)), 0));
    g.check_order(Seat(0), &[], &station).expect("a Launch Site stands in East Asia");
    let queued = |g: &Game| g.states.iter().map(|s| s.queue.len()).sum::<usize>() + g.colonies.iter().map(|c| c.queue.len()).sum::<usize>();
    let before = queued(&g);
    g.commit_orders(Seat(0), std::slice::from_ref(&station));
    assert_eq!(queued(&g), before, "a founding is pending, not queued");
    assert_eq!(g.pending.stations.len(), 1);
}

/// Ticket #332, R7: a place that changes hands keeps its queue, each build with its seat; the
/// new holder may cancel another seat's build, taking the item's Materials at their own price
/// and losing the Widgets done. A cancel of a build of one's own, at an index past the queue, at
/// a place one does not direct, or a second at one place in a turn, is refused.
#[test]
fn a_transferred_place_keeps_its_queue_and_a_cancel_refunds_the_canceller_at_their_own_price() {
    let mut g = game();
    calm(&mut g);
    let sid = StateId::Europe;
    let pro = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Prospectors).unwrap();
    g.take_control(sid, pro);
    g.seats[pro.index()].stockpile.materials = 500;
    // An Embassy, then a Bank -- which the Prospectors' order raises as their Investment Bank.
    g.commit_orders(pro, &[Order::BuildFacility { state: sid, kind: FacilityKind::Embassy }, Order::BuildFacility { state: sid, kind: FacilityKind::Bank }]);
    assert_eq!(g.state(sid).queue.iter().map(|b| (b.seat, b.widgets, b.done)).collect::<Vec<_>>(), vec![(pro, 3, 0), (pro, 3, 0)], "the Prospectors' figures");
    assert_eq!(g.state(sid).queue[1].item, BuildItem::Facility(FacilityKind::InvestmentBank));
    g.state_mut(sid).queue[0].done = 2;
    g.transfer_control(Place::State(sid), Seat(0), "Influence");
    assert_eq!(g.state(sid).queue.len(), 2, "the queue stays");
    assert!(g.state(sid).queue.iter().all(|b| b.seat == pro), "each build keeps its seat");
    // The new holder may cancel: the Embassy's Materials at the Custodians' price, 30, not the
    // Prospectors' 25; the 2 Widgets done are lost.
    directed(&mut g, sid);
    let cancel = Order::CancelBuild { place: Place::State(sid), index: 0 };
    assert_eq!(g.facility_materials(Seat(0), FacilityKind::Embassy), 30);
    assert_eq!(g.facility_materials(pro, FacilityKind::Embassy), 25);
    assert_eq!(g.order_cost(Seat(0), &cancel).materials, -30, "a refund is a negative cost");
    g.check_order(Seat(0), &[], &cancel).expect("legal on another seat's build at a place you direct");
    assert!(g.check_order(Seat(0), std::slice::from_ref(&cancel), &cancel).is_err(), "one cancel a turn at a place");
    assert!(g.check_order(Seat(0), &[], &Order::CancelBuild { place: Place::State(sid), index: 2 }).is_err(), "past the queue");
    assert!(g.check_order(pro, &[], &cancel).is_err(), "the Prospectors no longer direct Europe");
    let materials = g.seats[0].stockpile.materials;
    g.commit_orders(Seat(0), std::slice::from_ref(&cancel));
    assert_eq!(g.seats[0].stockpile.materials, materials + 30, "30 Materials to the canceller");
    assert_eq!(g.state(sid).queue.iter().map(|b| b.item).collect::<Vec<_>>(), vec![BuildItem::Facility(FacilityKind::InvestmentBank)], "the Embassy is gone");
    assert!(g.log.iter().any(|l| l.contains("cancelled the Embassy under way at") && l.contains("30 Materials")), "{:?}", g.log.iter().rev().take(5).collect::<Vec<_>>());
    // A build of one's own is not cancelled this way.
    g.commit_orders(Seat(0), &[Order::BuildFacility { state: sid, kind: FacilityKind::Bank }]);
    let own = g.state(sid).queue.iter().position(|b| b.seat == Seat(0)).unwrap();
    let err = g.check_order(Seat(0), &[], &Order::CancelBuild { place: Place::State(sid), index: own }).unwrap_err().0;
    assert!(err.contains("your own"), "{err}");
    // And the rival's Investment Bank still finishes as usual, for the Prospectors, in the
    // Custodians' Region.
    for _ in 0..2 {
        g.resolution_phase();
        g.turn += 1;
    }
    assert!(g.state(sid).facilities.iter().any(|f| f.kind == FacilityKind::InvestmentBank), "finished as usual");
    assert!(g.log.iter().any(|l| l.contains("Prospectors completed Investment Bank")), "{:?}", g.log.iter().filter(|l| l.contains("completed")).collect::<Vec<_>>());
}

/// Ticket #332, R10: `turns_to_build` is the Resolutions until a fresh order would complete at
/// this place's Widgets a turn, behind everything already in its queue: at least 1, and
/// `u32::MAX` where the place makes nothing; `queue_estimates` says the same of each build queued.
#[test]
fn turns_to_build_estimates_at_the_places_rate_behind_its_queue() {
    let mut g = game();
    calm(&mut g);
    let sid = StateId::EastAsia;
    directed(&mut g, sid);
    let place = Place::State(sid);
    let bank = BuildItem::Facility(FacilityKind::Bank);
    assert_eq!(g.widgets_at(place), 7, "a flat 4 and Industry Level 3");
    assert_eq!(g.turns_to_build(Seat(0), place, bank), 1, "4 Widgets at 7 a turn");
    assert_eq!(g.turns_to_build(Seat(0), place, BuildItem::IndustryLevel), 1);
    g.state_mut(sid).queue.push(Build { item: BuildItem::Facility(FacilityKind::PowerPlant), seat: Seat(0), widgets: 8, done: 0, coastal: false });
    assert_eq!(g.turns_to_build(Seat(0), place, bank), 2, "8 owed ahead and 4 more: 12 at 7 a turn, rounded up");
    assert_eq!(g.queue_estimates(place), vec![2], "the Power Plant itself: 8 at 7, rounded up");
    g.state_mut(sid).facilities.push(facility(FacilityKind::Factory));
    assert_eq!(g.widgets_at(place), 11, "and a Factory's four");
    assert_eq!(g.turns_to_build(Seat(0), place, bank), 2, "12 at 11 a turn: rounded up, not down");
    assert_eq!(g.queue_estimates(place), vec![1], "the Power Plant itself: 8 at 11");
    g.state_mut(sid).queue[0].done = 6;
    assert_eq!(g.turns_to_build(Seat(0), place, bank), 1, "2 owed ahead and 4 more: 6 at 11");
    assert_eq!(g.queue_estimates(place), vec![1]);
    // A place that makes nothing: the Core Module idled.
    let moon = colony(&mut g, Seat(0), BodyId::Moon, &[], 4);
    assert_eq!(g.turns_to_build(Seat(0), Place::Colony(moon), BuildItem::Module(ModuleKind::Shipyard)), 2, "8 at the Core Module's 4");
    let core = g.colony(moon).unwrap().modules.iter().position(|m| m.kind == ModuleKind::Core).unwrap();
    g.colony_mut(moon).unwrap().modules[core].mothballed = true;
    assert_eq!(g.turns_to_build(Seat(0), Place::Colony(moon), BuildItem::Module(ModuleKind::Shipyard)), u32::MAX, "nothing here makes Widgets");
    // The Under way block reads the same estimate.
    let u = g.under_way(Seat(0));
    assert_eq!(u.builds, vec![("Power Plant".to_string(), place, 1)]);
}

// -------------------------------------------- 0.09.0 ticket #332: the computer seats under Widgets

/// The score the AI's log gave the first candidate whose note contains `needle`, from the
/// `  take    12.0  build Mine in Mexico` lines `ai_orders` writes; a candidate it never scored is
/// nought. The three tests below read the RANKING, since what a seat can afford in one turn is
/// decided by its reserve and its whole list, and the rules under test are about where a build
/// goes, not how many it buys.
fn scored(g: &Game, needle: &str) -> f64 {
    g.log
        .iter()
        .filter(|l| l.starts_with("  ") && l.contains(needle))
        .filter_map(|l| l.split_whitespace().nth(1).and_then(|n| n.parse::<f64>().ok()))
        .next()
        .unwrap_or(0.0)
}

/// Ticket #332 (version 0.09.0): every seat wants a Mine early in its most Materials-lean Region,
/// at the Factory's weight. The seat holds an Energy-lean Region (Australia) and a Materials-lean
/// one (Central America), a Mine standing in the wrong one already so no bootstrap fires: the
/// Mine in Central America, whose lean makes it six a turn, outscores the one in Australia, which
/// stands first on the list and would make four, and is bought.
#[test]
fn the_ai_wants_an_early_mine_in_its_most_materials_lean_region() {
    let mut g = game();
    calm(&mut g);
    let cust = Seat(0);
    g.state_mut(StateId::EastAsia).control = Control::Neutral;
    g.take_control(StateId::Australia, cust);
    g.take_control(StateId::CentralAmerica, cust);
    g.state_mut(StateId::Australia).facilities.push(facility(FacilityKind::Mine));
    assert_eq!(g.tables.state(StateId::CentralAmerica).resource_lean, Resource::Materials);
    assert_eq!(g.tables.state(StateId::Australia).resource_lean, Resource::Energy);
    g.turn = 2;
    g.seats[0].stockpile.materials = 50;
    g.seats[0].stockpile.energy = 200;
    g.seats[0].income_last_turn.materials = 4;
    g.seats[0].income_last_turn.energy = 20;
    let orders = g.ai_orders(cust);
    let (mexico, australia) = (scored(&g, "build Mine in Mexico"), scored(&g, "build Mine in Australia"));
    assert!(mexico > australia, "the Materials-lean Region's Mine outscores the other's: Mexico {mexico}, Australia {australia}");
    assert!(
        orders.iter().any(|o| matches!(o, Order::BuildFacility { state: StateId::CentralAmerica, kind: FacilityKind::Mine })),
        "and it is bought: {orders:?}"
    );
    assert!(!orders.iter().any(|o| matches!(o, Order::BuildFacility { state: StateId::Australia, kind: FacilityKind::Mine })), "and the other is not: {orders:?}");
}

/// Ticket #332: a Factory Module is wanted at a Colony whose queue is two deep, ahead of the Mine
/// that outscores it while Materials are scarce; and not at a Colony with nothing under way and
/// no Shipyard, where its Widgets would be lost.
#[test]
fn the_ai_wants_a_factory_module_where_a_colonys_queue_is_two_deep() {
    let mut g = game();
    calm(&mut g);
    let cust = Seat(0);
    g.turn = 5;
    // Materials for the early Mine and the Scrubber the Custodians open with, the Trade Post that
    // scores the same 12 as the Factory Module and stands earlier on the list, and the Module.
    g.seats[0].stockpile.materials = 95;
    g.seats[0].stockpile.energy = 500;
    g.seats[0].income_last_turn.materials = 6;
    g.seats[0].income_last_turn.energy = 20;
    // The ISS emptied: with nobody aboard it has no Module slot, so it offers nothing that would
    // hold the Materials (its Trade Post scored the same 12 and stood earlier on the list).
    let iss = station_of(&g, cust, BodyId::Earth).unwrap();
    g.colony_mut(iss).unwrap().colonists = 0;
    let moon = colony(&mut g, cust, BodyId::Moon, &[], 4);
    for _ in 0..2 {
        g.colony_mut(moon).unwrap().queue.push(Build { item: BuildItem::Module(ModuleKind::Habitat), seat: cust, widgets: 4, done: 0, coastal: false });
    }
    let orders = g.ai_orders(cust);
    let (factory, mine) = (scored(&g, "build Factory at Mare"), scored(&g, "build Mine at Mare"));
    assert!(factory > mine, "the Factory Module outscores the Mine where the queue is two deep: Factory {factory}, Mine {mine}");
    let lines: Vec<String> = g.log.iter().filter(|l| l.starts_with("  take") || l.starts_with("  wait") || l.starts_with("  save")).take(12).cloned().collect();
    assert!(
        orders.iter().any(|o| matches!(o, Order::BuildModule { colony, kind: ModuleKind::Factory } if *colony == moon)),
        "and it is bought: {orders:?}\nscored: {lines:#?}"
    );
    // The same Colony idle: no Factory Module is offered at all.
    g.colony_mut(moon).unwrap().queue.clear();
    g.seats[0].stockpile.materials = 95;
    g.log.clear();
    g.ai_orders(cust);
    assert_eq!(scored(&g, "build Factory at Mare"), 0.0, "none where nothing is under way");
}

/// Ticket #332: Ships are built at the yard with the most Widgets. Two Shipyards: the ISS, whose
/// Core Module makes four Widgets a turn, and a Moon Colony with a Factory Module beside its Core,
/// eight. The Moon Colony was founded later, so it stands later on the list and would have lost a
/// tie; with Materials for everything the seat wants, every Ship ordered goes to the Moon.
#[test]
fn the_ai_builds_its_ships_at_the_yard_with_the_most_widgets() {
    let mut g = game();
    calm(&mut g);
    let cust = Seat(0);
    let iss = station_of(&g, cust, BodyId::Earth).unwrap();
    g.colony_mut(iss).unwrap().modules.push(Module::new(ModuleKind::Shipyard));
    g.colony_mut(iss).unwrap().colonists = 4;
    let moon = colony(&mut g, cust, BodyId::Moon, &[ModuleKind::Shipyard, ModuleKind::Factory], 4);
    assert_eq!(g.widgets_at(Place::Colony(iss)), 4, "the Core Module's four");
    assert_eq!(g.widgets_at(Place::Colony(moon)), 8, "and a Factory Module's four beside it");
    g.turn = 6;
    g.seats[0].stockpile.materials = 400;
    g.seats[0].stockpile.energy = 500;
    g.seats[0].stockpile.fuel = 100;
    g.seats[0].income_last_turn.materials = 6;
    g.seats[0].income_last_turn.energy = 20;
    let orders = g.ai_orders(cust);
    let scored_ships: Vec<String> = g.log.iter().filter(|l| l.contains("Ship at")).cloned().collect();
    let ships: Vec<&Order> = orders.iter().filter(|o| matches!(o, Order::BuildShip { .. })).collect();
    assert!(!ships.is_empty(), "a Ship is ordered somewhere: {orders:?}\nscored: {scored_ships:#?}");
    assert!(
        ships.iter().all(|o| matches!(o, Order::BuildShip { site, .. } if *site == Place::Colony(moon))),
        "every Ship at the Moon yard, eight Widgets against the ISS's four: {ships:?}\nscored: {scored_ships:#?}"
    );
}

// ---------------------------------- 0.09.0 ticket #333: one unit of population per million people

/// Ticket #333 (version 0.09.0): one unit of population is one million people, read off
/// `climate.toml` rather than a code constant, and a Colonist is one unit -- so a Pioneer takes
/// exactly one million people from its Region -- and the card keeps its shape, the unit figure to
/// one decimal with the people in brackets. The designer: *"pop 1 per million"*.
#[test]
fn one_unit_of_population_is_one_million_people_and_a_pioneer_takes_exactly_one() {
    let mut g = game();
    calm(&mut g);
    let people_per_unit = g.tables.climate.people_per_unit;
    assert_eq!(people_per_unit, 1_000_000.0, "one unit is one million people");
    // The world opens at 7,860 units, 7.86 billion people; China at 1,440, 1.44 billion.
    let world: f64 = g.tables.states.iter().map(|s| s.population).sum();
    assert_eq!(world, 7860.0);
    assert_eq!(g.tables.people_text(world), "7.86B");
    assert_eq!(g.tables.state(StateId::EastAsia).population, 1440.0);
    // A Pioneer takes one unit, one million people, from its Region, at the recruit.
    let before = g.state(StateId::EastAsia).population;
    g.commit_orders(Seat(0), &[Order::BuildEmigrants { state: StateId::EastAsia, n: 1 }]);
    let taken_people = (before - g.state(StateId::EastAsia).population) * people_per_unit;
    assert!((taken_people - 1_000_000.0).abs() < 1e-3, "a Pioneer took {taken_people} people, not one million");
    assert!((g.lift_population(Seat(0), 1) * people_per_unit - 1_000_000.0).abs() < 1e-3, "and the button's cost in people says one million");
    assert_eq!(g.tables.people_text(1.0), "1M", "one Colonist, in the people form");
    // The card's form: the unit figure to one decimal, the people in brackets.
    assert_eq!(g.tables.population_text(1454.5), "1454.5 (1.45B)");
    assert_eq!(g.tables.population_text(380.0), "380.0 (380M)");
    assert_eq!(g.tables.units_per_hundred_million(), 100.0, "a hundred units to the hundred million the cards quote by");
}

// ---------------------------------------------- 0.09.0 ticket #334: Armies raised from people

/// Ticket #334 (version 0.09.0): a raised Army takes people. In a Region, `[army]
/// population_each` units of its population -- one, one million people -- at the order, on top of
/// its Materials and Widgets, whatever the Army's strength; refused where the Region has not got
/// it, with a refusal that names the rule. The designer: *"armies from people too"*.
#[test]
fn a_raised_army_takes_one_unit_of_its_regions_population_and_is_refused_below_it() {
    let mut g = game();
    g.seats[0].stockpile.materials = 2000;
    let each = g.tables.army.population_each;
    assert_eq!(each, 1.0, "one unit of population an Army, one million people");
    let raise = Order::BuildArmy { place: Place::State(StateId::EastAsia) };
    let before = g.state(StateId::EastAsia).population;
    assert!(g.check_order(Seat(0), &[], &raise).is_ok());
    g.commit_orders(Seat(0), std::slice::from_ref(&raise));
    let taken = before - g.state(StateId::EastAsia).population;
    assert!((taken - each).abs() < 1e-9, "the raise took {taken} units of population, not {each}");
    assert!((taken * g.tables.climate.people_per_unit - 1_000_000.0).abs() < 1e-3, "one million people");
    assert_eq!(g.state(StateId::EastAsia).queue.len(), 1, "and the Army is in the queue, its Materials and Widgets as before");
    assert!(g.log.to_vec().iter().any(|l| l.contains("under arms")), "the Report names the people taken: {:?}", g.log.to_vec());
    // Below the figure the raise is refused, and the refusal names the rule.
    g.state_mut(StateId::EastAsia).population = each - 0.5;
    let err = g.check_order(Seat(0), &[], &raise).expect_err("a Region under one unit cannot raise an Army");
    assert!(err.0.contains("not the people for an Army"), "the refusal names the rule: {}", err.0);
    // At exactly the figure it may; two in one turn want two.
    g.state_mut(StateId::EastAsia).population = each;
    assert!(g.check_order(Seat(0), &[], &raise).is_ok(), "at exactly one unit the Region has the people");
    assert!(g.check_order(Seat(0), std::slice::from_ref(&raise), &raise).is_err(), "a second raise this turn wants a second unit");
    g.commit_orders(Seat(0), &[raise]);
    assert_eq!(g.state(StateId::EastAsia).population, 0.0, "the last unit went under arms");
}

/// Ticket #334 (b): a Colony's Army takes one Colonist at the raise, and its Module slot with them;
/// refused at fewer than two so the Core Module is never emptied. A Colony whose Colonists fall
/// below its Modules keeps them all -- ticket #97's rule -- and simply has no room until they are back.
#[test]
fn a_colonys_army_takes_one_colonist_and_is_refused_at_one() {
    let mut g = game();
    g.seats[0].stockpile.materials = 2000;
    assert_eq!(g.tables.army.colonists_each, 1, "one Colonist a raise");
    let c = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Barracks, ModuleKind::Mine, ModuleKind::Mine], 3);
    let raise = Order::BuildArmy { place: Place::Colony(c) };
    let pop = g.state(StateId::EastAsia).population;
    assert!(g.check_order(Seat(0), &[], &raise).is_ok());
    g.commit_orders(Seat(0), std::slice::from_ref(&raise));
    assert_eq!(g.colony(c).unwrap().colonists, 2, "one Colonist went under arms");
    assert_eq!(g.state(StateId::EastAsia).population, pop, "and no Region paid for a Colony's Army");
    assert!(g.log.to_vec().iter().any(|l| l.contains("one Colonist under arms")), "the Report names the Colonist: {:?}", g.log.to_vec());
    // The slot went with them: three Modules on a cap of two stand, and nothing more fits.
    let col = g.colony(c).unwrap();
    assert_eq!(g.module_slots(col), 2);
    assert_eq!(g.module_slots_used(col), 3, "the Barracks and both Mines stand: nothing is destroyed or mothballed");
    assert_eq!(g.free_module_slots(col), 0, "and there is no room until a Colonist arrives");
    // At one Colonist the raise is refused, and the refusal names the rule.
    let mut g = game();
    g.seats[0].stockpile.materials = 2000;
    let c = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Barracks], 1);
    let raise = Order::BuildArmy { place: Place::Colony(c) };
    let err = g.check_order(Seat(0), &[], &raise).expect_err("a Colony of one Colonist cannot raise an Army");
    assert!(err.0.contains("keeps at least one Colonist"), "the refusal names the rule: {}", err.0);
    g.colony_mut(c).unwrap().colonists = 2;
    assert!(g.check_order(Seat(0), &[], &raise).is_ok(), "at two it may: one stays with the Core");
}

/// Ticket #359 (version 0.09.1): a SHUT Barracks -- mothballed, or dark for want of Energy -- raises
/// no Army and repairs none, where it did both while it merely stood. Three refusals in the
/// Shipyard's shape: none, still building, shut. An Army already standing is untouched.
#[test]
fn a_shut_barracks_raises_and_repairs_no_army() {
    let mut g = game();
    g.seats[0].stockpile.materials = 2000;
    let c = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Barracks, ModuleKind::Mine], 4);
    let raise = Order::BuildArmy { place: Place::Colony(c) };
    assert!(g.check_order(Seat(0), &[], &raise).is_ok(), "a working Barracks raises");
    g.colony_mut(c).unwrap().modules[0].mothballed = true;
    assert_eq!(g.check_order(Seat(0), &[], &raise).unwrap_err().0, "the Barracks here is shut: mothballed, or dark for want of Energy");
    g.colony_mut(c).unwrap().modules[0].mothballed = false;
    g.colony_mut(c).unwrap().modules[0].online = false;
    assert_eq!(g.check_order(Seat(0), &[], &raise).unwrap_err().0, "the Barracks here is shut: mothballed, or dark for want of Energy", "dark counts as shut");
    g.colony_mut(c).unwrap().modules.remove(0);
    assert_eq!(g.check_order(Seat(0), &[], &raise).unwrap_err().0, "no Barracks here");
    build_now(&mut g, Place::Colony(c), BuildItem::Module(ModuleKind::Barracks), Seat(0));
    assert_eq!(g.check_order(Seat(0), &[], &raise).unwrap_err().0, "the Barracks here is still building");
    // Repair: the same three doors, on an Army standing at the Colony, which a shut Barracks
    // leaves standing.
    g.colony_mut(c).unwrap().queue.clear();
    g.colony_mut(c).unwrap().modules.insert(0, Module::new(ModuleKind::Barracks));
    let army = ArmyId(g.fresh_id());
    g.armies.push(Army { id: army, name: "the 1st".to_string(), home: ArmyHome::Colony(c), at: ArmyAt::Place(Place::Colony(c)), damage: 2, standing: false, stance: Stance::Hold, escaped: false, move_to: None, levy: false, raised_strength: 3 });
    let repair = Order::Repair { unit: UnitRef::Army(army), points: 1 };
    assert!(g.check_order(Seat(0), &[], &repair).is_ok(), "a working Barracks repairs");
    g.colony_mut(c).unwrap().modules[0].mothballed = true;
    assert_eq!(g.check_order(Seat(0), &[], &repair).unwrap_err().0, "the Barracks here is shut: mothballed, or dark for want of Energy");
    assert!(g.army(army).is_some(), "the garrison stands");
}

/// Ticket #359 (version 0.09.1): a shut Habitat still houses its people, and while it is OCCUPIED --
/// more Colonists than the working Habitats and the Core can hold -- the Colony makes everything at
/// half EXCEPT Energy, at the designer's word. An empty spare Habitat mothballed costs nothing, and
/// two shut Habitats halve once.
#[test]
fn an_occupied_shut_habitat_halves_the_colony_but_its_energy() {
    let setup = |colonists: u32, shut: &[usize]| {
        let mut g = game();
        calm(&mut g);
        g.seats[0].stockpile.energy = 500;
        let c = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Habitat, ModuleKind::Habitat, ModuleKind::Mine, ModuleKind::Mine, ModuleKind::Generator, ModuleKind::Relay], colonists);
        for i in shut {
            g.colony_mut(c).unwrap().modules[*i].mothballed = true;
        }
        (g, c)
    };
    let gain = |mut g: Game| {
        let (m, e) = (g.seats[0].stockpile.materials, g.seats[0].stockpile.energy);
        g.income_phase();
        (g.seats[0].stockpile.materials - m, g.seats[0].stockpile.energy - e, g.seats[0].allotment)
    };
    // Room: two Habitats of 4 and the Core's 4, twelve. Ten live here.
    let (g, c) = setup(10, &[]);
    assert_eq!(g.habitat_room(g.colony(c).unwrap()), 12);
    assert!(!g.habitat_halves(c), "every Habitat working");
    let (whole_m, whole_e, whole_a) = gain(g);
    // One shut: eight of room working, ten living. Occupied: half.
    let (g, c) = setup(10, &[0]);
    assert_eq!(g.habitat_room(g.colony(c).unwrap()), 12, "a shut Habitat still houses them: room unchanged");
    assert!(g.habitat_halves(c));
    let (half_m, half_e, half_a) = gain(g);
    // The Moon's Mines are the only Materials this seat makes off Earth; the rest is Earth's, whole.
    let (earth, _) = setup(0, &[]);
    let (earth_m, _, _) = {
        let mut e = earth;
        e.colonies.retain(|col| col.body != BodyId::Moon);
        gain(e)
    };
    assert_eq!(half_m - earth_m, (whole_m - earth_m) / 2, "the Colony's Materials at half, rounded down");
    assert_eq!(half_e, whole_e + 2, "Energy untouched -- the Generator whole, and the shut Habitat's 2 upkeep saved");
    assert!(half_a < whole_a, "the Relay's Allotment at half: {half_a} against {whole_a}");
    // Dark counts as shut.
    let (mut g, c) = setup(10, &[]);
    g.colony_mut(c).unwrap().modules[0].online = false;
    assert!(g.habitat_halves(c), "a dark Habitat is shut");
    // An empty spare: eight living on eight of working room. Not occupied, nothing halved.
    let (g, c) = setup(8, &[0]);
    assert!(!g.habitat_halves(c), "a spare Habitat standing empty costs nothing");
    // Two shut halve once, not twice.
    let (g, c) = setup(10, &[0, 1]);
    assert!(g.habitat_halves(c));
    let (twice_m, _, _) = gain(g);
    assert_eq!(twice_m, half_m, "half once, however many are shut");
}

/// Ticket #352 (version 0.09.1): a Research Lab's hover is its arithmetic, in the rule's order, with
/// the rounding last -- the designer's *"mouseover explains math for research output"*. The words
/// are pinned for the Custodians' China on turn 1, the example the ticket was decided on.
#[test]
fn a_research_labs_hover_is_its_arithmetic() {
    let g = game();
    assert_eq!(g.kind(Seat(0)), FactionKind::Custodians);
    let y = g.facility_yield(Seat(0), StateId::EastAsia, FacilityKind::ResearchLab);
    assert_eq!(y.research, 3);
    assert_eq!(
        y.chain.lines(6),
        vec![
            "2 base".to_string(),
            "× 1.32 for 1.44B people, weighted by Education".to_string(),
            "× 1.10 for Education 1.10".to_string(),
            "× 1.25 as the Custodians".to_string(),
            "= 3.62, rounded down to 3".to_string(),
        ]
    );
    // Past the ceiling the later factors share a line rather than any being dropped.
    let short = y.chain.lines(4);
    assert_eq!(short.len(), 4);
    assert_eq!(short[2], "× 1.10 for Education 1.10, × 1.25 as the Custodians");
}

/// Ticket #352: the chain IS the figure. For every Facility kind in every Region, and every Module
/// kind at a Colony, the chain's last value is what the game pays -- so no hover can say one thing
/// while Income does another.
#[test]
fn every_chain_ends_on_the_figure_the_game_pays() {
    let mut g = game();
    let c = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Habitat, ModuleKind::Mine, ModuleKind::Generator, ModuleKind::Observatory, ModuleKind::TradePost, ModuleKind::SolarArray, ModuleKind::Refinery], 6);
    let settle = |y: &Yield| {
        let v = if y.research > 0 { y.research } else { y.amount };
        let last = y.chain.lines(99).last().cloned().unwrap_or_default();
        (v, last)
    };
    let mut checked = 0;
    for sid in StateId::ALL {
        for fk in FacilityKind::ALL {
            let y = g.facility_yield(Seat(0), sid, fk);
            if !y.chain.multiplied() {
                continue;
            }
            let (v, last) = settle(&y);
            assert!(last.ends_with(&format!(" {v}")) || last == format!("= {v}"), "{fk:?} in {sid:?} pays {v} and its chain ends {last:?}");
            checked += 1;
        }
    }
    for (i, m) in g.colony(c).unwrap().modules.clone().iter().enumerate() {
        let y = g.module_yield_at(Seat(0), c, i);
        if !y.chain.multiplied() {
            continue;
        }
        let (v, last) = settle(&y);
        assert!(last.ends_with(&format!(" {v}")) || last == format!("= {v}"), "{:?} pays {v} and its chain ends {last:?}", m.kind);
        checked += 1;
    }
    assert!(checked > 50, "the board was actually walked: {checked}");
}

/// Ticket #334 (c): the Standing Army is the state's, and takes nobody -- neither when the game
/// begins nor when it is raised again two Incomes after it dies.
#[test]
fn the_standing_armys_respawn_takes_no_people() {
    let mut g = game();
    calm(&mut g);
    let sid = StateId::EastAsia;
    let standing = g.armies.iter().find(|a| a.standing && a.home == ArmyHome::State(sid)).map(|a| a.id).unwrap();
    g.destroy_army(standing, "battle", None);
    assert_eq!(g.state(sid).respawn_wait, 1, "two Incomes later");
    let before = g.state(sid).population;
    g.income_phase();
    assert!(!g.armies.iter().any(|a| a.standing && a.home == ArmyHome::State(sid)), "the first Income raises nothing");
    g.income_phase();
    assert!(g.armies.iter().any(|a| a.standing && a.home == ArmyHome::State(sid)), "the second raises it again");
    let after = g.state(sid).population;
    assert!(after >= before, "the Standing Army's return took {} units of population; it takes nobody", before - after);
}

/// Ticket #334 (d): the people are gone. A destroyed raised Army returns nobody to its Region, and a
/// Colony's returns no Colonist.
#[test]
fn a_destroyed_raised_army_returns_nobody() {
    let mut g = game();
    g.seats[0].stockpile.materials = 2000;
    let raise = Order::BuildArmy { place: Place::State(StateId::EastAsia) };
    g.commit_orders(Seat(0), &[raise]);
    let after_raise = g.state(StateId::EastAsia).population;
    let id = g.raise_army(Place::State(StateId::EastAsia), false);
    g.destroy_army(id, "battle", Some(ReportPlace::State(StateId::EastAsia)));
    assert!(!g.armies.iter().any(|a| a.id == id));
    assert_eq!(g.state(StateId::EastAsia).population, after_raise, "nobody came home");
    let c = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Barracks], 3);
    g.commit_orders(Seat(0), &[Order::BuildArmy { place: Place::Colony(c) }]);
    assert_eq!(g.colony(c).unwrap().colonists, 2);
    let id = g.raise_army(Place::Colony(c), false);
    g.destroy_army(id, "battle", Some(ReportPlace::Colony(c)));
    assert_eq!(g.colony(c).unwrap().colonists, 2, "no Colonist came back");
}

// ---------------------------------------------------------------- 0.09.0 ticket #335: orbits

/// A Ship of a seat's standing in one orbit of a Body: `None` is low orbit, `Some(n)` the ring of
/// Orbital Slot n. Ticket #335 (version 0.09.0) made that the one thing a Ship's position is.
fn ship_in(g: &mut Game, seat: Seat, kind: UnitKind, body: BodyId, slot: Option<u32>, stance: Stance) -> ShipId {
    let id = ShipId(g.fresh_id());
    let name = g.next_ship_name(kind);
    g.ships.push(Ship {
        id, name, kind, seat, damage: 0, at: ShipAt::Body(body), colonists: 0, warhead: false, colonists_education: 1.0, army: None,
        stance, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot,
    });
    id
}

/// A build that completes at the next Resolution: a queue row with no Widgets left to do. The
/// Shipyard's queue is the one door a Ship comes into a real game through, and ticket #335 makes
/// the orbit it comes into the yard's own.
fn build_now(g: &mut Game, place: Place, item: BuildItem, seat: Seat) {
    let b = Build { item, seat, widgets: 0, done: 0, coastal: false };
    match place {
        Place::State(s) => g.state_mut(s).queue.push(b),
        Place::Colony(c) => g.colony_mut(c).unwrap().queue.push(b),
    }
}

/// Ticket #335 (R1): where a Ship is. A Body's orbits are LOW ORBIT plus one per Orbital Slot --
/// twenty-one on the board -- and a new Ship starts in the orbit of the Shipyard that built it: a
/// station's yard at that station's ring, a ground Colony's yard in low orbit. A leg that names no
/// orbit arrives in low orbit, and the helper names an orbit for a player to read.
#[test]
fn a_new_ship_starts_in_the_orbit_of_the_yard_that_built_it() {
    let mut g = game();
    let total: usize = BodyId::ALL.iter().map(|b| g.orbits_of(*b).len()).sum();
    assert_eq!(total, 21, "low orbit plus one per Orbital Slot, over six Bodies");
    assert_eq!(g.orbits_of(BodyId::Mars)[0], Orbit::Low, "low orbit is the first of them");
    assert_eq!(g.orbit_name(BodyId::Mars, Orbit::Low), "Mars, low orbit");
    let iss = station_of(&g, Seat(0), BodyId::Earth).expect("the ISS");
    let iss_slot = g.colony(iss).unwrap().slot;
    assert_eq!(g.orbit_name(BodyId::Earth, Orbit::Slot(iss_slot)), "Earth, at ISS");
    assert!(!g.orbit_exists(BodyId::Phobos, Orbit::Slot(1)), "Phobos has one Orbital Slot");
    // A station's Shipyard puts its Ship at that station's own ring.
    build_now(&mut g, Place::Colony(iss), BuildItem::Unit(UnitKind::Frigate), Seat(0));
    g.resolution_phase();
    let built = g.ships.iter().find(|s| s.kind == UnitKind::Frigate).expect("the Frigate was built").clone();
    assert_eq!((built.at, g.ship_orbit(&built)), (ShipAt::Body(BodyId::Earth), Orbit::Slot(iss_slot)), "the orbit of the yard that built it");
    // A ground Colony's yard puts it in low orbit.
    let ground = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Shipyard], 4);
    build_now(&mut g, Place::Colony(ground), BuildItem::Unit(UnitKind::Battleship), Seat(0));
    g.resolution_phase();
    let built = g.ships.iter().find(|s| s.kind == UnitKind::Battleship).expect("the Battleship was built").clone();
    assert_eq!((built.at, g.ship_orbit(&built)), (ShipAt::Body(BodyId::Moon), Orbit::Low), "a ground yard builds into low orbit");
    // A leg that names no orbit arrives in low orbit, whatever orbit it left.
    let id = built.id;
    g.ship_mut(id).unwrap().slot = Some(0);
    let leg = Order::Transit { ship: id, to: BodyId::Earth, slot: None };
    g.commit_orders(Seat(0), std::slice::from_ref(&leg));
    for _ in 0..6 {
        if matches!(g.ship(id).unwrap().at, ShipAt::Body(_)) {
            break;
        }
        g.resolution_phase();
    }
    assert_eq!(g.ship(id).unwrap().at, ShipAt::Body(BodyId::Earth), "it arrived");
    assert_eq!(g.ship_orbit(g.ship(id).unwrap()), Orbit::Low, "a leg that named no orbit arrives in low orbit");
}

/// Ticket #335 (R2): changing orbit. An order for a Ship at a Body, to an orbit of that Body that
/// exists and is not the one it is in, costing `orbit_change_fuel` out of the Ship's own tank and
/// refused below it; one order a turn like any other; resolved WITH the transits, before the
/// Battles, so a Ship that changes orbit fights in its new one.
#[test]
fn an_orbit_change_costs_one_fuel_from_the_tank_and_lands_before_the_battles() {
    let mut g = game();
    let fuel = g.tables.orbit_change_fuel;
    assert_eq!(fuel, 1, "bodies.toml: the sibling hop's figure");
    let iss = station_of(&g, Seat(0), BodyId::Earth).expect("the ISS");
    let slot = g.colony(iss).unwrap().slot;
    let ship = ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Earth, None, Stance::Hold);
    g.ship_mut(ship).unwrap().fuel = 4;
    // Refused to the orbit it is already in, and to an orbit the Body has not got.
    let err = g.check_order(Seat(0), &[], &Order::ChangeOrbit { ship, slot: None }).unwrap_err().0;
    assert!(err.contains("already in"), "{err}");
    let slots = g.tables.body(BodyId::Earth).orbital_slots;
    let err = g.check_order(Seat(0), &[], &Order::ChangeOrbit { ship, slot: Some(slots) }).unwrap_err().0;
    assert!(err.contains("Orbital Slots"), "{err}");
    // The Stockpile pays nothing: the tank does.
    let order = Order::ChangeOrbit { ship, slot: Some(slot) };
    let cost = g.order_cost(Seat(0), &order);
    assert_eq!((cost.fuel, cost.materials), (0, 0), "an orbit change spends the tank, not the Stockpile");
    assert!(g.check_order(Seat(0), &[], &order).is_ok());
    let err = g.check_order(Seat(0), std::slice::from_ref(&order), &Order::Transit { ship, to: BodyId::Moon, slot: None }).unwrap_err().0;
    assert!(err.contains("already has an order"), "one order a turn: {err}");
    // A rival on Attack is waiting at the ring it is moving to: the move lands first, so the Ship
    // fights in its new orbit.
    ship_in(&mut g, Seat(1), UnitKind::Frigate, BodyId::Earth, Some(slot), Stance::Attack);
    g.commit_orders(Seat(0), std::slice::from_ref(&order));
    assert_eq!(g.ship(ship).unwrap().fuel, 4 - fuel, "the Fuel left the tank at the order");
    assert_eq!(g.ship_orbit(g.ship(ship).unwrap()), Orbit::Low, "and it has not moved yet");
    g.resolution_phase();
    assert_eq!(g.ship_orbit(g.ship(ship).unwrap()), Orbit::Slot(slot), "it moved with the transits");
    let at = Some(ReportPlace::Orbit(BodyId::Earth, Orbit::Slot(slot)));
    let line = g.report.battles.iter().find(|b| b.at == at).expect("a Battle at the ring it moved to");
    assert!(
        line.parties.iter().any(|p| p.seat == Some(Seat(0))),
        "it fought in its new orbit: {:?}",
        line.parties.iter().map(|p| p.seat).collect::<Vec<_>>()
    );
    // Refused below the figure, naming it.
    g.ship_mut(ship).unwrap().fuel = 0;
    let err = g.check_order(Seat(0), &[], &Order::ChangeOrbit { ship, slot: None }).unwrap_err().0;
    assert!(err.contains(&format!("needs {fuel}")), "{err}");
}

/// Ticket #335 (R3): what each orbit is for. LOW ORBIT touches the ground -- founding a Colony and
/// taking a lift from a Launch Site -- and a STATION'S OWN ORBIT touches that station: unloading
/// into it and refuelling at it. Neither reaches the other.
#[test]
fn low_orbit_touches_the_ground_and_a_stations_own_orbit_touches_the_station() {
    let mut g = game();
    bare_stations(&mut g);
    let iss = station_of(&g, Seat(0), BodyId::Earth).expect("the ISS");
    let slot = g.colony(iss).unwrap().slot;
    g.seats[0].stockpile.fuel = 100;
    // Refuelling: the station's own ring, never low orbit.
    let tanker = ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Earth, None, Stance::Hold);
    g.ship_mut(tanker).unwrap().fuel = 2;
    let err = g.check_order(Seat(0), &[], &Order::Refuel { ship: tanker }).unwrap_err().0;
    assert!(err.starts_with("Move this Ship to Earth, at ") && err.contains("then refuel next turn"), "a Ship in low orbit fuels at nothing: {err}");
    g.ship_mut(tanker).unwrap().slot = Some(slot);
    assert!(g.check_order(Seat(0), &[], &Order::Refuel { ship: tanker }).is_ok(), "at the ISS's ring it refuels");
    // Unloading into the station: its own ring, never low orbit.
    let hauler = ship_in(&mut g, Seat(0), UnitKind::ColonyShip, BodyId::Earth, None, Stance::Hold);
    g.ship_mut(hauler).unwrap().colonists = 2;
    let aboard = Order::Unload { ship: hauler, colonists: 2, army: false, into: UnloadTarget::Colony(iss) };
    let err = g.check_order(Seat(0), &[], &aboard).unwrap_err().0;
    assert!(err.contains("reached from"), "a station is not unloaded into from low orbit: {err}");
    g.ship_mut(hauler).unwrap().slot = Some(slot);
    assert!(g.check_order(Seat(0), &[], &aboard).is_ok(), "from its ring it is");
    // Ticket #357 (version 0.09.1): a lift from a Launch Site reaches ANY orbit of Earth, at the
    // designer's word, where ticket #335 held it to low orbit.
    g.state_mut(StateId::EastAsia).emigrants = 4;
    let lift = Order::Load { ship: hauler, colonists: 2, from: LoadSource::State(StateId::EastAsia), army: None };
    assert!(g.check_order(Seat(0), &[], &lift).is_ok(), "at the ISS's ring it takes the lift");
    // And the lift LANDS there: committed, it lifts at this turn's Resolution, which reads no orbit.
    let mut lifted = g.clone();
    lifted.commit_orders(Seat(0), std::slice::from_ref(&lift));
    lifted.resolution_phase();
    assert_eq!(lifted.ship(hauler).unwrap().colonists, 4, "two Pioneers lifted onto the two it carried, at the ISS's ring");
    assert_eq!(lifted.state(StateId::EastAsia).emigrants, 2, "and two still waiting");
    g.ship_mut(hauler).unwrap().slot = None;
    assert!(g.check_order(Seat(0), &[], &lift).is_ok(), "in low orbit it takes the lift");
    // Founding a Colony on the ground: low orbit alone.
    let settler = ship_in(&mut g, Seat(0), UnitKind::ColonyShip, BodyId::Moon, Some(0), Stance::Hold);
    g.ship_mut(settler).unwrap().colonists = 4;
    let ground = g.free_slots_on(BodyId::Moon)[0];
    let found = Order::Unload { ship: settler, colonists: 4, army: false, into: UnloadTarget::Slot(BodyId::Moon, ground) };
    let err = g.check_order(Seat(0), &[], &found).unwrap_err().0;
    assert!(err.contains("low orbit"), "a Colony is not founded from a station's ring: {err}");
    g.ship_mut(settler).unwrap().slot = None;
    assert!(g.check_order(Seat(0), &[], &found).is_ok(), "from low orbit it is");
}

/// Ticket #357 (version 0.09.1): a door the orbit shuts says the move that opens it FIRST, then the
/// rule, at the designer's word -- *"move ship to low earth orbit to load"*. Every refusal whose cure
/// is a change of orbit reads "Move this Ship to {orbit}, then {act} next turn: {rule}", and the two
/// that merged a second cause -- a blockaded station, an Army at another Body -- are split, so the
/// move is offered only where the move would open the door.
#[test]
fn a_door_the_orbit_shuts_says_the_move_first() {
    let mut g = game();
    bare_stations(&mut g);
    let iss = station_of(&g, Seat(0), BodyId::Earth).expect("the ISS");
    let slot = g.colony(iss).unwrap().slot;
    let iss_orbit = g.orbit_name(BodyId::Earth, Orbit::Slot(slot));
    let low = g.orbit_name(BodyId::Earth, Orbit::Low);
    let refusal = |g: &Game, o: &Order| g.check_order(Seat(0), &[], o).unwrap_err().0;
    // Refuel, from low orbit: move to the ISS's ring.
    let tanker = ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Earth, None, Stance::Hold);
    g.ship_mut(tanker).unwrap().fuel = 2;
    g.seats[0].stockpile.fuel = 100;
    assert_eq!(refusal(&g, &Order::Refuel { ship: tanker }), format!("Move this Ship to {iss_orbit}, then refuel next turn: a station fuels a Ship in its own orbit alone."));
    // Blockaded, no move helps: the refusal says so and offers none.
    let rival = ship_in(&mut g, Seat(1), UnitKind::Frigate, BodyId::Earth, Some(slot), Stance::Blockade);
    let err = refusal(&g, &Order::Refuel { ship: tanker });
    assert!(!err.contains("Move") && err.contains("blockaded"), "{err}");
    g.ship_mut(tanker).unwrap().slot = Some(slot);
    let err = refusal(&g, &Order::Refuel { ship: tanker });
    assert!(!err.contains("Move") && err.contains("blockaded"), "at the ring itself: {err}");
    g.ships.retain(|s| s.id != rival);
    // Unloading into the ISS from low orbit, and loading off it.
    let hauler = ship_in(&mut g, Seat(0), UnitKind::ColonyShip, BodyId::Earth, None, Stance::Hold);
    g.ship_mut(hauler).unwrap().colonists = 2;
    let iss_name = g.place_name(Place::Colony(iss));
    assert_eq!(refusal(&g, &Order::Unload { ship: hauler, colonists: 2, army: false, into: UnloadTarget::Colony(iss) }), format!("Move this Ship to {iss_orbit}, then unload next turn: {iss_name} is reached from there alone."));
    g.colony_mut(iss).unwrap().colonists = 2;
    g.ship_mut(hauler).unwrap().colonists = 0;
    assert_eq!(refusal(&g, &Order::Load { ship: hauler, colonists: 2, from: LoadSource::Colony(iss), army: None }), format!("Move this Ship to {iss_orbit}, then load next turn: {iss_name} is reached from there alone."));
    // Founding a Colony from a station's ring: move to low orbit.
    let settler = ship_in(&mut g, Seat(0), UnitKind::ColonyShip, BodyId::Moon, Some(0), Stance::Hold);
    g.ship_mut(settler).unwrap().colonists = 4;
    let ground = g.free_slots_on(BodyId::Moon)[0];
    let moon_low = g.orbit_name(BodyId::Moon, Orbit::Low);
    assert_eq!(refusal(&g, &Order::Unload { ship: settler, colonists: 4, army: false, into: UnloadTarget::Slot(BodyId::Moon, ground) }), format!("Move this Ship to {moon_low}, then found the Colony next turn: a Colony is founded from low orbit alone."));
    // An Army lifts from a Region's Launch Site into ANY orbit; from a Colony, only from the orbit
    // that touches it; and an Army at another Body is not offered a move at all.
    let carrier = ship_in(&mut g, Seat(0), UnitKind::Carrier, BodyId::Earth, Some(slot), Stance::Hold);
    let army = ArmyId(g.fresh_id());
    g.armies.push(Army { id: army, name: "the 1st".to_string(), home: ArmyHome::State(StateId::EastAsia), at: ArmyAt::Place(Place::State(StateId::EastAsia)), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None, levy: false, raised_strength: 2 });
    let lift_army = Order::Load { ship: carrier, colonists: 0, from: LoadSource::State(StateId::EastAsia), army: Some(army) };
    assert!(g.check_order(Seat(0), &[], &lift_army).is_ok(), "a Launch Site lifts an Army to a station's ring");
    let camp = colony(&mut g, Seat(0), BodyId::Earth, &[ModuleKind::Habitat], 2);
    let camp_name = g.place_name(Place::Colony(camp));
    g.armies.iter_mut().find(|a| a.id == army).unwrap().at = ArmyAt::Place(Place::Colony(camp));
    assert_eq!(refusal(&g, &lift_army), format!("Move this Ship to {low}, then load next turn: {camp_name} is reached from there alone."));
    let far = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Habitat], 2);
    g.armies.iter_mut().find(|a| a.id == army).unwrap().at = ArmyAt::Place(Place::Colony(far));
    let err = refusal(&g, &lift_army);
    assert!(!err.contains("Move") && err.contains("not at this Body"), "{err}");
}

/// Ticket #335 (R4): Orbital Control is LOW ORBIT's, and a Battery covers its OWN orbit. A warship
/// at a station's ring holds no Control; a station's Battery denies no Control of low orbit; a
/// ground Colony's Battery does.
#[test]
fn orbital_control_is_low_orbits_and_a_battery_covers_its_own_orbit() {
    let mut g = game();
    let station = station_at(&mut g, Seat(1), BodyId::Mars);
    let station_slot = g.colony(station).unwrap().slot;
    let ground = colony(&mut g, Seat(1), BodyId::Mars, &[ModuleKind::Habitat], 4);
    let ship = ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Mars, Some(station_slot), Stance::Hold);
    assert_eq!(g.orbital_control(BodyId::Mars), None, "a warship at a station's ring holds no Control of low orbit");
    g.ship_mut(ship).unwrap().slot = None;
    assert_eq!(g.orbital_control(BodyId::Mars), Some(Seat(0)), "in low orbit it holds it");
    assert!(!g.may_land(Seat(1), BodyId::Mars), "and the ground is shut to the rival");
    // A rival's Battery on the STATION covers its own ring and nothing else.
    g.colony_mut(station).unwrap().modules.push(Module::new(ModuleKind::Battery));
    assert!(g.battery_stands_against(Seat(0), BodyId::Mars, Orbit::Slot(station_slot)), "it covers the station's ring");
    assert!(!g.battery_stands_against(Seat(0), BodyId::Mars, Orbit::Low), "and not low orbit");
    assert_eq!(g.orbital_control(BodyId::Mars), Some(Seat(0)), "so Control of low orbit stands");
    // A rival's Battery on the GROUND stands in low orbit's line, and denies it.
    g.colony_mut(ground).unwrap().modules.push(Module::new(ModuleKind::Battery));
    assert!(g.battery_stands_against(Seat(0), BodyId::Mars, Orbit::Low), "a ground Colony's Battery covers low orbit");
    assert_eq!(g.orbital_control(BodyId::Mars), None, "and denies Control there");
    assert!(g.may_land(Seat(1), BodyId::Mars), "so its owner lands again");
}

/// Ticket #335 (R5a): battle parties form per ORBIT. Two Attacks at one Body in two orbits are two
/// Battles and two records, and a station's Battery stands in its own ring's fight alone -- the
/// specification's second refutation, that a stack on Attack in low orbit must not fight a
/// station's Battery in a high orbit.
#[test]
fn battle_parties_form_per_orbit_and_a_station_battery_fights_only_its_own_ring() {
    let mut g = game();
    calm(&mut g);
    let station = station_at(&mut g, Seat(1), BodyId::Mars);
    let slot = g.colony(station).unwrap().slot;
    g.colony_mut(station).unwrap().modules.push(Module::new(ModuleKind::Battery));
    // Low orbit: seat 0 on Attack, seat 1 holding. The station's ring: seat 2 on Attack.
    ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Mars, None, Stance::Attack);
    ship_in(&mut g, Seat(1), UnitKind::Frigate, BodyId::Mars, None, Stance::Hold);
    ship_in(&mut g, Seat(2), UnitKind::Frigate, BodyId::Mars, Some(slot), Stance::Attack);
    g.resolution_phase();
    let low = g.report.battles.iter().find(|b| b.at == Some(ReportPlace::Orbit(BodyId::Mars, Orbit::Low))).expect("a Battle in Mars low orbit").clone();
    let ring = g.report.battles.iter().find(|b| b.at == Some(ReportPlace::Orbit(BodyId::Mars, Orbit::Slot(slot)))).expect("a Battle at the station's ring").clone();
    assert_ne!(low.place, ring.place, "two Battles at one Body are two records: {} and {}", low.place, ring.place);
    assert!(low.place.contains("Mars orbit"), "low orbit keeps the wording every Battle record has had: {}", low.place);
    assert!(ring.place.contains(&g.station_name(BodyId::Mars, slot)), "and a station's ring names the station: {}", ring.place);
    let defenders = low.parties.iter().find(|p| p.seat == Some(Seat(1))).expect("seat 1 in low orbit");
    assert!(!defenders.units.contains("Battery"), "the station's Battery is not in low orbit's line: {}", defenders.units);
    let held = ring.parties.iter().find(|p| p.seat == Some(Seat(1))).expect("seat 1 at its own ring");
    assert!(held.units.contains("Battery"), "it stands in its own ring's line: {}", held.units);
}

/// Ticket #335 (R5b): an Intercept catches only arrivals into its OWN orbit. A picket in low orbit
/// never touches a Ship that flew straight to a station's ring.
#[test]
fn an_intercept_catches_only_arrivals_into_its_own_orbit() {
    let mut g = game();
    calm(&mut g);
    let picket = ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Mars, None, Stance::Intercept);
    let inbound = ship_in(&mut g, Seat(1), UnitKind::ColonyShip, BodyId::Earth, None, Stance::Hold);
    g.ship_mut(inbound).unwrap().at = ShipAt::Transit { from: BodyId::Earth, to: BodyId::Mars, turns_left: 1 };
    g.ship_mut(inbound).unwrap().slot = Some(0);
    g.resolution_phase();
    assert_eq!(g.war.interceptions[0], 0, "the arrival went to a ring the picket does not watch");
    // The same arrival into low orbit is caught.
    g.ship_mut(inbound).unwrap().at = ShipAt::Transit { from: BodyId::Earth, to: BodyId::Mars, turns_left: 1 };
    g.ship_mut(inbound).unwrap().slot = None;
    g.ship_mut(picket).unwrap().stance = Stance::Intercept;
    g.ship_mut(picket).unwrap().arrived_this_turn = false;
    g.resolution_phase();
    assert_eq!(g.war.interceptions[0], 1, "into low orbit it is caught");
}

/// Ticket #335 (R5c): a Blockade shuts the ORBIT it is given in. A station starves under a Blockade
/// of its own ring and not under one in low orbit; a Colony on the ground starves under a Blockade
/// in low orbit by a rival holding Orbital Control outright, and not under one at a station's ring.
#[test]
fn a_blockade_shuts_the_orbit_it_is_given_in() {
    let mut g = game();
    calm(&mut g);
    let station = station_at(&mut g, Seat(1), BodyId::Mars);
    let slot = g.colony(station).unwrap().slot;
    let ground = colony(&mut g, Seat(1), BodyId::Mars, &[ModuleKind::Habitat], 4);
    // A warship of the same seat holds low orbit throughout, so Orbital Control never moves and the
    // only thing that changes below is the orbit the Blockade is given in.
    ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Mars, None, Stance::Hold);
    let blockader = ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Mars, None, Stance::Blockade);
    assert_eq!(g.orbital_control(BodyId::Mars), Some(Seat(0)), "in low orbit it holds Control outright");
    assert_eq!(g.starved_by(ground), Some(Seat(0)), "so the ground starves under a Blockade in low orbit");
    assert_eq!(g.starved_by(station), None, "the station above does not");
    assert!(!g.slot_blockaded_against(Seat(1), BodyId::Mars, slot), "and its ring is not shut");
    g.ship_mut(blockader).unwrap().slot = Some(slot);
    assert_eq!(g.orbital_control(BodyId::Mars), Some(Seat(0)), "Control stands: a warship still holds low orbit");
    assert_eq!(g.starved_by(station), Some(Seat(0)), "at the ring the station starves");
    assert!(g.slot_blockaded_against(Seat(1), BodyId::Mars, slot), "and the ring is shut");
    assert_eq!(g.starved_by(ground), None, "and the ground is free: the Blockade is not in its orbit, Control or no Control");
}

/// Ticket #335 (R5d): the specification's fourth refutation. A human can now give a Blockade: the
/// transit names the rival station's orbit, the Ship arrives there, and the stance is ordered. At
/// version 0.08.8 every transit the interface sent named no slot, so this path did not exist.
#[test]
fn a_human_flies_to_a_rival_station_and_blockades_it() {
    let mut g = game();
    calm(&mut g);
    at_window(&mut g);
    let station = station_at(&mut g, Seat(1), BodyId::Mars);
    let slot = g.colony(station).unwrap().slot;
    let ship = ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Earth, None, Stance::Hold);
    let leg = Order::Transit { ship, to: BodyId::Mars, slot: Some(slot) };
    assert!(g.check_order(Seat(0), &[], &leg).is_ok(), "the leg names the station's ring");
    g.commit_orders(Seat(0), std::slice::from_ref(&leg));
    for _ in 0..12 {
        if matches!(g.ship(ship).unwrap().at, ShipAt::Body(BodyId::Mars)) {
            break;
        }
        g.resolution_phase();
    }
    assert_eq!(g.ship_orbit(g.ship(ship).unwrap()), Orbit::Slot(slot), "it arrived at the ring it named");
    let order = Order::ShipStance { body: BodyId::Mars, stance: Stance::Blockade };
    assert!(g.check_order(Seat(0), &[], &order).is_ok(), "and the Blockade is given");
    g.commit_orders(Seat(0), std::slice::from_ref(&order));
    assert_eq!(g.war.blockades_ordered[0], 1, "counted for the sweep");
    assert_eq!(g.starved_by(station), Some(Seat(0)), "the station is starved by a path a human walked");
}

/// Ticket #335 (R6): the orbit you are in is the orbit you must hold. A ground Colony is bombarded
/// from LOW ORBIT by a Faction holding Orbital Control there outright; a station from that
/// station's own ring, with no rival warship and no rival working Battery standing in it.
#[test]
fn a_bombard_holds_the_orbit_it_is_given_from() {
    let mut g = game();
    calm(&mut g);
    let station = station_at(&mut g, Seat(1), BodyId::Mars);
    let slot = g.colony(station).unwrap().slot;
    g.colony_mut(station).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    let ground = colony(&mut g, Seat(1), BodyId::Mars, &[ModuleKind::Habitat], 4);
    let ship = ship_in(&mut g, Seat(0), UnitKind::Battleship, BodyId::Mars, None, Stance::Hold);
    let at_ground = Order::Bombard { ship, colony: ground };
    let at_station = Order::Bombard { ship, colony: station };
    // From low orbit: the ground under an outright Orbital Control, and not the station above.
    assert_eq!(g.orbital_control(BodyId::Mars), Some(Seat(0)));
    assert!(g.check_order(Seat(0), &[], &at_ground).is_ok(), "the ground from low orbit");
    let err = g.check_order(Seat(0), &[], &at_station).unwrap_err().0;
    assert!(err.contains("given from"), "the station is not reached from low orbit: {err}");
    // From the station's ring: the station, and not the ground below.
    g.ship_mut(ship).unwrap().slot = Some(slot);
    assert!(g.check_order(Seat(0), &[], &at_station).is_ok(), "the station from its own ring");
    let err = g.check_order(Seat(0), &[], &at_ground).unwrap_err().0;
    assert!(err.contains("given from"), "the ground is not reached from a ring: {err}");
    // A rival warship in that ring, or a rival working Battery covering it, refuses it.
    let rival = ship_in(&mut g, Seat(2), UnitKind::Frigate, BodyId::Mars, Some(slot), Stance::Hold);
    let err = g.check_order(Seat(0), &[], &at_station).unwrap_err().0;
    assert!(err.contains("a rival still stands"), "{err}");
    g.ships.retain(|s| s.id != rival);
    g.colony_mut(station).unwrap().modules.push(Module::new(ModuleKind::Battery));
    let err = g.check_order(Seat(0), &[], &at_station).unwrap_err().0;
    assert!(err.contains("a rival still stands"), "a working Battery in the ring is the shield: {err}");
    g.colony_mut(station).unwrap().modules.last_mut().unwrap().mothballed = true;
    assert!(g.check_order(Seat(0), &[], &at_station).is_ok(), "mothballed, it shields nothing");
    // And the strike lands, its record at the orbit it was given from.
    std::sync::Arc::make_mut(&mut g.tables).influence.destruction_chance = 1.0;
    let before = g.colony(station).unwrap().modules.len();
    g.commit_orders(Seat(0), std::slice::from_ref(&at_station));
    g.resolution_phase();
    assert_eq!(g.colony(station).unwrap().modules.len(), before - 1, "one Module burned");
    let at = Some(ReportPlace::Orbit(BodyId::Mars, Orbit::Slot(slot)));
    assert!(g.report.battles.iter().any(|b| b.at == at), "the record is the ring's: {:?}", g.report.battles.iter().map(|b| b.place.clone()).collect::<Vec<_>>());
}

// ------------------------------------------- 0.09.0 ticket #335 (R7): the computer seats want orbits

/// Ticket #335 (R7): **a transit names low orbit by default, and a rival station's ring where the
/// seat means to blockade or attack that station.** Until this ticket every warship leg anywhere
/// named the richest rival station's slot whatever the seat thought of its holder, which is why no
/// computer warship ever held low orbit -- the lane to the ground, and the one orbit Orbital
/// Control is held in. Meaning it is now Cold or worse toward the holder (`war_cause`), with no
/// working Battery of the holder's standing in that ring to lift the Blockade.
#[test]
fn a_warship_sent_to_blockade_a_rival_station_names_that_stations_ring() {
    let board = |score: i64| -> Game {
        let mut g = game();
        calm(&mut g);
        at_window(&mut g);
        let station = station_at(&mut g, Seat(1), BodyId::Mars);
        g.colony_mut(station).unwrap().modules.push(Module::new(ModuleKind::Core));
        g.relations.score[0][1] = score;
        // A Colony of seat 0's on Earth's own surface, so the ground at EARTH is what the seat
        // wants at home and the Frigate standing in Earth's low orbit is the garrison holding it.
        // Without this the reading under test is drowned by the right answer at the wrong Body:
        // every Faction opens with a station over Earth, so the hull would move up to a rival's
        // ring here rather than take the leg to Mars, which is what this test is about.
        colony(&mut g, Seat(0), BodyId::Earth, &[ModuleKind::Habitat], 4);
        ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Earth, None, Stance::Hold);
        g
    };
    let cause = game().tables.ai.thresholds.war_cause;
    // Cold or worse toward the holder: the leg names the ring the Blockade will be given in.
    let mut g = board(-8);
    assert!(g.relations_score(Seat(0), Seat(1)) <= cause, "the seat means it: {}", g.relations_score(Seat(0), Seat(1)));
    let ring = g.colonies.iter().find(|c| c.in_orbit && c.body == BodyId::Mars).unwrap().slot;
    let orders = g.ai_orders(Seat(0));
    assert!(
        orders.iter().any(|o| matches!(o, Order::Transit { to: BodyId::Mars, slot: Some(n), .. } if *n == ring)),
        "the leg names the rival station's ring: {orders:?}"
    );
    // Warm toward the holder, and it means nothing by being there: low orbit, the default.
    let mut g = board(0);
    assert!(g.relations_score(Seat(0), Seat(1)) > cause, "no cause: {}", g.relations_score(Seat(0), Seat(1)));
    let orders = g.ai_orders(Seat(0));
    let mars: Vec<&Order> = orders.iter().filter(|o| matches!(o, Order::Transit { to: BodyId::Mars, .. })).collect();
    assert!(!mars.is_empty(), "it still flies to Mars: {orders:?}");
    assert!(mars.iter().all(|o| matches!(o, Order::Transit { slot: None, .. })), "with no cause the leg names low orbit: {mars:?}");
}

/// Ticket #335 (R7): **a seat that wants the ground wants Orbital Control of low orbit**, so its
/// warship's leg names low orbit even where a rival keeps a station it has every cause against.
/// Orbital Control is low orbit's since this ticket, and the ground waits on Control; a hull parked
/// at a ring three orbits up shuts nothing on the surface.
#[test]
fn a_seat_that_wants_the_ground_sends_its_warship_to_low_orbit() {
    let mut g = game();
    calm(&mut g);
    at_window(&mut g);
    let station = station_at(&mut g, Seat(1), BodyId::Mars);
    g.colony_mut(station).unwrap().modules.push(Module::new(ModuleKind::Core));
    g.relations.score[0][1] = -8;
    // As in the test above: a Colony of its own on Earth's surface, so the Frigate in Earth's low
    // orbit is the garrison of the Body it is standing at and the leg to Mars is what it weighs.
    colony(&mut g, Seat(0), BodyId::Earth, &[ModuleKind::Habitat], 4);
    ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Earth, None, Stance::Hold);
    let ring = g.colony(station).unwrap().slot;
    // With nothing of its own on the Martian surface, the ring is what the leg names.
    assert!(!g.ai_wants_the_ground(Seat(0), BodyId::Mars), "nothing on the ground yet");
    let orders = g.ai_orders(Seat(0));
    assert!(orders.iter().any(|o| matches!(o, Order::Transit { to: BodyId::Mars, slot: Some(n), .. } if *n == ring)), "{orders:?}");
    // A Colony of its own on the ground there, and the surface is the thing: low orbit.
    colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Habitat], 4);
    assert!(g.ai_wants_the_ground(Seat(0), BodyId::Mars), "a Colony of its own on the surface to keep");
    let orders = g.ai_orders(Seat(0));
    let mars: Vec<&Order> = orders.iter().filter(|o| matches!(o, Order::Transit { to: BodyId::Mars, .. })).collect();
    assert!(!mars.is_empty(), "it still flies to Mars: {orders:?}");
    assert!(mars.iter().all(|o| matches!(o, Order::Transit { slot: None, .. })), "the ground wants Control of low orbit: {mars:?}");
}

/// Ticket #335 (R7): **it changes orbit rather than flying away when what it wants is at the same
/// Body.** A warship already at a Body, sitting in low orbit with a rival station above it that
/// its seat means to shut, moves up to that ring -- where the Blockade it is for shuts something --
/// instead of taking the leg home. The engine lane built this for a Ship's own errands (a tank, a
/// load, the ground); the blockade appetite is the want added here, and the measured 0.08.8 board
/// had no candidate of the kind at all.
#[test]
fn a_warship_changes_orbit_to_the_ring_it_means_to_shut_rather_than_flying_away() {
    let mut g = game();
    calm(&mut g);
    let station = station_at(&mut g, Seat(1), BodyId::Mars);
    g.colony_mut(station).unwrap().modules.push(Module::new(ModuleKind::Core));
    let ring = g.colony(station).unwrap().slot;
    g.relations.score[0][1] = -8;
    let ship = ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Mars, None, Stance::Hold);
    assert!(!g.ai_wants_the_ground(Seat(0), BodyId::Mars), "nothing of its own on the surface");
    let orders = g.ai_orders(Seat(0));
    assert!(
        orders.iter().any(|o| matches!(o, Order::ChangeOrbit { ship: id, slot: Some(n) } if *id == ship && *n == ring)),
        "it moves up to the ring it means to shut: {orders:?}"
    );
    assert!(!orders.iter().any(|o| matches!(o, Order::Transit { ship: id, .. } if *id == ship)), "and does not fly away instead: {orders:?}");
    // With its own Colony on the ground below, the hull that holds low orbit stays in it: Orbital
    // Control is low orbit's, and the garrison is what the ground waits on.
    colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Habitat], 4);
    let orders = g.ai_orders(Seat(0));
    assert!(!orders.iter().any(|o| matches!(o, Order::ChangeOrbit { ship: id, .. } if *id == ship)), "it holds the lane: {orders:?}");
}

// ================================================================ #336 (version 0.09.0): twice the Influence off Earth

/// Ticket #336 (version 0.09.0): **a place off Earth costs 40 plus 20 a Colonist**, where a ground
/// Colony cost 10 a Colonist and a station 20 plus 10 a Colonist. The station base REPLACES the
/// Colony base rather than sitting on top of it, so a station and a ground Colony of the same crew
/// are worth the same figure -- which they have not been since stations existed, and which the
/// designer chose deliberately over keeping the distinction.
#[test]
fn a_place_off_earth_costs_forty_plus_twenty_a_colonist() {
    let mut g = game();
    let ground = colony(&mut g, Seat(1), BodyId::Mars, &[ModuleKind::Habitat], 0);
    let station = station_at(&mut g, Seat(1), BodyId::Mars);
    for people in [0u32, 1, 2, 4, 8] {
        g.colony_mut(ground).unwrap().colonists = people;
        g.colony_mut(station).unwrap().colonists = people;
        let want = 40 + 20 * people as i64;
        // The old figures: a ground Colony 10 a Colonist, a station 20 beside it.
        assert_eq!(g.influence_threshold(Place::Colony(ground)), want, "a ground Colony of {people} Colonists, where it was {}", 10 * people as i64);
        assert_eq!(g.influence_threshold(Place::Colony(station)), want, "a station of {people} Colonists, where it was {}", 20 + 10 * people as i64);
    }
}

/// Ticket #336 (version 0.09.0): **an empty place is no longer free.** A station was built with no
/// crew and was worth its station base alone; a ground Colony with nobody moved in was worth
/// nothing at all, and a single point of Standing took it. Both now carry the Colony base.
#[test]
fn an_empty_place_off_earth_is_no_longer_free_to_take() {
    let mut g = game();
    let c = colony(&mut g, Seat(1), BodyId::Moon, &[ModuleKind::Barracks], 0);
    assert_eq!(g.influence_threshold(Place::Colony(c)), 40, "the Colony base under every place off Earth, where an empty Colony was worth nothing");
    // A rival at 39 does not take it; at 40 it does. Nothing is held back by the challenge margin
    // here: seat 1's own Standing on the place is nought, so the threshold is the whole price.
    g.seats[0].influence.insert(Place::Colony(c), 39);
    g.seats[0].influenced_this_turn.push(Place::Colony(c));
    g.resolution_phase();
    assert_eq!(g.colony(c).unwrap().control, Control::Controlled(Seat(1)), "39 is under the base");
    g.seats[0].influence.insert(Place::Colony(c), 40);
    g.seats[0].influenced_this_turn.push(Place::Colony(c));
    g.resolution_phase();
    assert_eq!(g.colony(c).unwrap().control, Control::Controlled(Seat(0)), "40 takes it");
    // Ticket #336: and the take is counted at the transfer by the kind of place, where the sweep
    // scraped the log for every take together and could not tell a Region from a Colony.
    assert_eq!(g.war.takes_by_influence_colonies[0], 1, "one ground Colony taken by Influence");
    assert_eq!(g.war.takes_by_influence_stations[0], 0);
    assert_eq!(g.war.takes_by_influence_states[0], 0);
    // A station with nobody aboard carries the station base, and its take is counted as a station's.
    let st = station_at(&mut g, Seat(1), BodyId::Moon);
    assert_eq!(g.influence_threshold(Place::Colony(st)), 40, "the station base on an empty station");
    g.seats[0].influence.insert(Place::Colony(st), 40);
    g.seats[0].influenced_this_turn.push(Place::Colony(st));
    g.resolution_phase();
    assert_eq!(g.colony(st).unwrap().control, Control::Controlled(Seat(0)), "40 takes the station too");
    assert_eq!(g.war.takes_by_influence_stations[0], 1, "one station taken by Influence");
    assert_eq!(g.war.takes_by_influence_colonies[0], 1, "and the ground Colony is not counted twice");
}

/// Ticket #336 (version 0.09.0): **the challenge margin is untouched.** It is shared with Earth and
/// the designer kept it at 20, knowing what it means: on a settled place the holder's Standing plus
/// 20 is still the binding figure, so a place whose holder stands high costs exactly what it did.
#[test]
fn the_challenge_margin_off_earth_is_what_it_was() {
    let mut g = game();
    assert_eq!(g.tables.influence.challenge_margin, 20);
    let c = colony(&mut g, Seat(1), BodyId::Moon, &[ModuleKind::Habitat], 2);
    g.seats[1].influence.insert(Place::Colony(c), 300);
    // 300 + 20 against a threshold of 80: the margin binds, and 320 is what it was before the
    // thresholds doubled.
    assert_eq!(g.influence_needed_for(Seat(0), Place::Colony(c)), 320, "the holder's Standing plus the margin, as before");
    // Both seats spend here, so neither Standing decays and the arithmetic is the one written above.
    g.seats[0].influence.insert(Place::Colony(c), 319);
    g.seats[0].influenced_this_turn.push(Place::Colony(c));
    g.seats[1].influenced_this_turn.push(Place::Colony(c));
    g.resolution_phase();
    assert_eq!(g.colony(c).unwrap().control, Control::Controlled(Seat(1)), "319 is one short of the margin");
    g.seats[0].influence.insert(Place::Colony(c), 320);
    g.seats[0].influenced_this_turn.push(Place::Colony(c));
    g.seats[1].influenced_this_turn.push(Place::Colony(c));
    g.resolution_phase();
    assert_eq!(g.colony(c).unwrap().control, Control::Controlled(Seat(0)), "320 takes it, the figure it always was");
}

/// Ticket #336 (version 0.09.0): **the computer seats weigh a rival Colony by the price they would
/// pay**, where they ranked by fewest Colonists and never read the threshold at all. The board is
/// built so the two rules disagree: the emptier Colony is held by a seat standing high on it and
/// costs 220, the fuller one costs its threshold of 100. The old rule wanted the emptier; the new
/// one wants the cheaper.
#[test]
fn the_computer_wants_the_cheaper_rival_colony_not_the_emptier() {
    let mut g = game();
    calm(&mut g);
    // Every Region is seat 0's, so the Influence targets are the Colonies alone and the pick is
    // between them; and nothing is affordable but Influence, so the turn's one step is spent here.
    for sid in StateId::ALL {
        g.take_control(sid, Seat(0));
    }
    // And no rival stands on any of them, so the seat has nothing on Earth to hold either: every
    // Faction opens with a Standing on its own start state (ticket #75), which is a rival's here.
    for s in Seat::ALL {
        g.seat_mut(s).influence.retain(|p, _| !matches!(p, Place::State(_)));
    }
    // The three starting stations over Earth are rivals' Colonies too, and two Colonists apiece
    // makes them the cheapest places on the board; they go to seat 0 so the pick is the planted two.
    for c in g.colonies.iter_mut() {
        c.control = Control::Controlled(Seat(0));
    }
    let empty = colony(&mut g, Seat(1), BodyId::Moon, &[ModuleKind::Habitat], 1);
    let fuller = colony(&mut g, Seat(1), BodyId::Mars, &[ModuleKind::Habitat], 3);
    g.seats[1].influence.insert(Place::Colony(empty), 200);
    g.seats[0].stockpile.materials = 0;
    g.seats[0].stockpile.energy = 0;
    g.seats[0].stockpile.ducats = 0;
    g.seats[0].allotment = 100;
    assert_eq!(g.influence_needed_for(Seat(0), Place::Colony(empty)), 220, "the emptier place, held by a seat standing high on it");
    assert_eq!(g.influence_needed_for(Seat(0), Place::Colony(fuller)), 100, "the fuller place, at its threshold");
    let orders = g.ai_orders(Seat(0));
    // The Allotment covers twenty steps and the top-weighted target takes every one of them, so the
    // whole spend lands on whichever Colony the seat wants more.
    let spent: Vec<Place> = orders.iter().filter_map(|o| match o { Order::Influence { target, .. } => Some(*target), _ => None }).collect();
    assert!(spent.iter().all(|t| *t == Place::Colony(fuller)), "every step on the cheaper place, where the old rule put them all on the emptier: {spent:?}");
    assert!(!spent.is_empty(), "it spends its Allotment somewhere: {orders:?}");
}


// ---------------------------------------------------------------- Ticket #337: cards that ask

/// Ticket #337 (version 0.09.0) R1: **the deck is forty cards and every one of them is distinct.**
/// Fourteen kinds carried eighteen extra copies between them; every extra copy is cut and eighteen
/// choice cards take their places. Before this the same Solar Storm could be drawn three times in a
/// game; now no card is ever seen twice, and eighteen of the forty ask the table a question.
#[test]
fn the_deck_is_forty_distinct_cards_and_eighteen_of_them_ask_a_question() {
    let g = game();
    let t = &g.tables.events;
    assert_eq!(t.event.len(), 40, "forty kinds");
    let duplicated: Vec<String> = t.event.iter().filter(|e| e.copies != 1).map(|e| format!("{} x{}", e.name, e.copies)).collect();
    assert!(duplicated.is_empty(), "no kind is dealt twice: {duplicated:?}");
    assert_eq!(t.event.iter().map(|e| e.copies).sum::<u32>(), 40, "forty cards in all");
    assert_eq!(t.event.iter().filter(|e| e.asks()).count(), 18, "eighteen of them ask a question");
    // And the deck as dealt, once the off-Earth cards have joined, holds each of them once.
    let mut g = game();
    g.turn = t.off_earth_join_turn;
    g.question_phase();
    g.deck.cards.append(&mut g.deck.drawn);
    let mut ids: Vec<EventId> = g.deck.cards.iter().map(|c| { let Card::Event(e) = *c; e }).collect();
    assert_eq!(ids.len(), 40, "forty cards dealt");
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), 40, "and no card appears twice in the deck");
}

/// Ticket #337 R2: **the card is drawn and asked at the head of the turn, before orders**, and a
/// seat neither side of it can reach is NOT asked. The Hard Winter offers relief or raises Unrest
/// in every Region the seat holds; a Faction holding no Region has nothing to decide, and the spec
/// is wrong if such a seat is asked it.
#[test]
fn a_card_is_asked_before_orders_and_a_seat_it_cannot_reach_is_not_asked() {
    let mut g = game();
    // Seat 2 holds nothing at all, so the refusing side has nowhere to land on it.
    for sid in g.controlled_states(Seat(2)) {
        g.state_mut(sid).control = Control::Neutral;
    }
    assert!(g.controlled_states(Seat(2)).is_empty(), "seat 2 holds no Region");
    // The Question phase runs at the head of the turn, BEFORE any order is given.
    ask_the_card(&mut g, EventId::TheHardWinter);
    let q = g.pending_question().expect("the Hard Winter is asking");
    assert_eq!(q.card, EventId::TheHardWinter);
    assert_eq!(q.answer_of(Seat(0)), None, "a seat holding Regions is asked, and owes an answer");
    assert_eq!(q.answer_of(Seat(2)), Some(CardAnswer::NothingToDecide), "a seat with no Region held is NOT asked the Hard Winter");
    assert_eq!(q.unanswered(), Some(Seat(0)), "seat 0 is the first that still owes one");
    // And the Event phase has no Event to announce: nothing landed on the table, because what
    // happened happened to each seat by its own answer.
    g.event_phase();
    assert!(g.last_event.is_none(), "a choice card is not an Event that befell the table");
}

/// Ticket #337: **a seat that cannot pay a card's offer is asked anyway**, at the designer's word
/// when the first build skipped it: *"b"* -- the offer is greyed and refusing is its only move, so
/// a struggling Faction still feels the card. Being unable to pay is not the same as having nothing
/// to decide: the Hard Winter's refusing side raises Unrest in every Region the seat holds, and a
/// seat holding Regions but not the 40 Ducats must take that. Measured before the change: 34% of
/// seat-card pairs over eighty games were never asked at all, most of them for want of the price.
#[test]
fn a_seat_that_cannot_pay_is_asked_anyway_and_may_only_refuse() {
    let mut g = game();
    let broke = Seat(1);
    assert!(!g.controlled_states(broke).is_empty(), "the seat holds Regions, so the refusing side reaches it");
    g.seat_mut(broke).stockpile.ducats = 0;
    // A seat starts the game with far less than the relief costs, so the solvent one is given it.
    g.seat_mut(Seat(0)).stockpile.ducats = 100;
    ask_the_card(&mut g, EventId::TheHardWinter);
    let q = g.pending_question().expect("the Hard Winter is asking");
    assert_eq!(q.answer_of(broke), None, "a seat that cannot pay is still asked and still owes an answer");
    assert!(!g.may_take_card(broke), "but it cannot take what it cannot pay for");
    assert!(g.may_take_card(Seat(0)), "a solvent seat may take it");
    assert!(g.answer_card(broke, true).is_err(), "taking is refused at the door, not silently ignored");
    assert!(g.answer_card(broke, false).is_ok(), "refusing is its only move, and it is open");
}


/// Ticket #337 R2: **End Turn is refused while a human seat owes this turn's card an answer**, in
/// the same shape and through the same door as the Tech pick of #105, naming the card. The spec is
/// wrong if End Turn can be pressed with a question pending and unanswered.
#[test]
fn the_turn_will_not_end_while_a_human_seat_owes_this_turns_card_an_answer() {
    let mut g = game();
    pick_a_tech(&mut g); // so the only refusal left is the card's
    ask_the_card(&mut g, EventId::SalvageRights);
    let before = g.turn;
    let why = g.end_turn_refusal().expect("a card is asking, so the turn is refused");
    assert!(why.contains("Salvage Rights"), "and the refusal names the card: {why}");
    assert_eq!(g.end_turn(std::array::from_fn(|_| Vec::new())), Err(why), "the turn refuses with the same words");
    assert_eq!(g.turn, before, "and nothing advanced");
    // Answering clears it, and a seat answers once.
    g.answer_card(Seat(0), true).expect("the offer can be taken");
    assert!(g.answer_card(Seat(0), false).is_err(), "a seat that has answered cannot answer again");
    assert!(g.end_turn_refusal().is_none(), "answered, the turn may end");
    assert!(g.end_turn(std::array::from_fn(|_| Vec::new())).is_ok());
    assert_eq!(g.turn, before + 1);
    // A computer seat never holds the turn: it answers when its orders are computed.
    let mut s = Game::spectate(tables(), 7);
    s.start();
    assert!(s.end_turn_refusal().is_none(), "every seat is an AI here");
}

/// Ticket #337 R3: **the two sides are lists of effects composed in data, and every figure is a
/// field of `events.toml`.** The Hard Winter is read out of the table and both answers are played
/// on identical boards: the spec is wrong if two seats answering the same card differently produce
/// the same board.
#[test]
fn the_two_sides_of_a_card_are_composed_in_data_and_make_different_boards() {
    let mut taken = game();
    let mut refused = game();
    let card = taken.tables.event(EventId::TheHardWinter).choice.clone().expect("the Hard Winter asks a question");
    // Both sides are lists of effects, each carrying its own figures. Nothing below is a literal:
    // the relief and the Unrest are read out of the table and the board is checked against them.
    // Relief is PAID in this game, as the Relief order on a Region's card is paid, so the take
    // side's figure is negative: the purse falls by it.
    let relief = match card.take_does.first().expect("the take side is a list of effects") {
        CardEffect::Resources { ducats, .. } => *ducats,
        e => panic!("the take side of the Hard Winter is relief in Ducats: {e:?}"),
    };
    let rise = match card.refuse_does.first().expect("the refuse side is a list of effects") {
        CardEffect::UnrestAllHeld { unrest } => *unrest,
        e => panic!("the refuse side of the Hard Winter is Unrest in every held Region: {e:?}"),
    };
    assert!(relief < 0 && rise > 0.0, "the card carries its own figures: {relief} Ducats paid, {rise} Unrest");
    // A seat begins the game with far less than the relief costs, so it is given enough to choose.
    for g in [&mut taken, &mut refused] {
        g.seat_mut(Seat(0)).stockpile.ducats = 100;
    }
    let purse = taken.seat(Seat(0)).stockpile.ducats;
    let held = taken.controlled_states(Seat(0));
    let quiet: Vec<f64> = held.iter().map(|s| taken.state(*s).unrest).collect();
    for g in [&mut taken, &mut refused] {
        ask_the_card(g, EventId::TheHardWinter);
    }
    taken.answer_card(Seat(0), true).unwrap();
    refused.answer_card(Seat(0), false).unwrap();
    taken.apply_card_answers();
    refused.apply_card_answers();
    assert_eq!(taken.seat(Seat(0)).stockpile.ducats, purse + relief, "taking it pays out the relief the card names");
    assert_eq!(refused.seat(Seat(0)).stockpile.ducats, purse, "refusing it pays nothing");
    for (i, sid) in held.iter().enumerate() {
        assert_eq!(taken.state(*sid).unrest, quiet[i], "{:?}: taking it moves no Unrest", sid);
        assert_eq!(refused.state(*sid).unrest, quiet[i] + rise, "{:?}: refusing it raises Unrest by the card's figure", sid);
    }
}

/// Ticket #337 R4: **a computer seat answers by the rule the card carries**, read off its own
/// board, with the rule's figure in `events.toml` beside the card rather than in `ai.toml`. The
/// Emergency Shutdown is shut by a Faction already answerable for a lot of ppm and run hot by a
/// clean one, so a rival's answer tells the player something true about it.
#[test]
fn a_computer_seat_answers_its_card_by_the_rule_the_card_carries() {
    let mut g = game();
    let bar = match g.tables.event(EventId::EmergencyShutdown).choice.as_ref().expect("it asks").take_when {
        CardRule::BlameAtLeast { blame } => blame,
        ref r => panic!("the Emergency Shutdown answers to a Blame bar: {r:?}"),
    };
    // Both seats have a Power Plant, so the card reaches both of them.
    for seat in [Seat(1), Seat(2)] {
        let sid = g.controlled_states(seat)[0];
        g.state_mut(sid).facilities.push(facility(FacilityKind::PowerPlant));
    }
    g.seats[1].blame_emitted = bar + 1.0;
    g.seats[2].blame_emitted = 0.0;
    ask_the_card(&mut g, EventId::EmergencyShutdown);
    g.ai_answer_card(Seat(1));
    g.ai_answer_card(Seat(2));
    let q = g.pending_question().expect("the card is asking");
    assert_eq!(q.answer_of(Seat(1)), Some(CardAnswer::Taken), "over the bar at {} ppm, it shuts the plants", bar + 1.0);
    assert_eq!(q.answer_of(Seat(2)), Some(CardAnswer::Refused), "under the bar, it runs them hot");
    // And the answer is the seat's own: the two boards differ, so the rule read the board.
    assert_ne!(q.answer_of(Seat(1)), q.answer_of(Seat(2)), "the rule is a predicate over the seat's board, not a constant");
}

/// Ticket #337 R5: **the Report names each seat's answer, and says *nothing to decide* for a seat
/// that was not asked**; and the game counts answers by seat for the sweep.
#[test]
fn the_report_names_every_seats_answer_and_the_game_counts_them() {
    let mut g = game();
    for sid in g.controlled_states(Seat(3)) {
        g.state_mut(sid).control = Control::Neutral;
    }
    g.seat_mut(Seat(0)).stockpile.ducats = 100;
    ask_the_card(&mut g, EventId::TheHardWinter);
    g.answer_card(Seat(0), true).unwrap();
    g.answer_card(Seat(1), false).unwrap();
    g.answer_card(Seat(2), false).unwrap();
    assert!(g.answer_card(Seat(3), true).is_err(), "a seat that was not asked cannot answer");
    g.report = Report::default();
    g.apply_card_answers();
    let lines: Vec<String> = g.report.lines.iter().map(|l| l.text.clone()).collect();
    let said = |who: &str, what: &str| lines.iter().any(|l| l.contains("Hard Winter") && l.contains(who) && l.contains(what));
    assert!(said(&g.seat_name(Seat(0)), "took it"), "the Report names the seat that took it: {lines:?}");
    assert!(said(&g.seat_name(Seat(1)), "refused it"), "and the seat that refused: {lines:?}");
    assert!(said(&g.seat_name(Seat(3)), "nothing to decide"), "and says so for the seat that was not asked: {lines:?}");
    assert_eq!(g.choice_taken[0], 1, "one taken by seat 0");
    assert_eq!(g.choice_refused[1], 1, "one refused by seat 1");
    assert_eq!(g.choice_not_asked[3], 1, "one never asked of seat 3");
    assert_eq!(g.choice_taken[3], 0, "and nothing counted as an answer for it");
}

/// Ticket #337 R3, the effect the whole phase order exists for: **a seat that grounds its fleet
/// holds every transit of its own this turn** -- the Solar Storm's shape, for one seat. The card is
/// answered before orders are given, so the answer binds orders the player gives knowing it.
#[test]
fn grounding_the_fleet_holds_that_seats_transits_and_nobody_elses() {
    let mut g = game();
    let mk = |g: &mut Game, seat: Seat| {
        let id = ShipId(g.fresh_id());
        g.ships.push(Ship {
            name: String::new(), id, kind: UnitKind::Frigate, seat, damage: 0,
            at: ShipAt::Transit { from: BodyId::Earth, to: BodyId::Moon, turns_left: 1 },
            colonists: 0, warhead: false, colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false,
            arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None,
        });
        id
    };
    let mine = mk(&mut g, Seat(0));
    let theirs = mk(&mut g, Seat(1));
    ask_the_card(&mut g, EventId::GroundedFleet);
    g.answer_card(Seat(0), true).expect("seat 0 grounds its fleet");
    g.answer_card(Seat(1), false).expect("seat 1 flies on");
    assert!(g.card_holds_ships(Seat(0)), "seat 0 answered with the holding side");
    assert!(!g.card_holds_ships(Seat(1)), "seat 1 did not");
    g.resolution_phase();
    assert!(matches!(g.ship(mine).unwrap().at, ShipAt::Transit { .. }), "the grounded seat's Ship did not arrive");
    assert!(matches!(g.ship(theirs).unwrap().at, ShipAt::Body(BodyId::Moon)), "the seat that flew on arrived");
    assert_eq!(g.ship(theirs).unwrap().damage, 1, "and took the damage the refusing side carries");
    assert_eq!(g.ship(mine).unwrap().damage, 0, "while the grounded fleet took none");
}


/// Ticket #337 R3, and the error case the spec names: **a `trade_price` effect overrides the band
/// for the turns the card names, and the band resumes after.** The Cheap Ore Offer refused puts
/// Materials at 1, which is outside the band on purpose -- that is the point of the card -- and the
/// override is recorded rather than written into the price, so nothing of the band is lost.
///
/// The turns it names are the turns of ORDERS that follow it: the answer lands at the Resolution of
/// the turn it was given in, by which time that turn's trading is done, so a countdown spent at the
/// settle would burn one of its turns before a single order had been priced at it.
#[test]
fn a_card_that_moves_a_price_overrides_the_band_for_the_turns_it_names() {
    let mut g = game();
    let refuse = g.tables.event(EventId::CheapOreOffer).choice.clone().expect("it asks").refuse_does;
    let (to, turns) = match refuse.first().expect("the refusing side is a list of effects") {
        CardEffect::TradePrice { to, turns, .. } => (to.expect("the Cheap Ore Offer SETS the price"), *turns),
        e => panic!("the refusing side of the Cheap Ore Offer is a price: {e:?}"),
    };
    let row = Game::market_row(Resource::Materials).expect("Materials are traded");
    let band = g.market_price_at(row);
    assert_ne!(band, to, "the card's price is outside the band the market opens at");
    // The offer is 30 Materials for 20 Ducats, and a seat that cannot pay the 20 is not asked at
    // all -- a price it cannot meet is an effect that cannot land. So it is given the money first.
    g.seats[0].stockpile.ducats = 100;
    ask_the_card(&mut g, EventId::CheapOreOffer);
    let answered_on = g.turn;
    g.answer_card(Seat(0), false).expect("the offer can be refused");
    g.apply_card_answers();
    g.settle_market(); // the end of the turn it was answered in
    for t in answered_on + 1..answered_on + 1 + turns {
        g.turn = t;
        assert_eq!(g.market_price_at(row), to, "turn {t}: the price the card named stands");
        g.settle_market();
    }
    g.turn = answered_on + 1 + turns;
    assert_eq!(g.market_price_at(row), band, "and the band is the price again once the card is spent");
}


/// Ticket #337: **an ordinary card is held in silence and behaves exactly as it always did.** The
/// DRAW moved to the head of the turn so a choice card could be asked before orders; the
/// announcement, the target and the effect did not move an inch. The spec is wrong if an ordinary
/// card's behaviour changes, so this pins the whole path for one of the 22.
#[test]
fn an_ordinary_card_is_held_in_silence_and_lands_where_it_always_did() {
    let mut g = game();
    let before_lines = g.report.lines.len();
    ask_the_card(&mut g, EventId::Unrest);
    assert!(g.pending_question().is_none(), "an ordinary card asks nobody anything");
    let refusal = g.end_turn_refusal().unwrap_or_default();
    assert!(!refusal.contains("Unrest"), "and it never holds the turn: {refusal}");
    assert_eq!(g.report.lines.len(), before_lines, "nothing is said at the head of the turn: the card is HELD");
    assert!(g.last_event.is_none(), "and nothing has landed yet");
    // The Event phase takes it up: the target is chosen and the card announced, where it always was.
    g.event_phase();
    let e = g.last_event.clone().expect("the Event phase announces the card the Question phase held");
    assert_eq!(e.card, Card::Event(EventId::Unrest));
    let EventTarget::State(sid) = e.target else { panic!("the Unrest card lands on a Region: {:?}", e.target) };
    assert!(g.report.lines.len() > before_lines, "and the Report carries it, as it always did");
    // And Resolution (h) applies it by the card's own figure in `events.toml`, unchanged.
    let quiet = g.state(sid).unrest;
    g.apply_event_now();
    assert_eq!(g.state(sid).unrest, quiet + g.tables.events.unrest_card_unrest, "the card's own figure, unchanged");
}


/// Ticket #339 (version 0.09.0): **a refusal names the rule, not the price.** `check_order_inner`
/// tested affordability before it tested anything else, so an order that was both forbidden and
/// unaffordable was told what it cost and never told it was forbidden -- the fog carried on four
/// wayfinder maps since #230. The legality runs first now and the price last, so the sentence a
/// player reads is the one that still binds when the Materials are found.
#[test]
fn a_refusal_names_the_rule_before_the_price() {
    let mut g = game();
    g.seats[0].stockpile = Stockpile { materials: 0, fuel: 0, energy: 0, ducats: 0 };
    // Sub-Saharan Africa is nobody's, so a Factory there is forbidden -- and unaffordable as well.
    let forbidden = Order::BuildFacility { state: StateId::SubSaharanAfrica, kind: FacilityKind::Factory };
    assert!(g.order_cost(Seat(0), &forbidden).materials > 0, "the order has a price it could be told instead");
    let why = g.check_order(Seat(0), &[], &forbidden).expect_err("forbidden and unaffordable at once");
    assert!(why.0.contains("do not direct"), "the refusal names the rule, not the price: {}", why.0);
    // And an order that is only unaffordable is still told its price, which is the useful sentence there.
    let priced = Order::BuildFacility { state: StateId::EastAsia, kind: FacilityKind::Factory };
    let why = g.check_order(Seat(0), &[], &priced).expect_err("legal but unaffordable");
    assert!(why.0.contains("Materials"), "a legal order short of the money is told its price: {}", why.0);
}


/// Ticket #339 (version 0.09.0): **the march lines name the Army.** Armies have carried names since
/// 0.08.4 and the Report still said "Custodians Army moved from China to Russia" (#270, declared to
/// the playtesters as a rough edge). The march, the landing, the rival's march clause and the
/// rival's loading clause all read the name now, through `report.toml`'s own templates.
#[test]
fn the_reports_march_lines_name_the_army() {
    let mut g = game();
    let (home, target) = (StateId::EastAsia, StateId::Russia);
    let id = g.armies.iter().find(|a| a.standing && a.home == ArmyHome::State(home)).map(|a| a.id).expect("China's own Army");
    let name = g.army_name(g.army(id).expect("it stands")).clone();
    assert!(name.contains("Army") && name != "the Army", "it has a name of its own: {name}");
    // The rival's paragraph names it for a march and for a loading.
    let deed = g.rival_deed(Seat(0), &Order::MoveArmy { army: id, to: target }).expect("a march is a visible deed");
    assert!(deed.contains(&name), "the rival's march clause names the Army: {deed}");
    // And the Report's own march line names it.
    g.armies.iter_mut().find(|a| a.id == id).unwrap().move_to = Some(target);
    g.resolution_phase();
    let march: Vec<String> = g.report.lines.iter().filter(|l| l.text.contains("Russia") && l.text.contains("China")).map(|l| l.text.clone()).collect();
    assert!(!march.is_empty(), "the march is in the Report at all");
    assert!(march.iter().any(|t| t.contains(&name)), "the march line names the Army ({name}): {march:?}");
}


/// Ticket #339 (version 0.09.0): **the odds are the whole Battle's.** The attack button and the
/// Battle Report carried `first_round_odds` -- the aggressor's share of the strength in the first
/// exchange -- and `PLAYTEST.txt` asked the testers by name whether they knew what it meant. The
/// honest figure is the chance of holding the field when the Battle is over, and it is measured:
/// a thousand copies of the fight from the FIGURE's own seed, so it is the same every time it is
/// asked and asking it never spends one of the game's dice.
#[test]
fn the_attack_odds_are_the_whole_battles_and_never_touch_the_games_dice() {
    let (home, target) = (StateId::EastAsia, StateId::Russia);
    let place = Place::State(target);
    // Russia held by the Prospectors, with two Armies raised there beside its own, so an attacker
    // must destroy THREE before it holds the field. This is where the first exchange's share lies
    // hardest: it reads one pooled strength and says nothing about how many units have to die.
    let board = || {
        let mut g = game();
        g.take_control(target, Seat(1));
        g.raise_army(place, false);
        g.raise_army(place, false);
        g
    };
    let (mut a, b, mut untouched) = (board(), board(), board());
    let id = a.armies.iter().find(|x| x.standing && x.home == ArmyHome::State(home)).map(|x| x.id).expect("China's own Army");
    let odds = a.ground_battle_odds(place, Seat(0), &[id]);
    assert!(odds > 0.0 && odds < 1.0, "a defended Region is neither a certainty nor hopeless: {odds}");
    // The same board asked twice is the same figure: a seeded game is unchanged by looking at it.
    assert_eq!(odds, b.ground_battle_odds(place, Seat(0), &[id]), "two identical games read the same odds");
    assert_eq!(odds, a.ground_battle_odds(place, Seat(0), &[id]), "and asking twice does not move it");
    // And the game's own dice are where they were: the figure has a seed of its own.
    let (spent, fresh) = {
        use rand::Rng;
        (a.rng.random::<u64>(), untouched.rng.random::<u64>())
    };
    assert_eq!(spent, fresh, "reading the odds spent none of the game's dice");
    // It is a different number from the first exchange's share, which is what made it dishonest.
    let attacker = a.army_strength(a.army(id).expect("it stands"));
    let defence: i64 = a.defenders_at(place, Seat(0)).iter().filter_map(|d| a.army(*d)).map(|d| a.army_defended_strength(d)).sum();
    let first = combat::first_round_odds(attacker, defence);
    assert!(
        (odds - first).abs() > 0.15,
        "the whole Battle is not its first round: whole {odds:.3} against first-round {first:.3} (strength {attacker} against {defence}, {} defenders)",
        a.defenders_at(place, Seat(0)).len()
    );
    // The orbit has the same door, and it too is the same figure every time it is asked.
    let orbit = Orbit::Low;
    assert_eq!(a.orbit_battle_odds(BodyId::Moon, orbit, Seat(0)), 0.0, "no Ships of yours there, no odds");
    let mine = a_colony_ship(&mut a, Seat(0), BodyId::Moon);
    a.ships.iter_mut().find(|s| s.id == mine).unwrap().kind = UnitKind::Frigate;
    assert_eq!(a.orbit_battle_odds(BodyId::Moon, orbit, Seat(0)), 1.0, "an empty orbit is held by arriving in it");
    let theirs = a_colony_ship(&mut a, Seat(1), BodyId::Moon);
    a.ships.iter_mut().find(|s| s.id == theirs).unwrap().kind = UnitKind::Battleship;
    let in_orbit = a.orbit_battle_odds(BodyId::Moon, orbit, Seat(0));
    assert!(in_orbit > 0.0 && in_orbit < 1.0, "a Frigate against a Battleship is neither: {in_orbit}");
    assert_eq!(in_orbit, a.orbit_battle_odds(BodyId::Moon, orbit, Seat(0)), "asked twice, the same figure");
}


/// Ticket #339 (version 0.09.0): **the Relay and the Embassy are eyes.** The designer: *"its holder
/// reads a rival's building-by-building income at that Body"*, which the Faction window withholds.
/// One eye a Body -- a working Relay off Earth, a working Embassy on it -- and what it reads is the
/// Income phase's own figures, read for the RIVAL, so the Faction multipliers in them are theirs.
/// A seat without one reads nothing, which is the point of paying for one.
#[test]
fn a_relay_or_an_embassy_reads_a_rivals_income_where_a_seat_without_one_reads_nothing() {
    let mut g = game();
    // On Earth: the Prospectors hold Russia and run a Factory there.
    let theirs = StateId::Russia;
    g.take_control(theirs, Seat(1));
    g.state_mut(theirs).facilities.push(facility(FacilityKind::Factory));
    let place = Place::State(theirs);
    assert!(!g.has_eye(Seat(0), BodyId::Earth), "no Embassy, no eye");
    assert!(g.eye_income(Seat(0), place).is_none(), "and nothing is read without one");
    g.state_mut(StateId::EastAsia).facilities.push(facility(FacilityKind::Embassy));
    assert!(g.has_eye(Seat(0), BodyId::Earth), "an Embassy in a Region it directs is an eye on Earth");
    let read = g.eye_income(Seat(0), place).expect("the eye reads Russia");
    assert!(read.iter().any(|(name, _)| name == "Factory"), "building by building: {:?}", read.iter().map(|(n, _)| n.clone()).collect::<Vec<_>>());
    let factory = read.iter().find(|(n, _)| n == "Factory").map(|(_, y)| y.clone()).unwrap();
    assert_eq!(factory, g.facility_yield(Seat(1), theirs, FacilityKind::Factory), "and reads the RIVAL's figures, not the watcher's");
    // The Arkwrights have no Embassy anywhere, so they read nothing of the same Region.
    assert!(!g.has_eye(Seat(2), BodyId::Earth), "the Arkwrights built none");
    assert!(g.eye_income(Seat(2), place).is_none(), "a seat with no eye reads nothing a seat with one reads");
    // A seat never needs an eye on its own place.
    assert!(g.eye_income(Seat(0), Place::State(StateId::EastAsia)).is_none(), "its own income is on its own card");
    // Off Earth: a Relay at a Colony of the seat's at the same Body.
    let rival = colony(&mut g, Seat(1), BodyId::Moon, &[ModuleKind::Mine], 4);
    assert!(g.eye_income(Seat(0), Place::Colony(rival)).is_none(), "an Embassy on Earth is no eye at the Moon");
    let mine = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Relay], 4);
    let _ = mine;
    assert!(g.has_eye(Seat(0), BodyId::Moon), "a working Relay at the Body is the eye there");
    let read = g.eye_income(Seat(0), Place::Colony(rival)).expect("the eye reads their Colony");
    assert!(read.iter().any(|(name, _)| name == "Mine"), "Module by Module: {:?}", read.iter().map(|(n, _)| n.clone()).collect::<Vec<_>>());
    assert!(g.eye_income(Seat(2), Place::Colony(rival)).is_none(), "and a seat with no Relay there still reads nothing");
}

// ------------------------------------------- 0.09.1 ticket #343: the Missile Carrier

/// Ticket #343 (version 0.09.1): a Missile Carrier of `seat`'s, in an orbit of a Body, with or
/// without its Warhead.
fn carrier_in(g: &mut Game, seat: Seat, body: BodyId, slot: Option<u32>, warhead: bool) -> ShipId {
    let id = ship_in(g, seat, UnitKind::MissileCarrier, body, slot, Stance::Hold);
    g.ship_mut(id).unwrap().warhead = warhead;
    id
}

/// Ticket #343 (R1): the Missile Carrier is a Ship and never a warship. Its card carries the
/// resolution's figures; it holds no Orbital Control, it blockades nothing; and under ticket #326's
/// escort rule -- which reads exactly `is_warship()` -- it is struck only once its party has no
/// warship left standing, which is by decision the whole counter to it.
#[test]
fn a_missile_carrier_is_a_ship_and_never_a_warship() {
    let g = game();
    let card = g.tables.unit(UnitKind::MissileCarrier);
    assert_eq!((card.materials, card.widgets), (60, 16), "60 Materials and 16 Widgets");
    assert_eq!((card.strength, card.hit_points, card.pursuit), (0, 3, 0), "no strength, three hit points, no pursuit");
    assert_eq!(card.tank, 30, "every hull's tank");
    assert_eq!((card.carries_colonists, card.carries_army), (0, false), "it carries a Warhead and nothing else");
    assert!(UnitKind::SHIPS.contains(&UnitKind::MissileCarrier), "it is a Ship");
    assert!(!UnitKind::MissileCarrier.is_warship(), "and never a warship");
    assert_eq!(UnitKind::MissileCarrier.name(), "Missile Carrier");
    // It holds no Orbital Control, alone in an empty low orbit.
    let mut g = game();
    calm(&mut g);
    g.ships.retain(|s| s.at != ShipAt::Body(BodyId::Mars));
    carrier_in(&mut g, Seat(0), BodyId::Mars, None, true);
    assert_eq!(g.orbital_control(BodyId::Mars), None, "a Missile Carrier holds no orbit");
    // And it cannot blockade: the Blockade gate wants a warship of the seat's.
    let order = Order::ShipStance { body: BodyId::Mars, stance: Stance::Blockade };
    let err = g.check_order(Seat(0), &[], &order).unwrap_err().0;
    assert!(err.contains("no warship of yours"), "a Missile Carrier blockades nothing: {err}");
    // The escort rule, on the same `armed` flag the Resolution sets from `is_warship()`: a Frigate
    // attacks a Missile Carrier escorted by a Frigate. Every hit lands on the escort while it
    // stands; the carrier (3 hit points) is struck only after, and dies on the third.
    let carrier = Combatant::new(UnitRef::Ship(ShipId(3)), "Missile Carrier 3".to_string(), 0, 3, 0, 0, false).armed(UnitKind::MissileCarrier.is_warship());
    let mut a = vec![frigate(1)];
    let mut d = vec![carrier, frigate(2)];
    let chances = vec![true, true, true, false, true, true, true, false, true, true, true];
    let mut dice = Script { chances: VecDeque::from(chances), d6s: VecDeque::new(), picks: VecDeque::new() };
    combat::fight(&mut a, &mut d, &mut dice, 2.0);
    assert!(d[1].destroyed(), "the escort fell first, at {} hits", d[1].damage);
    assert_eq!(d[1].damage, 4, "every hit while it stood landed on the warship");
    assert_eq!(d[0].damage, 3, "the unescorted carrier was struck only after, and died");
}

/// Ticket #343 (R2): the Missile Carrier waits on a Tech of its own, the game's first weapon Tech,
/// through a `needs_tech` field that no unit card carried before this ticket. Every other unit row
/// leaves it absent, so nothing else changed.
#[test]
fn a_missile_carrier_waits_for_missile_technology() {
    let mut g = game();
    calm(&mut g);
    let yard = colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Shipyard], 4);
    // Ticket #87: a Ship is built with a full tank, so the yard needs the Fuel for one.
    g.seats[0].stockpile.materials = 10_000;
    g.seats[0].stockpile.fuel = 10_000;
    let order = Order::BuildShip { site: Place::Colony(yard), kind: UnitKind::MissileCarrier };
    let err = g.check_order(Seat(0), &[], &order).unwrap_err().0;
    assert!(err.contains("Missile Technology"), "the refusal names the Tech: {err}");
    with_tech(&mut g, TechId::MissileTechnology);
    let ok = g.check_order(Seat(0), &[], &order);
    assert!(ok.is_ok(), "with the Tech standing it is ordered: {:?}", ok.err());
    // Every other Ship is unchanged: no unit row but this one names a Tech.
    for k in UnitKind::SHIPS.into_iter().chain(std::iter::once(UnitKind::Army)) {
        let wants = g.tables.unit(k).needs_tech;
        if k == UnitKind::MissileCarrier {
            assert_eq!(wants, Some(TechId::MissileTechnology), "the carrier's row names it");
        } else {
            assert_eq!(wants, None, "{} names no Tech", k.name());
        }
    }
    // The Tech's own row: the rung and the cost are the designer's; the branch and the
    // prerequisite are the build's choice, named in the ticket for correction.
    let t = g.tables.tech(TechId::MissileTechnology);
    assert_eq!((t.rung, t.cost), (3, 48), "rung 3, cost 48");
    assert_eq!(t.branch, "Propulsion");
    assert_eq!(t.needs, vec![TechId::HardenedHulls]);
}

/// Ticket #343 (R3): the Launch gate. Its shape is a Bombard's -- your Ship, the right kind, not in
/// transit, in the orbit that touches the target and holding it outright, a rival's place, one
/// order per hull -- with two differences: it wants the WARHEAD aboard, and EARTH IS NOT EXCEPTED.
#[test]
fn a_launch_wants_its_warhead_the_orbit_held_and_a_rivals_place() {
    let mut g = game();
    calm(&mut g);
    // Seat 0 holds Mars low orbit outright with a Frigate; the carrier rides with it.
    g.ships.retain(|s| s.at != ShipAt::Body(BodyId::Mars));
    ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Mars, None, Stance::Hold);
    let ship = carrier_in(&mut g, Seat(0), BodyId::Mars, None, true);
    let theirs = colony(&mut g, Seat(1), BodyId::Mars, &[ModuleKind::Habitat], 4);
    let mine = colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Habitat], 4);
    let at_theirs = Order::Launch { ship, target: Place::Colony(theirs) };
    assert_eq!(g.orbital_control(BodyId::Mars), Some(Seat(0)));
    assert!(g.check_order(Seat(0), &[], &at_theirs).is_ok(), "the orbit held outright, a rival's place");
    // Not at a place of its own.
    let err = g.check_order(Seat(0), &[], &Order::Launch { ship, target: Place::Colony(mine) }).unwrap_err().0;
    assert!(err.contains("not a rival's place"), "{err}");
    // Not at a neutral Region either.
    let neutral = StateId::ALL.into_iter().find(|s| g.state(*s).control == Control::Neutral).expect("a neutral Region");
    let err = g.check_order(Seat(0), &[], &Order::Launch { ship, target: Place::State(neutral) }).unwrap_err().0;
    assert!(err.contains("not a rival's place") || err.contains("not at this Body"), "{err}");
    // Not from a Battleship.
    let gun = ship_in(&mut g, Seat(0), UnitKind::Battleship, BodyId::Mars, None, Stance::Hold);
    let err = g.check_order(Seat(0), &[], &Order::Launch { ship: gun, target: Place::Colony(theirs) }).unwrap_err().0;
    assert!(err.contains("only a Missile Carrier can Launch"), "{err}");
    g.ships.retain(|s| s.id != gun);
    // Not without the Warhead, and the refusal names the rearm.
    g.ship_mut(ship).unwrap().warhead = false;
    let err = g.check_order(Seat(0), &[], &at_theirs).unwrap_err().0;
    assert!(err.contains("fired its Warhead") && err.contains("Rearm"), "{err}");
    g.ship_mut(ship).unwrap().warhead = true;
    // Not from the wrong orbit: a station is reached from its own ring, not from low orbit.
    let station = station_at(&mut g, Seat(1), BodyId::Mars);
    let slot = g.colony(station).unwrap().slot;
    let err = g.check_order(Seat(0), &[], &Order::Launch { ship, target: Place::Colony(station) }).unwrap_err().0;
    assert!(err.starts_with("Move this Ship to Mars, at ") && err.contains("then Launch next turn"), "{err}");
    // Not with the orbit contested: a rival warship in low orbit takes the Control.
    let rival = ship_in(&mut g, Seat(2), UnitKind::Frigate, BodyId::Mars, None, Stance::Hold);
    let err = g.check_order(Seat(0), &[], &at_theirs).unwrap_err().0;
    assert!(err.contains("Orbital Control"), "the refusal names which: {err}");
    g.ships.retain(|s| s.id != rival);
    // From the ring, the station is lawful and the ground below is not.
    g.ship_mut(ship).unwrap().slot = Some(slot);
    assert!(g.check_order(Seat(0), &[], &Order::Launch { ship, target: Place::Colony(station) }).is_ok(), "the station from its own ring");
    let rival = ship_in(&mut g, Seat(2), UnitKind::Frigate, BodyId::Mars, Some(slot), Stance::Hold);
    let err = g.check_order(Seat(0), &[], &Order::Launch { ship, target: Place::Colony(station) }).unwrap_err().0;
    assert!(err.contains("a rival still stands"), "{err}");
    g.ships.retain(|s| s.id != rival);
    // One order a hull.
    g.ship_mut(ship).unwrap().slot = None;
    assert!(g.check_order(Seat(0), &[Order::Launch { ship, target: Place::Colony(theirs) }], &at_theirs).is_err(), "one order per hull");
    // AND EARTH IS A LAWFUL TARGET, where a Bombard is refused outright.
    let mut g = game();
    calm(&mut g);
    g.ships.retain(|s| s.at != ShipAt::Body(BodyId::Earth));
    ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Earth, None, Stance::Hold);
    let ship = carrier_in(&mut g, Seat(0), BodyId::Earth, None, true);
    let region = StateId::ALL.into_iter().find(|s| g.state(*s).control.director() == Some(Seat(1))).expect("a Region of seat 1's");
    assert_eq!(g.orbital_control(BodyId::Earth), Some(Seat(0)));
    assert!(g.check_order(Seat(0), &[], &Order::Launch { ship, target: Place::State(region) }).is_ok(), "a Region is a lawful target and Earth is not excepted");
}

/// Ticket #343 (R4): what a Launch does. Every building rolls the nuke's own chance, the Core
/// Module and the Archive spared; a share of the people dies; at a Region the Standing Army is
/// destroyed and the Industry Level falls, floored at the state card's own and raisable again;
/// rung 4 against the holder; and ON EARTH ONLY the war bucket and the Natural Sink move. The
/// Occupation's roll is untouched: it still reads `influence.destruction_chance` and spares nothing.
#[test]
fn a_launch_guts_a_region_and_leaves_the_occupations_roll_alone() {
    let mut g = game();
    calm(&mut g);
    std::sync::Arc::make_mut(&mut g.tables).nuke.destruction_chance = 1.0;
    g.ships.retain(|s| s.at != ShipAt::Body(BodyId::Earth));
    ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Earth, None, Stance::Hold);
    let ship = carrier_in(&mut g, Seat(0), BodyId::Earth, None, true);
    let region = StateId::ALL.into_iter().find(|s| g.state(*s).control.director() == Some(Seat(1))).expect("a Region of seat 1's");
    let floor = g.tables.state(region).industry_level;
    g.state_mut(region).industry_level = floor + 2;
    g.state_mut(region).facilities = vec![facility(FacilityKind::Factory), facility(FacilityKind::PowerPlant)];
    let people = g.state(region).population;
    // A Standing Army of its own, if the board has not given it one.
    if !g.armies.iter().any(|a| a.standing && a.home == ArmyHome::State(region)) {
        let id = ArmyId(g.fresh_id());
        g.armies.push(Army { id, name: "the Standing Army".to_string(), home: ArmyHome::State(region), at: ArmyAt::Place(Place::State(region)), damage: 0, standing: true, stance: Stance::Hold, escaped: false, move_to: None, levy: false, raised_strength: 2 });
    }
    let standing_lost = g.war.standing_armies_lost;
    let owed = g.relations.owed[1][0];
    let sink = g.climate.natural_sink;
    let war = g.climate.war_next[0];
    g.commit_orders(Seat(0), &[Order::Launch { ship, target: Place::State(region) }]);
    g.resolution_phase();
    let st = g.state(region);
    assert!(st.facilities.is_empty(), "every Facility rolled and burned: {:?}", st.facilities.iter().map(|f| f.kind).collect::<Vec<_>>());
    let share = 1.0 - st.population / people;
    assert!((0.40..=0.60).contains(&share), "between two fifths and three fifths of the people died: {share:.3}");
    assert_eq!(g.war.standing_armies_lost, standing_lost + 1, "the Standing Army with them");
    assert_eq!(g.state(region).industry_level, floor + 1, "the Industry Level fell by one");
    assert_eq!(g.relations.owed[1][0] - owed, 4, "rung 4 against the holder");
    assert!(!g.ship(ship).unwrap().warhead, "the Warhead is spent");
    // On Earth: the war bucket and the Sink both move, by the table's figures.
    assert!((g.climate.war_next[0] - war - 20.0).abs() < 1e-9, "20.0 ppm into the war bucket: {}", g.climate.war_next[0] - war);
    assert!((g.climate.natural_sink - sink - 0.25).abs() < 1e-9, "the Sink rose a quarter, for good: {}", g.climate.natural_sink - sink);
    assert_eq!(g.war.launches[0], 1);
    assert_eq!(g.war.launch_buildings_burned[0], 2);
    assert_eq!(g.war.industry_levels_lost[0], 1);
    assert!(g.war.launch_people_killed[0] > 0.0);
    // The setback is not ruin: the Region may raise its Industry Level again.
    g.seats[1].stockpile.materials = 10_000;
    assert!(g.check_order(Seat(1), &[], &Order::RaiseIndustry { state: region }).is_ok(), "a nuked Region raises its Industry Level again");
    // And the fall is floored at the card's own figure: a second strike takes nothing more.
    g.state_mut(region).industry_level = floor;
    let ship2 = carrier_in(&mut g, Seat(0), BodyId::Earth, None, true);
    g.commit_orders(Seat(0), &[Order::Launch { ship: ship2, target: Place::State(region) }]);
    g.resolution_phase();
    assert_eq!(g.state(region).industry_level, floor, "never below the board it started on");

    // OFF EARTH a Launch poisons nothing, and the Core Module and the Archive are spared.
    let mut g = game();
    calm(&mut g);
    std::sync::Arc::make_mut(&mut g.tables).nuke.destruction_chance = 1.0;
    g.ships.retain(|s| s.at != ShipAt::Body(BodyId::Mars));
    ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Mars, None, Stance::Hold);
    let ship = carrier_in(&mut g, Seat(0), BodyId::Mars, None, true);
    let theirs = colony(&mut g, Seat(1), BodyId::Mars, &[ModuleKind::Habitat, ModuleKind::Archive, ModuleKind::Mine], 8);
    let sink = g.climate.natural_sink;
    let war = g.climate.war_next[0];
    g.commit_orders(Seat(0), &[Order::Launch { ship, target: Place::Colony(theirs) }]);
    g.resolution_phase();
    let kinds: Vec<ModuleKind> = g.colony(theirs).unwrap().modules.iter().map(|m| m.kind).collect();
    assert_eq!(kinds.len(), 2, "everything but the two monuments burned: {kinds:?}");
    assert!(kinds.contains(&ModuleKind::Core) && kinds.contains(&ModuleKind::Archive), "the Core Module and the Archive are spared: {kinds:?}");
    assert!((g.climate.natural_sink - sink).abs() < 1e-9, "a nuke on Mars does not touch Earth's Sink");
    assert!((g.climate.war_next[0] - war).abs() < 1e-9, "nor Earth's air");

    // THE OCCUPATION IS UNCHANGED: it reads `influence.destruction_chance` and spares nothing.
    let mut g = game();
    calm(&mut g);
    {
        let t = std::sync::Arc::make_mut(&mut g.tables);
        t.influence.destruction_chance = 1.0;
        t.nuke.destruction_chance = 0.0;
    }
    let taken = colony(&mut g, Seat(1), BodyId::Moon, &[ModuleKind::Habitat, ModuleKind::Archive], 4);
    let before = g.colony(taken).unwrap().modules.len();
    assert_eq!(before, 3, "a Habitat, an Archive and the Core Module");
    g.colony_mut(taken).unwrap().control = Control::Occupied { occupier: Seat(0), previous: None, turns: g.tables.influence.occupation_turns, banked: 0 };
    let army = ArmyId(g.fresh_id());
    g.armies.push(Army { id: army, name: "the 1st".to_string(), home: ArmyHome::State(StateId::EastAsia), at: ArmyAt::Place(Place::Colony(taken)), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None, levy: false, raised_strength: 2 });
    g.resolution_phase();
    assert!(g.colony(taken).unwrap().modules.is_empty(), "the Occupation still burns EVERYTHING at a certain chance, the Core Module and the Archive included: {:?}", g.colony(taken).unwrap().modules.iter().map(|m| m.kind).collect::<Vec<_>>());
}

/// Ticket #343 (R5): a Rearm. It wants a Colony or station of the seat's Faction with a working
/// Shipyard, in the hull's own orbit, and it is a BUILD in that yard's queue: the table's Materials
/// at the order and its Widgets over the turns the yard takes to make them.
#[test]
fn rearming_wants_a_working_shipyard_in_the_hulls_own_orbit() {
    let mut g = game();
    calm(&mut g);
    g.ships.retain(|s| s.at != ShipAt::Body(BodyId::Mars));
    let ship = carrier_in(&mut g, Seat(0), BodyId::Mars, None, false);
    let order = Order::Rearm { ship };
    let err = g.check_order(Seat(0), &[], &order).unwrap_err().0;
    assert!(err.contains("no place of yours"), "nothing of the seat's in this orbit: {err}");
    let yard = colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Habitat], 4);
    let err = g.check_order(Seat(0), &[], &order).unwrap_err().0;
    assert!(err.contains("no working Shipyard"), "the refusal names the Shipyard: {err}");
    g.colony_mut(yard).unwrap().modules.push(Module::new(ModuleKind::Shipyard));
    assert!(g.check_order(Seat(0), &[], &order).is_ok(), "a working Shipyard of its own, in its own orbit");
    // A carrier that still has its Warhead is refused.
    g.ship_mut(ship).unwrap().warhead = true;
    let err = g.check_order(Seat(0), &[], &order).unwrap_err().0;
    assert!(err.contains("already carries its Warhead"), "{err}");
    g.ship_mut(ship).unwrap().warhead = false;
    // The price, and the build.
    assert_eq!(g.order_cost(Seat(0), &order).materials, 40, "40 Materials at the order");
    g.seats[0].stockpile.materials = 10_000;
    g.commit_orders(Seat(0), std::slice::from_ref(&order));
    let b = g.colony(yard).unwrap().queue.last().expect("the Warhead is in the yard's queue").clone();
    assert_eq!(b.item, BuildItem::Warhead(ship));
    assert_eq!(b.widgets, 8, "8 Widgets");
    g.colony_mut(yard).unwrap().queue.last_mut().unwrap().widgets = 0;
    g.resolution_phase();
    assert!(g.ship(ship).unwrap().warhead, "the Warhead is aboard again");
    assert!(g.colony(yard).unwrap().queue.is_empty(), "and the build is done");
}

/// Ticket #343 (R6): the Sink Weakens SUBTRACTS `sink_cut` where it used to assign `sink_after`.
/// On an untouched game the outcome is identical, 6.0 to 4.0, so no existing measurement moves;
/// what changes is that a Sink somebody has raised keeps what it was given. The projection
/// subtracts too, and -- the trap -- must NOT re-add the Scrubbers the assignment had to.
#[test]
fn the_sink_weakens_subtracts_and_the_projection_keeps_the_scrubbers_once() {
    let cut = game().tables.climate.breaks[break_at(&game(), "sink_weakens")].sink_cut;
    assert!((cut - 2.0).abs() < 1e-9, "the table carries a cut, not a figure to land on: {cut}");
    // On an untouched game: 6.0 - 2.0 = 4.0, exactly where the assignment put it.
    let fire = |start: f64| -> f64 {
        let mut g = game();
        calm(&mut g);
        bare_world(&mut g);
        g.climate.natural_sink = start;
        let i = break_at(&g, "sink_weakens");
        g.climate.breaks_fired = vec![true; g.tables.climate.breaks.len()];
        g.climate.breaks_fired[i] = false;
        // A Stock that leaves the Temperature still rising, so the phase's own update does not
        // carry it back under the Break's 2.0 before the Break is checked.
        g.climate.temperature = 2.0;
        g.climate.co2 = 1_000.0;
        g.climate_phase();
        assert!(g.climate.breaks_fired[break_at(&g, "sink_weakens")], "the Break fired");
        g.climate.natural_sink
    };
    assert!((fire(6.0) - 4.0).abs() < 1e-9, "an untouched game lands on 4.0 as it always did: {}", fire(6.0));
    assert!((fire(7.5) - 5.5).abs() < 1e-9, "a Sink that was RAISED keeps what it was given: {}", fire(7.5));
    // The projection. A world whose gross Emissions stand above the Sink once the Break has cut it
    // is on a collapse path; re-adding the Scrubbers to the cut figure would put the same world
    // comfortably under its own Sink and the forecast would say there is nothing to act on.
    let mut g = game();
    calm(&mut g);
    bare_world(&mut g);
    g.turn = 1;
    g.climate.natural_sink = 6.0;
    g.climate.co2 = 900.0;
    g.climate.temperature = 2.0;
    g.climate.permafrost = 0.0;
    let i = break_at(&g, "sink_weakens");
    g.climate.breaks_fired = vec![true; g.tables.climate.breaks.len()];
    g.climate.breaks_fired[i] = false;
    g.climate.last = EmissionsBreakdown { factories: 600.0, scrubbers: 400.0, ..Default::default() };
    // Projection sink 6 + 400 = 406, cut to 404: gross 600 stands 196 above it and the world burns.
    // Doubled, the cut figure would read 804 and the world would look to be cooling.
    assert!(!matches!(g.last_turn_to_act(), LastTurn::NoCollapse), "the forecast sees the collapse the cut Sink leaves: {:?}", g.last_turn_to_act());
}

/// Ticket #343 (R7): the computer. A seat with cause builds a Missile Carrier where a rival holds a
/// place it wants and cannot take, and fires at that place from an orbit it holds outright.
#[test]
fn the_computer_builds_a_missile_carrier_and_launches_with_cause() {
    let board = |score: i64| -> Game {
        let mut g = game();
        calm(&mut g);
        with_tech(&mut g, TechId::MissileTechnology);
        g.ships.retain(|s| s.at != ShipAt::Body(BodyId::Mars));
        ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Mars, None, Stance::Hold);
        colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Shipyard, ModuleKind::Factory], 4);
        colony(&mut g, Seat(1), BodyId::Mars, &[ModuleKind::Habitat, ModuleKind::Mine, ModuleKind::Generator], 8);
        g.relations.score[0][1] = score;
        g.seats[0].stockpile.materials = 10_000;
        g.seats[0].stockpile.fuel = 10_000;
        g
    };
    // No cause: no appetite, however rich the rival's Colony.
    let mut calm_board = board(0);
    assert!(calm_board.nuke_target(Seat(0)).is_none(), "no cause, nothing to fire at");
    let orders = calm_board.ai_orders(Seat(0));
    assert!(!orders.iter().any(|o| matches!(o, Order::BuildShip { kind: UnitKind::MissileCarrier, .. })), "no cause, no carrier: {orders:?}");
    // With cause: the rival's Colony is the target, and the seat orders a hull.
    let mut g = board(-8);
    assert!(g.nuke_target(Seat(0)).is_some(), "with cause there is a place it wants and cannot take");
    // The hull sits at Mars, so the target it can reach is the rival's Colony there, whatever
    // richer thing stands on Earth.
    let target = g.nuke_targets(Seat(0)).into_iter().find(|t| matches!(t, Place::Colony(c) if g.colony(*c).map(|c| c.body) == Some(BodyId::Mars))).expect("the rival's Colony at Mars is wanted");
    let orders = g.ai_orders(Seat(0));
    assert!(orders.iter().any(|o| matches!(o, Order::BuildShip { kind: UnitKind::MissileCarrier, .. })), "with cause it builds one: {orders:?}");
    // And with the hull built and armed, in the orbit it holds, it fires.
    let ship = carrier_in(&mut g, Seat(0), BodyId::Mars, None, true);
    let orders = g.ai_orders(Seat(0));
    assert!(orders.iter().any(|o| matches!(o, Order::Launch { ship: s, target: t } if *s == ship && *t == target)), "it fires at the place it wants: {orders:?}");
    // A spent hull is rearmed instead, at its own yard.
    g.ship_mut(ship).unwrap().warhead = false;
    let orders = g.ai_orders(Seat(0));
    assert!(orders.iter().any(|o| matches!(o, Order::Rearm { ship: s } if *s == ship)), "a spent hull goes back to the yard: {orders:?}");
    // The sweep counts: a resolved Launch is counted to the seat that fired it.
    g.ship_mut(ship).unwrap().warhead = true;
    g.commit_orders(Seat(0), &[Order::Launch { ship, target }]);
    g.resolution_phase();
    assert_eq!(g.war.launches[0], 1, "counted for the sweep");
    assert_eq!(g.war.missile_carriers_built, [0, 0, 0, 0], "and a hull placed by a test was never built");
}

// ---------------------------------------------------------------- Ticket #345: first to a Body

/// A loaded Colony Ship of one seat in LOW ORBIT at a Body, and the Unload that founds a ground
/// Colony in a named free slot of it. The slot is named so two seats can reach for one Body in one
/// Resolution without contesting a slot, which is the board R5 is about. The slot's yields are
/// pinned to its Body's card figures, as the `colony` helper pins them, so nothing here reads a
/// random draw.
fn lander(g: &mut Game, seat: Seat, body: BodyId, slot: u32) -> (ShipId, Order) {
    let id = ShipId(g.fresh_id());
    let name = g.next_ship_name(UnitKind::ColonyShip);
    g.ships.push(Ship {
        id, name, kind: UnitKind::ColonyShip, seat, damage: 0, at: ShipAt::Body(body), colonists: 4, warhead: false, colonists_education: 1.0,
        army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None,
    });
    g.slot_yields.insert((body, slot), SlotYields::of_body(g.tables.body(body)));
    (id, Order::Unload { ship: id, colonists: 4, army: false, into: UnloadTarget::Slot(body, slot) })
}

/// The Colony standing in a Body's slot, once a Resolution has put one there.
fn colony_in_slot(g: &Game, body: BodyId, slot: u32) -> ColonyId {
    g.colonies.iter().find(|c| c.body == body && c.slot == slot && !c.in_orbit).map(|c| c.id).expect("a ground Colony stands in that slot")
}

/// R1. The record names the seat that was first to a Body and the Colony it was first with, and a
/// Body's first is claimed once and for good: a second founding on the same Body, by anybody, never
/// grows a second row and never rewrites the row that is there.
#[test]
fn ticket_345_a_bodys_first_is_recorded_once_and_never_rewritten() {
    let mut g = game();
    calm(&mut g);
    assert!(g.first_at(BodyId::Moon).is_none(), "nobody has settled the Moon at the opening");
    assert!(g.body_firsts.is_empty(), "and the record is empty");
    let (_, down) = lander(&mut g, Seat(0), BodyId::Moon, 0);
    g.commit_orders(Seat(0), &[down]);
    g.resolution_phase();
    let first = colony_in_slot(&g, BodyId::Moon, 0);
    assert_eq!(g.first_at(BodyId::Moon), Some((Seat(0), first)), "the record names the seat and the Colony");
    assert_eq!(g.firsts_of(Seat(0)).len(), 1, "one Body claimed, one row");
    assert_eq!(g.firsts_of(Seat(1)).len(), 0, "and nothing for a seat that claimed nothing");
    // A rival lands on the same Body, in another slot, a turn later. The other slots stay open to
    // everybody: only the bonus was spent.
    g.turn += 1;
    let (_, late) = lander(&mut g, Seat(1), BodyId::Moon, 1);
    g.commit_orders(Seat(1), &[late]);
    g.resolution_phase();
    assert!(g.colonies.iter().any(|c| c.body == BodyId::Moon && c.slot == 1), "the rival's Colony stands");
    assert_eq!(g.body_firsts.len(), 1, "the record never grows a second row for one Body");
    assert_eq!(g.first_at(BodyId::Moon), Some((Seat(0), first)), "and never rewrites the row it has");
}

/// R2. What claims a first and what does not. Antarctica is on Earth and claims nothing, by either
/// of the two roads to its ice; a Space Station claims nothing and closes nothing, so the ground of
/// a Body a station orbits is still there to be taken; and the row a claim writes never names a
/// station. Venus can never be claimed at all, having no ground to land on -- no code says so, and
/// the day Venus is given a Colony Slot the rule turns on by itself.
#[test]
fn ticket_345_antarctica_and_a_station_claim_nothing_and_venus_has_no_ground_to_claim() {
    let mut g = game();
    calm(&mut g);
    g.antarctica_open = true;
    // (a) A Colony Ship's landing in Antarctica founds a ground Colony, and claims nothing.
    let (_, ice) = lander(&mut g, Seat(0), BodyId::Earth, 0);
    g.commit_orders(Seat(0), &[ice]);
    g.resolution_phase();
    assert!(g.colonies.iter().any(|c| c.body == BodyId::Earth && !c.in_orbit && c.slot == 0), "a Colony stands on the ice");
    assert!(g.first_at(BodyId::Earth).is_none(), "Antarctica is on Earth, and Earth is nobody's first");
    // (b) And neither does the other road to the same ice, the sea.
    g.antarctic_sends.push(AntarcticSend { seat: Seat(1), from: StateId::Europe, n: 4, education: 1.0, into: UnloadTarget::Slot(BodyId::Earth, 1), due_turn: g.turn });
    g.resolution_phase();
    assert!(g.colonies.iter().any(|c| c.body == BodyId::Earth && !c.in_orbit && c.slot == 1), "the Pioneers landed by sea");
    assert!(g.first_at(BodyId::Earth).is_none(), "the sea claims no first either");
    assert!(g.body_firsts.is_empty(), "and the record is still empty");
    // (c) A Space Station over Mars claims nothing and leaves the ground of Mars open.
    g.seats[0].stockpile.materials = 1_000;
    g.commit_orders(Seat(0), &[Order::BuildStation { body: BodyId::Mars, slot: 0 }]);
    g.resolution_phase();
    assert!(g.colonies.iter().any(|c| c.body == BodyId::Mars && c.in_orbit), "the station stands over Mars");
    assert!(g.first_at(BodyId::Mars).is_none(), "a station claims no Body");
    // The ground below it is still unclaimed, and the rival that lands takes the first.
    g.turn += 1;
    let (_, down) = lander(&mut g, Seat(1), BodyId::Mars, 0);
    g.commit_orders(Seat(1), &[down]);
    g.resolution_phase();
    let (who, what) = g.first_at(BodyId::Mars).expect("the ground of Mars was there to be taken");
    assert_eq!(who, Seat(1), "the seat that LANDED took it, not the seat in orbit above");
    assert!(!g.colony(what).unwrap().in_orbit, "the record never names a station");
    // (d) Venus has no ground at all, so nothing can ever land there to claim it.
    assert!(g.free_slots_on(BodyId::Venus).is_empty(), "Venus has no Colony Slot");
    assert_eq!(g.tables.body(BodyId::Venus).colony_slots(), 0, "and its card gives it none");
    assert!(g.first_at(BodyId::Venus).is_none(), "so Venus is unclaimable");
}

/// R3. The windfall. It is paid into an accumulator at the founding and into the Allotment at the
/// NEXT Income, which is the first moment anything can be spent -- the Income ASSIGNS the Allotment,
/// so a windfall written straight into `allotment` at the Resolution would be wiped before a point
/// of it could be spent and would pay exactly nothing. It is paid once: the accumulator is cleared
/// by the Income that paid it, and losing and retaking the Colony never pays it again.
#[test]
fn ticket_345_the_windfall_is_paid_into_the_allotment_of_the_turn_after_the_landing_and_paid_once() {
    let mut g = game();
    calm(&mut g);
    let plain = g.influence_allotment(Seat(0));
    let (_, down) = lander(&mut g, Seat(0), BodyId::Mars, 0);
    g.commit_orders(Seat(0), &[down]);
    g.resolution_phase();
    let windfall = g.tables.body(BodyId::Mars).first_windfall;
    assert_eq!(windfall, 15, "Mars pays 15, out of bodies.toml");
    assert_eq!(g.seat(Seat(0)).first_windfall, windfall, "the founding filled the accumulator");
    // The Allotment the next Income will ASSIGN carries it, and the Core's standing +1 beside it.
    let standing = g.tables.influence.first_settled_allotment;
    assert_eq!(g.influence_allotment(Seat(0)), plain + windfall + standing, "the windfall and the +1 are both in the figure the Income assigns");
    g.turn += 1;
    g.income_phase();
    assert_eq!(g.seat(Seat(0)).allotment, plain + windfall + standing, "and the Income paid them both into the Allotment, where they can be spent");
    assert_eq!(g.seat(Seat(0)).first_windfall, 0, "the accumulator is cleared by the Income that paid it");
    // A second Income pays the standing +1 again and the windfall never again.
    g.turn += 1;
    g.income_phase();
    assert_eq!(g.seat(Seat(0)).allotment, plain + standing, "the windfall is paid once");
    // Losing the Colony and taking it back never pays it again.
    let cid = colony_in_slot(&g, BodyId::Mars, 0);
    g.colony_mut(cid).unwrap().control = Control::Controlled(Seat(1));
    g.colony_mut(cid).unwrap().control = Control::Controlled(Seat(0));
    g.turn += 1;
    g.income_phase();
    assert_eq!(g.seat(Seat(0)).first_windfall, 0, "retaking the place pays no second windfall");
    assert_eq!(g.seat(Seat(0)).allotment, plain + standing, "and the Allotment carries the standing +1 alone");
}

/// R4. The Core's standing +1. It sleeps while a rival holds the Colony and never pays that rival;
/// it wakes when the founder takes the place back; it never hops to a second Colony of the
/// founder's on the same Body; and it keeps paying while the Colony is starved of Energy, where a
/// Relay or a Chorus goes quiet. That last is why it does not go through `building_allotment`.
#[test]
fn ticket_345_the_cores_standing_plus_one_sleeps_under_a_rival_never_hops_and_outlasts_a_starving() {
    let mut g = game();
    calm(&mut g);
    let standing = g.tables.influence.first_settled_allotment;
    assert_eq!(standing, 1, "one, out of influence.toml");
    let base_founder = g.influence_allotment(Seat(0));
    let base_rival = g.influence_allotment(Seat(1));
    let (_, down) = lander(&mut g, Seat(0), BodyId::Moon, 0);
    g.commit_orders(Seat(0), &[down]);
    g.resolution_phase();
    let cid = colony_in_slot(&g, BodyId::Moon, 0);
    // Clear the windfall so what is left in the figure is the standing +1 alone.
    g.seats[0].first_windfall = 0;
    assert_eq!(g.influence_allotment(Seat(0)) - base_founder, standing, "the founder is paid the +1 while it directs the place");
    // It sleeps under a rival, and never pays the rival.
    g.colony_mut(cid).unwrap().control = Control::Controlled(Seat(1));
    assert_eq!(g.influence_allotment(Seat(0)), base_founder, "it pays the founder nothing while a rival holds the place");
    assert_eq!(g.influence_allotment(Seat(1)), base_rival, "and it never pays the rival who took it");
    // And wakes when the founder takes it back.
    g.colony_mut(cid).unwrap().control = Control::Controlled(Seat(0));
    assert_eq!(g.influence_allotment(Seat(0)) - base_founder, standing, "and wakes when the founder takes it back");
    // It never hops: a second Colony of the founder's on the same Body pays nothing.
    g.turn += 1;
    let (_, again) = lander(&mut g, Seat(0), BodyId::Moon, 1);
    g.commit_orders(Seat(0), &[again]);
    g.resolution_phase();
    g.seats[0].first_windfall = 0;
    assert_eq!(g.influence_allotment(Seat(0)) - base_founder, standing, "a second Colony on the same Body pays no second +1");
    // A starved Colony keeps paying it, where a Relay goes quiet. `building_allotment` is what skips
    // a starved Colony, and this clause deliberately does not go through it.
    g.colony_mut(cid).unwrap().modules.push(Module::new(ModuleKind::Relay));
    let with_relay = g.influence_allotment(Seat(0));
    assert!(with_relay > base_founder + standing, "the Relay pays while the place is fed");
    // Ticket #278's starving: one rival holding Orbital Control of the Body outright with a stack
    // on Blockade in low orbit.
    ship_in(&mut g, Seat(1), UnitKind::Frigate, BodyId::Moon, None, Stance::Blockade);
    assert_eq!(g.starved_by(cid), Some(Seat(1)), "the Colony is starved");
    assert!(g.influence_allotment(Seat(0)) < with_relay, "the Relay at a starved Colony goes quiet");
    assert_eq!(g.influence_allotment(Seat(0)) - base_founder, standing, "and the +1 does not: being there first is not undone by a Blockade");
}

/// R4, the half of it the Faction multiplier decides. The Arkwrights convert Influence at x0.8, and
/// a clause INSIDE the multiplier gives them four fifths of it. Both halves of this rule sit outside
/// it, so an Arkwright first is worth exactly what any other Faction's is: the whole windfall and
/// the whole +1, as the Spaceport's clause already is.
#[test]
fn ticket_345_neither_half_is_shaved_by_the_arkwrights_multiplier() {
    let mut g = game();
    calm(&mut g);
    let ark = Seat::ALL.into_iter().find(|s| g.kind(*s) == FactionKind::Arkwrights).expect("an Arkwright sits at this table");
    assert!(g.tables.faction(FactionKind::Arkwrights).influence_multiplier < 1.0, "the Arkwrights convert at less than face value");
    let before = g.influence_allotment(ark);
    let (_, down) = lander(&mut g, ark, BodyId::Phobos, 0);
    g.commit_orders(ark, &[down]);
    g.resolution_phase();
    let windfall = g.tables.body(BodyId::Phobos).first_windfall;
    let standing = g.tables.influence.first_settled_allotment;
    assert_eq!(windfall, 20, "Phobos pays 20, out of bodies.toml");
    assert_eq!(g.influence_allotment(ark) - before, windfall + standing, "an Arkwright is paid the whole figure, not four fifths of it");
}

/// R5. Two seats founding a ground Colony at one Body in one Resolution, in different slots, both
/// land, and the first goes to the seat with the greater Ship stack strength at the Body --
/// `tiebreak_at_body`, the very function that settles two seats reaching for the SAME slot. The
/// designer, told that a pure random draw and the contested-slot rule are not the same thing:
/// *"let's keep the current system for ties."* A random draw parts only seats level on strength.
#[test]
fn ticket_345_a_body_reached_by_two_seats_at_once_goes_to_the_greater_fleet() {
    // The stronger fleet takes it, whichever seat the loop reaches first.
    let mut g = game();
    calm(&mut g);
    let (_, a) = lander(&mut g, Seat(0), BodyId::Mars, 0);
    let (_, b) = lander(&mut g, Seat(1), BodyId::Mars, 1);
    // A Frigate of seat 1's in an ORBITAL SLOT, not low orbit: it is stack strength at the Body and
    // not Orbital Control, so seat 0's landing is never barred and both Colonies are founded.
    ship_in(&mut g, Seat(1), UnitKind::Frigate, BodyId::Mars, Some(0), Stance::Hold);
    assert!(g.ship_stack_strength(Seat(1), BodyId::Mars) > g.ship_stack_strength(Seat(0), BodyId::Mars), "seat 1 has the stronger stack");
    g.commit_orders(Seat(0), &[a]);
    g.commit_orders(Seat(1), &[b]);
    g.resolution_phase();
    assert!(g.colonies.iter().any(|c| c.body == BodyId::Mars && c.slot == 0), "seat 0 landed too: different slots, both land");
    assert!(g.colonies.iter().any(|c| c.body == BodyId::Mars && c.slot == 1), "and so did seat 1");
    assert_eq!(g.body_firsts.len(), 1, "one Body, one row");
    assert_eq!(g.first_at(BodyId::Mars).map(|(s, _)| s), Some(Seat(1)), "the greater fleet at the Body took the first");
    assert_eq!(g.seat(Seat(0)).first_windfall, 0, "and the seat that lost it was paid nothing");
    assert_eq!(g.seat(Seat(1)).first_windfall, g.tables.body(BodyId::Mars).first_windfall, "while the winner was paid the windfall");
    // Level on strength, the draw parts them -- one of the two, never both, never neither.
    let mut drawn: Vec<Seat> = Vec::new();
    for seed in 1..14u64 {
        let mut g = with_seed(seed);
        calm(&mut g);
        let (_, a) = lander(&mut g, Seat(0), BodyId::Mars, 0);
        let (_, b) = lander(&mut g, Seat(1), BodyId::Mars, 1);
        assert_eq!(g.ship_stack_strength(Seat(0), BodyId::Mars), g.ship_stack_strength(Seat(1), BodyId::Mars), "two Colony Ships are level: neither has any strength");
        g.commit_orders(Seat(0), &[a]);
        g.commit_orders(Seat(1), &[b]);
        g.resolution_phase();
        assert_eq!(g.body_firsts.len(), 1, "exactly one of them claims it");
        let (who, _) = g.first_at(BodyId::Mars).expect("somebody claimed Mars");
        assert!(who == Seat(0) || who == Seat(1), "and it is one of the two that landed");
        if !drawn.contains(&who) {
            drawn.push(who);
        }
    }
    assert_eq!(drawn.len(), 2, "level on strength it is a draw, and over thirteen seeds it fell both ways: {drawn:?}");
}

/// R6. The computer, at Earth: the destination a loaded Colony Ship is sent to reads what an
/// unclaimed Body would pay, so a distant world nobody has settled becomes worth the voyage. It is
/// the change that makes the rule exist in play -- no computer seat founded a Colony anywhere in the
/// Mars system in eighty measured games before it.
///
/// Measured rather than asserted on one board, because the destination list weighs a slot's own
/// drawn yields against the flight, and on some boards the Mars system already wins without any
/// prize. The witness is the FIGURE: the same twelve boards are put to the computer twice, once
/// with `first_windfall_worth` as `ai.toml` has it and once with it at nought, and nothing else
/// differs.
#[test]
fn ticket_345_the_computer_sends_its_colony_ship_to_a_world_nobody_has_settled() {
    let mars_system_picks = |worth: f64, claimed: bool| -> usize {
        let mut picked = 0;
        for seed in 1..13u64 {
            let mut t = Tables::load(&default_data_dir()).expect("tables load");
            t.ai.thresholds.first_windfall_worth = worth;
            let mut g = Game::new(Arc::new(t), NewGame { seed, player: FactionKind::Custodians, player_is_ai: false, player_start: StateId::EastAsia });
            calm(&mut g);
            g.turn = g.next_window_turn(1);
            if claimed {
                // The record alone, with no Colony planted: nothing else about the board moves.
                for (i, b) in [BodyId::Moon, BodyId::Mars, BodyId::Phobos, BodyId::Deimos].into_iter().enumerate() {
                    g.body_firsts.push(BodyFirst { body: b, seat: Seat(2), colony: ColonyId(9_000 + i as u32) });
                }
            }
            let id = ShipId(g.fresh_id());
            let name = g.next_ship_name(UnitKind::ColonyShip);
            g.ships.push(Ship {
                id, name, kind: UnitKind::ColonyShip, seat: Seat(1), damage: 0, at: ShipAt::Body(BodyId::Earth), colonists: 4, warhead: false,
                colonists_education: 1.0, army: None, stance: Stance::Hold, escaped: false, arrived_this_turn: false, built_turn: 1, fuel: 30, slot: None,
            });
            g.seats[1].stockpile.fuel = 200;
            g.seats[1].stockpile.energy = 400;
            let dest = g.ai_orders(Seat(1)).iter().find_map(|o| match o {
                Order::Transit { ship: s, to, .. } if *s == id => Some(*to),
                _ => None,
            });
            if matches!(dest, Some(BodyId::Mars) | Some(BodyId::Phobos) | Some(BodyId::Deimos)) {
                picked += 1;
            }
        }
        picked
    };
    let figure = Tables::load(&default_data_dir()).expect("tables load").ai.thresholds.first_windfall_worth;
    assert!(figure > 0.0, "the figure is in ai.toml and is not nought");
    let (with_prize, without) = (mars_system_picks(figure, false), mars_system_picks(0.0, false));
    assert!(with_prize > without, "the windfall pulls the voyage out to the Mars system: {with_prize} boards of twelve against {without}");
    // And what it reads is the RECORD: with every first already claimed the figure buys nothing.
    let spent = mars_system_picks(figure, true);
    assert_eq!(spent, without, "a Body already claimed pays nothing, so the old ranking stands: {spent} against {without}");
}

/// R6, the half of it that lives at the Body rather than at Earth: the founding appetite itself is
/// lifted at a world still unclaimed, so a Ship that has arrived commits to the ground rather than
/// parking its load somewhere easier.
#[test]
fn ticket_345_the_founding_appetite_is_lifted_at_a_world_still_unclaimed() {
    let scored = |claimed: bool| -> f64 {
        let mut g = with_seed(7);
        calm(&mut g);
        let (ship, _) = lander(&mut g, Seat(1), BodyId::Mars, 0);
        let _ = ship;
        if claimed {
            let cid = colony(&mut g, Seat(2), BodyId::Mars, &[], 1);
            g.body_firsts.push(BodyFirst { body: BodyId::Mars, seat: Seat(2), colony: cid });
        }
        g.log.clear();
        let _ = g.ai_orders(Seat(1));
        // The scored list the computer wrote down, which is the only record of what it wanted and
        // by how much. "found a Colony at ... on Mars" is the candidate this rule lifts.
        g.log
            .iter()
            .filter(|l| l.contains("found a Colony at") && l.contains("on Mars"))
            .filter_map(|l| l.split_whitespace().nth(1).and_then(|n| n.parse::<f64>().ok()))
            .fold(0.0, f64::max)
    };
    let (unclaimed, taken) = (scored(false), scored(true));
    assert!(taken > 0.0, "the appetite is there either way: {taken}");
    assert!(unclaimed > taken, "and a world nobody has settled is wanted more: {unclaimed} against {taken}");
}

/// R7. What the game says when a Body's first is claimed: a Report line and a Moment, each naming
/// the Faction, the Body and the Colony.
#[test]
fn ticket_345_the_report_and_the_moment_name_the_faction_the_body_and_the_colony() {
    let mut g = game();
    calm(&mut g);
    g.report = Report::default();
    let (_, down) = lander(&mut g, Seat(0), BodyId::Deimos, 0);
    g.commit_orders(Seat(0), &[down]);
    g.resolution_phase();
    let cid = colony_in_slot(&g, BodyId::Deimos, 0);
    let (faction, body, place) = (g.seat_name(Seat(0)), g.tables.body(BodyId::Deimos).name.clone(), g.place_name(Place::Colony(cid)));
    let texts: Vec<String> = g.report.lines.iter().map(|l| l.text.clone()).collect();
    let line = texts.iter().find(|t| t.contains(&body) && t.contains("first")).unwrap_or_else(|| panic!("a Report line says who was first: {texts:?}"));
    assert!(line.contains(&faction), "the line names the Faction: {line}");
    assert!(line.contains(&place), "and the Colony: {line}");
    assert!(line.contains(&g.tables.body(BodyId::Deimos).first_windfall.to_string()), "and what it pays: {line}");
    let moment = g.report.moments.iter().find(|m| m.kind == MomentKind::FirstToABody).expect("a Moment stops the turn for it");
    assert!(moment.text.contains(&faction), "the Moment names the Faction: {}", moment.text);
    assert!(moment.text.contains(&body), "and the Body: {}", moment.text);
    assert!(moment.text.contains(&place), "and the Colony: {}", moment.text);
    assert_eq!(moment.place, Some(ReportPlace::Colony(cid)), "and points at the Colony");
    assert!(g.tables.report.moment(MomentKind::FirstToABody).is_some(), "and report.toml carries its card");
}

/// The error case the load check owns: a Body row with no `first_windfall` is a rule this build
/// cannot price, and the whole table is refused rather than quietly paying nothing.
#[test]
fn ticket_345_a_body_row_without_a_windfall_refuses_the_table() {
    let src = default_data_dir();
    let dir = std::env::temp_dir().join(format!("dying-earth-345-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a temporary data folder");
    for entry in std::fs::read_dir(&src).expect("the data folder") {
        let entry = entry.expect("a data file");
        if entry.path().is_file() {
            std::fs::copy(entry.path(), dir.join(entry.file_name())).expect("copy");
        }
    }
    assert!(Tables::load(&dir).is_ok(), "the copy loads before anything is taken out of it");
    let bodies = std::fs::read_to_string(dir.join("bodies.toml")).expect("bodies.toml");
    assert!(bodies.contains("first_windfall = 15"), "Mars carries its figure");
    let stripped: String = bodies.lines().filter(|l| l.trim() != "first_windfall = 15").collect::<Vec<_>>().join("\n");
    std::fs::write(dir.join("bodies.toml"), stripped).expect("write");
    let err = Tables::load(&dir).expect_err("a Body with no windfall is refused");
    assert!(format!("{err:?}").contains("first_windfall"), "and the refusal names the missing figure: {err:?}");
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------- Ticket #346: Battles cost Fuel

/// Ticket #346 (version 0.09.1): a Body with an empty low orbit and no station standing, so a test
/// puts exactly the hulls it reasons about into the fight and nothing arrives to join them.
fn empty_orbit(g: &mut Game, body: BodyId) {
    g.ships.retain(|s| s.at != ShipAt::Body(body));
    g.colonies.retain(|c| c.body != body);
}

/// Ticket #346 (R1): the charge. Every Ship named in any party of a SHIP Battle pays `battle_fuel`
/// out of its own tank, once, floored at nought -- struck or not, armed or not, whichever side it
/// is on and whether or not it opened the fight. A Battery pays nothing and never panics; a Ship
/// that was not in the Battle pays nothing.
#[test]
fn a_battle_takes_fuel_from_every_hull_named_in_it() {
    let mut g = game();
    calm(&mut g);
    empty_orbit(&mut g, BodyId::Mars);
    let charge = g.tables.melee.battle_fuel;
    assert_eq!(charge, 2, "[melee] battle_fuel");
    let attacker = ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Mars, None, Stance::Attack);
    let defender = ship_in(&mut g, Seat(1), UnitKind::Frigate, BodyId::Mars, None, Stance::Hold);
    let hauler = ship_in(&mut g, Seat(1), UnitKind::ColonyShip, BodyId::Mars, None, Stance::Hold);
    // A hull with less in the tank than the charge pays what it has and no more.
    let nearly = ship_in(&mut g, Seat(1), UnitKind::Frigate, BodyId::Mars, None, Stance::Hold);
    g.ship_mut(nearly).unwrap().fuel = 1;
    // A Battery of the defender's, in low orbit: it stands in the line and has no tank to charge.
    let ground = colony(&mut g, Seat(1), BodyId::Mars, &[ModuleKind::Battery], 4);
    // And a hull at another Body, which fights nothing.
    let elsewhere = ship_in(&mut g, Seat(2), UnitKind::Frigate, BodyId::Earth, None, Stance::Hold);
    g.resolution_phase();
    assert_eq!(g.war.orbit_attacks[0], 1, "one Battle was fought in Mars low orbit");
    // The charge is taken at the head of the melee, so the counters are exact whatever the dice did.
    assert_eq!(g.war.battle_fuel_burned[0], charge, "the aggressor's one hull paid the charge");
    assert_eq!(g.war.battle_fuel_burned[1], charge * 2 + 1, "two full tanks paid it and the near-empty one paid the 1 it had");
    assert_eq!(g.war.battle_fuel_burned[2], 0, "a seat that was not in the Battle paid nothing");
    for (id, what) in [(attacker, "the aggressor"), (defender, "the defender"), (hauler, "the unarmed hull")] {
        if let Some(s) = g.ship(id) {
            assert_eq!(s.fuel, 30 - charge, "{what} paid the charge out of its own tank");
        }
    }
    if let Some(s) = g.ship(nearly) {
        assert_eq!(s.fuel, 0, "a tank under the charge is emptied and never goes negative");
    }
    assert_eq!(g.ship(elsewhere).unwrap().fuel, 30, "a Ship that fought no Battle is untouched");
    assert!(g.colony(ground).is_some(), "the Battery's Colony stands; a Battery has no tank and is skipped");
}

/// Ticket #346 (R2): a hull that could not pay fights at half strength, whichever side it is on;
/// and the charge and the penalty read the SAME tank, taken before the charge, so a hull that
/// could just pay fights whole in the Battle that empties it.
#[test]
fn a_hull_that_could_not_pay_fights_at_half_strength() {
    let charge = game().tables.melee.battle_fuel;
    let share = game().tables.melee.dry_strength_share;
    assert_eq!(share, 0.5, "[melee] dry_strength_share");
    // A dry Frigate on each side of one Battle: both are halved, aggressor and defender alike.
    let fight = |mine: i64, theirs: i64| -> (i64, i64) {
        let mut g = game();
        calm(&mut g);
        empty_orbit(&mut g, BodyId::Mars);
        let a = ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Mars, None, Stance::Attack);
        let d = ship_in(&mut g, Seat(1), UnitKind::Frigate, BodyId::Mars, None, Stance::Hold);
        g.ship_mut(a).unwrap().fuel = mine;
        g.ship_mut(d).unwrap().fuel = theirs;
        g.resolution_phase();
        let line = g.report.battles.iter().find(|b| b.at == Some(ReportPlace::Orbit(BodyId::Mars, Orbit::Low))).expect("a Battle in Mars low orbit");
        let of = |seat: Seat| line.parties.iter().find(|p| p.seat == Some(seat)).expect("a party").strength;
        (of(Seat(0)), of(Seat(1)))
    };
    assert_eq!(fight(30, 30), (3, 3), "two fuelled Frigates fight at the card's strength");
    assert_eq!(fight(charge - 1, 30), (1, 3), "the aggressor a Fuel short fights at floor(3 x 0.5)");
    assert_eq!(fight(30, charge - 1), (3, 1), "and so does the defender: the penalty does not care which side");
    assert_eq!(fight(0, 0), (1, 1), "a tank at nought is the same penalty as a tank one short");
    // The refutation: the charge and the penalty agree about which hulls were dry. A hull with
    // EXACTLY the charge fights whole in the Battle that empties it.
    assert_eq!(fight(charge, charge), (3, 3), "a hull that could just pay fights whole, and is dry for the NEXT Battle");
    // No test may assert a penalty on a Colony Ship, a Carrier or a Missile Carrier: this one
    // asserts its absence. Half of nought is nought.
    let mut g = game();
    calm(&mut g);
    empty_orbit(&mut g, BodyId::Mars);
    ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Mars, None, Stance::Attack);
    for kind in [UnitKind::ColonyShip, UnitKind::Carrier, UnitKind::MissileCarrier] {
        let id = ship_in(&mut g, Seat(1), kind, BodyId::Mars, None, Stance::Hold);
        g.ship_mut(id).unwrap().fuel = 0;
        let s = g.ship(id).unwrap();
        assert_eq!(g.ship_strength(s), 0, "{} has no strength to halve", kind.name());
        assert_eq!(g.ship_dry_strength(s), 0, "and dry it still has none");
    }
    g.resolution_phase();
    let line = g.report.battles.iter().find(|b| b.at == Some(ReportPlace::Orbit(BodyId::Mars, Orbit::Low))).expect("a Battle");
    let unarmed = line.parties.iter().find(|p| p.seat == Some(Seat(1))).expect("the unarmed party").strength;
    assert_eq!(unarmed, 0, "three unarmed hulls, dry, bring nought and are not penalised");
}

/// Ticket #346 (R3): the Fuel bar on a warship's work. Below `battle_fuel` a warship holds no
/// Orbital Control, contests no orbit, blockades nothing and intercepts nobody. The bar is the
/// Battle charge and not "more than nought", so a warship holds an orbit exactly as long as it
/// could still fight for it.
#[test]
fn a_dry_warship_holds_no_orbit_blockades_nothing_and_intercepts_nobody() {
    let mut g = game();
    calm(&mut g);
    empty_orbit(&mut g, BodyId::Mars);
    let charge = g.tables.melee.battle_fuel;
    let mine = ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Mars, None, Stance::Hold);
    // Door 1: Orbital Control. At the bar exactly it holds; one under it does not.
    g.ship_mut(mine).unwrap().fuel = charge;
    assert_eq!(g.orbital_control(BodyId::Mars), Some(Seat(0)), "a warship AT the bar holds Orbital Control");
    g.ship_mut(mine).unwrap().fuel = charge - 1;
    assert_eq!(g.orbital_control(BodyId::Mars), None, "one Fuel under it, and it holds nothing");
    // Door 2: contesting an orbit. A dry rival warship is not a rival warship for this test.
    let rival = ship_in(&mut g, Seat(1), UnitKind::Frigate, BodyId::Mars, None, Stance::Hold);
    g.ship_mut(mine).unwrap().fuel = 30;
    assert!(!g.orbit_uncontested(Seat(0), BodyId::Mars, Orbit::Low), "a fuelled rival contests the orbit");
    g.ship_mut(rival).unwrap().fuel = charge - 1;
    assert!(g.orbit_uncontested(Seat(0), BodyId::Mars, Orbit::Low), "a DRY rival contests nothing");
    assert_eq!(g.orbital_control(BodyId::Mars), Some(Seat(0)), "and the fuelled hull takes the Control it was denying");
    g.ships.retain(|s| s.id != rival);
    // Door 3: the Blockade. A station of seat 1's, and a warship of seat 0's at its ring.
    let station = station_at(&mut g, Seat(1), BodyId::Mars);
    let slot = g.colony(station).unwrap().slot;
    g.ship_mut(mine).unwrap().slot = Some(slot);
    g.ship_mut(mine).unwrap().stance = Stance::Blockade;
    assert!(g.slot_blockaded_against(Seat(1), BodyId::Mars, slot), "a fuelled warship on Blockade shuts the ring");
    g.ship_mut(mine).unwrap().fuel = charge - 1;
    assert!(!g.slot_blockaded_against(Seat(1), BodyId::Mars, slot), "a dry one shuts nothing");
    assert!(g.slot_blockaders(BodyId::Mars, slot).is_empty(), "and is no blockader of that slot");
    // And the order itself is refused while no warship of the seat's can pay for one.
    let err = g.check_order(Seat(0), &[], &Order::ShipStance { body: BodyId::Mars, stance: Stance::Blockade }).unwrap_err().0;
    assert!(err.contains("Fuel"), "the refusal says the tank is why: {err}");
    g.ships.retain(|s| s.id != mine);
    // Door 4: the Intercept. A dry picket catches nothing.
    let picket = ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Mars, None, Stance::Intercept);
    g.ship_mut(picket).unwrap().fuel = charge - 1;
    let inbound = ship_in(&mut g, Seat(1), UnitKind::ColonyShip, BodyId::Earth, None, Stance::Hold);
    g.ship_mut(inbound).unwrap().at = ShipAt::Transit { from: BodyId::Earth, to: BodyId::Mars, turns_left: 1 };
    g.resolution_phase();
    assert_eq!(g.war.interceptions[0], 0, "a picket under the bar intercepts nobody");
    // The same picket, fuelled, catches the same arrival.
    let mut g = game();
    calm(&mut g);
    empty_orbit(&mut g, BodyId::Mars);
    let picket = ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Mars, None, Stance::Intercept);
    g.ship_mut(picket).unwrap().fuel = charge;
    let inbound = ship_in(&mut g, Seat(1), UnitKind::ColonyShip, BodyId::Earth, None, Stance::Hold);
    g.ship_mut(inbound).unwrap().at = ShipAt::Transit { from: BodyId::Earth, to: BodyId::Mars, turns_left: 1 };
    g.resolution_phase();
    assert_eq!(g.war.interceptions[0], 1, "at the bar it catches it");
}

/// Ticket #346 (R3, read live): the bar is read off the tank at the moment it is asked, as Orbital
/// Control always has been. A fleet that spends its last Fuel winning a Battle loses the orbit it
/// just won, and a fresh Frigate arriving next turn takes it.
#[test]
fn a_fleet_that_spends_its_last_fuel_winning_a_battle_loses_the_orbit_it_won() {
    let mut g = game();
    calm(&mut g);
    empty_orbit(&mut g, BodyId::Mars);
    let charge = g.tables.melee.battle_fuel;
    let winner = ship_in(&mut g, Seat(0), UnitKind::Battleship, BodyId::Mars, None, Stance::Attack);
    g.ship_mut(winner).unwrap().fuel = charge;
    let loser = ship_in(&mut g, Seat(1), UnitKind::ColonyShip, BodyId::Mars, None, Stance::Hold);
    g.ship_mut(loser).unwrap().fuel = 30;
    assert_eq!(g.orbital_control(BodyId::Mars), Some(Seat(0)), "before the Battle it holds the orbit");
    g.resolution_phase();
    assert_eq!(g.ship(winner).expect("the Battleship lived").fuel, 0, "the Battle emptied its tank");
    assert_eq!(g.orbital_control(BodyId::Mars), None, "and it lost the orbit at that moment, not a turn later");
    assert_eq!(g.war.hulls_left_dry[0], 1, "the sweep counts the hull the Battle left dry");
    // A fresh Frigate arriving takes what the winner can no longer hold.
    ship_in(&mut g, Seat(2), UnitKind::Frigate, BodyId::Mars, None, Stance::Hold);
    assert_eq!(g.orbital_control(BodyId::Mars), Some(Seat(2)), "the fresh hull takes it");
}

/// Ticket #346 (R4): `stranded` is NOT changed. The zero-Fuel trap stays exactly where it was, at
/// the designer's word -- "for now its stranded" -- and this test pins it so a later ticket has to
/// change it deliberately. Not one line of `stranded` moves in this ticket.
#[test]
fn stranded_is_not_changed_by_the_battle_charge() {
    let mut g = game();
    calm(&mut g);
    // The MOON, whose cheapest leg out (6, to Earth) is three times the Battle bar: at Mars the
    // two figures happen to be equal, and a pin written there cannot tell them apart -- measured,
    // by wiring `stranded` to the bar on purpose and watching a Mars pin pass anyway.
    empty_orbit(&mut g, BodyId::Moon);
    let ship = ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Moon, None, Stance::Hold);
    let cheapest = g.cheapest_leg_from(Seat(0), BodyId::Moon).expect("a leg off the Moon");
    let bar = g.tables.melee.battle_fuel;
    assert!(cheapest > bar, "the Moon's cheapest leg ({cheapest}) is above the Battle bar ({bar}), so the two can be told apart");
    // A full tank flies home: nobody is stranded with fuel for the cheapest leg.
    assert!(!g.stranded(ship), "a full tank is not stranded");
    g.ship_mut(ship).unwrap().fuel = cheapest;
    assert!(!g.stranded(ship), "exactly the cheapest leg is not stranded");
    // EVERYTHING under the cheapest leg is stranded, with no station of its own -- including every
    // tank at or above the Battle bar, which is the pin: `stranded` reads the LEG and never the
    // bar, and a later ticket wiring the two together turns this red.
    for fuel in 0..cheapest {
        g.ship_mut(ship).unwrap().fuel = fuel;
        assert!(g.stranded(ship), "a tank of {fuel} cannot pay the cheapest leg of {cheapest}, so it is stranded");
    }
    // A station of its own IN ANOTHER ORBIT rescues it only while the tank can pay the orbit
    // change: at nought, with a station in sight, it is stranded. This is the trap the designer
    // left standing -- "for now its stranded" -- and it is pinned here so a later ticket moves it
    // on purpose.
    let station = station_at(&mut g, Seat(0), BodyId::Moon);
    let slot = g.colony(station).unwrap().slot;
    let change = g.tables.orbit_change_fuel;
    g.ship_mut(ship).unwrap().fuel = change;
    assert!(!g.stranded(ship), "with the orbit change in the tank the station rescues it");
    g.ship_mut(ship).unwrap().fuel = change - 1;
    assert!(g.stranded(ship), "a Fuel short of the orbit change, and the station in sight rescues nothing");
    // In the station's OWN orbit it is never stranded, at nought or at anything.
    g.ship_mut(ship).unwrap().slot = Some(slot);
    g.ship_mut(ship).unwrap().fuel = 0;
    assert!(!g.stranded(ship), "in the station's own ring an empty tank refuels");
}

/// Ticket #346 (R5): the computer weighs the Fuel a Battle would cost against the prize, and will
/// not open one that strands its fleet for nothing. A WEIGHT in `ai.toml`, never a prohibition.
#[test]
fn the_computer_weighs_the_fuel_a_battle_would_cost() {
    let weight = game().tables.ai.thresholds.battle_fuel_weight;
    assert!(weight > 0.0 && weight < 1.0, "a weight and not a prohibition: {weight}");
    let board = |fuel: i64, colony_of_mine: bool| -> Game {
        let mut g = game();
        calm(&mut g);
        empty_orbit(&mut g, BodyId::Mars);
        let mine = ship_in(&mut g, Seat(0), UnitKind::Battleship, BodyId::Mars, None, Stance::Hold);
        g.ship_mut(mine).unwrap().fuel = fuel;
        ship_in(&mut g, Seat(1), UnitKind::ColonyShip, BodyId::Mars, None, Stance::Hold);
        if colony_of_mine {
            colony(&mut g, Seat(0), BodyId::Mars, &[ModuleKind::Habitat], 4);
        }
        g.relations.score[0][1] = -8;
        g.relations.score[1][0] = -8;
        g
    };
    // Full tanks: nothing to weigh, the appetite is whole.
    let full = board(30, false);
    assert_eq!(full.ai_battle_fuel_weight(Seat(0), BodyId::Mars, Some(Orbit::Low)), 1.0, "a fleet that can pay and still fight weighs nothing against the attack");
    // Tanks at exactly the charge: paying it leaves the whole line under the bar, and the seat
    // holds nothing at the Body the orbit was wanted for.
    let charge = full.tables.melee.battle_fuel;
    let stranding = board(charge, false);
    assert_eq!(stranding.ai_battle_fuel_weight(Seat(0), BodyId::Mars, Some(Orbit::Low)), weight, "a Battle that strands the fleet for nothing is discounted");
    // The same fleet with a Colony of its own below: the orbit is the thing it came for, so it
    // pays at full appetite.
    let prize = board(charge, true);
    assert_eq!(prize.ai_battle_fuel_weight(Seat(0), BodyId::Mars, Some(Orbit::Low)), 1.0, "with a Colony below, the orbit is worth the tank");
    // And the discount reaches the orders: with the figure at 1.0 the seat opens the Battle, and
    // with the table's own figure it does not.
    let mut loosened = board(charge, false);
    std::sync::Arc::make_mut(&mut loosened.tables).ai.thresholds.battle_fuel_weight = 1.0;
    let orders = loosened.ai_orders(Seat(0));
    assert!(
        orders.iter().any(|o| matches!(o, Order::ShipStance { body: BodyId::Mars, stance: Stance::Attack })),
        "unweighed, the seat opens the Battle: {orders:?}"
    );
    let mut weighed = board(charge, false);
    let orders = weighed.ai_orders(Seat(0));
    assert!(
        !orders.iter().any(|o| matches!(o, Order::ShipStance { body: BodyId::Mars, stance: Stance::Attack })),
        "weighed, it does not strand its fleet for nothing: {orders:?}"
    );
}

/// Ticket #346 (R6): what the game says. The Battle's own line carries what the fight cost in Fuel
/// and names any hull that fought dry, out of `report.toml` and never a code literal.
#[test]
fn the_battle_line_says_what_the_battle_cost_in_fuel() {
    let mut g = game();
    calm(&mut g);
    empty_orbit(&mut g, BodyId::Mars);
    let charge = g.tables.melee.battle_fuel;
    ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Mars, None, Stance::Attack);
    let dry = ship_in(&mut g, Seat(1), UnitKind::Frigate, BodyId::Mars, None, Stance::Hold);
    g.ship_mut(dry).unwrap().fuel = charge - 1;
    let dry_name = g.ship(dry).unwrap().name.clone();
    g.resolution_phase();
    let line = g.report.battles.iter().find(|b| b.at == Some(ReportPlace::Orbit(BodyId::Mars, Orbit::Low))).expect("a Battle in Mars low orbit");
    assert!(line.result.contains(&format!("{} Fuel", charge * 2 - 1)), "the line says what the Battle cost in Fuel: {}", line.result);
    assert!(line.result.contains(&dry_name), "and names the hull that fought dry: {}", line.result);
    // Both sentences are templates in report.toml, so a change of wording is a change of data.
    assert!(g.tables.report.phrase("battle_fuel", &[("n", "7".to_string())]).contains('7'), "the cost is a [phrase] in report.toml");
    assert!(g.tables.report.phrase("battle_fought_dry", &[("hulls", "PMV Aurora".to_string())]).contains("PMV Aurora"), "and so is the dry hull's clause");
    // A Battle in which nothing fought dry says the cost and nothing else.
    let mut g = game();
    calm(&mut g);
    empty_orbit(&mut g, BodyId::Mars);
    ship_in(&mut g, Seat(0), UnitKind::Frigate, BodyId::Mars, None, Stance::Attack);
    ship_in(&mut g, Seat(1), UnitKind::Frigate, BodyId::Mars, None, Stance::Hold);
    g.resolution_phase();
    let line = g.report.battles.iter().find(|b| b.at == Some(ReportPlace::Orbit(BodyId::Mars, Orbit::Low))).expect("a Battle");
    assert!(line.result.contains(&format!("{} Fuel", charge * 2)), "two full tanks, the whole charge twice: {}", line.result);
    assert!(!line.result.contains("fought dry"), "and nothing fought dry: {}", line.result);
}

/// Ticket #346, the error cases the load check owns: `[melee]` without `battle_fuel` or without
/// `dry_strength_share` is a rule this build cannot price, and a share outside 0..1 is not a share.
/// The whole table is refused rather than quietly charging nothing.
#[test]
fn ticket_346_a_melee_without_its_fuel_figures_refuses_the_table() {
    let src = default_data_dir();
    let dir = std::env::temp_dir().join(format!("dying-earth-346-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a temporary data folder");
    for entry in std::fs::read_dir(&src).expect("the data folder") {
        let entry = entry.expect("a data file");
        if entry.path().is_file() {
            std::fs::copy(entry.path(), dir.join(entry.file_name())).expect("copy");
        }
    }
    assert!(Tables::load(&dir).is_ok(), "the copy loads before anything is taken out of it");
    let units = std::fs::read_to_string(dir.join("units.toml")).expect("units.toml");
    for figure in ["battle_fuel = 2", "dry_strength_share = 0.5"] {
        assert!(units.contains(figure), "[melee] carries {figure}");
        let stripped: String = units.lines().filter(|l| l.trim() != figure).collect::<Vec<_>>().join("\n");
        std::fs::write(dir.join("units.toml"), stripped).expect("write");
        let err = Tables::load(&dir).expect_err("a [melee] missing a Fuel figure is refused");
        let name = figure.split(' ').next().unwrap();
        assert!(format!("{err:?}").contains(name), "and the refusal names the missing figure: {err:?}");
    }
    // A share outside 0..1 is refused by the check, not by serde.
    let bad = units.replace("dry_strength_share = 0.5", "dry_strength_share = 1.5");
    std::fs::write(dir.join("units.toml"), bad).expect("write");
    let err = Tables::load(&dir).expect_err("a share above 1 is refused");
    assert!(format!("{err:?}").contains("dry_strength_share"), "{err:?}");
    let bad = units.replace("battle_fuel = 2", "battle_fuel = -1");
    std::fs::write(dir.join("units.toml"), bad).expect("write");
    let err = Tables::load(&dir).expect_err("a negative charge is refused");
    assert!(format!("{err:?}").contains("battle_fuel"), "{err:?}");
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------- ticket #353: seven untrue messages
//
// Version 0.09.1. Seven things the game SAYS that are not so. Nothing here is a rule: no figure
// moves and the sweep reads the same on every line. Each test was watched red against its own
// defect restored on purpose, because a message is exactly the kind of thing a suite does not
// notice, and a test never seen red certifies nothing.

/// Defect 1. A refusal names WHAT IS MISSING. "no Shipyard here" was said over a Shipyard standing
/// mothballed at the Colony the player was looking at, which is the plainest kind of lie the game
/// can tell. The three cases are three refusals, the shape `rearm_site` has used since #343.
#[test]
fn a_shipyard_refusal_says_whether_it_is_absent_shut_or_still_building() {
    let mut g = game();
    let cid = colony(&mut g, Seat(0), BodyId::Moon, &[], 4);
    let order = Order::BuildShip { site: Place::Colony(cid), kind: UnitKind::Frigate };
    // No Shipyard at all: the old sentence, which is true here and only here.
    let why = g.check_order(Seat(0), &[], &order).unwrap_err().0;
    assert_eq!(why, "no Shipyard here", "with no Shipyard the refusal names the absence");
    // One in the queue. It is not absent, it is not finished, and the refusal says which.
    g.colony_mut(cid).unwrap().queue.push(Build { item: BuildItem::Module(ModuleKind::Shipyard), seat: Seat(0), widgets: 6, done: 0, coastal: false });
    let why = g.check_order(Seat(0), &[], &order).unwrap_err().0;
    assert!(why.contains("still building"), "a Shipyard under way is not an absent one: {why}");
    assert!(!why.contains("no Shipyard"), "and the refusal never denies what the player can see in the queue: {why}");
    // Standing and mothballed: shut, not absent.
    g.colony_mut(cid).unwrap().queue.clear();
    g.colony_mut(cid).unwrap().modules.push(Module::new(ModuleKind::Shipyard));
    let last = g.colony_mut(cid).unwrap().modules.len() - 1;
    g.colony_mut(cid).unwrap().modules[last].mothballed = true;
    let why = g.check_order(Seat(0), &[], &order).unwrap_err().0;
    assert!(why.contains("shut"), "a mothballed Shipyard is shut, not absent: {why}");
    assert!(!why.contains("no Shipyard"), "it never says there is no Shipyard while one stands: {why}");
    // Standing and dark for want of Energy: shut too, and by the same sentence, which names both.
    g.colony_mut(cid).unwrap().modules[last].mothballed = false;
    g.colony_mut(cid).unwrap().modules[last].online = false;
    let dark = g.check_order(Seat(0), &[], &order).unwrap_err().0;
    assert_eq!(dark, why, "mothballed and dark are one refusal: the Shipyard is shut, and it names both reasons");
    // Working: the door is open. The Ship is paid for in full at the order, tank and all.
    g.colony_mut(cid).unwrap().modules[last].online = true;
    g.seats[0].stockpile.materials = 500;
    g.seats[0].stockpile.fuel = 500;
    assert!(g.check_order(Seat(0), &[], &order).is_ok(), "{:?}", g.check_order(Seat(0), &[], &order));
}

/// Defect 2. A slot is NAMED, not numbered. A player reading "in slot 3 on the Moon" has nothing to
/// click and no way to find the place; the slot has had a name since #45, and it is the Colony's own.
#[test]
fn a_founding_names_its_slot_and_never_numbers_it() {
    let mut g = game();
    calm(&mut g);
    let slot = g.free_slots_on(BodyId::Moon)[0];
    let name = g.tables.body(BodyId::Moon).slots[slot as usize].name.clone();
    let (_, found) = colony_ship_ready(&mut g, BodyId::Moon);
    let mut orders: [Vec<Order>; SEAT_COUNT] = std::array::from_fn(|_| Vec::new());
    orders[0] = vec![found];
    pick_a_tech(&mut g);
    answer_the_card(&mut g);
    g.end_turn(orders).expect("the turn should end");
    let founded: Vec<String> = g.report.lines.iter().filter(|l| l.kind == LineKind::ColonyFounded).map(|l| l.text.clone()).collect();
    assert!(!founded.is_empty(), "a Colony was founded this turn");
    assert!(founded.iter().any(|t| t.contains(&name)), "the founding names {name}: {founded:?}");
    for t in &founded {
        assert!(!t.contains("in slot "), "and never numbers the slot: {t}");
    }
    assert!(g.log.to_vec().iter().any(|l| l.contains(&format!("founded a Colony at {name}"))), "the log names it too");
}

/// The one phrase ticket #353 put at every clamp site, or `None` if this Report has none.
fn no_room_line(g: &Game) -> Option<String> {
    g.report.lines.iter().find(|l| l.text.contains("found no Habitat room")).map(|l| l.text.clone())
}

/// Defect 3. A partial unload SAYS SO. A Colony Ship carries more than the Core Module's four, the
/// engine quietly clamps, and nothing anywhere told the player who was left. All FOUR clamp sites,
/// one phrase: the specification named three and missed the disembarkation into a standing Colony.
#[test]
fn a_partial_unload_says_how_many_found_no_habitat_room() {
    // (a) A Colony Ship founding. The Core Module holds four and seven are aboard.
    let mut g = game();
    calm(&mut g);
    let slot = g.free_slots_on(BodyId::Moon)[0];
    let (ship, _) = colony_ship_ready(&mut g, BodyId::Moon);
    g.ship_mut(ship).unwrap().colonists = 7;
    let found = Order::Unload { ship, colonists: 7, army: false, into: UnloadTarget::Slot(BodyId::Moon, slot) };
    let mut orders: [Vec<Order>; SEAT_COUNT] = std::array::from_fn(|_| Vec::new());
    orders[0] = vec![found];
    pick_a_tech(&mut g);
    answer_the_card(&mut g);
    g.end_turn(orders).expect("the turn should end");
    assert_eq!(g.ship(ship).map(|s| s.colonists), Some(3), "three of the seven stayed aboard");
    let said = no_room_line(&g)
        .unwrap_or_else(|| panic!("a line saying who found no Habitat room: {:?}", g.report.lines.iter().map(|l| &l.text).collect::<Vec<_>>()));
    assert!(said.contains('3'), "it names the three who did not land: {said}");

    // (b) A Colony that already stands, with room for one of the three aboard. The fourth site.
    let mut g = game();
    calm(&mut g);
    let full = colony(&mut g, Seat(0), BodyId::Moon, &[], 3);
    let ship = a_colony_ship(&mut g, Seat(0), BodyId::Moon);
    g.ship_mut(ship).unwrap().colonists = 3;
    g.commit_orders(Seat(0), &[Order::Unload { ship, colonists: 3, army: false, into: UnloadTarget::Colony(full) }]);
    g.report.lines.clear();
    g.resolution_phase();
    let said = no_room_line(&g)
        .unwrap_or_else(|| panic!("a disembarkation clamps too: {:?}", g.report.lines.iter().map(|l| &l.text).collect::<Vec<_>>()));
    assert!(said.contains('2'), "one of the three landed and two did not: {said}");

    // (c) A founding by sea in Antarctica, six sent into a Core Module that holds four.
    let mut g = game();
    calm(&mut g);
    g.antarctica_open = true;
    let home = g.controlled_states(Seat(0))[0];
    let slot = g.free_slots_on(BodyId::Earth)[0];
    g.state_mut(home).emigrants = 6;
    g.commit_orders(Seat(0), &[Order::SendToAntarctica { state: home, n: 6, into: UnloadTarget::Slot(BodyId::Earth, slot) }]);
    g.resolution_phase();
    g.turn += 1;
    g.report.lines.clear();
    g.resolution_phase();
    let col = g.colonies.iter().find(|c| c.body == BodyId::Earth && !c.in_orbit).map(|c| c.id).expect("a Colony in Antarctica");
    let said = no_room_line(&g)
        .unwrap_or_else(|| panic!("a sea founding clamps too: {:?}", g.report.lines.iter().map(|l| &l.text).collect::<Vec<_>>()));
    assert!(said.contains('2'), "four landed and two came home: {said}");

    // (d) And a join at that same Colony, once a Habitat has widened it to four berths free.
    g.colony_mut(col).unwrap().modules.push(Module::new(ModuleKind::Habitat));
    let free = g.habitat_room(g.colony(col).unwrap()).saturating_sub(g.colony(col).unwrap().colonists);
    assert!(free > 0, "the Habitat made room: {free}");
    g.state_mut(home).emigrants = free + 2;
    g.commit_orders(Seat(0), &[Order::SendToAntarctica { state: home, n: free + 2, into: UnloadTarget::Colony(col) }]);
    g.resolution_phase();
    g.turn += 1;
    g.report.lines.clear();
    g.resolution_phase();
    let said = no_room_line(&g)
        .unwrap_or_else(|| panic!("a join clamps too: {:?}", g.report.lines.iter().map(|l| &l.text).collect::<Vec<_>>()));
    assert!(said.contains('2'), "the room was filled and two came home: {said}");
}

/// Defect 4. `lift` FILLS AS FAR AS THE ROOM GOES, which is what the headless driver's own help has
/// promised all along; it refused the whole order instead. Only a lift that would move nobody is
/// refused, and that refusal names the room.
#[test]
fn a_lift_to_a_station_fills_as_far_as_the_habitat_room_goes() {
    let mut g = game();
    calm(&mut g);
    bare_stations(&mut g);
    let iss = station_of(&g, Seat(0), BodyId::Earth).expect("the Custodians start with a station");
    let room = g.habitat_room(g.colony(iss).unwrap()).saturating_sub(g.colony(iss).unwrap().colonists);
    assert!(room > 0, "the station has room to fill: {room}");
    let home = g.controlled_states(Seat(0))[0];
    g.state_mut(home).emigrants = room + 3;
    let lift = Order::LiftToStation { state: home, n: room + 3, colony: iss };
    assert!(g.check_order(Seat(0), &[], &lift).is_ok(), "an order it can partly fill is not refused: {:?}", g.check_order(Seat(0), &[], &lift));
    let before = g.colony(iss).unwrap().colonists;
    g.commit_orders(Seat(0), std::slice::from_ref(&lift));
    assert_eq!(g.colony(iss).unwrap().colonists, before + room, "it filled the room and no more");
    assert_eq!(g.state(home).emigrants, 3, "the three who did not fit are still waiting at home");
    // Ticket #353, the designer's answer to "should a clamped lift say who stayed?": "yes". The
    // three who did not fit are standing in their Region and the Report says so. A partial unload
    // got this line in the same ticket; a partial lift is the same silence one step earlier.
    let said: Vec<String> = g.report.lines.iter().map(|l| l.text.clone()).collect();
    assert!(
        said.iter().any(|t| t.contains("still waiting in") && t.starts_with("3 more")),
        "a line names the Pioneers a clamped lift left behind: {said:?}"
    );
    // Room for nobody is not an order: it is refused, and the refusal names the room.
    let why = g.check_order(Seat(0), &[], &Order::LiftToStation { state: home, n: 3, colony: iss }).unwrap_err().0;
    assert!(why.contains("room for nobody"), "a lift that moves nobody is refused by name: {why}");
}

/// Defect 5. The Research Directive line NAMES WHAT IT BOUGHT. Three Factions in four were told to
/// have paid an Archive fund they do not have; the wording for what each of them really buys has
/// been in `report.toml` since #235.
#[test]
fn a_research_directive_deed_names_what_that_faction_bought() {
    let g = game();
    // Seat 0 is the Custodians, seat 1 the Prospectors, seat 2 the Arkwrights, seat 3 the Archivists.
    let set = Order::SetResearchDirective { percent: 40 };
    let custodians = g.rival_deed(Seat(0), &set).expect("a deed");
    assert!(custodians.contains("Natural Sink"), "the Custodians' Directive feeds the Sink: {custodians}");
    assert!(!custodians.contains("Archive"), "and never an Archive fund they cannot hold: {custodians}");
    let prospectors = g.rival_deed(Seat(1), &set).expect("a deed");
    assert!(prospectors.contains("coffers"), "the Prospectors' Directive pays Ducats: {prospectors}");
    assert!(!prospectors.contains("Archive"), "{prospectors}");
    let arkwrights = g.rival_deed(Seat(2), &set).expect("a deed");
    assert!(arkwrights.contains("propellant"), "the Arkwrights' Directive makes Fuel: {arkwrights}");
    assert!(!arkwrights.contains("Archive"), "{arkwrights}");
    let archivists = g.rival_deed(Seat(3), &set).expect("a deed");
    assert!(archivists.contains("Archive fund"), "and the Archivists' really does pay the Archive: {archivists}");
    // Nought sends the whole of it to the shared Tech, which is true of all four alike.
    for seat in Seat::ALL {
        let off = g.rival_deed(seat, &Order::SetResearchDirective { percent: 0 }).expect("a deed");
        assert!(off.contains("shared Tech"), "{off}");
    }
}

/// Defect 6. A turn whose loudest line is a CARD ANSWER still opens with a headline. The interface
/// filtered the answer out of `headline()` and got `None` for it, so the dispatch opened with
/// nothing at all. The answer is filed under a kind with no rank instead, and the next line by rank
/// falls through on its own.
#[test]
fn a_turn_whose_loudest_line_is_a_card_answer_still_opens_with_a_headline() {
    assert_eq!(LineKind::Card.headline_rank(), None, "the turn's card and its answers never headline");
    let mut g = game();
    // The engine files them there itself: every seat's answer, and the question that was asked.
    let card = EventId::ALL.into_iter().find(|id| g.tables.event(*id).asks()).expect("a card that asks");
    g.report.lines.clear();
    g.question = Some(Question { card, answers: [Some(CardAnswer::Refused); SEAT_COUNT] });
    g.apply_card_answers();
    assert_eq!(g.report.lines.len(), SEAT_COUNT, "four answers: {:?}", g.report.lines.iter().map(|l| &l.text).collect::<Vec<_>>());
    for l in &g.report.lines {
        assert_eq!(l.kind.headline_rank(), None, "an answer never headlines: {}", l.text);
    }
    assert!(g.report.headline().is_none(), "four answers and nothing else is a quiet turn");
    // A quiet turn that also completed a build opens with the build, not with silence.
    g.report_line(LineKind::BuildComplete, None, "A Factory was completed.".to_string());
    let head = g.report.headline().expect("the headline falls through to the next line by rank");
    assert_eq!(head.kind, LineKind::BuildComplete, "{}", head.text);
    // And a card the player has already read in its own modal does not open the Report either.
    let ordinary = EventId::ALL.into_iter().find(|id| !g.tables.event(*id).asks()).expect("a card that does not ask");
    g.report.lines.clear();
    g.draw = CardDraw::Ordinary(ordinary);
    g.event_phase();
    assert!(!g.report.lines.is_empty(), "the drawn card writes a Report line");
    for l in &g.report.lines {
        assert_eq!(l.kind.headline_rank(), None, "the drawn card was shown by its own modal: {}", l.text);
    }
    // And so does the question asked at the head of the turn, which the Card modal holds the screen
    // with until it is answered. The roll is a roll, so the deck is stacked and it is asked again
    // until a card comes.
    g.question = None;
    g.deck.off_earth_joined = true;
    stand_on_a_drawing_turn(&mut g);
    for _ in 0..500 {
        g.report.lines.clear();
        g.deck.cards = vec![Card::Event(card)];
        g.question_phase();
        if !g.report.lines.is_empty() {
            break;
        }
    }
    assert!(!g.report.lines.is_empty(), "a choice card was asked within 500 rolls");
    for l in &g.report.lines {
        assert_eq!(l.kind.headline_rank(), None, "the question its own modal already showed: {}", l.text);
    }
}

/// Defect 7. The headless driver's grammar says what the engine's refusal says. `build archive`
/// read "the Archivists only" and named none of the three rules the engine has enforced since #199.
#[test]
fn the_drivers_build_archive_entry_names_the_three_rules_the_engine_enforces() {
    let driver = include_str!("../examples/play.rs");
    let entry: String = driver.split("build archive <colony>").nth(1).expect("the driver has a `build archive` entry").lines().take(5).collect::<Vec<_>>().join(" ");
    assert!(entry.contains("Archivists"), "{entry}");
    assert!(entry.contains("The Upload"), "the Tech the engine's own refusal names: {entry}");
    assert!(entry.to_ascii_lowercase().contains("off earth"), "off Earth, which neither Antarctica nor a station over Earth is: {entry}");
    assert!(entry.to_ascii_lowercase().contains("four colonists"), "and the four Colonists who must already live there: {entry}");
}

// ---------------------------------------------------------------- Ticket #350: your own Condition

/// Ticket #350 (version 0.09.1): the turn-1 Report line said "get twelve Colonists off Earth" to
/// every Faction, which is half of the Custodians' and the Prospectors' Condition and wrong for the
/// Arkwrights and the Archivists. It now carries the player's own, in the designer's approved words.
fn turn_one_line(player: FactionKind) -> String {
    let t = tables();
    let start = t.faction(player).opens_on;
    let mut g = Game::new(t, NewGame { seed: 7, player, player_is_ai: false, player_start: start });
    g.start();
    g.report
        .lines
        .iter()
        .find(|l| l.text.starts_with("Your rivals are"))
        .map(|l| l.text.clone())
        .unwrap_or_else(|| panic!("no turn-1 rivals line: {:?}", g.report.lines))
}

#[test]
fn ticket_350_the_turn_one_line_names_each_factions_own_condition() {
    for (kind, words) in [
        (
            FactionKind::Custodians,
            "Build, spread Influence, and reach Stabilization, three Climate phases running with Emissions under the Natural Sink, with 12 Colonists living off Earth, before the Temperature reaches +3.0 C.",
        ),
        (FactionKind::Prospectors, "Build, spread Influence, and put 2,500 Ducats in the Venture Capital Fund with 12 Colonists living off Earth, before the Temperature reaches +3.0 C."),
        (FactionKind::Arkwrights, "Build, spread Influence, and get 30 Colonists living off Earth, spread over three Bodies, before the Temperature reaches +3.0 C."),
        (FactionKind::Archivists, "Build, spread Influence, and build the Archive off Earth, pay 125 Research into it, and upload 12 Colonists, before the Temperature reaches +3.0 C."),
    ] {
        let line = turn_one_line(kind);
        assert!(line.ends_with(words), "{kind:?}: {line}");
        assert!(!line.contains("twelve Colonists off Earth"), "{kind:?} is not told the old line: {line}");
    }
}

/// The figures are filled from the bars, never typed, so a moved bar moves the line -- which is the
/// whole of how the old line came to lie.
#[test]
fn ticket_350_a_moved_bar_moves_the_line() {
    let mut t = (*tables()).clone();
    t.factions[FactionKind::Prospectors as usize].victory_first.bar = 3000.0;
    t.factions[FactionKind::Prospectors as usize].victory_second.bar = 8.0;
    assert_eq!(t.victory_short(FactionKind::Prospectors), "put 3,000 Ducats in the Venture Capital Fund with eight Colonists living off Earth,");
    let start = t.faction(FactionKind::Prospectors).opens_on;
    let mut g = Game::new(Arc::new(t), NewGame { seed: 7, player: FactionKind::Prospectors, player_is_ai: false, player_start: start });
    g.start();
    assert!(g.report.lines.iter().any(|l| l.text.contains("put 3,000 Ducats")), "{:?}", g.report.lines);
}

#[test]
fn ticket_350_a_figure_reads_as_a_sentence_says_it() {
    use dying_earth_engine::data::figure;
    assert_eq!(figure(3.0), "three");
    assert_eq!(figure(0.0), "nought");
    assert_eq!(figure(12.0), "12");
    assert_eq!(figure(125.0), "125");
    assert_eq!(figure(2500.0), "2,500");
    assert_eq!(figure(1_250_000.0), "1,250,000");
}

/// The load check: a placeholder the Faction's own Condition cannot fill is refused, rather than a
/// brace printed to the player on turn 1.
#[test]
fn ticket_350_a_placeholder_the_condition_cannot_fill_refuses_the_table() {
    let src = default_data_dir();
    let dir = std::env::temp_dir().join(format!("dying-earth-350-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a temporary data folder");
    for entry in std::fs::read_dir(&src).expect("the data folder") {
        let entry = entry.expect("a data file");
        if entry.path().is_file() {
            std::fs::copy(entry.path(), dir.join(entry.file_name())).expect("copy");
        }
    }
    assert!(Tables::load(&dir).is_ok(), "the copy loads before anything is changed in it");
    let factions = std::fs::read_to_string(dir.join("factions.toml")).expect("factions.toml");
    assert!(factions.contains("spread over {bodies} Bodies"), "the Arkwrights' clause names its Bodies");
    // The Arkwrights' second part counts Bodies, not a bar, so {second} has nothing to fill it.
    std::fs::write(dir.join("factions.toml"), factions.replace("spread over {bodies} Bodies", "spread over {second} Bodies")).expect("write");
    let err = Tables::load(&dir).expect_err("an unfillable placeholder is refused");
    assert!(format!("{err:?}").contains("victory_short uses {second}"), "and the refusal names it: {err:?}");
    // And the other way: the Prospectors' second part is a bar, so {bodies} has nothing to fill it.
    assert!(factions.contains("put {first} Ducats"), "the Prospectors' clause names its Fund");
    std::fs::write(dir.join("factions.toml"), factions.replace("put {first} Ducats", "put {bodies} Ducats")).expect("write");
    let err = Tables::load(&dir).expect_err("a {bodies} with no Bodies is refused");
    assert!(format!("{err:?}").contains("victory_short uses {bodies}"), "and the refusal names it: {err:?}");
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------- Ticket #349: Pressed

/// Ticket #349 (version 0.09.1): seat 0 holds `place` at `mine`, every rival's Standing there is
/// cleared, and the given rivals stand where the test says.
fn held_at(g: &mut Game, place: Place, mine: i64, rivals: &[(Seat, i64)]) {
    for s in Seat::ALL {
        g.seats[s.0 as usize].influence.remove(&place);
    }
    g.seats[0].influence.insert(place, mine);
    for (s, v) in rivals {
        g.seats[s.0 as usize].influence.insert(place, *v);
    }
    if let Place::State(id) = place {
        g.state_mut(id).control = Control::Controlled(Seat(0));
    }
}

#[test]
fn ticket_349_a_rival_within_ten_of_your_standing_presses_the_place() {
    let mut g = game();
    let africa = Place::State(StateId::NorthAfrica);
    held_at(&mut g, africa, 50, &[(Seat(1), 40)]);
    assert!(g.pressed(africa), "40 is within 10 of 50");
    held_at(&mut g, africa, 50, &[(Seat(1), 39)]);
    assert!(!g.pressed(africa), "39 is not");
    held_at(&mut g, africa, 50, &[(Seat(1), 60)]);
    assert!(g.pressed(africa), "a rival above you presses it too");
    held_at(&mut g, africa, 50, &[]);
    assert!(!g.pressed(africa), "no rival, nothing pressing");
}

/// The case the card's old test missed: it tested only `nearest_challenger`, which is the rival
/// nearest its OWN price. On South America (threshold 50) held at 25, the threshold is the price:
/// seat 1 carries all the Blame, so its threshold is 75, and it stands at 16 -- within 10 of you, 59
/// short. Seat 2 carries none, stands at 14 -- outside the band -- and is only 36 short, so it is
/// the one `nearest_challenger` names. The old card test read seat 2 and stayed quiet.
#[test]
fn ticket_349_every_rival_is_tested_not_only_the_nearest_to_its_price() {
    let mut g = game();
    let south_america = Place::State(StateId::SouthAmerica);
    for s in &mut g.seats {
        s.blame_emitted = 0.0;
    }
    g.seats[1].blame_emitted = 100.0;
    held_at(&mut g, south_america, 25, &[(Seat(1), 16), (Seat(2), 14)]);
    assert!(g.blame_threshold_multiplier_on(Seat(1), south_america) > 1.4, "seat 1's Blame raises its price here");
    let (nearest, theirs, _) = g.nearest_challenger(south_america).expect("a challenger");
    assert_eq!((nearest, theirs), (Seat(2), 14), "the rival nearest its own price is seat 2, outside the band");
    assert!(g.pressed(south_america), "and seat 1 presses it all the same");
    held_at(&mut g, south_america, 25, &[(Seat(2), 14)]);
    assert!(!g.pressed(south_america), "without seat 1, nothing presses");
}

#[test]
fn ticket_349_a_place_nobody_holds_is_never_pressed() {
    let mut g = game();
    let africa = Place::State(StateId::NorthAfrica);
    held_at(&mut g, africa, 50, &[(Seat(1), 45)]);
    g.state_mut(StateId::NorthAfrica).control = Control::Neutral;
    assert!(!g.pressed(africa));
    // A holder with no Standing at all is pressed by any rival with some, but not by a zero.
    held_at(&mut g, africa, 0, &[(Seat(1), 0)]);
    assert!(!g.pressed(africa), "a rival at nought never presses");
}

#[test]
fn ticket_349_the_list_is_every_pressed_place_held_in_a_fixed_order() {
    let mut g = game();
    for s in &mut g.states {
        if s.control == Control::Controlled(Seat(0)) {
            s.control = Control::Neutral;
        }
    }
    let (a, b, c) = (Place::State(StateId::SouthAmerica), Place::State(StateId::NorthAfrica), Place::State(StateId::CentralAmerica));
    held_at(&mut g, a, 50, &[(Seat(3), 45)]);
    held_at(&mut g, b, 50, &[(Seat(1), 42)]);
    held_at(&mut g, c, 50, &[(Seat(1), 10)]);
    assert_eq!(g.pressed_places(Seat(0)), vec![b, a], "North Africa and South America, in id order; Central America is not pressed");
    assert!(g.pressed_places(Seat(1)).iter().all(|p| !matches!(p, Place::State(StateId::NorthAfrica | StateId::SouthAmerica))), "a rival is not told of places it does not hold");
}

/// The band is a figure in the data, not a number in the code.
#[test]
fn ticket_349_the_band_is_read_from_the_data() {
    assert_eq!(tables().influence.pressed_band, 10);
    let mut t = (*tables()).clone();
    t.influence.pressed_band = 12;
    let mut g = Game::new(Arc::new(t), NewGame { seed: 7, player: FactionKind::Custodians, player_is_ai: false, player_start: StateId::EastAsia });
    let africa = Place::State(StateId::NorthAfrica);
    held_at(&mut g, africa, 50, &[(Seat(1), 39)]);
    assert!(g.pressed(africa), "39 is within 12 of 50");
}

/// A Colony is a place like a Region: Pressed on the same test, and listed after the Regions.
#[test]
fn ticket_349_a_colony_is_pressed_on_the_same_test_and_listed_after_the_regions() {
    let mut g = game();
    for s in &mut g.states {
        if s.control == Control::Controlled(Seat(0)) {
            s.control = Control::Neutral;
        }
    }
    g.colonies.retain(|c| c.control.controller() != Some(Seat(0)));
    let moon = Place::Colony(colony(&mut g, Seat(0), BodyId::Moon, &[ModuleKind::Habitat], 4));
    held_at(&mut g, moon, 50, &[(Seat(2), 39)]);
    assert!(!g.pressed(moon), "39 is outside the band on a Colony too");
    held_at(&mut g, moon, 50, &[(Seat(2), 40)]);
    assert!(g.pressed(moon));
    let africa = Place::State(StateId::NorthAfrica);
    held_at(&mut g, africa, 50, &[(Seat(1), 45)]);
    assert_eq!(g.pressed_places(Seat(0)), vec![africa, moon], "the Region first, then the Colony");
    // A Colony a rival holds is not seat 0's to be warned of.
    g.colonies.iter_mut().find(|c| Place::Colony(c.id) == moon).expect("the Colony").control = Control::Controlled(Seat(2));
    assert_eq!(g.pressed_places(Seat(0)), vec![africa]);
}

// ---------------------------------------------------------------- Ticket #351: the Shortfall forecast

/// Ticket #351 (version 0.09.1): seat 0 in East Asia with a Factory (2), a Refinery (3) and a
/// Research Lab (3) and nothing else that costs Energy but the station's Core, and `energy` stored.
fn short_board(energy: i64) -> Game {
    let mut g = game();
    g.seats[0].stockpile.energy = energy;
    let st = g.state_mut(StateId::EastAsia);
    st.facilities.clear();
    st.facilities.push(facility(FacilityKind::Factory));
    st.facilities.push(facility(FacilityKind::Refinery));
    st.facilities.push(facility(FacilityKind::ResearchLab));
    g
}

#[test]
fn ticket_351_the_forecast_names_the_deficit_and_what_goes_dark_in_order_and_where() {
    let g = short_board(4);
    let f = g.shortfall_forecast(Seat(0), &[]).expect("4 stored against 8 and the Core is short");
    // 4 - 8 - 1 (the Core) = -5.
    assert_eq!(f.short_by, 5);
    let dark: Vec<(String, String)> = f.dark.iter().map(|d| (d.name.clone(), d.at.clone())).collect();
    assert_eq!(dark, vec![("Refinery".to_string(), "in China".to_string()), ("Research Lab".to_string(), "in China".to_string())]);
    assert_eq!(f.dark.iter().map(|d| d.name.as_str()).collect::<Vec<_>>(), g.shortfall_order(Seat(0)), "the same order the Income rule shuts in");
    assert!(short_board(20).shortfall_forecast(Seat(0), &[]).is_none(), "a full store covers the bill: no alarm");
}

/// The forecast reads the Energy left after this turn's orders, as the top bar's figure does. No
/// order spends Energy and it cannot be sold, so the order that moves it is a purchase.
#[test]
fn ticket_351_the_forecast_counts_this_turns_orders() {
    let g = short_board(4);
    let buy = |amount| Order::Buy { resource: Resource::Energy, amount };
    let f = g.shortfall_forecast(Seat(0), &[buy(3)]).expect("7 against 9 is still short");
    assert_eq!(f.short_by, 2, "the purchase counts");
    assert_eq!(f.dark.len(), 1, "and one building is enough now");
    assert!(g.shortfall_forecast(Seat(0), &[buy(10)]).is_none(), "buying 10 clears it");
}


/// The Report line after the fact names the Sink's loss when a Scrubber was shut.
#[test]
fn ticket_351_the_report_line_names_the_sinks_loss() {
    let mut g = short_board(0);
    let st = g.state_mut(StateId::EastAsia);
    st.facilities.clear();
    st.facilities.push(facility(FacilityKind::Scrubber));
    st.facilities.push(facility(FacilityKind::Scrubber));
    g.income_phase();
    let line = g.report.lines.iter().find(|l| l.text.contains("Energy ran short")).map(|l| l.text.clone()).unwrap_or_else(|| panic!("{:?}", g.report.lines));
    assert!(line.contains("the Natural Sink loses 6.0 ppm"), "two Scrubbers at 3.0 each: {line}");
    let mut g = short_board(0);
    g.income_phase();
    let line = g.report.lines.iter().find(|l| l.text.contains("Energy ran short")).map(|l| l.text.clone()).expect("a shortfall");
    assert!(!line.contains("Natural Sink"), "no Scrubber, no Sink clause: {line}");
}

/// Found in review: the Sink counts a Scrubber only where its Region has a controller, so one shut in
/// a Region occupied from neutral costs the Sink nothing and the Report line must not say otherwise.
#[test]
fn ticket_351_a_scrubber_the_sink_never_counted_names_no_loss() {
    let mut g = short_board(0);
    let st = g.state_mut(StateId::EastAsia);
    st.facilities.clear();
    st.facilities.push(facility(FacilityKind::Scrubber));
    st.control = Control::Occupied { occupier: Seat(0), previous: None, turns: 1, banked: 0 };
    assert_eq!(g.scrubber_removal(), 0.0, "the Sink does not count it");
    g.income_phase();
    let line = g.report.lines.iter().find(|l| l.text.contains("Energy ran short")).map(|l| l.text.clone()).unwrap_or_else(|| panic!("{:?}", g.report.lines));
    assert!(line.contains("Scrubber"), "it is shut: {line}");
    assert!(!line.contains("Natural Sink"), "and costs the Sink nothing: {line}");
}

/// Ticket #367 (version 0.09.2): **no card of either kind on the first turn.** The deck is not
/// touched before `first_draw_turn`, so the card on top waits for the first roll and nothing is
/// lost. Sixty seeds, because the roll is a coin at the base Temperature: before the rule about
/// thirty of them drew on turn 1, and one is enough to fail this.
#[test]
fn no_card_of_either_kind_is_drawn_on_the_first_turn_and_the_deck_is_untouched() {
    let t = tables();
    assert_eq!(t.events.first_draw_turn, 2, "the figure in events.toml");
    for seed in 1..=60 {
        let mut g = with_seed(seed);
        let dealt = g.deck.cards.len();
        g.start();
        assert_eq!(g.turn, 1);
        assert_eq!(g.draw, CardDraw::NoCard, "seed {seed}: turn 1 drew {:?}", g.draw);
        assert!(g.pending_question().is_none(), "seed {seed}: turn 1 asked a question");
        assert_eq!(g.deck.cards.len(), dealt, "seed {seed}: the deck is untouched on turn 1");
        assert!(g.deck.drawn.is_empty(), "seed {seed}: nothing is in the drawn pile");
    }
    // And on the first turn that may draw, the roll is the ordinary one: over sixty seeds some
    // draw and some do not, so the rule holds off turn 1 alone and does not silence the deck.
    let mut drew = 0;
    for seed in 1..=60 {
        let mut g = with_seed(seed);
        g.turn = t.events.first_draw_turn;
        g.question_phase();
        if g.draw != CardDraw::NoCard {
            drew += 1;
        }
    }
    assert!((10..=50).contains(&drew), "turn {}: {drew} of 60 seeds drew, which is not a coin", t.events.first_draw_turn);
}

/// Ticket #370 (version 0.09.2): **the player's Colonists still aboard off Earth are reported every
/// turn they wait**, one line a Body, under Ships; a rival's are not; and the line says when a
/// rival's Orbital Control stops the landing.
#[test]
fn colonists_waiting_aboard_off_earth_are_reported_every_turn_under_ships() {
    let mut g = fresh();
    g.start();
    let waiting = |g: &Game| g.report.lines.iter().filter(|l| l.text.contains("wait aboard")).cloned().collect::<Vec<_>>();
    // A Colony Ship of the player's with four aboard in low orbit of the Moon, never unloaded.
    let (_, _) = colony_ship_ready(&mut g, BodyId::Moon);
    // And a rival's, the same, over Mars: not the player's business.
    let (theirs, _) = colony_ship_ready(&mut g, BodyId::Mars);
    g.ships.iter_mut().find(|s| s.id == theirs).unwrap().seat = Seat(1);
    let quiet_turn = |g: &mut Game| {
        pick_a_tech(g);
        answer_the_card(g);
        g.end_turn(std::array::from_fn(|_| Vec::new())).expect("the turn should end");
    };
    quiet_turn(&mut g);
    let lines = waiting(&g);
    assert_eq!(lines.len(), 1, "one line, the player's, not the rival's: {lines:?}");
    assert_eq!(lines[0].text, "4 Colonists wait aboard in low orbit of the Moon.");
    assert_eq!(lines[0].kind, LineKind::Ship);
    assert_eq!(lines[0].place, Some(ReportPlace::Body(BodyId::Moon)), "the line points at the Body");
    assert_eq!(lines[0].section(), Section::Ships, "under Ships");
    assert_eq!(lines[0].kind.headline_rank(), None, "never the headline");
    // Every turn they wait, not only the first.
    quiet_turn(&mut g);
    assert_eq!(waiting(&g).len(), 1, "said again the next turn");
    // A rival warship takes Orbital Control of the Moon, and the line says the landing is blocked.
    // The Resolution is run by hand, since the computer plays seat 1 and would order the frigate
    // elsewhere in a whole turn; the Report is cleared first, as a new turn clears it.
    ship_in(&mut g, Seat(1), UnitKind::Frigate, BodyId::Moon, None, Stance::Hold);
    assert_eq!(g.orbital_control(BodyId::Moon), Some(Seat(1)), "the fixture: the rival holds the orbit");
    g.report.lines.clear();
    g.resolution_phase();
    let lines = waiting(&g);
    assert_eq!(lines.len(), 1, "{lines:?}");
    assert_eq!(lines[0].text, "4 Colonists wait aboard in low orbit of the Moon, blocked by rivals' control of the orbit.");
}

/// Ticket #370: **every Ship line reads under Ships**, a heading of its own above Your works, where
/// it read under In space before; the four other headings keep their order.
#[test]
fn ship_lines_read_under_ships_above_your_works() {
    assert_eq!(LineKind::Ship.section(Some(ReportPlace::Body(BodyId::Mars))), Section::Ships);
    assert_eq!(LineKind::Ship.section(None), Section::Ships);
    assert_eq!(Section::ALL, [Section::InSpace, Section::OnEarth, Section::TheClimate, Section::Ships, Section::YourWorks]);
    assert_eq!(Section::Ships.name_for(false), "Ships");
    assert_eq!(Section::Ships.name_for(true), "Ships");
    // A Ship's arrival, the oldest Ship line, files there through a whole turn.
    let mut g = fresh();
    g.start();
    let (id, _) = colony_ship_ready(&mut g, BodyId::Earth);
    let mut orders: [Vec<Order>; SEAT_COUNT] = std::array::from_fn(|_| Vec::new());
    orders[0] = vec![Order::Transit { ship: id, to: BodyId::Moon, slot: None }];
    pick_a_tech(&mut g);
    answer_the_card(&mut g);
    g.end_turn(orders).expect("the turn should end");
    // A Report lives one turn, so the arrival is read on the turn it happens.
    let mut arrived = None;
    for _ in 0..9 {
        if let Some(l) = g.report.lines.iter().find(|l| l.text.contains("arrived at")) {
            arrived = Some(l.clone());
            break;
        }
        pick_a_tech(&mut g);
        answer_the_card(&mut g);
        g.end_turn(std::array::from_fn(|_| Vec::new())).expect("the turn should end");
    }
    let arrived = arrived.expect("the Colony Ship arrived at the Moon within nine turns");
    assert_eq!(arrived.section(), Section::Ships, "{}", arrived.text);
}

/// Ticket #367: the loader refuses a `first_draw_turn` of nought, since turn 0 is not a turn and
/// the figure would read as "draw before the game starts". Read from a copy of the data folder
/// with that one line changed, so the test is about the loader and not about a hand-built table.
#[test]
fn the_loader_refuses_a_first_draw_turn_of_nought() {
    let src = default_data_dir();
    let dir = std::env::temp_dir().join(format!("dying-earth-first-draw-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp data dir");
    for entry in std::fs::read_dir(&src).expect("data dir") {
        let path = entry.expect("entry").path();
        if path.is_file() {
            std::fs::copy(&path, dir.join(path.file_name().unwrap())).expect("copy");
        }
    }
    assert!(Tables::load(&dir).is_ok(), "the copy loads before anything is changed");
    let events = std::fs::read_to_string(src.join("events.toml")).expect("events.toml");
    assert!(events.contains("first_draw_turn = 2"), "the fixture reads the shipped figure");
    std::fs::write(dir.join("events.toml"), events.replace("first_draw_turn = 2", "first_draw_turn = 0")).expect("write");
    let err = Tables::load(&dir).err().map(|e| e.to_string());
    // And 1, the rule as it stood before this ticket (a draw on the first turn), is accepted, as
    // the refusal's own text promises.
    std::fs::write(dir.join("events.toml"), events.replace("first_draw_turn = 2", "first_draw_turn = 1")).expect("write");
    let one = Tables::load(&dir).map(|t| t.events.first_draw_turn);
    let _ = std::fs::remove_dir_all(&dir);
    let err = err.expect("a first_draw_turn of 0 is refused");
    assert!(err.contains("first_draw_turn"), "the refusal names the figure: {err}");
    assert_eq!(one.ok(), Some(1), "a first_draw_turn of 1 loads");
}

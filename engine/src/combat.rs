//! The battle algorithm (spec 10). One algorithm serves space and ground.
//! It works on plain combatants so tests can drive it with fixed dice.
//!
//! Ticket #50: a Battle is a melee. Every Faction present at the place is hostile to every other,
//! so the algorithm takes N parties rather than two sides. The two-party case reduces exactly to the
//! old one: with parties a and d, the attacker's chance to land a hit is a / (a + d).

use crate::orders::UnitRef;
use crate::UnitKind;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use rand_chacha::ChaCha8Rng;

/// The randomness a battle needs, so a test can script it.
pub trait Dice {
    /// True with probability `p`.
    fn chance(&mut self, p: f64) -> bool;
    fn d6(&mut self) -> u32;
    /// A uniform index below `n` (n > 0).
    fn pick(&mut self, n: usize) -> usize;
}

impl<R: Rng> Dice for R {
    fn chance(&mut self, p: f64) -> bool {
        if p <= 0.0 {
            return false;
        }
        if p >= 1.0 {
            return true;
        }
        self.random::<f64>() < p
    }
    fn d6(&mut self) -> u32 {
        self.random_range(1..=6)
    }
    fn pick(&mut self, n: usize) -> usize {
        self.random_range(0..n)
    }
}

#[derive(Debug, Clone)]
pub struct Combatant {
    pub unit: UnitRef,
    pub name: String,
    pub strength: i64,
    pub hit_points: u32,
    pub damage: u32,
    pub pursuit: u32,
    pub evade: bool,
    pub engaged: bool,
    pub escaped: bool,
    /// Set once the enemy's pursuer has had its chance at this unit.
    pub pursued: bool,
    /// Ticket #297 (version 0.08.6): dug in, it never rolls to disengage. Its defence bonus is
    /// already in `strength`; the caller adds it.
    pub dug_in: bool,
    /// Ticket #326 (version 0.08.8): a warship, a Battery or an Army; a Colony Ship and a Carrier
    /// are not. While a party has an engaged armed unit, hits on the party land on its armed units
    /// and its unarmed ones are neither struck nor pursued: the escort takes the fire. Set by the
    /// caller from the hull, since the melee does not know kinds; true unless it says otherwise.
    pub armed: bool,
    /// Ticket #381 (version 0.09.2): what the unit is, for the picture of the Battle. A hull is
    /// set by the caller from its card; an Army and a Battery are read off the reference.
    pub kind: BattleUnit,
}

impl Combatant {
    pub fn new(unit: UnitRef, name: impl Into<String>, strength: i64, hit_points: u32, damage: u32, pursuit: u32, evade: bool) -> Combatant {
        let kind = match unit {
            UnitRef::Ship(_) => BattleUnit::Ship(UnitKind::Frigate),
            UnitRef::Army(_) => BattleUnit::Army,
            UnitRef::Battery { .. } => BattleUnit::Battery,
        };
        Combatant { unit, name: name.into(), strength, hit_points, damage, pursuit, evade, engaged: true, escaped: false, pursued: false, dug_in: false, armed: true, kind }
    }

    /// Ticket #381: the same unit, of this kind -- a hull's caller says which hull.
    pub fn kind(mut self, kind: BattleUnit) -> Combatant {
        self.kind = kind;
        self
    }

    /// Ticket #297: the same unit, dug in.
    pub fn dug_in(mut self, dug_in: bool) -> Combatant {
        self.dug_in = dug_in;
        self
    }

    /// Ticket #326: the same unit, armed or not.
    pub fn armed(mut self, armed: bool) -> Combatant {
        self.armed = armed;
        self
    }
}

/// Ticket #326: whether the party still has an armed unit engaged, which is what covers its
/// unarmed ones.
fn escorted(side: &[Combatant]) -> bool {
    side.iter().any(|c| c.engaged && !c.destroyed() && c.armed)
}

impl Combatant {
    pub fn destroyed(&self) -> bool {
        self.damage >= self.hit_points
    }
}

/// What one battle did, per party, in the order the parties were given.
#[derive(Debug, Clone, Default)]
pub struct BattleStats {
    pub rounds: u32,
    /// Hits each party landed on the others.
    pub hits: Vec<u32>,
    pub destroyed: Vec<Vec<String>>,
    pub escaped: Vec<Vec<String>>,
    /// Ticket #381 (version 0.09.2): the blow-by-blow record.
    pub log: BattleLog,
}

/// Ticket #381 (version 0.09.2): what a unit of a Battle is, for the picture the Report draws of
/// it. The Report is drawn after the destroyed are gone from the board, so the record carries the
/// kind itself rather than a reference to look it up by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BattleUnit {
    Ship(UnitKind),
    Army,
    Battery,
}

/// Ticket #381 (version 0.09.2): **the round log**, the blow-by-blow record of a Battle, at the
/// designer's word: *"what happens when a ship attacks a station or another ship and can we show
/// it."* Until this ticket the melee mutated its units in place and kept totals alone -- rounds,
/// hits per party, the destroyed and the escaped -- so nothing could show HOW a Battle went, only
/// how it ended, and the one question a player could not answer was why they lost. Every unit is
/// named by its party and its place in the line, which is how the picture finds it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BattleLog {
    /// The line as it stood before the first blow, party by party, in the parties' order.
    pub parties: Vec<Vec<LogUnit>>,
    /// The opening -- the Evade rolls and their pursuit, before any exchange -- and then one entry
    /// per round fought.
    pub rounds: Vec<LogRound>,
}

/// One unit as the Battle opened.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogUnit {
    pub name: String,
    pub kind: BattleUnit,
    pub hit_points: u32,
    /// The damage it brought to the Battle.
    pub damage: u32,
    pub armed: bool,
}

/// One round of the log: what was struck, who left and was chased, and where every unit stood
/// when it was over.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogRound {
    /// True for the opening, in which only Evade rolls and their pursuit happen.
    pub opening: bool,
    /// The hits of the exchange, in the order they landed.
    pub blows: Vec<Blow>,
    /// The units that disengaged this round, by party and place in the line.
    pub left: Vec<(usize, usize)>,
    /// The pursuit's hits on the leavers.
    pub chased: Vec<Blow>,
    /// Every unit's state when the round was over, party by party.
    pub after: Vec<Vec<UnitState>>,
}

/// One hit: the party that landed it, and the unit it landed on. `covering` is the escort rule at
/// work -- the party struck still had an unarmed hull engaged, and the hit went to an armed one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Blow {
    pub by: usize,
    pub party: usize,
    pub unit: usize,
    pub covering: bool,
}

/// Where one unit stood at the end of a round.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnitState {
    pub damage: u32,
    pub engaged: bool,
    pub escaped: bool,
    pub destroyed: bool,
}

/// Ticket #381: every unit's state now, party by party.
fn snapshot(parties: &[&mut [Combatant]]) -> Vec<Vec<UnitState>> {
    parties.iter().map(|p| p.iter().map(|c| UnitState { damage: c.damage, engaged: c.engaged, escaped: c.escaped, destroyed: c.destroyed() }).collect()).collect()
}

/// Ticket #381: whether a hit on this party is the escort taking it -- an unarmed unit of the
/// party stands engaged behind an armed one.
fn covering(side: &[Combatant]) -> bool {
    escorted(side) && side.iter().any(|c| c.engaged && !c.destroyed() && !c.armed)
}

impl BattleStats {
    pub fn all_destroyed(&self) -> Vec<String> {
        self.destroyed.iter().flatten().cloned().collect()
    }
    pub fn all_escaped(&self) -> Vec<String> {
        self.escaped.iter().flatten().cloned().collect()
    }
    /// The hits one party landed, 0 if there is no such party.
    pub fn hits_of(&self, party: usize) -> u32 {
        self.hits.get(party).copied().unwrap_or(0)
    }
}

/// The First Playable's figures, kept for the two-sided callers and the tests; the game reads
/// its own from `units.toml [melee]` since ticket #327 (version 0.08.8).
pub const MAX_ROUNDS: u32 = 3;
pub const HIT_ROLLS: u32 = 3;

fn total_strength(side: &[Combatant]) -> i64 {
    side.iter().filter(|c| c.engaged && !c.destroyed()).map(|c| c.strength).sum()
}

fn any_engaged(side: &[Combatant]) -> bool {
    side.iter().any(|c| c.engaged && !c.destroyed())
}

/// Ticket #326 (version 0.08.8): drawn uniformly among the party's engaged ARMED units while it
/// has any; its unarmed hulls are struck only when no armed unit of its remains engaged.
fn random_engaged(side: &[Combatant], dice: &mut dyn Dice) -> Option<usize> {
    let covered = escorted(side);
    let idx: Vec<usize> = side.iter().enumerate().filter(|(_, c)| c.engaged && !c.destroyed() && (c.armed || !covered)).map(|(i, _)| i).collect();
    if idx.is_empty() {
        None
    } else {
        Some(idx[dice.pick(idx.len())])
    }
}

/// Ticket #50: one party's chance to land a hit is its share of the total strength present.
/// With two parties this is the attacker's old p = A / (A + D).
pub fn hit_share(strengths: &[i64], party: usize) -> f64 {
    let total: i64 = strengths.iter().sum();
    if total <= 0 {
        return 0.0;
    }
    strengths.get(party).copied().unwrap_or(0) as f64 / total as f64
}

/// Ticket #50: a party's hits are spread across the enemy parties in proportion to the strength
/// each has present. The attacking party's own entry is zero; with one enemy it is a certainty.
pub fn target_shares(strengths: &[i64], attacker: usize) -> Vec<f64> {
    let enemy_total: i64 = strengths.iter().enumerate().filter(|(i, _)| *i != attacker).map(|(_, s)| *s).sum();
    strengths
        .iter()
        .enumerate()
        .map(|(i, s)| {
            if i == attacker || enemy_total <= 0 {
                0.0
            } else {
                *s as f64 / enemy_total as f64
            }
        })
        .collect()
}

/// Ticket #339 (version 0.09.0): **the chance of winning the BATTLE**, where `first_round_odds`
/// below is the chance of winning its first exchange. `PLAYTEST.txt` asked the testers by name
/// whether they knew what the older number meant, and since ticket #335 put a Battle in every orbit
/// it is on screen oftener than ever.
///
/// It is MEASURED rather than derived: the melee has disengage rolls, pursuit, an escort rule and
/// hit points, so there is no closed form to write down and no small state space to enumerate. The
/// parties are copied and fought `trials` times from `seed`, which is the figure's own seed and
/// never the game's, so the answer is the same every time it is asked and asking it cannot move a
/// seeded game by a single roll.
///
/// What counts as a win is what the board counts: `party` **holds the field** -- a unit of its
/// neither destroyed nor escaped, and nobody else's left standing on the place. That is exactly the
/// test `alone_at` makes when it begins an Occupation.
pub fn whole_battle_odds(parties: &[Vec<Combatant>], party: usize, divisor: f64, rounds: u32, rolls: u32, trials: u32, seed: u64) -> f64 {
    if trials == 0 || parties.len() < 2 || party >= parties.len() {
        return 0.0;
    }
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let mut won = 0u32;
    for _ in 0..trials {
        let mut copy: Vec<Vec<Combatant>> = parties.to_vec();
        {
            let mut slices: Vec<&mut [Combatant]> = copy.iter_mut().map(|p| p.as_mut_slice()).collect();
            melee(&mut slices, &mut rng as &mut dyn Dice, divisor, rounds, rolls);
        }
        if holds_the_field(&copy, party) {
            won += 1;
        }
    }
    won as f64 / trials as f64
}

/// Ticket #339: who is left standing ON the place when the melee ends -- a unit neither destroyed
/// nor escaped, since a unit that ran is not there to hold anything.
fn holds_the_field(parties: &[Vec<Combatant>], party: usize) -> bool {
    let standing = |p: &Vec<Combatant>| p.iter().any(|c| !c.destroyed() && !c.escaped);
    parties.get(party).is_some_and(standing) && parties.iter().enumerate().all(|(i, p)| i == party || !standing(p))
}

/// Spec 10.4: the chance the attacker wins more hit-rolls than the defender in the first round.
/// Ticket #50: in a melee the "defender" strength is the sum of every other party present.
/// Ticket #339 (version 0.09.0): kept, and still the computer's bar, but it is no longer what a
/// player is shown -- see `whole_battle_odds` above.
pub fn first_round_odds(attacker_strength: i64, defender_strength: i64) -> f64 {
    let total = attacker_strength + defender_strength;
    if total <= 0 {
        return 0.0;
    }
    let p = attacker_strength as f64 / total as f64;
    p * p * p + 3.0 * p * p * (1.0 - p)
}

/// Spec 10.2: the disengage chance of one unit. Ticket #295 (version 0.08.6): over `divisor`,
/// the table's figure (`[disengage]` in units.toml), where the 2 was written here.
pub fn disengage_chance(c: &Combatant, divisor: f64) -> f64 {
    if c.evade {
        0.5
    } else if c.damage == 0 || divisor <= 0.0 {
        0.0
    } else {
        (c.damage as f64 / c.hit_points as f64) / divisor
    }
}

/// Run one battle between two parties to its end. Kept for the two-sided callers and the tests;
/// it is `melee` with two parties.
pub fn fight(attackers: &mut [Combatant], defenders: &mut [Combatant], dice: &mut dyn Dice, divisor: f64) -> BattleStats {
    melee(&mut [attackers, defenders], dice, divisor, MAX_ROUNDS, HIT_ROLLS)
}

/// Run one melee to its end: every party is hostile to every other. Units are mutated in place;
/// escaped units are marked. `divisor` is the disengage roll's (ticket #295). Ticket #327
/// (version 0.08.8): `rounds` and `rolls` are the caller's -- a Ship melee rolls once a round for
/// every engaged armed unit across every party, never fewer than the table's figure; a ground
/// melee rolls the table's figure.
pub fn melee(parties: &mut [&mut [Combatant]], dice: &mut dyn Dice, divisor: f64, rounds: u32, rolls: u32) -> BattleStats {
    let n = parties.len();
    // Ticket #381 (version 0.09.2): the line as it opened, for the log.
    let opened: Vec<Vec<LogUnit>> = parties
        .iter()
        .map(|p| p.iter().map(|c| LogUnit { name: c.name.clone(), kind: c.kind, hit_points: c.hit_points, damage: c.damage, armed: c.armed }).collect())
        .collect();
    let mut stats = BattleStats { rounds: 0, hits: vec![0; n], destroyed: vec![Vec::new(); n], escaped: vec![Vec::new(); n], log: BattleLog { parties: opened, rounds: Vec::new() } };
    // Evade rolls at the start of the battle, at current damage (spec 9.2).
    let mut opening = LogRound { opening: true, ..Default::default() };
    for (pi, party) in parties.iter_mut().enumerate() {
        for (ci, c) in party.iter_mut().enumerate().filter(|(_, c)| c.engaged && c.evade && !c.destroyed()) {
            if dice.chance(0.5) {
                c.engaged = false;
                c.escaped = true;
                opening.left.push((pi, ci));
            }
        }
    }
    pursue(parties, dice, &mut stats, &mut opening.chased);
    opening.after = snapshot(parties);
    stats.log.rounds.push(opening);
    for _round in 0..rounds {
        if parties.iter().filter(|p| any_engaged(p)).count() < 2 {
            break;
        }
        stats.rounds += 1;
        let mut round = LogRound::default();
        // Strengths and the engaged parties are read once, at the start of the round, as the
        // two-sided battle read A and D once.
        let strengths: Vec<i64> = parties.iter().map(|p| total_strength(p)).collect();
        let live: Vec<usize> = (0..n).filter(|i| any_engaged(parties[*i])).collect();
        let total: i64 = strengths.iter().sum();
        for _ in 0..rolls {
            if total <= 0 {
                break;
            }
            let Some(hitter) = pick_weighted(&strengths, &live, dice) else { break };
            let targets: Vec<usize> = live.iter().copied().filter(|i| *i != hitter).collect();
            let Some(target) = pick_weighted(&strengths, &targets, dice) else { continue };
            if let Some(i) = random_engaged(parties[target], dice) {
                let covered = covering(parties[target]);
                parties[target][i].damage += 1;
                stats.hits[hitter] += 1;
                round.blows.push(Blow { by: hitter, party: target, unit: i, covering: covered });
            }
        }
        // Disengage. Ticket #297 (version 0.08.6): a dug-in unit never rolls.
        for (pi, party) in parties.iter_mut().enumerate() {
            for (ci, c) in party.iter_mut().enumerate().filter(|(_, c)| c.engaged && !c.destroyed() && !c.dug_in) {
                let p = disengage_chance(c, divisor);
                if p > 0.0 && dice.chance(p) {
                    c.engaged = false;
                    c.escaped = true;
                    round.left.push((pi, ci));
                }
            }
        }
        pursue(parties, dice, &mut stats, &mut round.chased);
        // Remove destroyed units.
        for party in parties.iter_mut() {
            for c in party.iter_mut() {
                if c.destroyed() {
                    c.engaged = false;
                    c.escaped = false;
                }
            }
        }
        round.after = snapshot(parties);
        stats.log.rounds.push(round);
    }
    for (i, party) in parties.iter().enumerate() {
        for c in party.iter() {
            if c.destroyed() {
                stats.destroyed[i].push(c.name.clone());
            } else if c.escaped {
                stats.escaped[i].push(c.name.clone());
            }
        }
    }
    stats
}

/// One draw over `live`, each weighted by its strength, spending one `chance` roll per candidate
/// but the last, and none at all when there is only one candidate. With two candidates this is the
/// single roll at p = strengths[live[0]] / (a + d) the two-sided battle made.
fn pick_weighted(strengths: &[i64], live: &[usize], dice: &mut dyn Dice) -> Option<usize> {
    match live.len() {
        0 => None,
        1 => Some(live[0]),
        _ => {
            let mut left: i64 = live.iter().map(|i| strengths[*i]).sum();
            for i in &live[..live.len() - 1] {
                let s = strengths[*i];
                if left <= 0 {
                    return Some(*i);
                }
                if dice.chance(s as f64 / left as f64) {
                    return Some(*i);
                }
                left -= s;
            }
            Some(live[live.len() - 1])
        }
    }
}

/// Units that have just disengaged (escaped, not yet pursued) are chased by the enemy's best
/// pursuer. Ticket #50: the pursuer is the highest Pursuit among all enemy units still engaged,
/// whichever party it belongs to.
fn pursue(parties: &mut [&mut [Combatant]], dice: &mut dyn Dice, stats: &mut BattleStats, chased: &mut Vec<Blow>) {
    let n = parties.len();
    for i in 0..n {
        let leaver_idx: Vec<usize> = parties[i]
            .iter()
            .enumerate()
            .filter(|(_, c)| c.escaped && !c.destroyed() && !c.engaged && !c.pursued)
            .map(|(j, _)| j)
            .collect();
        for li in leaver_idx {
            // Ticket #326 (version 0.08.8): an unarmed hull that runs while an armed unit of its
            // party still stands engaged is covered, and not pursued.
            if !parties[i][li].armed && escorted(parties[i]) {
                parties[i][li].pursued = true;
                continue;
            }
            let leaver_strength = parties[i][li].strength;
            // The best pursuer among every other party's engaged units.
            let mut best: Option<(u32, i64, usize)> = None;
            for (j, party) in parties.iter().enumerate() {
                if j == i {
                    continue;
                }
                for e in party.iter().filter(|e| e.engaged && !e.destroyed()) {
                    if best.map(|(p, _, _)| e.pursuit > p).unwrap_or(true) {
                        best = Some((e.pursuit, e.strength, j));
                    }
                }
            }
            parties[i][li].pursued = true;
            let Some((pursuit, strength, party)) = best else { continue };
            if pursuit == 0 {
                continue;
            }
            if dice.d6() <= pursuit {
                let p = if strength + leaver_strength <= 0 { 0.0 } else { strength as f64 / (strength + leaver_strength) as f64 };
                if dice.chance(p) {
                    parties[i][li].damage += 1;
                    stats.hits[party] += 1;
                    chased.push(Blow { by: party, party: i, unit: li, covering: false });
                }
            }
        }
    }
}

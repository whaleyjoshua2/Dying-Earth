//! The battle algorithm (spec 10). One algorithm serves space and ground.
//! It works on plain combatants so tests can drive it with fixed dice.

use crate::orders::UnitRef;
use rand::Rng;

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
}

impl Combatant {
    pub fn new(unit: UnitRef, name: impl Into<String>, strength: i64, hit_points: u32, damage: u32, pursuit: u32, evade: bool) -> Combatant {
        Combatant { unit, name: name.into(), strength, hit_points, damage, pursuit, evade, engaged: true, escaped: false, pursued: false }
    }
}

impl Combatant {
    pub fn destroyed(&self) -> bool {
        self.damage >= self.hit_points
    }
}

#[derive(Debug, Clone, Default)]
pub struct BattleStats {
    pub rounds: u32,
    pub hits_by_attacker: u32,
    pub hits_by_defender: u32,
    pub destroyed: Vec<String>,
    pub escaped: Vec<String>,
}

pub const MAX_ROUNDS: u32 = 3;
pub const HIT_ROLLS: u32 = 3;

fn total_strength(side: &[Combatant]) -> i64 {
    side.iter().filter(|c| c.engaged && !c.destroyed()).map(|c| c.strength).sum()
}

fn any_engaged(side: &[Combatant]) -> bool {
    side.iter().any(|c| c.engaged && !c.destroyed())
}

fn random_engaged(side: &[Combatant], dice: &mut dyn Dice) -> Option<usize> {
    let idx: Vec<usize> = side.iter().enumerate().filter(|(_, c)| c.engaged && !c.destroyed()).map(|(i, _)| i).collect();
    if idx.is_empty() {
        None
    } else {
        Some(idx[dice.pick(idx.len())])
    }
}

/// Spec 10.4: the chance the attacker wins more hit-rolls than the defender in the first round.
pub fn first_round_odds(attacker_strength: i64, defender_strength: i64) -> f64 {
    let total = attacker_strength + defender_strength;
    if total <= 0 {
        return 0.0;
    }
    let p = attacker_strength as f64 / total as f64;
    p * p * p + 3.0 * p * p * (1.0 - p)
}

/// Spec 10.2: the disengage chance of one unit.
pub fn disengage_chance(c: &Combatant) -> f64 {
    if c.evade {
        0.5
    } else if c.damage == 0 {
        0.0
    } else {
        (c.damage as f64 / c.hit_points as f64) / 2.0
    }
}

/// Run one battle to its end. Units are mutated in place; escaped units are marked.
pub fn fight(attackers: &mut [Combatant], defenders: &mut [Combatant], dice: &mut dyn Dice) -> BattleStats {
    let mut stats = BattleStats::default();
    // Evade rolls at the start of the battle, at current damage (spec 9.2).
    for side in [&mut *attackers, &mut *defenders] {
        for c in side.iter_mut().filter(|c| c.engaged && c.evade && !c.destroyed()) {
            if dice.chance(0.5) {
                c.engaged = false;
                c.escaped = true;
            }
        }
    }
    // Pursue units that fled before round one.
    pursue(attackers, defenders, dice, &mut stats);
    for _round in 0..MAX_ROUNDS {
        if !any_engaged(attackers) || !any_engaged(defenders) {
            break;
        }
        stats.rounds += 1;
        let a = total_strength(attackers);
        let d = total_strength(defenders);
        for _ in 0..HIT_ROLLS {
            if a + d <= 0 {
                break;
            }
            let p = a as f64 / (a + d) as f64;
            if dice.chance(p) {
                if let Some(i) = random_engaged(defenders, dice) {
                    defenders[i].damage += 1;
                    stats.hits_by_attacker += 1;
                }
            } else if let Some(i) = random_engaged(attackers, dice) {
                attackers[i].damage += 1;
                stats.hits_by_defender += 1;
            }
        }
        // Disengage.
        for side in [&mut *attackers, &mut *defenders] {
            for c in side.iter_mut().filter(|c| c.engaged && !c.destroyed()) {
                let p = disengage_chance(c);
                if p > 0.0 && dice.chance(p) {
                    c.engaged = false;
                    c.escaped = true;
                }
            }
        }
        pursue(attackers, defenders, dice, &mut stats);
        // Remove destroyed units.
        for side in [&mut *attackers, &mut *defenders] {
            for c in side.iter_mut() {
                if c.destroyed() {
                    c.engaged = false;
                    c.escaped = false;
                }
            }
        }
    }
    for side in [&*attackers, &*defenders] {
        for c in side.iter() {
            if c.destroyed() {
                stats.destroyed.push(c.name.clone());
            } else if c.escaped {
                stats.escaped.push(c.name.clone());
            }
        }
    }
    stats
}

/// Units that have just disengaged (escaped, not yet pursued) are chased by the enemy's best pursuer.
fn pursue(attackers: &mut [Combatant], defenders: &mut [Combatant], dice: &mut dyn Dice, stats: &mut BattleStats) {
    let mut pursue_side = |leavers: &mut [Combatant], enemies: &mut [Combatant], enemy_is_attacker: bool| {
        let leaver_idx: Vec<usize> =
            leavers.iter().enumerate().filter(|(_, c)| c.escaped && !c.destroyed() && !c.engaged && !c.pursued).map(|(i, _)| i).collect();
        for li in leaver_idx {
            let leaver_strength = leavers[li].strength;
            let pursuer = enemies
                .iter()
                .filter(|e| e.engaged && !e.destroyed())
                .max_by_key(|e| e.pursuit)
                .map(|e| (e.pursuit, e.strength));
            leavers[li].pursued = true;
            let Some((pursuit, strength)) = pursuer else { continue };
            if pursuit == 0 {
                continue;
            }
            if dice.d6() <= pursuit {
                let p = if strength + leaver_strength <= 0 { 0.0 } else { strength as f64 / (strength + leaver_strength) as f64 };
                if dice.chance(p) {
                    leavers[li].damage += 1;
                    if enemy_is_attacker {
                        stats.hits_by_attacker += 1;
                    } else {
                        stats.hits_by_defender += 1;
                    }
                }
            }
        }
    };
    pursue_side(attackers, defenders, false);
    pursue_side(defenders, attackers, true);
}

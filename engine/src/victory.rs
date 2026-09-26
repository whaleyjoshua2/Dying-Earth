//! Victory and defeat (spec 15), for four seats since ticket #50.

use crate::data::{VictoryFirstKind, VictorySecondKind};
use crate::ids::*;
use crate::state::*;

#[derive(Debug, Clone)]
pub struct Progress {
    pub first_name: String,
    pub first_value: f64,
    pub first_bar: f64,
    /// Ticket #51: the second part is a Faction figure too, not Off-world Presence for everyone.
    pub second_name: String,
    pub second_value: f64,
    pub second_bar: f64,
    /// The second part in the words of the Faction's card, for the panel.
    pub second_text: String,
    /// Ticket #51: the first part is at its bar but something else denies it (the Archive is
    /// complete and offline). Set with the reason.
    pub first_held_back: Option<String>,
}

impl Progress {
    pub fn first_fraction(&self) -> f64 {
        (self.first_value / self.first_bar).clamp(0.0, 1.0)
    }
    pub fn second_fraction(&self) -> f64 {
        (self.second_value / self.second_bar).clamp(0.0, 1.0)
    }
    /// The lower fraction of the two parts.
    pub fn score(&self) -> f64 {
        self.first_fraction().min(self.second_fraction())
    }
    /// Ticket #261: is the first part met on its own terms -- at its bar and not held back?
    pub fn first_met(&self) -> bool {
        self.first_value >= self.first_bar && self.first_held_back.is_none()
    }
    pub fn second_met(&self) -> bool {
        self.second_value >= self.second_bar
    }
    /// Ticket #261: the part still short -- its name, its value and its bar -- the lower of the two
    /// by fraction, which is the figure a player can still act on.
    pub fn short_part(&self) -> (&str, f64, f64) {
        if self.first_fraction() <= self.second_fraction() {
            (&self.first_name, self.first_value, self.first_bar)
        } else {
            (&self.second_name, self.second_value, self.second_bar)
        }
    }
    pub fn met(&self) -> bool {
        self.first_value >= self.first_bar && self.second_value >= self.second_bar && self.first_held_back.is_none()
    }
    /// How far past the bar, for the both-met tiebreak: the lower of the two parts' fractions, unclamped.
    pub fn margin(&self) -> f64 {
        (self.first_value / self.first_bar).min(self.second_value / self.second_bar)
    }
}

impl Game {
    /// Ticket #261 (version 0.08.4): **the rival's Moment.** A rival crossing three quarters of the
    /// way to its Victory Condition, or meeting one part of it with the other still short,
    /// interrupts the player as the game's other Moments do -- once a step, latched on the seat, so
    /// a seat that dips and recrosses is not announced twice. Rivals only, at the designer's word:
    /// the player has the Victory window, and the Moment exists because nothing told them about the
    /// others. Two suggestions proposed it with different steps (half, three quarters, one turn
    /// from; and three quarters once plus the gate Tech); the designer took three quarters and the
    /// one step the engine can state exactly -- one part met -- since "one turn from" wants a
    /// projection the engine does not make. The gate Tech's own Moment was left as it is.
    fn rival_moments(&mut self, progress: &[Progress]) {
        if self.outcome.is_some() {
            return;
        }
        let share = self.tables.victory.rival_moment_share;
        for seat in Seat::ALL.into_iter().filter(|s| *s != Seat(0)) {
            let p = &progress[seat.index()];
            if p.met() {
                continue;
            }
            let (part, value, bar) = p.short_part();
            let (part, figure) = (part.to_string(), format!("{value:.0} of {bar:.0}"));
            let faction = self.seat_name(seat);
            if !self.seat(seat).rival_steps_announced[0] && p.score() >= share {
                self.seat_mut(seat).rival_steps_announced[0] = true;
                let text = self.say("rival_three_quarters", &[("faction", faction.clone()), ("part", part.clone()), ("value", format!("{value:.0}")), ("bar", format!("{bar:.0}"))]);
                self.log(text.clone());
                self.moment(MomentKind::RivalProgress, &[("text", text), ("figure", figure.clone())], None);
                if let Some(m) = self.report.moments.last_mut() {
                    m.seat = Some(seat);
                }
            }
            let one_met = p.first_met() != p.second_met();
            if !self.seat(seat).rival_steps_announced[1] && one_met {
                self.seat_mut(seat).rival_steps_announced[1] = true;
                let met = if p.first_met() { p.first_name.clone() } else { p.second_name.clone() };
                let text = self.say("rival_one_part_met", &[("faction", faction), ("met", met), ("part", part), ("value", format!("{value:.0}")), ("bar", format!("{bar:.0}"))]);
                self.log(text.clone());
                self.moment(MomentKind::RivalProgress, &[("text", text), ("figure", figure)], None);
                if let Some(m) = self.report.moments.last_mut() {
                    m.seat = Some(seat);
                }
            }
        }
    }

    pub fn progress(&self, seat: Seat) -> Progress {
        let s = self.seat(seat);
        // Ticket #50: the first part is whatever the Faction's card names, at the bar on the card.
        let card = self.tables.faction(s.kind).victory_first;
        let first_value = match card.kind {
            VictoryFirstKind::VentureFund => s.venture_fund as f64,
            VictoryFirstKind::StabilizationRun => s.stabilization_run as f64,
            VictoryFirstKind::ColonistsOffEarth => self.off_world_colonists(seat) as f64,
            VictoryFirstKind::ResearchProduced => s.research_total as f64,
            // Ticket #68: the Research paid into the Archive; the bar only tells while the Archive
            // is running.
            // Ticket #347 (version 0.09.1): the fund, whole and unclamped. This read used to be
            // `.min(self.archive_fund_cap(seat))`, and with the cap at a quarter of the figure until
            // the Module stood the Archivists' first part could not pass 0.25 however much they
            // banked -- 0 wins of 80 across eighty measured games, with the fund at exactly the cap
            // in every seating. The clamp is removed rather than left standing now that the cap is
            // the Archive's own figure, because a clamp that looks live and is not is the thing a
            // reader trusts and a future change breaks.
            VictoryFirstKind::ArchiveResearch => s.archive_fund as f64,
        };
        // Ticket #51: the second part is whatever the card names, at the card's own figures.
        let second = self.tables.faction(s.kind).victory_second;
        let (second_value, second_bar, second_text) = match second.kind {
            VictorySecondKind::OffWorldPresence => (
                self.off_world_colonists(seat) as f64,
                second.bar,
                format!("{} of {:.0} Colonists living off Earth", self.off_world_colonists(seat), second.bar),
            ),
            VictorySecondKind::ColoniesOnBodies => (
                self.bodies_settled(seat, second.colonists_each) as f64,
                second.bodies as f64,
                format!("{} of {} Bodies with {} Colonists or more", self.bodies_settled(seat, second.colonists_each), second.bodies, second.colonists_each),
            ),
            // Ticket #192 (version 0.08.0): the uploaded, not the living. Monotonic, and it asks the
            // same question the build gate asks, of the same place.
            VictorySecondKind::ColonistsUploaded => (
                self.seat(seat).uploaded as f64,
                second.bar,
                format!("{} of {:.0} Colonists uploaded into the Archive", self.seat(seat).uploaded, second.bar),
            ),
        };
        let mut first_held_back = match card.kind {
            VictoryFirstKind::ArchiveResearch if self.archive_complete(seat) && !self.archive_online(seat) => {
                Some("the Archive is complete but not running".to_string())
            }
            _ => None,
        };
        // Ticket #84 (version 0.06.0): the Faction's gate Tech must stand before it can win.
        // Progress accrues regardless; Provisional Findings opens nothing.
        if first_held_back.is_none()
            && let Some(gate) = self.tables.victory_gate(self.kind(seat))
            && !self.has_tech(gate)
        {
            first_held_back = Some(format!("needs {}, not yet researched", self.tables.tech(gate).name));
        }
        Progress {
            first_name: card.kind.name().to_string(),
            first_value,
            first_bar: card.bar,
            second_name: second.kind.name().to_string(),
            second_value,
            second_bar,
            second_text,
            first_held_back,
        }
    }

    /// Phase 7: End. Victory checks in their stated order, then Collapse, then the turn advances.
    pub fn end_phase(&mut self) {
        let progress: Vec<Progress> = Seat::ALL.into_iter().map(|s| self.progress(s)).collect();
        self.rival_moments(&progress);
        let met: Vec<Seat> = Seat::ALL.into_iter().filter(|s| progress[s.index()].met()).collect();
        if met.len() == 1 {
            self.outcome = Some(Outcome::Win { seat: met[0], margin_note: "met its Victory Condition".into() });
        } else if met.len() > 1 {
            let best = met.iter().map(|s| progress[s.index()].margin()).fold(f64::MIN, f64::max);
            let leaders: Vec<Seat> = met.iter().copied().filter(|s| progress[s.index()].margin() >= best).collect();
            self.outcome = Some(if leaders.len() == 1 {
                Outcome::Win { seat: leaders[0], margin_note: "more than one met its Victory Condition; the larger margin".into() }
            } else {
                Outcome::Draw { note: "more than one met its Victory Condition by the same margin".into() }
            });
        }
        if self.outcome.is_none() && self.climate.temperature >= self.tables.climate.collapse_line {
            self.outcome = Some(Outcome::Collapse);
        }
        if self.outcome.is_none() && self.turn >= self.tables.victory.turns {
            // Rank every seat by score, then Colonists off Earth, then Colonies held, then a draw.
            let leaders = |key: &dyn Fn(Seat) -> f64, from: &[Seat]| -> Vec<Seat> {
                let best = from.iter().map(|s| key(*s)).fold(f64::MIN, f64::max);
                from.iter().copied().filter(|s| key(*s) >= best).collect()
            };
            let all: Vec<Seat> = Seat::ALL.to_vec();
            let mut note = "higher score at the last turn";
            let mut top = leaders(&|s| progress[s.index()].score(), &all);
            if top.len() > 1 {
                note = "tie broken by Colonists off Earth";
                top = leaders(&|s| self.off_world_colonists(s) as f64, &top);
            }
            if top.len() > 1 {
                note = "tie broken by Colonies held";
                top = leaders(&|s| self.owned_colonies(s).len() as f64, &top);
            }
            self.outcome = Some(if top.len() == 1 {
                Outcome::Win { seat: top[0], margin_note: note.into() }
            } else {
                Outcome::Draw { note: "equal at the last turn".into() }
            });
        }
        if let Some(o) = self.outcome.clone() {
            let line = match &o {
                Outcome::Win { seat, margin_note } => format!("Game over on turn {}: the {} win ({}).", self.turn, self.seat_name(*seat), margin_note),
                Outcome::Draw { note } => format!("Game over on turn {}: a draw ({}).", self.turn, note),
                Outcome::Collapse => format!("Game over on turn {}: Collapse at {:+.1} C. Nobody wins.", self.turn, self.climate.temperature),
            };
            self.log(line);
            let text = match &o {
                Outcome::Win { seat, margin_note } => {
                    self.say("game_over_win", &[("turn", self.turn.to_string()), ("faction", self.seat_name(*seat)), ("note", margin_note.clone())])
                }
                Outcome::Draw { note } => self.say("game_over_draw", &[("turn", self.turn.to_string()), ("note", note.clone())]),
                Outcome::Collapse => {
                    self.say("game_over_collapse", &[("turn", self.turn.to_string()), ("temperature", format!("{:+.1}", self.climate.temperature))])
                }
            };
            self.report_line(LineKind::Note, None, text);
        }
    }

    pub fn outcome_text(&self) -> String {
        match &self.outcome {
            None => "The game is still on.".to_string(),
            Some(Outcome::Win { seat, margin_note }) => format!("The {} win: {}.", self.seat_name(*seat), margin_note),
            Some(Outcome::Draw { note }) => format!("A draw: {note}."),
            Some(Outcome::Collapse) => "Collapse. Nobody wins.".to_string(),
        }
    }
}

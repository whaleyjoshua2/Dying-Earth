//! Victory and defeat (spec 15).

use crate::ids::*;
use crate::state::*;

#[derive(Debug, Clone)]
pub struct Progress {
    pub first_name: String,
    pub first_value: f64,
    pub first_bar: f64,
    pub presence: u32,
    pub presence_bar: u32,
}

impl Progress {
    pub fn first_fraction(&self) -> f64 {
        (self.first_value / self.first_bar).clamp(0.0, 1.0)
    }
    pub fn presence_fraction(&self) -> f64 {
        (self.presence as f64 / self.presence_bar as f64).clamp(0.0, 1.0)
    }
    /// The lower fraction of the two parts.
    pub fn score(&self) -> f64 {
        self.first_fraction().min(self.presence_fraction())
    }
    pub fn met(&self) -> bool {
        self.first_value >= self.first_bar && self.presence >= self.presence_bar
    }
    /// How far past the bar, for the both-met tiebreak: the lower of the two parts' fractions, unclamped.
    pub fn margin(&self) -> f64 {
        (self.first_value / self.first_bar).min(self.presence as f64 / self.presence_bar as f64)
    }
}

impl Game {
    pub fn progress(&self, seat: Seat) -> Progress {
        let v = &self.tables.victory;
        let s = self.seat(seat);
        let (first_name, first_value, first_bar) = match s.kind {
            FactionKind::Prospectors => ("Extraction Total".to_string(), s.extraction_total as f64, v.extraction_total as f64),
            FactionKind::Custodians => ("Stabilization run".to_string(), s.stabilization_run as f64, v.stabilization_turns as f64),
        };
        Progress { first_name, first_value, first_bar, presence: self.off_world_colonists(seat), presence_bar: v.off_world_presence }
    }

    /// Phase 7: End. Victory checks in their stated order, then Collapse, then the turn advances.
    pub fn end_phase(&mut self) {
        let p0 = self.progress(Seat(0));
        let p1 = self.progress(Seat(1));
        match (p0.met(), p1.met()) {
            (true, false) => {
                self.outcome = Some(Outcome::Win { seat: Seat(0), margin_note: "met its Victory Condition".into() });
            }
            (false, true) => {
                self.outcome = Some(Outcome::Win { seat: Seat(1), margin_note: "met its Victory Condition".into() });
            }
            (true, true) => {
                let (m0, m1) = (p0.margin(), p1.margin());
                self.outcome = Some(if m0 > m1 {
                    Outcome::Win { seat: Seat(0), margin_note: "both met their Victory Conditions; larger margin".into() }
                } else if m1 > m0 {
                    Outcome::Win { seat: Seat(1), margin_note: "both met their Victory Conditions; larger margin".into() }
                } else {
                    Outcome::Draw { note: "both met their Victory Conditions by the same margin".into() }
                });
            }
            (false, false) => {}
        }
        if self.outcome.is_none() && self.climate.temperature >= self.tables.climate.collapse_line {
            self.outcome = Some(Outcome::Collapse);
        }
        if self.outcome.is_none() && self.turn >= self.tables.victory.turns {
            let (s0, s1) = (p0.score(), p1.score());
            self.outcome = Some(if s0 > s1 {
                Outcome::Win { seat: Seat(0), margin_note: "higher score at the last turn".into() }
            } else if s1 > s0 {
                Outcome::Win { seat: Seat(1), margin_note: "higher score at the last turn".into() }
            } else {
                let (c0, c1) = (self.off_world_colonists(Seat(0)), self.off_world_colonists(Seat(1)));
                if c0 != c1 {
                    Outcome::Win { seat: if c0 > c1 { Seat(0) } else { Seat(1) }, margin_note: "tie broken by Colonists off Earth".into() }
                } else {
                    let (n0, n1) = (self.owned_colonies(Seat(0)).len(), self.owned_colonies(Seat(1)).len());
                    if n0 != n1 {
                        Outcome::Win { seat: if n0 > n1 { Seat(0) } else { Seat(1) }, margin_note: "tie broken by Colonies held".into() }
                    } else {
                        Outcome::Draw { note: "equal at the last turn".into() }
                    }
                }
            });
        }
        if let Some(o) = &self.outcome {
            let line = match o {
                Outcome::Win { seat, margin_note } => format!("Game over on turn {}: the {} win ({}).", self.turn, self.seat_name(*seat), margin_note),
                Outcome::Draw { note } => format!("Game over on turn {}: a draw ({}).", self.turn, note),
                Outcome::Collapse => format!("Game over on turn {}: Collapse at {:+.1} C. Nobody wins.", self.turn, self.climate.temperature),
            };
            self.log(line.clone());
            self.report.lines.push(line);
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

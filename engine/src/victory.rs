//! Victory and defeat (spec 15), for four seats since ticket #50.

use crate::data::VictoryFirstKind;
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
        let s = self.seat(seat);
        // Ticket #50: the first part is whatever the Faction's card names, at the bar on the card.
        let card = self.tables.faction(s.kind).victory_first;
        let first_value = match card.kind {
            VictoryFirstKind::ExtractionTotal => s.extraction_total as f64,
            VictoryFirstKind::StabilizationRun => s.stabilization_run as f64,
            VictoryFirstKind::ColonistsOffEarth => self.off_world_colonists(seat) as f64,
            VictoryFirstKind::ResearchProduced => s.research_total as f64,
        };
        Progress {
            first_name: card.kind.name().to_string(),
            first_value,
            first_bar: card.bar,
            presence: self.off_world_colonists(seat),
            presence_bar: self.tables.victory.off_world_presence,
        }
    }

    /// Phase 7: End. Victory checks in their stated order, then Collapse, then the turn advances.
    pub fn end_phase(&mut self) {
        let progress: Vec<Progress> = Seat::ALL.into_iter().map(|s| self.progress(s)).collect();
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

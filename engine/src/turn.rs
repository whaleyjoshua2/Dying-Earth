//! The seven phases in order (spec 6), and how a game starts.

use crate::ids::*;
use crate::orders::Order;
use crate::state::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Income,
    Climate,
    Report,
    Orders,
    Event,
    Resolution,
    End,
}

impl Phase {
    pub fn name(self) -> &'static str {
        match self {
            Phase::Income => "Income",
            Phase::Climate => "Climate",
            Phase::Report => "Report",
            Phase::Orders => "Orders",
            Phase::Event => "Event",
            Phase::Resolution => "Resolution",
            Phase::End => "End",
        }
    }
}

impl Game {
    /// Run phases 1 to 3 of the first turn, leaving the game in its first Orders phase.
    pub fn start(&mut self) {
        self.report = Report { turn: self.turn, ..Default::default() };
        self.report.lines.push(format!(
            "Turn 1. You are the {}. You hold {} with a Launch Site and its Standing Army. The Stockpile holds {} Materials, {} Fuel and {} Energy.",
            self.seat_name(Seat(0)),
            self.tables.state(self.controlled_states(Seat(0))[0]).name,
            self.seat(Seat(0)).stockpile.materials,
            self.seat(Seat(0)).stockpile.fuel,
            self.seat(Seat(0)).stockpile.energy
        ));
        self.report.lines.push(format!(
            "The {} hold {}. Build, spread Influence, and get twelve Colonists off Earth before the Temperature reaches +{:.1} C.",
            self.seat_name(Seat(1)),
            self.tables.state(self.controlled_states(Seat(1))[0]).name,
            self.tables.climate.collapse_line
        ));
        if self.seat(Seat(0)).ai {
            let pick = self.ai_tech_pick(Seat(0));
            self.pick_tech(Seat(0), pick).ok();
        }
        self.log(format!("--- Turn {} ---", self.turn));
        self.log("Phase 1: Income");
        self.income_phase();
        self.log("Phase 2: Climate");
        self.climate_phase();
        self.log("Phase 3: Report");
        self.report_phase();
    }

    fn report_phase(&mut self) {
        self.report.turn = self.turn;
        for l in self.report.lines.clone() {
            self.log(format!("  Report: {l}"));
        }
    }

    /// End Turn: the player's orders are committed, the AI orders, and the turn runs to the next Orders phase.
    /// `orders[i]` is used for a human seat; an AI seat computes its own.
    pub fn end_turn(&mut self, orders: [Vec<Order>; 2]) {
        if self.is_over() {
            return;
        }
        self.log("Phase 4: Orders");
        let mut all: [Vec<Order>; 2] = [Vec::new(), Vec::new()];
        for seat in Seat::ALL {
            if self.seat(seat).ai {
                all[seat.index()] = self.ai_orders(seat);
            } else {
                all[seat.index()] = orders[seat.index()].clone();
            }
        }
        // Report for the coming turn starts collecting now.
        self.report = Report::default();
        for seat in Seat::ALL {
            let list = all[seat.index()].clone();
            if let Err((i, e)) = self.check_orders(seat, &list) {
                // An illegal order in a list is dropped with a note; the rest stand.
                self.log(format!("{}: order {} dropped: {}", self.seat_name(seat), i + 1, e));
                let mut kept = Vec::new();
                for o in list {
                    if self.check_order(seat, &kept, &o).is_ok() {
                        kept.push(o);
                    }
                }
                self.commit_orders(seat, &kept);
            } else {
                self.commit_orders(seat, &list);
            }
            self.log(format!("{} gave {} order(s).", self.seat_name(seat), all[seat.index()].len()));
        }
        self.log("Phase 5: Event");
        self.event_phase();
        self.log("Phase 6: Resolution");
        self.resolution_phase();
        self.log("Phase 7: End");
        self.end_phase();
        if self.is_over() {
            return;
        }
        self.turn += 1;
        self.log(format!("--- Turn {} ---", self.turn));
        self.log("Phase 1: Income");
        self.income_phase();
        self.log("Phase 2: Climate");
        self.climate_phase();
        self.log("Phase 3: Report");
        self.report_phase();
    }
}

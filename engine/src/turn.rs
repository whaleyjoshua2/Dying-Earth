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
        // Ticket #58: the first Report keeps its explanation as the headline.
        let home = self.tables.state(self.controlled_states(Seat(0))[0]).name.clone();
        let names: Vec<String> = Seat::ALL.into_iter().skip(1).map(|s| format!("the {}", self.seat_name(s))).collect();
        let seating = self.say(
            "seating",
            &[
                ("date", self.date_text()),
                ("faction", self.seat_name(Seat(0))),
                ("state", home.clone()),
                ("rivals", Game::and_list(&names)),
            ],
        );
        self.report_line(LineKind::Seating, None, seating);
        let holding = self.say(
            "start_holding",
            &[
                ("state", home),
                ("materials", self.seat(Seat(0)).stockpile.materials.to_string()),
                ("fuel", self.seat(Seat(0)).stockpile.fuel.to_string()),
                ("energy", self.seat(Seat(0)).stockpile.energy.to_string()),
            ],
        );
        self.report_line(LineKind::YourWorks, Some(ReportPlace::State(self.controlled_states(Seat(0))[0])), holding);
        // Ticket #50: three rivals, not one.
        let rivals: Vec<String> = Seat::ALL
            .into_iter()
            .skip(1)
            .map(|s| format!("the {} in {}", self.seat_name(s), self.tables.state(self.controlled_states(s)[0]).name))
            .collect();
        let text = self.say(
            "start_rivals",
            &[("rivals", rivals.join(", ")), ("collapse", format!("{:.1}", self.tables.climate.collapse_line))],
        );
        self.report_line(LineKind::Note, None, text);
        if self.seat(Seat(0)).ai {
            let pick = self.ai_tech_pick(Seat(0));
            self.pick_tech(Seat(0), pick).ok();
        }
        self.log(format!("--- Turn {} ---", self.turn));
        // Ticket #173 (version 0.07.6): the first turn freezes its findings Tech too.
        self.research.findings_tech = self.research.current;
        self.log("Phase 1: Income");
        self.income_phase();
        self.log("Phase 2: Climate");
        self.climate_phase();
        self.log("Phase 3: Report");
        self.report_phase();
    }

    fn report_phase(&mut self) {
        self.report.turn = self.turn;
        // The drawn card is already in the log where it was drawn, so it is not logged twice.
        for l in self.report.lines.clone() {
            if l.kind == LineKind::Event {
                continue;
            }
            self.log(format!("  Report: {}", l.text));
        }
    }

    /// Ticket #105 (version 0.07.0): why the turn cannot end yet, or `None`. The rule lives HERE
    /// rather than in an interface, so every caller is bound by it -- the game, the headless driver,
    /// and anything built later.
    ///
    /// It used to live in one `add_enabled` in the interface, and the headless driver added this
    /// same version did not know about it: twelve playtest games were played in which declining to
    /// pick froze the tech tree for good, and that was reported as "the strongest strategy in the
    /// game". It was the harness, not the game. A rule only one caller enforces is a habit.
    pub fn end_turn_refusal(&self) -> Option<String> {
        let owed = self.research.awaiting_pick?;
        if self.seat(owed).ai || self.available_techs().is_empty() {
            return None;
        }
        Some(format!(
            "The {} hold the Research Lead and owe the table a Tech. Choose what the world researches next; the turn cannot end until you do.",
            self.seat_name(owed)
        ))
    }

    /// End Turn: the player's orders are committed, the AI orders, and the turn runs to the next Orders phase.
    /// `orders[i]` is used for a human seat; an AI seat computes its own.
    ///
    /// Ticket #105: refuses, and says why, while a human Lead owes a pick. Nothing is committed and
    /// nothing advances when it refuses.
    pub fn end_turn(&mut self, orders: [Vec<Order>; SEAT_COUNT]) -> Result<(), String> {
        if let Some(why) = self.end_turn_refusal() {
            return Err(why);
        }
        if self.is_over() {
            return Ok(());
        }
        // Ticket #173 (version 0.07.6): the Lead's pick becomes final HERE and nowhere earlier. The
        // designer: *"tech choice is not locked in until the turn is ended."* Everything a pick used
        // to do the instant it was clicked -- spending the banked Research, throwing away the
        // shortlist, and possibly finishing the Tech outright -- happens now, after the refusal
        // check, so a turn that is refused leaves the pick as changeable as it was.
        self.commit_pick();
        self.log("Phase 4: Orders");
        let mut all: [Vec<Order>; SEAT_COUNT] = std::array::from_fn(|_| Vec::new());
        // Report for the coming turn starts collecting now, before the AI seats order, so their
        // scored lists survive into it (ticket #50: three AI seats write to it, one after another).
        self.report = Report::default();
        // Ticket #178: stamped HERE as well as in `report_phase`, because a game that ends this turn
        // returns below without reaching either `self.turn += 1` or `report_phase`, and the heading is
        // drawn from `report.turn`. Without it the last Report of every game read January 2030. A turn
        // that carries on overwrites this with the new turn in `report_phase`, exactly as before.
        self.report.turn = self.turn;
        for seat in Seat::ALL {
            if self.seat(seat).ai {
                all[seat.index()] = self.ai_orders(seat);
            } else {
                all[seat.index()] = orders[seat.index()].clone();
            }
        }
        for seat in Seat::ALL {
            let list = all[seat.index()].clone();
            let kept = if let Err((i, e)) = self.check_orders(seat, &list) {
                // An illegal order in a list is dropped with a note; the rest stand.
                self.log(format!("{}: order {} dropped: {}", self.seat_name(seat), i + 1, e));
                let mut kept = Vec::new();
                for o in list {
                    if self.check_order(seat, &kept, &o).is_ok() {
                        kept.push(o);
                    }
                }
                kept
            } else {
                list
            };
            // Ticket #58: an AI seat's paragraph starts here, from the orders it actually committed,
            // and the Resolution adds what came of them. The scored list it chose from stays in the
            // log and nowhere else.
            if self.seat(seat).ai {
                let told = crate::orders::merged_for_report(&kept);
                let deeds: Vec<String> = told.iter().filter_map(|o| self.rival_deed(seat, o)).collect();
                self.report.ai_lines.push(AiReport { seat, deeds });
            }
            self.commit_orders(seat, &kept);
            self.log(format!("{} gave {} order(s).", self.seat_name(seat), all[seat.index()].len()));
        }
        self.log("Phase 5: Event");
        self.event_phase();
        self.log("Phase 6: Resolution");
        self.resolution_phase();
        // Ticket #191 (version 0.08.0): the turn's offences are charged once, the quiet pairs
        // recover, and the slate is wiped.
        self.settle_relations();
        // Ticket #220 (version 0.08.2): the turn's trading moves each price once, and the tally is
        // wiped -- beside the Relations settle, and for the same reason: both read a whole turn's
        // worth of acts and neither can be judged an order at a time.
        self.settle_market();
        self.log("Phase 7: End");
        self.end_phase();
        if self.is_over() {
            return Ok(());
        }
        self.turn += 1;
        self.log(format!("--- Turn {} ---", self.turn));
        // Ticket #173 (version 0.07.6): the Archivists read the Tech under research for the whole of
        // the turn to come, frozen here. Their Provisional Findings gives them half its effect while
        // the turn is still being ordered, and a Lead that may change its pick would otherwise
        // re-price orders already placed.
        self.research.findings_tech = self.research.current;
        self.log("Phase 1: Income");
        self.income_phase();
        self.log("Phase 2: Climate");
        self.climate_phase();
        self.log("Phase 3: Report");
        self.report_phase();
        Ok(())
    }
}

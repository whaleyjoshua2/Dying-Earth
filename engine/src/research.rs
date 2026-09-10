//! Research and the shared Tech Tree (spec 12).

use crate::ids::*;
use crate::state::*;

impl Game {
    /// Techs whose prerequisites are met and that are not done or under research.
    pub fn available_techs(&self) -> Vec<TechId> {
        TechId::ALL
            .into_iter()
            .filter(|t| !self.research.done.contains(t) && self.research.current != Some(*t))
            .filter(|t| self.tables.tech(*t).needs.iter().all(|n| self.research.done.contains(n)))
            .collect()
    }

    /// Research from one seat's Labs flows entirely into the Tech under research.
    pub fn accrue_research(&mut self, seat: Seat, amount: i64) {
        if amount <= 0 {
            return;
        }
        if self.research.current.is_none() {
            self.research.unallocated += amount;
            return;
        }
        self.research.progress += amount;
        self.research.contributions[seat.index()] += amount;
        self.check_tech_complete();
    }

    /// Research nobody produced (a Breakthrough).
    pub fn add_research_unattributed(&mut self, amount: i64) {
        if self.research.current.is_none() {
            self.research.unallocated += amount;
            return;
        }
        self.research.progress += amount;
        self.check_tech_complete();
    }

    /// The Research Lead (spec 12.2, ticket #50): the highest contributor to the Tech just done.
    /// A tie goes to the seat that has picked least recently, never-picked counting as longest ago;
    /// a tie the picking order does not settle is drawn at random.
    pub fn research_lead(&mut self) -> Seat {
        let tied = self.research_lead_candidates();
        self.random_tie(&tied)
    }

    /// The seats still tied for the Research Lead once the least-recent-picker rule has been
    /// applied. One seat is the Lead; more than one goes to a random draw.
    pub fn research_lead_candidates(&self) -> Vec<Seat> {
        let c = self.research.contributions;
        let top = c.iter().copied().max().unwrap_or(0);
        let mut tied: Vec<Seat> = Seat::ALL.into_iter().filter(|s| c[s.index()] == top).collect();
        if tied.len() > 1 {
            // Never picked counts as longest ago; otherwise the earliest turn it last picked.
            let key = |s: &Seat| self.research.last_picked_turn[s.index()].unwrap_or(0);
            let oldest = tied.iter().map(key).min().unwrap_or(0);
            tied.retain(|s| key(s) == oldest);
        }
        tied
    }

    fn check_tech_complete(&mut self) {
        loop {
            let Some(tech) = self.research.current else { return };
            let cost = self.tables.tech(tech).cost;
            if self.research.progress < cost {
                return;
            }
            let overflow = self.research.progress - cost;
            let c = self.research.contributions;
            let lead = self.research_lead();
            self.research.done.push(tech);
            self.research.current = None;
            self.research.progress = 0;
            self.research.contributions = [0; SEAT_COUNT];
            self.research.unallocated += overflow;
            self.research.last_lead = Some(lead);
            let shares: Vec<String> = Seat::ALL.into_iter().map(|s| format!("{} {}", self.seat_name(s), c[s.index()])).collect();
            let line = format!(
                "{} is complete; every Faction has it. The {} led ({}) and pick the next Tech.",
                self.tables.tech(tech).name,
                self.seat_name(lead),
                shares.join(", ")
            );
            self.report.lines.push(line.clone());
            self.log(line);
            if self.available_techs().is_empty() {
                self.research.awaiting_pick = None;
                return;
            }
            if self.seat(lead).ai {
                let pick = self.ai_tech_pick(lead);
                self.pick_tech(lead, pick).ok();
            } else {
                self.research.awaiting_pick = Some(lead);
                return;
            }
        }
    }

    /// Set the Tech under research. Unallocated Research flows in at once.
    pub fn pick_tech(&mut self, seat: Seat, tech: TechId) -> Result<(), String> {
        if self.research.current.is_some() {
            return Err("a Tech is already under research".into());
        }
        if !self.available_techs().contains(&tech) {
            return Err("that Tech is not available yet".into());
        }
        self.research.current = Some(tech);
        self.research.awaiting_pick = None;
        self.research.last_picked_turn[seat.index()] = Some(self.turn);
        let carried = std::mem::take(&mut self.research.unallocated);
        self.research.progress = 0;
        let line = format!("{} chose {} as the next Tech.", self.seat_name(seat), self.tables.tech(tech).name);
        self.log(line);
        if carried > 0 {
            self.research.progress += carried;
            self.check_tech_complete();
        }
        Ok(())
    }

    /// The AI's fixed pick order (spec 16.4), then the cheapest available.
    pub fn ai_tech_pick(&self, seat: Seat) -> TechId {
        let picks = self.tables.ai_tech_picks(self.kind(seat));
        let available = self.available_techs();
        for t in &picks.order {
            if available.contains(t) {
                return *t;
            }
        }
        let mut rest: Vec<TechId> = available.clone();
        rest.retain(|t| Some(*t) != picks.never && Some(*t) != picks.last);
        rest.sort_by_key(|t| self.tables.tech(*t).cost);
        if let Some(t) = rest.first() {
            return *t;
        }
        if let Some(last) = picks.last.filter(|t| available.contains(t)) {
            return last;
        }
        available[0]
    }

    /// The tech panel line: every seat's share of the Tech under research, and who picks next.
    pub fn research_lead_text(&self) -> String {
        let c = self.research.contributions;
        let total: i64 = c.iter().sum();
        let pct = |v: i64| if total == 0 { 100 / SEAT_COUNT as i64 } else { v * 100 / total };
        let shares: Vec<String> = Seat::ALL.into_iter().map(|s| format!("{} {}%", self.seat_name(s), pct(c[s.index()]))).collect();
        let tied = self.research_lead_candidates();
        let next = if tied.len() == 1 {
            format!("{} pick next", self.seat_name(tied[0]))
        } else {
            let names: Vec<String> = tied.iter().map(|s| self.seat_name(*s)).collect();
            format!("{} are tied; the next pick is drawn at random", names.join(" and "))
        };
        format!("{} - {}.", shares.join(", "), next)
    }
}

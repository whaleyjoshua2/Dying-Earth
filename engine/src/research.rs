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

    fn check_tech_complete(&mut self) {
        loop {
            let Some(tech) = self.research.current else { return };
            let cost = self.tables.tech(tech).cost;
            if self.research.progress < cost {
                return;
            }
            let overflow = self.research.progress - cost;
            let c = self.research.contributions;
            let lead = if c[1] > c[0] { Seat(1) } else { Seat(0) };
            self.research.done.push(tech);
            self.research.current = None;
            self.research.progress = 0;
            self.research.contributions = [0, 0];
            self.research.unallocated += overflow;
            self.research.last_lead = Some(lead);
            let line = format!(
                "{} is complete; every Faction has it. The {} led ({} to {}) and pick the next Tech.",
                self.tables.tech(tech).name,
                self.seat_name(lead),
                c[lead.index()],
                c[lead.other().index()]
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
        let picks = &self.tables.ai.tech_picks;
        let available = self.available_techs();
        let list: Vec<TechId> = match self.kind(seat) {
            FactionKind::Prospectors => picks.prospectors.clone(),
            FactionKind::Custodians => picks.custodians.clone(),
        };
        for t in &list {
            if available.contains(t) {
                return *t;
            }
        }
        let mut rest: Vec<TechId> = available.clone();
        if self.kind(seat) == FactionKind::Prospectors {
            rest.retain(|t| *t != picks.prospectors_never && *t != picks.prospectors_last);
        }
        rest.sort_by_key(|t| self.tables.tech(*t).cost);
        if let Some(t) = rest.first() {
            return *t;
        }
        if available.contains(&picks.prospectors_last) {
            return picks.prospectors_last;
        }
        available[0]
    }

    /// The tech panel line: "Custodians 41%, Prospectors 59% - Prospectors pick next."
    pub fn research_lead_text(&self) -> String {
        let c = self.research.contributions;
        let total = c[0] + c[1];
        let pct = |v: i64| if total == 0 { 50 } else { v * 100 / total };
        let lead = if c[1] > c[0] { Seat(1) } else { Seat(0) };
        format!(
            "{} {}%, {} {}% - {} pick next.",
            self.seat_name(Seat(0)),
            pct(c[0]),
            self.seat_name(Seat(1)),
            pct(c[1]),
            self.seat_name(lead)
        )
    }
}

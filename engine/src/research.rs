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
            // Ticket #105: it waits under its owner's name, so it counts toward the Lead when it lands.
            self.research.unallocated[seat.index()] += amount;
            return;
        }
        self.research.progress += amount;
        self.research.contributions[seat.index()] += amount;
        self.check_tech_complete();
    }

    /// Research nobody produced (a Breakthrough).
    pub fn add_research_unattributed(&mut self, amount: i64) {
        if self.research.current.is_none() {
            self.research.unattributed += amount;
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
            // Ticket #105: the spill past a Tech's cost belongs to nobody in particular, so it
            // waits unattributed rather than being credited to a seat by accident.
            self.research.unattributed += overflow;
            self.research.last_lead = Some(lead);
            let shares: Vec<String> = Seat::ALL.into_iter().map(|s| format!("{} {}", self.seat_name(s), c[s.index()])).collect();
            let line = format!(
                "{} is complete; every Faction has it. The {} led ({}) and pick the next Tech.",
                self.tables.tech(tech).name,
                self.seat_name(lead),
                shares.join(", ")
            );
            self.log(line);
            let text = self.say(
                "tech_complete",
                &[("tech", self.tables.tech(tech).name.clone()), ("faction", self.seat_name(lead)), ("shares", shares.join(", "))],
            );
            self.report_line(LineKind::TechComplete, None, text);
            // Ticket #58: the Tech Moment names the Lead and the margin, and says what the AI picked
            // and why. It is filled in before the pick, so the note can name the Tech chosen.
            self.moment(
                MomentKind::TechComplete,
                &[
                    ("tech", self.tables.tech(tech).name.clone()),
                    ("faction", self.seat_name(lead)),
                    ("lead", c[lead.index()].to_string()),
                    ("cost", cost.to_string()),
                ],
                None,
            );
            if let Some(m) = self.report.moments.last_mut() {
                m.tech = Some(tech);
            }
            if self.available_techs().is_empty() {
                self.research.awaiting_pick = None;
                self.research.shortlist = Vec::new();
                return;
            }
            // Ticket #98: the Lead chooses from a drawn shortlist, not from everything.
            self.draw_shortlist(lead);
            if self.seat(lead).ai {
                let (pick, why) = self.ai_tech_pick_with_reason(lead);
                let note = self.phrase(why, &[("faction", self.seat_name(lead)), ("tech", self.tables.tech(pick).name.clone())]);
                if let Some(m) = self.report.moments.last_mut() {
                    m.note = Some(note);
                }
                self.pick_tech(lead, pick).ok();
            } else {
                let note = self.phrase("you_pick", &[]);
                if let Some(m) = self.report.moments.last_mut() {
                    m.note = Some(note);
                }
                self.research.awaiting_pick = Some(lead);
                return;
            }
        }
    }

    /// Ticket #98 (version 0.07.0): draw the Techs this Lead may choose between. The Lead's OWN
    /// Victory gate is always on the list once its prerequisites are met, so a Faction can be
    /// denied a rival's gate but never its own; the rest are drawn from what is available, from the
    /// game's own generator. Fewer available than the list holds means the list is all of them,
    /// which is the same free choice an empty list gives.
    pub fn draw_shortlist(&mut self, lead: Seat) {
        let available = self.available_techs();
        let size = self.tables.shortlist.size;
        if available.len() <= size {
            self.research.shortlist = Vec::new();
            return;
        }
        let mut drawn: Vec<TechId> = Vec::new();
        if let Some(gate) = self.tables.victory_gate(self.kind(lead))
            && available.contains(&gate)
        {
            drawn.push(gate);
        }
        let mut rest: Vec<TechId> = available.into_iter().filter(|t| !drawn.contains(t)).collect();
        while drawn.len() < size && !rest.is_empty() {
            let i = crate::combat::Dice::pick(&mut self.rng, rest.len());
            drawn.push(rest.remove(i));
        }
        // In tree order, so the panel reads the same way twice running.
        drawn.sort_by_key(|t| t.index());
        self.research.shortlist = drawn;
    }

    /// Ticket #98: the Techs `seat` may pick right now. An empty shortlist is a free choice of
    /// everything available, which is how the game opens.
    pub fn pickable_techs(&self) -> Vec<TechId> {
        if self.research.shortlist.is_empty() {
            self.available_techs()
        } else {
            self.research.shortlist.clone()
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
        // Ticket #98 (version 0.07.0): the Lead chooses from the drawn shortlist. An empty list is
        // a free choice of everything available, which is how the game opens.
        if !self.research.shortlist.is_empty() && !self.research.shortlist.contains(&tech) {
            let names: Vec<String> = self.research.shortlist.iter().map(|t| self.tables.tech(*t).name.clone()).collect();
            return Err(format!("that Tech is not on this turn's shortlist: {}", names.join(", ")));
        }
        self.research.current = Some(tech);
        self.research.awaiting_pick = None;
        self.research.shortlist = Vec::new();
        self.research.last_picked_turn[seat.index()] = Some(self.turn);
        // Ticket #105: everything banked since the last Tech pours in, each seat's share landing
        // under its own name so the Lead line tells the truth.
        let banked = std::mem::take(&mut self.research.unallocated);
        let loose = std::mem::take(&mut self.research.unattributed);
        let carried: i64 = banked.iter().sum::<i64>() + loose;
        for s in Seat::ALL {
            self.research.contributions[s.index()] += banked[s.index()];
        }
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
        self.ai_tech_pick_with_reason(seat).0
    }

    /// The same pick, with the `report.toml` phrase that says why it was made (ticket #58): the
    /// Faction's own first choice off its list, the cheapest left, or the one it leaves until last.
    pub fn ai_tech_pick_with_reason(&self, seat: Seat) -> (TechId, &'static str) {
        let picks = self.tables.ai_tech_picks(self.kind(seat));
        // Ticket #98: the AI is held to the same shortlist a human Lead is.
        let available = self.pickable_techs();
        // Ticket #84 (version 0.06.0): as Research Lead, the AI opens its own door once its first
        // part is past `gate_pick_fraction` of its bar or from `gate_pick_turn`, whichever comes
        // first, the road to the gate standing.
        if let Some(gate) = self.tables.victory_gate(self.kind(seat))
            && available.contains(&gate)
        {
            let th = &self.tables.ai.thresholds;
            let p = self.progress(seat);
            if p.first_fraction() >= th.gate_pick_fraction || self.turn >= th.gate_pick_turn {
                return (gate, "pick_first_choice");
            }
        }
        for t in &picks.order {
            if available.contains(t) {
                return (*t, "pick_first_choice");
            }
        }
        let mut rest: Vec<TechId> = available.clone();
        rest.retain(|t| Some(*t) != picks.never && Some(*t) != picks.last);
        rest.sort_by_key(|t| self.tables.tech(*t).cost);
        if let Some(t) = rest.first() {
            return (*t, "pick_cheapest");
        }
        if let Some(last) = picks.last.filter(|t| available.contains(t)) {
            return (last, "pick_last");
        }
        (available[0], "pick_cheapest")
    }

    // ---------------------------------------------------------------- Ticket #51: the Archive fund

    /// Fund the Archive (ticket #51, rebuilt for version 0.07.0). The Archivists declare where
    /// their Labs' Research goes, and the declaration is read HERE, at Income, before a single
    /// point reaches the shared Tech. What the fund has room for under its cap never enters the
    /// Tech at all, so it contributes nothing to the Research Lead; the rest goes on to the Tech,
    /// and nothing is wasted. The payment that fills the fund with the Module standing completes
    /// the Archive. Answers how much was taken, which the caller keeps back from `accrue_research`.
    ///
    /// Version 0.06.0 ran this as an Orders-phase order that clawed the Research back out of the
    /// shared Tech after Income had already paid it in. A turn whose Research completed a Tech
    /// zeroed every seat's contribution first, so there was nothing left to claw back: the
    /// Archivists banked nothing while the Report still said they had funded the Archive. Measured
    /// in playtest at about 32 of the Archive's 80 Research lost over one game.
    pub fn bank_archive_research(&mut self, seat: Seat, research: i64) -> i64 {
        if research <= 0 || !self.seat(seat).archive_funding || self.kind(seat) != FactionKind::Archivists {
            return 0;
        }
        let cap = self.archive_fund_cap(seat);
        let before = self.seat(seat).archive_fund;
        let banked = research.min((cap - before).max(0));
        if banked <= 0 {
            return 0;
        }
        let after = before + banked;
        {
            let s = self.seat_mut(seat);
            s.archive_fund = after;
            s.funding_archive = true;
        }
        let line = format!("The {} are funding the Archive: {} Research banked, {} of {} in the fund.", self.seat_name(seat), banked, after, cap);
        self.log(line);
        let args = vec![("faction", self.seat_name(seat)), ("banked", banked.to_string()), ("fund", after.to_string()), ("cap", cap.to_string())];
        let text = self.say("archive_funded", &args);
        self.report_line_of(seat, LineKind::YourWorks, LineKind::Archive, None, text);
        // Ticket #68: the payment that fills the fund with the Module standing completes the Archive.
        let required = self.tables.archive.research;
        if self.archive_built(seat) && before < required && after >= required {
            self.archive_completed(seat);
        }
        banked
    }

    /// Ticket #68: the Archive is complete, whether the last Research or the Module came last.
    pub fn archive_completed(&mut self, seat: Seat) {
        let Some(cid) = self.archive_colony(seat) else { return };
        let place = Place::Colony(cid);
        let line = format!("The {} completed the Archive at {}: it stands, and every point of its Research is paid.", self.seat_name(seat), self.place_name(place));
        self.log(line);
        let text = self.say("archive_complete", &[("faction", self.seat_name(seat)), ("place", self.place_name(place))]);
        self.report_line(LineKind::Archive, Some(place.into()), text);
        let research = self.tables.archive.research;
        self.moment(MomentKind::ArchiveComplete, &[("faction", self.seat_name(seat)), ("place", self.place_name(place)), ("research", research.to_string())], Some(place.into()));
    }

    /// Ticket #51: whether the Archivists diverted this turn's Research, for the tech panel.
    pub fn funding_archive(&self, seat: Seat) -> bool {
        self.seat(seat).funding_archive
    }

    /// The tech panel line: every seat's share of the Tech under research, and who picks next.
    pub fn research_lead_text(&self) -> String {
        let c = self.research.contributions;
        let total: i64 = c.iter().sum();
        let pct = |v: i64| if total == 0 { 100 / SEAT_COUNT as i64 } else { v * 100 / total };
        let shares: Vec<String> = Seat::ALL.into_iter().map(|s| format!("{} {}%", self.seat_name(s), pct(c[s.index()]))).collect();
        let funders: Vec<String> = Seat::ALL.into_iter().filter(|s| self.funding_archive(*s)).map(|s| self.seat_name(s)).collect();
        let tied = self.research_lead_candidates();
        let next = if tied.len() == 1 {
            format!("{} pick next", self.seat_name(tied[0]))
        } else {
            let names: Vec<String> = tied.iter().map(|s| self.seat_name(*s)).collect();
            format!("{} are tied; the next pick is drawn at random", names.join(" and "))
        };
        let funding = if funders.is_empty() { String::new() } else { format!(" The {} are funding the Archive this turn.", funders.join(" and ")) };
        format!("{} - {}.{}", shares.join(", "), next, funding)
    }
}

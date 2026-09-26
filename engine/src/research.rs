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
    ///
    /// Ticket #348 (version 0.09.1): and the NEXT RUNG of the Lead's chain is drawn while the gate
    /// itself is still out of reach. Ticket #98's guarantee fired only once the gate was available,
    /// and a Tech is available only once its prerequisites are done -- so the promise that a
    /// Faction is never denied its own gate could not be kept until the chain had already been
    /// climbed by luck. The Custodians' chain is two Techs and every one of them was on their list;
    /// the other three are four Techs and each was missing some of its own.
    ///
    /// THE RESERVED PLACE IS STILL ONE OF THREE. The gate is available only when every antecedent
    /// is done, and a rung is owed only when some antecedent is not, so at most one of the two
    /// clauses below can ever fire. They are written as two independent `if`s rather than an
    /// `else`, because the claim belongs in a test that can watch it rather than in a branch that
    /// hides it: `a_draw_never_forces_both_the_gate_and_its_chain`.
    ///
    /// This reaches the HUMAN Lead too, which is the point of it: the Lead's draw is the Lead's
    /// draw whoever holds the seat.
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
        // Ticket #348: the next rung of the chain, which is owed exactly when the gate is not.
        if let Some(rung) = self.next_gate_rung(lead) {
            drawn.push(rung);
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

    /// Ticket #348 (version 0.09.1): the next rung of this seat's OWN gate chain -- the cheapest
    /// Tech standing between it and its Victory gate that is not yet researched and whose own
    /// prerequisites are met. Ties on cost break by the Tech's place in the tree, so a seeded game
    /// is not moved by two antecedents costing the same.
    ///
    /// `None` once the chain is climbed, which is EXACTLY when the gate itself becomes available:
    /// the gate is available only when every antecedent is done, and an unresearched antecedent
    /// exists only when some antecedent is not. The two are mutually exclusive, which is why the
    /// shortlist can reserve a place for whichever of them applies without ever reserving two.
    /// `a_draw_never_forces_both_the_gate_and_its_chain` is the test that says so.
    ///
    /// The cheapest unresearched antecedent may not be available -- its own prerequisites may be
    /// unmet, or it may be the Tech under research this minute -- and then the cheapest one that IS
    /// available is taken, and nothing at all if none is.
    pub fn next_gate_rung(&self, seat: Seat) -> Option<TechId> {
        let gate = self.tables.victory_gate(self.kind(seat))?;
        if self.research.done.contains(&gate) {
            return None;
        }
        let available = self.available_techs();
        self.tables.gate_chain(self.kind(seat)).into_iter().find(|t| !self.research.done.contains(t) && available.contains(t))
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

    /// Set the Tech under research.
    ///
    /// Ticket #173 (version 0.07.6): a **human** Lead's pick is provisional -- the designer: *"tech
    /// choice is not locked in until the turn is ended"* -- so this records the choice and stops
    /// there, and the same seat may call it again to change its mind for as long as the turn lasts.
    /// Nothing is spent, the shortlist is kept (redrawing it would be a free reroll), and the Tech
    /// cannot complete. `commit_pick` at the head of `end_turn` does all of that. A **computer**
    /// seat's pick commits in the same breath, since it never changes its mind and the loop that
    /// finishes a Tech expects the next one to be standing.
    pub fn pick_tech(&mut self, seat: Seat, tech: TechId) -> Result<(), String> {
        // A committed Tech is settled; only an uncommitted pick of this seat's own may be replaced.
        if self.research.current.is_some() && self.research.pick_committed {
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
        self.research.pick_committed = false;
        self.research.picked_by = Some(seat);
        // Ticket #173: a computer seat never changes its mind, and the loop that finishes a Tech
        // expects the next one standing, so its pick is committed here and now.
        if self.seat(seat).ai {
            self.commit_pick();
        }
        Ok(())
    }

    /// Ticket #173 (version 0.07.6): make a pick final. Everything that used to happen the instant
    /// a Tech was chosen happens here instead: the shortlist is thrown away, the seat is stamped for
    /// the Lead tie-break, the banked Research pours in under each seat's own name, the log line is
    /// written, and only now may the Tech complete -- which, for a human Lead, means a Tech finished
    /// by its banked Research lands with the turn's other Moments rather than in the middle of the
    /// Orders phase. Runs at the head of `end_turn`, and for a computer seat inside `pick_tech`.
    pub fn commit_pick(&mut self) {
        if self.research.pick_committed {
            return;
        }
        let Some(tech) = self.research.current else {
            self.research.pick_committed = true;
            return;
        };
        let seat = self.research.picked_by.unwrap_or(Seat(0));
        self.research.pick_committed = true;
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
        if research <= 0 || self.seat(seat).research_directive == 0 || self.kind(seat) != FactionKind::Archivists {
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

    /// Ticket #235 (version 0.08.3): the **Research Directive**. A Faction sends a share of its
    /// Research, chosen as a percentage and standing until changed, somewhere other than the shared
    /// Tech. Read HERE, at Income, before a point reaches the Tech -- the shape version 0.07.0 gave
    /// the Archivists' switch, and for the reason recorded on `bank_archive_research`.
    ///
    /// Returns what was actually taken, which the caller keeps back from `accrue_research`. It can
    /// be less than the directive asked for: the Archive fund has a cap, and nothing is taken that
    /// cannot be used.
    pub fn spend_research_directive(&mut self, seat: Seat, research: i64) -> i64 {
        let percent = self.seat(seat).research_directive.min(self.research_directive_cap(seat));
        // Recorded BEFORE anything is spent and whatever comes of it: this is the directive that
        // was in force at this Income, and the next one settles Provisional Findings from it.
        self.seat_mut(seat).directive_last_income = percent;
        let want = if research > 0 && percent > 0 { research * percent as i64 / 100 } else { 0 };
        if want <= 0 {
            return 0;
        }
        let t = self.tables.research_directive.clone();
        let name = self.seat_name(seat);
        match self.kind(seat) {
            FactionKind::Archivists => self.bank_archive_research(seat, want),
            FactionKind::Custodians => {
                // For GOOD, not for the turn. The Sink itself moves, as a Break moves it.
                let ppm = want as f64 * t.custodians_ppm_per_point;
                self.climate.natural_sink += ppm;
                // Ticket #265 (version 0.08.4): remembered as this seat's, and credited to its
                // removal at every Climate phase from now on -- "credit", the designer said.
                self.seat_mut(seat).directive_sink += ppm;
                let sink = self.climate.natural_sink;
                let text = self.say("directive_sink", &[("faction", name), ("research", want.to_string()), ("ppm", format!("{ppm:.2}")), ("sink", format!("{sink:.2}"))]);
                self.report_line_of(seat, LineKind::YourWorks, LineKind::Note, None, text);
                want
            }
            FactionKind::Prospectors => {
                let paid = self.pay_directive_remainder(seat, want as f64 * t.prospectors_ducats_per_point);
                self.seat_mut(seat).stockpile.ducats += paid;
                let text = self.say("directive_ducats", &[("faction", name), ("research", want.to_string()), ("n", paid.to_string())]);
                self.report_line_of(seat, LineKind::YourWorks, LineKind::Note, None, text);
                want
            }
            FactionKind::Arkwrights => {
                let paid = self.pay_directive_remainder(seat, want as f64 * t.arkwrights_fuel_per_point);
                self.seat_mut(seat).stockpile.fuel += paid;
                let text = self.say("directive_fuel", &[("faction", name), ("research", want.to_string()), ("n", paid.to_string())]);
                self.report_line_of(seat, LineKind::YourWorks, LineKind::Note, None, text);
                want
            }
        }
    }

    /// Ticket #235: pay out a fractional rate, carrying what is left over to the next turn. The
    /// Prospectors earn 0.8 of a Ducat a point and the Arkwrights 0.2 of a Fuel, so flooring every
    /// turn would quietly lose up to a fifth of everything diverted.
    fn pay_directive_remainder(&mut self, seat: Seat, earned: f64) -> i64 {
        let s = self.seat_mut(seat);
        s.directive_remainder += earned;
        let whole = s.directive_remainder.floor();
        s.directive_remainder -= whole;
        whole as i64
    }

    /// Ticket #235: the most this seat may direct away from the shared Tech. The Archivists' runs
    /// to 100 -- their switch always sent ALL of it to the Archive, and the slider that replaced it
    /// keeps that reach -- where every other Faction stops at half.
    pub fn research_directive_cap(&self, seat: Seat) -> u8 {
        if self.kind(seat) == FactionKind::Archivists {
            self.tables.research_directive.archivists_max
        } else {
            self.tables.research_directive.max
        }
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

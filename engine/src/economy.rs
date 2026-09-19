//! The Income phase (spec 6 phase 1, 7.1, 7.2, 12.1).

use crate::data::VictoryFirstKind;
use crate::ids::*;
use crate::state::*;

/// One producer the shortfall rule can shut down, in the order it shuts them.
#[derive(Debug, Clone)]
struct Producer {
    place: ProducerPlace,
    name: &'static str,
    is_module: bool,
    upkeep: i64,
    output: Option<(Resource, i64)>,
    /// Materials or Fuel from a Mine, Refinery or Factory count toward the Extraction Total.
    extraction: bool,
    research: i64,
    online: bool,
    /// Ticket #82: the Facility on Earth whose mothball doubles this Module, if one does.
    doubled_by: Option<&'static str>,
}

/// One building's per-turn figures at today's multipliers, for the cards and the build buttons.
/// (Not `Copy` since ticket #90: the detail line is a String.)
#[derive(Debug, Clone, PartialEq)]
pub struct Yield {
    pub resource: Option<Resource>,
    pub amount: i64,
    pub research: i64,
    pub upkeep: i64,
    pub emissions: f64,
    /// Ticket #36: Influence Allotment added while it stands, and standing raised each turn.
    pub allotment: i64,
    pub standing: i64,
    /// Ticket #82 (version 0.06.0): the Facility on Earth whose mothball doubles this Module, if
    /// one does (the Custodians' signature).
    pub doubled_by: Option<&'static str>,
    /// Ticket #90 (version 0.06.0): how the figure was reached, for the card ("2 x 12 Colonists +
    /// 3 x 2 Bodies"), when a Module's arithmetic is worth showing.
    pub detail: Option<String>,
}

impl Yield {
    /// "+6 Materials, 2 Energy upkeep, 1.0 Emissions" in the glossary's words.
    pub fn text(&self) -> String {
        let mut parts = Vec::new();
        match self.resource {
            Some(Resource::Materials) => parts.push(format!("+{} Materials", self.amount)),
            Some(Resource::Fuel) => parts.push(format!("+{} Fuel", self.amount)),
            Some(Resource::Energy) => parts.push(format!("+{} Energy", self.amount)),
            Some(Resource::Ducats) => parts.push(format!("+{} Ducats", self.amount)),
            Some(Resource::Research) | None => {}
        }
        if self.research > 0 {
            parts.push(format!("+{} Research", self.research));
        }
        // Ticket #90: the arithmetic, when a Module has one worth showing.
        if let Some(d) = &self.detail {
            parts.push(format!("({d})"));
        }
        // Ticket #82: the Custodians' Production Moved.
        if let Some(f) = self.doubled_by {
            parts.push(format!("doubled by an idle {f} on Earth"));
        }
        if self.allotment > 0 {
            parts.push(format!("+{} Influence Allotment", self.allotment));
        }
        if self.standing > 0 {
            parts.push(format!("standing here +{} a turn", self.standing));
        }
        if parts.is_empty() {
            parts.push("no output".to_string());
        }
        if self.upkeep > 0 {
            parts.push(format!("{} Energy upkeep", self.upkeep));
        }
        if self.emissions > 0.0 {
            parts.push(format!("{:.1} Emissions", self.emissions));
        }
        parts.join(", ")
    }
}

#[derive(Debug, Clone, Copy)]
enum ProducerPlace {
    Facility(StateId, usize),
    Module(ColonyId, usize),
}

impl Game {
    /// Phase 1: Income. Standing Armies replenish, producers produce, upkeep is paid with the shortfall rule.
    pub fn income_phase(&mut self) {
        // Ticket #51: Provisional Findings holds this turn only if last turn's Research went to the
        // shared Tech, so it is settled before any yield reads a Tech.
        for seat in Seat::ALL {
            // Ticket #235 (version 0.08.3): a THRESHOLD, where this was binary. The control was a
            // switch, so any funding at all turned Provisional Findings off; it is a slider now,
            // and at the designer's word -- "make it a threshold 75%" -- the rule holds while at
            // least that share of last turn's Research still went to the shared Tech. A directive
            // of 25 or less keeps it; anything above trades it away. Both old positions are
            // unchanged: 0 keeps the rule and 100 loses it.
            // It reads the DECLARED directive, not what was actually taken. Two reasons, and the
            // test that made the difference visible is in the suite: rounding means a 26% directive
            // on 14 Research takes 3 points, which is 21% applied, so a player who set 26 would
            // keep a rule they had chosen to trade away -- and nothing on screen would explain it.
            // What a player sets is what they are answerable for.
            let contributed = 100u32.saturating_sub(self.seat(seat).directive_last_income as u32);
            let floor = self.tables.research_directive.provisional_min_contribution as u32;
            let s = self.seat_mut(seat);
            s.provisional_findings = contributed >= floor;
            s.funding_archive = false;
        }
        self.replenish_standing_armies();
        // Ticket #134 (version 0.07.3): a standing Max order ends by itself when its place is no
        // longer the seat's to spend on, and the Report says so.
        for seat in Seat::ALL {
            if let Some(place) = self.seat(seat).max_standing
                && !self.directs(seat, place)
            {
                self.seat_mut(seat).max_standing = None;
                let text = format!("The standing Max order on {} ends: it is no longer yours to spend on.", self.place_name(place));
                self.log(text.clone());
                let at = match place {
                    Place::State(s) => ReportPlace::State(s),
                    Place::Colony(c) => ReportPlace::Colony(c),
                };
                self.report_line(LineKind::Note, Some(at), text);
            }
        }
        for s in &mut self.ships {
            s.escaped = false;
            s.arrived_this_turn = false;
        }
        for a in &mut self.armies {
            a.escaped = false;
        }
        for seat in Seat::ALL {
            self.income_for(seat);
        }
        // Ticket #185 (version 0.08.0): the Schools do their work for the turn.
        self.run_schools();
        self.neutral_research();
        self.solar_maximum_next = false;
        // Ticket #76: a Drought lasts one Income.
        for s in &mut self.states {
            s.drought = false;
        }
        for d in &mut self.discoveries {
            d.turns_left = d.turns_left.saturating_sub(1);
        }
        self.discoveries.retain(|d| d.turns_left > 0);
        for seat in Seat::ALL {
            let allot = self.influence_allotment(seat);
            let s = self.seat_mut(seat);
            s.allotment = allot;
            // Ticket #183 (version 0.08.0): last turn's Spaceport Influence has now been paid into
            // this Allotment, so the tally is cleared and begins again.
            s.spaceport_influence = 0;
        }
    }

    /// Ticket #69 (version 0.05.5): a Research Lab in a Nation State nobody holds, or one under
    /// Occupation, runs itself, pays no Energy, and pays half its yield (rounded down, per Lab) into
    /// the Tech under research, counting toward nobody's Research Lead, as a Breakthrough's Research
    /// does. The yield is a Faction-less one: the row's figure by the state's people and schooling,
    /// times Public Science once the world has it. A Lab idled by a Wildfire or mothballed pays nothing.
    fn neutral_research(&mut self) {
        let mut total = 0i64;
        let mut names: Vec<String> = Vec::new();
        for sid in StateId::ALL {
            if !matches!(self.state(sid).control, Control::Neutral | Control::Occupied { .. }) {
                continue;
            }
            let labs = self.state(sid).facilities.iter().filter(|f| f.kind == FacilityKind::ResearchLab && f.working() && !f.offline_until_resolution).count() as i64;
            if labs == 0 {
                continue;
            }
            let paid = labs * (self.world_lab_yield(sid) / 2);
            if paid > 0 {
                total += paid;
                names.push(self.tables.state(sid).name.clone());
            }
        }
        if total == 0 {
            return;
        }
        self.add_research_unattributed(total);
        self.research.neutral_total += total;
        let states = names.join(" and ");
        let line = format!("The Labs of {states}, in no one's hands, added {total} Research to the Tech under research.");
        self.log(line);
        let text = self.say("neutral_research", &[("states", states), ("n", total.to_string())]);
        self.report_line(LineKind::Note, None, text);
    }

    /// Ticket #69: what one Research Lab in this state makes for the world, with no Faction's
    /// multiplier: the row's figure x the population factor x the Education Level, x Public Science
    /// once every Faction has it, rounded down.
    pub fn world_lab_yield(&self, sid: StateId) -> i64 {
        let t = &self.tables;
        let base = t.facility(FacilityKind::ResearchLab).produces.as_ref().map(|p| p.amount).unwrap_or(0) as f64;
        let public = if self.has_tech(TechId::PublicScience) { t.tech(TechId::PublicScience).value } else { 1.0 };
        // Ticket #185 (version 0.08.0): the LIVE Education Level, which a School moves.
        (base * self.population_factor(sid) * self.education_level(sid) * public).floor() as i64
    }

    fn replenish_standing_armies(&mut self) {
        for sid in StateId::ALL {
            let occupied = self.state(sid).control.is_occupied();
            let has = self.armies.iter().any(|a| a.standing && a.home == ArmyHome::State(sid));
            if !has {
                // A destroyed Standing Army is raised again at strength 1 (an assumption, see the ticket).
                let cap = self.standing_army_cap(sid);
                let hp = self.tables.unit(UnitKind::Army).hit_points;
                self.spawn_standing_army(sid);
                let dmg = cap.saturating_sub(1).min(hp - 1);
                if let Some(a) = self.armies.iter_mut().find(|a| a.standing && a.home == ArmyHome::State(sid)) {
                    a.damage = dmg;
                }
                continue;
            }
            // Ticket #52: at Unrest 4 the Standing Army stops replenishing.
            if occupied || !self.army_replenishes(sid) {
                continue;
            }
            if let Some(a) = self.armies.iter_mut().find(|a| a.standing && a.home == ArmyHome::State(sid)) {
                a.damage = a.damage.saturating_sub(1);
            }
        }
    }

    /// What one Facility in a Nation State makes each turn for its director, at today's multipliers:
    /// the same figures the Income phase pays (spec 7.1, 12.1) and the Climate phase charges (11.2).
    pub fn facility_yield(&self, seat: Seat, sid: StateId, kind: FacilityKind) -> Yield {
        let t = &self.tables;
        let fac = t.faction(self.kind(seat));
        let card = t.state(sid);
        let fc = t.facility(kind);
        let mut y = Yield { resource: None, amount: 0, research: 0, upkeep: fc.energy_upkeep, emissions: 0.0, allotment: fc.influence_allotment, standing: fc.standing_per_turn, doubled_by: None, detail: None };
        if let Some(p) = &fc.produces {
            match p.resource {
                Resource::Research => {
                    // Ticket #69: an Occupied state's Lab works for the world (`neutral_research`),
                    // not for the occupier, who pays its upkeep and draws nothing.
                    if self.state(sid).control.is_occupied() {
                        y.research = 0;
                    } else {
                        // Ticket #185 (version 0.08.0): the LIVE Education Level, which a School moves.
                        let mut r = p.amount as f64 * self.population_factor(sid) * self.education_level(sid) * fac.research_multiplier;
                        r *= self.tech_multiplier(seat, TechId::PublicScience);
                        // Ticket #84: the Upload stacks on Public Science.
                        r *= self.tech_multiplier(seat, TechId::TheUpload);
                        y.research = r.floor() as i64;
                    }
                }
                Resource::Ducats => {
                    // A Bank (ticket #35): its amount times the state's gdp / 10.
                    let v = p.amount as f64 * card.gdp as f64 / 10.0 * fac.output_multiplier;
                    y.resource = Some(Resource::Ducats);
                    y.amount = v.floor() as i64;
                }
                res => {
                    let mut v = p.amount as f64;
                    if card.resource_lean == res {
                        v *= 1.5;
                    }
                    v *= fac.output_multiplier;
                    v *= self.tech_output_multiplier_facility(seat, kind);
                    y.resource = Some(res);
                    y.amount = v.floor() as i64;
                }
            }
        }
        let pp = self.tech_multiplier(seat, TechId::CleanPower);
        let fr = self.tech_multiplier(seat, TechId::CleanManufacturing);
        y.emissions = fc.emissions
            * fac.emissions_multiplier
            * match kind.common().unwrap_or(kind) {
                FacilityKind::PowerPlant => pp,
                FacilityKind::Factory | FacilityKind::Refinery => fr,
                _ => 1.0,
            };
        // Ticket #54: while a Strip Permit runs, every Facility in the state produces double. It
        // multiplies the output, never the Emissions: the price is paid once, when the permit ends.
        if self.strip_permit_running(sid) {
            let m = self.tables.strip_permit.multiplier;
            y.amount = (y.amount as f64 * m).floor() as i64;
            y.research = (y.research as f64 * m).floor() as i64;
        }
        // Ticket #52: at Unrest 7 every Facility in the state produces at half, rounded down, and
        // emits at half. What it adds to the Allotment and to the standing is untouched.
        if self.facilities_at_half(sid) {
            y.amount /= 2;
            y.research /= 2;
            y.emissions *= 0.5;
        }
        y
    }

    /// What one Module in a Colony makes each turn for its director. Modules never emit.
    pub fn module_yield(&self, seat: Seat, cid: ColonyId, kind: ModuleKind) -> Yield {
        let t = &self.tables;
        let fac = t.faction(self.kind(seat));
        let mc = t.module(kind);
        let mut y = Yield { resource: None, amount: 0, research: 0, upkeep: mc.energy_upkeep, emissions: 0.0, allotment: mc.influence_allotment, standing: mc.standing_per_turn, doubled_by: None, detail: None };
        // Ticket #232 (version 0.08.3): Relay Networks takes a Relay's Allotment from 1 to 2. Its
        // Standing is untouched: Standing holds one particular place, where the Allotment is the
        // Faction's whole diplomatic budget, and "+1 influence" reads as the budget everywhere
        // else in the game.
        if kind == ModuleKind::Relay {
            y.allotment += self.tech_addition(seat, TechId::RelayNetworks);
        }
        let Some(col) = self.colony(cid) else { return y };
        // Ticket #57: the yield is the Colony Slot's own, not its Body's. The Body's figures are
        // what the slot drew from when the game started; a station in orbit keeps the Body's.
        if let Some(p) = &mc.produces {
            if kind == ModuleKind::TradePost {
                // Ticket #90 (version 0.06.0): trade is a network. `amount` Ducats per Colonist of
                // the Faction at this Body, plus `per_other_body` for every other Body the Faction
                // holds; no Body yield; the Faction's output multiplier applies.
                let here = self.colonists_at_body(seat, col.body) as i64;
                let others = self.bodies_held(seat).into_iter().filter(|b| *b != col.body).count() as i64;
                let per_other = t.trade_post.per_other_body;
                let raw = p.amount * here + per_other * others;
                y.resource = Some(Resource::Ducats);
                y.amount = (raw as f64 * fac.output_multiplier).floor() as i64;
                y.detail = Some(format!("{} x {here} Colonists + {per_other} x {others} Bodies", p.amount));
            } else if p.resource == Resource::Research {
                // Ticket #80 (version 0.06.0): the Observatory. No Body yield and no output
                // multiplier: its amount, plus one per cent for every Colonist at its Colony, times
                // the Faction's Research multiplier and Public Science, rounded down, as a Lab is.
                let per = t.observatory.research_per_colonist;
                // Ticket #81: a Faction may carry a second Research multiplier for off Earth (the
                // Archivists' 1.75), a station over Earth counting as off and Antarctica as on.
                let research_multiplier = if self.off_earth(col) { fac.research_multiplier_off_earth.unwrap_or(fac.research_multiplier) } else { fac.research_multiplier };
                // Ticket #140 (version 0.07.3): the slot's Research yield on the ground, the Body's
                // on a station -- the first Body yield a station has read. The designer traded the
                // Habitat yield for it: *"Replace habitat bonuses with science bonuses."*
                let science = self.research_yield_at(col);
                // Ticket #189 (version 0.08.0): and its Colony's Education Level -- the weighted
                // average of the people who settled it, raised by an Institute -- exactly as a
                // Research Lab reads its Region's. This is what gives a Colony's figure a job.
                // Ticket #188 (version 0.08.0): and the per-Colonist bonus is moderated by the
                // Colony's schooling too, exactly as a Region's population bonus is, so the rule
                // reads the same in both halves of the game.
                let mut r = p.amount as f64 * science * col.education * (1.0 + col.colonists as f64 * per * col.education) * research_multiplier;
                r *= self.tech_multiplier(seat, TechId::PublicScience);
                // Ticket #84: the Upload stacks on Public Science.
                r *= self.tech_multiplier(seat, TechId::TheUpload);
                y.research = r.floor() as i64;
            } else {
                // Ticket #89: a sun-scaled Module (the Solar Array) reads the sunlight where its
                // Body stands instead of a Body yield, is silenced by a Solar Storm turn, and
                // rounds to the nearest whole.
                let yield_ = if mc.sun_scaled { self.sun_factor(col.body) } else { self.colony_yields(col).of_module(kind) };
                let mut v = p.amount as f64 * yield_ * fac.output_multiplier * self.tech_output_multiplier_module(seat, kind);
                for d in &self.discoveries {
                    if d.body == col.body && d.kind == kind {
                        v *= d.multiplier;
                    }
                }
                if mc.sun_scaled && self.event_is(EventId::SolarStorm) {
                    v = 0.0;
                }
                y.resource = Some(p.resource);
                y.amount = if mc.sun_scaled { v.round() as i64 } else { v.floor() as i64 };
                // Ticket #92: a working Mass Driver at the Colony gives each Mine there more, after
                // everything.
                if kind == ModuleKind::Mine && col.modules.iter().any(|m| m.kind == ModuleKind::MassDriver && m.working()) {
                    y.amount += t.mass_driver.mine_bonus;
                }
            }
        }
        // Ticket #92: a Mass Driver makes nothing itself; the card says what it does.
        if kind == ModuleKind::MassDriver {
            let md = &t.mass_driver;
            y.detail = Some(format!("departures from here {} Fuel cheaper, never under {}; each Mine here +{} Materials", md.fuel_off, md.fuel_min, md.mine_bonus));
        }
        // Ticket #51: the Archive draws its Energy only once it is complete; ticket #68: that is
        // standing with its Research paid in full. Until then it costs nothing to run.
        if kind == ModuleKind::Archive {
            let complete = self.seat(seat).archive_fund >= t.archive.research;
            y.upkeep = if complete { mc.energy_upkeep } else { 0 };
        }
        y.upkeep = (y.upkeep as f64 * self.tech_multiplier(seat, TechId::ClosedLoopColonies)).floor() as i64;
        y
    }

    /// Ticket #82 (version 0.06.0): the Modules a seat's mothballed Facilities on Earth double,
    /// as (Colony, Module index), in the order they were paired.
    pub fn doubled_modules(&self, seat: Seat) -> Vec<(ColonyId, usize)> {
        let pairs = &self.tables.faction(self.kind(seat)).mothball_pairs;
        let mut out = Vec::new();
        if pairs.is_empty() {
            return out;
        }
        for (fk, mk) in pairs {
            // Ticket #82: one idle Facility of the kind doubles one Module of its pair, the most
            // productive undoubled one off Earth first (a station over Earth is off Earth;
            // Antarctica is not); with no idle Facility there is no bonus.
            let idle = self.directed_states(seat).iter().flat_map(|s| self.state(*s).facilities.iter()).filter(|f| f.kind == *fk && f.mothballed).count();
            if idle == 0 {
                continue;
            }
            let mut candidates: Vec<((ColonyId, usize), i64)> = Vec::new();
            for cid in self.directed_colonies(seat) {
                let col = self.colony(cid).unwrap();
                if !self.off_earth(col) {
                    continue;
                }
                for (i, m) in col.modules.iter().enumerate() {
                    if m.kind == *mk && !m.mothballed {
                        let y = self.module_yield(seat, cid, *mk);
                        candidates.push(((cid, i), y.amount.max(y.research)));
                    }
                }
            }
            candidates.sort_by_key(|c| std::cmp::Reverse(c.1));
            out.extend(candidates.into_iter().take(idle).map(|(k, _)| k));
        }
        out
    }

    /// Ticket #82: one Module's figures, the doubling applied on the final figure if it is one of
    /// `doubled_modules`, named for the Facility whose mothball pays for it.
    pub fn module_yield_at(&self, seat: Seat, cid: ColonyId, index: usize) -> Yield {
        let Some(kind) = self.colony(cid).and_then(|c| c.modules.get(index)).map(|m| m.kind) else {
            return Yield { resource: None, amount: 0, research: 0, upkeep: 0, emissions: 0.0, allotment: 0, standing: 0, doubled_by: None, detail: None };
        };
        let mut y = self.module_yield(seat, cid, kind);
        if self.doubled_modules(seat).contains(&(cid, index)) {
            let pairs = &self.tables.faction(self.kind(seat)).mothball_pairs;
            y.amount *= 2;
            y.research *= 2;
            y.doubled_by = pairs.iter().find(|(_, m)| **m == kind).map(|(f, _)| f.name());
        }
        y
    }

    fn producers_of(&self, seat: Seat) -> Vec<Producer> {
        let mut out = Vec::new();
        for sid in self.directed_states(seat) {
            let st = self.state(sid);
            for (i, f) in st.facilities.iter().enumerate() {
                // Ticket #54: a mothballed Facility makes nothing and pays no Energy upkeep, so it
                // is not on the shortfall list at all and Income never turns it back on.
                if f.mothballed {
                    continue;
                }
                let y = self.facility_yield(seat, sid, f.kind);
                // Ticket #76: a Drought halves what the state's Facilities make at this Income.
                let dry = if st.drought { self.tables.events.drought_output_multiplier } else { 1.0 };
                let halve = |v: i64| if st.drought { (v as f64 * dry).floor() as i64 } else { v };
                out.push(Producer {
                    place: ProducerPlace::Facility(sid, i),
                    name: f.kind.name(),
                    is_module: false,
                    upkeep: y.upkeep,
                    output: y.resource.map(|r| (r, halve(y.amount))),
                    extraction: matches!(f.kind, FacilityKind::Factory | FacilityKind::Refinery),
                    research: halve(y.research),
                    online: !f.offline_until_resolution,
                    doubled_by: None,
                });
            }
        }
        for cid in self.directed_colonies(seat) {
            let col = self.colony(cid).unwrap();
            // Ticket #51: an Occupied Colony's Archive is offline, whoever is directing the Colony.
            let occupied = col.control.is_occupied();
            for (i, m) in col.modules.iter().enumerate() {
                // Ticket #54: a mothballed Module, the same way.
                if m.mothballed {
                    continue;
                }
                // Ticket #82: the doubling, if this Module has one, is on the figure.
                let y = self.module_yield_at(seat, cid, i);
                out.push(Producer {
                    place: ProducerPlace::Module(cid, i),
                    name: m.kind.name(),
                    is_module: true,
                    upkeep: y.upkeep,
                    output: y.resource.map(|r| (r, y.amount)),
                    extraction: matches!(m.kind, ModuleKind::Mine | ModuleKind::Refinery),
                    // Ticket #80: an Observatory's Research.
                    research: y.research,
                    online: !col.grid_failed && !m.offline_until_resolution && !(m.kind == ModuleKind::Archive && occupied),
                    doubled_by: y.doubled_by,
                });
            }
        }
        out
    }

    /// Solar Maximum (ticket #25): Power Plants and Generators make more at the next Income.
    fn solar_maximum_multiplier(&self) -> f64 {
        if !self.solar_maximum_next {
            return 1.0;
        }
        let e = &self.tables.events;
        // A Solar Maximum reads a whole Tech or none: it picks between two Event figures rather than
        // scaling one, so Provisional Findings, which halves an effect, has nothing to halve here.
        if self.has_tech(TechId::EfficientGrids) { e.solar_maximum_multiplier_with_tech } else { e.solar_maximum_multiplier }
    }

    fn tech_output_multiplier_facility(&self, seat: Seat, kind: FacilityKind) -> f64 {
        // Ticket #181 (version 0.08.0): a Unique Facility takes the Techs of the job it does, so
        // Efficient Grids reaches a Reactor and the Solar Maximum bites it.
        let kind = kind.common().unwrap_or(kind);
        let mut m = 1.0;
        if kind == FacilityKind::PowerPlant {
            m *= self.solar_maximum_multiplier();
        }
        match kind {
            FacilityKind::PowerPlant => m *= self.tech_multiplier(seat, TechId::EfficientGrids),
            FacilityKind::Factory => m *= self.tech_multiplier(seat, TechId::DeepMining),
            FacilityKind::Refinery => m *= self.tech_multiplier(seat, TechId::AutomatedRefining),
            _ => {}
        }
        m
    }

    /// Ticket #232 (version 0.08.3): `pub(crate)` so the AI can read the TECH-ONLY factor when
    /// weighing a build. It must not read the finished yield instead: that carries the slot's own
    /// yield, which is at least 1 everywhere, so a weight scaled by it would lift every Mine on
    /// the board with no Tech researched at all. Measured when this was built the wrong way round:
    /// Mines standing over twenty games went from 16 to 329.
    pub(crate) fn tech_output_multiplier_module(&self, seat: Seat, kind: ModuleKind) -> f64 {
        let mut m = 1.0;
        // Ticket #89: a Solar Array reads the Sun as a Generator does.
        if matches!(kind, ModuleKind::Generator | ModuleKind::SolarArray) {
            m *= self.solar_maximum_multiplier();
        }
        match kind {
            ModuleKind::Generator | ModuleKind::SolarArray => m *= self.tech_multiplier(seat, TechId::EfficientGrids),
            // Ticket #84: the Extraction Charter stacks on Deep Mining.
            // Ticket #232 (version 0.08.3): and Beneficiation stacks on both, INSIDE this one
            // chain, so the whole product is floored once by the caller. A tenth applied after a
            // rounding would have been nothing at all on a small Colony.
            ModuleKind::Mine => {
                m *= self.tech_multiplier(seat, TechId::DeepMining) * self.tech_multiplier(seat, TechId::ExtractionCharter) * self.tech_multiplier(seat, TechId::Beneficiation)
            }
            ModuleKind::Refinery => m *= self.tech_multiplier(seat, TechId::AutomatedRefining),
            _ => {}
        }
        m
    }

    /// A controlled state's base Ducats a turn (ticket #35): gdp x Industry Level / 10, rounded down,
    /// the formula living in `Tables::base_ducats` since ticket #132 so the start screen reads the same one.
    /// Ticket #83 (version 0.06.0): times its controller's `ducats_multiplier` (the Prospectors' 1.2).
    pub fn state_ducats(&self, sid: StateId) -> i64 {
        let base = self.tables.base_ducats(sid, self.state(sid).industry_level);
        let m = self.state(sid).control.controller().map(|s| self.tables.faction(self.kind(s)).ducats_multiplier).unwrap_or(1.0);
        (base as f64 * m).floor() as i64
    }

    /// Upkeep of every Ship and non-standing Army of a seat; always paid first (spec 7.2).
    pub fn unit_upkeep(&self, seat: Seat) -> i64 {
        let ships: i64 = self.ships.iter().filter(|s| s.seat == seat).map(|s| self.tables.unit(s.kind).energy_upkeep).sum();
        let armies: i64 = self
            .armies
            .iter()
            .filter(|a| !a.standing && self.army_seat(a) == Some(seat))
            .map(|_| self.tables.unit(UnitKind::Army).energy_upkeep)
            .sum();
        ships + armies
    }

    /// The names of producers shut down by the shortfall rule, in the order they would be shut.
    pub fn shortfall_order(&self, seat: Seat) -> Vec<String> {
        let mut producers = self.producers_of(seat);
        let (_, shut) = self.apply_shortfall(seat, &mut producers);
        shut
    }

    /// Runs the shortfall rule over a producer list; returns the final Energy balance and what was shut.
    /// Ticket #184 (version 0.08.0): what an online Reactor gives its holder back of this turn's
    /// Energy bill -- the difference between the whole upkeep of everything it owns and
    /// `reactor_upkeep` of it, taken off the TOTAL and floored once. The Archive is left out of the
    /// sum and pays its own figure in full, at the designer's word.
    ///
    /// Off the total and never per building: upkeep figures are small whole numbers and most of what
    /// a seat owns costs 2 or 3, so per building a 75% rule floors to a 50% cut on a 2 and a 33% cut
    /// on a 3, which would make the cheapest building the most Energy-efficient thing to own. **No
    /// stacking**: a second Reactor is an ordinary power station worth its 6 Energy and nothing more.
    /// Ships and Armies are not buildings and their upkeep is untouched.
    ///
    /// Only a CONTROLLER collects: an occupier pays a Unique Facility's upkeep and draws nothing from
    /// its clause, which is the precedent ticket #69 set for an occupied Research Lab.
    /// Ticket #181 (version 0.08.0): whether this seat CONTROLS the place a producer stands in.
    /// Every Unique Facility clause reads this rather than direction, because an occupier pays a
    /// Unique Facility's upkeep and draws nothing from its clause until control transfers -- the
    /// precedent ticket #69 set for an occupied Research Lab, which works for the world and not for
    /// the occupier.
    fn controls_producer(&self, seat: Seat, place: ProducerPlace) -> bool {
        match place {
            ProducerPlace::Facility(sid, _) => self.state(sid).control == Control::Controlled(seat),
            ProducerPlace::Module(cid, _) => self.colony(cid).map(|c| c.control == Control::Controlled(seat)).unwrap_or(false),
        }
    }

    fn reactor_relief(&self, seat: Seat, producers: &[Producer]) -> i64 {
        let lit = producers.iter().any(|p| {
            p.online
                && p.name == FacilityKind::Reactor.name()
                && matches!(p.place, ProducerPlace::Facility(sid, _) if self.state(sid).control == Control::Controlled(seat))
        });
        if !lit {
            return 0;
        }
        let archive = ModuleKind::Archive.name();
        let bill: i64 = producers.iter().filter(|p| p.online && p.name != archive).map(|p| p.upkeep).sum();
        bill - (bill as f64 * self.tables.unique.reactor_upkeep).floor() as i64
    }

    fn apply_shortfall(&self, seat: Seat, producers: &mut [Producer]) -> (i64, Vec<String>) {
        // Ticket #184: the balance is recomputed from scratch whenever a building goes dark, because
        // the Reactor's relief is a share of the bill and shrinks with it.
        let balance_now = |g: &Self, ps: &[Producer]| -> i64 {
            let energy_in: i64 = ps.iter().filter(|p| p.online).filter_map(|p| p.output.filter(|(r, _)| *r == Resource::Energy).map(|(_, v)| v)).sum();
            let upkeep: i64 = ps.iter().filter(|p| p.online).map(|p| p.upkeep).sum();
            g.seat(seat).stockpile.energy + energy_in - upkeep - g.unit_upkeep(seat) + g.reactor_relief(seat, ps)
        };
        let mut balance = balance_now(self, producers);
        // Ticket #164 (version 0.07.5): the Core Module is never shut for want of Energy. It is the
        // walls of the place rather than a building in it -- it cannot be mothballed either -- so
        // its upkeep is paid whatever else goes dark.
        let mut order: Vec<usize> =
            (0..producers.len()).filter(|i| producers[*i].online && producers[*i].upkeep > 0 && producers[*i].name != ModuleKind::Core.name()).collect();
        order.sort_by(|a, b| {
            let pa = &producers[*a];
            let pb = &producers[*b];
            pb.upkeep
                .cmp(&pa.upkeep)
                .then_with(|| pb.is_module.cmp(&pa.is_module))
                .then_with(|| pa.name.cmp(pb.name))
        });
        let mut shut = Vec::new();
        for i in order {
            if balance >= 0 {
                break;
            }
            producers[i].online = false;
            shut.push(producers[i].name.to_string());
            balance = balance_now(self, producers);
        }
        (balance, shut)
    }

    fn income_for(&mut self, seat: Seat) {
        let mut producers = self.producers_of(seat);
        let (balance, shut) = self.apply_shortfall(seat, &mut producers);
        let mut gained = Stockpile::default();
        let mut research = 0;
        let mut off_earth = 0;
        let mut doubled_turns = 0;
        let mut extraction = 0;
        let mut sources: Vec<(String, Resource, i64)> = Vec::new();
        for p in &producers {
            let where_ = match p.place {
                ProducerPlace::Facility(sid, i) => {
                    self.state_mut(sid).facilities[i].online = p.online;
                    self.tables.state(sid).name.clone()
                }
                ProducerPlace::Module(cid, i) => {
                    if let Some(c) = self.colony_mut(cid) {
                        c.modules[i].online = p.online;
                    }
                    self.place_name(Place::Colony(cid))
                }
            };
            if !p.online {
                continue;
            }
            // Ticket #82: a doubled Module says what doubled it, and is counted for the measurement.
            let where_ = match p.doubled_by {
                Some(f) => {
                    doubled_turns += 1;
                    format!("{where_} (doubled by an idle {f} on Earth)")
                }
                None => where_,
            };
            research += p.research;
            if p.research > 0 {
                sources.push((format!("{} in {}", p.name, where_), Resource::Research, p.research));
                // Ticket #80: Research made off Earth, for the measurement.
                if let ProducerPlace::Module(cid, _) = p.place
                    && self.colony(cid).map(|c| self.off_earth(c)).unwrap_or(false)
                {
                    off_earth += p.research;
                }
            }
            if let Some((res, v)) = p.output {
                match res {
                    Resource::Materials => gained.materials += v,
                    Resource::Fuel => gained.fuel += v,
                    Resource::Energy => gained.energy += v,
                    Resource::Ducats => gained.ducats += v,
                    Resource::Research => {}
                }
                sources.push((format!("{} in {}", p.name, where_), res, v));
                // Ticket #72: the Materials output the Venture Capital Fund takes its share of.
                if p.extraction && res == Resource::Materials {
                    extraction += v;
                }
            }
            if p.upkeep > 0 {
                sources.push((format!("{} in {} (upkeep)", p.name, where_), Resource::Energy, -p.upkeep));
            }
        }
        let unit_upkeep = self.unit_upkeep(seat);
        if unit_upkeep > 0 {
            sources.push(("Ships and Armies (upkeep)".to_string(), Resource::Energy, -unit_upkeep));
        }
        // Ticket #35: every controlled state's economy pays Ducats, gdp x Industry Level / 10.
        for sid in self.controlled_states(seat) {
            let v = self.state_ducats(sid);
            if v > 0 {
                gained.ducats += v;
                sources.push((format!("Economy of {}", self.tables.state(sid).name), Resource::Ducats, v));
            }
        }
        // Ticket #186 (version 0.08.0): every Academy the seat CONTROLS pays a flat Ducat while it
        // is online, wherever it stands -- Region, Colony or station. Flat rather than scaled by GDP,
        // which would make it a second Bank built where the money already is rather than where
        // schooling is wanted. It is also why a captured Academy pays its captor exactly what it paid
        // its builder: 1 run through the largest output multiplier in the game, x1.25, floors to 1.
        let academies = producers
            .iter()
            .filter(|p| p.online && p.name == FacilityKind::Academy.name() && self.controls_producer(seat, p.place))
            .count() as i64;
        let academy_ducats = academies * self.tables.unique.academy_ducats;
        if academy_ducats > 0 {
            gained.ducats += academy_ducats;
            let word = if academies == 1 { "Academy" } else { "Academies" };
            sources.push((format!("{academies} {word}"), Resource::Ducats, academy_ducats));
        }
        // Ticket #182 (version 0.08.0): the Investment Bank's interest. ONE per Region pays, however
        // many stand there, on the Venture Capital Fund's balance BEFORE this turn's banking is added
        // -- which is the balance here, since the banking below is what adds it. The Materials are
        // CREATED rather than drawn from the Stockpile, and they go straight into the Fund, so they
        // compound from next turn. A floor of one applies to ONE building only, Faction-wide, so an
        // early Investment Bank is never literally worthless without paying ~500 over a game.
        //
        // In a non-Prospector's hands the same share applies to that Faction's DUCAT INCOME instead,
        // at a minimum of one Ducat, because no other Faction has a Fund for it to pay into. The cap
        // travels with the clause, so capturing a Prospector Region is never better than being the
        // Prospectors there.
        let mut paying_regions: std::collections::BTreeSet<StateId> = std::collections::BTreeSet::new();
        for p in producers.iter().filter(|p| p.online && p.name == FacilityKind::InvestmentBank.name()) {
            if let ProducerPlace::Facility(sid, _) = p.place
                && self.state(sid).control == Control::Controlled(seat)
            {
                paying_regions.insert(sid);
            }
        }
        let u_interest = self.tables.unique.investment_bank_interest;
        let u_floor = self.tables.unique.investment_bank_floor;
        let mut interest_to_fund = 0i64;
        if !paying_regions.is_empty() {
            let n = paying_regions.len() as i64;
            if self.tables.faction(self.kind(seat)).victory_first.kind == VictoryFirstKind::VentureFund {
                let per = (self.seat(seat).venture_fund as f64 * u_interest).floor() as i64;
                interest_to_fund = (n * per).max(u_floor);
                sources.push((format!("{n} Investment Bank (interest banked)"), Resource::Materials, interest_to_fund));
            } else {
                let per = ((gained.ducats as f64 * u_interest).floor() as i64).max(u_floor);
                let paid = n * per;
                gained.ducats += paid;
                sources.push((format!("{n} Investment Bank (interest)"), Resource::Ducats, paid));
            }
        }
        // Ticket #72 (version 0.05.5): the Venture Capital Fund takes its share of the Materials the
        // seat's Factories and Mines paid, rounded down, before the Stockpile sees them.
        let share = self.seat(seat).venture_share;
        let banked = if share > 0.0 { (extraction as f64 * share).floor() as i64 } else { 0 };
        if banked > 0 {
            gained.materials -= banked;
            sources.push(("Venture Capital Fund (banked)".to_string(), Resource::Materials, -banked));
        }
        // Ticket #226 (version 0.08.2): a research agreement pays both parties a tenth more Research
        // while it stands, applied here -- after the buildings' own multipliers, on the Faction's
        // income, so it stacks with Public Science and with Provisional Findings rather than
        // competing with them. It reaches the Tech Tree, which already completes with the game half
        // run, so the sweep must report the turn the tree finishes.
        let agreement = self.research_agreement_multiplier(seat);
        if agreement > 1.0 && research > 0 {
            let lifted = (research as f64 * agreement).floor() as i64;
            if lifted > research {
                sources.push(("Research agreement".to_string(), Resource::Research, lifted - research));
                research = lifted;
            }
        }
        self.seat_mut(seat).income_sources = sources;
        let before = self.seat(seat).stockpile;
        let clamped = balance.max(0);
        {
            let s = self.seat_mut(seat);
            s.stockpile.materials += gained.materials;
            s.stockpile.fuel += gained.fuel;
            s.stockpile.energy = clamped;
            s.stockpile.ducats += gained.ducats;
            s.income_last_turn = Stockpile {
                materials: gained.materials,
                fuel: gained.fuel,
                energy: clamped - before.energy,
                ducats: gained.ducats,
            };
            s.research_last_turn = research;
            // Ticket #50: every seat keeps both running totals; a Faction's card says which one its
            // Victory Condition counts.
            s.research_total += research;
            s.research_off_earth_total += off_earth;
            s.doubled_module_turns += doubled_turns;
            s.venture_fund += banked + interest_to_fund;
            s.venture_banked_last_turn = banked;
        }
        if !shut.is_empty() {
            let line = format!("{}: Energy ran short; shut down {}.", self.seat_name(seat), shut.join(", "));
            self.log(line);
            let text = self.say("energy_short", &[("faction", self.seat_name(seat)), ("buildings", shut.join(", "))]);
            self.report_line_of(seat, LineKind::YourWorks, LineKind::Note, None, text);
        }
        if balance < 0 {
            let line = format!("{}: Energy fell to zero even with every producer off.", self.seat_name(seat));
            self.log(line);
            let text = self.say("energy_zero", &[("faction", self.seat_name(seat))]);
            self.report_line_of(seat, LineKind::YourWorks, LineKind::Note, None, text);
        }
        // Version 0.07.0: the Archivists' standing declaration is read here, before a point of
        // Research reaches the shared Tech. What the fund has room for never enters the Tech at
        // all, so a turn that completes a Tech can no longer swallow the whole payment.
        let banked = self.spend_research_directive(seat, research);
        self.accrue_research(seat, research - banked);
        self.log(format!(
            "Income {}: +{} Materials, +{} Fuel, Energy {} -> {}, Research {}.",
            self.seat_name(seat),
            gained.materials,
            gained.fuel,
            before.energy,
            clamped,
            research
        ));
    }
}

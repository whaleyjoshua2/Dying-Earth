//! The Income phase (spec 6 phase 1, 7.1, 7.2, 12.1).

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
}

/// One building's per-turn figures at today's multipliers, for the cards and the build buttons.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Yield {
    pub resource: Option<Resource>,
    pub amount: i64,
    pub research: i64,
    pub upkeep: i64,
    pub emissions: f64,
    /// Ticket #36: Influence Allotment added while it stands, and standing raised each turn.
    pub allotment: i64,
    pub standing: i64,
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
            let funded = self.seat(seat).funding_archive;
            let s = self.seat_mut(seat);
            s.provisional_findings = !funded;
            s.funding_archive = false;
        }
        self.replenish_standing_armies();
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
            self.seat_mut(seat).allotment = allot;
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
        (base * self.population_factor(sid) * t.state(sid).education_level * public).floor() as i64
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
        let mut y = Yield { resource: None, amount: 0, research: 0, upkeep: fc.energy_upkeep, emissions: 0.0, allotment: fc.influence_allotment, standing: fc.standing_per_turn };
        if let Some(p) = &fc.produces {
            match p.resource {
                Resource::Research => {
                    // Ticket #69: an Occupied state's Lab works for the world (`neutral_research`),
                    // not for the occupier, who pays its upkeep and draws nothing.
                    if self.state(sid).control.is_occupied() {
                        y.research = 0;
                    } else {
                        let mut r = p.amount as f64 * self.population_factor(sid) * card.education_level * fac.research_multiplier;
                        r *= self.tech_multiplier(seat, TechId::PublicScience);
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
            * match kind {
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
        let mut y = Yield { resource: None, amount: 0, research: 0, upkeep: mc.energy_upkeep, emissions: 0.0, allotment: mc.influence_allotment, standing: mc.standing_per_turn };
        let Some(col) = self.colony(cid) else { return y };
        // Ticket #57: the yield is the Colony Slot's own, not its Body's. The Body's figures are
        // what the slot drew from when the game started; a station in orbit keeps the Body's.
        if let Some(p) = &mc.produces {
            let yield_ = self.colony_yields(col).of_module(kind);
            let mut v = p.amount as f64 * yield_ * fac.output_multiplier * self.tech_output_multiplier_module(seat, kind);
            for d in &self.discoveries {
                if d.body == col.body && d.kind == kind {
                    v *= d.multiplier;
                }
            }
            y.resource = Some(p.resource);
            y.amount = v.floor() as i64;
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
                let y = self.module_yield(seat, cid, m.kind);
                out.push(Producer {
                    place: ProducerPlace::Module(cid, i),
                    name: m.kind.name(),
                    is_module: true,
                    upkeep: y.upkeep,
                    output: y.resource.map(|r| (r, y.amount)),
                    extraction: matches!(m.kind, ModuleKind::Mine | ModuleKind::Refinery),
                    research: 0,
                    online: !col.grid_failed && !m.offline_until_resolution && !(m.kind == ModuleKind::Archive && occupied),
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

    fn tech_output_multiplier_module(&self, seat: Seat, kind: ModuleKind) -> f64 {
        let mut m = 1.0;
        if kind == ModuleKind::Generator {
            m *= self.solar_maximum_multiplier();
        }
        match kind {
            ModuleKind::Generator => m *= self.tech_multiplier(seat, TechId::EfficientGrids),
            ModuleKind::Mine => m *= self.tech_multiplier(seat, TechId::DeepMining),
            ModuleKind::Refinery => m *= self.tech_multiplier(seat, TechId::AutomatedRefining),
            _ => {}
        }
        m
    }

    /// A controlled state's base Ducats a turn (ticket #35): gdp x Industry Level / 10, rounded down.
    pub fn state_ducats(&self, sid: StateId) -> i64 {
        (self.tables.state(sid).gdp * self.state(sid).industry_level as i64) / 10
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
    fn apply_shortfall(&self, seat: Seat, producers: &mut [Producer]) -> (i64, Vec<String>) {
        let energy_in: i64 = producers
            .iter()
            .filter(|p| p.online)
            .filter_map(|p| p.output.filter(|(r, _)| *r == Resource::Energy).map(|(_, v)| v))
            .sum();
        let upkeep: i64 = producers.iter().filter(|p| p.online).map(|p| p.upkeep).sum();
        let mut balance = self.seat(seat).stockpile.energy + energy_in - upkeep - self.unit_upkeep(seat);
        let mut order: Vec<usize> = (0..producers.len()).filter(|i| producers[*i].online && producers[*i].upkeep > 0).collect();
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
            let p = &mut producers[i];
            p.online = false;
            balance += p.upkeep;
            if let Some((Resource::Energy, v)) = p.output {
                balance -= v;
            }
            shut.push(p.name.to_string());
        }
        (balance, shut)
    }

    fn income_for(&mut self, seat: Seat) {
        let mut producers = self.producers_of(seat);
        let (balance, shut) = self.apply_shortfall(seat, &mut producers);
        let mut gained = Stockpile::default();
        let mut research = 0;
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
            research += p.research;
            if p.research > 0 {
                sources.push((format!("{} in {}", p.name, where_), Resource::Research, p.research));
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
        // Ticket #72 (version 0.05.5): the Venture Capital Fund takes its share of the Materials the
        // seat's Factories and Mines paid, rounded down, before the Stockpile sees them.
        let share = self.seat(seat).venture_share;
        let banked = if share > 0.0 { (extraction as f64 * share).floor() as i64 } else { 0 };
        if banked > 0 {
            gained.materials -= banked;
            sources.push(("Venture Capital Fund (banked)".to_string(), Resource::Materials, -banked));
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
            s.venture_fund += banked;
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
        self.accrue_research(seat, research);
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

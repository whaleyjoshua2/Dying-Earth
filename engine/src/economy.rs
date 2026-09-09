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
        self.solar_maximum_next = false;
        for d in &mut self.discoveries {
            d.turns_left = d.turns_left.saturating_sub(1);
        }
        self.discoveries.retain(|d| d.turns_left > 0);
        for seat in Seat::ALL {
            let allot = self.influence_allotment(seat);
            self.seat_mut(seat).allotment = allot;
        }
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
            if occupied {
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
        let mut y = Yield { resource: None, amount: 0, research: 0, upkeep: fc.energy_upkeep, emissions: 0.0 };
        if let Some(p) = &fc.produces {
            match p.resource {
                Resource::Research => {
                    let mut r = p.amount as f64 * self.population_factor(sid) * card.education_level * fac.research_multiplier;
                    if self.has_tech(TechId::PublicScience) {
                        r *= t.tech(TechId::PublicScience).value;
                    }
                    y.research = r.floor() as i64;
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
                    v *= self.tech_output_multiplier_facility(kind);
                    y.resource = Some(res);
                    y.amount = v.floor() as i64;
                }
            }
        }
        let pp = if self.has_tech(TechId::CleanPower) { t.tech(TechId::CleanPower).value } else { 1.0 };
        let fr = if self.has_tech(TechId::CleanManufacturing) { t.tech(TechId::CleanManufacturing).value } else { 1.0 };
        y.emissions = fc.emissions
            * fac.emissions_multiplier
            * match kind {
                FacilityKind::PowerPlant => pp,
                FacilityKind::Factory | FacilityKind::Refinery => fr,
                _ => 1.0,
            };
        y
    }

    /// What one Module in a Colony makes each turn for its director. Modules never emit.
    pub fn module_yield(&self, seat: Seat, cid: ColonyId, kind: ModuleKind) -> Yield {
        let t = &self.tables;
        let fac = t.faction(self.kind(seat));
        let mc = t.module(kind);
        let mut y = Yield { resource: None, amount: 0, research: 0, upkeep: mc.energy_upkeep, emissions: 0.0 };
        let Some(col) = self.colony(cid) else { return y };
        let body = t.body(col.body);
        if let Some(p) = &mc.produces {
            let yield_ = match kind {
                ModuleKind::Mine => body.mine_yield,
                ModuleKind::Generator => body.generator_yield,
                ModuleKind::Refinery => body.refinery_yield,
                // A Trade Post (ticket #35) follows the Habitat yield: trade goes where people live.
                ModuleKind::TradePost => body.habitat_yield,
                _ => 1.0,
            };
            let mut v = p.amount as f64 * yield_ * fac.output_multiplier * self.tech_output_multiplier_module(kind);
            for d in &self.discoveries {
                if d.body == col.body && d.kind == kind {
                    v *= d.multiplier;
                }
            }
            y.resource = Some(p.resource);
            y.amount = v.floor() as i64;
        }
        if self.has_tech(TechId::ClosedLoopColonies) {
            y.upkeep = (y.upkeep as f64 * t.tech(TechId::ClosedLoopColonies).value).floor() as i64;
        }
        y
    }

    fn producers_of(&self, seat: Seat) -> Vec<Producer> {
        let mut out = Vec::new();
        for sid in self.directed_states(seat) {
            let st = self.state(sid);
            for (i, f) in st.facilities.iter().enumerate() {
                let y = self.facility_yield(seat, sid, f.kind);
                out.push(Producer {
                    place: ProducerPlace::Facility(sid, i),
                    name: f.kind.name(),
                    is_module: false,
                    upkeep: y.upkeep,
                    output: y.resource.map(|r| (r, y.amount)),
                    extraction: matches!(f.kind, FacilityKind::Factory | FacilityKind::Refinery),
                    research: y.research,
                    online: !f.offline_until_resolution,
                });
            }
        }
        for cid in self.directed_colonies(seat) {
            let col = self.colony(cid).unwrap();
            for (i, m) in col.modules.iter().enumerate() {
                let y = self.module_yield(seat, cid, m.kind);
                out.push(Producer {
                    place: ProducerPlace::Module(cid, i),
                    name: m.kind.name(),
                    is_module: true,
                    upkeep: y.upkeep,
                    output: y.resource.map(|r| (r, y.amount)),
                    extraction: matches!(m.kind, ModuleKind::Mine | ModuleKind::Refinery),
                    research: 0,
                    online: !col.grid_failed && !m.offline_until_resolution,
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
        if self.has_tech(TechId::EfficientGrids) { e.solar_maximum_multiplier_with_tech } else { e.solar_maximum_multiplier }
    }

    fn tech_output_multiplier_facility(&self, kind: FacilityKind) -> f64 {
        let t = &self.tables;
        let mut m = 1.0;
        if kind == FacilityKind::PowerPlant {
            m *= self.solar_maximum_multiplier();
        }
        match kind {
            FacilityKind::PowerPlant if self.has_tech(TechId::EfficientGrids) => m *= t.tech(TechId::EfficientGrids).value,
            FacilityKind::Factory if self.has_tech(TechId::DeepMining) => m *= t.tech(TechId::DeepMining).value,
            FacilityKind::Refinery if self.has_tech(TechId::AutomatedRefining) => m *= t.tech(TechId::AutomatedRefining).value,
            _ => {}
        }
        m
    }

    fn tech_output_multiplier_module(&self, kind: ModuleKind) -> f64 {
        let t = &self.tables;
        let mut m = 1.0;
        if kind == ModuleKind::Generator {
            m *= self.solar_maximum_multiplier();
        }
        match kind {
            ModuleKind::Generator if self.has_tech(TechId::EfficientGrids) => m *= t.tech(TechId::EfficientGrids).value,
            ModuleKind::Mine if self.has_tech(TechId::DeepMining) => m *= t.tech(TechId::DeepMining).value,
            ModuleKind::Refinery if self.has_tech(TechId::AutomatedRefining) => m *= t.tech(TechId::AutomatedRefining).value,
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
                if p.extraction && matches!(res, Resource::Materials | Resource::Fuel) {
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
            if s.kind == FactionKind::Prospectors {
                s.extraction_total += extraction;
            }
        }
        if !shut.is_empty() {
            let line = format!("{}: Energy ran short; shut down {}.", self.seat_name(seat), shut.join(", "));
            self.report.lines.push(line.clone());
            self.log(line);
        }
        if balance < 0 {
            let line = format!("{}: Energy fell to zero even with every producer off.", self.seat_name(seat));
            self.report.lines.push(line.clone());
            self.log(line);
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

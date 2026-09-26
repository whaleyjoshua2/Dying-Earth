//! The Income phase (spec 6 phase 1, 7.1, 7.2, 12.1).

use crate::data::VictoryFirstKind;
use crate::ids::*;
use crate::state::*;

/// Ticket #351 (version 0.09.1): what the next Income's Shortfall would do, for the alarm on the
/// top bar and the driver's summary.
#[derive(Debug, Clone, PartialEq)]
pub struct ShortfallForecast {
    /// How far short of the bill the seat stands with everything running.
    pub short_by: i64,
    /// What goes dark, in the order the rule shuts it.
    pub dark: Vec<GoesDark>,
}

/// One building the Shortfall would shut: its name and where it stands (*"in China"*, *"at
/// Tiangong over Earth"*). What a Scrubber costs the Natural Sink is left to the Report line after
/// the fact, at the designer's word: the alarm's job is what goes dark.
#[derive(Debug, Clone, PartialEq)]
pub struct GoesDark {
    pub name: String,
    pub at: String,
}

/// One producer the shortfall rule can shut down, in the order it shuts them.
#[derive(Debug, Clone)]
struct Producer {
    place: ProducerPlace,
    name: &'static str,
    is_module: bool,
    upkeep: i64,
    output: Option<(Resource, i64)>,
    /// Materials or Fuel from a Mine, Refinery or Factory count toward the Extraction Total.
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
    /// Ticket #280 (version 0.08.5): what the building DOES when that is not a resource, or beside
    /// one -- the `does` sentence on its row in `facilities.toml` or `modules.toml`, written where
    /// "no output" was written before, at the designer's word. Static prose from the data, so a
    /// figure in it is a data figure and cannot drift as a hand-written hover did.
    pub does: Option<String>,
    /// Ticket #352 (version 0.09.1): how the figure it makes was reached, step by step.
    pub chain: Chain,
    /// Ticket #358 (version 0.09.1): Influence Allotment paid OUTSIDE the Faction multiplier -- the
    /// Chorus's per-Colonist point, on the Spaceport's argument (#183). `allotment` is inside it.
    pub allotment_outside: i64,
}

/// Ticket #352 (version 0.09.1): one step of how a building's figure was reached, in the order the
/// rule takes it. The designer: *"mouseover explains math for research output"*, and every
/// multiplied figure with it.
#[derive(Debug, Clone, PartialEq)]
pub enum Step {
    Base(f64, String),
    Times(f64, String),
    Over(f64, String),
    Plus(i64, String),
    Floor,
    Round,
    Half(String),
}

/// Ticket #352: the arithmetic of a yield, kept as it is DONE. `facility_yield` and `module_yield`
/// compute their figures through this, so the hover that prints it is the rule itself and cannot
/// drift from it. A factor of exactly 1 multiplies and records nothing.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Chain {
    pub steps: Vec<Step>,
    v: f64,
}

impl Chain {
    pub fn base(v: f64, why: impl Into<String>) -> Chain {
        Chain { steps: vec![Step::Base(v, why.into())], v }
    }
    pub fn times(&mut self, f: f64, why: impl FnOnce() -> String) {
        self.v *= f;
        if (f - 1.0).abs() > 1e-9 {
            self.steps.push(Step::Times(f, why()));
        }
    }
    pub fn over(&mut self, d: f64, why: impl Into<String>) {
        self.v /= d;
        self.steps.push(Step::Over(d, why.into()));
    }
    pub fn plus(&mut self, n: i64, why: impl Into<String>) -> i64 {
        self.v += n as f64;
        self.steps.push(Step::Plus(n, why.into()));
        self.v as i64
    }
    pub fn floor(&mut self) -> i64 {
        self.steps.push(Step::Floor);
        self.v = self.v.floor();
        self.v as i64
    }
    pub fn round(&mut self) -> i64 {
        self.steps.push(Step::Round);
        self.v = self.v.round();
        self.v as i64
    }
    /// Integer half, rounded down, as the rule halves a whole figure.
    pub fn half(&mut self, why: impl Into<String>) -> i64 {
        self.v = ((self.v as i64) / 2) as f64;
        self.steps.push(Step::Half(why.into()));
        self.v as i64
    }
    /// Whether anything multiplied the base, which is when the chain is worth showing.
    pub fn multiplied(&self) -> bool {
        self.steps.iter().any(|s| matches!(s, Step::Times(..) | Step::Over(..) | Step::Plus(..) | Step::Half(_)))
    }

    /// The chain as a hover's lines, at most `max`: the base, every step, and "= 3.63, rounded down
    /// to 3" where the rule rounds. Past `max` the later steps share a line, as ticket #352 asked.
    pub fn lines(&self, max: usize) -> Vec<String> {
        // Two places, as the approved example reads ("x 1.10"), and a whole number bare ("2 base").
        let fig = |f: f64| if (f - f.round()).abs() < 1e-9 { format!("{}", f.round() as i64) } else { format!("{f:.2}") };
        let mut out: Vec<String> = Vec::new();
        let mut steps: Vec<String> = Vec::new();
        let mut v = 0.0;
        let flush = |out: &mut Vec<String>, steps: &mut Vec<String>| {
            out.append(steps);
        };
        for s in &self.steps {
            match s {
                Step::Base(b, why) => {
                    v = *b;
                    out.push(if why.is_empty() { format!("{} base", fig(*b)) } else { format!("{} {why}", fig(*b)) });
                }
                Step::Times(f, why) => {
                    v *= f;
                    steps.push(format!("× {} {why}", fig(*f)));
                }
                Step::Over(d, why) => {
                    v /= d;
                    steps.push(if why.is_empty() { format!("÷ {}", fig(*d)) } else { format!("÷ {} {why}", fig(*d)) });
                }
                Step::Plus(n, why) => {
                    flush(&mut out, &mut steps);
                    v += *n as f64;
                    out.push(format!("+ {n} {why}"));
                }
                Step::Floor | Step::Round => {
                    flush(&mut out, &mut steps);
                    let n = if matches!(s, Step::Floor) { v.floor() } else { v.round() };
                    let how = if matches!(s, Step::Floor) { "rounded down" } else { "rounded" };
                    out.push(if (n - v).abs() < 1e-9 { format!("= {}", fig(n)) } else { format!("= {}, {how} to {}", fig(v), fig(n)) });
                    v = n;
                }
                Step::Half(why) => {
                    flush(&mut out, &mut steps);
                    v = ((v as i64) / 2) as f64;
                    out.push(format!("halved {why}: {}", fig(v)));
                }
            }
        }
        flush(&mut out, &mut steps);
        // Too long: fold the "×" lines together from the end, two at a time, until it fits.
        while out.len() > max {
            let Some(i) = (1..out.len()).rev().find(|&i| i > 1 && out[i].starts_with('×') && out[i - 1].starts_with('×')) else { break };
            let joined = format!("{}, {}", out[i - 1], out[i]);
            out[i - 1] = joined;
            out.remove(i);
        }
        out
    }
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
            // Ticket #332 (version 0.09.0): a Factory's Widgets, the work half of every build.
            Some(Resource::Widgets) => parts.push(format!("+{} Widgets", self.amount)),
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
        // Ticket #280 (version 0.08.5): what it does, in the data's words.
        if let Some(d) = &self.does {
            parts.push(d.clone());
        }
        // Ticket #358 (version 0.09.1): both halves, inside and outside the multiplier, as paid.
        if self.allotment + self.allotment_outside > 0 {
            parts.push(format!("+{} Influence Allotment", self.allotment + self.allotment_outside));
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
        self.arm_neutrals();
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
        self.starve_blockaded_colonies();
        for seat in Seat::ALL {
            self.income_for(seat);
        }
        // Ticket #185 (version 0.08.0): the Schools do their work for the turn.
        self.run_schools();
        self.neutral_research();
        self.solar_maximum_next = false;
        // Ticket #76: a Drought lasts one Income. Ticket #257: so does a Storm Surge on a wall.
        for s in &mut self.states {
            s.drought = false;
            s.storm_surge = false;
        }
        // Ticket #337 (version 0.09.0): and so does a choice card's cut to a Facility kind.
        for s in &mut self.seats {
            s.card_facility = None;
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
            // Ticket #345 (version 0.09.1): and so has the first-to-a-Body windfall, on the same
            // line and for the same reason. It is paid once; the accumulator begins again at nought
            // and stays there until the seat is first to another Body.
            s.first_windfall = 0;
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
            // Ticket #282 (version 0.08.5): a Levy is standing too, but it is not THE Standing Army,
            // so its presence never suppresses the raising of one.
            let has = self.armies.iter().any(|a| a.standing && !a.levy && a.home == ArmyHome::State(sid));
            if !has {
                // A destroyed Standing Army is raised again at strength 1. Ticket #282: two Incomes
                // after it died, not the next -- the assumption this comment carried from ticket
                // #50 was settled at the designer's word -- so a won Battle opens a window.
                if self.state(sid).respawn_wait > 0 {
                    self.state_mut(sid).respawn_wait -= 1;
                    continue;
                }
                // Ticket #296 (version 0.08.6): at strength 1 means damage one under its own hit
                // points, which are its live strength now rather than the card's 5.
                let cap = self.standing_army_cap(sid);
                self.spawn_standing_army(sid);
                let dmg = cap.saturating_sub(1);
                if let Some(a) = self.armies.iter_mut().find(|a| a.standing && !a.levy && a.home == ArmyHome::State(sid)) {
                    a.damage = dmg;
                }
                continue;
            }
            // Ticket #296 (version 0.08.6): a Region's own Army whose damage has reached its
            // strength is destroyed, not left at nought. Its strength moves live -- a Constabulary
            // gone offline, Unrest past the threshold -- so the check is made every Income, and the
            // destroyed Standing Army takes the two-Income road back like one killed in a Battle.
            let spent: Vec<ArmyId> = self.armies.iter().filter(|a| a.standing && a.home == ArmyHome::State(sid) && a.damage >= self.army_hit_points(a)).map(|a| a.id).collect();
            for id in spent {
                self.destroy_army(id, "its strength spent", Some(ReportPlace::State(sid)));
            }
            // Ticket #52: at Unrest 4 the Standing Army stops replenishing.
            if occupied || !self.army_replenishes(sid) {
                continue;
            }
            // Ticket #282: a Levy heals on the same terms.
            for a in self.armies.iter_mut().filter(|a| a.levy && a.home == ArmyHome::State(sid)) {
                a.damage = a.damage.saturating_sub(1);
            }
            if let Some(a) = self.armies.iter_mut().find(|a| a.standing && !a.levy && a.home == ArmyHome::State(sid)) {
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
        let mut y = Yield { resource: None, amount: 0, research: 0, upkeep: fc.energy_upkeep, emissions: 0.0, allotment: fc.influence_allotment, standing: fc.standing_per_turn, doubled_by: None, detail: None, does: fc.does.clone(), chain: Chain::default(), allotment_outside: 0 };
        if let Some(p) = &fc.produces {
            match p.resource {
                Resource::Research => {
                    // Ticket #69: an Occupied state's Lab works for the world (`neutral_research`),
                    // not for the occupier, who pays its upkeep and draws nothing.
                    if self.state(sid).control.is_occupied() {
                        y.research = 0;
                    } else {
                        // Ticket #185 (version 0.08.0): the LIVE Education Level, which a School moves.
                        // Ticket #352 (version 0.09.1): through the chain, in the rule's order.
                        let edu = self.education_level(sid);
                        let mut r = Chain::base(p.amount as f64, "base");
                        r.times(self.population_factor(sid), || format!("for {} people, weighted by Education", t.people_text(self.state(sid).population)));
                        r.times(edu, || format!("for Education {edu:.2}"));
                        r.times(fac.research_multiplier, || format!("as the {}", fac.name));
                        r.times(self.tech_multiplier(seat, TechId::PublicScience), || t.tech(TechId::PublicScience).name.clone());
                        // Ticket #84: the Upload stacks on Public Science.
                        r.times(self.tech_multiplier(seat, TechId::TheUpload), || t.tech(TechId::TheUpload).name.clone());
                        y.research = r.floor();
                        y.chain = r;
                    }
                }
                Resource::Ducats => {
                    // A Bank (ticket #35): its amount times the state's gdp / 10.
                    let mut v = Chain::base(p.amount as f64, "base");
                    v.times(card.gdp as f64, || "for GDP".to_string());
                    v.over(10.0, "");
                    v.times(fac.output_multiplier, || format!("as the {}", fac.name));
                    y.resource = Some(Resource::Ducats);
                    y.amount = v.floor();
                    y.chain = v;
                }
                // Ticket #332 (version 0.09.0): a Factory's Widgets come through here too, so the
                // Faction's output multiplier, the Strip Permit and Unrest 7 below reach them. No
                // Region leans Widgets and no Tech lifts them, so the lean and Deep Mining stay
                // Materials rules and reach the Mine alone.
                res => {
                    let mut v = Chain::base(p.amount as f64, "base");
                    if card.resource_lean == res {
                        v.times(1.5, || format!("as this Region leans {}", res.name()));
                    }
                    v.times(fac.output_multiplier, || format!("as the {}", fac.name));
                    v.times(self.tech_output_multiplier_facility(seat, kind), || "for Techs".to_string());
                    y.resource = Some(res);
                    y.amount = v.floor();
                    y.chain = v;
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
            y.chain.times(m, || "for the Strip Permit".to_string());
            y.chain.floor();
        }
        // Ticket #52: at Unrest 7 every Facility in the state produces at half, rounded down, and
        // emits at half. What it adds to the Allotment and to the standing is untouched.
        if self.facilities_at_half(sid) {
            y.amount /= 2;
            y.research /= 2;
            y.emissions *= 0.5;
            y.chain.half("at Unrest 7");
        }
        y
    }

    /// What one Module in a Colony makes each turn for its director. Modules never emit.
    pub fn module_yield(&self, seat: Seat, cid: ColonyId, kind: ModuleKind) -> Yield {
        let t = &self.tables;
        let fac = t.faction(self.kind(seat));
        let mc = t.module(kind);
        let mut y = Yield { resource: None, amount: 0, research: 0, upkeep: mc.energy_upkeep, emissions: 0.0, allotment: mc.influence_allotment, standing: mc.standing_per_turn, doubled_by: None, detail: None, does: mc.does.clone(), chain: Chain::default(), allotment_outside: 0 };
        // Ticket #239 (version 0.08.3): a Unique Module does its sibling's job, so every lookup
        // keyed by kind -- the Techs that multiply it, the slot's yield, a Discovery on it --
        // reads the COMMON kind. Without this the Arkwrights' Chorus would be the one Relay in
        // the game that Relay Networks does not reach, and an Exchange would miss the Trade
        // Post's network rule and pay a flat 2.
        let job = kind.common().unwrap_or(kind);
        // Ticket #232 (version 0.08.3): Relay Networks takes a Relay's Allotment from 1 to 2. Its
        // Standing is untouched: Standing holds one particular place, where the Allotment is the
        // Faction's whole diplomatic budget, and "+1 influence" reads as the budget everywhere
        // else in the game.
        if job == ModuleKind::Relay {
            y.allotment += self.tech_addition(seat, TechId::RelayNetworks);
        }
        let Some(col) = self.colony(cid) else { return y };
        // Ticket #239 (version 0.08.3): the Arkwrights' Chorus, one more Influence in the
        // Allotment for every `chorus_colonists` at its OWN Colony, rounded down -- the
        // Observatory's rule for reading a Colony's people, not the Trade Post's rule for reading
        // a Body's. The Allotment rather than Standing, which is what "+1 Influence" has meant
        // since ticket #232: the Faction's diplomatic budget everywhere, not a hold on one place.
        if kind == ModuleKind::Chorus && t.unique.chorus_colonists > 0 {
            let extra = col.colonists as i64 / t.unique.chorus_colonists;
            if extra > 0 {
                // Ticket #358 (version 0.09.1): outside the Faction multiplier, at the designer's word.
                y.allotment_outside += extra;
                y.detail = Some(format!("{} Colonists here, {extra} more Influence", col.colonists));
            }
        }
        // Ticket #57: the yield is the Colony Slot's own, not its Body's. The Body's figures are
        // what the slot drew from when the game started; a station in orbit keeps the Body's.
        if let Some(p) = &mc.produces {
            if job == ModuleKind::TradePost {
                // Ticket #90 (version 0.06.0): trade is a network. `amount` Ducats per Colonist of
                // the Faction at this Body, plus `per_other_body` for every other Body the Faction
                // holds; no Body yield; the Faction's output multiplier applies.
                let here = self.colonists_at_body(seat, col.body) as i64;
                let others = self.bodies_held(seat).into_iter().filter(|b| *b != col.body).count() as i64;
                let per_other = t.trade_post.per_other_body;
                let raw = p.amount * here + per_other * others;
                let mut v = Chain::base(raw as f64, format!("from {} x {here} Colonists + {per_other} x {others} Bodies", p.amount));
                v.times(fac.output_multiplier, || format!("as the {}", fac.name));
                y.resource = Some(Resource::Ducats);
                y.amount = v.floor();
                y.detail = Some(format!("{} x {here} Colonists + {per_other} x {others} Bodies", p.amount));
                // Ticket #239 (version 0.08.3): the Prospectors' Exchange pays one more, flat and
                // AFTER the multiplier, for the Academy's reason -- 1 through the largest output
                // multiplier in the game floors back to 1, so a captured Exchange pays its captor
                // exactly what it paid its builder.
                if kind == ModuleKind::Exchange {
                    y.amount = v.plus(t.unique.exchange_ducats, "for the Exchange");
                    y.detail = Some(format!("{} x {here} Colonists + {per_other} x {others} Bodies, and {} for the Exchange", p.amount, t.unique.exchange_ducats));
                }
                y.chain = v;
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
                let mut r = Chain::base(p.amount as f64, "base");
                r.times(science, || "for this site's science".to_string());
                r.times(col.education, || format!("for Education {:.2}", col.education));
                r.times(1.0 + col.colonists as f64 * per * col.education, || format!("for {} Colonists, weighted by Education", col.colonists));
                r.times(research_multiplier, || format!("as the {}", fac.name));
                r.times(self.tech_multiplier(seat, TechId::PublicScience), || t.tech(TechId::PublicScience).name.clone());
                // Ticket #84: the Upload stacks on Public Science.
                r.times(self.tech_multiplier(seat, TechId::TheUpload), || t.tech(TechId::TheUpload).name.clone());
                y.research = r.floor();
                y.chain = r;
            } else {
                // Ticket #89: a sun-scaled Module (the Solar Array) reads the sunlight where its
                // Body stands instead of a Body yield, is silenced by a Solar Storm turn, and
                // rounds to the nearest whole.
                // Ticket #332 (version 0.09.0): a Widget-making Module (the Factory, the Core) is
                // FLAT -- no Body yield, no slot yield -- at the designer's word; the Faction's
                // output multiplier still applies, as it does to the Factory on Earth.
                let yield_ = if mc.sun_scaled {
                    self.sun_factor(col.body)
                } else if p.resource == Resource::Widgets {
                    1.0
                } else {
                    self.colony_yields(col).of_module(job)
                };
                let mut v = Chain::base(p.amount as f64, "base");
                v.times(yield_, || if mc.sun_scaled { "for the sunlight here".to_string() } else { "for this site's yield".to_string() });
                v.times(fac.output_multiplier, || format!("as the {}", fac.name));
                v.times(self.tech_output_multiplier_module(seat, job), || "for Techs".to_string());
                for d in &self.discoveries {
                    if d.body == col.body && d.kind == job {
                        v.times(d.multiplier, || "for a Discovery".to_string());
                    }
                }
                if mc.sun_scaled && self.event_is(EventId::SolarStorm) {
                    v.times(0.0, || "in a Solar Storm".to_string());
                }
                y.resource = Some(p.resource);
                y.amount = if mc.sun_scaled { v.round() } else { v.floor() };
                // Ticket #239 (version 0.08.3): the Archivists' Heliostat makes one more, added
                // AFTER the inverse square scaling and after the rounding, so the point is worth
                // the same at every distance rather than 0.43 at Mars and 1.91 at Venus. A Solar
                // Storm silences a Heliostat as it silences a Solar Array: nothing is added to nought.
                if kind == ModuleKind::Heliostat && y.amount > 0 {
                    y.amount = v.plus(t.unique.heliostat_energy, "for the Heliostat");
                }
                // Ticket #92: a working Mass Driver at the Colony gives each Mine there more, after
                // everything.
                if job == ModuleKind::Mine && col.modules.iter().any(|m| m.kind == ModuleKind::MassDriver && m.working()) {
                    y.amount = v.plus(t.mass_driver.mine_bonus, "for the Mass Driver");
                }
                y.chain = v;
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
            return Yield { resource: None, amount: 0, research: 0, upkeep: 0, emissions: 0.0, allotment: 0, standing: 0, doubled_by: None, detail: None, does: None, chain: Chain::default(), allotment_outside: 0 };
        };
        let mut y = self.module_yield(seat, cid, kind);
        if self.doubled_modules(seat).contains(&(cid, index)) {
            let pairs = &self.tables.faction(self.kind(seat)).mothball_pairs;
            y.amount *= 2;
            y.research *= 2;
            y.doubled_by = pairs.iter().find(|(_, m)| **m == kind).map(|(f, _)| f.name());
            let by = y.doubled_by.unwrap_or("Facility");
            y.chain.times(2.0, || format!("for an idle {by} on Earth"));
        }
        // Ticket #359 (version 0.09.1): an occupied Habitat standing shut halves everything but
        // the Energy. Ticket #352 (version 0.09.1): applied HERE, on the one figure every reader
        // takes, so the Module's hover shows it; it was applied at Income and on the Widget sum.
        if self.habitat_halves(cid) && y.resource != Some(Resource::Energy) {
            y.amount /= 2;
            y.research /= 2;
            // Ticket #358 (version 0.09.1): and the Influence, now that the Allotment reads this.
            y.allotment /= 2;
            y.allotment_outside /= 2;
            y.chain.half("while a shut Habitat houses people here");
        }
        y
    }

    /// Ticket #282 (version 0.08.5): neutral states arm when threatened. A neutral Region with a
    /// foreign Army next door, or a neighbour under Occupation, raises a Levy at Industry + 2 --
    /// while its Unrest is under the Standing Army's threshold, since a restive state musters
    /// nothing -- and stands it down at the Income after the threat has passed, or the moment the
    /// Region is no longer neutral. The Report says both.
    /// Ticket #302 (version 0.08.6): a threatened neutral ARMS FOR GOOD -- its Standing Army gains
    /// the table's `threat_steps` at the Income a threat begins, once per threat episode, where
    /// ticket #282 raised a Levy that stood down again. The threat is the same one: a built Army of
    /// any Faction next door, or a neighbour Occupied, and a restive Region (Unrest at the
    /// Standing Army's threshold) does not arm, as it did not raise a Levy.
    fn arm_neutrals(&mut self) {
        for sid in StateId::ALL {
            let threatened = self.neutral_threatened(sid);
            let was = self.state(sid).threatened;
            if threatened && !was && self.army_replenishes(sid) {
                self.state_mut(sid).armed += self.tables.standing_army.threat_steps;
                self.levies_raised += 1;
                let (state, n) = (self.tables.state(sid).name.clone(), self.standing_army_cap(sid));
                self.log(format!("{state} arms while a foreign Army stands next door: its Standing Army will stand at {n} from now on."));
                let text = self.say("neutral_armed", &[("state", state), ("n", n.to_string())]);
                self.report_line(LineKind::Army, Some(ReportPlace::State(sid)), text);
            }
            self.state_mut(sid).threatened = threatened;
        }
    }

    /// Ticket #278 (version 0.08.5): every Colony starved this Income is named in its holder's
    /// Report, counted for the sweep, and charged as an offence at weight 1 against the blockader --
    /// the rung the 0.08.2 spec promised and nothing fired until now. Read live, once an Income, as
    /// every blockade test is; `income_for` makes the Colony's Modules yield nothing.
    fn starve_blockaded_colonies(&mut self) {
        let starved: Vec<(ColonyId, Seat, Seat)> =
            self.colonies.iter().filter_map(|c| Some((c.id, c.control.director()?, self.starved_by(c.id)?))).collect();
        for (cid, holder, by) in starved {
            self.seat_mut(holder).blockade_turns_suffered += 1;
            self.seat_mut(by).blockade_turns_imposed += 1;
            self.offend_by(by, holder, 1);
            let (place, who) = (self.place_name(Place::Colony(cid)), self.seat_name(by));
            self.log(format!("{place} is blockaded by the {who}: it makes nothing this turn."));
            let text = self.say("starved", &[("place", place), ("faction", who)]);
            self.report_line(LineKind::Note, Some(ReportPlace::Colony(cid)), text);
        }
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
                // Ticket #257 (version 0.08.4): a Storm Surge that broke on the state's Sea Wall cuts
                // what its COASTAL Facilities make at this Income; an inland one is untouched.
                let dry = if st.drought { self.tables.events.drought_output_multiplier } else { 1.0 };
                let surge = if st.storm_surge && f.coastal { self.tables.events.storm_surge_coastal_multiplier } else { 1.0 };
                // Ticket #337 (version 0.09.0): a choice card answered last turn may cut one kind of
                // this seat's Facilities at this Income -- the Drought's shape, per seat and per
                // kind, which is what the Strike and the Emergency Shutdown both want.
                let card = self.card_facility_multiplier(seat, f.kind);
                let scale = dry * surge * card;
                let halve = |v: i64| if scale < 1.0 { (v as f64 * scale).floor() as i64 } else { v };
                out.push(Producer {
                    place: ProducerPlace::Facility(sid, i),
                    name: f.kind.name(),
                    is_module: false,
                    upkeep: y.upkeep,
                    output: y.resource.map(|r| (r, halve(y.amount))),
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
            // Ticket #278 (version 0.08.5): a starved Colony -- a station under a rival's Blockade,
            // a ground Colony under a rival's outright Orbital Control with a blockading stack --
            // makes NOTHING and still pays its upkeep, at the designer's word: the squeeze is the
            // point, and the offline switch below, which forgives the upkeep, is the wrong shape.
            let starved = self.starved_by(cid).is_some();
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
                    output: if starved { None } else { y.resource.map(|r| (r, y.amount)) },
                    // Ticket #80: an Observatory's Research. Ticket #359: halved in `module_yield_at`.
                    research: if starved { 0 } else { y.research },
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
            // Ticket #332 (version 0.09.0): Deep Mining follows the Materials to the Mine; the
            // Factory makes Widgets now and no Tech lifts those.
            FacilityKind::Mine => m *= self.tech_multiplier(seat, TechId::DeepMining),
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

    /// Ticket #332 (version 0.09.0): the Widgets a place makes this turn, the work half of every
    /// build there. A Region makes `[widgets] region_base` plus its Industry Level times
    /// `per_industry_level` with no Factory at all (a flat four and one a level, the designer's
    /// figures) -- its own, whoever directs it, so a build in a Region that threw its holder
    /// off still crawls on -- plus every working Factory's Widget yield through `facility_yield`,
    /// which wants a director. A Colony or station makes every working Module's Widget yield
    /// through `module_yield_at` (the Core Module's one, a Factory Module's four, Production Moved's
    /// doubling applied); a starved or grid-failed Colony makes none, as it makes nothing at Income.
    /// Never banked: what `resolve_builds` does not apply this turn is lost.
    pub fn widgets_at(&self, place: Place) -> i64 {
        match place {
            Place::State(sid) => {
                let st = self.state(sid);
                let mut n = self.tables.widgets.region_base as i64 + st.industry_level as i64 * self.tables.widgets.per_industry_level as i64;
                if let Some(seat) = st.control.director() {
                    for f in st.facilities.iter().filter(|f| f.working()) {
                        let y = self.facility_yield(seat, sid, f.kind);
                        if y.resource == Some(Resource::Widgets) {
                            n += y.amount;
                        }
                    }
                }
                n
            }
            Place::Colony(cid) => {
                let Some(col) = self.colony(cid) else { return 0 };
                let Some(seat) = col.control.director() else { return 0 };
                if col.grid_failed || self.starved_by(cid).is_some() {
                    return 0;
                }
                let mut n = 0;
                for (i, m) in col.modules.iter().enumerate() {
                    if !m.working() {
                        continue;
                    }
                    let y = self.module_yield_at(seat, cid, i);
                    if y.resource == Some(Resource::Widgets) {
                        n += y.amount;
                    }
                }
                // Ticket #359 (version 0.09.1): at half under an occupied Habitat standing shut,
                // per Module in `module_yield_at`.
                n
            }
        }
    }

    /// Ticket #332: the Widgets a build of `item` needs for this seat: the row's figure times the
    /// Faction's Materials discount for the kind (a Facility's and an Industry raise's, a Module's,
    /// a Ship's; an Army has none), rounded down, never below 1 -- a figure of nought would complete
    /// at a place that makes nothing.
    pub fn build_widgets(&self, seat: Seat, item: BuildItem) -> u32 {
        let t = &self.tables;
        let fac = t.faction(self.kind(seat));
        let (row, m) = match item {
            BuildItem::Facility(k) => (t.facility(k).widgets, fac.facility_materials_multiplier),
            BuildItem::IndustryLevel => (t.industry_level.widgets, fac.facility_materials_multiplier),
            BuildItem::Module(k) => (t.module(k).widgets, fac.module_materials_multiplier),
            BuildItem::Unit(UnitKind::Army) => (t.unit(UnitKind::Army).widgets, 1.0),
            BuildItem::Unit(k) => (t.unit(k).widgets, fac.ship_materials_multiplier),
            // Ticket #343 (version 0.09.1): a Warhead is priced flat, the Faction's Ship discount
            // left out. The discount is a shipwright's, and reloading is not shipbuilding.
            BuildItem::Warhead(_) => (t.nuke.rearm_widgets, 1.0),
        };
        ((row as f64 * m).floor() as u32).max(1)
    }

    /// Ticket #332: the builds under way at a place, in the order they were given.
    pub fn queue_at(&self, place: Place) -> &[Build] {
        match place {
            Place::State(s) => &self.state(s).queue,
            Place::Colony(c) => self.colony(c).map(|c| c.queue.as_slice()).unwrap_or(&[]),
        }
    }

    /// Ticket #332: the Widgets a place's queue still owes, every build's figure less what is done.
    pub fn widgets_owed(&self, place: Place) -> i64 {
        self.queue_at(place).iter().map(|b| b.widgets.saturating_sub(b.done) as i64).sum()
    }

    /// Ticket #332: the Resolutions until `owed` Widgets are made at `rate` a turn: at least 1, and
    /// `u32::MAX` for a place that makes nothing.
    fn resolutions_for(owed: i64, rate: i64) -> u32 {
        if rate <= 0 {
            return u32::MAX;
        }
        (((owed.max(0) + rate - 1) / rate) as u32).max(1)
    }

    /// Ticket #332: for each build in a place's queue, in order, the Resolutions until it would
    /// complete at the place's Widgets a turn behind everything ahead of it. An estimate for the
    /// cards and the Under way block: the rate is today's and may move.
    pub fn queue_estimates(&self, place: Place) -> Vec<u32> {
        let rate = self.widgets_at(place);
        let mut owed = 0i64;
        self.queue_at(place)
            .iter()
            .map(|b| {
                owed += b.widgets.saturating_sub(b.done) as i64;
                Self::resolutions_for(owed, rate)
            })
            .collect()
    }

    /// Ticket #332: the Resolutions until a fresh order of `item` at `place` would complete, at
    /// this place's Widgets a turn behind everything already in its queue; at least 1, and
    /// `u32::MAX` if the place makes nothing. The build buttons' "8 Widgets, 2 turns here".
    pub fn turns_to_build(&self, seat: Seat, place: Place, item: BuildItem) -> u32 {
        let owed = self.widgets_owed(place) + self.build_widgets(seat, item) as i64;
        Self::resolutions_for(owed, self.widgets_at(place))
    }

    /// Ticket #332: what `item` costs `seat` in Materials at `place` -- the Faction's own price, a
    /// Module's with the Colony's working Mines taken off. The refund a cancelled build pays its
    /// canceller, at the CANCELLER's price rather than the builder's.
    pub fn item_materials(&self, seat: Seat, place: Place, item: BuildItem) -> i64 {
        match (item, place) {
            (BuildItem::Facility(k), _) => self.facility_materials(seat, k),
            (BuildItem::IndustryLevel, _) => self.industry_cost(seat),
            (BuildItem::Module(k), Place::Colony(c)) => self.module_materials_at(seat, c, k),
            (BuildItem::Module(k), Place::State(_)) => self.module_materials(seat, k),
            (BuildItem::Unit(UnitKind::Army), _) => self.tables.unit(UnitKind::Army).materials,
            (BuildItem::Unit(k), _) => self.ship_materials(seat, k),
            // Ticket #343 (version 0.09.1): the rearm's flat price, for the same reason.
            (BuildItem::Warhead(_), _) => self.tables.nuke.rearm_materials,
        }
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
        let (_, shut) = self.apply_shortfall(seat, &mut producers, self.seat(seat).stockpile.energy);
        shut.into_iter().map(|i| producers[i].name.to_string()).collect()
    }

    /// Ticket #351 (version 0.09.1): **the Shortfall forecast** -- what the next Income would shut,
    /// run by the Income rule itself on the Energy left after this turn's `pending` orders, as the
    /// top bar's figure reads it -- today that is a purchase of Energy, which clears or shrinks the
    /// alarm the turn it is ordered, since no order spends Energy and it cannot be sold. None when
    /// nothing would go dark. The designer: *"alarm when there is a
    /// projected energy deficit and buildings will go offline next turn."*
    pub fn shortfall_forecast(&self, seat: Seat, pending: &[crate::orders::Order]) -> Option<ShortfallForecast> {
        let stored = self.remaining(seat, pending).0.energy;
        let mut producers = self.producers_of(seat);
        let (_, shut) = self.apply_shortfall(seat, &mut producers, stored);
        if shut.is_empty() {
            return None;
        }
        // The deficit is the balance with everything still running, read before a single thing is shut.
        let short_by = -self.energy_balance(seat, &self.producers_of(seat), stored);
        let dark = shut
            .into_iter()
            .map(|i| {
                let p = &producers[i];
                let at = match p.place {
                    ProducerPlace::Facility(sid, _) => format!("in {}", self.tables.state(sid).name),
                    ProducerPlace::Module(cid, _) => format!("at {}", self.place_name(Place::Colony(cid))),
                };
                GoesDark { name: p.name.to_string(), at }
            })
            .collect();
        Some(ShortfallForecast { short_by, dark })
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

    /// The Energy left after Income with these producers as they stand: `stored`, plus what the
    /// online ones make, less their upkeep and the Ships' and Armies', plus the Reactor's relief.
    fn energy_balance(&self, seat: Seat, ps: &[Producer], stored: i64) -> i64 {
        let energy_in: i64 = ps.iter().filter(|p| p.online).filter_map(|p| p.output.filter(|(r, _)| *r == Resource::Energy).map(|(_, v)| v)).sum();
        let upkeep: i64 = ps.iter().filter(|p| p.online).map(|p| p.upkeep).sum();
        stored + energy_in - upkeep - self.unit_upkeep(seat) + self.reactor_relief(seat, ps)
    }

    /// Ticket #351 (version 0.09.1): the rule starts from `stored` rather than reading the stockpile,
    /// so the forecast can run it on the Energy left after this turn's orders; Income passes the
    /// stockpile itself. It returns the balance after, and the INDEX of each producer it shut, in
    /// the order it shut them.
    fn apply_shortfall(&self, seat: Seat, producers: &mut [Producer], stored: i64) -> (i64, Vec<usize>) {
        // Ticket #184: the balance is recomputed from scratch whenever a building goes dark, because
        // the Reactor's relief is a share of the bill and shrinks with it.
        let mut balance = self.energy_balance(seat, producers, stored);
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
            shut.push(i);
            balance = self.energy_balance(seat, producers, stored);
        }
        (balance, shut)
    }

    fn income_for(&mut self, seat: Seat) {
        let mut producers = self.producers_of(seat);
        let (balance, shut_at) = self.apply_shortfall(seat, &mut producers, self.seat(seat).stockpile.energy);
        // Ticket #366 (version 0.09.2): each building WITH its place -- "Spaceport in China" -- as
        // the alarm's hover names them (#351); the playtest could not tell which Region went dark.
        let shut: Vec<String> = shut_at
            .iter()
            .map(|i| {
                let p = &producers[*i];
                match p.place {
                    ProducerPlace::Facility(sid, _) => format!("{} in {}", p.name, self.tables.state(sid).name),
                    ProducerPlace::Module(cid, _) => format!("{} at {}", p.name, self.place_name(Place::Colony(cid))),
                }
            })
            .collect();
        // Ticket #351 (version 0.09.1): what the Natural Sink loses with the Scrubbers shut, which the
        // Report line names -- the case where a Custodian loses the game without noticing. Only where
        // the Region has a controller, which is where `scrubber_removal_by_seat` counts it: one in a
        // Region occupied from neutral never added to the Sink, so shutting it takes nothing off.
        let sink_lost = self.tables.facility(FacilityKind::Scrubber).sink_per_turn
            * shut_at
                .iter()
                .filter(|i| {
                    matches!(producers[**i].place, ProducerPlace::Facility(sid, f)
                        if self.state(sid).control.controller().is_some()
                            && self.state(sid).facilities.get(f).map(|x| x.kind == FacilityKind::Scrubber).unwrap_or(false))
                })
                .count() as f64;
        let mut gained = Stockpile::default();
        let mut research = 0;
        let mut off_earth = 0;
        let mut doubled_turns = 0;
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
            // Ticket #332 (version 0.09.0): Widgets are a rate, not a stock. They are made and spent
            // at their place at Resolution (`widgets_at`, `resolve_builds`); Income neither banks
            // them nor lists them among the sources.
            if let Some((res, v)) = p.output.filter(|(r, _)| *r != Resource::Widgets) {
                match res {
                    Resource::Materials => gained.materials += v,
                    Resource::Fuel => gained.fuel += v,
                    Resource::Energy => gained.energy += v,
                    Resource::Ducats => gained.ducats += v,
                    Resource::Research | Resource::Widgets => {}
                }
                sources.push((format!("{} in {}", p.name, where_), res, v));
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
                // Ticket #240 (version 0.08.3): the Fund holds Ducats, so this interest is paid
                // in Ducats. The rate and the floor were fitted against a Materials fund and are
                // left where they are; the bar was set against a measured Fund that already
                // carried them, so moving both at once would have priced neither.
                let per = (self.seat(seat).venture_fund as f64 * u_interest).floor() as i64;
                interest_to_fund = (n * per).max(u_floor);
                sources.push((format!("{n} Investment Bank (interest banked)"), Resource::Ducats, interest_to_fund));
            } else {
                let per = ((gained.ducats as f64 * u_interest).floor() as i64).max(u_floor);
                let paid = n * per;
                gained.ducats += paid;
                sources.push((format!("{n} Investment Bank (interest)"), Resource::Ducats, paid));
            }
        }
        // Ticket #72 (version 0.05.5): the Venture Capital Fund took its share of the Materials the
        // seat's Factories and Mines paid, rounded down, before the Stockpile saw them.
        //
        // Ticket #240 (version 0.08.3): it takes its share of **Ducat income** instead, at Income
        // and before the seat can spend a coin of it. The designer: the hoard is counted in Ducats
        // now, and *"a"* -- a share of income rather than a relabelled share of output.
        //
        // This changes what the Condition ASKS. A share of Materials output skimmed a resource the
        // seat stockpiles anyway; measured over 120 games, every seat ends every game holding about
        // FIVE Ducats, so a Ducat share competes with the seat's whole economy -- Influence bought,
        // Relief paid, repairs. "Bank it or spend it" is the decision, which is what a venture fund
        // actually is, and it is why the bar could not simply be converted at the market rate.
        let share = self.seat(seat).venture_share;
        let banked = if share > 0.0 { (gained.ducats as f64 * share).floor() as i64 } else { 0 };
        if banked > 0 {
            gained.ducats -= banked;
            sources.push(("Venture Capital Fund (banked)".to_string(), Resource::Ducats, -banked));
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
        // Ticket #257 (version 0.08.4): the Sea Walls' keep. Each rise a working wall has held adds
        // `upkeep_per_rise` Materials a turn -- half a Material, which is not a whole number, so the
        // fraction is carried on the seat and whole Materials are paid as they accrue; nothing is
        // lost to rounding. Paid out of this Income's Materials, and short of Materials the walls
        // stand unkept this turn and hold nothing -- the Energy shortfall rule's shape, for
        // Materials, since a wall nobody pays for is a wall nobody mans.
        let rises: u32 = self.directed_states(seat).iter().flat_map(|sid| self.state(*sid).facilities.iter()).filter(|f| f.kind == FacilityKind::SeaWall && f.working()).map(|f| f.rises_held).sum();
        let mut keep_due = 0i64;
        if rises > 0 {
            let per = self.tables.sea_wall.upkeep_per_rise;
            let s = self.seat_mut(seat);
            s.sea_wall_upkeep_owed += rises as f64 * per;
            keep_due = s.sea_wall_upkeep_owed.floor() as i64;
            if keep_due > 0 {
                s.sea_wall_upkeep_owed -= keep_due as f64;
                sources.push(("Sea Walls (keep)".to_string(), Resource::Materials, -keep_due));
            }
        }
        let keep_short = keep_due > 0 && self.seat(seat).stockpile.materials + gained.materials < keep_due;
        if keep_short {
            // Nothing is paid: the Materials stay, the walls go unkept, and the fraction they would
            // have paid is not owed twice.
            sources.retain(|(n, _, _)| n != "Sea Walls (keep)");
            self.seat_mut(seat).sea_wall_upkeep_owed = 0.0;
            keep_due = 0;
        }
        gained.materials -= keep_due;
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
        if keep_short {
            let mut names = Vec::new();
            for sid in self.directed_states(seat) {
                let name = self.tables.state(sid).name.clone();
                let mut any = false;
                for f in self.state_mut(sid).facilities.iter_mut() {
                    if f.kind == FacilityKind::SeaWall && f.working() {
                        f.online = false;
                        any = true;
                    }
                }
                if any {
                    names.push(name);
                }
            }
            let line = format!("{}: Materials ran short; the Sea Wall in {} stands unkept this turn and holds nothing.", self.seat_name(seat), Game::and_list(&names));
            self.log(line.clone());
            let text = self.say("sea_wall_unkept", &[("faction", self.seat_name(seat)), ("states", Game::and_list(&names))]);
            self.report_line_of(seat, LineKind::YourWorks, LineKind::Note, None, text);
        }
        if !shut.is_empty() {
            let line = format!("{}: Energy ran short; shut down {}.", self.seat_name(seat), shut.join(", "));
            self.log(line);
            let sink = if sink_lost > 0.0 { self.phrase("energy_short_sink", &[("ppm", format!("{sink_lost:.1}"))]) } else { String::new() };
            let text = self.say("energy_short", &[("faction", self.seat_name(seat)), ("buildings", shut.join(", ")), ("sink", sink)]);
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

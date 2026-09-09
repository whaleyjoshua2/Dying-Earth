//! The AI faction (spec 16): enumerate every legal action, score it, spend greedily.

use crate::combat::first_round_odds;
use crate::ids::*;
use crate::orders::*;
use crate::state::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cat {
    Producer,
    RaiseIndustry,
    ResearchLab,
    Habitat,
    LaunchSiteOrShipyard,
    ColonyShip,
    Warship,
    ArmyOrBarracks,
    /// Ticket #36: an Embassy or a Relay.
    BuildInfluence,
    Influence,
    Transit,
    LoadUnload,
    FoundColony,
    Restoration,
    StanceAttack,
    StanceIntercept,
    StanceHold,
    StanceEvade,
}

#[derive(Debug, Clone)]
struct Candidate {
    orders: Vec<Order>,
    cat: Cat,
    base: f64,
    gap: f64,
    denial: f64,
    threat: f64,
    opportunity: f64,
    note: String,
    /// The stack a Stance candidate belongs to; one Stance per stack.
    stack: Option<String>,
}

impl Candidate {
    fn score(&self) -> f64 {
        self.base * self.gap * self.denial * self.threat * self.opportunity
    }
}

/// Which part of its Victory Condition a seat is furthest behind on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Behind {
    First,
    Presence,
}

impl Game {
    fn base_weight(&self, seat: Seat, cat: Cat) -> f64 {
        let w = self.tables.ai_weights(self.kind(seat));
        match cat {
            Cat::Producer => w.build_producer,
            Cat::RaiseIndustry => w.raise_industry,
            Cat::ResearchLab => w.build_research_lab,
            Cat::Habitat => w.build_habitat,
            Cat::LaunchSiteOrShipyard => w.build_launch_site_or_shipyard,
            Cat::ColonyShip => w.build_colony_ship,
            Cat::Warship => w.build_warship,
            Cat::ArmyOrBarracks => w.build_army_or_barracks,
            Cat::BuildInfluence => w.build_influence,
            Cat::Influence => w.influence,
            Cat::Transit => w.transit,
            Cat::LoadUnload => w.load_unload,
            Cat::FoundColony => w.found_colony,
            Cat::Restoration => w.restoration,
            Cat::StanceAttack => w.stance_attack,
            Cat::StanceIntercept => w.stance_intercept,
            Cat::StanceHold => w.stance_hold,
            Cat::StanceEvade => w.stance_evade,
        }
    }

    /// Pace (spec 16.3): the value the schedule expects now, by linear interpolation.
    fn expected(schedule: &[[i64; 2]], turn: u32) -> f64 {
        let t = turn as f64;
        let mut prev = (0.0, 0.0);
        for [tt, v] in schedule {
            let (tt, v) = (*tt as f64, *v as f64);
            if t <= tt {
                let span = tt - prev.0;
                if span <= 0.0 {
                    return v;
                }
                return prev.1 + (v - prev.1) * (t - prev.0) / span;
            }
            prev = (tt, v);
        }
        prev.1
    }

    /// Victory gap multiplier and the part it applies to.
    fn victory_gap(&self, seat: Seat) -> (f64, Behind) {
        let pace = &self.tables.ai.pace;
        let m = &self.tables.ai.multipliers;
        let turn = self.turn;
        let ratio_of = |actual: f64, expected: f64| if expected <= 0.0 { 1.0 } else { (actual / expected).min(1.0) };
        let presence_ratio = ratio_of(self.off_world_colonists(seat) as f64, Self::expected(&pace.colonists, turn));
        let first_ratio = match self.kind(seat) {
            FactionKind::Prospectors => {
                ratio_of(self.seat(seat).extraction_total as f64, Self::expected(&pace.prospector_extraction, turn))
            }
            FactionKind::Custodians => {
                let net = self.climate.last.counted() - self.climate.last.total_sink();
                if turn >= pace.custodian_under_sink_by_turn {
                    if net < 0.0 { 1.0 } else { (1.0 - net / 10.0).clamp(0.0, 1.0) }
                } else if turn >= pace.custodian_within_by_turn {
                    if net <= pace.custodian_within_ppm { 1.0 } else { (pace.custodian_within_ppm / net).clamp(0.0, 1.0) }
                } else {
                    1.0
                }
            }
        };
        let (ratio, behind) = if first_ratio <= presence_ratio { (first_ratio, Behind::First) } else { (presence_ratio, Behind::Presence) };
        let mult = if ratio >= 1.0 { 1.0 } else { 1.0 + (m.victory_gap_max - 1.0) * ((1.0 - ratio) / 0.5).min(1.0) };
        (mult, behind)
    }

    fn rival_near_win(&self, seat: Seat) -> bool {
        let p = self.progress(seat.other());
        p.score() >= self.tables.ai.thresholds.near_win_fraction
    }

    /// Within one turn of winning: one part met and the other nearly.
    fn rival_one_turn_from_win(&self, seat: Seat) -> bool {
        let p = self.progress(seat.other());
        (p.first_value >= p.first_bar && p.presence + 4 >= p.presence_bar)
            || (p.presence >= p.presence_bar && p.first_value >= p.first_bar * 0.9)
    }

    fn enemy_present_or_inbound(&self, seat: Seat, body: BodyId) -> bool {
        self.ships.iter().any(|s| s.seat != seat && (s.at == ShipAt::Body(body) || matches!(s.at, ShipAt::Transit { to, .. } if to == body)))
    }

    fn enemy_army_near(&self, seat: Seat, place: Place) -> bool {
        match place {
            Place::State(s) => {
                let mut near = vec![s];
                near.extend(self.tables.state(s).neighbours.iter().copied());
                self.armies.iter().any(|a| {
                    !a.standing && self.army_seat(a) == Some(seat.other()) && matches!(a.at, ArmyAt::Place(Place::State(x)) if near.contains(&x))
                })
            }
            Place::Colony(c) => {
                let body = self.colony(c).map(|c| c.body);
                self.armies.iter().any(|a| self.army_seat(a) == Some(seat.other()) && a.at == ArmyAt::Place(place))
                    || body.map(|b| self.ships.iter().any(|s| s.seat != seat && s.army.is_some() && (s.at == ShipAt::Body(b) || matches!(s.at, ShipAt::Transit { to, .. } if to == b)))).unwrap_or(false)
            }
        }
    }

    /// The Body whose yields best serve the part the AI is furthest behind on (spec 16.4).
    fn best_body_for(&self, seat: Seat, behind: Behind) -> BodyId {
        let t = &self.tables;
        let mut bodies: Vec<BodyId> = [BodyId::Moon, BodyId::Mars].into_iter().filter(|b| !self.free_slots_on(*b).is_empty()).collect();
        if bodies.is_empty() {
            return BodyId::Moon;
        }
        let key = |b: &BodyId| -> f64 {
            let c = t.body(*b);
            match (self.kind(seat), behind) {
                (FactionKind::Prospectors, Behind::First) => c.mine_yield + c.refinery_yield,
                (_, Behind::Presence) => c.habitat_yield,
                (FactionKind::Custodians, Behind::First) => c.generator_yield + c.habitat_yield,
            }
        };
        bodies.sort_by(|a, b| key(b).partial_cmp(&key(a)).unwrap());
        bodies[0]
    }

    /// Every Energy upkeep the seat pays now: Facilities, Modules, Ships and Armies.
    fn total_upkeep(&self, seat: Seat) -> i64 {
        self.unit_upkeep(seat)
            + self.directed_states(seat).iter().flat_map(|s| self.state(*s).facilities.iter()).map(|f| self.tables.facility(f.kind).energy_upkeep).sum::<i64>()
            + self.directed_colonies(seat).iter().flat_map(|c| self.colony(*c).unwrap().modules.iter()).map(|m| self.tables.module(m.kind).energy_upkeep).sum::<i64>()
    }

    /// Energy the seat's producers make in a turn, at current multipliers.
    fn energy_production(&self, seat: Seat) -> i64 {
        let fac = self.tables.faction(self.kind(seat)).output_multiplier;
        let grids = if self.has_tech(TechId::EfficientGrids) { self.tables.tech(TechId::EfficientGrids).value } else { 1.0 };
        let mut e = 0.0;
        for sid in self.directed_states(seat) {
            let lean = if self.tables.state(sid).resource_lean == Resource::Energy { 1.5 } else { 1.0 };
            for f in &self.state(sid).facilities {
                if f.kind == FacilityKind::PowerPlant {
                    e += (6.0 * lean * fac * grids).floor();
                }
            }
        }
        for cid in self.directed_colonies(seat) {
            let col = self.colony(cid).unwrap();
            for m in &col.modules {
                if m.kind == ModuleKind::Generator {
                    e += (5.0 * self.tables.body(col.body).generator_yield * fac * grids).floor();
                }
            }
        }
        e as i64
    }

    /// Spec 16.2: the resource the seat is shortest of, relative to its next three affordable actions.
    /// Energy is a draining balance, so its shortage is measured in turns of drain over those actions.
    fn scarcest(&self, seat: Seat) -> Resource {
        let s = self.seat(seat).stockpile;
        // The three cheapest builds it could afford now stand in for "its next three affordable actions".
        let mut costs: Vec<(i64, i64)> = Vec::new();
        for f in &self.tables.facilities {
            costs.push((f.materials, f.energy_upkeep));
        }
        for m in &self.tables.modules {
            costs.push((m.materials, m.energy_upkeep));
        }
        for u in &self.tables.units {
            costs.push((u.materials, u.energy_upkeep));
        }
        costs.sort();
        let next: Vec<(i64, i64)> = costs.into_iter().filter(|(m, _)| *m <= s.materials.max(20)).take(3).collect();
        let need_materials: i64 = next.iter().map(|(m, _)| *m).sum::<i64>().max(1);
        let need_upkeep: i64 = next.iter().map(|(_, u)| *u).sum::<i64>();
        let drain = (self.total_upkeep(seat) + need_upkeep - self.energy_production(seat)).max(0);
        let energy_ratio = if drain == 0 { f64::INFINITY } else { s.energy as f64 / (3.0 * drain as f64) };
        let materials_ratio = s.materials as f64 / need_materials as f64;
        let has_ships = self.ships.iter().any(|x| x.seat == seat);
        let fuel_ratio = if has_ships { s.fuel as f64 / 6.0 } else { f64::INFINITY };
        if energy_ratio <= materials_ratio && energy_ratio <= fuel_ratio {
            Resource::Energy
        } else if materials_ratio <= fuel_ratio {
            Resource::Materials
        } else {
            Resource::Fuel
        }
    }

    /// Ticket #41: the rival's standing on a place the seat holds is within the challenge margin of
    /// the seat's own, so the place could be lost to a short push.
    fn standing_pressed(&self, seat: Seat, place: Place) -> bool {
        let mine = self.seat(seat).influence.get(&place).copied().unwrap_or(0);
        let rival = self.seat(seat.other()).influence.get(&place).copied().unwrap_or(0);
        rival > 0 && rival + self.tables.influence.challenge_margin >= mine
    }

    /// Spec 16.2: the Energy balance is within one turn's upkeep of zero.
    fn energy_tight(&self, seat: Seat) -> bool {
        let drain = self.total_upkeep(seat) - self.energy_production(seat);
        self.seat(seat).stockpile.energy - drain <= self.total_upkeep(seat)
    }

    pub fn ai_orders(&mut self, seat: Seat) -> Vec<Order> {
        let (gap, behind) = self.victory_gap(seat);
        let kind = self.kind(seat);
        let m = self.tables.ai.multipliers.clone();
        let th = self.tables.ai.thresholds.clone();
        let rival_run = self.seat(seat.other()).stabilization_run >= 1;
        let rival_near = self.rival_near_win(seat);
        let rival_one_turn = self.rival_one_turn_from_win(seat);
        let scarce = self.scarcest(seat);
        let tight = self.energy_tight(seat);
        let allotment = self.seat(seat).allotment;
        let margin = self.tables.influence.challenge_margin;
        let materials_income = self.seat(seat).income_last_turn.materials;
        let no_materials_income = materials_income == 0
            && !self.directed_states(seat).iter().any(|s| self.state(*s).facilities.iter().any(|f| f.kind == FacilityKind::Factory))
            && !self.directed_colonies(seat).iter().any(|c| self.colony(*c).unwrap().modules.iter().any(|m| m.kind == ModuleKind::Mine))
            && !self.states.iter().flat_map(|s| s.queue.iter()).any(|b| b.seat == seat && b.item == BuildItem::Facility(FacilityKind::Factory));
        let queued_power: i64 = self.states.iter().flat_map(|s| s.queue.iter()).filter(|b| b.seat == seat && b.item == BuildItem::Facility(FacilityKind::PowerPlant)).count() as i64 * 6
            + self.colonies.iter().flat_map(|c| c.queue.iter()).filter(|b| b.seat == seat && b.item == BuildItem::Module(ModuleKind::Generator)).count() as i64 * 5;
        let energy_short = self.energy_production(seat) + queued_power < self.total_upkeep(seat) + 2;
        // Bootstrap needs: a producer of one of these counts as advancing whatever the seat is behind on.
        // The merely scarcest resource only gets the within-category preference.
        let mut needs: Vec<Resource> = Vec::new();
        if no_materials_income {
            needs.push(Resource::Materials);
        }
        if energy_short {
            needs.push(Resource::Energy);
        }
        // Ticket #41 tried adding Ducats to this list (a Bank or Trade Post preferred while the seat
        // cannot afford a step of bought Influence). Measured over twenty seeds it cost the Custodians
        // every win: a Bank on turn one displaced the Research Lab, and 4 Ducats a turn buys 2
        // Influence, which never repays 25 Materials the way a Factory does. Banks and Trade Posts
        // stay plain producers at their base weight.
        let mut cands: Vec<Candidate> = Vec::new();

        let mut push = |orders: Vec<Order>, cat: Cat, base: f64, gap: f64, denial: f64, threat: f64, opportunity: f64, note: String, stack: Option<String>| {
            cands.push(Candidate { orders, cat, base, gap, denial, threat, opportunity, note, stack });
        };

        let advances_first = |cat: Cat, item: Option<&str>| -> bool {
            match kind {
                FactionKind::Prospectors => cat == Cat::Producer && item.map(|i| i != "Power Plant" && i != "Generator").unwrap_or(false) || cat == Cat::RaiseIndustry,
                FactionKind::Custodians => cat == Cat::Restoration || cat == Cat::ResearchLab,
            }
        };
        let advances_presence = |cat: Cat| matches!(cat, Cat::Habitat | Cat::ColonyShip | Cat::FoundColony | Cat::LoadUnload | Cat::Transit);
        let gap_for = |cat: Cat, item: Option<&str>| -> f64 {
            // Nothing advances without Energy: while it is the scarcest resource, an Energy producer
            // counts as advancing whichever part the Faction is behind on.
            let energy_producer = cat == Cat::Producer && matches!(item, Some("Power Plant") | Some("Generator"));
            if energy_producer && needs.contains(&Resource::Energy) {
                return gap;
            }
            // Likewise nothing is built without Materials: until the seat has any Materials income,
            // a Materials producer counts as advancing the part it is behind on.
            let materials_producer = cat == Cat::Producer && matches!(item, Some("Factory") | Some("Mine"));
            if materials_producer && needs.contains(&Resource::Materials) {
                return gap;
            }
            match behind {
                Behind::First if advances_first(cat, item) => gap,
                Behind::Presence if advances_presence(cat) => gap,
                _ => 1.0,
            }
        };
        let emits = |item: &str| matches!(item, "Industry Level" | "Power Plant" | "Factory" | "Refinery");
        let denial_for = |cat: Cat, item: &str| -> f64 {
            match kind {
                FactionKind::Prospectors if rival_run && (emits(item) || cat == Cat::Transit) => m.denial,
                FactionKind::Custodians if rival_near && matches!(cat, Cat::Restoration | Cat::Influence) => m.denial,
                _ => 1.0,
            }
        };

        // --- Earth builds
        for sid in self.directed_states(seat) {
            let free = self.free_slots(sid);
            let has_launch = self.state(sid).facilities.iter().any(|f| f.kind == FacilityKind::LaunchSite);
            if free > 0 {
                for fk in FacilityKind::ALL {
                    let (cat, mut base) = match fk {
                        FacilityKind::Factory | FacilityKind::PowerPlant | FacilityKind::Refinery | FacilityKind::Bank => (Cat::Producer, self.base_weight(seat, Cat::Producer)),
                        FacilityKind::ResearchLab => (Cat::ResearchLab, self.base_weight(seat, Cat::ResearchLab)),
                        FacilityKind::Embassy => (Cat::BuildInfluence, self.base_weight(seat, Cat::BuildInfluence)),
                        FacilityKind::LaunchSite => {
                            if has_launch {
                                continue;
                            }
                            (Cat::LaunchSiteOrShipyard, self.base_weight(seat, Cat::LaunchSiteOrShipyard))
                        }
                    };
                    let produces = self.tables.facility(fk).produces.as_ref().map(|p| p.resource);
                    if cat == Cat::Producer {
                        if produces.map(|p| needs.contains(&p) || p == scarce).unwrap_or(false) {
                            base *= 1.5;
                        }
                        if produces == Some(Resource::Energy) && tight {
                            base += m.energy_shortage_bonus;
                        }
                    }
                    let name = fk.name();
                    // Ticket #41: the first Embassy in a state is a threat answer while the rival's
                    // standing presses on the seat's own there. (An opportunity multiplier on the seat's
                    // most valuable state was tried too: four Embassies a game, and no wins.)
                    let first_embassy = fk == FacilityKind::Embassy
                        && !self.state(sid).facilities.iter().any(|f| f.kind == FacilityKind::Embassy)
                        && !self.state(sid).queue.iter().any(|b| b.item == BuildItem::Facility(FacilityKind::Embassy));
                    let sway = if first_embassy && self.standing_pressed(seat, Place::State(sid)) { m.threat } else { 1.0 };
                    push(vec![Order::BuildFacility { state: sid, kind: fk }], cat, base, gap_for(cat, Some(name)), denial_for(cat, name), sway, 1.0, format!("build {} in {}", name, self.tables.state(sid).name), None);
                }
            }
            let base = self.base_weight(seat, Cat::RaiseIndustry);
            push(vec![Order::RaiseIndustry { state: sid }], Cat::RaiseIndustry, base, gap_for(Cat::RaiseIndustry, Some("Industry Level")), denial_for(Cat::RaiseIndustry, "Industry Level"), 1.0, 1.0, format!("raise Industry Level in {}", self.tables.state(sid).name), None);
            if has_launch {
                for uk in UnitKind::SHIPS {
                    let cat = if uk == UnitKind::ColonyShip { Cat::ColonyShip } else { Cat::Warship };
                    let threat = if cat == Cat::Warship && self.enemy_present_or_inbound(seat, BodyId::Earth) { m.threat } else { 1.0 };
                    // A Colony Ship is only worth building when there is somewhere to found.
                    if uk == UnitKind::ColonyShip {
                        let colony_ships = self.ships.iter().filter(|s| s.seat == seat && s.kind == UnitKind::ColonyShip).count();
                        let queued = self.states.iter().flat_map(|s| s.queue.iter()).chain(self.colonies.iter().flat_map(|c| c.queue.iter())).filter(|b| b.seat == seat && b.item == BuildItem::Unit(UnitKind::ColonyShip)).count();
                        if colony_ships + queued >= 2 {
                            continue;
                        }
                    }
                    push(vec![Order::BuildShip { site: Place::State(sid), kind: uk }], cat, self.base_weight(seat, cat), gap_for(cat, None), 1.0, threat, 1.0, format!("build {} at {}", uk.name(), self.tables.state(sid).name), None);
                }
            }
            if self.state(sid).control == Control::Controlled(seat) {
                let threat = if self.enemy_army_near(seat, Place::State(sid)) { m.threat } else { 1.0 };
                let armies = self.armies.iter().filter(|a| !a.standing && self.army_seat(a) == Some(seat)).count();
                if armies < 2 {
                    push(vec![Order::BuildArmy { place: Place::State(sid) }], Cat::ArmyOrBarracks, self.base_weight(seat, Cat::ArmyOrBarracks), 1.0, 1.0, threat, 1.0, format!("build Army in {}", self.tables.state(sid).name), None);
                }
            }
        }

        // --- Colony builds
        for cid in self.directed_colonies(seat) {
            let col = self.colony(cid).unwrap().clone();
            let threat = if self.enemy_present_or_inbound(seat, col.body) || self.enemy_army_near(seat, Place::Colony(cid)) { m.threat } else { 1.0 };
            for mk in ModuleKind::ALL {
                let (cat, mut base) = match mk {
                    ModuleKind::Mine | ModuleKind::Generator | ModuleKind::Refinery | ModuleKind::TradePost => (Cat::Producer, self.base_weight(seat, Cat::Producer)),
                    ModuleKind::Relay => (Cat::BuildInfluence, self.base_weight(seat, Cat::BuildInfluence)),
                    ModuleKind::Habitat => (Cat::Habitat, self.base_weight(seat, Cat::Habitat)),
                    ModuleKind::Shipyard => {
                        if col.modules.iter().any(|m| m.kind == ModuleKind::Shipyard) {
                            continue;
                        }
                        (Cat::LaunchSiteOrShipyard, self.base_weight(seat, Cat::LaunchSiteOrShipyard))
                    }
                    ModuleKind::Barracks => {
                        if col.modules.iter().any(|m| m.kind == ModuleKind::Barracks) {
                            continue;
                        }
                        (Cat::ArmyOrBarracks, self.base_weight(seat, Cat::ArmyOrBarracks))
                    }
                };
                let produces = self.tables.module(mk).produces.as_ref().map(|p| p.resource);
                if cat == Cat::Producer {
                    if produces.map(|p| needs.contains(&p) || p == scarce).unwrap_or(false) {
                        base *= 1.5;
                    }
                    if produces == Some(Resource::Energy) && tight {
                        base += m.energy_shortage_bonus;
                    }
                }
                // A Habitat is only worth building when Colonists are coming.
                if mk == ModuleKind::Habitat {
                    let room = self.habitat_room(&col).saturating_sub(col.colonists);
                    if room >= 4 {
                        continue;
                    }
                }
                // Ticket #41: the first Relay at a Colony is a threat answer while the rival's standing
                // presses on the seat's own there, once the Colony has a producer Module (a Relay before
                // the first Mine starved the Colony). A second Relay is worth its base weight.
                let has_producer = col.modules.iter().any(|m| matches!(m.kind, ModuleKind::Mine | ModuleKind::Generator | ModuleKind::Refinery | ModuleKind::TradePost));
                let first_relay = mk == ModuleKind::Relay
                    && has_producer
                    && !col.modules.iter().any(|m| m.kind == ModuleKind::Relay)
                    && !col.queue.iter().any(|b| b.item == BuildItem::Module(ModuleKind::Relay));
                let t = if matches!(cat, Cat::ArmyOrBarracks) {
                    threat
                } else if first_relay && self.standing_pressed(seat, Place::Colony(cid)) {
                    m.threat
                } else {
                    1.0
                };
                push(vec![Order::BuildModule { colony: cid, kind: mk }], cat, base, gap_for(cat, Some(mk.name())), 1.0, t, 1.0, format!("build {} at {}", mk.name(), self.place_name(Place::Colony(cid))), None);
            }
            if col.modules.iter().any(|m| m.kind == ModuleKind::Barracks) && !self.armies.iter().any(|a| a.home == ArmyHome::Colony(cid)) {
                push(vec![Order::BuildArmy { place: Place::Colony(cid) }], Cat::ArmyOrBarracks, self.base_weight(seat, Cat::ArmyOrBarracks), 1.0, 1.0, threat, 1.0, format!("build Army at {}", self.place_name(Place::Colony(cid))), None);
            }
            if col.modules.iter().any(|m| m.kind == ModuleKind::Shipyard) {
                for uk in UnitKind::SHIPS {
                    let cat = if uk == UnitKind::ColonyShip { Cat::ColonyShip } else { Cat::Warship };
                    push(vec![Order::BuildShip { site: Place::Colony(cid), kind: uk }], cat, self.base_weight(seat, cat), gap_for(cat, None), 1.0, if cat == Cat::Warship { threat } else { 1.0 }, 1.0, format!("build {} at {}", uk.name(), self.place_name(Place::Colony(cid))), None);
                }
            }
        }

        // --- Influence, in units of 5 (spec 16.1), on the target rule of 16.4.
        let step = th.influence_step;
        let mut targets: Vec<(Place, f64)> = Vec::new();
        let my_states = self.controlled_states(seat);
        for sid in StateId::ALL {
            let st = self.state(sid);
            if st.control.controller() == Some(seat) || sid == StateId::Antarctica {
                continue;
            }
            let card = self.tables.state(sid);
            let near = card.neighbours.iter().any(|n| my_states.contains(n));
            // Ticket #34: the state's Influence value plus its Industry Level, closest first.
            let value = (self.state_influence_value(sid) + st.industry_level as i64) as f64 + if near { 2.0 } else { 0.0 };
            let neutral_bonus = if st.control == Control::Neutral { 1.0 } else { 0.6 };
            targets.push((Place::State(sid), value * neutral_bonus));
        }
        for c in &self.colonies {
            if c.control.controller() == Some(seat.other()) {
                targets.push((Place::Colony(c.id), 3.0 - (c.colonists as f64).min(2.0)));
            }
        }
        targets.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        for (rank, (target, _)) in targets.iter().enumerate() {
            let have = self.seat(seat).influence.get(target).copied().unwrap_or(0);
            let threshold = self.influence_threshold(*target);
            // Ticket #33: a controlled place needs a standing above the controller's as well, by the
            // challenge margin (ticket #41).
            let needed = match self.place_control(*target).controller() {
                Some(c) => threshold.max(self.seat(c).influence.get(target).copied().unwrap_or(0) + margin),
                None => threshold,
            };
            let base = self.base_weight(seat, Cat::Influence) * (1.0 - 0.15 * rank as f64).max(0.3);
            let opp = if needed - have <= step { m.opportunity } else { 1.0 };
            let denial = if kind == FactionKind::Custodians && rival_near && self.place_control(*target).controller() == Some(seat.other()) { m.denial } else { 1.0 };
            let bought = if self.tables.ducats.per_influence > 0 { self.seat(seat).stockpile.ducats / self.tables.ducats.per_influence } else { 0 };
            let copies = ((allotment + bought) / step).max(0);
            for _ in 0..copies {
                push(vec![Order::Influence { target: *target, amount: step }], Cat::Influence, base, 1.0, denial, 1.0, opp, format!("spend {} Influence on {}", step, self.place_name(*target)), None);
            }
        }
        // Buy Influence with Ducats (ticket #35), in units of the step, weighted like Influence itself.
        let ducats = self.seat(seat).stockpile.ducats;
        let per = self.tables.ducats.per_influence;
        let buys = if per > 0 { ducats / (per * step) } else { 0 };
        for _ in 0..buys {
            push(vec![Order::BuyInfluence { amount: step }], Cat::Influence, self.base_weight(seat, Cat::Influence) * 0.9, 1.0, 1.0, 1.0, 1.0, format!("buy {} Influence for {} Ducats", step, per * step), None);
        }
        // Ticket #42: the trading window. While Materials are the scarcest resource (or the bootstrap
        // need), Ducats buy them in lots of 10 at a producer's weight; the AI does not sell.
        let per_materials = self.tables.ducats.per_materials;
        if (scarce == Resource::Materials || needs.contains(&Resource::Materials)) && per_materials > 0 {
            let lots = self.seat(seat).stockpile.ducats / (per_materials * 10);
            for _ in 0..lots.min(4) {
                push(vec![Order::Buy { resource: Resource::Materials, amount: 10 }], Cat::Producer, self.base_weight(seat, Cat::Producer) * 1.5, 1.0, 1.0, 1.0, 1.0, format!("buy 10 Materials for {} Ducats", per_materials * 10), None);
            }
        }
        // Hold own places where a rival's standing approaches yours (ticket #33: spending raises your standing).
        let mut owned: Vec<Place> = self.controlled_states(seat).into_iter().map(Place::State).collect();
        owned.extend(self.colonies.iter().filter(|c| c.control.controller() == Some(seat)).map(|c| Place::Colony(c.id)));
        for place in owned {
            let rival = self.seat(seat.other()).influence.get(&place).copied().unwrap_or(0);
            let mine = self.seat(seat).influence.get(&place).copied().unwrap_or(0);
            if rival > 0 && rival + 2 * step >= mine {
                let opp = if rival + step >= mine { m.opportunity } else { 1.0 };
                push(vec![Order::Influence { target: place, amount: step }], Cat::Influence, self.base_weight(seat, Cat::Influence), 1.0, 1.0, m.threat, opp, format!("hold {} with {} Influence", self.place_name(place), step), None);
            }
        }

        // --- Restoration
        if kind == FactionKind::Custodians {
            let energy = self.seat(seat).stockpile.energy;
            let steps = (energy / self.tables.restoration.energy_per_step).max(0);
            let net = self.climate.last.counted() - self.climate.last.total_sink();
            let needed = if net > 0.0 { (net / self.tables.restoration.sink_per_step).ceil() as i64 } else { 0 };
            for i in 0..steps.min(needed + 1) {
                let opp = if i + 1 == needed { m.opportunity } else { 1.0 };
                push(vec![Order::Restoration { steps: 1 }], Cat::Restoration, self.base_weight(seat, Cat::Restoration), gap_for(Cat::Restoration, None), denial_for(Cat::Restoration, ""), 1.0, opp, "spend 10 Energy on Restoration".into(), None);
            }
        }

        // --- Ships: load, unload, found, transit
        let ships: Vec<Ship> = self.ships.iter().filter(|s| s.seat == seat && matches!(s.at, ShipAt::Body(_)) && !s.arrived_this_turn).cloned().collect();
        let presence_needed = self.tables.victory.off_world_presence.saturating_sub(self.off_world_colonists(seat));
        for s in &ships {
            let ShipAt::Body(body) = s.at else { continue };
            let card = self.tables.unit(s.kind);
            let ship_name = format!("{} {}", s.kind.name(), s.id.0);
            if s.kind == UnitKind::ColonyShip {
                if body == BodyId::Earth && s.colonists < card.carries_colonists {
                    // Load from the most populous directed state.
                    let from = self.directed_states(seat).into_iter().max_by(|a, b| self.state(*a).population.partial_cmp(&self.state(*b).population).unwrap());
                    if let Some(st) = from {
                        let n = card.carries_colonists - s.colonists;
                        let opp = if presence_needed <= n { m.opportunity } else { 1.0 };
                        push(vec![Order::Load { ship: s.id, colonists: n, from: LoadSource::State(st), army: None }], Cat::LoadUnload, self.base_weight(seat, Cat::LoadUnload), gap_for(Cat::LoadUnload, None), 1.0, 1.0, opp, format!("load {} Colonists onto {}", n, ship_name), None);
                    }
                }
                if s.colonists > 0 && body != BodyId::Earth {
                    let free = self.free_slots_on(body);
                    if let Some(slot) = free.first() {
                        let opp = if free.len() == 1 || presence_needed <= s.colonists { m.opportunity } else { 1.0 };
                        push(vec![Order::Unload { ship: s.id, colonists: s.colonists, army: false, into: UnloadTarget::Slot(body, *slot) }], Cat::FoundColony, self.base_weight(seat, Cat::FoundColony), gap_for(Cat::FoundColony, None), 1.0, 1.0, opp, format!("found a Colony in slot {} on {}", slot + 1, self.tables.body(body).name), None);
                    }
                    for c in self.colonies.iter().filter(|c| c.body == body && c.control.director() == Some(seat)) {
                        let room = self.habitat_room(c).saturating_sub(c.colonists);
                        if room > 0 {
                            let n = room.min(s.colonists);
                            let opp = if presence_needed <= n { m.opportunity } else { 1.0 };
                            push(vec![Order::Unload { ship: s.id, colonists: n, army: false, into: UnloadTarget::Colony(c.id) }], Cat::LoadUnload, self.base_weight(seat, Cat::LoadUnload), gap_for(Cat::LoadUnload, None), 1.0, 1.0, opp, format!("disembark {} Colonists into {}", n, self.place_name(Place::Colony(c.id))), None);
                        }
                    }
                }
                if s.colonists > 0 && body == BodyId::Earth {
                    let dest = self.best_body_for(seat, behind);
                    let own_room = self.colonies.iter().any(|c| c.control.director() == Some(seat) && self.habitat_room(c) > c.colonists);
                    let mut dests = vec![dest];
                    if own_room {
                        for c in &self.colonies {
                            if c.control.director() == Some(seat) && !dests.contains(&c.body) {
                                dests.push(c.body);
                            }
                        }
                    }
                    for d in dests {
                        push(vec![Order::Transit { ship: s.id, to: d }], Cat::Transit, self.base_weight(seat, Cat::Transit), gap_for(Cat::Transit, None), 1.0, 1.0, 1.0, format!("send {} to {}", ship_name, self.tables.body(d).name), None);
                    }
                }
                if s.colonists == 0 && body != BodyId::Earth {
                    push(vec![Order::Transit { ship: s.id, to: BodyId::Earth }], Cat::Transit, self.base_weight(seat, Cat::Transit) * 0.8, gap_for(Cat::Transit, None), 1.0, 1.0, 1.0, format!("send {} back to Earth", ship_name), None);
                }
            }
            if s.kind.is_warship() || (s.kind == UnitKind::ColonyShip && s.army.is_some()) {
                // Warships go where the Faction has or wants Colonies, or where the rival is.
                let mut dests: Vec<BodyId> = Vec::new();
                for c in &self.colonies {
                    if c.body != body && (c.control.director() == Some(seat) || c.control.director() == Some(seat.other())) && !dests.contains(&c.body) {
                        dests.push(c.body);
                    }
                }
                for d in dests {
                    let threat = if self.enemy_present_or_inbound(seat, d) { m.threat } else { 1.0 };
                    let enemy_colony = self.colonies.iter().any(|c| c.body == d && c.control.director() == Some(seat.other()));
                    let denial = if enemy_colony && rival_one_turn { m.denial } else { 1.0 };
                    let base = self.base_weight(seat, Cat::Transit) * if kind == FactionKind::Prospectors { 0.9 } else { 0.6 };
                    push(vec![Order::Transit { ship: s.id, to: d }], Cat::Transit, base, 1.0, denial, threat, 1.0, format!("send {} to {}", ship_name, self.tables.body(d).name), None);
                }
                // Load an Army aboard a Battleship or Colony Ship at Earth for an attack on a rival Colony.
                if card.carries_army && s.army.is_none() && body == BodyId::Earth && kind == FactionKind::Prospectors {
                    let army = self.armies.iter().find(|a| !a.standing && self.army_seat(a) == Some(seat) && matches!(a.at, ArmyAt::Place(Place::State(_))));
                    let enemy_colony = self.colonies.iter().any(|c| c.control.director() == Some(seat.other()));
                    if let (Some(a), true) = (army, enemy_colony) {
                        push(vec![Order::Load { ship: s.id, colonists: 0, from: LoadSource::State(StateId::Asia), army: Some(a.id) }], Cat::LoadUnload, self.base_weight(seat, Cat::LoadUnload) * 0.8, 1.0, 1.0, 1.0, 1.0, format!("load an Army onto {}", ship_name), None);
                    }
                }
                if let Some(aid) = s.army.filter(|_| body != BodyId::Earth) {
                    {
                        for c in self.colonies.iter().filter(|c| c.body == body) {
                            let enemy = c.control.director() == Some(seat.other());
                            let mine = c.control.director() == Some(seat);
                            if enemy || mine {
                                let odds = first_round_odds(self.army_strength(self.army(aid).unwrap()), self.army_stack_strength(seat.other(), Place::Colony(c.id)));
                                if enemy && odds < th.attack_odds && !rival_one_turn {
                                    continue;
                                }
                                let base = self.base_weight(seat, Cat::LoadUnload);
                                push(vec![Order::Unload { ship: s.id, colonists: 0, army: true, into: UnloadTarget::Colony(c.id) }], Cat::LoadUnload, base, 1.0, if enemy && rival_one_turn { m.denial } else { 1.0 }, if mine { m.threat } else { 1.0 }, 1.0, format!("land an Army at {}", self.place_name(Place::Colony(c.id))), None);
                            }
                        }
                    }
                }
            }
        }

        // --- Stances for every Ship stack
        for body in BodyId::ALL {
            let stack: Vec<&Ship> = self.ships.iter().filter(|s| s.seat == seat && s.at == ShipAt::Body(body)).collect();
            if stack.is_empty() {
                continue;
            }
            let key = format!("ships@{body:?}");
            let my_str = self.ship_stack_strength(seat, body);
            let enemy_str = self.ship_stack_strength(seat.other(), body);
            let enemy_here = self.ships.iter().any(|s| s.seat != seat && s.at == ShipAt::Body(body));
            let inbound_target = self.ships.iter().any(|s| s.seat != seat && matches!(s.kind, UnitKind::ColonyShip | UnitKind::Battleship) && matches!(s.at, ShipAt::Transit { to, .. } if to == body));
            let threat = if self.enemy_present_or_inbound(seat, body) { m.threat } else { 1.0 };
            let total_hp: u32 = stack.iter().map(|s| self.tables.unit(s.kind).hit_points).sum();
            let total_dmg: u32 = stack.iter().map(|s| s.damage).sum();
            let warships = stack.iter().any(|s| s.kind.is_warship());
            push(vec![Order::ShipStance { body, stance: Stance::Hold }], Cat::StanceHold, self.base_weight(seat, Cat::StanceHold), 1.0, 1.0, threat, 1.0, format!("Hold at {}", self.tables.body(body).name), Some(key.clone()));
            if warships && enemy_here {
                let odds = first_round_odds(my_str, enemy_str);
                let my_colony_here = self.colonies.iter().any(|c| c.body == body && c.control.controller() == Some(seat));
                let allowed = match kind {
                    FactionKind::Prospectors => odds >= th.attack_odds,
                    FactionKind::Custodians => odds >= th.attack_odds && my_colony_here && self.orbital_control(body) == Some(seat.other()),
                } || (rival_one_turn && odds >= th.attack_odds_versus_near_winner);
                if allowed {
                    push(vec![Order::ShipStance { body, stance: Stance::Attack }], Cat::StanceAttack, self.base_weight(seat, Cat::StanceAttack), 1.0, if rival_one_turn { m.denial } else { 1.0 }, 1.0, 1.0, format!("Attack at {} (odds {:.0}%)", self.tables.body(body).name, odds * 100.0), Some(key.clone()));
                }
            }
            if warships && self.orbital_control(body) == Some(seat) && inbound_target {
                push(vec![Order::ShipStance { body, stance: Stance::Intercept }], Cat::StanceIntercept, self.base_weight(seat, Cat::StanceIntercept), 1.0, 1.0, threat, 1.0, format!("Intercept at {}", self.tables.body(body).name), Some(key.clone()));
            }
            if total_hp > 0 && (total_dmg as f64) / (total_hp as f64) >= th.evade_damage_fraction {
                push(vec![Order::ShipStance { body, stance: Stance::Evade }], Cat::StanceEvade, self.base_weight(seat, Cat::StanceEvade) * 10.0, 1.0, 1.0, 1.0, 1.0, format!("Evade at {}", self.tables.body(body).name), Some(key.clone()));
            }
        }

        // --- Armies on Earth: Stances and attacks on neighbours
        let mut places: Vec<Place> = StateId::ALL.into_iter().map(Place::State).collect();
        places.extend(self.colonies.iter().map(|c| Place::Colony(c.id)));
        for place in places {
            let mine = self.armies_of_seat_at(seat, place);
            if mine.is_empty() {
                continue;
            }
            let key = format!("armies@{place:?}");
            let threat = if self.enemy_army_near(seat, place) { m.threat } else { 1.0 };
            push(vec![Order::ArmyStance { place, stance: Stance::Hold }], Cat::StanceHold, self.base_weight(seat, Cat::StanceHold), 1.0, 1.0, threat, 1.0, format!("Hold at {}", self.place_name(place)), Some(key.clone()));
            let my_str = self.army_stack_strength(seat, place);
            let total_hp: u32 = mine.iter().map(|_| self.tables.unit(UnitKind::Army).hit_points).sum();
            let total_dmg: u32 = mine.iter().filter_map(|id| self.army(*id)).map(|a| a.damage).sum();
            if total_hp > 0 && (total_dmg as f64) / (total_hp as f64) >= th.evade_damage_fraction {
                push(vec![Order::ArmyStance { place, stance: Stance::Evade }], Cat::StanceEvade, self.base_weight(seat, Cat::StanceEvade) * 10.0, 1.0, 1.0, 1.0, 1.0, format!("Evade at {}", self.place_name(place)), Some(key.clone()));
            }
            // Attack where this seat does not direct the place and defenders stand.
            if self.place_director(place) != Some(seat) {
                let def: i64 = self.defenders_at(place, seat).iter().filter_map(|id| self.army(*id)).map(|a| self.army_strength(a)).sum();
                let odds = first_round_odds(my_str, def);
                let lost_place = self.place_control(place).controller() == Some(seat) || matches!(self.place_control(place), Control::Occupied { previous: Some(p), .. } if p == seat);
                let allowed = match kind {
                    FactionKind::Prospectors => odds >= th.attack_odds || def == 0,
                    FactionKind::Custodians => lost_place && (odds >= th.attack_odds || def == 0),
                } || (rival_one_turn && odds >= th.attack_odds_versus_near_winner);
                if allowed {
                    push(vec![Order::ArmyStance { place, stance: Stance::Attack }], Cat::StanceAttack, self.base_weight(seat, Cat::StanceAttack) * 1.5, 1.0, if rival_one_turn { m.denial } else { 1.0 }, 1.0, 1.0, format!("Attack at {} (odds {:.0}%)", self.place_name(place), odds * 100.0), Some(key.clone()));
                }
            }
            // Moves into neighbouring states with a non-standing Army: Prospectors take weak neutrals.
            if let Place::State(sid) = place {
                for aid in &mine {
                    let a = self.army(*aid).unwrap();
                    if a.standing || a.damage > 2 {
                        continue;
                    }
                    for n in &self.tables.state(sid).neighbours {
                        let ctrl = self.state(*n).control;
                        if ctrl == Control::Controlled(seat) || *n == StateId::Antarctica {
                            continue;
                        }
                        let def: i64 = self.defenders_at(Place::State(*n), seat).iter().filter_map(|id| self.army(*id)).map(|a| self.army_strength(a)).sum();
                        let odds = first_round_odds(self.army_strength(a), def);
                        let allowed = match kind {
                            FactionKind::Prospectors => odds >= th.attack_odds,
                            FactionKind::Custodians => matches!(ctrl, Control::Occupied { previous: Some(p), .. } if p == seat) && odds >= th.attack_odds,
                        };
                        if allowed {
                            let value = (self.tables.state(*n).industry_level + self.tables.state(*n).size) as f64 / 7.0;
                            push(vec![Order::MoveArmy { army: *aid, to: *n }], Cat::StanceAttack, self.base_weight(seat, Cat::StanceAttack) * (1.0 + value), 1.0, 1.0, 1.0, 1.0, format!("march on {} (odds {:.0}%)", self.tables.state(*n).name, odds * 100.0), None);
                        }
                    }
                }
            }
        }

        // --- Sort and spend greedily; Stances last, one per stack.
        cands.sort_by(|a, b| b.score().partial_cmp(&a.score()).unwrap_or(std::cmp::Ordering::Equal));
        let mut chosen: Vec<Order> = Vec::new();
        let mut stacks_done: Vec<String> = Vec::new();
        let mut lines: Vec<String> = Vec::new();
        // Saving: once a legal, higher-scored action is out of reach now but within one more turn of
        // Materials income, Materials are held for it rather than spent on lower-scored actions.
        let mut reserve: Option<String> = None;
        for c in cands.iter().filter(|c| c.stack.is_none()) {
            let mut ok = true;
            let mut trial = chosen.clone();
            let materials_cost: i64 = c.orders.iter().map(|o| self.order_cost(seat, o).materials).sum();
            if materials_cost > 0 {
                if let Some(note) = &reserve {
                    lines.push(format!("  save  {:6.1}  {} (holding Materials for {})", c.score(), c.note, note));
                    continue;
                }
                let (left, _) = self.remaining(seat, &chosen);
                if materials_cost > left.materials
                    && materials_cost <= left.materials + materials_income
                    && c.orders.iter().all(|o| self.check_order_legality(seat, &chosen, o).is_ok())
                {
                    reserve = Some(c.note.clone());
                    lines.push(format!("  wait  {:6.1}  {} (affordable next turn)", c.score(), c.note));
                    continue;
                }
            }
            for o in &c.orders {
                match self.check_order(seat, &trial, o) {
                    Ok(_) => trial.push(o.clone()),
                    Err(e) => {
                        lines.push(format!("  skip  {:6.1}  {} ({})", c.score(), c.note, e));
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                lines.push(format!("  take  {:6.1}  {} [{:?}]", c.score(), c.note, c.cat));
                chosen = trial;
            }
        }
        for c in cands.iter().filter(|c| c.stack.is_some()) {
            let key = c.stack.clone().unwrap();
            if stacks_done.contains(&key) {
                continue;
            }
            let mut trial = chosen.clone();
            let mut ok = true;
            for o in &c.orders {
                match self.check_order(seat, &trial, o) {
                    Ok(_) => trial.push(o.clone()),
                    Err(_) => {
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                lines.push(format!("  take  {:6.1}  {}", c.score(), c.note));
                chosen = trial;
                stacks_done.push(key);
            }
        }
        self.log(format!("AI {} scored {} actions (gap x{:.2} on {:?}):", self.seat_name(seat), cands.len(), gap, behind));
        for l in &lines {
            self.log(l.clone());
        }
        self.report.ai_lines = lines;
        chosen
    }
}

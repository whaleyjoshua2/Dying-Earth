//! The AI faction (spec 16): enumerate every legal action, score it, spend greedily.

use crate::combat::first_round_odds;
use crate::data::VictoryFirstKind;
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
    /// Ticket #52: a Constabulary, Relief and Resettle.
    Constabulary,
    Relief,
    Resettle,
    /// Ticket #51: divert this turn's Research into the Archive fund.
    FundArchive,
    /// Ticket #51: order the next stage of the Archive.
    ArchiveStage,
    Influence,
    Transit,
    LoadUnload,
    FoundColony,
    /// Ticket #54: the Scrubber, which took Restoration's place and its Stabilization gap.
    Scrubber,
    /// Ticket #54: Mothball, Restart and Decommission, on a Facility or a Module.
    Mothball,
    Restart,
    Decommission,
    /// Ticket #54: the Custodians' Leapfrog and the Prospectors' Strip Permit.
    Leapfrog,
    StripPermit,
    StanceAttack,
    StanceIntercept,
    StanceHold,
    StanceEvade,
}

/// Ticket #50 removed the denial multiplier: every AI pursues its own Victory Condition and never
/// spends a multiplier on holding a rival back.
#[derive(Debug, Clone)]
struct Candidate {
    orders: Vec<Order>,
    cat: Cat,
    base: f64,
    gap: f64,
    threat: f64,
    opportunity: f64,
    note: String,
    /// The stack a Stance candidate belongs to; one Stance per stack.
    stack: Option<String>,
}

impl Candidate {
    fn score(&self) -> f64 {
        self.base * self.gap * self.threat * self.opportunity
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
            Cat::Constabulary => w.build_constabulary,
            Cat::Relief => w.relief,
            Cat::Resettle => w.resettle,
            Cat::FundArchive => w.fund_archive,
            Cat::ArchiveStage => w.build_archive_stage,
            Cat::Influence => w.influence,
            Cat::Transit => w.transit,
            Cat::LoadUnload => w.load_unload,
            Cat::FoundColony => w.found_colony,
            Cat::Scrubber => w.build_scrubber,
            Cat::Mothball => w.mothball,
            Cat::Restart => w.restart,
            Cat::Decommission => w.decommission,
            Cat::Leapfrog => w.leapfrog,
            Cat::StripPermit => w.strip_permit,
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

    /// The measure this seat's Victory Condition counts first (ticket #50).
    fn first_kind(&self, seat: Seat) -> VictoryFirstKind {
        self.tables.faction(self.kind(seat)).victory_first.kind
    }

    /// Victory gap multiplier and the part it applies to.
    fn victory_gap(&self, seat: Seat) -> (f64, Behind) {
        let pace = self.tables.ai_pace(self.kind(seat));
        let m = &self.tables.ai.multipliers;
        let turn = self.turn;
        let ratio_of = |actual: f64, expected: f64| if expected <= 0.0 { 1.0 } else { (actual / expected).min(1.0) };
        let presence_ratio = ratio_of(self.off_world_colonists(seat) as f64, Self::expected(&pace.colonists, turn));
        let first_ratio = match self.first_kind(seat) {
            // A Stabilization run has no useful interpolation: the pace is in ppm off the Sink.
            VictoryFirstKind::StabilizationRun => {
                let net = self.climate.last.counted() - self.climate.last.total_sink();
                if turn >= pace.under_sink_by_turn {
                    if net < 0.0 { 1.0 } else { (1.0 - net / 10.0).clamp(0.0, 1.0) }
                } else if turn >= pace.within_by_turn {
                    if net <= pace.within_ppm { 1.0 } else { (pace.within_ppm / net).clamp(0.0, 1.0) }
                } else {
                    1.0
                }
            }
            _ => ratio_of(self.progress(seat).first_value, Self::expected(&pace.first, turn)),
        };
        let (ratio, behind) = if first_ratio <= presence_ratio { (first_ratio, Behind::First) } else { (presence_ratio, Behind::Presence) };
        let mult = if ratio >= 1.0 { 1.0 } else { 1.0 + (m.victory_gap_max - 1.0) * ((1.0 - ratio) / 0.5).min(1.0) };
        (mult, behind)
    }

    fn enemy_present_or_inbound(&self, seat: Seat, body: BodyId) -> bool {
        self.ships.iter().any(|s| s.seat != seat && (s.at == ShipAt::Body(body) || matches!(s.at, ShipAt::Transit { to, .. } if to == body)))
    }

    /// Ticket #50: an Army of ANY other seat, not only one rival's.
    fn enemy_army_near(&self, seat: Seat, place: Place) -> bool {
        let theirs = |a: &Army| self.army_seat(a).map(|o| o != seat).unwrap_or(false);
        match place {
            Place::State(s) => {
                let mut near = vec![s];
                near.extend(self.tables.state(s).neighbours.iter().copied());
                self.armies.iter().any(|a| !a.standing && theirs(a) && matches!(a.at, ArmyAt::Place(Place::State(x)) if near.contains(&x)))
            }
            Place::Colony(c) => {
                let body = self.colony(c).map(|c| c.body);
                self.armies.iter().any(|a| theirs(a) && a.at == ArmyAt::Place(place))
                    || body.map(|b| self.ships.iter().any(|s| s.seat != seat && s.army.is_some() && (s.at == ShipAt::Body(b) || matches!(s.at, ShipAt::Transit { to, .. } if to == b)))).unwrap_or(false)
            }
        }
    }

    /// The total strength of every other seat's Ships at a Body (ticket #50): a Battle there is a
    /// melee, so the odds preview counts all of them.
    pub fn enemy_ship_strength(&self, seat: Seat, body: BodyId) -> i64 {
        seat.others().iter().map(|s| self.ship_stack_strength(*s, body)).sum()
    }

    /// Whether any other seat directs this Colony.
    fn rival_holds(&self, seat: Seat, c: &Colony) -> bool {
        c.control.director().map(|d| d != seat).unwrap_or(false)
    }

    /// The Body whose yields best serve the part the AI is furthest behind on (spec 16.4).
    fn best_body_for(&self, seat: Seat, behind: Behind) -> BodyId {
        let t = &self.tables;
        let mut bodies: Vec<BodyId> = BodyId::ALL.into_iter().filter(|b| *b != BodyId::Earth && !self.free_slots_on(*b).is_empty()).collect();
        if bodies.is_empty() {
            return BodyId::Moon;
        }
        // Ticket #51, the Arkwrights' spread rule: once one Body of theirs holds the 4 Colonists
        // Diaspora asks of each, the next Colony goes to a Body they are not on yet.
        let each = t.faction(self.kind(seat)).victory_second.colonists_each;
        let spreading = each > 0 && BodyId::ALL.into_iter().any(|b| b != BodyId::Earth && self.colonists_at_body(seat, b) >= each);
        let key = |b: &BodyId| -> f64 {
            let c = t.body(*b);
            let yields = match behind {
                Behind::Presence => c.habitat_yield,
                Behind::First => match self.first_kind(seat) {
                    VictoryFirstKind::ExtractionTotal => c.mine_yield + c.refinery_yield,
                    VictoryFirstKind::ColonistsOffEarth => c.habitat_yield,
                    VictoryFirstKind::StabilizationRun | VictoryFirstKind::ResearchProduced | VictoryFirstKind::ArchiveStages => {
                        c.generator_yield + c.habitat_yield
                    }
                },
            };
            let fresh = if spreading && self.colonists_at_body(seat, *b) == 0 { 10.0 } else { 0.0 };
            yields + fresh
        };
        bodies.sort_by(|a, b| key(b).partial_cmp(&key(a)).unwrap());
        bodies[0]
    }

    /// Every Energy upkeep the seat pays now: Facilities, Modules, Ships and Armies.
    fn total_upkeep(&self, seat: Seat) -> i64 {
        self.unit_upkeep(seat)
            // Ticket #54: a mothballed building pays no upkeep, so it is no part of the drain.
            + self.directed_states(seat).iter().flat_map(|s| self.state(*s).facilities.iter()).filter(|f| !f.mothballed).map(|f| self.tables.facility(f.kind).energy_upkeep).sum::<i64>()
            + self.directed_colonies(seat).iter().flat_map(|c| self.colony(*c).unwrap().modules.iter()).filter(|m| !m.mothballed).map(|m| self.tables.module(m.kind).energy_upkeep).sum::<i64>()
    }

    /// Energy the seat's producers make in a turn, at current multipliers.
    fn energy_production(&self, seat: Seat) -> i64 {
        let fac = self.tables.faction(self.kind(seat)).output_multiplier;
        let grids = if self.has_tech(TechId::EfficientGrids) { self.tables.tech(TechId::EfficientGrids).value } else { 1.0 };
        let mut e = 0.0;
        for sid in self.directed_states(seat) {
            let lean = if self.tables.state(sid).resource_lean == Resource::Energy { 1.5 } else { 1.0 };
            for f in self.state(sid).facilities.iter().filter(|f| !f.mothballed) {
                if f.kind == FacilityKind::PowerPlant {
                    e += (6.0 * lean * fac * grids).floor();
                }
            }
        }
        for cid in self.directed_colonies(seat) {
            let col = self.colony(cid).unwrap();
            for m in col.modules.iter().filter(|m| !m.mothballed) {
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

    /// Ticket #43: a Carrier is worth building when the seat has a free Army on Earth, a rival Colony
    /// to land on, and no empty Carrier (or one on the way) already.
    fn wants_carrier(&self, seat: Seat) -> bool {
        if self.kind(seat) != FactionKind::Prospectors {
            return false;
        }
        let free_army = self.armies.iter().any(|a| !a.standing && self.army_seat(a) == Some(seat) && matches!(a.at, ArmyAt::Place(Place::State(_))));
        let enemy_colony = self.colonies.iter().any(|c| self.rival_holds(seat, c));
        let empty_carrier = self.ships.iter().any(|s| s.seat == seat && s.kind == UnitKind::Carrier && s.army.is_none());
        let queued = self.states.iter().flat_map(|s| s.queue.iter()).any(|b| b.seat == seat && b.item == BuildItem::Unit(UnitKind::Carrier));
        free_army && enemy_colony && !empty_carrier && !queued
    }

    /// Ticket #41: the rival's standing on a place the seat holds is within the challenge margin of
    /// the seat's own, so the place could be lost to a short push.
    fn standing_pressed(&self, seat: Seat, place: Place) -> bool {
        let mine = self.seat(seat).influence.get(&place).copied().unwrap_or(0);
        let rival = self.rival_standing(seat, place);
        rival > 0 && rival + self.tables.influence.challenge_margin >= mine
    }

    /// The highest Standing any other seat has on a place (ticket #50).
    fn rival_standing(&self, seat: Seat, place: Place) -> i64 {
        seat.others().iter().map(|s| self.seat(*s).influence.get(&place).copied().unwrap_or(0)).max().unwrap_or(0)
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
        let first_kind = self.first_kind(seat);
        let scarce = self.scarcest(seat);
        let tight = self.energy_tight(seat);
        let allotment = self.seat(seat).allotment;
        let margin = self.tables.influence.challenge_margin;
        let materials_income = self.seat(seat).income_last_turn.materials;
        let no_materials_income = materials_income == 0
            && !self.directed_states(seat).iter().any(|s| self.state(*s).facilities.iter().any(|f| f.kind == FacilityKind::Factory))
            && !self.directed_colonies(seat).iter().any(|c| self.colony(*c).unwrap().modules.iter().any(|m| m.kind == ModuleKind::Mine))
            && !self.states.iter().flat_map(|s| s.queue.iter()).any(|b| b.seat == seat && b.item == BuildItem::Facility(FacilityKind::Factory));
        // Ticket #46: until the seat has a Shipyard anywhere, one counts as advancing whatever it is behind on.
        let no_shipyard = !self.directed_colonies(seat).iter().any(|c| {
            let col = self.colony(*c).unwrap();
            col.modules.iter().any(|m| m.kind == ModuleKind::Shipyard) || col.queue.iter().any(|b| b.item == BuildItem::Module(ModuleKind::Shipyard))
        });
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
        // Ticket #54: the state a Leapfrog would go to (the most populous the seat controls) and the
        // one a Strip Permit would (the one whose Facilities make the most), settled once.
        let most_populous = self
            .controlled_states(seat)
            .into_iter()
            .filter(|s| self.leapfrog_would_bite(*s))
            .max_by(|a, b| self.state(*a).population.partial_cmp(&self.state(*b).population).unwrap_or(std::cmp::Ordering::Equal));
        let output_of = |sid: StateId| -> i64 {
            self.state(sid).facilities.iter().filter(|f| !f.mothballed).map(|f| self.facility_yield(seat, sid, f.kind).amount).sum()
        };
        let highest_output = self.controlled_states(seat).into_iter().filter(|s| !self.state(*s).strip_permit_used).max_by_key(|s| output_of(*s));
        // Ticket #54: behind on the Extraction pace itself, whichever part the seat is furthest
        // behind on overall: the Strip Permit is bought against the Extraction schedule.
        let behind_on_extraction = first_kind == VictoryFirstKind::ExtractionTotal && {
            let pace = self.tables.ai_pace(kind);
            let want = Self::expected(&pace.first, self.turn);
            want > 0.0 && (self.progress(seat).first_value) < want
        };
        let mut cands: Vec<Candidate> = Vec::new();

        let mut push = |orders: Vec<Order>, cat: Cat, base: f64, gap: f64, threat: f64, opportunity: f64, note: String, stack: Option<String>| {
            cands.push(Candidate { orders, cat, base, gap, threat, opportunity, note, stack });
        };

        // What advances the Faction's own first Victory part (ticket #50).
        let advances_first = |cat: Cat, item: Option<&str>| -> bool {
            match first_kind {
                VictoryFirstKind::ExtractionTotal => {
                    cat == Cat::Producer && item.map(|i| i != "Power Plant" && i != "Generator").unwrap_or(false)
                        || cat == Cat::RaiseIndustry
                        // Ticket #54: a Strip Permit is three turns of double Extraction.
                        || cat == Cat::StripPermit
                }
                // Ticket #54: a Scrubber is what a Custodian buys Stabilization with now.
                VictoryFirstKind::StabilizationRun => cat == Cat::Scrubber || cat == Cat::Leapfrog || cat == Cat::ResearchLab,
                VictoryFirstKind::ColonistsOffEarth => matches!(cat, Cat::Habitat | Cat::ColonyShip | Cat::FoundColony | Cat::LoadUnload | Cat::Transit),
                VictoryFirstKind::ResearchProduced => cat == Cat::ResearchLab,
                // Ticket #51: the Archive wants Research, a fund and stages, and a Colony off Earth
                // to stand at, which the Colony Ship, the transit and the founding provide.
                VictoryFirstKind::ArchiveStages => {
                    matches!(cat, Cat::ArchiveStage | Cat::FundArchive | Cat::ResearchLab | Cat::ColonyShip | Cat::FoundColony | Cat::Transit | Cat::LoadUnload)
                }
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
            if cat == Cat::LaunchSiteOrShipyard && item == Some("Shipyard") && no_shipyard {
                return gap;
            }
            match behind {
                Behind::First if advances_first(cat, item) => gap,
                Behind::Presence if advances_presence(cat) => gap,
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
                        // Ticket #52: a Constabulary is worth raising only where Unrest has taken hold.
                        FacilityKind::Constabulary => {
                            if self.state(sid).unrest < 5.0 {
                                continue;
                            }
                            (Cat::Constabulary, self.base_weight(seat, Cat::Constabulary))
                        }
                        FacilityKind::LaunchSite => {
                            if has_launch {
                                continue;
                            }
                            (Cat::LaunchSiteOrShipyard, self.base_weight(seat, Cat::LaunchSiteOrShipyard))
                        }
                        // Ticket #54: a Scrubber has its own weight, its own cap and no build slot,
                        // so it is enumerated below rather than here.
                        FacilityKind::Scrubber => continue,
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
                    // Ticket #52: a Constabulary in a state the seat has just Occupied is worth more:
                    // Occupation is what put the Unrest there, and Pacification halves above 4.
                    let just_occupied = fk == FacilityKind::Constabulary && self.state(sid).control.is_occupied();
                    let sway = if (first_embassy && self.standing_pressed(seat, Place::State(sid))) || just_occupied { m.threat } else { 1.0 };
                    push(vec![Order::BuildFacility { state: sid, kind: fk }], cat, base, gap_for(cat, Some(name)), sway, 1.0, format!("build {} in {}", name, self.tables.state(sid).name), None);
                }
            }
            // Ticket #54: a Scrubber takes no build slot, so it is offered whether or not one is
            // free, up to the state's cap, and only while the seat's Energy is not already tight:
            // 4 Energy upkeep with nothing to run it is a Facility shut down at the next Income.
            if kind == FactionKind::Custodians
                && self.state(sid).control == Control::Controlled(seat)
                && self.scrubbers_committed(sid) < self.scrubber_cap(sid)
                && !tight
            {
                let cat = Cat::Scrubber;
                let opp = if self.scrubbers_online(sid) == 0 { m.opportunity } else { 1.0 };
                push(
                    vec![Order::BuildFacility { state: sid, kind: FacilityKind::Scrubber }],
                    cat,
                    self.base_weight(seat, cat),
                    gap_for(cat, Some("Scrubber")),
                    1.0,
                    opp,
                    format!("build a Scrubber in {} ({} of {})", self.tables.state(sid).name, self.scrubbers_committed(sid) + 1, self.scrubber_cap(sid)),
                    None,
                );
            }
            // Ticket #54: Leapfrog, the Custodians' other clause, on the most populous state they
            // hold once they have Ducats to spare.
            if kind == FactionKind::Custodians
                && self.seat(seat).stockpile.ducats + 3 * self.seat(seat).income_last_turn.ducats >= self.tables.ducats.per_leapfrog
                && self.state(sid).control == Control::Controlled(seat)
                && self.leapfrog_would_bite(sid)
                && most_populous == Some(sid)
            {
                push(
                    vec![Order::Leapfrog { state: sid }],
                    Cat::Leapfrog,
                    self.base_weight(seat, Cat::Leapfrog),
                    gap_for(Cat::Leapfrog, None),
                    1.0,
                    1.0,
                    format!("Leapfrog {} ({:.2} per hundred million now)", self.tables.state(sid).name, self.population_coefficient(sid)),
                    None,
                );
            }
            // Ticket #54: the Strip Permit, on the Prospectors' highest-output state while they are
            // behind on the Extraction pace. It is free, and its price falls due three turns later.
            if kind == FactionKind::Prospectors
                && behind_on_extraction
                && !self.state(sid).strip_permit_used
                && self.state(sid).control == Control::Controlled(seat)
                && highest_output == Some(sid)
            {
                push(
                    vec![Order::StripPermit { state: sid }],
                    Cat::StripPermit,
                    self.base_weight(seat, Cat::StripPermit),
                    gap_for(Cat::StripPermit, Some("Strip Permit")),
                    1.0,
                    1.0,
                    format!("issue a Strip Permit in {}", self.tables.state(sid).name),
                    None,
                );
            }
            let base = self.base_weight(seat, Cat::RaiseIndustry);
            push(vec![Order::RaiseIndustry { state: sid }], Cat::RaiseIndustry, base, gap_for(Cat::RaiseIndustry, Some("Industry Level")), 1.0, 1.0, format!("raise Industry Level in {}", self.tables.state(sid).name), None);
            // Ticket #46: no Ship is built at a Launch Site; Shipyards on stations and Colonies build them.
            if self.state(sid).control == Control::Controlled(seat) {
                let threat = if self.enemy_army_near(seat, Place::State(sid)) { m.threat } else { 1.0 };
                let armies = self.armies.iter().filter(|a| !a.standing && self.army_seat(a) == Some(seat)).count();
                if armies < 2 {
                    push(vec![Order::BuildArmy { place: Place::State(sid) }], Cat::ArmyOrBarracks, self.base_weight(seat, Cat::ArmyOrBarracks), 1.0, threat, 1.0, format!("build Army in {}", self.tables.state(sid).name), None);
                }
            }
        }

        // --- Colony builds
        for cid in self.directed_colonies(seat) {
            let col = self.colony(cid).unwrap().clone();
            let threat = if self.enemy_present_or_inbound(seat, col.body) || self.enemy_army_near(seat, Place::Colony(cid)) { m.threat } else { 1.0 };
            for mk in ModuleKind::BUILDABLE {
                // Ticket #46: a station holds only a Shipyard and Habitats, and a Habitat over Earth
                // houses nobody who counts as off Earth, so the AI builds none there.
                if col.in_orbit && (mk != ModuleKind::Shipyard && (mk != ModuleKind::Habitat || col.body == BodyId::Earth)) {
                    continue;
                }
                let (cat, mut base) = match mk {
                    ModuleKind::Mine | ModuleKind::Generator | ModuleKind::Refinery | ModuleKind::TradePost => (Cat::Producer, self.base_weight(seat, Cat::Producer)),
                    ModuleKind::Relay => (Cat::BuildInfluence, self.base_weight(seat, Cat::BuildInfluence)),
                    // Ticket #51: the Archive is never an ordinary Module build; it has its own order.
                    ModuleKind::Archive => continue,
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
                push(vec![Order::BuildModule { colony: cid, kind: mk }], cat, base, gap_for(cat, Some(mk.name())), t, 1.0, format!("build {} at {}", mk.name(), self.place_name(Place::Colony(cid))), None);
            }
            if col.modules.iter().any(|m| m.kind == ModuleKind::Barracks) && !self.armies.iter().any(|a| a.home == ArmyHome::Colony(cid)) {
                push(vec![Order::BuildArmy { place: Place::Colony(cid) }], Cat::ArmyOrBarracks, self.base_weight(seat, Cat::ArmyOrBarracks), 1.0, threat, 1.0, format!("build Army at {}", self.place_name(Place::Colony(cid))), None);
            }
            if col.modules.iter().any(|m| m.kind == ModuleKind::Shipyard) {
                for uk in UnitKind::SHIPS {
                    let cat = if uk == UnitKind::ColonyShip { Cat::ColonyShip } else { Cat::Warship };
                    // Ticket #43: a Carrier is built over Earth, where Armies board, and only when one wants carrying.
                    if uk == UnitKind::Carrier && (col.body != BodyId::Earth || !self.wants_carrier(seat)) {
                        continue;
                    }
                    push(vec![Order::BuildShip { site: Place::Colony(cid), kind: uk }], cat, self.base_weight(seat, cat), gap_for(cat, None), if cat == Cat::Warship { threat } else { 1.0 }, 1.0, format!("build {} at {}", uk.name(), self.place_name(Place::Colony(cid))), None);
                }
            }
        }

        // --- Influence, in units of 5 (spec 16.1), on the target rule of 16.4.
        let step = th.influence_step;
        let mut targets: Vec<(Place, f64)> = Vec::new();
        let my_states = self.controlled_states(seat);
        for sid in StateId::ALL {
            let st = self.state(sid);
            if st.control.controller() == Some(seat) {
                continue;
            }
            let card = self.tables.state(sid);
            let near = card.neighbours.iter().any(|n| my_states.contains(n));
            // Ticket #34: the state's Influence value plus its Industry Level, closest first.
            let value = (self.state_influence_value(sid) + st.industry_level as i64) as f64 + if near { 2.0 } else { 0.0 };
            let neutral_bonus = if st.control == Control::Neutral { 1.0 } else { 0.6 };
            targets.push((Place::State(sid), value * neutral_bonus));
        }
        // Ticket #50: any rival's Colony, the fewest Colonists first.
        for c in &self.colonies {
            if c.control.controller().map(|o| o != seat).unwrap_or(false) {
                targets.push((Place::Colony(c.id), 3.0 - (c.colonists as f64).min(2.0)));
            }
        }
        targets.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        for (rank, (target, _)) in targets.iter().enumerate() {
            let have = self.seat(seat).influence.get(target).copied().unwrap_or(0);
            let threshold = self.influence_threshold_for(seat, *target);
            // Ticket #33: a controlled place needs a standing above the controller's as well, by the
            // challenge margin (ticket #41).
            let needed = match self.place_control(*target).controller() {
                Some(c) => threshold.max(self.seat(c).influence.get(target).copied().unwrap_or(0) + margin),
                None => threshold,
            };
            let base = self.base_weight(seat, Cat::Influence) * (1.0 - 0.15 * rank as f64).max(0.3);
            let opp = if needed - have <= step { m.opportunity } else { 1.0 };
            let bought = if self.tables.ducats.per_influence > 0 { self.seat(seat).stockpile.ducats / self.tables.ducats.per_influence } else { 0 };
            let copies = ((allotment + bought) / step).max(0);
            for _ in 0..copies {
                push(vec![Order::Influence { target: *target, amount: step }], Cat::Influence, base, 1.0, 1.0, opp, format!("spend {} Influence on {}", step, self.place_name(*target)), None);
            }
        }
        // Buy Influence with Ducats (ticket #35), in units of the step, weighted like Influence itself.
        let ducats = self.seat(seat).stockpile.ducats;
        let per = self.tables.ducats.per_influence;
        let buys = if per > 0 { ducats / (per * step) } else { 0 };
        for _ in 0..buys {
            push(vec![Order::BuyInfluence { amount: step }], Cat::Influence, self.base_weight(seat, Cat::Influence) * 0.9, 1.0, 1.0, 1.0, format!("buy {} Influence for {} Ducats", step, per * step), None);
        }
        // Ticket #42: the trading window. While Materials are the scarcest resource (or the bootstrap
        // need), Ducats buy them in lots of 10 at a producer's weight; the AI does not sell.
        let per_materials = self.tables.ducats.per_materials;
        if (scarce == Resource::Materials || needs.contains(&Resource::Materials)) && per_materials > 0 {
            let lots = self.seat(seat).stockpile.ducats / (per_materials * 10);
            for _ in 0..lots.min(4) {
                push(vec![Order::Buy { resource: Resource::Materials, amount: 10 }], Cat::Producer, self.base_weight(seat, Cat::Producer) * 1.5, 1.0, 1.0, 1.0, format!("buy 10 Materials for {} Ducats", per_materials * 10), None);
            }
        }
        // Hold own places where a rival's standing approaches yours (ticket #33: spending raises your standing).
        let mut owned: Vec<Place> = self.controlled_states(seat).into_iter().map(Place::State).collect();
        owned.extend(self.colonies.iter().filter(|c| c.control.controller() == Some(seat)).map(|c| Place::Colony(c.id)));
        for place in owned {
            let rival = self.rival_standing(seat, place);
            let mine = self.seat(seat).influence.get(&place).copied().unwrap_or(0);
            if rival > 0 && rival + 2 * step >= mine {
                let opp = if rival + step >= mine { m.opportunity } else { 1.0 };
                push(vec![Order::Influence { target: place, amount: step }], Cat::Influence, self.base_weight(seat, Cat::Influence), 1.0, m.threat, opp, format!("hold {} with {} Influence", self.place_name(place), step), None);
            }
        }

        // --- Stations (ticket #46): one over each Body where the seat has a producing Colony and none yet.
        // Ticket #51: over Earth the foothold is a Nation State with a working Launch Site, so a
        // Faction that starts with no station (the Arkwrights) can build its first; while the seat
        // has no Shipyard anywhere, that first station is the thing that unlocks every Ship, so it
        // takes the opportunity multiplier.
        let has_shipyard = self.colonies.iter().any(|c| c.control.director() == Some(seat) && c.modules.iter().any(|m| m.kind == ModuleKind::Shipyard));
        for body in BodyId::ALL {
            let has_station = self.colonies.iter().any(|c| c.in_orbit && c.body == body && c.control.director() == Some(seat));
            let foothold = match body {
                BodyId::Earth => self.directed_states(seat).iter().any(|s| self.state(*s).facilities.iter().any(|f| f.kind == FacilityKind::LaunchSite && f.working())),
                _ => self.colonies.iter().any(|c| !c.in_orbit && c.body == body && c.control.director() == Some(seat) && c.modules.iter().any(|m| matches!(m.kind, ModuleKind::Mine | ModuleKind::Generator | ModuleKind::Refinery))),
            };
            if has_station || !foothold {
                continue;
            }
            if let Some(slot) = self.free_orbital_slots(body).first() {
                let opp = if has_shipyard { 1.0 } else { m.opportunity };
                push(vec![Order::BuildStation { body, slot: *slot }], Cat::LaunchSiteOrShipyard, self.base_weight(seat, Cat::LaunchSiteOrShipyard), 1.0, 1.0, opp, format!("build {} over {}", self.station_name(body, *slot), self.tables.body(body).name), None);
            }
        }

        // --- The Archive (ticket #51). The Archivist AI funds it whenever the next stage still
        // wants Research, from turn one if it likes, and otherwise contributes to the shared Tech;
        // it raises the Archive at the first Colony off Earth it took, one stage at a time.
        if kind == FactionKind::Archivists {
            let home = self.archive_colony(seat).or_else(|| {
                self.colonies
                    .iter()
                    .filter(|c| c.control.director() == Some(seat) && self.may_hold_archive(c))
                    .min_by_key(|c| (c.founded_turn, c.id.0))
                    .map(|c| c.id)
            });
            let per = self.tables.archive.research_per_stage;
            let fund = self.seat(seat).archive_fund;
            let left = self.archive_fund_cap(seat);
            if left > 0 && fund < left && self.seat(seat).research_last_turn > 0 {
                let opp = if fund + self.seat(seat).research_last_turn >= per { m.opportunity } else { 1.0 };
                push(vec![Order::FundArchive], Cat::FundArchive, self.base_weight(seat, Cat::FundArchive), gap_for(Cat::FundArchive, None), 1.0, opp, format!("fund the Archive with this turn's {} Research", self.seat(seat).research_last_turn), None);
            }
            if let Some(cid) = home
                && fund >= per
            {
                let next = self.archive_stages_committed(seat) + 1;
                push(vec![Order::BuildArchiveStage { colony: cid }], Cat::ArchiveStage, self.base_weight(seat, Cat::ArchiveStage), gap_for(Cat::ArchiveStage, None), 1.0, m.opportunity, format!("raise stage {} of the Archive at {}", next, self.place_name(Place::Colony(cid))), None);
            }
        }

        // --- Ticket #52: Relief where Unrest has taken hold, and Resettle into a calm state of
        // the seat's own. Relief is one point per 10 Ducats the seat can spare, from Unrest 6, at
        // the opportunity multiplier from 9, where one more turn would throw the seat off.
        let u = self.tables.unrest.clone();
        let ducats = self.seat(seat).stockpile.ducats;
        for sid in self.directed_states(seat) {
            let n = self.state(sid).unrest;
            if n < 6.0 {
                continue;
            }
            let points = if u.relief_ducats > 0 { (ducats / u.relief_ducats).min(n.ceil() as i64) } else { 0 };
            let opp = if n >= 9.0 { m.opportunity } else { 1.0 };
            for _ in 0..points {
                push(
                    vec![Order::Relief { state: sid }],
                    Cat::Relief,
                    self.base_weight(seat, Cat::Relief),
                    1.0,
                    1.0,
                    opp,
                    format!("pay Relief in {} (Unrest {})", self.tables.state(sid).name, Game::unrest_figure(n)),
                    None,
                );
            }
        }
        // Resettle: while the world's population is falling there are flows to steer, and the
        // calmest state the seat directs is the one that can take them.
        if self.population_growth_rate() < 0.0
            && let Some(sid) = self
                .directed_states(seat)
                .into_iter()
                .filter(|s| self.state(*s).unrest < 3.0)
                .min_by(|a, b| self.state(*a).unrest.partial_cmp(&self.state(*b).unrest).unwrap_or(std::cmp::Ordering::Equal))
        {
            push(
                vec![Order::Resettle { state: sid }],
                Cat::Resettle,
                self.base_weight(seat, Cat::Resettle),
                1.0,
                1.0,
                1.0,
                format!("resettle this turn's refugees in {}", self.tables.state(sid).name),
                None,
            );
        }

        // --- Ticket #54: Mothball, Restart and Decommission.
        // A mothball answers an Energy shortfall a turn ahead: the highest-upkeep building that
        // produces nothing is the one to shut. A Custodian behind on Stabilization with Scrubbers
        // standing and a run of nothing mothballs its dirtiest Facility instead, since its own
        // industry is what is keeping the net above the Sink.
        {
            let upkeep_of = |b: &BuildingRef| -> i64 {
                match b {
                    BuildingRef::Facility(sid, i) => self.state(*sid).facilities.get(*i).map(|f| self.tables.facility(f.kind).energy_upkeep).unwrap_or(0),
                    BuildingRef::Module(cid, i) => self
                        .colony(*cid)
                        .and_then(|c| c.modules.get(*i))
                        .map(|md| self.module_yield(seat, *cid, md.kind).upkeep)
                        .unwrap_or(0),
                }
            };
            let mut standing: Vec<(BuildingRef, &'static str, bool, bool, f64)> = Vec::new();
            for sid in self.directed_states(seat) {
                for (i, f) in self.state(sid).facilities.iter().enumerate() {
                    let produces = self.tables.facility(f.kind).produces.is_some();
                    standing.push((BuildingRef::Facility(sid, i), f.kind.name(), f.mothballed, produces, self.facility_yield(seat, sid, f.kind).emissions));
                }
            }
            for cid in self.directed_colonies(seat) {
                let col = self.colony(cid).unwrap();
                for (i, md) in col.modules.iter().enumerate() {
                    if md.kind == ModuleKind::Archive {
                        continue;
                    }
                    let produces = self.tables.module(md.kind).produces.is_some();
                    standing.push((BuildingRef::Module(cid, i), md.kind.name(), md.mothballed, produces, 0.0));
                }
            }
            // The one to mothball for Energy: standing, working, making nothing, dearest to run.
            let idle_cost: Option<&(BuildingRef, &str, bool, bool, f64)> =
                standing.iter().filter(|(b, _, moth, produces, _)| !*moth && !*produces && upkeep_of(b) > 0).max_by_key(|(b, _, _, _, _)| upkeep_of(b));
            if tight && let Some((b, name, _, _, _)) = idle_cost {
                push(
                    vec![Order::Change { building: *b, what: BuildingChange::Mothball }],
                    Cat::Mothball,
                    self.base_weight(seat, Cat::Mothball),
                    1.0,
                    1.0,
                    m.opportunity,
                    format!("mothball the {} at {} (Energy is a turn from short)", name, self.place_name(b.place())),
                    None,
                );
            }
            // The Custodians' other reason to mothball: a Stabilization run that will not start.
            let scrubbers_stand = self.controlled_states(seat).iter().any(|s| self.scrubbers_online(*s) > 0);
            if kind == FactionKind::Custodians && self.seat(seat).stabilization_run == 0 && scrubbers_stand {
                let dirtiest = standing
                    .iter()
                    .filter(|(_, _, moth, _, em)| !*moth && *em > 0.0)
                    .max_by(|a, b| a.4.partial_cmp(&b.4).unwrap_or(std::cmp::Ordering::Equal));
                if let Some((b, name, _, _, em)) = dirtiest {
                    push(
                        vec![Order::Change { building: *b, what: BuildingChange::Mothball }],
                        Cat::Mothball,
                        self.base_weight(seat, Cat::Mothball),
                        gap_for(Cat::Scrubber, None),
                        1.0,
                        1.0,
                        format!("mothball the {} at {} ({:.1} Emissions, and the run is still nothing)", name, self.place_name(b.place()), em),
                        None,
                    );
                }
            }
            // Restart once Energy is back above two turns of upkeep; otherwise scrap it for half.
            let restart_ok = self.seat(seat).stockpile.energy > 2 * self.total_upkeep(seat);
            for (b, name, mothballed, _, _) in standing.iter().filter(|(_, _, moth, _, _)| *moth) {
                if restart_ok {
                    push(
                        vec![Order::Change { building: *b, what: BuildingChange::Restart }],
                        Cat::Restart,
                        self.base_weight(seat, Cat::Restart),
                        1.0,
                        1.0,
                        1.0,
                        format!("restart the {} at {}", name, self.place_name(b.place())),
                        None,
                    );
                } else if let BuildingRef::Facility(sid, i) = b
                    && *mothballed
                    && self.state(*sid).facilities.get(*i).map(|f| self.takes_slot(f.kind)).unwrap_or(false)
                    && self.free_slots(*sid) == 0
                {
                    // Scrapping is for the slot: a mothballed Facility that cannot be restarted yet
                    // and is holding the state's last slot. A Scrubber takes no slot, so it is never
                    // scrapped, and a Colony has no slot limit, so a Module never is either.
                    push(
                        vec![Order::Change { building: *b, what: BuildingChange::Decommission }],
                        Cat::Decommission,
                        self.base_weight(seat, Cat::Decommission),
                        1.0,
                        1.0,
                        1.0,
                        format!("decommission the mothballed {} in {} for its slot", name, self.tables.state(*sid).name),
                        None,
                    );
                }
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
                let capacity = self.colony_ship_capacity(seat);
                if body == BodyId::Earth && s.colonists < capacity {
                    // Load from the most populous directed state.
                    // Ticket #46: only a state with a working Launch Site lifts them.
                    let from = self
                        .directed_states(seat)
                        .into_iter()
                        .filter(|s| self.state(*s).facilities.iter().any(|f| f.kind == FacilityKind::LaunchSite && f.working()))
                        .max_by(|a, b| self.state(*a).population.partial_cmp(&self.state(*b).population).unwrap());
                    if let Some(st) = from {
                        let n = capacity - s.colonists;
                        let opp = if presence_needed <= n { m.opportunity } else { 1.0 };
                        push(vec![Order::Load { ship: s.id, colonists: n, from: LoadSource::State(st), army: None }], Cat::LoadUnload, self.base_weight(seat, Cat::LoadUnload), gap_for(Cat::LoadUnload, None), 1.0, opp, format!("load {} Colonists onto {}", n, ship_name), None);
                    }
                }
                if s.colonists > 0 && body != BodyId::Earth {
                    let free = self.free_slots_on(body);
                    if let Some(slot) = free.first() {
                        let opp = if free.len() == 1 || presence_needed <= s.colonists { m.opportunity } else { 1.0 };
                        push(vec![Order::Unload { ship: s.id, colonists: s.colonists, army: false, into: UnloadTarget::Slot(body, *slot) }], Cat::FoundColony, self.base_weight(seat, Cat::FoundColony), gap_for(Cat::FoundColony, None), 1.0, opp, format!("found a Colony at {} on {}", self.tables.body(body).slots[*slot as usize].name, self.tables.body(body).name), None);
                    }
                }
                // Ticket #44: Antarctica, Earth's slots. A foothold, not Presence: half weight and no gap,
                // so it is taken when the Ship cannot go anywhere better.
                if s.colonists > 0 && body == BodyId::Earth {
                    if let Some(slot) = self.free_slots_on(BodyId::Earth).first() {
                        push(vec![Order::Unload { ship: s.id, colonists: s.colonists, army: false, into: UnloadTarget::Slot(body, *slot) }], Cat::FoundColony, self.base_weight(seat, Cat::FoundColony) * 0.5, 1.0, 1.0, 1.0, format!("found a Colony at {}", self.tables.body(BodyId::Earth).slots[*slot as usize].name), None);
                    }
                    // Ticket #46: Colonists on a station over Earth are still on Earth for Presence; never park them there.
                    for c in self.colonies.iter().filter(|c| c.body == body && c.body != BodyId::Earth && c.control.director() == Some(seat)) {
                        let room = self.habitat_room(c).saturating_sub(c.colonists);
                        if room > 0 {
                            let n = room.min(s.colonists);
                            let opp = if presence_needed <= n { m.opportunity } else { 1.0 };
                            push(vec![Order::Unload { ship: s.id, colonists: n, army: false, into: UnloadTarget::Colony(c.id) }], Cat::LoadUnload, self.base_weight(seat, Cat::LoadUnload), gap_for(Cat::LoadUnload, None), 1.0, opp, format!("disembark {} Colonists into {}", n, self.place_name(Place::Colony(c.id))), None);
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
                        push(vec![Order::Transit { ship: s.id, to: d }], Cat::Transit, self.base_weight(seat, Cat::Transit), gap_for(Cat::Transit, None), 1.0, 1.0, format!("send {} to {}", ship_name, self.tables.body(d).name), None);
                    }
                }
                if s.colonists == 0 && body != BodyId::Earth {
                    push(vec![Order::Transit { ship: s.id, to: BodyId::Earth }], Cat::Transit, self.base_weight(seat, Cat::Transit) * 0.8, gap_for(Cat::Transit, None), 1.0, 1.0, format!("send {} back to Earth", ship_name), None);
                }
            }
            // Ticket #43: an empty Carrier away from Earth goes home for an Army.
            if s.kind == UnitKind::Carrier && s.army.is_none() && body != BodyId::Earth {
                push(vec![Order::Transit { ship: s.id, to: BodyId::Earth }], Cat::Transit, self.base_weight(seat, Cat::Transit) * 0.8, 1.0, 1.0, 1.0, format!("send {} back to Earth", ship_name), None);
            }
            if s.kind.is_warship() || s.army.is_some() {
                // Warships go where the Faction has or wants Colonies, or where a rival is: any
                // rival, nearest by transit turns first (ticket #50).
                let mut dests: Vec<BodyId> = Vec::new();
                for c in &self.colonies {
                    if c.body != body && (c.control.director() == Some(seat) || self.rival_holds(seat, c)) && !dests.contains(&c.body) {
                        dests.push(c.body);
                    }
                }
                dests.sort_by_key(|d| self.transit_cost_for(seat, body, *d).0);
                for d in dests {
                    let threat = if self.enemy_present_or_inbound(seat, d) { m.threat } else { 1.0 };
                    let base = self.base_weight(seat, Cat::Transit) * if kind == FactionKind::Prospectors { 0.9 } else { 0.6 };
                    push(vec![Order::Transit { ship: s.id, to: d }], Cat::Transit, base, 1.0, threat, 1.0, format!("send {} to {}", ship_name, self.tables.body(d).name), None);
                }
                // Load an Army aboard a Carrier at Earth for an attack on a rival Colony (ticket #43).
                if card.carries_army && s.army.is_none() && body == BodyId::Earth && kind == FactionKind::Prospectors {
                    // Ticket #46: the Army lifts from its own state, which needs a working Launch Site.
                    let army = self.armies.iter().find_map(|a| match a.at {
                        ArmyAt::Place(Place::State(st))
                            if !a.standing
                                && self.army_seat(a) == Some(seat)
                                && self.state(st).control.director() == Some(seat)
                                && self.state(st).facilities.iter().any(|f| f.kind == FacilityKind::LaunchSite && f.working()) =>
                        {
                            Some((a.id, st))
                        }
                        _ => None,
                    });
                    let enemy_colony = self.colonies.iter().any(|c| self.rival_holds(seat, c));
                    if let (Some((aid, st)), true) = (army, enemy_colony) {
                        push(vec![Order::Load { ship: s.id, colonists: 0, from: LoadSource::State(st), army: Some(aid) }], Cat::LoadUnload, self.base_weight(seat, Cat::LoadUnload) * 0.8, 1.0, 1.0, 1.0, format!("load an Army onto {}", ship_name), None);
                    }
                }
                if let Some(aid) = s.army.filter(|_| body != BodyId::Earth) {
                    {
                        for c in self.colonies.iter().filter(|c| c.body == body) {
                            let enemy = self.rival_holds(seat, c);
                            let mine = c.control.director() == Some(seat);
                            if enemy || mine {
                                let defence: i64 =
                                    self.defenders_at(Place::Colony(c.id), seat).iter().filter_map(|id| self.army(*id)).map(|a| self.army_strength(a)).sum();
                                let odds = first_round_odds(self.army_strength(self.army(aid).unwrap()), defence);
                                if enemy && odds < th.attack_odds {
                                    continue;
                                }
                                let base = self.base_weight(seat, Cat::LoadUnload);
                                push(vec![Order::Unload { ship: s.id, colonists: 0, army: true, into: UnloadTarget::Colony(c.id) }], Cat::LoadUnload, base, 1.0, if mine { m.threat } else { 1.0 }, 1.0, format!("land an Army at {}", self.place_name(Place::Colony(c.id))), None);
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
            let enemy_str = self.enemy_ship_strength(seat, body);
            let enemy_here = self.ships.iter().any(|s| s.seat != seat && s.at == ShipAt::Body(body));
            let inbound_target = self.ships.iter().any(|s| s.seat != seat && matches!(s.kind, UnitKind::ColonyShip | UnitKind::Carrier) && matches!(s.at, ShipAt::Transit { to, .. } if to == body));
            let threat = if self.enemy_present_or_inbound(seat, body) { m.threat } else { 1.0 };
            let total_hp: u32 = stack.iter().map(|s| self.tables.unit(s.kind).hit_points).sum();
            let total_dmg: u32 = stack.iter().map(|s| s.damage).sum();
            let warships = stack.iter().any(|s| s.kind.is_warship());
            push(vec![Order::ShipStance { body, stance: Stance::Hold }], Cat::StanceHold, self.base_weight(seat, Cat::StanceHold), 1.0, threat, 1.0, format!("Hold at {}", self.tables.body(body).name), Some(key.clone()));
            if warships && enemy_here {
                // Ticket #50: the odds are against the sum of every other seat's strength present.
                let odds = first_round_odds(my_str, enemy_str);
                let my_colony_here = self.colonies.iter().any(|c| c.body == body && c.control.controller() == Some(seat));
                let held_against_me = self.orbital_control(body).map(|o| o != seat).unwrap_or(false);
                let allowed = match kind {
                    FactionKind::Prospectors => odds >= th.attack_odds,
                    // The Custodians attack only to break a blockade at a Body where they have a
                    // Colony; the Arkwrights and the Archivists fight on the same terms (ticket #50).
                    _ => odds >= th.attack_odds && my_colony_here && held_against_me,
                };
                if allowed {
                    push(vec![Order::ShipStance { body, stance: Stance::Attack }], Cat::StanceAttack, self.base_weight(seat, Cat::StanceAttack), 1.0, 1.0, 1.0, format!("Attack at {} (odds {:.0}%)", self.tables.body(body).name, odds * 100.0), Some(key.clone()));
                }
            }
            if warships && self.orbital_control(body) == Some(seat) && inbound_target {
                push(vec![Order::ShipStance { body, stance: Stance::Intercept }], Cat::StanceIntercept, self.base_weight(seat, Cat::StanceIntercept), 1.0, threat, 1.0, format!("Intercept at {}", self.tables.body(body).name), Some(key.clone()));
            }
            if total_hp > 0 && (total_dmg as f64) / (total_hp as f64) >= th.evade_damage_fraction {
                push(vec![Order::ShipStance { body, stance: Stance::Evade }], Cat::StanceEvade, self.base_weight(seat, Cat::StanceEvade) * 10.0, 1.0, 1.0, 1.0, format!("Evade at {}", self.tables.body(body).name), Some(key.clone()));
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
            push(vec![Order::ArmyStance { place, stance: Stance::Hold }], Cat::StanceHold, self.base_weight(seat, Cat::StanceHold), 1.0, threat, 1.0, format!("Hold at {}", self.place_name(place)), Some(key.clone()));
            let my_str = self.army_stack_strength(seat, place);
            let total_hp: u32 = mine.iter().map(|_| self.tables.unit(UnitKind::Army).hit_points).sum();
            let total_dmg: u32 = mine.iter().filter_map(|id| self.army(*id)).map(|a| a.damage).sum();
            if total_hp > 0 && (total_dmg as f64) / (total_hp as f64) >= th.evade_damage_fraction {
                push(vec![Order::ArmyStance { place, stance: Stance::Evade }], Cat::StanceEvade, self.base_weight(seat, Cat::StanceEvade) * 10.0, 1.0, 1.0, 1.0, format!("Evade at {}", self.place_name(place)), Some(key.clone()));
            }
            // Attack where this seat does not direct the place and defenders stand.
            if self.place_director(place) != Some(seat) {
                let def: i64 = self.defenders_at(place, seat).iter().filter_map(|id| self.army(*id)).map(|a| self.army_strength(a)).sum();
                let odds = first_round_odds(my_str, def);
                let lost_place = self.place_control(place).controller() == Some(seat) || matches!(self.place_control(place), Control::Occupied { previous: Some(p), .. } if p == seat);
                let allowed = match kind {
                    FactionKind::Prospectors => odds >= th.attack_odds || def == 0,
                    // Ticket #50: the Arkwrights and the Archivists fight on the Custodians' terms.
                    _ => lost_place && (odds >= th.attack_odds || def == 0),
                };
                if allowed {
                    push(vec![Order::ArmyStance { place, stance: Stance::Attack }], Cat::StanceAttack, self.base_weight(seat, Cat::StanceAttack) * 1.5, 1.0, 1.0, 1.0, format!("Attack at {} (odds {:.0}%)", self.place_name(place), odds * 100.0), Some(key.clone()));
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
                        if ctrl == Control::Controlled(seat) {
                            continue;
                        }
                        let def: i64 = self.defenders_at(Place::State(*n), seat).iter().filter_map(|id| self.army(*id)).map(|a| self.army_strength(a)).sum();
                        let odds = first_round_odds(self.army_strength(a), def);
                        let allowed = match kind {
                            FactionKind::Prospectors => odds >= th.attack_odds,
                            _ => matches!(ctrl, Control::Occupied { previous: Some(p), .. } if p == seat) && odds >= th.attack_odds,
                        };
                        if allowed {
                            let value = (self.tables.state(*n).industry_level + self.tables.state(*n).size) as f64 / 7.0;
                            push(vec![Order::MoveArmy { army: *aid, to: *n }], Cat::StanceAttack, self.base_weight(seat, Cat::StanceAttack) * (1.0 + value), 1.0, 1.0, 1.0, format!("march on {} (odds {:.0}%)", self.tables.state(*n).name, odds * 100.0), None);
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
        // Ticket #54: the same for Ducats, held for a higher-scored Ducat action (a Leapfrog) that
        // three turns of Ducat income would bring within reach.
        let ducat_income = self.seat(seat).income_last_turn.ducats;
        let mut ducat_reserve: Option<String> = None;
        for c in cands.iter().filter(|c| c.stack.is_none()) {
            let mut ok = true;
            let mut trial = chosen.clone();
            let ducats_cost: i64 = c.orders.iter().map(|o| self.order_cost(seat, o).ducats).sum();
            if ducats_cost > 0 {
                if let Some(note) = &ducat_reserve {
                    lines.push(format!("  save  {:6.1}  {} (holding Ducats for {})", c.score(), c.note, note));
                    continue;
                }
                let (left, _) = self.remaining(seat, &chosen);
                if ducats_cost > left.ducats
                    && ducats_cost <= left.ducats + 3 * ducat_income
                    && c.orders.iter().all(|o| self.check_order_legality(seat, &chosen, o).is_ok())
                {
                    ducat_reserve = Some(c.note.clone());
                    lines.push(format!("  wait  {:6.1}  {} (affordable within three turns)", c.score(), c.note));
                    continue;
                }
            }
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
        // Ticket #50: three AI seats order each turn, so the Report keeps every seat's list under a
        // heading naming the Faction rather than the last seat's alone.
        self.report.ai_lines.push(AiReport { seat, lines });
        chosen
    }
}

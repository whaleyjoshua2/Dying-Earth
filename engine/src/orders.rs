//! Orders: what a seat may do in the Orders phase, what each costs, and what is legal (spec 6, 7.3, 7.4, 8, 9).

use crate::ids::*;
use crate::state::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitRef {
    Ship(ShipId),
    Army(ArmyId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadSource {
    State(StateId),
    Colony(ColonyId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnloadTarget {
    /// Found a Colony into a free slot (spec 9.4).
    Slot(BodyId, u32),
    /// Disembark into an existing Colony on this Body (own: into Habitats; enemy: an Army lands to attack).
    Colony(ColonyId),
}

/// Ticket #54 (version 0.05): one standing building, by its place and its position in that place's
/// list. Orders are given and resolved inside one turn, so the position cannot move under them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildingRef {
    Facility(StateId, usize),
    Module(ColonyId, usize),
}

impl BuildingRef {
    pub fn place(self) -> Place {
        match self {
            BuildingRef::Facility(s, _) => Place::State(s),
            BuildingRef::Module(c, _) => Place::Colony(c),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Order {
    BuildFacility { state: StateId, kind: FacilityKind },
    RaiseIndustry { state: StateId },
    BuildModule { colony: ColonyId, kind: ModuleKind },
    BuildShip { site: Place, kind: UnitKind },
    BuildArmy { place: Place },
    Repair { unit: UnitRef, points: u32 },
    Transit { ship: ShipId, to: BodyId },
    ShipStance { body: BodyId, stance: Stance },
    ArmyStance { place: Place, stance: Stance },
    MoveArmy { army: ArmyId, to: StateId },
    Load { ship: ShipId, colonists: u32, from: LoadSource, army: Option<ArmyId> },
    Unload { ship: ShipId, colonists: u32, army: bool, into: UnloadTarget },
    Influence { target: Target, amount: i64 },
    /// Version 0.03 (ticket #35): Ducats buy Influence for this turn's Allotment, and pay for
    /// repairs in place of Materials. Ticket #54 retired Restoration and both its orders.
    BuyInfluence { amount: i64 },
    RepairWithDucats { unit: UnitRef, points: u32 },
    /// Version 0.04 (ticket #42): the trading window. Buy Materials, Fuel or Energy for Ducats;
    /// sell Materials or Fuel for half the buying price; buy a building outright for Ducats at
    /// twice its Materials cost. What is bought is spendable in the same turn's orders.
    Buy { resource: Resource, amount: i64 },
    Sell { resource: Resource, amount: i64 },
    BuildFacilityWithDucats { state: StateId, kind: FacilityKind },
    BuildModuleWithDucats { colony: ColonyId, kind: ModuleKind },
    /// Version 0.04 (ticket #46): a Space Station in an orbital slot, built for Materials from a
    /// Nation State with a Launch Site (over Earth) or a Colony of the seat's (elsewhere).
    BuildStation { body: BodyId, slot: u32 },
    /// Version 0.05 (ticket #51): the next stage of the Archive, at a Colony off Earth. Paid in
    /// Materials from the Stockpile and Research already banked in the Archive fund.
    BuildArchiveStage { colony: ColonyId },
    /// Version 0.05 (ticket #51): this turn's Research from the Archivists' Labs goes into the
    /// Archive fund instead of the shared Tech, and counts nothing toward the Research Lead.
    FundArchive,
    /// Version 0.05 (ticket #52): Relief. Ducats spent on a Nation State you direct, lowering its
    /// Unrest by one. Any number of times a turn, cancellable like any order.
    Relief { state: StateId },
    /// Version 0.05 (ticket #52): Resettle. Once a turn per Faction: this turn every refugee flow
    /// leaving a state you direct goes entirely to the chosen state, and you gain Standing there.
    Resettle { state: StateId },
    /// Version 0.05 (ticket #54): Mothball, Restart or Decommission one standing Facility in a
    /// Nation State you direct, or one Module in a Colony you direct.
    Change { building: BuildingRef, what: BuildingChange },
    /// Version 0.05 (ticket #54): Leapfrog. The Custodians only, on a Nation State they control:
    /// 50 Ducats lowers its people's Emissions coefficient by one Industry Level's worth, for good.
    Leapfrog { state: StateId },
    /// Version 0.05 (ticket #54): the Strip Permit. The Prospectors only, free, once per Nation
    /// State ever: three turns of doubled Facility output, then a permanent price in Baseline
    /// Emissions and Unrest.
    StripPermit { state: StateId },
}

impl Order {
    /// The Nation State a build order takes a slot in, whichever way it is paid.
    pub fn build_state(&self) -> Option<StateId> {
        match self {
            Order::BuildFacility { state, .. } | Order::BuildFacilityWithDucats { state, .. } => Some(*state),
            _ => None,
        }
    }
    /// The Facility a build order raises, whichever way it is paid (ticket #54).
    pub fn build_facility(&self) -> Option<FacilityKind> {
        match self {
            Order::BuildFacility { kind, .. } | Order::BuildFacilityWithDucats { kind, .. } => Some(*kind),
            _ => None,
        }
    }
    /// The Colony and Module a build order queues, whichever way it is paid.
    pub fn build_module(&self) -> Option<(ColonyId, ModuleKind)> {
        match self {
            Order::BuildModule { colony, kind } | Order::BuildModuleWithDucats { colony, kind } => Some((*colony, *kind)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Cost {
    pub materials: i64,
    pub fuel: i64,
    pub energy: i64,
    pub influence: i64,
    pub ducats: i64,
}

impl Cost {
    pub fn add(&mut self, other: Cost) {
        self.materials += other.materials;
        self.fuel += other.fuel;
        self.energy += other.energy;
        self.influence += other.influence;
        self.ducats += other.ducats;
    }
    pub fn text(&self) -> String {
        let mut parts = Vec::new();
        if self.materials > 0 {
            parts.push(format!("{} Materials", self.materials));
        }
        if self.fuel > 0 {
            parts.push(format!("{} Fuel", self.fuel));
        }
        if self.energy > 0 {
            parts.push(format!("{} Energy", self.energy));
        }
        if self.influence > 0 {
            parts.push(format!("{} Influence", self.influence));
        }
        if self.ducats > 0 {
            parts.push(format!("{} Ducats", self.ducats));
        }
        if parts.is_empty() { "free".to_string() } else { parts.join(", ") }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderError(pub String);

impl std::fmt::Display for OrderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

fn fail<T>(msg: impl Into<String>) -> Result<T, OrderError> {
    Err(OrderError(msg.into()))
}

/// Things committed at End Turn that act later in Resolution.
#[derive(Debug, Clone, Default)]
pub struct Pending {
    pub repairs: Vec<(Seat, UnitRef, u32)>,
    pub cargo: Vec<(Seat, Order)>,
    /// Ticket #46: stations ordered this turn.
    pub stations: Vec<(Seat, BodyId, u32)>,
    /// Ticket #52: Relief orders paid this turn, one entry per point.
    pub relief: Vec<(Seat, StateId)>,
    /// Ticket #52: Resettle orders paid this turn, one per Faction at most.
    pub resettle: Vec<(Seat, StateId)>,
    pub influence: Vec<(Seat, Target, i64)>,
    /// Attack orders in the order given, for battle ordering (spec 10.1).
    pub attack_sequence: u32,
}

impl Game {
    /// The cost of one order for a seat, before legality.
    pub fn order_cost(&self, seat: Seat, order: &Order) -> Cost {
        let t = &self.tables;
        match order {
            Order::BuildFacility { kind, .. } => Cost { materials: t.facility(*kind).materials, ..Default::default() },
            Order::RaiseIndustry { .. } => Cost { materials: self.industry_cost(seat), ..Default::default() },
            // Ticket #51: a Faction's card may make its Modules and its Colony Ships cost less.
            Order::BuildModule { kind, .. } => Cost { materials: self.module_materials(seat, *kind), ..Default::default() },
            Order::BuildShip { kind, .. } => Cost { materials: self.ship_materials(seat, *kind), ..Default::default() },
            Order::BuildArmy { .. } => Cost { materials: t.unit(UnitKind::Army).materials, ..Default::default() },
            Order::Repair { points, .. } => {
                Cost { materials: t.repair.materials_per_point * *points as i64, ..Default::default() }
            }
            Order::Transit { ship, to } => {
                let from = match self.ship(*ship).map(|s| s.at) {
                    Some(ShipAt::Body(b)) => b,
                    _ => BodyId::Earth,
                };
                Cost { fuel: self.transit_cost_for(seat, from, *to).1, ..Default::default() }
            }
            Order::Influence { amount, .. } => Cost { influence: *amount, ..Default::default() },
            // Ticket #54: a Mothball and a Strip Permit are free; a Restart costs Materials and a
            // Leapfrog Ducats; a Decommission pays Materials back, which arrive at its Resolution.
            Order::Change { what: BuildingChange::Restart, .. } => Cost { materials: t.mothball.restart_materials, ..Default::default() },
            Order::Leapfrog { .. } => Cost { ducats: t.ducats.per_leapfrog, ..Default::default() },
            Order::BuyInfluence { amount } => Cost { ducats: t.ducats.per_influence * *amount, ..Default::default() },
            // A purchase is a negative cost in the resource bought, so `remaining` and `commit_orders`
            // add it without a special case; a sale is the mirror, with a negative Ducat cost.
            Order::Buy { resource, amount } => {
                let ducats = self.trade_price(*resource).unwrap_or(0) * *amount;
                match resource {
                    Resource::Materials => Cost { materials: -*amount, ducats, ..Default::default() },
                    Resource::Fuel => Cost { fuel: -*amount, ducats, ..Default::default() },
                    Resource::Energy => Cost { energy: -*amount, ducats, ..Default::default() },
                    _ => Cost::default(),
                }
            }
            Order::Sell { resource, amount } => {
                let ducats = -self.sale_price(*resource, *amount);
                match resource {
                    Resource::Materials => Cost { materials: *amount, ducats, ..Default::default() },
                    Resource::Fuel => Cost { fuel: *amount, ducats, ..Default::default() },
                    _ => Cost::default(),
                }
            }
            Order::BuildFacilityWithDucats { kind, .. } => Cost { ducats: t.facility(*kind).materials * t.ducats.per_building_material, ..Default::default() },
            Order::BuildStation { .. } => Cost { materials: self.station_materials(seat), ..Default::default() },
            Order::BuildModuleWithDucats { kind, .. } => Cost { ducats: self.module_materials(seat, *kind) * t.ducats.per_building_material, ..Default::default() },
            // Ticket #51: a stage of the Archive costs Materials here and Research from the fund,
            // which is not part of the Stockpile and so is checked in the legality rules below.
            Order::BuildArchiveStage { .. } => Cost { materials: t.module(ModuleKind::Archive).materials, ..Default::default() },
            // Ticket #52: Relief and Resettle are paid in Ducats.
            Order::Relief { .. } => Cost { ducats: t.unrest.relief_ducats, ..Default::default() },
            Order::Resettle { .. } => Cost { ducats: t.unrest.resettle_ducats, ..Default::default() },
            Order::RepairWithDucats { points, .. } => Cost { ducats: t.ducats.per_repair_point * *points as i64, ..Default::default() },
            _ => Cost::default(),
        }
    }

    /// What the seat still has after its pending orders. Bought Influence counts toward the Allotment.
    /// Ticket #42: Ducats per unit in the trading window, or None for what it does not sell.
    pub fn trade_price(&self, resource: Resource) -> Option<i64> {
        let d = &self.tables.ducats;
        match resource {
            Resource::Materials => Some(d.per_materials),
            Resource::Fuel => Some(d.per_fuel),
            Resource::Energy => Some(d.per_energy),
            _ => None,
        }
    }

    /// Ticket #42: what the window pays for a lot, or None for what it does not buy back.
    pub fn sale_price(&self, resource: Resource, amount: i64) -> i64 {
        if !matches!(resource, Resource::Materials | Resource::Fuel) {
            return 0;
        }
        let d = &self.tables.ducats;
        let per = self.trade_price(resource).unwrap_or(0);
        if d.sell_divisor <= 0 {
            return 0;
        }
        per * amount / d.sell_divisor
    }

    pub fn remaining(&self, seat: Seat, pending: &[Order]) -> (Stockpile, i64) {
        let mut cost = Cost::default();
        let mut bought = 0;
        for o in pending {
            cost.add(self.order_cost(seat, o));
            if let Order::BuyInfluence { amount } = o {
                bought += *amount;
            }
        }
        let s = self.seat(seat).stockpile;
        (
            Stockpile { materials: s.materials - cost.materials, fuel: s.fuel - cost.fuel, energy: s.energy - cost.energy, ducats: s.ducats - cost.ducats },
            self.seat(seat).allotment + bought - cost.influence,
        )
    }

    /// Check an order against the board and the orders already pending; return its cost if legal.
    pub fn check_order(&self, seat: Seat, pending: &[Order], order: &Order) -> Result<Cost, OrderError> {
        self.check_order_inner(seat, pending, order, true)
    }

    /// The same check without the affordability part: is the order legal at all?
    pub fn check_order_legality(&self, seat: Seat, pending: &[Order], order: &Order) -> Result<Cost, OrderError> {
        self.check_order_inner(seat, pending, order, false)
    }

    fn check_order_inner(&self, seat: Seat, pending: &[Order], order: &Order, enforce_cost: bool) -> Result<Cost, OrderError> {
        let cost = self.order_cost(seat, order);
        let (left, influence_left) = self.remaining(seat, pending);
        if enforce_cost {
            if cost.materials > left.materials {
                return fail(format!("needs {} Materials, {} left", cost.materials, left.materials));
            }
            if cost.fuel > left.fuel {
                return fail(format!("needs {} Fuel, {} left", cost.fuel, left.fuel));
            }
            if cost.energy > left.energy {
                return fail(format!("needs {} Energy, {} left", cost.energy, left.energy));
            }
            if cost.influence > influence_left {
                return fail(format!("needs {} Influence, {} left", cost.influence, influence_left));
            }
            if cost.ducats > left.ducats {
                return fail(format!("needs {} Ducats, {} left", cost.ducats, left.ducats));
            }
        }
        match order {
            Order::BuyInfluence { amount } => {
                if *amount <= 0 {
                    return fail("buy a positive amount");
                }
                Ok(cost)
            }
            Order::RepairWithDucats { unit, points } => {
                // The same legality as a Materials repair; only the payment differs.
                let materials_form = Order::Repair { unit: *unit, points: *points };
                self.check_order_inner(seat, pending, &materials_form, false).map(|_| cost)
            }
            Order::Buy { resource, amount } => {
                if *amount <= 0 {
                    return fail("buy a positive amount");
                }
                if self.trade_price(*resource).is_none() {
                    return fail("not for sale");
                }
                Ok(cost)
            }
            Order::Sell { resource, amount } => {
                if *amount <= 0 {
                    return fail("sell a positive amount");
                }
                if !matches!(resource, Resource::Materials | Resource::Fuel) {
                    return fail("the window buys only Materials and Fuel");
                }
                // The affordability check above already refused a lot larger than what is left.
                Ok(cost)
            }
            Order::BuildFacilityWithDucats { state, kind } => {
                let materials_form = Order::BuildFacility { state: *state, kind: *kind };
                self.check_order_inner(seat, pending, &materials_form, false).map(|_| cost)
            }
            Order::BuildModuleWithDucats { colony, kind } => {
                let materials_form = Order::BuildModule { colony: *colony, kind: *kind };
                self.check_order_inner(seat, pending, &materials_form, false).map(|_| cost)
            }
            Order::FundArchive => {
                if self.kind(seat) != FactionKind::Archivists {
                    return fail("only the Archivists fund the Archive");
                }
                if pending.iter().any(|o| matches!(o, Order::FundArchive)) {
                    return fail("the Archive is already being funded this turn");
                }
                Ok(cost)
            }
            Order::BuildArchiveStage { colony } => {
                if self.kind(seat) != FactionKind::Archivists {
                    return fail("only the Archivists build the Archive");
                }
                let Some(col) = self.colony(*colony) else { return fail("no such Colony") };
                if col.control.director() != Some(seat) {
                    return fail("you do not direct this Colony");
                }
                if !self.may_hold_archive(col) {
                    return fail("the Archive stands at a Colony off Earth; Antarctica and a station over Earth will not do");
                }
                // At most one Archive per Faction, wherever it stands.
                if let Some(home) = self.archive_colony(seat)
                    && home != *colony
                {
                    return fail(format!("the Archive already stands at {}", self.place_name(Place::Colony(home))));
                }
                let stages = self.tables.archive.stages;
                let committed = self.archive_stages_committed(seat)
                    + pending.iter().filter(|o| matches!(o, Order::BuildArchiveStage { .. })).count() as u32;
                if committed >= stages {
                    return fail("every stage of the Archive is built or on order");
                }
                // One stage at a time: four stages of two turns are eight turns of building.
                let building = self.colonies.iter().flat_map(|c| c.queue.iter()).any(|b| b.seat == seat && b.item == BuildItem::Module(ModuleKind::Archive))
                    || pending.iter().any(|o| matches!(o, Order::BuildArchiveStage { .. }));
                if building {
                    return fail("a stage of the Archive is already building; one stage at a time");
                }
                // The Research must already be banked: a stage ordered this turn cannot be paid out
                // of this turn's funding, which has not happened yet.
                let per = self.tables.archive.research_per_stage;
                let spent = pending.iter().filter(|o| matches!(o, Order::BuildArchiveStage { .. })).count() as i64 * per;
                let banked = self.seat(seat).archive_fund - spent;
                if enforce_cost && banked < per {
                    return fail(format!("stage {} needs {} Research banked in the Archive fund, {} there", committed + 1, per, banked.max(0)));
                }
                Ok(cost)
            }
            Order::BuildStation { body, slot } => {
                if !self.free_orbital_slots(*body).contains(slot) {
                    return fail("that orbital slot is taken, or there is no such slot");
                }
                if pending.iter().any(|o| matches!(o, Order::BuildStation { body: b, slot: s } if b == body && s == slot)) {
                    return fail("a station is already ordered there");
                }
                let foothold = match body {
                    BodyId::Earth => self.directed_states(seat).iter().any(|s| self.state(*s).facilities.iter().any(|f| f.kind == FacilityKind::LaunchSite && f.working())),
                    b => self.colonies.iter().any(|c| !c.in_orbit && c.body == *b && c.control.director() == Some(seat)),
                };
                if !foothold {
                    return fail(if *body == BodyId::Earth { "needs a Nation State of yours with a Launch Site" } else { "needs a Colony of yours on this Body" });
                }
                Ok(cost)
            }
            Order::BuildFacility { state, kind } => {
                if self.state(*state).control.director() != Some(seat) {
                    return fail("you do not direct this Nation State");
                }
                // Ticket #54: a Scrubber takes no build slot, so it neither needs one nor uses one.
                if self.takes_slot(*kind) {
                    let pending_here = pending
                        .iter()
                        .filter(|o| o.build_state() == Some(*state) && o.build_facility().map(|k| self.takes_slot(k)).unwrap_or(false))
                        .count() as u32;
                    if self.free_slots(*state) <= pending_here {
                        return fail("no free build slot");
                    }
                }
                // Ticket #54: the Scrubber, the Custodians' signature Facility.
                if *kind == FacilityKind::Scrubber {
                    if self.kind(seat) != FactionKind::Custodians {
                        return fail("only the Custodians build a Scrubber");
                    }
                    if self.state(*state).control != Control::Controlled(seat) {
                        return fail("a Scrubber needs a Nation State you control");
                    }
                    let ordered = pending
                        .iter()
                        .filter(|o| o.build_state() == Some(*state) && o.build_facility() == Some(FacilityKind::Scrubber))
                        .count() as u32;
                    let cap = self.scrubber_cap(*state);
                    if self.scrubbers_committed(*state) + ordered >= cap {
                        return fail(format!("this Nation State holds its {cap} Scrubbers already"));
                    }
                }
                // Ticket #52: at most one Constabulary per Nation State.
                if *kind == FacilityKind::Constabulary
                    && (self.state(*state).facilities.iter().any(|f| f.kind == FacilityKind::Constabulary)
                        || self.state(*state).queue.iter().any(|b| b.item == BuildItem::Facility(FacilityKind::Constabulary))
                        || pending.iter().any(|o| matches!(o.build_state(), Some(s) if s == *state) && matches!(o, Order::BuildFacility { kind: FacilityKind::Constabulary, .. } | Order::BuildFacilityWithDucats { kind: FacilityKind::Constabulary, .. })))
                {
                    return fail("this Nation State already has a Constabulary");
                }
                Ok(cost)
            }
            // Ticket #52: Relief, on a state you direct, any number of times a turn.
            Order::Relief { state } => {
                if self.state(*state).control.director() != Some(seat) {
                    return fail("Relief is paid in a Nation State you direct");
                }
                Ok(cost)
            }
            // Ticket #52: Resettle, once a turn per Faction, on a state you direct.
            Order::Resettle { state } => {
                if self.state(*state).control.director() != Some(seat) {
                    return fail("Resettle sends the refugees to a Nation State you direct");
                }
                if pending.iter().any(|o| matches!(o, Order::Resettle { .. })) {
                    return fail("one Resettle a turn");
                }
                Ok(cost)
            }
            Order::RaiseIndustry { state } => {
                if self.state(*state).control.director() != Some(seat) {
                    return fail("you do not direct this Nation State");
                }
                if pending.iter().any(|o| matches!(o, Order::RaiseIndustry { state: s } if s == state))
                    || self.state(*state).queue.iter().any(|b| b.item == BuildItem::IndustryLevel)
                {
                    return fail("Industry Level is already being raised here");
                }
                Ok(cost)
            }
            Order::BuildModule { colony, kind } => {
                let Some(col) = self.colony(*colony) else { return fail("no such Colony") };
                if col.control.director() != Some(seat) {
                    return fail("you do not direct this Colony");
                }
                // Ticket #51: the Archive is never placed by an ordinary build order.
                if *kind == ModuleKind::Archive {
                    return fail("the Archive is raised one stage at a time, from its own button");
                }
                // Ticket #46: a station holds only a Shipyard and Habitats.
                if col.in_orbit && !matches!(kind, ModuleKind::Shipyard | ModuleKind::Habitat) {
                    return fail("a station holds only a Shipyard and Habitats");
                }
                if matches!(kind, ModuleKind::Shipyard | ModuleKind::Barracks) {
                    let has = col.modules.iter().any(|m| m.kind == *kind)
                        || col.queue.iter().any(|b| b.item == BuildItem::Module(*kind))
                        || pending.iter().any(|o| o.build_module() == Some((*colony, *kind)));
                    if has {
                        return fail(format!("this Colony already has a {}", kind.name()));
                    }
                }
                Ok(cost)
            }
            Order::BuildShip { site, kind } => {
                if !UnitKind::SHIPS.contains(kind) {
                    return fail("not a Ship");
                }
                match site {
                    // Ticket #46: Ships are built only at Shipyards, on a station or a Colony.
                    Place::State(_) => return fail("Ships are built at a Shipyard, on a station or a Colony"),
                    Place::Colony(c) => {
                        let Some(col) = self.colony(*c) else { return fail("no such Colony") };
                        if col.control.director() != Some(seat) {
                            return fail("you do not direct this Colony");
                        }
                        if !col.modules.iter().any(|m| m.kind == ModuleKind::Shipyard && m.working()) {
                            return fail("no Shipyard here");
                        }
                    }
                }
                Ok(cost)
            }
            Order::BuildArmy { place } => {
                match place {
                    Place::State(s) => {
                        if self.state(*s).control != Control::Controlled(seat) {
                            return fail("an Army needs a Nation State you control");
                        }
                    }
                    Place::Colony(c) => {
                        let Some(col) = self.colony(*c) else { return fail("no such Colony") };
                        if col.control.director() != Some(seat) {
                            return fail("you do not direct this Colony");
                        }
                        if !col.modules.iter().any(|m| m.kind == ModuleKind::Barracks) {
                            return fail("no Barracks here");
                        }
                        let has_army = self.armies.iter().any(|a| a.home == ArmyHome::Colony(*c))
                            || col.queue.iter().any(|b| b.item == BuildItem::Unit(UnitKind::Army))
                            || pending.iter().any(|o| matches!(o, Order::BuildArmy { place: p } if p == place));
                        if has_army {
                            return fail("this Barracks already holds an Army");
                        }
                    }
                }
                Ok(cost)
            }
            Order::Repair { unit, points } => {
                if *points == 0 {
                    return fail("nothing to repair");
                }
                match unit {
                    UnitRef::Ship(id) => {
                        let Some(ship) = self.ship(*id) else { return fail("no such Ship") };
                        if ship.seat != seat {
                            return fail("not your Ship");
                        }
                        if *points > ship.damage {
                            return fail("more repair than damage");
                        }
                        let ShipAt::Body(body) = ship.at else { return fail("a Ship in transit cannot be repaired") };
                        if !self.has_repair_yard(seat, body) {
                            return fail("needs a Launch Site or Shipyard at this Body");
                        }
                        if pending.iter().any(|o| matches!(o, Order::Transit { ship: s, .. } if s == id)) {
                            return fail("a Ship cannot repair and move in one turn");
                        }
                    }
                    UnitRef::Army(id) => {
                        let Some(army) = self.army(*id) else { return fail("no such Army") };
                        if self.army_seat(army) != Some(seat) {
                            return fail("not your Army");
                        }
                        if *points > army.damage {
                            return fail("more repair than damage");
                        }
                        match army.at {
                            ArmyAt::Place(Place::State(s)) => {
                                if self.state(s).control != Control::Controlled(seat) {
                                    return fail("an Army repairs only in a Nation State you control");
                                }
                            }
                            ArmyAt::Place(Place::Colony(c)) => {
                                let ok = self
                                    .colony(c)
                                    .map(|c| c.control.director() == Some(seat) && c.modules.iter().any(|m| m.kind == ModuleKind::Barracks))
                                    .unwrap_or(false);
                                if !ok {
                                    return fail("an Army repairs only at a Colony with a Barracks");
                                }
                            }
                            ArmyAt::Aboard(_) => return fail("an Army aboard a Ship cannot repair"),
                        }
                        if pending.iter().any(|o| matches!(o, Order::MoveArmy { army: a, .. } if a == id)) {
                            return fail("an Army cannot repair and move in one turn");
                        }
                    }
                }
                Ok(cost)
            }
            Order::Transit { ship, to } => {
                let Some(s) = self.ship(*ship) else { return fail("no such Ship") };
                if s.seat != seat {
                    return fail("not your Ship");
                }
                let ShipAt::Body(from) = s.at else { return fail("already in transit") };
                if from == *to {
                    return fail("already there");
                }
                if s.arrived_this_turn {
                    return fail("arrived this turn; it may act next turn");
                }
                if pending.iter().any(|o| matches!(o, Order::Transit { ship: x, .. } | Order::Load { ship: x, .. } | Order::Unload { ship: x, .. } if x == ship)) {
                    return fail("this Ship already has an order");
                }
                Ok(cost)
            }
            Order::ShipStance { body, .. } => {
                if self.ships_at(seat, *body).is_empty() {
                    return fail("no Ships of yours there");
                }
                Ok(cost)
            }
            Order::ArmyStance { place, .. } => {
                if self.armies_of_seat_at(seat, *place).is_empty() {
                    return fail("no Armies of yours there");
                }
                Ok(cost)
            }
            Order::MoveArmy { army, to } => {
                let Some(a) = self.army(*army) else { return fail("no such Army") };
                if self.army_seat(a) != Some(seat) {
                    return fail("not your Army");
                }
                if self.army_stands_down(a) {
                    return fail("this Army stands down");
                }
                let ArmyAt::Place(Place::State(from)) = a.at else { return fail("this Army is not in a Nation State") };
                if matches!(a.home, ArmyHome::Colony(_)) {
                    return fail("a Colony's Army never leaves");
                }
                if !self.tables.state(from).neighbours.contains(to) {
                    return fail("not a neighbouring continent");
                }
                if pending.iter().any(|o| matches!(o, Order::MoveArmy { army: x, .. } | Order::Load { army: Some(x), .. } if x == army)) {
                    return fail("this Army already has an order");
                }
                Ok(cost)
            }
            Order::Load { ship, colonists, from, army } => {
                let Some(s) = self.ship(*ship) else { return fail("no such Ship") };
                if s.seat != seat {
                    return fail("not your Ship");
                }
                let ShipAt::Body(body) = s.at else { return fail("in transit") };
                let card = self.tables.unit(s.kind);
                // Ticket #51: what a Colony Ship carries is a Faction figure (Steerage doubles it)
                // and rises with Expanded Habitats; nothing else carries Colonists.
                let capacity = if s.kind == UnitKind::ColonyShip { self.colony_ship_capacity(seat) } else { card.carries_colonists };
                if *colonists == 0 && army.is_none() {
                    return fail("nothing to load");
                }
                if s.colonists + *colonists > capacity {
                    return fail(format!("this Ship carries at most {capacity} Colonists"));
                }
                if pending.iter().any(|o| matches!(o, Order::Transit { ship: x, .. } | Order::Load { ship: x, .. } | Order::Unload { ship: x, .. } if x == ship)) {
                    return fail("this Ship already has an order");
                }
                if *colonists > 0 {
                    match from {
                        LoadSource::State(st) => {
                            if body != BodyId::Earth {
                                return fail("that Nation State is not at this Body");
                            }
                            if self.state(*st).control.director() != Some(seat) {
                                return fail("you do not direct that Nation State");
                            }
                            // Ticket #46: a lift to orbit needs a Launch Site there.
                            if !self.state(*st).facilities.iter().any(|f| f.kind == FacilityKind::LaunchSite && f.working()) {
                                return fail("a lift to orbit needs a working Launch Site there");
                            }
                            // Ticket #51, Steerage: a lift may cost the state more than one tenth
                            // of a person per Colonist.
                            if self.state(*st).population < self.lift_population(seat, *colonists) {
                                return fail("not enough people there");
                            }
                        }
                        LoadSource::Colony(c) => {
                            let Some(col) = self.colony(*c) else { return fail("no such Colony") };
                            if col.body != body || col.control.director() != Some(seat) {
                                return fail("that Colony is not yours at this Body");
                            }
                            if col.colonists < *colonists {
                                return fail("not enough Colonists there");
                            }
                        }
                    }
                }
                if let Some(aid) = army {
                    if !card.carries_army {
                        return fail("this Ship cannot carry an Army");
                    }
                    if s.army.is_some() {
                        return fail("this Ship already carries an Army");
                    }
                    let Some(a) = self.army(*aid) else { return fail("no such Army") };
                    if self.army_seat(a) != Some(seat) || a.standing && self.army_stands_down(a) {
                        return fail("not your Army");
                    }
                    if matches!(a.at, ArmyAt::Place(Place::State(st)) if !self.state(st).facilities.iter().any(|f| f.kind == FacilityKind::LaunchSite && f.working())) {
                        return fail("a lift to orbit needs a working Launch Site there");
                    }
                    if matches!(a.home, ArmyHome::Colony(_)) {
                        return fail("a Colony's Army never leaves");
                    }
                    let here = match a.at {
                        ArmyAt::Place(Place::State(_)) => body == BodyId::Earth,
                        ArmyAt::Place(Place::Colony(c)) => self.colony(c).map(|c| c.body == body).unwrap_or(false),
                        ArmyAt::Aboard(_) => false,
                    };
                    if !here {
                        return fail("that Army is not at this Body");
                    }
                }
                Ok(cost)
            }
            Order::Unload { ship, colonists, army, into } => {
                let Some(s) = self.ship(*ship) else { return fail("no such Ship") };
                if s.seat != seat {
                    return fail("not your Ship");
                }
                let ShipAt::Body(body) = s.at else { return fail("in transit") };
                if *colonists > s.colonists {
                    return fail("not that many Colonists aboard");
                }
                if *army && s.army.is_none() {
                    return fail("no Army aboard");
                }
                if *colonists == 0 && !*army {
                    return fail("nothing to unload");
                }
                if pending.iter().any(|o| matches!(o, Order::Transit { ship: x, .. } | Order::Load { ship: x, .. } | Order::Unload { ship: x, .. } if x == ship)) {
                    return fail("this Ship already has an order");
                }
                match into {
                    UnloadTarget::Slot(b, slot) => {
                        if *b != body {
                            return fail("that slot is not at this Body");
                        }
                        if s.kind != UnitKind::ColonyShip || *colonists == 0 {
                            return fail("only a Colony Ship with Colonists founds a Colony");
                        }
                        if !self.free_slots_on(body).contains(slot) {
                            return fail("that Colony Slot is taken");
                        }
                    }
                    UnloadTarget::Colony(c) => {
                        let Some(col) = self.colony(*c) else { return fail("no such Colony") };
                        if col.body != body {
                            return fail("that Colony is not at this Body");
                        }
                        if *colonists > 0 {
                            if col.control.director() != Some(seat) {
                                return fail("Colonists may only disembark into your own Colony");
                            }
                            if col.colonists + *colonists > self.habitat_room(col) {
                                return fail("no room in its Habitats");
                            }
                        }
                    }
                }
                Ok(cost)
            }
            Order::Influence { target, amount } => {
                if *amount <= 0 {
                    return fail("spend a positive amount");
                }
                match target {
                    Place::State(_) => {}
                    Place::Colony(c) => {
                        if self.colony(*c).is_none() {
                            return fail("no such Colony");
                        }
                    }
                }
                Ok(cost)
            }
            // Ticket #54: Mothball, Restart and Decommission.
            Order::Change { building, what } => self.check_change(seat, pending, *building, *what).map(|_| cost),
            // Ticket #54: Leapfrog, the Custodians only, on a state they control.
            Order::Leapfrog { state } => {
                if self.kind(seat) != FactionKind::Custodians {
                    return fail("only the Custodians Leapfrog");
                }
                if self.state(*state).control != Control::Controlled(seat) {
                    return fail("Leapfrog needs a Nation State you control");
                }
                // Every Leapfrog already pending this turn has to come off before the next one bites.
                let per = self.tables.climate.population_emissions_per_level;
                let queued = pending.iter().filter(|o| matches!(o, Order::Leapfrog { state: s } if s == state)).count() as f64;
                let base = self.tables.climate.population_emissions_base;
                if self.population_coefficient(*state) - queued * per <= base + 1e-9 {
                    return fail("its people already emit the base figure; a Leapfrog here would buy nothing");
                }
                Ok(cost)
            }
            // Ticket #54: the Strip Permit, the Prospectors only, once per state ever.
            Order::StripPermit { state } => {
                if self.kind(seat) != FactionKind::Prospectors {
                    return fail("only the Prospectors issue a Strip Permit");
                }
                if self.state(*state).control != Control::Controlled(seat) {
                    return fail("a Strip Permit needs a Nation State you control");
                }
                if self.state(*state).strip_permit_used {
                    return fail("this Nation State has had its Strip Permit");
                }
                if pending.iter().any(|o| matches!(o, Order::StripPermit { state: s } if s == state)) {
                    return fail("a Strip Permit is already ordered here");
                }
                Ok(cost)
            }
        }
    }

    /// Ticket #54: is this Mothball, Restart or Decommission legal? Named so the Ducat-paid and
    /// Materials-paid forms and the interface can all ask the same question.
    fn check_change(&self, seat: Seat, pending: &[Order], building: BuildingRef, what: BuildingChange) -> Result<(), OrderError> {
        let place = building.place();
        if self.place_control(place).director() != Some(seat) {
            return fail("you do not direct this place");
        }
        if pending.iter().any(|o| matches!(o, Order::Change { building: b, .. } if *b == building)) {
            return fail("this building already has an order this turn");
        }
        let (mothballed, changing, is_archive) = match building {
            BuildingRef::Facility(sid, i) => match self.state(sid).facilities.get(i) {
                Some(f) => (f.mothballed, f.change.is_some(), false),
                None => return fail("no such Facility"),
            },
            BuildingRef::Module(cid, i) => match self.colony(cid).and_then(|c| c.modules.get(i)) {
                Some(m) => (m.mothballed, m.change.is_some(), m.kind == ModuleKind::Archive),
                None => return fail("no such Module"),
            },
        };
        if is_archive {
            return fail("the Archive is raised and lost by its own rules; it is not mothballed");
        }
        if changing {
            return fail("this building is already being mothballed, restarted or decommissioned");
        }
        match what {
            BuildingChange::Mothball if mothballed => fail("it is already mothballed"),
            BuildingChange::Restart if !mothballed => fail("it is not mothballed"),
            _ => Ok(()),
        }
    }

    pub fn has_repair_yard(&self, seat: Seat, body: BodyId) -> bool {
        match body {
            BodyId::Earth => self
                .directed_states(seat)
                .iter()
                .any(|s| self.state(*s).facilities.iter().any(|f| f.kind == FacilityKind::LaunchSite && f.working())),
            b => self
                .colonies
                .iter()
                .any(|c| c.body == b && c.control.director() == Some(seat) && c.modules.iter().any(|m| m.kind == ModuleKind::Shipyard && m.working())),
        }
    }

    /// Validate a whole order list in sequence.
    pub fn check_orders(&self, seat: Seat, orders: &[Order]) -> Result<(), (usize, OrderError)> {
        for (i, o) in orders.iter().enumerate() {
            self.check_order(seat, &orders[..i], o).map_err(|e| (i, e))?;
        }
        Ok(())
    }

    /// Pay for and record every order of a seat at End Turn (spec 7.3: costs are paid at once).
    pub fn commit_orders(&mut self, seat: Seat, orders: &[Order]) {
        for order in orders {
            let cost = self.order_cost(seat, order);
            {
                let st = &mut self.seat_mut(seat).stockpile;
                st.materials -= cost.materials;
                st.fuel -= cost.fuel;
                st.energy -= cost.energy;
                st.ducats -= cost.ducats;
            }
            self.seat_mut(seat).allotment -= cost.influence;
            let turn = self.turn;
            match order {
                Order::BuildFacility { state, kind } | Order::BuildFacilityWithDucats { state, kind } => {
                    let due = turn + self.tables.facility(*kind).build_turns - 1;
                    self.state_mut(*state).queue.push(Build { item: BuildItem::Facility(*kind), seat, due_turn: due });
                }
                Order::RaiseIndustry { state } => {
                    let due = turn + self.tables.industry_level.build_turns - 1;
                    self.state_mut(*state).queue.push(Build { item: BuildItem::IndustryLevel, seat, due_turn: due });
                }
                Order::BuildModule { colony, kind } | Order::BuildModuleWithDucats { colony, kind } => {
                    let due = turn + self.tables.module(*kind).build_turns - 1;
                    if let Some(c) = self.colony_mut(*colony) {
                        c.queue.push(Build { item: BuildItem::Module(*kind), seat, due_turn: due });
                    }
                }
                Order::BuildShip { site, kind } => {
                    let due = turn + self.tables.unit(*kind).build_turns - 1;
                    let b = Build { item: BuildItem::Unit(*kind), seat, due_turn: due };
                    match site {
                        Place::State(s) => self.state_mut(*s).queue.push(b),
                        Place::Colony(c) => {
                            if let Some(c) = self.colony_mut(*c) {
                                c.queue.push(b)
                            }
                        }
                    }
                }
                Order::BuildArmy { place } => {
                    let due = turn + self.tables.unit(UnitKind::Army).build_turns - 1;
                    let b = Build { item: BuildItem::Unit(UnitKind::Army), seat, due_turn: due };
                    match place {
                        Place::State(s) => self.state_mut(*s).queue.push(b),
                        Place::Colony(c) => {
                            if let Some(c) = self.colony_mut(*c) {
                                c.queue.push(b)
                            }
                        }
                    }
                }
                Order::Repair { unit, points } => self.pending.repairs.push((seat, *unit, *points)),
                Order::Transit { ship, to } => {
                    let from = match self.ship(*ship).map(|s| s.at) {
                        Some(ShipAt::Body(b)) => b,
                        _ => continue,
                    };
                    let (turns, _) = self.transit_cost(from, *to);
                    let name = self.tables.body(*to).name.clone();
                    if let Some(s) = self.ship_mut(*ship) {
                        s.at = ShipAt::Transit { from, to: *to, turns_left: turns };
                    }
                    self.log(format!("{} launches {} toward {} ({} turns).", self.seat_name(seat), ship, name, turns));
                }
                Order::ShipStance { body, stance } => {
                    for s in self.ships.iter_mut().filter(|s| s.seat == seat && s.at == ShipAt::Body(*body)) {
                        s.stance = *stance;
                    }
                    if *stance == Stance::Attack {
                        self.pending.attack_sequence += 1;
                    }
                }
                Order::ArmyStance { place, stance } => {
                    let ids = self.armies_of_seat_at(seat, *place);
                    for a in self.armies.iter_mut().filter(|a| ids.contains(&a.id)) {
                        a.stance = *stance;
                    }
                }
                Order::MoveArmy { army, to } => {
                    if let Some(a) = self.army_mut(*army) {
                        a.move_to = Some(*to);
                    }
                }
                Order::Load { from: LoadSource::State(_), .. } => {
                    // Ticket #46: a lift from a Nation State is a launch.
                    self.climate.launches_pending[seat.index()] += 1;
                    self.pending.cargo.push((seat, order.clone()));
                }
                Order::Load { .. } | Order::Unload { .. } => self.pending.cargo.push((seat, order.clone())),
                Order::BuildStation { body, slot } => self.pending.stations.push((seat, *body, *slot)),
                Order::BuildArchiveStage { colony } => {
                    // Ticket #51: the Research leaves the fund now, with the Materials; the stage
                    // itself rises in the Colony's queue like any other build.
                    let per = self.tables.archive.research_per_stage;
                    self.seat_mut(seat).archive_fund -= per;
                    let due = turn + self.tables.module(ModuleKind::Archive).build_turns - 1;
                    if let Some(c) = self.colony_mut(*colony) {
                        c.queue.push(Build { item: BuildItem::Module(ModuleKind::Archive), seat, due_turn: due });
                    }
                    let stage = self.archive_stages_committed(seat);
                    let line = format!("The {} began stage {} of the Archive at {}.", self.seat_name(seat), stage, self.place_name(Place::Colony(*colony)));
                    self.log(line.clone());
                    self.report.lines.push(line);
                }
                Order::FundArchive => self.fund_archive(seat),
                // Ticket #52: both act at Resolution; Resettle also steers the next Climate phase's
                // refugee flows, which is the first flow after these orders are given.
                Order::Relief { state } => self.pending.relief.push((seat, *state)),
                Order::Resettle { state } => {
                    self.seat_mut(seat).resettle_to = Some(*state);
                    self.pending.resettle.push((seat, *state));
                }
                Order::Influence { target, amount } => self.pending.influence.push((seat, *target, *amount)),
                // Ticket #54: the change is written on the building itself and lands at the
                // Resolution of its due turn, so nothing has to track a position between turns.
                Order::Change { building, what } => {
                    let turns = match what {
                        BuildingChange::Mothball => 1,
                        BuildingChange::Restart => self.tables.mothball.restart_turns.max(1),
                        BuildingChange::Decommission => self.tables.mothball.decommission_turns.max(1),
                    };
                    let due = turn + turns - 1;
                    let change = PendingChange { what: *what, due_turn: due, seat };
                    match building {
                        BuildingRef::Facility(sid, i) => {
                            if let Some(f) = self.state_mut(*sid).facilities.get_mut(*i) {
                                f.change = Some(change);
                            }
                        }
                        BuildingRef::Module(cid, i) => {
                            if let Some(m) = self.colony_mut(*cid).and_then(|c| c.modules.get_mut(*i)) {
                                m.change = Some(change);
                            }
                        }
                    }
                }
                // Ticket #54: Leapfrog is permanent and takes hold at once, before the next Climate
                // phase reads the state's coefficient.
                Order::Leapfrog { state } => {
                    let per = self.tables.climate.population_emissions_per_level;
                    self.state_mut(*state).leapfrog += per;
                    let line = format!(
                        "The {} Leapfrogged {}: its people now emit {:.2} per hundred million.",
                        self.seat_name(seat),
                        self.tables.state(*state).name,
                        self.population_coefficient(*state)
                    );
                    self.log(line.clone());
                    self.report.lines.push(line);
                }
                // Ticket #54: the Strip Permit runs from the next Income for `turns` turns.
                Order::StripPermit { state } => {
                    let turns = self.tables.strip_permit.turns;
                    {
                        let st = self.state_mut(*state);
                        st.strip_permit_used = true;
                        st.strip_permit_ends = Some(turn + turns);
                    }
                    let line = format!(
                        "The {} issued a Strip Permit in {}: every Facility there produces double for {} turns.",
                        self.seat_name(seat),
                        self.tables.state(*state).name,
                        turns
                    );
                    self.log(line.clone());
                    self.report.lines.push(line);
                }
                Order::BuyInfluence { amount } => {
                    self.seat_mut(seat).allotment += amount;
                    self.log(format!("{} bought {} Influence with Ducats.", self.seat_name(seat), amount));
                }
                Order::RepairWithDucats { unit, points } => self.pending.repairs.push((seat, *unit, *points)),
                Order::Buy { resource, amount } => {
                    self.log(format!("{} bought {} {} for {} Ducats.", self.seat_name(seat), amount, resource.name(), cost.ducats));
                }
                Order::Sell { resource, amount } => {
                    self.log(format!("{} sold {} {} for {} Ducats.", self.seat_name(seat), amount, resource.name(), -cost.ducats));
                }
            }
        }
    }
}

//! Orders: what a seat may do in the Orders phase, what each costs, and what is legal (spec 6, 7.3, 7.4, 8, 9).

use crate::ids::*;
use crate::state::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnitRef {
    Ship(ShipId),
    Army(ArmyId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadSource {
    State(StateId),
    Colony(ColonyId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnloadTarget {
    /// Found a Colony into a free slot (spec 9.4).
    Slot(BodyId, u32),
    /// Disembark into an existing Colony on this Body (own: into Habitats; enemy: an Army lands to attack).
    Colony(ColonyId),
}

/// Ticket #54 (version 0.05): one standing building, by its place and its position in that place's
/// list. Orders are given and resolved inside one turn, so the position cannot move under them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Order {
    BuildFacility { state: StateId, kind: FacilityKind },
    RaiseIndustry { state: StateId },
    BuildModule { colony: ColonyId, kind: ModuleKind },
    BuildShip { site: Place, kind: UnitKind },
    BuildArmy { place: Place },
    Repair { unit: UnitRef, points: u32 },
    Transit { ship: ShipId, to: BodyId },
    /// Version 0.06.0 (ticket #87): fill a Ship's tank from the Stockpile at a Body where its
    /// Faction holds a Space Station, as far as the Stockpile can pay.
    Refuel { ship: ShipId },
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
    /// Version 0.05 (ticket #51): the Archive, at a Colony off Earth. Version 0.05.5 (ticket #68):
    /// one Module, paid in Materials from the Stockpile; its Research is paid into the fund after.
    BuildArchive { colony: ColonyId },
    /// Version 0.05 (ticket #51): this turn's Research from the Archivists' Labs goes into the
    /// Archive fund instead of the shared Tech, and counts nothing toward the Research Lead.
    FundArchive,
    /// Version 0.05.5 (ticket #73): muster Emigrants, the built Colonists, in a Nation State the
    /// seat directs: up to four a turn per Faction, in one state, at a tenth of a person each.
    BuildEmigrants { state: StateId, n: u32 },
    /// Version 0.05.5 (ticket #73): send waiting Emigrants to Antarctica by sea, into a free slot
    /// (founding a Colony) or the seat's own Colony there; a turn to arrive, no launch.
    SendToAntarctica { state: StateId, n: u32, into: UnloadTarget },
    /// Version 0.05.5 (ticket #72): the Prospectors set the share of their Materials output the
    /// Venture Capital Fund banks each Income, in whole percent (a step of 10, 0 to 80).
    SetVentureShare { share: u32 },
    /// Version 0.05.5 (ticket #72): the Prospectors take Materials back out of the Fund, nine
    /// tenths of them returning to the Stockpile.
    DrawVenture { amount: i64 },
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
            // Ticket #72: the Faction's own Facility price (the Prospectors' 15% off).
            Order::BuildFacility { kind, .. } => Cost { materials: self.facility_materials(seat, *kind), ..Default::default() },
            Order::RaiseIndustry { .. } => Cost { materials: self.industry_cost(seat), ..Default::default() },
            // Ticket #51: a Faction's card may make its Modules and its Colony Ships cost less.
            // Ticket #88: and the Colony's working Mines take more off.
            Order::BuildModule { colony, kind } => Cost { materials: self.module_materials_at(seat, *colony, *kind), ..Default::default() },
            // Ticket #87: a Ship is built with a full tank, its Fuel paid at the build.
            Order::BuildShip { kind, .. } => Cost { materials: self.ship_materials(seat, *kind), fuel: t.unit(*kind).tank, ..Default::default() },
            Order::BuildArmy { .. } => Cost { materials: t.unit(UnitKind::Army).materials, ..Default::default() },
            Order::Repair { points, .. } => {
                Cost { materials: t.repair.materials_per_point * *points as i64, ..Default::default() }
            }
            // Ticket #87: a transit spends the Ship's tank, not the Stockpile; a Refuel takes from
            // the Stockpile what the tank wants and the Stockpile can pay.
            Order::Transit { .. } => Cost::default(),
            Order::Refuel { ship } => Cost { fuel: self.refuel_amount(seat, *ship), ..Default::default() },
            Order::Influence { amount, .. } => Cost { influence: *amount, ..Default::default() },
            // Ticket #54: a Mothball and a Strip Permit are free; a Restart costs Materials and a
            // Leapfrog Ducats; a Decommission pays Materials back, which arrive at its Resolution.
            Order::Change { what: BuildingChange::Restart, .. } => Cost { materials: t.mothball.restart_materials, ..Default::default() },
            Order::Leapfrog { .. } => Cost { ducats: t.ducats.per_leapfrog, ..Default::default() },
            Order::BuyInfluence { amount } => Cost { ducats: t.ducats.per_influence * *amount, ..Default::default() },
            // A purchase is a negative cost in the resource bought, so `remaining` and `commit_orders`
            // add it without a special case; a sale is the mirror, with a negative Ducat cost.
            Order::Buy { resource, amount } => {
                // Ticket #83: the lot's price, times the seat's market multiplier, rounded down.
                let ducats = self.market_price(seat, self.trade_price(*resource).unwrap_or(0) * *amount);
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
            Order::BuildFacilityWithDucats { kind, .. } => Cost { ducats: self.market_price(seat, self.facility_materials(seat, *kind) * t.ducats.per_building_material), ..Default::default() },
            Order::BuildStation { .. } => Cost { materials: self.station_materials(seat), ..Default::default() },
            Order::BuildModuleWithDucats { colony, kind } => Cost { ducats: self.market_price(seat, self.module_materials_at(seat, *colony, *kind) * t.ducats.per_building_material), ..Default::default() },
            // Ticket #68: the Archive Module costs its row's Materials; the Research comes after.
            // Ticket #88: the Archive is a Module, so its Colony's working Mines take off too.
            Order::BuildArchive { colony } => Cost { materials: self.module_materials_at(seat, *colony, ModuleKind::Archive), ..Default::default() },
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

    /// Ticket #83 (version 0.06.0): what the window charges this seat for a lot priced at `ducats`:
    /// times the Faction's market multiplier (the Prospectors' 0.85), rounded down.
    pub fn market_price(&self, seat: Seat, ducats: i64) -> i64 {
        (ducats as f64 * self.tables.faction(self.kind(seat)).market_multiplier).floor() as i64
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
                // Ticket #68: at the cap a turn of funding is refused, and the Research stays with
                // the shared Tech; until the Module stands the cap is a quarter of the requirement.
                if self.seat(seat).archive_fund >= self.archive_fund_cap(seat) {
                    return if self.archive_built(seat) {
                        fail("the Archive's Research is paid in full")
                    } else {
                        fail(format!("the Archive fund holds its quarter ({}) until the Archive stands at a Colony off Earth", self.archive_fund_cap(seat)))
                    };
                }
                Ok(cost)
            }
            Order::BuildArchive { colony } => {
                if self.kind(seat) != FactionKind::Archivists {
                    return fail("only the Archivists build the Archive");
                }
                let Some(col) = self.colony(*colony) else { return fail("no such Colony") };
                if col.control.director() != Some(seat) {
                    return fail("you do not direct this Colony");
                }
                if !self.may_hold_archive(col) {
                    return fail("the Archive stands at a Colony off Earth; Antarctica will not do");
                }
                // At most one Archive per Faction, wherever it stands.
                if let Some(home) = self.archive_colony(seat)
                    && home != *colony
                {
                    return fail(format!("the Archive already stands at {}", self.place_name(Place::Colony(home))));
                }
                // Ticket #68: one Module, built once.
                if self.archive_built(seat) {
                    return fail("the Archive already stands");
                }
                if self.archive_ordered(seat) || pending.iter().any(|o| matches!(o, Order::BuildArchive { .. })) {
                    return fail("the Archive is already building");
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
                // Ticket #56: a Facility that needs a Tech waits for it. Provisional Findings gives
                // half an effect, never half an unlock, so the Tech must be done.
                if let Some(t) = self.tables.facility(*kind).needs_tech
                    && !self.has_tech(t)
                {
                    return fail(format!("{} needs {}", kind.name(), self.tables.tech(t).name));
                }
                // Ticket #54: a Scrubber takes no build slot, so it neither needs one nor uses one.
                // Ticket #56: the slot it does need is coastal or inland, and a Sea Wall wants a
                // coastal one. The orders already pending in this state take their slots first.
                if self.takes_slot(*kind) {
                    let (mut taken_coastal, mut taken_inland) = (0, 0);
                    for k in pending
                        .iter()
                        .filter(|o| o.build_state() == Some(*state))
                        .filter_map(|o| o.build_facility())
                        .filter(|k| self.takes_slot(*k))
                    {
                        match self.next_slot_is_coastal(*state, k, taken_coastal, taken_inland) {
                            Some(true) => taken_coastal += 1,
                            Some(false) => taken_inland += 1,
                            None => {}
                        }
                    }
                    if self.next_slot_is_coastal(*state, *kind, taken_coastal, taken_inland).is_none() {
                        return fail(if self.tables.facility(*kind).coastal_only { "no free coastal slot" } else { "no free build slot" });
                    }
                }
                // Ticket #56: at most one Sea Wall stands in a Nation State.
                if *kind == FacilityKind::SeaWall
                    && (self.state(*state).facilities.iter().any(|f| f.kind == FacilityKind::SeaWall)
                        || self.state(*state).queue.iter().any(|b| b.item == BuildItem::Facility(FacilityKind::SeaWall))
                        || pending.iter().any(|o| o.build_state() == Some(*state) && o.build_facility() == Some(FacilityKind::SeaWall)))
                {
                    return fail("this Nation State already has a Sea Wall");
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
                    return fail("the Archive is raised from its own button");
                }
                // Ticket #46: a station holds only a Shipyard and Habitats; ticket #80: and Observatories.
                if col.in_orbit && !matches!(kind, ModuleKind::Shipyard | ModuleKind::Habitat | ModuleKind::Observatory) {
                    return fail("a station holds only a Shipyard, Habitats and Observatories");
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
                if pending.iter().any(|o| matches!(o, Order::Transit { ship: x, .. } | Order::Load { ship: x, .. } | Order::Unload { ship: x, .. } | Order::Refuel { ship: x } if x == ship)) {
                    return fail("this Ship already has an order");
                }
                // Ticket #87: the leg is paid from the tank.
                let (_, fuel) = self.transit_cost_for(seat, from, *to);
                if s.fuel < fuel {
                    return fail(format!("the tank holds {} Fuel of {}; this leg needs {fuel}", s.fuel, self.tables.unit(s.kind).tank));
                }
                Ok(cost)
            }
            Order::Refuel { ship } => {
                let Some(s) = self.ship(*ship) else { return fail("no such Ship") };
                if s.seat != seat {
                    return fail("not your Ship");
                }
                let ShipAt::Body(body) = s.at else { return fail("in transit") };
                if !self.own_station_at(seat, body) {
                    return fail(format!("no station of yours over {} to refuel at", self.tables.body(body).name));
                }
                if s.fuel >= self.tables.unit(s.kind).tank {
                    return fail("the tank is full");
                }
                if cost.fuel <= 0 {
                    return fail("no Fuel in the Stockpile to fill it with");
                }
                if pending.iter().any(|o| matches!(o, Order::Transit { ship: x, .. } | Order::Refuel { ship: x } if x == ship)) {
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
                // Ticket #86: at Earth a warming world crowds a Colony Ship beyond its capacity.
                let capacity = if s.kind == UnitKind::ColonyShip {
                    if body == BodyId::Earth { self.colony_ship_crowded_capacity(seat) } else { self.colony_ship_capacity(seat) }
                } else {
                    card.carries_colonists
                };
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
                            // Ticket #73: a Launch Site lifts only the Emigrants waiting there; the
                            // population was paid when they mustered.
                            if self.state(*st).emigrants < *colonists {
                                return fail(format!("only {} Emigrants are waiting there", self.state(*st).emigrants));
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
                        // Ticket #56: Antarctica is shut under the ice until the world is warm enough.
                        if *b == BodyId::Earth && !self.antarctica_open {
                            return fail(format!("the Antarctic ice has not opened: it opens at {:+.1} C", self.tables.climate.antarctica_opens_at));
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
            // Ticket #73: Emigrants muster four a turn per Faction, in one state it directs.
            Order::BuildEmigrants { state, n } => {
                if self.state(*state).control.director() != Some(seat) {
                    return fail("you do not direct that Nation State");
                }
                let cap = self.emigrants_per_turn(seat);
                if *n == 0 || *n > cap {
                    return fail(format!("up to {cap} Emigrants a turn"));
                }
                if pending.iter().any(|o| matches!(o, Order::BuildEmigrants { .. })) {
                    return fail("Emigrants are already mustering this turn: one state a turn");
                }
                if self.state(*state).population < self.lift_population(seat, *n) {
                    return fail("not enough people there");
                }
                Ok(cost)
            }
            // Ticket #73: waiting Emigrants go to Antarctica by sea, from any state the seat directs.
            Order::SendToAntarctica { state, n, into } => {
                if self.state(*state).control.director() != Some(seat) {
                    return fail("you do not direct that Nation State");
                }
                if !self.antarctica_open {
                    return fail(format!("the Antarctic ice has not opened: it opens at {:+.1} C", self.tables.climate.antarctica_opens_at));
                }
                let sending: u32 = pending.iter().map(|o| if let Order::SendToAntarctica { state: s, n, .. } = o { if s == state { *n } else { 0 } } else { 0 }).sum();
                let waiting = self.state(*state).emigrants.saturating_sub(sending);
                if *n == 0 || *n > waiting {
                    return fail(format!("{waiting} Emigrants are waiting there"));
                }
                match into {
                    UnloadTarget::Slot(b, slot) => {
                        if *b != BodyId::Earth || !self.free_slots_on(BodyId::Earth).contains(slot) {
                            return fail("that Antarctic slot is not free");
                        }
                        if pending.iter().any(|o| matches!(o, Order::SendToAntarctica { into: UnloadTarget::Slot(_, s), .. } if s == slot)) {
                            return fail("Emigrants are already bound for that slot this turn");
                        }
                    }
                    UnloadTarget::Colony(c) => {
                        let Some(col) = self.colony(*c) else { return fail("no such Colony") };
                        if col.body != BodyId::Earth || col.in_orbit || col.control.director() != Some(seat) {
                            return fail("that is not your Colony in Antarctica");
                        }
                    }
                }
                Ok(cost)
            }
            // Ticket #72: the Venture Capital Fund's two orders, the Prospectors only.
            Order::SetVentureShare { share } => {
                if self.kind(seat) != FactionKind::Prospectors {
                    return fail("only the Prospectors have a Venture Capital Fund");
                }
                let v = &self.tables.venture;
                let step = (v.share_step * 100.0).round() as u32;
                let max = (v.max_share * 100.0).round() as u32;
                if step == 0 || share % step != 0 || *share > max {
                    return fail(format!("the share moves in steps of {step}% from 0% to {max}%"));
                }
                if pending.iter().any(|o| matches!(o, Order::SetVentureShare { .. })) {
                    return fail("the share is already being set this turn");
                }
                Ok(cost)
            }
            Order::DrawVenture { amount } => {
                if self.kind(seat) != FactionKind::Prospectors {
                    return fail("only the Prospectors have a Venture Capital Fund");
                }
                let drawn: i64 = pending.iter().map(|o| if let Order::DrawVenture { amount } = o { *amount } else { 0 }).sum();
                let fund = self.seat(seat).venture_fund - drawn;
                if *amount <= 0 || *amount > fund {
                    return fail(format!("the Fund holds {}", fund.max(0)));
                }
                Ok(cost)
            }
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
                    // Ticket #56: the build reserves the slot it will stand in, coastal or inland.
                    let coastal = self.next_slot_is_coastal(*state, *kind, 0, 0).unwrap_or(false);
                    self.state_mut(*state).queue.push(Build { item: BuildItem::Facility(*kind), seat, due_turn: due, coastal });
                }
                Order::RaiseIndustry { state } => {
                    let due = turn + self.tables.industry_level.build_turns - 1;
                    self.state_mut(*state).queue.push(Build { item: BuildItem::IndustryLevel, seat, due_turn: due, coastal: false });
                }
                Order::BuildModule { colony, kind } | Order::BuildModuleWithDucats { colony, kind } => {
                    let due = turn + self.tables.module(*kind).build_turns - 1;
                    if let Some(c) = self.colony_mut(*colony) {
                        c.queue.push(Build { item: BuildItem::Module(*kind), seat, due_turn: due, coastal: false });
                    }
                }
                Order::BuildShip { site, kind } => {
                    let due = turn + self.tables.unit(*kind).build_turns - 1;
                    let b = Build { item: BuildItem::Unit(*kind), seat, due_turn: due, coastal: false };
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
                    let b = Build { item: BuildItem::Unit(UnitKind::Army), seat, due_turn: due, coastal: false };
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
                    // Ticket #87: the leg's Fuel, with the Faction's and the Tech's multipliers, from the tank.
                    let (_, fuel) = self.transit_cost_for(seat, from, *to);
                    let name = self.tables.body(*to).name.clone();
                    if let Some(s) = self.ship_mut(*ship) {
                        s.at = ShipAt::Transit { from, to: *to, turns_left: turns };
                        s.fuel = (s.fuel - fuel).max(0);
                    }
                    self.log(format!("{} launches {} toward {} ({} turns, {} Fuel from the tank).", self.seat_name(seat), ship, name, turns, fuel));
                }
                // Ticket #87: the Fuel came out of the Stockpile with the order's cost; it goes into the tank.
                Order::Refuel { ship } => {
                    let amount = cost.fuel;
                    let tank = self.ship(*ship).map(|s| self.tables.unit(s.kind).tank).unwrap_or(0);
                    if let Some(s) = self.ship_mut(*ship) {
                        s.fuel = (s.fuel + amount).min(tank);
                    }
                    self.log(format!("{} refuels {} with {} Fuel.", self.seat_name(seat), ship, amount));
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
                Order::BuildArchive { colony } => {
                    // Ticket #68: the Module rises in the Colony's queue like any other build, three
                    // turns from its own row; the Research is paid into the fund once it stands.
                    let due = turn + self.tables.module(ModuleKind::Archive).build_turns - 1;
                    if let Some(c) = self.colony_mut(*colony) {
                        c.queue.push(Build { item: BuildItem::Module(ModuleKind::Archive), seat, due_turn: due, coastal: false });
                    }
                    let line = format!("The {} began the Archive at {}.", self.seat_name(seat), self.place_name(Place::Colony(*colony)));
                    self.log(line);
                    let text = self.say("archive_begun", &[("faction", self.seat_name(seat)), ("colony", self.place_name(Place::Colony(*colony)))]);
                    self.report_line(LineKind::Archive, Some(ReportPlace::Colony(*colony)), text);
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
                // Ticket #73: Emigrants muster now, at the population's cost, and calm the state;
                // nothing lifts them before next turn, which is the turn to muster.
                Order::BuildEmigrants { state, n } => {
                    let cost = self.lift_population(seat, *n);
                    {
                        let st = self.state_mut(*state);
                        st.population = (st.population - cost).max(0.0);
                        st.emigrants += n;
                    }
                    let fell = self.lower_unrest(*state, self.tables.emigrants.unrest_fall);
                    let line = format!("{} Emigrants mustered in {} for the {}; its Unrest fell by {} to {}.", n, self.tables.state(*state).name, self.seat_name(seat), Game::unrest_figure(fell), self.unrest_text(*state));
                    self.log(line);
                    let text = self.say(
                        "emigrants_mustered",
                        &[("n", n.to_string()), ("state", self.tables.state(*state).name.clone()), ("fell", Game::unrest_figure(fell).to_string()), ("unrest", self.unrest_text(*state))],
                    );
                    self.report_line_of(seat, LineKind::YourWorks, LineKind::Note, Some(ReportPlace::State(*state)), text);
                }
                // Ticket #73: Emigrants leave for Antarctica by sea, and land a turn later.
                Order::SendToAntarctica { state, n, into } => {
                    let left = self.state(*state).emigrants.saturating_sub(*n);
                    self.state_mut(*state).emigrants = left;
                    let due = turn + self.tables.emigrants.antarctica_turns;
                    self.antarctic_sends.push(AntarcticSend { seat, from: *state, n: *n, into: *into, due_turn: due });
                    let line = format!("{} Emigrants left {} for Antarctica by sea, for the {}.", n, self.tables.state(*state).name, self.seat_name(seat));
                    self.log(line);
                }
                // Ticket #72: the Fund's orders land now; the share is read at the next Income.
                Order::SetVentureShare { share } => {
                    self.seat_mut(seat).venture_share = *share as f64 / 100.0;
                    let line = format!("The {} set the Venture Capital Fund to bank {}% of their Materials output.", self.seat_name(seat), share);
                    self.log(line);
                }
                Order::DrawVenture { amount } => {
                    let back = (*amount as f64 * self.tables.venture.draw_return).floor() as i64;
                    {
                        let s = self.seat_mut(seat);
                        s.venture_fund -= amount;
                        s.stockpile.materials += back;
                    }
                    let line = format!("The {} drew {} Materials from the Venture Capital Fund; {} came back to the Stockpile.", self.seat_name(seat), amount, back);
                    self.log(line);
                }
                Order::Leapfrog { state } => {
                    let per = self.tables.climate.population_emissions_per_level;
                    self.state_mut(*state).leapfrog += per;
                    let line = format!(
                        "The {} Leapfrogged {}: its people now emit {:.2} per hundred million.",
                        self.seat_name(seat),
                        self.tables.state(*state).name,
                        self.population_coefficient(*state)
                    );
                    self.log(line);
                    let text = self.say(
                        "leapfrog",
                        &[
                            ("faction", self.seat_name(seat)),
                            ("state", self.tables.state(*state).name.clone()),
                            ("coefficient", format!("{:.2}", self.population_coefficient(*state))),
                        ],
                    );
                    self.report_line(LineKind::Climate, Some(ReportPlace::State(*state)), text);
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
                    self.log(line);
                    let text = self.say(
                        "strip_permit",
                        &[("faction", self.seat_name(seat)), ("state", self.tables.state(*state).name.clone()), ("turns", turns.to_string())],
                    );
                    self.report_line(LineKind::Note, Some(ReportPlace::State(*state)), text);
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

/// Ticket #58: orders the board would show as one act are told as one. Three Influence orders on the
/// same place are one spend of their sum, three buys of Materials are one purchase, and an order
/// repeated exactly is said once. Everything else keeps the order it was given in.
pub fn merged_for_report(list: &[Order]) -> Vec<Order> {
    let mut out: Vec<Order> = Vec::new();
    for o in list {
        let merged = match o {
            Order::Influence { target, amount } => out
                .iter_mut()
                .find_map(|x| match x {
                    Order::Influence { target: t, amount: a } if t == target => Some(a),
                    _ => None,
                })
                .map(|a| *a += amount),
            Order::BuyInfluence { amount } => out
                .iter_mut()
                .find_map(|x| match x {
                    Order::BuyInfluence { amount: a } => Some(a),
                    _ => None,
                })
                .map(|a| *a += amount),
            Order::Buy { resource, amount } => out
                .iter_mut()
                .find_map(|x| match x {
                    Order::Buy { resource: r, amount: a } if r == resource => Some(a),
                    _ => None,
                })
                .map(|a| *a += amount),
            Order::Sell { resource, amount } => out
                .iter_mut()
                .find_map(|x| match x {
                    Order::Sell { resource: r, amount: a } if r == resource => Some(a),
                    _ => None,
                })
                .map(|a| *a += amount),
            other => out.iter().find(|x| *x == other).map(|_| ()),
        };
        if merged.is_none() {
            out.push(o.clone());
        }
    }
    out
}

impl Game {
    /// Ticket #58: one clause saying what a rival Faction did with one order it committed. Only
    /// what the board or its cards would show: nothing the AI scored, waited for or skipped. `None`
    /// for an order that leaves no visible mark.
    pub fn rival_deed(&self, _seat: Seat, order: &Order) -> Option<String> {
        let r = |key: &str, args: &[(&str, String)]| Some(self.tables.report.rival(key, args));
        let place = |p: Place| self.place_name(p);
        let building = |b: BuildingRef| -> String {
            match b {
                BuildingRef::Facility(sid, i) => {
                    self.state(sid).facilities.get(i).map(|f| f.kind.name().to_string()).unwrap_or_else(|| "building".into())
                }
                BuildingRef::Module(cid, i) => {
                    self.colony(cid).and_then(|c| c.modules.get(i)).map(|m| m.kind.name().to_string()).unwrap_or_else(|| "building".into())
                }
            }
        };
        let unit_of = |u: UnitRef| -> String {
            match u {
                UnitRef::Ship(id) => self.ship(id).map(|s| s.kind.name().to_string()).unwrap_or_else(|| "Ship".into()),
                UnitRef::Army(_) => "an Army".to_string(),
            }
        };
        match order {
            Order::BuildFacility { state, kind } => {
                r("build_facility", &[("building", kind.name().to_string()), ("state", self.tables.state(*state).name.clone())])
            }
            Order::BuildFacilityWithDucats { state, kind } => {
                r("build_facility_ducats", &[("building", kind.name().to_string()), ("state", self.tables.state(*state).name.clone())])
            }
            Order::RaiseIndustry { state } => r("raise_industry", &[("state", self.tables.state(*state).name.clone())]),
            Order::BuildModule { colony, kind } => {
                r("build_module", &[("building", kind.name().to_string()), ("colony", place(Place::Colony(*colony)))])
            }
            Order::BuildModuleWithDucats { colony, kind } => {
                r("build_module_ducats", &[("building", kind.name().to_string()), ("colony", place(Place::Colony(*colony)))])
            }
            Order::BuildShip { site, kind } => r("build_ship", &[("unit", kind.name().to_string()), ("place", place(*site))]),
            Order::BuildArmy { place: p } => r("build_army", &[("place", place(*p))]),
            Order::BuildStation { body, .. } => r("build_station", &[("body", self.tables.body(*body).name.clone())]),
            // Ticket #87.
            Order::Refuel { ship } => {
                let body = self.ship(*ship).and_then(|s| match s.at {
                    ShipAt::Body(b) => Some(self.tables.body(b).name.clone()),
                    _ => None,
                });
                r("refuel", &[("unit", unit_of(UnitRef::Ship(*ship))), ("body", body.unwrap_or_else(|| "space".to_string()))])
            }
            Order::BuildArchive { colony } => r("build_archive", &[("colony", place(Place::Colony(*colony)))]),
            Order::FundArchive => r("fund_archive", &[]),
            Order::Repair { unit, .. } | Order::RepairWithDucats { unit, .. } => r("repair", &[("unit", unit_of(*unit))]),
            Order::Transit { ship, to } => {
                let unit = self.ship(*ship).map(|s| s.kind.name().to_string()).unwrap_or_else(|| "Ship".into());
                r("transit", &[("unit", unit), ("body", self.tables.body(*to).name.clone())])
            }
            Order::ShipStance { body, stance } => {
                r("ship_stance", &[("body", self.tables.body(*body).name.clone()), ("stance", stance.name().to_string())])
            }
            Order::ArmyStance { place: p, stance } => r("army_stance", &[("place", place(*p)), ("stance", stance.name().to_string())]),
            Order::MoveArmy { to, .. } => r("move_army", &[("state", self.tables.state(*to).name.clone())]),
            Order::Load { colonists, from, .. } => {
                let where_ = match from {
                    LoadSource::State(s) => self.tables.state(*s).name.clone(),
                    LoadSource::Colony(c) => place(Place::Colony(*c)),
                };
                if *colonists > 0 {
                    r("load_colonists", &[("n", colonists.to_string()), ("place", where_)])
                } else {
                    r("load_army", &[("place", where_)])
                }
            }
            Order::Unload { into, .. } => {
                let where_ = match into {
                    UnloadTarget::Slot(b, i) => format!("{} slot {}", self.tables.body(*b).name, i + 1),
                    UnloadTarget::Colony(c) => place(Place::Colony(*c)),
                };
                r("unload", &[("place", where_)])
            }
            Order::Influence { target, amount } => r("influence", &[("n", amount.to_string()), ("place", place(*target))]),
            Order::BuyInfluence { amount } => r("buy_influence", &[("n", amount.to_string())]),
            Order::Buy { resource, amount } => r("buy", &[("n", amount.to_string()), ("resource", resource.name().to_string())]),
            Order::Sell { resource, amount } => r("sell", &[("n", amount.to_string()), ("resource", resource.name().to_string())]),
            Order::Relief { state } => r("relief", &[("state", self.tables.state(*state).name.clone())]),
            Order::Resettle { state } => r("resettle", &[("state", self.tables.state(*state).name.clone())]),
            Order::Change { building: b, what } => {
                let key = match what {
                    BuildingChange::Mothball => "mothball",
                    BuildingChange::Restart => "restart",
                    BuildingChange::Decommission => "decommission",
                };
                r(key, &[("building", building(*b)), ("place", place(b.place()))])
            }
            Order::BuildEmigrants { state, n } => r("build_emigrants", &[("n", n.to_string()), ("state", self.tables.state(*state).name.clone())]),
            Order::SendToAntarctica { state, n, .. } => r("send_antarctica", &[("n", n.to_string()), ("state", self.tables.state(*state).name.clone())]),
            Order::SetVentureShare { share } => r("set_venture_share", &[("share", share.to_string())]),
            Order::DrawVenture { amount } => r("draw_venture", &[("n", amount.to_string())]),
            Order::Leapfrog { state } => r("leapfrog", &[("state", self.tables.state(*state).name.clone())]),
            Order::StripPermit { state } => r("strip_permit", &[("state", self.tables.state(*state).name.clone())]),
        }
    }
}

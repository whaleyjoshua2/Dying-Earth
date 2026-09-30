//! Ticket #430 (version 0.09.5): **fog of war** -- what one seat can see of the others. Nothing
//! here is stored: every answer is worked out from the board as it stands, so a save carries no
//! fog and a loaded game is fogged exactly as the unsaved one was.
//!
//! The designer's rules, from the ticket:
//! - **A Body off Earth** is seen by a seat with any place of its own there (a Colony or a
//!   station), any Ship of its own at it, or a working Relay (or the Unique that does a Relay's
//!   job) at a Colony it directs there.
//! - **On Earth** a seat sees the Regions it directs and their neighbours; a working Embassy in a
//!   Region it directs shows the whole of Earth. Earth's orbits and Antarctica are read as a Body
//!   off Earth is.
//! - **Books** -- a rival's stockpiles, income totals and Ships in flight -- are open when the rival
//!   is Cordial or better TOWARD the viewer, or the two stand under an Accord.
//! - **Doings** -- a rival's Under way list and its Report lines -- are open when the rival is
//!   Friendly toward the viewer, or under an Accord.
//! - What STANDS at a place (who holds it, its buildings, its Colonists), Research and Victory
//!   progress, and the Moments stay open to everyone.
use crate::ids::*;
use crate::state::*;

impl Game {
    /// Is `viewer` looking at `body`? Off Earth: a place, a Ship, or a working Relay of its own
    /// there. For Earth this is the ORBITAL half -- stations over Earth and the Antarctic Colonies --
    /// read the same way; the Regions answer `sees_state`.
    pub fn sees_body(&self, viewer: Seat, body: BodyId) -> bool {
        if self.reveal_all {
            return true;
        }
        self.colonies.iter().any(|c| c.body == body && c.control.director() == Some(viewer))
            || self.ships.iter().any(|s| s.seat == viewer && s.at == ShipAt::Body(body))
            || (body != BodyId::Earth && self.has_eye(viewer, body))
    }

    /// Is `viewer` looking at this Region? One it directs, a neighbour of one it directs, or any
    /// Region at all once a working Embassy of its own stands on Earth.
    pub fn sees_state(&self, viewer: Seat, s: StateId) -> bool {
        if self.reveal_all || self.has_eye(viewer, BodyId::Earth) {
            return true;
        }
        let mine = |x: StateId| self.state(x).control.director() == Some(viewer);
        mine(s) || self.tables.state(s).neighbours.iter().any(|n| mine(*n))
    }

    /// Is `viewer` looking at this place? A Region by `sees_state`; a Colony or station by its Body.
    pub fn sees_place(&self, viewer: Seat, place: Place) -> bool {
        match place {
            Place::State(s) => self.sees_state(viewer, s),
            Place::Colony(c) => self.colony(c).is_some_and(|c| self.sees_body(viewer, c.body)),
        }
    }

    /// Are `rival`'s books open to `viewer` -- its stockpiles, income totals and Ships in flight?
    /// Its own always; a rival's when the RIVAL is Cordial or better toward the viewer (it shares),
    /// or the two stand under an Accord.
    pub fn sees_books(&self, viewer: Seat, rival: Seat) -> bool {
        self.reveal_all || viewer == rival || matches!(self.relations_level(rival, viewer), "Cordial" | "Friendly") || self.under_accord(viewer, rival)
    }

    /// Are `rival`'s doings open to `viewer` -- its Under way list and its Report lines? When the
    /// rival is Friendly toward the viewer, or under an Accord.
    pub fn sees_doings(&self, viewer: Seat, rival: Seat) -> bool {
        self.reveal_all || viewer == rival || self.relations_level(rival, viewer) == "Friendly" || self.under_accord(viewer, rival)
    }

    fn under_accord(&self, x: Seat, y: Seat) -> bool {
        self.accords.iter().any(|a| a.holds(x, y))
    }

    /// Can `viewer` see this Ship in full? Its own; a rival's at a Body it sees; a rival's in flight
    /// when the rival's books are open. A Ship at a Body it does not see is still COUNTED there
    /// (`ships_seen_as_count`), whose and how many.
    pub fn sees_ship(&self, viewer: Seat, ship: &Ship) -> bool {
        match ship.at {
            _ if ship.seat == viewer || self.reveal_all => true,
            ShipAt::Body(b) => self.sees_body(viewer, b),
            ShipAt::Transit { .. } => self.sees_books(viewer, ship.seat),
        }
    }

    /// Can `viewer` see this Army in full? Its own; one at a place it sees; one aboard a Ship it
    /// sees. An Army at a Region it does not see is counted there, whose and how many.
    pub fn sees_army(&self, viewer: Seat, army: &Army) -> bool {
        if self.reveal_all || self.army_seat(army) == Some(viewer) {
            return true;
        }
        match army.at {
            ArmyAt::Place(p) => self.sees_place(viewer, p),
            ArmyAt::Aboard(s) => self.ship(s).is_some_and(|s| self.sees_ship(viewer, s)),
        }
    }

    /// Is `viewer` looking at a Report's place? A Region or Colony as `sees_place`, a Body or one of
    /// its orbits as `sees_body`.
    pub fn sees_report_place(&self, viewer: Seat, p: crate::report::ReportPlace) -> bool {
        use crate::report::ReportPlace;
        match p {
            ReportPlace::State(s) => self.sees_state(viewer, s),
            ReportPlace::Colony(c) => self.sees_place(viewer, Place::Colony(c)),
            ReportPlace::Body(b) | ReportPlace::Orbit(b, _) => self.sees_body(viewer, b),
        }
    }

    /// Where an order was given, for the fog's reading of a rival's Report line: the Region or
    /// Colony it names, or the Body a Ship was at when it was ordered. `None` for an order with no
    /// place on the board (Influence bought, a trade, a Smear, a directive, an Accord), which a
    /// player reads only while the rival is Friendly or under an Accord.
    pub fn order_place(&self, o: &crate::orders::Order) -> Option<crate::report::ReportPlace> {
        use crate::orders::{BuildingRef, LoadSource, Order, UnloadTarget};
        use crate::report::ReportPlace;
        let at_ship = |id: ShipId| match self.ship(id)?.at {
            ShipAt::Body(b) => Some(ReportPlace::Body(b)),
            ShipAt::Transit { .. } => None,
        };
        match o {
            Order::BuildFacility { state, .. } | Order::BuildFacilityWithDucats { state, .. } | Order::RaiseIndustry { state } => Some(ReportPlace::State(*state)),
            Order::BuildEmigrants { state, .. } | Order::SendToAntarctica { state, .. } | Order::LiftToStation { state, .. } => Some(ReportPlace::State(*state)),
            Order::Relief { state } | Order::Agitate { state } | Order::Resettle { state } | Order::Leapfrog { state } | Order::StripPermit { state } | Order::ExodusCall { state } => Some(ReportPlace::State(*state)),
            Order::MoveArmy { to, .. } => Some(ReportPlace::State(*to)),
            Order::BuildModule { colony, .. } | Order::BuildModuleWithDucats { colony, .. } | Order::BuildArchive { colony } | Order::Upload { colony, .. } => Some(ReportPlace::Colony(*colony)),
            Order::BuildShip { site, .. } => Some(ReportPlace::of(*site)),
            Order::BuildArmy { place } | Order::ArmyStance { place, .. } | Order::CancelBuild { place, .. } => Some(ReportPlace::of(*place)),
            Order::Influence { target, .. } => Some(ReportPlace::of(*target)),
            Order::Launch { target, .. } => Some(ReportPlace::of(*target)),
            Order::Bombard { colony, .. } => Some(ReportPlace::Colony(*colony)),
            Order::BuildStation { body, .. } | Order::ShipStance { body, .. } => Some(ReportPlace::Body(*body)),
            Order::Transit { ship, .. } | Order::ChangeOrbit { ship, .. } | Order::Refuel { ship } | Order::Rearm { ship } => at_ship(*ship),
            Order::Load { ship, from, .. } => match from {
                LoadSource::State(s) => Some(ReportPlace::State(*s)),
                LoadSource::Colony(c) => Some(ReportPlace::Colony(*c)),
            }
            .or_else(|| at_ship(*ship)),
            Order::Unload { ship, into, .. } => match into {
                UnloadTarget::Colony(c) => Some(ReportPlace::Colony(*c)),
                UnloadTarget::Slot(b, _) => Some(ReportPlace::Body(*b)),
            }
            .or_else(|| at_ship(*ship)),
            Order::Change { building, .. } => Some(match building {
                BuildingRef::Facility(s, _) => ReportPlace::State(*s),
                BuildingRef::Module(c, _) => ReportPlace::Colony(*c),
            }),
            _ => None,
        }
    }
}

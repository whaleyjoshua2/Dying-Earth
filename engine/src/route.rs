//! Ticket #631 (version 0.1.0.6), at the designer's word: **standing ferry routes** for Colony
//! Ships. A routed Colony Ship loads at its HOME end -- the orbit it stood in when the route was
//! set -- flies to its FAR end, a Colony or station of its Faction's, unloads, and comes back,
//! refuelling at each end that is a depot. Each turn's step is a set of ordinary orders the player
//! can read and cancel, placed by the interface as Max places the Allotment; cancelling one skips
//! that turn only.

use crate::ids::*;
use crate::orders::{LoadSource, Order, UnloadTarget};
use crate::report::{LineKind, ReportPlace};
use crate::state::*;
use serde::{Deserialize, Serialize};

/// One Colony Ship's standing route.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Route {
    pub ship: ShipId,
    /// The home end: a Body and its orbit, `None` low orbit, as a Transit names one.
    pub home: BodyId,
    pub home_slot: Option<u32>,
    /// The far end: a Colony or station of the route's Faction's.
    pub far: ColonyId,
    /// Q13: it leaves only when the leg costs at most `ferry.window_slack` more than on its window.
    pub wait_window: bool,
    /// Turns it has waited at home with anyone aboard, short of full (Q5).
    #[serde(default)]
    pub waited: u32,
    /// Turns it has waited at the far end with no room there (Q15: a computer seat ends the route).
    #[serde(default)]
    pub idle: u32,
    /// The wait the Report last named, so it is said once (Q8).
    #[serde(default)]
    pub said: Option<RouteWait>,
}

impl Route {
    pub fn new(ship: ShipId, home: BodyId, home_slot: Option<u32>, far: ColonyId, wait_window: bool) -> Route {
        Route { ship, home, home_slot, far, wait_window, waited: 0, idle: 0, said: None }
    }
}

/// Q8, Q13: why a routed Ship stays where it is this turn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RouteWait {
    /// Nobody to load at home.
    Nobody,
    /// No room at the far end for those aboard.
    NoRoom,
    /// Short of Fuel or Fissionables for a leg that would not strand it.
    Fuel,
    /// Waiting for the leg's window.
    Window,
}

/// One turn of a route: the orders to place, and the wait, if it stays.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RouteStep {
    pub orders: Vec<Order>,
    pub wait: Option<RouteWait>,
}

/// Q7: what the Ship card shows beside the route button.
#[derive(Debug, Clone, PartialEq)]
pub struct RouteQuote {
    /// The out leg and the back leg, at the turn it would leave.
    pub fuel: f64,
    pub fissionables: f64,
    pub tank: f64,
    pub fission_tank: f64,
    /// Turns until it may leave for its window; `Some(0)` now, `None` not within a cycle.
    pub leaves_in: Option<u32>,
    pub home_depot: bool,
    pub far_depot: bool,
}

impl Game {
    /// The route a Ship runs, if any.
    pub fn route_of(&self, ship: ShipId) -> Option<&Route> {
        self.routes.iter().find(|r| r.ship == ship)
    }

    fn route_far(&self, r: &Route) -> Option<(BodyId, Orbit)> {
        self.colony(r.far).map(|c| (c.body, self.colony_orbit(c)))
    }

    /// Q13: whether a leg may leave on `turn` -- no window to wait for, or within the slack of it.
    pub fn route_window_ok(&self, from: Port, to: Port, turn: u32) -> bool {
        match self.crossing_ports(from, to, turn) {
            None => true,
            Some((offset, tr)) => tr.fuel_per_degree * offset.abs() <= self.tables.ferry.window_slack + 1e-9,
        }
    }

    /// Q13: the turns from `turn` until the leg's window lets it leave, over one cycle of its crossing.
    pub fn route_leaves_in(&self, from: Port, to: Port, turn: u32) -> Option<u32> {
        let Some((_, tr)) = self.crossing_ports(from, to, turn) else { return Some(0) };
        let cycle = (tr.synodic_days / tr.days_per_turn).ceil() as u32;
        (0..=cycle).find(|k| self.route_window_ok(from, to, turn + k))
    }

    /// A leg's turns and Fuel leaving at the first turn from `turn` it may, by the window rule.
    fn route_leg(&self, seat: Seat, from: Port, to: Port, turn: u32, nuclear: bool, wait_window: bool) -> (u32, f64) {
        let wait = if wait_window { self.route_leaves_in(from, to, turn).unwrap_or(0) } else { 0 };
        self.journey_cost_hull(seat, from, to, turn + wait, nuclear)
    }

    /// Q7: the round trip from the Ship's orbit to the far end and back, for the card.
    pub fn route_quote(&self, seat: Seat, ship: ShipId, far: ColonyId, wait_window: bool) -> Option<RouteQuote> {
        let s = self.ship(ship)?;
        let ShipAt::Body(body) = s.at else { return None };
        let c = self.colony(far)?;
        let (home, there) = (self.port(body, self.ship_orbit(s)), self.port(c.body, self.colony_orbit(c)));
        let leaves_in = if wait_window { self.route_leaves_in(home, there, self.turn) } else { Some(0) };
        let leave = self.turn + leaves_in.unwrap_or(0);
        let (turns, out) = self.journey_cost_hull(seat, home, there, leave, s.nuclear);
        let (_, back) = self.route_leg(seat, there, home, leave + turns + 1, s.nuclear, wait_window);
        Some(RouteQuote {
            fuel: tenth(out + back),
            fissionables: tenth(self.leg_fissionables(s, out) + self.leg_fissionables(s, back)),
            tank: self.tank_of(seat, s.kind),
            fission_tank: if s.nuclear { self.fission_tank_of(seat, s.kind) } else { 0.0 },
            leaves_in,
            home_depot: self.depot_in_orbit(seat, body, self.ship_orbit(s)).is_some(),
            far_depot: self.depot_in_orbit(seat, c.body, self.colony_orbit(c)).is_some(),
        })
    }

    /// Why a route from the Ship's orbit to `far` cannot be set, or `None` when it can (Q1, Q4, Q7).
    pub fn route_refusal(&self, seat: Seat, ship: ShipId, far: ColonyId, wait_window: bool) -> Option<String> {
        let Some(s) = self.ship(ship) else { return Some("no such Ship".into()) };
        if s.seat != seat {
            return Some("not your Ship".into());
        }
        if s.kind != UnitKind::ColonyShip {
            return Some("only a Colony Ship ferries".into());
        }
        let ShipAt::Body(body) = s.at else { return Some("in transit".into()) };
        let orbit = self.ship_orbit(s);
        let Some(c) = self.colony(far) else { return Some("no such Colony".into()) };
        if c.control.director() != Some(seat) {
            return Some("the far end must be yours".into());
        }
        if c.body == body && self.colony_orbit(c) == orbit {
            return Some("already at the far end".into());
        }
        if !self.route_home_ok(seat, body, orbit) {
            return Some(if body == BodyId::Earth { "a station ring over Earth must be yours" } else { "nothing to load here" }.into());
        }
        let q = self.route_quote(seat, ship, far, wait_window)?;
        let one = |fuel: f64, fission: f64| fuel <= q.tank + 1e-9 && fission <= q.fission_tank + 1e-9;
        let home = self.port(body, orbit);
        let there = self.port(c.body, self.colony_orbit(c));
        // Each leg at the turn it would leave: on its window, where it waits for one.
        let leave = self.turn + q.leaves_in.unwrap_or(0);
        let (turns, out) = self.journey_cost_hull(seat, home, there, leave, s.nuclear);
        // From the review: with no depot at home the first leg flies on what the tank holds now.
        if q.far_depot && !q.home_depot && (s.fuel + 1e-9 < out || s.fissionables + 1e-9 < self.leg_fissionables(s, out)) {
            return Some(format!("the tank holds {} Fuel; the first leg needs {}", figure(s.fuel), figure(out)));
        }
        match (q.home_depot, q.far_depot) {
            (false, false) => Some("no depot at either end".into()),
            (true, true) => {
                let back = self.route_leg(seat, there, home, leave + turns + 1, s.nuclear, wait_window).1;
                let leg = out.max(back);
                (!one(leg, self.leg_fissionables(s, leg))).then(|| format!("a leg needs {} Fuel, tank {}", figure(leg), figure(q.tank)))
            }
            _ => (!one(q.fuel, q.fissionables)).then(|| format!("round trip {} Fuel, tank {}", figure(q.fuel), figure(q.tank))),
        }
    }

    /// Q1: an orbit a route may call home -- over Earth, low orbit or a station's ring of the seat's
    /// own; elsewhere, an orbit that touches a Colony or station of the seat's.
    fn route_home_ok(&self, seat: Seat, body: BodyId, orbit: Orbit) -> bool {
        (body == BodyId::Earth && orbit == Orbit::Low) || !self.route_homes(seat, body, orbit).is_empty()
    }

    /// Q1: whether each end of a route fills a tank -- home, then far -- for the card.
    pub fn route_depots(&self, seat: Seat, r: &Route) -> (bool, bool) {
        let home = self.depot_in_orbit(seat, r.home, Orbit::of(r.home_slot)).is_some();
        let far = self.route_far(r).is_some_and(|(b, o)| self.depot_in_orbit(seat, b, o).is_some());
        (home, far)
    }

    /// The seat's Colonies and stations touched by this orbit, which a route loads from off Earth.
    fn route_homes(&self, seat: Seat, body: BodyId, orbit: Orbit) -> Vec<ColonyId> {
        self.colonies.iter().filter(|c| c.body == body && self.colony_orbit(c) == orbit && c.control.director() == Some(seat)).map(|c| c.id).collect()
    }

    /// Q2, Q3: the loads at home. Over Earth, Pioneers from every Region that can reach the orbit,
    /// the most waiting first; elsewhere, from each Colony or station the orbit touches, whatever
    /// stands beyond one Colonist for each Module it has built.
    fn route_loads(&self, seat: Seat, s: &Ship, body: BodyId, orbit: Orbit, pend: &mut Vec<Order>) -> u32 {
        let aboard = |pend: &[Order]| -> u32 {
            pend.iter().filter_map(|o| match o {
                Order::Load { ship, colonists, .. } if *ship == s.id => Some(*colonists),
                _ => None,
            }).sum()
        };
        let mut room = self.colony_ship_capacity(seat).saturating_sub(s.colonists + aboard(pend));
        let offers: Vec<(LoadSource, u32)> = if body == BodyId::Earth {
            let mut from: Vec<(LoadSource, u32)> = self.directed_states(seat).into_iter().map(|st| (LoadSource::State(st), self.state(st).emigrants)).filter(|(_, n)| *n > 0).collect();
            from.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
            from
        } else {
            self.route_homes(seat, body, orbit).into_iter().filter_map(|c| self.colony(c)).map(|c| (LoadSource::Colony(c.id), c.colonists.saturating_sub(c.modules.len() as u32))).filter(|(_, n)| *n > 0).collect()
        };
        for (from, n) in offers {
            if room == 0 {
                break;
            }
            let load = Order::Load { ship: s.id, colonists: n.min(room), from, army: None };
            if self.check_order(seat, pend, &load).is_ok() {
                room -= n.min(room);
                pend.push(load);
            }
        }
        aboard(pend)
    }

    /// This turn's step of the Ship's route, against the orders already placed.
    pub fn route_step(&self, seat: Seat, ship: ShipId, pending: &[Order]) -> RouteStep {
        match self.route_of(ship) {
            Some(r) => self.route_step_with(seat, r, pending),
            None => RouteStep::default(),
        }
    }

    /// The step a route would take: at the far end with people aboard, unload what there is room
    /// for; at home, load, and leave once full or after `ferry.wait_turns` turns' wait with anyone
    /// aboard; leaving either end, refuel at a depot, wait for the window if asked, and never fly
    /// a leg that would strand it. Off its route, it flies to the end it wants.
    pub fn route_step_with(&self, seat: Seat, r: &Route, pending: &[Order]) -> RouteStep {
        let mut step = RouteStep::default();
        let Some(s) = self.ship(r.ship) else { return step };
        let ShipAt::Body(body) = s.at else { return step };
        let Some((far_body, far_orbit)) = self.route_far(r) else { return step };
        let orbit = self.ship_orbit(s);
        let home_orbit = Orbit::of(r.home_slot);
        let at_home = body == r.home && orbit == home_orbit;
        let at_far = body == far_body && orbit == far_orbit;
        let mut pend: Vec<Order> = pending.to_vec();
        let refuel = |g: &Game, pend: &mut Vec<Order>| {
            let o = Order::Refuel { ship: s.id };
            if g.depot_in_orbit(seat, body, orbit).is_some() && g.check_order(seat, pend, &o).is_ok() {
                pend.push(o);
            }
        };
        if at_far && s.colonists > 0 {
            let into = UnloadTarget::Colony(r.far);
            let n = self.unload_most(s.id, into);
            if n == 0 {
                step.wait = Some(RouteWait::NoRoom);
                return step;
            }
            refuel(self, &mut pend);
            let o = Order::Unload { ship: s.id, colonists: n, army: false, into };
            if self.check_order(seat, &pend, &o).is_ok() {
                pend.push(o);
            }
            step.orders = pend.split_off(pending.len());
            return step;
        }
        let (to_body, to_orbit) = if at_home {
            (far_body, far_orbit)
        } else if at_far || s.colonists == 0 {
            (r.home, home_orbit)
        } else {
            (far_body, far_orbit)
        };
        if at_home {
            let loaded = self.route_loads(seat, s, body, orbit, &mut pend);
            let aboard = s.colonists + loaded;
            if aboard == 0 {
                step.wait = Some(RouteWait::Nobody);
                step.orders = pend.split_off(pending.len());
                return step;
            }
            if aboard < self.colony_ship_capacity(seat) && r.waited < self.tables.ferry.wait_turns {
                step.orders = pend.split_off(pending.len());
                return step;
            }
        }
        // Off its route at the Body it wants: an orbit change, not a flight.
        // From the review: refuelled first, and never stranding it either.
        if body == to_body {
            refuel(self, &mut pend);
            let cost = self.orbit_change_cost(seat, body, orbit, to_orbit);
            let back = if self.depot_in_orbit(seat, body, to_orbit).is_some() { 0.0 } else { self.orbit_change_cost(seat, body, to_orbit, orbit) };
            let short = self.tank_after_refuel(seat, &pend, s) + 1e-9 < cost + back || self.fission_after_refuel(seat, &pend, s) + 1e-9 < self.leg_fissionables(s, cost + back);
            let o = Order::ChangeOrbit { ship: s.id, slot: to_orbit.slot() };
            if short {
                step.wait = Some(RouteWait::Fuel);
            } else if self.check_order(seat, &pend, &o).is_ok() {
                pend.push(o);
            }
            step.orders = pend.split_off(pending.len());
            return step;
        }
        let (from, to) = (self.port(body, orbit), self.port(to_body, to_orbit));
        if r.wait_window && !self.route_window_ok(from, to, self.turn) {
            step.wait = Some(RouteWait::Window);
            step.orders = pend.split_off(pending.len());
            return step;
        }
        refuel(self, &mut pend);
        // Q8: never strands itself -- the leg, and the way back too where the end it flies to
        // fills no tank.
        let (turns, out) = self.journey_cost_hull(seat, from, to, self.turn, s.nuclear);
        let back = if self.depot_in_orbit(seat, to_body, to_orbit).is_some() { 0.0 } else { self.route_leg(seat, to, from, self.turn + turns + 1, s.nuclear, r.wait_window).1 };
        let short = self.tank_after_refuel(seat, &pend, s) + 1e-9 < out + back || self.fission_after_refuel(seat, &pend, s) + 1e-9 < self.leg_fissionables(s, out) + self.leg_fissionables(s, back);
        let go = Order::Transit { ship: s.id, to: to_body, slot: to_orbit.slot() };
        if short {
            // The refuel stands: a tank filled is the way out of the wait.
            step.wait = Some(RouteWait::Fuel);
        } else if self.check_order(seat, &pend, &go).is_ok() {
            pend.push(go);
        }
        step.orders = pend.split_off(pending.len());
        step
    }

    /// Q10: the route as the Ship's card and roster line read it.
    pub fn route_text(&self, r: &Route) -> String {
        let home = if r.home == BodyId::Earth { "Earth".to_string() } else { self.route_homes_name(r) };
        let far = self.colony(r.far).map(|c| self.place_name(Place::Colony(c.id))).unwrap_or_else(|| "?".into());
        format!("Ferry: {home} \u{2194} {far}")
    }

    fn route_homes_name(&self, r: &Route) -> String {
        let orbit = Orbit::of(r.home_slot);
        let seat = self.ship(r.ship).map(|s| s.seat).unwrap_or(Seat(0));
        match self.route_homes(seat, r.home, orbit).first() {
            Some(c) => self.place_name(Place::Colony(*c)),
            None => self.orbit_name(r.home, orbit),
        }
    }

    fn route_wait_line(&self, r: &Route, w: RouteWait) -> String {
        let Some(s) = self.ship(r.ship) else { return String::new() };
        let ship = self.ship_name(s);
        let far = self.colony(r.far).map(|c| self.place_name(Place::Colony(c.id))).unwrap_or_default();
        let key = match w {
            RouteWait::Nobody => "ferry_wait_nobody",
            RouteWait::NoRoom => "ferry_wait_room",
            RouteWait::Fuel => "ferry_wait_fuel",
            RouteWait::Window => "ferry_wait_window",
        };
        self.say(key, &[("ship", ship), ("colony", far)])
    }

    /// Ticket #631: run at the head of each turn, after the Income. A route whose end is lost ends;
    /// a route whose Ship is gone goes with it; the wait counters move; and a wait the Report has
    /// not yet named is named, once, to the seat that runs it.
    pub fn tend_routes(&mut self) {
        let routes = std::mem::take(&mut self.routes);
        let mut kept = Vec::new();
        for mut r in routes {
            let Some(s) = self.ship(r.ship).cloned() else { continue };
            let seat = s.seat;
            let far_ok = self.colony(r.far).is_some_and(|c| c.control.director() == Some(seat));
            let home_ok = self.route_home_ok(seat, r.home, Orbit::of(r.home_slot));
            if !far_ok || !home_ok {
                let lost = if far_ok { self.orbit_name(r.home, Orbit::of(r.home_slot)) } else { self.colony(r.far).map(|c| self.place_name(Place::Colony(c.id))).unwrap_or_else(|| "its far end".into()) };
                let text = self.say("ferry_ended", &[("ship", self.ship_name(&s)), ("place", lost)]);
                self.log(text.clone());
                if !self.seat(seat).ai {
                    self.report_line_of(seat, LineKind::Ship, LineKind::Ship, None, text);
                }
                continue;
            }
            let ShipAt::Body(body) = s.at else {
                (r.waited, r.idle) = (0, 0);
                kept.push(r);
                continue;
            };
            let orbit = self.ship_orbit(&s);
            let at_home = body == r.home && orbit == Orbit::of(r.home_slot);
            let at_far = self.route_far(&r) == Some((body, orbit));
            r.waited = if at_home && s.colonists > 0 { r.waited + 1 } else { 0 };
            r.idle = if at_far && s.colonists > 0 && self.unload_most(s.id, UnloadTarget::Colony(r.far)) == 0 { r.idle + 1 } else { 0 };
            kept.push(r);
        }
        self.routes = kept;
        for i in 0..self.routes.len() {
            let r = self.routes[i].clone();
            let Some(seat) = self.ship(r.ship).map(|s| s.seat) else { continue };
            let wait = self.route_step_with(seat, &r, &[]).wait;
            if wait.is_some() && wait != r.said {
                let text = self.route_wait_line(&r, wait.unwrap_or(RouteWait::Fuel));
                self.log(text.clone());
                if !self.seat(seat).ai {
                    let place = self.ship(r.ship).and_then(|s| match s.at {
                        ShipAt::Body(b) => Some(ReportPlace::Body(b)),
                        _ => None,
                    });
                    self.report_line_of(seat, LineKind::Ship, LineKind::Ship, place, text);
                }
            }
            self.routes[i].said = wait;
        }
    }
}

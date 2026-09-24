//! The Resolution phase, (a) through (h), in the order spec 6 gives (with 8, 9, 10).

use crate::combat::{self, Combatant, Dice};
use crate::ids::*;
use crate::orders::*;
use crate::state::*;

impl Game {
    pub fn resolution_phase(&mut self) {
        // Flags that last "until the next Resolution" clear now.
        // Ticket #54: a mothballed building is never woken by a flag clearing; only a Restart wakes it.
        for s in &mut self.states {
            for f in &mut s.facilities {
                if f.offline_until_resolution {
                    f.offline_until_resolution = false;
                    f.online = !f.mothballed;
                }
            }
        }
        for c in &mut self.colonies {
            if c.grid_failed {
                c.grid_failed = false;
                for m in &mut c.modules {
                    m.online = !m.mothballed;
                }
            }
            for m in c.modules.iter_mut().filter(|m| m.offline_until_resolution) {
                m.offline_until_resolution = false;
                m.online = !m.mothballed;
            }
        }
        self.blackout_stances();
        self.resolve_transits(); // (a)
        self.resolve_battles(); // (b)
        self.resolve_bombards(); // (b'), ticket #328: after the orbit is fought for
        self.resolve_occupation(); // (c)
        self.resolve_influence(); // (d)
        self.resolve_changes(); // (e), ticket #54: a decommission frees its slot before a build wants it
        // Ticket #99 (version 0.07.0): cargo lands BEFORE the yards finish, so winning the battle
        // for an orbit wins the turn. The other way round, a warship the loser's Shipyard completed
        // in this same Resolution denied a landing to the seat that had just won the orbit -- which
        // cost a playtested Archivist the turn they had fought three ships for.
        self.resolve_cargo(); // (g)
        // Ticket #300 (version 0.08.6): a landed Army may Attack on the turn it lands. Until this
        // version it came down after the Battles at Hold, so an invasion of a Colony was a landing
        // turn and then an Attack turn. Now the Armies that landed this turn on Attack fight a
        // second ground pass at their Colonies, and a landing that met nobody occupies at once.
        let landed = std::mem::take(&mut self.pending.landed);
        if !landed.is_empty() {
            let places: Vec<Place> = self.colonies.iter().map(|c| Place::Colony(c.id)).collect();
            self.ground_battles(places.clone(), Some(&landed));
            self.resolve_occupation_at(places);
        }
        self.resolve_builds(); // (e)
        self.resolve_repairs(); // (f)
        self.resolve_antarctic(); // (g), ticket #73: Emigrants by sea land a turn after they left
        self.apply_event_now(); // (h)
        self.apply_card_answers(); // (h), ticket #337: what each seat's answer to this turn's card does
        self.resolve_strip_permits(); // (h), ticket #54: a permit that ran out charges its price
        self.resolve_unrest(); // (i), ticket #52
        self.resolve_credits(); // (j), ticket #268: carbon credits change hands
        self.pending = Pending::default();
        for a in &mut self.armies {
            a.move_to = None;
        }
    }

    /// Comms Blackout: every stack fights as if its Stance were Hold, unless Public Science is held.
    fn blackout_stances(&mut self) {
        if self.event_is(EventId::CommsBlackout) && !self.has_tech(TechId::PublicScience) {
            for s in &mut self.ships {
                s.stance = Stance::Hold;
            }
            for a in &mut self.armies {
                a.stance = Stance::Hold;
            }
        }
    }

    // ------------------------------------------------------------------ (a)

    fn resolve_transits(&mut self) {
        // Ticket #335 (version 0.09.0): the orbit changes ordered this turn land WITH the transits,
        // before the Battles, so a Ship that changed orbit fights in its new one and an Intercept
        // below watches the orbit it moved to. The Fuel left the tank when the order was given.
        for (seat, ship, slot) in std::mem::take(&mut self.pending.orbit_changes) {
            let Some(s) = self.ship(ship).filter(|s| s.seat == seat) else { continue };
            let ShipAt::Body(body) = s.at else { continue };
            let name = self.ship_name(s);
            if let Some(s) = self.ship_mut(ship) {
                s.slot = slot;
            }
            self.war.orbit_changes[seat.index()] += 1;
            let orbit = Orbit::of(slot);
            let where_ = self.orbit_name(body, orbit);
            self.log(format!("{} moved {} to {}.", self.seat_name(seat), name, where_));
            let text = self.say("orbit_changed", &[("faction", self.seat_name(seat)), ("ship", name), ("orbit", where_)]);
            self.report_line(LineKind::Ship, Some(ReportPlace::Orbit(body, orbit)), text);
        }
        let storm = self.event_is(EventId::SolarStorm) && !self.has_tech(TechId::EfficientTransit);
        // Ticket #337 (version 0.09.0): a seat that grounded its fleet to answer this turn's card
        // holds every transit of its own -- the Solar Storm's shape, for one seat -- and a seat
        // that turned a hull aside to answer a distress call holds that one. Both are read here,
        // before the transits, which is the whole reason the card is asked before orders.
        let grounded: [bool; SEAT_COUNT] = Seat::ALL.map(|s| self.card_holds_ships(s));
        let turned_aside: Vec<ShipId> = Seat::ALL.into_iter().filter_map(|s| self.card_holds_one_ship(s)).collect();
        let mut arrivals: Vec<(Seat, BodyId, ShipId)> = Vec::new();
        for s in &mut self.ships {
            if let ShipAt::Transit { from, to, turns_left } = s.at {
                if storm || grounded[s.seat.index()] || turned_aside.contains(&s.id) {
                    continue;
                }
                let left = turns_left.saturating_sub(1);
                if left == 0 {
                    s.at = ShipAt::Body(to);
                    s.arrived_this_turn = true;
                    s.stance = Stance::Hold;
                    arrivals.push((s.seat, to, s.id));
                } else {
                    s.at = ShipAt::Transit { from, to, turns_left: left };
                }
            }
        }
        if storm && self.ships.iter().any(|s| matches!(s.at, ShipAt::Transit { .. })) {
            let text = self.say("solar_storm", &[]);
            self.report_line(LineKind::Ship, None, text);
        }
        // Ticket #86 (version 0.06.0): a crowded Colony Ship rolls for its extras once, at arrival:
        // each Colonist above the safe capacity dies with a chance of `death_chance_per_extra`
        // times the number of extras. Nothing but the Report and the Moment counts them.
        for (seat, body, id) in &arrivals {
            let Some((kind, aboard)) = self.ship(*id).map(|s| (s.kind, s.colonists)) else { continue };
            if kind != UnitKind::ColonyShip {
                continue;
            }
            let safe = self.colony_ship_capacity(*seat);
            let extras = aboard.saturating_sub(safe);
            if extras == 0 {
                continue;
            }
            let p = (self.tables.crowding.death_chance_per_extra * extras as f64).clamp(0.0, 1.0);
            let mut lost = 0u32;
            for _ in 0..extras {
                if self.rng.chance(p) {
                    lost += 1;
                }
            }
            if lost == 0 {
                continue;
            }
            // Ticket #210 (version 0.08.1): the Report names the hull, not its kind and id.
            let ship_name = self.ship(*id).map(|s| self.ship_name(s)).unwrap_or_else(|| format!("{} {}", kind.name(), id.0));
            if let Some(s) = self.ship_mut(*id) {
                s.colonists -= lost;
            }
            self.seat_mut(*seat).lost_in_transit += lost as i64;
            let line = format!("{} {}: {} of the {} crowded aboard died on the way to {}.", self.seat_name(*seat), ship_name, lost, extras, self.tables.body(*body).name);
            self.log(line.clone());
            self.report_line(LineKind::Ship, None, line);
            self.moment(
                MomentKind::LostInTransit,
                &[
                    ("faction", self.seat_name(*seat)),
                    ("ship", ship_name),
                    ("body", self.tables.body(*body).name.clone()),
                    ("n", lost.to_string()),
                    ("of", extras.to_string()),
                ],
                None,
            );
        }
        for (seat, body, id) in &arrivals {
            // Ticket #210 (version 0.08.1): the hull's name where its kind stood.
            let kind = self.ship(*id).map(|s| self.ship_name(s)).unwrap_or_else(|| "Ship".to_string());
            let line = format!("{} {} arrived at {}.", self.seat_name(*seat), kind, self.tables.body(*body).name);
            let text = self.say(
                "ship_arrived",
                &[("faction", self.seat_name(*seat)), ("ship", kind.clone()), ("body", self.tables.body(*body).name.clone())],
            );
            self.report_line(LineKind::Ship, Some(ReportPlace::Body(*body)), text);
            self.ai_deed(*seat, "arrived", &[("unit", kind), ("body", self.tables.body(*body).name.clone())]);
            self.log(line);
        }
        // Intercept battles (ticket #50): one melee per intercepting stack, against every arriving
        // enemy stack that turn. Ticket #335 (version 0.09.0): **an Intercept catches only arrivals
        // into its own orbit**, so a picket in low orbit never touches a Ship that flew straight to
        // a station's ring, and one at a station's ring never touches a landing.
        for body in BodyId::ALL {
            for orbit in self.orbits_of(body) {
                for seat in Seat::ALL {
                    let interceptors: Vec<ShipId> = self
                        .ships
                        .iter()
                        .filter(|s| s.seat == seat && self.ship_in_orbit(s, body, orbit) && s.stance == Stance::Intercept && !s.escaped)
                        .map(|s| s.id)
                        .collect();
                    if interceptors.is_empty() {
                        continue;
                    }
                    let mut parties: Vec<(Seat, bool, Vec<UnitRef>)> = vec![(seat, true, interceptors.into_iter().map(UnitRef::Ship).collect())];
                    for other in seat.others() {
                        // Ticket #320 (version 0.08.8): a partner under Passage is no target for Intercept.
                        if self.accord_has(seat, other, Term::Passage) {
                            continue;
                        }
                        let arriving: Vec<UnitRef> = arrivals
                            .iter()
                            .filter(|(s, b, id)| *s == other && *b == body && self.ship(*id).map(|x| !x.escaped && self.ship_orbit(x) == orbit).unwrap_or(false))
                            .map(|(_, _, id)| UnitRef::Ship(*id))
                            .collect();
                        if !arriving.is_empty() {
                            parties.push((other, false, arriving));
                        }
                    }
                    if parties.len() < 2 {
                        continue;
                    }
                    let name = format!("{} (interception)", self.orbit_battle_name(body, orbit));
                    self.ship_melee(&name, body, orbit, &parties);
                    // Ticket #319 (version 0.08.8): counted, so the sweep can say whether Intercept fires.
                    self.war.interceptions[seat.index()] += 1;
                }
            }
        }
    }

    // ------------------------------------------------------------------ (b)

    fn resolve_battles(&mut self) {
        // Ship battles (ticket #50): any stack ordered Attack pulls every other Faction's Ships at
        // that Body into one melee. Evade stacks still try to disengage; Hold stacks fight.
        // Ticket #335 (version 0.09.0): **battle parties form per ORBIT.** An Attack fights the
        // orbit the stack sits in, so two Battles at one Body are two melees and two records, and a
        // station's Battery never fires on a fight in low orbit.
        for body in BodyId::ALL {
            for orbit in self.orbits_of(body) {
                let aggressors: Vec<Seat> = Seat::ALL
                    .into_iter()
                    .filter(|seat| self.ships.iter().any(|s| s.seat == *seat && self.ship_in_orbit(s, body, orbit) && s.stance == Stance::Attack && !s.escaped))
                    .collect();
                if aggressors.is_empty() {
                    continue;
                }
                // Ticket #324 (version 0.08.8): a seat's working Batteries stand in its line after
                // its Ships, so a stack on Attack in an orbit with nothing but a Battery in it
                // fights the Battery, and a Battery beside its owner's stack fights with it.
                // Ticket #335: the Batteries of THIS orbit -- a station's its own ring, a ground
                // Colony's low orbit.
                let parties: Vec<(Seat, bool, Vec<UnitRef>)> = Seat::ALL
                    .into_iter()
                    .filter_map(|seat| {
                        let units: Vec<UnitRef> = self
                            .ships
                            .iter()
                            .filter(|s| s.seat == seat && self.ship_in_orbit(s, body, orbit) && !s.escaped)
                            .map(|s| UnitRef::Ship(s.id))
                            .chain(self.batteries_at(seat, body, orbit).into_iter().map(|(colony, index)| UnitRef::Battery { colony, index }))
                            .collect();
                        if units.is_empty() {
                            None
                        } else {
                            Some((seat, aggressors.contains(&seat), units))
                        }
                    })
                    .collect();
                if parties.len() < 2 {
                    continue;
                }
                let name = self.orbit_battle_name(body, orbit);
                self.ship_melee(&name, body, orbit, &parties);
            }
        }
        // Army moves and attacks on Earth.
        let moving: Vec<(ArmyId, StateId)> = self.armies.iter().filter_map(|a| a.move_to.map(|t| (a.id, t))).collect();
        for (id, to) in moving {
            let Some(a) = self.army(id) else { continue };
            let Some(seat) = self.army_seat(a) else { continue };
            if a.escaped || self.army_stands_down(a) {
                continue;
            }
            let entering_own = self.state(to).control == Control::Controlled(seat);
            let from = match a.at {
                ArmyAt::Place(Place::State(s)) => s,
                _ => continue,
            };
            // Ticket #320 (version 0.08.8): PASSAGE. A march into a Region held by a partner under
            // a Passage Accord is not an attack: the Army arrives on Hold, fights nobody and is
            // counted as no march on a held Region, at the designer's word. Without Passage a
            // march into a held Region is what it always was.
            let passage = matches!(self.state(to).control, Control::Controlled(h) if h != seat && self.accord_has(seat, h, Term::Passage));
            let a = self.army_mut(id).unwrap();
            a.at = ArmyAt::Place(Place::State(to));
            a.move_to = None;
            if passage {
                a.stance = Stance::Hold;
            } else if !entering_own {
                a.stance = Stance::Attack;
            }
            // Ticket #286 (version 0.08.5): a march on a neutral, or on a Region a rival holds.
            match self.state(to).control {
                Control::Neutral => self.war.marches_neutral[seat.index()] += 1,
                Control::Controlled(r) if r != seat && !passage => self.war.marches_held[seat.index()] += 1,
                _ => {}
            }
            // Ticket #339 (version 0.09.0): the march line names the Army, as the destruction line
            // has since #281 -- read AFTER the move, since the Army is the same Army either way.
            let name = self.army(id).map(|a| self.army_name(a)).unwrap_or_else(|| "the Army".to_string());
            let line = format!(
                "{} ({}) moved from {} to {}{}.",
                name,
                self.seat_name(seat),
                self.tables.state(from).name,
                self.tables.state(to).name,
                if entering_own { "" } else { " and attacks" }
            );
            self.log(line);
            let text = self.say(
                "army_moved",
                &[
                    ("army", name),
                    ("faction", self.seat_name(seat)),
                    ("from", self.tables.state(from).name.clone()),
                    ("to", self.tables.state(to).name.clone()),
                    ("attacks", if entering_own { String::new() } else { self.phrase("attacks", &[]) }),
                ],
            );
            self.report_line(LineKind::Army, Some(ReportPlace::State(to)), text);
        }
        // Ground battles (ticket #50): an attacking Army fights every other Faction's Armies at the
        // place, and two Factions attacking the same place the same turn make one melee of all parties.
        let mut places: Vec<Place> = StateId::ALL.into_iter().map(Place::State).collect();
        places.extend(self.colonies.iter().map(|c| Place::Colony(c.id)));
        self.ground_battles(places, None);
    }

    /// The ground melees at `places`. Ticket #300 (version 0.08.6): called a second time after
    /// cargo has landed, with `only_landed` naming the Armies that came down this turn, so a
    /// landed Army fights the turn it lands; the orbit is fought first, at (b), and the ground is
    /// reached by whoever won it. Only a place where one of those Armies stands on Attack fights
    /// in the second pass, so nothing already fought at (b) is fought twice.
    fn ground_battles(&mut self, places: Vec<Place>, only_landed: Option<&[ArmyId]>) {
        // Ticket #320 (version 0.08.8): a rival Army standing in a held Region with no Passage
        // behind it is on Attack, as any rival Army in a Region not its seat's would be -- the
        // guest whose Accord ended, at the designer's word. A guest under Passage stands as it is.
        if only_landed.is_none() {
            let turned: Vec<ArmyId> = self
                .armies
                .iter()
                .filter(|a| {
                    matches!(a.stance, Stance::Hold | Stance::DigIn)
                        && matches!(a.at, ArmyAt::Place(Place::State(s)) if matches!(self.state(s).control, Control::Controlled(h) if self.army_seat(a).is_some_and(|seat| seat != h && !self.accord_has(seat, h, Term::Passage))))
                })
                .map(|a| a.id)
                .collect();
            for id in turned {
                if let Some(a) = self.army_mut(id) {
                    a.stance = Stance::Attack;
                }
            }
        }
        for place in places {
            if let Some(landed) = only_landed
                && !self.armies.iter().any(|a| landed.contains(&a.id) && a.at == ArmyAt::Place(place) && a.stance == Stance::Attack && !a.escaped)
            {
                continue;
            }
            let aggressors: Vec<Seat> = Seat::ALL
                .into_iter()
                .filter(|seat| {
                    self.place_director(place) != Some(*seat)
                        && self.armies.iter().any(|a| {
                            a.at == ArmyAt::Place(place)
                                && self.army_seat(a) == Some(*seat)
                                && a.stance == Stance::Attack
                                && !a.escaped
                                && !self.army_stands_down(a)
                        })
                })
                .collect();
            if aggressors.is_empty() {
                continue;
            }
            let mut parties: Vec<(Option<Seat>, bool, Vec<ArmyId>)> = Vec::new();
            for seat in &aggressors {
                let mine: Vec<ArmyId> = self
                    .armies
                    .iter()
                    .filter(|a| {
                        a.at == ArmyAt::Place(place) && self.army_seat(a) == Some(*seat) && a.stance == Stance::Attack && !a.escaped && !self.army_stands_down(a)
                    })
                    .map(|a| a.id)
                    .collect();
                parties.push((Some(*seat), true, mine));
            }
            // Everyone else at the place defends: the other Factions' Armies and the place's own.
            for owner in Seat::ALL.into_iter().map(Some).chain(std::iter::once(None)) {
                if owner.map(|s| aggressors.contains(&s)).unwrap_or(false) {
                    continue;
                }
                let theirs: Vec<ArmyId> = self
                    .armies
                    .iter()
                    .filter(|a| a.at == ArmyAt::Place(place) && self.army_seat(a) == owner && !self.army_stands_down(a) && !a.escaped)
                    .filter(|a| self.army_strength(a) > 0 || !a.standing)
                    .map(|a| a.id)
                    .collect();
                if !theirs.is_empty() {
                    parties.push((owner, false, theirs));
                }
            }
            if parties.len() < 2 {
                continue;
            }
            let name = self.place_name(place);
            self.army_melee(&name, place, &aggressors, &parties);
            // Ticket #298 (version 0.08.6): the Battle itself burns nothing. Until this version a
            // quarter of every building at the place rolled after every ground Battle, win or lose,
            // and again on the transfer, so force burned twice where Influence never did; at the
            // designer's word only a place that transfers by the three-turn clock rolls now.
            // Ticket #282 (version 0.08.5): a neutral Region attacked that still has a defender
            // standing, unescaped, has held, and arms for good: +1 to its Standing Army, to
            // Industry + 4, at the designer's word.
            if let Place::State(sid) = place
                && self.state(sid).control == Control::Neutral
                && self.armies.iter().any(|a| a.at == ArmyAt::Place(place) && self.army_seat(a).is_none() && !a.escaped && self.army_strength(a) > 0)
            {
                self.neutral_held(sid);
            }
        }
    }

    /// Ticket #282: the step a neutral Region earns by holding, and the Report line that says so.
    /// Ticket #302 (version 0.08.6): no ceiling, and the step is the table's.
    pub fn neutral_held(&mut self, sid: StateId) {
        self.neutral_holds += 1;
        self.state_mut(sid).armed += self.tables.standing_army.held_step;
        let (state, n) = (self.tables.state(sid).name.clone(), self.standing_army_cap(sid));
        self.log(format!("{state} held against the attack and arms: its Standing Army will stand at {n}."));
        let text = self.say("neutral_held", &[("state", state), ("n", n.to_string())]);
        self.report_line(LineKind::Army, Some(ReportPlace::State(sid)), text);
    }

    /// Ticket #279 (version 0.08.5): Battles pollute. Whether a place is on Earth -- a Region, Earth
    /// orbit, or a Colony on Earth (Antarctica) -- since Mars orbit fouls nobody's air.
    fn on_earth_place(&self, place: Place) -> bool {
        match place {
            Place::State(_) => true,
            Place::Colony(c) => self.colony(c).is_some_and(|c| c.body == BodyId::Earth),
        }
    }

    /// Ticket #279: so many ppm into next Climate phase's war bucket, worn by `seat` -- or by nobody,
    /// for a neutral Region's own Army -- and only for a Battle on Earth.
    fn charge_war(&mut self, on_earth: bool, seat: Option<Seat>, ppm: f64) {
        if !on_earth || ppm <= 0.0 {
            return;
        }
        match seat {
            Some(s) => self.climate.war_next[s.index()] += ppm,
            None => self.climate.war_next_nobody += ppm,
        }
    }

    /// Ticket #279: every party's hits in a Battle, charged by the table's rate per hit.
    fn charge_war_hits(&mut self, on_earth: bool, line: &BattleLine) {
        let per_hit = self.tables.climate.war_ppm_per_hit;
        for p in &line.parties {
            self.charge_war(on_earth, p.seat, p.hits as f64 * per_hit);
        }
    }

    pub fn place_director(&self, place: Place) -> Option<Seat> {
        match place {
            Place::State(s) => self.state(s).control.director(),
            Place::Colony(c) => self.colony(c).and_then(|c| c.control.director()),
        }
    }

    /// Ticket #318 (version 0.08.8): the presence an Occupation needs, to begin and to hold alike:
    /// an Army of the seat at the place that did not escape and does not stand down. Until this
    /// ticket the holding check read every Army of the seat there, escaped or not, so an Army that
    /// ran kept an Occupation it could not have begun until the next Resolution cleared the flag.
    pub fn present_at(&self, place: Place, seat: Seat) -> bool {
        self.armies.iter().any(|a| a.at == ArmyAt::Place(place) && self.army_seat(a) == Some(seat) && !a.escaped && !self.army_stands_down(a))
    }

    /// Ticket #284 (version 0.08.5): a seat is alone at a place when it has an Army there on Attack
    /// that did not escape, and no defender is left engaged. Read by the Battle line and by the
    /// Occupation alike, so the two can never disagree about `escaped` again.
    pub fn alone_at(&self, place: Place, seat: Seat) -> bool {
        let attacking = self.armies.iter().any(|a| a.at == ArmyAt::Place(place) && self.army_seat(a) == Some(seat) && a.stance == Stance::Attack && !a.escaped);
        attacking && self.defenders_at(place, seat).is_empty()
    }

    /// Armies at a place that fight against `attacker`: every Army not of that seat, not standing down, not escaped.
    pub fn defenders_at(&self, place: Place, attacker: Seat) -> Vec<ArmyId> {
        self.armies
            .iter()
            .filter(|a| a.at == ArmyAt::Place(place) && self.army_seat(a) != Some(attacker) && !self.army_stands_down(a) && !a.escaped)
            .filter(|a| self.army_strength(a) > 0 || !a.standing)
            // Ticket #320 (version 0.08.8): a guest under Passage defends nothing.
            .filter(|a| !self.guest_at(a))
            .map(|a| a.id)
            .collect()
    }

    /// Ticket #320 (version 0.08.8): a guest is an Army standing in a Region held by another seat
    /// under a Passage Accord between them, not on Attack. It fights nobody and defends nothing
    /// while the Accord stands: it is no party to a Battle at the place and no defender of it.
    pub fn guest_at(&self, a: &Army) -> bool {
        a.stance != Stance::Attack
            && matches!(a.at, ArmyAt::Place(Place::State(s)) if matches!(self.state(s).control, Control::Controlled(h) if self.army_seat(a).is_some_and(|seat| seat != h && self.accord_has(seat, h, Term::Passage))))
    }

    /// Ticket #324 (version 0.08.8): a unit of an orbital line, Ship or Battery, as a combatant.
    fn unit_combatant(&self, u: UnitRef) -> Combatant {
        match u {
            UnitRef::Ship(id) => self.ship_combatant(id),
            UnitRef::Army(id) => self.army_combatant(id, false),
            UnitRef::Battery { colony, index } => self.battery_combatant(colony, index),
        }
    }

    /// Ticket #324 (version 0.08.8): a Battery in the line: its card's strength and hit points, its
    /// own damage, no pursuit, and never a roll to disengage, since it cannot leave -- the flag Dig
    /// In sets on an Army (#297) says exactly that. Hardened Hulls does not reach it.
    fn battery_combatant(&self, colony: ColonyId, index: usize) -> Combatant {
        let card = self.tables.module(ModuleKind::Battery);
        let damage = self.colony(colony).and_then(|c| c.modules.get(index)).map(|m| m.damage).unwrap_or(0);
        let name = format!("the Battery at {}", self.place_name(Place::Colony(colony)));
        Combatant::new(UnitRef::Battery { colony, index }, name, card.strength, card.hit_points, damage, 0, false).dug_in(true)
    }

    fn ship_combatant(&self, id: ShipId) -> Combatant {
        let s = self.ship(id).unwrap();
        let card = self.tables.unit(s.kind);
        // Ticket #281 (version 0.08.5): by name, as every other surface has it since 0.08.1.
        // Ticket #326 (version 0.08.8): a Colony Ship or a Carrier is unarmed, and its escorts take
        // the fire while any stands engaged.
        Combatant::new(UnitRef::Ship(id), self.ship_name(s), self.ship_strength(s), card.hit_points, s.damage, card.pursuit, s.stance == Stance::Evade).armed(s.kind.is_warship())
    }

    /// `defending`: the Army is not on the aggressor's side of this melee. Ticket #302 (version
    /// 0.08.6): a defender fights at its strength plus its defence -- a Region's own Army's
    /// Constabulary and calm -- plus Dig In's term if dug in; an aggressor at its strength alone.
    fn army_combatant(&self, id: ArmyId, defending: bool) -> Combatant {
        let a = self.army(id).unwrap();
        let card = self.tables.unit(UnitKind::Army);
        // Ticket #281 (version 0.08.5): by name -- the 1st Chinese Army -- as every other surface has
        // it since 0.08.4; a neutral Region's own Army is named the same way.
        let name = self.army_name(a);
        // Ticket #296 (version 0.08.6): hit points are the Army's own, not the card's. Ticket #297:
        // dug in, it never rolls to disengage.
        let dug_in = self.army_dug_in(a);
        let strength = if defending { self.army_defended_strength(a) } else { self.army_strength(a) };
        Combatant::new(UnitRef::Army(id), name, strength, self.army_hit_points(a), a.damage, card.pursuit, a.stance == Stance::Evade).dug_in(dug_in)
    }

    // ------------------------------------------------------------------ ticket #339: the odds

    /// Ticket #339 (version 0.09.0): **the chance of winning the whole Battle** if `attackers`
    /// attack `place` -- the figure the attack button and the Battle Report should carry, where
    /// both carried the first exchange's share of strength before. Measured over
    /// `[melee] odds_trials` copies of the fight from the figure's own seed, so it is the same
    /// every time it is asked and asking it never touches the game's dice. 1.0 where nobody
    /// defends: the field is taken by walking on to it.
    ///
    /// The defenders are grouped by whose they are, exactly as `ground_battles` groups them, since
    /// a melee spreads its hits across the parties and two parties of one Army each are not one
    /// party of two.
    pub fn ground_battle_odds(&self, place: Place, seat: Seat, attackers: &[ArmyId]) -> f64 {
        let mine: Vec<Combatant> = attackers.iter().filter(|id| self.army(**id).is_some()).map(|id| self.army_combatant(*id, false)).collect();
        if mine.is_empty() {
            return 0.0;
        }
        let defenders = self.defenders_at(place, seat);
        let mut parties = vec![mine];
        for owner in Seat::ALL.into_iter().map(Some).chain(std::iter::once(None)) {
            if owner == Some(seat) {
                continue;
            }
            let theirs: Vec<Combatant> = defenders
                .iter()
                .filter(|id| self.army(**id).map(|a| self.army_seat(a) == owner).unwrap_or(false))
                .map(|id| self.army_combatant(*id, true))
                .collect();
            if !theirs.is_empty() {
                parties.push(theirs);
            }
        }
        if parties.len() < 2 {
            return 1.0;
        }
        self.whole_battle_odds(&parties, self.tables.melee.rolls)
    }

    /// Ticket #339 (version 0.09.0): the same figure for one ORBIT of a Body -- the Ships of the
    /// seat and its working Batteries there against every other Faction's, one party each, and the
    /// hit rolls a Ship melee's: one a round for every engaged armed unit, never fewer than the
    /// table's figure (ticket #327).
    pub fn orbit_battle_odds(&self, body: BodyId, orbit: Orbit, seat: Seat) -> f64 {
        let line = |s: Seat| -> Vec<Combatant> {
            self.ships
                .iter()
                .filter(|x| x.seat == s && self.ship_in_orbit(x, body, orbit) && !x.escaped)
                .map(|x| UnitRef::Ship(x.id))
                .chain(self.batteries_at(s, body, orbit).into_iter().map(|(colony, index)| UnitRef::Battery { colony, index }))
                .map(|u| self.unit_combatant(u))
                .collect()
        };
        let mine = line(seat);
        if mine.is_empty() {
            return 0.0;
        }
        let mut parties = vec![mine];
        for other in seat.others() {
            let theirs = line(other);
            if !theirs.is_empty() {
                parties.push(theirs);
            }
        }
        if parties.len() < 2 {
            return 1.0;
        }
        let armed = parties.iter().flatten().filter(|c| c.armed && c.engaged && !c.destroyed()).count() as u32;
        self.whole_battle_odds(&parties, armed.max(self.tables.melee.rolls))
    }

    /// Ticket #339: the trials, the rounds, the disengage divisor and the seed, all from the data,
    /// so the two doors above cannot drift apart. Party 0 is always the one asking.
    fn whole_battle_odds(&self, parties: &[Vec<Combatant>], rolls: u32) -> f64 {
        let m = &self.tables.melee;
        combat::whole_battle_odds(parties, 0, self.tables.disengage.divisor, m.rounds, rolls, m.odds_trials, m.odds_seed)
    }

    /// One melee of Ship stacks in one ORBIT of a Body (ticket #50; ticket #335, version 0.09.0,
    /// which made the orbit the place a Battle is fought and recorded in).
    fn ship_melee(&mut self, place: &str, body: BodyId, orbit: Orbit, parties: &[(Seat, bool, Vec<UnitRef>)]) {
        // Ticket #191 (version 0.08.0): opening a Battle offends everyone on the other side of it.
        for (aggressor, _, _) in parties.iter().filter(|(_, agg, _)| *agg) {
            for (other, _, _) in parties {
                // Ticket #222 (version 0.08.2): opening a Battle is rung 3.
                self.offend_by(*aggressor, *other, 3);
            }
        }
        let units: Vec<(Option<Seat>, bool, Vec<Combatant>)> =
            parties.iter().map(|(seat, agg, ids)| (Some(*seat), *agg, ids.iter().map(|u| self.unit_combatant(*u)).collect())).collect();
        // Ticket #327 (version 0.08.8): one hit roll a round for every engaged armed unit present,
        // never fewer than the table's figure.
        let armed = units.iter().flat_map(|(_, _, c)| c.iter()).filter(|c| c.armed && c.engaged && !c.destroyed()).count() as u32;
        let rolls = armed.max(self.tables.melee.rolls);
        let mut line = self.run_melee(place, Some(ReportPlace::Orbit(body, orbit)), units, rolls);
        // Ticket #286 (version 0.08.5): counted by the seat that opened it.
        for (seat, _, _) in parties.iter().filter(|(_, agg, _)| *agg) {
            self.war.battles[seat.index()] += 1;
            self.war.orbit_attacks[seat.index()] += 1;
        }
        // Ticket #335 (version 0.09.0): Orbital Control is LOW ORBIT's, so only a Battle fought
        // there can have moved it; a fight at a station's ring says nothing about the ground.
        if orbit.is_low() {
            match self.orbital_control(body) {
                Some(s) if parties.iter().any(|(seat, agg, _)| *agg && *seat == s) => {
                    line.result.push_str(&format!(" The {} hold Orbital Control.", self.seat_name(s)))
                }
                Some(s) => line.result.push_str(&format!(" The {} keep Orbital Control.", self.seat_name(s))),
                // Ticket #324: and why not, when a Battery is the reason.
                None if Seat::ALL.iter().any(|s| !self.batteries_at(*s, body, Orbit::Low).is_empty()) => {
                    line.result.push_str(" Nobody holds Orbital Control: a Battery stands.")
                }
                None => line.result.push_str(" Nobody holds Orbital Control."),
            }
        }
        self.charge_war_hits(body == BodyId::Earth, &line);
        self.log(line.text(&|s| self.seat_name(s), "neutral"));
        self.report.battles.push(line);
    }

    /// One melee of Armies at a ground place (ticket #50).
    fn army_melee(&mut self, place_name: &str, place: Place, aggressors: &[Seat], parties: &[(Option<Seat>, bool, Vec<ArmyId>)]) {
        // Ticket #191: as in orbit. A neutral state's own Armies are nobody's Faction, so a Battle
        // against them offends nobody -- which is most Battles: 46 of the 55 measured over 80 games.
        for aggressor in aggressors {
            for other in parties.iter().filter_map(|(s, _, _)| *s) {
                // Ticket #222: rung 3, as above.
                self.offend_by(*aggressor, other, 3);
            }
        }
        let units: Vec<(Option<Seat>, bool, Vec<Combatant>)> =
            parties.iter().map(|(seat, agg, ids)| (*seat, *agg, ids.iter().map(|id| self.army_combatant(*id, !*agg)).collect())).collect();
        let rolls = self.tables.melee.rolls;
        let mut line = self.run_melee(place_name, Some(place.into()), units, rolls);
        // Ticket #286 (version 0.08.5): counted by the seat that opened it, and against a neutral.
        for seat in aggressors {
            self.war.battles[seat.index()] += 1;
        }
        if parties.iter().any(|(seat, _, _)| seat.is_none()) {
            self.war.battles_vs_neutral += 1;
        }
        self.charge_war_hits(self.on_earth_place(place), &line);
        // Ticket #284 (version 0.08.5): the same predicate `resolve_occupation` reads, so the line
        // never promises an Occupation an escaped attacker will not begin (a measured defect).
        for seat in aggressors {
            if self.alone_at(place, *seat) {
                line.result.push_str(&format!(" The {} are alone at the place; Occupation begins.", self.seat_name(*seat)));
            }
        }
        self.log(line.text(&|s| self.seat_name(s), "neutral"));
        self.report.battles.push(line);
    }

    /// Ticket #281 (version 0.08.5): `at` is the real place, for the Report's line to jump to; the
    /// party text names every unit and what it took; an aggressor carries the first-round odds it
    /// faced, as the attack button quoted them.
    fn run_melee(&mut self, place: &str, at: Option<ReportPlace>, parties: Vec<(Option<Seat>, bool, Vec<Combatant>)>, rolls: u32) -> BattleLine {
        let mut parties = parties;
        let before: Vec<Vec<u32>> = parties.iter().map(|(_, _, c)| c.iter().map(|x| x.damage).collect()).collect();
        let strengths: Vec<i64> = parties.iter().map(|(_, _, c)| c.iter().map(|x| x.strength).sum()).collect();
        let total: i64 = strengths.iter().sum();
        let stats = {
            let mut slices: Vec<&mut [Combatant]> = parties.iter_mut().map(|(_, _, c)| c.as_mut_slice()).collect();
            let mut rng = self.rng.clone();
            let stats = combat::melee(&mut slices, &mut rng as &mut dyn Dice, self.tables.disengage.divisor, self.tables.melee.rounds, rolls);
            self.rng = rng;
            stats
        };
        // What each unit took, by name: "TSV Valiant took 2 hits; PMV Aurora escaped".
        let describe = |side: &[Combatant], before: &[u32]| -> String {
            side.iter()
                .zip(before)
                .map(|(c, b)| {
                    let took = c.damage.saturating_sub(*b);
                    let hits = format!("{took} hit{}", if took == 1 { "" } else { "s" });
                    if c.destroyed() {
                        format!("{} destroyed", c.name)
                    } else if c.escaped {
                        format!("{} escaped{}", c.name, if took > 0 { format!(" after {hits}") } else { String::new() })
                    } else {
                        format!("{} took {hits}", c.name)
                    }
                })
                .collect::<Vec<_>>()
                .join("; ")
        };
        let listed: Vec<BattleParty> = parties
            .iter()
            .enumerate()
            .map(|(i, (seat, agg, c))| BattleParty {
                seat: *seat,
                aggressor: *agg,
                units: describe(c, &before[i]),
                strength: strengths[i],
                hits: stats.hits_of(i),
                destroyed: stats.destroyed.get(i).cloned().unwrap_or_default(),
                escaped: stats.escaped.get(i).cloned().unwrap_or_default(),
                odds: if *agg { Some(combat::first_round_odds(strengths[i], total - strengths[i])) } else { None },
            })
            .collect();
        // Ticket #295 (version 0.08.6): escapes counted at the event, by the seat the unit fought
        // for, so the sweep can say what the disengage figure does.
        let mut any_escape = false;
        for (i, (seat, _, _)) in parties.iter().enumerate() {
            let n = stats.escaped.get(i).map(|e| e.len() as u32).unwrap_or(0);
            if n == 0 {
                continue;
            }
            any_escape = true;
            match seat {
                Some(s) => self.war.escapes[s.index()] += n,
                None => self.war.escapes_neutral += n,
            }
        }
        if any_escape {
            self.war.battles_with_escape += 1;
        }
        let line = BattleLine { place: place.to_string(), parties: listed, result: format!("{} round(s).", stats.rounds), at };
        // The Battle's own line goes in BEFORE the losses are applied, so among the rank-4 lines a
        // turn holds it is the earliest and headlines over "PMV Magellan destroyed (battle)".
        self.battle_line_and_moment(&line);
        for (_, _, c) in &parties {
            self.apply_combatants(c, at);
        }
        line
    }

    /// Ticket #281 (version 0.08.5): every Battle is a line of the Report at its place -- ranked
    /// with a Ship destroyed when a unit died, unranked when nobody lost one -- and a Moment when a
    /// unit died, naming it. Until now a Battle was a block at the foot of the dispatch and nothing
    /// else: no headline, and no Moment unless a building burned.
    fn battle_line_and_moment(&mut self, line: &BattleLine) {
        let lost: Vec<String> = line.parties.iter().flat_map(|p| p.destroyed.iter().cloned()).collect();
        let aggressors: Vec<String> = line.parties.iter().filter(|p| p.aggressor).map(|p| p.seat.map(|s| self.seat_name(s)).unwrap_or_else(|| "neutral".to_string())).collect();
        let odds = line.parties.iter().find(|p| p.aggressor).and_then(|p| p.odds).unwrap_or(0.0);
        let outcome = if lost.is_empty() { "nobody lost a unit".to_string() } else { format!("{} destroyed", Game::and_list(&lost)) };
        let text = self.say(
            "battle",
            &[("place", line.place.clone()), ("faction", aggressors.join(" and the ")), ("odds", format!("{:.0}", odds * 100.0)), ("outcome", outcome.clone())],
        );
        let kind = if lost.is_empty() { LineKind::Battle } else { LineKind::DecisiveBattle };
        self.report_line(kind, line.at, text);
        if !lost.is_empty() {
            self.moment(MomentKind::DecisiveBattle, &[("place", line.place.clone()), ("result", outcome), ("figure", format!("{} lost", lost.len()))], line.at);
        }
    }

    fn apply_combatants(&mut self, side: &[Combatant], at: Option<ReportPlace>) {
        // Ticket #324 (version 0.08.8): Batteries shot to nothing, removed after the loop from the
        // highest index down, so an earlier removal does not shift a later one's index.
        let mut fallen: Vec<(ColonyId, usize)> = Vec::new();
        for c in side {
            match c.unit {
                UnitRef::Ship(id) => {
                    if c.destroyed() {
                        self.destroy_ship(id, "battle");
                    } else if let Some(s) = self.ship_mut(id) {
                        s.damage = c.damage;
                        s.escaped = c.escaped;
                    }
                }
                UnitRef::Army(id) => {
                    if c.destroyed() {
                        self.destroy_army(id, "battle", at);
                    } else if let Some(a) = self.army_mut(id) {
                        a.damage = c.damage;
                        a.escaped = c.escaped;
                    }
                }
                UnitRef::Battery { colony, index } => {
                    if let Some(m) = self.colony_mut(colony).and_then(|col| col.modules.get_mut(index)) {
                        m.damage = c.damage;
                    }
                    if c.destroyed() {
                        fallen.push((colony, index));
                    }
                }
            }
        }
        fallen.sort_by(|a, b| b.cmp(a));
        for (colony, index) in fallen {
            self.destroy_battery(colony, index);
        }
    }

    /// Ticket #324 (version 0.08.8): a Battery destroyed in a Battle is gone from its Colony,
    /// counted against the seat that held it, and said in the log.
    pub fn destroy_battery(&mut self, colony: ColonyId, index: usize) {
        let Some(col) = self.colony_mut(colony) else { return };
        if col.modules.get(index).map(|m| m.kind) != Some(ModuleKind::Battery) {
            return;
        }
        let holder = col.control.director();
        col.modules.remove(index);
        if let Some(h) = holder {
            self.war.batteries_lost[h.index()] += 1;
        }
        let whose = holder.map(|h| self.seat_name(h)).unwrap_or_else(|| "Nobody's".to_string());
        self.log(format!("{whose} Battery at {} destroyed (battle).", self.place_name(Place::Colony(colony))));
    }

    pub fn destroy_ship(&mut self, id: ShipId, why: &str) {
        let Some(pos) = self.ships.iter().position(|s| s.id == id) else { return };
        let ship = self.ships.remove(pos);
        if ship.kind.is_warship() {
            self.war.warships_lost[ship.seat.index()] += 1;
        }
        if let Some(a) = ship.army {
            let at = match ship.at {
                ShipAt::Body(b) => Some(ReportPlace::Body(b)),
                _ => None,
            };
            self.destroy_army(a, "lost with its Carrier", at);
        }
        let line = format!(
            "{} {} destroyed ({}){}.",
            self.seat_name(ship.seat),
            ship.kind.name(),
            why,
            if ship.colonists > 0 { format!(" with {} Colonists aboard", ship.colonists) } else { String::new() }
        );
        self.log(line);
        let cargo = if ship.colonists > 0 { self.phrase("cargo_aboard", &[("n", ship.colonists.to_string())]) } else { String::new() };
        let text = self.say(
            "ship_destroyed",
            &[("faction", self.seat_name(ship.seat)), ("ship", self.ship_name(&ship)), ("why", why.to_string()), ("cargo", cargo)],
        );
        let place = match ship.at {
            ShipAt::Body(b) => Some(ReportPlace::Body(b)),
            _ => None,
        };
        self.report_line(LineKind::DecisiveBattle, place, text);
    }

    /// Ticket #281 (version 0.08.5): an Army destroyed is a line of the Report by name, as a Ship
    /// has been since 0.08.1; from ticket #50 to here it left no trace but the Battle block's list.
    pub fn destroy_army(&mut self, id: ArmyId, why: &str, at: Option<ReportPlace>) {
        let Some(pos) = self.armies.iter().position(|a| a.id == id) else { return };
        let army = self.armies.remove(pos);
        // Ticket #286 (version 0.08.5): counted for the sweep, by the seat it fought for.
        match (self.army_seat(&army), army.standing) {
            (Some(s), false) => self.war.armies_lost[s.index()] += 1,
            (_, true) if !army.levy => self.war.standing_armies_lost += 1,
            _ => {}
        }
        // Ticket #282 (version 0.08.5): THE Standing Army returns two Incomes later, not the next.
        // Ticket #318 (version 0.08.8): the two is the table's `respawn_incomes`, where a 1 was
        // written here; the wait is the Incomes that raise nothing before the one that does.
        if army.standing && !army.levy && let ArmyHome::State(sid) = army.home {
            self.state_mut(sid).respawn_wait = self.tables.standing_army.respawn_incomes.saturating_sub(1);
        }
        for s in &mut self.ships {
            if s.army == Some(id) {
                s.army = None;
            }
        }
        let who = self.army_seat(&army).map(|s| self.seat_name(s)).unwrap_or_else(|| "neutral".to_string());
        let name = self.army_name(&army);
        self.log(format!("{who} {name} destroyed ({why})."));
        let text = self.say("army_destroyed", &[("faction", who), ("army", name), ("why", why.to_string())]);
        self.report_line(LineKind::DecisiveBattle, at, text);
    }

    /// Spec 8.5: every Facility or Module at a place rolls a 1-in-4 chance to be destroyed.
    /// Ticket #279 (version 0.08.5): `charged` are the seats that wear what burns -- the taker on a
    /// transfer -- at the table's ppm per building, split between them.
    /// Ticket #298 (version 0.08.6): called from one place now, the transfer by the three-turn
    /// clock; a Battle burns nothing and a Pacified transfer rolls nothing. A Unique Facility is
    /// rolled like any other building, at the designer's word. Returns what burned; the Moment for
    /// a place taken by force is the transfer's, fired whether or not anything burned.
    fn destruction_rolls(&mut self, place: Place, why: &str, charged: &[Seat]) -> Vec<String> {
        let p = self.tables.influence.destruction_chance;
        let mut lost = Vec::new();
        match place {
            Place::State(s) => {
                let n = self.state(s).facilities.len();
                let keep: Vec<bool> = (0..n).map(|_| !self.rng.chance(p)).collect();
                let st = self.state_mut(s);
                let mut i = 0;
                st.facilities.retain(|f| {
                    let k = keep[i];
                    i += 1;
                    if !k {
                        lost.push(f.kind.name().to_string());
                    }
                    k
                });
            }
            Place::Colony(c) => {
                if let Some(col) = self.colony(c) {
                    let n = col.modules.len();
                    let keep: Vec<bool> = (0..n).map(|_| !self.rng.chance(p)).collect();
                    let col = self.colony_mut(c).unwrap();
                    let mut i = 0;
                    col.modules.retain(|m| {
                        let k = keep[i];
                        i += 1;
                        if !k {
                            lost.push(m.kind.name().to_string());
                        }
                        k
                    });
                    // Colonists beyond the Habitats left are lost with them.
                    let room = self.habitat_room(self.colony(c).unwrap());
                    if let Some(col) = self.colony_mut(c) {
                        col.colonists = col.colonists.min(room);
                    }
                }
            }
        }
        if !lost.is_empty() && !charged.is_empty() {
            let each = lost.len() as f64 * self.tables.climate.war_ppm_per_building / charged.len() as f64;
            let on_earth = self.on_earth_place(place);
            for s in charged {
                self.charge_war(on_earth, Some(*s), each);
            }
        }
        if !lost.is_empty() {
            let line = format!("{} was {}: {} destroyed.", self.place_name(place), why, lost.join(", "));
            self.log(line);
            let text = self.say(
                "units_destroyed",
                &[("place", self.place_name(place)), ("why", why.to_string()), ("lost", lost.join(", "))],
            );
            self.report_line(LineKind::DecisiveBattle, Some(place.into()), text);
        }
        lost
    }

    /// Ticket #328 (version 0.08.8): Bombard. A Battleship at a Body off Earth whose seat holds
    /// Orbital Control there outright -- checked again here, since the Battles just fought may have
    /// sunk it or taken the orbit -- strikes a rival's Colony at the Body: one Module drawn at
    /// random (the Core Module and the Archive left out) rolls the destruction chance; a burned
    /// Habitat takes the Colonists beyond the room left with it. Rung 3 against the holder, a
    /// non-aggression Accord broken with it, the war ppm of a burned building charged to the
    /// bombarder; a Report line hit or miss, a Moment and the Battle mark when anything burned.
    fn resolve_bombards(&mut self) {
        let bombards = std::mem::take(&mut self.pending.bombards);
        for (seat, ship, colony) in bombards {
            let Some(s) = self.ship(ship) else { continue };
            let ShipAt::Body(body) = s.at else { continue };
            if s.seat != seat || s.escaped || s.kind != UnitKind::Battleship || body == BodyId::Earth {
                continue;
            }
            let orbit = self.ship_orbit(s);
            let ship_name = self.ship_name(s);
            let ship_strength = self.ship_strength(s);
            let Some(col) = self.colony(colony) else { continue };
            let Some(holder) = col.control.director().filter(|h| *h != seat) else { continue };
            if col.body != body {
                continue;
            }
            // Ticket #335 (version 0.09.0): **the orbit you are in is the orbit you must hold**,
            // read again here as the Control always was, since the Battles just fought may have
            // sunk the Battleship's escort or put a rival back in the ring: a ground Colony from
            // low orbit under an outright Orbital Control, a station from its own orbit with no
            // rival warship and no rival working Battery left in it.
            if self.colony_orbit(col) != orbit {
                continue;
            }
            let holds = if orbit.is_low() { self.orbital_control(body) == Some(seat) } else { self.orbit_uncontested(seat, body, orbit) };
            if !holds {
                continue;
            }
            let targets: Vec<usize> = col.modules.iter().enumerate().filter(|(_, m)| !matches!(m.kind, ModuleKind::Core | ModuleKind::Archive)).map(|(i, _)| i).collect();
            let place = self.place_name(Place::Colony(colony));
            self.war.bombards[seat.index()] += 1;
            self.offend_by(seat, holder, 3);
            let p = self.tables.influence.destruction_chance;
            let mut hit = false;
            let mut dead = 0u32;
            let module = if targets.is_empty() {
                None
            } else {
                let i = targets[self.rng.pick(targets.len())];
                let name = self.colony(colony).map(|c| c.modules[i].kind.name().to_string()).unwrap_or_default();
                if self.rng.chance(p) {
                    hit = true;
                    let before = self.colony(colony).map(|c| c.colonists).unwrap_or(0);
                    if let Some(c) = self.colony_mut(colony) {
                        c.modules.remove(i);
                    }
                    let room = self.colony(colony).map(|c| self.habitat_room(c)).unwrap_or(0);
                    if let Some(c) = self.colony_mut(colony) {
                        c.colonists = c.colonists.min(room);
                        dead = before.saturating_sub(c.colonists);
                    }
                }
                Some(name)
            };
            let module_name = module.clone().unwrap_or_else(|| "walls".to_string());
            let text = if hit {
                self.war.modules_burned[seat.index()] += 1;
                let ppm = self.tables.climate.war_ppm_per_building;
                self.charge_war(false, Some(seat), ppm);
                let dead_words = if dead > 0 { self.phrase("bombard_dead", &[("n", dead.to_string())]) } else { String::new() };
                self.say("bombard_hit", &[("faction", self.seat_name(seat)), ("ship", ship_name.clone()), ("place", place.clone()), ("module", module_name.clone()), ("dead", dead_words)])
            } else {
                self.say("bombard_miss", &[("faction", self.seat_name(seat)), ("ship", ship_name.clone()), ("place", place.clone()), ("module", module_name.clone())])
            };
            self.log(text.clone());
            // Ticket #335 (version 0.09.0): the record lives at the orbit the strike was given from.
            let at = Some(ReportPlace::Orbit(body, orbit));
            self.report_line(if hit { LineKind::DecisiveBattle } else { LineKind::Battle }, at, text.clone());
            if hit {
                let result = format!("bombarded by the {}: the {} destroyed{}", self.seat_name(seat), module_name, if dead > 0 { format!(", {dead} Colonists dead") } else { String::new() });
                self.moment(MomentKind::DecisiveBattle, &[("place", place.clone()), ("result", result), ("figure", "1 lost".to_string())], at);
            }
            // The Battle mark and the band's row read the Battle record, so a Bombard is one.
            let outcome = if hit { format!("the {module_name} destroyed") } else { format!("the {module_name} struck, standing") };
            self.report.battles.push(BattleLine {
                place: format!("{} (bombardment)", self.orbit_battle_name(body, orbit)),
                parties: vec![
                    BattleParty { seat: Some(seat), aggressor: true, units: format!("{ship_name} bombarded {place}"), strength: ship_strength, hits: u32::from(hit), destroyed: Vec::new(), escaped: Vec::new(), odds: Some(p) },
                    BattleParty { seat: Some(holder), aggressor: false, units: format!("{place}: {outcome}"), strength: 0, hits: 0, destroyed: if hit { vec![module_name] } else { Vec::new() }, escaped: Vec::new(), odds: None },
                ],
                result: text,
                at,
            });
        }
    }

    // ------------------------------------------------------------------ (c)

    fn resolve_occupation(&mut self) {
        let mut places: Vec<Place> = StateId::ALL.into_iter().map(Place::State).collect();
        places.extend(self.colonies.iter().map(|c| Place::Colony(c.id)));
        self.resolve_occupation_at(places);
    }

    /// Ticket #300 (version 0.08.6): the Occupation pass over a given set of places, so the
    /// Colonies a landing reached this turn can be checked again after the second ground pass --
    /// a landing that meets nobody begins its Occupation the same turn.
    fn resolve_occupation_at(&mut self, places: Vec<Place>) {
        for place in places {
            let control = self.place_control(place);
            match control {
                Control::Occupied { occupier, previous, turns, banked } => {
                    // Ticket #318 (version 0.08.8): held by the presence that begins one.
                    if !self.present_at(place, occupier) {
                        // Occupation broken. Ticket #299 (version 0.08.6): at a cost, whether the
                        // last Army marched off, was lifted or was destroyed -- the place hands back
                        // at +2 Unrest, the occupier takes a rung-2 offence from the previous holder
                        // (a neutral charges nobody), and the Standing the Occupation banked is wiped.
                        // Until this version a break was silent and free.
                        let back = match previous {
                            Some(p) => Control::Controlled(p),
                            None => Control::Neutral,
                        };
                        self.set_place_control(place, back);
                        self.war.occupations_broken[occupier.index()] += 1;
                        if let Place::State(sid) = place {
                            let n = self.tables.unrest.occupation_break;
                            self.raise_unrest(sid, n, UnrestSource::Plain);
                        }
                        if let Some(p) = previous {
                            self.offend_by(occupier, p, self.tables.relations.occupation_broken_offence);
                        }
                        if banked > 0 {
                            let s = self.seat_mut(occupier);
                            if let Some(v) = s.influence.get_mut(&place) {
                                *v = (*v - banked).max(0);
                            }
                        }
                        let holder = previous.map(|p| self.seat_name(p)).unwrap_or_else(|| "nobody".to_string());
                        let line = format!("Occupation of {} by the {} broke: back to the {} at +{:.0} Unrest, an offence.", self.place_name(place), self.seat_name(occupier), holder, self.tables.unrest.occupation_break);
                        self.log(line);
                        let text = self.say(
                            "occupation_broken",
                            &[
                                ("place", self.place_name(place)),
                                ("faction", self.seat_name(occupier)),
                                ("holder", holder),
                                ("unrest", Game::unrest_figure(self.tables.unrest.occupation_break)),
                                ("standing", banked.to_string()),
                            ],
                        );
                        self.report_line(LineKind::Occupation, Some(place.into()), text);
                        continue;
                    }
                    if !self.defenders_at(place, occupier).is_empty() {
                        continue; // defenders re-engaged; the count does not advance
                    }
                    let turns = turns + 1;
                    // Ticket #52: every turn of Occupation adds one to the state's Unrest.
                    if let Place::State(sid) = place {
                        let n = self.tables.unrest.occupation_per_turn;
                        self.raise_unrest(sid, n, UnrestSource::Plain);
                    }
                    let gain = self.occupation_gain(place, occupier);
                    self.set_place_control(place, Control::Occupied { occupier, previous, turns, banked: banked + gain });
                    let have = self.seat(occupier).influence.get(&place).copied().unwrap_or(0);
                    let pacified = have > 0 && have >= self.influence_threshold(place);
                    if pacified || turns >= self.tables.influence.occupation_turns {
                        self.transfer_control(place, occupier, if pacified { "Pacified" } else { "Occupation complete" });
                    }
                }
                _ => {
                    for seat in Seat::ALL {
                        if control.director() == Some(seat) {
                            continue;
                        }
                        if !self.alone_at(place, seat) {
                            continue;
                        }
                        let previous = control.controller();
                        self.set_place_control(place, Control::Occupied { occupier: seat, previous, turns: 1, banked: 0 });
                        self.war.occupations_begun[seat.index()] += 1;
                        // Ticket #52: an Occupation begins at +3 Unrest, damped by nothing.
                        if let Place::State(sid) = place {
                            let n = self.tables.unrest.occupation_start;
                            self.raise_unrest(sid, n, UnrestSource::Plain);
                        }
                        let line = format!("The {} occupy {}.", self.seat_name(seat), self.place_name(place));
                        self.log(line);
                        let text = self.say("occupation_begun", &[("faction", self.seat_name(seat)), ("place", self.place_name(place))]);
                        self.report_line(LineKind::Occupation, Some(place.into()), text);
                        let gain = self.occupation_gain(place, seat);
                        self.set_place_control(place, Control::Occupied { occupier: seat, previous, turns: 1, banked: gain });
                        let have = self.seat(seat).influence.get(&place).copied().unwrap_or(0);
                        let pacified = have > 0 && have >= self.influence_threshold(place);
                        if pacified || self.tables.influence.occupation_turns <= 1 {
                            self.transfer_control(place, seat, if pacified { "Pacified" } else { "Occupation complete" });
                        }
                        break;
                    }
                }
            }
        }
    }

    /// Returns the Standing gained, which the Occupation banks (ticket #299).
    fn occupation_gain(&mut self, place: Place, seat: Seat) -> i64 {
        // Ticket #187 (version 0.08.0): an occupier's Standing is an outsider's by definition, so
        // Resistance bites it. Otherwise invading would be the way round a well-schooled population.
        let gain = self.standing_from(place, self.pacification_gain(place));
        let s = self.seat_mut(seat);
        *s.influence.entry(place).or_insert(0) += gain;
        s.influenced_this_turn.push(place);
        gain
    }

    pub fn place_control(&self, place: Place) -> Control {
        match place {
            Place::State(s) => self.state(s).control,
            Place::Colony(c) => self.colony(c).map(|c| c.control).unwrap_or(Control::Neutral),
        }
    }

    fn set_place_control(&mut self, place: Place, control: Control) {
        match place {
            // Ticket #53: a state that changed hands this turn does not get its natural fall.
            Place::State(s) => {
                let was = self.state(s).control;
                let changed = was.director() != control.director() || was.controller() != control.controller();
                let st = self.state_mut(s);
                st.control = control;
                st.changed_hands |= changed;
                // Ticket #53: the neutrality clock starts over whenever control is written.
                self.restart_neutrality_clock(s);
            }
            Place::Colony(c) => {
                if let Some(col) = self.colony_mut(c) {
                    col.control = control;
                }
            }
        }
    }

    /// Ticket #54: every Scrubber in a Nation State is destroyed when the state changes hands, by
    /// Influence, by Occupation or by being thrown off. They are the Custodians' own works, and they
    /// do not pass to whoever takes the place.
    pub fn destroy_scrubbers(&mut self, sid: StateId, why: &str) {
        let n = self.state(sid).facilities.iter().filter(|f| f.kind == FacilityKind::Scrubber).count();
        if n == 0 {
            return;
        }
        self.state_mut(sid).facilities.retain(|f| f.kind != FacilityKind::Scrubber);
        self.state_mut(sid).queue.retain(|b| b.item != BuildItem::Facility(FacilityKind::Scrubber));
        let line = format!(
            "{} Scrubber(s) in {} were destroyed when the state {}.",
            n,
            self.tables.state(sid).name,
            why
        );
        self.log(line);
        let text = self.say("scrubbers_destroyed", &[("n", n.to_string()), ("state", self.tables.state(sid).name.clone()), ("why", why.to_string())]);
        self.report_line(LineKind::Climate, Some(ReportPlace::State(sid)), text);
    }

    /// Control passes to `seat` (spec 8.3, 8.5): rivals' Influence wiped, a destruction roll, Armies follow.
    pub fn transfer_control(&mut self, place: Place, seat: Seat, why: &str) {
        // Ticket #286 (version 0.08.5): a take that was not by Influence was by force.
        // Ticket #336 (version 0.09.0): and one that WAS is counted here by the kind of place, where
        // the sweep used to count them all together by scraping the log for lines ending
        // `(Influence).` -- a counter that could not say whether the doubling off Earth had moved
        // anything, since a Region and a Colony read the same.
        if self.place_control(place).controller() != Some(seat) {
            if why != "Influence" {
                self.war.takes_by_force[seat.index()] += 1;
            } else {
                let i = seat.index();
                match place {
                    Place::State(_) => self.war.takes_by_influence_states[i] += 1,
                    Place::Colony(c) if self.colony(c).map(|col| col.in_orbit).unwrap_or(false) => self.war.takes_by_influence_stations[i] += 1,
                    Place::Colony(_) => self.war.takes_by_influence_colonies[i] += 1,
                }
            }
        }
        // Ticket #282 (version 0.08.5): a Levy was the neutral Region's; it stands down the moment
        // the Region is somebody's.
        if let Place::State(sid) = place {
            self.armies.retain(|a| !(a.levy && a.home == ArmyHome::State(sid)));
        }
        // Ticket #54: the Scrubbers go first, before the place has a new owner to hold them.
        if let Place::State(sid) = place
            && self.place_control(place).controller() != Some(seat)
        {
            self.destroy_scrubbers(sid, "changed hands");
        }
        // Ticket #51: an Archive is destroyed when its Colony changes hands, whether by Occupation
        // or by Influence. The Archive fund is kept, so the Archivists can start again.
        if let Place::Colony(c) = place
            && self.place_control(place).controller() != Some(seat)
            && self.colony(c).map(|col| col.modules.iter().any(|m| m.kind == ModuleKind::Archive)).unwrap_or(false)
        {
            let owner = self.place_control(place).controller();
            if let Some(col) = self.colony_mut(c) {
                col.modules.retain(|m| m.kind != ModuleKind::Archive);
            }
            let whose = owner.map(|o| self.seat_name(o)).unwrap_or_else(|| "nobody".to_string());
            let line = format!("The Archive at {} was destroyed when the Colony passed out of the {}' hands; their Archive fund is kept.", self.place_name(place), whose);
            self.log(line);
            let text = self.say("archive_destroyed", &[("place", self.place_name(place)), ("faction", whose)]);
            self.report_line(LineKind::Archive, Some(place.into()), text);
        }
        self.set_place_control(place, Control::Controlled(seat));
        // Standings persist through a transfer (ticket #33): the old controller keeps its own and
        // can contest the place back.
        let line = format!("{} now belongs to the {} ({}).", self.place_name(place), self.seat_name(seat), why);
        self.log(line);
        let text = self.say(
            "control_changed",
            &[("place", self.place_name(place)), ("faction", self.seat_name(seat)), ("why", why.to_string())],
        );
        self.report_line(LineKind::ControlChanged, Some(place.into()), text);
        self.moment(
            MomentKind::ControlChanged,
            &[("place", self.place_name(place)), ("faction", self.seat_name(seat))],
            Some(place.into()),
        );
        // Version 0.03 (ticket #31): a place taken by Influence keeps everything; only a place
        // that Occupation transfers rolls for destruction. Ticket #298 (version 0.08.6): and only
        // one that the three-turn clock transfers -- a place that transfers by PACIFIED is taken
        // whole, at the designer's word, so "beat the Army, then win the people" keeps what it wins.
        // The Moment for a place taken by force fires on every take by force, burned or not: the
        // taking is the news, not the fire.
        if why != "Influence" {
            let lost = if why == "Pacified" { Vec::new() } else { self.destruction_rolls(place, "taken", &[seat]) };
            let result = if lost.is_empty() { format!("{} taken whole by the {} ({why}).", self.place_name(place), self.seat_name(seat)) } else { format!("{} was taken: {} destroyed.", self.place_name(place), lost.join(", ")) };
            let figure = if lost.is_empty() { "nothing lost".to_string() } else { format!("{} lost", lost.len()) };
            self.moment(MomentKind::PlaceTakenByForce, &[("place", self.place_name(place)), ("result", result), ("figure", figure)], Some(place.into()));
        }
        // Armies at the place that fought for the old owner stand for the new one only if they are the place's own.
        // Foreign Armies keep their own home and seat; nothing to do.
    }

    // ------------------------------------------------------------------ (d)

    fn resolve_influence(&mut self) {
        // Version 0.03 (ticket #33): every Faction keeps a standing on every place; spending on a place
        // you control raises your own standing there.
        let spent = std::mem::take(&mut self.pending.influence);
        // Ticket #222 (version 0.08.2): an instance is a PLACE, never an order. Two spends on one
        // Region in a turn are one offence; spends on two of their Regions are two. Tracked here
        // because the pending list is per ORDER and the rule is per place -- without this, splitting
        // one spend across three orders would cost three times the damage, which is exactly the trap
        // 0.08.0's flat per-turn charge was written to avoid.
        let mut charged: Vec<(Seat, Seat, Target)> = Vec::new();
        for (seat, target, amount) in spent {
            let own = self.place_control(target).controller() == Some(seat);
            // Ticket #187 (version 0.08.0): Resistance. An outsider's Influence buys less Standing at
            // a well-schooled place and more at a badly-schooled one; the controller converts in full.
            let gained = if own { amount } else { self.standing_from(target, amount) };
            // Ticket #191 (version 0.08.0): spending on a place another Faction HOLDS is an offence,
            // and their view of you falls once for the turn however many orders you put in. A
            // NEUTRAL place is never an offence, however hotly contested: two Factions bidding for
            // empty ground are competing, not crossing each other.
            if let Some(victim) = self.place_control(target).director()
                && victim != seat
            {
                // Ticket #222: rung 1, once per place per turn.
                if !charged.contains(&(seat, victim, target)) {
                    charged.push((seat, victim, target));
                    self.offend_by(seat, victim, 1);
                }
            }
            let s = self.seat_mut(seat);
            *s.influence.entry(target).or_insert(0) += gained;
            s.influenced_this_turn.push(target);
            let bite = if gained == amount { String::new() } else { format!(" (worth {gained} there)") };
            self.log(format!("{} spent {} Influence {} {}{}.", self.seat_name(seat), amount, if own { "holding" } else { "on" }, self.place_name(target), bite));
        }
        // Embassies and Relays (ticket #36) raise their place's standing for its controller each turn,
        // which counts as Influence received, so the standing does not decay.
        let mut rises: Vec<(Seat, Place, i64)> = Vec::new();
        for st in &self.states {
            if let Some(c) = st.control.controller() {
                let r: i64 = st.facilities.iter().filter(|f| f.working()).map(|f| self.tables.facility(f.kind).standing_per_turn).sum();
                if r > 0 {
                    rises.push((c, Place::State(st.id), r));
                }
            }
        }
        for col in &self.colonies {
            if let Some(c) = col.control.controller() {
                let r: i64 = col.modules.iter().filter(|m| m.working()).map(|m| self.tables.module(m.kind).standing_per_turn).sum();
                if r > 0 {
                    rises.push((c, Place::Colony(col.id), r));
                }
            }
        }
        for (seat, place, r) in rises {
            let s = self.seat_mut(seat);
            *s.influence.entry(place).or_insert(0) += r;
            s.influenced_this_turn.push(place);
        }
        // Decay on every standing that received nothing this turn: 1 on a place you control, 2
        // elsewhere -- and, since ticket #266 (version 0.08.4), 1 or 3 on a Region you do not hold
        // by your Blame share, which `standing_decay_for` reads for every place.
        for seat in Seat::ALL {
            let decays: Vec<(Place, i64)> = self.seat(seat).influence.keys().map(|t| (*t, self.standing_decay_for(seat, *t))).collect();
            let s = self.seat_mut(seat);
            let touched = std::mem::take(&mut s.influenced_this_turn);
            for (t, d) in decays {
                if !touched.contains(&t)
                    && let Some(v) = s.influence.get_mut(&t)
                {
                    *v = (*v - d).max(0);
                }
            }
            s.influence.retain(|_, v| *v > 0);
        }
        // Thresholds: a neutral place needs the threshold; a controlled place needs a standing at least
        // the controller's plus the challenge margin (version 0.04, ticket #41) and at least the threshold.
        // Ticket #53: the threshold is the CHALLENGER's own, since Blame raises it seat by seat; the
        // challenge margin is the same for everyone.
        // Ticket #60: that whole figure is `influence_needed_for`, which the AI and the state card
        // read too, so what a card prints and what this applies cannot drift apart.
        let mut targets: Vec<Place> = StateId::ALL.into_iter().map(Place::State).collect();
        targets.extend(self.colonies.iter().map(|c| Place::Colony(c.id)));
        for target in targets {
            let controller = self.place_control(target).controller();
            let qualifying: Vec<Seat> = Seat::ALL
                .into_iter()
                .filter(|s| {
                    let have = self.seat(*s).influence.get(&target).copied().unwrap_or(0);
                    controller != Some(*s) && have > 0 && have >= self.influence_needed_for(*s, target)
                })
                .collect();
            // Ticket #50: among challengers who all qualify the same turn, the higher Standing takes
            // the place. Ticket #70 (version 0.05.5): an exact tie on a HELD place leaves it with
            // its holder, as before; on a neutral place the lot decides, drawn from the game's own
            // generator as a contested orbital slot is, so a seed replays the same draw.
            let winner = match qualifying.len() {
                0 => continue,
                1 => qualifying[0],
                _ => {
                    let standing = |s: &Seat| self.seat(*s).influence.get(&target).copied().unwrap_or(0);
                    let top = qualifying.iter().map(standing).max().unwrap_or(0);
                    let leaders: Vec<Seat> = qualifying.iter().copied().filter(|s| standing(s) == top).collect();
                    if leaders.len() > 1 {
                        let names: Vec<String> = leaders.iter().map(|s| self.seat_name(*s)).collect();
                        if controller.is_some() {
                            let line = format!("{} is claimed by {} at the same Standing; it stays as it is.", self.place_name(target), names.join(" and "));
                            self.log(line);
                            let text = self.say("claim_tied", &[("place", self.place_name(target)), ("factions", names.join(" and "))]);
                            self.report_line(LineKind::Note, Some(target.into()), text);
                            continue;
                        }
                        let drawn = self.random_tie(&leaders);
                        let line = format!("{} is claimed by {} at the same Standing; the lot falls to the {}.", self.place_name(target), names.join(" and "), self.seat_name(drawn));
                        self.log(line);
                        let text = self.say("claim_lot", &[("place", self.place_name(target)), ("factions", names.join(" and ")), ("winner", self.seat_name(drawn))]);
                        self.report_line(LineKind::Note, Some(target.into()), text);
                        drawn
                    } else {
                        leaders[0]
                    }
                }
            };
            self.transfer_control(target, winner, "Influence");
        }
    }

    /// Who takes a contested slot at a Body: the greater Ship stack strength in orbit, and among
    /// seats tied at the top a random draw from the game's own generator (ticket #50).
    pub fn tiebreak_at_body(&mut self, body: BodyId, among: &[Seat]) -> Seat {
        let top = among.iter().map(|s| self.ship_stack_strength(*s, body)).max().unwrap_or(0);
        let tied: Vec<Seat> = among.iter().copied().filter(|s| self.ship_stack_strength(*s, body) == top).collect();
        self.random_tie(&tied)
    }

    // ------------------------------------------------------------------ (e)

    /// Ticket #332 (version 0.09.0): a build completes at the Resolution its place's Widgets reach
    /// its figure, the queue served in the order it was given. Each place's Widgets this turn
    /// (`widgets_at`) fill the earliest build first and flow on to the next, nothing split; what is
    /// left over is lost, since Widgets are a rate and never a stock. Every build whose count has
    /// reached its figure then completes, in queue order, through `complete_build` as before. Made,
    /// applied and lost are counted to the place's director for the sweep, and every directed
    /// place's queue depth is sampled here for the median.
    ///
    /// Launch Pad Fire (ticket #32): the Region it struck applies no Widgets this turn, unless
    /// Clean Propellant is held; before this version it pushed a due Ship back a turn.
    fn resolve_builds(&mut self) {
        let mut pad_fire: Option<StateId> = self.last_event.as_ref().and_then(|ev| match (ev.card, ev.target) {
            (Card::Event(EventId::LaunchPadFire), EventTarget::State(s)) => Some(s),
            _ => None,
        });
        if self.has_tech(TechId::CleanPropellant) {
            pad_fire = None;
        }
        let places: Vec<Place> = StateId::ALL.iter().map(|s| Place::State(*s)).chain(self.colonies.iter().map(|c| Place::Colony(c.id))).collect();
        let mut completed: Vec<(Place, Build)> = Vec::new();
        self.widgets.applied_last = [0; SEAT_COUNT];
        // Ticket #337 (version 0.09.0): the Overtime card's extra shift, at the busiest Region of
        // every seat that worked it. They are Widgets like any other: applied to the queue in
        // order, counted to the director as made and applied, and lost where nothing wants them.
        let overtime = self.card_widgets_now();
        for place in places {
            let director = self.place_director(place);
            let made = self.widgets_at(place) + overtime.iter().filter(|(p, _)| *p == place).map(|(_, n)| n).sum::<i64>();
            if director.is_some() {
                let depth = self.queue_at(place).len() as u32;
                self.widgets.queue_depths.push(depth);
            }
            let fired = matches!(place, Place::State(s) if pad_fire == Some(s));
            if fired && made > 0 {
                let line = format!("Launch Pad Fire: {} applies no Widgets this turn; {made} lost.", self.place_name(place));
                self.log(line);
                let text = self.say("launch_pad_fire", &[("place", self.place_name(place)), ("widgets", made.to_string())]);
                self.report_line(LineKind::Note, Some(place.into()), text);
            }
            let mut remaining = if fired { 0 } else { made };
            let queue: &mut Vec<Build> = match place {
                Place::State(s) => &mut self.state_mut(s).queue,
                Place::Colony(c) => {
                    let Some(col) = self.colony_mut(c) else { continue };
                    &mut col.queue
                }
            };
            for b in queue.iter_mut() {
                let take = remaining.min(b.widgets.saturating_sub(b.done) as i64);
                b.done += take as u32;
                remaining -= take;
            }
            let mut i = 0;
            while i < queue.len() {
                if queue[i].done >= queue[i].widgets {
                    completed.push((place, queue.remove(i)));
                } else {
                    i += 1;
                }
            }
            let applied = if fired { 0 } else { made - remaining };
            if let Some(d) = director {
                let w = &mut self.widgets;
                w.made[d.index()] += made;
                w.applied[d.index()] += applied;
                w.applied_last[d.index()] += applied;
                w.lost[d.index()] += made - applied;
            }
        }
        for (place, b) in completed {
            self.complete_build(place, b);
        }
    }

    /// Ticket #54: every Mothball, Restart and Decommission whose turn has come. A decommission
    /// refunds half the building's Materials, rounded down, and frees its slot; in a Nation State a
    /// mothball and a decommission each add their Unrest, and in a Colony neither adds anything.
    fn resolve_changes(&mut self) {
        let turn = self.turn;
        let mut lines: Vec<String> = Vec::new();
        // Ticket #58: the dispatch's own copy, each with the heading it belongs under.
        let mut said: Vec<(LineKind, Option<ReportPlace>, String)> = Vec::new();
        let mut unrest: Vec<(StateId, BuildingChange)> = Vec::new();
        for sid in StateId::ALL {
            // Highest position first, so a removal never shifts one still to come.
            let due: Vec<usize> = (0..self.state(sid).facilities.len())
                .rev()
                .filter(|i| self.state(sid).facilities[*i].change.map(|c| c.due_turn <= turn).unwrap_or(false))
                .collect();
            for i in due {
                let change = self.state(sid).facilities[i].change.unwrap();
                let kind = self.state(sid).facilities[i].kind;
                let refund = self.tables.facility(kind).materials / 2;
                let f = &mut self.state_mut(sid).facilities[i];
                f.change = None;
                match change.what {
                    BuildingChange::Mothball => {
                        f.mothballed = true;
                        f.online = false;
                    }
                    BuildingChange::Restart => {
                        f.mothballed = false;
                        f.online = true;
                    }
                    BuildingChange::Decommission => {
                        self.state_mut(sid).facilities.remove(i);
                        self.seat_mut(change.seat).stockpile.materials += refund;
                    }
                }
                let where_ = self.tables.state(sid).name.clone();
                lines.push(match change.what {
                    BuildingChange::Decommission => format!(
                        "The {} decommissioned the {} in {}: {} Materials back and its slot free.",
                        self.seat_name(change.seat),
                        kind.name(),
                        where_,
                        refund
                    ),
                    w => format!("The {} {} the {} in {}.", self.seat_name(change.seat), w.done(), kind.name(), where_),
                });
                let args = vec![
                    ("faction", self.seat_name(change.seat)),
                    ("done", change.what.done().to_string()),
                    ("building", kind.name().to_string()),
                    ("state", where_.clone()),
                    ("refund", refund.to_string()),
                ];
                let text = match change.what {
                    BuildingChange::Decommission => self.say("building_decommissioned_state", &args),
                    _ => self.say("building_changed_state", &args),
                };
                let mine = crate::report::line_kind_of(change.seat, LineKind::YourWorks, LineKind::Note, self.spectator);
                said.push((mine, Some(ReportPlace::State(sid)), text));
                if matches!(change.what, BuildingChange::Mothball | BuildingChange::Decommission) {
                    unrest.push((sid, change.what));
                }
            }
        }
        let cids: Vec<ColonyId> = self.colonies.iter().map(|c| c.id).collect();
        for cid in cids {
            let len = self.colony(cid).map(|c| c.modules.len()).unwrap_or(0);
            let due: Vec<usize> = (0..len)
                .rev()
                .filter(|i| self.colony(cid).unwrap().modules[*i].change.map(|c| c.due_turn <= turn).unwrap_or(false))
                .collect();
            // Ticket #197: whether anything actually came down here, read before `due` is consumed.
            let changed_here = !due.is_empty();
            for i in due {
                let change = self.colony(cid).unwrap().modules[i].change.unwrap();
                let kind = self.colony(cid).unwrap().modules[i].kind;
                let refund = self.module_materials(change.seat, kind) / 2;
                let col = self.colony_mut(cid).unwrap();
                let m = &mut col.modules[i];
                m.change = None;
                match change.what {
                    BuildingChange::Mothball => {
                        m.mothballed = true;
                        m.online = false;
                    }
                    BuildingChange::Restart => {
                        m.mothballed = false;
                        m.online = true;
                    }
                    BuildingChange::Decommission => {
                        col.modules.remove(i);
                        self.seat_mut(change.seat).stockpile.materials += refund;
                    }
                }
                let where_ = self.place_name(Place::Colony(cid));
                lines.push(match change.what {
                    BuildingChange::Decommission => {
                        format!("The {} decommissioned the {} at {}: {} Materials back.", self.seat_name(change.seat), kind.name(), where_, refund)
                    }
                    w => format!("The {} {} the {} at {}.", self.seat_name(change.seat), w.done(), kind.name(), where_),
                });
                let args = vec![
                    ("faction", self.seat_name(change.seat)),
                    ("done", change.what.done().to_string()),
                    ("building", kind.name().to_string()),
                    ("colony", where_.clone()),
                    ("refund", refund.to_string()),
                ];
                let text = match change.what {
                    BuildingChange::Decommission => self.say("building_decommissioned_colony", &args),
                    _ => self.say("building_changed_colony", &args),
                };
                let mine = crate::report::line_kind_of(change.seat, LineKind::YourWorks, LineKind::Archive, self.spectator);
                said.push((mine, Some(ReportPlace::Colony(cid)), text));
            }
            // Colonists beyond the Habitats a decommission left are lost with them.
            //
            // Ticket #197: ONLY where a change actually resolved here. This ran on every Colony every
            // phase, and `habitat_room` reads the CURRENT holder's Habitat capacity multiplier -- which
            // only the Arkwrights have -- so an Arkwright station passing to another Faction shrank in
            // the same Resolution and its people were deleted, with no log line and no Report line:
            // twenty Colonists gone in silence over eighty measured games. A cap that falls below what
            // already stands destroys nothing, exactly as a Module cap does not (`CONTEXT.md`,
            // **Colony**); only a building coming down takes people with it. And now it says so, since
            // this was the one way Colonists were lost without a word -- crowding has always logged and
            // raised a Moment.
            if changed_here && let Some(col) = self.colony(cid) {
                let room = self.habitat_room(col);
                let had = col.colonists;
                if had > room {
                    let lost = had - room;
                    let where_ = self.place_name(Place::Colony(cid));
                    self.colony_mut(cid).unwrap().colonists = room;
                    lines.push(format!("{lost} Colonists at {where_} had nowhere to live and were lost."));
                    let text = self.say("colonists_no_room", &[("n", lost.to_string()), ("colony", where_)]);
                    let mine = match self.colony(cid).and_then(|c| c.control.controller()) {
                        Some(seat) => crate::report::line_kind_of(seat, LineKind::YourWorks, LineKind::Archive, self.spectator),
                        None => LineKind::Archive,
                    };
                    said.push((mine, Some(ReportPlace::Colony(cid)), text));
                }
            }
        }
        for (sid, what) in unrest {
            let rose = match what {
                BuildingChange::Mothball => self.unrest_from_mothball(sid),
                _ => self.unrest_from_decommission(sid),
            };
            if rose > 0.0 {
                lines.push(format!(
                    "{}: Unrest rose by {} to {}.",
                    self.tables.state(sid).name,
                    Game::unrest_figure(rose),
                    self.unrest_text(sid)
                ));
                let text = self.say(
                    "unrest_rose_state",
                    &[("state", self.tables.state(sid).name.clone()), ("rose", Game::unrest_figure(rose).to_string()), ("unrest", self.unrest_text(sid))],
                );
                said.push((LineKind::Unrest, Some(ReportPlace::State(sid)), text));
            }
        }
        for line in lines {
            self.log(line);
        }
        for (kind, place, text) in said {
            self.report_line(kind, place, text);
        }
    }

    /// Ticket #54: a Strip Permit whose last doubled turn has run charges its price: the state's
    /// Baseline Emissions rise for good, and its Unrest with them.
    fn resolve_strip_permits(&mut self) {
        let t = self.tables.strip_permit.clone();
        for sid in StateId::ALL {
            if self.state(sid).strip_permit_ends != Some(self.turn) {
                continue;
            }
            self.state_mut(sid).strip_permit_ends = None;
            self.state_mut(sid).baseline_rise += t.baseline_rise;
            let rose = self.raise_unrest(sid, t.unrest, UnrestSource::Plain);
            let line = format!(
                "The Strip Permit in {} ran out: its Baseline Emissions stand at {:.1} for good, and its Unrest rose by {} to {}.",
                self.tables.state(sid).name,
                self.baseline_emissions(sid),
                Game::unrest_figure(rose),
                self.unrest_text(sid)
            );
            self.log(line);
            let text = self.say(
                "strip_permit_ended",
                &[
                    ("state", self.tables.state(sid).name.clone()),
                    ("baseline", format!("{:.1}", self.baseline_emissions(sid))),
                    ("rose", Game::unrest_figure(rose).to_string()),
                    ("unrest", self.unrest_text(sid)),
                ],
            );
            self.report_line(LineKind::Unrest, Some(ReportPlace::State(sid)), text);
        }
    }

    fn complete_build(&mut self, place: Place, b: Build) {
        let turn = self.turn;
        let name = b.item.name();
        match (place, b.item) {
            (Place::State(s), BuildItem::Facility(k)) => {
                // Ticket #54: a Scrubber takes no build slot, so it is never lost for want of one.
                // Ticket #56: it stands in the row its order reserved, if that row still has room,
                // and in the other one if it does not. A Sea Wall stands on the coast or nowhere.
                let coastal_only = self.tables.facility(k).coastal_only;
                let row = if !self.takes_slot(k) {
                    Some(false)
                } else if b.coastal {
                    if self.free_coastal(s) > 0 {
                        Some(true)
                    } else if !coastal_only && self.free_inland(s) > 0 {
                        Some(false)
                    } else {
                        None
                    }
                } else {
                    self.next_slot_is_coastal(s, k, 0, 0)
                };
                let Some(coastal) = row else {
                    let line = format!("{} at {} had no slot left and was lost.", name, self.place_name(place));
                    self.log(line);
                    let text = self.say("build_lost", &[("building", name.clone()), ("place", self.place_name(place))]);
                    self.report_line(LineKind::Note, Some(place.into()), text);
                    return;
                };
                self.state_mut(s).facilities.push(if coastal { Facility::in_coastal_slot(k) } else { Facility::new(k) });
            }
            (Place::State(s), BuildItem::IndustryLevel) => {
                self.state_mut(s).industry_level += 1;
            }
            // Ticket #68 (version 0.05.5): the Archive Module stands. If its Research is already paid
            // (a fund kept from a destroyed Archive) it is complete at once; otherwise it says what
            // it still wants.
            (Place::Colony(c), BuildItem::Module(ModuleKind::Archive)) => {
                if let Some(col) = self.colony_mut(c)
                    && !col.modules.iter().any(|m| m.kind == ModuleKind::Archive)
                {
                    col.modules.push(Module::new(ModuleKind::Archive));
                }
                let research = self.tables.archive.research;
                let left = (research - self.seat(b.seat).archive_fund).max(0);
                if left == 0 {
                    self.archive_completed(b.seat);
                } else {
                    let line = format!("The {} raised the Archive at {}; it wants {} more Research.", self.seat_name(b.seat), self.place_name(place), left);
                    self.log(line);
                    let text = self.say("archive_built", &[("faction", self.seat_name(b.seat)), ("place", self.place_name(place)), ("left", left.to_string())]);
                    self.report_line(LineKind::Archive, Some(place.into()), text);
                }
                return;
            }
            (Place::Colony(c), BuildItem::Module(k)) => {
                if let Some(col) = self.colony_mut(c) {
                    col.modules.push(Module::new(k));
                }
            }
            (_, BuildItem::Unit(UnitKind::Army)) => {
                // Ticket #270 (version 0.08.4): raised through the one door, and named there.
                self.raise_army(place, false);
                if let Some(s) = self.place_director(place) {
                    self.war.armies_built[s.index()] += 1;
                }
            }
            (_, BuildItem::Unit(kind)) => {
                let body = match place {
                    Place::State(_) => BodyId::Earth,
                    Place::Colony(c) => self.colony(c).map(|c| c.body).unwrap_or(BodyId::Earth),
                };
                // Ticket #335 (version 0.09.0): **a new Ship starts in the orbit of the Shipyard
                // that built it** -- a station's yard puts it at that station's ring, a ground
                // Colony's yard and a Launch Site on Earth in low orbit.
                let slot = match place {
                    Place::State(_) => None,
                    Place::Colony(c) => self.colony(c).map(|c| self.colony_orbit(c)).unwrap_or(Orbit::Low).slot(),
                };
                let id = ShipId(self.fresh_id());
                // Ticket #210 (version 0.08.1): named at the build, from the list its kind draws
                // from, taking the first name no Ship on the board is using. This is the ONE place a
                // Ship comes into a real game, so it is the one place a name is given.
                let name = self.next_ship_name(kind);
                if kind.is_warship() {
                    self.war.warships_built[b.seat.index()] += 1;
                }
                self.ships.push(Ship {
                    id,
                    name,
                    // Ticket #335 (version 0.09.0): the orbit of the yard that built it.
                    slot,
                    kind,
                    seat: b.seat,
                    damage: 0,
                    at: ShipAt::Body(body),
                    colonists: 0,
                    // Ticket #189 (version 0.08.0): empty, so the figure is the neutral one.
                    colonists_education: 1.0,
                    army: None,
                    stance: Stance::Hold,
                    escaped: false,
                    arrived_this_turn: false,
                    built_turn: turn,
                    // Ticket #87: built with a full tank, paid at the build.
                    fuel: self.tables.unit(kind).tank,
                });
            }
            _ => {}
        }
        let line = format!("{} completed {} at {}.", self.seat_name(b.seat), name, self.place_name(place));
        self.log(line);
        let text = self.say("build_complete", &[("faction", self.seat_name(b.seat)), ("building", name.clone()), ("place", self.place_name(place))]);
        self.report_line_of(b.seat, LineKind::YourBuild, LineKind::BuildComplete, Some(place.into()), text);
        self.ai_deed(b.seat, "completed", &[("building", name.clone()), ("place", self.place_name(place))]);
    }

    // ------------------------------------------------------------------ (f)

    fn resolve_repairs(&mut self) {
        let repairs = std::mem::take(&mut self.pending.repairs);
        for (seat, unit, points) in repairs {
            match unit {
                UnitRef::Ship(id) => {
                    if let Some(s) = self.ship_mut(id).filter(|s| matches!(s.at, ShipAt::Body(_))) {
                        s.damage = s.damage.saturating_sub(points);
                    }
                }
                UnitRef::Army(id) => {
                    if let Some(a) = self.army_mut(id) {
                        a.damage = a.damage.saturating_sub(points);
                    }
                }
                // Ticket #324 (version 0.08.8): a Battery, at its own Colony.
                UnitRef::Battery { colony, index } => {
                    if let Some(m) = self.colony_mut(colony).and_then(|c| c.modules.get_mut(index)).filter(|m| m.kind == ModuleKind::Battery) {
                        m.damage = m.damage.saturating_sub(points);
                    }
                }
            }
            self.log(format!("{} repaired {} point(s).", self.seat_name(seat), points));
        }
    }

    // ------------------------------------------------------------------ (g)

    /// Ticket #73 (version 0.05.5): Emigrants sent by sea land in Antarctica the turn after they
    /// left: into the free slot they were bound for, founding a Colony, or into the Faction's own
    /// Colony there while it has room. If the slot was taken meanwhile they try the Faction's own
    /// Antarctic Colony; if nothing has room they come home to the state they left.
    fn resolve_antarctic(&mut self) {
        let turn = self.turn;
        let due: Vec<AntarcticSend> = self.antarctic_sends.iter().filter(|s| s.due_turn <= turn).cloned().collect();
        self.antarctic_sends.retain(|s| s.due_turn > turn);
        for s in due {
            let landed = match s.into {
                UnloadTarget::Slot(_, slot) if self.antarctica_open && self.free_slots_on(BodyId::Earth).contains(&slot) => {
                    self.found_antarctic_colony(s.seat, slot, s.n, s.from, s.education);
                    true
                }
                UnloadTarget::Colony(c) => self.join_antarctic_colony(s.seat, c, s.n, s.from, s.education),
                UnloadTarget::Slot(..) => {
                    let own = self
                        .colonies
                        .iter()
                        .find(|c| c.body == BodyId::Earth && !c.in_orbit && c.control.director() == Some(s.seat) && self.habitat_room(c) > c.colonists)
                        .map(|c| c.id);
                    match own {
                        Some(c) => self.join_antarctic_colony(s.seat, c, s.n, s.from, s.education),
                        None => false,
                    }
                }
            };
            if !landed {
                // Ticket #189 (version 0.08.0): they come home knowing what they left knowing.
                let blended = Game::blend(self.state(s.from).emigrants, self.state(s.from).emigrants_education, s.n, s.education);
                let st = self.state_mut(s.from);
                st.emigrants += s.n;
                st.emigrants_education = blended;
                let line = format!("{} Pioneers from {} found no room in Antarctica and came home.", s.n, self.tables.state(s.from).name);
                self.log(line);
                let text = self.say("emigrants_returned", &[("n", s.n.to_string()), ("state", self.tables.state(s.from).name.clone())]);
                self.report_line_of(s.seat, LineKind::YourWorks, LineKind::Note, Some(ReportPlace::State(s.from)), text);
            }
        }
    }

    /// Ticket #73: Emigrants found a Colony in a free Antarctic slot, as a Colony Ship's unload does.
    fn found_antarctic_colony(&mut self, seat: Seat, slot: u32, n: u32, from: StateId, taught: f64) {
        let id = ColonyId(self.fresh_id());
        self.colonies.push(Colony {
            id,
            body: BodyId::Earth,
            slot,
            control: Control::Controlled(seat),
            // Ticket #164 (version 0.07.5): a founding gives the Core Module, where it gave a
            // free Habitat. It holds four, so a Colony founded by sea takes its four at once.
            modules: vec![Module::new(ModuleKind::Core)],
            colonists: 0,
            // Ticket #189 (version 0.08.0): a bare Colony knows nothing until its people arrive,
            // one line below.
            education: 1.0,
            settler_education: 1.0,
            queue: Vec::new(),
            grid_failed: false,
            founded_turn: self.turn,
            in_orbit: false,
        });
        let room = self.habitat_room(self.colony(id).unwrap());
        let moved = n.min(room);
        // Ticket #189: the founders bring their schooling with them.
        self.settle_people(id, moved, taught);
        if moved < n {
            let back = n - moved;
            let blended = Game::blend(self.state(from).emigrants, self.state(from).emigrants_education, back, taught);
            let st = self.state_mut(from);
            st.emigrants += back;
            st.emigrants_education = blended;
        }
        let slot_name = self.tables.body(BodyId::Earth).slots[slot as usize].name.clone();
        let line = format!("The {} founded a Colony at {} in Antarctica with {} Pioneers from {}.", self.seat_name(seat), slot_name, moved, self.tables.state(from).name);
        self.log(line);
        let text = self.say(
            "colony_founded",
            &[("faction", self.seat_name(seat)), ("slot", (slot + 1).to_string()), ("body", self.tables.body(BodyId::Earth).name.clone()), ("n", moved.to_string())],
        );
        self.report_line(LineKind::ColonyFounded, Some(ReportPlace::Colony(id)), text);
        let antarctic = self.colonies.iter().filter(|c| c.control.director() == Some(seat) && c.body == BodyId::Earth && !c.in_orbit).count();
        // Ticket #85: Antarctica is on Earth, so its founding has phrases of its own, the count an ordinal.
        let note = if antarctic <= 1 {
            self.phrase("first_antarctic_colony", &[])
        } else {
            self.phrase("more_antarctic_colonies", &[("ordinal", crate::report::ordinal(antarctic))])
        };
        self.moment(
            MomentKind::ColonyFounded,
            &[("faction", self.seat_name(seat)), ("colony", self.place_name(Place::Colony(id))), ("note", note), ("n", moved.to_string())],
            Some(ReportPlace::Colony(id)),
        );
        self.ai_deed(seat, "founded", &[("colony", self.place_name(Place::Colony(id)))]);
    }

    /// Ticket #73: Emigrants join the seat's own Antarctic Colony while it has room; the rest go home.
    fn join_antarctic_colony(&mut self, seat: Seat, c: ColonyId, n: u32, from: StateId, taught: f64) -> bool {
        let Some(col) = self.colony(c) else { return false };
        if col.body != BodyId::Earth || col.in_orbit || col.control.director() != Some(seat) {
            return false;
        }
        let room = self.habitat_room(col).saturating_sub(col.colonists);
        let moved = n.min(room);
        if moved == 0 {
            return false;
        }
        // Ticket #189 (version 0.08.0): the arrivals average into what the Colony already knows.
        self.settle_people(c, moved, taught);
        if moved < n {
            let back = n - moved;
            let blended = Game::blend(self.state(from).emigrants, self.state(from).emigrants_education, back, taught);
            let st = self.state_mut(from);
            st.emigrants += back;
            st.emigrants_education = blended;
        }
        let line = format!("{} Pioneers from {} landed in Antarctica and joined {}.", moved, self.tables.state(from).name, self.place_name(Place::Colony(c)));
        self.log(line);
        let text = self.say("emigrants_arrived", &[("n", moved.to_string()), ("state", self.tables.state(from).name.clone()), ("colony", self.place_name(Place::Colony(c)))]);
        self.report_line(LineKind::Antarctica, Some(ReportPlace::Colony(c)), text);
        true
    }

    fn resolve_cargo(&mut self) {
        let cargo = std::mem::take(&mut self.pending.cargo);
        // Ticket #46: stations ordered this turn, one per orbital slot. Ticket #50: more than one
        // seat for one slot is settled at the Body, ties drawn at random.
        let stations = std::mem::take(&mut self.pending.stations);
        let mut slots_done: Vec<(BodyId, u32)> = Vec::new();
        for (seat, body, slot) in stations.iter() {
            if self.station_at(*body, *slot).is_some() || slots_done.contains(&(*body, *slot)) {
                continue;
            }
            slots_done.push((*body, *slot));
            let mut contenders: Vec<Seat> = Vec::new();
            for (s2, b2, sl2) in stations.iter() {
                if b2 == body && sl2 == slot && !contenders.contains(s2) {
                    contenders.push(*s2);
                }
            }
            let seat = &if contenders.len() > 1 { self.tiebreak_at_body(*body, &contenders) } else { *seat };
            let id = ColonyId(self.fresh_id());
            // Ticket #164 (version 0.07.5): a station is founded with its Core Module, so it can take
            // four people the turn it stands, where a bare one could hold nobody until a Habitat was
            // built out of an allowance it no longer has.
            self.colonies.push(Colony { id, body: *body, slot: *slot, control: Control::Controlled(*seat), modules: vec![Module::new(ModuleKind::Core)], colonists: 0, education: 1.0, settler_education: 1.0, queue: Vec::new(), grid_failed: false, founded_turn: self.turn, in_orbit: true });
            let line = format!("{} built {}.", self.seat_name(*seat), self.place_name(Place::Colony(id)));
            self.log(line);
            let text = self.say("station_built", &[("faction", self.seat_name(*seat)), ("station", self.place_name(Place::Colony(id)))]);
            self.report_line_of(*seat, LineKind::YourBuild, LineKind::BuildComplete, Some(ReportPlace::Colony(id)), text);
        }
        // Founding orders into the same Colony Slot from more than one seat are decided at the Body,
        // ties drawn at random (ticket #50).
        let mut founding: Vec<(Seat, ShipId, BodyId, u32)> = Vec::new();
        for (seat, order) in &cargo {
            if let Order::Unload { ship, into: UnloadTarget::Slot(b, slot), .. } = order {
                founding.push((*seat, *ship, *b, *slot));
            }
        }
        let mut blocked: Vec<ShipId> = Vec::new();
        let mut contested: Vec<(BodyId, u32)> = Vec::new();
        for (_, _, body, slot) in founding.clone() {
            if contested.contains(&(body, slot)) {
                continue;
            }
            let mut contenders: Vec<Seat> = Vec::new();
            for (s, _, b, sl) in &founding {
                if *b == body && *sl == slot && !contenders.contains(s) {
                    contenders.push(*s);
                }
            }
            if contenders.len() < 2 {
                continue;
            }
            contested.push((body, slot));
            let winner = self.tiebreak_at_body(body, &contenders);
            for (s, ship, b, sl) in &founding {
                if *b == body && *sl == slot && *s != winner {
                    blocked.push(*ship);
                }
            }
        }
        for (seat, order) in cargo {
            match order {
                Order::Load { ship, colonists, from, army } => {
                    let Some(s) = self.ship(ship) else { continue };
                    let ShipAt::Body(body) = s.at else { continue };
                    if colonists > 0 {
                        match from {
                            LoadSource::State(st) => {
                                if self.state(st).control.director() != Some(seat) {
                                    continue;
                                }
                                // Ticket #73: the lift takes the Emigrants waiting there; the
                                // population was paid when they mustered.
                                if self.state(st).emigrants < colonists {
                                    continue;
                                }
                                // Ticket #189 (version 0.08.0): what they know goes aboard with them.
                                let taught = self.take_emigrants(st, colonists);
                                // Ticket #183 (version 0.08.0): a lift onto a Ship is a launch too.
                                self.pay_spaceport(seat, st, colonists);
                                self.load_people(ship, colonists, taught);
                            }
                            LoadSource::Colony(c) => {
                                if self.colony(c).map(|col| col.colonists < colonists).unwrap_or(true) {
                                    continue;
                                }
                                let taught = self.take_colonists(c, colonists);
                                self.load_people(ship, colonists, taught);
                            }
                        }
                    }
                    if let Some(aid) = army {
                        let ok = self.army(aid).map(|a| matches!(a.at, ArmyAt::Place(_)) && self.army_seat(a) == Some(seat)).unwrap_or(false);
                        if ok {
                            if let Some(a) = self.army_mut(aid) {
                                a.at = ArmyAt::Aboard(ship);
                                a.move_to = None;
                            }
                            if let Some(s) = self.ship_mut(ship) {
                                s.army = Some(aid);
                            }
                        }
                    }
                    let cargo: String = if colonists > 0 { format!("{colonists} Colonists") } else { "an Army".into() };
                    let line = format!("{} loaded {} at {}.", self.seat_name(seat), cargo, self.tables.body(body).name);
                    self.log(line);
                    let text = self.say(
                        "loaded",
                        &[("faction", self.seat_name(seat)), ("cargo", cargo), ("body", self.tables.body(body).name.clone())],
                    );
                    self.report_line_of(seat, LineKind::YourWorks, LineKind::Ship, Some(ReportPlace::Body(body)), text);
                }
                Order::Unload { ship, colonists, army, into } => {
                    if blocked.contains(&ship) {
                        let line = format!("{}: another Faction took that Colony Slot first.", self.seat_name(seat));
                        self.log(line);
                        let text = self.say("slot_taken", &[("faction", self.seat_name(seat))]);
                        self.report_line(LineKind::Ship, None, text);
                        continue;
                    }
                    let Some(s) = self.ship(ship) else { continue };
                    let ShipAt::Body(body) = s.at else { continue };
                    // Ticket #99 (version 0.07.0): the ground answers to Orbital Control, a station
                    // only to a warship sitting in its own Orbital Slot. A Faction is never shut out
                    // of a place it holds by a ship that never touched it.
                    // Ticket #335 (version 0.09.0): and the Ship is in the orbit that touches the
                    // place -- low orbit for the ground, a station's own ring for the station. The
                    // gate said so when the order was given; it is read again here because the
                    // orbit changes and the Battles have run since.
                    let barred = match into {
                        UnloadTarget::Colony(cid) => !self.may_unload_into(seat, cid) || !self.colony(cid).map(|c| self.ship_may_touch(s, c)).unwrap_or(false),
                        UnloadTarget::Slot(_, _) => !self.may_land(seat, body) || !self.ship_in_orbit(s, body, Orbit::Low),
                    };
                    if barred {
                        let line = format!("{} could not land at {}: the orbit is contested.", self.seat_name(seat), self.tables.body(body).name);
                        self.log(line);
                        let text = self.say("landing_contested", &[("faction", self.seat_name(seat)), ("body", self.tables.body(body).name.clone())]);
                        self.report_line(LineKind::Ship, Some(ReportPlace::Body(body)), text);
                        continue;
                    }
                    let aboard_army = s.army;
                    match into {
                        UnloadTarget::Slot(b, slot) => {
                            // Ticket #56: Antarctica is shut until the ice opens at +1.6 C.
                            if b != body || !self.free_slots_on(b).contains(&slot) || colonists == 0 || (b == BodyId::Earth && !self.antarctica_open) {
                                continue;
                            }
                            let n = colonists.min(self.ship(ship).map(|s| s.colonists).unwrap_or(0));
                            let id = ColonyId(self.fresh_id());
                            self.colonies.push(Colony {
                                id,
                                body: b,
                                slot,
                                control: Control::Controlled(seat),
                                // Ticket #164 (version 0.07.5): the Core Module in the free
                                // Habitat's place. It holds four, which is a Colony Ship's load.
                                modules: vec![Module::new(ModuleKind::Core)],
                                colonists: 0,
                                // Ticket #189 (version 0.08.0): it knows nothing until its people land.
                                education: 1.0,
                                settler_education: 1.0,
                                queue: Vec::new(),
                                grid_failed: false,
                                founded_turn: self.turn,
                                in_orbit: false,
                            });
                            let room = self.habitat_room(self.colony(id).unwrap());
                            let moved = n.min(room);
                            // Ticket #189: the founders bring what they know.
                            let taught = self.unload_people(ship, moved);
                            self.settle_people(id, moved, taught);
                            if let Some(aid) = aboard_army.filter(|_| army) {
                                self.land_army(aid, ship, Place::Colony(id));
                            }
                            let line = format!("The {} founded a Colony in slot {} on {} with {} Colonists.", self.seat_name(seat), slot + 1, self.tables.body(b).name, moved);
                            self.log(line);
                            let text = self.say(
                                "colony_founded",
                                &[
                                    ("faction", self.seat_name(seat)),
                                    ("slot", (slot + 1).to_string()),
                                    ("body", self.tables.body(b).name.clone()),
                                    ("n", moved.to_string()),
                                ],
                            );
                            self.report_line(LineKind::ColonyFounded, Some(ReportPlace::Colony(id)), text);
                            let off_earth =
                                self.colonies.iter().filter(|c| c.control.director() == Some(seat) && c.body != BodyId::Earth && !c.in_orbit).count();
                            // Ticket #85: the count is an ordinal.
                            let note = if off_earth <= 1 {
                                self.phrase("first_colony", &[])
                            } else {
                                self.phrase("more_colonies", &[("ordinal", crate::report::ordinal(off_earth))])
                            };
                            self.moment(
                                MomentKind::ColonyFounded,
                                &[
                                    ("faction", self.seat_name(seat)),
                                    ("colony", self.place_name(Place::Colony(id))),
                                    ("note", note),
                                    ("n", moved.to_string()),
                                ],
                                Some(ReportPlace::Colony(id)),
                            );
                            self.ai_deed(seat, "founded", &[("colony", self.place_name(Place::Colony(id)))]);
                        }
                        UnloadTarget::Colony(cid) => {
                            let Some(col) = self.colony(cid) else { continue };
                            if col.body != body {
                                continue;
                            }
                            if colonists > 0 && col.control.director() == Some(seat) {
                                let room = self.habitat_room(col).saturating_sub(col.colonists);
                                let n = colonists.min(room).min(self.ship(ship).map(|s| s.colonists).unwrap_or(0));
                                if let Some(c) = self.colony_mut(cid) {
                                    c.colonists += n;
                                }
                                if let Some(s) = self.ship_mut(ship) {
                                    s.colonists -= n;
                                }
                                let line = format!("{} Colonists disembarked into {}.", n, self.place_name(Place::Colony(cid)));
                                self.log(line);
                                let text = self.say("disembarked", &[("n", n.to_string()), ("colony", self.place_name(Place::Colony(cid)))]);
                                self.report_line_of(seat, LineKind::YourWorks, LineKind::Ship, Some(ReportPlace::Colony(cid)), text);
                            }
                            if let Some(aid) = aboard_army.filter(|_| army) {
                                self.war.armies_landed[seat.index()] += 1;
                                self.land_army(aid, ship, Place::Colony(cid));
                                // Ticket #339 (version 0.09.0): the Army that came down, by name.
                                let name = self.army(aid).map(|a| self.army_name(a)).unwrap_or_else(|| "the Army".to_string());
                                let line = format!("{} ({}) landed at {}.", name, self.seat_name(seat), self.place_name(Place::Colony(cid)));
                                self.log(line);
                                let text = self.say("army_landed", &[("army", name), ("faction", self.seat_name(seat)), ("colony", self.place_name(Place::Colony(cid)))]);
                                self.report_line(LineKind::Army, Some(ReportPlace::Colony(cid)), text);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // ------------------------------------------------------------------ (i), ticket #52

    /// Ticket #176 (version 0.07.6): one line per Region, in **net**, spoken only when the net is
    /// worth at least `report_net_floor` of a person. The designer: *"reduce report clutter by
    /// reporting only net migration from refugees and only when migration occurs"*, and, asked how:
    /// one line per Region, half a person as the floor, the drafted wording, and the largest cause
    /// kept.
    ///
    /// Measured before the change, over ten computer-played games: the worst turn spent **39 of its
    /// 74 Report lines** on refugees, the median turn 12, and refugees were **30% of the median
    /// Report**. A Region spoke once per cause and once more for arriving, so one that lost people
    /// to the sea and to the heat spoke twice, and one that took ten and sent ten away spoke twice
    /// while netting nothing.
    ///
    /// `arrived` is the GROSS that came in and `rose` what Unrest did about it, both charged on the
    /// gross by the rule. Where the gross differs from the net the line names it, because otherwise
    /// the Unrest figure would be unexplained -- it is the one place the Report says why Unrest rose.
    fn report_net_migration(&mut self, sid: StateId, arrived: f64, rose: f64) {
        let left: f64 = self.state(sid).refugees_out.iter().map(|(_, n)| *n).sum();
        let net = arrived - left;
        if net.abs() < self.tables.unrest.report_net_floor {
            return;
        }
        let state = self.tables.state(sid).name.clone();
        let text = if net > 0.0 {
            let n = format!("{net:.1}");
            let gross = format!("{arrived:.1}");
            match (rose > 0.0, (arrived - net).abs() >= self.tables.unrest.report_net_floor) {
                // Unrest rose, and some of what arrived was cancelled by what left: name both, or
                // the Unrest figure is charged on a number the line never gives.
                (true, true) => self.say(
                    "refugees_net_in_gross",
                    &[("n", n), ("gross", gross), ("state", state), ("rose", Game::unrest_figure(rose).to_string()), ("unrest", self.unrest_text(sid))],
                ),
                (true, false) => self.say(
                    "refugees_net_in",
                    &[("n", n), ("state", state), ("rose", Game::unrest_figure(rose).to_string()), ("unrest", self.unrest_text(sid))],
                ),
                // Too few to move Unrest, or Unrest already at the ceiling: the arrival alone.
                (false, _) => self.say("refugees_net_in_quiet", &[("n", n), ("state", state)]),
            }
        } else {
            // The largest cause survives, at the designer's word: *why* is the most interesting word
            // in the old line and the cheapest to keep. `mostly` only where there was more than one.
            let out = &self.state(sid).refugees_out;
            let why = out.iter().max_by(|a, b| a.1.total_cmp(&b.1)).map(|(c, _)| c.clone()).unwrap_or_default();
            let key = if out.len() > 1 { "refugees_net_out_mostly" } else { "refugees_net_out" };
            self.say(key, &[("n", format!("{:.1}", -net)), ("state", state), ("why", why)])
        };
        self.report_line(LineKind::Refugees, Some(ReportPlace::State(sid)), text);
    }

    /// Unrest settles last, once every rise of the turn is in: the refugees the turn's flows
    /// brought, then the falls (Relief, a Constabulary, and the natural fall in a turn nothing
    /// raised it), then the throw-off, then the Report lines for crossing a threshold.
    /// Ticket #268 (version 0.08.4): **carbon credits.** The Custodians' offer for the turns to come
    /// is set from their order; then every purchase is filled first come first served out of this
    /// turn's offer. A ppm bought comes off the buyer's ledger for good and off the seller's credit
    /// -- and past what the seller held, onto the seller's ledger as Blame taken, at the designer's
    /// word: "any amount and take the blame". A buyer left short gets the Ducats back for what it
    /// did not get; the rest land with the Custodians. A purchase is an act of friendship both ways.
    pub fn resolve_credits(&mut self) {
        for (seat, ppm) in std::mem::take(&mut self.pending.credit_offers) {
            self.seat_mut(seat).credits_offered = ppm;
            let text = self.say("credits_offered", &[("faction", self.seat_name(seat)), ("n", ppm.to_string())]);
            self.report_line_of(seat, LineKind::YourWorks, LineKind::Note, None, text);
            self.ai_deed(seat, "offer_credits", &[("n", ppm.to_string())]);
        }
        let Some(seller) = self.credit_seller() else { return };
        let mut left = self.seat(seller).credits_offered;
        for (buyer, ppm, paid) in std::mem::take(&mut self.pending.credit_buys) {
            let take = ppm.min(left).max(0);
            let kept = if ppm > 0 { paid * take / ppm } else { 0 };
            let back = paid - kept;
            if back > 0 {
                self.seat_mut(buyer).stockpile.ducats += back;
                let text = self.say("credits_short", &[("faction", self.seat_name(buyer)), ("n", (ppm - take).to_string()), ("back", back.to_string())]);
                self.report_line_of(buyer, LineKind::YourWorks, LineKind::Note, None, text);
            }
            if take <= 0 {
                continue;
            }
            left -= take;
            self.seat_mut(buyer).credits_bought += take as f64;
            self.seat_mut(seller).credits_sold += take as f64;
            self.seat_mut(seller).stockpile.ducats += kept;
            self.credit(buyer, seller);
            self.credit(seller, buyer);
            let (who, whom) = (self.seat_name(buyer), self.seat_name(seller));
            self.log(format!("The {who} bought {take} ppm of carbon credit from the {whom} for {kept} Ducats."));
            let text = self.say("credits_bought", &[("faction", who), ("n", take.to_string()), ("seller", whom), ("ducats", kept.to_string())]);
            self.report_line(LineKind::Note, None, text);
            self.ai_deed(buyer, "buy_credits", &[("n", take.to_string())]);
        }
    }

    pub fn resolve_unrest(&mut self) {
        let u = self.tables.unrest.clone();
        // The refugees the turn's flows brought, charged once so the per-turn cap counts them all.
        // Ticket #176 (version 0.07.6): Unrest is charged on the GROSS arrivals, as the rule has
        // always had it -- a Region that takes ten people and sends ten away has still absorbed ten
        // people's worth of grievance -- and only the Report's line speaks in net.
        for sid in StateId::ALL {
            let arrived = self.state(sid).refugees_in;
            let rose = if arrived > 0.0 {
                let want = (arrived / u.refugees_per).floor().min(u.refugees_max);
                let r = self.raise_unrest(sid, want, UnrestSource::Refugees);
                if r > 0.0 {
                    let line = format!("{:.1} people arrived in {}; Unrest rose by {} to {}.", arrived, self.tables.state(sid).name, Game::unrest_figure(r), self.unrest_text(sid));
                    self.log(line);
                }
                r
            } else {
                0.0
            };
            self.report_net_migration(sid, arrived, rose);
        }
        // Resettle (rule 9): the Standing the chosen state gives its Faction.
        for (seat, sid) in std::mem::take(&mut self.pending.resettle) {
            let gain = u.resettle_standing;
            *self.seat_mut(seat).influence.entry(Place::State(sid)).or_insert(0) += gain;
            let line = format!("The {} resettled this turn's refugees in {} (+{} Standing there).", self.seat_name(seat), self.tables.state(sid).name, gain);
            self.log(line);
            let text = self.say(
                "resettled",
                &[("faction", self.seat_name(seat)), ("state", self.tables.state(sid).name.clone()), ("standing", gain.to_string())],
            );
            self.report_line(LineKind::Refugees, Some(ReportPlace::State(sid)), text);
        }
        // Ticket #267 (version 0.08.4): Smear campaigns land -- ppm on the target's ledger for good,
        // an offence at an Influence push's weight, and a Report line naming who paid.
        for (seat, target, amount) in std::mem::take(&mut self.pending.smears) {
            let ppm = amount as f64 * self.tables.influence.smear.ppm_per_influence;
            self.seat_mut(target).blame_smeared += ppm;
            self.offend_by(seat, target, 1);
            let (who, whom) = (self.seat_name(seat), self.seat_name(target));
            self.log(format!("The {who} smeared the {whom}: {ppm:.0} ppm laid on their Blame."));
            let text = self.say("smear", &[("faction", who), ("target", whom.clone()), ("ppm", format!("{ppm:.0}"))]);
            self.report_line(LineKind::Note, None, text);
            self.ai_deed(seat, "smear", &[("n", amount.to_string()), ("faction", whom)]);
        }
        // Ticket #277 (version 0.08.5): Greenwash campaigns land -- ppm off the seat's own ledger for
        // good, no offence, and a public Report line, so a rival can answer with a Smear.
        for (seat, amount) in std::mem::take(&mut self.pending.greenwashes) {
            let ppm = amount as f64 * self.tables.influence.greenwash.ppm_per_influence;
            self.seat_mut(seat).blame_cleaned += ppm;
            let who = self.seat_name(seat);
            self.log(format!("The {who} greenwashed: {ppm:.0} ppm off their Blame."));
            let text = self.say("greenwash", &[("faction", who), ("ppm", format!("{ppm:.0}"))]);
            self.report_line(LineKind::Note, None, text);
            self.ai_deed(seat, "greenwash", &[("n", amount.to_string())]);
        }
        // Ticket #269 (version 0.08.4): Agitate lands before Relief, so a holder's Relief the same
        // turn answers it. One point, damped by a working Constabulary, an offence against the
        // holder, and a Report line naming who paid.
        for (seat, sid) in std::mem::take(&mut self.pending.agitates) {
            let Some(holder) = self.place_control(Place::State(sid)).controller() else { continue };
            let rose = self.raise_unrest(sid, u.agitate_points, UnrestSource::Agitate);
            self.offend_by(seat, holder, 1);
            self.seat_mut(seat).agitates_issued += 1;
            let (who, name) = (self.seat_name(seat), self.tables.state(sid).name.clone());
            let text = if rose > 0.0 {
                self.log(format!("The {who} agitated in {name}: Unrest rose by {} to {}.", Game::unrest_figure(rose), self.unrest_text(sid)));
                self.say("agitate", &[("faction", who), ("state", name.clone()), ("rose", Game::unrest_figure(rose).to_string()), ("unrest", self.unrest_text(sid))])
            } else {
                self.log(format!("The {who} agitated in {name}; the Constabulary held it to nothing."));
                self.say("agitate_damped", &[("faction", who), ("state", name.clone())])
            };
            self.report_line(LineKind::Unrest, Some(ReportPlace::State(sid)), text);
            self.ai_deed(seat, "agitate", &[("state", name)]);
        }
        // Relief (rule 3): one point per order, paid for in Ducats at the Orders phase.
        let mut relieved: Vec<(Seat, StateId, f64)> = Vec::new();
        for (seat, sid) in std::mem::take(&mut self.pending.relief) {
            let fell = self.lower_unrest(sid, u.relief_points);
            match relieved.iter_mut().find(|(s, x, _)| *s == seat && *x == sid) {
                Some((_, _, n)) => *n += fell,
                None => relieved.push((seat, sid, fell)),
            }
        }
        for (seat, sid, fell) in relieved.into_iter().filter(|(_, _, n)| *n > 0.0) {
            let line = format!("The {} paid Relief in {}: Unrest fell by {} to {}.", self.seat_name(seat), self.tables.state(sid).name, Game::unrest_figure(fell), self.unrest_text(sid));
            self.log(line);
            let text = self.say(
                "relief",
                &[
                    ("faction", self.seat_name(seat)),
                    ("state", self.tables.state(sid).name.clone()),
                    ("fell", Game::unrest_figure(fell).to_string()),
                    ("unrest", self.unrest_text(sid)),
                ],
            );
            self.report_line(LineKind::Unrest, Some(ReportPlace::State(sid)), text);
        }
        // What calms a state by standing in it (a Constabulary now, a Scrubber later), then the
        // natural fall. Ticket #53: the fall lands every turn, whatever else happened, so a rise
        // and the fall net out; only a state that changed hands this turn goes without it.
        for sid in StateId::ALL {
            let calm = self.calming_fall(sid);
            self.lower_unrest(sid, calm);
            if !self.state(sid).changed_hands {
                self.lower_unrest(sid, u.natural_fall);
            }
        }
        // The throw-off (rule 5), after the falls: a Faction can still buy a state back from the brink.
        for sid in StateId::ALL {
            if self.state(sid).unrest < u.throw_off_threshold {
                continue;
            }
            let Control::Controlled(seat) = self.state(sid).control else { continue };
            self.throw_off(sid, seat);
        }
        // The Report lines for crossing 4, 7 and 10, once each way.
        for sid in StateId::ALL {
            let now = self.state(sid).unrest;
            let was = self.state(sid).unrest_reported;
            for line in [u.army_threshold, u.facility_threshold] {
                if now >= line && was < line {
                    let text = format!("{}: Unrest reached {} - {}.", self.tables.state(sid).name, self.unrest_text(sid), self.unrest_note(sid));
                    self.log(text);
                    let said = self.say(
                        "unrest_threshold",
                        &[("state", self.tables.state(sid).name.clone()), ("unrest", self.unrest_text(sid)), ("note", self.unrest_note(sid))],
                    );
                    self.report_line(LineKind::Unrest, Some(ReportPlace::State(sid)), said);
                    break;
                }
            }
            let st = self.state_mut(sid);
            st.unrest_reported = now;
            st.changed_hands = false;
            st.refugees_in = 0.0;
        }
    }

    /// Ticket #52, rule 5: a controlled state at 10 throws its controller off. It goes neutral,
    /// every Faction's Standing stays, the Armies standing there become the state's own, its build
    /// queue is kept, and its Unrest settles back to 5.
    fn throw_off(&mut self, sid: StateId, seat: Seat) {
        let u = &self.tables.unrest;
        let back = u.throw_off_reset;
        self.destroy_scrubbers(sid, "threw off its controller");
        self.state_mut(sid).control = Control::Neutral;
        self.state_mut(sid).unrest = back;
        // Ticket #53: a state that is thrown off counts six fresh turns of neutrality.
        self.restart_neutrality_clock(sid);
        let ids: Vec<ArmyId> = self.armies.iter().filter(|a| a.at == ArmyAt::Place(Place::State(sid))).map(|a| a.id).collect();
        for id in ids {
            if let Some(a) = self.army_mut(id) {
                a.home = ArmyHome::State(sid);
                a.stance = Stance::Hold;
                a.move_to = None;
            }
        }
        let line = format!(
            "{} threw off the {}: it is neutral again, its Armies are its own, and its Unrest settles at {}.",
            self.tables.state(sid).name,
            self.seat_name(seat),
            Game::unrest_figure(back)
        );
        self.log(line);
        let text = self.say(
            "threw_off",
            &[
                ("state", self.tables.state(sid).name.clone()),
                ("faction", self.seat_name(seat)),
                ("unrest", Game::unrest_figure(back).to_string()),
            ],
        );
        self.report_line(LineKind::ControlChanged, Some(ReportPlace::State(sid)), text);
        self.moment(
            MomentKind::ControlChanged,
            &[("place", self.tables.state(sid).name.clone()), ("faction", "nobody".to_string())],
            Some(ReportPlace::State(sid)),
        );
    }

    /// Ticket #52: what one turn of Occupation adds to the occupier's Standing (spec 8.5): the
    /// place's threshold over three, rounded up, and over six while the state's Unrest is at 4 or
    /// more, so a restive population is half as easy to Pacify.
    pub fn pacification_gain(&self, place: Place) -> i64 {
        let u = &self.tables.unrest;
        let restive = matches!(place, Place::State(s) if self.state(s).unrest >= u.pacification_unrest);
        let divisor = if restive { u.pacification_divisor_unrest } else { u.pacification_divisor };
        let threshold = self.influence_threshold(place);
        if divisor <= 0 { threshold } else { (threshold + divisor - 1) / divisor }
    }

    /// Ticket #300 (version 0.08.6): the stance is inferred from whose place it is, no new field --
    /// Attack at a Colony this seat does not direct, which is what the landing button says, Hold at
    /// its own -- and the Army is remembered for the second ground pass of this Resolution.
    fn land_army(&mut self, aid: ArmyId, ship: ShipId, place: Place) {
        let seat = self.army(aid).and_then(|a| self.army_seat(a));
        let attacking = seat.is_some() && self.place_director(place) != seat;
        if let Some(a) = self.army_mut(aid) {
            a.at = ArmyAt::Place(place);
            a.stance = if attacking { Stance::Attack } else { Stance::Hold };
        }
        if let Some(s) = self.ship_mut(ship) {
            s.army = None;
        }
        self.pending.landed.push(aid);
    }
}

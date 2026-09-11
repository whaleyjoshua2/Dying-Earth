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
        self.resolve_occupation(); // (c)
        self.resolve_influence(); // (d)
        self.resolve_changes(); // (e), ticket #54: a decommission frees its slot before a build wants it
        self.resolve_builds(); // (e)
        self.resolve_repairs(); // (f)
        self.resolve_cargo(); // (g)
        self.apply_event_now(); // (h)
        self.resolve_strip_permits(); // (h), ticket #54: a permit that ran out charges its price
        self.resolve_unrest(); // (i), ticket #52
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
        let storm = self.event_is(EventId::SolarStorm) && !self.has_tech(TechId::EfficientTransit);
        let mut arrivals: Vec<(Seat, BodyId, ShipId)> = Vec::new();
        for s in &mut self.ships {
            if let ShipAt::Transit { from, to, turns_left } = s.at {
                if storm {
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
        for (seat, body, id) in &arrivals {
            let kind = self.ship(*id).map(|s| s.kind.name()).unwrap_or("Ship").to_string();
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
        // enemy stack that turn.
        for body in BodyId::ALL {
            for seat in Seat::ALL {
                let interceptors: Vec<ShipId> = self
                    .ships
                    .iter()
                    .filter(|s| s.seat == seat && s.at == ShipAt::Body(body) && s.stance == Stance::Intercept && !s.escaped)
                    .map(|s| s.id)
                    .collect();
                if interceptors.is_empty() {
                    continue;
                }
                let mut parties: Vec<(Seat, bool, Vec<ShipId>)> = vec![(seat, true, interceptors)];
                for other in seat.others() {
                    let arriving: Vec<ShipId> = arrivals
                        .iter()
                        .filter(|(s, b, id)| *s == other && *b == body && self.ship(*id).map(|x| !x.escaped).unwrap_or(false))
                        .map(|(_, _, id)| *id)
                        .collect();
                    if !arriving.is_empty() {
                        parties.push((other, false, arriving));
                    }
                }
                if parties.len() < 2 {
                    continue;
                }
                let name = format!("{} orbit (interception)", self.tables.body(body).name);
                self.ship_melee(&name, body, &parties);
            }
        }
    }

    // ------------------------------------------------------------------ (b)

    fn resolve_battles(&mut self) {
        // Ship battles (ticket #50): any stack ordered Attack pulls every other Faction's Ships at
        // that Body into one melee. Evade stacks still try to disengage; Hold stacks fight.
        for body in BodyId::ALL {
            let aggressors: Vec<Seat> = Seat::ALL
                .into_iter()
                .filter(|seat| self.ships.iter().any(|s| s.seat == *seat && s.at == ShipAt::Body(body) && s.stance == Stance::Attack && !s.escaped))
                .collect();
            if aggressors.is_empty() {
                continue;
            }
            let parties: Vec<(Seat, bool, Vec<ShipId>)> = Seat::ALL
                .into_iter()
                .filter_map(|seat| {
                    let ships: Vec<ShipId> =
                        self.ships.iter().filter(|s| s.seat == seat && s.at == ShipAt::Body(body) && !s.escaped).map(|s| s.id).collect();
                    if ships.is_empty() {
                        None
                    } else {
                        Some((seat, aggressors.contains(&seat), ships))
                    }
                })
                .collect();
            if parties.len() < 2 {
                continue;
            }
            let name = format!("{} orbit", self.tables.body(body).name);
            self.ship_melee(&name, body, &parties);
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
            let a = self.army_mut(id).unwrap();
            a.at = ArmyAt::Place(Place::State(to));
            a.move_to = None;
            if !entering_own {
                a.stance = Stance::Attack;
            }
            let line = format!(
                "{} Army moved from {} to {}{}.",
                self.seat_name(seat),
                self.tables.state(from).name,
                self.tables.state(to).name,
                if entering_own { "" } else { " and attacks" }
            );
            self.log(line);
            let text = self.say(
                "army_moved",
                &[
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
        for place in places {
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
            self.destruction_rolls(place, "attacked");
        }
    }

    pub fn place_director(&self, place: Place) -> Option<Seat> {
        match place {
            Place::State(s) => self.state(s).control.director(),
            Place::Colony(c) => self.colony(c).and_then(|c| c.control.director()),
        }
    }

    /// Armies at a place that fight against `attacker`: every Army not of that seat, not standing down, not escaped.
    pub fn defenders_at(&self, place: Place, attacker: Seat) -> Vec<ArmyId> {
        self.armies
            .iter()
            .filter(|a| a.at == ArmyAt::Place(place) && self.army_seat(a) != Some(attacker) && !self.army_stands_down(a) && !a.escaped)
            .filter(|a| self.army_strength(a) > 0 || !a.standing)
            .map(|a| a.id)
            .collect()
    }

    fn ship_combatant(&self, id: ShipId) -> Combatant {
        let s = self.ship(id).unwrap();
        let card = self.tables.unit(s.kind);
        Combatant::new(UnitRef::Ship(id), format!("{} {}", self.seat_name(s.seat), s.kind.name()), self.ship_strength(s), card.hit_points, s.damage, card.pursuit, s.stance == Stance::Evade)
    }

    fn army_combatant(&self, id: ArmyId) -> Combatant {
        let a = self.army(id).unwrap();
        let card = self.tables.unit(UnitKind::Army);
        let owner = match self.army_seat(a) {
            Some(s) => self.seat_name(s),
            None => "neutral".to_string(),
        };
        let name = if a.standing { format!("{owner} Standing Army") } else { format!("{owner} Army") };
        Combatant::new(UnitRef::Army(id), name, self.army_strength(a), card.hit_points, a.damage, card.pursuit, a.stance == Stance::Evade)
    }

    /// One melee of Ship stacks at a Body (ticket #50).
    fn ship_melee(&mut self, place: &str, body: BodyId, parties: &[(Seat, bool, Vec<ShipId>)]) {
        let units: Vec<(Option<Seat>, bool, Vec<Combatant>)> =
            parties.iter().map(|(seat, agg, ids)| (Some(*seat), *agg, ids.iter().map(|id| self.ship_combatant(*id)).collect())).collect();
        let mut line = self.run_melee(place, units);
        match self.orbital_control(body) {
            Some(s) if parties.iter().any(|(seat, agg, _)| *agg && *seat == s) => {
                line.result.push_str(&format!(" The {} hold Orbital Control.", self.seat_name(s)))
            }
            Some(s) => line.result.push_str(&format!(" The {} keep Orbital Control.", self.seat_name(s))),
            None => line.result.push_str(" Nobody holds Orbital Control."),
        }
        self.log(line.text(&|s| self.seat_name(s), "neutral"));
        self.report.battles.push(line);
    }

    /// One melee of Armies at a ground place (ticket #50).
    fn army_melee(&mut self, place_name: &str, place: Place, aggressors: &[Seat], parties: &[(Option<Seat>, bool, Vec<ArmyId>)]) {
        let units: Vec<(Option<Seat>, bool, Vec<Combatant>)> =
            parties.iter().map(|(seat, agg, ids)| (*seat, *agg, ids.iter().map(|id| self.army_combatant(*id)).collect())).collect();
        let mut line = self.run_melee(place_name, units);
        for seat in aggressors {
            if self.defenders_at(place, *seat).is_empty() && !self.armies_of_seat_at(*seat, place).is_empty() {
                line.result.push_str(&format!(" The {} are alone at the place; Occupation begins.", self.seat_name(*seat)));
            }
        }
        self.log(line.text(&|s| self.seat_name(s), "neutral"));
        self.report.battles.push(line);
    }

    fn run_melee(&mut self, place: &str, parties: Vec<(Option<Seat>, bool, Vec<Combatant>)>) -> BattleLine {
        let describe = |side: &[Combatant]| -> String {
            let mut names: Vec<String> = side.iter().map(|c| c.name.split(' ').skip(1).collect::<Vec<_>>().join(" ")).collect();
            names.sort();
            names.join(", ")
        };
        let mut parties = parties;
        let described: Vec<String> = parties.iter().map(|(_, _, c)| describe(c)).collect();
        let strengths: Vec<i64> = parties.iter().map(|(_, _, c)| c.iter().map(|x| x.strength).sum()).collect();
        let stats = {
            let mut slices: Vec<&mut [Combatant]> = parties.iter_mut().map(|(_, _, c)| c.as_mut_slice()).collect();
            let mut rng = self.rng.clone();
            let stats = combat::melee(&mut slices, &mut rng as &mut dyn Dice);
            self.rng = rng;
            stats
        };
        let listed: Vec<BattleParty> = parties
            .iter()
            .enumerate()
            .map(|(i, (seat, agg, _))| BattleParty {
                seat: *seat,
                aggressor: *agg,
                units: described[i].clone(),
                strength: strengths[i],
                hits: stats.hits_of(i),
                destroyed: stats.destroyed.get(i).cloned().unwrap_or_default(),
                escaped: stats.escaped.get(i).cloned().unwrap_or_default(),
            })
            .collect();
        for (_, _, c) in &parties {
            self.apply_combatants(c);
        }
        BattleLine { place: place.to_string(), parties: listed, result: format!("{} round(s).", stats.rounds) }
    }

    fn apply_combatants(&mut self, side: &[Combatant]) {
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
                        self.destroy_army(id);
                    } else if let Some(a) = self.army_mut(id) {
                        a.damage = c.damage;
                        a.escaped = c.escaped;
                    }
                }
            }
        }
    }

    pub fn destroy_ship(&mut self, id: ShipId, why: &str) {
        let Some(pos) = self.ships.iter().position(|s| s.id == id) else { return };
        let ship = self.ships.remove(pos);
        if let Some(a) = ship.army {
            self.destroy_army(a);
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
            &[("faction", self.seat_name(ship.seat)), ("ship", ship.kind.name().to_string()), ("why", why.to_string()), ("cargo", cargo)],
        );
        let place = match ship.at {
            ShipAt::Body(b) => Some(ReportPlace::Body(b)),
            _ => None,
        };
        self.report_line(LineKind::DecisiveBattle, place, text);
    }

    pub fn destroy_army(&mut self, id: ArmyId) {
        self.armies.retain(|a| a.id != id);
        for s in &mut self.ships {
            if s.army == Some(id) {
                s.army = None;
            }
        }
    }

    /// Spec 8.5: every Facility or Module at a place rolls a 1-in-4 chance to be destroyed.
    fn destruction_rolls(&mut self, place: Place, why: &str) {
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
        if !lost.is_empty() {
            let line = format!("{} was {}: {} destroyed.", self.place_name(place), why, lost.join(", "));
            self.log(line);
            let text = self.say(
                "units_destroyed",
                &[("place", self.place_name(place)), ("why", why.to_string()), ("lost", lost.join(", "))],
            );
            self.report_line(LineKind::DecisiveBattle, Some(place.into()), text.clone());
            self.moment(
                MomentKind::DecisiveBattle,
                &[("place", self.place_name(place)), ("result", text), ("figure", format!("{} lost", lost.len()))],
                Some(place.into()),
            );
        }
    }

    // ------------------------------------------------------------------ (c)

    fn resolve_occupation(&mut self) {
        let mut places: Vec<Place> = StateId::ALL.into_iter().map(Place::State).collect();
        places.extend(self.colonies.iter().map(|c| Place::Colony(c.id)));
        for place in places {
            let control = self.place_control(place);
            match control {
                Control::Occupied { occupier, previous, turns } => {
                    if self.armies_of_seat_at(occupier, place).is_empty() {
                        // Occupation broken.
                        let back = match previous {
                            Some(p) => Control::Controlled(p),
                            None => Control::Neutral,
                        };
                        self.set_place_control(place, back);
                        let line = format!("Occupation of {} by the {} ended.", self.place_name(place), self.seat_name(occupier));
                        self.log(line);
                        let text = self.say("occupation_ended", &[("place", self.place_name(place)), ("faction", self.seat_name(occupier))]);
                        self.report_line(LineKind::Occupation, Some(place.into()), text);
                        continue;
                    }
                    if !self.defenders_at(place, occupier).is_empty() {
                        continue; // defenders re-engaged; the count does not advance
                    }
                    let turns = turns + 1;
                    self.set_place_control(place, Control::Occupied { occupier, previous, turns });
                    // Ticket #52: every turn of Occupation adds one to the state's Unrest.
                    if let Place::State(sid) = place {
                        let n = self.tables.unrest.occupation_per_turn;
                        self.raise_unrest(sid, n, UnrestSource::Plain);
                    }
                    self.occupation_gain(place, occupier);
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
                        let attackers: Vec<ArmyId> = self
                            .armies
                            .iter()
                            .filter(|a| a.at == ArmyAt::Place(place) && self.army_seat(a) == Some(seat) && a.stance == Stance::Attack && !a.escaped)
                            .map(|a| a.id)
                            .collect();
                        if attackers.is_empty() || !self.defenders_at(place, seat).is_empty() {
                            continue;
                        }
                        let previous = control.controller();
                        self.set_place_control(place, Control::Occupied { occupier: seat, previous, turns: 1 });
                        // Ticket #52: an Occupation begins at +3 Unrest, damped by nothing.
                        if let Place::State(sid) = place {
                            let n = self.tables.unrest.occupation_start;
                            self.raise_unrest(sid, n, UnrestSource::Plain);
                        }
                        let line = format!("The {} occupy {}.", self.seat_name(seat), self.place_name(place));
                        self.log(line);
                        let text = self.say("occupation_begun", &[("faction", self.seat_name(seat)), ("place", self.place_name(place))]);
                        self.report_line(LineKind::Occupation, Some(place.into()), text);
                        self.occupation_gain(place, seat);
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

    fn occupation_gain(&mut self, place: Place, seat: Seat) {
        let gain = self.pacification_gain(place);
        let s = self.seat_mut(seat);
        *s.influence.entry(place).or_insert(0) += gain;
        s.influenced_this_turn.push(place);
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
        // that Occupation transfers rolls for destruction.
        if why != "Influence" {
            self.destruction_rolls(place, "taken");
        }
        // Armies at the place that fought for the old owner stand for the new one only if they are the place's own.
        // Foreign Armies keep their own home and seat; nothing to do.
    }

    // ------------------------------------------------------------------ (d)

    fn resolve_influence(&mut self) {
        // Version 0.03 (ticket #33): every Faction keeps a standing on every place; spending on a place
        // you control raises your own standing there.
        let spent = std::mem::take(&mut self.pending.influence);
        for (seat, target, amount) in spent {
            let own = self.place_control(target).controller() == Some(seat);
            let s = self.seat_mut(seat);
            *s.influence.entry(target).or_insert(0) += amount;
            s.influenced_this_turn.push(target);
            self.log(format!("{} spent {} Influence {} {}.", self.seat_name(seat), amount, if own { "holding" } else { "on" }, self.place_name(target)));
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
        // Decay on every standing that received nothing this turn: 1 on a place you control, 2 elsewhere.
        let decay = self.tables.influence.decay;
        let decay_own = self.tables.influence.decay_controlled;
        for seat in Seat::ALL {
            let owned: Vec<Place> = self.seat(seat).influence.keys().filter(|t| self.place_control(**t).controller() == Some(seat)).copied().collect();
            let s = self.seat_mut(seat);
            let touched = std::mem::take(&mut s.influenced_this_turn);
            for (t, v) in s.influence.iter_mut() {
                if !touched.contains(t) {
                    let d = if owned.contains(t) { decay_own } else { decay };
                    *v = (*v - d).max(0);
                }
            }
            s.influence.retain(|_, v| *v > 0);
        }
        // Thresholds: a neutral place needs the threshold; a controlled place needs a standing at least
        // the controller's plus the challenge margin (version 0.04, ticket #41) and at least the threshold.
        // Ticket #53: the threshold is the CHALLENGER's own, since Blame raises it seat by seat; the
        // challenge margin is the same for everyone.
        let margin = self.tables.influence.challenge_margin;
        let mut targets: Vec<Place> = StateId::ALL.into_iter().map(Place::State).collect();
        targets.extend(self.colonies.iter().map(|c| Place::Colony(c.id)));
        for target in targets {
            let controller = self.place_control(target).controller();
            let held = controller.map(|c| self.seat(c).influence.get(&target).copied().unwrap_or(0) + margin);
            let qualifying: Vec<Seat> = Seat::ALL
                .into_iter()
                .filter(|s| {
                    let have = self.seat(*s).influence.get(&target).copied().unwrap_or(0);
                    let needed = match held {
                        Some(over) => self.influence_threshold_for(*s, target).max(over),
                        None => self.influence_threshold_for(*s, target),
                    };
                    controller != Some(*s) && have > 0 && have >= needed
                })
                .collect();
            // Ticket #50: among challengers who all qualify the same turn, the higher Standing takes
            // the place; an exact tie goes to nobody and everything stays as it is until next turn.
            let winner = match qualifying.len() {
                0 => continue,
                1 => qualifying[0],
                _ => {
                    let standing = |s: &Seat| self.seat(*s).influence.get(&target).copied().unwrap_or(0);
                    let top = qualifying.iter().map(standing).max().unwrap_or(0);
                    let leaders: Vec<Seat> = qualifying.iter().copied().filter(|s| standing(s) == top).collect();
                    if leaders.len() > 1 {
                        let names: Vec<String> = leaders.iter().map(|s| self.seat_name(*s)).collect();
                        let line = format!("{} is claimed by {} at the same Standing; it stays as it is.", self.place_name(target), names.join(" and "));
                        self.log(line);
                        let text = self.say("claim_tied", &[("place", self.place_name(target)), ("factions", names.join(" and "))]);
                        self.report_line(LineKind::Note, Some(target.into()), text);
                        continue;
                    }
                    leaders[0]
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

    fn resolve_builds(&mut self) {
        let turn = self.turn;
        // Launch Pad Fire (ticket #32): every Ship due this turn at that state completes next turn instead,
        // unless Clean Propellant is held.
        let mut pad_fire: Option<StateId> = self.last_event.as_ref().and_then(|ev| match (ev.card, ev.target) {
            (Card::Event(EventId::LaunchPadFire), EventTarget::State(s)) => Some(s),
            _ => None,
        });
        if self.has_tech(TechId::CleanPropellant) {
            pad_fire = None;
        }
        let mut completed: Vec<(Place, Build)> = Vec::new();
        for sid in StateId::ALL {
            let st = self.state_mut(sid);
            let mut i = 0;
            while i < st.queue.len() {
                if st.queue[i].due_turn <= turn {
                    let b = st.queue.remove(i);
                    completed.push((Place::State(sid), b));
                } else {
                    i += 1;
                }
            }
        }
        let cids: Vec<ColonyId> = self.colonies.iter().map(|c| c.id).collect();
        for cid in cids {
            let col = self.colony_mut(cid).unwrap();
            let mut i = 0;
            while i < col.queue.len() {
                if col.queue[i].due_turn <= turn {
                    let b = col.queue.remove(i);
                    completed.push((Place::Colony(cid), b));
                } else {
                    i += 1;
                }
            }
        }
        for (place, mut b) in completed {
            let is_ship = matches!(b.item, BuildItem::Unit(k) if k != UnitKind::Army);
            if is_ship && pad_fire.map(|s| place == Place::State(s)).unwrap_or(false) {
                b.due_turn = turn + 1;
                self.requeue(place, b.clone());
                let line = format!("Launch Pad Fire: the {} {} at {} completes next turn instead.", self.seat_name(b.seat), b.item.name(), self.place_name(place));
                self.log(line);
                let text = self.say(
                    "launch_pad_fire",
                    &[("faction", self.seat_name(b.seat)), ("item", b.item.name().to_string()), ("place", self.place_name(place))],
                );
                self.report_line(LineKind::Note, Some(place.into()), text);
                continue;
            }
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
            if let Some(col) = self.colony(cid) {
                let room = self.habitat_room(col);
                if let Some(col) = self.colony_mut(cid) {
                    col.colonists = col.colonists.min(room);
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

    fn requeue(&mut self, place: Place, b: Build) {
        match place {
            Place::State(s) => self.state_mut(s).queue.push(b),
            Place::Colony(c) => {
                if let Some(col) = self.colony_mut(c) {
                    col.queue.push(b)
                }
            }
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
            // Ticket #51: a stage of the Archive raises the one that stands rather than adding another.
            (Place::Colony(c), BuildItem::Module(ModuleKind::Archive)) => {
                let stages = self.tables.archive.stages;
                let stage = if let Some(col) = self.colony_mut(c) {
                    match col.modules.iter_mut().find(|m| m.kind == ModuleKind::Archive) {
                        Some(m) => {
                            m.stage += 1;
                            m.stage
                        }
                        None => {
                            let mut m = Module::new(ModuleKind::Archive);
                            m.stage = 1;
                            col.modules.push(m);
                            1
                        }
                    }
                } else {
                    0
                };
                let line = if stage >= stages {
                    format!("The {} completed the Archive at {}: every stage stands.", self.seat_name(b.seat), self.place_name(place))
                } else {
                    format!("The {} completed stage {} of {} of the Archive at {}.", self.seat_name(b.seat), stage, stages, self.place_name(place))
                };
                self.log(line);
                let text = if stage >= stages {
                    self.say("archive_complete", &[("faction", self.seat_name(b.seat)), ("place", self.place_name(place))])
                } else {
                    self.say(
                        "archive_stage_complete",
                        &[
                            ("faction", self.seat_name(b.seat)),
                            ("stage", stage.to_string()),
                            ("stages", stages.to_string()),
                            ("place", self.place_name(place)),
                        ],
                    )
                };
                self.report_line(LineKind::Archive, Some(place.into()), text);
                if stage >= stages {
                    self.moment(
                        MomentKind::ArchiveComplete,
                        &[("faction", self.seat_name(b.seat)), ("place", self.place_name(place)), ("stages", stages.to_string())],
                        Some(place.into()),
                    );
                }
                return;
            }
            (Place::Colony(c), BuildItem::Module(k)) => {
                if let Some(col) = self.colony_mut(c) {
                    col.modules.push(Module::new(k));
                }
            }
            (_, BuildItem::Unit(UnitKind::Army)) => {
                let id = ArmyId(self.fresh_id());
                let home = match place {
                    Place::State(s) => ArmyHome::State(s),
                    Place::Colony(c) => ArmyHome::Colony(c),
                };
                self.armies.push(Army { id, home, at: ArmyAt::Place(place), damage: 0, standing: false, stance: Stance::Hold, escaped: false, move_to: None });
            }
            (_, BuildItem::Unit(kind)) => {
                let body = match place {
                    Place::State(_) => BodyId::Earth,
                    Place::Colony(c) => self.colony(c).map(|c| c.body).unwrap_or(BodyId::Earth),
                };
                let id = ShipId(self.fresh_id());
                self.ships.push(Ship {
                    id,
                    kind,
                    seat: b.seat,
                    damage: 0,
                    at: ShipAt::Body(body),
                    colonists: 0,
                    army: None,
                    stance: Stance::Hold,
                    escaped: false,
                    arrived_this_turn: false,
                    built_turn: turn,
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
            }
            self.log(format!("{} repaired {} point(s).", self.seat_name(seat), points));
        }
    }

    // ------------------------------------------------------------------ (g)

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
            self.colonies.push(Colony { id, body: *body, slot: *slot, control: Control::Controlled(*seat), modules: Vec::new(), colonists: 0, queue: Vec::new(), grid_failed: false, founded_turn: self.turn, in_orbit: true });
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
                                // Ticket #51, Steerage: the population a lift takes is a Faction figure.
                                let cost = self.lift_population(seat, colonists);
                                let p = &mut self.state_mut(st).population;
                                *p = (*p - cost).max(0.0);
                            }
                            LoadSource::Colony(c) => {
                                let Some(col) = self.colony_mut(c) else { continue };
                                if col.colonists < colonists {
                                    continue;
                                }
                                col.colonists -= colonists;
                            }
                        }
                        if let Some(s) = self.ship_mut(ship) {
                            s.colonists += colonists;
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
                    if !self.may_land(seat, body) {
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
                                modules: vec![Module::new(ModuleKind::Habitat)],
                                colonists: 0,
                                queue: Vec::new(),
                                grid_failed: false,
                                founded_turn: self.turn,
                                in_orbit: false,
                            });
                            let room = self.habitat_room(self.colony(id).unwrap());
                            let moved = n.min(room);
                            self.colony_mut(id).unwrap().colonists = moved;
                            if let Some(s) = self.ship_mut(ship) {
                                s.colonists -= moved;
                            }
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
                            let note = if off_earth <= 1 {
                                self.phrase("first_colony", &[])
                            } else {
                                self.phrase("more_colonies", &[("count", off_earth.to_string())])
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
                                self.land_army(aid, ship, Place::Colony(cid));
                                let line = format!("{} landed an Army at {}.", self.seat_name(seat), self.place_name(Place::Colony(cid)));
                                self.log(line);
                                let text = self.say("army_landed", &[("faction", self.seat_name(seat)), ("colony", self.place_name(Place::Colony(cid)))]);
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

    /// Unrest settles last, once every rise of the turn is in: the refugees the turn's flows
    /// brought, then the falls (Relief, a Constabulary, and the natural fall in a turn nothing
    /// raised it), then the throw-off, then the Report lines for crossing a threshold.
    pub fn resolve_unrest(&mut self) {
        let u = self.tables.unrest.clone();
        // The refugees the turn's flows brought, charged once so the per-turn cap counts them all.
        for sid in StateId::ALL {
            let arrived = self.state(sid).refugees_in;
            if arrived <= 0.0 {
                continue;
            }
            let want = (arrived / u.refugees_per).floor().min(u.refugees_max);
            let rose = self.raise_unrest(sid, want, UnrestSource::Refugees);
            if rose > 0.0 {
                let line = format!("{:.1} people arrived in {}; Unrest rose by {} to {}.", arrived, self.tables.state(sid).name, Game::unrest_figure(rose), self.unrest_text(sid));
                self.log(line);
                let text = self.say(
                    "refugees_arrived",
                    &[
                        ("n", format!("{arrived:.1}")),
                        ("state", self.tables.state(sid).name.clone()),
                        ("rose", Game::unrest_figure(rose).to_string()),
                        ("unrest", self.unrest_text(sid)),
                    ],
                );
                self.report_line(LineKind::Refugees, Some(ReportPlace::State(sid)), text);
            }
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

    fn land_army(&mut self, aid: ArmyId, ship: ShipId, place: Place) {
        if let Some(a) = self.army_mut(aid) {
            a.at = ArmyAt::Place(place);
            a.stance = Stance::Hold;
        }
        if let Some(s) = self.ship_mut(ship) {
            s.army = None;
        }
    }
}

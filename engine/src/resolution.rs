//! The Resolution phase, (a) through (h), in the order spec 6 gives (with 8, 9, 10).

use crate::combat::{self, Combatant, Dice};
use crate::ids::*;
use crate::orders::*;
use crate::state::*;

impl Game {
    pub fn resolution_phase(&mut self) {
        // Flags that last "until the next Resolution" clear now.
        for s in &mut self.states {
            for f in &mut s.facilities {
                if f.offline_until_resolution {
                    f.offline_until_resolution = false;
                    f.online = true;
                }
            }
        }
        for c in &mut self.colonies {
            if c.grid_failed {
                c.grid_failed = false;
                for m in &mut c.modules {
                    m.online = true;
                }
            }
            for m in c.modules.iter_mut().filter(|m| m.offline_until_resolution) {
                m.offline_until_resolution = false;
                m.online = true;
            }
        }
        self.blackout_stances();
        self.resolve_transits(); // (a)
        self.resolve_battles(); // (b)
        self.resolve_occupation(); // (c)
        self.resolve_influence(); // (d)
        self.resolve_builds(); // (e)
        self.resolve_repairs(); // (f)
        self.resolve_cargo(); // (g)
        self.apply_event_now(); // (h)
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
            self.report.lines.push("Solar Storm: no transit advanced this turn.".into());
        }
        for (seat, body, id) in &arrivals {
            let line = format!("{} {} arrived at {}.", self.seat_name(*seat), self.ship(*id).map(|s| s.kind.name()).unwrap_or("Ship"), self.tables.body(*body).name);
            self.report.lines.push(line.clone());
            self.log(line);
        }
        // Intercept battles: the intercepting stack attacks the arrivals.
        for body in BodyId::ALL {
            for seat in Seat::ALL {
                let arriving: Vec<ShipId> = arrivals.iter().filter(|(s, b, _)| *s == seat && *b == body).map(|(_, _, id)| *id).collect();
                if arriving.is_empty() {
                    continue;
                }
                let enemy = seat.other();
                let interceptors: Vec<ShipId> = self
                    .ships
                    .iter()
                    .filter(|s| s.seat == enemy && s.at == ShipAt::Body(body) && s.stance == Stance::Intercept && !s.escaped)
                    .map(|s| s.id)
                    .collect();
                if interceptors.is_empty() {
                    continue;
                }
                let name = format!("{} orbit (interception)", self.tables.body(body).name);
                self.ship_battle(&name, enemy, seat, &interceptors, &arriving);
            }
        }
    }

    // ------------------------------------------------------------------ (b)

    fn resolve_battles(&mut self) {
        // Ship battles: an Attack stack engages every enemy stack at its Body.
        for body in BodyId::ALL {
            for seat in Seat::ALL {
                let attackers: Vec<ShipId> = self
                    .ships
                    .iter()
                    .filter(|s| s.seat == seat && s.at == ShipAt::Body(body) && s.stance == Stance::Attack && !s.escaped)
                    .map(|s| s.id)
                    .collect();
                if attackers.is_empty() {
                    continue;
                }
                let enemy = seat.other();
                let defenders: Vec<ShipId> = self
                    .ships
                    .iter()
                    .filter(|s| s.seat == enemy && s.at == ShipAt::Body(body) && !s.escaped)
                    .map(|s| s.id)
                    .collect();
                if defenders.is_empty() {
                    continue;
                }
                let name = format!("{} orbit", self.tables.body(body).name);
                self.ship_battle(&name, seat, enemy, &attackers, &defenders);
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
            self.log(line.clone());
            self.report.lines.push(line);
        }
        // Ground battles at every place where an Attack stack faces defenders.
        let mut places: Vec<Place> = StateId::ALL.into_iter().map(Place::State).collect();
        places.extend(self.colonies.iter().map(|c| Place::Colony(c.id)));
        for place in places {
            for seat in Seat::ALL {
                let attackers: Vec<ArmyId> = self
                    .armies
                    .iter()
                    .filter(|a| a.at == ArmyAt::Place(place) && self.army_seat(a) == Some(seat) && a.stance == Stance::Attack && !a.escaped && !self.army_stands_down(a))
                    .map(|a| a.id)
                    .collect();
                if attackers.is_empty() {
                    continue;
                }
                if self.place_director(place) == Some(seat) {
                    continue;
                }
                let defenders: Vec<ArmyId> = self.defenders_at(place, seat);
                if defenders.is_empty() {
                    continue;
                }
                let defender_seat = self.armies.iter().find(|a| a.id == defenders[0]).and_then(|a| self.army_seat(a));
                let name = self.place_name(place);
                self.army_battle(&name, place, seat, defender_seat, &attackers, &defenders);
                self.destruction_rolls(place, "attacked");
            }
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

    fn ship_battle(&mut self, place: &str, attacker: Seat, defender: Seat, attackers: &[ShipId], defenders: &[ShipId]) {
        let mut a: Vec<Combatant> = attackers.iter().map(|id| self.ship_combatant(*id)).collect();
        let mut d: Vec<Combatant> = defenders.iter().map(|id| self.ship_combatant(*id)).collect();
        let line = self.run_battle(place, attacker, Some(defender), &mut a, &mut d);
        self.apply_combatants(&a);
        self.apply_combatants(&d);
        let mut line = line;
        if let Some(body) = BodyId::ALL.into_iter().find(|b| place.starts_with(&self.tables.body(*b).name)) {
            match self.orbital_control(body) {
                Some(s) if s == attacker => line.result.push_str(&format!(" The {} hold Orbital Control.", self.seat_name(s))),
                Some(s) => line.result.push_str(&format!(" The {} keep Orbital Control.", self.seat_name(s))),
                None => {}
            }
        }
        self.log(line.text(&|s| self.seat_name(s), "neutral"));
        self.report.battles.push(line);
    }

    fn army_battle(&mut self, place_name: &str, place: Place, attacker: Seat, defender: Option<Seat>, attackers: &[ArmyId], defenders: &[ArmyId]) {
        let mut a: Vec<Combatant> = attackers.iter().map(|id| self.army_combatant(*id)).collect();
        let mut d: Vec<Combatant> = defenders.iter().map(|id| self.army_combatant(*id)).collect();
        let mut line = self.run_battle(place_name, attacker, defender, &mut a, &mut d);
        self.apply_combatants(&a);
        self.apply_combatants(&d);
        if self.defenders_at(place, attacker).is_empty() && !self.armies_of_seat_at(attacker, place).is_empty() {
            line.result.push_str(" The defenders are gone; Occupation begins.");
        }
        self.log(line.text(&|s| self.seat_name(s), "neutral"));
        self.report.battles.push(line);
    }

    fn run_battle(&mut self, place: &str, attacker: Seat, defender: Option<Seat>, a: &mut [Combatant], d: &mut [Combatant]) -> BattleLine {
        let describe = |side: &[Combatant]| -> String {
            let mut names: Vec<String> = side.iter().map(|c| c.name.split(' ').skip(1).collect::<Vec<_>>().join(" ")).collect();
            names.sort();
            names.join(", ")
        };
        let a_units = describe(a);
        let d_units = describe(d);
        let a_str: i64 = a.iter().map(|c| c.strength).sum();
        let d_str: i64 = d.iter().map(|c| c.strength).sum();
        let mut rng = self.rng.clone();
        let stats = combat::fight(a, d, &mut rng as &mut dyn Dice);
        self.rng = rng;
        BattleLine {
            place: place.to_string(),
            attacker,
            defender,
            attacker_units: a_units,
            defender_units: d_units,
            attacker_strength: a_str,
            defender_strength: d_str,
            hits_by_attacker: stats.hits_by_attacker,
            hits_by_defender: stats.hits_by_defender,
            destroyed: stats.destroyed,
            escaped: stats.escaped,
            result: format!("{} round(s).", stats.rounds),
        }
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
        self.log(line.clone());
        self.report.lines.push(line);
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
            self.log(line.clone());
            self.report.lines.push(line);
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
                        self.log(line.clone());
                        self.report.lines.push(line);
                        continue;
                    }
                    if !self.defenders_at(place, occupier).is_empty() {
                        continue; // defenders re-engaged; the count does not advance
                    }
                    let turns = turns + 1;
                    self.set_place_control(place, Control::Occupied { occupier, previous, turns });
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
                        let line = format!("The {} occupy {}.", self.seat_name(seat), self.place_name(place));
                        self.log(line.clone());
                        self.report.lines.push(line);
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
        let gain = (self.influence_threshold(place) + 2) / 3;
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
            Place::State(s) => self.state_mut(s).control = control,
            Place::Colony(c) => {
                if let Some(col) = self.colony_mut(c) {
                    col.control = control;
                }
            }
        }
    }

    /// Control passes to `seat` (spec 8.3, 8.5): rivals' Influence wiped, a destruction roll, Armies follow.
    pub fn transfer_control(&mut self, place: Place, seat: Seat, why: &str) {
        self.set_place_control(place, Control::Controlled(seat));
        for s in Seat::ALL {
            self.seat_mut(s).influence.remove(&place);
        }
        let line = format!("{} now belongs to the {} ({}).", self.place_name(place), self.seat_name(seat), why);
        self.log(line.clone());
        self.report.lines.push(line);
        self.destruction_rolls(place, "taken");
        // Armies at the place that fought for the old owner stand for the new one only if they are the place's own.
        // Foreign Armies keep their own home and seat; nothing to do.
    }

    // ------------------------------------------------------------------ (d)

    fn resolve_influence(&mut self) {
        let spent = std::mem::take(&mut self.pending.influence);
        for (seat, target, amount) in spent {
            let own = self.place_director(target) == Some(seat) || self.place_control(target).controller() == Some(seat);
            if own {
                let rival = seat.other();
                let r = self.seat_mut(rival);
                if let Some(v) = r.influence.get_mut(&target) {
                    *v = (*v - amount).max(0);
                }
                self.log(format!("{} spent {} Influence defending {}.", self.seat_name(seat), amount, self.place_name(target)));
            } else {
                let s = self.seat_mut(seat);
                *s.influence.entry(target).or_insert(0) += amount;
                s.influenced_this_turn.push(target);
                self.log(format!("{} spent {} Influence on {}.", self.seat_name(seat), amount, self.place_name(target)));
            }
        }
        // Decay on every accumulation that received nothing this turn.
        let decay = self.tables.influence.decay;
        for seat in Seat::ALL {
            let s = self.seat_mut(seat);
            let touched = std::mem::take(&mut s.influenced_this_turn);
            for (t, v) in s.influence.iter_mut() {
                if !touched.contains(t) {
                    *v = (*v - decay).max(0);
                }
            }
            s.influence.retain(|_, v| *v > 0);
        }
        // Thresholds.
        let mut targets: Vec<Place> = StateId::ALL.into_iter().map(Place::State).collect();
        targets.extend(self.colonies.iter().map(|c| Place::Colony(c.id)));
        for target in targets {
            let threshold = self.influence_threshold(target);
            let controller = self.place_control(target).controller();
            // Meeting a threshold takes real Influence: a Colony with no Colonists has a threshold of
            // zero, and zero accumulated Influence must not claim it.
            let qualifying: Vec<Seat> = Seat::ALL
                .into_iter()
                .filter(|s| {
                    let have = self.seat(*s).influence.get(&target).copied().unwrap_or(0);
                    controller != Some(*s) && have > 0 && have >= threshold
                })
                .collect();
            let winner = match qualifying.len() {
                0 => continue,
                1 => qualifying[0],
                _ => self.tiebreak(target),
            };
            self.transfer_control(target, winner, "Influence");
        }
    }

    /// The same tiebreak for a Colony Slot: the Ship stacks in orbit decide.
    pub fn tiebreak_at_body(&mut self, body: BodyId) -> Seat {
        let (a, b) = (self.ship_stack_strength(Seat(0), body), self.ship_stack_strength(Seat(1), body));
        if a != b {
            return if a > b { Seat(0) } else { Seat(1) };
        }
        loop {
            let (ra, rb) = (self.rng.d6(), self.rng.d6());
            if ra != rb {
                return if ra > rb { Seat(0) } else { Seat(1) };
            }
        }
    }

    /// Spec 6: greater total unit strength present, else a d6 each, re-rolled on a tie.
    pub fn tiebreak(&mut self, place: Place) -> Seat {
        let strength = |g: &Game, seat: Seat| -> i64 {
            let ground = g.army_stack_strength(seat, place);
            let orbit = match place {
                Place::Colony(c) => g.colony(c).map(|c| g.ship_stack_strength(seat, c.body)).unwrap_or(0),
                Place::State(_) => g.ship_stack_strength(seat, BodyId::Earth),
            };
            ground + orbit
        };
        let (a, b) = (strength(self, Seat(0)), strength(self, Seat(1)));
        if a != b {
            return if a > b { Seat(0) } else { Seat(1) };
        }
        loop {
            let (ra, rb) = (self.rng.d6(), self.rng.d6());
            if ra != rb {
                return if ra > rb { Seat(0) } else { Seat(1) };
            }
        }
    }

    // ------------------------------------------------------------------ (e)

    fn resolve_builds(&mut self) {
        let turn = self.turn;
        // Equipment Failure and Launch Failure pick one build of the target seat.
        let mut delay_one: Option<Seat> = None;
        let mut launch_fail: Option<Seat> = None;
        if let Some(ev) = &self.last_event {
            match (ev.card, ev.target) {
                (Card::Event(EventId::EquipmentFailure), EventTarget::Seat(s)) => delay_one = Some(s),
                (Card::Event(EventId::LaunchFailure), EventTarget::Seat(s)) => launch_fail = Some(s),
                _ => {}
            }
        }
        let clean = self.has_tech(TechId::CleanPropellant);
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
            if delay_one == Some(b.seat) {
                delay_one = None;
                b.due_turn = turn + 1;
                self.requeue(place, b.clone());
                let line = format!("Equipment Failure: the {} {} at {} completes next turn instead.", self.seat_name(b.seat), b.item.name(), self.place_name(place));
                self.log(line.clone());
                self.report.lines.push(line);
                continue;
            }
            if launch_fail == Some(b.seat) && matches!(b.item, BuildItem::Unit(k) if k != UnitKind::Army) {
                launch_fail = None;
                if clean {
                    b.due_turn = turn + 1;
                    self.requeue(place, b.clone());
                    let line = format!("Launch Failure: the {} {} is delayed one turn (Clean Propellant).", self.seat_name(b.seat), b.item.name());
                    self.log(line.clone());
                    self.report.lines.push(line);
                } else {
                    let line = format!("Launch Failure: the {} {} is destroyed on the pad, no refund.", self.seat_name(b.seat), b.item.name());
                    self.log(line.clone());
                    self.report.lines.push(line);
                }
                continue;
            }
            self.complete_build(place, b);
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
                if self.state(s).facilities.len() as u32 >= self.build_slots(s) {
                    let line = format!("{} at {} had no slot left and was lost.", name, self.place_name(place));
                    self.log(line.clone());
                    self.report.lines.push(line);
                    return;
                }
                self.state_mut(s).facilities.push(Facility { kind: k, online: true, offline_until_resolution: false });
            }
            (Place::State(s), BuildItem::IndustryLevel) => {
                self.state_mut(s).industry_level += 1;
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
        self.log(line.clone());
        self.report.lines.push(line);
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
        // Founding orders into the same slot from both seats are decided by the tiebreak.
        let mut founding: Vec<(Seat, ShipId, u32, BodyId, u32)> = Vec::new();
        for (seat, order) in &cargo {
            if let Order::Unload { ship, colonists, into: UnloadTarget::Slot(b, slot), .. } = order {
                founding.push((*seat, *ship, *colonists, *b, *slot));
            }
        }
        let mut blocked: Vec<ShipId> = Vec::new();
        for i in 0..founding.len() {
            for j in (i + 1)..founding.len() {
                let (sa, ship_a, _, ba, slot_a) = founding[i];
                let (sb, ship_b, _, bb, slot_b) = founding[j];
                if sa != sb && ba == bb && slot_a == slot_b {
                    let winner = self.tiebreak_at_body(ba);
                    if winner == sa {
                        blocked.push(ship_b);
                    } else {
                        blocked.push(ship_a);
                    }
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
                                let p = &mut self.state_mut(st).population;
                                *p = (*p - 0.1 * colonists as f64).max(0.0);
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
                    let line = format!("{} loaded {} at {}.", self.seat_name(seat), if colonists > 0 { format!("{colonists} Colonists") } else { "an Army".into() }, self.tables.body(body).name);
                    self.log(line.clone());
                    self.report.lines.push(line);
                }
                Order::Unload { ship, colonists, army, into } => {
                    if blocked.contains(&ship) {
                        let line = format!("{}: the rival took that Colony Slot first.", self.seat_name(seat));
                        self.log(line.clone());
                        self.report.lines.push(line);
                        continue;
                    }
                    let Some(s) = self.ship(ship) else { continue };
                    let ShipAt::Body(body) = s.at else { continue };
                    if !self.may_land(seat, body) {
                        let line = format!("{} could not land at {}: the orbit is contested.", self.seat_name(seat), self.tables.body(body).name);
                        self.log(line.clone());
                        self.report.lines.push(line);
                        continue;
                    }
                    let aboard_army = s.army;
                    match into {
                        UnloadTarget::Slot(b, slot) => {
                            if b != body || !self.free_slots_on(b).contains(&slot) || colonists == 0 {
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
                            self.log(line.clone());
                            self.report.lines.push(line);
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
                                self.log(line.clone());
                                self.report.lines.push(line);
                            }
                            if let Some(aid) = aboard_army.filter(|_| army) {
                                self.land_army(aid, ship, Place::Colony(cid));
                                let line = format!("{} landed an Army at {}.", self.seat_name(seat), self.place_name(Place::Colony(cid)));
                                self.log(line.clone());
                                self.report.lines.push(line);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
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

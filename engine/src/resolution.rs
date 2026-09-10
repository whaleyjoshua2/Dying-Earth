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
            self.log(line.clone());
            self.report.lines.push(line);
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
            self.log(line.clone());
            self.report.lines.push(line);
        }
        self.set_place_control(place, Control::Controlled(seat));
        // Standings persist through a transfer (ticket #33): the old controller keeps its own and
        // can contest the place back.
        let line = format!("{} now belongs to the {} ({}).", self.place_name(place), self.seat_name(seat), why);
        self.log(line.clone());
        self.report.lines.push(line);
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
                let r: i64 = st.facilities.iter().filter(|f| f.online).map(|f| self.tables.facility(f.kind).standing_per_turn).sum();
                if r > 0 {
                    rises.push((c, Place::State(st.id), r));
                }
            }
        }
        for col in &self.colonies {
            if let Some(c) = col.control.controller() {
                let r: i64 = col.modules.iter().filter(|m| m.online).map(|m| self.tables.module(m.kind).standing_per_turn).sum();
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
        let margin = self.tables.influence.challenge_margin;
        let mut targets: Vec<Place> = StateId::ALL.into_iter().map(Place::State).collect();
        targets.extend(self.colonies.iter().map(|c| Place::Colony(c.id)));
        for target in targets {
            let threshold = self.influence_threshold(target);
            let controller = self.place_control(target).controller();
            let needed = match controller {
                Some(c) => threshold.max(self.seat(c).influence.get(&target).copied().unwrap_or(0) + margin),
                None => threshold,
            };
            let qualifying: Vec<Seat> = Seat::ALL
                .into_iter()
                .filter(|s| {
                    let have = self.seat(*s).influence.get(&target).copied().unwrap_or(0);
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
                        self.log(line.clone());
                        self.report.lines.push(line);
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
                self.log(line.clone());
                self.report.lines.push(line);
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
                self.log(line.clone());
                self.report.lines.push(line);
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
            self.log(line.clone());
            self.report.lines.push(line);
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
                    let line = format!("{} loaded {} at {}.", self.seat_name(seat), if colonists > 0 { format!("{colonists} Colonists") } else { "an Army".into() }, self.tables.body(body).name);
                    self.log(line.clone());
                    self.report.lines.push(line);
                }
                Order::Unload { ship, colonists, army, into } => {
                    if blocked.contains(&ship) {
                        let line = format!("{}: another Faction took that Colony Slot first.", self.seat_name(seat));
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

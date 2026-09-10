//! The Event Deck (spec 13, amended by ticket #25): thirty cards, no Calm Cards, a draw chance that
//! rises with the Temperature, and eighteen Events.

use crate::data::Tables;
use crate::ids::*;
use crate::state::*;
use rand::seq::SliceRandom;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

/// The deck as the table deals it, shuffled with the game seed; never reshuffled.
pub fn new_deck(tables: &Tables, rng: &mut ChaCha8Rng) -> Deck {
    let mut cards: Vec<Card> = Vec::new();
    for e in &tables.events.event {
        for _ in 0..e.copies {
            cards.push(Card::Event(e.id));
        }
    }
    cards.shuffle(rng);
    Deck { cards, drawn: Vec::new() }
}

impl Game {
    /// The chance a card is drawn this turn: the base at +1.2 C, plus a step per full 0.2 C above it.
    pub fn draw_chance(&self) -> f64 {
        let e = &self.tables.events;
        let base = self.tables.climate.base_temperature;
        let steps = ((self.climate.temperature - base) / e.draw_chance_step_degrees + 1e-9).floor().max(0.0);
        (e.draw_chance_base + e.draw_chance_per_step * steps).clamp(0.0, 1.0)
    }

    /// One roll against the draw chance.
    pub fn rolls_a_card(&mut self) -> bool {
        let p = self.draw_chance();
        self.rng.random::<f64>() < p
    }

    /// Phase 5: maybe draw one card and choose its target. The effect applies in Resolution.
    pub fn event_phase(&mut self) {
        let chance = self.draw_chance();
        if !self.rolls_a_card() {
            self.last_event = None;
            let text = format!("No Event this turn (a card comes {:.0}% of turns at this Temperature).", chance * 100.0);
            self.log(format!("Event: {text}"));
            self.report.event = Some(text);
            return;
        }
        let Some(card) = self.deck.cards.pop() else {
            self.last_event = None;
            self.log("Event: the deck is empty.");
            self.report.event = Some("The Event Deck is empty.".to_string());
            return;
        };
        self.deck.drawn.push(card);
        let Card::Event(id) = card;
        let drawn = self.target_event(id);
        self.log(format!("Event: {}", drawn.text));
        self.report.event = Some(drawn.text.clone());
        self.last_event = Some(drawn);
    }

    pub fn climate_scale(&self) -> f64 {
        1.0 + (self.climate.temperature - self.tables.climate.base_temperature).max(0.0) / 2.0
    }

    fn target_event(&mut self, id: EventId) -> DrawnEvent {
        let t = self.tables.clone();
        let card = t.event(id);
        let scale = if card.kind == EventKind::Climate { self.climate_scale() } else { 1.0 };
        let turn = self.turn;
        let (target, text) = match id {
            EventId::SolarStorm | EventId::RadiationSurge | EventId::CommsBlackout | EventId::MeteorShower => {
                (EventTarget::Everyone, format!("{}: {}.", card.name, card.effect))
            }
            EventId::SolarMaximum => {
                let m = if self.has_tech(TechId::EfficientGrids) { t.events.solar_maximum_multiplier_with_tech } else { t.events.solar_maximum_multiplier };
                (EventTarget::Everyone, format!("{}: every Power Plant and Generator makes x{} at the next Income.", card.name, m))
            }
            EventId::PermafrostThaw => {
                let e = if self.has_tech(TechId::GreenConsensus) { t.events.permafrost_emissions / 2.0 } else { t.events.permafrost_emissions } * scale;
                (EventTarget::Everyone, format!("{}: +{:.1} Emissions next turn (x{:.2} at this Temperature).", card.name, e, scale))
            }
            EventId::LaunchPadFire => {
                let states: Vec<StateId> = StateId::ALL
                    .into_iter()
                    .filter(|s| self.state(*s).facilities.iter().any(|f| f.kind == FacilityKind::LaunchSite))
                    .collect();
                match self.pick_uniform(&states) {
                    Some(s) => {
                        let ships = self.state(s).queue.iter().filter(|b| b.due_turn <= turn && matches!(b.item, BuildItem::Unit(k) if k != UnitKind::Army)).count();
                        let delay = if self.has_tech(TechId::CleanPropellant) { "no Ship is delayed (Clean Propellant)".to_string() } else if ships > 0 { format!("{ships} Ship(s) due there complete next turn instead") } else { "no Ship was due there".to_string() };
                        (EventTarget::State(s), format!("{} in {}: its Launch Site is offline until the next Resolution; {}.", card.name, t.state(s).name, delay))
                    }
                    None => (EventTarget::None, format!("{}: no Launch Site stands anywhere, so nothing happens.", card.name)),
                }
            }
            EventId::LabourDispute => match self.pick_state_by_population() {
                Some(s) => {
                    let what = if self.has_tech(TechId::PublicScience) { "one of its Facilities makes nothing at the next Income (Public Science)" } else { "its Facilities make nothing at the next Income" };
                    (EventTarget::State(s), format!("{} in {}: {what}.", card.name, t.state(s).name))
                }
                None => (EventTarget::None, format!("{}: nobody lives anywhere, so nothing happens.", card.name)),
            },
            EventId::GridFailure => {
                let cols: Vec<ColonyId> = self.colonies.iter().filter(|c| !c.modules.is_empty()).map(|c| c.id).collect();
                match self.pick_uniform(&cols) {
                    Some(c) => {
                        let what = if self.has_tech(TechId::ClosedLoopColonies) { "no effect (Closed-Loop Colonies)" } else { "its Modules are offline until the next Resolution" };
                        (EventTarget::Colony(c), format!("{} at {}: {what}.", card.name, self.place_name(Place::Colony(c))))
                    }
                    None => (EventTarget::None, format!("{}: no Colony exists, so nothing happens.", card.name)),
                }
            }
            EventId::ReactorLeak => {
                let cols: Vec<ColonyId> = self.colonies.iter().filter(|c| c.modules.iter().any(|m| m.kind == ModuleKind::Generator)).map(|c| c.id).collect();
                match self.pick_uniform(&cols) {
                    Some(c) => {
                        let what = if self.has_tech(TechId::ClosedLoopColonies) { "no effect (Closed-Loop Colonies)".to_string() } else { format!("its Generators are offline until the next Resolution and its holder loses {} Energy", t.events.reactor_leak_energy) };
                        (EventTarget::Colony(c), format!("{} at {}: {what}.", card.name, self.place_name(Place::Colony(c))))
                    }
                    None => (EventTarget::None, format!("{}: no Colony has a Generator, so nothing happens.", card.name)),
                }
            }
            EventId::DustStorm => {
                if self.colonies.iter().any(|c| c.body == BodyId::Mars && !c.modules.is_empty()) {
                    let what = if self.has_tech(TechId::ClosedLoopColonies) { "no effect (Closed-Loop Colonies)" } else { "every Module on Mars is offline until the next Resolution" };
                    (EventTarget::Body(BodyId::Mars), format!("{}: {what}.", card.name))
                } else {
                    (EventTarget::None, format!("{}: nobody lives on Mars, so nothing happens.", card.name))
                }
            }
            EventId::RichSeam | EventId::IceDeposit => {
                let kind = if id == EventId::RichSeam { ModuleKind::Mine } else { ModuleKind::Refinery };
                let bodies: Vec<BodyId> = BodyId::ALL
                    .into_iter()
                    .filter(|b| *b != BodyId::Earth)
                    .filter(|b| self.colonies.iter().any(|c| c.body == *b && c.modules.iter().any(|m| m.kind == kind)))
                    .collect();
                match self.pick_uniform(&bodies) {
                    Some(b) => {
                        let tech = if id == EventId::RichSeam { TechId::DeepMining } else { TechId::AutomatedRefining };
                        let m = if self.has_tech(tech) { t.events.discovery_multiplier_with_tech } else { t.events.discovery_multiplier };
                        (EventTarget::Body(b), format!("{} on {}: its {}s produce x{} for {} turns.", card.name, t.body(b).name, kind.name(), m, t.events.discovery_turns))
                    }
                    None => (EventTarget::None, format!("{}: no {} exists off Earth, so nothing happens.", card.name, kind.name())),
                }
            }
            EventId::Breakthrough => match self.research.current {
                Some(tech) => {
                    let r = if self.has_tech(TechId::PublicScience) { t.events.breakthrough_research_public_science } else { t.events.breakthrough_research };
                    (EventTarget::Tech, format!("{}: +{} Research toward {}.", card.name, r, t.tech(tech).name))
                }
                None => (EventTarget::None, format!("{}: no Tech is under research, so nothing happens.", card.name)),
            },
            EventId::Heatwave | EventId::Wildfire | EventId::Unrest => {
                let state = self.pick_state_by_population();
                match state {
                    Some(s) => {
                        let name = t.state(s).name.clone();
                        let text = match id {
                            EventId::Heatwave => {
                                let loss = if self.has_tech(TechId::GreenConsensus) { t.events.heatwave_loss_green_consensus } else { t.events.heatwave_loss };
                                format!("{} in {}: population -{:.1}% now (x{:.2} at this Temperature).", card.name, name, loss * scale * 100.0, scale)
                            }
                            EventId::Wildfire => {
                                let em = if self.has_tech(TechId::CleanManufacturing) { 0.0 } else { t.events.wildfire_emissions * scale };
                                format!("{} in {}: one Facility offline until next Resolution; +{:.1} Emissions next turn.", card.name, name, em)
                            }
                            _ => format!("{} in {}: its Standing Army takes {} damage and every Faction's Influence there drops by {}.", card.name, name, t.events.unrest_army_damage, t.events.unrest_influence_loss),
                        };
                        (EventTarget::State(s), text)
                    }
                    None => (EventTarget::None, format!("{}: nobody lives anywhere, so nothing happens.", card.name)),
                }
            }
            EventId::StormSurge => {
                let states: Vec<StateId> = StateId::ALL
                    .into_iter()
                    .filter(|s| t.state(*s).coastal_exposure > 0 && self.state(*s).thresholds_fired.iter().any(|f| !f))
                    .collect();
                match self.pick_uniform(&states) {
                    Some(s) => (EventTarget::State(s), format!("{} in {}: its next sea-level threshold applies now.", card.name, t.state(s).name)),
                    None => (EventTarget::None, format!("{}: no exposed coast has a threshold ahead, so nothing happens.", card.name)),
                }
            }
        };
        DrawnEvent { card: Card::Event(id), target, scale, text }
    }

    fn pick_uniform<T: Copy>(&mut self, list: &[T]) -> Option<T> {
        if list.is_empty() {
            None
        } else {
            Some(list[self.rng.random_range(0..list.len())])
        }
    }

    fn pick_state_by_population(&mut self) -> Option<StateId> {
        let total: f64 = self.states.iter().map(|s| s.population).sum();
        if total <= 0.0 {
            return None;
        }
        let mut r = self.rng.random_range(0.0..total);
        for s in &self.states {
            if r < s.population {
                return Some(s.id);
            }
            r -= s.population;
        }
        self.states.iter().rev().find(|s| s.population > 0.0).map(|s| s.id)
    }

    /// Whether this turn's card is the named Event.
    pub fn event_is(&self, id: EventId) -> bool {
        matches!(&self.last_event, Some(DrawnEvent { card: Card::Event(e), target, .. }) if *e == id && *target != EventTarget::None)
    }

    /// Resolution (h): the card effects that act now, in card text order.
    pub fn apply_event_now(&mut self) {
        let Some(ev) = self.last_event.clone() else { return };
        let t = self.tables.clone();
        let Card::Event(id) = ev.card;
        match (id, ev.target) {
            (EventId::RadiationSurge, EventTarget::Everyone) | (EventId::MeteorShower, EventTarget::Everyone) => {
                if !self.has_tech(TechId::HardenedHulls) {
                    let in_transit = id == EventId::RadiationSurge;
                    let dmg = if in_transit { 1 } else { t.events.meteor_damage };
                    let mut hit = 0;
                    for s in self.ships.iter_mut().filter(|s| matches!(s.at, ShipAt::Transit { .. }) == in_transit) {
                        s.damage += dmg;
                        hit += 1;
                    }
                    let destroyed: Vec<ShipId> = self.ships.iter().filter(|s| s.damage >= t.unit(s.kind).hit_points).map(|s| s.id).collect();
                    for sid in destroyed {
                        self.destroy_ship(sid, t.event(id).name.as_str());
                    }
                    if hit > 0 {
                        self.report.lines.push(format!("{} damaged {} Ship(s).", t.event(id).name, hit));
                    }
                }
            }
            (EventId::SolarMaximum, EventTarget::Everyone) => {
                self.solar_maximum_next = true;
            }
            (EventId::PermafrostThaw, EventTarget::Everyone) => {
                let e = if self.has_tech(TechId::GreenConsensus) { t.events.permafrost_emissions / 2.0 } else { t.events.permafrost_emissions };
                self.climate.card_emissions_next += e * ev.scale;
            }
            (EventId::GridFailure, EventTarget::Colony(c)) => {
                let immune = self.has_tech(TechId::ClosedLoopColonies);
                if let Some(col) = self.colony_mut(c).filter(|_| !immune) {
                    col.grid_failed = true;
                    for m in &mut col.modules {
                        m.online = false;
                    }
                }
            }
            (EventId::ReactorLeak, EventTarget::Colony(c)) => {
                if !self.has_tech(TechId::ClosedLoopColonies) {
                    let holder = self.colony(c).and_then(|c| c.control.director());
                    if let Some(col) = self.colony_mut(c) {
                        for m in col.modules.iter_mut().filter(|m| m.kind == ModuleKind::Generator) {
                            m.online = false;
                            m.offline_until_resolution = true;
                        }
                    }
                    if let Some(h) = holder {
                        let s = &mut self.seat_mut(h).stockpile;
                        s.energy = (s.energy - t.events.reactor_leak_energy).max(0);
                    }
                }
            }
            (EventId::DustStorm, EventTarget::Body(b)) => {
                if !self.has_tech(TechId::ClosedLoopColonies) {
                    for col in self.colonies.iter_mut().filter(|c| c.body == b) {
                        col.grid_failed = true;
                        for m in &mut col.modules {
                            m.online = false;
                        }
                    }
                }
            }
            (EventId::RichSeam, EventTarget::Body(b)) | (EventId::IceDeposit, EventTarget::Body(b)) => {
                let kind = if id == EventId::RichSeam { ModuleKind::Mine } else { ModuleKind::Refinery };
                let tech = if id == EventId::RichSeam { TechId::DeepMining } else { TechId::AutomatedRefining };
                let m = if self.has_tech(tech) { t.events.discovery_multiplier_with_tech } else { t.events.discovery_multiplier };
                self.discoveries.push(Discovery { body: b, kind, multiplier: m, turns_left: t.events.discovery_turns });
            }
            (EventId::Breakthrough, EventTarget::Tech) => {
                let r = if self.has_tech(TechId::PublicScience) { t.events.breakthrough_research_public_science } else { t.events.breakthrough_research };
                self.add_research_unattributed(r);
            }
            (EventId::Heatwave, EventTarget::State(s)) => {
                let loss = if self.has_tech(TechId::GreenConsensus) { t.events.heatwave_loss_green_consensus } else { t.events.heatwave_loss };
                let st = self.state_mut(s);
                st.population = (st.population * (1.0 - loss * ev.scale)).max(0.0);
            }
            (EventId::LaunchPadFire, EventTarget::State(s)) => {
                let st = self.state_mut(s);
                for f in st.facilities.iter_mut().filter(|f| f.kind == FacilityKind::LaunchSite) {
                    f.offline_until_resolution = true;
                    f.online = false;
                }
            }
            (EventId::LabourDispute, EventTarget::State(s)) => {
                let n = self.state(s).facilities.len();
                if n == 0 {
                    return;
                }
                if self.has_tech(TechId::PublicScience) {
                    let i = self.rng.random_range(0..n);
                    let st = self.state_mut(s);
                    st.facilities[i].offline_until_resolution = true;
                    st.facilities[i].online = false;
                } else {
                    for f in &mut self.state_mut(s).facilities {
                        f.offline_until_resolution = true;
                        f.online = false;
                    }
                }
            }
            (EventId::Wildfire, EventTarget::State(s)) => {
                let n = self.state(s).facilities.len();
                if n > 0 {
                    let i = self.rng.random_range(0..n);
                    let st = self.state_mut(s);
                    st.facilities[i].offline_until_resolution = true;
                    st.facilities[i].online = false;
                }
                if !self.has_tech(TechId::CleanManufacturing) {
                    self.state_mut(s).wildfire_emissions_next += t.events.wildfire_emissions * ev.scale;
                }
            }
            (EventId::Unrest, EventTarget::State(s)) => {
                let hp = t.unit(UnitKind::Army).hit_points;
                let mut dead = None;
                if let Some(a) = self.armies.iter_mut().find(|a| a.standing && a.home == ArmyHome::State(s)) {
                    a.damage += t.events.unrest_army_damage;
                    if a.damage >= hp {
                        dead = Some(a.id);
                    }
                }
                if let Some(id) = dead {
                    self.destroy_army(id);
                }
                for seat in Seat::ALL {
                    if let Some(v) = self.seat_mut(seat).influence.get_mut(&Place::State(s)) {
                        *v = (*v - t.events.unrest_influence_loss).max(0);
                    }
                }
            }
            (EventId::StormSurge, EventTarget::State(s)) => {
                if let Some(i) = self.state(s).thresholds_fired.iter().position(|f| !f) {
                    self.apply_sea_threshold(s, i);
                }
            }
            _ => {}
        }
    }
}

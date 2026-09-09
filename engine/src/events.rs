//! The Event Deck (spec 13).

use crate::data::Tables;
use crate::ids::*;
use crate::state::*;
use rand::seq::SliceRandom;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

/// Twenty cards, shuffled with the game seed; never reshuffled.
pub fn new_deck(tables: &Tables, rng: &mut ChaCha8Rng) -> Deck {
    let mut cards: Vec<Card> = EventId::ALL.iter().map(|e| Card::Event(*e)).collect();
    for _ in 0..tables.events.calm_cards {
        cards.push(Card::Calm);
    }
    cards.shuffle(rng);
    Deck { cards, drawn: Vec::new() }
}

impl Game {
    /// Replace one Calm Card still in the deck with a Climate card chosen uniformly. False if none is left.
    pub fn swap_one_calm(&mut self) -> bool {
        let calm: Vec<usize> = self.deck.cards.iter().enumerate().filter(|(_, c)| **c == Card::Calm).map(|(i, _)| i).collect();
        if calm.is_empty() {
            return false;
        }
        let which = calm[self.rng.random_range(0..calm.len())];
        let climate = EventId::CLIMATE[self.rng.random_range(0..EventId::CLIMATE.len())];
        self.deck.cards[which] = Card::Event(climate);
        true
    }

    /// Phase 5: draw one card and choose its target. The effect applies in Resolution.
    pub fn event_phase(&mut self) {
        let Some(card) = self.deck.cards.pop() else {
            self.last_event = None;
            self.log("Event: the deck is empty.");
            return;
        };
        self.deck.drawn.push(card);
        let drawn = match card {
            Card::Calm => DrawnEvent { card, target: EventTarget::None, scale: 1.0, text: "Calm. Nothing happens.".to_string() },
            Card::Event(id) => self.target_event(id),
        };
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
            EventId::SolarStorm | EventId::RadiationSurge | EventId::CommsBlackout => {
                (EventTarget::Everyone, format!("{}: {}.", card.name, card.effect))
            }
            EventId::EquipmentFailure => {
                let seats: Vec<Seat> = Seat::ALL.into_iter().filter(|s| self.builds_due(*s, turn) > 0).collect();
                match self.pick_uniform(&seats) {
                    Some(s) => (EventTarget::Seat(s), format!("{} strikes the {}: one build due this turn completes next turn instead.", card.name, self.seat_name(s))),
                    None => (EventTarget::None, format!("{}: no build was due, so nothing happens.", card.name)),
                }
            }
            EventId::LaunchFailure => {
                let seats: Vec<Seat> = Seat::ALL.into_iter().filter(|s| self.ships_due(*s, turn) > 0).collect();
                match self.pick_uniform(&seats) {
                    Some(s) => {
                        let what = if self.has_tech(TechId::CleanPropellant) { "delayed one turn (Clean Propellant)" } else { "destroyed, no refund" };
                        (EventTarget::Seat(s), format!("{} strikes the {}: one Ship due this turn is {what}.", card.name, self.seat_name(s)))
                    }
                    None => (EventTarget::None, format!("{}: no Ship was due, so nothing happens.", card.name)),
                }
            }
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
            EventId::RichSeam | EventId::IceDeposit => {
                let kind = if id == EventId::RichSeam { ModuleKind::Mine } else { ModuleKind::Refinery };
                let bodies: Vec<BodyId> = [BodyId::Moon, BodyId::Mars]
                    .into_iter()
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
            EventId::Heatwave | EventId::Wildfire => {
                let state = self.pick_state_by_population();
                match state {
                    Some(s) => {
                        let name = t.state(s).name.clone();
                        let text = if id == EventId::Heatwave {
                            let loss = if self.has_tech(TechId::GreenConsensus) { t.events.heatwave_loss_green_consensus } else { t.events.heatwave_loss };
                            format!("{} in {}: population -{:.1}% now (x{:.2} at this Temperature).", card.name, name, loss * scale * 100.0, scale)
                        } else {
                            let em = if self.has_tech(TechId::CleanManufacturing) { 0.0 } else { t.events.wildfire_emissions * scale };
                            format!("{} in {}: one Facility offline until next Resolution; +{:.1} Emissions next turn.", card.name, name, em)
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

    pub fn builds_due(&self, seat: Seat, turn: u32) -> usize {
        self.states.iter().flat_map(|s| s.queue.iter()).chain(self.colonies.iter().flat_map(|c| c.queue.iter()))
            .filter(|b| b.seat == seat && b.due_turn <= turn)
            .count()
    }

    pub fn ships_due(&self, seat: Seat, turn: u32) -> usize {
        self.states.iter().flat_map(|s| s.queue.iter()).chain(self.colonies.iter().flat_map(|c| c.queue.iter()))
            .filter(|b| b.seat == seat && b.due_turn <= turn && matches!(b.item, BuildItem::Unit(k) if k != UnitKind::Army))
            .count()
    }

    /// Whether this turn's card is the named Event.
    pub fn event_is(&self, id: EventId) -> bool {
        matches!(&self.last_event, Some(DrawnEvent { card: Card::Event(e), target, .. }) if *e == id && *target != EventTarget::None)
    }

    /// Resolution (h): the card effects that act now, in card text order.
    pub fn apply_event_now(&mut self) {
        let Some(ev) = self.last_event.clone() else { return };
        let t = self.tables.clone();
        let Card::Event(id) = ev.card else { return };
        match (id, ev.target) {
            (EventId::RadiationSurge, EventTarget::Everyone) => {
                if !self.has_tech(TechId::HardenedHulls) {
                    let mut hit = Vec::new();
                    for s in self.ships.iter_mut().filter(|s| matches!(s.at, ShipAt::Transit { .. })) {
                        s.damage += 1;
                        hit.push(s.id);
                    }
                    let destroyed: Vec<ShipId> = self.ships.iter().filter(|s| s.damage >= t.unit(s.kind).hit_points).map(|s| s.id).collect();
                    for id in destroyed {
                        self.destroy_ship(id, "Radiation Surge");
                    }
                    if !hit.is_empty() {
                        self.report.lines.push(format!("Radiation Surge damaged {} Ship(s) in transit.", hit.len()));
                    }
                }
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
            (EventId::StormSurge, EventTarget::State(s)) => {
                if let Some(i) = self.state(s).thresholds_fired.iter().position(|f| !f) {
                    self.apply_sea_threshold(s, i);
                }
            }
            _ => {}
        }
    }
}

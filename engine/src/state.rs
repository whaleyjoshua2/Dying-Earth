//! The whole state of one game, in glossary words.

use crate::data::Tables;
use crate::ids::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Stockpile {
    pub materials: i64,
    pub fuel: i64,
    pub energy: i64,
    /// Version 0.03 (ticket #35).
    pub ducats: i64,
}

/// A Nation State is neutral, controlled, or occupied (spec 8.1, 8.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Control {
    Neutral,
    Controlled(Seat),
    Occupied { occupier: Seat, previous: Option<Seat>, turns: u32 },
}

impl Control {
    /// The seat that gives orders here and collects production, if any.
    pub fn director(self) -> Option<Seat> {
        match self {
            Control::Neutral => None,
            Control::Controlled(s) => Some(s),
            Control::Occupied { occupier, .. } => Some(occupier),
        }
    }
    /// The seat that formally controls the place (an occupier does not, yet).
    pub fn controller(self) -> Option<Seat> {
        match self {
            Control::Controlled(s) => Some(s),
            Control::Occupied { previous, .. } => previous,
            Control::Neutral => None,
        }
    }
    pub fn is_occupied(self) -> bool {
        matches!(self, Control::Occupied { .. })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Facility {
    pub kind: FacilityKind,
    /// Shut down this turn by the Energy shortfall rule, or knocked offline by a card.
    pub online: bool,
    /// Set by a Wildfire: offline until the next Resolution.
    pub offline_until_resolution: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub kind: ModuleKind,
    pub online: bool,
    /// Knocked offline by a card until the next Resolution (Reactor Leak).
    #[allow(dead_code)]
    pub offline_until_resolution: bool,
}

impl Module {
    pub fn new(kind: ModuleKind) -> Module {
        Module { kind, online: true, offline_until_resolution: false }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildItem {
    Facility(FacilityKind),
    IndustryLevel,
    Module(ModuleKind),
    Unit(UnitKind),
}

impl BuildItem {
    pub fn name(self) -> String {
        match self {
            BuildItem::Facility(k) => k.name().to_string(),
            BuildItem::IndustryLevel => "Industry Level".to_string(),
            BuildItem::Module(k) => k.name().to_string(),
            BuildItem::Unit(k) => k.name().to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Build {
    pub item: BuildItem,
    pub seat: Seat,
    pub due_turn: u32,
}

#[derive(Debug, Clone)]
pub struct NationState {
    pub id: StateId,
    pub population: f64,
    pub industry_level: u32,
    pub control: Control,
    pub facilities: Vec<Facility>,
    pub queue: Vec<Build>,
    pub lost_slots: u32,
    /// Sea-level thresholds already applied to this state, by index into the table.
    pub thresholds_fired: Vec<bool>,
    /// Extra Emissions charged next Climate phase by a Wildfire.
    pub wildfire_emissions_next: f64,
}

#[derive(Debug, Clone)]
pub struct Colony {
    pub id: ColonyId,
    pub body: BodyId,
    pub slot: u32,
    pub control: Control,
    pub modules: Vec<Module>,
    pub colonists: u32,
    pub queue: Vec<Build>,
    /// Grid Failure: Modules offline until the next Resolution.
    pub grid_failed: bool,
    pub founded_turn: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShipAt {
    Body(BodyId),
    Transit { from: BodyId, to: BodyId, turns_left: u32 },
}

#[derive(Debug, Clone)]
pub struct Ship {
    pub id: ShipId,
    pub kind: UnitKind,
    pub seat: Seat,
    pub damage: u32,
    pub at: ShipAt,
    pub colonists: u32,
    pub army: Option<ArmyId>,
    pub stance: Stance,
    pub escaped: bool,
    pub arrived_this_turn: bool,
    /// Turn this Ship was built, so an Army it carries can be told apart from one boarded later.
    pub built_turn: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmyHome {
    State(StateId),
    Colony(ColonyId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmyAt {
    Place(Place),
    Aboard(ShipId),
}

#[derive(Debug, Clone)]
pub struct Army {
    pub id: ArmyId,
    pub home: ArmyHome,
    pub at: ArmyAt,
    pub damage: u32,
    pub standing: bool,
    pub stance: Stance,
    pub escaped: bool,
    /// Ordered to move or attack this turn; consumed in Resolution.
    pub move_to: Option<StateId>,
}

#[derive(Debug, Clone, Default)]
pub struct EmissionsBreakdown {
    pub state_industry: f64,
    pub factories: f64,
    pub power_plants: f64,
    pub refineries: f64,
    pub launches: f64,
    pub population: f64,
    /// Emissions added by Event cards (Wildfire, Permafrost Thaw); never counted against Stabilization.
    pub cards: f64,
    pub sink: f64,
    pub restoration: f64,
}

impl EmissionsBreakdown {
    /// Emissions that count against a Stabilization run: buildings, launches and population, not cards.
    pub fn counted(&self) -> f64 {
        self.state_industry + self.factories + self.power_plants + self.refineries + self.launches + self.population
    }
    pub fn total(&self) -> f64 {
        self.counted() + self.cards
    }
    pub fn total_sink(&self) -> f64 {
        self.sink + self.restoration
    }
    pub fn net(&self) -> f64 {
        self.total() - self.total_sink()
    }
}

#[derive(Debug, Clone)]
pub struct Climate {
    pub co2: f64,
    pub temperature: f64,
    pub last: EmissionsBreakdown,
    /// Launches from Earth since the last Climate phase, per seat, charged next time.
    pub launches_pending: [u32; 2],
    /// Emissions a card (Permafrost Thaw) adds at the next Climate phase, worldwide.
    pub card_emissions_next: f64,
    /// Restoration bought in the last Orders phase, in ppm, for the next Climate phase only.
    pub restoration_next: f64,
}

#[derive(Debug, Clone)]
pub struct Research {
    pub current: Option<TechId>,
    pub progress: i64,
    pub contributions: [i64; 2],
    pub done: Vec<TechId>,
    /// Research produced while no Tech was chosen; flows into the next one.
    pub unallocated: i64,
    /// Who must pick the next Tech, when a human has to.
    pub awaiting_pick: Option<Seat>,
    pub last_lead: Option<Seat>,
}

impl Research {
    pub fn has(&self, t: TechId) -> bool {
        self.done.contains(&t)
    }
}

/// A card in the deck. Since ticket #25 every card is an Event; there are no Calm Cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Card {
    Event(EventId),
}

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
    pub drawn: Vec<Card>,
}

impl Deck {
    pub fn climate_cards_left(&self) -> usize {
        self.cards
            .iter()
            .filter(|c| matches!(c, Card::Event(e) if EventId::CLIMATE.contains(e)))
            .count()
    }
    pub fn count(&self, id: EventId) -> usize {
        self.cards.iter().filter(|c| **c == Card::Event(id)).count()
    }
}

/// A drawn card with its target and size, for the popup and the log.
#[derive(Debug, Clone)]
pub struct DrawnEvent {
    pub card: Card,
    pub target: EventTarget,
    pub scale: f64,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventTarget {
    None,
    Everyone,
    Seat(Seat),
    Colony(ColonyId),
    Body(BodyId),
    State(StateId),
    Tech,
}

/// A temporary effect from a Discovery card.
#[derive(Debug, Clone, Copy)]
pub struct Discovery {
    pub body: BodyId,
    pub kind: ModuleKind,
    pub multiplier: f64,
    pub turns_left: u32,
}

#[derive(Debug, Clone)]
pub struct SeatState {
    pub kind: FactionKind,
    pub ai: bool,
    pub stockpile: Stockpile,
    pub extraction_total: i64,
    pub stabilization_run: u32,
    pub influence: BTreeMap<Target, i64>,
    /// Targets that received Influence this turn (spent or gained by Occupation), so they do not decay.
    pub influenced_this_turn: Vec<Target>,
    pub allotment: i64,
    pub research_last_turn: i64,
    pub income_last_turn: Stockpile,
    /// Last Income by source (ticket #31): "Factory in Asia", the resource, the amount; upkeep as negatives.
    pub income_sources: Vec<(String, Resource, i64)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Win { seat: Seat, margin_note: String },
    Draw { note: String },
    Collapse,
}

/// One line of the Battle Report (spec 10.3).
#[derive(Debug, Clone)]
pub struct BattleLine {
    pub place: String,
    pub attacker: Seat,
    pub defender: Option<Seat>,
    pub attacker_units: String,
    pub defender_units: String,
    pub attacker_strength: i64,
    pub defender_strength: i64,
    pub hits_by_attacker: u32,
    pub hits_by_defender: u32,
    pub destroyed: Vec<String>,
    pub escaped: Vec<String>,
    pub result: String,
}

impl BattleLine {
    pub fn text(&self, names: &dyn Fn(Seat) -> String, neutral: &str) -> String {
        let def = self.defender.map(names).unwrap_or_else(|| neutral.to_string());
        format!(
            "{}: {} ({}, strength {}) attacked {} ({}, strength {}). Hits {} to {}. Destroyed: {}. Escaped: {}. {}",
            self.place,
            names(self.attacker),
            self.attacker_units,
            self.attacker_strength,
            def,
            self.defender_units,
            self.defender_strength,
            self.hits_by_attacker,
            self.hits_by_defender,
            if self.destroyed.is_empty() { "none".to_string() } else { self.destroyed.join(", ") },
            if self.escaped.is_empty() { "none".to_string() } else { self.escaped.join(", ") },
            self.result
        )
    }
}

/// Everything the Report popup shows at the start of a turn (spec 17.5).
#[derive(Debug, Clone, Default)]
pub struct Report {
    pub turn: u32,
    pub battles: Vec<BattleLine>,
    pub event: Option<String>,
    pub lines: Vec<String>,
    pub ai_lines: Vec<String>,
}

/// Everything about one game. Fields are public because the interface reads all of them.
#[derive(Debug, Clone)]
pub struct Game {
    pub tables: std::sync::Arc<Tables>,
    pub seed: u64,
    pub rng: ChaCha8Rng,
    pub turn: u32,
    pub seats: [SeatState; 2],
    pub states: Vec<NationState>,
    pub colonies: Vec<Colony>,
    pub ships: Vec<Ship>,
    pub armies: Vec<Army>,
    pub climate: Climate,
    pub research: Research,
    pub deck: Deck,
    pub discoveries: Vec<Discovery>,
    /// Solar Maximum: every Power Plant and Generator makes more at the next Income.
    pub solar_maximum_next: bool,
    pub last_event: Option<DrawnEvent>,
    pub report: Report,
    pub outcome: Option<Outcome>,
    pub next_id: u32,
    /// Orders committed at End Turn that act later in Resolution.
    pub pending: crate::orders::Pending,
    /// Lines for the simulate log and the dev diary; the interface ignores them.
    pub log: Vec<String>,
}

pub struct NewGame {
    pub seed: u64,
    pub seats: [(FactionKind, bool); 2],
    pub player_start: StateId,
}

impl Game {
    pub fn new(tables: std::sync::Arc<Tables>, setup: NewGame) -> Game {
        let mut rng = ChaCha8Rng::seed_from_u64(setup.seed);
        let start = Stockpile { materials: tables.start.materials, fuel: tables.start.fuel, energy: tables.start.energy, ducats: tables.start.ducats };
        let seat = |kind: FactionKind, ai: bool| SeatState {
            kind,
            ai,
            stockpile: start,
            extraction_total: 0,
            stabilization_run: 0,
            influence: BTreeMap::new(),
            influenced_this_turn: Vec::new(),
            allotment: 0,
            research_last_turn: 0,
            income_last_turn: Stockpile::default(),
            income_sources: Vec::new(),
        };
        let states: Vec<NationState> = tables
            .states
            .iter()
            .map(|c| NationState {
                id: c.id,
                population: c.population,
                industry_level: c.industry_level,
                control: Control::Neutral,
                facilities: c.start_facilities.iter().map(|k| Facility { kind: *k, online: true, offline_until_resolution: false }).collect(),
                queue: Vec::new(),
                lost_slots: 0,
                thresholds_fired: vec![false; tables.climate.sea_level_thresholds.len()],
                wildfire_emissions_next: 0.0,
            })
            .collect();
        let deck = crate::events::new_deck(&tables, &mut rng);
        let mut game = Game {
            seed: setup.seed,
            rng,
            turn: 1,
            seats: [seat(setup.seats[0].0, setup.seats[0].1), seat(setup.seats[1].0, setup.seats[1].1)],
            states,
            colonies: Vec::new(),
            ships: Vec::new(),
            armies: Vec::new(),
            climate: Climate {
                co2: tables.climate.starting_co2,
                temperature: tables.climate.base_temperature,
                last: EmissionsBreakdown::default(),
                launches_pending: [0, 0],
                card_emissions_next: 0.0,
                restoration_next: 0.0,
            },
            research: Research {
                current: None,
                progress: 0,
                contributions: [0, 0],
                done: Vec::new(),
                unallocated: 0,
                awaiting_pick: Some(Seat(0)),
                last_lead: None,
            },
            deck,
            discoveries: Vec::new(),
            solar_maximum_next: false,
            last_event: None,
            report: Report::default(),
            outcome: None,
            next_id: 1,
            pending: crate::orders::Pending::default(),
            log: Vec::new(),
            tables,
        };
        // Standing Armies for every state.
        for id in StateId::ALL {
            game.spawn_standing_army(id);
        }
        // Starting positions (spec 14.3): the player's pick, then the AI's continent (section 20).
        game.take_control(setup.player_start, Seat(0));
        let ai_start = game.ai_start_state(setup.player_start);
        game.take_control(ai_start, Seat(1));
        for (sid, seat) in [(setup.player_start, Seat(0)), (ai_start, Seat(1))] {
            let st = game.state_mut(sid);
            st.control = Control::Controlled(seat);
            st.facilities.push(Facility { kind: FacilityKind::LaunchSite, online: true, offline_until_resolution: false });
        }
        game.log.push(format!(
            "New game, seed {}. Seat 0 {} ({}) starts in {}; seat 1 {} ({}) starts in {}.",
            setup.seed,
            game.seats[0].kind.name(),
            if game.seats[0].ai { "AI" } else { "player" },
            game.tables.state(setup.player_start).name,
            game.seats[1].kind.name(),
            if game.seats[1].ai { "AI" } else { "player" },
            game.tables.state(ai_start).name
        ));
        game
    }

    /// The uncontrolled state with the highest Industry Level, ties by population (section 20).
    pub fn ai_start_state(&self, taken: StateId) -> StateId {
        let mut best: Option<StateId> = None;
        for c in &self.tables.states {
            if c.id == taken || c.id == StateId::Antarctica {
                continue;
            }
            let better = match best {
                None => true,
                Some(b) => {
                    let bc = self.tables.state(b);
                    c.industry_level > bc.industry_level
                        || (c.industry_level == bc.industry_level && c.population > bc.population)
                }
            };
            if better {
                best = Some(c.id);
            }
        }
        best.unwrap_or(StateId::Asia)
    }

    pub fn fresh_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn state(&self, id: StateId) -> &NationState {
        &self.states[id.index()]
    }
    pub fn state_mut(&mut self, id: StateId) -> &mut NationState {
        &mut self.states[id.index()]
    }
    pub fn colony(&self, id: ColonyId) -> Option<&Colony> {
        self.colonies.iter().find(|c| c.id == id)
    }
    pub fn colony_mut(&mut self, id: ColonyId) -> Option<&mut Colony> {
        self.colonies.iter_mut().find(|c| c.id == id)
    }
    pub fn ship(&self, id: ShipId) -> Option<&Ship> {
        self.ships.iter().find(|s| s.id == id)
    }
    pub fn ship_mut(&mut self, id: ShipId) -> Option<&mut Ship> {
        self.ships.iter_mut().find(|s| s.id == id)
    }
    pub fn army(&self, id: ArmyId) -> Option<&Army> {
        self.armies.iter().find(|a| a.id == id)
    }
    pub fn army_mut(&mut self, id: ArmyId) -> Option<&mut Army> {
        self.armies.iter_mut().find(|a| a.id == id)
    }
    pub fn seat(&self, s: Seat) -> &SeatState {
        &self.seats[s.index()]
    }
    pub fn seat_mut(&mut self, s: Seat) -> &mut SeatState {
        &mut self.seats[s.index()]
    }
    pub fn kind(&self, s: Seat) -> FactionKind {
        self.seats[s.index()].kind
    }
    pub fn seat_name(&self, s: Seat) -> String {
        let k = self.kind(s);
        if self.seats[0].kind == self.seats[1].kind {
            format!("{} (seat {})", k.name(), s.0 + 1)
        } else {
            k.name().to_string()
        }
    }
    pub fn place_name(&self, p: Place) -> String {
        match p {
            Place::State(s) => self.tables.state(s).name.clone(),
            Place::Colony(c) => match self.colony(c) {
                Some(col) => format!("Colony {} on {}", col.slot + 1, self.tables.body(col.body).name),
                None => format!("{c}"),
            },
        }
    }
    pub fn has_tech(&self, t: TechId) -> bool {
        self.research.has(t)
    }

    /// Which seat an Army fights for, if any: it follows its home (spec 8.4).
    pub fn army_seat(&self, a: &Army) -> Option<Seat> {
        match a.home {
            ArmyHome::State(s) => match self.state(s).control {
                Control::Controlled(seat) => Some(seat),
                _ => None,
            },
            ArmyHome::Colony(c) => self.colony(c).and_then(|c| c.control.controller()),
        }
    }

    /// True while a Standing Army stands down under Occupation.
    pub fn army_stands_down(&self, a: &Army) -> bool {
        match a.home {
            ArmyHome::State(s) => a.standing && self.state(s).control.is_occupied(),
            ArmyHome::Colony(c) => self.colony(c).map(|c| c.control.is_occupied()).unwrap_or(true),
        }
    }

    pub fn standing_army_cap(&self, s: StateId) -> u32 {
        self.state(s).industry_level + 1
    }

    pub fn army_strength(&self, a: &Army) -> i64 {
        if a.standing {
            let cap = match a.home {
                ArmyHome::State(s) => self.standing_army_cap(s) as i64,
                ArmyHome::Colony(_) => self.tables.unit(UnitKind::Army).strength,
            };
            (cap - a.damage as i64).max(0)
        } else {
            self.tables.unit(UnitKind::Army).strength
        }
    }

    pub fn ship_strength(&self, s: &Ship) -> i64 {
        let base = self.tables.unit(s.kind).strength;
        if base == 0 {
            return 0;
        }
        base + if self.has_tech(TechId::HardenedHulls) { self.tables.tech(TechId::HardenedHulls).value as i64 } else { 0 }
    }

    pub fn spawn_standing_army(&mut self, s: StateId) {
        let id = ArmyId(self.fresh_id());
        self.armies.push(Army {
            id,
            home: ArmyHome::State(s),
            at: ArmyAt::Place(Place::State(s)),
            damage: 0,
            standing: true,
            stance: Stance::Hold,
            escaped: false,
            move_to: None,
        });
    }

    pub fn take_control(&mut self, s: StateId, seat: Seat) {
        self.state_mut(s).control = Control::Controlled(seat);
    }

    pub fn controlled_states(&self, seat: Seat) -> Vec<StateId> {
        self.states.iter().filter(|s| s.control == Control::Controlled(seat)).map(|s| s.id).collect()
    }

    /// States this seat directs: controlled, plus occupied by it.
    pub fn directed_states(&self, seat: Seat) -> Vec<StateId> {
        self.states.iter().filter(|s| s.control.director() == Some(seat)).map(|s| s.id).collect()
    }

    pub fn owned_colonies(&self, seat: Seat) -> Vec<ColonyId> {
        self.colonies.iter().filter(|c| c.control.controller() == Some(seat)).map(|c| c.id).collect()
    }

    pub fn directed_colonies(&self, seat: Seat) -> Vec<ColonyId> {
        self.colonies.iter().filter(|c| c.control.director() == Some(seat)).map(|c| c.id).collect()
    }

    /// Build slots a Nation State has now (spec 4.2, 11.4).
    pub fn build_slots(&self, s: StateId) -> u32 {
        let st = self.state(s);
        (self.tables.state(s).size + st.industry_level).saturating_sub(st.lost_slots)
    }

    pub fn slots_used(&self, s: StateId) -> u32 {
        let st = self.state(s);
        st.facilities.len() as u32
            + st.queue.iter().filter(|b| matches!(b.item, BuildItem::Facility(_))).count() as u32
    }

    pub fn free_slots(&self, s: StateId) -> u32 {
        self.build_slots(s).saturating_sub(self.slots_used(s))
    }

    pub fn population_factor(&self, s: StateId) -> f64 {
        1.0 + self.state(s).population / 50.0
    }

    pub fn habitat_room(&self, c: &Colony) -> u32 {
        let per = self.tables.module(ModuleKind::Habitat).holds_colonists
            + if self.has_tech(TechId::ExpandedHabitats) { self.tables.tech(TechId::ExpandedHabitats).value as u32 } else { 0 };
        let yield_ = self.tables.body(c.body).habitat_yield;
        let habitats = c.modules.iter().filter(|m| m.kind == ModuleKind::Habitat).count() as f64;
        (habitats * per as f64 * yield_).floor() as u32
    }

    /// Colonists living in Habitats off Earth, for one seat (spec 15).
    pub fn off_world_colonists(&self, seat: Seat) -> u32 {
        self.colonies.iter().filter(|c| c.control.controller() == Some(seat)).map(|c| c.colonists).sum()
    }

    pub fn influence_threshold(&self, target: Target) -> i64 {
        let t = &self.tables.influence;
        let raw = match target {
            Place::State(s) => t.state_threshold_base + t.state_threshold_per_size * self.tables.state(s).size as i64,
            Place::Colony(c) => self.colony(c).map(|c| t.colony_threshold_per_colonist * c.colonists as i64).unwrap_or(i64::MAX / 4),
        };
        if self.has_tech(TechId::GreenConsensus) {
            let m = self.tables.tech(TechId::GreenConsensus).influence_threshold_multiplier.unwrap_or(0.75);
            (raw as f64 * m).floor() as i64
        } else {
            raw
        }
    }

    /// A Nation State's Influence value (ticket #34): its card figure plus one per Industry Level raised.
    pub fn state_influence_value(&self, s: StateId) -> i64 {
        let card = self.tables.state(s);
        card.influence + (self.state(s).industry_level as i64 - card.industry_level as i64).max(0)
    }

    /// What the seat's online Embassies and Relays add to its Allotment (ticket #36).
    pub fn building_allotment(&self, seat: Seat) -> i64 {
        let earth: i64 = self
            .controlled_states(seat)
            .iter()
            .flat_map(|s| self.state(*s).facilities.iter())
            .filter(|f| f.online)
            .map(|f| self.tables.facility(f.kind).influence_allotment)
            .sum();
        let space: i64 = self
            .owned_colonies(seat)
            .iter()
            .flat_map(|c| self.colony(*c).into_iter().flat_map(|c| c.modules.iter()))
            .filter(|m| m.online)
            .map(|m| self.tables.module(m.kind).influence_allotment)
            .sum();
        earth + space
    }

    /// The Allotment: the base plus every controlled state's Influence value plus the buildings,
    /// times the Faction multiplier.
    pub fn influence_allotment(&self, seat: Seat) -> i64 {
        let t = &self.tables.influence;
        let states: i64 = self.controlled_states(seat).iter().map(|s| self.state_influence_value(*s)).sum();
        let base = t.allotment_base + states + self.building_allotment(seat);
        let m = self.tables.faction(self.kind(seat)).influence_multiplier;
        (base as f64 * m).floor() as i64
    }

    pub fn ships_at(&self, seat: Seat, body: BodyId) -> Vec<ShipId> {
        self.ships.iter().filter(|s| s.seat == seat && s.at == ShipAt::Body(body)).map(|s| s.id).collect()
    }

    pub fn armies_at(&self, place: Place) -> Vec<ArmyId> {
        self.armies.iter().filter(|a| a.at == ArmyAt::Place(place)).map(|a| a.id).collect()
    }

    pub fn armies_of_seat_at(&self, seat: Seat, place: Place) -> Vec<ArmyId> {
        self.armies
            .iter()
            .filter(|a| a.at == ArmyAt::Place(place) && self.army_seat(a) == Some(seat))
            .map(|a| a.id)
            .collect()
    }

    pub fn ship_stack_strength(&self, seat: Seat, body: BodyId) -> i64 {
        self.ships.iter().filter(|s| s.seat == seat && s.at == ShipAt::Body(body)).map(|s| self.ship_strength(s)).sum()
    }

    pub fn army_stack_strength(&self, seat: Seat, place: Place) -> i64 {
        self.armies
            .iter()
            .filter(|a| a.at == ArmyAt::Place(place) && self.army_seat(a) == Some(seat))
            .map(|a| self.army_strength(a))
            .sum()
    }

    pub fn free_slots_on(&self, body: BodyId) -> Vec<u32> {
        let total = self.tables.body(body).colony_slots;
        (0..total).filter(|i| !self.colonies.iter().any(|c| c.body == body && c.slot == *i)).collect()
    }

    pub fn colony_at(&self, body: BodyId, slot: u32) -> Option<&Colony> {
        self.colonies.iter().find(|c| c.body == body && c.slot == slot)
    }

    /// Orbital Control at a Body (spec 9.3): a warship there, and no enemy warship still engaged.
    pub fn orbital_control(&self, body: BodyId) -> Option<Seat> {
        let warships = |seat: Seat| {
            self.ships.iter().any(|s| s.seat == seat && s.at == ShipAt::Body(body) && s.kind.is_warship() && !s.escaped)
        };
        let a = warships(Seat(0));
        let b = warships(Seat(1));
        match (a, b) {
            (true, false) => Some(Seat(0)),
            (false, true) => Some(Seat(1)),
            _ => None,
        }
    }

    /// Whether a seat may land Armies and Colonists at a Body (spec 9.3).
    pub fn may_land(&self, seat: Seat, body: BodyId) -> bool {
        match self.orbital_control(body) {
            Some(s) => s == seat,
            None => {
                // Nobody holds it: allowed only if nobody contests it, meaning no enemy warship present.
                !self.ships.iter().any(|s| s.seat != seat && s.at == ShipAt::Body(body) && s.kind.is_warship() && !s.escaped)
            }
        }
    }

    pub fn transit_cost(&self, from: BodyId, to: BodyId) -> (u32, i64) {
        let card = if from == BodyId::Earth || to == BodyId::Earth {
            self.tables.body(if from == BodyId::Earth { to } else { from })
        } else {
            self.tables.body(BodyId::Mars)
        };
        let fuel = if self.has_tech(TechId::EfficientTransit) {
            (card.transit_fuel as f64 * self.tables.tech(TechId::EfficientTransit).value).floor() as i64
        } else {
            card.transit_fuel
        };
        (card.transit_turns.max(1), fuel)
    }

    pub fn industry_cost(&self, seat: Seat) -> i64 {
        match self.kind(seat) {
            FactionKind::Prospectors => self.tables.industry_level.materials_cheap_industry,
            FactionKind::Custodians => self.tables.industry_level.materials,
        }
    }

    pub fn is_over(&self) -> bool {
        self.outcome.is_some()
    }

    pub fn log(&mut self, line: impl Into<String>) {
        self.log.push(line.into());
    }
}

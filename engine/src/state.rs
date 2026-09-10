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
    /// Version 0.04 (ticket #46): a Space Station in an orbital slot rather than a Colony on the ground.
    pub in_orbit: bool,
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
    pub launches_pending: [u32; SEAT_COUNT],
    /// Emissions a card (Permafrost Thaw) adds at the next Climate phase, worldwide.
    pub card_emissions_next: f64,
    /// Restoration bought in the last Orders phase, in ppm, for the next Climate phase only.
    pub restoration_next: f64,
}

#[derive(Debug, Clone)]
pub struct Research {
    pub current: Option<TechId>,
    pub progress: i64,
    pub contributions: [i64; SEAT_COUNT],
    pub done: Vec<TechId>,
    /// Research produced while no Tech was chosen; flows into the next one.
    pub unallocated: i64,
    /// Who must pick the next Tech, when a human has to.
    pub awaiting_pick: Option<Seat>,
    pub last_lead: Option<Seat>,
    /// Ticket #50: the turn each seat last picked a Tech, so a tie in contributions goes to the
    /// seat that has picked least recently. None means it has never picked, which counts as longest ago.
    pub last_picked_turn: [Option<u32>; SEAT_COUNT],
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
    /// Ticket #50: all the Research this seat's own Labs have produced, counted at production.
    pub research_total: i64,
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

/// One party in a Battle (ticket #50): a Battle is a melee of every Faction present, so the
/// Battle Report lists each of them rather than an attacker and a defender.
#[derive(Debug, Clone)]
pub struct BattleParty {
    /// None for a neutral state's own Armies.
    pub seat: Option<Seat>,
    /// True for the party whose Attack or Intercept started the Battle.
    pub aggressor: bool,
    pub units: String,
    pub strength: i64,
    pub hits: u32,
    pub destroyed: Vec<String>,
    pub escaped: Vec<String>,
}

/// One line of the Battle Report (spec 10.3), amended by ticket #50: every party present.
#[derive(Debug, Clone)]
pub struct BattleLine {
    pub place: String,
    pub parties: Vec<BattleParty>,
    pub result: String,
}

impl BattleLine {
    /// The seat whose order started the Battle, for the log and the map.
    pub fn aggressor(&self) -> Option<Seat> {
        self.parties.iter().find(|p| p.aggressor).and_then(|p| p.seat)
    }
    pub fn text(&self, names: &dyn Fn(Seat) -> String, neutral: &str) -> String {
        let listed: Vec<String> = self
            .parties
            .iter()
            .map(|p| {
                format!(
                    "{}{} ({}, strength {}, {} hit(s) landed; destroyed: {}; escaped: {})",
                    p.seat.map(names).unwrap_or_else(|| neutral.to_string()),
                    if p.aggressor { ", attacking," } else { "," },
                    p.units,
                    p.strength,
                    p.hits,
                    if p.destroyed.is_empty() { "none".to_string() } else { p.destroyed.join(", ") },
                    if p.escaped.is_empty() { "none".to_string() } else { p.escaped.join(", ") },
                )
            })
            .collect();
        format!("{}: {}. {}", self.place, listed.join(" against "), self.result)
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
    pub seats: [SeatState; SEAT_COUNT],
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

/// Ticket #50: every game seats all four Factions. The player picks one Faction and a start
/// continent; seat 0 is theirs and seats 1 to 3 hold the other three in enum order, all AI.
pub struct NewGame {
    pub seed: u64,
    pub player: FactionKind,
    /// True in simulate mode, where seat 0 plays itself.
    pub player_is_ai: bool,
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
            research_total: 0,
            income_last_turn: Stockpile::default(),
            income_sources: Vec::new(),
        };
        // Seat 0 is the player's Faction; the other three follow in enum order (ticket #50).
        let mut kinds: Vec<FactionKind> = vec![setup.player];
        kinds.extend(FactionKind::ALL.into_iter().filter(|k| *k != setup.player));
        let seats: [SeatState; SEAT_COUNT] = std::array::from_fn(|i| seat(kinds[i], i > 0 || setup.player_is_ai));
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
            seats,
            states,
            colonies: Vec::new(),
            ships: Vec::new(),
            armies: Vec::new(),
            climate: Climate {
                co2: tables.climate.starting_co2,
                temperature: tables.climate.base_temperature,
                last: EmissionsBreakdown::default(),
                launches_pending: [0; SEAT_COUNT],
                card_emissions_next: 0.0,
                restoration_next: 0.0,
            },
            research: Research {
                current: None,
                progress: 0,
                contributions: [0; SEAT_COUNT],
                done: Vec::new(),
                unallocated: 0,
                awaiting_pick: Some(Seat(0)),
                last_lead: None,
                last_picked_turn: [None; SEAT_COUNT],
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
        // Version 0.04 (ticket #46): a bare station over Earth for each Faction whose card names one.
        // Ticket #50: the Arkwrights name none, so two of Earth's five slots stand free.
        for seat in Seat::ALL {
            let Some(want) = game.tables.faction(game.kind(seat)).start_station.clone() else { continue };
            let Some(slot) = game.tables.body(BodyId::Earth).stations.iter().position(|n| *n == want) else { continue };
            let id = ColonyId(game.fresh_id());
            game.colonies.push(Colony { id, body: BodyId::Earth, slot: slot as u32, control: Control::Controlled(seat), modules: Vec::new(), colonists: 0, queue: Vec::new(), grid_failed: false, founded_turn: 1, in_orbit: true });
        }
        // Starting positions (spec 14.3, ticket #50): the player's pick, then each AI seat in turn.
        let mut taken = vec![setup.player_start];
        for _ in Seat::ALL.into_iter().skip(1) {
            let pick = game.ai_start_state(&taken);
            taken.push(pick);
        }
        for (sid, seat) in taken.iter().zip(Seat::ALL) {
            game.take_control(*sid, seat);
            let st = game.state_mut(*sid);
            st.facilities.push(Facility { kind: FacilityKind::LaunchSite, online: true, offline_until_resolution: false });
        }
        let places: Vec<String> = Seat::ALL
            .into_iter()
            .map(|seat| {
                format!(
                    "seat {} {} ({}) starts in {}",
                    seat.0,
                    game.seats[seat.index()].kind.name(),
                    if game.seats[seat.index()].ai { "AI" } else { "player" },
                    game.tables.state(taken[seat.index()]).name
                )
            })
            .collect();
        let line = format!("New game, seed {}. {}.", setup.seed, places.join("; "));
        game.log.push(line);
        game
    }

    /// The start state for the next AI seat (spec 14.3, ticket #50): the free Nation State that is
    /// NOT adjacent to any state already taken, with the highest Industry Level, ties by population;
    /// if every free state touches a taken one, the highest Industry Level free state, ties by
    /// population. A tie the population does not settle keeps the table's order.
    pub fn ai_start_state(&self, taken: &[StateId]) -> StateId {
        let adjacent: Vec<StateId> = taken.iter().flat_map(|t| self.tables.state(*t).neighbours.iter().copied()).collect();
        let free: Vec<&crate::data::StateCard> = self.tables.states.iter().filter(|c| !taken.contains(&c.id)).collect();
        let best = |list: &[&crate::data::StateCard]| -> Option<StateId> {
            let mut best: Option<&crate::data::StateCard> = None;
            for c in list {
                let better = match best {
                    None => true,
                    Some(b) => c.industry_level > b.industry_level || (c.industry_level == b.industry_level && c.population > b.population),
                };
                if better {
                    best = Some(c);
                }
            }
            best.map(|c| c.id)
        };
        let spread: Vec<&crate::data::StateCard> = free.iter().copied().filter(|c| !adjacent.contains(&c.id)).collect();
        best(&spread).or_else(|| best(&free)).unwrap_or(StateId::Asia)
    }

    /// Ticket #50: break a tie among seats by a draw from the game's own generator, so a seed stays
    /// replayable and no seat wins a tie for sitting first.
    pub fn random_tie(&mut self, tied: &[Seat]) -> Seat {
        match tied.len() {
            0 => Seat(0),
            1 => tied[0],
            n => {
                let i = crate::combat::Dice::pick(&mut self.rng, n);
                tied[i]
            }
        }
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
    /// Every game seats four different Factions (ticket #50), so a Faction's name names its seat.
    pub fn seat_name(&self, s: Seat) -> String {
        self.kind(s).name().to_string()
    }
    pub fn place_name(&self, p: Place) -> String {
        match p {
            Place::State(s) => self.tables.state(s).name.clone(),
            Place::Colony(c) => match self.colony(c) {
                // Ticket #46: a station is named for its orbital slot.
                Some(col) if col.in_orbit => format!("{} over {}", self.station_name(col.body, col.slot), self.tables.body(col.body).name),
                // Ticket #45: a Colony is named for its slot, a real place on its Body.
                Some(col) => format!("{} on {}", self.tables.body(col.body).slots[col.slot as usize].name, self.tables.body(col.body).name),
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
        // A station's Habitats are built for orbit: no Body yield applies (ticket #46).
        let yield_ = if c.in_orbit { 1.0 } else { self.tables.body(c.body).habitat_yield };
        let habitats = c.modules.iter().filter(|m| m.kind == ModuleKind::Habitat).count() as f64;
        (habitats * per as f64 * yield_).floor() as u32
    }

    /// Colonists living in Habitats off Earth, for one seat (spec 15).
    pub fn off_world_colonists(&self, seat: Seat) -> u32 {
        // Ticket #44: Colonists in Antarctica live on Earth.
        self.colonies.iter().filter(|c| c.control.controller() == Some(seat) && c.body != BodyId::Earth).map(|c| c.colonists).sum()
    }

    pub fn influence_threshold(&self, target: Target) -> i64 {
        let t = &self.tables.influence;
        let raw = match target {
            Place::State(s) => t.state_threshold_base + t.state_threshold_per_size * self.tables.state(s).size as i64,
            Place::Colony(c) => self
                .colony(c)
                .map(|c| t.colony_threshold_per_colonist * c.colonists as i64 + if c.in_orbit { t.station_threshold_base } else { 0 })
                .unwrap_or(i64::MAX / 4),
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
        let total = self.tables.body(body).colony_slots();
        (0..total).filter(|i| !self.colonies.iter().any(|c| !c.in_orbit && c.body == body && c.slot == *i)).collect()
    }

    pub fn colony_at(&self, body: BodyId, slot: u32) -> Option<&Colony> {
        self.colonies.iter().find(|c| !c.in_orbit && c.body == body && c.slot == slot)
    }

    /// Ticket #46: the orbital slots with no station yet.
    pub fn free_orbital_slots(&self, body: BodyId) -> Vec<u32> {
        let total = self.tables.body(body).orbital_slots;
        (0..total).filter(|i| !self.colonies.iter().any(|c| c.in_orbit && c.body == body && c.slot == *i)).collect()
    }

    pub fn station_at(&self, body: BodyId, slot: u32) -> Option<&Colony> {
        self.colonies.iter().find(|c| c.in_orbit && c.body == body && c.slot == slot)
    }

    /// The name an orbital slot's station carries, from bodies.toml.
    pub fn station_name(&self, body: BodyId, slot: u32) -> String {
        self.tables.body(body).stations.get(slot as usize).cloned().unwrap_or_else(|| format!("Station {}", slot + 1))
    }

    /// Orbital Control at a Body (spec 9.3, ticket #50): held by the one seat with a Frigate or
    /// Battleship there and no other seat's warship still engaged. Two or more, and nobody holds it.
    pub fn orbital_control(&self, body: BodyId) -> Option<Seat> {
        let mut holders = Seat::ALL
            .into_iter()
            .filter(|seat| self.ships.iter().any(|s| s.seat == *seat && s.at == ShipAt::Body(body) && s.kind.is_warship() && !s.escaped));
        match (holders.next(), holders.next()) {
            (Some(one), None) => Some(one),
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

    /// Ticket #45: a satellite and its parent are a local hop apart; two satellites of one parent a
    /// sibling hop; Earth and the Moon reach anything else at that Body's card figures; anything else
    /// (a moon of Mars to the Moon, say) is the farther card.
    pub fn transit_cost(&self, from: BodyId, to: BodyId) -> (u32, i64) {
        let t = &self.tables;
        let parent = |b: BodyId| t.body(b).parent;
        let near_earth = |b: BodyId| b == BodyId::Earth || parent(b) == Some(BodyId::Earth);
        let far = |b: BodyId| (t.body(b).transit_turns, t.body(b).transit_fuel);
        let (turns, fuel) = if parent(from) == Some(to) {
            (t.body(from).local_turns, t.body(from).local_fuel)
        } else if parent(to) == Some(from) {
            (t.body(to).local_turns, t.body(to).local_fuel)
        } else if parent(from).is_some() && parent(from) == parent(to) && !near_earth(from) {
            t.sibling_transit
        } else if near_earth(from) {
            far(to)
        } else if near_earth(to) || far(from).0 >= far(to).0 {
            far(from)
        } else {
            far(to)
        };
        let fuel = if self.has_tech(TechId::EfficientTransit) { (fuel as f64 * t.tech(TechId::EfficientTransit).value).floor() as i64 } else { fuel };
        (turns.max(1), fuel)
    }

    /// Cheap Industry, the Prospectors' signature rule (spec 14.2).
    pub fn industry_cost(&self, seat: Seat) -> i64 {
        match self.kind(seat) {
            FactionKind::Prospectors => self.tables.industry_level.materials_cheap_industry,
            _ => self.tables.industry_level.materials,
        }
    }

    pub fn is_over(&self) -> bool {
        self.outcome.is_some()
    }

    pub fn log(&mut self, line: impl Into<String>) {
        self.log.push(line.into());
    }
}

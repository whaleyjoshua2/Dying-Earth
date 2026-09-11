//! The whole state of one game, in glossary words.

use crate::data::Tables;
use crate::ids::*;
pub use crate::report::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Stockpile {
    pub materials: i64,
    pub fuel: i64,
    pub energy: i64,
    /// Version 0.03 (ticket #35).
    pub ducats: i64,
}

/// A Nation State is neutral, controlled, or occupied (spec 8.1, 8.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

/// Ticket #52: where a rise in Unrest comes from, which decides what damps it (rule 4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnrestSource {
    /// A population fall, build slots lost to Sea Level, or a Heatwave, Wildfire or Storm Surge.
    Climate,
    /// Refugees arriving.
    Refugees,
    /// Occupation, a mothball, a decommission, the Unrest card: nothing damps these.
    Plain,
}

/// Ticket #54 (version 0.05): what a Mothball, Restart or Decommission order does to a building.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildingChange {
    /// Free, lands at this Resolution: the building produces nothing, pays no Energy upkeep and
    /// emits nothing, and keeps its slot.
    Mothball,
    /// Materials and a turn: it works again.
    Restart,
    /// A turn: half its Materials come back, its slot is freed and it is gone.
    Decommission,
}

impl BuildingChange {
    pub fn name(self) -> &'static str {
        match self {
            BuildingChange::Mothball => "Mothball",
            BuildingChange::Restart => "Restart",
            BuildingChange::Decommission => "Decommission",
        }
    }
    /// The past tense the Report uses when the change lands.
    pub fn done(self) -> &'static str {
        match self {
            BuildingChange::Mothball => "mothballed",
            BuildingChange::Restart => "restarted",
            BuildingChange::Decommission => "decommissioned",
        }
    }
}

/// Ticket #54: a change ordered on one building and the turn its Resolution lands it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingChange {
    pub what: BuildingChange,
    pub due_turn: u32,
    pub seat: Seat,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Facility {
    pub kind: FacilityKind,
    /// Shut down this turn by the Energy shortfall rule, or knocked offline by a card.
    pub online: bool,
    /// Set by a Wildfire: offline until the next Resolution.
    pub offline_until_resolution: bool,
    /// Ticket #53: a neutral Nation State that developed itself runs this Facility on its own, so it
    /// emits at x1.0 to nobody's Blame while nobody directs the state. A directed state ignores it.
    pub self_run: bool,
    /// Ticket #54: mothballed. It makes nothing, pays no upkeep, emits nothing, counts as online for
    /// no rule, and keeps its build slot until it is decommissioned.
    pub mothballed: bool,
    /// Ticket #54: a Mothball, Restart or Decommission ordered and not yet landed.
    pub change: Option<PendingChange>,
    /// Ticket #56: whether this Facility stands in one of its state's coastal slots. The sea takes
    /// coastal slots only, so a coastal Facility is the one it can destroy.
    pub coastal: bool,
}

impl Facility {
    pub fn new(kind: FacilityKind) -> Facility {
        Facility { kind, online: true, offline_until_resolution: false, self_run: false, mothballed: false, change: None, coastal: false }
    }
    /// Ticket #56: a Facility standing in a coastal slot.
    pub fn in_coastal_slot(kind: FacilityKind) -> Facility {
        Facility { coastal: true, ..Facility::new(kind) }
    }
    /// Ticket #54: standing, running and not mothballed - what every "while it is online" rule means.
    pub fn working(&self) -> bool {
        self.online && !self.mothballed
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Module {
    pub kind: ModuleKind,
    pub online: bool,
    /// Knocked offline by a card until the next Resolution (Reactor Leak).
    #[allow(dead_code)]
    pub offline_until_resolution: bool,
    /// Ticket #54: mothballed, exactly as a Facility is.
    pub mothballed: bool,
    pub change: Option<PendingChange>,
}

impl Module {
    pub fn new(kind: ModuleKind) -> Module {
        Module { kind, online: true, offline_until_resolution: false, mothballed: false, change: None }
    }
    /// Ticket #54: standing, running and not mothballed.
    pub fn working(&self) -> bool {
        self.online && !self.mothballed
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Build {
    pub item: BuildItem,
    pub seat: Seat,
    pub due_turn: u32,
    /// Ticket #56: the slot a Facility build in a Nation State reserved, coastal or inland. False
    /// for everything else, which has no slot of this kind to reserve.
    pub coastal: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NationState {
    pub id: StateId,
    pub population: f64,
    pub industry_level: u32,
    pub control: Control,
    pub facilities: Vec<Facility>,
    pub queue: Vec<Build>,
    /// Ticket #56: COASTAL slots the sea has taken, for good. The sea takes nothing else.
    pub lost_slots: u32,
    /// Ticket #56: what stood in those slots when the sea took them, oldest first, so the state
    /// card can say what a lost slot cost.
    pub drowned: Vec<FacilityKind>,
    /// Sea-level thresholds already applied to this state, by index into the table.
    pub thresholds_fired: Vec<bool>,
    /// Extra Emissions charged next Climate phase by a Wildfire.
    pub wildfire_emissions_next: f64,
    /// Ticket #52: Unrest, 0 to 10 (9 while the state is neutral). Ticket #53: it moves in halves.
    pub unrest: f64,
    /// Ticket #53: the state changed hands this turn, which is the one turn its Unrest does not
    /// fall: a population with a fresh grievance is not calmed by the passing of a month.
    pub changed_hands: bool,
    /// Population that arrived here as refugees this turn; charged as Unrest once, at the end of
    /// Resolution, so the per-turn cap counts the whole turn's flows together.
    pub refugees_in: f64,
    /// The Unrest last named in the Report, so a crossing of 4, 7 or 10 is reported once.
    pub unrest_reported: f64,
    /// Ticket #53: the turn the state's current run of neutrality began, None while it is held or
    /// Occupied. Neutral Development counts from here, so a state taken and then freed starts a
    /// fresh six-turn count.
    pub neutral_since: Option<u32>,
    /// Ticket #54: what Leapfrog has taken off this state's per-person Emissions coefficient, for
    /// good. It starts at 0 and the coefficient never falls below the table's base.
    pub leapfrog: f64,
    /// Ticket #54: what a spent Strip Permit added to the card's Baseline Emissions, for good.
    pub baseline_rise: f64,
    /// Ticket #54: a Strip Permit has been taken here; one per state, ever.
    pub strip_permit_used: bool,
    /// Ticket #54: the last turn whose Income this state's Facilities double, while one runs.
    pub strip_permit_ends: Option<u32>,
}

/// Ticket #57: a calendar month of game time. Turn 1 is January 2030.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date {
    pub year: i64,
    pub month: u32,
}

impl Date {
    pub const MONTHS: [&'static str; 12] =
        ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    pub fn month_name(&self) -> &'static str {
        Date::MONTHS[(self.month.clamp(1, 12) - 1) as usize]
    }

    /// "July 2030".
    pub fn text(&self) -> String {
        format!("{} {}", self.month_name(), self.year)
    }
}

/// Ticket #57: a draw from a triangular distribution centred on 1.0 with `spread` either side, from
/// a uniform draw in 0..1. The distribution is symmetric, so its median is its mode and half the
/// draws fall each side; nothing ever falls outside 1 +/- spread.
pub fn triangular(u: f64, spread: f64) -> f64 {
    let u = u.clamp(0.0, 1.0);
    if u < 0.5 {
        1.0 - spread * (1.0 - (2.0 * u).sqrt())
    } else {
        1.0 + spread * (1.0 - (2.0 * (1.0 - u)).sqrt())
    }
}

/// Ticket #57: one Colony Slot's own four yields, drawn when the game starts as its Body's figures
/// times a factor from a triangular distribution centred on 1.0. Every yield a Module in a Colony
/// reads is the slot's, not the Body's; the Body's figures are what the slot drew from.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SlotYields {
    pub mine: f64,
    pub generator: f64,
    pub refinery: f64,
    pub habitat: f64,
}

impl SlotYields {
    /// The Body's own figures, which a station in orbit and any slot off the table read.
    pub fn of_body(card: &crate::data::BodyCard) -> SlotYields {
        SlotYields { mine: card.mine_yield, generator: card.generator_yield, refinery: card.refinery_yield, habitat: card.habitat_yield }
    }

    pub fn of_module(&self, kind: ModuleKind) -> f64 {
        match kind {
            ModuleKind::Mine => self.mine,
            ModuleKind::Generator => self.generator,
            ModuleKind::Refinery => self.refinery,
            // A Trade Post (ticket #35) follows the Habitat yield: trade goes where people live.
            ModuleKind::Habitat | ModuleKind::TradePost => self.habitat,
            _ => 1.0,
        }
    }

    /// "M 1.31 G 0.68 R 1.52 H 1.44", the figures the Surface Map writes under a slot's name.
    pub fn text(&self) -> String {
        format!("M {:.2} G {:.2} R {:.2} H {:.2}", self.mine, self.generator, self.refinery, self.habitat)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShipAt {
    Body(BodyId),
    Transit { from: BodyId, to: BodyId, turns_left: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArmyHome {
    State(StateId),
    Colony(ColonyId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArmyAt {
    Place(Place),
    Aboard(ShipId),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmissionsBreakdown {
    pub state_industry: f64,
    pub factories: f64,
    pub power_plants: f64,
    pub refineries: f64,
    pub launches: f64,
    pub population: f64,
    /// Emissions added by Event cards (Wildfire, the Methane Burst); never counted against Stabilization.
    pub cards: f64,
    /// Ticket #55: what the Permafrost Thaw Break adds every Climate phase once it has fired. Its
    /// own line: nobody's Blame, and never counted against a Stabilization run.
    pub permafrost: f64,
    pub sink: f64,
    /// Ticket #54: what the Scrubbers standing and online this Climate phase add to the Sink. It
    /// took Restoration's place in the breakdown and in the Stabilization sum.
    pub scrubbers: f64,
    /// Ticket #53: the Emissions of the sources each seat controlled this turn, which is what its
    /// Blame is made of. Cards are nobody's, and neither is a neutral state's industry or people.
    pub by_seat: [f64; SEAT_COUNT],
}

impl EmissionsBreakdown {
    /// Emissions that count against a Stabilization run: buildings, launches and population, not cards.
    pub fn counted(&self) -> f64 {
        self.state_industry + self.factories + self.power_plants + self.refineries + self.launches + self.population
    }
    pub fn total(&self) -> f64 {
        self.counted() + self.cards + self.permafrost
    }
    pub fn total_sink(&self) -> f64 {
        self.sink + self.scrubbers
    }
    pub fn net(&self) -> f64 {
        self.total() - self.total_sink()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Climate {
    pub co2: f64,
    pub temperature: f64,
    pub last: EmissionsBreakdown,
    /// Launches from Earth since the last Climate phase, per seat, charged next time.
    pub launches_pending: [u32; SEAT_COUNT],
    /// Emissions a card (the Methane Burst) adds at the next Climate phase, worldwide.
    /// Ticket #54: `removal_next` went with Restoration. What a Faction takes back is now the
    /// Scrubbers standing at the Climate phase, read off the board (`scrubber_removal_by_seat`).
    pub card_emissions_next: f64,
    /// Ticket #55: the Natural Sink as it stands. It opens at the table's figure and the Sink
    /// Weakens Break lowers it for good; every reader of the Sink reads this, so a weakened Sink
    /// moves the Stabilization bar and the Custodian AI's own pace with it.
    pub natural_sink: f64,
    /// Ticket #55: ppm the Permafrost Thaw Break adds to the world's Emissions every Climate phase.
    pub permafrost: f64,
    /// Ticket #55: which Breaks have fired, by index into `climate.toml`'s list. Each fires once.
    pub breaks_fired: Vec<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// Ticket #69 (version 0.05.5): every point the Labs of neutral and Occupied states have paid
    /// into the shared Tech over the game, for the simulation's report.
    #[serde(default)]
    pub neutral_total: i64,
}

impl Research {
    pub fn has(&self, t: TechId) -> bool {
        self.done.contains(&t)
    }
}

/// A card in the deck. Since ticket #25 every card is an Event; there are no Calm Cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Card {
    Event(EventId),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawnEvent {
    pub card: Card,
    pub target: EventTarget,
    pub scale: f64,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Discovery {
    pub body: BodyId,
    pub kind: ModuleKind,
    pub multiplier: f64,
    pub turns_left: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatState {
    pub kind: FactionKind,
    pub ai: bool,
    pub stockpile: Stockpile,
    /// Ticket #72 (version 0.05.5): Materials banked in the Venture Capital Fund (the Prospectors'
    /// first Victory part), the share of Materials output banked each Income, and what last
    /// Income banked. The running Extraction Total this replaces is retired.
    #[serde(default)]
    pub venture_fund: i64,
    #[serde(default)]
    pub venture_share: f64,
    #[serde(default)]
    pub venture_banked_last_turn: i64,
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
    /// Ticket #51: Research banked for the Archive, capped at what its remaining stages still need.
    pub archive_fund: i64,
    /// Ticket #51: Fund the Archive was ordered this turn, so this turn's Lab Research went to the
    /// fund and contributed nothing to the Research Lead.
    pub funding_archive: bool,
    /// Ticket #51: Provisional Findings is in force this turn, because last turn's Research went to
    /// the shared Tech. True at the start of the game.
    pub provisional_findings: bool,
    /// Ticket #52: the state a Resettle order steers this Faction's refugee flows to, until the
    /// next Climate phase spends it.
    pub resettle_to: Option<StateId>,
    /// Ticket #53: every ppm the sources this Faction controlled have emitted, over the whole game.
    pub blame_emitted: f64,
    /// Ticket #53: every ppm this Faction has removed, over the whole game.
    pub blame_removed: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    Win { seat: Seat, margin_note: String },
    Draw { note: String },
    Collapse,
}

/// One party in a Battle (ticket #50): a Battle is a melee of every Faction present, so the
/// Battle Report lists each of them rather than an attacker and a defender.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// Ticket #56: Antarctica's three Colony Slots on Earth are shut under the ice until the
    /// Temperature has stood at or above `antarctica_opens_at` in a Climate phase. Once open they
    /// stay open, however far the Temperature comes back down.
    pub antarctica_open: bool,
    /// Ticket #57: every Colony Slot's own four yields, drawn from the seeded generator when the
    /// game starts, keyed by Body and slot. A free slot has them too: they are what a Colony
    /// founded there would get.
    pub slot_yields: BTreeMap<(BodyId, u32), SlotYields>,
    /// Ticket #64: nobody sits at this game; all four seats are the computer's and the interface
    /// is a spectator's. It changes what the Report is written for, and nothing about the rules.
    pub spectator: bool,
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
            venture_fund: 0,
            venture_share: 0.0,
            venture_banked_last_turn: 0,
            stabilization_run: 0,
            influence: BTreeMap::new(),
            influenced_this_turn: Vec::new(),
            allotment: 0,
            research_last_turn: 0,
            research_total: 0,
            income_last_turn: Stockpile::default(),
            income_sources: Vec::new(),
            archive_fund: 0,
            funding_archive: false,
            provisional_findings: true,
            resettle_to: None,
            blame_emitted: 0.0,
            blame_removed: 0.0,
        };
        // Seat 0 is the player's Faction; the other three follow in enum order (ticket #50).
        let mut kinds: Vec<FactionKind> = vec![setup.player];
        kinds.extend(FactionKind::ALL.into_iter().filter(|k| *k != setup.player));
        let seats: [SeatState; SEAT_COUNT] = std::array::from_fn(|i| seat(kinds[i], i > 0 || setup.player_is_ai));
        let mut states: Vec<NationState> = tables
            .states
            .iter()
            .map(|c| NationState {
                id: c.id,
                population: c.population,
                industry_level: c.industry_level,
                control: Control::Neutral,
                // Ticket #56: the start Facilities take coastal slots first, in the table's order.
                facilities: {
                    let start_slots = c.size + c.industry_level + tables.base_slots;
                    let coastal = (tables.coastal_per_exposure * c.coastal_exposure).min(start_slots.saturating_sub(1));
                    let mut on_the_coast = 0;
                    c.start_facilities
                        .iter()
                        .copied()
                        .map(|k| {
                            // Ticket #69 (version 0.05.5): a start Research Lab stands inland, so the
                            // sea never takes the world's Research.
                            if k == FacilityKind::ResearchLab {
                                return Facility::new(k);
                            }
                            if on_the_coast < coastal {
                                on_the_coast += 1;
                                Facility::in_coastal_slot(k)
                            } else {
                                Facility::new(k)
                            }
                        })
                        .collect()
                },
                queue: Vec::new(),
                lost_slots: 0,
                drowned: Vec::new(),
                thresholds_fired: vec![false; tables.climate.sea_level_thresholds.len()],
                wildfire_emissions_next: 0.0,
                unrest: c.unrest,
                changed_hands: false,
                refugees_in: 0.0,
                unrest_reported: c.unrest,
                // Every state is neutral when the game opens; the clocks are staggered below, and
                // `take_control` clears the four the Factions begin holding.
                neutral_since: Some(1),
                leapfrog: 0.0,
                baseline_rise: 0.0,
                strip_permit_used: false,
                strip_permit_ends: None,
            })
            .collect();
        // Ticket #53: the opening clocks are staggered by the seed across the first development
        // period, so the states that start neutral do not all develop on the same turn.
        {
            use rand::Rng;
            let period = tables.development.turns.max(1);
            for st in states.iter_mut() {
                st.neutral_since = Some(1 + rng.random_range(0..period));
            }
        }
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
                natural_sink: tables.climate.natural_sink,
                permafrost: 0.0,
                breaks_fired: vec![false; tables.climate.breaks.len()],
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
                neutral_total: 0,
            },
            deck,
            discoveries: Vec::new(),
            solar_maximum_next: false,
            last_event: None,
            report: Report::default(),
            outcome: None,
            next_id: 1,
            pending: crate::orders::Pending::default(),
            antarctica_open: false,
            slot_yields: BTreeMap::new(),
            spectator: false,
            log: Vec::new(),
            tables,
        };
        // Ticket #57: every Colony Slot on every Body draws its own four yields, in Body order then
        // slot order, so a seed always deals the same board.
        game.draw_slot_yields();
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
            game.add_start_facility(*sid, FacilityKind::LaunchSite);
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

    /// The start state for the next AI seat (spec 14.3, ticket #50). The rule itself lives on the
    /// tables, since ticket #64 asks it before there is a game.
    pub fn ai_start_state(&self, taken: &[StateId]) -> StateId {
        self.tables.ai_start_state(taken)
    }

    /// Ticket #64: a game nobody sits at. All four seats are the computer's, seat 0 holds the
    /// Custodians so the seating reads as it does in simulate mode, and seat 0's start is the first
    /// pick of the same spreading rule the other three are dealt by, not a continent anybody chose.
    pub fn spectate(tables: std::sync::Arc<Tables>, seed: u64) -> Game {
        let start = tables.ai_start_state(&[]);
        let mut game = Game::new(tables, NewGame { seed, player: FactionKind::Custodians, player_is_ai: true, player_start: start });
        game.spectator = true;
        game
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

    // ---------------------------------------------------------------- Ticket #51: Provisional Findings

    /// Provisional Findings (ticket #51): the Archivists already have half the effect of the Tech
    /// under research, while last turn's Research went to the shared Tech.
    pub fn provisional_findings(&self, seat: Seat) -> bool {
        self.kind(seat) == FactionKind::Archivists && self.seat(seat).provisional_findings
    }

    /// True while this seat reads the Tech at half strength: it is not done, but it is the one under
    /// research and Provisional Findings is in force.
    fn reads_half(&self, seat: Seat, t: TechId) -> bool {
        !self.has_tech(t) && self.research.current == Some(t) && self.provisional_findings(seat)
    }

    /// A multiplier Tech read for one seat: its full value once done, `1 + (value - 1) / 2` under
    /// Provisional Findings, and 1.0 (no effect) otherwise.
    pub fn tech_multiplier(&self, seat: Seat, t: TechId) -> f64 {
        let v = self.tables.tech(t).value;
        if self.has_tech(t) {
            v
        } else if self.reads_half(seat, t) {
            1.0 + (v - 1.0) / 2.0
        } else {
            1.0
        }
    }

    /// An additive Tech read for one seat: its full value once done, half rounded down under
    /// Provisional Findings, and 0 otherwise.
    pub fn tech_addition(&self, seat: Seat, t: TechId) -> i64 {
        let v = self.tables.tech(t).value as i64;
        if self.has_tech(t) {
            v
        } else if self.reads_half(seat, t) {
            v / 2
        } else {
            0
        }
    }

    // ---------------------------------------------------------------- Ticket #51: the Archive

    /// The Colony holding this seat's Archive Module.
    pub fn archive_colony(&self, seat: Seat) -> Option<ColonyId> {
        self.colonies
            .iter()
            .find(|c| c.control.controller() == Some(seat) && c.modules.iter().any(|m| m.kind == ModuleKind::Archive))
            .map(|c| c.id)
    }

    /// Ticket #68 (version 0.05.5): the Archive Module stands at a Colony of this seat's.
    pub fn archive_built(&self, seat: Seat) -> bool {
        self.archive_colony(seat).is_some()
    }

    /// Ticket #68: the Archive Module is in a Colony's build queue for this seat.
    pub fn archive_ordered(&self, seat: Seat) -> bool {
        self.colonies.iter().flat_map(|c| c.queue.iter()).any(|b| b.seat == seat && b.item == BuildItem::Module(ModuleKind::Archive))
    }

    /// Ticket #68: what the Archive fund may hold. The whole requirement once the Module stands;
    /// only `banked_before_built` of it (a quarter) until then.
    pub fn archive_fund_cap(&self, seat: Seat) -> i64 {
        let a = &self.tables.archive;
        if self.archive_built(seat) { a.research } else { (a.research as f64 * a.banked_before_built).floor() as i64 }
    }

    /// Ticket #68: the Archive Module stands and every point of its Research is paid.
    pub fn archive_complete(&self, seat: Seat) -> bool {
        self.archive_built(seat) && self.seat(seat).archive_fund >= self.tables.archive.research
    }

    /// The Archive is complete, its Colony is not Occupied, and the Module is online: the state the
    /// Archivists' Victory Condition asks for.
    pub fn archive_online(&self, seat: Seat) -> bool {
        if !self.archive_complete(seat) {
            return false;
        }
        let Some(cid) = self.archive_colony(seat) else { return false };
        let Some(col) = self.colony(cid) else { return false };
        !col.control.is_occupied() && col.modules.iter().any(|m| m.kind == ModuleKind::Archive && m.online)
    }

    /// Colonists living at the Colony that holds this seat's Archive.
    pub fn colonists_at_archive(&self, seat: Seat) -> u32 {
        self.archive_colony(seat).and_then(|c| self.colony(c)).map(|c| c.colonists).unwrap_or(0)
    }

    /// A Colony off Earth may hold the Archive; Antarctica and a station over Earth may not.
    pub fn may_hold_archive(&self, c: &Colony) -> bool {
        c.body != BodyId::Earth
    }

    // ---------------------------------------------------------------- Ticket #51: Steerage and the rest

    /// What one Colony Ship of this seat carries: the card figure, +2 with Expanded Habitats
    /// (version 0.04 section 4), times the Faction's own multiplier (Steerage doubles it).
    pub fn colony_ship_capacity(&self, seat: Seat) -> u32 {
        let base = self.tables.unit(UnitKind::ColonyShip).carries_colonists as i64 + self.tech_addition(seat, TechId::ExpandedHabitats);
        let m = self.tables.faction(self.kind(seat)).colony_ship_capacity_multiplier;
        (base.max(0) as f64 * m).floor().max(0.0) as u32
    }

    /// The population a lift from a Launch Site takes for this many Colonists (Steerage doubles it).
    pub fn lift_population(&self, seat: Seat, colonists: u32) -> f64 {
        0.1 * colonists as f64 * self.tables.faction(self.kind(seat)).lift_population_multiplier
    }

    /// What a Colony Module costs this seat in Materials, rounded down (ticket #51).
    pub fn module_materials(&self, seat: Seat, kind: ModuleKind) -> i64 {
        let base = self.tables.module(kind).materials as f64;
        (base * self.tables.faction(self.kind(seat)).module_materials_multiplier).floor() as i64
    }

    /// What a Space Station costs this seat in Materials, rounded down (ticket #51).
    /// Ticket #72 (version 0.05.5): what a Facility costs this seat in Materials: the row's figure
    /// times the Faction's multiplier (the Prospectors' 0.85), rounded down.
    pub fn facility_materials(&self, seat: Seat, kind: FacilityKind) -> i64 {
        let base = self.tables.facility(kind).materials as f64;
        (base * self.tables.faction(self.kind(seat)).facility_materials_multiplier).floor() as i64
    }

    pub fn station_materials(&self, seat: Seat) -> i64 {
        let base = self.tables.station_materials as f64;
        (base * self.tables.faction(self.kind(seat)).station_materials_multiplier).floor() as i64
    }

    /// What a Ship costs this seat: the units.toml figure, or the Faction's own Colony Ship price.
    pub fn ship_materials(&self, seat: Seat, kind: UnitKind) -> i64 {
        let card = self.tables.faction(self.kind(seat));
        match (kind, card.colony_ship_materials) {
            (UnitKind::ColonyShip, Some(m)) => m,
            _ => self.tables.unit(kind).materials,
        }
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
        base + self.tech_addition(s.seat, TechId::HardenedHulls)
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
        self.restart_neutrality_clock(s);
    }

    /// Ticket #53: called wherever a Nation State's control is written. A state that is neutral
    /// counts its six turns from the turn after the change, since the change lands in a Resolution
    /// the turn is already half through; a state held or Occupied has no clock at all.
    pub fn restart_neutrality_clock(&mut self, s: StateId) {
        let turn = self.turn;
        let st = self.state_mut(s);
        st.neutral_since = if st.control == Control::Neutral { Some(turn + 1) } else { None };
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

    /// Build slots a Nation State has now (spec 4.2, 11.4, ticket #56): Size + Industry Level +
    /// `base_slots`, less the coastal slots the sea has taken. They come in two rows, coastal and
    /// inland, and the sea takes only from the first.
    pub fn build_slots(&self, s: StateId) -> u32 {
        self.coastal_slots(s) + self.inland_slots(s)
    }

    /// Ticket #56: the slots a state begins the game with: Size + `base_slots` + the Industry Level
    /// on its card. Every slot beyond these came from a raise, and every raise adds an inland slot.
    pub fn start_slots(&self, s: StateId) -> u32 {
        let card = self.tables.state(s);
        card.size + card.industry_level + self.tables.base_slots
    }

    /// Ticket #56: the coastal slots the state began with, before the sea took any:
    /// `coastal_per_exposure` x Coastal Exposure, never more than the start slots less one.
    pub fn coastal_slots_start(&self, s: StateId) -> u32 {
        let exposure = self.tables.state(s).coastal_exposure;
        (self.tables.coastal_per_exposure * exposure).min(self.start_slots(s).saturating_sub(1))
    }

    /// Ticket #56: the coastal slots it has left, once the sea has had its thresholds.
    pub fn coastal_slots(&self, s: StateId) -> u32 {
        self.coastal_slots_start(s).saturating_sub(self.state(s).lost_slots)
    }

    /// Ticket #56: its inland slots, which the sea never touches and a raise always adds to.
    pub fn inland_slots(&self, s: StateId) -> u32 {
        let raised = self.state(s).industry_level.saturating_sub(self.tables.state(s).industry_level);
        self.start_slots(s) - self.coastal_slots_start(s) + raised
    }

    /// Ticket #56: coastal slots with something standing or building in them.
    pub fn coastal_used(&self, s: StateId) -> u32 {
        self.slots_used_in(s, true)
    }

    /// Ticket #56: inland slots with something standing or building in them.
    pub fn inland_used(&self, s: StateId) -> u32 {
        self.slots_used_in(s, false)
    }

    fn slots_used_in(&self, s: StateId, coastal: bool) -> u32 {
        let st = self.state(s);
        st.facilities.iter().filter(|f| self.takes_slot(f.kind) && f.coastal == coastal).count() as u32
            + st.queue.iter().filter(|b| b.coastal == coastal && matches!(b.item, BuildItem::Facility(k) if self.takes_slot(k))).count() as u32
    }

    /// Ticket #56: which row the next Facility of this kind would stand in, or None when it does not
    /// fit at all. A Sea Wall always wants a coastal slot; anything else fills an inland slot while
    /// one is free and takes a coastal one otherwise. `taken_coastal` and `taken_inland` are slots
    /// already spoken for by orders in the same turn that have not been committed yet.
    pub fn next_slot_is_coastal(&self, s: StateId, kind: FacilityKind, taken_coastal: u32, taken_inland: u32) -> Option<bool> {
        if !self.takes_slot(kind) {
            return Some(false);
        }
        let free_coastal = self.free_coastal(s).saturating_sub(taken_coastal);
        let free_inland = self.free_inland(s).saturating_sub(taken_inland);
        if self.tables.facility(kind).coastal_only {
            return if free_coastal > 0 { Some(true) } else { None };
        }
        if free_inland > 0 {
            Some(false)
        } else if free_coastal > 0 {
            Some(true)
        } else {
            None
        }
    }

    /// Ticket #56: a Facility that stands from the start, or arrives without an order (a Faction's
    /// Launch Site). It takes a coastal slot first, as the start Facilities do.
    pub fn add_start_facility(&mut self, s: StateId, kind: FacilityKind) {
        let coastal = self.takes_slot(kind) && self.free_coastal(s) > 0;
        let f = if coastal { Facility::in_coastal_slot(kind) } else { Facility::new(kind) };
        self.state_mut(s).facilities.push(f);
    }

    pub fn free_coastal(&self, s: StateId) -> u32 {
        self.coastal_slots(s).saturating_sub(self.coastal_used(s))
    }

    pub fn free_inland(&self, s: StateId) -> u32 {
        self.inland_slots(s).saturating_sub(self.inland_used(s))
    }

    /// Ticket #54: a Scrubber takes no build slot, so neither the standing ones nor the ones on
    /// order count against the state's slots.
    pub fn takes_slot(&self, kind: FacilityKind) -> bool {
        !self.tables.facility(kind).no_slot
    }

    pub fn slots_used(&self, s: StateId) -> u32 {
        let st = self.state(s);
        st.facilities.iter().filter(|f| self.takes_slot(f.kind)).count() as u32
            + st.queue.iter().filter(|b| matches!(b.item, BuildItem::Facility(k) if self.takes_slot(k))).count() as u32
    }

    pub fn free_slots(&self, s: StateId) -> u32 {
        self.build_slots(s).saturating_sub(self.slots_used(s))
    }

    pub fn population_factor(&self, s: StateId) -> f64 {
        1.0 + self.state(s).population / 50.0
    }

    pub fn habitat_room(&self, c: &Colony) -> u32 {
        // Ticket #51: Expanded Habitats and the Faction's own Habitat capacity are read for whoever
        // holds the Colony, since Provisional Findings gives the Archivists half the Tech early.
        let seat = c.control.controller();
        let per = self.tables.module(ModuleKind::Habitat).holds_colonists as i64
            + seat.map(|s| self.tech_addition(s, TechId::ExpandedHabitats)).unwrap_or(0);
        let faction = seat.map(|s| self.tables.faction(self.kind(s)).habitat_capacity_multiplier).unwrap_or(1.0);
        // A station's Habitats are built for orbit: no Body yield applies (ticket #46). On a surface
        // it is the slot's own Habitat yield, not the Body's (ticket #57).
        let yield_ = if c.in_orbit { 1.0 } else { self.slot_yields(c.body, c.slot).habitat };
        let habitats = c.modules.iter().filter(|m| m.kind == ModuleKind::Habitat).count() as f64;
        (habitats * per.max(0) as f64 * yield_ * faction).floor() as u32
    }

    /// Ticket #51: the seat's Colonists at one Body, counting a station over it as being there.
    pub fn colonists_at_body(&self, seat: Seat, body: BodyId) -> u32 {
        self.colonies.iter().filter(|c| c.control.controller() == Some(seat) && c.body == body).map(|c| c.colonists).sum()
    }

    /// Ticket #51: the Bodies off Earth where this seat holds at least `each` Colonists. Antarctica
    /// and the stations over Earth are not Bodies for this: Earth is left out.
    pub fn bodies_settled(&self, seat: Seat, each: u32) -> u32 {
        BodyId::ALL
            .into_iter()
            .filter(|b| *b != BodyId::Earth && self.colonists_at_body(seat, *b) >= each)
            .count() as u32
    }

    /// Colonists living in Habitats off Earth, for one seat (spec 15).
    pub fn off_world_colonists(&self, seat: Seat) -> u32 {
        // Ticket #44: Colonists in Antarctica live on Earth.
        self.colonies.iter().filter(|c| c.control.controller() == Some(seat) && c.body != BodyId::Earth).map(|c| c.colonists).sum()
    }

    pub fn influence_threshold(&self, target: Target) -> i64 {
        self.threshold_with(target, if self.has_tech(TechId::GreenConsensus) { self.green_consensus_multiplier() } else { 1.0 })
    }

    /// The threshold as one seat reads it (ticket #51): Provisional Findings gives the Archivists
    /// half of Green Consensus while it is under research. Ticket #53: and its Blame raises it on
    /// every Nation State it does not control.
    pub fn influence_threshold_for(&self, seat: Seat, target: Target) -> i64 {
        let full = self.green_consensus_multiplier();
        let m = if self.has_tech(TechId::GreenConsensus) {
            full
        } else if self.research.current == Some(TechId::GreenConsensus) && self.provisional_findings(seat) {
            1.0 + (full - 1.0) / 2.0
        } else {
            1.0
        };
        self.threshold_with(target, m * self.blame_threshold_multiplier_on(seat, target))
    }

    /// Ticket #60: the Standing this seat needs to take `target` as it stands now -- its own
    /// threshold on a neutral place, and on a held one the greater of that and the holder's
    /// Standing plus the challenge margin ticket #41 put there. The Resolution, the AI and the
    /// state card all read this one computation, so the figure a card prints and the figure the
    /// Resolution applies cannot drift apart again, as they did from #41 to here.
    pub fn influence_needed_for(&self, seat: Seat, target: Target) -> i64 {
        let threshold = self.influence_threshold_for(seat, target);
        match self.place_control(target).controller() {
            Some(c) => threshold.max(self.seat(c).influence.get(&target).copied().unwrap_or(0) + self.tables.influence.challenge_margin),
            None => threshold,
        }
    }

    /// Ticket #53: Blame raises this seat's threshold on a Nation State it does not control, and
    /// on nothing else: never on a Colony, never on a Space Station, never on a place it holds.
    pub fn blame_threshold_multiplier_on(&self, seat: Seat, target: Target) -> f64 {
        match target {
            Place::State(_) if self.place_control(target).controller() != Some(seat) => self.blame_threshold_multiplier(seat),
            _ => 1.0,
        }
    }

    /// Ticket #53: the ppm this Faction is answerable for, over the whole game: what the sources it
    /// controlled emitted, less what it removed, floored at zero.
    pub fn blame(&self, seat: Seat) -> f64 {
        let s = self.seat(seat);
        (s.blame_emitted - s.blame_removed).max(0.0)
    }

    /// Ticket #53: the ppm a Faction removed beyond everything it ever emitted, which is what the
    /// panels call a credit. Zero for everyone who has emitted more than they took back.
    pub fn blame_credit(&self, seat: Seat) -> f64 {
        let s = self.seat(seat);
        (s.blame_removed - s.blame_emitted).max(0.0)
    }

    /// Ticket #53: the four Factions' Blame added together.
    pub fn blame_total(&self) -> f64 {
        Seat::ALL.into_iter().map(|s| self.blame(s)).sum()
    }

    /// Ticket #53: this Faction's share of the table's Blame; zero when nobody has any.
    pub fn blame_share(&self, seat: Seat) -> f64 {
        let total = self.blame_total();
        if total <= 0.0 { 0.0 } else { self.blame(seat) / total }
    }

    /// Ticket #53: 1 + (share - a fair quarter), floored at x1.0 and capped by the table.
    pub fn blame_threshold_multiplier(&self, seat: Seat) -> f64 {
        let b = &self.tables.influence.blame;
        (1.0 + (self.blame_share(seat) - b.fair_share)).clamp(1.0, b.cap)
    }

    fn green_consensus_multiplier(&self) -> f64 {
        self.tables.tech(TechId::GreenConsensus).influence_threshold_multiplier.unwrap_or(0.75)
    }

    fn threshold_with(&self, target: Target, multiplier: f64) -> i64 {
        let t = &self.tables.influence;
        let raw = match target {
            Place::State(s) => t.state_threshold_base + t.state_threshold_per_size * self.tables.state(s).size as i64,
            Place::Colony(c) => self
                .colony(c)
                .map(|c| t.colony_threshold_per_colonist * c.colonists as i64 + if c.in_orbit { t.station_threshold_base } else { 0 })
                .unwrap_or(i64::MAX / 4),
        };
        // Ticket #53: the multiplier now runs both ways (Green Consensus down, Blame up), so it is
        // always applied; the epsilon keeps 40 x 0.75 x 1.25 off the wrong side of a whole number.
        if (multiplier - 1.0).abs() < 1e-9 { raw } else { (raw as f64 * multiplier + 1e-9).floor() as i64 }
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
            .filter(|f| f.working())
            .map(|f| self.tables.facility(f.kind).influence_allotment)
            .sum();
        let space: i64 = self
            .owned_colonies(seat)
            .iter()
            .flat_map(|c| self.colony(*c).into_iter().flat_map(|c| c.modules.iter()))
            .filter(|m| m.working())
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

    // ------------------------------------------- Ticket #57: a Colony Slot's own four yields

    /// Draw every Colony Slot's four yields from the game's own generator: the parent Body's figure
    /// times a factor from a triangular distribution centred on 1.0 with `slot_yield_spread` either
    /// side, rounded to two decimals. Called once, when the game is made.
    fn draw_slot_yields(&mut self) {
        use rand::Rng;
        let spread = self.tables.slot_yield_spread;
        for body in BodyId::ALL {
            let card = self.tables.body(body).clone();
            for slot in 0..card.colony_slots() {
                let mut draw = |base: f64| {
                    let u: f64 = self.rng.random_range(0.0..1.0);
                    (base * triangular(u, spread) * 100.0).round() / 100.0
                };
                let y = SlotYields {
                    mine: draw(card.mine_yield),
                    generator: draw(card.generator_yield),
                    refinery: draw(card.refinery_yield),
                    habitat: draw(card.habitat_yield),
                };
                self.slot_yields.insert((body, slot), y);
            }
        }
    }

    /// One Colony Slot's four yields. A slot the table does not know (a station's orbital slot) reads
    /// its Body's own figures, so nothing off the surface changes.
    pub fn slot_yields(&self, body: BodyId, slot: u32) -> SlotYields {
        self.slot_yields.get(&(body, slot)).copied().unwrap_or_else(|| SlotYields::of_body(self.tables.body(body)))
    }

    /// The four yields a Colony reads: its slot's on a surface, its Body's in orbit, where a
    /// station's Habitats take no Body yield at all (ticket #46).
    pub fn colony_yields(&self, c: &Colony) -> SlotYields {
        if c.in_orbit {
            SlotYields::of_body(self.tables.body(c.body))
        } else {
            self.slot_yields(c.body, c.slot)
        }
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
    /// (a moon of Mars to the Moon, say) is the farther card. Ticket #57: a crossing between the
    /// Earth system and the Mars system pays what the phase angle this turn makes it pay instead.
    pub fn transit_cost(&self, from: BodyId, to: BodyId) -> (u32, i64) {
        self.transit_cost_at(from, to, self.turn)
    }

    /// The same at any turn, for the window tooltip and the AI's planning.
    pub fn transit_cost_at(&self, from: BodyId, to: BodyId, turn: u32) -> (u32, i64) {
        let tech = if self.has_tech(TechId::EfficientTransit) { self.tables.tech(TechId::EfficientTransit).value } else { 1.0 };
        self.transit_cost_with(from, to, 1.0, tech, turn)
    }

    /// The transit as one seat pays it (ticket #51): the Faction's own Fuel multiplier first, then
    /// Efficient Transit, multiplicative, rounded down once at the end.
    pub fn transit_cost_for(&self, seat: Seat, from: BodyId, to: BodyId) -> (u32, i64) {
        self.transit_cost_for_at(seat, from, to, self.turn)
    }

    pub fn transit_cost_for_at(&self, seat: Seat, from: BodyId, to: BodyId, turn: u32) -> (u32, i64) {
        let faction = self.tables.faction(self.kind(seat)).transit_fuel_multiplier;
        self.transit_cost_with(from, to, faction, self.tech_multiplier(seat, TechId::EfficientTransit), turn)
    }

    fn transit_cost_with(&self, from: BodyId, to: BodyId, faction: f64, tech: f64, turn: u32) -> (u32, i64) {
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
        // Ticket #57: a crossing between the two systems is not a fixed card any more. It costs the
        // Hohmann flight and the card's Fuel at the window, and more of both the further the phase
        // angle stands from it; the Faction multiplier and Efficient Transit apply after.
        let (turns, fuel) = match self.crossing_offset(from, to, turn) {
            None => (turns, fuel as f64),
            Some(offset) => {
                let tr = &t.transit;
                let days = tr.days_at_window + tr.days_per_degree * offset.abs();
                let turns = ((days / tr.days_per_turn).ceil() as u32).clamp(1, tr.max_turns);
                (turns, fuel as f64 * (1.0 + tr.fuel_per_degree * offset.abs()))
            }
        };
        let fuel = (fuel * faction * tech).floor() as i64;
        (turns.max(1), fuel)
    }

    // ---------------------------------------------- Ticket #57: the calendar and the real sky

    /// The year and month of a turn. Turn 1 is the game's start month, January 2030 (`victory.toml`),
    /// and each Turn is `months_per_turn` calendar months after the last (two since ticket #67,
    /// version 0.05.5), named by its first month alone: turn 2 is March 2030.
    pub fn date(&self, turn: u32) -> Date {
        let v = &self.tables.victory;
        let months = v.start_month - 1 + (turn.max(1) - 1) as i64 * v.months_per_turn;
        Date { year: v.start_year + months.div_euclid(12), month: (months.rem_euclid(12) + 1) as u32 }
    }

    /// "July 2030" for the turn the game stands on.
    pub fn date_text(&self) -> String {
        self.date(self.turn).text()
    }

    /// The instant a turn's sky is read at: the first of its month, 00:00 UTC, so turn 1 is exactly
    /// 2030-01-01 00:00 UTC, the moment the game begins.
    pub fn julian_day(&self, turn: u32) -> f64 {
        let d = self.date(turn);
        crate::ephemeris::julian_day(d.year, d.month as i64, 1)
    }

    /// Where a Body stands in the real sky at a turn: its heliocentric ecliptic longitude in degrees
    /// (0..360, J2000 frame). The Moon reads Earth's, Phobos and Deimos read Mars's.
    pub fn heliocentric_longitude(&self, body: BodyId, turn: u32) -> f64 {
        crate::ephemeris::position(self.tables.planet(body), self.julian_day(turn)).longitude
    }

    /// The phase angle at a turn: Mars's heliocentric longitude less Earth's, folded to -180..180.
    pub fn phase_angle(&self, turn: u32) -> f64 {
        crate::ephemeris::wrap_180(self.heliocentric_longitude(BodyId::Mars, turn) - self.heliocentric_longitude(BodyId::Earth, turn))
    }

    /// The window offset for a departure at a turn: the signed difference in degrees between the
    /// phase angle and the Hohmann departure angle. Zero is the launch window.
    ///
    /// Ticket #67 (version 0.05.5): a turn spans two months, over which the phase angle moves
    /// nearly thirty degrees, so the offset is the nearest the angle comes to the window ANYWHERE
    /// in the turn, from its first instant to the next turn's: zero when it crosses the window
    /// inside the turn, else the nearer end. Read at the first instant alone, no turn would ever
    /// stand at the window and every "window" crossing would pay a little over the card.
    pub fn window_offset(&self, turn: u32) -> f64 {
        self.span_offset(turn, self.tables.transit.hohmann_angle)
    }

    /// The same for the flight home, which wants Earth ahead of Mars instead.
    pub fn return_window_offset(&self, turn: u32) -> f64 {
        self.span_offset(turn, self.tables.transit.return_hohmann_angle)
    }

    /// The signed offset of the phase angle from `angle` over the span of a turn (see `window_offset`).
    /// A change of sign between the turn's two ends is a crossing only when the ends are near each
    /// other; a jump across the far side of the circle (+170 to -170) is not.
    fn span_offset(&self, turn: u32, angle: f64) -> f64 {
        let start = crate::ephemeris::wrap_180(self.phase_angle(turn) - angle);
        let end = crate::ephemeris::wrap_180(self.phase_angle(turn + 1) - angle);
        if start.signum() != end.signum() && (start - end).abs() < 90.0 {
            0.0
        } else if start.abs() <= end.abs() {
            start
        } else {
            end
        }
    }

    /// Which two systems a transit crosses, if it crosses at all, and the offset it pays. `None` for
    /// a hop inside the Earth system or inside the Mars system: those are unchanged.
    pub fn crossing_offset(&self, from: BodyId, to: BodyId, turn: u32) -> Option<f64> {
        let system = |b: BodyId| match b {
            BodyId::Earth | BodyId::Moon => 0,
            BodyId::Mars | BodyId::Phobos | BodyId::Deimos => 1,
        };
        match (system(from), system(to)) {
            (0, 1) => Some(self.window_offset(turn)),
            (1, 0) => Some(self.return_window_offset(turn)),
            _ => None,
        }
    }

    /// The turn the Mars window falls on, looked for from `from` forward over one synodic cycle:
    /// the turn whose window offset is smallest in magnitude.
    pub fn next_window_turn(&self, from: u32) -> u32 {
        let tr = &self.tables.transit;
        let cycle = (tr.synodic_days / tr.days_per_turn).ceil() as u32;
        let from = from.max(1);
        (from..=from + cycle)
            .min_by(|a, b| self.window_offset(*a).abs().partial_cmp(&self.window_offset(*b).abs()).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(from)
    }

    /// The tooltip the Solar System Map shows over Mars, Phobos or Deimos (ticket #57).
    pub fn window_text(&self, body: BodyId) -> String {
        let name = self.tables.body(body).name.clone();
        let window = self.next_window_turn(self.turn);
        let (now_turns, now_fuel) = self.transit_cost_at(BodyId::Earth, body, self.turn);
        let (win_turns, win_fuel) = self.transit_cost_at(BodyId::Earth, body, window);
        let when = match window.saturating_sub(self.turn) {
            0 => "this turn".to_string(),
            1 => "next turn".to_string(),
            n => format!("in {n} turns"),
        };
        format!(
            "{} window: {} ({}). Flight now: {} turns, {} Fuel. At the window: {} turns, {} Fuel.",
            name,
            when,
            self.date(window).text(),
            now_turns,
            now_fuel,
            win_turns,
            win_fuel
        )
    }

    // ---------------------------------------------------------------- Ticket #54: the Scrubber

    /// How many Scrubbers one Nation State may hold: clamp(round(population / 2), 2, 10).
    pub fn scrubber_cap(&self, s: StateId) -> u32 {
        let c = &self.tables.scrubber;
        if c.per_population <= 0.0 {
            return c.max;
        }
        ((self.state(s).population / c.per_population).round().max(0.0) as u32).clamp(c.min, c.max)
    }

    /// Scrubbers standing in a state, plus any on order there: what the cap is read against.
    pub fn scrubbers_committed(&self, s: StateId) -> u32 {
        let st = self.state(s);
        st.facilities.iter().filter(|f| f.kind == FacilityKind::Scrubber).count() as u32
            + st.queue.iter().filter(|b| b.item == BuildItem::Facility(FacilityKind::Scrubber)).count() as u32
    }

    /// Scrubbers standing and online in a state: the ones that enlarge the Sink and calm the place.
    pub fn scrubbers_online(&self, s: StateId) -> u32 {
        self.state(s).facilities.iter().filter(|f| f.kind == FacilityKind::Scrubber && f.working()).count() as u32
    }

    /// Ticket #54: the ppm each seat's Scrubbers take out of the air at this Climate phase. A
    /// Scrubber belongs to whoever controls its state, and counts as removal for that seat's Blame.
    pub fn scrubber_removal_by_seat(&self) -> [f64; SEAT_COUNT] {
        let per = self.tables.facility(FacilityKind::Scrubber).sink_per_turn;
        let mut out = [0.0; SEAT_COUNT];
        for st in &self.states {
            let Some(seat) = st.control.controller() else { continue };
            out[seat.index()] += per * self.scrubbers_online(st.id) as f64;
        }
        out
    }

    /// What the whole table takes back this Climate phase.
    pub fn scrubber_removal(&self) -> f64 {
        self.scrubber_removal_by_seat().iter().sum()
    }

    // ---------------------------------------------------------------- Ticket #54: the two coefficients

    /// A Nation State's Baseline Emissions now: its card figure plus whatever a spent Strip Permit
    /// added for good.
    pub fn baseline_emissions(&self, s: StateId) -> f64 {
        self.tables.state(s).baseline_emissions + self.state(s).baseline_rise
    }

    /// Ticket #54: what one hundred million people in this state emit a turn, before Green Consensus
    /// and the Faction multiplier: `base + per_level x Industry Level`, less what Leapfrog has taken
    /// off, never below `base`.
    pub fn population_coefficient(&self, s: StateId) -> f64 {
        let c = &self.tables.climate;
        let st = self.state(s);
        let raw = c.population_emissions_base + c.population_emissions_per_level * st.industry_level as f64;
        (raw - st.leapfrog).max(c.population_emissions_base)
    }

    /// How many times Leapfrog has been bought here, for the card.
    pub fn leapfrogs(&self, s: StateId) -> u32 {
        let per = self.tables.climate.population_emissions_per_level;
        if per <= 0.0 { 0 } else { (self.state(s).leapfrog / per).round().max(0.0) as u32 }
    }

    /// Ticket #54: whether a Leapfrog would lower this state's coefficient at all. At the base it
    /// would buy nothing, so the order is refused rather than taking 50 Ducats for nothing.
    pub fn leapfrog_would_bite(&self, s: StateId) -> bool {
        self.population_coefficient(s) > self.tables.climate.population_emissions_base + 1e-9
    }

    // ---------------------------------------------------------------- Ticket #54: the Strip Permit

    /// True while a Strip Permit is doubling this state's Facility output.
    pub fn strip_permit_running(&self, s: StateId) -> bool {
        self.state(s).strip_permit_ends.map(|e| self.turn <= e).unwrap_or(false)
    }

    /// How many more turns of Income the doubling has after this one.
    pub fn strip_permit_turns_left(&self, s: StateId) -> u32 {
        self.state(s).strip_permit_ends.map(|e| e.saturating_sub(self.turn)).unwrap_or(0)
    }

    /// Cheap Industry, the Prospectors' signature rule (spec 14.2).
    pub fn industry_cost(&self, seat: Seat) -> i64 {
        match self.kind(seat) {
            FactionKind::Prospectors => self.tables.industry_level.materials_cheap_industry,
            _ => self.tables.industry_level.materials,
        }
    }

    // ---------------------------------------------------------------- Ticket #52: Unrest

    /// The four green Techs of rule 4.
    pub const GREEN_TECHS: [TechId; 4] = [TechId::CleanPropellant, TechId::CleanPower, TechId::CleanManufacturing, TechId::GreenConsensus];

    /// A state's Unrest now.
    pub fn unrest(&self, s: StateId) -> f64 {
        self.state(s).unrest
    }

    /// Ticket #53: Unrest moves in halves, so print the fraction only when there is one.
    pub fn unrest_figure(v: f64) -> String {
        if (v - v.round()).abs() < 1e-9 { format!("{}", v.round() as i64) } else { format!("{v:.1}") }
    }

    /// A state's Unrest as the card and the map print it.
    pub fn unrest_text(&self, s: StateId) -> String {
        Self::unrest_figure(self.state(s).unrest)
    }

    /// The ceiling on this state's Unrest: 10 where a Faction holds or occupies it, 9 while it is
    /// neutral, so a neutral state never throws anybody off (rule 5).
    pub fn unrest_cap(&self, s: StateId) -> f64 {
        let u = &self.tables.unrest;
        if self.state(s).control == Control::Neutral { u.neutral_max } else { u.max }
    }

    /// How many of the four green Techs are complete.
    pub fn green_techs_done(&self) -> usize {
        Self::GREEN_TECHS.into_iter().filter(|t| self.has_tech(*t)).count()
    }

    /// A Constabulary standing and online in the state.
    pub fn constabulary_online(&self, s: StateId) -> bool {
        self.state(s).facilities.iter().any(|f| f.kind == FacilityKind::Constabulary && f.working())
    }

    /// How much smaller a rise from `source` is here (rule 4 of #52, widened on #53 so the green
    /// Techs moderate arriving refugees as well as the climate). Never below zero; never a fall.
    pub fn unrest_damping(&self, s: StateId, source: UnrestSource) -> f64 {
        let u = &self.tables.unrest;
        if source == UnrestSource::Plain {
            return 0.0;
        }
        let mut d = 0.0;
        let done = self.green_techs_done();
        if done >= 4 {
            d += u.green_techs_four;
        } else if done >= 2 {
            d += u.green_techs_two;
        }
        if self.constabulary_online(s) {
            d += u.constabulary_damping;
        }
        d
    }

    /// Raise a state's Unrest, damped by rule 4 and capped by `unrest_cap`. Returns what it rose by.
    /// Damping never lowers Unrest by itself: a rise damped away is no rise at all.
    pub fn raise_unrest(&mut self, s: StateId, amount: f64, source: UnrestSource) -> f64 {
        if amount <= 0.0 {
            return 0.0;
        }
        let damped = (amount - self.unrest_damping(s, source)).max(0.0);
        if damped <= 0.0 {
            return 0.0;
        }
        let cap = self.unrest_cap(s);
        let before = self.state(s).unrest;
        let after = (before + damped).clamp(0.0, cap.max(before));
        self.state_mut(s).unrest = after;
        after - before
    }

    /// Lower a state's Unrest, never below zero. Returns what it fell by.
    pub fn lower_unrest(&mut self, s: StateId, amount: f64) -> f64 {
        if amount <= 0.0 {
            return 0.0;
        }
        let before = self.state(s).unrest;
        let after = (before - amount).max(0.0);
        self.state_mut(s).unrest = after;
        before - after
    }

    /// Ticket #52: a Facility mothballed in this state; called by the Mothball order of ticket #54
    /// when it lands. A Module mothballed in a Colony raises nothing.
    pub fn unrest_from_mothball(&mut self, s: StateId) -> f64 {
        let n = self.tables.unrest.per_mothball;
        self.raise_unrest(s, n, UnrestSource::Plain)
    }

    /// Ticket #52: a Facility decommissioned in this state, the same way (ticket #54).
    pub fn unrest_from_decommission(&mut self, s: StateId) -> f64 {
        let n = self.tables.unrest.per_decommission;
        self.raise_unrest(s, n, UnrestSource::Plain)
    }

    /// The hook of rule 3 for what lowers Unrest by standing in the state: a Constabulary, and
    /// since ticket #54 a Scrubber too (`unrest.toml`, `scrubber_fall`). A state with both gets both.
    pub fn calming_fall(&self, s: StateId) -> f64 {
        let u = &self.tables.unrest;
        let mut n = 0.0;
        if self.constabulary_online(s) {
            n += u.constabulary_fall;
        }
        if self.scrubbers_online(s) > 0 {
            n += u.scrubber_fall;
        }
        n
    }

    /// Ticket #52, a hook for the neutral-development rule of a later ticket: a state at
    /// `no_development_at` or more does not develop itself.
    pub fn may_develop(&self, s: StateId) -> bool {
        self.state(s).unrest < self.tables.unrest.no_development_at
    }

    /// The Standing Army replenishes only below the first threshold (rule 5).
    pub fn army_replenishes(&self, s: StateId) -> bool {
        self.state(s).unrest < self.tables.unrest.army_threshold
    }

    /// At the second threshold every Facility in the state produces and emits at half (rule 5).
    pub fn facilities_at_half(&self, s: StateId) -> bool {
        self.state(s).unrest >= self.tables.unrest.facility_threshold
    }

    /// What the state's Unrest does now, in words, for the card and the map.
    pub fn unrest_note(&self, s: StateId) -> String {
        let u = &self.tables.unrest;
        let n = self.state(s).unrest;
        let held = self.state(s).control != Control::Neutral;
        if n >= u.throw_off_threshold {
            "it throws its controller off at this Resolution".to_string()
        } else if held && n >= u.throw_off_threshold - 1.0 {
            "Facilities at half and no replenishment; one more and it throws you off".to_string()
        } else if n >= u.facility_threshold {
            "every Facility here produces and emits at half, and the Standing Army does not replenish".to_string()
        } else if n >= u.army_threshold {
            "the Standing Army no longer replenishes".to_string()
        } else {
            "calm enough: the Standing Army replenishes and Facilities run in full".to_string()
        }
    }

    pub fn is_over(&self) -> bool {
        self.outcome.is_some()
    }

    pub fn log(&mut self, line: impl Into<String>) {
        self.log.push(line.into());
    }

    // ------------------------------------------------ Ticket #58: the dispatch

    /// One sentence from `report.toml`'s `[line]` table.
    pub fn say(&self, key: &str, args: &[(&str, String)]) -> String {
        self.tables.report.line(key, args)
    }

    /// One fragment from `report.toml`'s `[phrase]` table.
    pub fn phrase(&self, key: &str, args: &[(&str, String)]) -> String {
        self.tables.report.phrase(key, args)
    }

    /// Add one line to the dispatch, with the kind that places it in the severity order and under
    /// its heading, and the place it takes the player to when it is clicked.
    pub fn report_line(&mut self, kind: LineKind, place: Option<ReportPlace>, text: String) {
        self.report.lines.push(ReportLine { kind, place, text });
    }

    /// The same, for a line that belongs to the player when seat 0 did it and to the board
    /// otherwise: the player's builds, lifts, repairs and funding go under "Your works".
    pub fn report_line_of(&mut self, seat: Seat, mine: LineKind, theirs: LineKind, place: Option<ReportPlace>, text: String) {
        let kind = crate::report::line_kind_of(seat, mine, theirs, self.spectator);
        self.report_line(kind, place, text);
    }

    /// Add a Moment the turn may stop for. The cap of two and the switches are applied when the
    /// Report is shown, so every Moment a turn earned is kept here.
    pub fn moment(&mut self, kind: MomentKind, args: &[(&str, String)], place: Option<ReportPlace>) {
        let Some(card) = self.tables.report.moment(kind) else { return };
        let (text, figure) = (crate::report::render(&card.text, args), crate::report::render(&card.figure, args));
        self.report.moments.push(Moment { kind, text, figure, place, tech: None, note: None });
    }

    /// A sentence about what one AI seat's turn came to, appended to that Faction's paragraph.
    pub fn ai_deed(&mut self, seat: Seat, key: &str, args: &[(&str, String)]) {
        if !self.seat(seat).ai {
            return;
        }
        let text = self.tables.report.rival(key, args);
        if let Some(entry) = self.report.ai_lines.iter_mut().find(|e| e.seat == seat) {
            entry.deeds.push(text);
        }
    }

    /// What one rival Faction did this turn, as one paragraph.
    pub fn rival_paragraph(&self, seat: Seat) -> Option<String> {
        let entry = self.report.ai_lines.iter().find(|e| e.seat == seat)?;
        if entry.deeds.is_empty() {
            return None;
        }
        let deeds = Game::and_list(&entry.deeds);
        Some(self.tables.report.rival("paragraph", &[("faction", self.seat_name(seat)), ("deeds", deeds)]))
    }

    /// Ticket #58, widened by #64: the paragraphs the Report ends on. A player's game tells what the
    /// three rivals did; a spectated game tells what all four Factions did, seat 0 included, and a
    /// Faction that gave no orders says so rather than dropping out of the list.
    pub fn faction_paragraphs(&self) -> Vec<(Seat, String)> {
        if !self.spectator {
            return Seat::ALL.into_iter().skip(1).filter_map(|s| self.rival_paragraph(s).map(|p| (s, p))).collect();
        }
        Seat::ALL
            .into_iter()
            .map(|s| {
                let text = self.rival_paragraph(s).unwrap_or_else(|| self.tables.report.rival("nothing", &[("faction", self.seat_name(s))]));
                (s, text)
            })
            .collect()
    }
}

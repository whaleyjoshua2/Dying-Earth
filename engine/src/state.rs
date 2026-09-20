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
    /// Ticket #269 (version 0.08.4): a rival's Agitate. A working Constabulary damps it by half, as
    /// the police would; the green Techs, which moderate the climate's rises, do not.
    Agitate,
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
    /// Ticket #257 (version 0.08.4): a Sea Wall's count of the Sea Level thresholds it has held
    /// back. The wall is not destroyed absorbing one any more; each rise it holds adds
    /// `sea_wall_upkeep_per_rise` Materials a turn to its keep. Zero on every other kind.
    #[serde(default)]
    pub rises_held: u32,
}

impl Facility {
    pub fn new(kind: FacilityKind) -> Facility {
        Facility { kind, online: true, offline_until_resolution: false, self_run: false, mothballed: false, change: None, coastal: false, rises_held: 0 }
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
    /// Ticket #270 (version 0.08.4): how many Armies have ever been raised from this Region, so a
    /// re-raised Standing Army takes the next number and no name is given twice.
    #[serde(default)]
    pub armies_raised: u32,
    /// Ticket #56: what stood in those slots when the sea took them, oldest first, so the state
    /// card can say what a lost slot cost.
    pub drowned: Vec<FacilityKind>,
    /// Sea-level thresholds already applied to this state, by index into the table.
    pub thresholds_fired: Vec<bool>,
    /// Extra Emissions charged next Climate phase by a Wildfire.
    pub wildfire_emissions_next: f64,
    /// Ticket #76 (version 0.05.5): a Drought landed here: its Facilities make half at the next Income.
    #[serde(default)]
    pub drought: bool,
    /// Ticket #257 (version 0.08.4): a Storm Surge broke on this state's Sea Wall: the wall held, and
    /// the Facilities in its coastal slots make less at the next Income.
    #[serde(default)]
    pub storm_surge: bool,
    /// Ticket #73 (version 0.05.5): Emigrants waiting here, mustered and not yet lifted or sent.
    /// They are people of this state until they leave it: a new holder gets them.
    #[serde(default)]
    pub emigrants: u32,
    /// Ticket #189 (version 0.08.0): the mean Education Level of the Emigrants waiting here. They
    /// take this state's figure at the moment they MUSTER, so a second batch mustered after a School
    /// has run averages in higher, and the pile carries one number and a count.
    #[serde(default = "neutral_education")]
    pub emigrants_education: f64,
    /// Ticket #185 (version 0.08.0): what a School has added to this state's Education Level, above
    /// the figure on its card. It climbs a step a turn while a School stands and is online, to the
    /// ceiling, and falls back at the same rate when it stops -- so it never drops below the card.
    /// The Education Level was a fixed card figure until now and nothing in the game moved it.
    #[serde(default)]
    pub schooling: f64,
    /// Ticket #52: Unrest, 0 to 10 (9 while the state is neutral). Ticket #53: it moves in halves.
    pub unrest: f64,
    /// Ticket #53: the state changed hands this turn, which is the one turn its Unrest does not
    /// fall: a population with a fresh grievance is not calmed by the passing of a month.
    pub changed_hands: bool,
    /// Population that arrived here as refugees this turn; charged as Unrest once, at the end of
    /// Resolution, so the per-turn cap counts the whole turn's flows together.
    pub refugees_in: f64,
    /// Ticket #176 (version 0.07.6): what LEFT here this turn, kept by the cause that drove them
    /// out -- `the sea`, `the heat`, `the reefs`. Arrivals were counted per Region and departures
    /// were not, so the Report could only speak per flow: a Region that lost people to two causes
    /// spoke twice, and one that took ten and sent ten away spoke twice while netting nothing.
    /// The designer: *"reduce report clutter by reporting only net migration from refugees and only
    /// when migration occurs."* A net figure needs this counter beside `refugees_in`, and the cause
    /// is kept with it because the largest one survives into the line. Zeroed where `refugees_in`
    /// is, at the head of the Climate phase and again once Unrest has settled.
    #[serde(default)]
    pub refugees_out: Vec<(String, f64)>,
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
    /// Ticket #108 (version 0.07.0): what Leapfrog has taken off this state's Baseline Emissions,
    /// for good. The Baseline never falls below nothing.
    #[serde(default)]
    pub baseline_cut: f64,
    /// Ticket #54: a Strip Permit has been taken here; one per state, ever.
    pub strip_permit_used: bool,
    /// Ticket #54: the last turn whose Income this state's Facilities double, while one runs.
    pub strip_permit_ends: Option<u32>,
    /// Ticket #237 (version 0.08.3): an Exodus Call has been made here; one per state, ever, the
    /// shape ticket #54 gave the Strip Permit.
    #[serde(default)]
    pub exodus_call_used: bool,
    /// Ticket #237: the last turn a Call here doubles the muster and suspends its double cost.
    #[serde(default)]
    pub exodus_call_ends: Option<u32>,
    /// Ticket #238 (version 0.08.3): the turn this state's CURRENT holder took it, and who that is.
    /// Nothing recorded how long a Region had been held before this -- `changed_hands` is true for
    /// the single turn of a handover and `neutral_since` counts a run of neutrality -- so the
    /// three-turn rule needed a clock of its own.
    ///
    /// Both are maintained in `restart_neutrality_clock`, which runs after EVERY write to
    /// `control`: there are exactly two in the engine and the second calls it explicitly, so a new
    /// way of taking a Region cannot quietly forget to reset the clock.
    #[serde(default)]
    pub held_since: Option<u32>,
    #[serde(default)]
    pub held_by: Option<Seat>,
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
    /// Ticket #140 (version 0.07.3): Research, for an Observatory; the Habitat yield until then.
    pub research: f64,
}

impl SlotYields {
    /// The Body's own figures, which a station in orbit and any slot off the table read.
    pub fn of_body(card: &crate::data::BodyCard) -> SlotYields {
        SlotYields { mine: card.mine_yield, generator: card.generator_yield, refinery: card.refinery_yield, research: card.research_yield }
    }

    pub fn of_module(&self, kind: ModuleKind) -> f64 {
        match kind {
            ModuleKind::Mine => self.mine,
            ModuleKind::Generator => self.generator,
            ModuleKind::Refinery => self.refinery,
            // Ticket #140 (version 0.07.3): the fourth yield is the Observatory's. A Habitat holds
            // the same everywhere now, and a Trade Post (which followed the Habitat yield from
            // ticket #35 until the network of ticket #90 stopped reading it) reads nothing.
            ModuleKind::Observatory => self.research,
            _ => 1.0,
        }
    }

    /// "M 1.31 G 0.68 R 1.52 H 1.44", the figures the Surface Map writes under a slot's name.
    pub fn text(&self) -> String {
        format!("M {:.2} G {:.2} R {:.2} S {:.2}", self.mine, self.generator, self.refinery, self.research)
    }
}

/// Ticket #189 (version 0.08.0): what a pool of people with no recorded schooling counts as. Saves
/// are refused across versions, so this can never fire on a real save; it is the honest neutral.
fn neutral_education() -> f64 {
    1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Colony {
    pub id: ColonyId,
    pub body: BodyId,
    pub slot: u32,
    pub control: Control,
    pub modules: Vec<Module>,
    pub colonists: u32,
    /// Ticket #189 (version 0.08.0): this Colony's Education Level as it stands -- the weighted
    /// average of the people living here, raised by an Institute. Arriving settlers average into it
    /// by head count, Institute gains and all, so a shipload of poorly-schooled people dilutes what
    /// an Institute has built.
    #[serde(default = "neutral_education")]
    pub education: f64,
    /// Ticket #189: the same average with no Institute ever counted -- what this Colony's people
    /// know on their own. It is the floor an Institute's gains decay back to, so a Colony whose
    /// Institute goes dark returns to what its settlers brought rather than falling to nothing.
    #[serde(default = "neutral_education")]
    pub settler_education: f64,
    pub queue: Vec<Build>,
    /// Grid Failure: Modules offline until the next Resolution.
    pub grid_failed: bool,
    pub founded_turn: u32,
    /// Version 0.04 (ticket #46): a Space Station in an orbital slot rather than a Colony on the ground.
    pub in_orbit: bool,
}

/// Ticket #263 (version 0.08.4): a seat's builds begun and Ships in transit, soonest first --
/// `(what, where, turns until it lands)` and `(name, from, to, turns left)`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UnderWay {
    pub builds: Vec<(String, Place, u32)>,
    pub transits: Vec<(String, String, String, u32)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShipAt {
    Body(BodyId),
    Transit { from: BodyId, to: BodyId, turns_left: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ship {
    pub id: ShipId,
    /// Ticket #210 (version 0.08.1): the hull's own name, without its Faction's prefix -- `Magellan`,
    /// not `TSV Magellan`. The prefix belongs to whoever flies it and is read from the seat when the
    /// Ship is drawn, so a name travels with the hull and a prefix with its owner. Unique across the
    /// whole board: two Challengers on two sides in one game is a bug report waiting to happen.
    #[serde(default)]
    pub name: String,
    pub kind: UnitKind,
    pub seat: Seat,
    pub damage: u32,
    pub at: ShipAt,
    pub colonists: u32,
    /// Ticket #189 (version 0.08.0): the mean Education Level of the people aboard, carried from
    /// the Region they were mustered in.
    #[serde(default = "neutral_education")]
    pub colonists_education: f64,
    pub army: Option<ArmyId>,
    pub stance: Stance,
    pub escaped: bool,
    pub arrived_this_turn: bool,
    /// Turn this Ship was built, so an Army it carries can be told apart from one boarded later.
    pub built_turn: u32,
    /// Ticket #87 (version 0.06.0): the Fuel in its tank. Filled at the yard, spent by transits,
    /// refilled only by a Refuel order at a Body with a station of its own.
    #[serde(default)]
    pub fuel: i64,
    /// Ticket #99 (version 0.07.0): the Orbital Slot this Ship sits in, chosen with the leg that
    /// brought it. A warship in a slot blockades that slot; `None` is the Body at large, which
    /// blockades nothing. A Ship built at a Shipyard starts at the Body at large.
    #[serde(default)]
    pub slot: Option<u32>,
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
    /// Ticket #270 (version 0.08.4): its name, given when it is raised -- an ordinal from its home,
    /// "the 2nd Chinese Army", "the Tycho Garrison" -- Standing Armies included, and kept through
    /// every change of hands, since an Army is its Region's. Empty on a save from before this
    /// version, when the old form is read instead.
    #[serde(default)]
    pub name: String,
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

/// Ticket #153 (version 0.07.4): one turn of the **Emissions history** -- the breakdown the Climate
/// phase settled, the Stock and Temperature it left, and any Breaks that fired in it. One record
/// per Climate phase, kept for the whole game and saved with it, so the top bar's hover and the
/// Climate Panel can draw the world's Emissions turn by turn.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmissionsRecord {
    pub turn: u32,
    pub breakdown: EmissionsBreakdown,
    pub co2: f64,
    pub temperature: f64,
    /// Indices into `climate.toml`'s Breaks of those that fired this phase.
    pub breaks: Vec<usize>,
    /// Ticket #166 (version 0.07.5): Earth's people and space's, as the top bar counts them, read
    /// AFTER the phase has settled the heat's losses and moved the Refugees. Emigrants waiting on a
    /// card and Colonists aboard a Ship are in neither, exactly as they are in neither figure on the
    /// bar. `#[serde(default)]` so a save written earlier in this version still loads.
    #[serde(default)]
    pub earth_population: f64,
    #[serde(default)]
    pub space_population: u32,
}

/// Ticket #264 (version 0.08.4): one Faction's standing at the end of one Climate phase -- how far
/// along its Victory Condition it is (the score, the lower of its two parts' fractions) and its
/// share of the table's Blame -- with the three things the chart ticks on its axis: Antarctica
/// open (the world's), the Faction's gate Tech done, the Archive complete. One record per seat per
/// Climate phase, written beside the Emissions record so the two charts share an axis, saved with
/// the game; a save from before this version loads with an empty history and the chart grows from
/// there. The first per-Faction history the game keeps.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VictoryRecord {
    pub turn: u32,
    pub score: f64,
    pub blame_share: f64,
    pub gate_done: bool,
    pub archive_complete: bool,
    pub antarctica_open: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Climate {
    pub co2: f64,
    pub temperature: f64,
    pub last: EmissionsBreakdown,
    /// Ticket #153 (version 0.07.4): every Climate phase so far, oldest first. A save from before
    /// this version loads with an empty history and the graph grows from there.
    #[serde(default)]
    pub history: Vec<EmissionsRecord>,
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
    /// Research produced while no Tech was chosen; flows into the next one. Ticket #105 (version
    /// 0.07.0): PER SEAT, so Research banked between Techs still counts toward the Research Lead
    /// when it lands. Before this it was one pool and arrived unattributed, which produced Lead
    /// lines reading "the Prospectors led (Archivists 0, Custodians 0, Prospectors 0, Arkwrights 0)"
    /// -- arithmetically right and unreadable as anything but a bug.
    #[serde(default)]
    pub unallocated: [i64; SEAT_COUNT],
    /// Ticket #105: Research nobody produced (a Breakthrough card), waiting for a Tech to pour into.
    /// It is genuinely nobody's and counts toward no seat's Lead.
    #[serde(default)]
    pub unattributed: i64,
    /// Who must pick the next Tech, when a human has to.
    pub awaiting_pick: Option<Seat>,
    /// Ticket #98 (version 0.07.0): the Techs the Research Lead may choose between. Drawn when a
    /// Tech completes, at `shortlist_size` from what is available, always carrying the Lead's own
    /// Victory gate once its prerequisites are met. EMPTY means a free choice of everything
    /// available, which is how the game opens: the first Tech of the game is picked from the whole
    /// of rung 1.
    #[serde(default)]
    pub shortlist: Vec<TechId>,
    pub last_lead: Option<Seat>,
    /// Ticket #50: the turn each seat last picked a Tech, so a tie in contributions goes to the
    /// seat that has picked least recently. None means it has never picked, which counts as longest ago.
    pub last_picked_turn: [Option<u32>; SEAT_COUNT],
    /// Ticket #69 (version 0.05.5): every point the Labs of neutral and Occupied states have paid
    /// into the shared Tech over the game, for the simulation's report.
    #[serde(default)]
    pub neutral_total: i64,
    /// Ticket #173 (version 0.07.6): whether `current` has been **committed**. A human Lead's pick
    /// is provisional until the turn ends -- the designer: *"tech choice is not locked in until the
    /// turn is ended"* -- so the pick is recorded here at once but the banked Research is not poured
    /// into it, the shortlist is not thrown away, and the Tech cannot complete, until `commit_pick`
    /// runs at the head of `end_turn`. A computer seat's pick commits in the same breath. False with
    /// no Tech under research means nothing is owed.
    #[serde(default = "crate::state::yes")]
    pub pick_committed: bool,
    /// Ticket #173: which seat made the pick that is waiting to be committed.
    #[serde(default)]
    pub picked_by: Option<Seat>,
    /// Ticket #173: the Tech the Archivists' Provisional Findings reads for the whole of this turn,
    /// frozen at the turn's head. Their signature gives them half the effect of the Tech under
    /// research while the turn is still being ordered; with a pick that can change, reading `current`
    /// live would re-price orders already placed, so the turn reads this instead.
    #[serde(default)]
    pub findings_tech: Option<TechId>,
}

/// Serde's default for `pick_committed`: a save written before ticket #173 has no provisional pick
/// in it, so whatever Tech it carries is committed.
pub fn yes() -> bool {
    true
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
    /// Ticket #259 (version 0.08.4): whether the off-Earth cards have been shuffled in yet. A save
    /// from before this version loads with them never joined -- and never dealt, so a game already
    /// past the joining turn gets them on its next Event phase.
    #[serde(default)]
    pub off_earth_joined: bool,
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

/// Ticket #73 (version 0.05.5): Emigrants on the sea to Antarctica, landing on `due_turn`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntarcticSend {
    pub seat: Seat,
    pub from: StateId,
    pub n: u32,
    /// Ticket #189 (version 0.08.0): what the people aboard know, carried across the sea with them.
    #[serde(default = "neutral_education")]
    pub education: f64,
    pub into: crate::orders::UnloadTarget,
    pub due_turn: u32,
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
    /// Ticket #257 (version 0.08.4): Materials the seat's Sea Walls are owed in keep and have not yet
    /// paid. Half a Material a turn per rise held is not a whole number, so the fraction is carried
    /// here and the whole Materials are paid as they accrue; nothing is lost to rounding.
    #[serde(default)]
    pub sea_wall_upkeep_owed: f64,
    /// Ticket #261 (version 0.08.4): which steps of the rival's Moment this seat has fired -- three
    /// quarters of the way, and one part met. Once each, so a seat that dips and recrosses is not
    /// announced twice.
    #[serde(default)]
    pub rival_steps_announced: [bool; 2],
    /// Ticket #264 (version 0.08.4): this seat's Victory history, one record per Climate phase.
    #[serde(default)]
    pub victory_history: Vec<VictoryRecord>,
    /// Ticket #265 (version 0.08.4): the ppm a turn this seat's Research Directive has added to the
    /// Natural Sink, for good, over the whole game -- the Custodians' alone. Counted as removal at
    /// every Climate phase, at the designer's word ("credit"), since that enlargement takes that
    /// much CO2 out of the air every turn it stands.
    #[serde(default)]
    pub directive_sink: f64,
    /// Ticket #267 (version 0.08.4): ppm laid on this seat's Blame ledger by rivals' Smear
    /// campaigns, for good. Counted in `blame` and shown as its own figure, so the panel never
    /// says the seat put it in the air.
    #[serde(default)]
    pub blame_smeared: f64,
    /// Ticket #268 (version 0.08.4): ppm of carbon credit this seat has bought over the game, which
    /// comes off its Blame ledger; ppm it has sold, which comes off its credit and, past what it
    /// held, goes onto its ledger as Blame taken; and the ppm it offers a turn, standing until
    /// changed -- the Custodians' alone.
    #[serde(default)]
    pub credits_bought: f64,
    #[serde(default)]
    pub credits_sold: f64,
    #[serde(default)]
    pub credits_offered: i64,
    /// Ticket #272 (version 0.08.4): Agitates this seat has landed over the game, for the sweep.
    #[serde(default)]
    pub agitates_issued: u32,
    /// Ticket #227 (version 0.08.2): units this seat has bought and sold through the Trading window
    /// over the whole game. Kept because floating prices are only fair if more than one hand is on
    /// them, and the sweep had no way to say whose were.
    pub bought_units: i64,
    pub sold_units: i64,
    /// Ticket #183 (version 0.08.0): Influence the seat's Spaceports earned lifting Emigrants off
    /// Earth this turn, waiting to be paid into NEXT turn's Allotment. Read and cleared at Income.
    #[serde(default)]
    pub spaceport_influence: i64,
    /// Ticket #192 (version 0.08.0): Colonists uploaded into the Archive, all told. The Archivists'
    /// second Victory part counts this rather than who happens to be living beside the Module, and
    /// it only ever climbs: an uploaded Colonist cannot be lost to a raid, a crowding death or a
    /// handover.
    #[serde(default)]
    pub uploaded: u32,
    pub stabilization_run: u32,
    pub influence: BTreeMap<Target, i64>,
    /// Targets that received Influence this turn (spent or gained by Occupation), so they do not decay.
    pub influenced_this_turn: Vec<Target>,
    pub allotment: i64,
    pub research_last_turn: i64,
    /// Ticket #50: all the Research this seat's own Labs have produced, counted at production.
    pub research_total: i64,
    /// Ticket #80 (version 0.06.0): the part of it made by Observatories at Colonies and stations
    /// not at Earth (Antarctica and a station over Earth are on Earth), for the measurement.
    #[serde(default)]
    pub research_off_earth_total: i64,
    /// Ticket #82 (version 0.06.0): Module-turns doubled by a mothballed Facility on Earth over the
    /// game (the Custodians' signature), for the measurement.
    #[serde(default)]
    pub doubled_module_turns: i64,
    /// Ticket #86 (version 0.06.0): Colonists this seat lost in transit to crowding over the game.
    #[serde(default)]
    pub lost_in_transit: i64,
    pub income_last_turn: Stockpile,
    /// Last Income by source (ticket #31): "Factory in Asia", the resource, the amount; upkeep as negatives.
    pub income_sources: Vec<(String, Resource, i64)>,
    /// Ticket #51: Research banked for the Archive, capped at what its remaining stages still need.
    pub archive_fund: i64,
    /// Ticket #51: Fund the Archive was ordered this turn, so this turn's Lab Research went to the
    /// fund and contributed nothing to the Research Lead. Version 0.07.0: set at Income by
    /// `bank_archive_research`, and true only when a point was actually banked.
    pub funding_archive: bool,
    /// Ticket #235 (version 0.08.3): the **Research Directive** -- the share of this seat's
    /// Research, as a percentage, that goes somewhere other than the shared Tech. It replaces the
    /// Archivists' `archive_funding` bool, which was the same idea with two positions: their
    /// directive runs to 100 (all of it to the Archive) where every other Faction's stops at 50.
    ///
    /// Set by an order, read at the NEXT Income, and it holds until it is set again -- the shape
    /// version 0.07.0 gave the Archivists' switch, kept because a per-turn order would mean
    /// re-deciding this thirty-six times a game.
    #[serde(default)]
    pub research_directive: u8,
    /// Ticket #235: the directive that was in force at the LAST Income, which is what Provisional
    /// Findings is settled from. It is kept apart from `research_directive` for the lag: the rule
    /// reads what happened to *last* turn's Research, so a directive set this turn is paid at this
    /// Income and felt at the next one -- the shape version 0.07.0 gave the Archivists' switch.
    ///
    /// It holds what was DECLARED rather than what landed. Rounding means a 26% directive on 14
    /// Research takes 3 points, which is 21% of them, and a player who set 26 would otherwise keep
    /// a rule they had chosen to trade away with nothing on screen to explain it.
    #[serde(default)]
    pub directive_last_income: u8,
    /// Ticket #235: the fraction of a Ducat or a Fuel a conversion has earned and not yet paid.
    /// The Prospectors' rate is 0.8 a point and the Arkwrights' 0.2, so flooring every turn would
    /// quietly lose up to a fifth of what was diverted. It is carried instead.
    #[serde(default)]
    pub directive_remainder: f64,
    /// Ticket #114 (version 0.07.1): the Defence split is set to repeat. It is a STANDING ORDER and
    /// not an automatic spend: the interface places this turn's split as ordinary pending orders
    /// every turn while it is on, so the player sees exactly what it did and can cancel any of it
    /// before ending the turn. The flag lives here rather than in the interface so it survives a
    /// save, the way the Venture share and the Archive's funding do.
    #[serde(default)]
    pub max_standing: Option<Target>,
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


/// Ticket #191 (version 0.08.0): Relations. Every Faction keeps a score for every other -- ONE PER
/// ORDERED PAIR, so twelve in a four-seat game, and the Arkwrights' view of the Prospectors is a
/// different number from the Prospectors' view of the Arkwrights. Measured over 80 games, of the 317
/// pairs where anything happened 49% were purely one-directional and 80% of offending pair-turns had
/// only one direction firing, so a shared number would have thrown all of that away.
///
/// The score falls for each OFFENDING TURN -- a turn in which the offender spent any Influence on a
/// place the victim holds, or opened a Battle against them -- charged per TURN and never per order.
/// It recovers slowly while a pair is quiet and stops at neutral: it never rises above it.
///
/// **In version 0.08.0 the score does nothing mechanical.** It is read, not spent: no rule reads it
/// and the computer players do not read it. It is built now so that it can be watched for a version
/// and given teeth in 0.09 with evidence rather than a guess.
/// Ticket #226 (version 0.08.2): one Term of an Accord. Tribute is not here: it is a one-turn order
/// that pays and is done, where these hold until the Accord ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Term {
    /// Neither spends Influence on a place the other holds, nor opens a Battle against them. NOT
    /// enforced by refusing the order: the act is allowed and costs +3, and ends the whole Accord.
    NonAggression,
    /// Neither treats the other's Ships as a target, and a Blockade does not shut the other out of
    /// the slot. Measured near-dead on today's board -- the Blockade rung fired zero times in 80
    /// games -- and kept deliberately, because it will matter for Mars.
    Passage,
    /// Either may Refuel at the other's Space Stations. Also near-dead today: off-Earth stations run
    /// at 0-1 a game and three Factions each hold their own station over Earth.
    Refuel,
    /// Both parties' Research rises a tenth while it stands. Needs Friendly to STRIKE, and the gate
    /// is checked only at that moment: a pair that worked nine or more acts to reach Friendly should
    /// not lose it because a rival's emissions ticked up and moved a Blame step.
    ResearchAgreement,
}

/// Ticket #226 (version 0.08.2): an Accord between two Factions, holding one or more Terms.
///
/// `diplomacy` is on Relations' own `_Avoid_` list in the glossary, so the system has a word of its
/// own. Ending one takes a turn's notice and is free; VIOLATING a term while it stands costs +3 and
/// ends the whole Accord -- without which a Faction could violate non-aggression every turn, pay 3
/// each time, and keep drawing a permanent tenth of extra Research from a partner it was attacking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Accord {
    pub a: Seat,
    pub b: Seat,
    pub terms: Vec<Term>,
    /// The turn it was struck, which is what `accord_kept` counts from.
    pub struck: u32,
    /// The last turn its keeping was paid for, so a long Accord pays every `accord_kept_turns`.
    pub paid: u32,
    /// Ticket #226: declared over on this turn; it lapses at the next turn's start and costs
    /// nothing. That also gives the board a visible tell -- an Accord announced as ending is a turn
    /// of warning that something is coming.
    pub ending: bool,
}

impl Accord {
    pub fn holds(&self, x: Seat, y: Seat) -> bool {
        !self.ending && ((self.a == x && self.b == y) || (self.a == y && self.b == x))
    }
}

/// Ticket #220 (version 0.08.2): the Trading window's prices, and the net units moved this turn
/// that will shift them at the next settle.
///
/// Measured before it was built, and the measurement is why the rule ships in the shape it does:
/// over 80 games the computer seats placed **zero** Sell orders and bought **only Materials** --
/// 29,440 units, no Fuel, no Energy -- with the Prospectors alone 61% of all trading. So Materials
/// sees one-way upward pressure from the computer, and Fuel and Energy move only when a human
/// trades them. The designer chose to ship it anyway; giving the seats an appetite to sell, and to
/// buy the other two, is a separate ticket.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Market {
    /// The live price of Materials, Fuel and Energy, in that order. Zero means "not opened yet" and
    /// is read as the card figure, so a fresh game and an old save behave alike.
    pub price: [i64; 3],
    /// Net units of each bought less sold this turn, across the whole table.
    pub net: [i64; 3],
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Relations {
    /// `score[viewer][subject]`: the DEEDS half of what the seat at `viewer` thinks of the seat at
    /// `subject` -- everything the pair has done to each other. Since ticket #221 this is no longer
    /// the whole score: what is shown is this plus a Blame term read afresh each settle, and ONLY
    /// the shown figure clamps to the scale. This one may run past it, to `deeds_ceiling`, which is
    /// what lets a pair carrying a heavy Blame penalty still climb to Friendly on deeds alone.
    pub score: [[i64; SEAT_COUNT]; SEAT_COUNT],
    /// Consecutive turns `subject` has not offended `viewer`.
    pub quiet: [[u32; SEAT_COUNT]; SEAT_COUNT],
    /// Ticket #222 (version 0.08.2): what `subject` has cost `viewer` this turn, summed over every
    /// instance -- a place, an act, never an order -- and capped when the turn settles. It replaced
    /// a bare flag: an offence now has a weight, so a Battle is not a bid.
    pub owed: [[i64; SEAT_COUNT]; SEAT_COUNT],
    /// `offended[viewer][subject]`: whether `subject` offended `viewer` at all this turn. Kept
    /// beside `owed` because the scar counts TURNS, not points.
    pub offended: [[bool; SEAT_COUNT]; SEAT_COUNT],
    /// Ticket #223 (version 0.08.2): whether `subject` did `viewer` a kindness this turn -- a
    /// tribute paid, or an Accord kept. One act a turn per ordered pair, whatever was done.
    pub credited: [[bool; SEAT_COUNT]; SEAT_COUNT],
    /// Ticket #225 (version 0.08.2): how many turns `subject` has offended `viewer` across the whole
    /// game, which is what ratchets the floor. Turns and not points, so retuning the ladder later
    /// does not silently move every scar with it.
    pub offending_turns: [[u32; SEAT_COUNT]; SEAT_COUNT],
    /// Ticket #225: the best this pair can recover to, lowered a step every `scar_turns` offending
    /// turns and never below `scar_floor`. It caps the DEEDS figure, not the shown score, so a
    /// Faction that cleans up its emissions does not find its forgiveness eaten by an old grudge.
    pub floor: [[i64; SEAT_COUNT]; SEAT_COUNT],
    /// `fell[viewer][subject]`: whether the score moved down this turn, for the Report line.
    pub fell: [[bool; SEAT_COUNT]; SEAT_COUNT],
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
    /// Ticket #272 (version 0.08.4): cards drawn with nowhere to land over the game, for the sweep.
    pub events_no_target: u32,
    pub discoveries: Vec<Discovery>,
    /// Ticket #73: Emigrants on the sea to Antarctica.
    pub antarctic_sends: Vec<AntarcticSend>,
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
    /// Ticket #191 (version 0.08.0): what every Faction thinks of every other.
    pub relations: Relations,
    /// Ticket #220 (version 0.08.2): the Trading window's prices, which move with what the table
    /// bought and sold. `Game` is not itself a serde type -- the save is assembled by hand in
    /// `save.rs` -- so this carries no attribute; a zero price there reads as the card figure, which
    /// is how a save written before this version opens at the right place.
    pub market: Market,
    /// Ticket #226 (version 0.08.2): the Accords standing between pairs of Factions.
    pub accords: Vec<Accord>,
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
            sea_wall_upkeep_owed: 0.0,
            rival_steps_announced: [false; 2],
            victory_history: Vec::new(),
            directive_sink: 0.0,
            blame_smeared: 0.0,
            credits_bought: 0.0,
            credits_sold: 0.0,
            credits_offered: 0,
            agitates_issued: 0,
            bought_units: 0,
            sold_units: 0,
            spaceport_influence: 0,
            uploaded: 0,
            stabilization_run: 0,
            influence: BTreeMap::new(),
            influenced_this_turn: Vec::new(),
            allotment: 0,
            research_last_turn: 0,
            research_total: 0,
            research_off_earth_total: 0,
            doubled_module_turns: 0,
            lost_in_transit: 0,
            income_last_turn: Stockpile::default(),
            income_sources: Vec::new(),
            archive_fund: 0,
            funding_archive: false,
            research_directive: 0,
            directive_last_income: 0,
            directive_remainder: 0.0,
            max_standing: None,
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
                // Ticket #185 (version 0.08.0): no School has run yet, so the state reads its card.
                schooling: 0.0,
                // Ticket #189 (version 0.08.0): nobody is waiting, so the figure is the neutral one.
                emigrants_education: 1.0,
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
                armies_raised: 0,
                drowned: Vec::new(),
                thresholds_fired: vec![false; tables.climate.sea_level_thresholds.len()],
                wildfire_emissions_next: 0.0,
                drought: false,
                storm_surge: false,
                emigrants: 0,
                unrest: c.unrest,
                changed_hands: false,
                refugees_in: 0.0,
                refugees_out: Vec::new(),
                unrest_reported: c.unrest,
                // Every state is neutral when the game opens; the clocks are staggered below, and
                // `take_control` clears the four the Factions begin holding.
                neutral_since: Some(1),
                leapfrog: 0.0,
                baseline_rise: 0.0,
                baseline_cut: 0.0,
                strip_permit_used: false,
            exodus_call_used: false,
            held_since: None,
            held_by: None,
            exodus_call_ends: None,
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
                history: Vec::new(),
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
                unallocated: [0; SEAT_COUNT],
                unattributed: 0,
                awaiting_pick: Some(Seat(0)),
                // Ticket #98: empty at the opening, so the first Tech of the game is a free choice
                // from the whole of rung 1.
                shortlist: Vec::new(),
                last_lead: None,
                last_picked_turn: [None; SEAT_COUNT],
                neutral_total: 0,
                // Ticket #173: nothing picked yet, so nothing is waiting to be committed.
                pick_committed: true,
                picked_by: None,
                findings_tech: None,
            },
            deck,
            events_no_target: 0,
            discoveries: Vec::new(),
            antarctic_sends: Vec::new(),
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
            relations: Relations::default(),
            market: Market::default(),
            accords: Vec::new(),
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
            // Ticket #164 (version 0.07.5): the three starting stations stand with their Core Modules.
            game.colonies.push(Colony { id, body: BodyId::Earth, slot: slot as u32, control: Control::Controlled(seat), modules: vec![Module::new(ModuleKind::Core)], colonists: 0, education: 1.0, settler_education: 1.0, queue: Vec::new(), grid_failed: false, founded_turn: 1, in_orbit: true });
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
            // Ticket #181 (version 0.08.0): a Faction's start Region's Facilities come up as that
            // Faction's own versions, the Launch Site just added included. The consequences are
            // asymmetric and were accepted knowingly: every start Region is handed a Launch Site, so
            // the ARKWRIGHTS hold a Spaceport from turn 1; ten of the fourteen Regions start with a
            // Power Plant, so the ARCHIVISTS usually hold a Reactor; and neither the Bank nor the
            // School is in any Region's start Facilities, so the PROSPECTORS and the CUSTODIANS start
            // with nothing of theirs and must build for their clause.
            let faction = game.kind(seat);
            for f in game.state_mut(*sid).facilities.iter_mut() {
                f.kind = f.kind.built_by(faction);
            }
            // Ticket #75 (version 0.05.5): a claim on its home from turn 1. The seat's Standing on its
            // start state begins at the state's threshold, so a challenger needs the threshold plus
            // the margin at once and the holder's spending counts from a real footing; with nothing
            // there, a richer rival took seat 0's start state on turn 7 in every seed.
            let claim = game.influence_threshold(Place::State(*sid));
            game.seats[seat.index()].influence.insert(Place::State(*sid), claim);
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
    /// Ticket #173 (version 0.07.6): the Tech read here is the one **frozen at the turn's head**,
    /// not whatever is under research this instant. A Lead may change its pick until the turn ends,
    /// and a half-effect that moved with it would re-price orders already placed.
    fn reads_half(&self, seat: Seat, t: TechId) -> bool {
        !self.has_tech(t)
            && self.research.findings_tech == Some(t)
            && self.provisional_findings(seat)
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
        self.tech_addition_of(seat, t, self.tables.tech(t).value)
    }

    /// Ticket #207 (version 0.08.1): what a Tech adds to a HABITAT's capacity, which is not always
    /// what it adds elsewhere. Expanded Habitats is read both here and by `colony_ship_capacity`,
    /// and one `value` served both until the designer wanted the Habitat clause moved on its own:
    /// *"q5 b"* -- split the figure, so housing and transport can be tuned apart. A Tech with no
    /// `habitat_colonists` of its own answers with its `value`, as every other Tech in the tree does.
    pub fn habitat_addition(&self, seat: Seat, t: TechId) -> i64 {
        let card = self.tables.tech(t);
        self.tech_addition_of(seat, t, card.habitat_colonists.unwrap_or(card.value))
    }

    /// The shared body: the whole figure once the Tech stands, half of it while the Archivists are
    /// reading it early through Provisional Findings, nothing otherwise.
    fn tech_addition_of(&self, seat: Seat, t: TechId, value: f64) -> i64 {
        let v = value as i64;
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

    /// Ticket #192 (version 0.08.0): how many Colonists this seat may still upload at `colony` this
    /// turn -- everyone living there, less whatever is already ordered. The Archive may only draw
    /// from the population of the place it stands at.
    pub fn uploadable_at(&self, colony: ColonyId, already: u32) -> u32 {
        self.colony(colony).map(|c| c.colonists).unwrap_or(0).saturating_sub(already)
    }

    /// A Colony off Earth may hold the Archive; Antarctica may not. Ticket #81: a station over
    /// Earth is off Earth, so it may.
    /// Ticket #209 (version 0.08.1): **not in Earth orbit**, which until now was the only place it
    /// had ever stood. `off_earth` counts a station over Earth as off Earth, and ticket #192
    /// measured the Archive ordered on turn 1 at such a station, with nobody living on it, in 80 of
    /// 80 games -- always Axiom. So this does not narrow a choice the Archivists were making; it
    /// moves the Archive to a place no seat has been, which is the point and also the risk.
    ///
    /// What is left is every Body but Earth, its satellites included: the Moon counts, at the
    /// designer's word. Barring the satellites too was considered and refused with numbers -- in 80
    /// measured games there are **0 Mars-system Colonies, 0 Venus stations and 0 Colonies on Phobos
    /// or Deimos** -- so it would have been a rule the computer could never satisfy at all. Reduces
    /// to `body != Earth`, since Antarctica was already barred for being ON Earth.
    /// Ticket #210 (version 0.08.1): the name a Ship of this kind would be built with -- the first
    /// name in its list that no Ship on the board is already using. In LIST ORDER, deliberately: ids
    /// are sequential and deterministic, so this draws no randomness at all, where a random pick
    /// would shift every later roll in a seeded game and make a sweep incomparable with its baseline.
    /// A Colony Ship draws from the colony list; a Frigate, a Battleship and a Carrier from the
    /// warship list, the Carrier included at the designer's word because it sails with a fleet.
    /// Once a list is exhausted it begins again with a numeral -- Magellan, Magellan II, Magellan III
    /// -- which a thirty-six-turn game will never reach and an eighty-game sweep might.
    pub fn next_ship_name(&self, kind: UnitKind) -> String {
        let list = if kind == UnitKind::ColonyShip { &self.tables.ship_names.colony.names } else { &self.tables.ship_names.warship.names };
        if list.is_empty() {
            return String::new();
        }
        let taken: std::collections::HashSet<&str> = self.ships.iter().map(|s| s.name.as_str()).collect();
        for pass in 0..1000u32 {
            for base in list {
                let name = if pass == 0 { base.clone() } else { format!("{base} {}", Self::numeral(pass + 1)) };
                if !taken.contains(name.as_str()) {
                    return name;
                }
            }
        }
        list[0].clone()
    }

    /// A small Roman numeral for a reused name. Beyond what any game reaches it falls back to the
    /// figure itself, which is ugly and unreachable rather than wrong.
    fn numeral(n: u32) -> String {
        const ROMAN: [&str; 19] = ["II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI", "XII", "XIII", "XIV", "XV", "XVI", "XVII", "XVIII", "XIX", "XX"];
        ROMAN.get(n as usize - 2).map(|r| r.to_string()).unwrap_or_else(|| n.to_string())
    }

    /// Ticket #210 (version 0.08.1): what a Ship is called on screen -- its Faction's prefix and its
    /// own name, `TSV Magellan`. A Ship built before this version, or one whose list was empty, has
    /// no name and falls back to the kind and id it always had.
    pub fn ship_name(&self, s: &Ship) -> String {
        if s.name.is_empty() {
            return format!("{} {}", s.kind.name(), s.id.0);
        }
        format!("{} {}", self.tables.faction(self.kind(s.seat)).ship_prefix, s.name)
    }

    pub fn may_hold_archive(&self, c: &Colony) -> bool {
        c.body != BodyId::Earth
    }

    /// Ticket #209 (version 0.08.1): the Archivists have nowhere to put the Archive and have not
    /// begun it. Their whole Victory Condition waits on a journey they have never made -- their AI
    /// already weights founding a Colony at 9 and a Colony Ship at 8, yet Antarctica and Earth orbit
    /// are so much cheaper than a transit that nothing further ever won the comparison. While this
    /// holds, the chain that carries them off Earth is worth more to them than its standing weight
    /// says, and `archive_needs_a_place` in `ai.toml` is how much more.
    pub fn archive_is_homeless(&self, seat: Seat) -> bool {
        self.kind(seat) == FactionKind::Archivists
            && !self.archive_built(seat)
            && !self.archive_ordered(seat)
            && !self.colonies.iter().any(|c| c.control.director() == Some(seat) && self.may_hold_archive(c))
    }

    // ---------------------------------------------------------------- Ticket #51: Coach Class and the rest

    /// Ticket #73: how many Emigrants this seat may muster in a turn (Coach Class doubles it).
    pub fn emigrants_per_turn(&self, seat: Seat) -> u32 {
        (self.tables.emigrants.per_turn as f64 * self.tables.faction(self.kind(seat)).emigrants_multiplier).floor() as u32
    }

    /// Ticket #237 (version 0.08.3): is an Exodus Call running here this turn?
    pub fn exodus_call_running(&self, s: StateId) -> bool {
        self.state(s).exodus_call_ends.is_some_and(|last| self.turn <= last)
    }

    /// Ticket #237: what this seat may recruit in THIS state this turn. A Call doubles it -- the
    /// Arkwrights' eight becomes sixteen, doubling the figure they actually use rather than the
    /// base nobody else's Faction changes.
    pub fn emigrants_per_turn_in(&self, seat: Seat, s: StateId) -> u32 {
        let per = self.emigrants_per_turn(seat);
        if self.exodus_call_running(s) { per * self.tables.exodus_call.muster_multiplier } else { per }
    }

    /// Ticket #237: the population a muster takes HERE. A Call suspends Coach Class's double
    /// charge for as long as it runs, which is the whole point of it.
    ///
    /// Measured before it was decided: the Arkwrights' home state runs 20 units to 1 over a game
    /// as it is, because Coach Class charges them twice a head, so an order that only doubled the
    /// COUNT would have burned the country twice as fast and deepened the thing that already caps
    /// them at 3 wins of 80. The designer took this reading -- the Call moves people without
    /// eating the source faster than anyone else does.
    pub fn muster_population_in(&self, seat: Seat, s: StateId, colonists: u32) -> f64 {
        if self.exodus_call_running(s) {
            self.tables.emigrants.population_each * colonists as f64
        } else {
            self.lift_population(seat, colonists)
        }
    }

    /// What one Colony Ship of this seat carries: the card figure, +2 with Expanded Habitats
    /// (version 0.04 section 4), times the Faction's own multiplier (Coach Class doubles it).
    pub fn colony_ship_capacity(&self, seat: Seat) -> u32 {
        // Ticket #84: Generation Ships stacks on Expanded Habitats.
        let base = self.tables.unit(UnitKind::ColonyShip).carries_colonists as i64 + self.tech_addition(seat, TechId::ExpandedHabitats) + self.tech_addition(seat, TechId::GenerationShips);
        let m = self.tables.faction(self.kind(seat)).colony_ship_capacity_multiplier;
        (base.max(0) as f64 * m).floor().max(0.0) as u32
    }

    /// Ticket #89 (version 0.06.0): how much sunlight a Body gets against Earth's: the inverse
    /// square of its mean distance from the Sun (a satellite reads its parent's).
    pub fn sun_factor(&self, body: BodyId) -> f64 {
        let a = self.tables.planet(body).a;
        if a <= 0.0 { 1.0 } else { (1.0 / a).powi(2) }
    }

    /// Ticket #90 (version 0.06.0): the Bodies where the seat holds a Colony or a Space Station,
    /// Earth counting for a station over it or any Nation State the seat directs.
    pub fn bodies_held(&self, seat: Seat) -> Vec<BodyId> {
        BodyId::ALL
            .into_iter()
            .filter(|b| {
                self.colonies.iter().any(|c| c.body == *b && c.control.director() == Some(seat))
                    || (*b == BodyId::Earth && !self.directed_states(seat).is_empty())
            })
            .collect()
    }

    /// Ticket #90: whether the seat already holds a Trade Post, standing or on order, at this Body.
    pub fn trade_post_at_body(&self, seat: Seat, body: BodyId) -> bool {
        self.colonies.iter().filter(|c| c.body == body && c.control.director() == Some(seat)).any(|c| {
            c.modules.iter().any(|m| m.kind == ModuleKind::TradePost) || c.queue.iter().any(|b| b.item == BuildItem::Module(ModuleKind::TradePost))
        })
    }

    /// Ticket #87 (version 0.06.0): whether the seat holds a Space Station over this Body, where
    /// its Ships may refuel.
    pub fn own_station_at(&self, seat: Seat, body: BodyId) -> bool {
        self.colonies.iter().any(|c| c.in_orbit && c.body == body && c.control.director() == Some(seat))
    }

    /// Ticket #87: what a Refuel order takes from the Stockpile: what the tank wants, as far as
    /// the Stockpile can pay.
    pub fn refuel_amount(&self, seat: Seat, ship: ShipId) -> i64 {
        let Some(s) = self.ship(ship) else { return 0 };
        let want = (self.tables.unit(s.kind).tank - s.fuel).max(0);
        want.min(self.seat(seat).stockpile.fuel.max(0))
    }

    /// Ticket #87: the cheapest leg a seat's Ship can fly from this Body today, in Fuel.
    pub fn cheapest_leg_from(&self, seat: Seat, body: BodyId) -> Option<i64> {
        BodyId::ALL.into_iter().filter(|b| *b != body).map(|b| self.transit_cost_for(seat, body, b).1).min()
    }

    /// Ticket #87: a Ship at a Body whose tank cannot pay any leg from there, with no station of
    /// its own to refuel at, is stranded until a station of its own stands in orbit there.
    pub fn stranded(&self, ship: ShipId) -> bool {
        let Some(s) = self.ship(ship) else { return false };
        let ShipAt::Body(body) = s.at else { return false };
        if self.own_station_at(s.seat, body) {
            return false;
        }
        match self.cheapest_leg_from(s.seat, body) {
            Some(cheapest) => s.fuel < cheapest,
            None => false,
        }
    }

    /// Ticket #86 (version 0.06.0): how many Colonists a Colony Ship at Earth may take beyond its
    /// capacity, from the Temperature: `per_step` for every full `step` degrees above `above`, at
    /// most `cap`; the same for every Faction.
    pub fn crowd_extra(&self) -> u32 {
        let c = &self.tables.crowding;
        let over = self.climate.temperature - c.above;
        if over <= 0.0 || c.step <= 0.0 {
            return 0;
        }
        // A hair of tolerance so +2.0 reads a full step over +1.8 in floating point.
        let steps = ((over + 1e-9) / c.step).floor() as u32;
        (steps * c.per_step).min(c.cap)
    }

    /// Ticket #86: the capacity plus the crowd.
    pub fn colony_ship_crowded_capacity(&self, seat: Seat) -> u32 {
        self.colony_ship_capacity(seat) + self.crowd_extra()
    }

    /// Ticket #141 (version 0.07.3): how many of a Nation State's waiting Emigrants the pending
    /// orders already send away, by sea or by lift, so the same people are never ordered twice.
    pub fn emigrants_leaving(&self, pending: &[crate::orders::Order], state: StateId) -> u32 {
        use crate::orders::Order;
        pending
            .iter()
            .map(|o| match o {
                Order::SendToAntarctica { state: s, n, .. } | Order::LiftToStation { state: s, n, .. } if *s == state => *n,
                _ => 0,
            })
            .sum()
    }

    /// The population a lift from a Launch Site takes for this many Colonists (Coach Class doubles it).
    pub fn lift_population(&self, seat: Seat, colonists: u32) -> f64 {
        // Ticket #73: paid when the Emigrants muster, not when a Ship lifts them.
        self.tables.emigrants.population_each * colonists as f64 * self.tables.faction(self.kind(seat)).lift_population_multiplier
    }

    // ------------------------------------------ Ticket #189 (version 0.08.0): people carry their schooling
    //
    // Every pool of people -- the Emigrants waiting on a Region's card, the Colonists aboard a Ship,
    // the people living at a Colony -- is a head count AND a mean Education Level. These four are
    // the only way a pool is moved, so a caller cannot move people and forget to move what they
    // know. Deaths never move a mean: the dead are drawn evenly from the people there.

    /// The weighted mean of two pools, by head count. An empty result keeps the neutral figure.
    pub fn blend(a_n: u32, a_e: f64, b_n: u32, b_e: f64) -> f64 {
        let total = a_n + b_n;
        if total == 0 {
            return 1.0;
        }
        (a_e * a_n as f64 + b_e * b_n as f64) / total as f64
    }

    /// Muster `n` Emigrants in a Nation State: they take its Education Level as it stands NOW, and
    /// average into whoever is already waiting there.
    pub fn muster_emigrants(&mut self, s: StateId, n: u32) {
        let taught = self.education_level(s);
        let st = self.state(s);
        let blended = Game::blend(st.emigrants, st.emigrants_education, n, taught);
        let st = self.state_mut(s);
        st.emigrants += n;
        st.emigrants_education = blended;
    }

    /// Take `n` Emigrants off a Nation State's card and say what they know. The pile's mean does not
    /// move: the ones who left are no better or worse taught than the ones who stayed.
    pub fn take_emigrants(&mut self, s: StateId, n: u32) -> f64 {
        let taught = self.state(s).emigrants_education;
        let st = self.state_mut(s);
        st.emigrants = st.emigrants.saturating_sub(n);
        taught
    }

    /// Put `n` people who know `taught` aboard a Ship, averaging with whoever is already aboard.
    pub fn load_people(&mut self, id: ShipId, n: u32, taught: f64) {
        let Some(s) = self.ship(id) else { return };
        let blended = Game::blend(s.colonists, s.colonists_education, n, taught);
        if let Some(s) = self.ship_mut(id) {
            s.colonists += n;
            s.colonists_education = blended;
        }
    }

    /// Take `n` people off a Ship and say what they know; the mean aboard does not move.
    pub fn unload_people(&mut self, id: ShipId, n: u32) -> f64 {
        let taught = self.ship(id).map(|s| s.colonists_education).unwrap_or(1.0);
        if let Some(s) = self.ship_mut(id) {
            s.colonists = s.colonists.saturating_sub(n);
        }
        taught
    }

    /// Settle `n` people who know `taught` at a Colony, averaging by head count into BOTH figures:
    /// the live one an Institute has been raising, and the settler average that is its decay floor.
    pub fn settle_people(&mut self, c: ColonyId, n: u32, taught: f64) {
        let Some(col) = self.colony(c) else { return };
        let live = Game::blend(col.colonists, col.education, n, taught);
        let settlers = Game::blend(col.colonists, col.settler_education, n, taught);
        if let Some(col) = self.colony_mut(c) {
            col.colonists += n;
            col.education = live;
            col.settler_education = settlers;
        }
    }

    /// Take `n` people off a Colony and say what they know; neither figure moves.
    pub fn take_colonists(&mut self, c: ColonyId, n: u32) -> f64 {
        let taught = self.colony(c).map(|x| x.education).unwrap_or(1.0);
        if let Some(col) = self.colony_mut(c) {
            col.colonists = col.colonists.saturating_sub(n);
        }
        taught
    }

    /// Ticket #187 (version 0.08.0): a place's Education Level -- a Region's card plus its Schools,
    /// a Colony's settlers plus its Institute.
    pub fn place_education(&self, place: Place) -> f64 {
        match place {
            Place::State(s) => self.education_level(s),
            Place::Colony(c) => self.colony(c).map(|x| x.education).unwrap_or(1.0),
        }
    }

    /// Ticket #187: how hard a place is to sway, from how well it is schooled. 1.0 at the pivot and
    /// no effect; down to `1 - band` at the lowest figure any card carries, up to `1 + band` at the
    /// School's ceiling. Each side scales to its OWN end, because the range is asymmetric -- 0.30
    /// below the pivot and 1.00 above -- and one coefficient would leave the floor unreachable.
    pub fn resistance(&self, place: Place) -> f64 {
        let r = &self.tables.influence.resistance;
        let e = self.place_education(place);
        if (e - r.pivot).abs() < 1e-9 {
            1.0
        } else if e < r.pivot {
            let span = (r.pivot - r.low).max(1e-9);
            1.0 - r.band * ((r.pivot - e) / span).clamp(0.0, 1.0)
        } else {
            let span = (r.high - r.pivot).max(1e-9);
            1.0 + r.band * ((e - r.pivot) / span).clamp(0.0, 1.0)
        }
    }

    /// Ticket #187: what `spent` Influence actually becomes at `place`, for a Faction that does not
    /// control it: `spent / resistance`, rounded down. A well-schooled place gives less back than
    /// was put in and a badly-schooled one gives more. The CONTROLLER converts in full and never
    /// calls this, so reinforcing a place you hold is never taxed.
    pub fn standing_from(&self, place: Place, spent: i64) -> i64 {
        let r = self.resistance(place);
        if r <= 0.0 {
            return spent;
        }
        (spent as f64 / r).floor() as i64
    }

    /// Ticket #185 (version 0.08.0): a Nation State's Education Level as it stands -- the figure on
    /// its card plus whatever a School has added. It was the card figure alone until now, and
    /// nothing in the game moved it.
    pub fn education_level(&self, s: StateId) -> f64 {
        self.tables.state(s).education_level + self.state(s).schooling
    }

    /// Ticket #185: the School's work, run once a turn at Income. It climbs a step while a School
    /// stands and is online and falls back at the same rate when it does not, so what took five
    /// turns to build takes five turns to lose; it never goes above the ceiling, and never below
    /// the state's own card.
    pub fn run_schools(&mut self) {
        let step = self.tables.school.per_turn;
        let ceiling = self.tables.school.ceiling;
        for sid in StateId::ALL {
            let card = self.tables.state(sid).education_level;
            let open = self.state(sid).facilities.iter().any(|f| f.kind.does_the_job_of(FacilityKind::School) && f.working());
            let now = self.state(sid).schooling;
            let next = if open { (now + step).min((ceiling - card).max(0.0)) } else { (now - step).max(0.0) };
            self.state_mut(sid).schooling = next;
        }
        // Ticket #185 (version 0.08.0): the Institutes do the same off Earth. A Colony's live figure
        // climbs to the ceiling while one stands and is online, and falls back to the settlers' own
        // average -- what its people know without a school -- when it stops. It never falls below
        // that: a Colony whose Institute goes dark has not become less educated, its school shut.
        let ids: Vec<ColonyId> = self.colonies.iter().map(|c| c.id).collect();
        for cid in ids {
            let Some(col) = self.colony(cid) else { continue };
            let open = col.modules.iter().any(|m| m.kind.does_the_job_of(ModuleKind::Institute) && m.working());
            let floor = col.settler_education;
            let now = col.education;
            let next = if open { (now + step).min(ceiling) } else { (now - step).max(floor) };
            if let Some(col) = self.colony_mut(cid) {
                col.education = next;
            }
        }
    }

    /// Ticket #196 (version 0.08.0): how many Emigrants this seat could muster in this state right
    /// now -- its per-turn cap, or what the state's people can pay for, whichever is smaller.
    ///
    /// A batch was all-or-nothing until now, and Coach Class costs the Arkwrights twice the population
    /// for twice the batch: 8 x 2.0 = 16.0 people, where Australia carries 10.1 to 12.6 and is the
    /// only one of the fourteen Regions below 16. Measured, an Arkwright AI holding it was refused
    /// on all 243 turns it tried and mustered nothing in twenty games. A muster takes what the
    /// Region can pay for.
    pub fn emigrants_affordable(&self, seat: Seat, s: StateId) -> u32 {
        let per = self.emigrants_per_turn(seat);
        let each = self.lift_population(seat, 1);
        if each <= 0.0 {
            return per;
        }
        let afford = (self.state(s).population / each).floor().max(0.0) as u32;
        per.min(afford)
    }

    /// What a Colony Module costs this seat in Materials, rounded down (ticket #51).
    pub fn module_materials(&self, seat: Seat, kind: ModuleKind) -> i64 {
        let base = self.tables.module(kind).materials as f64;
        (base * self.tables.faction(self.kind(seat)).module_materials_multiplier).floor() as i64
    }

    /// Ticket #88 (version 0.06.0): the working Mines a Colony holds (not mothballed, not still
    /// building).
    pub fn working_mines(&self, c: &Colony) -> usize {
        c.modules.iter().filter(|m| m.kind == ModuleKind::Mine && m.working()).count()
    }

    /// Ticket #88: what a Module costs this seat at this Colony: the row times the Faction's
    /// multiplier, times the in-situ step for the Colony's working Mines, rounded down, never
    /// below the floor of the row. Ships and stations never take it.
    pub fn module_materials_at(&self, seat: Seat, colony: ColonyId, kind: ModuleKind) -> i64 {
        let row = self.tables.module(kind).materials as f64;
        let faction = self.tables.faction(self.kind(seat)).module_materials_multiplier;
        let t = &self.tables.in_situ;
        let step = match self.colony(colony).map(|c| self.working_mines(c)).unwrap_or(0) {
            0 => 1.0,
            1 => t.one_mine,
            _ => t.two_mines,
        };
        let price = (row * faction * step).max(row * t.floor);
        price.floor() as i64
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

    /// What a Ship costs this seat: the units.toml figure, or the Faction's own Colony Ship price;
    /// ticket #83 (version 0.06.0): times the Faction's Ship multiplier, rounded down (the
    /// Arkwrights' 0.85).
    pub fn ship_materials(&self, seat: Seat, kind: UnitKind) -> i64 {
        let card = self.tables.faction(self.kind(seat));
        let base = match (kind, card.colony_ship_materials) {
            (UnitKind::ColonyShip, Some(m)) => m,
            _ => self.tables.unit(kind).materials,
        };
        (base as f64 * card.ship_materials_multiplier).floor() as i64
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

    /// Ticket #270 (version 0.08.4): every Army is raised here, named as it is raised.
    pub fn raise_army(&mut self, place: Place, standing: bool) -> ArmyId {
        let id = ArmyId(self.fresh_id());
        let home = match place {
            Place::State(s) => ArmyHome::State(s),
            Place::Colony(c) => ArmyHome::Colony(c),
        };
        // An ordinal among every Army ever raised from this home, so a re-raised Standing Army is
        // the next number and no name is ever given twice: the 1st Chinese Army is the one the game
        // began with, the 2nd the first anyone built. A Colony's is its Garrison, numbered from the
        // second. At the designer's word: an Army is its Region's, so its name says which Region.
        let name = match place {
            Place::State(s) => {
                let st = self.state_mut(s);
                st.armies_raised += 1;
                let nth = st.armies_raised as usize;
                format!("the {} {} Army", Game::ordinal(nth), self.tables.state(s).demonym)
            }
            Place::Colony(c) => {
                let nth = self.armies.iter().filter(|a| a.home == home).count() + 1;
                let site = self.colony(c).map(|col| self.tables.body(col.body).slots[col.slot as usize].name.clone()).unwrap_or_else(|| "Colony".to_string());
                if nth == 1 { format!("the {site} Garrison") } else { format!("the {} {site} Garrison", Game::ordinal(nth)) }
            }
        };
        self.armies.push(Army { id, name, home, at: ArmyAt::Place(place), damage: 0, standing, stance: Stance::Hold, escaped: false, move_to: None });
        id
    }

    /// Ticket #270: what an Army is called -- its name, or for a save from before names the old form.
    pub fn army_name(&self, a: &Army) -> String {
        if !a.name.is_empty() {
            return a.name.clone();
        }
        if a.standing { "the Standing Army".to_string() } else { "the Army".to_string() }
    }

    /// Ticket #270: "1st", "2nd", "3rd", "4th", "11th", "21st".
    pub fn ordinal(n: usize) -> String {
        let suffix = match (n % 10, n % 100) {
            (1, 11) | (2, 12) | (3, 13) => "th",
            (1, _) => "st",
            (2, _) => "nd",
            (3, _) => "rd",
            _ => "th",
        };
        format!("{n}{suffix}")
    }

    pub fn spawn_standing_army(&mut self, s: StateId) {
        // Ticket #270 (version 0.08.4): raised through the one door, and named there.
        self.raise_army(Place::State(s), true);
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
        // Ticket #238 (version 0.08.3): and the hold clock, reset only when the HOLDER changes.
        // A state written back to the same seat -- which happens -- keeps its clock, or the three
        // faction-only orders could be denied forever by a repeated write nobody can see.
        let who = st.control.controller();
        if who != st.held_by {
            st.held_by = who;
            st.held_since = who.map(|_| turn);
        }
    }

    /// Ticket #238 (version 0.08.3): may this seat remake this Region yet? The Strip Permit, the
    /// Leapfrog and the Exodus Call all change a country for good, and a Faction that has just
    /// walked in does not get to do that.
    ///
    /// A Region with NO clock recorded passes. That is every Region in a save written before this
    /// version, and the alternative -- treating a missing clock as "just arrived" -- would silently
    /// disable three Faction orders in every old save, which is a worse surprise than a save that
    /// is briefly generous.
    pub fn may_remake(&self, seat: Seat, s: StateId) -> bool {
        match self.turns_held(seat, s) {
            None => true,
            Some(held) => held >= self.tables.faction_orders.min_turns_held,
        }
    }

    /// Ticket #238: the turn this seat may first remake this Region, for the refusal to name.
    pub fn may_remake_on_turn(&self, s: StateId) -> u32 {
        let min = self.tables.faction_orders.min_turns_held;
        self.state(s).held_since.map(|since| since + min).unwrap_or(self.turn)
    }

    /// Ticket #238 (version 0.08.3): how many whole turns this seat has held this Region, or None
    /// if it does not hold it. The turn of the taking does not count, so a Region taken on turn 10
    /// answers 0 that turn, 1 on turn 11, and opens its Faction-only order on turn 13.
    pub fn turns_held(&self, seat: Seat, s: StateId) -> Option<u32> {
        let st = self.state(s);
        if st.control.controller() != Some(seat) {
            return None;
        }
        st.held_since.map(|since| self.turn.saturating_sub(since))
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

    /// Ticket #143 (version 0.07.3): the population figure's unit, in people. A Region's figure, a
    /// Colonist and an Emigrant are all counted in it, so `Region population 76.0` is 380 million
    /// people and one Colonist is five million. (A hundred million, with a Colonist a tenth of one,
    /// until this ticket.) The designer: *"I want country cards to use the actual population."*
    pub const PEOPLE_PER_UNIT: f64 = 5_000_000.0;

    /// Units in a hundred million people, since the cards quote per-person Emissions at that rate.
    pub const UNITS_PER_HUNDRED_MILLION: f64 = 100_000_000.0 / Game::PEOPLE_PER_UNIT;

    /// A population figure written as real people: `1.14B`, `380M`, `20M`.
    pub fn people_text(units: f64) -> String {
        let people = units * Game::PEOPLE_PER_UNIT;
        if people >= 1_000_000_000.0 {
            format!("{:.2}B", people / 1_000_000_000.0)
        } else {
            format!("{:.0}M", people / 1_000_000.0)
        }
    }

    /// The card's form: the figure in units to one decimal, and the real number beside it.
    pub fn population_text(units: f64) -> String {
        format!("{units:.1} ({})", Game::people_text(units))
    }

    /// Ticket #143: everyone on Earth -- the Regions' figures and the Colonists in Antarctica.
    pub fn earth_population(&self) -> f64 {
        let regions: f64 = self.states.iter().map(|s| s.population).sum();
        let antarctica: u32 = self.colonies.iter().filter(|c| !self.off_earth(c)).map(|c| c.colonists).sum();
        regions + antarctica as f64
    }

    /// Ticket #143: everyone living off Earth, in Habitats, a station over Earth counting as off and
    /// Antarctica as on -- the Off-world Presence count, summed over every seat. People aboard a Ship
    /// are not yet anywhere and are not counted.
    pub fn space_population(&self) -> u32 {
        self.colonies.iter().filter(|c| self.off_earth(c)).map(|c| c.colonists).sum()
    }

    pub fn population_factor(&self, s: StateId) -> f64 {
        // Ticket #143 (version 0.07.3): the unit is five million people, so 1,000 units is the five
        // billion that 50 hundred-million was.
        //
        // Ticket #188 (version 0.08.0): the BONUS -- the part above 1 -- is scaled by the state's
        // schooling, so a great many badly-schooled people are worth less to a Research Lab than a
        // great many well-schooled ones. The base 1 stays, so no Region is ever worth less than one
        // with nobody in it. Uncapped in both directions: capping it would make the rule a pure nerf
        // and cancel the whole interaction with the School, which is what makes it matter.
        //
        // The Education Level therefore applies TWICE to a Lab -- here, and as the outright
        // multiplier it has always been. That compounding is the point.
        1.0 + (self.state(s).population / 1000.0) * self.education_level(s)
    }

    /// Ticket #97 (version 0.07.0): the Modules this Colony or Space Station may hold: the table's
    /// free allowance, and one more for every `per_colonist` Colonists living there. One formula
    /// for the ground and for orbit, so a Space Station founded bare holds the allowance and grows
    /// only as its people arrive.
    pub fn module_slots(&self, c: &Colony) -> u32 {
        let s = &self.tables.slots;
        s.base + c.colonists / s.per_colonist.max(1)
    }

    /// Ticket #97: the Modules standing or building here that count against the cap. A mothballed
    /// Module keeps its slot and one under construction reserves one, exactly as a Facility does in
    /// a Nation State; the Archive is exempt and counted on neither side.
    pub fn module_slots_used(&self, c: &Colony) -> u32 {
        // Ticket #164 (version 0.07.5): the Core Module is exempt as the Archive is -- it is what a
        // founding gives, not something bought out of the allowance.
        let exempt = |k: ModuleKind| k == ModuleKind::Archive || k == ModuleKind::Core;
        let standing = c.modules.iter().filter(|m| !exempt(m.kind)).count() as u32;
        let building = c
            .queue
            .iter()
            .filter(|b| matches!(b.item, BuildItem::Module(k) if !exempt(k)))
            .count() as u32;
        standing + building
    }

    /// Ticket #97: the room left. A cap that has fallen below what already stands (Colonists lost
    /// to crowding, a Habitat destroyed, the place changing hands) destroys nothing and mothballs
    /// nothing: it simply leaves no room until the count is back under.
    pub fn free_module_slots(&self, c: &Colony) -> u32 {
        self.module_slots(c).saturating_sub(self.module_slots_used(c))
    }

    pub fn habitat_room(&self, c: &Colony) -> u32 {
        // Ticket #51: Expanded Habitats and the Faction's own Habitat capacity are read for whoever
        // holds the Colony, since Provisional Findings gives the Archivists half the Tech early.
        let seat = c.control.controller();
        let per = self.tables.module(ModuleKind::Habitat).holds_colonists as i64
            // Ticket #207 (version 0.08.1): the HABITAT clause of Expanded Habitats, which is +4
            // where the Colony Ship clause it shares a card with is +2.
            + seat.map(|s| self.habitat_addition(s, TechId::ExpandedHabitats)).unwrap_or(0);
        let faction = seat.map(|s| self.tables.faction(self.kind(s)).habitat_capacity_multiplier).unwrap_or(1.0);
        // Ticket #140 (version 0.07.3): a Habitat holds the same everywhere. It read the slot's
        // Habitat yield on a surface (ticket #57) and 1.0 in orbit (ticket #46) until the designer
        // traded that yield for a Research one.
        let habitats = c.modules.iter().filter(|m| m.kind == ModuleKind::Habitat).count() as f64;
        // Ticket #164 (version 0.07.5): the Core Module holds people too, and holds a FLAT figure --
        // Expanded Habitats and the Arkwrights' capacity multiplier reach a Habitat and not this, at
        // the designer's word: *"yes everyone arkwrights can always build habitats."* It is what
        // lets a station founded this turn take its first four before anything is built.
        let core: u32 = c.modules.iter().filter(|m| m.kind == ModuleKind::Core).map(|_| self.tables.module(ModuleKind::Core).holds_colonists).sum();
        (habitats * per.max(0) as f64 * faction).floor() as u32 + core
    }

    /// Ticket #140 (version 0.07.3): what an Observatory here is multiplied by -- the slot's own
    /// Research yield on a surface, and in orbit the Body's, since a station over Mars is doing
    /// Mars science. The first Body yield a station has ever read.
    pub fn research_yield_at(&self, c: &Colony) -> f64 {
        if c.in_orbit { self.tables.body(c.body).research_yield } else { self.slot_yields(c.body, c.slot).research }
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

    /// Ticket #81 (version 0.06.0): whether a Colony is off Earth for every rule that asks. Any
    /// Body but Earth is; so is a station over Earth; Antarctica (Earth's ground) is not.
    pub fn off_earth(&self, c: &Colony) -> bool {
        c.body != BodyId::Earth || c.in_orbit
    }

    /// Colonists living in Habitats off Earth, for one seat (spec 15).
    pub fn off_world_colonists(&self, seat: Seat) -> u32 {
        // Ticket #44: Colonists in Antarctica live on Earth. Ticket #81: those on a station over
        // Earth do not.
        self.colonies.iter().filter(|c| c.control.controller() == Some(seat) && self.off_earth(c)).map(|c| c.colonists).sum()
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
            Some(c) => threshold.max(self.seat(c).influence.get(&target).copied().unwrap_or(0) + self.challenge_margin_for(Some(seat), target)),
            None => threshold,
        }
    }

    /// Ticket #190 (version 0.08.0): the challenge margin at `target` -- the base, plus what a
    /// Constabulary adds where one stands and is online. The Constabulary protects WHOEVER HOLDS the
    /// place, not the Faction that raised it: a police force serves the government of the day, and
    /// the building needs no memory of who paid for it, so a Faction that builds one in a Region it
    /// later loses has made its own job harder. At most one stands in a Region, so no stacking
    /// question arises. It does nothing on a neutral place, which has no margin at all -- the caller
    /// asks for a margin only when a holder is there to be challenged.
    pub fn challenge_margin_at(&self, target: Target) -> i64 {
        self.challenge_margin_for(None, target)
    }

    /// Ticket #224 (version 0.08.2): the margin a NAMED challenger faces, which since this version
    /// is a property of the ORDERED PAIR rather than of the place. The Constabulary's own clause is
    /// deliberately not: it protects whoever holds the place and asks nothing about who wants it.
    ///
    /// The term is read from the CONTROLLER'S view of the challenger and applies only while that is
    /// negative -- `the people here have learned to distrust you`. A positive score never LOWERS the
    /// margin: that would make friendship a weapon and punish a player who spent nine acts earning
    /// it. `floor(|score| / 4)` gives 0 down to Wary, 1 at Cold and 2 at Hostile; the true maximum
    /// is 2, because the shown score clamps at -10 and `floor(10 / 4)` is 2. A cap of 3 would be
    /// dead text, so it is written as the 2 it actually is.
    ///
    /// The whole SHOWN score counts, Blame included, at the designer's word. The recorded risk: the
    /// Custodians already win 42 of 80 and Blame already raises a dirty Faction's Threshold
    /// everywhere, so this hands the strongest Faction a second permanent advantage against the
    /// weakest -- worth about +1 on 45% of turns against the Prospectors, +2 once deeds stack on.
    pub fn challenge_margin_for(&self, challenger: Option<Seat>, target: Target) -> i64 {
        let t = &self.tables.influence;
        let relations = match (challenger, self.place_control(target).controller()) {
            (Some(ch), Some(holder)) if ch != holder => {
                let score = self.relations_score(holder, ch);
                if score < 0 { (score.abs() / 4).min(t.relations_margin_cap) } else { 0 }
            }
            _ => 0,
        };
        let guarded = matches!(target, Place::State(s) if self.state(s).facilities.iter().any(|f| f.kind == FacilityKind::Constabulary && f.working()));
        // Ticket #201 (version 0.08.1): Civil Defense doubles what a Constabulary is worth at the
        // gate -- 10 where it adds 5 without. The Tech is the world's, as every Tech is, so it
        // helps whoever holds a garrisoned Region and hinders whoever wants one, which is the same
        // asymmetry the Constabulary itself has carried since ticket #190.
        let garrison = if self.has_tech(TechId::CivilDefense) { t.constabulary_margin_defended } else { t.constabulary_margin };
        t.challenge_margin + relations + if guarded { garrison } else { 0 }
    }

    /// Ticket #263 (version 0.08.4): what a seat has under way -- every build it has begun, with
    /// the turns until it lands, and every Ship of its in transit, with its name, its road and the
    /// turns left. The Faction window's Under way block reads this; the Report or the AI could.
    /// Builds are counted by the seat that ORDERED them (`Build.seat`), so a build begun in a Region
    /// that has since changed hands stays with whoever paid for it.
    pub fn under_way(&self, seat: Seat) -> UnderWay {
        let mut builds: Vec<(String, Place, u32)> = Vec::new();
        for sid in StateId::ALL {
            for b in self.state(sid).queue.iter().filter(|b| b.seat == seat) {
                builds.push((b.item.name(), Place::State(sid), b.due_turn.saturating_sub(self.turn) + 1));
            }
        }
        for c in &self.colonies {
            for b in c.queue.iter().filter(|b| b.seat == seat) {
                builds.push((b.item.name(), Place::Colony(c.id), b.due_turn.saturating_sub(self.turn) + 1));
            }
        }
        let mut transits: Vec<(String, String, String, u32)> = Vec::new();
        for s in self.ships.iter().filter(|s| s.seat == seat) {
            if let ShipAt::Transit { from, to, turns_left } = s.at {
                transits.push((self.ship_name(s), self.tables.body(from).name.clone(), self.tables.body(to).name.clone(), turns_left));
            }
        }
        // Soonest first, at the designer's word; the name breaks a tie so the order is stable.
        builds.sort_by(|a, b| a.2.cmp(&b.2).then_with(|| a.0.cmp(&b.0)));
        transits.sort_by(|a, b| a.3.cmp(&b.3).then_with(|| a.0.cmp(&b.0)));
        UnderWay { builds, transits }
    }

    /// Ticket #262 (version 0.08.4): the rival nearest to taking a held place -- its seat, its
    /// Standing there, and the price it must reach (`influence_needed_for`, its own threshold with
    /// Blame inside it, or the holder's Standing plus the margin for that pair). None on a place
    /// nobody holds, or where no rival has a Standing.
    pub fn nearest_challenger(&self, place: Place) -> Option<(Seat, i64, i64)> {
        let holder = self.place_control(place).controller()?;
        Seat::ALL
            .into_iter()
            .filter(|s| *s != holder)
            .filter_map(|s| {
                let standing = self.seat(s).influence.get(&place).copied().unwrap_or(0);
                (standing > 0).then(|| (s, standing, self.influence_needed_for(s, place)))
            })
            // The nearest to its OWN price, at the designer's word -- not the highest Standing. They
            // differ when Blame or Relations move one rival's price and not another's, and the
            // nearer one takes the place first, which is what the line is warning of.
            .min_by_key(|(_, standing, price)| *price - *standing)
    }

    /// Ticket #257 (version 0.08.4): does a Sea Wall stand and work in this state? The Climate
    /// phase, the Storm Surge card and the card all ask the same question.
    pub fn sea_wall_working(&self, sid: StateId) -> bool {
        self.state(sid).facilities.iter().any(|f| f.kind == FacilityKind::SeaWall && f.working())
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
        // Ticket #267 (version 0.08.4): the ledger, not the physics -- what rivals have laid on
        // this seat by Smear counts, at the designer's word, so the share the rules read diverges
        // from what the seat put in the air.
        // Ticket #268: credits bought come off; credits sold past what was held go on.
        let oversold = (s.credits_sold - s.blame_removed).max(0.0);
        (s.blame_emitted - s.blame_removed + s.blame_smeared + oversold - s.credits_bought).max(0.0)
    }

    /// Ticket #53 defined the credit as the ppm removed BEYOND everything ever emitted -- and
    /// ticket #265 (version 0.08.4) measured it at zero in ten games of ten: the Custodians scrub a
    /// third to a half of what they emit, never more than all of it. At the designer's word the
    /// credit is now what the Faction has REMOVED, full stop -- its Scrubbers' ppm over the game,
    /// and the Custodians' Directive into the Sink -- so the word has a figure in every game, and
    /// a carbon credit has a supply that exists. Blame itself is unchanged: emitted less removed,
    /// never below nothing.
    pub fn blame_credit(&self, seat: Seat) -> f64 {
        // Ticket #268: less what has been sold as carbon credits.
        let s = self.seat(seat);
        (s.blame_removed - s.credits_sold).max(0.0)
    }

    /// Ticket #268 (version 0.08.4): the seat that sells carbon credits -- the Custodians'.
    pub fn credit_seller(&self) -> Option<Seat> {
        Seat::ALL.into_iter().find(|s| self.kind(*s) == FactionKind::Custodians)
    }

    /// Ticket #268: the price multiplier the seller's Relations level toward `buyer` sets, or None
    /// where the seller refuses -- Hostile, or no seller at the table.
    pub fn credit_price_multiplier(&self, buyer: Seat) -> Option<f64> {
        let seller = self.credit_seller()?;
        let c = &self.tables.carbon_credits;
        match self.relations_level(seller, buyer) {
            "Friendly" => Some(c.friendly),
            "Cordial" => Some(c.cordial),
            "Neutral" => Some(c.neutral),
            "Wary" => Some(c.wary),
            "Cold" => Some(c.cold),
            _ => None,
        }
    }

    /// Ticket #268: what `ppm` of carbon credit costs `buyer` in Ducats, at the table price times
    /// the seller's view of them, rounded up so a lot is never free.
    pub fn credit_cost(&self, buyer: Seat, ppm: i64) -> Option<i64> {
        let m = self.credit_price_multiplier(buyer)?;
        Some(((ppm * self.tables.carbon_credits.price_per_ppm) as f64 * m).ceil() as i64)
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

    /// Ticket #266 (version 0.08.4): what a seat's Standing on `place` loses in a turn it received
    /// nothing. A held place decays `decay_controlled`; a Colony or a station `decay`; a Region the
    /// seat does not hold reads the seat's Blame share by the step rule -- `decay_slow` at or below
    /// `decay_slow_below`, `decay_fast` at or above `decay_fast_from`, `decay` between. Ticket #53
    /// wrote "never on Standing decay" into the Blame rule; the designer reversed that here.
    pub fn standing_decay_for(&self, seat: Seat, place: Place) -> i64 {
        let t = &self.tables.influence;
        if self.place_control(place).controller() == Some(seat) {
            return t.decay_controlled;
        }
        if !matches!(place, Place::State(_)) {
            return t.decay;
        }
        let share = self.blame_share(seat);
        if share <= t.blame.decay_slow_below {
            t.blame.decay_slow
        } else if share >= t.blame.decay_fast_from {
            t.blame.decay_fast
        } else {
            t.decay
        }
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

    /// Ticket #134 (version 0.07.3): whether this seat directs a place -- a Region it controls or
    /// occupies, or a Colony likewise -- which is what a standing Max order needs of its place.
    pub fn directs(&self, seat: Seat, place: Place) -> bool {
        match place {
            Place::State(s) => self.directed_states(seat).contains(&s),
            Place::Colony(c) => self.directed_colonies(seat).contains(&c),
        }
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
        // Ticket #183 (version 0.08.0): what the Spaceports earned last turn is added AFTER the
        // multiplier, at face value. This is a deliberate departure from the Embassy, whose
        // contribution sits inside it and so gives the Arkwrights 1.6 rather than 2: their x0.8 says
        // they are bad at diplomacy, and this clause says they are good at moving people, which is a
        // different kind of Influence. Applying their designed weakness to the rule written to mend
        // it would be the rule arguing with itself.
        (base as f64 * m).floor() as i64 + self.seat(seat).spaceport_influence
    }

    /// Ticket #183 (version 0.08.0): the Spaceport's clause. +1 Influence for every Emigrant it lifts
    /// OFF EARTH, which means the two launches -- onto a Ship in orbit, or onto a Space Station of
    /// the seat's over Earth. The sea to Antarctica pays NOTHING: it is explicitly not a launch and
    /// Antarctica is explicitly on Earth, so an Arkwright choosing the ice is choosing to forgo the
    /// Influence. Lifting an Army pays nothing either, because the clause is per Emigrant.
    ///
    /// It is paid ONCE per Emigrant however many Spaceports stand -- the Emigrant is what is counted,
    /// not the building -- and only where the seat CONTROLS the Region, per ticket #181. There is no
    /// cap: the designer's word was that the muster limit is the brake, "8 a turn is already the
    /// brake".
    pub fn pay_spaceport(&mut self, seat: Seat, from: StateId, n: u32) {
        if n == 0 || self.state(from).control != Control::Controlled(seat) {
            return;
        }
        if !self.state(from).facilities.iter().any(|f| f.kind == FacilityKind::Spaceport && f.working()) {
            return;
        }
        self.seats[seat.index()].spaceport_influence += n as i64;
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
                    research: draw(card.research_yield),
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

    /// Whether a seat may land Armies and Colonists on the GROUND of a Body (spec 9.3).
    ///
    /// Ticket #99 (version 0.07.0): only a rival holding Orbital Control outright shuts the surface.
    /// Before this, any enemy warship present shut the whole Body to everyone else, so a single
    /// frigate in Earth orbit locked a Faction out of its own Space Station for nine turns, and two
    /// rivals' warships present punished the bystander hardest by denying everybody. A blockade is
    /// now the business of one Orbital Slot: see `slot_blockaded_against`.
    pub fn may_land(&self, seat: Seat, body: BodyId) -> bool {
        match self.orbital_control(body) {
            Some(s) => s == seat,
            None => true,
        }
    }

    /// Ticket #99 (version 0.07.0): a rival warship sitting in this Orbital Slot blockades it. The
    /// blockade stops Colonists and Armies being unloaded into the station standing there, and stops
    /// that station refuelling a Ship; it reaches no further, and never touches the ground.
    pub fn slot_blockaded_against(&self, seat: Seat, body: BodyId, slot: u32) -> bool {
        self.ships
            .iter()
            .any(|s| s.seat != seat && s.kind.is_warship() && !s.escaped && s.at == ShipAt::Body(body) && s.slot == Some(slot))
    }

    /// Ticket #99: the seats blockading this slot, for the card and the Report.
    pub fn slot_blockaders(&self, body: BodyId, slot: u32) -> Vec<Seat> {
        let mut v: Vec<Seat> = self
            .ships
            .iter()
            .filter(|s| s.kind.is_warship() && !s.escaped && s.at == ShipAt::Body(body) && s.slot == Some(slot))
            .map(|s| s.seat)
            .collect();
        v.sort();
        v.dedup();
        v
    }

    /// Ticket #99: a station of this seat's at this Body that a rival warship is not blockading, so
    /// a Refuel has somewhere to draw from.
    pub fn refuelling_station(&self, seat: Seat, body: BodyId) -> bool {
        self.colonies.iter().any(|c| {
            c.in_orbit && c.body == body && c.control.controller() == Some(seat) && !self.slot_blockaded_against(seat, body, c.slot)
        })
    }

    /// Ticket #99: whether a Colony of this seat's can be reached at all -- a station in a
    /// blockaded slot cannot, a Colony on the ground reads `may_land`.
    pub fn may_unload_into(&self, seat: Seat, colony: ColonyId) -> bool {
        let Some(c) = self.colony(colony) else { return false };
        if c.in_orbit {
            !self.slot_blockaded_against(seat, c.body, c.slot)
        } else {
            self.may_land(seat, c.body)
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
        let (turns, fuel) = self.transit_cost_with(from, to, faction, self.tech_multiplier(seat, TechId::EfficientTransit), turn);
        // Ticket #92 (version 0.06.0): a working Mass Driver of the seat's at the Body it leaves
        // takes a flat figure off, after the multipliers, never below the minimum.
        if self.mass_driver_at(seat, from) {
            let md = &self.tables.mass_driver;
            return (turns, (fuel - md.fuel_off).max(md.fuel_min));
        }
        (turns, fuel)
    }

    /// Ticket #92: whether the seat has a working Mass Driver at a ground Colony on this Body.
    pub fn mass_driver_at(&self, seat: Seat, body: BodyId) -> bool {
        self.colonies
            .iter()
            .filter(|c| !c.in_orbit && c.body == body && c.control.director() == Some(seat))
            .any(|c| c.modules.iter().any(|m| m.kind == ModuleKind::MassDriver && m.working()))
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
        // Ticket #93: the crossing names its own transfer table, Mars's or Venus's.
        let (turns, fuel) = match self.crossing(from, to, turn) {
            None => (turns, fuel as f64),
            Some((offset, tr)) => {
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
        self.span_offset_of(BodyId::Mars, turn, angle)
    }

    /// Ticket #93 (version 0.06.0): the phase angle of any planet against Earth, folded to
    /// -180..180: Mars's for the Mars window, Venus's for Venus's.
    pub fn phase_angle_of(&self, body: BodyId, turn: u32) -> f64 {
        crate::ephemeris::wrap_180(self.heliocentric_longitude(body, turn) - self.heliocentric_longitude(BodyId::Earth, turn))
    }

    /// Ticket #93: `span_offset` for any planet's window.
    fn span_offset_of(&self, body: BodyId, turn: u32, angle: f64) -> f64 {
        let start = crate::ephemeris::wrap_180(self.phase_angle_of(body, turn) - angle);
        let end = crate::ephemeris::wrap_180(self.phase_angle_of(body, turn + 1) - angle);
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
        self.crossing(from, to, turn).map(|(offset, _)| offset)
    }

    /// Which system a Body belongs to: 0 the Earth system, 1 the Mars system, 2 Venus (ticket #93).
    pub fn system_of(body: BodyId) -> u8 {
        match body {
            BodyId::Earth | BodyId::Moon => 0,
            BodyId::Mars | BodyId::Phobos | BodyId::Deimos => 1,
            BodyId::Venus => 2,
        }
    }

    /// Ticket #93: whether a leg runs between two Bodies at all. Every leg runs but the one
    /// between Venus and the Mars system, which this version does not offer: fly by Earth.
    pub fn leg_allowed(from: BodyId, to: BodyId) -> bool {
        let (a, b) = (Self::system_of(from), Self::system_of(to));
        !matches!((a, b), (1, 2) | (2, 1))
    }

    /// Ticket #93: the crossing a transit makes, with its offset from the window and the transfer
    /// table that prices it: Earth to Mars or back on the Mars sky, Earth to Venus or back on
    /// Venus's. `None` for a hop inside a system.
    pub fn crossing(&self, from: BodyId, to: BodyId, turn: u32) -> Option<(f64, &crate::data::TransitTable)> {
        let t = &self.tables;
        match (Self::system_of(from), Self::system_of(to)) {
            (0, 1) => Some((self.window_offset(turn), &t.transit)),
            (1, 0) => Some((self.return_window_offset(turn), &t.transit)),
            (0, 2) => Some((self.span_offset_of(BodyId::Venus, turn, t.transit_venus.hohmann_angle), &t.transit_venus)),
            (2, 0) => Some((self.span_offset_of(BodyId::Venus, turn, t.transit_venus.return_hohmann_angle), &t.transit_venus)),
            _ => None,
        }
    }

    /// Ticket #93: the turn Venus's window falls on, looked for from `from` forward over one of its
    /// synodic cycles.
    pub fn next_venus_window_turn(&self, from: u32) -> u32 {
        let tr = &self.tables.transit_venus;
        let cycle = (tr.synodic_days / tr.days_per_turn).ceil() as u32;
        let from = from.max(1);
        let off = |t: u32| self.span_offset_of(BodyId::Venus, t, tr.hohmann_angle).abs();
        (from..=from + cycle).min_by(|a, b| off(*a).partial_cmp(&off(*b)).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or(from)
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
        // Ticket #93: Venus has a window of its own.
        let window = if body == BodyId::Venus { self.next_venus_window_turn(self.turn) } else { self.next_window_turn(self.turn) };
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
        // Ticket #108: Leapfrog takes a bite out of it, and it never falls below nothing.
        (self.tables.state(s).baseline_emissions + self.state(s).baseline_rise - self.state(s).baseline_cut).max(0.0)
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
        if source != UnrestSource::Agitate {
            let done = self.green_techs_done();
            if done >= 4 {
                d += u.green_techs_four;
            } else if done >= 2 {
                d += u.green_techs_two;
            }
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
    /// Ticket #191 (version 0.08.0): mark that `offender` has crossed `victim` this turn. Charged per
    /// TURN, so a second offence in the same turn is free: a pair that offends puts in a median 15.6
    /// Influence across about three separate orders, and charging per order would make a big push and
    /// a small one differ by a factor nobody can read off the board.
    pub fn offend(&mut self, offender: Seat, victim: Seat) {
        self.offend_by(offender, victim, self.tables.relations.fall_per_offending_turn);
    }

    /// Ticket #222 (version 0.08.2): an offence with a WEIGHT. A turn charges the sum of every
    /// instance -- a place spent on, an act committed -- capped when the turn settles; an instance is
    /// never an ORDER, so splitting one spend across three orders cannot multiply the damage. That
    /// is what keeps the spirit of 0.08.0's per-turn charge while giving the designer the
    /// proportionality they asked for: a turn pushing on three of a rival's places genuinely costs
    /// three times a turn pushing on one.
    pub fn offend_by(&mut self, offender: Seat, victim: Seat, weight: i64) {
        if offender == victim || weight <= 0 {
            return;
        }
        // Ticket #226 (version 0.08.2): non-aggression is NOT enforced by refusing the order -- the
        // act is allowed and paid for. Acting against a term while your word still stands breaks the
        // whole Accord, which is what stops a Faction violating it every turn, paying 3 each time,
        // and keeping a permanent tenth of extra Research from a partner it is attacking.
        if self.accord_has(offender, victim, Term::NonAggression) {
            self.break_accord(offender, victim);
        }
        let (v, o) = (victim.index(), offender.index());
        self.relations.owed[v][o] += weight;
        self.relations.offended[v][o] = true;
    }

    /// Ticket #223 (version 0.08.2): `giver` did `receiver` a kindness this turn. One act a turn per
    /// ordered pair, whatever was done, so a rich Faction cannot buy a whole relationship in a turn.
    pub fn credit(&mut self, giver: Seat, receiver: Seat) {
        if giver != receiver {
            self.relations.credited[receiver.index()][giver.index()] = true;
        }
    }

    /// The DEEDS half alone: what this pair has done to each other, before Blame is read.
    pub fn relations_deeds(&self, viewer: Seat, subject: Seat) -> i64 {
        self.relations.score[viewer.index()][subject.index()]
    }

    /// Ticket #221 (version 0.08.2): what `viewer` thinks of `subject` -- the deeds plus the Blame
    /// term, clamped to the scale. This is the figure everything reads: the grid, the Report, the
    /// challenge margin and the Accords' Friendly gate.
    pub fn relations_score(&self, viewer: Seat, subject: Seat) -> i64 {
        let c = &self.tables.relations;
        let base = self.relations_deeds(viewer, subject) + self.blame_relations_term(viewer, subject);
        let pot = self.directive_relations_term(subject);
        // Ticket #236 (version 0.08.3): the reward may not lift a pair past the top of Cordial, the
        // step above Neutral. A pair already higher than that by deeds is not dragged DOWN to it --
        // the ceiling binds the boost, not the score.
        let with_pot = if pot > 0 { (base + pot).min(base.max(c.directive_boost_ceiling)) } else { base + pot };
        with_pot.clamp(c.worst, c.best)
    }

    /// Ticket #236 (version 0.08.3): what everyone else makes of how much of its Research `subject`
    /// gives the shared Tech. A TERM, like Blame's, read afresh every time rather than banked: a
    /// Faction that starts contributing again is forgiven the same turn, and one that stops is
    /// resented only while it does.
    ///
    /// It does not depend on the viewer. Blame's term reads a `resentment` coefficient per Faction
    /// because Factions care about pollution by different amounts; nothing in the designer's rule
    /// says they weigh generosity differently, and inventing a second coefficient would be a rule
    /// nobody asked for.
    pub fn directive_relations_term(&self, subject: Seat) -> i64 {
        let c = &self.tables.relations;
        let contribution = 100u8.saturating_sub(self.seat(subject).research_directive);
        if contribution >= 100 {
            c.directive_step
        } else if contribution < c.directive_min_contribution {
            -c.directive_step
        } else {
            0
        }
    }

    /// Ticket #221 (version 0.08.2): how much `viewer` holds `subject`'s Blame against them, as a
    /// LEVEL read afresh every settle rather than a charge added each turn. A Faction that cleans up
    /// is therefore forgiven without needing quiet turns.
    ///
    /// `floor((share - fair) / 0.10)` steps -- nothing below 0.35, one at 0.35, two at 0.45 -- times
    /// the RESENTING Faction's own coefficient. The coefficient belongs to the viewer and the share
    /// to the Faction being viewed, which is the whole asymmetry: the Custodians mind at twice the
    /// rate and the Prospectors do not mind at all.
    ///
    /// Measured over 200 games before it was built: the Prospectors sit above 0.35 from turn 1 and
    /// climb to about 0.53, while the Custodians never crossed 0.35 once. So this is very nearly a
    /// rule about how everyone feels about the Prospectors, and nobody ever resents the Custodians.
    /// Capped at `blame_cap` -- half the scale -- so Blame can make a pair Cold but never, by itself,
    /// Hostile: the last points are reserved for deeds.
    pub fn blame_relations_term(&self, viewer: Seat, subject: Seat) -> i64 {
        if viewer == subject {
            return 0;
        }
        let c = &self.tables.relations;
        let over = self.blame_share(subject) - self.tables.influence.blame.fair_share;
        if over < c.blame_step {
            return 0;
        }
        let steps = (over / c.blame_step).floor();
        let coefficient = self.tables.faction(self.kind(viewer)).resentment;
        // Toward zero, so the Arkwrights' x0.5 genuinely means "minds half as much" and does not
        // behave as x1 at the first step, which is the step most Factions ever reach.
        let raw = (steps * coefficient).trunc() as i64;
        -raw.min(c.blame_cap)
    }

    /// Ticket #226 (version 0.08.2): does an Accord standing between these two carry this Term?
    /// An Accord declared as ending carries nothing: it lapses at the next turn's start, and the
    /// turn's notice is what makes ending free where violating costs.
    pub fn accord_has(&self, x: Seat, y: Seat, term: Term) -> bool {
        self.accords.iter().any(|acc| acc.holds(x, y) && acc.terms.contains(&term))
    }

    /// Ticket #226: strike one. The Friendly gate on a research agreement is checked HERE and only
    /// here; once made it stands whatever the score later does.
    pub fn strike_accord(&mut self, a: Seat, b: Seat, terms: Vec<Term>) -> Result<(), String> {
        if a == b {
            return Err("a Faction cannot strike an Accord with itself".into());
        }
        if self.accords.iter().any(|acc| acc.holds(a, b)) {
            return Err("these two already hold an Accord".into());
        }
        if terms.contains(&Term::ResearchAgreement) && (self.relations_score(a, b) < 7 || self.relations_score(b, a) < 7) {
            return Err("a research agreement wants Friendly on both sides".into());
        }
        let turn = self.turn;
        self.accords.push(Accord { a, b, terms, struck: turn, paid: turn, ending: false });
        Ok(())
    }

    /// Ticket #226: declare it over. Free, and it lapses at the next turn's start.
    pub fn end_accord(&mut self, a: Seat, b: Seat) {
        if let Some(acc) = self.accords.iter_mut().find(|acc| acc.holds(a, b)) {
            acc.ending = true;
        }
    }

    /// Ticket #226: `breaker` acted against a term while their word still stood. The offence is
    /// weight 3 -- the same rung as opening a Battle -- and the whole Accord ends at once, so the +3
    /// is the lesser cost of betrayal and losing the arrangement is the real one.
    pub fn break_accord(&mut self, breaker: Seat, other: Seat) {
        if !self.accords.iter().any(|acc| acc.holds(breaker, other)) {
            return;
        }
        self.accords.retain(|acc| !acc.holds(breaker, other));
        self.offend_by(breaker, other, 3);
        let text = format!("{} broke their Accord with {}.", self.seat_name(breaker), self.seat_name(other));
        self.log(text);
    }

    /// Ticket #226 (version 0.08.2): run once a turn, beside the Relations settle.
    ///
    /// An Accord declared over lapses now -- the turn's notice is what makes ending free where
    /// violating costs +3 -- and one that has stood `accord_kept_turns` pays both sides their act.
    /// That act is also the only thing in the game that lifts a scarred pair's floor, one step at a
    /// time, which is what gives a wronged pair a road back.
    pub fn settle_accords(&mut self) {
        let turn = self.turn;
        let period = self.tables.relations.accord_kept_turns;
        let lapsed: Vec<(Seat, Seat)> = self.accords.iter().filter(|a| a.ending).map(|a| (a.a, a.b)).collect();
        for (a, b) in lapsed {
            self.log(format!("The Accord between {} and {} has lapsed.", self.seat_name(a), self.seat_name(b)));
        }
        self.accords.retain(|a| !a.ending);
        let mut paid: Vec<(Seat, Seat)> = Vec::new();
        for acc in self.accords.iter_mut() {
            if period > 0 && turn.saturating_sub(acc.paid) >= period {
                acc.paid = turn;
                paid.push((acc.a, acc.b));
            }
        }
        for (a, b) in paid {
            self.credit(a, b);
            self.credit(b, a);
            // The scar's only remedy: an Accord kept lifts the floor a step, on both sides.
            for (x, y) in [(a, b), (b, a)] {
                let f = &mut self.relations.floor[x.index()][y.index()];
                if *f < 0 {
                    *f += 1;
                }
            }
            self.log(format!("The Accord between {} and {} has held.", self.seat_name(a), self.seat_name(b)));
        }
    }

    /// Ticket #226 (version 0.08.2): would `seat` accept this offer from `from`?
    ///
    /// The arithmetic is the one `Candidate::score` already performs everywhere else: it accepts
    /// when what it gets exceeds what it gives, scaled by what it thinks of the offerer. Plus one
    /// hard floor -- it NEVER accepts a term that would lose it the game, so no non-aggression with
    /// a Faction one turn from its Victory Condition.
    ///
    /// This is the largest risk on the ticket and is recorded as such: a seat that accepts anything
    /// is exploitable, one that accepts nothing makes the whole system invisible. The sweep must
    /// report Accords struck, by term and by seat.
    pub fn accord_acceptable(&self, seat: Seat, from: Seat, terms: &[Term]) -> bool {
        // Never help somebody already at the door.
        if self.progress(from).score() >= 0.95 {
            return false;
        }
        let view = self.relations_score(seat, from);
        // Non-aggression from somebody it holds nothing against is easy; from somebody it loathes it
        // is not. A research agreement is its own gate and needs no opinion beyond Friendly.
        terms.iter().all(|t| match t {
            Term::ResearchAgreement => true,
            Term::NonAggression => view >= -2,
            Term::Passage | Term::Refuel => view >= -5,
        })
    }

    /// Ticket #226: a research agreement pays both parties a tenth more Research while it stands.
    pub fn research_agreement_multiplier(&self, seat: Seat) -> f64 {
        let any = Seat::ALL.into_iter().any(|other| other != seat && self.accord_has(seat, other, Term::ResearchAgreement));
        if any { 1.10 } else { 1.0 }
    }

    /// Ticket #221 (version 0.08.2): the named level a score reads as. The level is what a player
    /// reasons with; the number is the audit trail and rides on the hover.
    pub fn relations_level(&self, viewer: Seat, subject: Seat) -> &'static str {
        match self.relations_score(viewer, subject) {
            s if s >= 7 => "Friendly",
            s if s >= 3 => "Cordial",
            s if s >= -2 => "Neutral",
            s if s >= -5 => "Wary",
            s if s >= -8 => "Cold",
            _ => "Hostile",
        }
    }

    /// Ticket #191: charge the turn's offences, let the quiet pairs recover, and wipe the slate. Run
    /// once a turn, after the Resolution has recorded everything that happened.
    ///
    /// The recovery **stops at neutral** and never rises past it. The +10 half of the scale is
    /// reserved and nothing fills it in this version: letting peace accrue goodwill was measured and
    /// rejected, because half of all ordered pairs never interact at all in a whole game and the
    /// goodwill would mostly be between Factions on opposite sides of the board who have never met.
    pub fn settle_relations(&mut self) {
        let c = self.tables.relations.clone();
        for victim in Seat::ALL {
            for offender in Seat::ALL {
                if victim == offender {
                    continue;
                }
                let (v, o) = (victim.index(), offender.index());
                let was = self.relations.score[v][o];
                // Ticket #222: the turn's offences, summed over every instance and capped here --
                // the cap is a guard against one dramatic turn spending the whole scale, not a
                // working part of the rule; measured, it bites on 1.7% of offending pair-turns.
                let owed = self.relations.owed[v][o].min(c.turn_cap);
                // Ticket #223: an act and an offence in the same turn BOTH count, and the effect is
                // their sum -- a turn carrying a 3-point offence and a tribute nets -2.
                let earned = if self.relations.credited[v][o] { c.act_gain } else { 0 };
                if owed > 0 {
                    // Ticket #225: the scar counts TURNS, not points. Being raided once brutally and
                    // being ground down across fifteen turns are different things, and it is the
                    // second that "crossed often enough" describes.
                    self.relations.offending_turns[v][o] += 1;
                    if c.scar_turns > 0 && self.relations.offending_turns[v][o].is_multiple_of(c.scar_turns) {
                        self.relations.floor[v][o] = (self.relations.floor[v][o] - 1).max(c.scar_floor);
                    }
                }
                if owed > 0 || earned > 0 {
                    self.relations.quiet[v][o] = 0;
                    let moved = was - owed + earned;
                    // Only the SHOWN score clamps to the scale; the deeds figure may climb to
                    // `deeds_ceiling`, which is what lets a pair carrying a heavy Blame term still
                    // reach Friendly on deeds alone. Downward it stops at the scale's floor.
                    self.relations.score[v][o] = moved.clamp(c.worst, c.deeds_ceiling);
                } else {
                    let quiet = self.relations.quiet[v][o] + 1;
                    let period = if was > c.start { c.positive_quiet_turns } else { c.quiet_turns };
                    if period > 0 && quiet >= period {
                        self.relations.quiet[v][o] = 0;
                        // Below neutral a quiet pair RECOVERS toward it; above neutral it DECAYS
                        // toward it, at half the rate. Friendship that never lapses would mean four
                        // early acts fixing a pair for the whole game; friendship lapsing as fast as
                        // enmity would not be worth earning.
                        if was < c.start {
                            self.relations.score[v][o] = (was + c.recover).min(c.start);
                        } else if was > c.start {
                            self.relations.score[v][o] = (was - c.recover).max(c.start);
                        }
                    } else {
                        self.relations.quiet[v][o] = quiet;
                    }
                }
                // Ticket #225: the scar caps the DEEDS figure, so a pair crossed often enough can
                // never recover above its floor however long it stays quiet.
                self.relations.score[v][o] = self.relations.score[v][o].min(self.relations.floor[v][o]);
                self.relations.fell[v][o] = self.relations.score[v][o] < was;
                self.relations.owed[v][o] = 0;
                self.relations.offended[v][o] = false;
                self.relations.credited[v][o] = false;
            }
        }
        // The Report line goes in the OFFENDER's paragraph: it is what sends a player to the grid.
        for victim in Seat::ALL {
            for offender in Seat::ALL {
                if victim == offender || !self.relations.fell[victim.index()][offender.index()] {
                    continue;
                }
                let text = self.say("relations_fell", &[("victim", self.seat_name(victim)), ("offender", self.seat_name(offender))]);
                self.log(text.clone());
                self.report_line_of(offender, LineKind::YourWorks, LineKind::Note, None, text);
            }
        }
    }

    pub fn report_line_of(&mut self, seat: Seat, mine: LineKind, theirs: LineKind, place: Option<ReportPlace>, text: String) {
        let kind = crate::report::line_kind_of(seat, mine, theirs, self.spectator);
        self.report_line(kind, place, text);
    }

    /// Add a Moment the turn may stop for. The cap of two and the switches are applied when the
    /// Report is shown, so every Moment a turn earned is kept here.
    pub fn moment(&mut self, kind: MomentKind, args: &[(&str, String)], place: Option<ReportPlace>) {
        let Some(card) = self.tables.report.moment(kind) else { return };
        let (text, figure) = (crate::report::render(&card.text, args), crate::report::render(&card.figure, args));
        self.report.moments.push(Moment { kind, text, figure, place, tech: None, note: None, seat: None });
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

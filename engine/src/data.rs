//! The data tables under `assets/data`, one TOML file per table (spec 2.3).
//! Loaded once at startup and validated; a malformed table refuses to start,
//! naming the file and the row.

use crate::ids::*;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct DataError {
    pub file: String,
    pub message: String,
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.file, self.message)
    }
}

impl std::error::Error for DataError {}

#[derive(Debug, Clone, Deserialize)]
pub struct Produces {
    pub resource: Resource,
    pub amount: i64,
}

/// A Colony Slot: a real place on its Body, at its approximate longitude and latitude (ticket #45).
#[derive(Debug, Clone, Deserialize)]
pub struct SlotCard {
    pub name: String,
    pub lon: f32,
    pub lat: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BodyCard {
    pub id: BodyId,
    pub name: String,
    #[serde(default)]
    pub slots: Vec<SlotCard>,
    /// Ticket #46: orbital slots for stations, and the station names by slot.
    #[serde(default)]
    pub orbital_slots: u32,
    #[serde(default)]
    pub stations: Vec<String>,
    /// Ticket #45: the Body this one orbits, and the hop between them.
    #[serde(default)]
    pub parent: Option<BodyId>,
    #[serde(default)]
    pub local_turns: u32,
    #[serde(default)]
    pub local_fuel: i64,
    pub transit_turns: u32,
    pub transit_fuel: i64,
    /// Ticket #92 (version 0.06.0): a small world, where a Mass Driver may stand.
    #[serde(default)]
    pub low_gravity: bool,
    pub mine_yield: f64,
    pub generator_yield: f64,
    pub refinery_yield: f64,
    /// Ticket #140 (version 0.07.3): the fourth yield is Research, multiplying an Observatory; it
    /// was the Habitat yield, which multiplied a Habitat's room, until the designer traded it.
    pub research_yield: f64,
}

impl BodyCard {
    pub fn colony_slots(&self) -> u32 {
        self.slots.len() as u32
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StateCard {
    pub id: StateId,
    pub name: String,
    /// Ticket #270 (version 0.08.4): the adjective its Armies are named by -- "the 2nd Chinese Army".
    pub demonym: String,
    /// Version 0.07.2 (ticket #122): the two-letter code of the Nation's flag, a file in
    /// `assets/flags/`. Empty where a Region has none, and the card then draws no flag.
    #[serde(default)]
    pub flag: String,
    /// Version 0.07.2 (ticket #126): the Region's own colour, worn on the globe while nobody holds
    /// it and on the start screen, where nothing is held yet. Zero where a card has none, and the
    /// composer then leaves the photograph bare, as it did before this version.
    #[serde(default)]
    pub colour: [f32; 3],
    pub population: f64,
    pub industry_level: u32,
    pub resource_lean: Resource,
    pub baseline_emissions: f64,
    pub education_level: f64,
    pub size: u32,
    pub coastal_exposure: u32,
    pub neighbours: Vec<StateId>,
    /// What stands when the game begins (ticket #24); comes with the state whoever takes it.
    #[serde(default)]
    pub start_facilities: Vec<FacilityKind>,
    /// What the state adds to its controller's Influence Allotment each turn (ticket #34).
    #[serde(default)]
    pub influence: i64,
    /// World GDP share in tenths of a percent-ish weight (ticket #35): a controlled state makes
    /// gdp x Industry Level / 10 Ducats a turn, and a Bank there adds 4 x gdp / 10.
    #[serde(default)]
    pub gdp: i64,
    /// Ticket #52 (version 0.05): the Unrest the state starts with, 0 to 10.
    #[serde(default)]
    pub unrest: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FacilityCard {
    pub id: FacilityKind,
    pub name: String,
    pub materials: i64,
    /// Ticket #332 (version 0.09.0): the Widgets the build needs, four for every turn it took
    /// under the flat count this replaces. It completes at the Resolution its place's Widgets
    /// reach it, so a place making four a turn builds it at the old pace and a busier one slower.
    pub widgets: u32,
    pub energy_upkeep: i64,
    pub produces: Option<Produces>,
    /// Ticket #280 (version 0.08.5): what the building does, in a sentence, where it is not a
    /// resource or beside one; drawn where the row said "no output" before.
    #[serde(default)]
    pub does: Option<String>,
    pub emissions: f64,
    /// Ticket #36: what it adds to its controller's Allotment while it stands and is online.
    #[serde(default)]
    pub influence_allotment: i64,
    /// Ticket #36: how much its place's standing for its controller rises each turn.
    #[serde(default)]
    pub standing_per_turn: i64,
    /// Ticket #54: ppm this Facility adds to the Natural Sink each Climate phase while it is online
    /// (the Scrubber; 0.0 for every other row).
    #[serde(default)]
    pub sink_per_turn: f64,
    /// Ticket #54: true for a Facility that occupies no build slot (the Scrubber).
    #[serde(default)]
    pub no_slot: bool,
    /// Ticket #56: the Tech that unlocks this Facility, if any (the Sea Wall's Coastal Engineering).
    #[serde(default)]
    pub needs_tech: Option<TechId>,
    /// Ticket #56: true for a Facility that stands only in a coastal slot (the Sea Wall).
    #[serde(default)]
    pub coastal_only: bool,
}

/// Ticket #54 (version 0.05): the cap on Scrubbers in one Nation State (`facilities.toml`).
#[derive(Debug, Clone, Deserialize)]
pub struct ScrubberCard {
    pub per_population: f64,
    pub min: u32,
    pub max: u32,
}

/// Ticket #333 (version 0.09.0): what a Region's people are worth to a Research Lab, one point of
/// bonus per this many units of population (`facilities.toml`); a code literal of 1,000 units of
/// five million from ticket #143 (version 0.07.3) until this ticket, the same five billion people.
#[derive(Debug, Clone, Deserialize)]
pub struct PopulationFactorCard {
    pub population_per_point: f64,
}

/// Ticket #257 (version 0.08.4): what each Sea Level rise a Sea Wall has held back adds to its
/// keep, in Materials a turn (`facilities.toml`).
#[derive(Debug, Clone, Deserialize)]
pub struct SeaWallCard {
    pub upkeep_per_rise: f64,
}

/// Ticket #187 (version 0.08.0): the band a place's Education Level bends its Resistance through,
/// and the two anchors it is measured between (`influence.toml`).
#[derive(Debug, Clone, Deserialize)]
pub struct ResistanceCard {
    pub band: f64,
    pub pivot: f64,
    pub low: f64,
    pub high: f64,
}

/// Ticket #185 (version 0.08.0): what a School does to its state's Education Level, a step a turn
/// to a ceiling (`facilities.toml`). No maximum Education Level was ever declared -- the card
/// figures simply run 0.70 to 1.50 -- so the ceiling is this table's, and the step is the smallest
/// that reliably shows in a Lab's floored Research.
#[derive(Debug, Clone, Deserialize)]
pub struct SchoolCard {
    pub per_turn: f64,
    pub ceiling: f64,
}

/// Tickets #182, #184 and #186 (version 0.08.0): the figures the Unique Facility clauses read
/// (`facilities.toml`). The Spaceport has none: +1 Influence per Emigrant launched is the whole of
/// its clause and the designer refused a cap on it.
#[derive(Debug, Clone, Deserialize)]
pub struct UniqueCard {
    /// The share of the Venture Capital Fund an Investment Bank banks back into it, one per Region.
    pub investment_bank_interest: f64,
    /// The floor under that interest, Faction-wide and on one building only.
    pub investment_bank_floor: i64,
    /// What a Reactor's holder pays of its total Energy upkeep, the Archive excepted; off the total.
    pub reactor_upkeep: f64,
    /// What an Academy pays its holder a turn, flat, wherever it stands.
    pub academy_ducats: i64,
    /// Ticket #239 (version 0.08.3): what a Heliostat makes over a Solar Array, after sun scaling.
    pub heliostat_energy: i64,
    /// Ticket #239 (version 0.08.3): what an Exchange pays over a Trade Post, flat, after the
    /// output multiplier.
    pub exchange_ducats: i64,
    /// Ticket #239 (version 0.08.3): how many Colonists at a Chorus's own Colony buy it one more
    /// Influence in its holder's Allotment, rounded down.
    pub chorus_colonists: i64,
}

/// Ticket #54 (version 0.05): what a Restart and a Decommission cost (`facilities.toml`). A
/// Mothball is free and lands at the Resolution of the turn it is ordered.
#[derive(Debug, Clone, Deserialize)]
pub struct MothballCard {
    pub restart_materials: i64,
    pub restart_turns: u32,
    pub decommission_turns: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IndustryLevelCard {
    pub materials: i64,
    pub materials_cheap_industry: i64,
    /// Ticket #332 (version 0.09.0): the Widgets a raise needs, in place of its flat turn.
    pub widgets: u32,
}

/// Ticket #332 (version 0.09.0): the Widgets a Region makes a turn with no Factory at all: a flat
/// `region_base` and `per_industry_level` more per point of Industry Level (`facilities.toml`), at
/// the designer's word *"4 +1 per industry level"*. A Colony's base is its Core Module's own row.
#[derive(Debug, Clone, Deserialize)]
pub struct WidgetsCard {
    pub region_base: u32,
    pub per_industry_level: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModuleCard {
    pub id: ModuleKind,
    pub name: String,
    pub materials: i64,
    /// Ticket #332 (version 0.09.0): the Widgets the build needs, four for every former turn.
    pub widgets: u32,
    pub energy_upkeep: i64,
    pub produces: Option<Produces>,
    /// Ticket #280 (version 0.08.5): what the Module does, in a sentence, where it is not a
    /// resource or beside one.
    #[serde(default)]
    pub does: Option<String>,
    #[serde(default)]
    pub holds_colonists: u32,
    /// Ticket #324 (version 0.08.8): the Battery's figures in the orbital Battle, nought for every
    /// other Module. Hardened Hulls does not reach them; a Module is not a hull.
    #[serde(default)]
    pub strength: i64,
    #[serde(default)]
    pub hit_points: u32,
    #[serde(default)]
    pub influence_allotment: i64,
    #[serde(default)]
    pub standing_per_turn: i64,
    /// Version 0.04 (ticket #44): what the Module emits on Earth, as its counterpart Facility does.
    #[serde(default)]
    pub earth_emissions: f64,
    /// Ticket #89 (version 0.06.0): only a Space Station holds it.
    #[serde(default)]
    pub station_only: bool,
    /// Ticket #89: its output scales with the inverse square of its Body's mean distance from the
    /// Sun instead of a Body yield, and a Solar Storm turn silences it.
    #[serde(default)]
    pub sun_scaled: bool,
    /// Ticket #92 (version 0.06.0): the Tech that must stand before it can be built, if any.
    #[serde(default)]
    pub needs_tech: Option<TechId>,
    /// Ticket #92: only a ground Colony on a low-gravity Body holds it.
    #[serde(default)]
    pub low_gravity_only: bool,
}

/// Ticket #92 (version 0.06.0): the Mass Driver's figures: the Fuel it takes off the owner's
/// departures (never below the minimum) and what each Mine at its Colony makes more.
#[derive(Debug, Clone, Deserialize)]
pub struct MassDriverCard {
    pub fuel_off: i64,
    pub fuel_min: i64,
    pub mine_bonus: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnitCard {
    pub id: UnitKind,
    pub name: String,
    pub materials: i64,
    /// Ticket #332 (version 0.09.0): the Widgets the build needs, four for every former turn.
    pub widgets: u32,
    pub energy_upkeep: i64,
    pub strength: i64,
    pub hit_points: u32,
    pub pursuit: u32,
    /// Ticket #87 (version 0.06.0): the Fuel a Ship of this type carries; an Army none.
    #[serde(default)]
    pub tank: i64,
    pub carries_colonists: u32,
    pub carries_army: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RepairCard {
    pub materials_per_point: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TechCard {
    pub id: TechId,
    pub name: String,
    pub branch: String,
    pub rung: u32,
    pub cost: i64,
    pub needs: Vec<TechId>,
    pub effect: String,
    pub value: f64,
    /// Ticket #207 (version 0.08.1): Expanded Habitats is read in TWO places -- what a Habitat
    /// holds and what a Colony Ship carries -- and until now one `value` served both, so neither
    /// could be moved without the other. This is the Habitat clause where a Tech wants its own
    /// figure for it; `value` stays the Colony Ship's. Absent, `value` answers for both, which is
    /// every other Tech in the tree.
    #[serde(default)]
    pub habitat_colonists: Option<f64>,
    #[serde(default)]
    pub influence_threshold_multiplier: Option<f64>,
    /// Ticket #84 (version 0.06.0): the Faction whose Victory Condition this Tech opens, if any.
    #[serde(default)]
    pub gate_for: Option<FactionKind>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EventCard {
    pub id: EventId,
    pub name: String,
    pub kind: EventKind,
    pub target: String,
    pub effect: String,
    #[serde(default)]
    pub blunted_by: Option<TechId>,
    /// How many copies sit in the deck (ticket #25).
    #[serde(default = "one")]
    pub copies: u32,
    /// Ticket #259 (version 0.08.4): a card that can only land off Earth. Out of the deck at the
    /// start, joining it on `off_earth_join_turn`.
    #[serde(default)]
    pub off_earth: bool,
}

fn one() -> u32 {
    1
}

#[derive(Debug, Clone, Deserialize)]
pub struct EventsTable {
    /// Ticket #25: no Calm Cards; each turn a card is drawn with this chance at the base Temperature,
    /// rising by `draw_chance_per_step` for every full `draw_chance_step_degrees` above it.
    pub draw_chance_base: f64,
    pub draw_chance_per_step: f64,
    pub draw_chance_step_degrees: f64,
    pub solar_maximum_multiplier: f64,
    pub solar_maximum_multiplier_with_tech: f64,
    pub methane_emissions: f64,
    pub meteor_damage: u32,
    /// Ticket #52: the Unrest card is a flat rise in the state's Unrest; the Army damage and the
    /// Standing loss it carried until version 0.05 are gone.
    pub unrest_card_unrest: f64,
    pub reactor_leak_energy: i64,
    pub breakthrough_research: i64,
    pub breakthrough_research_public_science: i64,
    pub discovery_multiplier: f64,
    pub discovery_multiplier_with_tech: f64,
    pub discovery_turns: u32,
    pub heatwave_loss: f64,
    pub heatwave_loss_green_consensus: f64,
    pub wildfire_emissions: f64,
    /// Ticket #76 (version 0.05.5): Drought, Volcanic Eruption; Helium-3 Vein reads the discovery figures.
    pub drought_output_multiplier: f64,
    pub drought_unrest: f64,
    pub volcanic_co2: f64,
    /// Ticket #257 (version 0.08.4): what a Storm Surge does to a state whose Sea Wall holds --
    /// the Facilities in its coastal slots make this much of their output at the next Income.
    pub storm_surge_coastal_multiplier: f64,
    /// Ticket #259: the turn the off-Earth cards are shuffled into the deck.
    pub off_earth_join_turn: u32,
    pub event: Vec<EventCard>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionCard {
    pub id: FactionKind,
    pub name: String,
    pub blurb: String,
    /// Ticket #260 (version 0.08.4): what the Faction says of itself, one line, on the card under
    /// its name. The blurb says what it does; the motto is the Faction speaking.
    #[serde(default)]
    pub motto: String,
    /// Ticket #210 (version 0.08.1): the prefix every Ship of this Faction wears in front of its
    /// name. It belongs to the HOLDER, not the hull -- a name travels with the ship, a prefix with
    /// whoever flies it -- so it is read from the seat at drawing time and never stored on the Ship.
    pub ship_prefix: String,
    pub output_multiplier: f64,
    pub emissions_multiplier: f64,
    pub research_multiplier: f64,
    /// Ticket #81 (version 0.06.0): the Research multiplier for an Observatory off Earth (a station
    /// over Earth included, Antarctica not); absent, the one figure serves both.
    #[serde(default)]
    pub research_multiplier_off_earth: Option<f64>,
    pub influence_multiplier: f64,
    /// Ticket #82 (version 0.06.0): the Custodians' Production Moved. While a Facility of the key's
    /// kind in a state they direct is mothballed, one Module of the value's kind off Earth makes
    /// double. Empty on every other card.
    #[serde(default)]
    pub mothball_pairs: std::collections::BTreeMap<FacilityKind, ModuleKind>,
    pub signature: String,
    /// Ticket #203 (version 0.08.1): the Faction's Unique Facility in one sentence -- what it is,
    /// what it replaces, and what it does beyond the common building's job. It was on no card until
    /// the Faction window went in: the four Unique Facilities arrived in version 0.08.0 (tickets
    /// #182 to #186) and the signature rules were never rewritten to name them, so a player could
    /// build one without ever being told what it was for.
    pub unique: String,
    /// The Victory Condition in prose, for the cards and the panel.
    pub victory: String,
    /// Ticket #50: the first part of the Victory Condition, in figures.
    pub victory_first: VictoryFirstCard,
    /// Ticket #51: the second part, generalised the way #50 generalised the first.
    pub victory_second: VictorySecondCard,
    pub colour: [f32; 3],
    /// Ticket #100 (version 0.07.0), renamed on ticket #195 (version 0.08.0): the Region the start
    /// screen opens its globe on. It has ONE reader, and its only job is pointing the start globe's
    /// camera, which the old name `home` did not say -- it read as a starting position, which it has
    /// never been: any of the fourteen may still be chosen.
    pub opens_on: StateId,
    /// Ticket #46: the station over Earth the Faction starts with, by name in bodies.toml.
    /// Ticket #50: the Arkwrights start with none, so this is optional.
    #[serde(default)]
    pub start_station: Option<String>,
    /// Ticket #290 (version 0.08.6): the Colonists aboard that station when the game opens, from
    /// nowhere -- no Region is debited for them. Two, so a starting station has two Module slots
    /// free at once where a bare one had none. Nothing without a `start_station`.
    #[serde(default)]
    pub start_colonists: u32,
    /// Ticket #290 (version 0.08.6): Pioneers waiting in the Faction's start Region on turn one, a
    /// gift outside the recruit rate that takes no population. The Arkwrights' two, since they have
    /// no station for two Colonists to be aboard. Spelled `emigrants` as the engine spells the
    /// field it fills (see **Pioneer** in `CONTEXT.md`).
    #[serde(default)]
    pub start_emigrants: u32,
    // Ticket #51: the per-Faction figures the Arkwrights' card carries. Every one is neutral by
    // default, so a card that names none plays exactly as it did before.
    /// What a Habitat here holds, times this.
    #[serde(default = "one_f64")]
    pub habitat_capacity_multiplier: f64,
    /// Transit Fuel times this, before Efficient Transit.
    #[serde(default = "one_f64")]
    pub transit_fuel_multiplier: f64,
    /// Coach Class: what a Colony Ship carries, times this.
    #[serde(default = "one_f64")]
    pub colony_ship_capacity_multiplier: f64,
    /// Coach Class: the population a lift from a Launch Site takes, times this.
    #[serde(default = "one_f64")]
    pub lift_population_multiplier: f64,
    /// Coach Class: what a Colony Ship costs, in place of the units.toml figure.
    #[serde(default)]
    pub colony_ship_materials: Option<i64>,
    /// Ticket #83 (version 0.06.0): every Ship's Materials, times this, rounded down (the
    /// Arkwrights' 0.85).
    #[serde(default = "one_f64")]
    pub ship_materials_multiplier: f64,
    /// Ticket #83: a controlled Nation State's own GDP income, times this, rounded down (the
    /// Prospectors' 1.2). Banks and Trade Posts take the general output multiplier instead.
    #[serde(default = "one_f64")]
    pub ducats_multiplier: f64,
    /// Ticket #83: what the Trading window charges this Faction for a lot of Materials, Fuel or
    /// Energy, or a building bought outright, times this, rounded down (the Prospectors' 0.85).
    /// Influence and selling are untouched.
    #[serde(default = "one_f64")]
    pub market_multiplier: f64,
    /// Ticket #221 (version 0.08.2): how much THIS Faction minds another's Blame -- the coefficient
    /// on the resentment step. The Custodians at 2 mind twice as much as the Archivists at 1; the
    /// Arkwrights at 0.5 mind half; the Prospectors at 0 do not mind at all, which is a statement
    /// about them and not an oversight.
    #[serde(default = "resentment_default")]
    pub resentment: f64,
    /// A Space Station's Materials, times this.
    #[serde(default = "one_f64")]
    pub station_materials_multiplier: f64,
    /// A Colony Module's Materials, times this.
    #[serde(default = "one_f64")]
    pub module_materials_multiplier: f64,
    /// Ticket #72 (version 0.05.5): a Facility's Materials, times this (the Prospectors' 0.85).
    #[serde(default = "one_f64")]
    pub facility_materials_multiplier: f64,
    /// Ticket #73 (version 0.05.5): Emigrants mustered a turn, times this (Coach Class's 2.0).
    #[serde(default = "one_f64")]
    pub emigrants_multiplier: f64,
}

fn one_f64() -> f64 {
    1.0
}

/// Ticket #50: which measure a Faction's first Victory part counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VictoryFirstKind {
    /// Ticket #72 (version 0.05.5): Materials banked in the Prospectors' Venture Capital Fund; the
    /// running Extraction Total it replaces is retired.
    VentureFund,
    StabilizationRun,
    ColonistsOffEarth,
    ResearchProduced,
    /// Ticket #51: the Archive. Ticket #68 (version 0.05.5): counted as the Research paid into it,
    /// which the fund holds only a quarter of until the Module stands; the bar only tells while the
    /// Archive is running.
    ArchiveResearch,
}

impl VictoryFirstKind {
    pub fn name(self) -> &'static str {
        match self {
            VictoryFirstKind::VentureFund => "Venture Capital Fund",
            VictoryFirstKind::StabilizationRun => "Stabilization run",
            VictoryFirstKind::ColonistsOffEarth => "Colonists off Earth",
            VictoryFirstKind::ResearchProduced => "Research produced",
            VictoryFirstKind::ArchiveResearch => "The Archive",
        }
    }
}

/// Ticket #51: which measure a Faction's second Victory part counts. Off-world Presence was the
/// only one until now; the Arkwrights count Bodies settled, the Archivists Colonists at the Archive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VictorySecondKind {
    OffWorldPresence,
    ColoniesOnBodies,
    /// Ticket #192 (version 0.08.0): Colonists UPLOADED into the Archive, replacing "Colonists living
    /// at the Archive's Colony". The count is monotonic -- uploaded people cannot be lost to a raid,
    /// a crowding death or a handover -- and it closes the odd case the old wording allowed, where a
    /// Faction won by having twelve people standing NEXT TO a finished Archive rather than inside it.
    ColonistsUploaded,
}

impl VictorySecondKind {
    pub fn name(self) -> &'static str {
        match self {
            VictorySecondKind::OffWorldPresence => "Off-world Presence",
            VictorySecondKind::ColoniesOnBodies => "Bodies settled",
            VictorySecondKind::ColonistsUploaded => "Colonists uploaded",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct VictorySecondCard {
    pub kind: VictorySecondKind,
    /// For off_world_presence and colonists_uploaded.
    #[serde(default)]
    pub bar: f64,
    /// For colonies_on_bodies: how many Bodies, and how many Colonists on each.
    #[serde(default)]
    pub bodies: u32,
    #[serde(default)]
    pub colonists_each: u32,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct VictoryFirstCard {
    pub kind: VictoryFirstKind,
    pub bar: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StartCard {
    pub materials: i64,
    pub fuel: i64,
    pub energy: i64,
    pub research: i64,
    #[serde(default)]
    pub ducats: i64,
}

/// Ticket #35: what Ducats buy.
#[derive(Debug, Clone, Deserialize)]
pub struct ResearchDirectiveCard {
    pub max: u8,
    pub archivists_max: u8,
    pub provisional_min_contribution: u8,
    pub custodians_ppm_per_point: f64,
    pub prospectors_ducats_per_point: f64,
    pub arkwrights_fuel_per_point: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DucatsCard {
    pub per_influence: i64,
    /// Ticket #54: what one Leapfrog costs the Custodians; it replaced `per_restoration_step`.
    pub per_leapfrog: i64,
    pub per_exodus_call: i64,
    pub per_repair_point: i64,
    /// Version 0.04 (ticket #42): the trading window's prices.
    pub per_materials: i64,
    pub per_fuel: i64,
    pub per_energy: i64,
    pub sell_divisor: i64,
    pub per_building_material: i64,
    pub bank_per_gdp_tenth: f64,
    /// Ticket #220 (version 0.08.2): how far a price may wander either side of the card figure,
    /// which is now the MIDPOINT of its band rather than a constant. At 1 every price stays a whole
    /// number and no resource can become free, which is why all three base prices rose by one to
    /// make room -- Materials 2 to 3, Fuel 3 to 4, Energy 1 to 2. Kept optional so an old table
    /// still loads, as `trade_post_base` is.
    #[serde(default = "price_band_default")]
    pub price_band: i64,
    /// Ticket #220: the NET units of one resource, bought less sold across the whole table in a
    /// turn, that move its price one step. 20 is a measured figure: a trading turn carries a median
    /// 20 units across all four seats, so a normal turn's trading is visible without pegging the
    /// price at its band's edge.
    #[serde(default = "price_step_default")]
    pub price_step_units: i64,
    /// Ticket #90 (version 0.06.0): retired; kept optional so an old table still loads.
    #[serde(default)]
    pub trade_post_base: Option<f64>,
}

/// Ticket #55 (version 0.05): what one Break does when it fires. The figures each kind reads sit
/// beside it on the same `[[break]]` row, so the designer can move a Break or add one in the table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BreakEffect {
    /// Every Nation State at `exposure` Coastal Exposure takes `unrest` Unrest, damped like any
    /// climate rise, and loses `population_loss` of its people, who flow as refugees. No climate effect.
    CoastalUnrest,
    /// `emissions` ppm join the world's Emissions in this and every later Climate phase, on their
    /// own line: nobody's Blame, and never counted against a Stabilization run.
    EmissionsPerTurn,
    /// The Natural Sink falls to `sink_after` for good. This one does bear on Stabilization: the
    /// Sink is the bar the run is measured against.
    WeakenSink,
    /// One Sea Level threshold's slot loss, displacement and Unrest lands at once on every state,
    /// out of sequence. The scheduled thresholds still fire on their own turns.
    SeaLevelThreshold,
    /// `co2` ppm join the CO2 Stock once, and `baseline_rise` is added to `state`'s Baseline
    /// Emissions for good. Both are the world's doing: nobody's Blame, exempt from Stabilization.
    CarbonPulse,
}

/// Ticket #55: one Break — a Temperature at which a permanent change fires once.
#[derive(Debug, Clone, Deserialize)]
pub struct BreakCard {
    pub id: String,
    pub name: String,
    /// It fires in the first Climate phase whose Temperature stands at or above this.
    pub temperature: f64,
    pub effect: BreakEffect,
    /// What the Report says happened, in a sentence. It says it happened, never that it is coming.
    pub happened: String,
    /// The card line: one line of prose under the sentence.
    pub text: String,
    #[serde(default)]
    pub exposure: u32,
    #[serde(default)]
    pub unrest: f64,
    #[serde(default)]
    pub population_loss: f64,
    #[serde(default)]
    pub emissions: f64,
    #[serde(default)]
    pub sink_after: f64,
    #[serde(default)]
    pub co2: f64,
    #[serde(default)]
    pub baseline_rise: f64,
    #[serde(default)]
    pub state: Option<StateId>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClimateTable {
    pub starting_co2: f64,
    pub base_temperature: f64,
    pub degrees_per_ppm_step: f64,
    pub ppm_step: f64,
    pub natural_sink: f64,
    pub collapse_line: f64,
    pub temperature_lag_fraction: f64,
    pub population_growth: f64,
    /// Ticket #333 (version 0.09.0): the population figure's unit, in people. A Region's figure, a
    /// Colonist and a Pioneer are all counted in it, so `Region population 380.0` is 380 million
    /// people and one Colonist is one million. A code constant of five million from ticket #143
    /// (version 0.07.3) until this ticket; every per-unit rate in the data is in this unit.
    pub people_per_unit: f64,
    pub population_loss_per_tenth_degree: f64,
    /// Ticket #54: Population Emissions per unit are `base + per_level x Industry Level`,
    /// less the state's own Leapfrog adjustment, never below `base`. Quoted to the player per
    /// hundred million people (`Tables::units_per_hundred_million`).
    /// Ticket #108: what one Leapfrog takes off a Nation State's Baseline Emissions.
    pub leapfrog_baseline_cut: f64,
    pub population_emissions_base: f64,
    pub population_emissions_per_level: f64,
    pub launch_emissions: f64,
    /// Ticket #279 (version 0.08.5): what a Battle on Earth puts in the air, per hit landed and
    /// per building burned in the rolls after one.
    pub war_ppm_per_hit: f64,
    pub war_ppm_per_building: f64,
    pub sea_level_thresholds: Vec<f64>,
    /// Ticket #55: the Temperature at which Antarctica's Colony Slots open. The sea-level ticket
    /// will read it; the Climate Panel's Temperature bar draws its notch here already.
    pub antarctica_opens_at: f64,
    /// Ticket #55: the Breaks, in rising Temperature order (`[[break]]` in `climate.toml`).
    #[serde(rename = "break", default)]
    pub breaks: Vec<BreakCard>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InfluenceTable {
    pub allotment_base: i64,
    pub state_threshold_base: i64,
    pub state_threshold_per_size: i64,
    pub colony_threshold_per_colonist: i64,
    pub decay: i64,
    /// Ticket #33: decay on a place the Faction controls.
    pub decay_controlled: i64,
    /// Version 0.04 (ticket #41): a challenger needs the controller's standing plus this.
    pub challenge_margin: i64,
    /// Ticket #224 (version 0.08.2): the most the Relations term may add to a challenge margin. Two,
    /// which is also its true maximum: the shown score clamps at -10 and the term is `|score| / 4`.
    #[serde(default = "relations_margin_cap_default")]
    pub relations_margin_cap: i64,
    /// Ticket #190 (version 0.08.0): what an online Constabulary adds to the margin in its Region.
    pub constabulary_margin: i64,
    /// Ticket #201 (version 0.08.1): what a Constabulary adds instead, once Civil Defense stands.
    pub constabulary_margin_defended: i64,
    /// Ticket #46: a station's threshold starts here.
    #[serde(default)]
    pub station_threshold_base: i64,
    pub occupation_turns: u32,
    pub destruction_chance: f64,
    /// Ticket #187 (version 0.08.0): how far a place's schooling bends what an outsider's Influence
    /// buys there. See `influence.toml`.
    pub resistance: ResistanceCard,
    /// Ticket #53: what a Faction's share of the table's Blame does to its Influence thresholds.
    pub blame: BlameTable,
    /// Ticket #267: the Smear campaign's rate.
    pub smear: SmearTable,
    /// Ticket #277 (version 0.08.5): the Greenwash's rate and price.
    pub greenwash: GreenwashTable,
}

/// Ticket #267 (version 0.08.4): the Smear campaign, in `influence.toml` under `[smear]`.
#[derive(Debug, Clone, Deserialize)]
pub struct SmearTable {
    /// ppm laid on the target's Blame ledger per Influence spent.
    pub ppm_per_influence: f64,
}

/// Ticket #277 (version 0.08.5): the Greenwash, in `influence.toml` under `[greenwash]`.
#[derive(Debug, Clone, Deserialize)]
pub struct GreenwashTable {
    /// ppm taken off the seat's own Blame ledger per Influence spent.
    pub ppm_per_influence: f64,
    /// Ducats charged beside every Influence spent.
    pub ducats_per_influence: i64,
    /// The Ducats a computer seat keeps back past the campaign's price before it will pay for one.
    pub ai_ducats_reserve: i64,
}

/// Ticket #53 (version 0.05): Blame, in `influence.toml` under `[blame]`.
#[derive(Debug, Clone, Deserialize)]
pub struct BlameTable {
    /// The share of the table's Blame that costs a Faction nothing: an even quarter of four seats.
    pub fair_share: f64,
    /// The most a Faction's thresholds can be multiplied by, however dirty it is.
    pub cap: f64,
    /// Ticket #266 (version 0.08.4): Blame moderates the decay of a Standing on a Region the
    /// Faction does not hold -- a step rule, at the designer's word. A share at or below
    /// `decay_slow_below` decays `decay_slow` a turn; at or above `decay_fast_from`, `decay_fast`;
    /// between, the plain `decay`. Never on a Colony or a station, never on a held place.
    pub decay_slow_below: f64,
    pub decay_fast_from: f64,
    pub decay_slow: i64,
    pub decay_fast: i64,
}

/// Ticket #52 (version 0.05): every number that moves a Nation State's Unrest (`unrest.toml`).
#[derive(Debug, Clone, Deserialize)]
pub struct UnrestTable {
    /// Ticket #269 (version 0.08.4): Agitate's price in Ducats and Influence, and what it adds.
    pub agitate_ducats: i64,
    pub agitate_influence: i64,
    pub agitate_points: f64,
    pub max: f64,
    pub neutral_max: f64,
    pub population_fall: f64,
    pub population_fall_big: f64,
    pub population_fall_big_fraction: f64,
    pub per_sea_level_slot: f64,
    pub climate_card: f64,
    pub per_mothball: f64,
    pub per_decommission: f64,
    pub refugees_per: f64,
    pub refugees_max: f64,
    /// Ticket #176 (version 0.07.6): the Report stays silent about a Region whose net migration is
    /// smaller than this. Half a person, the figure Unrest itself rounds by.
    pub report_net_floor: f64,
    pub occupation_start: f64,
    pub occupation_per_turn: f64,
    /// Ticket #299 (version 0.08.6): what a broken Occupation adds when the place hands back.
    pub occupation_break: f64,
    pub unrest_card: f64,
    pub natural_fall: f64,
    pub relief_ducats: i64,
    pub relief_points: f64,
    pub constabulary_fall: f64,
    /// The hook a Scrubber joins on its own ticket; nothing reads it yet.
    pub scrubber_fall: f64,
    pub green_techs_two: f64,
    pub green_techs_four: f64,
    pub constabulary_damping: f64,
    pub army_threshold: f64,
    pub facility_threshold: f64,
    pub throw_off_threshold: f64,
    pub throw_off_reset: f64,
    pub no_development_at: f64,
    pub pacification_divisor: i64,
    pub pacification_divisor_unrest: i64,
    pub pacification_unrest: f64,
    pub heat_share: f64,
    pub sea_loss_per_exposure: f64,
    pub sea_share: f64,
    pub resettle_ducats: i64,
    pub resettle_standing: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VictoryTable {
    pub stabilization_turns: u32,
    pub off_world_presence: u32,
    pub turns: u32,
    /// Ticket #57: the game begins on the first of this month. Ticket #67 (version 0.05.5): a Turn
    /// is `months_per_turn` calendar months, two since 0.05.5, and is named by its first month.
    #[serde(default = "twenty_thirty")]
    pub start_year: i64,
    #[serde(default = "january")]
    pub start_month: i64,
    #[serde(default = "one_month")]
    pub months_per_turn: i64,
    /// Ticket #261 (version 0.08.4): the share of the way to its Victory Condition at which a
    /// rival's Moment fires.
    pub rival_moment_share: f64,
}

fn twenty_thirty() -> i64 {
    2030
}
fn january() -> i64 {
    1
}
fn one_month() -> i64 {
    1
}

/// Ticket #57: one planet's Keplerian elements at J2000 and their rates per Julian century, as
/// JPL's approximate-positions table prints them (`ephemeris.toml`).
#[derive(Debug, Clone, Deserialize)]
pub struct PlanetElements {
    pub id: BodyId,
    pub a: f64,
    pub a_rate: f64,
    pub e: f64,
    pub e_rate: f64,
    pub inclination: f64,
    pub inclination_rate: f64,
    pub mean_longitude: f64,
    pub mean_longitude_rate: f64,
    pub perihelion_longitude: f64,
    pub perihelion_longitude_rate: f64,
    pub node_longitude: f64,
    pub node_longitude_rate: f64,
}

/// Ticket #57: what a transit between the Earth system and the Mars system costs, by how far the
/// phase angle stands from the Hohmann departure angle (`ephemeris.toml`, `[transit]`).
#[derive(Debug, Clone, Deserialize)]
pub struct TransitTable {
    /// The minimum-energy flight, in days: what a transit takes at the window.
    pub days_at_window: f64,
    /// The departure phase angle (Mars's heliocentric longitude less Earth's) that flight wants.
    pub hohmann_angle: f64,
    /// The same for the flight home, Earth leading.
    pub return_hohmann_angle: f64,
    /// Days added to the flight, and the fraction of the card's Fuel added, per degree off it.
    pub days_per_degree: f64,
    pub fuel_per_degree: f64,
    /// The days in a Turn.
    pub days_per_turn: f64,
    /// However far from the window, a transit is never longer than this.
    pub max_turns: u32,
    /// How often the phase angle comes round again: the cycle a window is looked for in.
    pub synodic_days: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct EphemerisFile {
    planet: Vec<PlanetElements>,
    transit: TransitTable,
    /// Ticket #93 (version 0.06.0): the Earth-Venus transfer, the same shape.
    transit_venus: TransitTable,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AiWeights {
    pub build_producer: f64,
    /// Ticket #56: raise a Sea Wall in a coastal slot before the sea takes it.
    pub build_sea_wall: f64,
    pub raise_industry: f64,
    pub build_research_lab: f64,
    /// Ticket #80 (version 0.06.0): an Observatory is offered, at the Research Lab weight, at a
    /// Colony or station holding this many Colonists.
    pub observatory_colonists: u32,
    /// Ticket #81: the Observatory's own weight, apart from the Lab's.
    pub build_observatory: f64,
    pub build_habitat: f64,
    pub build_launch_site_or_shipyard: f64,
    pub build_colony_ship: f64,
    pub build_warship: f64,
    pub build_army_or_barracks: f64,
    /// Ticket #36: an Embassy or a Relay.
    pub build_influence: f64,
    /// Ticket #51: divert this turn's Research into the Archive fund.
    pub fund_archive: f64,
    /// Ticket #51: build the Archive. Ticket #68: one Module, from its own button.
    pub build_archive: f64,
    /// Ticket #192 (version 0.08.0): read Colonists at the Archive's place into it.
    pub upload: f64,
    /// Ticket #52: pay Relief on a state the seat directs.
    pub relief: f64,
    /// Ticket #52: raise a Constabulary in a restive state.
    pub build_constabulary: f64,
    /// Ticket #267 (version 0.08.4): smear a rival the seat is Cold or Hostile toward whose Blame
    /// share stands above the fair quarter.
    pub smear: f64,
    /// Ticket #268 (version 0.08.4): buy carbon credits from the Custodians while above a fair share.
    pub buy_credits: f64,
    /// Ticket #277 (version 0.08.5): greenwash the seat's own Blame while above a fair share and
    /// credits are not to be had.
    pub greenwash: f64,
    /// Ticket #269 (version 0.08.4): agitate in a Region held by a rival the seat is Cold or Hostile toward.
    pub agitate: f64,
    /// Ticket #52: steer this turn's refugee flows into one calm state.
    pub resettle: f64,
    /// Ticket #227 (version 0.08.2): how readily this seat offers an Accord. Modest by default: an
    /// offer costs nothing and a refusal is not an offence, but a table where every seat proposes
    /// every turn would bury the player in yes-or-no questions.
    #[serde(default = "accord_weight_default")]
    pub accord: f64,
    pub influence: f64,
    pub transit: f64,
    pub load_unload: f64,
    pub found_colony: f64,
    /// Ticket #54: the Scrubber, which took Restoration's place and its Stabilization gap.
    pub build_scrubber: f64,
    /// Ticket #54: Mothball, Restart and Decommission, on a Facility or a Module.
    pub mothball: f64,
    pub restart: f64,
    pub decommission: f64,
    /// Ticket #54: the Custodians' Leapfrog and the Prospectors' Strip Permit.
    pub leapfrog: f64,
    pub strip_permit: f64,
    pub stance_attack: f64,
    pub stance_intercept: f64,
    pub stance_hold: f64,
    pub stance_evade: f64,
    /// Ticket #278 (version 0.08.5): blockade the rival station whose slot the stack sits in.
    pub stance_blockade: f64,
    /// Ticket #297 (version 0.08.6): dig in where a rival's Army stands next door and the seat has
    /// no cause to attack, and wherever it occupies.
    pub stance_dig_in: f64,
}



#[derive(Debug, Clone, Deserialize)]
pub struct AiMultipliers {
    pub victory_gap_max: f64,
    pub threat: f64,
    pub opportunity: f64,
    pub energy_shortage_bonus: f64,
    /// Ticket #181 (version 0.08.0): the slight bias a seat gets toward its own Unique Facility.
    pub unique_bias: f64,
    /// Ticket #182: what one Material in the Venture Capital Fund adds to the Prospectors' appetite
    /// for an Investment Bank, since the building's worth is a share of that balance.
    pub investment_bank_per_fund: f64,
    /// Ticket #209 (version 0.08.1): what the Archivists' appetite for the whole off-Earth chain --
    /// a Colony Ship, a Launch Site or Shipyard, a load, a transit, a founding -- is multiplied by
    /// while they have nowhere the Archive may stand. It applies to no other Faction and stops the
    /// moment they hold such a place.
    pub archive_needs_a_place: f64,
    /// Ticket #332 (version 0.09.0): the pace of a build. A build candidate is weighed against
    /// the same build at the seat's other places by the Resolutions until this place's Widgets
    /// would finish it behind its queue: the soonest place at full weight, every other at
    /// soonest / turns, never below this floor. A place that makes no Widgets is at the floor.
    pub build_pace_floor: f64,
}

/// Ticket #50: one pace schedule per Faction. `first` is the schedule for the Faction's first
/// Victory part as [turn, value]; a Faction whose first part is a Stabilization run uses the ppm
/// figures instead, since a run has no useful interpolation.
#[derive(Debug, Clone, Deserialize)]
pub struct AiPace {
    #[serde(default)]
    pub first: Vec<[i64; 2]>,
    #[serde(default)]
    pub within_ppm: f64,
    #[serde(default)]
    pub within_by_turn: u32,
    #[serde(default)]
    pub under_sink_by_turn: u32,
    pub colonists: Vec<[i64; 2]>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AiThresholds {
    pub attack_odds: f64,
    /// Ticket #284 (version 0.08.5): the Relations score at or below which a seat has cause to
    /// attack a place a rival holds -- Cold or worse.
    pub war_cause: i64,
    pub evade_damage_fraction: f64,
    pub influence_step: i64,
    /// Ticket #75: a held state's worth on the Influence target list, as a share of a neutral one's.
    pub held_state_weight: f64,
    /// Ticket #84 (version 0.06.0): as Research Lead the AI picks its Victory gate once its first
    /// part is past this fraction of its bar, or from this turn, whichever comes first.
    pub gate_pick_fraction: f64,
    pub gate_pick_turn: u32,
    /// Ticket #236 (version 0.08.3): what a seat directs away from the shared Tech, by how much it
    /// wants the Tech under research. Before this it went to its cap on turn 2 and never moved,
    /// which made any contribution threshold unreachable and the shared-pot rule a flat tax.
    pub directive_when_wanted: u8,
    pub directive_when_indifferent: u8,
    /// Ticket #332 (version 0.09.0): the early Mine. Through this turn, while the seat directs
    /// fewer Mines on Earth (standing or on order) than `early_mines`, the Mine in its most
    /// Materials-lean Region -- the one where a Mine would make the most -- is wanted at the
    /// Factory's weight with the victory-gap and opportunity multipliers.
    pub early_mine_turn: u32,
    pub early_mines: u32,
    /// Ticket #332: a Factory Module is wanted at a Colony whose queue is this many builds deep,
    /// or where a Ship is wanted (a Shipyard standing or on order).
    pub factory_module_queue_depth: usize,
    /// Ticket #335 (version 0.09.0): how many warships a seat wants holding LOW ORBIT at a Body
    /// whose ground it wants, before the next hull's leg names a rival station's ring instead.
    pub low_orbit_warships: u32,
}

/// Ticket #50: one pick list per Faction. `order` is tried first, then the cheapest available
/// Tech that is not `never`, and `last` only when nothing else is left.
#[derive(Debug, Clone, Deserialize)]
pub struct AiTechPicks {
    pub order: Vec<TechId>,
    #[serde(default)]
    pub last: Option<TechId>,
    #[serde(default)]
    pub never: Option<TechId>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AiTable {
    pub weights: BTreeMap<FactionKind, AiWeights>,
    pub multipliers: AiMultipliers,
    pub pace: BTreeMap<FactionKind, AiPace>,
    pub thresholds: AiThresholds,
    pub tech_picks: BTreeMap<FactionKind, AiTechPicks>,
}

#[derive(Debug, Clone, Deserialize)]
struct BodiesFile {
    body: Vec<BodyCard>,
    #[serde(default = "one_u32")]
    sibling_turns: u32,
    #[serde(default = "one_i64")]
    sibling_fuel: i64,
    /// Ticket #335 (version 0.09.0): what an orbit change costs from the Ship's own tank.
    #[serde(default = "one_i64")]
    orbit_change_fuel: i64,
    #[serde(default = "forty")]
    station_materials: i64,
    /// Ticket #57: how far a Colony Slot's own yields may fall either side of its Body's.
    #[serde(default = "quarter")]
    slot_yield_spread: f64,
}
fn forty() -> i64 {
    40
}
fn quarter() -> f64 {
    0.25
}
fn one_u32() -> u32 {
    1
}
fn one_i64() -> i64 {
    1
}
#[derive(Debug, Clone, Deserialize)]
struct StatesFile {
    state: Vec<StateCard>,
    development: DevelopmentTable,
    strip_permit: StripPermitTable,
    /// Ticket #56: the build slots every Nation State has on top of Size and Industry Level.
    #[serde(default = "three")]
    base_slots: u32,
    /// Ticket #56: coastal slots per point of Coastal Exposure, capped at the start slots less one.
    #[serde(default = "three")]
    coastal_per_exposure: u32,
}

fn three() -> u32 {
    3
}

/// Ticket #54 (version 0.05): the Strip Permit, in `nation_states.toml` under `[strip_permit]`.
/// The designer named these `strip_permit_turns`, `strip_permit_multiplier`,
/// `strip_permit_baseline_rise` and `strip_permit_unrest`; inside their own table the prefix would
/// only repeat itself, as with `[development]`.
#[derive(Debug, Clone, Deserialize)]
pub struct StripPermitTable {
    /// How many turns of Income every Facility in the state produces double.
    pub turns: u32,
    /// What it multiplies that output by.
    pub multiplier: f64,
    /// What the state's Baseline Emissions rise by, for good, when it ends.
    pub baseline_rise: f64,
    /// What its Unrest rises by when it ends.
    pub unrest: f64,
}

/// Ticket #53 (version 0.05): Neutral Development, in `nation_states.toml` under `[development]`.
/// The designer named these `development_turns`, `development_max_level` and
/// `development_stops_at_temperature`; inside their own table the prefix would only repeat itself.
#[derive(Debug, Clone, Deserialize)]
pub struct DevelopmentTable {
    /// Turns of unbroken neutrality between one raise of the Industry Level and the next.
    pub turns: u32,
    /// The Industry Level a neutral state develops itself up to, and no further.
    pub max_level: u32,
    /// A world standing at this Temperature or above develops nothing.
    pub stops_at_temperature: f64,
}
#[derive(Debug, Clone, Deserialize)]
struct FacilitiesFile {
    facility: Vec<FacilityCard>,
    industry_level: IndustryLevelCard,
    widgets: WidgetsCard,
    scrubber: ScrubberCard,
    population_factor: PopulationFactorCard,
    sea_wall: SeaWallCard,
    mothball: MothballCard,
    school: SchoolCard,
    unique: UniqueCard,
}
/// Ticket #51: the Archive. Its Materials, build turns and Energy upkeep sit on its Module row.
/// Ticket #68 (version 0.05.5): the Research it requires in all, and the share of it the fund may
/// hold before the Module stands.
/// Ticket #97 (version 0.07.0): the Modules a Colony or a Space Station may hold: `base` free, and
/// one more for every `per_colonist` Colonists living there.
#[derive(Debug, Clone, Deserialize)]
pub struct SlotsCard {
    pub base: u32,
    pub per_colonist: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ArchiveCard {
    pub research: i64,
    pub banked_before_built: f64,
    /// Ticket #192 (version 0.08.0): Colonists who must live at the place before it may be ORDERED.
    pub colonists_to_order: u32,
}

/// Ticket #80 (version 0.06.0): the Observatory's one figure beyond its row: the share of its
/// Research each Colonist at its Colony adds (one per cent).
#[derive(Debug, Clone, Deserialize)]
pub struct ObservatoryCard {
    pub research_per_colonist: f64,
}

/// Ticket #88 (version 0.06.0): build it where you dig. A Module at a Colony with one working
/// Mine costs `one_mine` of its price, with two or more `two_mines`, never below `floor` of the row.
#[derive(Debug, Clone, Deserialize)]
pub struct InSituCard {
    pub one_mine: f64,
    pub two_mines: f64,
    pub floor: f64,
}

/// Ticket #90 (version 0.06.0): the Trade Post's network figure, Ducats for every other Body the
/// Faction holds; the per-Colonist figure is the row's `produces.amount`.
#[derive(Debug, Clone, Deserialize)]
pub struct TradePostCard {
    pub per_other_body: i64,
}

#[derive(Debug, Clone, Deserialize)]
struct ModulesFile {
    module: Vec<ModuleCard>,
    slots: SlotsCard,
    archive: ArchiveCard,
    observatory: ObservatoryCard,
    in_situ: InSituCard,
    trade_post: TradePostCard,
    mass_driver: MassDriverCard,
}
/// Ticket #295 (version 0.08.6): the disengage roll's figure, in data at last. After every round a
/// damaged unit leaves with chance damage over hit points over `divisor`; the First Playable wrote
/// the 2 into the code and never tuned it. Evade's flat half is not this figure.
#[derive(Debug, Clone, Deserialize)]
pub struct DisengageCard {
    pub divisor: f64,
}

/// Ticket #296 (version 0.08.6): what a Region's people add to its own Army -- `constabulary`
/// while a working Constabulary stands there, `calm` while Unrest is under the Standing Army's
/// threshold. Ticket #302: as DEFENCE, while it defends, never as hit points; and the steps a
/// neutral arms by: `threat_steps` at the Income a threat begins, `held_step` for an attack held.
#[derive(Debug, Clone, Deserialize)]
pub struct StandingArmyCard {
    pub constabulary: u32,
    pub calm: u32,
    pub threat_steps: u32,
    pub held_step: u32,
    /// Ticket #318 (version 0.08.8): the Incomes after a Standing Army's death before it is raised
    /// again, at strength one; 2 since ticket #282, where it was a literal in code.
    pub respawn_incomes: u32,
}

/// Ticket #297 (version 0.08.6): what an Army dug in adds to its strength while it defends. Hit
/// points do not follow it, at the designer's word.
#[derive(Debug, Clone, Deserialize)]
pub struct DigInCard {
    pub defence: i64,
}

/// Ticket #334 (version 0.09.0): the people a raised Army takes, at the order. In a Region
/// `population_each` units of its population (one unit, one million people since ticket #333); at
/// a Colony `colonists_each` Colonists, refused where fewer than one more live there so the Core is
/// never emptied. The Standing Army takes nobody, and nobody returns.
#[derive(Debug, Clone, Deserialize)]
pub struct ArmyCard {
    pub population_each: f64,
    pub colonists_each: u32,
}

/// Ticket #327 (version 0.08.8): the melee's shape, in data: its rounds, and the hit rolls a
/// round on the ground and the floor for a Ship melee, which rolls once for every engaged armed
/// unit present when that is more.
#[derive(Debug, Clone, Deserialize)]
pub struct MeleeCard {
    pub rounds: u32,
    pub rolls: u32,
}

#[derive(Debug, Clone, Deserialize)]
struct UnitsFile {
    unit: Vec<UnitCard>,
    repair: RepairCard,
    crowding: CrowdingCard,
    disengage: DisengageCard,
    melee: MeleeCard,
    standing_army: StandingArmyCard,
    dig_in: DigInCard,
    army: ArmyCard,
}

/// Ticket #86 (version 0.06.0): a warming Earth fills the Colony Ships. `per_step` Colonists
/// beyond capacity for every full `step` degrees above `above`, at most `cap`; at arrival each
/// extra dies with a chance of `death_chance_per_extra` times the number of extras.
#[derive(Debug, Clone, Deserialize)]
pub struct CrowdingCard {
    pub above: f64,
    pub step: f64,
    pub per_step: u32,
    pub cap: u32,
    pub death_chance_per_extra: f64,
}
#[derive(Debug, Clone, Deserialize)]
struct TechsFile {
    tech: Vec<TechCard>,
    shortlist: ShortlistCard,
}

/// Ticket #98 (version 0.07.0): how many Techs the Research Lead may choose between once the game
/// is under way. The opening pick is not drawn.
#[derive(Debug, Clone, Deserialize)]
pub struct ShortlistCard {
    pub size: usize,
}
#[derive(Debug, Clone, Deserialize)]
struct FactionsFile {
    faction: Vec<FactionCard>,
    start: StartCard,
    ducats: DucatsCard,
    research_directive: ResearchDirectiveCard,
    venture_capital: VentureCard,
    carbon_credits: CarbonCreditsCard,
    emigrants: EmigrantsCard,
    faction_orders: FactionOrdersCard,
    exodus_call: ExodusCallCard,
    relations: RelationsCard,
}

/// Ticket #191 (version 0.08.0): the Relations scale and what moves it (`factions.toml`).
#[derive(Debug, Clone, Deserialize)]
pub struct RelationsCard {
    /// Ticket #236 (version 0.08.3): the share of its Research a Faction must still give the
    /// shared Tech to escape the shared-pot penalty, the size of the term either way, and the
    /// ceiling the reward may lift a pair to.
    pub directive_min_contribution: u8,
    pub directive_step: i64,
    pub directive_boost_ceiling: i64,
    pub best: i64,
    pub worst: i64,
    pub start: i64,
    pub fall_per_offending_turn: i64,
    /// Ticket #299 (version 0.08.6): the weight of an Occupation broken, the rung between a bid
    /// (1) and a Battle (3).
    pub occupation_broken_offence: i64,
    pub recover: i64,
    pub quiet_turns: u32,
    /// Ticket #222 (version 0.08.2): the most a single turn may charge, however much was done in it.
    /// A guard against one dramatic turn spending the whole scale, not a working part of the rule --
    /// measured, it bites on 1.7% of offending pair-turns.
    #[serde(default = "turn_cap_default")]
    pub turn_cap: i64,
    /// Ticket #221: the share above a fair quarter that buys one step of resentment.
    #[serde(default = "blame_step_default")]
    pub blame_step: f64,
    /// Ticket #221: the most Blame alone may cost, half the scale. Blame can make a pair Cold but
    /// never, by itself, Hostile.
    #[serde(default = "blame_cap_default")]
    pub blame_cap: i64,
    /// Ticket #223: what one act of friendship is worth, once a turn per ordered pair.
    #[serde(default = "act_gain_default")]
    pub act_gain: i64,
    /// Ticket #223: how high the DEEDS figure may climb past the scale, so a pair carrying a heavy
    /// Blame term can still reach Friendly on deeds alone.
    #[serde(default = "deeds_ceiling_default")]
    pub deeds_ceiling: i64,
    /// Ticket #223: quiet turns to lose one point of a POSITIVE score -- twice the period below
    /// neutral, so friendship lapses at half the rate enmity heals.
    #[serde(default = "positive_quiet_default")]
    pub positive_quiet_turns: u32,
    /// Ticket #225: offending TURNS per step of the floor.
    #[serde(default = "scar_turns_default")]
    pub scar_turns: u32,
    /// Ticket #225: the lowest the floor may go. -3 rather than -5 deliberately: since every act
    /// that raises Relations is an Accord act, a pair floored below the level an Accord can be
    /// struck at would have no road back at all.
    #[serde(default = "scar_floor_default")]
    pub scar_floor: i64,
    /// Ticket #226 (version 0.08.2): what one tribute costs, in Ducats or in Materials. A FIXED
    /// price, not an amount the giver chooses: the Relations gain is a flat `act_gain`, so without a
    /// fixed price a one-Ducat tribute would buy the same point as a fifty-Ducat one.
    #[serde(default = "tribute_ducats_default")]
    pub tribute_ducats: i64,
    #[serde(default = "tribute_materials_default")]
    pub tribute_materials: i64,
    /// Ticket #226: turns an Accord must stand to pay its keeping, repeatably. Four, matching the
    /// quiet-turn recovery cadence already in the rules -- and it is also the only thing that lifts
    /// a scarred pair's floor.
    #[serde(default = "accord_kept_default")]
    pub accord_kept_turns: u32,
}

fn turn_cap_default() -> i64 {
    8
}
fn blame_step_default() -> f64 {
    0.10
}
fn blame_cap_default() -> i64 {
    5
}
fn act_gain_default() -> i64 {
    1
}
fn deeds_ceiling_default() -> i64 {
    15
}
fn positive_quiet_default() -> u32 {
    8
}
fn scar_turns_default() -> u32 {
    4
}
fn scar_floor_default() -> i64 {
    -3
}

/// Ticket #73 (version 0.05.5): Emigrants, the built Colonists: how many a Faction musters a turn,
/// the population each takes, what a batch takes off the state's Unrest, and how many turns the sea
/// crossing to Antarctica takes.
#[derive(Debug, Clone, Deserialize)]
pub struct FactionOrdersCard {
    pub min_turns_held: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExodusCallCard {
    pub turns: u32,
    pub muster_multiplier: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmigrantsCard {
    pub per_turn: u32,
    pub population_each: f64,
    pub unrest_fall: f64,
    pub antarctica_turns: u32,
}

/// Ticket #72 (version 0.05.5): the Prospectors' Venture Capital Fund: the largest share of their
/// Materials output that may be banked a turn, the step the share moves in, and what a draw returns.
/// Ticket #268 (version 0.08.4): carbon credits, in `factions.toml` under `[carbon_credits]`.
#[derive(Debug, Clone, Deserialize)]
pub struct CarbonCreditsCard {
    /// Ducats per ppm, before the seller's view of the buyer multiplies it.
    pub price_per_ppm: i64,
    /// The most ppm one buyer may take in a turn.
    pub cap_per_turn: i64,
    /// The price multiplier by the seller's Relations level toward the buyer; Hostile refuses.
    pub friendly: f64,
    pub cordial: f64,
    pub neutral: f64,
    pub wary: f64,
    pub cold: f64,
    /// The computer Custodians oversell by the cap when their Ducats stand below this.
    pub ai_oversell_when_ducats_below: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VentureCard {
    pub max_share: f64,
    pub share_step: f64,
    pub draw_return: f64,
}

/// Ticket #169 (version 0.07.5): one note at the head of one turn of a tutorial game.
#[derive(Debug, Clone, Deserialize)]
pub struct TutorialNote {
    /// The turn this note opens, counting from 1.
    pub turn: u32,
    /// The big line at the top of the note.
    pub figure: String,
    /// The sentence under it.
    pub text: String,
    /// The quieter line under that.
    pub note: Option<String>,
}

/// Ticket #169: the tutorial's notes, in the order they are shown.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct TutorialTable {
    #[serde(default)]
    pub note: Vec<TutorialNote>,
}

/// Every table, loaded and checked.
#[derive(Debug, Clone)]
pub struct Tables {
    pub bodies: Vec<BodyCard>,
    /// Ticket #45: the hop between two satellites of the same Body.
    pub sibling_transit: (u32, i64),
    /// Ticket #335 (version 0.09.0): the Fuel an orbit change takes from a Ship's own tank.
    pub orbit_change_fuel: i64,
    /// Ticket #46: what a station costs.
    pub station_materials: i64,
    /// Ticket #57: how far a Colony Slot's own four yields may fall either side of its Body's.
    pub slot_yield_spread: f64,
    /// Ticket #57: the Keplerian elements of Earth and Mars, and the transit table (`ephemeris.toml`).
    pub planets: Vec<PlanetElements>,
    pub transit: TransitTable,
    /// Ticket #93 (version 0.06.0): the Earth-Venus transfer.
    pub transit_venus: TransitTable,
    pub states: Vec<StateCard>,
    /// Ticket #53: how a neutral Nation State develops itself (`nation_states.toml`).
    pub development: DevelopmentTable,
    /// Ticket #54: the Strip Permit's figures (`nation_states.toml`).
    pub strip_permit: StripPermitTable,
    /// Ticket #56: the build slots every state has beyond Size and Industry Level, and how many
    /// coastal slots a point of Coastal Exposure buys (`nation_states.toml`'s header).
    pub base_slots: u32,
    pub coastal_per_exposure: u32,
    pub facilities: Vec<FacilityCard>,
    pub industry_level: IndustryLevelCard,
    /// Ticket #332 (version 0.09.0): a Region's base Widgets per Industry Level.
    pub widgets: WidgetsCard,
    /// Ticket #54: the Scrubber cap and the Mothball prices (`facilities.toml`).
    pub scrubber: ScrubberCard,
    /// Ticket #333 (version 0.09.0): the Research Lab's population factor divisor (`facilities.toml`).
    pub population_factor: PopulationFactorCard,
    pub mothball: MothballCard,
    /// Ticket #257: the Sea Wall's keep per rise held.
    pub sea_wall: SeaWallCard,
    /// Ticket #185: the School's step and ceiling.
    pub school: SchoolCard,
    /// Tickets #182, #184, #186: the three Unique Facility clause figures that have one.
    pub unique: UniqueCard,
    pub modules: Vec<ModuleCard>,
    /// Ticket #97: how many Modules a Colony or a Space Station may hold.
    pub slots: SlotsCard,
    /// Ticket #98: how many Techs the Research Lead chooses between.
    pub shortlist: ShortlistCard,
    /// Ticket #51: the Archive's stages and their Research price.
    pub archive: ArchiveCard,
    /// Ticket #80: the Observatory's Research per Colonist.
    pub observatory: ObservatoryCard,
    /// Ticket #88: the discount a Colony's working Mines give its Modules.
    pub in_situ: InSituCard,
    /// Ticket #90: the Trade Post's network figure.
    pub trade_post: TradePostCard,
    /// Ticket #92: the Mass Driver's Fuel cut and Mine bonus.
    pub mass_driver: MassDriverCard,
    pub units: Vec<UnitCard>,
    pub repair: RepairCard,
    /// Ticket #86: the crowd a warming Earth puts aboard a Colony Ship, and what it risks.
    pub crowding: CrowdingCard,
    /// Ticket #295 (version 0.08.6): the disengage roll's divisor.
    pub disengage: DisengageCard,
    /// Ticket #327 (version 0.08.8): the melee's rounds and rolls.
    pub melee: MeleeCard,
    /// Ticket #296 (version 0.08.6): what a Region's people add to its own Armies.
    pub standing_army: StandingArmyCard,
    /// Ticket #297 (version 0.08.6): what digging in adds to a defending Army.
    pub dig_in: DigInCard,
    /// Ticket #334 (version 0.09.0): the people a raised Army takes.
    pub army: ArmyCard,
    pub techs: Vec<TechCard>,
    pub events: EventsTable,
    pub factions: Vec<FactionCard>,
    pub start: StartCard,
    pub research_directive: ResearchDirectiveCard,
    pub exodus_call: ExodusCallCard,
    pub faction_orders: FactionOrdersCard,
    pub ducats: DucatsCard,
    pub venture: VentureCard,
    /// Ticket #268: carbon credits.
    pub carbon_credits: CarbonCreditsCard,
    pub emigrants: EmigrantsCard,
    /// Ticket #191: the Relations scale and what moves it.
    pub relations: RelationsCard,
    pub climate: ClimateTable,
    pub influence: InfluenceTable,
    /// Ticket #52: `unrest.toml`.
    pub unrest: UnrestTable,
    pub victory: VictoryTable,
    pub ai: AiTable,
    /// Ticket #58: every sentence the Report says (`report.toml`).
    pub report: crate::report::ReportTable,
    /// Ticket #169 (version 0.07.5): the tutorial's notes (`tutorial.toml`), one at the head of each
    /// of a tutorial game's first turns. They are words and nothing else -- no rule reads them -- but
    /// they live here with every other sentence the game says, so they can be rewritten without a build.
    pub tutorial: TutorialTable,
    /// Ticket #210 (version 0.08.1): the two lists every Ship is named from.
    pub ship_names: ShipNames,
}

/// Ticket #210 (version 0.08.1): the names a Ship may be given, in two lists. A Colony Ship draws
/// from `colony`; a Frigate, a Battleship and a Carrier from `warship`. Order matters: a Ship takes
/// the first unused name in list order, which is deterministic and draws no randomness, so naming
/// cannot shift a seeded game's rolls and make a sweep incomparable with its baseline.
#[derive(Debug, Clone, Deserialize)]
pub struct ShipNames {
    pub colony: ShipNameList,
    pub warship: ShipNameList,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ShipNameList {
    pub names: Vec<String>,
}

fn read<T: for<'de> Deserialize<'de>>(dir: &Path, file: &str) -> Result<T, DataError> {
    let path: PathBuf = dir.join(file);
    let text = std::fs::read_to_string(&path).map_err(|e| DataError {
        file: path.display().to_string(),
        message: format!("cannot read: {e}"),
    })?;
    toml::from_str(&text).map_err(|e| DataError {
        file: path.display().to_string(),
        message: e.to_string(),
    })
}

fn err(file: &str, message: impl Into<String>) -> DataError {
    DataError { file: file.to_string(), message: message.into() }
}

impl Tables {
    /// Load every table from a directory (normally `assets/data`).
    pub fn load(dir: &Path) -> Result<Tables, DataError> {
        let bodies: BodiesFile = read(dir, "bodies.toml")?;
        let states: StatesFile = read(dir, "nation_states.toml")?;
        let facilities: FacilitiesFile = read(dir, "facilities.toml")?;
        let modules: ModulesFile = read(dir, "modules.toml")?;
        let units: UnitsFile = read(dir, "units.toml")?;
        let techs: TechsFile = read(dir, "techs.toml")?;
        let events: EventsTable = read(dir, "events.toml")?;
        let factions: FactionsFile = read(dir, "factions.toml")?;
        let climate: ClimateTable = read(dir, "climate.toml")?;
        let influence: InfluenceTable = read(dir, "influence.toml")?;
        let unrest: UnrestTable = read(dir, "unrest.toml")?;
        let victory: VictoryTable = read(dir, "victory.toml")?;
        let ai: AiTable = read(dir, "ai.toml")?;
        let ephemeris: EphemerisFile = read(dir, "ephemeris.toml")?;
        let report: crate::report::ReportTable = read(dir, "report.toml")?;
        let tutorial: TutorialTable = read(dir, "tutorial.toml")?;
        let ship_names: ShipNames = read(dir, "ship_names.toml")?;
        let tables = Tables {
            ship_names,
            sibling_transit: (bodies.sibling_turns, bodies.sibling_fuel),
            orbit_change_fuel: bodies.orbit_change_fuel,
            station_materials: bodies.station_materials,
            slot_yield_spread: bodies.slot_yield_spread,
            planets: ephemeris.planet,
            transit: ephemeris.transit,
            transit_venus: ephemeris.transit_venus,
            bodies: bodies.body,
            states: states.state,
            development: states.development,
            strip_permit: states.strip_permit,
            base_slots: states.base_slots,
            coastal_per_exposure: states.coastal_per_exposure,
            facilities: facilities.facility,
            industry_level: facilities.industry_level,
            widgets: facilities.widgets,
            scrubber: facilities.scrubber,
            population_factor: facilities.population_factor,
            mothball: facilities.mothball,
            sea_wall: facilities.sea_wall,
            school: facilities.school,
            unique: facilities.unique,
            slots: modules.slots,
            archive: modules.archive,
            observatory: modules.observatory,
            in_situ: modules.in_situ,
            trade_post: modules.trade_post,
            mass_driver: modules.mass_driver,
            modules: modules.module,
            units: units.unit,
            repair: units.repair,
            crowding: units.crowding,
            disengage: units.disengage,
            melee: units.melee,
            standing_army: units.standing_army,
            dig_in: units.dig_in,
            army: units.army,
            techs: techs.tech,
            shortlist: techs.shortlist,
            events,
            factions: factions.faction,
            start: factions.start,
            ducats: factions.ducats,
            exodus_call: factions.exodus_call,
            faction_orders: factions.faction_orders,
            research_directive: factions.research_directive,
            venture: factions.venture_capital,
            carbon_credits: factions.carbon_credits,
            emigrants: factions.emigrants,
            relations: factions.relations,
            climate,
            influence,
            unrest,
            victory,
            ai,
            report,
            tutorial,
        };
        tables.validate()?;
        Ok(tables)
    }

    fn validate(&self) -> Result<(), DataError> {
        // Ticket #58: every sentence the Report says, with every placeholder the engine supplies.
        self.report.check().map_err(|m| err("report.toml", m))?;
        // Every fixed id must have exactly one row, in the engine's order.
        check_rows("bodies.toml", &BodyId::ALL, self.bodies.iter().map(|b| b.id))?;
        check_rows("nation_states.toml", &StateId::ALL, self.states.iter().map(|s| s.id))?;
        check_rows("facilities.toml", &FacilityKind::ALL, self.facilities.iter().map(|f| f.id))?;
        check_rows("modules.toml", &ModuleKind::ALL, self.modules.iter().map(|m| m.id))?;
        check_rows(
            "units.toml",
            &[UnitKind::ColonyShip, UnitKind::Frigate, UnitKind::Battleship, UnitKind::Carrier, UnitKind::Army],
            self.units.iter().map(|u| u.id),
        )?;
        check_rows("techs.toml", &TechId::ALL, self.techs.iter().map(|t| t.id))?;
        check_rows("events.toml", &EventId::ALL, self.events.event.iter().map(|e| e.id))?;
        check_rows("factions.toml", &FactionKind::ALL, self.factions.iter().map(|f| f.id))?;
        // Ticket #50: every Faction needs its own weights, pace and Tech picks.
        for k in FactionKind::ALL {
            if !self.ai.weights.contains_key(&k) {
                return Err(err("ai.toml", format!("no [weights.{}] table for the {}", k.id(), k.name())));
            }
            if !self.ai.pace.contains_key(&k) {
                return Err(err("ai.toml", format!("no [pace.{}] table for the {}", k.id(), k.name())));
            }
            if !self.ai.tech_picks.contains_key(&k) {
                return Err(err("ai.toml", format!("no [tech_picks.{}] table for the {}", k.id(), k.name())));
            }
        }
        // Ticket #50: a Faction's start station, when it has one, must name an orbital slot over Earth.
        for f in &self.factions {
            if let Some(name) = &f.start_station
                && !self.body(BodyId::Earth).stations.contains(name)
            {
                return Err(err("factions.toml", format!("row {}: start_station {:?} is no orbital slot over Earth", f.name, name)));
            }
            // Ticket #290 (version 0.08.6): the people aboard at the start need a station to be
            // aboard, and fit in its Core Module.
            if f.start_colonists > 0 && f.start_station.is_none() {
                return Err(err("factions.toml", format!("row {}: start_colonists without a start_station", f.name)));
            }
            if f.start_colonists > self.module(ModuleKind::Core).holds_colonists {
                return Err(err("factions.toml", format!("row {}: start_colonists {} would not fit in the Core Module", f.name, f.start_colonists)));
            }
            if f.victory_first.bar <= 0.0 {
                return Err(err("factions.toml", format!("row {}: victory_first.bar must be positive", f.name)));
            }
            // Ticket #51: whichever second part a card names, its own figures must be positive.
            let second_ok = match f.victory_second.kind {
                VictorySecondKind::OffWorldPresence | VictorySecondKind::ColonistsUploaded => f.victory_second.bar > 0.0,
                VictorySecondKind::ColoniesOnBodies => f.victory_second.bodies > 0 && f.victory_second.colonists_each > 0,
            };
            if !second_ok {
                return Err(err("factions.toml", format!("row {}: victory_second needs a positive bar, or bodies and colonists_each", f.name)));
            }
            for (what, m) in [
                ("habitat_capacity_multiplier", f.habitat_capacity_multiplier),
                ("transit_fuel_multiplier", f.transit_fuel_multiplier),
                ("colony_ship_capacity_multiplier", f.colony_ship_capacity_multiplier),
                ("lift_population_multiplier", f.lift_population_multiplier),
                ("station_materials_multiplier", f.station_materials_multiplier),
                ("module_materials_multiplier", f.module_materials_multiplier),
            ] {
                if m <= 0.0 {
                    return Err(err("factions.toml", format!("row {}: {} must be positive", f.name, what)));
                }
            }
        }
        for s in &self.states {
            for n in &s.neighbours {
                let back = &self.states[n.index()];
                if !back.neighbours.contains(&s.id) {
                    return Err(err(
                        "nation_states.toml",
                        format!("row {}: neighbour {} does not list it back", s.name, back.name),
                    ));
                }
            }
            if s.population < 0.0 || s.education_level <= 0.0 {
                return Err(err("nation_states.toml", format!("row {}: population or education out of range", s.name)));
            }
            // A Launch Site is added for a Faction start state, so leave one slot for it.
            let start_slots = s.size + s.industry_level + self.base_slots;
            if s.start_facilities.len() as u32 + 1 > start_slots {
                return Err(err("nation_states.toml", format!("row {}: {} start_facilities do not fit its {} build slots with a Launch Site", s.name, s.start_facilities.len(), start_slots)));
            }
            // Ticket #56: every state keeps at least one coastal slot and one inland slot, so the
            // sea always has something to take and a raise always has somewhere to go.
            if start_slots < 2 {
                return Err(err("nation_states.toml", format!("row {}: {start_slots} start slots leave no room for a coastal slot and an inland one", s.name)));
            }
        }
        // Ticket #56: a Facility that names an unlocking Tech must name a real one, and a
        // coastal-only Facility must take a slot at all.
        for f in &self.facilities {
            if f.coastal_only && f.no_slot {
                return Err(err("facilities.toml", format!("row {}: a coastal-only Facility must take a build slot", f.name)));
            }
        }
        // Ticket #332 (version 0.09.0): a build of nought Widgets would complete at a place that
        // makes none, which is not a rule anybody wrote. Every row that can be ordered carries a
        // figure; the Core Module alone is exempt, since nobody orders it (its Materials are nought
        // for the same reason).
        if let Some(f) = self.facilities.iter().find(|f| f.widgets == 0) {
            return Err(err("facilities.toml", format!("row {}: widgets must be at least 1", f.name)));
        }
        if let Some(m) = self.modules.iter().find(|m| m.widgets == 0 && m.id != ModuleKind::Core) {
            return Err(err("modules.toml", format!("row {}: widgets must be at least 1", m.name)));
        }
        if let Some(u) = self.units.iter().find(|u| u.widgets == 0) {
            return Err(err("units.toml", format!("row {}: widgets must be at least 1", u.name)));
        }
        if self.industry_level.widgets == 0 || self.widgets.region_base + self.widgets.per_industry_level == 0 {
            return Err(err("facilities.toml", "[industry_level] widgets must be at least 1, and [widgets] region_base or per_industry_level must be"));
        }
        // Ticket #334 (version 0.09.0): an Army is raised from people, so a raise that took nobody
        // is not a rule anybody wrote; and a Colony's take must leave the Core its one Colonist.
        if self.army.population_each <= 0.0 || !self.army.population_each.is_finite() || self.army.colonists_each == 0 {
            return Err(err("units.toml", "[army] population_each must be positive and colonists_each at least 1"));
        }
        for t in &self.techs {
            for n in &t.needs {
                if *n == t.id {
                    return Err(err("techs.toml", format!("row {}: needs itself", t.name)));
                }
            }
            if t.cost <= 0 {
                return Err(err("techs.toml", format!("row {}: cost must be positive", t.name)));
            }
        }
        for u in &self.units {
            if u.hit_points == 0 {
                return Err(err("units.toml", format!("row {}: hit points must be positive", u.name)));
            }
        }
        if self.climate.sea_level_thresholds.is_empty() {
            return Err(err("climate.toml", "sea_level_thresholds is empty"));
        }
        // Ticket #55: the Breaks are read in order, so the panel's "next" line and the fired flags
        // both depend on the list rising. A Break that does nothing is a typo, not a rule.
        let mut previous = f64::MIN;
        for b in &self.climate.breaks {
            if b.temperature <= previous {
                return Err(err("climate.toml", format!("[[break]] {}: the Breaks must rise in Temperature", b.id)));
            }
            previous = b.temperature;
            if self.climate.breaks.iter().filter(|o| o.id == b.id).count() > 1 {
                return Err(err("climate.toml", format!("[[break]] {}: two Breaks share an id", b.id)));
            }
            let figures = match b.effect {
                BreakEffect::CoastalUnrest => b.unrest > 0.0 || b.population_loss > 0.0,
                BreakEffect::EmissionsPerTurn => b.emissions > 0.0,
                BreakEffect::WeakenSink => b.sink_after > 0.0,
                BreakEffect::SeaLevelThreshold => true,
                BreakEffect::CarbonPulse => b.co2 > 0.0 || (b.baseline_rise > 0.0 && b.state.is_some()),
            };
            if !figures {
                return Err(err("climate.toml", format!("[[break]] {}: its effect kind has no figures to act on", b.id)));
            }
        }
        // Ticket #52: the Unrest ladder must be in order and every start value on it.
        let u = &self.unrest;
        if !(u.army_threshold < u.facility_threshold && u.facility_threshold < u.throw_off_threshold && u.throw_off_threshold <= u.max) {
            return Err(err("unrest.toml", "the thresholds must rise: army_threshold < facility_threshold < throw_off_threshold <= max"));
        }
        if u.neutral_max > u.max || u.refugees_per <= 0.0 || u.report_net_floor <= 0.0 {
            return Err(err("unrest.toml", "neutral_max must not exceed max, and refugees_per and report_net_floor must be positive"));
        }
        // Ticket #333 (version 0.09.0): the unit is a divisor in every people figure the interface
        // prints, and the Research divisor is one in every Lab's yield.
        if self.climate.people_per_unit <= 0.0 {
            return Err(err("climate.toml", "people_per_unit must be positive"));
        }
        if self.population_factor.population_per_point <= 0.0 {
            return Err(err("facilities.toml", "[population_factor] population_per_point must be positive"));
        }
        // Ticket #295 (version 0.08.6): a divisor of nought would be a certain escape at any damage.
        if self.disengage.divisor <= 0.0 {
            return Err(err("units.toml", "[disengage] divisor must be positive"));
        }
        // Ticket #327: a melee of no rounds or no rolls is no melee.
        if self.melee.rounds == 0 || self.melee.rolls == 0 {
            return Err(err("units.toml", "[melee] rounds and rolls must both be at least 1"));
        }
        for s in &self.states {
            if s.unrest < 0.0 || s.unrest > u.max {
                return Err(err("nation_states.toml", format!("row {}: unrest {} is outside 0..={}", s.name, s.unrest, u.max)));
            }
        }
        if self.victory.turns == 0 {
            return Err(err("victory.toml", "turns must be positive"));
        }
        // Ticket #57: the game's first date, and the sky it opens on.
        if !(1..=12).contains(&self.victory.start_month) {
            return Err(err("victory.toml", format!("start_month {} is no month", self.victory.start_month)));
        }
        if self.shortlist.size < 2 {
            return Err(err("techs.toml", "[shortlist] size must be at least 2: a list of one is not a choice"));
        }
        if self.slots.per_colonist == 0 {
            return Err(err("modules.toml", "[slots] per_colonist must be at least 1: a Colonist has to buy something"));
        }
        // Ticket #324: a Module that fights needs hit points to lose, or the first hit is its last.
        if let Some(m) = self.modules.iter().find(|m| m.strength > 0 && m.hit_points == 0) {
            return Err(err("modules.toml", format!("[[module]] {} has strength and no hit_points", m.name)));
        }
        if self.archive.research <= 0 || !(0.0..=1.0).contains(&self.archive.banked_before_built) {
            return Err(err("modules.toml", "[archive] needs research above zero and banked_before_built from 0 to 1"));
        }
        if !(1..=12).contains(&self.victory.months_per_turn) {
            return Err(err("victory.toml", format!("months_per_turn {} must be from 1 to 12", self.victory.months_per_turn)));
        }
        if !(0.0..1.0).contains(&self.slot_yield_spread) {
            return Err(err("bodies.toml", format!("slot_yield_spread {} must be at least 0 and under 1", self.slot_yield_spread)));
        }
        for id in [BodyId::Earth, BodyId::Mars, BodyId::Venus] {
            if !self.planets.iter().any(|p| p.id == id) {
                return Err(err("ephemeris.toml", format!("no [[planet]] row for {}: the sky needs Earth's elements, Mars's and Venus's", id.name())));
            }
        }
        for p in &self.planets {
            if p.a <= 0.0 || !(0.0..1.0).contains(&p.e) {
                return Err(err("ephemeris.toml", format!("row {}: a must be positive and e between 0 and 1", p.id.name())));
            }
        }
        let tr = &self.transit;
        if tr.days_at_window <= 0.0 || tr.days_per_turn <= 0.0 || tr.max_turns == 0 || tr.synodic_days <= 0.0 {
            return Err(err("ephemeris.toml", "[transit] needs days_at_window, days_per_turn, max_turns and synodic_days above zero"));
        }
        if tr.days_per_degree < 0.0 || tr.fuel_per_degree < 0.0 {
            return Err(err("ephemeris.toml", "[transit] days_per_degree and fuel_per_degree cannot be negative"));
        }
        if self.events.event.iter().map(|e| e.copies).sum::<u32>() == 0 {
            return Err(err("events.toml", "the deck has no cards; give some Event a copies count above zero"));
        }
        if !(0.0..=1.0).contains(&self.events.draw_chance_base) || self.events.draw_chance_step_degrees <= 0.0 {
            return Err(err("events.toml", "draw_chance_base must be between 0 and 1 and draw_chance_step_degrees positive"));
        }
        Ok(())
    }

    pub fn body(&self, id: BodyId) -> &BodyCard {
        &self.bodies[id.index()]
    }
    /// Ticket #57: the elements a Body reads its place in the sky from. A satellite reads its
    /// parent's row: at this scale the Moon stands where Earth stands, and Phobos where Mars does.
    pub fn planet(&self, id: BodyId) -> &PlanetElements {
        let want = match id {
            BodyId::Moon => BodyId::Earth,
            BodyId::Phobos | BodyId::Deimos => BodyId::Mars,
            other => other,
        };
        self.planets.iter().find(|p| p.id == want).expect("validate() checked Earth and Mars have rows")
    }
    pub fn state(&self, id: StateId) -> &StateCard {
        &self.states[id.index()]
    }

    /// The start state for the next AI seat (spec 14.3, ticket #50): the free Nation State that is
    /// NOT adjacent to any state already taken, with the highest Industry Level, ties by population;
    /// if every free state touches a taken one, the highest Industry Level free state, ties by
    /// population. A tie the population does not settle keeps the table's order.
    ///
    /// Ticket #64: it lives on the tables rather than on a game, because a spectated game has to
    /// pick seat 0's start by this same rule before there is a game to ask.
    pub fn ai_start_state(&self, taken: &[StateId]) -> StateId {
        let adjacent: Vec<StateId> = taken.iter().flat_map(|t| self.state(*t).neighbours.iter().copied()).collect();
        let free: Vec<&StateCard> = self.states.iter().filter(|c| !taken.contains(&c.id)).collect();
        let best = |list: &[&StateCard]| -> Option<StateId> {
            let mut best: Option<&StateCard> = None;
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
        let spread: Vec<&StateCard> = free.iter().copied().filter(|c| !adjacent.contains(&c.id)).collect();
        best(&spread).or_else(|| best(&free)).unwrap_or(StateId::EastAsia)
    }
    pub fn facility(&self, kind: FacilityKind) -> &FacilityCard {
        &self.facilities[kind as usize]
    }
    pub fn module(&self, kind: ModuleKind) -> &ModuleCard {
        &self.modules[kind as usize]
    }
    pub fn unit(&self, kind: UnitKind) -> &UnitCard {
        &self.units[kind as usize]
    }
    pub fn tech(&self, id: TechId) -> &TechCard {
        &self.techs[id.index()]
    }
    pub fn event(&self, id: EventId) -> &EventCard {
        &self.events.event[id as usize]
    }
    pub fn faction(&self, kind: FactionKind) -> &FactionCard {
        &self.factions[kind as usize]
    }
    pub fn ai_weights(&self, kind: FactionKind) -> &AiWeights {
        &self.ai.weights[&kind]
    }
    pub fn ai_pace(&self, kind: FactionKind) -> &AiPace {
        &self.ai.pace[&kind]
    }
    pub fn ai_tech_picks(&self, kind: FactionKind) -> &AiTechPicks {
        &self.ai.tech_picks[&kind]
    }

    /// Ticket #84 (version 0.06.0): the Tech that opens this Faction's Victory Condition, if one does.
    pub fn victory_gate(&self, kind: FactionKind) -> Option<TechId> {
        self.techs.iter().find(|t| t.gate_for == Some(kind)).map(|t| t.id)
    }
}

fn check_rows<T: Copy + PartialEq + fmt::Debug>(
    file: &str,
    expected: &[T],
    actual: impl Iterator<Item = T>,
) -> Result<(), DataError> {
    let actual: Vec<T> = actual.collect();
    if actual.len() != expected.len() {
        return Err(err(file, format!("expected {} rows, found {}", expected.len(), actual.len())));
    }
    for (i, want) in expected.iter().enumerate() {
        if actual[i] != *want {
            return Err(err(file, format!("row {}: expected id {:?}, found {:?}", i + 1, want, actual[i])));
        }
    }
    Ok(())
}

/// Where the tables live relative to the executable or the workspace: `assets/data`.
pub fn default_data_dir() -> PathBuf {
    if let Ok(root) = std::env::var("DYING_EARTH_ASSETS") {
        return PathBuf::from(root).join("data");
    }
    let exe_side = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join("assets").join("data")));
    if let Some(p) = exe_side.filter(|p| p.is_dir()) {
        return p;
    }
    // Development: the workspace root holds `assets/`.
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest.parent().map(|p| p.join("assets").join("data")).unwrap_or_else(|| PathBuf::from("assets/data"))
}

/// Ticket #132 (version 0.07.3): the figures a start is chosen on, read from the cards alone. The
/// start globe's Region panel shows a Region's Ducats a turn and its Emissions before any game
/// exists, so these are computed from the card as the game will compute them at turn 1 -- and the
/// Ducats formula lives HERE, in one place, so that `Game::state_ducats` and the panel can never
/// disagree and the ticket that moves the formula moves one line.
impl Tables {
    /// Ticket #333 (version 0.09.0): units in a hundred million people, since the cards quote
    /// per-person Emissions at that rate: a hundred at one million a unit, twenty at five.
    pub fn units_per_hundred_million(&self) -> f64 {
        100_000_000.0 / self.climate.people_per_unit
    }

    /// A population figure written as real people: `1.94B`, `380M`, `1M`. Ticket #143 (version
    /// 0.07.3) on `Game` with the unit a code constant; ticket #333 (version 0.09.0) moved it here,
    /// where the start screen, which has no game yet, can read it too.
    pub fn people_text(&self, units: f64) -> String {
        let people = units * self.climate.people_per_unit;
        if people >= 1_000_000_000.0 {
            format!("{:.2}B", people / 1_000_000_000.0)
        } else {
            format!("{:.0}M", people / 1_000_000.0)
        }
    }

    /// The card's form: the figure in units to one decimal, and the real number beside it,
    /// `1454.5 (1.45B)`.
    pub fn population_text(&self, units: f64) -> String {
        format!("{units:.1} ({})", self.people_text(units))
    }

    /// The base Ducats a Region's economy pays a turn at an Industry Level. Ticket #35 set it at
    /// GDP x Industry Level / 10, rounded down, under which ten of the fourteen Regions paid nothing
    /// at the start; ticket #139 (version 0.07.3) made it **GDP x Industry Level / 5, rounded down,
    /// never below 1** -- the designer: *"Saudi Arabia can't pay 0"* -- so every Region pays, the
    /// rich pay double, and a small economy pays a flat one until GDP x Industry reaches 10.
    pub fn base_ducats(&self, sid: StateId, industry_level: u32) -> i64 {
        ((self.state(sid).gdp * industry_level as i64) / 5).max(1)
    }

    /// What a Region's economy would pay `faction` a turn as the game opens: the base at the card's
    /// Industry Level, times the Faction's Ducats multiplier (ticket #83).
    pub fn start_ducats(&self, sid: StateId, faction: FactionKind) -> i64 {
        (self.base_ducats(sid, self.state(sid).industry_level) as f64 * self.faction(faction).ducats_multiplier).floor() as i64
    }

    /// What a Region emits a turn as the game opens under `faction`: its industry, its people and
    /// its start Facilities, each times the Faction's Emissions multiplier, as `emissions_now` will
    /// count them at turn 1 before any Tech, Strip Permit or Leapfrog has moved a figure.
    pub fn start_emissions(&self, sid: StateId, faction: FactionKind) -> f64 {
        let card = self.state(sid);
        let c = &self.climate;
        let m = self.faction(faction).emissions_multiplier;
        let industry = card.baseline_emissions * card.industry_level as f64 * m;
        let people = (c.population_emissions_base + c.population_emissions_per_level * card.industry_level as f64) * card.population * m;
        let facilities: f64 = card.start_facilities.iter().map(|k| self.facility(*k).emissions * m).sum();
        industry + people + facilities
    }
}


/// Ticket #220 (version 0.08.2): defaults for a table written before the market moved.
fn price_band_default() -> i64 {
    1
}

fn price_step_default() -> i64 {
    20
}


/// Ticket #221 (version 0.08.2): a Faction card written before resentment existed minds at the
/// ordinary rate.
fn resentment_default() -> f64 {
    1.0
}


/// Ticket #224 (version 0.08.2).
fn relations_margin_cap_default() -> i64 {
    2
}


/// Ticket #226 (version 0.08.2).
fn tribute_ducats_default() -> i64 {
    25
}
fn tribute_materials_default() -> i64 {
    15
}
fn accord_kept_default() -> u32 {
    4
}


/// Ticket #227 (version 0.08.2).
fn accord_weight_default() -> f64 {
    3.0
}

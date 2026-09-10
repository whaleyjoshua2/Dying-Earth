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
    pub mine_yield: f64,
    pub generator_yield: f64,
    pub refinery_yield: f64,
    pub habitat_yield: f64,
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
    pub build_turns: u32,
    pub energy_upkeep: i64,
    pub produces: Option<Produces>,
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
    pub build_turns: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModuleCard {
    pub id: ModuleKind,
    pub name: String,
    pub materials: i64,
    pub build_turns: u32,
    pub energy_upkeep: i64,
    pub produces: Option<Produces>,
    #[serde(default)]
    pub holds_colonists: u32,
    #[serde(default)]
    pub influence_allotment: i64,
    #[serde(default)]
    pub standing_per_turn: i64,
    /// Version 0.04 (ticket #44): what the Module emits on Earth, as its counterpart Facility does.
    #[serde(default)]
    pub earth_emissions: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnitCard {
    pub id: UnitKind,
    pub name: String,
    pub materials: i64,
    pub build_turns: u32,
    pub energy_upkeep: i64,
    pub strength: i64,
    pub hit_points: u32,
    pub pursuit: u32,
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
    #[serde(default)]
    pub influence_threshold_multiplier: Option<f64>,
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
    pub event: Vec<EventCard>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactionCard {
    pub id: FactionKind,
    pub name: String,
    pub blurb: String,
    pub output_multiplier: f64,
    pub emissions_multiplier: f64,
    pub research_multiplier: f64,
    pub influence_multiplier: f64,
    pub signature: String,
    /// The Victory Condition in prose, for the cards and the panel.
    pub victory: String,
    /// Ticket #50: the first part of the Victory Condition, in figures.
    pub victory_first: VictoryFirstCard,
    /// Ticket #51: the second part, generalised the way #50 generalised the first.
    pub victory_second: VictorySecondCard,
    pub colour: [f32; 3],
    /// Ticket #46: the station over Earth the Faction starts with, by name in bodies.toml.
    /// Ticket #50: the Arkwrights start with none, so this is optional.
    #[serde(default)]
    pub start_station: Option<String>,
    // Ticket #51: the per-Faction figures the Arkwrights' card carries. Every one is neutral by
    // default, so a card that names none plays exactly as it did before.
    /// What a Habitat here holds, times this.
    #[serde(default = "one_f64")]
    pub habitat_capacity_multiplier: f64,
    /// Transit Fuel times this, before Efficient Transit.
    #[serde(default = "one_f64")]
    pub transit_fuel_multiplier: f64,
    /// Steerage: what a Colony Ship carries, times this.
    #[serde(default = "one_f64")]
    pub colony_ship_capacity_multiplier: f64,
    /// Steerage: the population a lift from a Launch Site takes, times this.
    #[serde(default = "one_f64")]
    pub lift_population_multiplier: f64,
    /// Steerage: what a Colony Ship costs, in place of the units.toml figure.
    #[serde(default)]
    pub colony_ship_materials: Option<i64>,
    /// A Space Station's Materials, times this.
    #[serde(default = "one_f64")]
    pub station_materials_multiplier: f64,
    /// A Colony Module's Materials, times this.
    #[serde(default = "one_f64")]
    pub module_materials_multiplier: f64,
}

fn one_f64() -> f64 {
    1.0
}

/// Ticket #50: which measure a Faction's first Victory part counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VictoryFirstKind {
    ExtractionTotal,
    StabilizationRun,
    ColonistsOffEarth,
    ResearchProduced,
    /// Ticket #51: stages of the Archive complete; complete counts only while it is online.
    ArchiveStages,
}

impl VictoryFirstKind {
    pub fn name(self) -> &'static str {
        match self {
            VictoryFirstKind::ExtractionTotal => "Extraction Total",
            VictoryFirstKind::StabilizationRun => "Stabilization run",
            VictoryFirstKind::ColonistsOffEarth => "Colonists off Earth",
            VictoryFirstKind::ResearchProduced => "Research produced",
            VictoryFirstKind::ArchiveStages => "The Archive",
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
    ColonistsAtArchive,
}

impl VictorySecondKind {
    pub fn name(self) -> &'static str {
        match self {
            VictorySecondKind::OffWorldPresence => "Off-world Presence",
            VictorySecondKind::ColoniesOnBodies => "Bodies settled",
            VictorySecondKind::ColonistsAtArchive => "Colonists at the Archive",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct VictorySecondCard {
    pub kind: VictorySecondKind,
    /// For off_world_presence and colonists_at_archive.
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
pub struct DucatsCard {
    pub per_influence: i64,
    /// Ticket #54: what one Leapfrog costs the Custodians; it replaced `per_restoration_step`.
    pub per_leapfrog: i64,
    pub per_repair_point: i64,
    /// Version 0.04 (ticket #42): the trading window's prices.
    pub per_materials: i64,
    pub per_fuel: i64,
    pub per_energy: i64,
    pub sell_divisor: i64,
    pub per_building_material: i64,
    pub bank_per_gdp_tenth: f64,
    pub trade_post_base: f64,
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
    pub population_loss_per_tenth_degree: f64,
    /// Ticket #54: Population Emissions per hundred million are `base + per_level x Industry Level`,
    /// less the state's own Leapfrog adjustment, never below `base`.
    pub population_emissions_base: f64,
    pub population_emissions_per_level: f64,
    pub launch_emissions: f64,
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
    /// Ticket #46: a station's threshold starts here.
    #[serde(default)]
    pub station_threshold_base: i64,
    pub occupation_turns: u32,
    pub destruction_chance: f64,
    /// Ticket #53: what a Faction's share of the table's Blame does to its Influence thresholds.
    pub blame: BlameTable,
}

/// Ticket #53 (version 0.05): Blame, in `influence.toml` under `[blame]`.
#[derive(Debug, Clone, Deserialize)]
pub struct BlameTable {
    /// The share of the table's Blame that costs a Faction nothing: an even quarter of four seats.
    pub fair_share: f64,
    /// The most a Faction's thresholds can be multiplied by, however dirty it is.
    pub cap: f64,
}

/// Ticket #52 (version 0.05): every number that moves a Nation State's Unrest (`unrest.toml`).
#[derive(Debug, Clone, Deserialize)]
pub struct UnrestTable {
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
    pub occupation_start: f64,
    pub occupation_per_turn: f64,
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
    pub extraction_total: i64,
    pub stabilization_turns: u32,
    pub off_world_presence: u32,
    pub turns: u32,
    /// Ticket #57: the game begins on the first of this month, and a Turn is a calendar month.
    #[serde(default = "twenty_thirty")]
    pub start_year: i64,
    #[serde(default = "january")]
    pub start_month: i64,
}

fn twenty_thirty() -> i64 {
    2030
}
fn january() -> i64 {
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
}

#[derive(Debug, Clone, Deserialize)]
pub struct AiWeights {
    pub build_producer: f64,
    /// Ticket #56: raise a Sea Wall in a coastal slot before the sea takes it.
    pub build_sea_wall: f64,
    pub raise_industry: f64,
    pub build_research_lab: f64,
    pub build_habitat: f64,
    pub build_launch_site_or_shipyard: f64,
    pub build_colony_ship: f64,
    pub build_warship: f64,
    pub build_army_or_barracks: f64,
    /// Ticket #36: an Embassy or a Relay.
    pub build_influence: f64,
    /// Ticket #51: divert this turn's Research into the Archive fund.
    pub fund_archive: f64,
    /// Ticket #51: order the next stage of the Archive.
    pub build_archive_stage: f64,
    /// Ticket #52: pay Relief on a state the seat directs.
    pub relief: f64,
    /// Ticket #52: raise a Constabulary in a restive state.
    pub build_constabulary: f64,
    /// Ticket #52: steer this turn's refugee flows into one calm state.
    pub resettle: f64,
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
}



#[derive(Debug, Clone, Deserialize)]
pub struct AiMultipliers {
    pub victory_gap_max: f64,
    pub threat: f64,
    pub opportunity: f64,
    pub energy_shortage_bonus: f64,
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
    pub evade_damage_fraction: f64,
    pub influence_step: i64,
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
    scrubber: ScrubberCard,
    mothball: MothballCard,
}
/// Ticket #51: the Archive, the first Project. Its Materials, build turns and Energy upkeep sit on
/// its Module row; how many stages it has and what each costs in Research live here.
#[derive(Debug, Clone, Deserialize)]
pub struct ArchiveCard {
    pub stages: u32,
    pub research_per_stage: i64,
}

#[derive(Debug, Clone, Deserialize)]
struct ModulesFile {
    module: Vec<ModuleCard>,
    archive: ArchiveCard,
}
#[derive(Debug, Clone, Deserialize)]
struct UnitsFile {
    unit: Vec<UnitCard>,
    repair: RepairCard,
}
#[derive(Debug, Clone, Deserialize)]
struct TechsFile {
    tech: Vec<TechCard>,
}
#[derive(Debug, Clone, Deserialize)]
struct FactionsFile {
    faction: Vec<FactionCard>,
    start: StartCard,
    ducats: DucatsCard,
}

/// Every table, loaded and checked.
#[derive(Debug, Clone)]
pub struct Tables {
    pub bodies: Vec<BodyCard>,
    /// Ticket #45: the hop between two satellites of the same Body.
    pub sibling_transit: (u32, i64),
    /// Ticket #46: what a station costs.
    pub station_materials: i64,
    /// Ticket #57: how far a Colony Slot's own four yields may fall either side of its Body's.
    pub slot_yield_spread: f64,
    /// Ticket #57: the Keplerian elements of Earth and Mars, and the transit table (`ephemeris.toml`).
    pub planets: Vec<PlanetElements>,
    pub transit: TransitTable,
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
    /// Ticket #54: the Scrubber cap and the Mothball prices (`facilities.toml`).
    pub scrubber: ScrubberCard,
    pub mothball: MothballCard,
    pub modules: Vec<ModuleCard>,
    /// Ticket #51: the Archive's stages and their Research price.
    pub archive: ArchiveCard,
    pub units: Vec<UnitCard>,
    pub repair: RepairCard,
    pub techs: Vec<TechCard>,
    pub events: EventsTable,
    pub factions: Vec<FactionCard>,
    pub start: StartCard,
    pub ducats: DucatsCard,
    pub climate: ClimateTable,
    pub influence: InfluenceTable,
    /// Ticket #52: `unrest.toml`.
    pub unrest: UnrestTable,
    pub victory: VictoryTable,
    pub ai: AiTable,
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
        let tables = Tables {
            sibling_transit: (bodies.sibling_turns, bodies.sibling_fuel),
            station_materials: bodies.station_materials,
            slot_yield_spread: bodies.slot_yield_spread,
            planets: ephemeris.planet,
            transit: ephemeris.transit,
            bodies: bodies.body,
            states: states.state,
            development: states.development,
            strip_permit: states.strip_permit,
            base_slots: states.base_slots,
            coastal_per_exposure: states.coastal_per_exposure,
            facilities: facilities.facility,
            industry_level: facilities.industry_level,
            scrubber: facilities.scrubber,
            mothball: facilities.mothball,
            archive: modules.archive,
            modules: modules.module,
            units: units.unit,
            repair: units.repair,
            techs: techs.tech,
            events,
            factions: factions.faction,
            start: factions.start,
            ducats: factions.ducats,
            climate,
            influence,
            unrest,
            victory,
            ai,
        };
        tables.validate()?;
        Ok(tables)
    }

    fn validate(&self) -> Result<(), DataError> {
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
            if f.victory_first.bar <= 0.0 {
                return Err(err("factions.toml", format!("row {}: victory_first.bar must be positive", f.name)));
            }
            // Ticket #51: whichever second part a card names, its own figures must be positive.
            let second_ok = match f.victory_second.kind {
                VictorySecondKind::OffWorldPresence | VictorySecondKind::ColonistsAtArchive => f.victory_second.bar > 0.0,
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
        if self.archive.stages == 0 || self.archive.research_per_stage <= 0 {
            return Err(err("modules.toml", "[archive] needs stages and research_per_stage above zero"));
        }
        // Ticket #52: the Unrest ladder must be in order and every start value on it.
        let u = &self.unrest;
        if !(u.army_threshold < u.facility_threshold && u.facility_threshold < u.throw_off_threshold && u.throw_off_threshold <= u.max) {
            return Err(err("unrest.toml", "the thresholds must rise: army_threshold < facility_threshold < throw_off_threshold <= max"));
        }
        if u.neutral_max > u.max || u.refugees_per <= 0.0 {
            return Err(err("unrest.toml", "neutral_max must not exceed max, and refugees_per must be positive"));
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
        if !(0.0..1.0).contains(&self.slot_yield_spread) {
            return Err(err("bodies.toml", format!("slot_yield_spread {} must be at least 0 and under 1", self.slot_yield_spread)));
        }
        for id in [BodyId::Earth, BodyId::Mars] {
            if !self.planets.iter().any(|p| p.id == id) {
                return Err(err("ephemeris.toml", format!("no [[planet]] row for {}: the sky needs Earth's elements and Mars's", id.name())));
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

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
    pub unrest: i64,
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
    pub permafrost_emissions: f64,
    pub meteor_damage: u32,
    /// Ticket #52: the Unrest card is a flat rise in the state's Unrest; the Army damage and the
    /// Standing loss it carried until version 0.05 are gone.
    pub unrest_card_unrest: i64,
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
pub struct RestorationCard {
    pub energy_per_step: i64,
    pub sink_per_step: f64,
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
    pub per_restoration_step: i64,
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
    pub population_emissions_per_hundred_million: f64,
    pub launch_emissions: f64,
    pub sea_level_thresholds: Vec<f64>,
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
}

/// Ticket #52 (version 0.05): every number that moves a Nation State's Unrest (`unrest.toml`).
#[derive(Debug, Clone, Deserialize)]
pub struct UnrestTable {
    pub max: i64,
    pub neutral_max: i64,
    pub population_fall: i64,
    pub population_fall_big: i64,
    pub population_fall_big_fraction: f64,
    pub per_sea_level_slot: i64,
    pub climate_card: i64,
    pub per_mothball: i64,
    pub per_decommission: i64,
    pub refugees_per: f64,
    pub refugees_max: i64,
    pub occupation_start: i64,
    pub occupation_per_turn: i64,
    pub unrest_card: i64,
    pub natural_fall: i64,
    pub relief_ducats: i64,
    pub relief_points: i64,
    pub constabulary_fall: i64,
    /// The hook a Scrubber joins on its own ticket; nothing reads it yet.
    pub scrubber_fall: i64,
    pub green_techs_two: i64,
    pub green_techs_four: i64,
    pub constabulary_damping: i64,
    pub army_threshold: i64,
    pub facility_threshold: i64,
    pub throw_off_threshold: i64,
    pub throw_off_reset: i64,
    pub no_development_at: i64,
    pub pacification_divisor: i64,
    pub pacification_divisor_unrest: i64,
    pub pacification_unrest: i64,
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
}

#[derive(Debug, Clone, Deserialize)]
pub struct AiWeights {
    pub build_producer: f64,
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
    pub restoration: f64,
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
}
fn forty() -> i64 {
    40
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
}
#[derive(Debug, Clone, Deserialize)]
struct FacilitiesFile {
    facility: Vec<FacilityCard>,
    industry_level: IndustryLevelCard,
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
    restoration: RestorationCard,
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
    pub states: Vec<StateCard>,
    pub facilities: Vec<FacilityCard>,
    pub industry_level: IndustryLevelCard,
    pub modules: Vec<ModuleCard>,
    /// Ticket #51: the Archive's stages and their Research price.
    pub archive: ArchiveCard,
    pub units: Vec<UnitCard>,
    pub repair: RepairCard,
    pub techs: Vec<TechCard>,
    pub events: EventsTable,
    pub factions: Vec<FactionCard>,
    pub restoration: RestorationCard,
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
        let tables = Tables {
            sibling_transit: (bodies.sibling_turns, bodies.sibling_fuel),
            station_materials: bodies.station_materials,
            bodies: bodies.body,
            states: states.state,
            facilities: facilities.facility,
            industry_level: facilities.industry_level,
            archive: modules.archive,
            modules: modules.module,
            units: units.unit,
            repair: units.repair,
            techs: techs.tech,
            events,
            factions: factions.faction,
            restoration: factions.restoration,
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
            if s.start_facilities.len() as u32 + 1 > s.size + s.industry_level {
                return Err(err("nation_states.toml", format!("row {}: {} start_facilities do not fit its {} build slots with a Launch Site", s.name, s.start_facilities.len(), s.size + s.industry_level)));
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
            if s.unrest < 0 || s.unrest > u.max {
                return Err(err("nation_states.toml", format!("row {}: unrest {} is outside 0..={}", s.name, s.unrest, u.max)));
            }
        }
        if self.victory.turns == 0 {
            return Err(err("victory.toml", "turns must be positive"));
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

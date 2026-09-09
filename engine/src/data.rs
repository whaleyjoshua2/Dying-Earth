//! The data tables under `assets/data`, one TOML file per table (spec 2.3).
//! Loaded once at startup and validated; a malformed table refuses to start,
//! naming the file and the row.

use crate::ids::*;
use serde::Deserialize;
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

#[derive(Debug, Clone, Deserialize)]
pub struct BodyCard {
    pub id: BodyId,
    pub name: String,
    pub colony_slots: u32,
    pub transit_turns: u32,
    pub transit_fuel: i64,
    pub mine_yield: f64,
    pub generator_yield: f64,
    pub refinery_yield: f64,
    pub habitat_yield: f64,
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
    pub unrest_army_damage: u32,
    pub unrest_influence_loss: i64,
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
    pub victory: String,
    pub colour: [f32; 3],
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
    pub occupation_turns: u32,
    pub destruction_chance: f64,
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
pub struct AiWeightsByFaction {
    pub prospectors: AiWeights,
    pub custodians: AiWeights,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AiMultipliers {
    pub victory_gap_max: f64,
    pub denial: f64,
    pub threat: f64,
    pub opportunity: f64,
    pub energy_shortage_bonus: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AiPace {
    pub prospector_extraction: Vec<[i64; 2]>,
    pub custodian_within_ppm: f64,
    pub custodian_within_by_turn: u32,
    pub custodian_under_sink_by_turn: u32,
    pub colonists: Vec<[i64; 2]>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AiThresholds {
    pub attack_odds: f64,
    pub attack_odds_versus_near_winner: f64,
    pub evade_damage_fraction: f64,
    pub influence_step: i64,
    pub near_win_fraction: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AiTechPicks {
    pub prospectors: Vec<TechId>,
    pub prospectors_last: TechId,
    pub prospectors_never: TechId,
    pub custodians: Vec<TechId>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AiTable {
    pub weights: AiWeightsByFaction,
    pub multipliers: AiMultipliers,
    pub pace: AiPace,
    pub thresholds: AiThresholds,
    pub tech_picks: AiTechPicks,
}

#[derive(Debug, Clone, Deserialize)]
struct BodiesFile {
    body: Vec<BodyCard>,
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
#[derive(Debug, Clone, Deserialize)]
struct ModulesFile {
    module: Vec<ModuleCard>,
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
}

/// Every table, loaded and checked.
#[derive(Debug, Clone)]
pub struct Tables {
    pub bodies: Vec<BodyCard>,
    pub states: Vec<StateCard>,
    pub facilities: Vec<FacilityCard>,
    pub industry_level: IndustryLevelCard,
    pub modules: Vec<ModuleCard>,
    pub units: Vec<UnitCard>,
    pub repair: RepairCard,
    pub techs: Vec<TechCard>,
    pub events: EventsTable,
    pub factions: Vec<FactionCard>,
    pub restoration: RestorationCard,
    pub start: StartCard,
    pub climate: ClimateTable,
    pub influence: InfluenceTable,
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
        let victory: VictoryTable = read(dir, "victory.toml")?;
        let ai: AiTable = read(dir, "ai.toml")?;
        let tables = Tables {
            bodies: bodies.body,
            states: states.state,
            facilities: facilities.facility,
            industry_level: facilities.industry_level,
            modules: modules.module,
            units: units.unit,
            repair: units.repair,
            techs: techs.tech,
            events,
            factions: factions.faction,
            restoration: factions.restoration,
            start: factions.start,
            climate,
            influence,
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
            &[UnitKind::ColonyShip, UnitKind::Frigate, UnitKind::Battleship, UnitKind::Army],
            self.units.iter().map(|u| u.id),
        )?;
        check_rows("techs.toml", &TechId::ALL, self.techs.iter().map(|t| t.id))?;
        check_rows("events.toml", &EventId::ALL, self.events.event.iter().map(|e| e.id))?;
        check_rows(
            "factions.toml",
            &[FactionKind::Custodians, FactionKind::Prospectors],
            self.factions.iter().map(|f| f.id),
        )?;
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
        match kind {
            FactionKind::Custodians => &self.ai.weights.custodians,
            FactionKind::Prospectors => &self.ai.weights.prospectors,
        }
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

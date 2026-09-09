//! The fixed vocabulary of the First Playable: every id a data table may name.
//! Names follow `CONTEXT.md`.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BodyId {
    Earth,
    Moon,
    Mars,
}

impl BodyId {
    pub const ALL: [BodyId; 3] = [BodyId::Earth, BodyId::Moon, BodyId::Mars];
    pub fn index(self) -> usize {
        self as usize
    }
    pub fn name(self) -> &'static str {
        match self {
            BodyId::Earth => "Earth",
            BodyId::Moon => "the Moon",
            BodyId::Mars => "Mars",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateId {
    Africa,
    Antarctica,
    Asia,
    Australia,
    Europe,
    NorthAmerica,
    SouthAmerica,
    // Version 0.02 (ticket #26): two more, cut from Europe and Asia.
    Russia,
    MiddleEast,
}

impl StateId {
    pub const ALL: [StateId; 9] = [
        StateId::Africa,
        StateId::Antarctica,
        StateId::Asia,
        StateId::Australia,
        StateId::Europe,
        StateId::NorthAmerica,
        StateId::SouthAmerica,
        StateId::Russia,
        StateId::MiddleEast,
    ];
    pub fn index(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FacilityKind {
    Factory,
    PowerPlant,
    Refinery,
    ResearchLab,
    LaunchSite,
}

impl FacilityKind {
    pub const ALL: [FacilityKind; 5] = [
        FacilityKind::Factory,
        FacilityKind::PowerPlant,
        FacilityKind::Refinery,
        FacilityKind::ResearchLab,
        FacilityKind::LaunchSite,
    ];
    pub fn name(self) -> &'static str {
        match self {
            FacilityKind::Factory => "Factory",
            FacilityKind::PowerPlant => "Power Plant",
            FacilityKind::Refinery => "Refinery",
            FacilityKind::ResearchLab => "Research Lab",
            FacilityKind::LaunchSite => "Launch Site",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleKind {
    Mine,
    Generator,
    Refinery,
    Habitat,
    Shipyard,
    Barracks,
}

impl ModuleKind {
    pub const ALL: [ModuleKind; 6] = [
        ModuleKind::Mine,
        ModuleKind::Generator,
        ModuleKind::Refinery,
        ModuleKind::Habitat,
        ModuleKind::Shipyard,
        ModuleKind::Barracks,
    ];
    pub fn name(self) -> &'static str {
        match self {
            ModuleKind::Mine => "Mine",
            ModuleKind::Generator => "Generator",
            ModuleKind::Refinery => "Refinery",
            ModuleKind::Habitat => "Habitat",
            ModuleKind::Shipyard => "Shipyard",
            ModuleKind::Barracks => "Barracks",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnitKind {
    ColonyShip,
    Frigate,
    Battleship,
    Army,
}

impl UnitKind {
    pub const SHIPS: [UnitKind; 3] = [UnitKind::ColonyShip, UnitKind::Frigate, UnitKind::Battleship];
    pub fn name(self) -> &'static str {
        match self {
            UnitKind::ColonyShip => "Colony Ship",
            UnitKind::Frigate => "Frigate",
            UnitKind::Battleship => "Battleship",
            UnitKind::Army => "Army",
        }
    }
    pub fn is_warship(self) -> bool {
        matches!(self, UnitKind::Frigate | UnitKind::Battleship)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Resource {
    Materials,
    Fuel,
    Energy,
    Research,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TechId {
    EfficientGrids,
    CleanPower,
    CleanManufacturing,
    CleanPropellant,
    EfficientTransit,
    HardenedHulls,
    ExpandedHabitats,
    ClosedLoopColonies,
    DeepMining,
    AutomatedRefining,
    PublicScience,
    GreenConsensus,
}

impl TechId {
    pub const ALL: [TechId; 12] = [
        TechId::EfficientGrids,
        TechId::CleanPower,
        TechId::CleanManufacturing,
        TechId::CleanPropellant,
        TechId::EfficientTransit,
        TechId::HardenedHulls,
        TechId::ExpandedHabitats,
        TechId::ClosedLoopColonies,
        TechId::DeepMining,
        TechId::AutomatedRefining,
        TechId::PublicScience,
        TechId::GreenConsensus,
    ];
    pub fn index(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventId {
    SolarStorm,
    RadiationSurge,
    CommsBlackout,
    /// Version 0.03 (ticket #32): replaced Equipment Failure and Launch Failure, which singled out a Faction.
    LaunchPadFire,
    LabourDispute,
    GridFailure,
    RichSeam,
    IceDeposit,
    Breakthrough,
    Heatwave,
    Wildfire,
    StormSurge,
    // Version 0.02 (ticket #25): six more.
    SolarMaximum,
    MeteorShower,
    DustStorm,
    Unrest,
    ReactorLeak,
    PermafrostThaw,
}

impl EventId {
    pub const ALL: [EventId; 18] = [
        EventId::SolarStorm,
        EventId::RadiationSurge,
        EventId::CommsBlackout,
        EventId::LaunchPadFire,
        EventId::LabourDispute,
        EventId::GridFailure,
        EventId::RichSeam,
        EventId::IceDeposit,
        EventId::Breakthrough,
        EventId::Heatwave,
        EventId::Wildfire,
        EventId::StormSurge,
        EventId::SolarMaximum,
        EventId::MeteorShower,
        EventId::DustStorm,
        EventId::Unrest,
        EventId::ReactorLeak,
        EventId::PermafrostThaw,
    ];
    pub const CLIMATE: [EventId; 4] = [EventId::Heatwave, EventId::Wildfire, EventId::StormSurge, EventId::PermafrostThaw];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    Solar,
    Failure,
    Discovery,
    Climate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FactionKind {
    Custodians,
    Prospectors,
}

impl FactionKind {
    pub fn name(self) -> &'static str {
        match self {
            FactionKind::Custodians => "Custodians",
            FactionKind::Prospectors => "Prospectors",
        }
    }
    pub fn other(self) -> FactionKind {
        match self {
            FactionKind::Custodians => FactionKind::Prospectors,
            FactionKind::Prospectors => FactionKind::Custodians,
        }
    }
}

/// One of the two seats at the table. Seat 0 is the player's seat (the one that wins ties).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Seat(pub u8);

impl Seat {
    pub const ALL: [Seat; 2] = [Seat(0), Seat(1)];
    pub fn other(self) -> Seat {
        Seat(1 - self.0)
    }
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ShipId(pub u32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ArmyId(pub u32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ColonyId(pub u32);

/// A ground place: a Nation State or a Colony.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Place {
    State(StateId),
    Colony(ColonyId),
}

/// Anything Influence can be spent on.
pub type Target = Place;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Stance {
    Attack,
    Hold,
    Intercept,
    Evade,
}

impl Stance {
    pub fn name(self) -> &'static str {
        match self {
            Stance::Attack => "Attack",
            Stance::Hold => "Hold",
            Stance::Intercept => "Intercept",
            Stance::Evade => "Evade",
        }
    }
}

impl fmt::Display for ShipId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ship {}", self.0)
    }
}
impl fmt::Display for ArmyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Army {}", self.0)
    }
}
impl fmt::Display for ColonyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Colony {}", self.0)
    }
}

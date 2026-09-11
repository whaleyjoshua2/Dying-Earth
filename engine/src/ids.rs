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
    /// Version 0.04 (ticket #45): the moons of Mars.
    Phobos,
    Deimos,
}

impl BodyId {
    pub const ALL: [BodyId; 5] = [BodyId::Earth, BodyId::Moon, BodyId::Mars, BodyId::Phobos, BodyId::Deimos];
    pub fn index(self) -> usize {
        self as usize
    }
    pub fn name(self) -> &'static str {
        match self {
            BodyId::Earth => "Earth",
            BodyId::Moon => "the Moon",
            BodyId::Mars => "Mars",
            BodyId::Phobos => "Phobos",
            BodyId::Deimos => "Deimos",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateId {
    // Version 0.05 (ticket #53): twelve Nation States. Africa split at the Sahara, Asia into three,
    // and Central America and the Caribbean cut out of North America. Antarctica stays off the list
    // (ticket #44): it is Earth's three Colony Slots.
    SubSaharanAfrica,
    NorthAfrica,
    EastAsia,
    SouthAsia,
    SouthEastAsia,
    Australia,
    Europe,
    NorthAmerica,
    CentralAmerica,
    SouthAmerica,
    // Version 0.02 (ticket #26): cut from Europe and Asia.
    Russia,
    MiddleEast,
}

impl StateId {
    pub const ALL: [StateId; 12] = [
        StateId::SubSaharanAfrica,
        StateId::NorthAfrica,
        StateId::EastAsia,
        StateId::SouthAsia,
        StateId::SouthEastAsia,
        StateId::Australia,
        StateId::Europe,
        StateId::NorthAmerica,
        StateId::CentralAmerica,
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
    /// Version 0.03 (ticket #35): makes Ducats.
    Bank,
    /// Version 0.03 (ticket #36): raises Influence.
    Embassy,
    /// Version 0.05 (ticket #52): lowers its state's Unrest and damps what raises it.
    Constabulary,
    /// Version 0.05 (ticket #54): the Custodians' signature Facility. Takes no build slot, enlarges
    /// the Natural Sink while it is online, and is destroyed if its state changes hands.
    Scrubber,
    /// Version 0.05 (ticket #56): the Sea Wall. Always stands in a coastal slot, at most one per
    /// Nation State, and it absorbs the state's next Sea Level threshold and is destroyed doing it.
    SeaWall,
}

impl FacilityKind {
    pub const ALL: [FacilityKind; 10] = [
        FacilityKind::Factory,
        FacilityKind::PowerPlant,
        FacilityKind::Refinery,
        FacilityKind::ResearchLab,
        FacilityKind::LaunchSite,
        FacilityKind::Bank,
        FacilityKind::Embassy,
        FacilityKind::Constabulary,
        FacilityKind::Scrubber,
        FacilityKind::SeaWall,
    ];
    pub fn name(self) -> &'static str {
        match self {
            FacilityKind::Factory => "Factory",
            FacilityKind::PowerPlant => "Power Plant",
            FacilityKind::Refinery => "Refinery",
            FacilityKind::ResearchLab => "Research Lab",
            FacilityKind::LaunchSite => "Launch Site",
            FacilityKind::Bank => "Bank",
            FacilityKind::Embassy => "Embassy",
            FacilityKind::Constabulary => "Constabulary",
            FacilityKind::Scrubber => "Scrubber",
            FacilityKind::SeaWall => "Sea Wall",
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
    /// Version 0.03 (ticket #35): makes Ducats off Earth.
    TradePost,
    /// Version 0.03 (ticket #36): raises Influence off Earth.
    Relay,
    /// Version 0.05 (ticket #51): the Archive. Only the Archivists build it, from its own button and
    /// never through the ordinary Module build order; one Module since ticket #68 (version 0.05.5).
    Archive,
}

impl ModuleKind {
    pub const ALL: [ModuleKind; 9] = [
        ModuleKind::Mine,
        ModuleKind::Generator,
        ModuleKind::Refinery,
        ModuleKind::Habitat,
        ModuleKind::Shipyard,
        ModuleKind::Barracks,
        ModuleKind::TradePost,
        ModuleKind::Relay,
        ModuleKind::Archive,
    ];
    /// The Modules an ordinary build order may place (ticket #51: the Archive is not one of them).
    pub const BUILDABLE: [ModuleKind; 8] = [
        ModuleKind::Mine,
        ModuleKind::Generator,
        ModuleKind::Refinery,
        ModuleKind::Habitat,
        ModuleKind::Shipyard,
        ModuleKind::Barracks,
        ModuleKind::TradePost,
        ModuleKind::Relay,
    ];
    pub fn name(self) -> &'static str {
        match self {
            ModuleKind::Mine => "Mine",
            ModuleKind::Generator => "Generator",
            ModuleKind::Refinery => "Refinery",
            ModuleKind::Habitat => "Habitat",
            ModuleKind::Shipyard => "Shipyard",
            ModuleKind::Barracks => "Barracks",
            ModuleKind::TradePost => "Trade Post",
            ModuleKind::Relay => "Relay",
            ModuleKind::Archive => "The Archive",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnitKind {
    ColonyShip,
    Frigate,
    Battleship,
    /// Version 0.04 (ticket #43): the transport for one Army; a Colony Ship carries Colonists only.
    Carrier,
    Army,
}

impl UnitKind {
    pub const SHIPS: [UnitKind; 4] = [UnitKind::ColonyShip, UnitKind::Carrier, UnitKind::Frigate, UnitKind::Battleship];
    pub fn name(self) -> &'static str {
        match self {
            UnitKind::ColonyShip => "Colony Ship",
            UnitKind::Frigate => "Frigate",
            UnitKind::Battleship => "Battleship",
            UnitKind::Carrier => "Carrier",
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
    /// Version 0.03 (ticket #35): money, which buys Influence, Restoration and repairs.
    Ducats,
}

impl Resource {
    pub fn name(self) -> &'static str {
        match self {
            Resource::Materials => "Materials",
            Resource::Fuel => "Fuel",
            Resource::Energy => "Energy",
            Resource::Research => "Research",
            Resource::Ducats => "Ducats",
        }
    }
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
    /// Version 0.05 (ticket #56): the thirteenth Tech, Industry rung 2 beside Clean Power. It
    /// unlocks the Sea Wall and nothing else.
    CoastalEngineering,
}

impl TechId {
    pub const ALL: [TechId; 13] = [
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
        TechId::CoastalEngineering,
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
    // Ticket #55 (version 0.05): the Permafrost Thaw card became the Methane Burst, so the name is
    // free for the Break that thaws the permafrost for good.
    MethaneBurst,
    // Ticket #76 (version 0.05.5): four more, for a deck of forty over thirty-six turns.
    Drought,
    VolcanicEruption,
    Moonquake,
    HeliumVein,
}

impl EventId {
    pub const ALL: [EventId; 22] = [
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
        EventId::MethaneBurst,
        EventId::Drought,
        EventId::VolcanicEruption,
        EventId::Moonquake,
        EventId::HeliumVein,
    ];
    pub const CLIMATE: [EventId; 6] = [EventId::Heatwave, EventId::Wildfire, EventId::StormSurge, EventId::MethaneBurst, EventId::Drought, EventId::VolcanicEruption];
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
    /// Version 0.05 (ticket #50): every game seats all four Factions.
    Arkwrights,
    Archivists,
}

impl FactionKind {
    pub const ALL: [FactionKind; 4] = [FactionKind::Custodians, FactionKind::Prospectors, FactionKind::Arkwrights, FactionKind::Archivists];
    pub fn name(self) -> &'static str {
        match self {
            FactionKind::Custodians => "Custodians",
            FactionKind::Prospectors => "Prospectors",
            FactionKind::Arkwrights => "Arkwrights",
            FactionKind::Archivists => "Archivists",
        }
    }
    /// The id the data tables and the command line use.
    pub fn id(self) -> &'static str {
        match self {
            FactionKind::Custodians => "custodians",
            FactionKind::Prospectors => "prospectors",
            FactionKind::Arkwrights => "arkwrights",
            FactionKind::Archivists => "archivists",
        }
    }
    /// Parse a Faction by its full id; nothing else is accepted, since two ids share a first letter.
    pub fn from_id(s: &str) -> Option<FactionKind> {
        FactionKind::ALL.into_iter().find(|k| k.id() == s.to_ascii_lowercase())
    }
    pub fn index(self) -> usize {
        self as usize
    }
}

/// How many seats every game has (ticket #50).
pub const SEAT_COUNT: usize = 4;

/// One of the four seats at the table. Seat 0 is the player's seat, whichever Faction it picked;
/// seats 1 to 3 hold the other three Factions in enum order. A seat wins no tie for being first:
/// every tie is drawn at random from the game's own generator (ticket #50).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Seat(pub u8);

impl Seat {
    pub const ALL: [Seat; SEAT_COUNT] = [Seat(0), Seat(1), Seat(2), Seat(3)];
    pub fn index(self) -> usize {
        self.0 as usize
    }
    /// Every other seat at the table.
    pub fn others(self) -> Vec<Seat> {
        Seat::ALL.into_iter().filter(|s| *s != self).collect()
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

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
    /// Version 0.06.0 (ticket #93): Venus, a Body of orbits only: no Colony Slots, three Orbital
    /// Slots, its own Launch Window on the real sky.
    Venus,
}

impl BodyId {
    pub const ALL: [BodyId; 6] = [BodyId::Earth, BodyId::Moon, BodyId::Mars, BodyId::Phobos, BodyId::Deimos, BodyId::Venus];
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
            BodyId::Venus => "Venus",
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
    // Version 0.07.2 (ticket #125): Japan and Korea cut out of East Asia, the Arabian Peninsula out
    // of the Middle East. The Regions are named for their Nations since ticket #122 -- the id keeps
    // the ground it covers, the card carries the name -- so these read `japan` and `arabian_peninsula`.
    Japan,
    ArabianPeninsula,
}

impl StateId {
    pub const ALL: [StateId; 14] = [
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
        StateId::Japan,
        StateId::ArabianPeninsula,
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
    /// Nation State, and it absorbs every Sea Level threshold that reaches the state. Ticket #257
    /// (version 0.08.4): it stands through them -- it was destroyed absorbing one before -- and each
    /// rise it has held adds to its keep.
    SeaWall,
    /// Version 0.08.0 (ticket #185): the School. At most one per Nation State. While it stands and is
    /// online it raises its state's Education Level by a step a turn to a ceiling, and the figure
    /// decays back at the same rate when it stops.
    School,
    /// Version 0.08.0 (ticket #182): the Investment Bank, the Prospectors' Unique Facility, which
    /// replaces the Bank on their build list at the common price and banks 1% of the Venture
    /// Capital Fund's balance into the Fund each turn, one per Region.
    InvestmentBank,
    /// Version 0.08.0 (ticket #183): the Spaceport, the Arkwrights' Unique Facility, which replaces
    /// the Launch Site and pays +1 Influence for every Emigrant it lifts off Earth.
    Spaceport,
    /// Version 0.08.0 (ticket #184): the Reactor, the Archivists' Unique Facility, which replaces
    /// the Power Plant and takes 75% off the total Energy upkeep of everything its holder owns,
    /// the Archive excepted.
    Reactor,
    /// Version 0.08.0 (ticket #186): the Academy, the Custodians' Unique Facility, which replaces
    /// the School and pays +1 Ducat a turn on top of the schooling. Off Earth it is a Unique
    /// Module of the same name, replacing the Institute.
    Academy,
}

impl FacilityKind {
    pub const ALL: [FacilityKind; 15] = [
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
        FacilityKind::School,
        FacilityKind::InvestmentBank,
        FacilityKind::Spaceport,
        FacilityKind::Reactor,
        FacilityKind::Academy,
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
            FacilityKind::School => "School",
            FacilityKind::InvestmentBank => "Investment Bank",
            FacilityKind::Spaceport => "Spaceport",
            FacilityKind::Reactor => "Reactor",
            FacilityKind::Academy => "Academy",
        }
    }

    /// Version 0.08.0 (ticket #181): the Faction this kind is the Unique Facility of, or `None` for
    /// a common kind. A Unique Facility replaces a common one on exactly one Faction's build list;
    /// it is never destroyed on capture and pays whoever holds it, which is why this says who
    /// BUILDS it and never who benefits.
    pub fn unique_to(self) -> Option<FactionKind> {
        match self {
            FacilityKind::InvestmentBank => Some(FactionKind::Prospectors),
            FacilityKind::Spaceport => Some(FactionKind::Arkwrights),
            FacilityKind::Reactor => Some(FactionKind::Archivists),
            FacilityKind::Academy => Some(FactionKind::Custodians),
            _ => None,
        }
    }

    /// The common kind a Unique Facility replaces, or `None` for a common kind. Used where a rule
    /// is about the building's JOB rather than its owner -- the Sea Level taking a coastal slot,
    /// a Tech that reads Power Plants -- so that a Reactor is a Power Plant everywhere but the
    /// build list.
    pub fn common(self) -> Option<FacilityKind> {
        match self {
            FacilityKind::InvestmentBank => Some(FacilityKind::Bank),
            FacilityKind::Spaceport => Some(FacilityKind::LaunchSite),
            FacilityKind::Reactor => Some(FacilityKind::PowerPlant),
            FacilityKind::Academy => Some(FacilityKind::School),
            _ => None,
        }
    }

    /// What `faction` builds when it orders this kind: its own Unique Facility where it has one for
    /// this job, otherwise the kind itself. This is the whole of the substitution -- a Faction never
    /// builds the common version of a job it has a Unique Facility for, and never builds another
    /// Faction's.
    pub fn built_by(self, faction: FactionKind) -> FacilityKind {
        for unique in FacilityKind::ALL {
            if unique.common() == Some(self) && unique.unique_to() == Some(faction) {
                return unique;
            }
        }
        self
    }

    /// True where this kind does the job the common `kind` does -- itself, or the Unique Facility
    /// that replaces it. Every count of "how many Power Plants stand here" wants this.
    pub fn does_the_job_of(self, kind: FacilityKind) -> bool {
        self == kind || self.common() == Some(kind)
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
    /// Version 0.06.0 (ticket #80): the Observatory, the one Module that makes Research, on a
    /// Colony or a Space Station; each Colonist at its Colony adds one per cent.
    Observatory,
    /// Version 0.06.0 (ticket #89): the Solar Array, a Module only a Space Station holds, making
    /// Energy that scales with the inverse square of its Body's distance from the Sun.
    SolarArray,
    /// Version 0.06.0 (ticket #92): the Mass Driver, a Module only a ground Colony on a low-gravity
    /// Body holds, behind Efficient Transit: the owner's departures four Fuel cheaper, its Mines +1.
    MassDriver,
    /// Version 0.07.5 (ticket #164): the **Core Module**, which every Colony and every Space Station
    /// is founded with. It holds four Colonists, is never ordered, never mothballed and never
    /// decommissioned, and stands outside the Module count as the Archive does. It is the walls of
    /// the place rather than a building in it. Appended last because `Tables::module` indexes this
    /// enum by discriminant.
    Core,
    /// Version 0.08.0 (ticket #185): the Institute, the School's form off Earth. At most one per
    /// Colony or Space Station; while it stands and is online it raises its place's Education Level
    /// a step a turn to the ceiling, and the figure decays back to the settlers' own average.
    Institute,
    /// Version 0.08.0 (ticket #186): the Academy, the Custodians' Unique Module, which replaces the
    /// Institute on their build list at the common price and pays +1 Ducat a turn on top of the
    /// schooling. It wears the same name as their Unique Facility on Earth, at the designer's word.
    Academy,
    /// Version 0.08.3 (ticket #239): the Heliostat, the Archivists' Unique Module, which replaces
    /// the Solar Array at the common price and makes one more Energy -- added AFTER the inverse
    /// square scaling, so it is worth the same at Mars as at Venus rather than 0.43 of a point at
    /// one and 1.91 at the other.
    Heliostat,
    /// Version 0.08.3 (ticket #239): the Exchange, the Prospectors' Unique Module, which replaces
    /// the Trade Post at the common price and pays one more Ducat a turn, flat and AFTER the
    /// output multiplier, exactly as the Academy's Ducat is.
    Exchange,
    /// Version 0.08.3 (ticket #239): the Chorus, the Arkwrights' Unique Module, which replaces the
    /// Relay at the common price and adds one more Influence to its holder's Allotment for every
    /// `chorus_colonists` living at its own Colony, rounded down. The more people stand here, the
    /// further the voice carries -- which is the Faction whose whole game is moving people.
    Chorus,
    /// Version 0.08.8 (ticket #324): the Battery, the one Module that fights. It is a party in the
    /// orbital Battle at its Body, on Hold, at its card's strength and hit points, and while one
    /// stands and works no rival holds Orbital Control there; its owner gains none by it. Appended
    /// last, as the Core Module was, for `Tables::module`.
    Battery,
}

impl ModuleKind {
    pub const ALL: [ModuleKind; 19] = [
        ModuleKind::Mine,
        ModuleKind::Generator,
        ModuleKind::Refinery,
        ModuleKind::Habitat,
        ModuleKind::Shipyard,
        ModuleKind::Barracks,
        ModuleKind::TradePost,
        ModuleKind::Relay,
        ModuleKind::Archive,
        ModuleKind::Observatory,
        ModuleKind::SolarArray,
        ModuleKind::MassDriver,
        ModuleKind::Core,
        ModuleKind::Institute,
        ModuleKind::Academy,
        ModuleKind::Heliostat,
        ModuleKind::Exchange,
        ModuleKind::Chorus,
        ModuleKind::Battery,
    ];
    /// The Modules an ordinary build order may place (ticket #51: the Archive is not one of them).
    pub const BUILDABLE: [ModuleKind; 17] = [
        ModuleKind::Mine,
        ModuleKind::Generator,
        ModuleKind::Refinery,
        ModuleKind::Habitat,
        ModuleKind::Shipyard,
        ModuleKind::Barracks,
        ModuleKind::TradePost,
        ModuleKind::Relay,
        ModuleKind::Observatory,
        ModuleKind::SolarArray,
        ModuleKind::MassDriver,
        ModuleKind::Institute,
        ModuleKind::Academy,
        ModuleKind::Heliostat,
        ModuleKind::Exchange,
        ModuleKind::Chorus,
        ModuleKind::Battery,
    ];
    pub fn name(self) -> &'static str {
        match self {
            ModuleKind::Core => "Core Module",
            ModuleKind::Institute => "Institute",
            ModuleKind::Academy => "Academy",
            ModuleKind::Heliostat => "Heliostat",
            ModuleKind::Exchange => "Exchange",
            ModuleKind::Chorus => "Chorus",
            ModuleKind::Mine => "Mine",
            ModuleKind::Generator => "Generator",
            ModuleKind::Refinery => "Refinery",
            ModuleKind::Habitat => "Habitat",
            ModuleKind::Shipyard => "Shipyard",
            ModuleKind::Barracks => "Barracks",
            ModuleKind::TradePost => "Trade Post",
            ModuleKind::Relay => "Relay",
            ModuleKind::Archive => "The Archive",
            ModuleKind::Observatory => "Observatory",
            ModuleKind::SolarArray => "Solar Array",
            ModuleKind::MassDriver => "Mass Driver",
            ModuleKind::Battery => "Battery",
        }
    }

    /// Version 0.08.0 (ticket #186): the Faction whose Unique Module this is, or `None`. The Archive
    /// is deliberately NOT one: it is a Faction-only Module that is destroyed on capture, which is
    /// the opposite rule, and `CONTEXT.md` keeps the two apart.
    pub fn unique_to(self) -> Option<FactionKind> {
        match self {
            ModuleKind::Academy => Some(FactionKind::Custodians),
            ModuleKind::Heliostat => Some(FactionKind::Archivists),
            ModuleKind::Exchange => Some(FactionKind::Prospectors),
            ModuleKind::Chorus => Some(FactionKind::Arkwrights),
            _ => None,
        }
    }

    /// The common kind a Unique Module replaces, or `None` for a common kind.
    pub fn common(self) -> Option<ModuleKind> {
        match self {
            ModuleKind::Academy => Some(ModuleKind::Institute),
            ModuleKind::Heliostat => Some(ModuleKind::SolarArray),
            ModuleKind::Exchange => Some(ModuleKind::TradePost),
            ModuleKind::Chorus => Some(ModuleKind::Relay),
            _ => None,
        }
    }

    /// What `faction` builds when it orders this kind: its own Unique Module where it has one, else
    /// the kind itself.
    pub fn built_by(self, faction: FactionKind) -> ModuleKind {
        for unique in ModuleKind::ALL {
            if unique.common() == Some(self) && unique.unique_to() == Some(faction) {
                return unique;
            }
        }
        self
    }

    /// True where this kind does the job the common `kind` does -- itself, or the Unique Module that
    /// replaces it.
    pub fn does_the_job_of(self, kind: ModuleKind) -> bool {
        self == kind || self.common() == Some(kind)
    }

    /// Ticket #239 (version 0.08.3): may this Module stand on a Space Station rather than a ground
    /// Colony? Ticket #80 allowed a Shipyard, Habitats and Observatories, ticket #89 the Solar
    /// Array, which stands nowhere else, and ticket #185 the Institute, since a station carries
    /// the Observatories an Institute multiplies.
    ///
    /// It lives here because it was written out twice -- once in the interface's build list and
    /// once in the computer's -- as a list of KINDS, and a list of kinds cannot know about a
    /// Unique Module. The moment three more Uniques existed, the Prospectors lost the Trade Post
    /// row from every station without gaining the Exchange and the Archivists lost the Solar Array
    /// without gaining the Heliostat, because `built_by` swaps the common kind out and the list
    /// then threw the Unique away. Answering by the JOB makes every future Unique follow its
    /// sibling with nothing to remember.
    pub fn stands_on_a_station(self) -> bool {
        matches!(
            self.common().unwrap_or(self),
            ModuleKind::Shipyard | ModuleKind::Habitat | ModuleKind::Observatory | ModuleKind::SolarArray | ModuleKind::TradePost | ModuleKind::Institute | ModuleKind::Battery
        )
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
    /// Ticket #201 (version 0.08.1): the eighteenth Tech, Society rung 2. With it standing, a
    /// Constabulary adds 10 to the challenge margin where it adds 5 without.
    CivilDefense,
    /// Version 0.06.0 (ticket #84): the four gates, one per Faction, on rung 3. Each opens its
    /// Faction's Victory Condition and is a Tech for everyone besides.
    PlanetaryStewardship,
    ExtractionCharter,
    GenerationShips,
    TheUpload,
    /// Ticket #232 (version 0.08.3): Extraction rung 2. Every Mine makes a tenth more, ANTARCTICA
    /// INCLUDED -- the designer's list said "off world mines" and the measurement is why it does
    /// not: of sixteen Mines standing at the end of twenty games, THIRTEEN were Antarctic and only
    /// three were off Earth, so an off-world-only clause would have touched 0.15 Mines a game. The
    /// name carries no space word for the same reason.
    Beneficiation,
    /// Ticket #232 (version 0.08.3): Off-world Living rung 2. A Relay's Influence Allotment goes
    /// from 1 to 2. Asked for as "+1 influence" on every Habitat and moved to the Relay at the
    /// designer's word -- "I forgot about the relay - let's give the +1 influence to the relay
    /// instead" -- which also spares the Relay being made redundant by the Habitat.
    RelayNetworks,
}

impl TechId {
    pub const ALL: [TechId; 20] = [
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
        TechId::CivilDefense,
        TechId::PlanetaryStewardship,
        TechId::ExtractionCharter,
        TechId::GenerationShips,
        TechId::TheUpload,
        TechId::Beneficiation,
        TechId::RelayNetworks,
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
    /// Ticket #278 (version 0.08.5): a warship stack ordered to blockade the rival station in the
    /// Orbital Slot it sits in. Ships only. A Blockade is chosen, at the designer's word, never a
    /// side effect of a Ship's presence: a stack on any other stance blockades nothing.
    Blockade,
    /// Ticket #297 (version 0.08.6): an Army dug in. Armies only: +2 strength while defending and
    /// no disengage roll, and it can neither march nor board a Carrier until its stance is changed
    /// and the turn has passed. A neutral Region's own Armies are always dug in, since nobody can
    /// order them. The third live stance an Army has, after Attack and Evade; Hold does nothing.
    DigIn,
}

impl Stance {
    pub fn name(self) -> &'static str {
        match self {
            Stance::Attack => "Attack",
            Stance::Hold => "Hold",
            Stance::Intercept => "Intercept",
            Stance::Evade => "Evade",
            Stance::Blockade => "Blockade",
            Stance::DigIn => "Dig In",
        }
    }

    /// Ticket #313 (version 0.08.7): what the stance does, in one sentence, for the stance row's
    /// label hovers and the roster rows, so the sentence lives once. `ships` picks the wording
    /// where a Ship's and an Army's differ: Intercept and Blockade are Ships' stances, Dig In an
    /// Army's, and an Army handed the other's says what it does instead. The designer's words for
    /// Intercept: *"fights what arrives this turn, before it can land"*, the one nobody could find.
    pub fn one_liner(self, ships: bool) -> &'static str {
        match (self, ships) {
            (Stance::Attack, _) => "Strikes at the place it is sent to, or fights where it stands.",
            (Stance::Hold, _) => "Stands and fights where it is; does nothing of its own.",
            (Stance::Evade, _) => "Avoids battle where it can: an even chance to slip away before the first exchange.",
            (Stance::DigIn, false) => "Dug in, it fights two stronger in defence and never disengages, and it cannot march or board a Carrier until its stance is changed and the turn has passed.",
            (Stance::DigIn, true) => "A Ship cannot dig in; it holds.",
            (Stance::Intercept, true) => "Fights what arrives this turn, before it can land.",
            (Stance::Intercept, false) => "An Army cannot intercept; it holds.",
            (Stance::Blockade, true) => "Shuts this orbital slot to every other Faction: no landing, no refuel, no building in it.",
            (Stance::Blockade, false) => "An Army cannot blockade; it holds.",
        }
    }

    /// Ticket #313 (version 0.08.7): the rule every stance shares, the second line of every hover.
    pub const PERSISTS: &'static str = "A stance persists until it is changed.";
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

//! Ticket #58: the Report as a dated dispatch.
//!
//! Every sentence the Report says lives in `assets/data/report.toml`; this module holds the kinds
//! a line can have, the severity order the headline and the Moments read, the four headings the
//! rest is grouped under, and the template renderer with its validation.

use crate::ids::{BodyId, ColonyId, Place, Seat, StateId, TechId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ---------------------------------------------------------------- where a line points

/// Where a Report line takes the player when it is clicked. `Place` names a Nation State or a
/// Colony; a Report line may also point at a Body with no Colony on it, so it has its own enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportPlace {
    State(StateId),
    Colony(ColonyId),
    Body(BodyId),
}

impl From<Place> for ReportPlace {
    fn from(p: Place) -> ReportPlace {
        match p {
            Place::State(s) => ReportPlace::State(s),
            Place::Colony(c) => ReportPlace::Colony(c),
        }
    }
}

// ---------------------------------------------------------------- what a line is about

/// What one Report line is about. The kind decides two things: the line's place in the severity
/// order the headline is chosen by (`headline_rank`), and which of the four headings it is grouped
/// under (`section`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LineKind {
    /// Turn 1 only: the sentence that explains the table. It always headlines.
    Seating,
    ColonyFounded,
    ControlChanged,
    Break,
    SeaLevel,
    /// A Battle in which a unit was destroyed, or one that changed Orbital Control.
    DecisiveBattle,
    /// Ticket #281 (version 0.08.5): a Battle nobody lost a unit in. A line at its place, so it is
    /// read and can be clicked to; never a headline, so a skirmish does not read over a Break.
    Battle,
    Occupation,
    TechComplete,
    Event,
    /// A build another Faction completed.
    BuildComplete,
    /// A build the player completed.
    YourBuild,
    Ship,
    Archive,
    Antarctica,
    Unrest,
    Refugees,
    Army,
    Development,
    Climate,
    /// The player's repairs, lifts, funding and economy.
    YourWorks,
    /// Anything with no better home.
    Note,
}

/// The four headings the dispatch groups its lines under, in the order they are shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Section {
    InSpace,
    OnEarth,
    TheClimate,
    YourWorks,
}

impl Section {
    pub const ALL: [Section; 4] = [Section::InSpace, Section::OnEarth, Section::TheClimate, Section::YourWorks];
    pub fn name(self) -> &'static str {
        self.name_for(false)
    }

    /// Ticket #64: a spectator has no works of their own, so the heading that carries them is named
    /// for what it now holds: every Faction's builds and works, not one seat's.
    pub fn name_for(self, spectator: bool) -> &'static str {
        match self {
            Section::InSpace => "In space",
            Section::OnEarth => "On Earth",
            Section::TheClimate => "The climate",
            Section::YourWorks if spectator => "Builds and works",
            Section::YourWorks => "Your works",
        }
    }
}

/// Ticket #64: which kind a line written "for a seat" carries. A player's game routes seat 0's
/// builds, lifts, repairs and funding to "Your works" and every other seat's to the board; a
/// spectated game has no seat of its own, so every seat's works take the same route and the
/// heading carries all four Factions.
pub fn line_kind_of(seat: Seat, mine: LineKind, theirs: LineKind, spectator: bool) -> LineKind {
    if spectator || seat == Seat(0) { mine } else { theirs }
}

impl LineKind {
    /// The severity order the headline is chosen by: a Colony founded, then a place changing
    /// controller, a Break or a Sea Level threshold, a decisive Battle, an Occupation, a Tech, an
    /// Event, and last a build completed. A kind with no rank never headlines.
    pub fn headline_rank(self) -> Option<u8> {
        Some(match self {
            LineKind::Seating => 0,
            LineKind::ColonyFounded => 1,
            LineKind::ControlChanged => 2,
            LineKind::Break | LineKind::SeaLevel | LineKind::Antarctica => 3,
            LineKind::DecisiveBattle => 4,
            LineKind::Occupation => 5,
            LineKind::TechComplete => 6,
            LineKind::Event => 7,
            LineKind::BuildComplete | LineKind::YourBuild => 8,
            _ => return None,
        })
    }

    /// Which heading the line is grouped under. Kinds that can happen either off Earth or on it
    /// follow their place: a Colony or a Body is In space, a Nation State is On Earth.
    pub fn section(self, place: Option<ReportPlace>) -> Section {
        let by_place = || match place {
            Some(ReportPlace::State(_)) | None => Section::OnEarth,
            Some(_) => Section::InSpace,
        };
        match self {
            LineKind::ColonyFounded | LineKind::Ship | LineKind::Archive | LineKind::Antarctica => Section::InSpace,
            LineKind::Unrest | LineKind::Refugees | LineKind::Army | LineKind::Occupation => Section::OnEarth,
            LineKind::Break | LineKind::SeaLevel | LineKind::Event | LineKind::Development | LineKind::Climate => Section::TheClimate,
            LineKind::TechComplete | LineKind::YourBuild | LineKind::YourWorks => Section::YourWorks,
            LineKind::ControlChanged | LineKind::DecisiveBattle | LineKind::Battle | LineKind::BuildComplete | LineKind::Note | LineKind::Seating => by_place(),
        }
    }
}

/// One line of the dispatch: what it is about, where it points, and what it says.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportLine {
    pub kind: LineKind,
    pub place: Option<ReportPlace>,
    pub text: String,
}

impl ReportLine {
    pub fn section(&self) -> Section {
        self.kind.section(self.place)
    }
}

// ---------------------------------------------------------------- Moments

/// The seven kinds of Moment: the short modal that stops the turn before the Report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MomentKind {
    ColonyFounded,
    ControlChanged,
    ClimateThreshold,
    DecisiveBattle,
    TechComplete,
    Antarctica,
    ArchiveComplete,
    /// Version 0.06.0 (ticket #86): Colonists lost to crowding on a Colony Ship's arrival.
    LostInTransit,
    /// Ticket #261 (version 0.08.4): a rival closing on its Victory Condition -- three quarters of
    /// the way, or one part met with the other short. Rivals only; the player has the Victory
    /// window. Nothing in the game told a player a rival was about to end it.
    RivalProgress,
    /// Ticket #281 (version 0.08.5): buildings burned in the rolls after a taking. It wore the
    /// Battle's name from ticket #50 to here, and fired for a Pacified transfer that fought nobody.
    PlaceTakenByForce,
}

impl MomentKind {
    pub const ALL: [MomentKind; 10] = [
        MomentKind::ColonyFounded,
        MomentKind::ControlChanged,
        MomentKind::ClimateThreshold,
        MomentKind::DecisiveBattle,
        MomentKind::TechComplete,
        MomentKind::Antarctica,
        MomentKind::ArchiveComplete,
        MomentKind::LostInTransit,
        MomentKind::RivalProgress,
        MomentKind::PlaceTakenByForce,
    ];

    /// The key its table carries in `report.toml`.
    pub fn key(self) -> &'static str {
        match self {
            MomentKind::ColonyFounded => "colony_founded",
            MomentKind::ControlChanged => "control_changed",
            MomentKind::ClimateThreshold => "climate_threshold",
            MomentKind::DecisiveBattle => "decisive_battle",
            MomentKind::TechComplete => "tech_complete",
            MomentKind::Antarctica => "antarctica",
            MomentKind::ArchiveComplete => "archive_complete",
            MomentKind::LostInTransit => "lost_in_transit",
            MomentKind::RivalProgress => "rival_progress",
            MomentKind::PlaceTakenByForce => "taken_by_force",
        }
    }

    /// What the checkbox in the Moments corner is called.
    pub fn name(self) -> &'static str {
        match self {
            MomentKind::ColonyFounded => "A Colony founded",
            MomentKind::ControlChanged => "A place changing hands",
            MomentKind::ClimateThreshold => "A Break or the sea rising",
            MomentKind::DecisiveBattle => "A Battle that cost a unit",
            MomentKind::TechComplete => "A Tech completed",
            MomentKind::Antarctica => "Antarctica opening",
            MomentKind::ArchiveComplete => "The Archive completed",
            MomentKind::LostInTransit => "Colonists lost in transit",
            MomentKind::RivalProgress => "A rival closing on its Victory Condition",
            MomentKind::PlaceTakenByForce => "A place taken by force",
        }
    }

    /// The same severity order the headline reads. Antarctica opening is a Temperature threshold
    /// crossed; the Archive completed is a build completed.
    pub fn rank(self) -> u8 {
        match self {
            MomentKind::ColonyFounded => 1,
            // Ticket #86: lives lost read before a place changing hands.
            MomentKind::LostInTransit => 2,
            MomentKind::ControlChanged => 2,
            MomentKind::ClimateThreshold | MomentKind::Antarctica => 3,
            MomentKind::DecisiveBattle | MomentKind::PlaceTakenByForce => 4,
            // Ticket #261: a rival about to win reads before a Tech and after a lost unit.
            MomentKind::RivalProgress => 5,
            MomentKind::TechComplete => 6,
            MomentKind::ArchiveComplete => 8,
        }
    }
}

/// One Moment: a sentence, a number, and where it happened.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Moment {
    pub kind: MomentKind,
    pub text: String,
    pub figure: String,
    pub place: Option<ReportPlace>,
    /// The Tech just completed, so the Moment can light its box on the Tech Tree.
    pub tech: Option<TechId>,
    /// The line under the sentence: what the AI picked and why, or that the player picks.
    pub note: Option<String>,
    /// Ticket #261 (version 0.08.4): the seat a Moment is about, when it is about one -- the rival's
    /// Moment, which wears that Faction's colour on its figure. None for every other kind.
    #[serde(default)]
    pub seat: Option<Seat>,
}

/// How many Moments a turn may stop for.
pub const MOMENTS_PER_TURN: usize = 2;

// ---------------------------------------------------------------- the Report

/// Ticket #50, rebuilt by #58: what one AI seat did in a turn, in plain sentences taken from the
/// orders it committed and what the Resolution made of them. The scored list it chose from is in
/// the simulate log and nowhere else.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiReport {
    pub seat: Seat,
    pub deeds: Vec<String>,
}

/// Everything the Report popup shows at the start of a turn (spec 17.5, ticket #58).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Report {
    pub turn: u32,
    pub battles: Vec<crate::state::BattleLine>,
    pub event: Option<String>,
    pub lines: Vec<ReportLine>,
    /// One entry per AI seat that ordered this turn, in seat order.
    pub ai_lines: Vec<AiReport>,
    /// Every Moment the turn earned, in the order they happened. The cap and the switches are
    /// applied by `moments_shown`.
    pub moments: Vec<Moment>,
}

impl Report {
    /// The line that heads the dispatch: the lowest severity rank, earliest line first.
    pub fn headline(&self) -> Option<&ReportLine> {
        self.headline_index().map(|i| &self.lines[i])
    }

    pub fn headline_index(&self) -> Option<usize> {
        self.lines
            .iter()
            .enumerate()
            .filter_map(|(i, l)| l.kind.headline_rank().map(|r| (r, i)))
            .min_by_key(|(r, i)| (*r, *i))
            .map(|(_, i)| i)
    }

    /// Every line that is not the headline, in the order the four headings are shown.
    pub fn sections(&self) -> Vec<(Section, Vec<&ReportLine>)> {
        let head = self.headline_index();
        Section::ALL
            .into_iter()
            .map(|s| {
                let lines: Vec<&ReportLine> =
                    self.lines.iter().enumerate().filter(|(i, l)| Some(*i) != head && l.section() == s).map(|(_, l)| l).collect();
                (s, lines)
            })
            .filter(|(_, l)| !l.is_empty())
            .collect()
    }

    /// The Moments the turn actually stops for: the switched-on kinds, highest severity first,
    /// ties in the order they happened, at most two.
    pub fn moments_shown(&self, on: &dyn Fn(MomentKind) -> bool) -> Vec<&Moment> {
        let mut kept: Vec<(usize, &Moment)> = self.moments.iter().enumerate().filter(|(_, m)| on(m.kind)).collect();
        kept.sort_by_key(|(i, m)| (m.kind.rank(), *i));
        kept.into_iter().take(MOMENTS_PER_TURN).map(|(_, m)| m).collect()
    }
}

// ---------------------------------------------------------------- the templates

#[derive(Debug, Clone, Deserialize)]
pub struct MomentCard {
    pub on: bool,
    pub text: String,
    pub figure: String,
}

/// `assets/data/report.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct ReportTable {
    pub line: BTreeMap<String, String>,
    pub phrase: BTreeMap<String, String>,
    pub rival: BTreeMap<String, String>,
    pub moments: BTreeMap<String, MomentCard>,
}

/// Fill `{name}` placeholders. An unknown placeholder is left standing, which is what the
/// validation at load looks for; nothing here can panic on bad data.
pub fn render(template: &str, args: &[(&str, String)]) -> String {
    let mut out = template.to_string();
    for (k, v) in args {
        out = out.replace(&format!("{{{k}}}"), v);
    }
    out
}

/// Every `{name}` a template uses.
pub fn placeholders(template: &str) -> Vec<String> {
    let mut found = Vec::new();
    let bytes: Vec<char> = template.chars().collect();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == '{'
            && let Some(end) = bytes[i + 1..].iter().position(|c| *c == '}')
        {
            let name: String = bytes[i + 1..i + 1 + end].iter().collect();
            if !name.is_empty() && name.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
                found.push(name);
            }
            i += end + 2;
            continue;
        }
        i += 1;
    }
    found
}

/// What the engine supplies for every `[line]` key. The loader refuses `report.toml` if a key is
/// missing, unknown, or uses a placeholder that is not on its row.
pub const LINE_ARGS: &[(&str, &[&str])] = &[
    ("seating", &["date", "faction", "state", "rivals"]),
    ("start_holding", &["state", "materials", "fuel", "energy"]),
    ("start_rivals", &["rivals", "collapse"]),
    ("solar_storm", &[]),
    ("ship_arrived", &["faction", "ship", "body"]),
    ("ship_destroyed", &["faction", "ship", "why", "cargo"]),
    // Ticket #281 (version 0.08.5): an Army destroyed, by name; and every Battle, as a line.
    ("army_destroyed", &["faction", "army", "why"]),
    // Ticket #282 (version 0.08.5): neutral states arm when threatened.
    ("levy_raised", &["state", "army", "n"]),
    ("levy_disbanded", &["state", "army"]),
    ("neutral_held", &["state", "n"]),
    ("battle", &["place", "faction", "odds", "outcome"]),
    ("event_damaged_ships", &["event", "n"]),
    ("loaded", &["faction", "cargo", "body"]),
    ("slot_taken", &["faction"]),
    ("landing_contested", &["faction", "body"]),
    ("colony_founded", &["faction", "slot", "body", "n"]),
    ("disembarked", &["n", "colony"]),
    ("station_built", &["faction", "station"]),
    ("antarctica_opens", &["n"]),
    ("archive_begun", &["faction", "colony"]),
    ("neutral_research", &["states", "n"]),
    ("emigrants_mustered", &["n", "state", "fell", "unrest"]),
    ("emigrants_arrived", &["n", "state", "colony"]),
    ("emigrants_returned", &["n", "state"]),
    ("emigrants_lifted", &["n", "state", "station"]),
    ("claim_lot", &["place", "factions", "winner"]),
    ("archive_built", &["faction", "place", "left"]),
    ("archive_complete", &["faction", "place"]),
    ("archive_destroyed", &["place", "faction"]),
    ("archive_funded", &["faction", "banked", "fund", "cap"]),
    // Ticket #235 (version 0.08.3): the three Research Directives that are not the Archive's.
    ("directive_sink", &["faction", "research", "ppm", "sink"]),
    ("exodus_call", &["faction", "state", "n", "turns"]),
    ("exodus_call_order", &["state"]),
    ("directive_ducats", &["faction", "research", "n"]),
    ("directive_fuel", &["faction", "research", "n"]),
    ("army_moved", &["faction", "from", "to", "attacks"]),
    ("army_landed", &["faction", "colony"]),
    // Ticket #297 (version 0.08.6): the turn an Army digs in.
    ("army_dug_in", &["faction", "place"]),
    ("units_destroyed", &["place", "why", "lost"]),
    ("occupation_begun", &["faction", "place"]),
    ("occupation_ended", &["place", "faction"]),
    ("control_changed", &["place", "faction", "why"]),
    ("claim_tied", &["place", "factions"]),
    ("threw_off", &["state", "faction", "unrest"]),
    ("unrest_rose_state", &["state", "rose", "unrest"]),
    ("unrest_card", &["state", "rose", "unrest"]),
    ("unrest_threshold", &["state", "unrest", "note"]),
    ("relief", &["faction", "state", "fell", "unrest"]),
    // Ticket #176 (version 0.07.6): one net line per Region, in place of one per flow and one for
    // arriving. `gross` appears only where what left cancelled some of what arrived, since Unrest
    // is charged on everyone who came.
    ("refugees_net_in", &["n", "state", "rose", "unrest"]),
    ("refugees_net_in_gross", &["n", "gross", "state", "rose", "unrest"]),
    ("refugees_net_in_quiet", &["n", "state"]),
    ("refugees_net_out", &["n", "state", "why"]),
    ("refugees_net_out_mostly", &["n", "state", "why"]),
    ("resettled", &["faction", "state", "standing"]),
    ("strip_permit", &["faction", "state", "turns"]),
    ("strip_permit_ended", &["state", "baseline", "rose", "unrest"]),
    ("building_changed_state", &["faction", "done", "building", "state"]),
    ("building_decommissioned_state", &["faction", "building", "state", "refund"]),
    ("building_changed_colony", &["faction", "done", "building", "colony"]),
    ("building_decommissioned_colony", &["faction", "building", "colony", "refund"]),
    ("build_lost", &["building", "place"]),
    ("colonists_no_room", &["n", "colony"]),
    // Ticket #191 (version 0.08.0): Relations, said in the offender's paragraph.
    ("relations_fell", &["victim", "offender"]),
    ("uploaded", &["n", "colony", "total"]),
    ("launch_pad_fire", &["faction", "item", "place"]),
    ("game_over_win", &["turn", "faction", "note"]),
    ("game_over_draw", &["turn", "note"]),
    ("game_over_collapse", &["turn", "temperature"]),
    ("event_drawn", &["text"]),
    ("break_fired", &["temperature", "name", "happened", "text"]),
    ("break_coastal", &["states", "exposure", "percent", "unrest"]),
    ("break_baseline", &["state", "rise"]),
    // Ticket #257 (version 0.08.4): the wall stands and its keep rises; a surge it holds; a keep unpaid.
    ("sea_wall", &["temperature", "state", "keep"]),
    // Ticket #259 (version 0.08.4): the off-Earth cards join the deck.
    ("deck_joined", &["n"]),
    // Ticket #261: the two steps a rival's Moment fires on.
    ("rival_three_quarters", &["faction", "part", "value", "bar"]),
    ("rival_one_part_met", &["faction", "met", "part", "value", "bar"]),
    // Ticket #267: a Smear campaign landed.
    ("smear", &["faction", "target", "ppm"]),
    // Ticket #277 (version 0.08.5): a Greenwash landed.
    ("greenwash", &["faction", "ppm"]),
    // Ticket #278 (version 0.08.5): a Colony starved under a Blockade this Income.
    ("starved", &["place", "faction"]),
    // Ticket #268: carbon credits offered and bought.
    ("credits_offered", &["faction", "n"]),
    ("credits_bought", &["faction", "n", "seller", "ducats"]),
    ("credits_short", &["faction", "n", "back"]),
    // Ticket #269: an Agitate landed, named for who paid.
    ("agitate", &["faction", "state", "rose", "unrest"]),
    ("agitate_damped", &["faction", "state"]),
    ("storm_surge_wall", &["state", "percent"]),
    ("sea_wall_unkept", &["faction", "states"]),
    ("sea_nothing_left", &["temperature", "state"]),
    ("sea_took", &["n", "slots", "state", "temperature"]),
    ("sea_took_destroying", &["n", "slots", "state", "temperature", "destroyed"]),
    ("heat_population", &["state", "percent", "after", "rose", "unrest"]),
    ("heat_unrest_only", &["state", "rose", "unrest"]),
    ("development", &["state", "level"]),
    ("development_woke", &["state", "level", "building"]),
    ("scrubbers_destroyed", &["n", "state", "why"]),
    ("leapfrog", &["faction", "state", "coefficient"]),
    ("tech_complete", &["tech", "faction", "shares"]),
    ("build_complete", &["faction", "building", "place"]),
    ("energy_short", &["faction", "buildings"]),
    ("energy_zero", &["faction"]),
];

/// Ticket #85 (version 0.06.0): a count as an ordinal, in words up to twelfth and as a figure
/// with its suffix past that.
pub fn ordinal(n: usize) -> String {
    const WORDS: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    if (1..=12).contains(&n) {
        return WORDS[n - 1].to_string();
    }
    let suffix = match (n % 10, n % 100) {
        (1, r) if r != 11 => "st",
        (2, r) if r != 12 => "nd",
        (3, r) if r != 13 => "rd",
        _ => "th",
    };
    format!("{n}{suffix}")
}

/// The same for `[phrase]`.
pub const PHRASE_ARGS: &[(&str, &[&str])] = &[
    ("attacks", &[]),
    ("cargo_aboard", &["n"]),
    ("sea_unrest", &["rose", "unrest"]),
    ("sea_inland", &[]),
    ("sea_inland_flipped", &["what"]),
    ("first_colony", &[]),
    ("more_colonies", &["ordinal"]),
    ("first_antarctic_colony", &[]),
    ("more_antarctic_colonies", &["ordinal"]),
    ("slot", &[]),
    ("slots", &[]),
    ("pick_first_choice", &["faction", "tech"]),
    ("pick_cheapest", &["faction", "tech"]),
    ("pick_last", &["faction", "tech"]),
    ("you_pick", &[]),
];

/// The same for `[rival]`.
pub const RIVAL_ARGS: &[(&str, &[&str])] = &[
    ("paragraph", &["faction", "deeds"]),
    ("nothing", &["faction"]),
    // Ticket #226 (version 0.08.2): the Accords, told as the board could see them.
    ("propose_accord", &["faction"]),
    ("end_accord", &["faction"]),
    ("tribute", &["faction"]),
    ("build_facility", &["building", "state"]),
    ("build_facility_ducats", &["building", "state"]),
    ("raise_industry", &["state"]),
    ("build_module", &["building", "colony"]),
    ("build_module_ducats", &["building", "colony"]),
    ("build_ship", &["unit", "place"]),
    ("build_army", &["place"]),
    ("build_station", &["body"]),
    ("build_archive", &["colony"]),
    // Ticket #192 (version 0.08.0): the Upload.
    ("upload", &["n", "colony"]),
    ("fund_archive", &[]),
    ("unfund_archive", &[]),
    ("max_on", &["place"]),
    ("max_off", &[]),
    ("build_emigrants", &["n", "state"]),
    ("send_antarctica", &["n", "state"]),
    ("lift_station", &["n", "state", "station"]),
    ("set_venture_share", &["share"]),
    ("draw_venture", &["n"]),
    ("repair", &["unit"]),
    ("transit", &["unit", "body"]),
    ("refuel", &["unit", "body"]),
    ("ship_stance", &["body", "stance"]),
    ("army_stance", &["place", "stance"]),
    ("move_army", &["state"]),
    ("load_colonists", &["n", "place"]),
    ("load_army", &["place"]),
    ("unload", &["place"]),
    ("influence", &["n", "place"]),
    ("buy_influence", &["n"]),
    ("buy", &["n", "resource"]),
    ("sell", &["n", "resource"]),
    ("relief", &["state"]),
    ("smear", &["n", "faction"]),
    ("greenwash", &["n"]),
    ("offer_credits", &["n"]),
    ("buy_credits", &["n"]),
    ("agitate", &["state"]),
    ("resettle", &["state"]),
    ("mothball", &["building", "place"]),
    ("restart", &["building", "place"]),
    ("decommission", &["building", "place"]),
    ("leapfrog", &["state"]),
    ("strip_permit", &["state"]),
    ("arrived", &["unit", "body"]),
    ("completed", &["building", "place"]),
    ("founded", &["colony"]),
];

/// The arguments a Moment's `text` and `figure` may use, by kind.
pub const MOMENT_ARGS: &[(&str, &[&str])] = &[
    ("colony_founded", &["faction", "colony", "note", "n"]),
    ("lost_in_transit", &["faction", "ship", "body", "n", "of"]),
    ("control_changed", &["place", "faction"]),
    ("climate_threshold", &["what", "figure"]),
    ("decisive_battle", &["place", "result", "figure"]),
    ("taken_by_force", &["place", "result", "figure"]),
    ("tech_complete", &["tech", "faction", "lead", "cost"]),
    ("antarctica", &["n"]),
    ("archive_complete", &["faction", "place", "research"]),
    // Ticket #261 (version 0.08.4): the sentence is a line card, as the climate threshold's is.
    ("rival_progress", &["text", "figure"]),
];

impl ReportTable {
    /// Every declared key present, no unknown key, and every placeholder one the engine supplies.
    pub fn check(&self) -> Result<(), String> {
        check_table("line", &self.line, LINE_ARGS)?;
        check_table("phrase", &self.phrase, PHRASE_ARGS)?;
        check_table("rival", &self.rival, RIVAL_ARGS)?;
        for (key, args) in MOMENT_ARGS {
            let Some(card) = self.moments.get(*key) else {
                return Err(format!("[moments.{key}] is missing"));
            };
            for (what, text) in [("text", &card.text), ("figure", &card.figure)] {
                for p in placeholders(text) {
                    if !args.contains(&p.as_str()) {
                        return Err(format!("[moments.{key}] {what} uses {{{p}}}, which the engine does not supply there"));
                    }
                }
            }
        }
        for key in self.moments.keys() {
            if !MOMENT_ARGS.iter().any(|(k, _)| k == key) {
                return Err(format!("[moments.{key}] is not a Moment the engine knows"));
            }
        }
        Ok(())
    }

    pub fn line(&self, key: &str, args: &[(&str, String)]) -> String {
        self.line.get(key).map(|t| render(t, args)).unwrap_or_else(|| format!("[{key}]"))
    }
    pub fn phrase(&self, key: &str, args: &[(&str, String)]) -> String {
        self.phrase.get(key).map(|t| render(t, args)).unwrap_or_else(|| format!("[{key}]"))
    }
    pub fn rival(&self, key: &str, args: &[(&str, String)]) -> String {
        self.rival.get(key).map(|t| render(t, args)).unwrap_or_else(|| format!("[{key}]"))
    }
    pub fn moment(&self, kind: MomentKind) -> Option<&MomentCard> {
        self.moments.get(kind.key())
    }
    pub fn moment_on(&self, kind: MomentKind) -> bool {
        self.moment(kind).map(|m| m.on).unwrap_or(true)
    }
}

fn check_table(name: &str, rows: &BTreeMap<String, String>, declared: &[(&str, &[&str])]) -> Result<(), String> {
    for (key, args) in declared {
        let Some(text) = rows.get(*key) else {
            return Err(format!("[{name}] has no {key}"));
        };
        for p in placeholders(text) {
            if !args.contains(&p.as_str()) {
                return Err(format!("[{name}] {key} uses {{{p}}}, which the engine does not supply there"));
            }
        }
    }
    for key in rows.keys() {
        if !declared.iter().any(|(k, _)| k == key) {
            return Err(format!("[{name}] {key} is not a sentence the engine asks for"));
        }
    }
    Ok(())
}

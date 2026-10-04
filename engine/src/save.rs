//! Ticket #59: a **Save** — one turn start written to a file, and read back.
//!
//! The file is readable text, so the designer can open it: a one-line RON header carrying the
//! version stamp and everything the Load list shows, then the whole game as pretty RON under it.
//! The `Tables` are not in it. A save records the seed and the state, and loads against the tables
//! on disk, so a save can never disagree with the rules the executable ships with.
//!
//! Nothing here knows where the saves folder is: every function takes the folder. The window crate
//! supplies the real one (`%LOCALAPPDATA%\DyingEarth\data\saves`) and a test supplies a temporary
//! one.

use crate::data::Tables;
use crate::ids::*;
use crate::orders::Pending;
use crate::state::*;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// The stamp at the head of every save. A file whose stamp is not this one is refused with a plain
/// message; a save is never migrated between versions.
///
/// Ticket #453 (version 0.09.6): moved to **8**. Two Regions joined the board (Pakistan, the United
/// Kingdom), so a save of fourteen has no card for two of the sixteen and three parents' figures moved.
pub const SAVE_VERSION: u32 = 8;

/// The rules version this executable plays, named beside the file's own in a refusal.
///
/// Ticket #195 (version 0.08.0): this had been left at 0.05.5 for six versions, which meant saves
/// were NOT in fact version-locked across any of them -- a fact the 0.08.0 charting relied on being
/// true. It has to move now whether or not it moved before: this version adds four Facility kinds
/// and a Module kind, renames a Victory part, and gives every seat two new fields, so a file written
/// by an earlier build describes a board this one cannot read. A refusal naming both versions is the
/// right answer to that, and a silent partial load is not.
/// Ticket #220 (version 0.08.2): moved again. The Trading window's prices are saved state now, and
/// a file written by an earlier build describes a market this one cannot read -- its prices are
/// absent, and the three base figures it was played at have each risen by one besides. A refusal
/// naming both versions is the right answer; a silent partial load is not.
/// Ticket #241 (version 0.08.3): moved again, and this version has more reason than most. A seat
/// carries a **Research Directive** where it carried a single Archive-funding flag (#235); a Region
/// carries a hold clock and an Exodus Call (#237, #238); and three new **Unique Modules** were
/// appended to `ModuleKind` (#239), so a colony written by an older build describes a Module list
/// this one indexes differently. A refusal naming both versions is the right answer.
/// Ticket #287 (version 0.08.5): moved again. A Region carries a converted-slot count and an
/// armed step (#276, #282); a seat carries Blame cleaned by Greenwash and blockade-turns (#277,
/// #278); an Army carries a levy flag (#282); the climate carries a war bucket (#279); a Battle
/// carries a place (#281); and the game carries the war's counters (#286). A refusal naming both
/// versions is the right answer.
/// Ticket #301 (version 0.08.6): moved again. An Army carries a Dig In stance and a fixed raised
/// strength (#297, #302); a Region carries a threat flag (#302); an Occupation carries the Standing
/// it banked (#299); the war's counters gained escapes, Dig Ins and landings (#295, #297, #300);
/// and the tables gained figures a save does not carry but a board from before would disagree
/// with. A refusal naming both versions is the right answer.
/// Ticket #314 (version 0.08.7): the presentation version added nothing to the save, so
/// `SAVE_VERSION` stands and a 0.08.6 save loads; only the name a save carries moves.
/// Ticket #329 (version 0.08.8): moved to 2. A Module carries damage and `ModuleKind` gained the
/// Battery, appended last (#324); the pending orders carry Bombards (#328); the war's counters
/// gained interceptions, Batteries lost, Bombards and Modules burned (#319, #324, #328); the
/// tables gained a respawn delay, combat figures on a Module card and the melee's shape (#318,
/// #324, #327). Every new field has a default, so a 0.08.7 file would parse; but it would then
/// be played under rules it was not written for, with Armies that march where they could not and
/// a melee that rolls differently, and a refusal naming both versions is the honest answer.
/// Ticket #332 (version 0.09.0): moved to 3. A Build carries a Widget figure and a count in place
/// of a due turn, so a queue written by an older build has no field this one reads and nothing
/// would ever complete; `FacilityKind` gained the Mine and `ModuleKind` the Factory, both
/// appended last; the game carries the Widgets counters; and every table row carries `widgets`
/// where it carried `build_turns`, so a board from before would be priced in a unit this version
/// does not have. A refusal naming both versions is the right answer.
/// Ticket #340 (version 0.09.0, the closing ticket): `SAVE_VERSION` stays at **3**, where #332 put
/// it, and does not move again; `GAME_VERSION` moves to 0.09.0 for the whole version. Taken
/// together, this is what a 0.08.8 save would not understand. Every Build carries the **Widgets**
/// put into it and the figure it needs, where it carried a turn to be due on, so an older queue
/// names a turn this version has no use for (#332). `FacilityKind` gained the **Mine** and
/// `ModuleKind` the **Factory**, both appended last, so an older file's two lists are indexed
/// differently here (#332). A Ship carries the **orbit** it sits in, low orbit or a Slot, and a
/// transit the orbit it is bound for, where an older file has a Ship at the Body at large and a
/// Battle keyed on the Body rather than on one orbit (#335). The deck carries **eighteen new Event
/// ids**, a question pending at the head of the turn and the answer each seat gave, none of which
/// an older file has a field for, and whose ids are not in the deck it was dealt (#337). And a unit
/// of population is **one million people** where it was five, so every Region figure in an older
/// file is five times too small read under these rules (#333). A refusal naming both versions is
/// the right answer, and a silent partial load is not.
/// Ticket #343 (version 0.09.1): moved to **4**. `UnitKind` gained the **Missile Carrier**,
/// appended last, so an older file's unit list is indexed differently here; a Ship carries a
/// **Warhead** and a queue may carry a Warhead build, neither of which an older file has a field
/// for; `TechId` gained **Missile Technology**, so an older file's Tech flags are a list of a
/// different length; the pending orders carry **Launches**; the war's counters gained the nuke's
/// five; and the Sink Weakens **subtracts** where it assigned, so a board from before was played
/// under a Sink rule this version does not have. A refusal naming both versions is the right
/// answer, and a silent partial load is not. `GAME_VERSION` is NOT moved here: the version's
/// closing ticket moves it for the whole version, as ticket #340 did for 0.09.0.
/// Ticket #345 (version 0.09.1): moved to **5**. The game carries the record of who was FIRST to
/// each Body, and a seat carries the `first_windfall` accumulator that record pays into; neither
/// has a field in an older file, so a 0.09.1 board loaded from one would have forgotten every
/// first already claimed and would hand the next founder a windfall the game had already paid.
/// Every Body row carries a `first_windfall` besides, so a board from before was played under a
/// rule this version does not have. A refusal naming both versions is the right answer.
/// Ticket #378 (version 0.09.2, the closing ticket): moved to **6**, and `GAME_VERSION` to 0.09.2
/// for the whole version. What a 0.09.1 save would not understand: the war's counters carry the
/// **Battles that cost a hull or a Battery**, which the chronicle tells (#381), so a board loaded
/// from an older file would have forgotten every one already fought; a Battle line carries its
/// **round log** and whether the player fought it mid-turn (#381, #383); the game carries the
/// **stacks that fought this turn** (#383); and the rules moved under the board -- every home
/// Region opened with its card's list where this version deals a package (#377), a stack's Attack
/// stood as a stance where this version fights it the moment it is ordered (#383), and the
/// Refugee Convoy landed 0.4 for 2 ppm where it lands 1.0 for half (#376). Every new field has a
/// default, so the file would parse; it would parse into a board this version was not playing. A
/// refusal naming both versions is the right answer, and a silent partial load is not.
/// Ticket #387 (version 0.09.3): moved to **7**. Every Stockpile figure, the Venture Capital Fund,
/// a Ship's tank, a seat's income and its income lines are carried to a **tenth** and written as
/// floats where an older file writes whole numbers, and a seat no longer carries the Sea Walls'
/// keep accumulator, since the half is paid each turn. An older file would in fact parse (the
/// reader takes a whole number as a float and passes over a field it no longer knows), into a
/// board the economy no longer plays: every price and income would then move under it. The
/// refusal rests on the stamp, as every move of it does, and names both versions. Ticket #393
/// (the same version) appends a twenty-second Tech, Nuclear Rockets, and rides the same move.
/// `GAME_VERSION` is NOT moved here: the version's closing ticket moves it for the whole version.
/// Ticket #401 (version 0.09.3, the closing ticket): moved to 0.09.3. `SAVE_VERSION` moved once for
/// the version, to 7, at the tenths (#387) and carries the Stadium (#389), the seventeenth Facility
/// kind, besides: a 0.09.2 file knows neither.
/// Ticket #417 (version 0.09.4, the closing ticket): moved to 0.09.4. `SAVE_VERSION` did not move
/// this version: every field it added reads a default from an older file, so a 0.09.3 save loads.
/// Ticket #432 (version 0.09.5, the closing ticket): moved to 0.09.5. `SAVE_VERSION` did not move:
/// the fog's Report fields read a default, and the two new Techs are appended, so a 0.09.4 save loads.
pub const GAME_VERSION: &str = "0.09.5";

/// The game autosaves at the start of the Report phase of every third turn.
pub const AUTOSAVE_EVERY: u32 = 3;

/// How many autosaves one game (one seed) keeps. The oldest goes when a fourth is written.
pub const AUTOSAVES_KEPT: usize = 3;

/// What the Save button says when it is dead.
pub const SAVE_PENDING_HOVER: &str =
    "A save captures the beginning of a turn, never half-entered orders. Cancel your orders, or end the turn, and save then.";

/// What the Load list calls a game nobody sits at.
pub const SPECTATING: &str = "Spectating";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SaveKind {
    /// Written by the Save button, at a turn start with no orders pending.
    Manual,
    /// Written by the game itself every third turn, and at game over.
    Autosave,
}

impl SaveKind {
    pub fn is_autosave(self) -> bool {
        self == SaveKind::Autosave
    }
}

/// The first line of a save: the stamp, and everything the Load list shows without reading the
/// game under it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveHeader {
    /// Always first, so a foreign file is refused by its stamp before anything else is read.
    pub save_version: u32,
    pub game_version: String,
    pub kind: SaveKind,
    pub seed: u64,
    pub turn: u32,
    /// The Faction in seat 0, or `Spectating` in a game nobody sits at.
    pub faction: String,
    pub spectator: bool,
    /// "December 2030", the month the turn stands on.
    pub date: String,
    pub temperature: f64,
}

/// One save the Load screen lists.
#[derive(Debug, Clone)]
pub struct SaveEntry {
    pub path: PathBuf,
    pub header: SaveHeader,
    /// The file's own modification time: when it was saved.
    pub saved: std::time::SystemTime,
    pub bytes: u64,
}

/// The whole game as it goes into a file: every field of `Game` but its `Tables`, which are the
/// rules on disk and the same for every game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedGame {
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
    /// Ticket #73: Emigrants on the sea to Antarctica.
    #[serde(default)]
    pub antarctic_sends: Vec<AntarcticSend>,
    pub solar_maximum_next: bool,
    pub last_event: Option<DrawnEvent>,
    pub report: Report,
    pub outcome: Option<Outcome>,
    pub next_id: u32,
    pub pending: Pending,
    pub antarctica_open: bool,
    pub slot_yields: BTreeMap<(BodyId, u32), SlotYields>,
    pub spectator: bool,
    pub log: Vec<String>,
    /// Ticket #191 (version 0.08.0): what every Faction thinks of every other.
    #[serde(default)]
    pub relations: Relations,
    /// Ticket #220 (version 0.08.2): the Trading window's live prices travel with the game, so a
    /// save reopens on the market it left. `#[serde(default)]` so a save written before this version
    /// still loads: its zeroes read as the card figures.
    #[serde(default)]
    pub market: Market,
    /// Ticket #226 (version 0.08.2): the Accords travel with the game.
    #[serde(default)]
    pub accords: Vec<Accord>,
    /// Ticket #272 (version 0.08.4): the sweep's count of Events drawn with nowhere to land.
    #[serde(default)]
    pub events_no_target: u32,
    /// Ticket #282 (version 0.08.5): the sweep's counts of Levies raised and neutral Regions that held.
    #[serde(default)]
    pub levies_raised: u32,
    #[serde(default)]
    pub neutral_holds: u32,
    /// Ticket #286 (version 0.08.5): the war's counters.
    #[serde(default)]
    pub war: WarCounters,
    /// Ticket #383 (version 0.09.2): the stacks that fought an Attack this turn. Empty in every
    /// save the game writes, since the Save button is dead after a fought Attack; carried for the
    /// driver and the tests, which may save at any point.
    #[serde(default)]
    pub fought: Vec<(Seat, BodyId)>,
    /// Ticket #332 (version 0.09.0): the Widgets counters.
    #[serde(default)]
    pub widgets: WidgetCounters,
    /// Ticket #337 (version 0.09.0): the turn's draw, the pending question with what each seat has
    /// answered, and the game's tally of answers. A save captures a turn START, which is exactly
    /// where a question is pending and unanswered, so it has to travel or the card would be lost.
    #[serde(default)]
    pub draw: CardDraw,
    #[serde(default)]
    pub question: Option<Question>,
    #[serde(default)]
    pub choice_taken: [u32; SEAT_COUNT],
    #[serde(default)]
    pub choice_refused: [u32; SEAT_COUNT],
    #[serde(default)]
    pub choice_not_asked: [u32; SEAT_COUNT],
    /// Ticket #345 (version 0.09.1): who was first to each Body.
    #[serde(default)]
    pub body_firsts: Vec<BodyFirst>,
    /// Ticket #444 (version 0.09.6): each Colony's growth toward its next Colonist.
    #[serde(default)]
    pub colony_growth: BTreeMap<ColonyId, f64>,
}

impl SavedGame {
    /// Everything but the tables, taken field by field. The destructuring is exhaustive on purpose:
    /// a field added to `Game` and forgotten here will not compile.
    pub fn of(g: &Game) -> SavedGame {
        let Game {
            tables: _,
            seed,
            rng,
            turn,
            seats,
            states,
            colonies,
            ships,
            armies,
            war,
            fought,
            widgets,
            levies_raised,
            neutral_holds,
            climate,
            research,
            deck,
            draw,
            question,
            choice_taken,
            choice_refused,
            choice_not_asked,
            discoveries,
            antarctic_sends,
            solar_maximum_next,
            last_event,
            report,
            outcome,
            next_id,
            pending,
            antarctica_open,
            slot_yields,
            spectator,
            log,
            relations,
            market,
            accords,
            events_no_target,
            body_firsts,
            colony_growth,
            reveal_all: _,
            waiting_last: _,
        } = g;
        SavedGame {
            seed: *seed,
            rng: rng.clone(),
            turn: *turn,
            seats: seats.clone(),
            states: states.clone(),
            colonies: colonies.clone(),
            ships: ships.clone(),
            armies: armies.clone(),
            climate: climate.clone(),
            research: research.clone(),
            deck: deck.clone(),
            draw: *draw,
            question: question.clone(),
            choice_taken: *choice_taken,
            choice_refused: *choice_refused,
            choice_not_asked: *choice_not_asked,
            discoveries: discoveries.clone(),
            antarctic_sends: antarctic_sends.clone(),
            solar_maximum_next: *solar_maximum_next,
            last_event: last_event.clone(),
            report: report.clone(),
            outcome: outcome.clone(),
            next_id: *next_id,
            pending: pending.clone(),
            antarctica_open: *antarctica_open,
            slot_yields: slot_yields.clone(),
            spectator: *spectator,
            log: log.clone(),
            relations: relations.clone(),
            market: market.clone(),
            accords: accords.clone(),
            events_no_target: *events_no_target,
            levies_raised: *levies_raised,
            neutral_holds: *neutral_holds,
            war: war.clone(),
            fought: fought.clone(),
            widgets: widgets.clone(),
            body_firsts: body_firsts.clone(),
            colony_growth: colony_growth.clone(),
        }
    }

    /// The saved state against the tables on disk.
    pub fn into_game(self, tables: Arc<Tables>) -> Game {
        Game {
            tables,
            seed: self.seed,
            rng: self.rng,
            turn: self.turn,
            seats: self.seats,
            states: self.states,
            colonies: self.colonies,
            ships: self.ships,
            armies: self.armies,
            war: self.war,
            fought: self.fought,
            widgets: self.widgets,
            levies_raised: self.levies_raised,
            neutral_holds: self.neutral_holds,
            climate: self.climate,
            research: self.research,
            deck: self.deck,
            draw: self.draw,
            question: self.question,
            choice_taken: self.choice_taken,
            choice_refused: self.choice_refused,
            choice_not_asked: self.choice_not_asked,
            discoveries: self.discoveries,
            antarctic_sends: self.antarctic_sends,
            solar_maximum_next: self.solar_maximum_next,
            last_event: self.last_event,
            report: self.report,
            outcome: self.outcome,
            next_id: self.next_id,
            pending: self.pending,
            antarctica_open: self.antarctica_open,
            slot_yields: self.slot_yields,
            spectator: self.spectator,
            relations: self.relations,
            market: self.market,
            accords: self.accords,
            events_no_target: self.events_no_target,
            body_firsts: self.body_firsts,
            colony_growth: self.colony_growth,
            log: self.log,
            reveal_all: false,
            waiting_last: Vec::new(),
        }
    }
}

/// The header a save of this game would carry.
pub fn header_of(game: &Game, kind: SaveKind) -> SaveHeader {
    SaveHeader {
        save_version: SAVE_VERSION,
        game_version: GAME_VERSION.to_string(),
        kind,
        seed: game.seed,
        turn: game.turn,
        faction: if game.spectator { SPECTATING.to_string() } else { game.seat_name(Seat(0)) },
        spectator: game.spectator,
        date: game.date_text(),
        temperature: game.climate.temperature,
    }
}

/// `save-<seed>-turn-<n>.ron`, or `autosave-<seed>-turn-<n>.ron`.
pub fn file_name(kind: SaveKind, seed: u64, turn: u32) -> String {
    match kind {
        SaveKind::Manual => format!("save-{seed}-turn-{turn}.ron"),
        SaveKind::Autosave => format!("autosave-{seed}-turn-{turn}.ron"),
    }
}

/// The whole file as text: the header on the first line, the game under it.
pub fn to_text(game: &Game, kind: SaveKind) -> Result<String, String> {
    let header = ron::ser::to_string(&header_of(game, kind)).map_err(|e| format!("the save's header could not be written: {e}"))?;
    // One line ending throughout: RON would otherwise write CRLF on Windows under a header line
    // written with LF, and a file of two minds is not a readable one.
    let pretty = ron::ser::PrettyConfig::new().struct_names(false).indentor("  ").new_line("\n");
    let body = ron::ser::to_string_pretty(&SavedGame::of(game), pretty).map_err(|e| format!("the game could not be written: {e}"))?;
    Ok(format!("{header}\n// Dying Earth: the whole state of one game, as RON. The line above is the save's header.\n{body}\n"))
}

/// Write a save into `dir`, creating the folder if it is not there. The write is atomic: the text
/// goes to a temporary file beside the target and is renamed over it, so a crash or a full disk
/// never leaves half a save where a good one used to be.
///
/// Every failure comes back as a sentence the interface can show; nothing here panics.
pub fn save_to(dir: &Path, game: &Game, kind: SaveKind) -> Result<PathBuf, String> {
    let text = to_text(game, kind)?;
    std::fs::create_dir_all(dir).map_err(|e| format!("The saves folder {} could not be made: {e}", dir.display()))?;
    let path = dir.join(file_name(kind, game.seed, game.turn));
    let tmp = path.with_extension("ron.tmp");
    std::fs::write(&tmp, text.as_bytes()).map_err(|e| format!("{} could not be written: {e}", tmp.display()))?;
    std::fs::rename(&tmp, &path).map_err(|e| {
        let _ = std::fs::remove_file(&tmp);
        format!("{} could not be written: {e}", path.display())
    })?;
    Ok(path)
}

/// Split a save's text into its header line and the game under it.
fn split(text: &str) -> Result<(&str, &str), String> {
    match text.split_once('\n') {
        Some((head, rest)) if !head.trim().is_empty() => Ok((head.trim(), rest)),
        _ => Err("This file is not a Dying Earth save: it has no header line.".to_string()),
    }
}

/// The header of one save file, without reading the game under it.
pub fn read_header(text: &str) -> Result<SaveHeader, String> {
    let (head, _) = split(text)?;
    let header: SaveHeader =
        ron::from_str(head).map_err(|e| format!("This file is not a Dying Earth save: its header could not be read ({e})."))?;
    Ok(header)
}

/// The refusal a save from another version earns, naming both versions. A save is never migrated.
pub fn version_refusal(header: &SaveHeader) -> Option<String> {
    if header.save_version == SAVE_VERSION {
        return None;
    }
    Some(format!(
        "This save was written by version {} of the rules; this is {}. It cannot be loaded.",
        header.game_version, GAME_VERSION
    ))
}

/// Read a save back into a game, against the tables on disk. A file from another version, or one
/// that is damaged, comes back as a sentence rather than a panic.
pub fn load_from(path: &Path, tables: Arc<Tables>) -> Result<Game, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{} could not be read: {e}", path.display()))?;
    let header = read_header(&text)?;
    if let Some(refusal) = version_refusal(&header) {
        return Err(refusal);
    }
    let (_, body) = split(&text)?;
    let saved: SavedGame = ron::from_str(body).map_err(|e| format!("This save is damaged and cannot be loaded ({e})."))?;
    let mut game = saved.into_game(tables);
    // Ticket #405 (version 0.09.4): a save from before the latch, written while the world stood
    // under the Sink, has had its first turn under it; it must not be announced again. A save
    // written since carries the latch set whenever the run is above nought, so this changes nothing.
    if game.seats.iter().any(|s| s.stabilization_run > 0) {
        game.climate.under_sink_eased = true;
    }
    // Ticket #442 (version 0.09.6): a Body with no ground has no low orbit, so a Ship a save left
    // there -- or flying there to arrive in it -- goes to the first station orbit.
    let lowless: Vec<(usize, BodyId)> = game
        .ships
        .iter()
        .enumerate()
        .filter_map(|(i, s)| match s.at {
            ShipAt::Body(b) | ShipAt::Transit { to: b, .. } if s.slot.is_none() && !game.has_low_orbit(b) => Some((i, b)),
            _ => None,
        })
        .collect();
    for (i, _) in lowless {
        game.ships[i].slot = Some(0);
    }
    Ok(game)
}

/// Every save in the folder, newest first. A folder that is not there, and a file that is not a
/// save, are both simply nothing to list.
pub fn list_saves(dir: &Path) -> Vec<SaveEntry> {
    let Ok(entries) = std::fs::read_dir(dir) else { return Vec::new() };
    let mut found: Vec<SaveEntry> = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("ron") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let Ok(header) = read_header(&text) else { continue };
        let (saved, bytes) = match entry.metadata() {
            Ok(m) => (m.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH), m.len()),
            Err(_) => (std::time::SystemTime::UNIX_EPOCH, 0),
        };
        found.push(SaveEntry { path, header, saved, bytes });
    }
    // Newest first, and a tie broken by the turn so two saves written in the same file-time tick
    // still read in the order they were made.
    found.sort_by(|a, b| b.saved.cmp(&a.saved).then(b.header.turn.cmp(&a.header.turn)));
    found
}

/// Keep only the newest `keep` autosaves of one game, by turn, and delete the rest. Returns what
/// was deleted.
pub fn rotate_autosaves(dir: &Path, seed: u64, keep: usize) -> Vec<PathBuf> {
    let mut mine: Vec<(u32, PathBuf)> = list_saves(dir)
        .into_iter()
        .filter(|e| e.header.kind.is_autosave() && e.header.seed == seed)
        .map(|e| (e.header.turn, e.path))
        .collect();
    // Newest turn first; everything past `keep` goes.
    mine.sort_by_key(|a| std::cmp::Reverse(a.0));
    let mut removed = Vec::new();
    for (_, path) in mine.into_iter().skip(keep) {
        if std::fs::remove_file(&path).is_ok() {
            removed.push(path);
        }
    }
    removed
}

/// Ticket #59: whether this turn start earns an autosave — every third turn, and the turn the game
/// ends on, whichever turn that is.
pub fn autosave_due(turn: u32, game_over: bool) -> bool {
    game_over || (turn > 0 && turn.is_multiple_of(AUTOSAVE_EVERY))
}

/// Write the autosave this turn start earns, if it earns one, and drop the oldest so only
/// `AUTOSAVES_KEPT` of this game's remain. `None` means none was due.
pub fn autosave(dir: &Path, game: &Game) -> Option<Result<PathBuf, String>> {
    if !autosave_due(game.turn, game.is_over()) {
        return None;
    }
    let written = save_to(dir, game, SaveKind::Autosave);
    if written.is_ok() {
        rotate_autosaves(dir, game.seed, AUTOSAVES_KEPT);
    }
    Some(written)
}

/// Ticket #59: a Save captures a turn start, so the Save button is dead while any order is pending.
/// Ticket #383 (version 0.09.2): and while a stack has fought an Attack this turn, which cannot be
/// taken back as an order can; the save waits for the next turn's head.
pub fn can_save_now(pending_orders: usize, fought: bool) -> bool {
    pending_orders == 0 && !fought
}

/// Ticket #383: why the Save button is dead after a fought Attack.
pub const SAVE_FOUGHT_HOVER: &str = "A Battle was fought this turn, so this is no longer a turn start; save at the next.";

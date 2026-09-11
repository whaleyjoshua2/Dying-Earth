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
pub const SAVE_VERSION: u32 = 1;

/// The rules version this executable plays, named beside the file's own in a refusal.
pub const GAME_VERSION: &str = "0.05.5";

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
            climate,
            research,
            deck,
            discoveries,
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
            discoveries: discoveries.clone(),
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
            climate: self.climate,
            research: self.research,
            deck: self.deck,
            discoveries: self.discoveries,
            solar_maximum_next: self.solar_maximum_next,
            last_event: self.last_event,
            report: self.report,
            outcome: self.outcome,
            next_id: self.next_id,
            pending: self.pending,
            antarctica_open: self.antarctica_open,
            slot_yields: self.slot_yields,
            spectator: self.spectator,
            log: self.log,
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
    Ok(saved.into_game(tables))
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
pub fn can_save_now(pending_orders: usize) -> bool {
    pending_orders == 0
}

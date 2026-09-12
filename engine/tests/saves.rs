//! Ticket #59: the tests for a Save — a turn start written to a file and read back.

use dying_earth_engine::data::{default_data_dir, Tables};
use dying_earth_engine::save::{self, SaveKind};
use dying_earth_engine::*;
use rand::Rng;
use std::path::PathBuf;
use std::sync::Arc;

fn tables() -> Arc<Tables> {
    Arc::new(Tables::load(&default_data_dir()).expect("tables load"))
}

/// A game the computer plays on every seat, run to the start of `turn`.
fn played_to(seed: u64, turn: u32) -> Game {
    let mut g = Game::spectate(tables(), seed);
    g.start();
    while g.turn < turn && !g.is_over() {
        g.end_turn(std::array::from_fn(|_| Vec::new())).expect("the turn should end");
    }
    assert_eq!(g.turn, turn, "the game reached turn {turn}");
    g
}

/// A folder of its own for one test, removed when the test ends.
struct TempDir(PathBuf);

impl TempDir {
    fn new(name: &str) -> TempDir {
        let n = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("dying-earth-{name}-{n}"));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("temp folder");
        TempDir(dir)
    }
    fn path(&self) -> &std::path::Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// (a) A game played to turn 7 goes to RON and comes back the same game: the same text a second
/// time, the same floats to the last bit, and a generator that draws exactly what it was going to.
#[test]
fn a_game_survives_the_round_trip_to_ron_and_back() {
    let dir = TempDir::new("round-trip");
    let mut original = played_to(11, 7);
    let path = save::save_to(dir.path(), &original, SaveKind::Manual).expect("the save is written");
    assert_eq!(path.file_name().unwrap().to_string_lossy(), "save-11-turn-7.ron", "the file is named for its seed and its turn");

    let mut loaded = save::load_from(&path, tables()).expect("the save is read back");

    // The whole state, written again: byte for byte the same file.
    let first = save::to_text(&original, SaveKind::Manual).unwrap();
    let second = save::to_text(&loaded, SaveKind::Manual).unwrap();
    assert_eq!(first, second, "a loaded game writes the same save text as the game it came from");

    // Floats to the last bit, which a text comparison alone would not catch.
    assert_eq!(loaded.climate.co2, original.climate.co2, "the CO2 Stock");
    assert_eq!(loaded.climate.temperature, original.climate.temperature, "the Temperature");
    assert_eq!(loaded.climate.natural_sink, original.climate.natural_sink, "the Natural Sink");
    for (a, b) in loaded.states.iter().zip(original.states.iter()) {
        assert_eq!(a.population, b.population, "the population of {:?}", b.id);
        assert_eq!(a.unrest, b.unrest, "the Unrest of {:?}", b.id);
    }
    for seat in Seat::ALL {
        assert_eq!(loaded.seat(seat).blame_emitted, original.seat(seat).blame_emitted, "the Blame of seat {seat:?}");
    }

    // The Event Deck in its shuffled order, and what has gone from it.
    assert_eq!(loaded.deck.cards, original.deck.cards, "the Event Deck, card for card");
    assert_eq!(loaded.deck.drawn, original.deck.drawn, "the cards already drawn");

    // The generator itself: the next hundred draws are the draws the unsaved game would have made.
    let theirs: Vec<f64> = (0..100).map(|_| original.rng.random::<f64>()).collect();
    let ours: Vec<f64> = (0..100).map(|_| loaded.rng.random::<f64>()).collect();
    assert_eq!(ours, theirs, "the generator's next draws");
}

/// (b) The proof that matters: a game saved at turn 7 and a game never saved play the next five
/// turns identically, down to the last line of the log.
#[test]
fn a_loaded_game_plays_on_exactly_as_the_unsaved_one_would() {
    let dir = TempDir::new("continuity");
    let mut original = played_to(23, 7);
    let path = save::save_to(dir.path(), &original, SaveKind::Manual).expect("the save is written");
    let mut loaded = save::load_from(&path, tables()).expect("the save is read back");

    while original.turn < 12 && !original.is_over() {
        original.end_turn(std::array::from_fn(|_| Vec::new())).expect("the turn should end");
    }
    while loaded.turn < 12 && !loaded.is_over() {
        loaded.end_turn(std::array::from_fn(|_| Vec::new())).expect("the turn should end");
    }

    assert_eq!(loaded.turn, original.turn, "both games stand on the same turn");
    assert_eq!(loaded.log.len(), original.log.len(), "both games wrote the same number of log lines");
    for (i, (a, b)) in loaded.log.iter().zip(original.log.iter()).enumerate() {
        assert_eq!(a, b, "log line {i} differs between the loaded game and the unsaved one");
    }
    assert_eq!(save::to_text(&loaded, SaveKind::Manual).unwrap(), save::to_text(&original, SaveKind::Manual).unwrap(), "the two boards at turn 12");
}

/// (c) A save stamped with another version is refused by a sentence naming both versions, and a
/// damaged file is refused by a sentence. Neither panics, and neither half-loads.
#[test]
fn a_save_from_another_version_and_a_damaged_file_are_both_refused_with_a_message() {
    let dir = TempDir::new("refusal");
    let game = played_to(5, 4);
    let good = save::save_to(dir.path(), &game, SaveKind::Manual).expect("the save is written");
    let text = std::fs::read_to_string(&good).unwrap();

    // The same save, stamped by the version before this one (ticket #68: read from the constant, so
    // the test follows the stamp when a version bumps it).
    let older = text.replacen("save_version:1", "save_version:0", 1).replacen(&format!("game_version:\"{}\"", save::GAME_VERSION), "game_version:\"0.05\"", 1);
    assert_ne!(older, text, "the stamp was actually changed");
    let stamped = dir.path().join("save-5-turn-4-from-0.05.ron");
    std::fs::write(&stamped, &older).unwrap();
    let refused = save::load_from(&stamped, tables()).expect_err("a save from another version is refused");
    assert_eq!(refused, format!("This save was written by version 0.05 of the rules; this is {}. It cannot be loaded.", save::GAME_VERSION), "the refusal names both versions");

    // A file that is not a save at all.
    let rubbish = dir.path().join("rubbish.ron");
    std::fs::write(&rubbish, "this is not a save at all\n{{{ broken\n").unwrap();
    let e = save::load_from(&rubbish, tables()).expect_err("a file that is not a save is refused");
    assert!(!e.is_empty(), "the refusal says something");

    // A save whose header is good and whose game is cut in half.
    let truncated = dir.path().join("truncated.ron");
    let half = &text[..text.len() / 2];
    std::fs::write(&truncated, half).unwrap();
    let e = save::load_from(&truncated, tables()).expect_err("a damaged save is refused");
    assert!(e.starts_with("This save is damaged"), "a damaged save says so, not {e:?}");

    // And the good file still loads, so the refusals are discriminating.
    save::load_from(&good, tables()).expect("the undamaged save still loads");
}

/// (d) The autosave cadence and the rotation: turns 3, 6, 9 and 12 each write one, and only the
/// last three of a game's autosaves are kept.
#[test]
fn the_game_autosaves_every_third_turn_and_keeps_the_last_three() {
    let dir = TempDir::new("cadence");
    assert_eq!(save::AUTOSAVE_EVERY, 3, "every third turn");
    assert_eq!(save::AUTOSAVES_KEPT, 3, "the last three per game");

    let mut g = Game::spectate(tables(), 31);
    g.start();
    let mut written: Vec<u32> = Vec::new();
    while g.turn < 12 && !g.is_over() {
        g.end_turn(std::array::from_fn(|_| Vec::new())).expect("the turn should end");
        if save::autosave(dir.path(), &g).transpose().expect("the autosave is written").is_some() {
            written.push(g.turn);
        }
    }
    assert_eq!(written, vec![3, 6, 9, 12], "an autosave at the start of every third turn and nowhere else");

    let kept: Vec<u32> = save::list_saves(dir.path()).iter().map(|e| e.header.turn).collect();
    let mut sorted = kept.clone();
    sorted.sort_unstable();
    assert_eq!(sorted, vec![6, 9, 12], "only the last three autosaves are left; the turn-3 one has gone");
    assert!(!dir.path().join("autosave-31-turn-3.ron").exists(), "the oldest autosave was deleted");
    for turn in [6, 9, 12] {
        assert!(dir.path().join(format!("autosave-31-turn-{turn}.ron")).exists(), "autosave-31-turn-{turn}.ron is there");
    }

    // Another game's autosaves are its own: rotation never touches a different seed.
    let other = played_to(32, 3);
    save::autosave(dir.path(), &other).expect("due at turn 3").expect("written");
    let seeds: Vec<u64> = save::list_saves(dir.path()).iter().map(|e| e.header.seed).collect();
    assert_eq!(seeds.iter().filter(|s| **s == 31).count(), 3, "seed 31 still keeps three");
    assert_eq!(seeds.iter().filter(|s| **s == 32).count(), 1, "seed 32 has its own one");
}

/// (d, second half) The Load list reads every save's Faction, turn, month, Temperature and seed
/// off the file, newest first, and says which are autosaves.
#[test]
fn the_load_list_reads_every_save_off_the_folder() {
    let dir = TempDir::new("listing");
    assert!(save::list_saves(dir.path()).is_empty(), "an empty folder lists nothing");
    assert!(save::list_saves(&dir.path().join("not-there")).is_empty(), "a folder that is not there lists nothing");

    let g = played_to(41, 5);
    save::save_to(dir.path(), &g, SaveKind::Manual).expect("a manual save");
    save::save_to(dir.path(), &g, SaveKind::Autosave).expect("an autosave");
    let list = save::list_saves(dir.path());
    assert_eq!(list.len(), 2, "both saves are listed");
    for e in &list {
        assert_eq!(e.header.seed, 41, "the seed");
        assert_eq!(e.header.turn, 5, "the turn");
        assert_eq!(e.header.date, g.date_text(), "the month");
        assert_eq!(e.header.temperature, g.climate.temperature, "the Temperature");
        assert_eq!(e.header.faction, save::SPECTATING, "a game nobody sits at is listed as Spectating");
        assert!(e.bytes > 0, "the file has a size");
    }
    assert_eq!(list.iter().filter(|e| e.header.kind.is_autosave()).count(), 1, "one of the two is an autosave");

    // A game somebody sits at is listed by its Faction.
    let mut mine = Game::new(tables(), NewGame { seed: 42, player: FactionKind::Arkwrights, player_is_ai: false, player_start: StateId::EastAsia });
    mine.start();
    save::save_to(dir.path(), &mine, SaveKind::Manual).expect("a manual save");
    let named = save::list_saves(dir.path()).into_iter().find(|e| e.header.seed == 42).expect("the new save is listed");
    assert_eq!(named.header.faction, "Arkwrights", "the player's Faction names the row");
    assert!(!named.header.spectator, "it is not a spectated game");
}

/// (e) The pure check the Save button reads: a save captures a turn start, so it is refused while
/// any order is pending.
#[test]
fn a_manual_save_is_refused_while_orders_are_pending() {
    assert!(save::can_save_now(0), "with nothing ordered, a save may be taken");
    assert!(!save::can_save_now(1), "one pending order is enough to refuse a save");
    assert!(!save::can_save_now(7), "so is seven");
    assert!(save::SAVE_PENDING_HOVER.contains("beginning of a turn"), "the hover says why");
}

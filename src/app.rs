//! Shared resources: the session (one game and the player's pending orders) and the view state.

pub use crate::textures::Textures;
use bevy::prelude::*;
use dying_earth_engine::save::{self, SaveKind};
use dying_earth_engine::*;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Play,
    /// `shot:<prefix>`: capture the four views off-screen and exit.
    Shot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    Title,
    /// Ticket #59: the Load list, reached from the title screen.
    Load,
    ChooseFaction,
    ChooseStart { faction: FactionKind },
    Playing,
    GameOver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Solar,
    Surface(BodyId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Selection {
    None,
    State(StateId),
    Colony(ColonyId),
    Slot(BodyId, u32),
    ShipStack(BodyId, Seat),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Popup {
    None,
    Event,
    /// Ticket #58: the nth Moment of this turn, shown before the Report.
    Moment(usize),
    Report,
    /// End Turn pressed with Influence unspent (ticket #31): ask once.
    ConfirmEndTurn,
}

#[derive(Resource)]
pub struct Session {
    pub tables: Arc<Tables>,
    pub game: Option<Game>,
    pub pending: Vec<Order>,
    pub screen: Screen,
    pub seed: u64,
    pub mode: Mode,
    pub shot_prefix: String,
    /// The Earth Map must be recomposed (control, occupation or the sea changed).
    pub earth_dirty: bool,
    pub last_error: Option<String>,
    /// Ticket #64: nobody is playing this game. All four seats are the computer's, the interface
    /// gives no orders, and every Faction's board is open to be read.
    pub spectator: bool,
    /// Ticket #64: Auto is ticked, so a turn runs every `AUTO_INTERVAL` seconds.
    pub auto: bool,
    /// Seconds since the last turn Auto ran; it does not count while a popup is up.
    pub auto_elapsed: f32,
    /// Ticket #59: the saves folder, or why there is not one. A game with nowhere to save keeps
    /// playing and says so.
    pub saves: Result<PathBuf, String>,
    /// Ticket #59: what the top bar says after a save, and how many seconds it has left to say it.
    pub save_notice: Option<(String, f32)>,
    /// Ticket #59: the Load screen's list, read from the folder when the screen opens.
    pub saves_list: Vec<SaveEntry>,
    /// Ticket #59: a Delete asks once; this is the row it is asking about.
    pub confirm_delete: Option<PathBuf>,
}

/// Ticket #59: how long "Saved." stands in the top bar.
pub const SAVE_NOTICE_SECONDS: f32 = 4.0;

/// Ticket #64: how long Auto waits between turns.
pub const AUTO_INTERVAL: f32 = 3.0;

/// Ticket #64: whether Auto runs the next turn now. It runs only while the box is ticked, only when
/// no Moment, Report or game-over popup is up, and only once the interval has passed; the clock is
/// stopped, not merely ignored, while a popup is open, so closing one does not fire a turn at once.
pub fn auto_should_advance(auto_on: bool, popup_open: bool, elapsed: f32) -> bool {
    auto_on && !popup_open && elapsed >= AUTO_INTERVAL
}

impl Session {
    /// Ticket #50: one colour per seat, four of them.
    pub fn colours(&self) -> Vec<[f32; 3]> {
        match &self.game {
            Some(g) => Seat::ALL.iter().map(|s| self.tables.faction(g.kind(*s)).colour).collect(),
            None => FactionKind::ALL.iter().map(|k| self.tables.faction(*k).colour).collect(),
        }
    }

    pub fn new_game(&mut self, faction: FactionKind, start: StateId) {
        let mut game = Game::new(self.tables.clone(), NewGame { seed: self.seed, player: faction, player_is_ai: false, player_start: start });
        game.start();
        self.begin(game, false);
    }

    /// Ticket #64: Spectate. The computer takes all four seats and picks every start by its own
    /// spreading rule, so no continent is chosen and no orders are ever given.
    pub fn spectate(&mut self) {
        let mut game = Game::spectate(self.tables.clone(), self.seed);
        game.start();
        self.begin(game, true);
    }

    fn begin(&mut self, game: Game, spectator: bool) {
        self.seed = game.seed;
        self.game = Some(game);
        self.pending.clear();
        self.screen = Screen::Playing;
        self.earth_dirty = true;
        self.spectator = spectator;
        self.auto = false;
        self.auto_elapsed = 0.0;
        self.save_notice = None;
        self.confirm_delete = None;
        self.last_error = None;
    }

    // ---------------------------------------------------------------- Ticket #59: saves

    /// The Save button: a manual save of this turn start. It is dead while an order is pending, so
    /// a save never holds half-entered orders.
    pub fn save_now(&mut self) {
        if !save::can_save_now(self.pending.len()) {
            self.note(save::SAVE_PENDING_HOVER.to_string());
            return;
        }
        let Some(game) = &self.game else { return };
        let note = match &self.saves {
            Err(e) => e.clone(),
            Ok(dir) => match save::save_to(dir, game, SaveKind::Manual) {
                Ok(_) => "Saved.".to_string(),
                Err(e) => e,
            },
        };
        self.note(note);
    }

    /// Ticket #59: the autosave this turn start earns, if it earns one. A folder that cannot be
    /// written says so in the top bar and the game plays on.
    fn autosave(&mut self) {
        let Ok(dir) = self.saves.clone() else { return };
        let Some(game) = &self.game else { return };
        if let Some(Err(e)) = save::autosave(&dir, game) {
            self.note(e);
        }
    }

    fn note(&mut self, text: String) {
        self.save_notice = Some((text, SAVE_NOTICE_SECONDS));
    }

    /// Read the saves folder afresh, newest first.
    pub fn refresh_saves(&mut self) {
        self.saves_list = match &self.saves {
            Ok(dir) => save::list_saves(dir),
            Err(_) => Vec::new(),
        };
        self.confirm_delete = None;
        self.last_error = None;
    }

    /// Load a save: the game comes back exactly where it was at that turn start.
    pub fn load_save(&mut self, path: &std::path::Path) -> Result<(), String> {
        let game = save::load_from(path, self.tables.clone())?;
        let spectator = game.spectator;
        let over = game.is_over();
        self.begin(game, spectator);
        if over {
            self.screen = Screen::GameOver;
        }
        Ok(())
    }

    pub fn delete_save(&mut self, path: &std::path::Path) {
        if let Err(e) = std::fs::remove_file(path) {
            self.last_error = Some(format!("{} could not be deleted: {e}", path.display()));
        }
        self.refresh_saves();
    }

    /// Try to add an order; on failure remember why so the panel can show it.
    pub fn place(&mut self, order: Order) -> bool {
        let Some(game) = &self.game else { return false };
        match game.check_order(Seat(0), &self.pending, &order) {
            Ok(_) => {
                self.pending.push(order);
                self.last_error = None;
                true
            }
            Err(e) => {
                self.last_error = Some(e.0);
                false
            }
        }
    }

    pub fn end_turn(&mut self) {
        let Some(game) = &mut self.game else { return };
        let orders = std::mem::take(&mut self.pending);
        let mut all: [Vec<Order>; SEAT_COUNT] = std::array::from_fn(|_| Vec::new());
        all[0] = orders;
        game.end_turn(all);
        self.earth_dirty = true;
        let over = game.is_over();
        // Ticket #59: the autosave is written at the start of the Report phase of every third turn,
        // and at game over, which is where `end_turn` leaves the game.
        self.autosave();
        if over {
            self.screen = Screen::GameOver;
        }
    }
}

#[derive(Resource)]
pub struct ViewState {
    pub view: View,
    pub last_surface: BodyId,
    pub yaw: f32,
    pub pitch: f32,
    pub solar_yaw: f32,
    pub zoom: f32,
    pub spin: f32,
    /// Ticket #100 (version 0.07.0): the start-screen globe has been taken hold of, so it stops
    /// spinning for good and answers the pointer from here on.
    pub start_grabbed: bool,
    /// Ticket #100: the Faction the start globe was last aimed for, so the opening view is set once
    /// and a drag is never undone by the next frame.
    pub start_aimed: Option<FactionKind>,
    pub selection: Selection,
    pub popup: Popup,
    pub show_tech: bool,
    /// The Tech Tree opened itself for a pending pick; it does not reopen until the next pick.
    pub tech_prompted: bool,
    pub show_climate: bool,
    /// Ticket #41: the Climate Panel was just reopened; put it back at its home position once.
    pub climate_reopen: bool,
    pub show_victory: bool,
    /// Ticket #42: the trading window, and the quantity on each of its four lines
    /// (Influence, Materials, Fuel, Energy).
    pub show_trade: bool,
    pub trade_amounts: [i64; 4],
    pub load_state: Option<StateId>,
    pub influence_amount: i64,
    pub attack_preview: bool,
    /// Ticket #58: which Moment kinds are switched on, remembered for the session. `None` until the
    /// player touches a checkbox, when it is filled from the defaults in `report.toml`.
    pub moments_on: Option<[bool; dying_earth_engine::MomentKind::ALL.len()]>,
    /// Ticket #57, a building aid (`hover:<body id>`): the Solar System Map draws that Body's launch
    /// window tooltip as though the pointer were on it, so a picture can be taken of it.
    pub force_hover: Option<BodyId>,
}

impl Default for ViewState {
    fn default() -> Self {
        ViewState {
            view: View::Surface(BodyId::Earth),
            last_surface: BodyId::Earth,
            yaw: crate::geo::yaw_facing(0.0, 0.0),
            pitch: 0.0,
            solar_yaw: 0.0,
            zoom: 1.0,
            spin: 0.0,
            start_grabbed: false,
            start_aimed: None,
            selection: Selection::None,
            popup: Popup::None,
            show_tech: false,
            tech_prompted: false,
            show_climate: true,
            climate_reopen: false,
            show_victory: false,
            show_trade: false,
            trade_amounts: [5, 10, 10, 10],
            load_state: None,
            influence_amount: 5,
            attack_preview: false,
            moments_on: None,
            force_hover: None,
        }
    }
}

impl ViewState {
    /// Ticket #58: whether a Moment kind stops the turn, the session's answer over the table's.
    pub fn moment_on(&self, tables: &Tables, kind: dying_earth_engine::MomentKind) -> bool {
        match &self.moments_on {
            Some(on) => on[dying_earth_engine::MomentKind::ALL.iter().position(|k| *k == kind).unwrap_or(0)],
            None => tables.report.moment_on(kind),
        }
    }

    /// The same, as a closure the engine's `moments_shown` can read.
    pub fn moments_of<'a>(&'a self, tables: &'a Tables, report: &'a dying_earth_engine::Report) -> Vec<&'a dying_earth_engine::Moment> {
        report.moments_shown(&|k| self.moment_on(tables, k))
    }

    pub fn enter_surface(&mut self, body: BodyId) {
        self.view = View::Surface(body);
        self.last_surface = body;
        self.selection = Selection::None;
        self.yaw = crate::geo::yaw_facing(0.0, 0.0);
        self.pitch = 0.0;
        self.zoom = 1.0;
    }
    pub fn swap(&mut self) {
        self.view = match self.view {
            View::Solar => View::Surface(self.last_surface),
            View::Surface(_) => View::Solar,
        };
        self.selection = Selection::None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ticket #64 (c): Auto advances only when it is ticked, no popup is up, and three seconds have
    /// passed since the last turn.
    #[test]
    fn auto_advances_only_when_ticked_unblocked_and_three_seconds_have_passed() {
        assert_eq!(AUTO_INTERVAL, 3.0, "three seconds a turn");
        // Ticked, nothing in the way, the interval passed.
        assert!(auto_should_advance(true, false, AUTO_INTERVAL));
        assert!(auto_should_advance(true, false, AUTO_INTERVAL + 0.5));
        // Unticked: never, however long it has been.
        assert!(!auto_should_advance(false, false, 10.0));
        // A Moment, the Report or the game-over popup is up: never, however long it has been.
        assert!(!auto_should_advance(true, true, 10.0));
        // Ticked and clear, but the interval has not passed.
        assert!(!auto_should_advance(true, false, 0.0));
        assert!(!auto_should_advance(true, false, AUTO_INTERVAL - 0.01));
    }
}

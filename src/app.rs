//! Shared resources: the session (one game and the player's pending orders) and the view state.

pub use crate::textures::Textures;
use bevy::prelude::*;
use dying_earth_engine::*;
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
        self.game = Some(game);
        self.pending.clear();
        self.screen = Screen::Playing;
        self.earth_dirty = true;
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
        if game.is_over() {
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
            force_hover: None,
        }
    }
}

impl ViewState {
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

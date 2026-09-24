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
    /// Ticket #109 (version 0.07.0): the credits, reached from the title screen. The resource icons
    /// are CC BY 3.0 and their licence wants their authors named where a player can see them.
    Credits,
    ChooseStart { faction: FactionKind },
    Playing,
    GameOver,
    /// Ticket #338 (version 0.09.0): **the chronicle**, the page the game-over box opens: the four
    /// Factions ranked as the End phase ranked them, a table of what each ended the game holding,
    /// and the population and Temperature charts. A page of its own and not a box over the board,
    /// at the designer's word, since two charts and a table of nine columns do not sit in 520
    /// pixels. It is reached only from the game-over box and goes back to it.
    Chronicle,
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
    /// Ticket #337 (version 0.09.0): **the turn's Choice Card, asking its question.** Its own popup
    /// rather than the Event's, because a card that asks something is not a notice: it carries two
    /// buttons whose faces say what each side does, neither of them a default, and NOTHING dismisses
    /// it but an answer -- not Escape, not a click outside, not the key that ends the turn. It is
    /// raised at the head of the turn, where the Event would be, and hands on to the Moments and
    /// the Report once it has been answered.
    Card,
    /// Ticket #169 (version 0.07.5): a tutorial game's note for this turn, shown before everything
    /// else, since it says what the turn is for.
    Tutorial,
    /// Ticket #58: the nth Moment of this turn, shown before the Report.
    Moment(usize),
    Report,
    /// End Turn pressed with Influence unspent (ticket #31): ask once.
    ConfirmEndTurn,
    /// Ticket #105 (version 0.07.0): the turn was refused, and this says why. A rule nobody can see
    /// refused by is as bad as no rule, so the refusal always speaks.
    Refused,
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
    /// Ticket #105 (version 0.07.0): why the last End Turn was refused, shown once in its own popup.
    pub refusal: Option<String>,
    /// Ticket #169 (version 0.07.5): this game was begun with the tutorial asked for, so a note
    /// opens each of its first turns. The designer chose a **guided free game** over a scripted
    /// one: nothing is forced and nothing is checked, the notes only say where to look. It goes
    /// false once the last note has been shown, and the game carries on as any other.
    pub tutorial: bool,
    /// Ticket #174 (version 0.07.6): the `Play Tutorial` tick at the foot of the Custodians' card
    /// on the Faction screen. The designer: *"move tutorial choice to a radio box on custodian card
    /// during faction selection."* It is remembered while the screen is open, so looking at another
    /// card and coming back does not clear it, and it is read when the game begins -- after the
    /// start has been chosen, because a ticked card still picks its own Region.
    pub tutorial_ticked: bool,
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

/// Ticket #174 (version 0.07.6): whether a game about to begin is a tutorial game. The tutorial is
/// asked for by a tick at the foot of the Custodians' card, so it is on only when that tick is on
/// AND the Faction being played is the Custodians -- a player who ticks it, thinks better of it and
/// plays the Prospectors instead gets no notes, since every note is written about the Custodians.
pub fn tutorial_wanted(ticked: bool, kind: FactionKind) -> bool {
    ticked && kind == FactionKind::Custodians
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
        // Ticket #105: the engine owns the rule now. If it refuses, nothing was committed and the
        // orders go back where they came from, so nothing the player typed is lost.
        let kept = all[0].clone();
        if let Err(why) = game.end_turn(all) {
            self.refusal = Some(why);
            self.pending = kept;
            return;
        }
        self.refusal = None;
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

/// Ticket #128 (version 0.07.2): a shortcut the keyboard system cannot act on by itself, because
/// acting needs the game and the action list the interface owns. The keyboard records it here and
/// the interface takes it on its next frame, so a key does exactly what its button does.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HotKey {
    Save,
    EndTurn,
}

/// Ticket #145 (version 0.07.3): what is clicked in the Hab View.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HabTile {
    Module(usize),
    Free,
}

/// Ticket #335 (version 0.09.0), a building aid: a block of the Ship stack's card that a headless
/// picture asks to be scrolled to, since a capture cannot drag a scrollbar.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StackBlock {
    /// The Transits row, one line per destination ORBIT since this ticket.
    Transits,
    /// The Change orbit door, one line per other orbit at this Body.
    ChangeOrbit,
}

/// Ticket #146 (version 0.07.3): what is clicked among a Region card's slot boxes -- a standing
/// Facility by its index, or a free box, whose strip offers the build buttons.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SlotBox {
    Facility(usize),
    Free,
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
    /// Ticket #292 (version 0.08.6): where the top bar ENDS this frame, in screen pixels, measured
    /// from the panel it is drawn in. Every window that opens under the bar reads it through
    /// `below_bar`, where each used to carry its own guess (120, 120 and 104) and the Trading and
    /// Victory windows carried none and opened over the bar's figures. The bar is drawn before the
    /// windows in the same frame, so the figure is never a frame stale.
    pub top_bar_bottom: f32,
    pub show_victory: bool,
    /// Ticket #203 (version 0.08.1): the Faction window, and which SEAT's page it is open on. The
    /// dropdown in its top right names the four Factions, but every live figure on the page is a
    /// seat's, so the seat is what is remembered. It opens on seat 0 -- the player's own Faction,
    /// and in a spectated game the first of the four -- because a player's own rulebook is the one
    /// this window is the only way back to.
    pub show_factions: bool,
    pub faction_seat: Seat,
    /// `factions:<kind> rulebook:1` (a building aid, ticket #203): the Faction window's rulebook
    /// header starts OPEN, so a headless picture can be taken of it. It is shut by default in play
    /// and nothing but the aid sets this.
    pub faction_rulebook_open: bool,
    /// Ticket #42: the trading window, and the quantity on each of its four lines
    /// (Influence, Materials, Fuel, Energy).
    pub show_trade: bool,
    /// Ticket #145 (version 0.07.3): the Hab View, open on one station or Colony, and the tile
    /// clicked in it -- a standing Module by index, or a free slot, whose strip offers the build
    /// buttons.
    pub hab_tile: Option<HabTile>,
    /// Ticket #146 (version 0.07.3): the slot box clicked on the selected Region's card.
    pub slot_box: Option<SlotBox>,
    pub trade_amounts: [i64; 4],
    pub load_state: Option<StateId>,
    /// Ticket #204 (version 0.08.1): the Region a Colony's own loader draws from, kept apart from
    /// `load_state` so that choosing a Region on a Ship's card does not move it on a station's.
    pub lift_state: Option<StateId>,
    pub influence_amount: i64,
    /// Ticket #256 (version 0.08.4): the Ducats the Withdraw field on the Victory window asks for,
    /// kept as `influence_amount` is so the field remembers what was typed between frames.
    pub venture_withdraw: i64,
    /// Ticket #267 (version 0.08.4): the Influence the Smear field on a rival's page asks for.
    pub smear_amount: i64,
    /// Ticket #277 (version 0.08.5): the Influence in the Greenwash field on the player's own page.
    pub greenwash_amount: i64,
    /// Ticket #268 (version 0.08.4): the ppm the carbon-credit field asks for, and the offer a
    /// Custodian player is setting.
    pub credits_amount: i64,
    pub credits_offer: i64,
    pub attack_preview: bool,
    /// Ticket #323 (version 0.08.8): the Region whose stack of the player's Armies is ARMED for a
    /// right-click march, set by a click on its shield; cleared by any other click, Escape, End
    /// Turn or a change of view. `armed_scroll` asks the card to scroll to its Armies block once.
    pub armed_stack: Option<StateId>,
    pub armed_scroll: bool,
    /// Ticket #335 (version 0.09.0), a building aid (`scroll:transits`, `scroll:orbits`): the Ship
    /// stack card scrolls to that block and stays there. A headless picture cannot scroll a panel,
    /// and on a card with four Ships on it both blocks sit well below the fold.
    pub stack_scroll: Option<StackBlock>,
    /// Ticket #58: which Moment kinds are switched on, remembered for the session. `None` until the
    /// player touches a checkbox, when it is filled from the defaults in `report.toml`.
    pub moments_on: Option<[bool; dying_earth_engine::MomentKind::ALL.len()]>,
    /// Ticket #57, a building aid (`hover:<body id>`): the Solar System Map draws that Body's launch
    /// window tooltip as though the pointer were on it, so a picture can be taken of it.
    pub force_hover: Option<BodyId>,
    /// Ticket #337 (version 0.09.0), a building aid (`cardshut:1`): the turn's Choice Card is SET
    /// ASIDE, so the board behind it -- the End Turn sun greyed, with the refusal on its hover --
    /// can be photographed. Nothing in play sets it: in play the card cannot be set aside at all,
    /// and the modal comes straight back whenever nothing else is up.
    pub card_aside: bool,
    /// Ticket #114 (version 0.07.1), Max since ticket #134: the last turn the standing order was placed, so it is
    /// placed once a turn and not once a frame.
    pub max_placed: Option<u32>,
    /// Ticket #128 (version 0.07.2): a shortcut pressed and not yet acted on.
    pub hotkey: Option<HotKey>,
    /// Ticket #126 (version 0.07.2): on the start screen, the Region under the pointer, the Region
    /// the player has clicked, and the one the globe was last composed with lit -- so the globe is
    /// recomposed when the lit Region changes and not every frame.
    pub start_hover: Option<StateId>,
    pub start_selected: Option<StateId>,
    pub start_lit_drawn: Option<StateId>,
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
            // Ticket #104 (version 0.07.0): a new game opens on a clear map. The Climate Panel is
            // a keystroke (C) or a button away.
            show_climate: false,
            climate_reopen: false,
            top_bar_bottom: 0.0,
            show_victory: false,
            show_factions: false,
            faction_seat: Seat(0),
            faction_rulebook_open: false,
            show_trade: false,
            hab_tile: None,
            slot_box: None,
            trade_amounts: [5, 10, 10, 10],
            load_state: None,
            lift_state: None,
            influence_amount: 5,
            venture_withdraw: 10,
            smear_amount: 5,
            greenwash_amount: 5,
            credits_amount: 10,
            credits_offer: 0,
            attack_preview: false, armed_stack: None, armed_scroll: false, stack_scroll: None,
            moments_on: None,
            force_hover: None,
            card_aside: false,
            max_placed: None,
            hotkey: None,
            start_hover: None,
            start_selected: None,
            start_lit_drawn: None,
        }
    }
}

impl ViewState {
    /// Ticket #292 (version 0.08.6): the y a window opens at to clear the top bar -- its measured
    /// foot plus the sixteen pixels egui itself leaves from an edge. Before the first frame has
    /// measured anything it reads sixteen, which is where egui would have put the window anyway.
    pub fn below_bar(&self) -> f32 {
        self.top_bar_bottom + 16.0
    }

    /// Ticket #292: where the Trading and Victory windows open -- under the bar and to the right
    /// of the Faction window's home (16 across, 524 wide), so a player with both open sees both.
    pub fn beside_faction_window(&self) -> bevy_egui::egui::Pos2 {
        bevy_egui::egui::pos2(560.0, self.below_bar())
    }

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
        self.armed_stack = None;
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
        self.armed_stack = None;
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

    /// Ticket #174 (version 0.07.6): the tutorial is asked for by the tick at the foot of the
    /// Custodians' card, so it runs only for a Custodian game. The designer moved the choice there
    /// from the title screen's button: *"move tutorial choice to a radio box on custodian card
    /// during faction selection."* Guarded here rather than in a picture because the tick's effect
    /// is one turn deep into a game no capture reaches.
    #[test]
    fn the_tutorial_runs_only_when_the_tick_and_the_custodians_agree() {
        assert!(tutorial_wanted(true, FactionKind::Custodians), "ticked, and playing them");
        // Ticked, then a different Faction played: the notes are all about the Custodians.
        for k in [FactionKind::Prospectors, FactionKind::Arkwrights, FactionKind::Archivists] {
            assert!(!tutorial_wanted(true, k), "{k:?} has no notes written for it");
        }
        // Never ticked: no notes for anyone, the Custodians included.
        for k in FactionKind::ALL {
            assert!(!tutorial_wanted(false, k), "{k:?} was not asked for");
        }
    }

    /// Ticket #104 (version 0.07.0): a new game opens on a clear map. This is guarded here rather
    /// than in a picture because the screenshot harness forces the Climate Panel open for its Earth
    /// picture (`shot.rs`: `view.show_climate = v == View::Surface(BodyId::Earth)`), so no capture
    /// can ever show the default.
    #[test]
    fn a_new_game_opens_with_no_panel_showing() {
        let view = ViewState::default();
        assert!(!view.show_climate, "the Climate Panel waits for C or its button");
        assert!(!view.show_tech, "and the Tech Tree waits to be asked for");
        assert!(!view.show_trade);
        assert!(!view.show_victory);
    }

}

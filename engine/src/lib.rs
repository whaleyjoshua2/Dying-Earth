//! The rules of Dying Earth, the First Playable, with no window attached.
//!
//! `Game` holds one game. The interface reads its fields, builds `Order`s, and calls
//! `end_turn`. Simulate mode drives two AIs through `sim::run`.

pub mod ai;
pub mod climate;
pub mod combat;
pub mod data;
pub mod economy;
pub mod ephemeris;
pub mod events;
pub mod ids;
pub mod orders;
pub mod report;
pub mod research;
pub mod resolution;
pub mod save;
pub mod sim;
pub mod state;
pub mod turn;
pub mod victory;

pub use climate::{LastTurn, Projection};
pub use data::{DataError, Tables};
pub use economy::Yield;
pub use ephemeris::Position;
pub use ids::*;
pub use orders::{BuildingRef, Cost, LoadSource, Order, OrderError, UnitRef, UnloadTarget};
pub use save::{SaveEntry, SaveHeader, SaveKind};
pub use state::*;
pub use turn::Phase;

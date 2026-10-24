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
pub mod visibility;

pub use climate::{LastTurn, Projection};
pub use data::{DataError, Tables, TutorialNote};
pub use economy::{Chain, GoesDark, ShortfallForecast, Step, Yield};
pub use ephemeris::Position;
pub use ids::*;
pub use orders::{BuildingRef, Cost, LoadSource, Order, OrderError, RearmSite, TradeGood, UnitRef, UnloadTarget};
pub use save::{SaveEntry, SaveHeader, SaveKind};
pub use state::*;
pub use turn::Phase;

/// Ticket #551 (version 0.1.0.2): **a count and its noun, singular at one** -- "1 Facility",
/// "3 Facilities", "1 turn" -- for every line a player reads. Until now the plural was written
/// inline at some places and dodged with "(s)" or left wrong ("1 Facilities") at the rest. The
/// noun's two forms are given whole, since English plurals are not all an added "s".
pub fn counted(n: impl std::fmt::Display, one: &str, many: &str) -> String {
    let n = n.to_string();
    let noun = if n == "1" || n == "-1" { one } else { many };
    format!("{n} {noun}")
}

#[cfg(test)]
mod tests {
    use super::counted;

    #[test]
    fn a_count_takes_its_noun_singular_at_one_alone() {
        assert_eq!(counted(1, "Facility", "Facilities"), "1 Facility");
        assert_eq!(counted(0usize, "Facility", "Facilities"), "0 Facilities");
        assert_eq!(counted(3u32, "turn", "turns"), "3 turns");
        assert_eq!(counted(1i64, "Army", "Armies"), "1 Army");
        assert_eq!(counted(11, "slot", "slots"), "11 slots");
    }
}

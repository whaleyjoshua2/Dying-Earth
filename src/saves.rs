//! Ticket #59: where this machine keeps the game's saves, when a save was written, and how the
//! folder is opened. The engine knows how to write and read a save; it never knows where.

use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// The folder every save goes in: `%LOCALAPPDATA%\DyingEarth\data\saves`, found through the
/// `directories` crate, which is where Microsoft's guidance for game developers puts a file the
/// player does not open by hand. Never beside the executable: installed under Program Files that
/// write simply fails for a standard user.
///
/// `savedir:<path>` on the command line overrides it. That is how a `shot:` run writes its saves
/// into a folder of its own instead of the player's.
pub fn saves_dir() -> Result<PathBuf, String> {
    if let Some(dir) = std::env::args().find_map(|a| a.strip_prefix("savedir:").map(PathBuf::from)) {
        return Ok(dir);
    }
    match directories::ProjectDirs::from("", "", "DyingEarth") {
        Some(dirs) => Ok(dirs.data_local_dir().join("saves")),
        None => Err("Windows would not say where this machine keeps its application data, so the game cannot save.".to_string()),
    }
}

/// Open the saves folder in Explorer — the player's own file manager, not a window of the game's.
pub fn open_folder(dir: &Path) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("The saves folder {} could not be made: {e}", dir.display()))?;
    std::process::Command::new("explorer.exe")
        .arg(dir)
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("The saves folder could not be opened: {e}"))
}

/// "9 September 2026, 14:32" — a file's own time, in this machine's time zone.
pub fn when_text(t: SystemTime) -> String {
    civil_text(local_seconds(t))
}

/// Seconds since the epoch, shifted into local time so the civil conversion below reads local.
fn local_seconds(t: SystemTime) -> i64 {
    let utc = match t.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => d.as_secs() as i64,
        Err(e) => -(e.duration().as_secs() as i64),
    };
    #[cfg(windows)]
    {
        // A FILETIME counts 100-nanosecond ticks from 1601-01-01; the offset to the Unix epoch is
        // 11,644,473,600 seconds. `FileTimeToLocalFileTime` applies this machine's own time zone.
        const EPOCH_OFFSET: i64 = 11_644_473_600;
        let ticks = (utc + EPOCH_OFFSET).max(0) as u64 * 10_000_000;
        let file = win::FileTime { low: ticks as u32, high: (ticks >> 32) as u32 };
        let mut local = win::FileTime { low: 0, high: 0 };
        // Safety: both arguments are owned, correctly sized FILETIME structures.
        let ok = unsafe { win::FileTimeToLocalFileTime(&file, &mut local) };
        if ok != 0 {
            let ticks = ((local.high as u64) << 32) | local.low as u64;
            return (ticks / 10_000_000) as i64 - EPOCH_OFFSET;
        }
    }
    utc
}

/// Seconds since the epoch as a date and a time, by Howard Hinnant's civil-from-days.
fn civil_text(secs: i64) -> String {
    const MONTHS: [&str; 12] =
        ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let days = secs.div_euclid(86_400);
    let rest = secs.rem_euclid(86_400);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    format!("{d} {} {year}, {:02}:{:02}", MONTHS[(m - 1) as usize], rest / 3_600, (rest % 3_600) / 60)
}

#[cfg(windows)]
mod win {
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct FileTime {
        pub low: u32,
        pub high: u32,
    }

    unsafe extern "system" {
        pub fn FileTimeToLocalFileTime(from: *const FileTime, to: *mut FileTime) -> i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ticket #59: the civil conversion the Load list's "when it was saved" column reads.
    #[test]
    fn a_file_time_reads_as_a_date_and_a_time() {
        assert_eq!(civil_text(0), "1 January 1970, 00:00", "the epoch itself");
        // 2026-09-10 14:32:00 UTC is 1_789_050_720 seconds after the epoch.
        assert_eq!(civil_text(1_789_050_720), "10 September 2026, 14:32", "a real instant");
        // A leap day, which a naive month table gets wrong.
        assert_eq!(civil_text(1_709_208_000), "29 February 2024, 12:00", "the leap day");
    }
}

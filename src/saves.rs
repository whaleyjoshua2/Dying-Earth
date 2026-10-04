//! Ticket #59: where this machine keeps the game's saves, when a save was written, and how the
//! folder is opened. The engine knows how to write and read a save; it never knows where.

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

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

/// Ticket #466 (version 0.09.7): the time of day on this machine, twelve-hour, for the top bar's
/// clock: "9:47 PM". No seconds and no date, at the designer's word.
pub fn clock_text(t: SystemTime) -> String {
    clock_of(local_seconds(t))
}

fn clock_of(local: i64) -> String {
    let rest = local.rem_euclid(86_400);
    let (hour, minute) = (rest / 3_600, (rest % 3_600) / 60);
    format!("{}:{minute:02} {}", if hour % 12 == 0 { 12 } else { hour % 12 }, if hour < 12 { "AM" } else { "PM" })
}

/// Ticket #466: how long this sitting has run, for the clock's hover.
pub fn playing_text(d: Duration) -> String {
    let minutes = d.as_secs() / 60;
    match (minutes / 60, minutes % 60) {
        (0, 0) => "Playing for under a minute".to_string(),
        (0, m) => format!("Playing for {m} min"),
        (h, m) => format!("Playing for {h} h {m} min"),
    }
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
    // Ticket #466 (version 0.09.7): local time on Linux too, where this read UTC. The C library
    // breaks the instant down in this machine's zone and says how far that zone stands from UTC.
    #[cfg(unix)]
    {
        let mut out = std::mem::MaybeUninit::<nix::Tm>::zeroed();
        // Safety: `utc` is a valid time_t and `out` is a zeroed, correctly laid-out `struct tm`
        // that `localtime_r` fills; it is read only when the call says it succeeded.
        let done = unsafe { nix::localtime_r(&utc, out.as_mut_ptr()) };
        if !done.is_null() {
            return utc + unsafe { out.assume_init() }.gmtoff as i64;
        }
    }
    utc
}

/// The C library's `struct tm` as glibc and the BSDs lay it out on 64-bit machines, for the one
/// field this file reads: the zone's offset from UTC in seconds.
#[cfg(unix)]
mod nix {
    use std::os::raw::{c_char, c_int, c_long};

    #[repr(C)]
    pub struct Tm {
        pub sec: c_int,
        pub min: c_int,
        pub hour: c_int,
        pub mday: c_int,
        pub mon: c_int,
        pub year: c_int,
        pub wday: c_int,
        pub yday: c_int,
        pub isdst: c_int,
        pub gmtoff: c_long,
        pub zone: *const c_char,
    }

    unsafe extern "C" {
        pub fn localtime_r(time: *const i64, out: *mut Tm) -> *mut Tm;
    }
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

    /// Ticket #466 (version 0.09.7): the top bar's clock, twelve-hour, and how long a sitting has run.
    #[test]
    fn the_clock_reads_twelve_hours_and_the_sitting_reads_hours_and_minutes() {
        let at = |h: i64, m: i64| clock_of(1_789_050_720 / 86_400 * 86_400 + h * 3_600 + m * 60 + 59);
        assert_eq!(at(21, 47), "9:47 PM");
        assert_eq!(at(0, 5), "12:05 AM", "midnight is twelve");
        assert_eq!(at(12, 0), "12:00 PM", "and so is noon");
        assert_eq!(at(9, 3), "9:03 AM");
        assert_eq!(playing_text(Duration::from_secs(59)), "Playing for under a minute");
        assert_eq!(playing_text(Duration::from_secs(12 * 60 + 30)), "Playing for 12 min");
        assert_eq!(playing_text(Duration::from_secs(80 * 60)), "Playing for 1 h 20 min");
    }
}

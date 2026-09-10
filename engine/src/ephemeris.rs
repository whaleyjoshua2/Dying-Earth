//! Ticket #57 (version 0.05): the real sky, and where Earth and Mars stand in it.
//!
//! The game begins at 2030-01-01 00:00:00 UTC and a Turn is a calendar month, so every turn has a
//! real date. This module turns that date into the heliocentric ecliptic longitudes of Earth and
//! Mars, propagating JPL's approximate Keplerian elements (Standish, the 1800 AD to 2050 AD table)
//! and solving Kepler's equation. The game is played on a plane, so longitude is all it needs; the
//! inclination is carried anyway because the elements come with it and dropping it would be a
//! second, silent approximation.
//!
//! The elements live in `assets/data/ephemeris.toml`; the sources and the reference positions this
//! was checked against are in `docs/research/earth-mars-ephemeris.md`.

use crate::data::PlanetElements;

/// The Julian Day number of midnight UTC on a Gregorian calendar date (Fliegel and Van Flandern).
/// 2030-01-01 00:00 UTC is 2462502.5.
pub fn julian_day(year: i64, month: i64, day: i64) -> f64 {
    let (y, m) = if month <= 2 { (year - 1, month + 12) } else { (year, month) };
    let a = y.div_euclid(100);
    let b = 2 - a + a.div_euclid(4);
    (365.25 * (y + 4716) as f64).floor() + (30.6001 * (m + 1) as f64).floor() + day as f64 + b as f64 - 1524.5
}

/// Julian centuries of 36525 days from the J2000.0 epoch (JD 2451545.0).
pub fn centuries_from_j2000(jd: f64) -> f64 {
    (jd - 2451545.0) / 36525.0
}

/// An angle in degrees folded into 0 <= x < 360.
pub fn wrap_360(deg: f64) -> f64 {
    let x = deg % 360.0;
    if x < 0.0 { x + 360.0 } else { x }
}

/// An angle in degrees folded into -180 < x <= 180: the signed difference between two directions.
pub fn wrap_180(deg: f64) -> f64 {
    let x = wrap_360(deg);
    if x > 180.0 { x - 360.0 } else { x }
}

/// One planet's elements propagated to a date, and the position they put it at.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    /// Heliocentric ecliptic longitude, J2000 frame, degrees in 0..360.
    pub longitude: f64,
    /// Distance from the Sun, in astronomical units.
    pub radius: f64,
}

/// Where a planet stands at a date, from its elements (JPL's six-step recipe).
pub fn position(el: &PlanetElements, jd: f64) -> Position {
    let t = centuries_from_j2000(jd);
    // 1. The elements at the date.
    let a = el.a + el.a_rate * t;
    let e = el.e + el.e_rate * t;
    let inc = (el.inclination + el.inclination_rate * t).to_radians();
    let l = el.mean_longitude + el.mean_longitude_rate * t;
    let peri = el.perihelion_longitude + el.perihelion_longitude_rate * t;
    let node = el.node_longitude + el.node_longitude_rate * t;
    // 2. The argument of perihelion and the mean anomaly, the anomaly folded into -180..180.
    let arg = (peri - node).to_radians();
    let node = node.to_radians();
    let mean_anomaly = wrap_180(l - peri);
    // 3. Kepler's equation, worked in degrees as JPL states it: E - e* sin E = M, e* = e in degrees.
    let e_star = e.to_degrees();
    let mut ecc = mean_anomaly + e_star * mean_anomaly.to_radians().sin();
    for _ in 0..12 {
        let d = (mean_anomaly - (ecc - e_star * ecc.to_radians().sin())) / (1.0 - e * ecc.to_radians().cos());
        ecc += d;
        if d.abs() < 1e-9 {
            break;
        }
    }
    let ecc = ecc.to_radians();
    // 4. The position in the plane of the orbit.
    let x_orbit = a * (ecc.cos() - e);
    let y_orbit = a * (1.0 - e * e).max(0.0).sqrt() * ecc.sin();
    // 5. The J2000 ecliptic plane.
    let (sin_w, cos_w) = arg.sin_cos();
    let (sin_o, cos_o) = node.sin_cos();
    let (sin_i, cos_i) = inc.sin_cos();
    let x = (cos_w * cos_o - sin_w * sin_o * cos_i) * x_orbit + (-sin_w * cos_o - cos_w * sin_o * cos_i) * y_orbit;
    let y = (cos_w * sin_o + sin_w * cos_o * cos_i) * x_orbit + (-sin_w * sin_o + cos_w * cos_o * cos_i) * y_orbit;
    let z = (sin_w * sin_i) * x_orbit + (cos_w * sin_i) * y_orbit;
    // 6. Longitude is all a game on a plane needs.
    Position { longitude: wrap_360(y.atan2(x).to_degrees()), radius: (x * x + y * y + z * z).sqrt() }
}

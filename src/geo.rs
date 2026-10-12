//! Globe geometry: longitude and latitude to and from Bevy's UV sphere, ray casts, fixed marker positions.
//!
//! Bevy's UV sphere (spec 2.1 trap) has its poles on Z. Texture column u runs around from +X toward +Y,
//! and u = 0 is longitude -180. Every globe is rotated -90 degrees about X to stand upright.

use bevy::prelude::*;
use dying_earth_engine::data::{BodyCard, Tables};
use dying_earth_engine::{BodyId, StateId};
use std::f32::consts::{FRAC_PI_2, TAU};

/// Ticket #50: four seats cannot be a left side and a right side, so a Body's Ship stack markers
/// sit at four fixed angles round it by seat index: 0 left, 1 right, 2 above, 3 below. The Solar
/// System Map is the XZ plane seen from above, so "above" is -Z and "below" is +Z.
pub fn stack_offset(seat: dying_earth_engine::Seat, radius: f32) -> Vec3 {
    let d = radius * 1.4;
    let (x, z) = match seat.index() % 4 {
        0 => (-d, 0.0),
        1 => (d, 0.0),
        2 => (0.0, -d),
        _ => (0.0, d),
    };
    Vec3::new(x, radius + 0.25, z)
}

/// The rotation that stands a UV sphere upright (poles on Y).
pub fn upright() -> Quat {
    Quat::from_rotation_x(-FRAC_PI_2)
}

/// A unit vector in the sphere's own (unrotated) space for a longitude and latitude in degrees.
pub fn local_from_lonlat(lon: f32, lat: f32) -> Vec3 {
    let sector = (lon + 180.0) / 360.0 * TAU;
    let (slat, clat) = lat.to_radians().sin_cos();
    Vec3::new(clat * sector.cos(), clat * sector.sin(), slat)
}

/// Longitude and latitude in degrees for a point in the sphere's own space.
pub fn lonlat_from_local(p: Vec3) -> (f32, f32) {
    let p = p.normalize_or_zero();
    let lat = p.z.clamp(-1.0, 1.0).asin().to_degrees();
    let mut sector = p.y.atan2(p.x);
    if sector < 0.0 {
        sector += TAU;
    }
    let lon = sector / TAU * 360.0 - 180.0;
    (lon, lat)
}

/// The yaw that turns a longitude and latitude to face a camera on +Z, given the upright rotation.
pub fn yaw_facing(lon: f32, lat: f32) -> f32 {
    let v = upright() * local_from_lonlat(lon, lat);
    (-v.x).atan2(v.z)
}

/// Where a ray meets a sphere, if it does: the nearer hit distance along the ray.
pub fn ray_sphere(origin: Vec3, dir: Vec3, center: Vec3, radius: f32) -> Option<f32> {
    let oc = origin - center;
    let b = oc.dot(dir);
    let c = oc.length_squared() - radius * radius;
    let disc = b * b - c;
    if disc < 0.0 {
        return None;
    }
    let t = -b - disc.sqrt();
    if t >= 0.0 { Some(t) } else { None }
}

/// The pixel of an equirectangular image under a longitude and latitude.
pub fn pixel_for(lon: f32, lat: f32, w: u32, h: u32) -> (u32, u32) {
    let x = (((lon + 180.0) / 360.0) * w as f32).clamp(0.0, w as f32 - 1.0) as u32;
    let y = (((90.0 - lat) / 180.0) * h as f32).clamp(0.0, h as f32 - 1.0) as u32;
    (x, y)
}

/// Colony Slot positions, spread across each globe, on the near side for the Moon (section 20).
pub fn slot_lonlat(card: &BodyCard, slot: u32) -> (f32, f32) {
    // Ticket #45: every slot is a named place at its real position, from bodies.toml.
    let s = &card.slots[slot as usize % card.slots.len().max(1)];
    (s.lon, s.lat)
}

/// A point on each Nation State to hang its icons from.
pub fn state_lonlat(state: StateId) -> (f32, f32) {
    match state {
        StateId::SubSaharanAfrica => (20.0, -2.0),
        StateId::NorthAfrica => (14.0, 27.0),
        StateId::EastAsia => (110.0, 38.0),
        StateId::SouthAsia => (78.0, 22.0),
        StateId::SouthEastAsia => (108.0, 4.0),
        StateId::Australia => (135.0, -25.0),
        StateId::Europe => (12.0, 50.0),
        StateId::NorthAmerica => (-100.0, 47.0),
        StateId::CentralAmerica => (-89.0, 17.0),
        StateId::SouthAmerica => (-60.0, -15.0),
        StateId::Russia => (90.0, 62.0),
        // Ticket #125 (version 0.07.2): the Middle East lost the peninsula, so its label moves up
        // to the Iranian plateau; the two new Regions sit on Honshu and on the Nejd.
        StateId::MiddleEast => (58.0, 31.0),
        StateId::Japan => (138.0, 37.0),
        StateId::ArabianPeninsula => (45.0, 23.0),
        // Ticket #453 (version 0.09.6): on the Indus plain and on the Scottish Lowlands, clear of the Union's label.
        // Ticket #482 (version 0.09.8): Kazakhstan's on the steppe; Iran's own moves east to stand
        // between Iran and Pakistan, above; Turkey's on Anatolia; South Africa's on the Highveld.
        StateId::Kazakhstan => (67.0, 47.0),
        StateId::UnitedKingdom => (-3.0, 56.5),
        StateId::Turkey => (36.0, 38.0),
        StateId::SouthAfrica => (25.0, -27.0),
        // Ticket #510 (version 0.1.0.0): Vietnam's on the Khorat plateau, between the Mekong and the
        // Chao Phraya; Argentina's on the pampas, clear of Brazil's over the Mato Grosso.
        StateId::Vietnam => (103.0, 17.0),
        StateId::Argentina => (-65.0, -33.0),
    }
}

/// Where each Body sits on the Solar System Map: on its own ring, at its true heliocentric ecliptic
/// longitude for the turn (ticket #57). The rings are drawn to be legible rather than to scale, so
/// only the angle is real; the map is the XZ plane seen from the north, where longitude runs
/// anticlockwise. `lon` is the longitude of the Body itself, which for a satellite is its parent's.
/// Ticket #512 (version 0.1.0.0): with no tilt -- the title screen's system, before there is a sky
/// to read; a game's places go through `solar_place`, each ring in its own orbital plane.
pub fn solar_position(body: BodyId, turn: u32, lon: f32) -> Vec3 {
    solar_position_with(body, turn, lon, &|_| Quat::IDENTITY)
}

/// Ticket #512 (version 0.1.0.0): `solar_position` with each ring tilted by `tilt` of its Body --
/// a satellite rides its parent's tilted ring and circles it in the parent's plane.
pub fn solar_position_with(body: BodyId, turn: u32, lon: f32, tilt: &dyn Fn(BodyId) -> Quat) -> Vec3 {
    let t = turn as f32;
    let primary = body.primary();
    let on_ring = ring_point(primary, lon, tilt(primary));
    match body {
        BodyId::Moon => on_ring + tilt(BodyId::Earth) * Vec3::new((t * 0.9).cos(), 0.0, (t * 0.9).sin()),
        // Ticket #45: the moons of Mars, close in, drawn far larger than life to be clickable.
        BodyId::Phobos => on_ring + tilt(BodyId::Mars) * Vec3::new(0.7 * (t * 1.3).cos(), 0.0, 0.7 * (t * 1.3).sin()),
        BodyId::Deimos => on_ring + tilt(BodyId::Mars) * Vec3::new(1.05 * (t * 0.7 + 2.0).cos(), 0.0, 1.05 * (t * 0.7 + 2.0).sin()),
        _ => on_ring,
    }
}

/// Ticket #512: the point on a Body's ring at a longitude, the ring tilted into its orbital plane.
pub fn ring_point(body: BodyId, lon: f32, tilt: Quat) -> Vec3 {
    let a = -lon.to_radians();
    let r = solar_ring(body);
    tilt * Vec3::new(a.cos() * r, 0.0, a.sin() * r)
}

/// Ticket #512: the tilt of a Body's orbital plane on the map at a date: its inclination, turned
/// about its ascending node, both read from the ephemeris (a satellite reads its parent's). The
/// map draws longitude anticlockwise from the north with Z pointing away from longitude 90, so the
/// node's direction is (cos -node, 0, sin -node) and a right-handed turn about it by the
/// inclination lifts the half of the orbit past the node to the north, as the sky does.
pub fn orbital_tilt(tables: &Tables, body: BodyId, jd: f64) -> Quat {
    let (inc, node) = dying_earth_engine::ephemeris::orbital_plane(tables.planet(body), jd);
    let a = -(node as f32).to_radians();
    Quat::from_axis_angle(Vec3::new(a.cos(), 0.0, a.sin()), (inc as f32).to_radians())
}

/// Ticket #155 (version 0.07.4): the radius of a planet's ring on the Solar System Map, the one
/// table the Bodies' places and the drawn rings both read, so the two cannot drift apart. Venus's
/// ring was 2.45 and Earth's 3.4 until this version: seen from the camera's forty-degree tilt the
/// gap between them squashed to less than the two discs' radii together, and the two overlapped
/// at every conjunction, about four times a game. The designer: *"adjust/rescale solar system view
/// to keep Venus and earth from overlapping."* Venus is in and Earth out; Mars stays.
///
/// Ticket #502 (version 0.1.0.0): Mercury inside Venus, so the Sun is drawn smaller and Venus a
/// little further out, to keep the same clearance between neighbours at a conjunction; Vesta and
/// Ceres beyond Mars's moons (in a drawn belt until ticket #512 took it away).
pub fn solar_ring(body: BodyId) -> f32 {
    match body {
        BodyId::Mercury => 1.25,
        BodyId::Venus => 2.2,
        BodyId::Earth => 3.8,
        BodyId::Mars => 6.0,
        BodyId::Vesta => 7.3,
        BodyId::Ceres => 7.9,
        // The satellites have no ring of their own; they ride their planet's.
        BodyId::Moon => solar_ring(BodyId::Earth),
        BodyId::Phobos | BodyId::Deimos => solar_ring(BodyId::Mars),
    }
}

/// Ticket #502: the Sun's radius on the Solar System Map.
pub const SOLAR_SUN: f32 = 0.55;

/// Ticket #512 (version 0.1.0.0): the Solar System Map's camera. The eye stands `SOLAR_EYE_DISTANCE`
/// from its target (times the zoom), at a pitch above the ecliptic the player drags between
/// nearly edge-on and nearly straight down; it opens at the forty degrees it always had.
pub const SOLAR_EYE_DISTANCE: f32 = 18.82;
pub const SOLAR_PITCH_OPENING: f32 = 0.6914;
pub const SOLAR_PITCH_RANGE: (f32, f32) = (0.09, 1.48);

/// Ticket #480 (version 0.09.8): where a Body's far orbits stand on the Solar System Map -- on the
/// Body's own ring, a sixth of the way round from it: the first (L4) ahead of it, the second (L5)
/// behind. The Sun's pair, at the designer's word, so they stand well clear of the Body's disc.
pub fn solar_far_point(game: &dying_earth_engine::Game, body: BodyId, k: u32) -> Vec3 {
    let lead = if k.is_multiple_of(2) { 60.0 } else { -60.0 };
    let jd = game.julian_day(game.turn);
    solar_position_with(body, game.turn, game.heliocentric_longitude(body, game.turn) as f32 + lead, &|b| orbital_tilt(&game.tables, b, jd))
}

/// The same, reading the turn and the longitude off a game in play.
/// Ticket #512 (version 0.1.0.0): a game's Body on its ring, the ring in its orbital plane.
pub fn solar_place(game: &dying_earth_engine::Game, body: BodyId) -> Vec3 {
    let jd = game.julian_day(game.turn);
    solar_position_with(body, game.turn, game.heliocentric_longitude(body, game.turn) as f32, &|b| orbital_tilt(&game.tables, b, jd))
}

pub fn solar_radius(body: BodyId) -> f32 {
    match body {
        BodyId::Earth => 0.42,
        BodyId::Moon => 0.16,
        BodyId::Mars => 0.32,
        BodyId::Phobos => 0.1,
        BodyId::Deimos => 0.08,
        BodyId::Venus => 0.40,
        BodyId::Mercury => 0.16,
        BodyId::Ceres => 0.14,
        BodyId::Vesta => 0.12,
    }
}

#[cfg(test)]
mod solar_tests {
    use super::*;
    use dying_earth_engine::data::default_data_dir;
    use dying_earth_engine::{FactionKind, Game, NewGame};
    use std::sync::Arc;

    fn game() -> Game {
        let tables = Arc::new(Tables::load(&default_data_dir()).expect("tables"));
        Game::new(tables.clone(), NewGame { seed: 7, player: FactionKind::Custodians, player_is_ai: false, player_start: StateId::EastAsia })
    }

    /// Ticket #512 (version 0.1.0.0): **each Body rides its ring in its own orbital plane.** Over the
    /// game's turns a Body's height above the map is its ring's radius times the sine of its real
    /// ecliptic latitude, within the little its eccentric orbit adds; Earth's ring is the ecliptic
    /// and never leaves it; a satellite rides its parent's plane; and with no tilt the old flat
    /// place comes back, which is what the title screen draws.
    #[test]
    fn a_body_stands_at_its_real_latitude_above_its_tilted_ring() {
        let g = game();
        for body in [BodyId::Mercury, BodyId::Venus, BodyId::Mars, BodyId::Ceres, BodyId::Vesta] {
            let mut worst: f32 = 0.0;
            let mut highest: f32 = 0.0;
            for turn in 1..=g.tables.victory.turns {
                let jd = g.julian_day(turn);
                let lon = g.heliocentric_longitude(body, turn) as f32;
                let p = solar_position_with(body, turn, lon, &|b| orbital_tilt(&g.tables, b, jd));
                let want = solar_ring(body) * (g.heliocentric_latitude(body, turn) as f32).to_radians().sin();
                worst = worst.max((p.y - want).abs());
                highest = highest.max(p.y.abs());
                assert!((p.length() - solar_ring(body)).abs() < 1e-4, "{body:?} turn {turn}: on its ring");
            }
            assert!(worst < 0.05, "{body:?}: the ring's height against the sky's latitude, worst {worst}");
            assert!(highest > 0.1, "{body:?} does leave the ecliptic: highest {highest}");
        }
        // The ephemeris gives Earth a few thousandths of a degree, so the ring is level to a thousandth.
        assert!(solar_place(&g, BodyId::Earth).y.abs() < 1e-3, "Earth's ring is the ecliptic");
        let moon = solar_place(&g, BodyId::Moon) - solar_place(&g, BodyId::Earth);
        assert!(moon.y.abs() < 1e-3, "the Moon circles Earth in Earth's plane");
        for turn in 1..=g.tables.victory.turns {
            let (inc, _) = g.orbital_plane(BodyId::Earth, turn);
            assert!(inc.abs() < 0.01, "{inc}");
            let jd = g.julian_day(turn);
            let mars = orbital_tilt(&g.tables, BodyId::Mars, jd);
            let phobos = solar_position_with(BodyId::Phobos, turn, 0.0, &|b| orbital_tilt(&g.tables, b, jd)) - ring_point(BodyId::Mars, 0.0, mars);
            assert!((mars.inverse() * phobos).y.abs() < 1e-5, "Phobos circles Mars in Mars's plane");
        }
        assert_eq!(solar_position(BodyId::Mercury, 3, 40.0), ring_point(BodyId::Mercury, 40.0, Quat::IDENTITY), "no tilt: the flat place");
        assert_eq!(solar_position(BodyId::Mercury, 3, 40.0).y, 0.0);
    }

    /// Ticket #512: the planes the map tilts by are the real ones: Mercury 7.0 degrees about a node
    /// at 48; Ceres 10.6; Venus 3.4; Mars 1.85; Vesta 7.1.
    #[test]
    fn the_tilts_are_the_ephemeris_planes() {
        let g = game();
        let plane = |b: BodyId| g.orbital_plane(b, 1);
        assert!((plane(BodyId::Mercury).0 - 7.0).abs() < 0.05 && (plane(BodyId::Mercury).1 - 48.3).abs() < 0.5, "{:?}", plane(BodyId::Mercury));
        assert!((plane(BodyId::Venus).0 - 3.39).abs() < 0.05, "{:?}", plane(BodyId::Venus));
        assert!((plane(BodyId::Mars).0 - 1.85).abs() < 0.05, "{:?}", plane(BodyId::Mars));
        assert!((plane(BodyId::Ceres).0 - 10.6).abs() < 0.1, "{:?}", plane(BodyId::Ceres));
        assert!((plane(BodyId::Vesta).0 - 7.14).abs() < 0.1, "{:?}", plane(BodyId::Vesta));
        assert_eq!(plane(BodyId::Phobos), plane(BodyId::Mars), "a satellite reads its parent's");
    }
}

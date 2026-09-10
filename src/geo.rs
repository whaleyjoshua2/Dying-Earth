//! Globe geometry: longitude and latitude to and from Bevy's UV sphere, ray casts, fixed marker positions.
//!
//! Bevy's UV sphere (spec 2.1 trap) has its poles on Z. Texture column u runs around from +X toward +Y,
//! and u = 0 is longitude -180. Every globe is rotated -90 degrees about X to stand upright.

use bevy::prelude::*;
use dying_earth_engine::data::BodyCard;
use dying_earth_engine::{BodyId, StateId};
use std::f32::consts::{FRAC_PI_2, TAU};

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
        StateId::Africa => (20.0, 5.0),
        StateId::Asia => (100.0, 30.0),
        StateId::Australia => (135.0, -25.0),
        StateId::Europe => (12.0, 50.0),
        StateId::NorthAmerica => (-100.0, 45.0),
        StateId::SouthAmerica => (-60.0, -15.0),
        StateId::Russia => (90.0, 62.0),
        StateId::MiddleEast => (46.0, 28.0),
    }
}

/// Where each Body sits on the Solar System Map, drawn to be legible rather than to scale.
pub fn solar_position(body: BodyId, turn: u32) -> Vec3 {
    let t = turn as f32;
    match body {
        BodyId::Earth => {
            let a = 0.6 + t * 0.12;
            Vec3::new(a.cos() * 3.4, 0.0, a.sin() * 3.4)
        }
        BodyId::Moon => solar_position(BodyId::Earth, turn) + Vec3::new(1.0 * (t * 0.9).cos(), 0.0, 1.0 * (t * 0.9).sin()),
        BodyId::Mars => {
            let a = 2.4 + t * 0.07;
            Vec3::new(a.cos() * 6.0, 0.0, a.sin() * 6.0)
        }
        // Ticket #45: the moons of Mars, close in, drawn far larger than life to be clickable.
        BodyId::Phobos => solar_position(BodyId::Mars, turn) + Vec3::new(0.7 * (t * 1.3).cos(), 0.0, 0.7 * (t * 1.3).sin()),
        BodyId::Deimos => solar_position(BodyId::Mars, turn) + Vec3::new(1.05 * (t * 0.7 + 2.0).cos(), 0.0, 1.05 * (t * 0.7 + 2.0).sin()),
    }
}

pub fn solar_radius(body: BodyId) -> f32 {
    match body {
        BodyId::Earth => 0.42,
        BodyId::Moon => 0.16,
        BodyId::Mars => 0.32,
        BodyId::Phobos => 0.1,
        BodyId::Deimos => 0.08,
    }
}

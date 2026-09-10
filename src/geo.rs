//! Globe geometry: longitude and latitude to and from Bevy's UV sphere, ray casts, fixed marker positions.
//!
//! Bevy's UV sphere (spec 2.1 trap) has its poles on Z. Texture column u runs around from +X toward +Y,
//! and u = 0 is longitude -180. Every globe is rotated -90 degrees about X to stand upright.

use bevy::prelude::*;
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
pub fn slot_lonlat(body: BodyId, slot: u32) -> (f32, f32) {
    match body {
        BodyId::Moon => [(-42.0, 22.0), (28.0, 28.0), (-18.0, -24.0), (38.0, -14.0)][slot as usize % 4],
        BodyId::Mars => [(-125.0, 22.0), (-62.0, -24.0), (-5.0, 12.0), (58.0, -20.0), (118.0, 26.0), (168.0, -4.0)][slot as usize % 6],
        // Ticket #44: Antarctica. The Peninsula, the interior and Wilkes Land.
        BodyId::Earth => [(-62.0, -72.0), (15.0, -80.0), (115.0, -74.0)][slot as usize % 3],
    }
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
    }
}

pub fn solar_radius(body: BodyId) -> f32 {
    match body {
        BodyId::Earth => 0.42,
        BodyId::Moon => 0.16,
        BodyId::Mars => 0.32,
    }
}

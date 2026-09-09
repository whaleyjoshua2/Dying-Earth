//! The real-world textures (spec 2.4) and the Earth Map composition: Nation States tinted in their
//! controller's colour, outlined, hatched when occupied, the sea creeping up as thresholds fire.

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use dying_earth_engine::*;
use std::path::Path;

pub struct Rgba {
    pub w: u32,
    pub h: u32,
    pub data: Vec<u8>,
}

impl Rgba {
    fn load(path: &Path) -> Result<Rgba, String> {
        let img = image::open(path).map_err(|e| format!("{}: {e}", path.display()))?.to_rgba8();
        let (w, h) = img.dimensions();
        Ok(Rgba { w, h, data: img.into_raw() })
    }
    pub fn to_image(&self) -> Image {
        Image::new(
            Extent3d { width: self.w, height: self.h, depth_or_array_layers: 1 },
            TextureDimension::D2,
            self.data.clone(),
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD,
        )
    }
}

/// Every picture the game needs, read once at startup from `assets/textures`.
#[derive(Resource)]
pub struct Textures {
    pub earth: Rgba,
    pub moon: Rgba,
    pub mars: Rgba,
    /// 0 = water, 1..7 = Nation State index + 1 (see `examples/prep_assets.rs`).
    pub mask: Vec<u8>,
}

impl Textures {
    pub fn load(dir: &Path) -> Result<Textures, String> {
        let earth = Rgba::load(&dir.join("earth.png"))?;
        let moon = Rgba::load(&dir.join("moon.png"))?;
        let mars = Rgba::load(&dir.join("mars.png"))?;
        let mask_img = image::open(dir.join("earth_states.png")).map_err(|e| format!("earth_states.png: {e}"))?.to_luma8();
        if mask_img.dimensions() != (earth.w, earth.h) {
            return Err("earth_states.png must match earth.png in size".into());
        }
        Ok(Textures { earth, moon, mars, mask: mask_img.into_raw() })
    }

    pub fn state_at(&self, x: u32, y: u32) -> Option<StateId> {
        let v = self.mask[(y * self.earth.w + x) as usize];
        if v == 0 { None } else { StateId::ALL.get(v as usize - 1).copied() }
    }

    /// The Earth Map for the current board (spec 17.1, 11.4).
    pub fn compose_earth(&self, game: &Game, colours: &[[f32; 3]; 2]) -> Rgba {
        let (w, h) = (self.earth.w, self.earth.h);
        let mut out = self.earth.data.clone();
        let warm = ((game.climate.temperature - game.tables.climate.base_temperature) / 1.8).clamp(0.0, 1.0) as f32;
        let fired: Vec<u32> = game.states.iter().map(|s| s.thresholds_fired.iter().filter(|f| **f).count() as u32).collect();
        let tint_of = |seat: Seat| -> [f32; 3] { colours[seat.index()] };
        for y in 0..h {
            for x in 0..w {
                let i = (y * w + x) as usize;
                let v = self.mask[i];
                if v == 0 {
                    continue;
                }
                let sid = StateId::ALL[v as usize - 1];
                let st = game.state(sid);
                let p = i * 4;
                let mut r = out[p] as f32 / 255.0;
                let mut g = out[p + 1] as f32 / 255.0;
                let mut b = out[p + 2] as f32 / 255.0;
                // The sea takes the coast: each fired threshold drowns a band of pixels beside the water.
                let drowned = fired[sid.index()] > 0 && self.near_water(x, y, fired[sid.index()] * 2);
                if drowned {
                    out[p] = 18;
                    out[p + 1] = 40;
                    out[p + 2] = 90;
                    continue;
                }
                // The globe browns as it warms.
                if warm > 0.0 && sid != StateId::Antarctica {
                    let k = warm * 0.35;
                    r = r * (1.0 - k) + 0.55 * k;
                    g = g * (1.0 - k) + 0.42 * k;
                    b = b * (1.0 - k) + 0.25 * k;
                }
                let tint = match st.control {
                    Control::Neutral => None,
                    Control::Controlled(s) => Some(tint_of(s)),
                    Control::Occupied { occupier, previous, .. } => {
                        // Hatched: the occupier's colour on diagonal stripes, the old colour (or none) between.
                        if ((x + y) / 10) % 2 == 0 { Some(tint_of(occupier)) } else { previous.map(tint_of) }
                    }
                };
                if let Some(t) = tint {
                    let k = 0.45;
                    r = r * (1.0 - k) + t[0] * k;
                    g = g * (1.0 - k) + t[1] * k;
                    b = b * (1.0 - k) + t[2] * k;
                }
                // Outlines: a dark line between two states, a light line along a controlled coast.
                let border = self.border_kind(x, y);
                match border {
                    1 => {
                        r *= 0.25;
                        g *= 0.25;
                        b *= 0.25;
                    }
                    2 if tint.is_some() => {
                        r = r * 0.4 + 0.6;
                        g = g * 0.4 + 0.6;
                        b = b * 0.4 + 0.6;
                    }
                    _ => {}
                }
                out[p] = (r * 255.0) as u8;
                out[p + 1] = (g * 255.0) as u8;
                out[p + 2] = (b * 255.0) as u8;
            }
        }
        Rgba { w, h, data: out }
    }

    fn near_water(&self, x: u32, y: u32, d: u32) -> bool {
        let (w, h) = (self.earth.w as i64, self.earth.h as i64);
        let d = d as i64;
        for dy in -d..=d {
            for dx in -d..=d {
                let xx = (x as i64 + dx).rem_euclid(w);
                let yy = (y as i64 + dy).clamp(0, h - 1);
                if self.mask[(yy * w + xx) as usize] == 0 {
                    return true;
                }
            }
        }
        false
    }

    /// 0 none, 1 between two Nation States, 2 along a coast.
    fn border_kind(&self, x: u32, y: u32) -> u8 {
        let (w, h) = (self.earth.w as i64, self.earth.h as i64);
        let me = self.mask[(y as i64 * w + x as i64) as usize];
        let mut kind = 0;
        for (dx, dy) in [(1i64, 0i64), (-1, 0), (0, 1), (0, -1)] {
            let xx = (x as i64 + dx).rem_euclid(w);
            let yy = (y as i64 + dy).clamp(0, h - 1);
            let o = self.mask[(yy * w + xx) as usize];
            if o != me {
                if o == 0 {
                    kind = kind.max(2);
                } else {
                    return 1;
                }
            }
        }
        kind
    }
}

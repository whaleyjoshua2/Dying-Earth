//! Ticket #109 (version 0.07.0): the resource icons, rendered from SVG at runtime.
//!
//! The icons are game-icons.net's, under CC BY 3.0, which is why `credits()` exists and why the
//! title screen has a Credits button: the licence wants the authors named where a player can see
//! them. The art is fetched unmodified into `assets/icons/`, so its provenance stays plain, and
//! what this module does to it is done in memory.
//!
//! Rendering from SVG rather than baking PNGs at build time was the designer's call, so that any
//! future icon is a drop-in and every one comes out crisp at whatever size the interface asks for.

use bevy::prelude::Resource;
use bevy_egui::egui;
use std::collections::BTreeMap;
use std::path::Path;

/// Every game-icons.net SVG opens with a full-canvas black rectangle behind its white glyph. Drawn
/// as it stands that is a black square in the middle of the interface, so it comes out before the
/// tree is parsed; the glyph left behind is white and tints to whatever colour a label wants.
const BACKGROUND_RECT: &str = r#"<path d="M0 0h512v512H0z"/>"#;

/// The side, in pixels, each icon is rendered at. Generous enough that the top bar and the popups
/// can both draw it without resampling artefacts.
const RENDER_SIZE: u32 = 64;

/// One line of the credit the licence requires: the icon, its author, and what the game calls it.
pub struct Credit {
    pub resource: &'static str,
    pub icon: &'static str,
    pub author: &'static str,
}

/// The icons in use, their authors, and the names they carry at game-icons.net. Anything added to
/// `assets/icons/` belongs here too: the credit is the licence's price, not a courtesy.
pub const CREDITS: [Credit; 5] = [
    Credit { resource: "Materials", icon: "Ore", author: "Faithtoken" },
    Credit { resource: "Fuel", icon: "Jerrycan", author: "Delapouite" },
    Credit { resource: "Energy", icon: "Electric", author: "Sbed" },
    Credit { resource: "Research", icon: "Microscope", author: "Lord Berandas" },
    Credit { resource: "Ducats", icon: "Coins", author: "Delapouite" },
];

#[derive(Resource, Default)]
pub struct Icons {
    loaded: BTreeMap<String, egui::TextureHandle>,
    /// Tried and failed, so the attempt is not repeated every frame.
    tried: bool,
}

impl Icons {
    /// Read every SVG in `dir` and render it once. Missing or broken art is not fatal: the
    /// interface falls back to the words it drew before there were any icons.
    pub fn load(&mut self, ctx: &egui::Context, dir: &Path) {
        if self.tried {
            return;
        }
        self.tried = true;
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("svg") {
                continue;
            }
            let Some(name) = path.file_stem().and_then(|s| s.to_str()).map(|s| s.to_string()) else { continue };
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            if let Some(image) = render(&text) {
                let handle = ctx.load_texture(format!("icon:{name}"), image, egui::TextureOptions::LINEAR);
                self.loaded.insert(name, handle);
            }
        }
        // Ticket #106 (version 0.07.0): the icons are also put where any tooltip can reach them.
        // A hover is drawn deep inside a panel, and threading the set through thirty-seven call
        // sites to draw a glyph would be a worse cure than the disease.
        let loaded = self.loaded.clone();
        ctx.data_mut(|d| d.insert_temp(egui::Id::new("icons"), loaded));
    }

    /// Ticket #106: one icon, fetched from egui's own store rather than passed down. `None` where
    /// the art did not load, so every caller falls back to its words.
    pub fn from_ctx(ctx: &egui::Context, name: &str, size: f32, tint: egui::Color32) -> Option<egui::Image<'static>> {
        let map: BTreeMap<String, egui::TextureHandle> = ctx.data(|d| d.get_temp(egui::Id::new("icons")))?;
        let handle = map.get(name)?;
        Some(egui::Image::new(egui::load::SizedTexture::from_handle(handle)).fit_to_exact_size(egui::vec2(size, size)).tint(tint))
    }

    pub fn get(&self, name: &str) -> Option<&egui::TextureHandle> {
        self.loaded.get(name)
    }

    /// The icon at `size`, tinted, ready to put in a row beside a label. `None` where the art did
    /// not load, so every caller can fall back to its words.
    pub fn image(&self, name: &str, size: f32, tint: egui::Color32) -> Option<egui::Image<'static>> {
        let handle = self.get(name)?;
        Some(egui::Image::new(egui::load::SizedTexture::from_handle(handle)).fit_to_exact_size(egui::vec2(size, size)).tint(tint))
    }
}

/// One SVG to one egui image, with the black backing rectangle taken out first.
fn render(text: &str) -> Option<egui::ColorImage> {
    let cleaned = text.replace(BACKGROUND_RECT, "");
    let tree = resvg::usvg::Tree::from_str(&cleaned, &resvg::usvg::Options::default()).ok()?;
    let mut pixmap = resvg::tiny_skia::Pixmap::new(RENDER_SIZE, RENDER_SIZE)?;
    let size = tree.size();
    let scale = (RENDER_SIZE as f32 / size.width()).min(RENDER_SIZE as f32 / size.height());
    let transform = resvg::tiny_skia::Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    // tiny-skia hands back premultiplied RGBA, which is what egui's Color32 holds.
    let pixels = pixmap
        .pixels()
        .iter()
        .map(|p| egui::Color32::from_rgba_premultiplied(p.red(), p.green(), p.blue(), p.alpha()))
        .collect();
    let side = RENDER_SIZE as usize;
    Some(egui::ColorImage { size: [side, side], source_size: egui::vec2(RENDER_SIZE as f32, RENDER_SIZE as f32), pixels })
}

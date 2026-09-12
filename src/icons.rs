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
pub const CREDITS: [Credit; 8] = [
    Credit { resource: "Materials", icon: "Mine Wagon", author: "Delapouite" },
    Credit { resource: "Fuel", icon: "Jerrycan", author: "Delapouite" },
    Credit { resource: "Energy", icon: "Electric", author: "Sbed" },
    Credit { resource: "Research", icon: "Microscope", author: "Lord Berandas" },
    Credit { resource: "Ducats", icon: "Banknote", author: "Delapouite" },
    // Ticket #112 (version 0.07.1): three figures that were words on the board.
    Credit { resource: "Population", icon: "Character", author: "Delapouite" },
    Credit { resource: "Influence", icon: "Megaphone", author: "Delapouite" },
    Credit { resource: "Emissions", icon: "Chimney", author: "Delapouite" },
];

#[derive(Resource, Default)]
pub struct Icons {
    loaded: BTreeMap<String, egui::TextureHandle>,
    /// Tried and failed, so the attempt is not repeated every frame.
    tried: bool,
    /// Ticket #122 (version 0.07.2): the Nations' flags, by two-letter code. A flag is neither
    /// square nor tintable, so it is a second set with its own renderer and its own fetch.
    flags: BTreeMap<String, egui::TextureHandle>,
    tried_flags: bool,
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

    /// Ticket #122 (version 0.07.2): read every SVG in `dir` as a **flag** -- rendered at four by
    /// three, never tinted, keyed by its two-letter file stem -- and put the set where any card can
    /// reach it. The art is flag-icons (github.com/lipis/flag-icons) under the MIT licence, whose
    /// text ships beside the files; the research on ticket #123 measured that a tricolour survives
    /// sixteen pixels and an emblem does not, which is why the card draws these at thirty-two.
    pub fn load_flags(&mut self, ctx: &egui::Context, dir: &Path) {
        if self.tried_flags {
            return;
        }
        self.tried_flags = true;
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("svg") {
                continue;
            }
            let Some(code) = path.file_stem().and_then(|s| s.to_str()).map(|s| s.to_string()) else { continue };
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            if let Some(image) = render_flag(&text) {
                let handle = ctx.load_texture(format!("flag:{code}"), image, egui::TextureOptions::LINEAR);
                self.flags.insert(code, handle);
            }
        }
        let flags = self.flags.clone();
        ctx.data_mut(|d| d.insert_temp(egui::Id::new("flags"), flags));
    }

    /// A Nation's flag at `height`, four by three, in its own colours. `None` where there is no
    /// such flag, and the card then draws the name alone.
    pub fn flag_from_ctx(ctx: &egui::Context, code: &str, height: f32) -> Option<egui::Image<'static>> {
        if code.is_empty() {
            return None;
        }
        let map: BTreeMap<String, egui::TextureHandle> = ctx.data(|d| d.get_temp(egui::Id::new("flags")))?;
        let handle = map.get(code)?;
        Some(egui::Image::new(egui::load::SizedTexture::from_handle(handle)).fit_to_exact_size(egui::vec2(height * 4.0 / 3.0, height)))
    }

    /// Ticket #106: one icon, fetched from egui's own store rather than passed down. `None` where
    /// the art did not load, so every caller falls back to its words.
    pub fn from_ctx(ctx: &egui::Context, name: &str, size: f32) -> Option<egui::Image<'static>> {
        let map: BTreeMap<String, egui::TextureHandle> = ctx.data(|d| d.get_temp(egui::Id::new("icons")))?;
        let handle = map.get(name)?;
        Some(egui::Image::new(egui::load::SizedTexture::from_handle(handle)).fit_to_exact_size(egui::vec2(size, size)).tint(fill(name)))
    }

    /// Ticket #113 (version 0.07.1): the texture itself, for the Surface Maps. A map label is
    /// painted straight onto the globe rather than laid out by a `Ui`, so it cannot take an
    /// `egui::Image` and needs the id to hand to `Painter::image`.
    pub fn texture_from_ctx(ctx: &egui::Context, name: &str) -> Option<egui::TextureId> {
        let map: BTreeMap<String, egui::TextureHandle> = ctx.data(|d| d.get_temp(egui::Id::new("icons")))?;
        Some(map.get(name)?.id())
    }

    pub fn get(&self, name: &str) -> Option<&egui::TextureHandle> {
        self.loaded.get(name)
    }

    /// The icon at `size`, in its own fill, ready to put in a row beside a label. `None` where the
    /// art did not load, so every caller can fall back to its words.
    pub fn image(&self, name: &str, size: f32) -> Option<egui::Image<'static>> {
        let handle = self.get(name)?;
        Some(egui::Image::new(egui::load::SizedTexture::from_handle(handle)).fit_to_exact_size(egui::vec2(size, size)).tint(fill(name)))
    }
}

/// Ticket #112 (version 0.07.1): **a figure's glyph carries one fill, everywhere it is drawn.** The
/// colour is decided HERE, by which figure it is, and no caller can pass one -- the two
/// constructors above took a tint until the Blame block used it to paint the chimney in each
/// Faction's colour, which made an icon's colour mean "whose" on one screen and "which figure" on
/// every other. The designer's rule is that an icon's colour belongs to the icon, so the parameter
/// is gone and this function is the only place an answer exists.
///
/// Ticket #112 (version 0.07.1): the eight fills, decided off pictures of the real bar and a real
/// Facility list and then measured. The route is in the dev diary for 2026-09-12: candidate 1
/// "Natural" gave each figure the colour of the thing it names; the designer amended it -- Ducats
/// to a green, the jerrycan redder, Emissions browner -- and the measurement caught two collisions
/// the swatches hid.
///
/// **Ducats' green sat 12 from population's**, in CIELAB, which is inside the range two colours are
/// mistaken for each other, and the two stand in the same Facility list. Population moved to a warm
/// tan; a bust of a person wants a skin tone anyway, and the green stayed where it was asked for.
///
/// **The jerrycan shifted redder landed 19 from the Prospectors' orange** -- nearer than the amber
/// it started as, because amber is at hue 36 degrees and the Prospectors at 28, so a partial shift
/// red lands on top of them. It goes past them to hue 13 instead, which clears at 27.
///
/// The worst remaining pair on the whole board is Materials against Research at 25, which is
/// comfortable. Every colour here is a fill and nothing else: see `fill` below for why no caller
/// may override one.
const FIGURES: [(&str, [u8; 3]); 8] = [
    ("materials", [168, 176, 186]),
    ("fuel", [226, 88, 62]),
    ("energy", [245, 222, 92]),
    ("research", [118, 206, 232]),
    ("ducats", [120, 214, 150]),
    ("population", [220, 186, 150]),
    ("influence", [188, 146, 236]),
    ("emissions", [146, 110, 84]),
];

/// The fallback where a figure has no colour of its own, and what every figure answered before this
/// version gave them one.
const NEUTRAL: [u8; 3] = [225, 220, 210];

pub fn fill(name: &str) -> egui::Color32 {
    rgb(FIGURES.iter().find(|(figure, _)| *figure == name).map(|(_, c)| *c).unwrap_or(NEUTRAL))
}

fn rgb(c: [u8; 3]) -> egui::Color32 {
    egui::Color32::from_rgb(c[0], c[1], c[2])
}

/// One SVG to one egui image, with the black backing rectangle taken out first.
/// A flag: four by three, at a size the card's thirty-two pixels can be drawn from cleanly, and
/// with nothing stripped -- the black backing rectangle is a game-icons habit, not a flag's.
fn render_flag(text: &str) -> Option<egui::ColorImage> {
    const W: u32 = 128;
    const H: u32 = 96;
    let tree = resvg::usvg::Tree::from_str(text, &resvg::usvg::Options::default()).ok()?;
    let mut pixmap = resvg::tiny_skia::Pixmap::new(W, H)?;
    let size = tree.size();
    let transform = resvg::tiny_skia::Transform::from_scale(W as f32 / size.width(), H as f32 / size.height());
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    let pixels = pixmap.pixels().iter().map(|p| egui::Color32::from_rgba_premultiplied(p.red(), p.green(), p.blue(), p.alpha())).collect();
    Some(egui::ColorImage { size: [W as usize, H as usize], source_size: egui::vec2(W as f32, H as f32), pixels })
}

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

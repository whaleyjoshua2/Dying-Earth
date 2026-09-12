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
    pub fn from_ctx(ctx: &egui::Context, name: &str, size: f32) -> Option<egui::Image<'static>> {
        let map: BTreeMap<String, egui::TextureHandle> = ctx.data(|d| d.get_temp(egui::Id::new("icons")))?;
        let handle = map.get(name)?;
        Some(egui::Image::new(egui::load::SizedTexture::from_handle(handle)).fit_to_exact_size(egui::vec2(size, size)).tint(fill(name)))
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
/// Today every figure answers the same neutral off-white. The open half of ticket #112 is whether
/// each figure takes a colour of its own; `palette:<n>` (a building aid, not part of the spec)
/// swaps in a candidate so the choice can be made off a picture of the real bar rather than off
/// colour names in prose.
///
/// The eight rows of each candidate are in the order the top bar draws them.
const NEUTRAL: [u8; 3] = [225, 220, 210];

/// Candidate 1, **Natural**: each figure takes the colour of the thing it names. Amber Fuel, gold
/// Ducats, cyan Research, violet Influence. Two of these sit in neighbouring hues to a Faction --
/// Research beside the Custodians' teal, Influence beside the Arkwrights' purple -- which is the
/// collision the ticket's fourth option retires a Faction hue to end.
const NATURAL: [(&str, [u8; 3]); 8] = [
    ("materials", [168, 176, 186]),
    ("fuel", [232, 168, 72]),
    ("energy", [245, 222, 92]),
    ("research", [118, 206, 232]),
    ("ducats", [224, 186, 84]),
    ("population", [150, 206, 146]),
    ("influence", [188, 146, 236]),
    ("emissions", [216, 122, 104]),
];

/// Candidate 2, **Clear of the Factions**: every hue picked from the bands the four Factions leave
/// empty -- yellows, greens, pinks and reds -- so no icon sits near teal, orange, purple or pale
/// blue. The cost is that Fuel is no longer amber and Ducats no longer gold: the colours stop
/// naming the thing and start being a code to learn.
const CLEAR: [(&str, [u8; 3]); 8] = [
    ("materials", [176, 174, 168]),
    ("fuel", [236, 200, 80]),
    ("energy", [198, 230, 100]),
    ("research", [120, 214, 150]),
    ("ducats", [238, 160, 170]),
    ("population", [236, 224, 196]),
    ("influence", [222, 128, 220]),
    ("emissions", [226, 102, 86]),
];

/// Candidate 3, **Muted**: the natural hue of candidate 1 at a fraction of its saturation, so each
/// glyph reads as a tinted white rather than as a colour. Nothing competes with a Faction chip
/// because nothing is saturated enough to. Whether the hues survive at sixteen pixels at all is the
/// question this candidate exists to answer, and the picture is the only way to answer it.
const MUTED: [(&str, [u8; 3]); 8] = [
    ("materials", [200, 202, 208]),
    ("fuel", [230, 208, 172]),
    ("energy", [234, 230, 186]),
    ("research", [190, 214, 224]),
    ("ducats", [226, 214, 178]),
    ("population", [202, 218, 200]),
    ("influence", [214, 202, 226]),
    ("emissions", [222, 196, 190]),
];

/// Candidate 4, **the designer's**: candidate 1 with three changes asked for by name -- Ducats
/// takes the green candidate 2 gave Research, the jerrycan shifts redder, and Emissions goes
/// browner. Measured against the Faction colours it leaves two collisions, which candidate 5
/// answers; this one exists so the two can be looked at side by side.
const DESIGNERS: [(&str, [u8; 3]); 8] = [
    ("materials", [168, 176, 186]),
    ("fuel", [224, 112, 76]),
    ("energy", [245, 222, 92]),
    ("research", [118, 206, 232]),
    ("ducats", [120, 214, 150]),
    ("population", [150, 206, 146]),
    ("influence", [188, 146, 236]),
    ("emissions", [146, 110, 84]),
];

/// Candidate 5, **the designer's, with the two measured collisions cleared**. The green asked for
/// for Ducats sits at a CIELAB distance of 12 from the population green it was to stand beside,
/// which is inside the range two colours are mistaken for each other, so **population** moves to a
/// warm tan -- a skin tone, which is what a bust of a person wants anyway -- and the green stays
/// where it was asked for. The jerrycan shifted redder landed at a distance of 19 from the
/// Prospectors' orange, nearer than it began, so it goes **further** red rather than part way.
const DESIGNERS_CLEARED: [(&str, [u8; 3]); 8] = [
    ("materials", [168, 176, 186]),
    ("fuel", [226, 88, 62]),
    ("energy", [245, 222, 92]),
    ("research", [118, 206, 232]),
    ("ducats", [120, 214, 150]),
    ("population", [220, 186, 150]),
    ("influence", [188, 146, 236]),
    ("emissions", [146, 110, 84]),
];

pub fn fill(name: &str) -> egui::Color32 {
    let table = match std::env::args().find_map(|a| a.strip_prefix("palette:").and_then(|v| v.parse::<u32>().ok())) {
        Some(1) => &NATURAL,
        Some(2) => &CLEAR,
        Some(3) => &MUTED,
        Some(4) => &DESIGNERS,
        Some(5) => &DESIGNERS_CLEARED,
        _ => return rgb(NEUTRAL),
    };
    rgb(table.iter().find(|(figure, _)| *figure == name).map(|(_, c)| *c).unwrap_or(NEUTRAL))
}

fn rgb(c: [u8; 3]) -> egui::Color32 {
    egui::Color32::from_rgb(c[0], c[1], c[2])
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

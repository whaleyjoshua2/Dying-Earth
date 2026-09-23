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
pub const CREDITS: [Credit; 43] = [
    Credit { resource: "Materials", icon: "Mine Wagon", author: "Delapouite" },
    Credit { resource: "Fuel", icon: "Jerrycan", author: "Delapouite" },
    Credit { resource: "Energy", icon: "Electric", author: "Sbed" },
    Credit { resource: "Research", icon: "Microscope", author: "Lord Berandas" },
    Credit { resource: "Ducats", icon: "Banknote", author: "Delapouite" },
    // Ticket #168 (version 0.07.5): a symbol for each Faction, worn on its card on the Faction
    // screen and nowhere else. Both authors were already credited above, so the screen gains four
    // rows and no new name.
    Credit { resource: "Custodians", icon: "Ecology", author: "Delapouite" },
    Credit { resource: "Prospectors", icon: "Mining Helmet", author: "Delapouite" },
    Credit { resource: "Arkwrights", icon: "Moon Orbit", author: "Delapouite" },
    Credit { resource: "Archivists", icon: "CPU", author: "Delapouite" },
    // Ticket #112 (version 0.07.1): three figures that were words on the board.
    Credit { resource: "Population", icon: "Character", author: "Delapouite" },
    Credit { resource: "Influence", icon: "Megaphone", author: "Delapouite" },
    Credit { resource: "Emissions", icon: "Chimney", author: "Delapouite" },
    // Ticket #127 (version 0.07.2): the five kinds of thing, worn in front of a name. Chosen off a
    // sheet at fourteen and sixteen pixels, since that is where a roster row draws them.
    Credit { resource: "Warship", icon: "Spaceship", author: "Delapouite" },
    Credit { resource: "Colony Ship", icon: "Rocket", author: "Lorc" },
    // Ticket #135 (version 0.07.3): the station's glyph is the game's own drawing and owes no credit;
    // it is in DRAWN below. (Delapouite's Defense Satellite wore the row for one version.)
    Credit { resource: "Colony", icon: "Habitat Dome", author: "Delapouite" },
    Credit { resource: "Region", icon: "Modern City", author: "Delapouite" },
    // Ticket #145 (version 0.07.3): the Hab View's tiles, one picture per Module kind, the first
    // candidate of each on ticket #144's sheets (Lorc's Mining for the Mine, since Gold Mine collides
    // with the Materials' cart). The Habitat wears the Colony's dome above and needs no file. The
    // slot-boxes ticket may swap any of these so the two drawings share one set.
    Credit { resource: "Module Mine", icon: "Mining", author: "Lorc" },
    Credit { resource: "Module Generator", icon: "Power Generator", author: "Delapouite" },
    Credit { resource: "Module Refinery", icon: "Refinery", author: "Delapouite" },
    Credit { resource: "Module Shipyard", icon: "Cargo Crane", author: "Lorc" },
    Credit { resource: "Module Barracks", icon: "Barracks", author: "Delapouite" },
    Credit { resource: "Module Trade Post", icon: "Trade", author: "Lorc" },
    Credit { resource: "Module Relay", icon: "Radio Tower", author: "Delapouite" },
    Credit { resource: "Module Observatory", icon: "Observatory", author: "Delapouite" },
    Credit { resource: "Module Solar Array", icon: "Solar Power", author: "Skoll" },
    Credit { resource: "Module Mass Driver", icon: "Mass Driver", author: "Sbed" },
    Credit { resource: "Module Archive", icon: "Archive Register", author: "Delapouite" },
    // Ticket #239 (version 0.08.3): the three Unique Modules. Each was judged against its common
    // sibling at 16 and 22 pixels on a rendered sheet before it was taken, which is how the
    // Exchange lost its first pick -- Strongbox is already the Prospectors' Investment Bank, so
    // their two Uniques would have worn one picture -- and how the Heliostat lost Sun, whose
    // sibling the Solar Array already carries a sun and which on a climate board reads as heat.
    Credit { resource: "Module Heliostat", icon: "Radar Dish", author: "Lorc" },
    Credit { resource: "Module Exchange", icon: "Shop", author: "Delapouite" },
    Credit { resource: "Module Chorus", icon: "Satellite Communication", author: "Delapouite" },
    // Ticket #146 (version 0.07.3): the slot boxes' pictures on a Region's card, the first candidate
    // of each kind on ticket #144's sheets except where the research warned -- a control tower for
    // the Launch Site (the shuttle is the Colony Ship's rocket family), a handshake for the Embassy
    // (the capitol is the Bank's building twice), handcuffs for the Constabulary (the badge is the
    // Army's shield shape), a fan for the Scrubber (the gas mask says poison). The Refinery shares
    // the Module's file.
    Credit { resource: "Facility Factory", icon: "Factory", author: "Delapouite" },
    Credit { resource: "Facility Power Plant", icon: "Nuclear Plant", author: "Delapouite" },
    Credit { resource: "Facility Launch Site", icon: "Control Tower", author: "Delapouite" },
    Credit { resource: "Facility Research Lab", icon: "Round Bottom Flask", author: "Lorc" },
    Credit { resource: "Facility Bank", icon: "Bank", author: "Delapouite" },
    Credit { resource: "Facility Embassy", icon: "Shaking Hands", author: "Delapouite" },
    Credit { resource: "Facility Constabulary", icon: "Handcuffs", author: "Lorc" },
    Credit { resource: "Facility Sea Wall", icon: "Dam", author: "Delapouite" },
    Credit { resource: "Facility Scrubber", icon: "Computer Fan", author: "Delapouite" },
    // Tickets #181 to #186 (version 0.08.0): the four Unique Facilities. Every candidate was
    // rendered at 16 and 28 pixels on the game's own ground and looked at before any was adopted,
    // and two were killed by that: a TURBINE for the Reactor, which at 16 pixels is the Scrubber's
    // computer fan exactly, and a HARBOUR DOCK for the Spaceport, which carries an anchor and so
    // says sea. All three authors are already named above, so the Credits screen gains four rows
    // and no new name.
    Credit { resource: "Facility Investment Bank", icon: "Strongbox", author: "Delapouite" },
    Credit { resource: "Facility Spaceport", icon: "Space Shuttle", author: "Delapouite" },
    Credit { resource: "Facility Reactor", icon: "Nuclear", author: "Sbed" },
    // The Academy took two rounds: the telescope says astronomy, which is the Observatory's job, and
    // the open book is the Archive's. The designer picked the test tubes, knowing they are the
    // Research Lab's round-bottom flask's sibling -- the one pair on the board worth re-reading at
    // 16 pixels whenever the icon sheet is next photographed.
    Credit { resource: "Facility Academy", icon: "Test Tubes", author: "Lorc" },
];

/// Ticket #146 (version 0.07.3): the icon key a Facility's slot box wears on a Region's card.
pub fn facility_icon(kind: dying_earth_engine::FacilityKind) -> &'static str {
    use dying_earth_engine::FacilityKind::*;
    match kind {
        Factory => "facility_factory",
        PowerPlant => "facility_power_plant",
        Refinery => "module_refinery",
        ResearchLab => "facility_research_lab",
        LaunchSite => "facility_launch_site",
        Bank => "facility_bank",
        Embassy => "facility_embassy",
        Constabulary => "facility_constabulary",
        SeaWall => "facility_sea_wall",
        Scrubber => "facility_scrubber",
        // Ticket #185 (version 0.08.0): the School.
        School => "facility_school",
        // Tickets #181 to #186 (version 0.08.0): the four Unique Facilities, each its own picture
        // rather than a corner mark on the common one -- at build-slot-box size, nine near-identical
        // grey boxes with a small Faction mark is not a thing a player reads.
        InvestmentBank => "facility_investment_bank",
        Spaceport => "facility_spaceport",
        Reactor => "facility_reactor",
        Academy => "facility_academy",
    }
}

/// Ticket #145 (version 0.07.3): the icon key a Module kind's tile wears in the Hab View. The
/// Habitat wears the Colony's own dome, at the designer's word; every other kind has a file of its own.
pub fn module_icon(kind: dying_earth_engine::ModuleKind) -> &'static str {
    use dying_earth_engine::ModuleKind::*;
    match kind {
        Mine => "module_mine",
        Generator => "module_generator",
        Refinery => "module_refinery",
        Habitat => "colony",
        Shipyard => "module_shipyard",
        Barracks => "module_barracks",
        TradePost => "module_trade_post",
        Relay => "module_relay",
        Observatory => "module_observatory",
        SolarArray => "module_solar_array",
        MassDriver => "module_mass_driver",
        // Ticket #185 (version 0.08.0): the Institute is the School off Earth and wears its
        // mortarboard, as the Refinery Facility wears the Refinery Module's own file.
        Institute => "facility_school",
        // Ticket #186 (version 0.08.0): the Custodians' Unique Module wears their Unique Facility's
        // picture, as the Institute wears the School's.
        Academy => "facility_academy",
        // Ticket #239 (version 0.08.3): the two Unique Modules whose Faction's Unique FACILITY is
        // no help -- a Reactor is not a solar mirror and an Investment Bank is not a shop -- so
        // each takes a picture of its own from the candidate set, chosen to be unmistakable from
        // its common sibling at build-tile size rather than a variation on it.
        Heliostat => "module_heliostat",
        Exchange => "module_exchange",
        Chorus => "module_chorus",
        Archive => "module_archive",
        // Ticket #324 (version 0.08.8): the Battery's glyph is the game's own drawing, in DRAWN.
        Battery => "module_battery",
        // Ticket #164 (version 0.07.5): the Core Module wears the station glyph the game drew for
        // itself on ticket #135 -- two solar panels on a bar with a module between them, which is
        // what a core module is. It owes no credit. A ground Colony's Core Module wears it too,
        // for want of a drawing of its own; the designer may want one.
        Core => "station",
    }
}

/// Ticket #168 (version 0.07.5): the icon key a Faction's symbol wears on its card. Nowhere else
/// draws it: the roster, the map labels, the Report and the Victory screen keep their Faction
/// colours and names, at the designer's word -- *"for now these symbols should only appear on the
/// faction selection screen."*
pub fn faction_symbol(kind: dying_earth_engine::FactionKind) -> &'static str {
    use dying_earth_engine::FactionKind::*;
    match kind {
        Custodians => "custodians",
        Prospectors => "prospectors",
        Arkwrights => "arkwrights",
        Archivists => "archivists",
    }
}

/// Ticket #135 (version 0.07.3): the SVGs in `assets/icons/` that are the game's own drawings, so
/// they owe nobody a credit and the Credits screen does not list them. The station's glyph -- two
/// solar panels on a bar with a central module, the ISS reduced to its silhouette -- was drawn for
/// the game after the designer passed on every station game-icons.net has, none of which is drawn
/// as a station. A drawn glyph still loads and tints through `Icons` like any other.
/// Ticket #185 (version 0.08.0): the School's is drawn for the game too -- a mortarboard: the flat
/// board, the cap beneath it and a tassel hanging right. game-icons.net has no school, and the
/// nearest candidates said other things (a laboratory, a capitol).
///
/// It was drawn as a pediment on columns first, and the icon sheet killed that at a glance: at 16
/// pixels it was the BANK's silhouette exactly, distinguished only by the currency glyph the Bank
/// carries. This is the collision ticket #146 already dodged once when it gave the Embassy a
/// handshake, "since the capitol is the Bank's building twice". The open book was unavailable too:
/// the Archive has it.
/// Ticket #317 (version 0.08.8): the Battle mark, two crossed blades, drawn by hand as the station
/// was, so it owes no credit. The board had no glyph for a fight; the mark is off-white like every
/// kind glyph and sits on a disc in the aggressor's colour, since on this board a colour says whose.
/// Ticket #324 (version 0.08.8): the Battery's glyph, a turret on a mount with its barrel raised,
/// drawn by hand as the Battle mark was, so it owes no credit.
pub const DRAWN: [&str; 4] = ["station", "facility_school", "battle", "module_battery"];

impl Credit {
    /// The file stem in `assets/icons/` this credit is for: the name, lower-cased, spaces to
    /// underscores ("Colony Ship" is `colony_ship.svg`).
    pub fn key(&self) -> String {
        self.resource.to_lowercase().replace(' ', "_")
    }
}

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
///
/// Ticket #127 (version 0.07.2): a ninth entry, **kind**, is the one fill every kind glyph wears --
/// "keep these off white," the designer said of the five -- named here rather than left to fall
/// through to NEUTRAL, so that retuning the fallback for some later glyph cannot move them.
const FIGURES: [(&str, [u8; 3]); 9] = [
    ("materials", [168, 176, 186]),
    ("fuel", [226, 88, 62]),
    ("energy", [245, 222, 92]),
    ("research", [118, 206, 232]),
    ("ducats", [120, 214, 150]),
    ("population", [220, 186, 150]),
    ("influence", [188, 146, 236]),
    ("emissions", [146, 110, 84]),
    ("kind", [236, 232, 224]),
];

/// Ticket #127 (version 0.07.2): the glyphs that say WHAT a thing is -- a warship, a Colony Ship, a
/// station, a Colony, a Region -- as against the figures above, which say how much of something.
/// On this board a colour means whose, so these carry the one off-white fill and never a colour
/// of their own. The Army's shield is drawn, not loaded, and takes the same fill.
pub const KINDS: [&str; 6] = ["warship", "colony_ship", "station", "colony", "region", "battle"];

/// The fill every kind glyph wears, for the shapes that are drawn rather than loaded.
pub fn kind_fill() -> egui::Color32 {
    fill("kind")
}

/// The fallback where a figure has no colour of its own, and what every figure answered before this
/// version gave them one.
const NEUTRAL: [u8; 3] = [225, 220, 210];

pub fn fill(name: &str) -> egui::Color32 {
    let name = if KINDS.contains(&name) { "kind" } else { name };
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Ticket #127 (version 0.07.2): every SVG in `assets/icons/` has its credit and every credit
    /// names an SVG that is there. The credit is the licence's price, so it is checked rather than
    /// remembered; and a kind glyph with no file would leave a row wearing nothing without a word.
    #[test]
    fn every_icon_is_credited_and_every_credit_has_its_icon() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/icons");
        let mut on_disk: Vec<String> = std::fs::read_dir(&dir)
            .expect("assets/icons")
            .flatten()
            .filter_map(|e| {
                let p = e.path();
                if p.extension().and_then(|x| x.to_str()) != Some("svg") {
                    return None;
                }
                p.file_stem().and_then(|s| s.to_str()).map(str::to_owned)
            })
            .collect();
        on_disk.sort();
        let mut credited: Vec<String> = CREDITS.iter().map(Credit::key).collect();
        // Ticket #135: a drawn glyph is the game's own and owes no credit, but it must still exist.
        credited.extend(DRAWN.iter().map(|s| s.to_string()));
        credited.sort();
        assert_eq!(on_disk, credited, "left: the SVGs in assets/icons; right: the keys CREDITS names plus DRAWN");
        for kind in KINDS {
            assert!(credited.iter().any(|k| k == kind), "kind glyph {kind} has neither a credit nor a drawing");
        }
        for d in DRAWN {
            assert!(!CREDITS.iter().any(|c| c.key() == d), "{d} is drawn and credited both");
        }
    }
}

//! Candidate sheets for the four Factions' symbols (ticket #167, version 0.07.5), rendered with
//! the same `resvg` the game uses. Copy to `examples/faction_sheet.rs` to run:
//!
//! `cargo run --release --example faction_sheet -- <manifest> <svg-root> <worn-dir> <out-dir>`
//!
//! It is ticket #144's `buildsheet.rs` with three changes, and nothing else: the sizes are a
//! Faction card's (96, 64, 40, 28, where a slot box wanted 64, 48, 32, 28), a candidate takes a
//! ROW rather than a column so that twelve of them make a tall narrow sheet a person can read at
//! once, and the tint comes off the manifest line so each Faction is drawn in its own colour and
//! no other. The manifest is one Faction per line,
//! `faction|r,g,b|name name name ...`, and the SVGs are read from `<svg-root>/<faction>/`.
//!
//! Each row is one candidate at 96, 64, 40 and 28 pixels as the game would draw it (the
//! game-icons backing rectangle stripped, the glyph over the dark panel), then the 28 blown up
//! three times without smoothing, since that is the size a Faction card's title row has and the
//! one where a glyph dissolves. Two sheets per Faction: untinted (the file's own white) and the
//! Faction's colour, applied the way egui's `Image::tint` multiplies. It never starts the game.
use image::{imageops, Rgba, RgbaImage};
use resvg::{tiny_skia, usvg};

const BACKGROUND_RECT: &str = r#"<path d="M0 0h512v512H0z"/>"#;
const BG: Rgba<u8> = Rgba([26, 26, 32, 255]);
/// A Faction card's sizes: 96 and 64 if the symbol stands large in the card, 40 and 28 if it sits
/// in the title row beside the 24-point name, and 24 because that is the exact side of the colour
/// swatch `faction_card` draws there today, so it is the size a symbol put in the swatch's place
/// would have. The 28 and the 24 are the ones that decide, and both are magnified.
const SIZES: [u32; 5] = [96, 64, 40, 28, 24];
/// The two sizes blown up, in the order they are drawn after the native row.
const MAGNIFIED: [u32; 2] = [28, 24];
const ZOOM: u32 = 3;
const GAP: u32 = 6;
const ROW_H: u32 = 96 + 2 * GAP;
const NUM_W: u32 = 30;

fn load(path: &str) -> Option<usvg::Tree> {
    let text = std::fs::read_to_string(path).ok()?;
    usvg::Tree::from_str(&text.replace(BACKGROUND_RECT, ""), &usvg::Options::default()).ok()
}

/// Rasterise into an `s`-pixel square, fit by the longer side, and tint by multiplying (egui's
/// `tint` is a per-channel multiply of the texture by the colour).
fn raster(tree: &usvg::Tree, s: u32, tint: [u8; 3]) -> RgbaImage {
    let size = tree.size();
    let scale = (s as f32 / size.width()).min(s as f32 / size.height());
    let mut pm = tiny_skia::Pixmap::new(s, s).unwrap();
    resvg::render(tree, tiny_skia::Transform::from_scale(scale, scale), &mut pm.as_mut());
    let mut img = RgbaImage::new(s, s);
    for (i, p) in pm.pixels().iter().enumerate() {
        let c = p.demultiply();
        let m = |v: u8, t: u8| ((v as u32 * t as u32) / 255) as u8;
        img.put_pixel(i as u32 % s, i as u32 / s, Rgba([m(c.red(), tint[0]), m(c.green(), tint[1]), m(c.blue(), tint[2]), c.alpha()]));
    }
    img
}

/// A 3x5 digit, so a row can be numbered against the table in the findings.
const DIGITS: [[u8; 5]; 10] = [
    [0b111, 0b101, 0b101, 0b101, 0b111],
    [0b010, 0b110, 0b010, 0b010, 0b111],
    [0b111, 0b001, 0b111, 0b100, 0b111],
    [0b111, 0b001, 0b111, 0b001, 0b111],
    [0b101, 0b101, 0b111, 0b001, 0b001],
    [0b111, 0b100, 0b111, 0b001, 0b111],
    [0b111, 0b100, 0b111, 0b101, 0b111],
    [0b111, 0b001, 0b001, 0b001, 0b001],
    [0b111, 0b101, 0b111, 0b101, 0b111],
    [0b111, 0b101, 0b111, 0b001, 0b111],
];

fn number(sheet: &mut RgbaImage, n: usize, x: u32, y: u32) {
    let k = 3;
    for (i, ch) in n.to_string().chars().enumerate() {
        let d = DIGITS[ch.to_digit(10).unwrap() as usize];
        for (r, row) in d.iter().enumerate() {
            for c in 0..3 {
                if row & (0b100 >> c) != 0 {
                    for dx in 0..k {
                        for dy in 0..k {
                            sheet.put_pixel(x + (i as u32 * 4 + c) * k + dx, y + r as u32 * k + dy, Rgba([170, 170, 180, 255]));
                        }
                    }
                }
            }
        }
    }
}

/// Width of one candidate's cell: the five native sizes, then the magnified 28 and 24.
fn cell_w() -> u32 {
    SIZES.iter().sum::<u32>() + GAP * (SIZES.len() as u32 + 1) + MAGNIFIED.iter().map(|m| m * ZOOM + GAP).sum::<u32>()
}

fn draw_candidate(sheet: &mut RgbaImage, tree: &usvg::Tree, x: u32, y: u32, tint: [u8; 3]) {
    let mut cx = x;
    for s in SIZES {
        let img = raster(tree, s, tint);
        imageops::overlay(sheet, &img, cx as i64, (y + (ROW_H - 2 * GAP - s) / 2 + GAP) as i64);
        cx += s + GAP;
    }
    for m in MAGNIFIED {
        let small = raster(tree, m, tint);
        let big = imageops::resize(&small, m * ZOOM, m * ZOOM, imageops::FilterType::Nearest);
        imageops::overlay(sheet, &big, cx as i64, (y + (ROW_H - m * ZOOM) / 2) as i64);
        cx += m * ZOOM + GAP;
    }
}

/// `rows` is one row per line of the sheet, each a list of file stems to draw across it. A
/// Faction's own sheet has one candidate to a row; the control sheet of what the board already
/// wears has four, since it is there to be scanned rather than judged.
fn sheet(rows: &[Vec<String>], dir: &str, tint: [u8; 3], across: usize) -> RgbaImage {
    let w = NUM_W + across as u32 * (cell_w() + GAP * 2) + GAP;
    let h = rows.len() as u32 * ROW_H + GAP;
    let mut s = RgbaImage::from_pixel(w, h, BG);
    for (r, cands) in rows.iter().enumerate() {
        let y = GAP + r as u32 * ROW_H;
        number(&mut s, r + 1, 6, y + ROW_H / 2 - 8);
        // A faint rule under each row so the eye keeps its place.
        for x in 0..w {
            s.put_pixel(x, y + ROW_H - GAP / 2, Rgba([40, 40, 48, 255]));
        }
        for (k, name) in cands.iter().enumerate() {
            let Some(tree) = load(&format!("{dir}/{name}.svg")) else {
                eprintln!("missing or unparsable: {dir}/{name}.svg");
                continue;
            };
            draw_candidate(&mut s, &tree, NUM_W + k as u32 * (cell_w() + GAP * 2), y, tint);
        }
    }
    s
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (manifest, root, worn, out) = (&args[1], &args[2], &args[3], &args[4]);
    std::fs::create_dir_all(out).unwrap();
    for line in std::fs::read_to_string(manifest).unwrap().lines().filter(|l| !l.trim().is_empty()) {
        let mut parts = line.split('|');
        let faction = parts.next().unwrap().trim().to_string();
        let tint: Vec<u8> = parts.next().unwrap().trim().split(',').map(|x| x.parse().unwrap()).collect();
        let names: Vec<Vec<String>> = parts.next().unwrap().split_whitespace().map(|n| vec![n.to_string()]).collect();
        let dir = format!("{root}/{faction}");
        for (label, t) in [("untinted", [255u8, 255, 255]), ("tinted", [tint[0], tint[1], tint[2]])] {
            let s = sheet(&names, &dir, t, 1);
            let path = format!("{out}/sheet-{faction}-{label}.png");
            s.save(&path).unwrap();
            println!("wrote {path}: {}x{} ({} rows)", s.width(), s.height(), names.len());
        }
    }
    // The glyphs the board already wears, four to a row, for the collision check: a candidate that
    // looks like one of these at 28 pixels is a collision, whatever it looks like at 96.
    let mut worn_names: Vec<String> = std::fs::read_dir(worn)
        .unwrap()
        .flatten()
        .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("svg"))
        .filter_map(|e| e.path().file_stem().and_then(|s| s.to_str()).map(str::to_string))
        .collect();
    worn_names.sort();
    let worn_rows: Vec<Vec<String>> = worn_names.chunks(4).map(<[String]>::to_vec).collect();
    let s = sheet(&worn_rows, worn, [236, 232, 224], 4);
    let path = format!("{out}/sheet-worn-offwhite.png");
    s.save(&path).unwrap();
    println!("wrote {path}: {}x{} ({})", s.width(), s.height(), worn_names.join(", "));
}

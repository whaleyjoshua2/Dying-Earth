//! Candidate sheets for the twenty-two building kinds (ticket #144, version 0.07.3), rendered with
//! the same `resvg` the game uses. Copy to `examples/building_sheet.rs` to run:
//!
//! `cargo run --release --example building_sheet -- <manifest> <svg-dir> <worn-dir> <out-dir> [r,g,b ...]`
//!
//! The manifest is one kind per line, `Kind|author/name author/name ...`. Each candidate is drawn
//! at 64, 48, 32 and 28 pixels as the game would draw it (backing rectangle stripped, glyph over
//! the dark panel), then the 28 blown up three times without smoothing, since that is the size a
//! slot box has and the one where a glyph dissolves. One sheet per tint: untinted (the file's own
//! white), then each `r,g,b` given, applied the way egui's `Image::tint` multiplies.
use image::{imageops, Rgba, RgbaImage};
use resvg::{tiny_skia, usvg};

const BACKGROUND_RECT: &str = r#"<path d="M0 0h512v512H0z"/>"#;
const BG: Rgba<u8> = Rgba([26, 26, 32, 255]);
const SIZES: [u32; 4] = [64, 48, 32, 28];
const ZOOM: u32 = 3;
const GAP: u32 = 6;
const ROW_H: u32 = 28 * ZOOM + 2 * GAP; // 96
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

/// Width of one candidate's cell: the four native sizes, then the magnified 28.
fn cell_w() -> u32 {
    SIZES.iter().sum::<u32>() + GAP * (SIZES.len() as u32 + 1) + 28 * ZOOM + GAP
}

fn draw_candidate(sheet: &mut RgbaImage, tree: &usvg::Tree, x: u32, y: u32, tint: [u8; 3]) {
    let mut cx = x;
    for s in SIZES {
        let img = raster(tree, s, tint);
        imageops::overlay(sheet, &img, cx as i64, (y + (ROW_H - GAP - s) / 2) as i64);
        cx += s + GAP;
    }
    let small = raster(tree, 28, tint);
    let big = imageops::resize(&small, 28 * ZOOM, 28 * ZOOM, imageops::FilterType::Nearest);
    imageops::overlay(sheet, &big, cx as i64, (y + GAP) as i64);
}

fn sheet(rows: &[(String, Vec<String>)], dir: &str, tint: [u8; 3], max_cands: usize) -> RgbaImage {
    let w = NUM_W + max_cands as u32 * (cell_w() + GAP * 2) + GAP;
    let h = rows.len() as u32 * ROW_H + GAP;
    let mut s = RgbaImage::from_pixel(w, h, BG);
    for (r, (_, cands)) in rows.iter().enumerate() {
        let y = GAP + r as u32 * ROW_H;
        number(&mut s, r + 1, 6, y + ROW_H / 2 - 8);
        // A faint rule under each row so the eye keeps its place.
        for x in 0..w {
            s.put_pixel(x, y + ROW_H - GAP / 2, Rgba([40, 40, 48, 255]));
        }
        for (k, cand) in cands.iter().enumerate() {
            let name = cand.rsplit('/').next().unwrap();
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
    let (manifest, dir, worn, out) = (&args[1], &args[2], &args[3], &args[4]);
    let tints: Vec<[u8; 3]> = args[5..]
        .iter()
        .map(|t| {
            let v: Vec<u8> = t.split(',').map(|x| x.parse().unwrap()).collect();
            [v[0], v[1], v[2]]
        })
        .collect();
    std::fs::create_dir_all(out).unwrap();
    let rows: Vec<(String, Vec<String>)> = std::fs::read_to_string(manifest)
        .unwrap()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let (kind, cands) = l.split_once('|').unwrap();
            (kind.to_string(), cands.split_whitespace().map(str::to_string).collect())
        })
        .collect();
    let max_cands = rows.iter().map(|(_, c)| c.len()).max().unwrap();
    // The ten Facilities are the first ten rows of the manifest, the Modules the rest.
    let (facilities, modules) = rows.split_at(10);
    // The glyphs already worn on the board, for the control: a candidate that looks like one of
    // these at 28 pixels is a collision, whatever it looks like at 64.
    let mut worn_names: Vec<String> = std::fs::read_dir(worn)
        .unwrap()
        .flatten()
        .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("svg"))
        .filter_map(|e| e.path().file_stem().and_then(|s| s.to_str()).map(str::to_string))
        .collect();
    worn_names.sort();
    let worn_rows: Vec<(String, Vec<String>)> = worn_names.chunks(3).map(|c| ("worn".to_string(), c.to_vec())).collect();
    let mut variants = vec![("untinted".to_string(), [255u8, 255, 255])];
    for (i, t) in tints.iter().enumerate() {
        variants.push((if i == 0 { "offwhite".to_string() } else { format!("tint{i}") }, *t));
    }
    for (label, tint) in &variants {
        for (group, rs) in [("facilities", facilities), ("modules", modules)] {
            let s = sheet(rs, dir, *tint, max_cands);
            let path = format!("{out}/sheet-{group}-{label}.png");
            s.save(&path).unwrap();
            println!("wrote {path}: {}x{}", s.width(), s.height());
        }
    }
    let s = sheet(&worn_rows, worn, [255, 255, 255], 3);
    let path = format!("{out}/sheet-worn-untinted.png");
    s.save(&path).unwrap();
    println!("wrote {path}: {}x{} ({})", s.width(), s.height(), worn_names.join(", "));
}

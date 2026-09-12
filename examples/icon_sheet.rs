//! Render every SVG in `assets/icons/` onto one labelled sheet, so an icon can be LOOKED at at a
//! size a person can judge rather than guessed at from its filename.
//!
//! `cargo run --release --example icon_sheet -- out.png`
//!
//! Version 0.07.1 adds icons for population, Influence and Emissions and reconsiders two of the
//! five that exist, and "does this glyph read as the thing it names" is a question no test can
//! answer. This renders them the way the game does — the full-canvas black backing rectangle every
//! game-icons.net SVG carries is stripped first, and the white glyph is drawn on the game's own
//! dark ground — so the sheet shows what a player sees, only bigger.

use std::path::Path;

const CELL: u32 = 160;
const PAD: u32 = 14;
const LABEL: u32 = 22;
const BACKGROUND_RECT: &str = r#"<path d="M0 0h512v512H0z"/>"#;

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "icon-sheet.png".to_string());
    // A second argument points the sheet at any folder of SVGs, so candidates can be judged before
    // they are adopted.
    let from = std::env::args().nth(2).unwrap_or_else(|| "assets/icons".to_string());
    let dir = Path::new(&from);
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .expect("assets/icons")
        .flatten()
        .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("svg"))
        .filter_map(|e| e.path().file_stem().and_then(|s| s.to_str()).map(|s| s.to_string()))
        .collect();
    names.sort();

    let cols = names.len().max(1) as u32;
    let width = cols * (CELL + PAD) + PAD;
    let height = CELL + LABEL + PAD * 3 + CELL;
    let mut sheet = image::RgbaImage::from_pixel(width, height, image::Rgba([26, 26, 32, 255]));

    for (i, name) in names.iter().enumerate() {
        let text = std::fs::read_to_string(dir.join(format!("{name}.svg"))).expect("read svg");
        let cleaned = text.replace(BACKGROUND_RECT, "");
        let tree = resvg::usvg::Tree::from_str(&cleaned, &resvg::usvg::Options::default()).expect("parse svg");
        let mut pixmap = resvg::tiny_skia::Pixmap::new(CELL, CELL).expect("pixmap");
        let size = tree.size();
        let scale = (CELL as f32 / size.width()).min(CELL as f32 / size.height());
        resvg::render(&tree, resvg::tiny_skia::Transform::from_scale(scale, scale), &mut pixmap.as_mut());
        let x0 = PAD + i as u32 * (CELL + PAD);
        for (x, y, p) in pixmap.pixels().iter().enumerate().map(|(n, p)| (n as u32 % CELL, n as u32 / CELL, p)) {
            if p.alpha() == 0 {
                continue;
            }
            // tiny-skia hands back premultiplied colour; over the sheet's dark ground.
            let a = p.alpha() as f32 / 255.0;
            let blend = |c: u8, under: u8| ((c as f32) + (under as f32) * (1.0 - a)) as u8;
            let under = *sheet.get_pixel(x0 + x, PAD + y);
            sheet.put_pixel(
                x0 + x,
                PAD + y,
                image::Rgba([blend(p.red(), under[0]), blend(p.green(), under[1]), blend(p.blue(), under[2]), 255]),
            );
        }
        // THE DECIDING VIEW: the same glyph rendered at the 16 pixels the top bar draws it at, then
        // blown up without smoothing. A glyph made of many small repeated shapes survives the first
        // row and dissolves here, which is exactly how the coins came to read as a mineral.
        let small = 16u32;
        let mut tiny = resvg::tiny_skia::Pixmap::new(small, small).expect("pixmap");
        let s2 = (small as f32 / size.width()).min(small as f32 / size.height());
        resvg::render(&tree, resvg::tiny_skia::Transform::from_scale(s2, s2), &mut tiny.as_mut());
        let zoom = CELL / small;
        let y0 = PAD * 2 + CELL + LABEL;
        for (x, y, p) in tiny.pixels().iter().enumerate().map(|(n, p)| (n as u32 % small, n as u32 / small, p)) {
            let a = p.alpha() as f32 / 255.0;
            for dx in 0..zoom {
                for dy in 0..zoom {
                    let (px, py) = (x0 + x * zoom + dx, y0 + y * zoom + dy);
                    if px >= width || py >= height {
                        continue;
                    }
                    let under = *sheet.get_pixel(px, py);
                    let blend = |c: u8, u: u8| ((c as f32) + (u as f32) * (1.0 - a)) as u8;
                    sheet.put_pixel(px, py, image::Rgba([blend(p.red(), under[0]), blend(p.green(), under[1]), blend(p.blue(), under[2]), 255]));
                }
            }
        }
        // A crude five-by-seven stripe under each cell is enough to tell the columns apart in
        // order; the names are printed to stdout beside the sheet, left to right.
        for n in 0..name.len().min(12) as u32 {
            for dx in 0..8 {
                for dy in 0..4 {
                    sheet.put_pixel(x0 + n * 10 + dx, PAD + CELL + 6 + dy, image::Rgba([150, 150, 160, 255]));
                }
            }
        }
    }
    sheet.save(&out).expect("save sheet");
    println!("{out}: {} icons, left to right: {}", names.len(), names.join(", "));
}

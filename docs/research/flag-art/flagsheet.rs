//! Contact sheets of candidate flag SVGs, rendered with the same resvg the game uses.
//! Usage: flagsheet <svg-root> <out-dir>
use image::{imageops, Rgba, RgbaImage};
use resvg::{tiny_skia, usvg};

const COUNTRIES: [&str; 19] = ["in","cn","br","ru","us","au","de","fr","gb","sa","ir","tr","eg","ng","za","id","mx","ca","jp"];
const SOURCES: [&str; 7] = ["fi4x3","fi1x1","cfi3x2","circle","twemoji","noto","openmoji"];
const BG: Rgba<u8> = Rgba([30, 30, 36, 255]); // roughly the game's dark panel

fn load(path: &str) -> Option<usvg::Tree> {
    let text = std::fs::read_to_string(path).ok()?;
    usvg::Tree::from_str(&text, &usvg::Options::default()).ok()
}

fn to_image(pm: &tiny_skia::Pixmap) -> RgbaImage {
    let (w, h) = (pm.width(), pm.height());
    let mut img = RgbaImage::new(w, h);
    for (i, p) in pm.pixels().iter().enumerate() {
        let c = p.demultiply();
        img.put_pixel(i as u32 % w, i as u32 / w, Rgba([c.red(), c.green(), c.blue(), c.alpha()]));
    }
    img
}

/// Direct: rasterise at exactly `h` pixels tall, width by the SVG's own aspect.
fn direct(tree: &usvg::Tree, h: u32) -> RgbaImage {
    let size = tree.size();
    let scale = h as f32 / size.height();
    let w = (size.width() * scale).round().max(1.0) as u32;
    let mut pm = tiny_skia::Pixmap::new(w, h).unwrap();
    resvg::render(tree, tiny_skia::Transform::from_scale(scale, scale), &mut pm.as_mut());
    to_image(&pm)
}

/// The game's path today (src/icons.rs): fit into a 64x64 pixmap, then let egui shrink it
/// bilinearly to the size asked for. Modelled with a triangle-filter resize of the 64 square.
fn game_path(tree: &usvg::Tree, h: u32) -> RgbaImage {
    const R: u32 = 64;
    let size = tree.size();
    let scale = (R as f32 / size.width()).min(R as f32 / size.height());
    let mut pm = tiny_skia::Pixmap::new(R, R).unwrap();
    resvg::render(tree, tiny_skia::Transform::from_scale(scale, scale), &mut pm.as_mut());
    imageops::resize(&to_image(&pm), h, h, imageops::FilterType::Triangle)
}

fn blit(sheet: &mut RgbaImage, img: &RgbaImage, x: u32, y: u32) {
    imageops::overlay(sheet, img, x as i64, y as i64);
}

fn sheet(root: &str, h: u32, game: bool) -> RgbaImage {
    let cell_w = h * 2 + 8; // wide enough for a 2:1 flag (Nigeria, Russia at 2:3 in Noto)
    let cell_h = h + 8;
    let cols = SOURCES.len() as u32 + if game { 1 } else { 0 };
    let mut s = RgbaImage::from_pixel(cols * cell_w + 8, COUNTRIES.len() as u32 * cell_h + 8, BG);
    for (r, c) in COUNTRIES.iter().enumerate() {
        for (k, src) in SOURCES.iter().enumerate() {
            let path = format!("{root}/{src}/{c}.svg");
            if let Some(tree) = load(&path) {
                let img = direct(&tree, h);
                blit(&mut s, &img, 8 + k as u32 * cell_w, 8 + r as u32 * cell_h);
            } else {
                eprintln!("missing or unparsable: {path}");
            }
        }
        if game {
            if let Some(tree) = load(&format!("{root}/fi4x3/{c}.svg")) {
                let img = game_path(&tree, h);
                blit(&mut s, &img, 8 + SOURCES.len() as u32 * cell_w, 8 + r as u32 * cell_h);
            }
        }
    }
    s
}


/// Confusables: groups of flag-icons 4:3 flags that differ only by an emblem or a shade, each group
/// on one row, at `h` pixels and again at 48 for reference. The eyes-on negative control: these
/// SHOULD become hard to tell apart at 16 px.
fn pairs(root: &str, groups: &[&[&str]], h: u32) -> RgbaImage {
    let ref_h = 48u32;
    let cell_w = ref_h * 4 / 3 + 8;
    let max = groups.iter().map(|g| g.len()).max().unwrap() as u32;
    let row_h = ref_h + h + 16;
    let mut s = RgbaImage::from_pixel(max * cell_w + 8, groups.len() as u32 * row_h + 8, BG);
    for (r, g) in groups.iter().enumerate() {
        for (k, c) in g.iter().enumerate() {
            let Some(tree) = load(&format!("{root}/fi4x3/{c}.svg")) else { eprintln!("missing {c}"); continue };
            let x = 8 + k as u32 * cell_w;
            let y = 8 + r as u32 * row_h;
            blit(&mut s, &direct(&tree, ref_h), x, y);
            blit(&mut s, &direct(&tree, h), x, y + ref_h + 4);
        }
    }
    s
}

fn magnify(img: &RgbaImage, k: u32) -> RgbaImage {
    imageops::resize(img, img.width() * k, img.height() * k, imageops::FilterType::Nearest)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (root, out) = (&args[1], &args[2]);
    std::fs::create_dir_all(out).unwrap();
    if args.get(3).map(|s| s.as_str()) == Some("pairs") {
        let groups: [&[&str]; 6] = [&["eg", "iq", "ye", "sy"], &["mx", "it"], &["id", "mc"], &["au", "nz"], &["ru", "si", "sk", "nl", "lu"], &["td", "ro"]];
        let s = pairs(root, &groups, 16);
        s.save(format!("{out}/confusables-16px-native.png")).unwrap();
        magnify(&s, 4).save(format!("{out}/confusables-16px-x4.png")).unwrap();
        println!("wrote confusables: {}x{}", s.width(), s.height());
        return;
    }
    for (h, game, k) in [(16u32, true, 6u32), (22, true, 5), (64, false, 1)] {
        let s = sheet(root, h, game);
        s.save(format!("{out}/sheet-{h}px-native.png")).unwrap();
        if k > 1 {
            magnify(&s, k).save(format!("{out}/sheet-{h}px-x{k}.png")).unwrap();
        }
        println!("wrote sheet-{h}px: {}x{}", s.width(), s.height());
    }
}

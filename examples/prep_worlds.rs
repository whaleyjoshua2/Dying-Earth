//! One-off asset preparation: the Ceres, Vesta and Mercury maps to the PNGs the game loads.
//!
//! `cargo run --release --example prep_worlds -- <ceres map> <vesta map> <mercury map>`
//!
//! Every output is 1024 x 512 RGB, simple cylindrical, 180 W at the left edge and 0 in the middle,
//! north up, as `examples/prep_moons.rs` makes Phobos and Deimos (`docs/research/new-worlds.md` §7).
//! - Ceres: USGS "Ceres Dawn FC Global Mosaic 140m" (DLR, Feb 2016), the page's 1024 x 512 JPEG
//!   preview, <https://astrogeology.usgs.gov/search/map/ceres_dawn_fc_global_mosaic_140m>
//!   (file ceres_dawn_fc_dlr_global_feb2016_1024.jpg). Already centred on 0.
//!   Credit: NASA/JPL-Caltech/UCLA/MPS/DLR/IDA.
//! - Vesta: USGS "Vesta Dawn FC HAMO Global Mosaic 60m" (Dec 2013), the page's 1024 x 512 JPEG
//!   preview, <https://astrogeology.usgs.gov/search/map/vesta_dawn_fc_hamo_global_mosaic_60m>
//!   (file vesta_dawn_fc_hamo_mosaic_global_1024.jpg; full GeoTIFF
//!   <https://planetarymaps.usgs.gov/mosaic/Vesta_Dawn_FC_HAMO_Mosaic_Global_74ppd.tif>). Its
//!   longitudes are the IAU Claudia Double-Prime system (Claudia at 146 E), the Gazetteer's own, so
//!   no shift. The dark far north (seasonal shadow when Dawn mapped it) is left as it is.
//!   Credit: NASA/JPL-Caltech/UCLA/MPS/DLR/IDA.
//! - Mercury: NASA Photojournal PIA17386, "Enhanced Color Mercury Map" (2013), 4096 x 2048 JPEG,
//!   <https://science.nasa.gov/photojournal/enhanced-color-mercury-map>. Enhanced colour, not true
//!   colour. It is centred on 180 E, so its halves are swapped to put 0 in the middle.
//!   Credit: NASA/Johns Hopkins University Applied Physics Laboratory/Carnegie Institution of
//!   Washington.
//!
//! Ceres and Vesta are greyscale: each gets a light contrast stretch (the 0.5th and 99.5th
//! percentile greys to black and white) and goes out with the grey in all three channels. Mercury
//! keeps its colours untouched.

use image::imageops::FilterType;
use image::{GrayImage, RgbImage};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 3 {
        eprintln!("usage: prep_worlds <ceres map> <vesta map> <mercury map>");
        std::process::exit(2);
    }
    for (src, out) in [(&args[0], "ceres.png"), (&args[1], "vesta.png")] {
        let grey = stretch(&open(src).to_luma8());
        let rgb = RgbImage::from_fn(grey.width(), grey.height(), |x, y| { let v = grey.get_pixel(x, y)[0]; image::Rgb([v, v, v]) });
        save(rgb, out);
    }
    let src = &args[2];
    let img = open(src).to_rgb8();
    let (w, h) = (img.width(), img.height());
    let swapped = RgbImage::from_fn(w, h, |x, y| *img.get_pixel((x + w / 2) % w, y));
    save(swapped, "mercury.png");
}

fn open(src: &str) -> image::DynamicImage {
    image::open(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

/// The 0.5th and 99.5th percentile greys stretched to 0 and 255.
fn stretch(img: &GrayImage) -> GrayImage {
    let mut hist = [0u64; 256];
    for p in img.pixels() { hist[p[0] as usize] += 1; }
    let total: u64 = hist.iter().sum();
    let at = |frac: f64| { let target = (total as f64 * frac) as u64; let mut run = 0; for (v, n) in hist.iter().enumerate() { run += n; if run > target { return v as f64; } } 255.0 };
    let (lo, hi) = (at(0.005), at(0.995));
    let mut out = img.clone();
    for p in out.pixels_mut() { p[0] = ((p[0] as f64 - lo) / (hi - lo).max(1.0) * 255.0).round().clamp(0.0, 255.0) as u8; }
    println!("stretched greys {lo} - {hi} to 0 - 255");
    out
}

fn save(img: RgbImage, out: &str) {
    let (w, h) = (img.width(), img.height());
    let resized = if (w, h) == (1024, 512) { img } else { image::imageops::resize(&img, 1024, 512, FilterType::Lanczos3) };
    let path = format!("assets/textures/{out}");
    resized.save(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
    println!("wrote {path} ({} x {} from {w} x {h})", resized.width(), resized.height());
}

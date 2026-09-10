//! One-off asset preparation for ticket #45: the Phobos and Deimos maps to the PNGs the game loads.
//!
//! `cargo run --example prep_moons -- <phobos map> <deimos map>`
//!
//! Sources, both public domain and both simple cylindrical with 180 W at the left edge and 0 in the
//! middle, which is the orientation `geo::local_from_lonlat` wraps on a sphere:
//! - Phobos: the USGS Viking cylindrical map (solarviews.com/raw/mars/phoboscyl1.jpg, 1440 x 720).
//! - Deimos: Philip Stooke's cylindrical map, control from Peter Thomas's shape model
//!   (solarviews.com/raw/mars/deimoscyl4.jpg, 3600 x 1800).
//!
//! Both moons are potato-shaped in life; a sphere wearing the map is what the engine draws.

use image::imageops::FilterType;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        eprintln!("usage: prep_moons <phobos map> <deimos map>");
        std::process::exit(2);
    }
    for (src, out) in [(&args[0], "phobos.png"), (&args[1], "deimos.png")] {
        let img = image::open(src).unwrap_or_else(|e| panic!("{src}: {e}")).to_rgb8();
        let resized = image::imageops::resize(&img, 1024, 512, FilterType::Lanczos3);
        let path = format!("assets/textures/{out}");
        resized.save(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
        println!("wrote {path} ({} x {} from {} x {})", resized.width(), resized.height(), img.width(), img.height());
    }
}

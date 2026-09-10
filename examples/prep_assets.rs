//! One-off asset preparation (spec 2.4, 4.2): convert the NASA/USGS JPEGs to the PNGs the game
//! loads, and derive the Nation State mask from the Blue Marble coastlines.
//!
//! `cargo run --example prep_assets -- <dir with earth.jpg moon.jpg mars.jpg> [preview.png]`
//!
//! Mask layout (`assets/textures/earth_states.png`, grey 8-bit): 0 = water, 1..9 = the Nation
//! State index in the order the mask was painted (Africa, Antarctica, Asia, Australia and Oceania, Europe,
//! ...); since ticket #44 Antarctica is no Nation State and the window maps its value (2) to none.
//! The old order was (Africa, Antarctica, Asia, Australia and Oceania, Europe,
//! North America, South America, Russia, the Middle East). Islands go with the nearest continent,
//! Central America and the Caribbean with North America. Since ticket #26 Russia is its own state
//! (with northern Kazakhstan, as before) and so is the Middle East (Turkey, the Caucasus, the Levant,
//! Iraq, Iran and the Arabian Peninsula).

use image::{GrayImage, ImageBuffer, Rgb, RgbImage};
use std::path::Path;

const AFRICA: u8 = 1;
const ANTARCTICA: u8 = 2;
const ASIA: u8 = 3;
const AUSTRALIA: u8 = 4;
const EUROPE: u8 = 5;
const NORTH_AMERICA: u8 = 6;
const SOUTH_AMERICA: u8 = 7;
const RUSSIA: u8 = 8;
const MIDDLE_EAST: u8 = 9;

/// Which Nation State a land pixel at (longitude, latitude) belongs to.
pub fn state_for(lon: f64, lat: f64) -> u8 {
    if lat < -60.0 {
        return ANTARCTICA;
    }
    // Chukotka, across the antimeridian.
    if lon < -169.0 && lat > 64.0 {
        return RUSSIA;
    }
    // Polynesia, before the Americas catch everything west of -30.
    if lon < -120.0 && lat < 5.0 && lat > -50.0 {
        return AUSTRALIA;
    }
    if lon < -30.0 || (lon < -20.0 && lat > 67.0) {
        return if lon > -82.0 && lat < 12.0 { SOUTH_AMERICA } else { NORTH_AMERICA };
    }
    if (lat < -10.0 && lon > 110.0) || (lat < 0.0 && lon > 128.0) || (lon > 150.0 && lat < 30.0 && lat > -50.0) {
        return AUSTRALIA;
    }
    // The Red Sea runs from (12.5N, 43.5E) up to (30N, 32.5E); Arabia lies east of that line.
    let red_sea_lon = 43.5 - (lat - 12.5) * (11.0 / 17.5);
    let arabia = lat > 12.0 && lon > red_sea_lon;
    if lat < 35.2 && lon > -20.0 && lon < 52.0 && !arabia {
        return AFRICA;
    }
    // Algeria and Tunisia reach past 35N; Spain and Sicily lie outside this band.
    if lat < 37.3 && lon > -1.5 && lon < 12.0 && !arabia {
        return AFRICA;
    }
    // Russia: Karelia and the north east of 31E, the heartland east of 40E above 45N, then Siberia
    // north of the Kazakh, Mongolian and Manchurian borders, out to the Pacific.
    // Russia: east of Finland and the Baltics above 55N (St Petersburg at 30E), east of Belarus
    // between 50 and 55N, east of the Ukrainian border below that down to the Caucasus, then Siberia
    // north of the Kazakh, Mongolian and Manchurian borders, out to the Pacific.
    if lon < 180.0
        && ((lat > 55.0 && lon > 30.0)
            || (lat > 50.0 && lat <= 55.0 && lon > 33.0)
            || (lat > 44.0 && lat <= 50.0 && lon > 40.0 && lon < 60.0)
            || (lat > 50.0 && (60.0..87.0).contains(&lon))
            || (lat > 53.0 && (87.0..120.0).contains(&lon))
            || (lat > 54.0 && lon >= 120.0))
    {
        return RUSSIA;
    }
    // The Middle East: from the Bosporus to Iran's eastern border, the Caucasus below 44N, and
    // everything east of the Red Sea line down to the Arabian Sea.
    if lat > 12.0 && lat < 44.0 && lon > 26.0 && lon < 61.0 && (lon > red_sea_lon || lat > 30.0) {
        return MIDDLE_EAST;
    }
    if lon >= -25.0 && ((lat > 35.0 && lon < 26.0) || (lat > 42.0 && lon <= 40.0)) {
        return EUROPE;
    }
    ASIA
}

fn is_water(p: &Rgb<u8>) -> bool {
    let [r, g, b] = p.0;
    (b as i32) > (r as i32) + 25 && b >= g
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let src = Path::new(args.first().map(|s| s.as_str()).unwrap_or("."));
    let out = Path::new("assets/textures");
    std::fs::create_dir_all(out).expect("assets/textures");
    for (name, w, h) in [("earth", 2048u32, 1024u32), ("moon", 1024, 512), ("mars", 1024, 512)] {
        let img = image::open(src.join(format!("{name}.jpg"))).unwrap_or_else(|e| panic!("{name}.jpg: {e}"));
        let img = img.resize_exact(w, h, image::imageops::FilterType::Lanczos3).to_rgb8();
        img.save(out.join(format!("{name}.png"))).expect("save png");
        println!("wrote {name}.png {w}x{h}");
    }
    let earth: RgbImage = image::open(out.join("earth.png")).expect("earth.png").to_rgb8();
    let (w, h) = earth.dimensions();
    let mut mask: GrayImage = ImageBuffer::new(w, h);
    let mut counts = [0u64; 10];
    for y in 0..h {
        for x in 0..w {
            let lon = (x as f64 + 0.5) / w as f64 * 360.0 - 180.0;
            let lat = 90.0 - (y as f64 + 0.5) / h as f64 * 180.0;
            let v = if is_water(earth.get_pixel(x, y)) { 0 } else { state_for(lon, lat) };
            counts[v as usize] += 1;
            mask.put_pixel(x, y, image::Luma([v]));
        }
    }
    mask.save(out.join("earth_states.png")).expect("save mask");
    println!("wrote earth_states.png; pixel counts water/AF/AN/AS/AU/EU/NA/SA/RU/ME = {counts:?}");
    if let Some(preview) = args.get(1) {
        let colours: [[u8; 3]; 10] = [[0, 0, 0], [230, 180, 60], [240, 240, 240], [220, 80, 80], [160, 90, 200], [70, 130, 220], [80, 190, 90], [230, 130, 40], [200, 200, 90], [60, 200, 200]];
        let mut img: RgbImage = ImageBuffer::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let v = mask.get_pixel(x, y).0[0] as usize;
                let base = earth.get_pixel(x, y).0;
                let c = colours[v];
                let px = if v == 0 { base } else { [((base[0] as u16 + c[0] as u16 * 2) / 3) as u8, ((base[1] as u16 + c[1] as u16 * 2) / 3) as u8, ((base[2] as u16 + c[2] as u16 * 2) / 3) as u8] };
                img.put_pixel(x, y, Rgb(px));
            }
        }
        img.save(preview).expect("save preview");
        println!("wrote {preview}");
    }
}

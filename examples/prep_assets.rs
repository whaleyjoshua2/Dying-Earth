//! One-off asset preparation (spec 2.4, 4.2): convert the NASA/USGS JPEGs to the PNGs the game
//! loads, and derive the Nation State mask from the Blue Marble coastlines.
//!
//! `cargo run --example prep_assets -- <dir with earth.jpg moon.jpg mars.jpg> [preview.png]`
//!
//! `cargo run --example prep_assets -- --mask-only` rewrites only the mask, from the `earth.png`
//! already in `assets/textures`, so the borders can be redrawn without the source JPEGs.
//!
//! Mask layout (`assets/textures/earth_states.png`, grey 8-bit): 0 = water, 1..13 = a Nation State.
//! Values 1 to 9 are the order the mask was first painted (Sub-Saharan Africa, Antarctica, East
//! Asia, Australia and Oceania, Europe, North America, South America, Russia, the Middle East);
//! since ticket #44 Antarctica is no Nation State and the window maps its value (2) to none. Ticket
//! #53 split the map into twelve and APPENDED its four new states (10 North Africa, 11 South Asia,
//! 12 South-East Asia, 13 Central America and the Caribbean) rather than renumbering, so every old
//! value still means what it meant and the map can be split again the same way.
//!
//! Islands go with the nearest continent. Since ticket #26 Russia is its own state (with northern
//! Kazakhstan) and so is the Middle East (Turkey, the Caucasus, the Levant, Iraq, Iran and the
//! Arabian Peninsula). The borders below are lines of longitude and latitude, close enough for a
//! globe drawn at 2048 by 1024: they are a board, not an atlas.

use image::{GrayImage, ImageBuffer, Rgb, RgbImage};
use std::path::Path;

const SUB_SAHARAN_AFRICA: u8 = 1;
const ANTARCTICA: u8 = 2;
const EAST_ASIA: u8 = 3;
const AUSTRALIA: u8 = 4;
const EUROPE: u8 = 5;
const NORTH_AMERICA: u8 = 6;
const SOUTH_AMERICA: u8 = 7;
const RUSSIA: u8 = 8;
const MIDDLE_EAST: u8 = 9;
// Ticket #53: the four states the twelve-state split added, appended so the values above keep
// their meaning.
const NORTH_AFRICA: u8 = 10;
const SOUTH_ASIA: u8 = 11;
const SOUTH_EAST_ASIA: u8 = 12;
const CENTRAL_AMERICA: u8 = 13;

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
    // The Americas. Ticket #53 cuts Central America and the Caribbean out of North America along
    // the United States border: San Diego (32.5N, 117W) to the Gulf at Brownsville (25.9N, 97W),
    // then everything east of the Gulf below Florida, and the islands up to the Bahamas.
    if lon < -30.0 || (lon < -20.0 && lat > 67.0) {
        if lon > -82.0 && lat < 12.0 {
            return SOUTH_AMERICA;
        }
        let border = 32.5 - (lon + 117.0) * 0.33;
        let mexico = (-118.0..=-97.0).contains(&lon) && lat < border;
        let isthmus = lon > -97.0 && lon < -59.0 && lat < 23.5;
        let bahamas = lon > -80.0 && lon < -70.0 && lat < 27.5;
        return if mexico || isthmus || bahamas { CENTRAL_AMERICA } else { NORTH_AMERICA };
    }
    if (lat < -10.0 && lon > 110.0) || (lat < 0.0 && lon > 128.0) || (lon > 150.0 && lat < 30.0 && lat > -50.0) {
        return AUSTRALIA;
    }
    // The Red Sea runs from (12.5N, 43.5E) up to (30N, 32.5E); Arabia lies east of that line.
    let red_sea_lon = 43.5 - (lat - 12.5) * (11.0 / 17.5);
    let arabia = lat > 12.0 && lon > red_sea_lon;
    // Ticket #53: Africa splits at the Sahara. North Africa is Morocco to Egypt and down the Nile
    // through Sudan; everything below is Sub-Saharan.
    let africa = |lat: f64| if lat >= 18.0 { NORTH_AFRICA } else { SUB_SAHARAN_AFRICA };
    if lat < 35.2 && lon > -20.0 && lon < 52.0 && !arabia {
        return africa(lat);
    }
    // Algeria and Tunisia reach past 35N; Spain and Sicily lie outside this band.
    if lat < 37.3 && lon > -1.5 && lon < 12.0 && !arabia {
        return africa(lat);
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
    // Ticket #53: what is left of Asia splits three ways. South Asia is the subcontinent up to the
    // Himalaya, whose line slopes from 37N at Iran's border down to 29N at the Burmese one. South-
    // East Asia is Myanmar round to the Philippines, below the southern Chinese border (24N to the
    // west, 22N past Hong Kong so Taiwan stays with East Asia). Everything else -- China, Mongolia,
    // the Koreas, Japan and Central Asia -- is East Asia.
    let himalaya = 37.0 - (lon - 61.0) * 0.25;
    if (61.0..93.0).contains(&lon) && lat < himalaya {
        return SOUTH_ASIA;
    }
    let indochina = if lon >= 108.0 { 22.0 } else { 24.0 };
    if lon >= 92.0 && lat < indochina {
        return SOUTH_EAST_ASIA;
    }
    EAST_ASIA
}

fn is_water(p: &Rgb<u8>) -> bool {
    let [r, g, b] = p.0;
    (b as i32) > (r as i32) + 25 && b >= g
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    // Ticket #53: redraw the borders from the earth.png already in the tree, no source JPEGs needed.
    let mask_only = args.iter().any(|a| a == "--mask-only");
    let src = Path::new(args.first().filter(|a| !a.starts_with("--")).map(|s| s.as_str()).unwrap_or("."));
    let out = Path::new("assets/textures");
    std::fs::create_dir_all(out).expect("assets/textures");
    for (name, w, h) in [("earth", 2048u32, 1024u32), ("moon", 1024, 512), ("mars", 1024, 512)] {
        if mask_only {
            continue;
        }
        let img = image::open(src.join(format!("{name}.jpg"))).unwrap_or_else(|e| panic!("{name}.jpg: {e}"));
        let img = img.resize_exact(w, h, image::imageops::FilterType::Lanczos3).to_rgb8();
        img.save(out.join(format!("{name}.png"))).expect("save png");
        println!("wrote {name}.png {w}x{h}");
    }
    let earth: RgbImage = image::open(out.join("earth.png")).expect("earth.png").to_rgb8();
    let (w, h) = earth.dimensions();
    let mut mask: GrayImage = ImageBuffer::new(w, h);
    let mut counts = [0u64; 14];
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
    println!("wrote earth_states.png; pixel counts water/SSA/AN/EA/AU/EU/NA/SA/RU/ME/NAF/SAS/SEA/CAC = {counts:?}");
    if let Some(preview) = args.iter().find(|a| a.ends_with(".png") && !a.starts_with("--")) {
        let colours: [[u8; 3]; 14] = [
            [0, 0, 0], [230, 180, 60], [240, 240, 240], [220, 80, 80], [160, 90, 200], [70, 130, 220], [80, 190, 90],
            [230, 130, 40], [200, 200, 90], [60, 200, 200], [250, 120, 170], [120, 220, 120], [180, 120, 240], [250, 220, 120],
        ];
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

//! One-off asset preparation (spec 2.4, 4.2): convert the NASA/USGS JPEGs to the PNGs the game
//! loads, and derive the Region mask from the Blue Marble coastlines and real country borders.
//!
//! `cargo run --example prep_assets -- <dir with earth.jpg moon.jpg mars.jpg> --borders <geojson> [preview.png]`
//!
//! `cargo run --example prep_assets -- --mask-only --borders <geojson> [preview.png]` rewrites only
//! the mask, from the `earth.png` already in `assets/textures`, so the borders can be redrawn
//! without the source JPEGs.
//!
//! Mask layout (`assets/textures/earth_states.png`, grey 8-bit): 0 = water, 1..15 = a Region.
//! Values 1 to 9 are the order the mask was first painted (Sub-Saharan Africa, Antarctica, East
//! Asia, Australia and Oceania, Europe, North America, South America, Russia, the Middle East);
//! since ticket #44 Antarctica is no Region and the window maps its value (2) to none. Ticket #53
//! split the map into twelve and APPENDED its four new Regions (10 North Africa, 11 South Asia, 12
//! South-East Asia, 13 Central America and the Caribbean) rather than renumbering; ticket #125
//! (version 0.07.2) appended two more (14 Japan and Korea, 15 the Arabian Peninsula) the same way,
//! so every old value still means what it meant. The Regions have been named for their Nations
//! since ticket #122 -- value 3 is the Region called China -- but the mask's names are the
//! geographic ones it was painted under, since that is what the value covers.
//!
//! **Since version 0.07.2 the borders are countries, not lines.** Until then every border was a
//! line of longitude or latitude ("a board, not an atlas"); the designer asked for real borders,
//! and ticket #124's research found Natural Earth's Admin 0 countries at 1:50m (public domain) and
//! that World Bank totals over a country-by-country assignment reproduce the game's own population
//! and GDP figures, so the ground can follow countries without the numbers being re-derived. Each
//! country polygon is filled into the equirectangular grid and given its Region by the table below;
//! the coastline still comes from the photograph, as it always did, so a pixel is water if
//! `earth.png` says so whatever the polygon says; and land the polygons do not cover -- fringe
//! pixels, atolls the photograph shows and Natural Earth omits -- takes the nearest Region by a
//! flood from the pixels that have one. One pixel is about twenty kilometres at the equator, so a
//! border is as good as that and no better.
//!
//! **The rule is: a country belongs whole to one Region, and an island belongs to its country.**
//! The designer took that block on ticket #125 with one exception, Cyprus, which files under
//! Western Asia and was moved to Europe. Its consequences worth knowing: Kazakhstan, the one country
//! the old lines split, goes whole to the Region called China; all of Indonesia goes to the Region
//! called Indonesia, with the real border across New Guinea at 141 E; Greenland stays with the
//! United States' Region; Hawaii goes with the United States and the Canaries with Spain. Natural
//! Earth keeps French Guiana, Reunion and Mayotte inside France's feature, so under this rule they
//! are the European Union's, and that is written down rather than hidden.
use image::{GrayImage, ImageBuffer, Rgb, RgbImage};
use std::collections::VecDeque;
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
const NORTH_AFRICA: u8 = 10;
const SOUTH_ASIA: u8 = 11;
const SOUTH_EAST_ASIA: u8 = 12;
const CENTRAL_AMERICA: u8 = 13;
const JAPAN_KOREA: u8 = 14;
const ARABIAN_PENINSULA: u8 = 15;
const VALUES: usize = 16;

/// The Region a country belongs to, by Natural Earth's `ADM0_A3`, then by its `SUBREGION`, then by
/// its `CONTINENT`. Every case the research flagged is decided here by name, so the table can be
/// read against ticket #125 line by line; the subregion and continent rows catch everything else.
fn region_for(adm0: &str, subregion: &str, continent: &str) -> u8 {
    match adm0 {
        // Ticket #125: the two Regions added in version 0.07.2.
        "JPN" | "KOR" | "PRK" => return JAPAN_KOREA,
        "SAU" | "ARE" | "OMN" | "YEM" | "QAT" | "BHR" | "KWT" => return ARABIAN_PENINSULA,
        // The Middle East that remains: Turkey to Iran, the Levant, Iraq and the Caucasus.
        "TUR" | "IRN" | "IRQ" | "SYR" | "LBN" | "ISR" | "PSX" | "PSE" | "JOR" | "ARM" | "AZE" | "GEO" => return MIDDLE_EAST,
        // Cyprus is in the European Union: the designer's one exception to the subregion rule.
        "CYP" => return EUROPE,
        // Greenland stays with the United States' Region, as it always was.
        "GRL" | "USA" | "CAN" | "BMU" | "SPM" => return NORTH_AMERICA,
        "MEX" => return CENTRAL_AMERICA,
        // Kazakhstan goes whole to China's Region with the rest of Central Asia, ending the one
        // place the old lines split a country.
        "KAZ" | "UZB" | "TKM" | "KGZ" | "TJK" | "CHN" | "MNG" | "TWN" | "HKG" | "MAC" => return EAST_ASIA,
        "AFG" | "IND" | "PAK" | "BGD" | "LKA" | "NPL" | "BTN" | "MDV" => return SOUTH_ASIA,
        // North Africa: Morocco to Egypt and down through Sudan, as the table has always said.
        "EGY" | "LBY" | "TUN" | "DZA" | "MAR" | "ESH" | "SDN" => return NORTH_AFRICA,
        "RUS" => return RUSSIA,
        "ATA" => return ANTARCTICA,
        _ => {}
    }
    match subregion {
        "South-Eastern Asia" => SOUTH_EAST_ASIA,
        "Southern Asia" => SOUTH_ASIA,
        "Eastern Asia" => EAST_ASIA,
        "Central Asia" => EAST_ASIA,
        "Western Asia" => MIDDLE_EAST,
        "Northern Africa" => NORTH_AFRICA,
        "Western Africa" | "Eastern Africa" | "Middle Africa" | "Southern Africa" | "Sub-Saharan Africa" => SUB_SAHARAN_AFRICA,
        "Central America" | "Caribbean" => CENTRAL_AMERICA,
        "Northern America" => NORTH_AMERICA,
        "South America" => SOUTH_AMERICA,
        "Northern Europe" | "Western Europe" | "Southern Europe" | "Eastern Europe" => EUROPE,
        "Australia and New Zealand" | "Melanesia" | "Micronesia" | "Polynesia" => AUSTRALIA,
        "Antarctica" => ANTARCTICA,
        _ => match continent {
            "Africa" => SUB_SAHARAN_AFRICA,
            "Europe" => EUROPE,
            "Asia" => EAST_ASIA,
            "North America" => NORTH_AMERICA,
            "South America" => SOUTH_AMERICA,
            "Oceania" => AUSTRALIA,
            "Antarctica" => ANTARCTICA,
            // "Seven seas (open ocean)" and the like: left to the flood, which gives the nearest.
            _ => 0,
        },
    }
}

fn is_water(p: &Rgb<u8>) -> bool {
    let [r, g, b] = p.0;
    (b as i32) > (r as i32) + 25 && b >= g
}

/// Fill one polygon -- an outer ring and its holes, as one even-odd shape -- into `grid`, in
/// equirectangular pixel space. Scanline over the rows the rings span; a pixel is inside where an
/// odd number of edges cross to its left.
fn fill_polygon(rings: &[Vec<(f64, f64)>], value: u8, grid: &mut [u8], w: u32, h: u32) {
    let to_px = |lon: f64, lat: f64| -> (f64, f64) { ((lon + 180.0) / 360.0 * w as f64, (90.0 - lat) / 180.0 * h as f64) };
    let mut edges: Vec<((f64, f64), (f64, f64))> = Vec::new();
    let (mut y_min, mut y_max) = (f64::MAX, f64::MIN);
    for ring in rings {
        for i in 0..ring.len() {
            let (ax, ay) = to_px(ring[i].0, ring[i].1);
            let (bx, by) = to_px(ring[(i + 1) % ring.len()].0, ring[(i + 1) % ring.len()].1);
            if ay == by {
                continue;
            }
            y_min = y_min.min(ay.min(by));
            y_max = y_max.max(ay.max(by));
            edges.push(((ax, ay), (bx, by)));
        }
    }
    if edges.is_empty() {
        return;
    }
    let row0 = y_min.floor().max(0.0) as u32;
    let row1 = (y_max.ceil() as u32).min(h);
    let mut xs: Vec<f64> = Vec::new();
    for row in row0..row1 {
        let yc = row as f64 + 0.5;
        xs.clear();
        for ((ax, ay), (bx, by)) in &edges {
            let (top, bottom) = if ay < by { (*ay, *by) } else { (*by, *ay) };
            if yc >= top && yc < bottom {
                let t = (yc - ay) / (by - ay);
                xs.push(ax + t * (bx - ax));
            }
        }
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for pair in xs.chunks(2) {
            if pair.len() < 2 {
                break;
            }
            let x0 = (pair[0] + 0.5).floor().max(0.0) as u32;
            let x1 = ((pair[1] + 0.5).floor() as u32).min(w);
            for x in x0..x1 {
                grid[(row * w + x) as usize] = value;
            }
        }
    }
}

fn ring_from(coords: &serde_json::Value) -> Vec<(f64, f64)> {
    coords
        .as_array()
        .map(|pts| pts.iter().filter_map(|p| Some((p.get(0)?.as_f64()?, p.get(1)?.as_f64()?))).collect())
        .unwrap_or_default()
}

/// Paint every country in the GeoJSON into a grid of Region values (0 where no polygon reaches).
fn paint_countries(geojson: &Path, w: u32, h: u32) -> (Vec<u8>, Vec<(String, u8)>) {
    let text = std::fs::read_to_string(geojson).unwrap_or_else(|e| panic!("{}: {e}", geojson.display()));
    let doc: serde_json::Value = serde_json::from_str(&text).expect("GeoJSON");
    let mut grid = vec![0u8; (w * h) as usize];
    let mut table: Vec<(String, u8)> = Vec::new();
    for feature in doc["features"].as_array().expect("features") {
        let props = &feature["properties"];
        let s = |k: &str| props[k].as_str().unwrap_or("");
        let value = region_for(s("ADM0_A3"), s("SUBREGION"), s("CONTINENT"));
        table.push((format!("{} ({})", s("NAME"), s("ADM0_A3")), value));
        if value == 0 {
            continue;
        }
        let geom = &feature["geometry"];
        match geom["type"].as_str().unwrap_or("") {
            "Polygon" => {
                let rings: Vec<Vec<(f64, f64)>> = geom["coordinates"].as_array().map(|rs| rs.iter().map(ring_from).collect()).unwrap_or_default();
                fill_polygon(&rings, value, &mut grid, w, h);
            }
            "MultiPolygon" => {
                for poly in geom["coordinates"].as_array().into_iter().flatten() {
                    let rings: Vec<Vec<(f64, f64)>> = poly.as_array().map(|rs| rs.iter().map(ring_from).collect()).unwrap_or_default();
                    fill_polygon(&rings, value, &mut grid, w, h);
                }
            }
            _ => {}
        }
    }
    (grid, table)
}

/// Land the polygons did not reach takes the Region of the nearest land that has one: a flood
/// from every assigned land pixel outward, four-connected, wrapping east to west. Water stays 0.
fn flood_unassigned(mask: &mut [u8], land: &[bool], w: u32, h: u32) -> usize {
    let mut queue: VecDeque<u32> = VecDeque::new();
    for i in 0..(w * h) {
        if land[i as usize] && mask[i as usize] != 0 {
            queue.push_back(i);
        }
    }
    let mut filled = 0usize;
    while let Some(i) = queue.pop_front() {
        let v = mask[i as usize];
        let (x, y) = (i % w, i / w);
        let neighbours = [
            Some(((x + w - 1) % w, y)),
            Some(((x + 1) % w, y)),
            if y > 0 { Some((x, y - 1)) } else { None },
            if y + 1 < h { Some((x, y + 1)) } else { None },
        ];
        for (nx, ny) in neighbours.into_iter().flatten() {
            let j = (ny * w + nx) as usize;
            if land[j] && mask[j] == 0 {
                mask[j] = v;
                filled += 1;
                queue.push_back(j as u32);
            }
        }
    }
    filled
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    // Ticket #53: redraw the borders from the earth.png already in the tree, no source JPEGs needed.
    let mask_only = args.iter().any(|a| a == "--mask-only");
    // Ticket #93 (version 0.06.0): `--venus` draws Venus's clouds, since no NASA map is in the
    // tree: a 1024 by 512 globe of cream and ochre bands with slow swirls, at the same size and
    // softness as the Moon and Mars maps so it sits beside them as one of them.
    if args.iter().any(|a| a == "--venus") {
        let (w, h) = (1024u32, 512u32);
        let out = Path::new("assets/textures");
        std::fs::create_dir_all(out).expect("assets/textures");
        let mut img: RgbImage = ImageBuffer::new(w, h);
        // Layered sines stand in for noise: no dependency, and deterministic.
        let noise = |x: f32, y: f32| -> f32 {
            let mut v = 0.0;
            let mut amp = 1.0;
            let mut f = 1.0;
            for k in 0..5 {
                let px = x * f + k as f32 * 1.7;
                let py = y * f * 0.6 + k as f32 * 0.9;
                let tau = std::f32::consts::TAU;
                v += amp * ((px * tau).sin() * (py * tau + (px * 3.1).cos()).cos());
                amp *= 0.5;
                f *= 2.1;
            }
            v
        };
        for y in 0..h {
            for x in 0..w {
                let u = x as f32 / w as f32;
                let v = y as f32 / h as f32;
                let lat = (v - 0.5) * 2.0;
                // Bands run with latitude; the swirls drift east with height, as Venus's clouds do.
                let band = ((v * 14.0 + noise(u * 0.5, v) * 0.8).sin() * 0.5 + 0.5) * 0.35;
                let swirl = noise(u + lat * 0.15, v * 1.3) * 0.12;
                let polar = (lat.abs().powi(3)) * 0.25;
                let t = (0.55 + band + swirl - polar).clamp(0.0, 1.0);
                // From deep ochre to pale cream.
                let r = 205.0 + 45.0 * t;
                let g = 165.0 + 65.0 * t;
                let b = 95.0 + 95.0 * t;
                img.put_pixel(x, y, Rgb([r.min(255.0) as u8, g.min(255.0) as u8, b.min(255.0) as u8]));
            }
        }
        img.save(out.join("venus.png")).expect("save venus.png");
        println!("wrote venus.png {w}x{h}");
        return;
    }
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
    // Ticket #125 (version 0.07.2): the borders come from the country data named after `--borders`.
    let Some(borders) = args.iter().position(|a| a == "--borders").and_then(|i| args.get(i + 1)) else {
        eprintln!("--borders <ne_50m_admin_0_countries.geojson> is wanted: the Region mask is drawn from country borders since version 0.07.2.");
        eprintln!("Natural Earth, public domain: https://www.naturalearthdata.com/downloads/50m-cultural-vectors/");
        std::process::exit(2);
    };
    let earth: RgbImage = image::open(out.join("earth.png")).expect("earth.png").to_rgb8();
    let (w, h) = earth.dimensions();
    let (painted, table) = paint_countries(Path::new(borders), w, h);
    let land: Vec<bool> = earth.pixels().map(|p| !is_water(p)).collect();
    let mut values: Vec<u8> = painted.iter().zip(&land).map(|(v, l)| if *l { *v } else { 0 }).collect();
    let filled = flood_unassigned(&mut values, &land, w, h);
    let mut mask: GrayImage = ImageBuffer::new(w, h);
    let mut counts = [0u64; VALUES];
    for y in 0..h {
        for x in 0..w {
            let v = values[(y * w + x) as usize];
            counts[v as usize] += 1;
            mask.put_pixel(x, y, image::Luma([v]));
        }
    }
    mask.save(out.join("earth_states.png")).expect("save mask");
    let unassigned: Vec<&(String, u8)> = table.iter().filter(|(_, v)| *v == 0).collect();
    println!("wrote earth_states.png from {} countries; {} land pixels took the nearest Region by flood", table.len(), filled);
    println!("pixel counts water/SSA/AN/EA/AU/EU/NA/SA/RU/ME/NAF/SAS/SEA/CAC/JK/ARB = {counts:?}");
    if !unassigned.is_empty() {
        println!("left to the flood, having no Region of their own: {}", unassigned.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>().join(", "));
    }
    if let Some(preview) = args.iter().find(|a| a.ends_with(".png") && !a.starts_with("--")) {
        let colours: [[u8; 3]; VALUES] = [
            [0, 0, 0], [230, 180, 60], [240, 240, 240], [220, 80, 80], [160, 90, 200], [70, 130, 220], [80, 190, 90],
            [230, 130, 40], [200, 200, 90], [60, 200, 200], [250, 120, 170], [120, 220, 120], [180, 120, 240], [250, 220, 120],
            // Ticket #125: Japan and Korea a sky blue, the Arabian Peninsula a violet, at the
            // designer's word -- and only here: on the board a Region wears its controller's colour.
            [100, 180, 240], [130, 70, 200],
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
        println!("wrote preview {preview}");
    }
}

// PROTOTYPE - throwaway. A procedural Earth texture, generated in code so that
// neither prototype needs an assets folder beside the exe. Identical copy in both.
//
// Layout: equirectangular, x = longitude (-180..180), y = latitude (+90 at top).
// Shows: ocean, blobby continents, ice caps, a 30-degree graticule, and three
// tinted "nation state" patches so the Earth view has something to point at.

pub const W: u32 = 1024;
pub const H: u32 = 512;

// (lon_min, lon_max, lat_min, lat_max, r, g, b) in degrees. Stewards green,
// Extractors orange, one neutral grey state.
const STATES: [(f32, f32, f32, f32, f32, f32, f32); 3] = [
    (-120.0, -60.0, 15.0, 55.0, 0.25, 0.85, 0.35),
    (20.0, 90.0, 20.0, 60.0, 0.95, 0.55, 0.15),
    (100.0, 150.0, -40.0, -10.0, 0.70, 0.70, 0.70),
];

fn landness(lon: f32, lat: f32) -> f32 {
    let n = (lon * 1.3).sin() * (lat * 2.1).cos()
        + 0.5 * (lon * 3.7 + 1.0).sin() * (lat * 1.7).sin()
        + 0.35 * (lon * 7.1).cos() * (lat * 5.3).cos()
        + 0.2 * (lon * 11.0 + lat * 3.0).sin();
    n - 0.15
}

pub fn earth_rgba() -> Vec<u8> {
    let mut px = Vec::with_capacity((W * H * 4) as usize);
    for y in 0..H {
        for x in 0..W {
            let u = x as f32 / W as f32;
            let v = y as f32 / H as f32;
            let lon = (u - 0.5) * std::f32::consts::TAU; // -pi..pi
            let lat = (0.5 - v) * std::f32::consts::PI; // +pi/2 at top
            let lon_d = lon.to_degrees();
            let lat_d = lat.to_degrees();
            let land = landness(lon, lat);

            let (mut r, mut g, mut b) = if land > 0.0 {
                let t = land.min(0.6) / 0.6;
                (0.30 + 0.25 * t, 0.50 + 0.10 * t, 0.18)
            } else {
                (0.04, 0.16, 0.48)
            };

            // nation-state tint on land
            if land > 0.0 {
                for (lo0, lo1, la0, la1, sr, sg, sb) in STATES {
                    if lon_d >= lo0 && lon_d <= lo1 && lat_d >= la0 && lat_d <= la1 {
                        r = r * 0.35 + sr * 0.65;
                        g = g * 0.35 + sg * 0.65;
                        b = b * 0.35 + sb * 0.65;
                    }
                }
            }

            // ice caps
            let ice = ((lat_d.abs() - 68.0) / 12.0).clamp(0.0, 1.0);
            r += (0.93 - r) * ice;
            g += (0.95 - g) * ice;
            b += (0.97 - b) * ice;

            // graticule every 30 degrees
            let near = |d: f32| (d.rem_euclid(30.0)).min(30.0 - d.rem_euclid(30.0)) < 0.35;
            if near(lon_d) || near(lat_d) {
                r *= 0.55;
                g *= 0.55;
                b *= 0.55;
            }

            px.push((r.clamp(0.0, 1.0) * 255.0) as u8);
            px.push((g.clamp(0.0, 1.0) * 255.0) as u8);
            px.push((b.clamp(0.0, 1.0) * 255.0) as u8);
            px.push(255);
        }
    }
    px
}

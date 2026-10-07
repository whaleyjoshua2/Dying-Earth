# Ceres, 4 Vesta and Mercury: the figures from good sources

**Date:** 2026-10-07
**Ticket:** #501 (part of #500). **Branch:** `research/new-worlds` (research note only; no code changed)

**What this is for:** the game will add three Bodies. This note gives what the game reads for a
Body today (travel, placement on the Solar System Map, gravity, sunlight, Colony Slots, a surface
map, and what the world holds) for Ceres, 4 Vesta and Mercury, each figure with its source.

It follows `docs/research/delta-v.md`: the same constants, the same formulas, the same 300 km low
orbits for planets, every leg **orbit to orbit**, and the same leave / gulf / arrive structure
(section 3.9.6 there). The method is checked against that note before it is used here: run on
Earth, Venus and Mars it returns the gulfs already in `assets/data/bodies.toml` (Earth-Mars 1.07,
Earth-Venus 0.64, Venus-Mars 3.49 against 3.48).

Every figure was fetched from a live source during this session or **computed** in this session
from fetched constants and ephemerides. Computed figures say so and show their inputs. Things
that could not be checked are marked **UNVERIFIED**. Nothing is stated from memory.

---

## 1. Summary

### 1.1 Travel, in the game's structure (km/s)

None of the three has air, so **no aerobraking is possible at any of them**: arriving costs the
same as leaving. Ceres and Vesta have such small gravity wells that leaving or arriving is about
a tenth of a km/s; almost the whole cost of reaching them is the gulf.

| Place | Leave | Arrive | Low orbit used | Note |
|---|---|---|---|---|
| Mercury | **1.17** | **1.17** | 300 km circular (as for the planets in delta-v.md) | No air |
| Ceres | **0.11** | **0.11** | 385 km, Dawn's flown lowest orbit (LAMO) | No air |
| Vesta | **0.08** | **0.08** | 210 km, Dawn's flown lowest orbit (LAMO) | No air |

| Gulf (same both ways) | Hohmann, coplanar (the game's method) | Real windows 2030-2050, ballistic: best / median / worst |
|---|---|---|
| Earth - Mercury | **8.75** | 8.5 / 11.7 - 11.8 / 14.5 |
| Earth - Ceres | **6.19** | 6.7 / 8.5 / 10.8 |
| Earth - Vesta | **5.48** | 5.5 / 7.4 / 8.5 |
| Mars - Ceres | **3.47** | 4.3 / 5.3 / 6.4 |
| Mars - Vesta | **2.56** | 3.0 / 3.3 / 4.2 |
| Ceres - Vesta | **0.96** | 1.9 (only one window, 2046, in 2031-2059) |
| Venus - Mercury | **5.39** | 5.5 / 7.0 - 7.2 / 9.2 |
| Mars - Mercury | **14.39** | 13.8 / 16.9 / 19.8 |
| Venus - Ceres | 9.48 | not searched |
| Venus - Vesta | 8.82 | not searched |
| Far orbit (Earth L4 / L5) - Mercury | 13.94 | not searched |
| Far orbit - Ceres | 10.81 | not searched |
| Far orbit - Vesta | 9.69 | not searched |

The first column is the figure the game's existing gulfs are made the same way as (circular,
coplanar orbits). The second is the same leave/arrive structure on real, inclined, eccentric
orbits from JPL ephemerides, without deep-space manoeuvres or gravity assists; "median" is the
median over the windows of both directions taken together (for the Mercury rows, the two
directions' medians). **Ceres, Vesta and Mercury are all
inclined 7 - 11 degrees to Earth's orbit, so real windows cost more than the coplanar figure
and vary a great deal from one window to the next** (section 3.3).

Whole journeys from the structure (coplanar gulfs):

| From \ To | LEO | Mars (low orbit) | Mercury | Ceres | Vesta |
|---|---|---|---|---|---|
| **LEO** | - | 4.52 | **13.12** | **9.50** | **8.76** |
| **Mars** | 3.02 | - | 16.97 | **4.99** | **4.05** |
| **Mercury** | **10.46** | 15.81 | - | 21.98 | 21.48 |
| **Ceres** | **6.84** | **3.83** | 21.98 | - | **1.15** |
| **Vesta** | **6.10** | **2.89** | 21.48 | **1.15** | - |

Mercury - Ceres and Mercury - Vesta use gulfs computed the same way (Hohmann between 0.387 and
2.767 / 2.362 au: 20.70 and 20.23, flights of 362 and 294 days); they are listed only to fill
the table. Nothing goes between Mercury and the asteroid belt cheaply.

### 1.2 Flight times and windows (Hohmann, circular orbits; 60-day turns)

| Crossing | Days at window | Turns | Phase angle out | Phase angle back | Synodic period |
|---|---|---|---|---|---|
| Earth - Mercury | 105.5 | 1.8 | **+108.3** | **-76.0** | 115.9 d (1.9 turns) |
| Earth - Ceres | 472.1 | 7.9 | **+78.9** | **-74.7** | 466.6 d (7.8 turns) |
| Earth - Vesta | 397.9 | 6.6 | **+71.9** | **-147.8** | 504.2 d (8.4 turns) |
| Mars - Ceres | 573.9 | 9.6 | **+57.1** | **+120.7** | 1161.5 d (19.4 turns) |
| Mars - Vesta | 494.5 | 8.2 | **+45.7** | **+79.1** | 1425.9 d (23.8 turns) |
| Ceres - Vesta | 749.9 | 12.5 | **-23.7** | **-19.4** | 6263 d (17.1 years) |
| Venus - Mercury | 75.6 | 1.3 | **-129.2** | **-59.0** | 144.6 d (2.4 turns) |
| Mars - Mercury | 170.5 | 2.8 | **-157.9** | **-90.6** | 100.9 d (1.7 turns) |

Phase angles are read as `ephemeris.toml` reads them: **the second-named body's heliocentric
longitude less the first-named body's**, for both legs. "Out" is the departure angle for the
flight from the first-named to the second; "back" is the angle at departure for the return.
Checked: the same arithmetic gives Earth-Mars 44.3 / 75.1 and Earth-Venus -54.0 / -36.0, the
figures in `ephemeris.toml`.

### 1.3 The worlds

| | Mercury | Ceres | Vesta | For comparison |
|---|---|---|---|---|
| Mean radius, km | 2,439.4 | 469.7 | 261.4 | Moon 1,737.4 |
| GM, km^3/s^2 | 22,031.868551 | 62.6284 | 17.2882844 | |
| Surface gravity, m/s^2 | **3.70** | **0.28** | **0.25** | Moon 1.62, Mars 3.71 (JPL) |
| Escape velocity, km/s | **4.25** | **0.52** | **0.36** | Moon 2.38 |
| **Low gravity by the game's line?** | **No** (as Mars) | **Yes** | **Yes** | The line today: Moon, Phobos, Deimos low; Mars, Earth not |
| Mean distance from the Sun, au | 0.387 | 2.767 | 2.362 | |
| Sunlight, x Earth's (mean) | **6.67** | **0.131** | **0.179** | Mars 0.431 |
| Sunlight range over the orbit | 4.59 - 10.58 | 0.112 - 0.154 | 0.151 - 0.216 | |
| Day | 176 Earth days (solar) | 9.07 h | 5.34 h | |
| Air | none | none | none | |

### 1.4 What could not be sourced

- **No published orbit-to-orbit delta-v for any leg to Ceres, Vesta or Mercury was found.**
  Every travel figure here is computed. The two cross-checks that exist agree: the departure
  burn from LEO toward Mercury (5.55) matches Wikipedia's 5.5 (secondary), and this note's
  real-window search reproduces JPL's own Small-Body Mission Design tool for Earth to Ceres
  to 0.07 km/s.
- **No ballistic window search was run for Venus - Ceres, Venus - Vesta, or the far orbits**;
  those rows are coplanar Hohmann only.
- **The surface-map licences are not public domain in the USGS sense** (section 7): the maps
  are NASA mission products made by NASA's partners (DLR for Dawn, APL and others for
  MESSENGER). NASA's guidelines put texture maps outside US copyright; the MESSENGER team's own
  statement speaks of "non-commercial educational and public information purposes". Whether
  that binds a game on itch.io is a question for the designer, not answered here.
- Some Colony Slot notes (section 6) rest on a paper's title or abstract that does not name
  the crater; those are marked.

---

## 2. Constants and method

### 2.1 Constants (all fetched this session)

| Quantity | Value | Source |
|---|---|---|
| GM Sun, au, GM Earth / Mars / Venus | as delta-v.md section 2.1 | <https://ssd.jpl.nasa.gov/astro_par.html> |
| GM Mercury | 22,031.868551 km^3/s^2 | <https://ssd.jpl.nasa.gov/astro_par.html> (fetched) |
| Mercury radius, mean / equatorial | 2,439.4 / 2,440.53 km | <https://ssd.jpl.nasa.gov/planets/phys_par.html> |
| Mercury equatorial gravity, escape velocity | 3.70 m/s^2, 4.25 km/s | same page |
| Mercury sidereal rotation | 58.6462 d | same page |
| GM Ceres | 62.6284 km^3/s^2 | JPL Small-Body Database (SBDB) API, citing Park et al. 2016, *Nature* 537:515 |
| Ceres radius, mean / equatorial | 469.7 / 482.1 km | <https://ssd.jpl.nasa.gov/planets/phys_par.html> (dwarf planet table); SBDB diameter 939.4 km |
| Ceres equatorial gravity, escape velocity | 0.27 m/s^2, 0.51 km/s | phys_par dwarf planet table |
| Ceres density, rotation | 2.162 g/cm^3, 9.074170 h | SBDB, citing Park et al. 2016 |
| GM Vesta | 17.2882844 km^3/s^2 | SBDB, citing Park et al. 2025, *Nature Astronomy*, doi 10.1038/s41550-025-02533-7 |
| Vesta mean diameter | 522.77 km (radius 261.385 km) | same |
| Vesta density, rotation | 3.460 g/cm^3, 5.3421276 h | same |
| Dawn LAMO altitude, Vesta | "an average altitude of 210 kilometers (130 miles)" | Rayman, JPL Dawn Journal, Feb 2012: <https://www.jpl.nasa.gov/blog/2012/2/highs-and-lows-of-exploring-the-giant-asteroid> |
| Dawn LAMO altitude, Ceres | "an average altitude of about 240 miles (385 kilometers)"; speed "610 mph" | <https://www.jpl.nasa.gov/blog/2015/11/dawn-begins-descent-to-closest-and-final-orbit-at-ceres> |
| Moon GM, radius | 4,902.800118 km^3/s^2, 1,737.4 km | as delta-v.md |

SBDB API calls: <https://ssd-api.jpl.nasa.gov/sbdb.api?sstr=1&phys-par=1> and `sstr=4`.

**Check on the Ceres low orbit:** the circular speed computed at 385 km, sqrt(62.6284 / 854.7) =
0.2707 km/s = 605 mph, against the 610 mph JPL gives. Agreed.

### 2.2 Low orbits chosen

- **Mercury: 300 km circular**, the altitude delta-v.md uses for every planet. MESSENGER flew a
  200 km periapsis (<https://messenger.jhuapl.edu/About/stationkeeping.html>: OCMs "lower the
  minimum altitude to 200 kilometers"). From 200 km Mercury's leave figure would be 1.20, not
  1.17.
- **Ceres: 385 km; Vesta: 210 km**, the lowest orbits Dawn actually flew. A different choice
  barely matters: the whole well of either is about 0.1 km/s deep.

### 2.3 Formulas

The same as delta-v.md section 2.2. For the leave / gulf / arrive split (delta-v.md 3.9.6),
with v_esc and v_circ taken at the low orbit:

```
leave  = arrive (no air) = v_esc - v_circ
excess(place, v_inf)     = sqrt(v_inf^2 + v_esc^2) - v_esc     # what a v_inf costs above escape
gulf(A, B)               = excess(A, v_inf at A) + excess(B, v_inf at B)
far orbit gulf (L4/L5)   = v_inf at 1 au + excess(B, v_inf at B)   # no well to burn in
```

`excess` is the same whether the burn leaves or captures, so each gulf is the same both ways for
a Hohmann transfer. Mars and Earth arrivals keep their aerobraked `arrive` figures (0.25, 0.54);
their excess is the same formula, as in delta-v.md.

### 2.4 Two ways of computing a crossing

1. **Hohmann between circular, coplanar orbits** at the semimajor axes of section 4 (the
   method behind every gulf already in the game).
2. **Real windows:** a Lambert solver (universal variables, single revolution, prograde) run on
   heliocentric state vectors from JPL Horizons (ecliptic J2000, DE441 and SB441-N16, 5-day steps,
   2025 - 2080), searching departure dates every 10 days (5 days for Mercury) from 2030 to 2050
   and flight times from 0.35 to 1.9 times the Hohmann time. For each departure date the
   cheapest flight time is kept; a "window" is a departure date cheaper than every other within
   0.4 synodic periods. Cost is the gulf of section 2.3, so it is directly comparable with
   column 1. Ballistic only: no deep-space manoeuvre, no gravity assist, no plane-change
   optimisation, so it is an upper bound on what a careful mission design would find.

**Validation of method 2:**

- Earth -> Mars, 2030 - 2050: gulf 1.04 - 1.72, median 1.45, departure v_inf 3.0 - 4.2.
  delta-v.md section 3.7 gives departure burns from NASA/TM-2010-216764 of 3.55 - 3.86 from
  LEO, which is v_inf 2.9 - 3.6. Consistent; and the coplanar 1.07 sits at the bottom of the real
  range, as it should.
- Earth -> Ceres against **JPL's SB Mission Design API** (mode M, ballistic Lambert on JPL's own
  ephemerides, <https://ssd-api.jpl.nasa.gov/mdesign.api?des=1&mjd0=62502&span=5475&tof-min=150&tof-max=1500&step=10>):

  | Window | JPL departure, flight, v_inf out / in | This note |
  |---|---|---|
  | 2030 | 2030-06-10, 460 d, 10.43 / 5.55 | 2030-06-09, 465 d, 10.47 / 5.55 |
  | 2034 | 2034-06-09, 540 d, 6.81 / 5.41 | 2034-06-08, 545 d, 6.88 / 5.38 |

  Agreement to 0.07 km/s and one grid step.

---

## 3. Travel

### 3.1 Leave and arrive, computed

```
Mercury, r = 2439.4 + 300 = 2739.4 km:  v_circ 2.836  v_esc 4.011   leave = arrive = 1.175
Ceres,   r = 469.7 + 385  =  854.7 km:  v_circ 0.271  v_esc 0.383   leave = arrive = 0.112
Vesta,   r = 261.4 + 210  =  471.4 km:  v_circ 0.192  v_esc 0.271   leave = arrive = 0.079
```

For context only (delta-v.md prices orbit to orbit; landings are not in it): the circular speed
at the surface, roughly what a landing from low orbit costs before gravity losses, is Mercury
3.01, Ceres 0.37, Vesta 0.26 km/s; escape from the surface is 4.25, 0.52, 0.36.

Phobos and Deimos, the game's other small worlds, leave for 0.94 and 0.70 because a ship from
them burns low over Mars; Ceres and Vesta have no parent, so they are cheaper still.

### 3.2 Gulfs, coplanar Hohmann (computed)

Semimajor axes: Mercury 0.38709927, Venus 0.72333566, Earth 1.00000261, Mars 1.52371034 au (JPL
table, section 4); Ceres 2.7671648, Vesta 2.3615074 au (the fitted mean elements of section 4).

| Pair | v_inf at first | v_inf at second | excess at first | excess at second | **Gulf** | Days |
|---|---|---|---|---|---|---|
| Earth - Mercury | 7.533 | 9.611 | 2.345 | 6.404 | **8.75** | 105.5 |
| Earth - Ceres | 6.316 | 4.859 | 1.694 | 4.491 | **6.19** | 472.1 |
| Earth - Vesta | 5.520 | 4.432 | 1.315 | 4.169 | **5.48** | 397.9 |
| Mars - Ceres | 3.274 | 2.816 | 1.008 | 2.459 | **3.47** | 573.9 |
| Mars - Vesta | 2.475 | 2.216 | 0.599 | 1.962 | **2.56** | 494.5 |
| Mars - Mercury | 8.770 | 12.584 | 5.191 | 9.197 | **14.39** | 170.5 |
| Venus - Mercury | 5.779 | 6.769 | 1.535 | 3.857 | **5.39** | 75.6 |
| Ceres - Vesta | 0.723 | 0.752 | 0.435 | 0.528 | **0.96** | 749.9 |
| Venus - Ceres | 9.077 | 6.378 | 3.476 | 6.007 | 9.48 | 421.1 |
| Venus - Vesta | 8.312 | 6.109 | 2.977 | 5.844 | 8.82 | 349.8 |
| *Check: Earth - Mars* | *2.945* | *2.649* | *0.390* | *0.681* | *1.07 (game: 1.07)* | *258.9* |
| *Check: Earth - Venus* | *2.495* | *2.707* | *0.281* | *0.356* | *0.64 (game: 0.64)* | *146.1* |
| *Check: Venus - Mars* | *5.763* | *4.768* | *1.527* | *1.962* | *3.49 (game: 3.48)* | *217.5* |

Mercury is the dear one: a ship must shed most of Earth's orbital speed to fall that far toward
the Sun, and arrives 9.6 km/s faster than Mercury with no air to brake on. **LEO -> low Mercury
orbit is 13.1 km/s, more than LEO -> Ceres (9.5).** Mars -> Mercury direct (17.0) costs more
than stopping in low Earth orbit on the way (3.02 + 13.12 = 16.1), because Earth's air takes
the arrival; Venus -> Mercury (9.5) is the cheapest way in. Every return to LEO here uses
delta-v.md's Earth-arrival assumption (aerobraking into LEO, unflown; arrive 0.54).

**Secondary cross-check:** Wikipedia "Delta-v budget" (Hohmann table, 300 km LEO) gives
"Δv from LEO" to Mercury **5.5** and transit time 3.5 months; this note's departure burn is
3.20 + 2.345 = **5.55**, and 105.5 days. Wikipedia lists no figure for Ceres or Vesta.
<https://en.wikipedia.org/w/index.php?title=Delta-v_budget&action=raw> (read raw).

### 3.3 Real windows 2030 - 2050 (computed, ballistic)

Gulf in km/s for each window found; v_inf at each end in the full output. Best window shown.

| Leg | Windows | Best | Median | Worst | Best window: depart, flight, v_inf out / in |
|---|---|---|---|---|---|
| Earth -> Mercury | 63 | 8.60 | 11.65 | 14.53 | flight 95 d, 9.44 / 8.16 |
| Mercury -> Earth | 63 | 8.49 | 11.81 | 14.20 | flight 100 d, 8.13 / 9.32 |
| Earth -> Ceres | 16 | 6.65 | 9.03 | 10.81 | 2045-12-17, 435 d, 6.81 / 5.07 |
| Ceres -> Earth | 9 | 6.67 | 7.69 | 9.70 | 2042-08-05, 495 d, 5.20 / 6.61 |
| Earth -> Vesta | 15 | 5.64 | 7.28 | 8.52 | 2037-06-27, 455 d, 6.57 / 4.08 |
| Vesta -> Earth | 15 | 5.48 | 7.46 | 8.29 | 2031-05-20, 415 d, 3.99 / 6.44 |
| Mars -> Ceres | 6 | 4.30 | 5.20 | 6.43 | 2038-12-29, 630 d, 3.45 / 3.55 |
| Ceres -> Mars | 6 | 4.33 | 5.25 | 6.43 | 2037-12-24, 760 d, 3.25 / 3.99 |
| Mars -> Vesta | 5 | 3.00 | 3.33 | 3.87 | 2041-01-12, 580 d, 2.76 / 2.52 |
| Vesta -> Mars | 5 | 3.02 | 3.32 | 4.21 | 2036-09-25, 570 d, 2.42 / 3.00 |
| Venus -> Mercury | 50 | 5.49 | 7.03 | 8.98 | flight 65 d, 7.40 / 5.83 |
| Mercury -> Venus | 50 | 5.56 | 7.18 | 9.22 | flight 80 d, 6.00 / 7.28 |
| Mars -> Mercury | 62 | 13.83 | 16.88 | 19.76 | flight 155 d |
| Mercury -> Mars | 73 | 14.14 | 16.92 | 19.08 | flight 190 d |
| Ceres -> Vesta | 1 | 1.98 | - | - | 2046-03-02, 1060 d, 0.88 / 1.65 |
| Vesta -> Ceres | 1 | 1.94 | - | - | 2046-06-10, 1070 d, 0.78 / 1.72 |

Why the real figures are higher, and so uneven:

- **Inclination.** Relative to Earth's orbit Ceres is inclined 10.6 degrees, Vesta 7.1, Mercury
  7.0 (section 4). A window whose arrival falls near the target's node is close to the coplanar
  cost; one that falls far from it pays a plane change at full orbital speed. That is why
  Earth -> Ceres alternates between about 7 and about 10 from one window to the next. Mutual
  inclinations, computed from the section 4 elements: Mars - Ceres 9.1, Mars - Vesta 6.2,
  **Ceres - Vesta 4.9** degrees, which is why Ceres - Vesta is 1.9 against the coplanar 0.96.
- **Eccentricity** helps or hurts by window. Mercury (e = 0.206) is sometimes cheaper than the
  circular figure (best 8.5 against 8.75) and usually dearer.
- **Ballistic only.** Real missions cut the bad windows with a deep-space manoeuvre or a
  flyby, so the "worst" column overstates what a planner would pay.

**Ceres - Vesta has one window in roughly 17 years** (synodic period 6,263 days). In the span
searched (departures 2031 - 2059) the only one is 2046.

### 3.4 What flown missions did (context, not a price)

- **MESSENGER** did not fly a direct transfer. It "relied on six planetary gravity-assist
  flybys to impart >91% of the trajectory's total velocity change", and its propulsion system
  "provided >2213 m/s of propulsive DV" over the whole mission (McAdams et al., *Johns Hopkins
  APL Technical Digest* 34(1), <https://jhuapl.edu/sites/default/files/2024-09/34-01-McAdams.pdf>).
  Its orbit insertion slowed it "by just over 0.86 kilometers ... per second" into an eccentric
  orbit and used "about 31% of the spacecraft's original allotment of propellant"
  (<https://messenger.jhuapl.edu/About/stationkeeping.html>). Launch to orbit took from August
  2004 to March 2011 (<https://messenger.jhuapl.edu/About/Mission-Design.html>). The direct
  figures here are what a ship pays to skip six years of flybys.
- **Dawn** reached Vesta and then Ceres on ion propulsion, a low-thrust spiral not comparable
  with impulsive figures: "total effective velocity change by a spacecraft of 25,700 mph
  (41,360 kph)", 11.49 km/s, over 5.87 years of firing (JPL news, 28 June 2018,
  <https://www.jpl.nasa.gov/news/dawns-engines-complete-firing-science-continues/>).

---

## 4. Orbital elements for the Solar System Map

Same form and units as `[[planet]]` in `assets/data/ephemeris.toml`: a in au, angles in degrees,
every rate per Julian century, J2000, ecliptic and equinox of J2000.

### 4.1 Mercury: JPL's table (the one `ephemeris.toml` already cites)

From <https://ssd.jpl.nasa.gov/planets/approx_pos.html>, Table 1 (1800 AD - 2050 AD), fetched:

```toml
[[planet]]
id = "mercury"
a = 0.38709927
a_rate = 0.00000037
e = 0.20563593
e_rate = 0.00001906
inclination = 7.00497902
inclination_rate = -0.00594749
mean_longitude = 252.25032350
mean_longitude_rate = 149472.67411175
perihelion_longitude = 77.45779628
perihelion_longitude_rate = 0.16047689
node_longitude = 48.33076593
node_longitude_rate = -0.12534081
```

### 4.2 Ceres and Vesta: fitted to JPL Horizons the same way

The JPL table carries no asteroids, and JPL publishes no mean-element table with rates for them;
the SBDB gives only osculating elements at one epoch. So the elements below were **computed**
here the way Standish made the planetary table: osculating heliocentric elements from Horizons
(ecliptic J2000; Ceres solution JPL#48, Vesta its SBDB orbit; DE441 with SB441-N16 perturbers),
sampled every 30 days from 1950 to 2100, and a straight line fitted to each of a, e, i, mean
longitude L, longitude of perihelion and node, centred on J2000.

**Validation of the method:** the same fit run on Mercury (Horizons body 199, 1950 - 2100,
10-day samples) returns a = 0.38709830, e = 0.20563165, i = 7.00497500, L = 252.25086,
perihelion 77.45609, node 48.33089, L rate 149472.67470: the JPL table to within 1e-6 au,
5e-6 in e, 0.0017 degrees in the angles.

```toml
[[planet]]
id = "ceres"
a = 2.76716476
a_rate = -0.00016363
e = 0.07762174
e_rate = -0.00005100
inclination = 10.59640602
inclination_rate = -0.02066984
mean_longitude = 160.78432733
mean_longitude_rate = 7819.40767576
perihelion_longitude = 153.11732409
perihelion_longitude_rate = 1.75978337
node_longitude = 80.55522405
node_longitude_rate = -1.50565775

[[planet]]
id = "vesta"
a = 2.36150737
a_rate = 0.00000427
e = 0.08934118
e_rate = 0.00032909
inclination = 7.13714420
inclination_rate = 0.00564144
mean_longitude = 234.43394156
mean_longitude_rate = 9918.85851311
perihelion_longitude = 254.24732846
perihelion_longitude_rate = 1.25007430
node_longitude = 103.96163686
node_longitude_rate = -0.90018449
```

**How good they are:** heliocentric longitude from these elements, against Horizons' state
vectors:

| Date | Ceres error | Vesta error |
|---|---|---|
| 2000-01-01 | 0.44 deg (0.020 au) | 0.25 deg (0.010 au) |
| 2030-01-01 | 0.02 deg (0.003 au) | 0.08 deg (0.004 au) |
| 2050-01-01 | 0.15 deg (0.008 au) | 0.16 deg (0.007 au) |
| 2080-01-01 | 0.20 deg (0.010 au) | 0.12 deg (0.005 au) |
| 2100-01-01 | 0.30 deg (0.015 au) | 0.04 deg (0.002 au) |

Under half a degree throughout: Jupiter's pull makes a and the perihelion wobble around the
fitted line (largest residuals: Ceres 0.004 au in a and 1.9 deg in perihelion longitude; Vesta
0.002 au and 0.9 deg), so these cannot be as tight as the planets' rows. For a map, and for
launch-window timing at 60-day turns, half a degree is about two days of the target's motion.

For reference, the osculating elements at J2000 itself (Horizons): Ceres a 2.76650, e 0.07838,
i 10.5834, node 80.4944, argument of perihelion 73.9229; the fitted mean values differ from
them by the size of the wobble.

Orbit extremes: Mercury perihelion 0.3075 / aphelion 0.4667 au; Ceres 2.550 / 2.983 au
(Horizons at J2000); Vesta 2.151 / 2.572 au (from the fitted a and e).

---

## 5. The worlds

### 5.1 Gravity, and the game's line

Computed as GM / R^2 with the mean radius (no rotation):

```
Mercury  22031.868551 / 2439.4^2 = 3.702 m/s^2     JPL equatorial figure 3.70
Ceres       62.6284   /  469.7^2 = 0.284 m/s^2     JPL equatorial figure 0.27 (larger equator, spin)
Vesta       17.2882844/ 261.385^2 = 0.253 m/s^2    no JPL table figure; computed only
Moon      4902.800118 / 1737.4^2 = 1.624 m/s^2
Mars     42828.375816 / 3389.50^2 = 3.728 m/s^2     JPL equatorial figure 3.71
```

The game has no number for "low gravity": `low_gravity = true` is set on the Moon, Phobos and
Deimos and not on Mars or Earth (`assets/data/bodies.toml`). By that line **Ceres and Vesta are
low gravity** (a sixth of the Moon's), and **Mercury is not** (Mars's gravity to within 1%).

### 5.2 Sunlight

The game scales Solar output by the inverse square of distance. At mean distance: **Mercury
6.67, Vesta 0.179, Ceres 0.131** times Earth's. Over the orbit: Mercury 4.6 - 10.6, Vesta
0.15 - 0.22, Ceres 0.11 - 0.15. NASA's own phrasing for Mercury is "as much as seven times
brighter" (<https://science.nasa.gov/mercury/facts/>).

Mercury's day is long: "One Mercury solar day ... equals 176 Earth days", with surface
temperatures from 430 C by day to -180 C by night (same page). Ceres turns in 9.07 h, Vesta in
5.34 h (SBDB).

### 5.3 Ground

All three have solid ground and no air. Ceres and Vesta are small enough that a landing costs
a few hundred metres per second (section 3.1); Mercury's costs about 3 km/s.

---

## 6. Colony Slot candidates

Coordinates are from the IAU **Gazetteer of Planetary Nomenclature** (USGS), downloaded as the
official point shapefiles, `https://asc-planetarynames-data.s3.us-west-2.amazonaws.com/{CERES,VESTA,MERCURY}_nomenclature_center_pts.zip`
(listed on <https://planetarynames.wr.usgs.gov/GIS_Downloads>). The metadata says the data are
"Public domain" and the extents use "a positive East longitude system". Centre longitudes in the
files run 0 - 360 east; below they are converted to the game's **-180 to 180, east positive**
(`bodies.toml`). Check of the Mercury convention: Hun Kal, the crater that defines Mercury's
20 degree W meridian, sits at 339.99 in the file.

**Vesta's longitudes are in the IAU "Claudia Double-Prime" system** (crater Claudia at 146 E).
The Gazetteer gives Claudia at 146.0 E, so it uses that system. The Dawn team's own papers and
quadrangle maps used the older "Dawn-Claudia" system, 150 degrees different (PDS document
*Body-Fixed Coordinate Systems for Asteroid (4) Vesta*,
<https://sbnarchive.psi.edu/pds3/dawn/grav/DWNVGRS_2/DOCUMENT/VESTA_COORDINATES_131018.HTM>:
"All of the team's mapping products use this Dawn-Claudia coordinate system"; Claudia
Double-Prime "was selected for the PDS Dawn archive"). **Coordinates and map must be in the same
system**; the USGS mosaic in section 7 is.

"Why" notes cite only what a fetched source says. Where a paper's abstract does not name the
feature, that is marked *(not named in the text read)*.

### 6.1 Ceres (eight)

| Name | Lat | Lon (game) | Type, size | Why |
|---|---|---|---|---|
| Occator | 19.82 | -120.67 | Crater, 92 km | The bright areas: "a large amount of sodium carbonate, ... the most concentrated known extraterrestrial occurrence of carbonate" (De Sanctis 2016); over a brine reservoir "about 25 miles (40 kilometers) deep and hundreds of miles wide" (JPL 2020) |
| Ahuna Mons | -10.48 | -43.80 | Mountain, 20 km | A "viscous cryovolcanic dome", extruded recently (Ruesch 2016) *(not named in the text read)* |
| Ernutet | 52.93 | 45.52 | Crater, 53 km | Aliphatic organic matter "mainly localized on a broad region of ~1000 square kilometers close to the ~50-kilometer Ernutet crater" (De Sanctis 2017) |
| Oxo | 42.21 | -0.40 | Crater, 10 km | Water "detected ... within ... Oxo, a 10-kilometer, geologically fresh crater" (Combe 2016) |
| Kerwan | -10.77 | 123.99 | Crater, 280 km | Largest named crater (Gazetteer) |
| Yalode | -42.58 | -67.52 | Crater, 260 km | Second largest (Gazetteer) |
| Urvara | -45.66 | -110.76 | Crater, 170 km | Large basin (Gazetteer) |
| Haulani | 5.80 | 10.77 | Crater, 34 km | Size only (Gazetteer) |

Alternates: Dantu (24.30, 138.23; 126 km), Hanami Planum (15.0, -130.0; the 555 km plateau
around Occator), Juling (-35.90, 168.48; 20 km), Vendimia Planitia (23.0, 135.0; 750 km plain).

### 6.2 Vesta (eight)

| Name | Lat | Lon (game) | Type, size | Why |
|---|---|---|---|---|
| Rheasilvia | -71.95 | 86.30 | Crater, 450 km | The south-pole basin; "a higher diogenitic component" (De Sanctis 2012); youngest giant impact, "about 500 kilometers across, formed about 1 billion years ago" (Marchi 2012); lowest hydrogen (Prettyman 2012) |
| Veneneia | -47.93 | -54.32 | Crater, 400 km | The older of the two southern basins (Marchi 2012: "two major collisions") *(not named in the text read)* |
| Divalia Fossae | -9.05 | -163.77 | Troughs, 549 km | The equatorial trough belt (Gazetteer) |
| Marcia | 8.98 | -20.45 | Crater, 68 km | Pitted terrain, "a relatively large volatile component" (Denevi 2012) *(not named in the text read)* |
| Arruntia | 39.44 | -138.41 | Crater, 10 km | Olivine: JPL image PIA17476, "Two Craters with Olivine" (Arruntia and Bellicia); olivine-rich "(more than 50 per cent by volume)" in the north (Ammannito 2013) |
| Bellicia | 37.73 | -162.24 | Crater, 42 km | The other olivine crater (same sources) |
| Feralia Planitia | 3.03 | 101.71 | Plain, 270 km | Equatorial plain (Gazetteer) |
| Vestalia Terra | -3.73 | 33.47 | Terra, 336 km | Equatorial highland (Gazetteer) |

Alternates: Saturnalia Fossae (28.05, 37.05; 345 km), Caesonia (31.20, -110.07; 104 km),
Aricia Tholus (13.38, -48.45; 39 km), Postumia (33.84, 33.77; 196 km).

**Map caution:** Vesta's far north was in seasonal shadow when Dawn mapped it; on the USGS
mosaic's 1,024-pixel preview (looked at this session) the top strip is dark and one block near
the north pole is blank. Slots north of about 45 N would sit on poor map. All eight above are
south of 40 N.

### 6.3 Mercury (eight)

| Name | Lat | Lon (game) | Type, size | Why |
|---|---|---|---|---|
| Caloris Planitia | 31.65 | 161.98 | Basin floor, 1,500 km | The largest basin; the northern plains "formed after the Caloris impact basin" (Head 2011) |
| Borealis Planitia | 67.30 | 32.60 | Plain, 3,450 km | The northern smooth plains, "more than 6% of the planet's surface", flood lavas (Head 2011) *(not named in the text read)* |
| Prokofiev | 85.77 | 62.92 | Crater, 112 km | "The largest crater in Mercury's north polar region found to host radar-bright material", imaged as surface water ice (APL news release, 15 Oct 2014) |
| Rachmaninoff | 27.66 | 57.37 | Crater, 305 km | Peak-ring basin (Gazetteer) |
| Raditladi | 27.15 | 119.06 | Crater, 258 km | Peak-ring basin (Gazetteer) |
| Rembrandt | -32.89 | 87.87 | Crater, 716 km | Large southern basin (Gazetteer) |
| Beethoven | -20.86 | -124.21 | Crater, 630 km | Large basin (Gazetteer) |
| Kuiper | -11.34 | -31.32 | Crater, 62 km | Size only (Gazetteer) |

Alternates: Tolstoj (-16.23, -164.64; 355 km), Hokusai (57.84, 16.65; 114 km), Chao Meng-Fu
(-88.42, -156.36; 141 km, near the south pole), Kandinsky (87.89, 78.78; 60 km, near the north
pole), Odin Planitia (23.60, -169.86; 473 km).

---

## 7. Surface maps

All three are **simple cylindrical (equirectangular), planetocentric latitude, east-positive
longitude from -180 to 180**, the orientation the game already uses (`examples/prep_moons.rs`:
"180 W at the left edge and 0 in the middle"). All need downsampling to the game's 1024 x 512.

| World | Product | Resolution | Pixels | Size | Download |
|---|---|---|---|---|---|
| Ceres | **Ceres Dawn FC Global Mosaic 140m** (DLR, Feb 2016), clear filter, greyscale | 140 m/px, 58.59 px/deg | 21,093 x 10,546 | 214 MB | <https://astrogeology.usgs.gov/search/map/ceres_dawn_fc_global_mosaic_140m>; 1,024-px JPEG preview on the same page |
| Vesta | **Vesta Dawn FC HAMO Global Mosaic 60m** (Dec 2013), clear filter, greyscale; "shifted 150 degrees eastward (such that Claudia crater is located at 146E)" | 60 m/px, 74.18 px/deg | 26,704 x 13,352 | 341 MB GeoTIFF | <https://planetarymaps.usgs.gov/mosaic/Vesta_Dawn_FC_HAMO_Mosaic_Global_74ppd.tif> |
| Mercury | **Mercury MESSENGER MDIS Global Color Mosaic 665m**, enhanced colour (1000 / 750 / 430 nm as R / G / B; not true colour) | 665 m/px, 64 px/deg | 23,054 x 11,527 | 772 MB GeoTIFF | <https://planetarymaps.usgs.gov/mosaic/Mercury_MESSENGER_ClrMosaic_global_665m_v3.tif> |
| Mercury | **Mercury MESSENGER MDIS Global Basemap BDR 166m**, monochrome | 166 m/px, 256 px/deg | 92,160 x 46,080 | 4 GB | <https://planetarymaps.usgs.gov/mosaic/Mercury_MESSENGER_MDIS_Basemap_BDR_Mosaic_Global_166m.tif> |
| Mercury | **PIA17386, Enhanced Color Mercury Map** (NASA Photojournal, 2013), simple cylindrical, centred on 180 E | 3.74 km/px | 4,096 x 2,048 | 24 MB TIFF, 1.75 MB JPEG | <https://science.nasa.gov/photojournal/enhanced-color-mercury-map> |

Product pages fetched: the three `astrogeology.usgs.gov/search/map/...` pages named above and
<https://astrogeology.usgs.gov/search/map/mercury_messenger_mdis_global_basemap_bdr_166m>. The
Ceres and Vesta previews were downloaded and looked at: Occator's bright spot falls at about
-122 longitude on the Ceres preview, where the Gazetteer puts it (-120.7), confirming the two
agree.

**PIA17386 is centred on 180 E**, not on 0: it would need its halves swapped before the game's
loader reads it. The USGS products are already centred on 0.

### 7.1 Licence: what each source says

| Source | What it says |
|---|---|
| USGS product pages (all four) | Use constraints: "Please cite authors". No licence named |
| USGS policy | "USGS-authored or produced data and information are considered to be in the U.S. Public Domain" (<https://www.usgs.gov/information-policies-and-instructions/copyrights-and-credits>). The Dawn mosaics are authored by "NASA/JPL-Caltech/UCLA/MPS/DLR/IDA"; the MESSENGER mosaics by the MESSENGER Team / Applied Coherent Technology. **They are distributed by USGS, not authored by it** |
| NASA guidelines | "NASA content - images, audio, video, and media files used in the rendition of 3-dimensional models, such as texture maps and polygon data in any format - generally are not subject to copyright in the United States." Commercial use must not "convey NASA's endorsement" (<https://www.nasa.gov/nasa-brand-center/images-and-media/>) |
| JPL image policy | JPL-site images "may be used for any purpose without prior permission"; for images owned by others, "restrictions are placed on commercial uses" (<https://www.jpl.nasa.gov/jpl-image-use-policy/>) |
| MESSENGER usage statement | "MESSENGER images are generally available for non-commercial educational and public information purposes ... No fee or written permission is required for their use, but please credit images to NASA/Johns Hopkins University Applied Physics Laboratory/Carnegie Institution of Washington" (<https://messenger.jhuapl.edu/Explore/Usage-Statement.html>) |
| Gazetteer (names, coordinates) | "Public domain" (shapefile metadata) |

Credit lines to carry: Ceres and Vesta "NASA/JPL-Caltech/UCLA/MPS/DLR/IDA"; Mercury
"NASA/Johns Hopkins University Applied Physics Laboratory/Carnegie Institution of Washington".

The Phobos and Deimos maps the game ships came from USGS and Stooke and were taken as public
domain (`examples/prep_moons.rs`). These three are less clear-cut: they are NASA mission data
(NASA says texture maps are generally not under US copyright), but made with foreign partners
(DLR and MPS in Germany built and processed Dawn's camera images), and the MESSENGER team's
statement names non-commercial use. **This note does not settle whether that permits shipping
them in the game.**

---

## 8. What each world holds (for yields)

Findings only; how they map to Materials, Energy, Fuel and Research is the designer's call.

### 8.1 Ceres: water, salts, clays, organics; little iron

- **Water ice under the surface at mid to high latitudes:** "At mid-to-high latitudes, the
  regolith contains high concentrations of hydrogen, consistent with broad expanses of water
  ice" (Prettyman et al. 2017, *Science* 355:55, doi 10.1126/science.aah6765). Exposed water
  at Oxo (Combe et al. 2016, *Science* 353:aaf3010, doi 10.1126/science.aaf3010).
- **Low iron:** "the concentration of iron on Ceres is lower than in" aqueously altered
  carbonaceous chondrites (Prettyman 2017).
- **Clays with ammonia:** "widespread ammoniated phyllosilicates across the surface" (De Sanctis
  et al. 2015, *Nature* 528:241, doi 10.1038/nature16172).
- **Salts and carbonates** at Occator: sodium carbonate, "ammonium carbonate or ammonium
  chloride" (De Sanctis et al. 2016, *Nature* 536:54, doi 10.1038/nature18290). Brine
  reservoir about 40 km deep; deposits "less than 2 million years old" and activity that "could
  be ongoing" (JPL, 10 Aug 2020, <https://www.jpl.nasa.gov/news/mystery-solved-bright-areas-on-ceres-come-from-salty-water-below/>;
  paper: Raymond et al. 2020, *Nature Astronomy* 4:741, doi 10.1038/s41550-020-1168-2, abstract
  not reached).
- **Organics:** aliphatic organic matter near Ernutet; "a very complex chemical environment,
  suggesting favorable environments to prebiotic chemistry" (De Sanctis et al. 2017, *Science*
  355:719, doi 10.1126/science.aaj2305).
- **Interior:** "a rocky core overlaid by a volatile-rich shell", shell 70 - 190 km thick of
  density 1,680 - 1,950 kg/m^3, "a mixture of volatiles and denser materials such as silicates
  and salts" (Park et al. 2016, *Nature* 537:515, doi 10.1038/nature18955).
- **Sunlight:** 0.13 of Earth's.

### 8.2 Vesta: basalt and pyroxene, an iron core, a little hydrogen

- **Basaltic crust, the HED meteorites' parent:** "The spatially resolved mineralogy of the
  surface reflects the composition of the HED meteorites"; "a core having an average radius of
  107 to 113 kilometers, indicating sufficient internal melting to segregate iron" (Russell et
  al. 2012, *Science* 336:684, doi 10.1126/science.1219381). Upper eucritic (basalt) crust,
  deeper diogenitic (pyroxene) crust exposed in Rheasilvia (De Sanctis et al. 2012, *Science*
  336:697, doi 10.1126/science.1219270).
- **Olivine** locally, "more than 50 per cent by volume", in the northern hemisphere (Ammannito
  et al. 2013, *Nature* 504:122, doi 10.1038/nature12665).
- **Hydrogen, but not ice:** "Vesta's regolith contains substantial amounts of hydrogen. The
  highest hydrogen concentrations coincide with older, low-albedo regions near the equator,
  where water ice is unstable", brought by infalling carbonaceous chondrites (Prettyman et al.
  2012, *Science* 338:242, doi 10.1126/science.1225354). Dark material "mainly from infall of
  hydrated carbonaceous material" (McCord et al. 2012, *Nature* 491:83, doi 10.1038/nature11561).
  Pitted terrain suggests "portions of the surface contain a relatively large volatile
  component" (Denevi et al. 2012, *Science* 338:246, doi 10.1126/science.1225374).
- **Bulk density 3.46 g/cm^3** (SBDB, Park 2025), against Ceres 2.16: rock, not ice.
- **Sunlight:** 0.18 of Earth's.

### 8.3 Mercury: sunlight, a great metal core, sulphur and carbon, polar ice

- **Polar ice:** water ice in permanently shadowed regions near the north pole, a buried layer
  of "nearly pure water ice"; "The total mass of water at Mercury's poles is inferred to be
  2 x 10^16 to 10^18 grams" (Lawrence et al. 2013, *Science* 339:292, doi 10.1126/science.1229953).
  Bright deposits consistent with surface ice, dark ones with "a surface layer of complex organic
  material that likely overlies buried ice" (Neumann et al. 2013, *Science* 339:296, doi
  10.1126/science.1229764).
- **Metal core:** "a large metallic core with a radius of about ... 2,074 kilometers, about 85%
  of the planet's radius" (<https://science.nasa.gov/mercury/facts/>); interior model of "a solid
  iron-sulfide layer and an iron-rich liquid outer core" (Smith et al. 2012, *Science* 336:214,
  doi 10.1126/science.1218809).
- **Surface low in iron, rich in sulphur:** "The sulfur abundance is at least 10 times higher than
  that of the silicate portion of Earth or the Moon, ... together with a low surface Fe abundance"
  (Nittler et al. 2011, *Science* 333:1847, doi 10.1126/science.1211567).
- **Carbon:** "an ancient carbon-bearing crust" (Peplowski et al. 2016, *Nature Geoscience*
  9:273, doi 10.1038/ngeo2669; title verified through Crossref, abstract not reached).
- **Volatiles:** hollows indicating "recent loss of volatiles" (Blewett et al. 2011, *Science*
  333:1856, doi 10.1126/science.1211681); potassium 1,150 ppm, thorium 220 ppb, uranium 90 ppb
  (Peplowski et al. 2011, *Science* 333:1850, doi 10.1126/science.1211576).
- **Sunlight:** 6.7 of Earth's on average, up to 10.6 at perihelion; a solar day of 176 Earth
  days.

---

## 9. What is uncertain

1. **Every travel figure is computed; none is published.** The two independent checks (JPL's
   mission-design tool for Earth -> Ceres; Wikipedia's Mercury departure) agree. Trust the
   coplanar gulfs as the game's own convention; trust the real-window ranges to about +/- 0.2
   (10-day grid, 5-day for Mercury).
2. **Coplanar versus real is the big choice.** For the game's existing gulfs the two differ
   little (Earth-Mars 1.07 coplanar, real median 1.45). For the new worlds they differ a lot:
   Earth-Ceres 6.19 coplanar, real median 8.5; Ceres-Vesta 0.96 against 1.9. That is the
   inclination, not an error.
3. **The real-window search is ballistic.** With deep-space manoeuvres the bad windows would come
   down; the best ones would barely move.
4. **Ceres and Vesta elements are a fit, not a published table.** Good to half a degree from 2000
   to 2100 (section 4.2); the method reproduces JPL's Mercury row.
5. **Map licences** (section 7.1): unresolved for a shipped game, especially Mercury.
6. **Vesta's coordinate system:** the Gazetteer and the USGS mosaic agree (Claudia
   Double-Prime); most Dawn papers use Dawn-Claudia, 150 degrees off. Any coordinate taken from
   a paper must be converted.
7. **Vesta's gravity** has no JPL table figure; it is computed from SBDB's GM and diameter
   (Park et al. 2025), not read.
8. **Not reached:** the abstracts of Raymond et al. 2020 and Peplowski et al. 2016 (Nature pages
   redirect to a login); Chabot et al. 2014 (*Geology*) PDF was damaged on download, so the
   Prokofiev ice claim rests on the APL news release.
9. **Feature notes marked *(not named in the text read)***: Ahuna Mons as Ruesch's dome,
   Veneneia as Marchi's older basin, Marcia as Denevi's pitted terrain, Borealis Planitia as
   Head's northern plains. The coordinates are sound; only the attribution is unverified.

---

## References

| URL | What was taken from it |
|---|---|
| <https://ssd.jpl.nasa.gov/planets/approx_pos.html> | Mercury's elements and rates, Table 1. Fetched with curl. |
| <https://ssd.jpl.nasa.gov/astro_par.html> | GM Mercury 22,031.868551. Fetched with curl. |
| <https://ssd.jpl.nasa.gov/planets/phys_par.html> | Mercury radius 2,440.53 / 2,439.4 km, gravity 3.70, escape 4.25, rotation 58.6462 d; Ceres radius 482.1 / 469.7 km, gravity 0.27, escape 0.51. Fetched with curl. |
| <https://ssd-api.jpl.nasa.gov/sbdb.api?sstr=1&phys-par=1>, `sstr=4` | Ceres GM 62.6284, diameter 939.4, density 2.162, rotation 9.074170 h (Park et al. 2016); Vesta GM 17.2882844, diameter 522.77, density 3.460, rotation 5.3421276 h (Park et al. 2025). |
| <https://ssd.jpl.nasa.gov/api/horizons.api> | Osculating elements 1950 - 2100 for Mercury (199), Ceres (`1;`), Vesta (`4;`); state vectors 2025 - 2080 for Mercury, Venus, Earth, Mars, Ceres, Vesta; ecliptic J2000. |
| <https://ssd-api.jpl.nasa.gov/mdesign.api?des=1&mjd0=62502&span=5475&tof-min=150&tof-max=1500&step=10> | JPL SB Mission Design, Earth -> Ceres ballistic map 2030 - 2045; used to validate this note's search. Documentation <https://ssd-api.jpl.nasa.gov/doc/mdesign.html>. |
| <https://www.jpl.nasa.gov/blog/2012/2/highs-and-lows-of-exploring-the-giant-asteroid> | Vesta LAMO, average altitude 210 km. |
| <https://www.jpl.nasa.gov/blog/2015/11/dawn-begins-descent-to-closest-and-final-orbit-at-ceres> | Ceres LAMO, 385 km, 610 mph. |
| <https://www.jpl.nasa.gov/news/dawns-engines-complete-firing-science-continues/> | Dawn's total 41,360 kph, 5.87 years of firing. |
| <https://jhuapl.edu/sites/default/files/2024-09/34-01-McAdams.pdf> | McAdams et al., MESSENGER trajectory: flybys >91% of velocity change; >2213 m/s propulsive. PDF text-extracted. |
| <https://messenger.jhuapl.edu/About/stationkeeping.html> | MESSENGER orbit insertion, just over 0.86 km/s, 31% of propellant; periapsis lowered to 200 km. |
| <https://messenger.jhuapl.edu/About/Mission-Design.html> | Flyby sequence; cruise manoeuvre table. |
| <https://en.wikipedia.org/w/index.php?title=Delta-v_budget&action=raw> | **Secondary cross-check only.** Mercury: 5.5 from LEO, 3.5 months. No Ceres or Vesta row. |
| <https://planetarynames.wr.usgs.gov/GIS_Downloads> and the three `..._nomenclature_center_pts.zip` files | Every feature name, centre latitude and longitude, diameter and type; "Public domain"; east-positive extents. Shapefile attribute tables parsed directly. |
| <https://planetarynames.wr.usgs.gov/TargetCoordinates> | Notes that small bodies follow a separate IAU system (no Ceres or Vesta row). |
| <https://sbnarchive.psi.edu/pds3/dawn/grav/DWNVGRS_2/DOCUMENT/VESTA_COORDINATES_131018.HTM> | Vesta's three coordinate systems; Claudia at 356 / 136 / 146 E; Claudia Double-Prime chosen for PDS. Read through a summarising fetch. |
| <https://astrogeology.usgs.gov/search/map/ceres_dawn_fc_global_mosaic_140m> | Ceres mosaic: 140 m, 21,093 x 10,546, 214 MB, "Please cite authors", credit line. Read through a summarising fetch. |
| <https://astrogeology.usgs.gov/search/map/vesta_dawn_fc_hamo_global_mosaic_60m> | Vesta mosaic: 60 m, 26,704 x 13,352, 341 MB, 150 degree shift to Claudia Double-Prime. Read through a summarising fetch. |
| <https://astrogeology.usgs.gov/search/map/mercury_messenger_mdis_global_color_mosaic_665m> | Mercury colour mosaic: 665 m, 23,054 x 11,527, 772 MB, enhanced colour bands. Read through a summarising fetch. |
| <https://astrogeology.usgs.gov/search/map/mercury_messenger_mdis_global_basemap_bdr_166m> | Mercury BDR: 166 m, 92,160 x 46,080, 4 GB. Read through a summarising fetch. |
| <https://science.nasa.gov/photojournal/enhanced-color-mercury-map> | PIA17386: 4,096 x 2,048, 3.74 km/px, centred 180 E, credit line. Read through a summarising fetch. |
| <https://www.jpl.nasa.gov/images/pia17476-two-craters-with-olivine/> | Title "Two Craters with Olivine" (Arruntia, Bellicia), from a search result; page not opened. |
| <https://www.nasa.gov/nasa-brand-center/images-and-media/> | Texture maps "generally are not subject to copyright in the United States". |
| <https://www.jpl.nasa.gov/jpl-image-use-policy/> | "may be used for any purpose"; third-party images restricted for commercial use. |
| <https://www.usgs.gov/information-policies-and-instructions/copyrights-and-credits> | USGS-authored data public domain; third-party material excepted. |
| <https://messenger.jhuapl.edu/Explore/Usage-Statement.html> | "non-commercial educational and public information purposes"; credit line. Fetched with curl. |
| <https://www.jpl.nasa.gov/news/mystery-solved-bright-areas-on-ceres-come-from-salty-water-below/> | Brine reservoir about 40 km deep, hundreds of miles wide; deposits under 2 million years old. |
| <https://www.jhuapl.edu/news/news-releases/141015-messenger-provides-first-optical-images-ice-near-mercurys-north-pole> | Prokofiev, largest north-polar crater with radar-bright material, imaged ice. |
| <https://science.nasa.gov/mercury/facts/> | Temperatures, 176-day solar day, "seven times brighter", core 2,074 km. |
| Europe PMC REST API (<https://www.ebi.ac.uk/europepmc/webservices/rest/search>) and Crossref (<https://api.crossref.org/works/>) | Titles, journals, volumes, DOIs and abstracts of: Park 2016; De Sanctis 2015, 2016, 2017; Prettyman 2017; Combe 2016; Ruesch 2016; Russell 2012; De Sanctis 2012; Marchi 2012; McCord 2012; Denevi 2012; Prettyman 2012; Ammannito 2013; Lawrence 2013; Neumann 2013; Nittler 2011; Peplowski 2011; Smith 2012; Head 2011; Blewett 2011. Crossref only (no abstract): Peplowski 2016, Raymond 2020. |

### Sources that could not be reached, and things not verified

- Nature's article pages for Peplowski 2016 and Raymond 2020 redirect to a login.
- Chabot et al. 2014, *Geology*, from <https://lib.jhuapl.edu/>: the downloaded PDF was damaged.
- No published orbit-to-orbit delta-v table for Ceres, Vesta or Mercury was found; the community
  solar-system delta-v maps were not fetched (secondary, and none of the figures here depend on
  them).

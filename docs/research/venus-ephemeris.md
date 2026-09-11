# Venus ephemeris, Earth-Venus launch windows and every Body's distance from the Sun

**Date:** 2026-09-11
**Branch:** research/venus-ephemeris (research note only; no code or data changed)
**Ticket:** [#91](https://github.com/whaleyjoshua2/Dying-Earth/issues/91), for the Venus ticket
[#93](https://github.com/whaleyjoshua2/Dying-Earth/issues/93) and the Solar Array ticket on map
[#79](https://github.com/whaleyjoshua2/Dying-Earth/issues/79).

**What this is for:** version 0.06.0 adds Venus as a sixth Body. The sky the Solar System Map
draws (`assets/data/ephemeris.toml`) needs Venus's row in the same table Earth and Mars already
use; the Venus ticket needs the Earth-Venus flight in the game's two-month turns and which of the
36 turns (January 2030 to December 2035) stand at a real Earth-Venus launch window; and the Solar
Array ticket needs each Body's mean distance from the Sun and the factors that gives relative to
Earth. This note follows `docs/research/earth-mars-ephemeris.md` in shape and method. That note
was written when a turn was one month and the game 24 turns; the calendar has since changed
(version 0.05.5, section 1: thirty-six turns of two months, `days_per_turn = 60`, a turn stands
at the window when the phase angle crosses the Hohmann angle anywhere inside the turn), and
everything here is worked in the current calendar.

Everything below was fetched from a live source during this session, or derived by arithmetic
in this session from numbers so fetched, unless explicitly marked **UNVERIFIED**. Nothing is
stated from memory.

---

## 1. Venus's Keplerian elements (JPL, Standish)

**Source page:** <https://ssd.jpl.nasa.gov/planets/approx_pos.html> (fetched, HTTP 200).
**Linked PDF:** <https://ssd.jpl.nasa.gov/txt/aprx_pos_planets.pdf>.

### 1.1 Table 1, valid 1800 AD to 2050 AD, the table `ephemeris.toml` uses

JPL's header: "Keplerian elements and their rates, with respect to the mean ecliptic and equinox
of J2000, valid for the time-interval 1800 AD - 2050 AD." Column units as JPL prints them:
`a` in au and au/Cy, `e` dimensionless (JPL's unit row says "rad, rad/Cy", a formatting artefact
noted in the Earth-Mars note), the four angles in deg and deg/Cy. The rows for the three planets
the game will draw, reproduced exactly (first line of each pair = value at J2000, second = rate
per Julian century):

```
Venus     0.72333566      0.00677672      3.39467605      181.97909950    131.60246718     76.67984255
          0.00000390     -0.00004107     -0.00078890    58517.81538729      0.00268329     -0.27769418
EM Bary   1.00000261      0.01671123     -0.00001531      100.46457166    102.93768193      0.0
          0.00000562     -0.00004392     -0.01294668    35999.37244981      0.32327364      0.0
Mars      1.52371034      0.09339410      1.84969142       -4.55343205    -23.94362959     49.55953891
          0.00001847      0.00007882     -0.00813131    19140.30268499      0.44441088     -0.29257343
```

| Body | | a (au) | e | I (deg) | L (deg) | long.peri. varpi (deg) | long.node. Omega (deg) |
|---|---|---|---|---|---|---|---|
| **Venus** | J2000 | 0.72333566 | 0.00677672 | 3.39467605 | 181.97909950 | 131.60246718 | 76.67984255 |
| **Venus** | per Cy | 0.00000390 | -0.00004107 | -0.00078890 | 58517.81538729 | 0.00268329 | -0.27769418 |

The EM Bary and Mars rows match `assets/data/ephemeris.toml` on `main` digit for digit.

### 1.2 The row as `ephemeris.toml` would carry it

Same keys, same units, same shape as the `earth` and `mars` entries; it can be pasted beside
them with no conversion. (This note does not add it; the Venus ticket does.)

```toml
[[planet]]
id = "venus"
a = 0.72333566
a_rate = 0.00000390
e = 0.00677672
e_rate = -0.00004107
inclination = 3.39467605
inclination_rate = -0.00078890
mean_longitude = 181.97909950
mean_longitude_rate = 58517.81538729
perihelion_longitude = 131.60246718
perihelion_longitude_rate = 0.00268329
node_longitude = 76.67984255
node_longitude_rate = -0.27769418
```

Venus has no moons, so nothing reads its row the way the Moon reads Earth's and Phobos and
Deimos read Mars's.

### 1.3 Table 2a, for the record, not for the file

The same body from JPL's Table 2a (valid 3000 BC to 3000 AD), which must not be mixed with
Table 1. Venus is a planet of the inner set, so Table 2b's `b, c, s, f` terms (Jupiter through
Neptune only) do not apply to it either way.

| Body | | a (au) | e | I (deg) | L (deg) | varpi (deg) | Omega (deg) |
|---|---|---|---|---|---|---|---|
| Venus | J2000 | 0.72332102 | 0.00676399 | 3.39777545 | 181.97970850 | 131.76755713 | 76.67261496 |
| Venus | per Cy | -0.00000026 | -0.00005107 | 0.00043494 | 58517.81560260 | 0.05679648 | -0.27274174 |

### 1.4 JPL's stated accuracy for Venus

From the page's "Accuracy" table, nominal errors in heliocentric longitude, latitude and distance
(the raw HTML cells were checked, since the digits run together in a text dump):

| Body | 1800-2050: lambda (arcsec) | phi (arcsec) | rho (1000 km) | 3000 BC-3000 AD: lambda | phi | rho |
|---|---|---|---|---|---|---|
| **Venus** | 20 | 1 | 4 | 40 | 30 | 8 |
| EM Bary | 20 | 8 | 6 | 40 | 15 | 15 |
| Mars | 40 | 2 | 25 | 100 | 40 | 30 |

20 arcsec is 0.0056 degrees, three orders of magnitude inside the one-degree tolerance the
Earth-Mars tests use.

### 1.5 The row checked against a published position (the ticket's item 5)

The six-step JPL algorithm (transcribed in section 1.2 of the Earth-Mars note) was implemented
again in this session, evaluated at JD 2462502.5 (2030-01-01 00:00 TDB, T = 0.3 centuries
exactly), and differenced against the JPL Horizons vectors in section 2. The EM Bary line
reproduces the Earth-Mars note's figure exactly (100.1823 versus 100.1845), which is the
check that this implementation is the same one.

| Body | Standish longitude | Horizons longitude | error | Standish radius | Horizons radius |
|---|---|---|---|---|---|
| **Venus** | 96.8514 deg | 96.8553 deg | **-0.0039 deg** (14 arcsec) | 0.7193044 au | 0.7193069 au |
| EM Bary | 100.1823 deg | 100.1845 deg | -0.0023 deg (8 arcsec) | 0.9833262 au | 0.9833306 au |

Heliocentric latitude: Standish 1.1764 deg, Horizons 1.1765 deg.

Over the whole game, day by day from 2030-01-01 to 2035-12-31 against the same Horizons series
(section 4.1), the largest longitude error of the Venus row is **-0.0071 deg (25 arcsec) on
2031-Apr-13** and the largest distance error **6,100 km on 2030-Jun-30**. Both are within a
factor of two of JPL's nominal figures and a hundred times inside the game's tolerance.

**Verdict: the transcription has no typos and the row is fit to draw from.**

---

## 2. Reference position for 2030-01-01 (test oracle)

**Source: JPL Horizons API**, <https://ssd.jpl.nasa.gov/api/horizons.api>, called with `curl`
from this machine, the same query as the Earth-Mars note's section 2.1 with `COMMAND='299'`
(Venus): `EPHEM_TYPE='VECTORS'`, `CENTER='500@10'`, `REF_PLANE='ECLIPTIC'`,
`REF_SYSTEM='ICRF'`, `VEC_TABLE='3'`, `OUT_UNITS='AU-D'`. Frame confirmed by the returned
header (`Reference frame : Ecliptic of J2000.0`), source `DE441`.

Venus (`COMMAND='299'`), verbatim:

```
$$SOE
2462502.500000000 = A.D. 2030-Jan-01 00:00:00.0000 TDB
 X =-8.584003295356184E-02 Y = 7.140138437548758E-01 Z = 1.476928523358952E-02
 VX=-2.015042213621220E-02 VY=-2.524095787310565E-03 VZ= 1.127958812566741E-03
 LT= 4.154370169735723E-03 RG= 7.193068970317134E-01 RR=-7.766822436775116E-05
$$EOE
```

The Earth-Moon barycentre (`3`) and Earth (`399`) vectors returned today are identical to those
quoted in the Earth-Mars note, section 2.2.

Derived (`lon = atan2(y, x)`, normalised to 0-360):

| Body | Heliocentric ecliptic longitude (deg) | latitude (deg) | radius (au) |
|---|---|---|---|
| **Venus (299)** | **96.8553** | 1.1765 | 0.7193069 |
| Earth-Moon barycentre (3) | 100.1845 | -0.0038 | 0.9833306 |
| Mars (499), from the Earth-Mars note | 337.8203 | -1.7535 | 1.3814419 |

**Phase angle, Venus longitude minus Earth longitude, on 2030-01-01: -3.33 deg.** Venus is five
days short of inferior conjunction (section 4.1) when the game opens: almost exactly between the
Earth and the Sun, and about as far from a launch window as it gets.

---

## 3. The Earth-Venus transfer

### 3.1 The Hohmann flight, from the Table 1 semimajor axes

a_E = 1.00000261 au, a_V = 0.72333566 au, transfer semimajor axis
a_t = (a_E + a_V) / 2 = 0.861669 au, half-period with the Julian year of 365.25636 days:

**146.08 days.** The Venus ticket's "near 146 days" is right. Recomputed from the NSSDC fact
sheet distances (108.210e6 km and 149.598e6 km) it is 146.1 days.

For the circular-coplanar model: heliocentric departure speed change 2.50 km/s at Earth and
2.71 km/s at Venus (the spacecraft must slow down to leave Earth's orbit inward, and is going
faster than Venus when it arrives). Compare Earth-Mars: 258.87 days.

### 3.2 The synodic period

- **NASA NSSDC Venus Fact Sheet: "Synodic period (days) 583.92"**
  <https://nssdc.gsfc.nasa.gov/planetary/factsheet/venusfact.html>. The same page gives sidereal
  orbit period 224.701 d, semimajor axis 108.210e6 km / 0.72333199 au, eccentricity 0.00677323,
  inclination 3.39471 deg. *Fetch note: as with the Mars sheet, this host's certificate store
  could not validate the NSSDC chain, so the page was retrieved with chain validation disabled;
  the value is independently confirmed below.*
- **Derived independently** from the Table 1 semimajor axes by Kepler's third law: Venus's period
  224.702 d, Earth's 365.258 d, synodic **583.93 days**. Agrees with NSSDC to 0.01 d.

**Use 583.9 days.** That is 19.2 months, or **9.73 two-month turns**: Earth-Venus windows come
round every nine to ten turns, against every thirteen for Mars.

### 3.3 In the game's turns

A turn is sixty days (`days_per_turn = 60`), and the Earth-Mars spec turns 259 days into five
turns, so days are rounded up.

- **Hohmann flight: 146.08 / 60 = 2.43, so three turns** on the window, as the Venus ticket's
  recommendation assumes. It is only just over two: a flight of 120 days or less would be two
  turns, and the real minimum-energy flights below run 110 to 127 days. Whether the game charges
  the Hohmann three or a real-mission two is the same design question the Earth-Mars note raised
  for Mars (259 days = 5 turns against a real 190 days = 4), and it is the Venus ticket's to put
  to the designer, not this note's.
- **Synodic period: 9.73 turns.**
- **The phase angle moves 37.0 degrees per turn** (relative mean motion 0.616514 deg/day), against
  27.7 for Mars. A two-month turn is a wide net; the crossing rule of version 0.05.5 section 1 is
  what makes a window land in exactly one turn.

### 3.4 The departure phase angles, in the game's convention

The game reads the phase as the other Body's heliocentric longitude less Earth's (`ephemeris.toml`,
`hohmann_angle = 44.0` for Mars). Venus's mean motion is 1.602119 deg/day; over 146.08 days it
advances 234.03 deg while the spacecraft sweeps 180, so at departure Venus must be
180 - 234.03 = **-54.03 degrees** from Earth: **Venus trails Earth by 54 degrees** at the moment
of departure and overtakes it during the cruise (an inner planet is the mirror of Mars, where
Mars leads by 44 and Earth overtakes).

For the return, Venus to Earth: Earth advances 0.985605 x 146.08 = 143.97 deg during the cruise,
so at Venus departure Earth must stand at L_V + 180 - 143.97 = L_V + 36.03. In the same
convention, Venus longitude less Earth longitude, that is **-36.03 degrees**. Both legs are
negative and only 18 degrees apart, which at 0.6165 deg/day puts the ideal return departure
about **29 days after the ideal outbound one** (section 4.4 checks this against the ephemeris).
If the code keeps one phase variable per Body, the convention lesson of the Earth-Mars note
(section 4.3) applies unchanged: both legs must read the angle the same way.

---

## 4. The real Earth-Venus launch windows, January 2030 to December 2035

Three independent readings agree. The first is geometric from the ephemeris, the second is a
Lambert search over the same ephemeris, the third is NASA's own trajectory database.

### 4.1 Ideal-Hohmann phase crossings and inferior conjunctions, from DE441

Horizons vectors for Venus (299) and the Earth-Moon barycentre (3), daily from 2029-01-01 to
2036-07-01 (2,741 rows each, same query as section 2 with `VEC_TABLE='2'`). The dates on which
the phase angle passes through -54.03 deg, and through zero (inferior conjunction):

| Ideal-Hohmann departure (phase = -54.03) | Game turn | Inferior conjunction (phase = 0) | Game turn |
|---|---|---|---|
| 2029-Oct-07 | before the game | 2030-Jan-06 | 1 |
| **2031-May-18** | **9** (May-June 2031) | 2031-Aug-11 | 10 |
| **2032-Dec-21** | **18** (November-December 2032) | 2033-Mar-20 | 20 |
| **2034-Jul-25** | **28** (July-August 2034) | 2034-Oct-21 | 29 |
| 2036-Mar-06 | after the game | 2036-May-30 | after |

### 4.2 Minimum-energy departures by Lambert search on the same vectors

A universal-variable Lambert solver (implemented in this session) was run over every second day
of departure from 2029 to mid-2036 and every second day of flight time from 60 to 400 days,
keeping for each departure date the flight with the lowest Earth-departure C3 (the square of the
hyperbolic excess speed, the handbook's measure in the Earth-Mars note), then taking the local
minima:

| Earth departure | Turn | C3 (km^2/s^2) | Flight | Venus arrival | Turn | Transfer angle | v_inf at Venus |
|---|---|---|---|---|---|---|---|
| 2029-Oct-26 | before | 7.94 | 160 d | 2030-Apr-04 | 2 | 215 deg (Type II) | 4.83 km/s |
| **2031-May-17** | **9** | 6.16 | 160 d | 2031-Oct-24 | 11 | 200 deg (Type II) | 3.95 km/s |
| **2032-Dec-25** | **18** | 6.94 | 134 d | 2033-May-08 | 21 | 164 deg (Type I) | 3.70 km/s |
| **2034-Jul-26** | **28** | 7.14 | 124 d | 2034-Nov-27 | 30 | 144 deg (Type I) | 4.89 km/s |

Departures within 2 km^2/s^2 of each minimum span about 38 days: 2031-Apr-25 to Jun-02,
2032-Dec-03 to 2033-Jan-10, 2034-Jul-02 to Aug-13. (The search also returns a spurious
"minimum" on 2035-Dec-20 at C3 26, which is the series ending in July 2036 cutting off the
arrivals of the real March 2036 window; it is not an opportunity inside the game.)

The Lambert minima fall within one to four days of the ideal-Hohmann crossings in section 4.1,
much closer than for Mars (two to four weeks there), because Venus's orbit is nearly circular
(e = 0.0068 against Mars's 0.0934); its 3.4-degree inclination is what costs a little C3 above
the 6.2 km^2/s^2 of the coplanar model.

### 4.3 NASA's Trajectory Browser (primary source for the real opportunities)

**Source:** NASA Ames Research Center Trajectory Browser, <https://trajbrowser.arc.nasa.gov/>,
"a web-based tool developed at the NASA Ames Research Center for finding preliminary
trajectories to planetary bodies" (NTRS 20130011675,
<https://ntrs.nasa.gov/citations/20130011675>). Queried live from this machine
(`traj_browser.php`, target `Venus`, one-way, rendezvous, launch years 2029 to 2036, maximum
duration 1 year, maximum delta-V 8 km/s, minimise delta-V, show all trajectories): 22
trajectories returned. The tool's dates are on a 16-day launch grid and its delta-V is from a
200 km low Earth orbit; C3 here is derived from that injection delta-V. The best of each
opportunity:

| Earth departure | Venus arrival | Flight | Injection delta-V from LEO | C3 (km^2/s^2) | Arrival delta-V |
|---|---|---|---|---|---|
| 2029-Oct-31 | 2030-Feb-18 | 110 d | 3.890 km/s | 15.1 | 0.683 km/s |
| **2031-May-30** | 2031-Oct-03 | 126 d | 3.754 km/s | 12.0 | 0.395 km/s |
| **2032-Dec-26** | 2033-May-01 | 126 d | 3.519 km/s | 6.6 | 0.598 km/s |
| **2034-Jul-25** | 2034-Nov-29 | 127 d | 3.510 km/s | 6.4 | 1.061 km/s |

Every one of the 22 trajectories departs in one of those four clusters (2029-Oct-31 to Dec-02,
2031-May-30 to Jul-01, 2032-Dec-10 to 2033-Feb-12, 2034-Jun-07 to Sep-11); the tool returns
nothing launching in 2030, 2035 or the first days of 2036. The browser's 2031 optimum is a
Type I flight where the Lambert search's is Type II, and its 2031 C3 is higher because the tool
minimises total delta-V including the arrival burn, not C3; the departure dates agree with
sections 4.1 and 4.2 to within its 16-day grid.

**Corroboration from missions now planned.** NASA's VERITAS page says "VERITAS is currently
planned to launch no earlier than 2031" and "The spacecraft will arrive at Venus after a
six-month cruise" (<https://science.nasa.gov/mission/veritas/overview/>), consistent with the
mid-2031 opportunity; the June 2031 target date reported in the trade press is **UNVERIFIED**
here, since the first-party page gives only the year. ESA's EnVision fact sheet gives "Planned
launch: November 2031" with "a 15-month cruise"
(<https://www.esa.int/Science_Exploration/Space_Science/Envision/Envision_factsheet>): a
launch well after the May-June window on a long indirect cruise, which is a reminder that real
missions do not all fly the minimum-energy direct transfer. NASA's DAVINCI page names no launch
date (<https://science.nasa.gov/mission/davinci/>); its dates in secondary sources are not used.

### 4.4 The return leg, checked against the ephemeris

Crossings of the ideal return phase of -36.03 deg, and the Lambert minimum-C3 Venus-to-Earth
departures (C3 relative to Venus) from the same vectors:

| Ideal return departure (phase = -36.03) | Lambert minimum-C3 Venus departure | C3 | Flight | Earth arrival | Turns |
|---|---|---|---|---|---|
| 2029-Nov-06 | 2029-Nov-27 | 9.4 | 188 d | 2030-Jun-03 | before the game to 3 |
| **2031-Jun-15** | 2031-Jun-18 | 7.5 | 172 d | 2031-Dec-07 | **9** to 12 |
| **2033-Jan-21** | 2033-Jan-22 | 7.9 | 136 d | 2033-Jun-07 | **19** to 21 |
| **2034-Aug-23** | 2034-Aug-25 | 8.3 | 114 d | 2034-Dec-17 | **28** to 30 |
| 2036-Apr-03 | | | | | after the game |

So a return window opens about four weeks after each outbound one: in the same turn for 2031
and 2034, and in the turn after for 2032-33 (the outbound crossing is 2032-Dec-21 in turn 18,
the return 2033-Jan-21 in turn 19). Whether a Ship that arrives at Venus three turns after
departing can catch a return is a question the Venus ticket answers only if it wants a return
leg at all.

### 4.5 The windows in the game's own words

- **The game opens five days before an inferior conjunction (2030-Jan-06)**: Venus is between
  Earth and the Sun and a Venus launch is at its worst. There is no Earth-Venus window in 2030.
- **Turn 9, May-June 2031**: the phase crosses -54 on 2031-May-18; minimum-energy departures
  run late April to early June (Lambert) or 30 May to 1 July (Trajectory Browser). A Hohmann
  flight arrives in turn 11 (October 2031); a real 126-day flight arrives in the same turn.
- **Turn 18, November-December 2032**: the crossing is 2032-Dec-21; departures early December to
  early January. Arrival in turn 21 (May 2033).
- **Turn 28, July-August 2034**: the crossing is 2034-Jul-25; departures early July to mid
  August. Arrival in turn 30 (November 2034).
- **No fourth window before the game ends**: the next crossing is 2036-Mar-06, three months after
  turn 36. With Mars at turns 7, 20 and 33, the two skies together give a window in turns 7, 9,
  18, 20, 28 and 33.

Read under the version 0.05.5 crossing rule (a turn stands at the window when the phase crosses
the Hohmann angle inside it), each Venus crossing falls in exactly one turn. Read at a turn's first
instant instead, the closest turns would be 9 (-65.4 deg, 11 short), 19 (-47.9, 6 past) and 28
(-70.0, 16 short); the rule of section 1 of 0.05.5 was made for Mars and works the same here.

### 4.6 The phase angle at the first instant of every turn (test oracle)

Venus longitude less Earth-Moon-barycentre longitude, wrapped to -180..+180, from the DE441
vectors at 00:00 TDB on the first day of each turn's first month:

| Turn | Date | Phase | Turn | Date | Phase | Turn | Date | Phase |
|---|---|---|---|---|---|---|---|---|
| 1 | 2030-Jan-01 | -3.33 | 13 | 2032-Jan-01 | +87.89 | 25 | 2034-Jan-01 | +178.28 |
| 2 | 2030-Mar-01 | +32.54 | 14 | 2032-Mar-01 | +122.82 | 26 | 2034-Mar-01 | -148.07 |
| 3 | 2030-May-01 | +69.56 | 15 | 2032-May-01 | +159.33 | 27 | 2034-May-01 | -110.27 |
| 4 | 2030-Jul-01 | +107.80 | 16 | 2032-Jul-01 | -161.04 | 28 | 2034-Jul-01 | -69.96 |
| 5 | 2030-Sep-01 | +148.33 | 17 | 2032-Sep-01 | -120.04 | 29 | 2034-Sep-01 | -30.81 |
| 6 | 2030-Nov-01 | -172.99 | 18 | 2032-Nov-01 | -83.22 | 30 | 2034-Nov-01 | +6.20 |
| 7 | 2031-Jan-01 | -138.00 | 19 | 2033-Jan-01 | -47.87 | 31 | 2035-Jan-01 | +43.03 |
| 8 | 2031-Mar-01 | -103.85 | 20 | 2033-Mar-01 | -12.28 | 32 | 2035-Mar-01 | +78.37 |
| 9 | 2031-May-01 | -65.37 | 21 | 2033-May-01 | +25.75 | 33 | 2035-May-01 | +114.76 |
| 10 | 2031-Jul-01 | -25.81 | 22 | 2033-Jul-01 | +63.80 | 34 | 2035-Jul-01 | +153.60 |
| 11 | 2031-Sep-01 | +12.99 | 23 | 2033-Sep-01 | +103.56 | 35 | 2035-Sep-01 | -165.28 |
| 12 | 2031-Nov-01 | +50.68 | 24 | 2033-Nov-01 | +142.51 | 36 | 2035-Nov-01 | -127.55 |

The Standish row reproduces these to 0.007 deg (section 1.5), so a test at one-degree tolerance
can use any row of this table as its oracle.

---

## 5. Every Body's mean distance from the Sun, for the Solar Array

From the Table 1 semimajor axes (`a` at J2000; the rates move `a` by under one part in a
million over the game and are ignored). A satellite stands where its parent stands, as
`ephemeris.toml` already says of the Moon, Phobos and Deimos: the Moon's 0.3844e6 km from Earth
(0.00257 au) and the two Martian moons' 9,378 km and 23,459 km from Mars (0.00006 and
0.00016 au), from the NSSDC Moon and Mars fact sheets, are far below the rounding here.

| Body | Reads the row of | a (au) | a (million km, NSSDC) | Inverse-square (a_E / a)^2 | Linear a_E / a |
|---|---|---|---|---|---|
| Earth, and the Moon | EM Bary | 1.00000261 | 149.598 | **1.0000** | **1.0000** |
| Venus | Venus | 0.72333566 | 108.210 | **1.9113** | **1.3825** |
| Mars, Phobos and Deimos | Mars | 1.52371034 | 227.956 | **0.4307** | **0.6563** |

Sunlight falls off with the square of the distance, so a panel at Venus receives 1.91 times what
it receives at Earth and one at Mars 0.43 times: that is the physical factor. The linear column
is the gentler alternative if the designer wants Mars's penalty softened (0.66 rather than 0.43)
and Venus's reward trimmed (1.38 rather than 1.91). Which one the Solar Array uses is the Solar
Array ticket's question; the Venus ticket already quotes the inverse-square 1.91, and that figure
is confirmed. The km column is from the NSSDC fact sheets
(<https://nssdc.gsfc.nasa.gov/planetary/factsheet/>) as a cross-check only; the factors are
computed from the JPL `a` alone so that they match what the game draws.

Because the orbits are not circles, the true distance wanders: Venus between 0.718 and 0.728 au
(factor 1.89 to 1.94), Mars between 1.38 and 1.67 au (0.36 to 0.52), Earth between 0.983 and
1.017. Whether the Solar Array reads the mean or the live distance the map already computes is
another question for its ticket; the mean is what the ticket asked for and what the table gives.

---

## References

| URL | What was taken from it |
|---|---|
| <https://ssd.jpl.nasa.gov/planets/approx_pos.html> | Table 1 and Table 2a Keplerian elements and rates for Venus (and EM Bary and Mars, matched against the repository's file); the accuracy table (Venus 20"/1"/4000 km over 1800-2050); the algorithm, transcribed in the Earth-Mars note. Fetched and text-extracted directly; the accuracy cells read from the raw HTML. |
| <https://ssd.jpl.nasa.gov/txt/aprx_pos_planets.pdf> | The PDF of the same, linked from the page; not used separately. |
| <https://ssd.jpl.nasa.gov/api/horizons.api> | JPL Horizons API. Heliocentric J2000-ecliptic state vectors (DE441) for Venus (299), the Earth-Moon barycentre (3) and Earth (399) at 2030-01-01 00:00 TDB, quoted verbatim in section 2; daily vectors for 299 and 3 from 2029-01-01 to 2036-07-01 used for the phase crossings, the Lambert searches, the per-turn oracle and the whole-span check of the Standish row. |
| <https://ssd-api.jpl.nasa.gov/doc/horizons.html> | Horizons API parameter documentation. |
| <https://trajbrowser.arc.nasa.gov/> and <https://ntrs.nasa.gov/citations/20130011675> | NASA Ames Trajectory Browser: the 22 Earth-to-Venus one-way rendezvous trajectories launching 2029-2036 in section 4.3 (departure and arrival dates, injection and arrival delta-V), decoded from the result page's embedded segment data (times are seconds past J2000). The NTRS entry describes the tool. |
| <https://nssdc.gsfc.nasa.gov/planetary/factsheet/venusfact.html> | Venus synodic period 583.92 d; sidereal period 224.701 d; semimajor axis 108.210e6 km / 0.72333199 au; eccentricity 0.00677323; inclination 3.39471 deg. Retrieved with TLS chain validation disabled (this host's certificate store); the synodic period is independently confirmed from the JPL elements. |
| <https://nssdc.gsfc.nasa.gov/planetary/factsheet/marsfact.html>, <https://nssdc.gsfc.nasa.gov/planetary/factsheet/earthfact.html> and <https://nssdc.gsfc.nasa.gov/planetary/factsheet/moonfact.html> | The km distances in section 5 (Mars 227.956e6 km, Earth 149.598e6 km, as the Earth-Mars note quotes them); Phobos and Deimos semimajor axes 9,378 and 23,459 km and the Moon's 0.3844e6 km, fetched this session (same TLS note). |
| <https://science.nasa.gov/mission/veritas/overview/> | "VERITAS is currently planned to launch no earlier than 2031" and "after a six-month cruise". |
| <https://www.esa.int/Science_Exploration/Space_Science/Envision/Envision_factsheet> | "Planned launch: November 2031"; "a 15-month cruise". |
| <https://science.nasa.gov/mission/davinci/> | Checked for a launch date; the page gives none. |
| `docs/research/earth-mars-ephemeris.md`, `assets/data/ephemeris.toml`, `docs/spec/version-0.05.5.md` section 1 (branch `version-0.05.5`) | The method followed, the file shape matched, and the two-month calendar, the sixty-day turn, the round-up of days to turns and the crossing rule for a window turn. |

### Sources that could not be reached, and things not verified

- **VERITAS's June 2031 target** is reported by the trade press but the first-party NASA page
  gives only "no earlier than 2031"; the month is **UNVERIFIED** and nothing here rests on it.
- **DAVINCI's launch date** could not be read from a first-party page and is not used.
- The Trajectory Browser's column semantics (which segment is the injection, which the arrival
  burn, and that `T` is seconds past J2000) were inferred from the data: the four-segment shape
  matches Earth departure, Sun-centred cruise, Venus arrival, Venus orbit; and the J2000 reading
  puts every trajectory inside the 2029-2036 launch years asked for, where a Unix-epoch reading
  would put them in 1999-2004. The departure dates are read directly from the decoded times, and
  they agree with the two independent readings in sections 4.1 and 4.2, so the inference is not
  load-bearing.
- No launch-window handbook for Venus in this decade was found on NTRS of the kind the Earth-Mars
  note used (NASA/TM-2010-216764 covers Mars only); the Trajectory Browser stands in as the NASA
  primary source, backed by the two ephemeris-based readings.

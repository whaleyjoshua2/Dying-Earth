# Real-life delta-v for every move the game prices in Fuel

**Date:** 2026-10-04
**Branch:** version-0.09.8 (research note only; no code changed)

**What this is for:** the game will rescale its Fuel costs to real delta-v. This note
gives one defensible figure per leg, in km/s, with its source, its assumptions, and a
range where sources or launch windows disagree. Every leg is **orbit to orbit**: no
landings, no launches from a surface.

Every figure below was either fetched from a live source during this session or computed
in this session from fetched constants. Computed figures are labelled **computed** and
show their inputs. The few items that could not be checked are marked **UNVERIFIED**.
Nothing is stated from memory.

---

## 1. Summary table

Shared assumptions for every **computed** row: impulsive burns, patched conics, circular
coplanar planet orbits, no plane changes, no gravity losses, no margins. Low orbits are
circular at **300 km** altitude (Earth, Mars, Venus) and **100 km** (Moon). Section 2
gives the constants and the method.

| Leg | Delta-v km/s (recommended) | Range | Assumption | Source |
|---|---|---|---|---|
| LEO -> low lunar orbit | **4.0** | 3.9 - 4.1 | Propulsive both ends; 3-5 day transfer | Computed 3.93 (5-day minimum-energy); Apollo 11 flew 3.99 (NASA SP-4029); Wikipedia 4.04 (secondary) |
| LEO -> low Mars orbit, propulsive | **5.7** | 5.6 - 6.3 | Hohmann, 259 d; propulsive capture to 300 km circular | Computed 5.68; range from NASA/TM-2010-216764 windows 2026-2045 |
| LEO -> low Mars orbit, aerocapture | **3.7** | 3.6 - 3.9 | Departure burn 3.59 plus ~0.1 to lift periapsis out of the air | Computed; window range from NASA/TM-2010-216764 |
| LEO -> low Venus orbit, propulsive | **6.8** | 6.7 - 7.3 | Hohmann, 146 d; propulsive capture to 300 km circular | Computed 6.80; range from JPL 82-43 Vol 1 Pt 1 windows |
| LEO -> low Venus orbit, aerocapture | **3.6** | 3.5 - 3.7 | Departure burn 3.48 plus ~0.1 circularisation | Computed; 99 m/s circularisation from NASA/TM-2006-214291 |
| Low Mars orbit -> Phobos | **1.2** | 1.2 | Hohmann from 300 km; rendezvous, not landing | Computed 1.20; method reproduces Foster (NASA Ames 2011) to 1 m/s |
| Low Mars orbit -> Deimos | **1.7** | 1.7 | Same | Computed 1.72 |
| Phobos <-> Deimos | **0.75** | 0.75 | Hohmann between the two moon orbits | Foster 748 m/s; computed 0.748 |
| LEO -> Phobos direct, propulsive | **5.5** | 5.5 - 6.2 | Capture burn at 300 km periapsis, circularise at Phobos | Computed 5.54 |
| LEO -> Deimos direct, propulsive | **5.3** | 5.2 - 5.9 | Same, at Deimos | Computed 5.26 |
| LEO -> Phobos direct, aerocapture | **4.1** | 4.1 - 4.4 | Aerocapture, then 0.53 to circularise at Phobos | Computed 4.12; Foster 538 m/s |
| LEO -> Deimos direct, aerocapture | **4.2** | 4.2 - 4.5 | Aerocapture, then 0.65 to circularise at Deimos | Computed 4.24; Foster 651 m/s |
| LEO -> Sun-Earth L4 or L5 | **4.0** | 3.5 - 5.4 | Two-year phasing orbit; departure 3.23 + stop burn 0.76 (L5) | Computed; supported by EASCO (arXiv 1109.2929) and ESA LISAmax (arXiv 2304.08287) |

### Three cases side by side (added 2026-10-04; the designer chose "aerobraking as flown")

"Aerobraking as flown" is what real orbiters have done: a **propulsive** capture burn into
a loose, high ellipse, months of passes through the upper air to shrink it, then a small
burn to lift periapsis clear. Single-pass aerocapture has not been flown by any mission
found in this research. Detail and sources in section 3.8.

| Leg from LEO | Propulsive | **Aerobraking as flown** | Aerocapture | Aerobraking adds | Note on the middle column |
|---|---|---|---|---|---|
| Low lunar orbit | 4.0 | **4.0** | 4.0 | - | No atmosphere; one figure |
| Low Mars orbit | 5.7 | **4.6** (4.4 - 5.2) | 3.7 | 3 - 6 months (flown: 2.5, 5, 11, 17) | 3.59 depart + 0.86 capture to 35 h ellipse + 0.1 trim. Flown four times |
| Low Venus orbit | 6.8 | **4.4** (3.9 - 5.3) | 3.6 | about 4 months (study figure; **never flown**) | 3.48 depart + 0.77 capture to 24 h ellipse + 0.1 trim. No mission has aerobraked from capture to low Venus orbit |
| Phobos | 5.5 direct; **5.2** by the three-burn route | **4.9** | 4.1 | 1 - 3 months (estimate; never flown) | Aerobraking helps: saves 0.3 on the best propulsive route |
| Deimos | 5.3 direct; **5.0** by the three-burn route | **5.0** | 4.2 | none | Aerobraking does not help; the propulsive three-burn route is the cheapest |
| Sun-Earth L4 / L5 | 4.0 | **4.0** | 4.0 | - | No atmosphere; one figure |

Unaffected by the choice: low Mars orbit -> Phobos 1.2, -> Deimos 1.7, Phobos <-> Deimos
0.75 (all outbound burns above the air). **Coming down** is different: Phobos -> low Mars
orbit and Deimos -> low Mars orbit can aerobrake, at about 0.6 and 0.7 instead of 1.2 and
1.7 (section 3.8.5).

Context-only figures (not recommended for pricing, secondary source):

| Leg | Delta-v km/s | Source |
|---|---|---|
| LEO -> Earth-Moon L1 | 3.77 | Wikipedia "Delta-v budget" (secondary) |
| LEO -> Earth-Moon L2 | 3.43 | Wikipedia "Delta-v budget" (secondary; uses a lunar swing-by) |
| LEO -> Earth escape (C3 = 0) | 3.20 computed; 3.22 Wikipedia | Floor for every interplanetary leg |

### The designer's remembered figures, checked

| Remembered | Verdict |
|---|---|
| Moon ~4 | **Right.** 3.9 - 4.0. |
| Mars ~5.7 | **Right** for propulsive capture in a typical window (5.68). |
| Venus ~6.9 | **Right** within rounding: 6.8. |
| Low Mars orbit -> Phobos ~1.4 | **Wrong, too high.** 1.20. |
| Low Mars orbit -> Deimos ~1.9 | **Wrong, too high.** 1.72. |
| Sun-Earth L4/L5 ~3.5 to 4.5 | **Right as a range**, and it is a time trade: 3.5 takes about five years, 4.0 about two, 4.7 (L5) to 5.4 (L4) about one. |

---

## 2. Constants and method for the computed figures

### 2.1 Constants (all fetched this session)

| Quantity | Value | Source |
|---|---|---|
| GM Sun | 1.32712440041279419e11 km^3/s^2 | <https://ssd.jpl.nasa.gov/astro_par.html> |
| Astronomical unit | 149,597,870.7 km | same |
| GM Earth | 398,600.435507 km^3/s^2 | same |
| GM Moon | 4,902.800118 km^3/s^2 | same |
| GM Mars system | 42,828.375816 km^3/s^2 | same |
| GM Venus | 324,858.592 km^3/s^2 | same |
| Radius Earth (equatorial) | 6,378.1366 km | <https://ssd.jpl.nasa.gov/planets/phys_par.html> |
| Radius Mars | 3,396.19 km | same |
| Radius Venus | 6,051.8 km | same |
| Radius Moon | 1,737.4 km | <https://ssd.jpl.nasa.gov/sats/phys_par/> (**value not re-read from the page this session; standard figure**) |
| Semimajor axis Venus / Earth / Mars | 0.72333566 / 1.00000261 / 1.52371034 au | <https://ssd.jpl.nasa.gov/planets/approx_pos.html> Table 1 |
| Semimajor axis Moon | 384,400 km | <https://ssd.jpl.nasa.gov/sats/elem/> |
| Semimajor axis Phobos / Deimos | 9,375 km / 23,457 km | same |

### 2.2 Formulas

```
v_circ(r)        = sqrt(mu / r)
v(r, a)          = sqrt(mu * (2/r - 1/a))                      # vis-viva
Hohmann r1 -> r2 : a = (r1 + r2)/2
                   dv1 = |v(r1, a) - v_circ(r1)|
                   dv2 = |v_circ(r2) - v(r2, a)|
Leave a circular orbit of radius r with hyperbolic excess v_inf:
                   dv = sqrt(v_inf^2 + 2*mu/r) - v_circ(r)
Capture into that orbit from v_inf: the same expression.
```

The last expression is the Oberth effect: a burn deep in a gravity well buys more than
the same burn far out. It is why the departure burn for Mars (v_inf = 2.95 km/s) costs
only 0.39 km/s more than bare escape, and why the Mars moons are cheap to reach.

---

## 3. Leg by leg

### 3.1 LEO -> low lunar orbit: 4.0 km/s

**Computed** (300 km LEO, r = 6,678.1 km; Moon at 384,400 km; 100 km lunar orbit,
r = 1,837.4 km):

```
transfer ellipse a = (6678.1 + 384400)/2 = 195,539 km        half-period 4.98 days
trans-lunar injection  = v(6678.1, a) - v_circ(6678.1) = 10.832 - 7.726 = 3.106
speed at apogee        = 0.188;  Moon's orbital speed = 1.025;  v_inf at Moon = 0.836
lunar orbit insertion  = sqrt(0.836^2 + 2*4902.8/1837.4) - sqrt(4902.8/1837.4) = 0.823
total                  = 3.93 km/s
```

From a 185 km LEO the total is 3.96; from 400 km, 3.91.

**Flown: Apollo 11, 3.99 km/s.** NASA SP-4029, *Apollo by the Numbers* (Orloff),
Apollo 11 tables: second S-IVB burn "Velocity Change 10,008.1 ft/sec" (3.050 km/s) from
a 100.4 x 98.9 n mi parking orbit; lunar orbit insertion "2917.5" ft/sec (0.889 km/s)
into 169.7 x 60.0 n mi; circularisation "158.8" ft/sec (0.048 km/s) into 66.1 x 54.5
n mi. Sum 3.988 km/s. Apollo flew a faster (about 73 hour) transfer than the 5-day
minimum, which is why its insertion burn is larger than the computed 0.82.
<https://www.nasa.gov/wp-content/uploads/2023/04/sp-4029.pdf>

**Secondary cross-check:** Wikipedia "Delta-v budget", Earth-Moon high-thrust table,
LEO to low lunar orbit **4.04**; to Earth-Moon L1 **3.77**; to Earth-Moon L2 **3.43**;
to escape (C3 = 0) **3.22**. <https://en.wikipedia.org/wiki/Delta-v_budget>

Three independent routes agree within 0.11 km/s. Confidence: **high**.

### 3.2 LEO -> low Mars orbit: 5.7 propulsive, 3.7 with aerocapture

**Computed**, Hohmann between circular orbits at 1.00000261 and 1.52371034 au:

```
heliocentric: v_inf leaving Earth = 2.945   v_inf arriving at Mars = 2.649   258.9 days
trans-Mars injection from 300 km LEO = sqrt(2.945^2 + 10.926^2) - 7.726 = 3.590
Mars orbit insertion to 300 km circular (r = 3696.2 km):
     sqrt(2.649^2 + 4.814^2) - 3.404 = 2.091
total propulsive = 5.68 km/s
```

LEO altitude barely matters: 5.70 from 200 km, 5.64 from 500 km.

**With aerocapture** the 2.09 insertion burn is replaced by one pass through the
atmosphere and a small burn at apoapsis to lift periapsis back out. **Computed** for a
post-capture orbit of 50 km x 300 km altitude: 0.060 km/s. Total 3.59 + 0.06 = 3.65;
recommended **3.7** to leave room for the clean-up burn that real designs carry (the
Venus study in 3.3 measured 99 m/s for the equivalent step). The heat shield is not
free - Wikipedia's article puts shields at roughly 15% of mass - but that is a mass
cost, not a delta-v cost, and belongs to game design.

**Secondary source disagrees on the split, not much on the total.** Wikipedia's
interplanetary table gives LEO -> Mars transfer orbit **4.3** and Mars transfer orbit ->
low Mars orbit **2.7** (total 7.0), against 3.6 + 2.1 here. Its 4.3 is higher than any
patched-conic departure for a Hohmann-class window (the NASA handbook windows in 3.7 give
3.55 - 3.86) and is not explained on the page; its cited source is a 2007 web page
("Rockets and Space Transportation"), not a technical report. **Treat the Wikipedia Mars
figures as loose; the computed ones are checked against NASA window data in 3.7.**

Confidence: **high** for the Hohmann figure; the real spread is the window (3.7).

### 3.3 LEO -> low Venus orbit: 6.8 propulsive, 3.6 with aerocapture

**Computed**, Hohmann between 1.00000261 and 0.72333566 au:

```
heliocentric: v_inf leaving Earth = 2.495   v_inf arriving at Venus = 2.707   146.1 days
trans-Venus injection from 300 km LEO = sqrt(2.495^2 + 10.926^2) - 7.726 = 3.481
Venus orbit insertion to 300 km circular (r = 6351.8 km):
     sqrt(2.707^2 + 10.114^2) - 7.151 = 3.318
total propulsive = 6.80 km/s
```

Venus is cheaper to *leave for* than Mars (3.48 against 3.59) but dearer to *stop at*
(3.32 against 2.09), because Venus is nearly Earth's mass and a low orbit sits deep in
its well.

**With aerocapture:** NASA/TM-2006-214291, *Systems Analysis for a Venus Aerocapture
Mission* (Lockwood, Starr et al., Langley, 2006), for a 300 x 300 km polar orbit:

> "Orbit circularization requires a 99 m/sec increase in velocity at apoapsis after some
> 4300 m/sec of velocity is removed during the single aerocapture pass."

So 3.48 + 0.10 = **3.6**. The same report prices the non-aerocapture alternative at
"Orbit Insertion dV: 2300 m/s" into a 4.4-hour ellipse plus 2,011 m/s shed over 670
aerobraking orbits in 122 days. Its mission arrives fast (entry at 11.25 km/s, launch
C3 = 8.3 km^2/s^2), so its 4.3 km/s is above the Hohmann 3.32; it is one real design
point, not a minimum.
<https://ntrs.nasa.gov/api/citations/20060010899/downloads/20060010899.pdf>

**Secondary cross-check:** Wikipedia gives 3.5 from LEO to a Venus transfer, matching
the computed 3.48.

Confidence: **high** for the Hohmann figure.

### 3.4 Low Mars orbit -> Phobos, -> Deimos, and Phobos <-> Deimos

**Computed**, Hohmann transfers around Mars, coplanar (both moons are within 2 deg of
Mars's equator; the low orbit is assumed equatorial):

| Transfer | Burn 1 | Burn 2 | Total km/s | Transfer time |
|---|---|---|---|---|
| 300 km orbit (r = 3,696 km) -> Phobos (9,375 km) | 0.673 | 0.530 | **1.20** | 2.2 h |
| 300 km orbit -> Deimos (23,457 km) | 1.070 | 0.646 | **1.72** | 6.7 h |
| Phobos <-> Deimos | 0.418 | 0.330 | **0.75** | 8.9 h |

"Phobos" here means matching Phobos's orbit (rendezvous). Landing adds only metres per
second: both moons have negligible gravity.

**Primary source:** C. Foster (NASA Ames), *Delta-V Budgets for Robotic and Human
Exploration of Phobos and Deimos*, Second International Conference on the Exploration of
Phobos and Deimos, March 2011.
<https://ntrs.nasa.gov/api/citations/20190026585/downloads/20190026585.pdf>
It gives **Phobos <-> Deimos = 748 m/s** ("Hohmann transfer available every 10.24 hr
synodic period"), identical to the computed 0.748. Foster does not start from a low
circular orbit; he starts from a "highly elliptical staging orbit" (250 km x 82,173 km
altitude) and gives staging orbit -> Phobos 845 m/s propulsive or 538 m/s with
aerobraking, and staging orbit -> Deimos 604 m/s. Running this note's formulas on his
staging orbit returns 844, 538 and 604 m/s, so **the method here reproduces the NASA
figures to 1 m/s**; the low-orbit figures in the table are the same method from a
different start.

A mixed case worth knowing: the inclined case is not free. Foster notes his figures "do
not include any possible out-of-plane maneuvers". A polar low Mars orbit to an equatorial
moon would cost far more than 1.2.

Confidence: **high**.

### 3.5 LEO -> Phobos and LEO -> Deimos directly

A ship bound for a moon should not circularise in low Mars orbit first. It burns once at
low periapsis (300 km) to drop into an ellipse whose high point is the moon's orbit, then
circularises there. **Computed**, from the Hohmann arrival v_inf of 2.649 km/s:

| Destination | Departure | Capture burn at 300 km | Circularise at moon | Arrival total | **LEO total** | With aerocapture |
|---|---|---|---|---|---|---|
| Low Mars orbit | 3.590 | 2.091 | - | 2.091 | **5.68** | 3.65 |
| Phobos | 3.590 | 1.418 | 0.530 | 1.948 | **5.54** | 4.12 |
| Deimos | 3.590 | 1.020 | 0.646 | 1.667 | **5.26** | 4.24 |

**Propulsively, Deimos is the cheapest place in the Mars system to reach, then Phobos,
then low Mars orbit** - the reverse of their heights. With aerocapture the order flips
back (low orbit 3.65, Phobos 4.12, Deimos 4.24), because the atmosphere does the deep
part of the braking for free and the moons then need a real burn to lift periapsis.

The aerocapture circularisation burns (0.530, 0.646) match Foster's "Option 2 with
aerobrake" figures of 538 and 651 m/s (his periapsis is 250 km, not 300).

**Secondary cross-check:** Wikipedia's interplanetary table chains Mars transfer orbit ->
capture orbit 0.9 -> Deimos transfer 0.2 -> Deimos 0.7 (sum **1.8** against 1.67 here),
and to Phobos 0.9 + 0.2 + 0.3 + 0.5 = **1.9** (against 1.95 here), versus **2.7** to low
Mars orbit. Same ordering, similar size.

Note the consistency trap for the game: LEO -> low Mars orbit (5.68) plus low Mars orbit
-> Deimos (1.72) is 7.40, far more than LEO -> Deimos direct (5.26). Real delta-v is not
additive through a low orbit. How the game handles that is a design decision.

Confidence: **high** for the arithmetic; the window spread of 3.7 applies on top.

### 3.6 LEO -> Sun-Earth L4 and L5: 4.0 km/s at two years; a pure time trade

L4 leads Earth by 60 deg on Earth's own orbit; L5 trails by 60 deg. Getting there is not
a climb but a **phasing** problem: leave Earth on a solar orbit with a slightly different
period, let the gap open to 60 deg over a whole number of laps, then burn to match
Earth's orbit again. ESA mission analysts state the rule directly (Martens, Khan, Bayle,
*LISAmax*, arXiv 2304.08287, section 5; authors are ESA's Mission Analysis Section):

> "To reach the near-L4 region, the transfer is a heading orbit with a semi-major axis
> below 1 au. To reach the near-L3 and near-L5 regions, the transfer is a trailing orbit
> with a semi-major axis above 1 au. When the desired location is reached, a maneuver is
> applied to insert into the circular target orbit and stop the drift relative to Earth.
> The required transfer [delta-v] is a function of the transfer duration, which can be a
> multiple of 1 year, minus (for L4) or plus (for L3 and L5) a fixed increment. The
> longer the transfer and the more heliocentric revolutions are completed, the lower the
> [delta-v]."

<https://arxiv.org/pdf/2304.08287>

**Computed** from that rule. For N laps the phasing orbit's period is (N + 1/6)/N years
for L5 and (N - 1/6)/N years for L4. The departure v_inf and the stop burn are equal (the
orbit is tangent to Earth's at the same point both times). Departure from 300 km LEO uses
the Oberth expression; the stop burn is in open space and is paid in full.

| Target | Laps | Time | v_inf km/s | Departure from LEO | Stop burn | **Total km/s** |
|---|---|---|---|---|---|---|
| L5 | 1 | 1.17 yr (14 mo) | 1.421 | 3.292 | 1.421 | **4.71** |
| L5 | 2 | 2.17 yr (26 mo) | 0.764 | 3.227 | 0.764 | **3.99** |
| L5 | 3 | 3.17 yr | 0.523 | 3.213 | 0.523 | **3.74** |
| L5 | 4 | 4.17 yr | 0.397 | 3.207 | 0.397 | **3.61** |
| L5 | 5 | 5.17 yr | 0.320 | 3.205 | 0.320 | **3.53** |
| L4 | 1 | 0.83 yr (10 mo) | 1.991 | 3.380 | 1.991 | **5.37** |
| L4 | 2 | 1.83 yr (22 mo) | 0.903 | 3.237 | 0.903 | **4.14** |
| L4 | 3 | 2.83 yr | 0.584 | 3.216 | 0.584 | **3.80** |
| L4 | 4 | 3.83 yr | 0.432 | 3.209 | 0.432 | **3.64** |
| L4 | 5 | 4.83 yr | 0.342 | 3.205 | 0.342 | **3.55** |

The floor is bare Earth escape, **3.20 km/s**, approached only as the trip time goes to
infinity. L4 is always a little dearer and a little quicker than L5 at the same lap
count. Unlike Mars and Venus **there is no launch window**: the points keep station with
Earth, so the cost is the same any month (the ESA paper shows a seasonal ripple from
Earth's eccentricity, a few hundred m/s peak to peak in its Figure 6).

**How this squares with published mission studies:**

- **EASCO** (Gopalswamy et al., NASA Goddard mission-design-lab study, arXiv 1109.2929,
  section 3.3), a Sun-Earth L5 spacecraft: "For the same transfer time [~2 years], the
  required launch C3 is ~1.0 km2/s2 and the delta-V is ~950 m/s" with chemical
  propulsion, and "~2 years for a launch C3 ... ~2.2 km2/s2 and a delta-V of ~1.5 km/s"
  with low-thrust electric propulsion. The computed two-lap L5 case needs C3 = 0.58 and a
  0.76 stop burn - the same class. Converting EASCO's chemical case to this note's terms:
  departure from LEO at C3 = 1.0 is 3.25, plus 0.95, total **4.2 km/s**.
  <https://arxiv.org/pdf/1109.2929>
- **LISAmax** (above): two-lap transfers to L4 and L5 by chemical propulsion; its Figure 6
  plots "Transfer Delta-V [m/s]" on an axis running 700 to 1,300, L5 lowest. The curve
  values could not be read from the text extraction, only the axis range. The computed
  two-lap stop burns (0.76 for L5, 0.90 for L4) sit inside that axis.
- **MOST** (Gopalswamy et al., arXiv 2303.02895), four spacecraft to L4/L5 and beyond via
  a lunar transfer and lunar gravity assist: whole-mission budgets of "~464, ~494, ~522,
  and ~521 m/s" after launch, with multi-year drifts. Shows that a lunar swing-by and
  patience push the post-launch cost down to about 0.5 km/s.
  <https://arxiv.org/pdf/2303.02895>
- **UNVERIFIED:** a Universitat Politecnica de Catalunya thesis
  (<https://recercat.cat/handle/2117/394633>) is reported by a search snippet to reach
  L4/L5 along manifolds from Sun-Earth L1/L2 orbits for about 260 m/s in 8.5 - 9 years,
  400 m/s in 6 years, 550 m/s in 4 years. The page refused the fetch (HTTP 403), so these
  numbers were **not read from the source**. They start from an L1/L2 orbit, not LEO.

**How good is the sourcing?** Plainly: **the weakest in this note.** No source found
gives a LEO-to-L4/L5 total as a single number. The total is this note's own two-body
calculation; the published studies confirm the *stop burn* and the *time trade* at the
two-year point and nowhere else. The physics is simple and the two-year agreement is
good (4.0 computed against 4.2 from EASCO), so the table is trustworthy to perhaps
+/- 0.3 km/s, but the one-lap rows in particular rest on the calculation alone.

### 3.7 How much the window matters

**Mars.** Source: Burke, Falck, McGuire, *Interplanetary Mission Design Handbook:
Earth-to-Mars Mission Opportunities 2026 to 2045*, NASA/TM-2010-216764 (already used in
`earth-mars-ephemeris.md`).
<https://ntrs.nasa.gov/api/citations/20100037210/downloads/20100037210.pdf>
Each opportunity table lists four optimised trajectories (lowest launch energy and lowest
arrival speed, for the short Type I and long Type II transfers) with C3 and "Mars arrival
excess speed". Converting each to this note's 300 km orbits (**computed** from the
handbook's figures) and taking the cheapest of the four:

| Opportunity | Cheapest of the four: departure + capture = total km/s | Lowest departure burn |
|---|---|---|
| 2026 | 3.63 + 2.05 = **5.68** | 3.61 |
| 2028 | 3.62 + 2.25 = **5.87** | 3.60 |
| 2031 | 3.76 + 2.52 = **6.27** | 3.57 |
| 2033 | 3.62 + 2.44 = **6.05** | 3.55 |
| 2035 | 3.66 + 2.11 = **5.77** | 3.66 |
| 2037 | 3.86 + 2.46 = **6.32** | 3.86 |
| 2039 | 3.74 + 2.12 = **5.86** | 3.74 |
| 2041 | 3.64 + 2.01 = **5.65** | 3.64 |
| 2043 | 3.60 + 2.16 = **5.76** | 3.60 |
| 2045 | 3.69 + 2.41 = **6.09** | 3.59 |

So between a good window and a bad one the propulsive total runs **5.65 to 6.32**, about
12%; the idealised Hohmann 5.68 is a *good*-window figure. With aerocapture only the
departure burn counts, **3.55 to 3.86**, about 9%. These are upper bounds on each
window's true minimum (the handbook lists four corner cases, not the minimum of the sum),
so the real spread is slightly narrower. The handbook's lowest C3 per opportunity runs
7.781 (2033) to 14.84 (2037) km^2/s^2. Picking the wrong trajectory *within* a window
costs more than picking a bad window: the 2031 minimum-C3 Type I row totals 7.54.
Leaving outside any window is a different order of cost again; see
`earth-mars-ephemeris.md` section 4.5.

The cause is Mars's eccentric orbit (e = 0.093, JPL Table 1).

**Venus.** Source: Sergeyevsky and Yin, *Interplanetary Mission Design Handbook, Volume
1, Part 1: Earth to Venus Ballistic Mission Opportunities, 1991-2005*, JPL Publication
82-43. <https://ntrs.nasa.gov/api/citations/19840019711/downloads/19840019711.pdf>
Its "Energy Minima" tables list the lowest launch energy (C3L) and lowest arrival speed
(VHP) per opportunity. Read from a poor scan; three opportunities (1993, 1996, 2002) were
illegible and are omitted, and 2001's Type II VHP was unreadable:

| Opportunity | Lowest C3L (km^2/s^2) | -> departure burn (computed) | Lowest VHP (km/s) | -> capture burn to 300 km (computed) |
|---|---|---|---|---|
| 1991 | 5.904 | 3.47 | 2.282 | 3.22 |
| 1994 | 7.918 | 3.56 | 3.043 | 3.41 |
| 1997 | 7.585 | 3.54 | 3.044 | 3.41 |
| 1999 | 5.946 | 3.47 | 2.834 | 3.35 |
| 2001 | 7.257 | 3.53 | 3.492 | 3.55 |
| 2004 | 8.718 | 3.59 | 3.203 | 3.46 |
| 2005 | 7.670 | 3.55 | 3.087 | 3.42 |

Venus windows are far more alike than Mars windows, because Venus's orbit is nearly
circular (e = 0.007): the departure burn moves only **3.47 to 3.59** (3%). The lowest-C3
and lowest-VHP entries are *different trajectories*, so they cannot simply be added; the
sum of each row (6.69 to 7.08) is a floor that no single trajectory reaches. The honest
range for a propulsive total is therefore **about 6.7 to 7.3**, with the upper end an
estimate rather than a read figure. Windows recur every 583.92 days (the handbook's
synodic period).

### 3.8 Aerobraking as flown

Added 2026-10-04 at the designer's request. The rule: departure burn from section 3, plus
a propulsive capture burn at 300 km periapsis into a loose ellipse, plus the trim burns
that end aerobraking. The air does the rest, over months.

#### 3.8.1 What real missions burned at Mars

| Mission | Orbit-insertion burn | Capture orbit | Aerobraking | Exit / trim burns | Source and quality |
|---|---|---|---|---|---|
| Mars Global Surveyor (1997) | **973.0 m/s** | 44.993 h; 262.9 x 54,025.9 km | Planned about 4 months; took **Sept 1997 - 4 Feb 1999 (17 months)** in two phases around a science hiatus, after a solar-array fault. Drag removed "about 1200 m/s" | Termination burn **61.9 m/s** (periapsis to 377 km, apoapsis 450 km); four trim burns of 1.1, 1.0, 1.0, 0.4 m/s | Esposito et al., AAS 98-384 (JPL navigation team): "The braking velocity-change was 973.0 m/sec." Johnston et al. (JPL) for the exit. Lyons (JPL) for 1,200 m/s. **Primary.** |
| Mars Odyssey (2001) | **Not found** in a primary source (19-minute burn) | 18.6 h; periapsis 292 km | **77 days**, 332 drag passes; saved "1.08 km/s" | "four small maneuvers" afterwards, sizes not given | Tartabini, Munk, Powell (NASA Langley), AIAA 2002-4537. **Primary**, but silent on the insertion burn's size. |
| Mars Reconnaissance Orbiter (2006) | **1,015 m/s**, about 26 minutes | 35 h; periapsis 300 km (reference) | **145 days** (4 April - 30 August 2006), 26 small corridor burns | Exit burn size not found | NASA PDS mission catalogue: "The delta-V required to accomplish this critical maneuver was 1015 m/s". Prince and Striepe (NASA Langley), AAS 07-244, for the 145 days. **Primary.** |
| ExoMars Trace Gas Orbiter (2016) | "more than 1.5 km/s", 139 minutes (ESA press release); 1,550 m/s in press coverage only | 4-sol ellipse, then lowered **propulsively** to a 1-sol orbit before aerobraking began | **342 days** (15 March 2017 - 20 February 2018) including a 2-month pause at solar conjunction; 952 passes; drag removed **1,017 m/s**, period 24 h -> 2.1 h | Manoeuvres during aerobraking **51 m/s** total, including walk-outs of 8.6 and 22.5 m/s; plus 9 m/s attitude control | Bellei et al. and Guilanya, Rivero (ESA/ESOC flight dynamics), ISSFD 2019, marked "non-peer review" conference papers by the operators. **Primary for aerobraking.** Insertion burn: ESA press release, **medium**. |

<https://ntrs.nasa.gov/api/citations/19980201716/downloads/19980201716.pdf> (MGS, AAS 98-384)
<https://ntrs.nasa.gov/api/citations/20000056881/downloads/20000056881.pdf> (MGS second phase)
<https://ntrs.nasa.gov/api/citations/20000057306/downloads/20000057306.pdf> (Lyons, Magellan and MGS compared)
<https://ntrs.nasa.gov/api/citations/20030005808/downloads/20030005808.pdf> (Odyssey)
<https://pds-geosciences.wustl.edu/mro/mro-m-rss-1-magr-v1/mrors_0xxx/catalog/mission.cat> (MRO catalogue)
<https://ntrs.nasa.gov/api/citations/20070010460/downloads/20070010460.pdf> (MRO, AAS 07-244)
<https://issfd.org/ISSFD_2019/ISSFD_2019_AIAC18_Bellei-Gabriele.pdf> (TGO navigation)
<https://www.esa.int/Newsroom/Press_Releases/ExoMars_TGO_reaches_Mars_orbit_while_EDM_situation_under_assessment> (TGO insertion)

**The coordinator's remembered "MRO about 1.0 km/s" is right: 1,015 m/s.**

The flown burns are bigger than the Hohmann case because real arrivals were faster. MRO's
1,015 m/s into a 35 h orbit implies an arrival v_inf of 2.96 km/s (**computed**), against
the Hohmann 2.65.

#### 3.8.2 Recommended: LEO -> low Mars orbit, aerobraking as flown = 4.6 km/s

**Computed**, Hohmann arrival (v_inf 2.649 km/s), capture at 300 km periapsis into an
MRO-like 35-hour ellipse (apoapsis altitude 44,560 km):

```
speed at periapsis on the arrival hyperbola = sqrt(2.649^2 + 4.814^2)      = 5.495
speed at periapsis of the 35 h ellipse (a = 25,825 km)                     = 4.639
capture burn                                                               = 0.856
drag then removes 4.639 - 3.404 = 1.235 (flown: 1.02 - 1.2, so this is within experience)
exit: from a 110 x 450 km orbit, raise periapsis to 300 km (0.045) and
      lower apoapsis to 300 km (0.034)                                     = 0.078
      (flown: MGS 61.9 m/s exit; TGO 51 m/s for all manoeuvres)
total = 3.590 + 0.856 + 0.078 = 4.52; with ~0.03 of corridor burns, 4.55 -> 4.6
```

**Range 4.4 - 5.2.** Low end: a 4-sol capture orbit in a good window (3.60 + 0.69 + 0.1).
High end: the 2031 window's fast arrival (v_inf 3.445: 3.76 + 1.28 + 0.1). The choice of
ellipse matters little: an 18.6 h orbit costs 0.95 to enter, a 45 h orbit 0.83.

**The coordinator's remembered 4.6 is right.**

#### 3.8.3 What real missions did at Venus

| Mission | Insertion burn | Capture orbit | Aerobraking | Source and quality |
|---|---|---|---|---|
| Venus Express (2006) | **1,251 m/s**, about 50 minutes | 400 x 350,000 km, 9 days | **None on arrival.** The 24-hour operational orbit was reached **propulsively**: seven burns of 5.8, 199.9, 105.3, 9.2, 8.0, 2.0 and 3.1 m/s (**333 m/s**). Its 2014 aerobraking was an end-of-life experiment: periapsis down to about 130 km (minimum 129.2 km), core phase 18 June - 11 July 2014, period cut from 24 h to just over 22 h, then 15 burns to climb back to about 460 km | ESA Science and Technology pages and ESA press release. **Primary (operator), public-information level.** |
| Magellan (1990; aerobraked 1993) | Solid rocket motor; size **not found** | 3.26 h; apoapsis about 8,467 km before aerobraking | **70 days**, ending 3 August 1993; apoapsis 8,467 -> 541 km; drag removed "about 1200 m/s"; final orbit 541 x 197 km. Done three years after arrival, not from capture | Lyons (JPL) and the NTRS abstract "The Magellan Venus Mapping Mission: Aerobraking Operations". **Primary.** |

<https://sci.esa.int/web/venus-express/-/38947-orbit-insertion>
<https://sci.esa.int/web/venus-express/-/39112-successful-orbit-insertion>
<https://www.esa.int/Newsroom/Press_Releases/Venus_Express_goes_gently_into_the_night>
<https://ntrs.nasa.gov/citations/20210004680>

**The coordinator's remembered "Venus Express about 1.25 km/s" is right: 1,251 m/s.** But
Venus Express is not an aerobraking-capture mission, and its burn is large because it
arrived fast (implied v_inf about 5.0 km/s, **computed** from 1,251 m/s at 400 km).

**No mission has aerobraked from a capture ellipse down to a low circular Venus orbit.**
Magellan started from an already tight 3.26-hour orbit and stopped at 541 x 197 km; Venus
Express only dipped. No other Venus aerobraking mission was found in this research. The
only source for the full manoeuvre is a design study: NASA/TM-2006-214291 (section 3.3)
prices it at a 2,300 m/s insertion into a 4.4-hour orbit plus 2,011 m/s of drag over
**122 days and 670 orbits**.

#### 3.8.4 Recommended: LEO -> low Venus orbit, aerobraking as flown = 4.4 km/s

**Computed**, Hohmann arrival (v_inf 2.707 km/s), capture at 300 km periapsis:

| Capture ellipse | Apoapsis altitude | Capture burn | Drag must then remove | LEO total with 0.09 trim |
|---|---|---|---|---|
| 9 days (as Venus Express) | 329,000 km | 0.451 | 2.87 | 4.02 |
| **24 hours** (Venus Express's working orbit) | 66,500 km | **0.772** | **2.55** | **4.34** |
| 4.4 hours (the NASA study's orbit) | 13,060 km | 1.708 | 1.61 | 5.28 |

```
recommended = 3.481 depart + 0.772 capture (24 h ellipse) + 0.089 trim = 4.34 -> 4.4
trim: from a 135 x 450 km orbit, raise periapsis to 300 km (0.047), lower apoapsis (0.042)
```

**Range 3.9 - 5.3**: 3.9 is a 9-day ellipse in the best window (3.47 + 0.35 + 0.1); 5.3 is
the study's tight 4.4-hour ellipse.

**The coordinator's remembered 4.8 is not wrong but is inconsistent.** It is 3.48 + 1.25
+ 0.1, pairing a slow (Hohmann) departure with Venus Express's burn, which belongs to a
fast arrival. The Mars figure uses the Hohmann arrival, so the like-for-like Venus figure
is **4.4**. 4.8 sits inside the range.

The honest caveat: the 24-hour case asks the air to remove 2.55 km/s. The most any
mission has shed by aerobraking is about 1.2 km/s (Magellan, MGS). At Venus this is a
design-study manoeuvre, not a flown one, whichever ellipse is chosen.

#### 3.8.5 Phobos and Deimos under the same rule

Foster's NASA Ames budget (section 3.4) answers this directly and this note's formulas
reproduce it. The cheapest propulsive route is **not** the two-burn "direct" route of
section 3.5. It is three burns: capture into a very loose ellipse, a small burn at the far
apoapsis (where speed is only 0.2 km/s) to lift periapsis up to the moon's orbit, then a
burn at the moon to circularise. **Computed** with Foster's staging orbit (250 x 82,173 km,
3.3 days), Hohmann arrival:

```
capture into the staging orbit = sqrt(2.649^2 + 4.847^2) - 4.747 = 0.777
```

| Route from the staging orbit | Phobos | Deimos | Foster's figure |
|---|---|---|---|
| Three-burn, no air (raise periapsis at apoapsis, circularise at moon) | 0.844 | 0.604 | 845 / 604 m/s |
| Two-burn, no air (lower apoapsis at periapsis, circularise at moon) | 1.172 | 0.888 | 1,171 / 888 m/s |
| Aerobrake apoapsis down to the moon's orbit, then circularise | 0.560 (drag removes 0.63) | 0.662 (drag removes 0.23) | 538 / 651 m/s |

(The aerobraked figures here assume a 110 km periapsis during the drag phase; Foster keeps
250 km, hence his slightly lower 538 and 651.)

| From LEO | Direct two-burn (section 3.5) | Three-burn propulsive | Aerobraking as flown | Cheapest honest route |
|---|---|---|---|---|
| Phobos | 5.54 | 3.590 + 0.777 + 0.844 = **5.21** | 3.590 + 0.777 + 0.004 + 0.560 = **4.93** | **4.9, with aerobraking** (saves 0.28) |
| Deimos | 5.26 | 3.590 + 0.777 + 0.604 = **4.97** | 3.590 + 0.777 + 0.004 + 0.662 = 5.03 | **5.0, without aerobraking** |

Foster says the same: the three-burn route is the "Min [delta-v] option for Deimos
regardless of aerobrake capability", and aerobraking is the minimum for Phobos only "if
aerobrake is available".

So: **the 5.5 and 5.3 of section 3.5 are not the right figures.** They are correct for a
two-burn arrival, but a three-burn arrival through a high staging orbit is cheaper by 0.3
and costs only a few days (the staging orbit's period is 3.3 days). Use **Phobos 5.2
propulsive / 4.9 aerobraked** and **Deimos 5.0 either way**. Deimos remains the cheapest
propulsive destination at Mars, and under this rule low Mars orbit (4.6) becomes the
cheapest overall.

No mission has aerobraked to a Mars moon's orbit; the Phobos figure is Foster's design
figure. The drag needed (0.63 km/s) is about half of what Mars orbiters have flown.

**Between low Mars orbit and the moons:** going **up** (1.2, 1.7) and **between** the moons
(0.75) cannot use the air; unaffected. Going **down** can: from Phobos, 0.56 to drop
periapsis into the air, then drag, then about 0.05 to lift periapsis at 300 km = **0.6**
instead of 1.2; from Deimos 0.66 + 0.05 = **0.7** instead of 1.7 (**computed**). Whether
the game prices down-legs differently from up-legs is a design decision.

#### 3.8.6 How long aerobraking adds

| Case | Months | Two-month turns |
|---|---|---|
| Mars Odyssey (flown) | 2.5 (77 days) | 1 - 2 |
| Mars Reconnaissance Orbiter (flown) | 5 (145 days, starting about 3 weeks after arrival) | 3 |
| Trace Gas Orbiter (flown) | 11 (342 days, 2 of them paused), after 5 months of propulsive orbit changes | 6 |
| Mars Global Surveyor (flown) | 17 (planned about 4; hardware fault) | 9 (planned 2) |
| Venus, capture to low orbit (NASA study, not flown) | 4 (122 days) | 2 |
| Magellan (flown, partial) | 2.3 (70 days, for 1.2 km/s) | 1 |
| Phobos (estimate, not flown) | 1 - 3, scaling Odyssey and MRO by the smaller 0.63 km/s | 1 |

A representative figure for Mars is **3 to 6 months: two or three turns**. The spread
between missions comes from spacecraft design (how much heating the solar panels tolerate)
far more than from the orbit, so the game may fairly pick a single number. For Venus, two
turns, from the study alone.

### 3.9 Every journey between the eight places, each direction on its own

Added 2026-10-04 (third pass). The game will price every pair of places, and each
direction separately. Everything in this section is **computed** in this session with the
constants and formulas of section 2 unless marked otherwise; nothing here was found as a
published pairwise table.

#### 3.9.1 Rules used for every cell

- Minimum-energy (Hohmann) transfer between circular coplanar orbits, at a good window.
- **Leaving** a planet's system is always propulsive. A ship leaving a moon first drops to
  a low pass over its planet and burns there (the Oberth saving); from the Moon that is a
  0.82 burn to fall to a 300 km perigee, then the escape burn at perigee; from Phobos or
  Deimos it is the three-burn route of section 3.8.5 run backwards.
- **Arriving where there is air** is "aerobraking as flown":
  - Mars: capture burn at 300 km into a 35-hour ellipse, drag, 0.08 trim (section 3.8.2).
  - Venus: capture burn at 300 km into a 24-hour ellipse, drag, 0.09 trim (section 3.8.4).
  - **Earth (new assumption, mirrors Venus): capture burn at 300 km perigee into a 24-hour
    ellipse (apogee altitude 71,400 km), drag down to 300 km, then 0.10 trim** (from a
    110 x 450 km orbit: raise perigee 0.05, lower apogee 0.05). A ship coming from the
    Moon is already on a bound ellipse, so it needs no capture burn at all: only the trim.
  - Phobos: aerobraked route of section 3.8.5. Deimos: no aerobraking (it does not help).
- Arriving at the Moon, L4 or L5: propulsive, no air.
- L4 and L5 are on Earth's orbit, outside any gravity well. Legs between them and Earth's
  system are phasing orbits (section 3.6), two laps. Legs from them to Venus or Mars are
  the same Hohmann transfers Earth uses.

**Aerobraking into low Earth orbit has not been flown by any mission found in this
research.** The drag required is 2.76 km/s from the 24-hour ellipse and 3.11 km/s on a
return from the Moon, against a flown maximum of about 1.2 km/s. The Earth-arrival figures
are a consistent application of the designer's rule, not a record of practice. One
secondary cross-check exists: Wikipedia's "Delta-v budget" table gives low lunar orbit ->
LEO as **0.90**, stating that "the return to LEO figures assume that a heat shield and
aerobraking/aerocapture are used"; the figure here is 0.92.

#### 3.9.2 The 8 x 8 table (km/s; row = from, column = to)

| From \ To | LEO | Moon | Venus | Mars | Phobos | Deimos | L4 | L5 |
|---|---|---|---|---|---|---|---|---|
| **LEO** | - | 3.93 | 4.34 | 4.52 | 4.93 | 4.97 | 4.14 | 3.99 |
| **Moon** (low lunar orbit) | 0.92 | - | 2.06 | 2.24 | 2.65 | 2.69 | 1.86 | 1.71 |
| **Venus** (low orbit) | 4.14 | 4.52 | - | 6.70 | 7.11 | 7.14 | 5.81 | 5.81 |
| **Mars** (low orbit) | 3.02 | 3.40 | 5.40 | - | 1.20 | 1.72 | 5.04 | 5.04 |
| **Phobos** | 2.55 | 2.93 | 4.93 | 0.61 | - | 0.75 | 4.57 | 4.57 |
| **Deimos** | 2.31 | 2.69 | 4.69 | 0.71 | 0.75 | - | 4.33 | 4.33 |
| **L4** | 1.33 | 1.71 | 3.36 | 3.88 | 4.29 | 4.33 | - | 2.84 |
| **L5** | 1.48 | 1.86 | 3.36 | 3.88 | 4.29 | 4.33 | 2.48 | - |

The LEO row reproduces the legs already in the game (4.0, 4.4, 4.6 with corridor burns,
4.9, 5.0, 4.0) before rounding.

Status of each cell:

| Cells | Status |
|---|---|
| LEO -> Moon | Computed; **sourced** cross-check Apollo 11 3.99 (section 3.1) |
| Moon -> LEO | Computed 0.92; secondary cross-check Wikipedia 0.90. **Unflown** as aerobraking |
| LEO -> Venus, Mars, Phobos, Deimos | Computed; sections 3.8.2 - 3.8.5. Mars capture burn checked against MRO's flown 1,015 m/s |
| Mars <-> Phobos <-> Deimos | Computed; reproduces Foster (NASA Ames) to 1 m/s. Down-legs by aerobraking are **unflown** |
| Everything arriving at LEO from Venus, Mars, Phobos, Deimos, L4, L5 | Computed under the Earth assumption above; **unflown** |
| Venus <-> Mars, Venus <-> Phobos / Deimos | Computed; **no source found**; see 3.9.3 |
| All L4 / L5 cells | Computed; the stop burn at two laps is corroborated by EASCO and LISAmax (section 3.6). L4 <-> L5 is computed alone, **no source** |
| All Moon <-> interplanetary cells | Computed; assume a burn at a 300 km Earth perigee on the way out or in. **No source** |

How each return to LEO is made up:

```
Moon   -> LEO: 0.823 leave lunar orbit + 0 capture + 0.098 trim                  = 0.92
Mars   -> LEO: 2.091 leave low Mars orbit (v_inf 2.649)
               + [sqrt(2.945^2 + 10.926^2) - 10.485] = 0.831 capture + 0.098     = 3.02
Venus  -> LEO: 3.318 leave low Venus orbit (v_inf 2.707)
               + [sqrt(2.495^2 + 10.926^2) - 10.485] = 0.722 capture + 0.098     = 4.14
L4     -> LEO: 0.764 leave L4 + 0.467 capture + 0.098                            = 1.33
L5     -> LEO: 0.903 leave L5 + 0.478 capture + 0.098                            = 1.48
Phobos -> LEO: 0.844 + 0.777 (three-burn, backwards) + 0.831 + 0.098             = 2.55
Deimos -> LEO: 0.604 + 0.777 + 0.831 + 0.098                                     = 2.31
```

(10.926 is escape speed at 300 km; 10.485 is perigee speed of the 24-hour ellipse.)

The asymmetry the designer asked for is large: LEO -> Mars 4.52 but Mars -> LEO 3.02;
LEO -> Moon 3.93 but Moon -> LEO 0.92. The cheap direction costs months (3.9.5).

#### 3.9.3 Venus <-> Mars, and Venus <-> Phobos / Deimos

Hohmann between 0.72333566 and 1.52371034 au: **217.5 days**; v_inf leaving or reaching
Venus **5.763 km/s**, at Mars **4.768 km/s**. These are about twice the Earth-Mars and
Earth-Venus figures, because the two orbits are far apart.

```
Venus -> Mars:   leave low Venus orbit sqrt(5.763^2 + 10.114^2) - 7.151 = 4.490
                 capture at Mars to 35 h ellipse sqrt(4.768^2 + 4.814^2) - 4.639 = 2.136
                 trim 0.078                                        total = 6.70
Mars -> Venus:   leave low Mars orbit sqrt(4.768^2 + 4.814^2) - 3.404 = 3.372
                 capture at Venus to 24 h ellipse sqrt(5.763^2 + 10.114^2) - 9.698 = 1.943
                 trim 0.089                                        total = 5.40
Venus -> Phobos 7.11, -> Deimos 7.14;  Phobos -> Venus 4.93, Deimos -> Venus 4.69
```

**How practical is it? Physically sound, never flown as a journey, and dearer than either
Earth leg - but cheaper than going by way of Earth.** Venus -> LEO -> Mars with a stop is
4.14 + 4.52 = 8.66 against 6.70 direct; Mars -> LEO -> Venus is 3.02 + 4.34 = 7.36 against
5.40. The window recurs every **333.9 days**, more often than either Earth window. No
published Venus-to-Mars orbit-to-orbit budget was found; the only related source read is
Foster's short-stay Mars mission, which uses a Venus **flyby** on the way (section 3.4
reference), showing the geometry is used in real mission design. The arrival speeds are
the weak point: 4.77 km/s at Mars and 5.76 km/s at Venus are faster than the arrivals of
the missions in section 3.8 (MRO 2.96, Venus Express about 5.0), so capture burns are
large (2.1 and 1.9) and real windows will vary more than Earth's do. Trust +/- 0.5.

#### 3.9.4 From L4 or L5 outward

A ship at L4 or L5 has already paid Earth's 3.20 km/s escape. But it has also lost the
Oberth saving: its departure burn is made in open space, so the full heliocentric speed
change is paid at face value.

| Leg | From L4 / L5 | From LEO | Saving | Why |
|---|---|---|---|---|
| -> Mars (low orbit) | 2.945 + 0.934 = **3.88** | 4.52 | 0.64 | LEO pays 3.59 to get v_inf 2.945; L4 pays 2.945 |
| -> Venus (low orbit) | 2.495 + 0.861 = **3.36** | 4.34 | 0.98 | LEO pays 3.48; L4 pays 2.495 |
| -> Moon | **1.71** (L4), **1.86** (L5) | 3.93 | - | Phasing back to Earth, perigee burn 0.12 - 0.13, lunar insertion 0.82 |
| L4 -> L5 | **2.84**, 2.33 years | - | - | Drop back 120 deg in two laps (v_inf 1.421 twice) |
| L5 -> L4 | **2.48**, 2.67 years | - | - | Advance 120 deg in three laps (v_inf 1.243 twice) |

So being "outside the well" is worth only 0.6 - 1.0 km/s toward a planet, not 3.2.
Coming back is worse: Mars -> L4 is **5.04** against Mars -> LEO 3.02, because L4 has no
air and no well to brake against.

L4 <-> L5 is a pure time trade, like section 3.6:

| Laps | L4 -> L5 | L5 -> L4 |
|---|---|---|
| 1 | 4.99 in 1.33 yr | 10.10 in 0.67 yr |
| 2 | **2.84 in 2.33 yr** | 3.98 in 1.67 yr |
| 3 | 1.99 in 3.33 yr | **2.48 in 2.67 yr** |
| 4 | 1.53 in 4.33 yr | 1.81 in 3.67 yr |

#### 3.9.5 Flight times and windows (60-day turns)

| Pair | Coast, days | Turns | Window recurs | Notes |
|---|---|---|---|---|
| Earth system <-> Moon | 5 | under 1 | none needed | |
| Earth system <-> Mars system | 259 | 4.3 | 779.9 days (13 turns) | Outbound: Mars 44.3 deg ahead. Return: Earth 75.1 deg behind Mars. Different dates |
| Earth system <-> Venus | 146 | 2.4 | 583.9 days (9.7 turns) | Outbound: Venus 54.0 deg behind Earth. Return: Earth 36.0 deg ahead of Venus |
| Venus <-> Mars system | 217 | 3.6 | **333.9 days (5.6 turns)** | Outbound: Mars 66.0 deg ahead of Venus. Return: Venus 168.4 deg behind Mars |
| L4 or L5 <-> Mars system | 259 | 4.3 | 779.9 days | Same period as Earth's, but about **130 days earlier for L4, 130 days later for L5** (60 deg at 0.4616 deg/day) |
| L4 or L5 <-> Venus | 146 | 2.4 | 583.9 days | About **97 days later for L4, 97 days earlier for L5** (60 deg at 0.6165 deg/day) |
| LEO or Moon -> L5; L4 -> LEO or Moon | 791 | 13.2 | **no window** | Two-lap phasing; any date |
| LEO or Moon -> L4; L5 -> LEO or Moon | 670 | 11.2 | **no window** | Two-lap phasing; any date |
| L4 -> L5 | 852 | 14.2 | **no window** | |
| L5 -> L4 | 974 | 16.2 | **no window** | |
| Within Mars system | under 1 | 0 | none needed | Hours |

Moon legs to the planets add 5 days and must catch the Moon on the right side of its
27-day orbit, which a 60-day turn swallows.

**Aerobraking then adds to the arrival** (section 3.8.6): Mars 3 - 6 months (2 - 3 turns),
flown; Venus about 4 months (2 turns), one study; **Earth about 4 - 6 months (2 - 3
turns), an estimate** scaled from the Venus study (2.0 km/s in 122 days) to Earth's
2.8 - 3.1 km/s, with no source. That estimate matters most on the Moon run: Moon -> LEO is
0.92 with months of aerobraking, or 3.93 in five days without it.

#### 3.9.6 The simplest honest pricing structure

**The candidate works. Recommended.** journey = leave(from) + gulf(system, system) +
arrive(to), with journeys inside one system keeping their own figure.

It works for a physical reason: every place in a system makes its escape or capture burn
at the same low pass over its planet, so "leave" and "arrive" can be measured to the edge
of the well (v_inf = 0) and the gulf is whatever remains, and that remainder is nearly the
same for every pair of places in the two systems.

| Place | Leave | Arrive | |
|---|---|---|---|
| LEO | 3.20 | 0.54 | arrive = capture to 24 h ellipse from escape speed, + trim |
| Moon | 0.92 | 0.92 | no air |
| Venus | 2.96 | 0.50 | |
| Mars | 1.41 | 0.25 | |
| Phobos | 0.94 | 0.66 | arrive uses aerobraking |
| Deimos | 0.70 | 0.70 | aerobraking does not help |
| L4, L5 | 0 | 0 | |

| Gulf (same both ways) | km/s |
|---|---|
| Earth - Venus | 0.64 |
| Earth - Mars | 1.07 |
| Earth - L4, Earth - L5 | 0.87 |
| Venus - Mars | 3.48 |
| Venus - L4, Venus - L5 | 2.85 |
| Mars - L4, Mars - L5 | 3.62 |
| L4 - L5 | 2.66 |

Local figures kept as they are (8): LEO -> Moon 3.93, Moon -> LEO 0.92, Mars -> Phobos
1.20, Mars -> Deimos 1.72, Phobos -> Mars 0.61, Deimos -> Mars 0.71, Phobos <-> Deimos
0.75 each way.

**Error against the full table, all 48 between-system cells:**

- Worst error **0.18 km/s**, on L4 <-> L5 (the true figures are 2.84 and 2.48; one gulf
  gives 2.66 both ways).
- Next worst **0.08**, on the Earth - L4 / L5 legs (L4 and L5 differ slightly by direction).
- **Every other cell is within 0.01 km/s.**

Example: LEO -> Mars = 3.20 + 1.07 + 0.25 = 4.52. Mars -> LEO = 1.41 + 1.07 + 0.54 = 3.02.
Venus -> Deimos = 2.96 + 3.48 + 0.70 = 7.14.

That is 16 leave/arrive figures (four of them zero), 10 gulfs (7 distinct values) and 8
local figures, in place of 56 cells. If the L4 / L5 asymmetry matters, make four gulfs
directional (LEO-side -> L4 0.94, -> L5 0.79, L4 -> Earth 0.79, L5 -> Earth 0.94, L4 -> L5
2.84, L5 -> L4 2.48) and the worst error falls to 0.01.

A cruder structure with **no gulf at all** (a best-fit leave and arrive per place) was also
tested: worst error 0.59, on L5 -> L4. Not recommended.

A least-squares fit of the candidate (free leave, arrive and gulf) gives the same worst
error, 0.17, so the hand-built edge-of-the-well figures above are as good as any.

---

## 4. What is uncertain

00. **The pairwise table (added 2026-10-04, third pass).** Least trusted, in order:
   (a) **every arrival at LEO by aerobraking** - unflown, the drag is 2.8 - 3.1 km/s, and
   the 4 - 6 month duration is an unsourced estimate; (b) **Venus <-> Mars** - no source,
   fast arrivals, trust +/- 0.5; (c) **L4 <-> L5** - computed alone; (d) the Moon <->
   planet legs assume a perigee burn at 300 km with ideal timing. The window offsets for
   L4 and L5 (130 and 97 days) are simple circular-orbit arithmetic.

0. **Venus "aerobraking as flown" (added 2026-10-04; now the least trusted figure with
   L4/L5).** It has not been flown. The 4.4 depends on choosing a 24-hour capture ellipse
   and on the air removing 2.55 km/s, twice anything achieved. The 4-month duration is one
   design study's. Mars Odyssey's insertion burn, Magellan's insertion burn, MRO's exit
   burn and TGO's exact insertion figure were not found in primary sources. The Phobos
   aerobraking time is an estimate. The 0.1 trim is computed and agrees with MGS (62 m/s)
   and TGO (51 m/s).

1. **Sun-Earth L4/L5 (least trusted).** No source gives a LEO-to-L4/L5 total. The figure
   is this note's two-body phasing calculation, corroborated by published studies only at
   the two-year point (EASCO: C3 ~1.0, ~950 m/s). Trust to about +/- 0.3 km/s. The
   manifold figures (260 - 550 m/s over 4 - 9 years) are **UNVERIFIED**.
2. **Venus window range.** The handbook scan is poor, three opportunities were illegible,
   and it gives separate minima rather than a joint minimum. The 7.3 upper end is an
   estimate.
3. **Aerocapture's last 0.1 km/s.** The post-capture burn is 0.06 computed for Mars and
   99 m/s in the one NASA Venus study read. No Mars aerocapture systems study was read
   for a matching figure. Every aerocapture figure here is
   a design-study figure. Heat-shield mass is not counted.
4. **Mars window figures are upper bounds per window**, built from the handbook's four
   corner trajectories rather than a minimised sum. The row labelling of those tables
   (which rows are Type I and which Type II) is inferred, as recorded in
   `earth-mars-ephemeris.md`; it does not affect the totals.
5. **Everything computed is idealised**: impulsive burns, circular coplanar planets, no
   plane changes, no gravity losses, no margins. Real budgets add 2 - 10%. The Mars-moon
   figures assume an equatorial low Mars orbit.
6. **Orbit altitudes are a choice.** 300 km (100 km at the Moon) throughout. Changing LEO
   between 200 and 500 km moves any leg by at most 0.06 km/s.
7. **Wikipedia's Mars departure figure (4.3) is unexplained** and disagrees with both the
   calculation (3.59) and the NASA handbook windows (3.55 - 3.86). It was not used.
8. **Moon radius** (1,737.4 km) and the Wikipedia table values were taken through a
   summarising fetch rather than read character by character; the Wikipedia Earth-Moon
   row was read twice with the same result.

---

## References

| URL | What was taken from it |
|---|---|
| <https://ssd.jpl.nasa.gov/astro_par.html> | GM of Sun, Earth, Moon, Mars system, Venus; the astronomical unit. Fetched with curl. |
| <https://ssd.jpl.nasa.gov/planets/phys_par.html> | Radii of Earth, Mars, Venus. Fetched with curl. |
| <https://ssd.jpl.nasa.gov/planets/approx_pos.html> | Semimajor axes of Venus, Earth-Moon barycentre, Mars (Table 1). Fetched with curl. |
| <https://ssd.jpl.nasa.gov/sats/elem/> | Semimajor axes of the Moon (384,400 km), Phobos (9,375 km), Deimos (23,457 km). Fetched with curl. |
| <https://www.nasa.gov/wp-content/uploads/2023/04/sp-4029.pdf> | Orloff, *Apollo by the Numbers*, NASA SP-4029. Apollo 11 velocity changes: translunar injection 10,008.1 ft/s, lunar orbit insertion 2,917.5 ft/s, circularisation 158.8 ft/s, with the orbits before and after. PDF downloaded and text-extracted. |
| <https://ntrs.nasa.gov/api/citations/20190026585/downloads/20190026585.pdf> | Foster, *Delta-V Budgets for Robotic and Human Exploration of Phobos and Deimos*, NASA Ames, 2011 (slides). Phobos <-> Deimos 748 m/s; staging orbit to Phobos 845 / 1,171 / 538 m/s and to Deimos 604 / 888 / 651 m/s; staging-orbit definition; the out-of-plane caveat. PDF text-extracted. |
| <https://ntrs.nasa.gov/api/citations/20100037210/downloads/20100037210.pdf> | Burke, Falck, McGuire, NASA/TM-2010-216764. C3 and Mars arrival excess speed for the 2026 - 2045 opportunities (Tables 1 - 11). PDF text-extracted. |
| <https://ntrs.nasa.gov/api/citations/19840019711/downloads/19840019711.pdf> | Sergeyevsky and Yin, JPL Publication 82-43 Vol 1 Pt 1, Earth to Venus 1991 - 2005. Energy-minima C3L and VHP per opportunity. Scanned PDF, text-extracted with visible OCR damage. |
| <https://ntrs.nasa.gov/api/citations/20060010899/downloads/20060010899.pdf> | Lockwood, Starr et al., NASA/TM-2006-214291, *Systems Analysis for a Venus Aerocapture Mission*. 99 m/s circularisation after aerocapture; 4,300 m/s removed in the pass; 2,300 m/s + 2,011 m/s aerobraking alternative; entry speed 11.25 km/s; C3 = 8.3. PDF text-extracted. |
| <https://arxiv.org/pdf/2304.08287> | Martens, Khan (ESA Mission Analysis Section), Bayle, *LISAmax*. The phasing rule for reaching Sun-Earth L4/L5 and the statement that delta-v falls with transfer duration; two-lap chemical transfers; Figure 6 axis range 700 - 1,300 m/s. PDF text-extracted. |
| <https://arxiv.org/pdf/1109.2929> | Gopalswamy et al., *Earth-Affecting Solar Causes Observatory (EASCO): a mission at the Sun-Earth L5*. Two-year transfer; chemical C3 ~1.0 km^2/s^2 and ~950 m/s; electric C3 ~2.2 and ~1.5 km/s. PDF text-extracted. |
| <https://arxiv.org/pdf/2303.02895> | Gopalswamy et al., *The Multiview Observatory for Solar Terrestrial Science (MOST)*. Post-launch budgets of ~464 - 522 m/s to L4/L5-region stations via lunar gravity assist. PDF text-extracted. |
| <https://en.wikipedia.org/wiki/Delta-v_budget> | **Secondary cross-check only.** LEO to low lunar orbit 4.04, to Earth-Moon L1 3.77, L2 3.43, escape 3.22; Mars chain 4.3 / 0.9 / 1.4 / 2.7 and the Phobos and Deimos steps; Venus 3.5. Read through a summarising fetch. |

Added 2026-10-04 for section 3.8:

| URL | What was taken from it |
|---|---|
| <https://ntrs.nasa.gov/api/citations/19980201716/downloads/19980201716.pdf> | Esposito et al. (JPL), *Mars Global Surveyor Navigation and Aerobraking at Mars*, AAS 98-384. Insertion burn 973.0 m/s; capture orbit 44.993 h, 262.9 x 54,025.9 km; first-phase history. PDF text-extracted. |
| <https://ntrs.nasa.gov/api/citations/20000056881/downloads/20000056881.pdf> | Johnston et al. (JPL), *The Strategy for the Second Phase of Aerobraking Mars Global Surveyor*. Termination 4 February 1999; 61.9 m/s exit burn; trim burns; the 1,250 m/s planned deficit. |
| <https://ntrs.nasa.gov/api/citations/20000057306/downloads/20000057306.pdf> | Lyons (JPL), *Aerobraking at Venus and Mars: A Comparison of the Magellan and Mars Global Surveyor Aerobraking Phases*. About 1,200 m/s removed by drag at each; periods 45 h -> 1.89 h and 3.26 h -> 1.5 h. |
| <https://ntrs.nasa.gov/api/citations/20030005808/downloads/20030005808.pdf> | Tartabini, Munk, Powell (NASA Langley), AIAA 2002-4537, Mars Odyssey. 18.6 h capture orbit, 292 km periapsis, 77 days, 332 passes, 1.08 km/s saved. |
| <https://pds-geosciences.wustl.edu/mro/mro-m-rss-1-magr-v1/mrors_0xxx/catalog/mission.cat> | NASA PDS mission catalogue for MRO. Insertion 1,015 m/s, about 26 minutes; 35 h capture orbit; aerobraking phases and exit at 450 km apoapsis. |
| <https://ntrs.nasa.gov/api/citations/20070010460/downloads/20070010460.pdf> | Prince and Striepe (NASA Langley), AAS 07-244, MRO aerobraking. 145 days, 4 April - 30 August 2006, 26 corridor burns. |
| <https://issfd.org/ISSFD_2019/ISSFD_2019_AIAC18_Bellei-Gabriele.pdf> | Bellei et al. (ESA/ESOC), TGO aerobraking navigation. 4-sol capture, propulsive change to 1-sol, 342 days, 952 passes, 1,017 m/s from drag, 51 m/s manoeuvres, 9 m/s attitude control. Operator conference paper, marked non-peer-review. |
| <https://issfd.org/ISSFD_2019/ISSFD_2019_AIAC18_Guilanyà_Jané-Robert_2.pdf> | Guilanya and Rivero (ESA/ESOC), TGO commanding. Walk-out burns 8.6 and 22.5 m/s. |
| <https://www.esa.int/Newsroom/Press_Releases/ExoMars_TGO_reaches_Mars_orbit_while_EDM_situation_under_assessment> | ESA press release: 139-minute burn, "more than 1.5 km/s". |
| <https://sci.esa.int/web/venus-express/-/38947-orbit-insertion> | ESA: Venus Express burn "approximately 1251" m/s, about 50 minutes; 400 x 350,000 km, 9-day capture orbit; table of the seven follow-up burns. |
| <https://sci.esa.int/web/venus-express/-/39112-successful-orbit-insertion> | ESA status report: "commanded delta-V was 1251" m/s. Read through a summarising fetch. |
| <https://www.esa.int/Newsroom/Press_Releases/Venus_Express_goes_gently_into_the_night> | ESA: the 24-hour, 66,000 km operational orbit and the 2014 aerobraking campaign. The 129.2 km minimum, the 18 June - 11 July dates and the 22 h result came from a search summary of this release, not a character-by-character read. |
| <https://ntrs.nasa.gov/citations/20210004680> | NTRS abstract, *The Magellan Venus Mapping Mission: Aerobraking Operations*. 70 days, ended 3 August 1993, apoapsis 8,467 -> 541 km, final 541 x 197 km. Abstract only; no PDF on NTRS. |

### Sources that could not be reached, and things not verified

- **Mars Odyssey's orbit-insertion delta-v** was not found in any source read (only its
  19-minute duration and 18.6 h result). **Magellan's** insertion delta-v likewise. TGO's
  "1,550 m/s" appears only in press coverage; ESA's own release says "more than 1.5 km/s".

- <https://recercat.cat/handle/2117/394633> (UPC thesis on manifold transfers to
  Sun-Earth L4/L5) returned HTTP 403. Its figures appear above only as **UNVERIFIED**.
- <https://iafastro.directory/iac/archive/browse/IAC-11/C1/4/11586/> ("Optimal Low-Cost
  Transfer to L4 and L5 Lagrangian points") was fetched and turned out to concern the
  **Earth-Moon** L4/L5 points, with no figures in the abstract. Not used.
- The well-known community solar-system delta-v maps were not fetched; none of the
  figures above depend on them.
- No Mars aerocapture systems study was read; the Mars post-aerocapture burn is computed.

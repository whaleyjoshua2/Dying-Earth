# Earth-Mars ephemeris, launch windows and transfer-cost research

**Date:** 2026-09-10
**Branch:** version-0.05 (research note only; no code changed)

**What this is for:** the game needs to place Earth and Mars on their real orbits over
its 24 monthly turns (Jan 2030 - Dec 2031), decide when a Mars launch window is open,
and price off-window departures. This note gathers the primary-source numbers needed to
implement that, plus a test oracle to check the implementation against, plus a
sanity-check of the proposed cost-model defaults.

Everything below was fetched from a live source during this session unless explicitly
marked **UNVERIFIED**. Nothing in this file is stated from memory.

---

## 1. JPL Keplerian elements for approximate positions of the major planets (Standish)

**Source page:** <https://ssd.jpl.nasa.gov/planets/approx_pos.html>
**Linked PDF (confirmed live, HTTP 200):** <https://ssd.jpl.nasa.gov/txt/aprx_pos_planets.pdf>
The page credits the underlying fit to "an article written by E.M. Standish and
J.G. Williams in 1992".

### 1.1 Table 1 - elements and rates, valid 1800 AD - 2050 AD

JPL's own header for this table:

> Keplerian elements and their rates, with respect to the mean ecliptic and equinox
> of J2000, valid for the time-interval 1800 AD - 2050 AD.

Column units, exactly as JPL prints them:

```
                 a              e               I                L            long.peri.      long.node.
             au, au/Cy     rad, rad/Cy      deg, deg/Cy      deg, deg/Cy      deg, deg/Cy     deg, deg/Cy
```

(Note: JPL labels the `e` column "rad, rad/Cy". Eccentricity is dimensionless; this is a
formatting artefact of the shared unit row. Treat `e` as a pure number.)

Rows for the two bodies the game needs, reproduced exactly:

```
EM Bary   1.00000261      0.01671123     -0.00001531      100.46457166     102.93768193        0.0
          0.00000562     -0.00004392     -0.01294668    35999.37244981       0.32327364        0.0
Mars      1.52371034      0.09339410      1.84969142       -4.55343205     -23.94362959       49.55953891
          0.00001847      0.00007882     -0.00813131    19140.30268499       0.44441088       -0.29257343
```

As a Markdown table (first line of each pair = value at J2000, second = rate per Julian
century):

| Body | | a (au) | e | I (deg) | L (deg) | long.peri. varpi (deg) | long.node. Omega (deg) |
|---|---|---|---|---|---|---|---|
| **EM Bary** | J2000 | 1.00000261 | 0.01671123 | -0.00001531 | 100.46457166 | 102.93768193 | 0.0 |
| **EM Bary** | per Cy | 0.00000562 | -0.00004392 | -0.01294668 | 35999.37244981 | 0.32327364 | 0.0 |
| **Mars** | J2000 | 1.52371034 | 0.09339410 | 1.84969142 | -4.55343205 | -23.94362959 | 49.55953891 |
| **Mars** | per Cy | 0.00001847 | 0.00007882 | -0.00813131 | 19140.30268499 | 0.44441088 | -0.29257343 |

JPL footnote on the same page: `EM Bary = Earth/Moon Barycenter`.

For completeness, the same two rows from **Table 2a** (valid 3000 BC - 3000 AD; the game
does not need these, but they are easy to confuse with Table 1 and must not be mixed in):

| Body | | a (au) | e | I (deg) | L (deg) | varpi (deg) | Omega (deg) |
|---|---|---|---|---|---|---|---|
| EM Bary | J2000 | 1.00000018 | 0.01673163 | -0.00054346 | 100.46691572 | 102.93005885 | -5.11260389 |
| EM Bary | per Cy | -0.00000003 | -0.00003661 | -0.01337178 | 35999.37306329 | 0.31795260 | -0.24123856 |
| Mars | J2000 | 1.52371243 | 0.09336511 | 1.85181869 | -4.56813164 | -23.91744784 | 49.71320984 |
| Mars | per Cy | 0.00000097 | 0.00009149 | -0.00724757 | 19140.29934243 | 0.45223625 | -0.26852431 |

The extra `b, c, s, f` terms in Table 2b apply **only to Jupiter through Neptune** and
only with the Table 2a element set. For Earth and Mars with Table 1, `b = c = s = f = 0`.

### 1.2 The algorithm, as JPL states it

Quoted / transcribed from the same page. `T_eph` is a Julian ephemeris date, "equivalent
to the post-2006 IAU Barycentric Dynamical Timescale, JDTDB".

**Step 1 - propagate the elements.**

```
T = (T_eph - 2451545.0) / 36525          # centuries past J2000.0
a = a0 + adot * T,  and likewise for e, I, L, varpi, Omega
```

**Step 2 - argument of perihelion and mean anomaly.**

```
omega = varpi - Omega
M     = L - varpi + b*T^2 + c*cos(f*T) + s*sin(f*T)
```

**Step 3 - reduce M and solve Kepler's equation.**

> Adjust the mean anomaly M to its equivalent angle in the range -180 deg <= M <= +180 deg
> and then obtain the eccentric anomaly, E, from the solution of Kepler's equation:

```
M = E - e_star * sin E        where  e_star = (180/pi) * e = 57.29578 * e
```

with M, E and `e_star` all in **degrees**. JPL's stated iteration:

```
E_0 = M + e_star * sin M
loop:
  dM   = M - (E_n - e_star * sin E_n)
  dE   = dM / (1 - e * cos E_n)          # note: plain e here, not e_star
  E_n1 = E_n + dE
until |dE| <= tol,  tol = 1e-6 degrees
```

JPL's footnote: "This starting guess ensures faster convergence, but E_0 = M or
E_0 = M - e_star sin M could also be used."

**Step 4 - heliocentric coordinates in the orbital plane** (x' axis from focus to
perihelion):

```
x' = a * (cos E - e)
y' = a * sqrt(1 - e^2) * sin E
z' = 0
```

**Step 5 - rotate into the J2000 ecliptic plane**, `r_ecl = Rz(-Omega) Rx(-I) Rz(-omega) r'`,
applied as:

```
x_ecl = ( cos omega * cos Omega - sin omega * sin Omega * cos I) * x'
      + (-sin omega * cos Omega - cos omega * sin Omega * cos I) * y'

y_ecl = ( cos omega * sin Omega + sin omega * cos Omega * cos I) * x'
      + (-sin omega * sin Omega + cos omega * cos Omega * cos I) * y'

z_ecl = ( sin omega * sin I) * x'
      + ( cos omega * sin I) * y'
```

**Step 6 (optional) - ICRF / J2000 equatorial coordinates**, with the J2000 obliquity
`epsilon = 23.43928 deg`:

```
x_eq = x_ecl
y_eq =  cos(eps) * y_ecl - sin(eps) * z_ecl
z_eq =  sin(eps) * y_ecl + cos(eps) * z_ecl
```

The game only needs heliocentric ecliptic longitude, so **step 6 is not required**;
longitude is `atan2(y_ecl, x_ecl)` from step 5.

### 1.3 JPL's stated accuracy

From the "Accuracy" table on the same page - "nominal errors in heliocentric longitude
lambda, latitude phi, and distance rho":

| Body | 1800-2050: lambda (arcsec) | phi (arcsec) | rho (1000 km) | 3000 BC-3000 AD: lambda | phi | rho |
|---|---|---|---|---|---|---|
| **EM Bary** | 20 | 8 | 6 | 40 | 15 | 15 |
| **Mars** | 40 | 2 | 25 | 100 | 40 | 30 |

20 arcsec = 0.0056 deg; 40 arcsec = 0.0111 deg. Both are **three orders of magnitude
inside the 1-degree test tolerance**, so the approximation is not the limiting factor
for anything the game does.

### 1.4 Independent confirmation that the algorithm as written reproduces reality

I implemented steps 1-5 exactly as above with the Table 1 rows and evaluated at
JD 2462502.5 (2030-01-01 00:00 TDB, T = 0.3 exactly), then differenced against JPL
Horizons (section 2):

| Body | JPL-approx longitude | Horizons longitude | error |
|---|---|---|---|
| EM Bary | 100.1823 deg | 100.1845 deg | **-0.0023 deg** (8.3 arcsec) |
| Mars | 337.8311 deg | 337.8203 deg | **+0.0108 deg** (39 arcsec) |

Both land inside JPL's own quoted error bars, which is a useful end-to-end check that
the transcription above has no typos in it.

---

## 2. Reference positions for 2030-01-01 (test oracle)

**Source: JPL Horizons API**, reached successfully with `curl` from this machine.
API base: <https://ssd.jpl.nasa.gov/api/horizons.api> (docs:
<https://ssd-api.jpl.nasa.gov/doc/horizons.html>; web UI:
<https://ssd.jpl.nasa.gov/horizons/app.html>)

### 2.1 The query

State-vector query, one call per body (`COMMAND` = 3 for the Earth-Moon barycentre,
399 for Earth, 499 for Mars):

```
https://ssd.jpl.nasa.gov/api/horizons.api
  ?format=text
  &COMMAND='3'            (or '399' / '499')
  &OBJ_DATA='NO'
  &MAKE_EPHEM='YES'
  &EPHEM_TYPE='VECTORS'
  &CENTER='500@10'        (Sun body centre -> heliocentric)
  &START_TIME='2030-01-01'
  &STOP_TIME='2030-01-02'
  &STEP_SIZE='1d'
  &REF_PLANE='ECLIPTIC'   (ecliptic and mean equinox of reference epoch)
  &REF_SYSTEM='ICRF'      (-> J2000 ecliptic frame)
  &VEC_TABLE='3'
  &OUT_UNITS='AU-D'
```

Frame confirmed by the returned header (`Reference frame: Ecliptic of J2000.0`),
ephemeris `DE441`.

### 2.2 Returned values, verbatim

Earth-Moon barycentre (`COMMAND='3'`):

```
$$SOE
2462502.500000000 = A.D. 2030-Jan-01 00:00:00.0000 TDB
 X =-1.738716158800040E-01 Y = 9.678366138373879E-01 Z =-6.496211954734581E-05
 VX=-1.721321910553961E-02 VY=-3.106235206093796E-03 VZ= 3.001570262969446E-07
 LT= 5.679243862748527E-03 RG= 9.833305924830275E-01 RR=-1.366576294191714E-05
$$EOE
```

Earth centre (`COMMAND='399'`):

```
$$SOE
2462502.500000000 = A.D. 2030-Jan-01 00:00:00.0000 TDB
 X =-1.738559342884080E-01 Y = 9.678616961981825E-01 Z =-6.371886579192515E-05
 VX=-1.721963337094923E-02 VY=-3.102273020980568E-03 VZ=-3.218872495253112E-07
 LT= 5.679370430125360E-03 RG= 9.833525069449979E-01 RR=-8.985343275369071E-06
$$EOE
```

Mars (`COMMAND='499'`):

```
$$SOE
2462502.500000000 = A.D. 2030-Jan-01 00:00:00.0000 TDB
 X = 1.278622087587370E+00 Y =-5.212680377259730E-01 Z =-4.227172872477204E-02
 VX= 5.814118067526615E-03 VY= 1.415514012801500E-02 VZ= 1.541391193308730E-04
 LT= 7.978542987988632E-03 RG= 1.381441894930928E+00 RR= 3.541367599683826E-05
$$EOE
```

### 2.3 Derived longitudes (`lon = atan2(y, x)`, normalised to 0-360)

**Epoch: 2030-01-01 00:00:00.0000 TDB = JD 2462502.5. Frame: heliocentric, ecliptic and
mean equinox of J2000 (Horizons `REF_PLANE='ECLIPTIC'`, `REF_SYSTEM='ICRF'`), source
DE441.**

| Body | Heliocentric ecliptic longitude (deg) | latitude (deg) | radius (au) |
|---|---|---|---|
| Earth-Moon barycentre (3) | **100.1845** | -0.0038 | 0.9833306 |
| Earth centre (399) | **100.1834** | -0.0037 | 0.9833525 |
| Mars (499) | **337.8203** | -1.7535 | 1.3814419 |

Earth centre and EM barycentre differ by 0.0011 deg on this date - irrelevant at
1-degree tolerance, so the test can use either.

**Earth -> Mars phase angle on 2030-01-01** (Mars longitude minus Earth longitude,
wrapped to -180..+180):

- using EM barycentre: **-122.3643 deg**
- using Earth centre: **-122.3631 deg**

Round figure for the test: **phase = -122.36 deg** (Mars trails Earth by about 122 deg
at the start of the game).

### 2.4 Cross-check with a second Horizons product

Horizons observer table quantity 18, "Geometric heliocentric J2000 ecliptic longitude
and latitude of target center at the instant light leaves it to be observed at print
time (down-leg light-time corrected)", printed for 2030-Jan-01 00:00 UT:

```
 Date__(UT)__HR:MN     hEcl-Lon hEcl-Lat
 2030-Jan-01 00:00     337.8132  -1.7536     (Mars, observed from Earth)
 2030-Jan-01 00:00     100.1719  -0.0037     (Earth, observed from Mars)
 2030-Jan-01 00:00     100.1731  -0.0038     (EM Bary, observed from Mars)
```

These sit 0.004-0.012 deg below the instantaneous vector values, exactly as expected
from the light-time retardation (~9.6 min each way at ~1.15 au separation) and the
UT-vs-TDB offset of ~72 s. **Use the vector values in section 2.3 as the oracle**, which
are geometric and instantaneous, and therefore unambiguous.

Confidence: **high**. Horizons was reached directly, values are quoted verbatim, and
two independent Horizons products plus the independently-implemented Standish
approximation all agree to well under 0.02 deg.

---

## 3. Real Earth-to-Mars launch windows around the game's span

### 3.1 Synodic period

- **NASA NSSDC Mars Fact Sheet: "Synodic period (days) 779.94"**
  <https://nssdc.gsfc.nasa.gov/planetary/factsheet/marsfact.html>
  (The same page gives Mars sidereal orbit period 686.980 d, semimajor axis
  227.956e6 km / 1.52366231 au, eccentricity 0.09341233, inclination 1.85061 deg. The
  Earth Fact Sheet <https://nssdc.gsfc.nasa.gov/planetary/factsheet/earthfact.html>
  gives sidereal orbit period 365.256 d, semimajor axis 149.598e6 km.)
  *Fetch note: this host's certificate store could not validate the NSSDC chain, so the
  page was retrieved with chain validation disabled. The value is independently
  confirmed below, so this does not weaken the finding.*
- **Derived independently** from the JPL Table 1 semimajor axes in section 1.1
  (a_E = 1.00000261 au, a_M = 1.52371034 au, Kepler's third law, Julian year
  365.25636 d): **779.93 days = 25.62 months**. Agrees with NSSDC to 0.01 d.
- NASA's own plain-language statement:
  <https://science.nasa.gov/planetary-science/programs/mars-exploration/mission-timeline/>
  > "about once every 26 months they are aligned in a way that allows the most
  > energy-efficient trip to Mars"

**Use 779.9 days / 25.6 months. Confirmed.**

### 3.2 Actual launch opportunities - NASA primary source

**Source:** L. M. Burke, R. D. Falck, M. L. McGuire, *Interplanetary Mission Design
Handbook: Earth-to-Mars Mission Opportunities 2026 to 2045*, NASA/TM-2010-216764,
NASA Glenn Research Center, October 2010.
Citation page: <https://ntrs.nasa.gov/citations/20100037210>
PDF: <https://ntrs.nasa.gov/api/citations/20100037210/downloads/20100037210.pdf>

Trajectories were generated with MIDAS, a patched-conic interplanetary trajectory
optimiser, using Lambert's theorem for the ballistic transfers. The handbook explains
its own summary tables:

> "This summary table provides the optimal mission characteristic values for both
> minimum departure energy and minimum Mars arrival delta-V. Each row in the table
> defines a particular optimized mission."

and defines the transfer classes:

> "Type I trajectories are characterized as having shorter trip times and Type II
> trajectories are characterized as having longer trip times usually with a lower
> required delta-V than Type I trajectories. Type I and Type II trajectories have
> heliocentric travel angles less than and greater than 180 [degrees] respectively."

**Table 3 - Earth to Mars, 2028 opportunity, energy minima** (dates m/d/yr as printed):

| Mission type | Earth departure | Mars arrival | Flight time | C3 (km^2/s^2) | Mars arrival excess speed (km/s) |
|---|---|---|---|---|---|
| Type 1 | 12/10/28 | 7/20/29 | 222 d (7.3 mo) | 9.048 | 4.892 |
| Type 2 | 12/2/28 | 10/16/29 | 318 d (10.4 mo) | 8.928 | 3.261 |
| Type 1 | 1/17/29 | 9/2/29 | 228 d (7.5 mo) | 24.12 | 3.593 |
| Type 2 | 11/20/28 | 9/18/29 | 302 d (9.9 mo) | 9.315 | 2.966 |

**Table 4 - Earth to Mars, 2031 opportunity, energy minima** (the key one for this game):

| Mission type | Earth departure | Mars arrival | Flight time | C3 (km^2/s^2) | Mars arrival excess speed (km/s) |
|---|---|---|---|---|---|
| Type 1 | 1/28/31 | 8/6/31 | 190 d (6.2 mo) | 9.00 | 5.541 |
| Type 2 | 2/23/31 | 1/9/32 | 320 d (10.5 mo) | 8.237 | 5.53 |
| Type 1 | 3/1/31 | 9/27/31 | 210 d (6.9 mo) | 17.89 | 3.777 |
| Type 2 | 12/13/30 | 9/25/31 | 286 d (9.4 mo) | 12.48 | 3.445 |

**Table 5 - Earth to Mars, 2033 opportunity, energy minima:**

| Mission type | Earth departure | Mars arrival | Flight time | C3 (km^2/s^2) | Mars arrival excess speed (km/s) |
|---|---|---|---|---|---|
| Type 1 | 4/6/33 | 10/1/33 | 178 d (5.8 mo) | 8.412 | 3.956 |
| Type 2 | 4/28/33 | 1/27/34 | 274 d (9.0 mo) | 7.781 | 4.377 |
| Type 1 | 4/20/33 | 11/6/33 | 200 d (6.6 mo) | 9.266 | 3.311 |
| Type 2 | 1/26/33 | 10/17/33 | 264 d (8.7 mo) | 17.78 | 3.831 |

Handbook Table 1 ("Data for optimal missions: 2026 to 2045") gives the overall lowest
C3 per opportunity: 2026 = 9.144 (Type II), **2028 = 8.928 (Type II)**,
**2031 = 8.237 (Type II)**, **2033 = 7.781 (Type II)**, 2035 = 10.19 (Type I),
2037 = 14.84, 2039 = 12.17, 2041 = 9.818, 2043 = 8.969, 2045 = 8.587.

### 3.3 The windows in the game's own words

The ticket asked for these at month-level precision. From the handbook data above:

- **Late 2028 window:** Earth departures from **20 November 2028 through mid-January
  2029**, with the energy-optimal departures clustered in **the first ten days of
  December 2028**. It is really a late-2028-into-early-2029 window, not a purely 2028 one.
- **Early 2031 window (the one the game lives inside):** Earth departures from
  **mid-December 2030 through the first days of March 2031**. The fast Type I optimum
  departs **28 January 2031** and arrives **6 August 2031** (190 days). The
  lowest-energy Type II option departs **23 February 2031** and arrives 9 January 2032,
  which is *after the game ends* - so within the game's 24 turns the usable answer is
  the **late-January / February 2031 Type I departure arriving August-September 2031**.
- **Spring 2033 window:** Earth departures from **late January 2033 through late April
  2033**, with the optimal pair in **early-to-late April 2033**. "Spring 2033" is
  correct.

### 3.4 Independent geometric check of those dates

Computed from the same DE441 Horizons vectors (2-day steps, 2027-2034), finding when the
phase angle (Mars longitude minus Earth longitude) passes through the ideal circular
Hohmann departure value of +44.35 deg:

| Ideal-Hohmann departure date (from ephemeris) | NASA handbook optimal Type I departure | offset |
|---|---|---|
| 2029-Jan-07 | 2028-Dec-10 | -28 d |
| 2031-Feb-10 | 2031-Jan-28 | -13 d |
| 2033-Mar-19 | 2033-Apr-06 | +18 d |

The spread of a few weeks is real and is caused by Mars's eccentricity (e = 0.0934) and
1.85-degree inclination, which the circular-coplanar Hohmann model does not capture, and
by real missions trading a little extra C3 for a shorter cruise. **A phase-angle-based
window rule agrees with NASA's optimised trajectories to within about two to four weeks**,
which at monthly turn granularity is at most one turn. That is good enough for the game.

Mars oppositions in the span, for reference (phase angle passes through zero):
2029-Mar-26, **2031-May-05**, 2033-Jun-29.

---

## 4. Porkchop-plot sanity check on the game's defaults

### 4.1 The 259-day Hohmann transfer time - CONFIRMED

Computed from the JPL Table 1 semimajor axes (a_E = 1.00000261 au,
a_M = 1.52371034 au), transfer semimajor axis a_t = (a_E + a_M)/2 = 1.261856 au,
half-period = **258.87 days**. Recomputed independently from the NSSDC Fact Sheet
figures (149.598e6 km and 227.956e6 km, Earth period 365.256 d) gives **258.8 days**.

**Verdict: 259 days is correct** for the idealised circular-coplanar Hohmann transfer.
Sources: <https://ssd.jpl.nasa.gov/planets/approx_pos.html>,
<https://nssdc.gsfc.nasa.gov/planetary/factsheet/marsfact.html>.

Caveat worth recording: **no real mission flies the 259-day Hohmann.** NASA's Mars
mission timeline page says "The interplanetary cruise phase ... lasts about 200 days"
(<https://science.nasa.gov/planetary-science/programs/mars-exploration/mission-timeline/>),
and every Type I row in the NASA handbook tables above is 178-228 days. The 259-day
figure is the theoretical minimum-energy case; real practice buys ~2 months of cruise
time for a modest C3 increase.

### 4.2 The +44 degree departure phase angle - CONFIRMED

Mars mean motion = 360 / (1.52371034^1.5 x 365.25636) = 0.524024 deg/day. Over 258.87
days Mars advances 135.654 deg. The spacecraft sweeps 180 deg. Therefore at departure
Mars must lead Earth by 180 - 135.654 = **+44.346 deg**.

**Verdict: the game's "+44 degrees, Mars longitude minus Earth longitude" is correct.**
Confirmed against the real ephemeris: propagating from the +44.35 crossings found in
section 3.4 and checking where Mars actually is 258.87 days later, the arrival point
misses Earth-departure-longitude-plus-180 by -10.2 deg (2029), +3.0 deg (2031) and
+16.2 deg (2033) - the residual eccentricity effect, not a model error.

### 4.3 The "-75 degree" return phase angle - **SIGN IS WRONG, MAGNITUDE IS RIGHT**

This is the one number in the ticket that does not survive checking, and it is flagged
here rather than silently corrected.

Circular derivation: Earth mean motion = 0.985605 deg/day; over 258.87 days Earth
advances 255.144 deg. For a Mars -> Earth Hohmann, the spacecraft sweeps 180 deg, so at
**Mars departure** Earth must be at `L_Mars + 180 - 255.144 = L_Mars - 75.144`.

Therefore, **using the game's own stated convention (phase = Mars longitude minus Earth
longitude):**

```
return Hohmann phase angle = +75.1 degrees      (NOT -75)
```

Earth **trails** Mars by 75 degrees of heliocentric longitude at the moment of Mars
departure, and overtakes during the cruise. The ticket's parenthetical "(Earth leading)"
is also the wrong way round for a longitude-based reading.

Verified directly against DE441: scanning 2027-2034 for Mars-departure dates at which a
258.87-day, 180-degree transfer actually arrives at Earth's true position:

| Mars departure (ephemeris) | phase Mars-Earth | phase Earth-Mars |
|---|---|---|
| 2028-Nov-12 | **+75.90** | -75.90 |
| 2030-Dec-20 | **+74.02** | -74.02 |
| 2033-Jan-24 | **+72.56** | -72.56 |

**Recommendation:** keep the magnitude 75 degrees; store the return target as
**+75 degrees** if the code's phase variable is `L_Mars - L_Earth` (as the outbound +44
implies), or **-75 degrees** if the return leg's phase variable is deliberately flipped
to `L_Earth - L_Mars`. Either is fine, but **the two legs must not use the same variable
with opposite conventions** - that is the bug this will otherwise cause. This is a
convention decision inside the code, not a game-design decision, but the *sign printed
in the spec* should be made unambiguous, so it is surfaced here.

Note also there is a real Mars -> Earth departure opportunity on roughly **20 December
2030**, i.e. in **turn 12 of the game**, and the next one is not until January 2033,
after the game ends. Whether the game wants a return window inside its span is a design
question for the user.

### 4.4 What the off-window cost model actually produces

Model as specified: `turns = ceil((259 + 1.5 * |offset_deg|) / 30)`, capped at 18;
`Fuel = card Fuel * (1 + |offset_deg| / 120)`.

| offset (deg) | model days | model turns | capped | Fuel multiplier |
|---|---|---|---|---|
| 0 | 259.0 | 9 | 9 | x1.00 |
| 15 | 281.5 | 10 | 10 | x1.12 |
| 30 | 304.0 | 11 | 11 | x1.25 |
| 45 | 326.5 | 11 | 11 | x1.38 |
| 60 | 349.0 | 12 | 12 | x1.50 |
| 90 | 394.0 | 14 | 14 | x1.75 |
| 120 | 439.0 | 15 | 15 | x2.00 |
| 150 | 484.0 | 17 | 17 | x2.25 |
| 180 | 529.0 | **18** | 18 | **x2.50** |

Two corrections to the ticket's own sanity checks:

- At offset 180 the model gives **18 turns (529 days), not "about 17 months"**. The
  x2.50 Fuel figure is right. The 18-turn cap therefore binds *exactly* at offset 180
  and never below it, which means **the cap is decorative** - it can only be reached at
  the single worst point of the cycle. If the cap is meant to bite, it has to be lower
  than 18, or the day coefficient higher than 1.5.
- The on-window baseline is **9 turns**, not 8: 259/30 = 8.63, ceil = 9. Nine of the
  game's 24 turns is a big commitment, which is a design consequence worth the user
  seeing - note that a real fast Type I 2031 transfer is 190 days = **7 turns**, so
  using the pure Hohmann 259 rather than a realistic 190-210 costs the player two extra
  turns for no physical reason.

### 4.5 Sanity checks against real porkchop behaviour

**"A fast Type I transfer near the window is really 6-7 months (180-210 days)."**
**CONFIRMED.** NASA handbook Type I flight times: 2028 = 222 and 228 d; 2031 = **190 and
210 d**; 2033 = **178 and 200 d**. NASA's own mission-timeline page says cruise is
"about 200 days". The 180-210 day band is exactly right for the 2031 and 2033 windows;
2028 runs slightly longer at 222-228 d.

**"Real off-window transfers lengthen and cost much more."** **CONFIRMED, and reality is
harsher than the model near the window.** From handbook Table 4 (2031), moving the Type I
Earth departure from 28 January to 1 March 2031 - **32 days**, about **15 degrees** of
phase offset at the 0.4616 deg/day Earth-Mars phase drift rate - raises C3 from 9.00 to
17.89 km^2/s^2. C3 is the square of departure hyperbolic excess speed, so departure
v_infinity rises from 3.00 to 4.23 km/s, **+41%**, for one month of slip. The game's
model charges **x1.12** at that offset. So the model is *far gentler* than reality in the
first month or two off-window.

**"At the far side of the synodic cycle this model gives about 17 months and 2.5x Fuel."**
Corrected to **18 turns and x2.50** (section 4.4). In reality, a 180-degree-offset
ballistic Earth-Mars transfer is not a "2.5x more expensive trip" - it is essentially
not a trip. The handbook is explicit that the mission space contains a divergence:

> "A notable ridge passes diagonally from the lower left to the upper right of the
> mission space separating the Type I and Type II trajectories. This dramatic rise is
> attributed to near-180 [degree] transfer angle trajectories."
> "Near-180 degree transfer trajectories require larger departure energies than orbits
> with less inclination because they are not able to take advantage of the energy
> provided by Earth's orbital velocity."

and it notes that only *conjunction-class* (Hohmann-like) missions were considered at
all. Departure energy across just the ten optimal windows already varies by more than
2x in C3 (7.781 to 18.7 km^2/s^2 for Type I) *at the best moment of each cycle*; being
half a synodic period out is a different order of problem again.

### 4.6 Defensibility verdict

**Verdict: the model is defensible as a monthly-turn strategy-game abstraction, with
three fixes.** It is not a simulator and should not be judged as one; what matters is
that it is (a) anchored on real numbers, (b) monotone in the right direction, and (c)
makes the window feel like a window.

What is solid:

- **259-day Hohmann baseline: correct** (258.87 d from JPL elements).
- **+44 degree departure phase angle: correct** (44.35 deg).
- **75 degree return phase magnitude: correct** (72.6-75.9 deg over 2028-2033).
- **Direction of both penalties: correct.** Real off-window transfers do get both longer
  and much more expensive, and the penalty really is roughly monotone in phase offset
  over the region a player will actually use.
- **Linear-in-offset shape: an acceptable lie.** Real porkchop cost is not linear, but
  over the first ~60 degrees of offset a linear ramp is a reasonable stand-in, and past
  that the true function is so steep that any smooth curve is equally fictional.
- **A one-degree-tolerance oracle is easily met.** The Standish approximation is accurate
  to 0.006-0.011 deg for these two bodies (section 1.4) - 100x inside tolerance.

What should be fixed or put to the user:

1. **The return phase angle's sign (section 4.3) is wrong as written.** Fix before
   implementing, or the return leg will target the wrong half of the orbit.
2. **The 18-turn cap never binds except at exactly 180 degrees** (section 4.4). Either
   lower the cap or raise the 1.5 coefficient - otherwise it is dead code that reads
   like a balance lever.
3. **The 259-day baseline makes the on-window trip 9 turns when the real fast transfer
   in the 2031 window is 190 days = 7 turns** (sections 4.1, 4.5). Nine of 24 turns is a
   large fraction of the game. Whether the trip *should* cost 9 turns or 7 is a design
   decision, not a physics one, and belongs to the user - but the physics permits 7, so
   the 9 is a choice rather than a constraint.

One further modelling note, offered rather than decided: the real cost function has a
**discontinuity near a 180-degree transfer angle**, not a smooth ramp - which is exactly
why real launch windows are sharp-edged and why the porkchop plot has two separate
basins (Type I and Type II) with a ridge between them. A model that reproduced that
ridge would make the window feel more like a window and less like a gradient. Whether
that texture is worth the extra rule is a design call for the user.

---

## References

| URL | What was taken from it |
|---|---|
| <https://ssd.jpl.nasa.gov/planets/approx_pos.html> | Table 1 and Table 2a Keplerian elements and rates for EM Bary and Mars; the six-step algorithm; Kepler's-equation iteration; the accuracy table (EM Bary 20"/8"/6000 km, Mars 40"/2"/25000 km over 1800-2050); attribution to Standish & Williams 1992. Fetched and text-extracted directly. |
| <https://ssd.jpl.nasa.gov/txt/aprx_pos_planets.pdf> | The PDF version of the above, linked from the JPL site. Confirmed live (HTTP 200); the HTML page was used as the authoritative text. |
| <https://ssd.jpl.nasa.gov/api/horizons.api> | JPL Horizons API. Heliocentric J2000-ecliptic state vectors (DE441) for Earth (399), Earth-Moon barycentre (3) and Mars (499) at 2030-01-01 00:00 TDB; the same vectors at 2-day steps 2027-2034 used to locate windows, oppositions and the return-leg phase angle; observer-table quantity 18 (hEcl-Lon) as a cross-check. Called successfully with curl; values quoted verbatim in section 2.2. |
| <https://ssd.jpl.nasa.gov/horizons/app.html> | The Horizons web UI, equivalent to the API queries above (not used directly; recorded so the query can be reproduced by hand). |
| <https://ssd-api.jpl.nasa.gov/doc/horizons.html> | Horizons API parameter documentation (REF_PLANE, REF_SYSTEM, VEC_TABLE, CENTER='500@10'). |
| <https://ntrs.nasa.gov/citations/20100037210> and <https://ntrs.nasa.gov/api/citations/20100037210/downloads/20100037210.pdf> | Burke, Falck & McGuire, *Interplanetary Mission Design Handbook: Earth-to-Mars Mission Opportunities 2026 to 2045*, NASA/TM-2010-216764, NASA Glenn, October 2010. Tables 3, 4 and 5 (2028, 2031 and 2033 energy minima: departure and arrival dates, C3, arrival excess speed); Table 1 (optimal C3 by opportunity 2026-2045); the Type I / Type II definitions; the near-180-degree ridge discussion; the conjunction-class-only assumption. PDF downloaded and text-extracted. |
| <https://science.nasa.gov/planetary-science/programs/mars-exploration/mission-timeline/> | NASA statement that windows recur "about once every 26 months" and that "the interplanetary cruise phase ... lasts about 200 days". |
| <https://nssdc.gsfc.nasa.gov/planetary/factsheet/marsfact.html> | Mars synodic period 779.94 d; sidereal orbit period 686.980 d; semimajor axis 227.956e6 km / 1.52366231 au; eccentricity 0.09341233; inclination 1.85061 deg. Retrieved with TLS chain validation disabled because this host's certificate store could not validate the chain; value independently confirmed by derivation from the JPL elements. |
| <https://nssdc.gsfc.nasa.gov/planetary/factsheet/earthfact.html> | Earth sidereal orbit period 365.256 d; semimajor axis 149.598e6 km. Same TLS note as above. |

### Sources that could not be reached, and things not verified

- Nothing needed for this note went unfetched. Every numeric claim above traces to a
  source in the table, or to arithmetic performed in this session on numbers from those
  sources (each such derivation is labelled as derived where it appears).
- The **one thing not independently confirmed** is the interpretation of the four rows in
  each handbook opportunity table. The handbook says only "Each row in the table defines
  a particular optimized mission. The trajectory characteristic optimized for that
  mission is in bold type", and the bolding did not survive text extraction from the PDF.
  From the numbers, rows 1-2 are plainly the minimum-C3 Type I and Type II missions and
  rows 3-4 the minimum-arrival-excess-speed Type I and Type II missions. This reading is
  **inferred from the values, not read from the document**, and does not affect any
  conclusion here - the departure dates and flight times are read directly off the table
  either way.
- No claim in this file is stated from memory, so there are no **UNVERIFIED** items.

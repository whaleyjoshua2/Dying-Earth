# One unit of population per million people: the build, the audit and the pictures

Ticket [#333](https://github.com/whaleyjoshua2/Dying-Earth/issues/333) on version 0.09.0. The
designer's line: *"pop 1 per million"*. One unit of population is one million people, a Colonist is
one unit, every Colonist count stands as written, and the unit moved out of the engine
(`PEOPLE_PER_UNIT`, a code constant of five million since ticket #143) into `climate.toml` as
`people_per_unit`, beside `population_growth`. `sweeps/` holds the probe and the three sweeps that
show what it measures.

| picture | what it shows |
|---|---|
| [`population-region-card-earth.png`](population-region-card-earth.png) | `shot:pop select:eastasia panel:0`. China's card on turn one: **Region population 1454.4 (1.45B)**, the unit figure to one decimal with the people in brackets (1,440 on the card, grown a percent), and under it *Its people emit 0.13 per hundred million (0.04 base + 0.03 x Industry Level 3)* -- the same quotation as before the unit moved, since the two rates divided by five as the figure multiplied. The top bar reads `Earth 7.94B`. |
| [`population-bar-hover-solar.png`](population-bar-hover-solar.png) | `shot:hover tip:"Population history"`. The top bar's population hover: *On Earth: India 1.96B, China 1.45B, Nigeria 1.15B, Indonesia 687M, and 10 more, 2.69B. Off Earth: Earth 6M. **One Colonist is 1 million people**; a station over Earth is off Earth and Antarctica is on it.* The figure is read off `people_per_unit`. The chart under it is scaled in people, `7.94B` on Earth's side and `6M` on space's. |

## What was built

- **The unit in data.** `people_per_unit = 1000000.0` in `climate.toml`, validated positive.
  `Tables::people_text` (`1.94B`, `380M`, `1M`), `Tables::population_text` (`1454.5 (1.45B)`) and
  `Tables::units_per_hundred_million` (a hundred now, twenty before) replace the two `Game`
  constants and the two `Game` formatters; they sit on `Tables` so the start screen, which has no
  game yet, reads the same figure as the Region card, the top bar, the Pioneer button and the
  population chart's scale.
- **The Research divisor in data.** `population_factor` divided the Region's figure by a code
  literal 1,000; it now reads `[population_factor] population_per_point = 5000.0` in
  `facilities.toml`, the same five billion people a point.
- **The fourteen Region figures** in `nation_states.toml` multiplied by five (Australia 50.0 to
  India 1940.0, 7,860 in all), and its header no longer says "hundreds of millions".
- **The two emission rates** divided by five (`0.0004`, `0.0003`), so the per-hundred-million
  quotations on the card, in the Report, in the Leapfrog order's line and in the Emissions hover are
  unchanged.
- **The Scrubber cap**, found by the audit and not on the ticket's list: `per_population` is an
  amount in units (one Scrubber per two hundred million people, as the Custodians' signature says),
  so 40 became 200; the cap is the same in every Region and the test pins it.
- **The Pioneer** takes `population_each = 1.0`, unchanged in data and now one million people. The
  Arkwrights' two stays two. This is the one figure the designer chose to let move in people.
- **`CONTEXT.md`**: **Colonist**, **Pioneer** and **Region** say one million since 0.09.0, five
  million from 0.07.3 until then.

## The two Unrest figures: 2.5, not 0.1

The resolution wrote that `refugees_per` and `report_net_floor` "divide by five (0.1)". Both are
amounts **in units**, not rates per unit: a point of Unrest cost 0.5 units of five million, two and
a half million people arriving, and the Report's floor was the same two and a half million. In the
new unit that is **2.5**. At 0.1 a point would cost a hundred thousand arrivals, twenty-five times
as sensitive, which is a rule change the resolution did not mean (it says the change is mechanical
and the proportional figures need nothing). The tree carries 2.5 and the tests pin it;
`sweeps/probe-resolution-literal-refugees-0.1.txt` is the twenty-seed sweep at 0.1, where
Constabularies rose from 108 to 211 and the median Techs completed fell from 20 to 16. The designer
can have 0.1 by changing the two numbers in `unrest.toml` and the two pins in `formulas.rs`.

## The audit

Every hit of `5_000_000`, `five million`, `5M`, `PEOPLE_PER_UNIT`, `UNITS_PER_HUNDRED_MILLION`,
`hundred million`, `/ 1000.0` and `population_factor` over `engine/src`, `src`, `assets/data`,
`engine/tests` and `docs/playtest/PLAYTEST.txt` (which has none):

| where | what | done |
|---|---|---|
| `engine/src/state.rs:2567,2570` | `PEOPLE_PER_UNIT`, `UNITS_PER_HUNDRED_MILLION` | removed; `people_per_unit` in `climate.toml`, `Tables::units_per_hundred_million` |
| `engine/src/state.rs:2573,2583` | `Game::people_text`, `Game::population_text` | moved to `Tables`, reading the data |
| `engine/src/state.rs:2613` | `population / 1000.0` | `[population_factor] population_per_point = 5000.0` |
| `engine/src/ai.rs:875`, `orders.rs:1985`, `src/ui.rs:3894,5281-5283,5572,8124-8127` | `Game::UNITS_PER_HUNDRED_MILLION` in per-hundred-million quotations | read the tables; the printed figures are unchanged |
| `src/ui.rs:148-151` (chart scale), `2226-2244` (top bar), `5256` (card), `5626` (Pioneer button) | `Game::people_text` / `population_text` | `game.tables.people_text` / `population_text` |
| `src/ui.rs:1774` (start screen card) | "in units of five million"; `Game::population_text` | "one million"; `session.tables.population_text` |
| `src/ui.rs:2235` | hover *One Colonist is five million people* | *One Colonist is {people_per_unit / 1e6} million people* |
| `src/ui.rs:5252`, `engine/src/data.rs:692` | comments | reworded |
| `assets/data/climate.toml:70-72,84-85` | comment; the two rates | comment rewritten; `0.002 -> 0.0004`, `0.0015 -> 0.0003` |
| `assets/data/nation_states.toml:28`; fourteen `population` rows | "hundreds of millions"; 1,572 units | corrected; x5, 7,860 |
| `assets/data/unrest.toml:32,38` | `refugees_per`, `report_net_floor` 0.5 | 2.5 (amounts in units; see above) |
| `assets/data/facilities.toml:214-219` | `[scrubber] per_population = 40.0` and its comment | 200.0; comment reworded |
| `assets/data/factions.toml:107` | comment "Population 388 ... 19.4" | 1940, with the history |
| `assets/data/factions.toml:38`, `report.toml:155` | "two hundred million people", "per hundred million" | unchanged, still true |
| `engine/tests/formulas.rs:1983,1989,5710` | messages "one unit of five million" | reworded; the 4.0 and 8.0 are units and did not move |
| `engine/tests/formulas.rs:2932` | world 1572.0 | 7860.0 (red first: `population 1572`) |
| `engine/tests/formulas.rs:3374-3375,3382-3397` | rates 0.002/0.0015, `UNITS_PER_HUNDRED_MILLION`, 288 | 0.0004/0.0003 (red first: `left: 0.002 right: 0.0004`), the tables' method, 1440 |
| `engine/tests/formulas.rs:3455-3456` | "Russia at 1.5", "South Asia at 19.4" | 150, 1940; pins `per_population` 200 |
| `engine/tests/formulas.rs:8356,8390` | `pop / 1000.0` | `/ 5000.0` (red first: `left: 7 right: 5`), pins the data figure |
| `engine/tests/formulas.rs:2491-2530,2550-2580` | "half a person" 0.5 pins, not on the ticket's list | 2.5 (red first: `left: 2.0 right: 1.0`; then `left: 0.1` against the 2.5 pin) |
| `engine/src/economy.rs:217,283` | callers of `population_factor` | nothing: they read the method |
| `assets/data/events.toml:143`, `engine/src/events.rs:264-393` | "Population -5% now", the population-weighted pick | nothing: fractions and proportions |
| `engine/src/ai.rs:591,1811-1855` | `emigrants_affordable`, the most-populous pick | nothing: proportional |
| `engine/src/climate.rs:417,540-551,718` | sea displacement, big-fall fraction, Break loss | nothing: fractions of the figure |

## Looked at

The two pictures above, opened and read before this was written. The fourteen other view captures
the two runs wrote were deleted.

## The probe, against ticket #332's closing sweep

Twenty seeds, Custodians-first seating, `--balance --steps=300`:

| | #332 final | baseline at HEAD `4ffa283` | control: Pioneer at five million | **probe** (the tree) | at `refugees_per` 0.1 |
|---|---|---|---|---|---|
| wins | [0, 4, 0, 0] | [0, 4, 0, 0] | [0, 4, 0, 0] | **[0, 2, 0, 0]** | [0, 2, 1, 0] |
| collapses / 20 | 16 | 16 | 16 | **18** | 17 |
| median collapse turn | 28 | 28 | 28 | 27 | 28 |
| Techs completed, median | 20 | 20 | 20 | 20 | 16 |
| Constabularies | 108 | 108 | 108 | 124 | 211 |
| Pioneer batches; Exodus Calls (Pioneers) | 1548; 33 (544) | same | same | 1626; 60 (992) | 1540; 51 (832) |

The baseline reproduces #332's block exactly, so the sweep is deterministic. The control -- the new
unit everywhere with the Pioneer's take put back at five million people -- is **byte for byte the
baseline**: every figure converted moves nothing. The probe's whole movement is the designed change,
a Pioneer taking one million people from its Region where it took five; Earth keeps more people, the
Arkwrights sound more Exodus Calls, and two more seeds collapse.

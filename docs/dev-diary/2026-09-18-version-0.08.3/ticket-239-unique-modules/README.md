# Ticket #239: three more Unique Modules, and a census that moved one of them

> unique modules / Arkwright's habitat holds their expanded allotment and gives them +1 influence /
> Archivist solar array +1 energy / Prospectors Trade Post +1 ducket

Every Faction now has a Unique Facility on Earth **and** a Unique Module off it. The Custodians'
Academy had been the only one since version 0.08.0; it is now joined by the Archivists'
**Heliostat**, the Prospectors' **Exchange** and the Arkwrights' **Chorus**.

## The census came first, and it said two of the three landed wrong

Counted over 120 games — 20 seeds across six collapse-pace rows — Modules standing at the end:

| Module | Custodians | Prospectors | Arkwrights | Archivists |
|---|---|---|---|---|
| Habitat | 0 | 0–1 | **0–22** | 0–5 |
| Solar Array | 0 | **12–17** | 0 | 0–9 |
| Trade Post | **0** | **0** | **0** | **0** |

Only the Arkwrights' Habitat landed on a building its own Faction builds. The Prospectors' Trade
Post landed on a building **nobody had ever built, in any game**, and the Archivists' Solar Array on
one the *Prospectors* build and they barely do.

## The Trade Post was invisible to the computer, and it was a bug

Not a gate and not a Tech. A Trade Post competes in the `Producer` category, whose ×1.5 bonus fires
only for a resource the seat is short of — and `scarcest()` returns **only Energy, Materials or
Fuel**, while the `needs` list holds only Materials or Energy. **Ducats can never be scarce.** So
the one Producer that makes Ducats could never earn the bonus its rivals routinely earn, and lost
every contest for ever. The `unique_bias` of 1.15 a Unique carries is nowhere near a ×1.5 gap.

It is not a weak building: the Prospectors' median Colonist count at their busiest Body is 8, so a
Trade Post there pays 2×8 = 16, ×1.25 = **about 20 Ducats a turn** — nearly two Regions' income.

There is a recorded failed attempt at this. Ticket #41 put Ducats on the bootstrap `needs` list and
*"it cost the Custodians every win — a Bank on turn one displaced the Research Lab"*; the comment is
still in `ai.rs`. That is a different mechanism — a global preference from turn one — from pricing
**one candidate at one place**, which is what tickets #232 did for the Mine and the Relay and what
the designer took here: *"q1 a"*.

So a Trade Post's weight now reads its real yield against its card's bare figure, **clamped to
0.25–2.0**, the same band the Mine's tech factor runs in. Unclamped the ratio reaches 5 at a
four-Colonist Colony and 12 at a rich one, which is exactly how #232's first attempt at the Mine put
329 Mines on the board.

### What it did, A/B over the same seeds

| | Trade Posts + Exchanges standing | wins by seat | collapses |
|---|---|---|---|
| step 300, fix **off** | 3 + 5 = **8** | [1, 18, 0, 0] | 1/20 |
| step 300, fix **on** | 41 + 32 = **73** | [0, 16, 0, 1] | 3/20 |
| step 420, fix **off** | 7 + 4 = **11** | [1, 18, 0, 1] | 0/20 |
| step 420, fix **on** | 44 + 29 = **73** | [1, 18, 0, 0] | 0/20 |

**The building now exists and the win column barely moves.** The Prospectors were already taking 18
of 20 at these settings; that is not this change. Collapses at step 300 go 1/20 to 3/20.

## The Arkwrights' Unique moved off the Habitat and onto the Relay

The ticket had asked whether their Habitat's ×1.5 capacity was *moving* into the building or merely
being *restated*, since Coach Class already says *"their Habitats hold half again as many."* The
designer answered by changing the building: *"let's rethink the arkwrights special module, add the
plus one influence to the relay and make it +1 for every 6 colonist rounded down."*

That dissolved the capacity question — Coach Class keeps its ×1.5 untouched, and nothing now
transfers room to a captor on capture. It also produced a better rule than the one it replaced: **a
Relay whose reach grows with the people standing around it**, on the Faction whose whole game is
moving people.

Two figures were put to the designer before it was built. **Relays stand 0 times in 120 games
either** — #232's fix to the Relay's weight only bites *after* Relay Networks is researched, because
before the Tech its `with/bare` ratio is exactly 1.0 — and the **median biggest Colony on the board
is 4 people**, a bare Core Module, so at a divisor of 6 a Colony must carry a Habitat above its Core
before a Chorus pays anything. Both were accepted and the Relay's weight was fixed the same way.

Measured after: the Chorus is built **12–13 times per 20 games** in the long rows and not at all in
the short ones, and Relays reach 27–58 for the Prospectors once Relay Networks lands. The clause
does what it was meant to do, late.

## The one that still does not work, and the reason is measured

**The Heliostat is built zero times, by anybody, in every row.** It is not opportunity: the
Archivists end with **19–22 stations carrying 71–88 Modules** and put no Solar Array on any of them.
Nor is it the station bug below — the figures are identical before and after that fix.

It is appetite, and the cause is the stacking flagged when the clause was chosen. A Solar Array is
raised by the **Energy-shortage bonus**, and the Archivists' Reactor takes **75% off the Energy
upkeep of everything they own**, so they are the one Faction never short of Energy. Only the
Prospectors build Solar Arrays at all (28–32 a batch); the Custodians manage 0–1 across 16 stations.

**Their Unique is therefore a building their own signature rule stops them wanting.** It works for a
human Archivist player and never appears in a sweep. Recorded rather than quietly fixed: raising it
is a balance change nobody has asked for, and the closing sweep is where it gets judged.

## A picture found a bug that no test could

`shot:` with `hab:1 habtile:free` puts a Colony's Module build list on screen, and the first
capture showed something wrong:

| picture | what it shows |
|---|---|
| [`build-list-before.png`](build-list-before.png) | The Prospectors' station list (top) and the Archivists' (bottom). The Prospectors have **no Trade Post row and no Exchange**. The Archivists have **no Solar Array and no Heliostat**. The common row vanished and the Unique never arrived. |
| [`build-list-after.png`](build-list-after.png) | The same two lists fixed: **Exchange 21** for the Prospectors, **Heliostat 25** for the Archivists, each in its sibling's place. |

The rule for what may stand on a Space Station was written out **twice** — once in the interface's
build list and once in the computer's — as a list of **kinds**. A list of kinds cannot know about a
Unique Module: `built_by` swaps the common kind out, and the list then throws the Unique away. It
had survived the Academy only because someone remembered to add `ModuleKind::Academy` to one of the
two copies by hand.

It is now one method, `ModuleKind::stands_on_a_station()`, answered by the **job**, so every future
Unique follows its sibling with nothing to remember. A test pins it and was witnessed red.

Extracting it exposed a difference that predates this ticket and was **left standing**: the
computer's copy never listed the Institute, so the AI has never raised one on a station while a
player may. Ticket #185 added it to one list and not the other. Changing it changes what the
computer builds, so it is reported rather than folded in.

## The icons were looked at before they were taken

| picture | what it shows |
|---|---|
| [`icon-candidates.png`](icon-candidates.png) | Twelve glyphs on the game's own dark ground: the three common siblings first, then four candidates each, with the 16- and 22-pixel rows beneath, which is the size a build tile actually uses. |

Two picks were **withdrawn by looking**:

- **Strongbox**, first proposed for the Exchange, is already the Prospectors' **Investment Bank** —
  their two Uniques would have worn one picture.
- **Sun**, first proposed for the Heliostat, reads cleanly but its sibling the Solar Array already
  carries a sun in its corner, and ticket #144's research had warned about this exact file:
  *"sun survives but says 'sun' or 'heat', which on this board is the climate, not a building."*

The designer chose **radar-dish** for the Heliostat, **shop** for the Exchange, and
**satellite-communication** for the Chorus — the last against the note that it is a smear of
diagonal dashes at 16 pixels, which is the designer's call to make and is recorded here so it can be
revisited from the tile rather than the sheet.

**Not photographed: the Chorus's own row.** `hab:1` selects a seat's first station, a Chorus is a
ground Module, and no combination of shot aids opens a ground Colony's build list. Its engine path
is proven instead by the 12–13 the computer built in the sweep and by the shared station rule the
two pictures above exercise.

## The test that proved nothing, again

The first version of the clause test read its expected figures from `g.tables.unique.*`. With all
three figures **zeroed in the data it still passed**, because both sides of every assertion read the
same table. This is the second time in version 0.08.3 — the Antarctic Beneficiation test on #232 was
the first — and the cure is the same: **literals**.

The Heliostat's literals are the interesting ones, because they discriminate a rule the prose could
not. At Mars the sun factor is 0.43, so a Solar Array's 6 rounds to **3** and a Heliostat makes
**4**. Adding the point *before* the scaling would give (6 + 1) × 0.43 = 3 — the same as no clause at
all. Only "after the scaling" reaches 4, and only Mars can tell them apart; the Moon, at full
sunlight, cannot.

| clause | broken to | test failed with |
|---|---|---|
| `heliostat_energy` | 0 | left 3, right 4 |
| `exchange_ducats` | 0 | left 18, right 19 |
| `chorus_colonists` | 99 | left 1, right 2 |
| the job-keyed Tech lookup | back to `kind` | *"Relay Networks must reach a Chorus as it reaches a Relay: 1 -> 1"* |
| `stands_on_a_station` | back to naming kinds | left false, right true |

Each was witnessed alone, then restored. `314 passed; 0 failed`, `6 passed; 0 failed`, clippy clean
with the denial.

## What else moved

`CONTEXT.md` gains **Heliostat**, **Exchange** and **Chorus**, and its **Unique Module** entry goes
from one to four. Its **Module** entry said *"Thirteen kinds since version 0.07.5"* and had been
wrong since version 0.08.0 added the Institute and the Academy; it now reads **eighteen** and says
so.

Captured with `target/release/dying-earth.exe shot:<prefix> player:<faction> hab:1 habtile:free
turns:8 panel:0 window:1920x1080`, off-screen, exit 0, and with
`cargo run --release --example icon_sheet`. Nothing was opened on the designer's desktop.

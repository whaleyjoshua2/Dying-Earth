# Version 0.08.1, the introductions version

Every picture and every sweep that decided this version. The spec is
[`docs/spec/version-0.08.1.md`](../../spec/version-0.08.1.md) and the map is
[Map: version 0.08.1](https://github.com/whaleyjoshua2/Dying-Earth/issues/202), whose closed tickets
hold the reasoning; where this folder and a ticket disagree, the ticket is right.

**Nothing here was opened on the designer's desktop.** Every capture is a headless `shot:` run with
the window placed off-screen, and the mockups are HTML rendered headlessly in Edge. Every one was
looked at before it was called done.

## Mockups — `mockups/`

Made while grilling the Faction window, before a line of it was built, so a layout could be argued
about by looking rather than by describing. They have their own README. Two things they found that
the discussion had not: laid out flat the window is about **950 pixels tall**, and putting the live
figures first **buries the Faction's glyph halfway down the page**.

## Pictures — `ui/`

| file | what it shows |
|---|---|
| `own-solar.png` | The **Faction window** on your own Faction: glyph at 64, dropdown top right, Victory progress, income with the breakdown on hover, Blame, Relations as two rows, Holdings, Rulebook shut. |
| `own-rulebook-solar.png` | The same with the Rulebook open. It ends around row 1020, so it fits an 1080 screen **with the glyph still on it** — which is what the collapsing header was for, and the one thing the mockup could not prove. |
| `rival-solar.png` | A rival's page: **"A rival's income is shown as totals only"**, and no per-building breakdown anywhere. |
| `spectator-solar.png` | A spectated game: the same page with **no disclosure line at all**. |
| `victory-solar.png` | The Victory window with **Relations and Blame gone**. |
| `top-bar-earth.png` | The bar carrying `Factions (F)` between `Victory (V)` and `Trading (R)`. |
| `station-loader-full.png` | The ISS at `Colonists 4 of 4 room`: the loader present and **disabled**, saying so. |
| `station-loader-live.png` | The Arkwrights' Orbital Reef at `Colonists 4 of 19 room`: `Lift 4 Emigrants`, **live**. |
| `antarctic-loader.png` | Lake Vostok's own card, the sea half of the same door. |
| `tutorial-note-1.png` … `-5.png` | The five tutorial notes. Note 5 reads **"Play on"** where 1–4 read "Go on", so the last-note detection still works at five. |
| `start-region-launch-site.png` | **China at turn 1 with a Launch Site**, which the data file does not list. The picture that corrected a charting-round claim. |
| `habitat-capacity.png` | The Orbital Reef at `Colonists 40 of 40 room` where the same board read 64 of 64. |
| `archive-barred-at-axiom.png` | `Build the Archive` **greyed** at Axiom — and `Archive fund 20 of 20` already at its cap on **turn 6**. |
| `station-faction-glyph.png` | The ISS wearing the Custodians' teal symbol ahead of the station glyph. |
| `ship-names.png` | The Mars stack, every line reading **`TSV Valiant`**. |
| `command-cluster-and-emigrants.png` | The cluster a seventh larger with every widget grown together, and `Emigrants waiting: 0` above `Muster 4 Emigrants`. |
| `research-bar-on-tech-tree.png` | The race bar at the head of the Tech Tree, with the **grey carry-over** segment. |
| `tech-tree-civil-defense.png` | The tree at eighteen Techs: rung 1 at 18, Coastal Engineering at 14, and **Civil Defense** on Society rung 2. |
| `education-level-live.png` | The Region card reading `Education Level 1.10` through the **live** function. |

**Two things a picture could not show, recorded rather than implied:** the founding hover (no
building aid parks a loaded Colony Ship at a Body with free slots), and the Education Level with a
School actually raising it (no aid raises a School). Both paths are covered by tests instead.

## Sweeps — `sweeps/`

All at the shipped climate cell — sink 6, step 300, permafrost 4.0, sink-after 4.0 — 20 seeds x four
seatings, the same shape as the 0.08.0 baseline.

| file | what it measures |
|---|---|
| `control-habitat-only.txt` | **The control.** Everything in 0.08.1 *except* the Archive rule. Archivists 57, Custodians 19, Prospectors 2, Arkwrights 1, 1 collapse in 80 — the 0.08.0 baseline, near enough. |
| `archive-rule-appetite-2.txt` | The Archive rule at an AI appetite of 2.0. Archivists **2**. |
| `archive-rule-appetite-5.txt` | The same at 5.0. Archivists **5** — so the appetite is *not* what binds them. |
| `tech-tree-eighteen.txt` | After the dearer tree. Median **18 of 18** Techs still completed. |
| `final-0.08.1.txt` | **The version as it ships.** |

### What they say

| | 0.08.0 | 0.08.1 |
|---|---|---|
| Custodians | 18 | **42** |
| Prospectors | 3 | **11** |
| Arkwrights | 0 | **7** |
| Archivists | **58** | **6** |
| Collapses | 1 of 80 | **14 of 80** |
| Techs completed, median | 17 of 17 | **18 of 18** |

**The control is the reason any of that can be attributed.** Two changes on this branch pressed the
same way, and without a single-variable run the 58-to-6 would have belonged to both. It belongs to
one:

- **The Habitat change moved nothing** — 58 to 57. The computer settles Antarctica and Earth orbit,
  where the Core Module's flat four does the work and Habitats are scarce.
- **The Archive rule moved everything**, by timing. It now stands in **32 of 80 games** at a median
  turn 23–31, against about 71 of 80 at 19–26.
- **The collapse rise is second-order.** Nothing in this version touches Emissions. The Archivists
  were ending games around turn 20; now nobody does, so the world gets more turns to cook.
- **The dearer tree did not bite.** +9.3% and an extra Tech, and the median game still finishes all
  eighteen — matching ticket #117's measurement of nought to three turns for a comparable rise.

### One measurement that was wrong, and how it was caught

The first appetite run came back **byte-identical to the control**, because the release sweep binary
had not been rebuilt after the rule was restored — so it measured the control build, where the
appetite is inert by construction. It looked like a healthy baseline and would have read as "a
stronger appetite fixes everything". It was caught by the binary's timestamp predating the source
file's, and every sweep since has had its binary rebuilt and checked first.

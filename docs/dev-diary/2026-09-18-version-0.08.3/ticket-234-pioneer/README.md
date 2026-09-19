# Ticket #234: the Emigrant becomes the Pioneer

## Why, and why it changed the ticket

The ticket was charted as a clarity question — two words for what looks like one thing — and offered
the designer a choice between renaming the label and folding the distinction away entirely. Both
readings were wrong about the reason. The designer:

> it's more about the word being politically loaded and racked with connotation

That reframing made a third answer available and better than either charted option: **pick a
different noun**. The distinction the rules need survives, the loaded word goes, no rule has to be
restated in a clumsy phrase, and no save breaks.

It matters that the distinction survived, because **two signature rules are stated in terms of it**.
Steerage reads *"every Emigrant costs their Region twice the population"*, and the Spaceport pays
*"+1 Influence for every Emigrant it lifts"*. Folding the word into **Colonist** would have made the
first of those flatly false — a Colonist living at a Colony costs no population.

## What changed

**Emigrant is now Pioneer, and the verb Muster is now Recruit.** The designer picked both, and was
told plainly that Pioneer carries its own freight — in American usage it is tied to westward
expansion, close kin to *settler*, which is already on the glossary's avoid list. Their answer:
*"the connotation such as it is, is not half as bad as emmigrants"*.

**Only prose moved.** The rename was applied by script to double-quoted string literals in Rust and
to the value side of `key = value` in TOML, skipping any literal shaped like an identifier:

| moved | left alone |
|---|---|
| 41 player-visible Rust strings | ~190 engine identifiers (`emigrants`, `BuildEmigrants`, `EmigrantsCard`) |
| 7 Report lines and 6 tutorial lines | the 7 Report **keys** (`emigrants_mustered = ...`), which code looks up by name |
| 6 Faction-card lines, 3 Facility lines | the `[emigrants]` data table and its keys |
| the glossary | the two **serialized save fields** on every Nation State |
| | the `emigrants:` screenshot flag |

Nothing a player reads says Emigrant any more; nothing that would break a save moved. The glossary
says so explicitly, so the next reader is not surprised to find the old spelling in the source.

**A sentence that needed no fixing after all.** The charting round flagged the top bar's population
hover as the one string with no clean rename, because it distinguishes *"Emigrants waiting on a card
and Colonists aboard a Ship"* and both halves would have become Colonists. With a distinct noun the
problem evaporates: it now reads **"Pioneers waiting on a card and Colonists aboard a Ship are in
neither line"**.

## Two glossary defects fixed on the way through

- **`Colonist` had no `_Avoid_` line and `Emigrant` had two.** The second — *settler, crew, worker,
  population resource* — plainly belonged to Colonist and had drifted down into the entry below it.
  Restored to its owner.
- **`recruit` was on the avoid list for this very term**, which the designer's new verb contradicts.
  The list now reads *emigrant, settler, migrant, passenger, colonist-in-waiting, pilgrim*.

## Looked at

| picture | what it shows |
|---|---|
| [`region-card.png`](region-card.png) | China's Region card with the renamed controls in the running game: the **Pioneers** heading, **Pioneers waiting: 4**, and the **Recruit 4 Pioneers** button. |

Four engine tests pinned the old strings end to end and were watched to fail first — `"4 Pioneers
recruited in China for the Custodians; its Unrest fell by 0.5 to 2.5."` is now asserted verbatim,
so the Report, the log and the order text are all guarded by name rather than by reading.

Captured with `target/release/dying-earth.exe shot:<prefix> select:eastasia emigrants:4 turns:6
window:1920x1080`, off-screen, exit 0. Note for the next reader: **`select:` takes the internal
StateId, not the displayed name** — China is `eastasia`, the European Union is `europe`. Two
captures were wasted on `select:china`, which silently matches nothing and shows no card at all.

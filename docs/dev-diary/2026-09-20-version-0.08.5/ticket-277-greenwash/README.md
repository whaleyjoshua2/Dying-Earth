# The Greenwash

Ticket [#277](https://github.com/whaleyjoshua2/Dying-Earth/issues/277) on
[map #275](https://github.com/whaleyjoshua2/Dying-Earth/issues/275).

| picture | what it shows |
|---|---|
| [`faction-window-own-page.png`](faction-window-own-page.png) | `shot: factions:1 panel:0`, 1280x800. The Faction window on the player's own page at turn 1: between Relations and Holdings, the new block **Greenwash campaign**, a field at 5 and the button **Greenwash for 5 Ducats**, beside the Blame line whose sixth clause it will feed. The Smear block sits in the same place on a rival's page; this is its mirror in place as well as in rule. |

The measurement, twenty seeds with the Custodians first:
[`sim-20-custodians-first.txt`](sim-20-custodians-first.txt).

## What was decided, in the designer's words

- *"q1 greenwash"* -- the name is **Greenwash**: the Blame falls without a gram of CO2 leaving the
  air. *Propaganda* stays on the Smear's avoid list, where it was.
- *"q2 a"* -- **2 ppm off per Influence, and a Ducat beside every Influence spent**, so cleaning
  your name costs strictly more than dirtying a rival's and half a Neutral carbon credit's Ducat
  price per ppm.
- *"q3 a"* -- it comes off **the whole ledger**, floored at nought as ever, not only the smeared
  part.
- *"a4 yeah"* -- **one campaign a turn, any amount** the Allotment and the Ducats cover.
- *"q5 public no offense"* -- the Report says who greenwashed, in public; no offence against
  anybody, so a rival's answer is a Smear, not a grudge.
- *"q6 yes"* -- the **Custodians may**, like anyone; their first lever on their own Blame.
- *"q7 sounds good"* -- a computer seat greenwashes at weight 4 when its own share stands above
  the fair quarter, carbon credits are not to be had, and it holds 20 Ducats past the price; where
  credits are to be had it buys them instead.
- *"q8 a"* -- the control on the **player's own page** of the Faction window.
- *"q9 that"* -- the Blame sentence gains a **sixth clause**, *"N cleaned by campaign"*, and is not
  restructured.

## Settled by the builder, to be corrected if wrong

- **"Credits not to be had"** means: no Custodians at the table other than the seat itself, or the
  Custodians offering nought, or Hostile toward the seat, or the price past its purse. The buyer's
  appetite sets a flag the Greenwash appetite reads, so a seat never proposes both in one turn.
- **The order costs both purses at once** through the ordinary cost check, so a seat short of either
  cannot place it, and the field on the window ranges over the smaller of the two.
- **The Report line** is *"The Prospectors greenwashed: 10 ppm off their Blame."*; the rival's deed
  line is *"spent 5 Influence and as many Ducats greenwashing their Blame"*.
- **A saved `blame_cleaned` on the seat** with a default of nought; old saves load unchanged. The
  sweep prints it beside the Smear figure: *"taken off by Greenwash [..]"*.

## Witnessed red

Two tests, each watched to fail under a deliberate mutation before it was believed. The rule test
with the ledger term removed: *"off the whole ledger: 100"* (100 expected 80). The appetite test
with the gate closed: *"dirty, rich, no credits on offer: it greenwashes"* against an order list
with no Greenwash in it. Restored, `335 passed`, `6 passed`, clippy clean with `-D warnings`.

## Measured, twenty seeds with the Custodians first

| figure | ticket #276's batch | this ticket |
|---|---|---|
| wins, Custodians / Prospectors / Arkwrights / Archivists, of 20 | 0 / 15 / 0 / 0 | **0 / 16 / 0 / 2** |
| collapses | 5 | **2** |
| Prospectors' Blame at the end, median ppm | 788 | **644** |
| Prospectors' share, median | 0.69 | **0.64** |
| ppm greenwashed a game, by seat, median | -- | **[0, 90, 0, 0]** |
| ppm laid on by Smear, median | [0, 135, 0, 0] | [0, 135, 0, 0] |
| carbon credits bought over the batch, by seat | [0, 732, 0, 0] | **[0, 1183, 6, 0]** |

Only the Prospectors greenwash, in every seed, 30 to 130 ppm a game. **Credits bought rose rather
than fell**: the computer buys when the Custodians are willing and greenwashes when they are not,
so the two levers add. The Prospectors' ledger is a fifth lighter and their share five points
lower; the thresholds they face fall with it. Two Archivist wins and three fewer collapses in a
batch of twenty are figures, not findings; the closing sweep measures all four seatings.

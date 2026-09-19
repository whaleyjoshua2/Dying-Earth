# Blame and Blame credit: the credit that was always zero

Ticket [#265](https://github.com/whaleyjoshua2/Dying-Earth/issues/265) on
[map #254](https://github.com/whaleyjoshua2/Dying-Earth/issues/254).

| picture | what it shows |
|---|---|
| [`faction-window-blame.png`](faction-window-blame.png) | `shot: turns:12 factions:1 panel:0`. The Faction window's Blame block after twelve turns: *Answerable for 65 ppm (emitted 77, removed 12 in credit), thresholds x1.00*. |
| [`climate-panel-blame-block.png`](climate-panel-blame-block.png) | The Climate Panel's four-Faction block at 1920x1080, one form for every seat: *Archivists: answerable for 45 ppm (emitted 45, removed 0 in credit); share 0.14, thresholds x1.00*. The sentence beneath it stays, as ticket #233 left it. |
| [`climate-panel-1080.png`](climate-panel-1080.png) | The whole panel at the size the game opens. |

## What was measured first

The ticket said how often a seat holds a Blame credit was unmeasured. The `sim` example was
taught to print what each seat removed and its credit, and ten games were run -- five with the
Custodians in seat 0, five with them as a computer seat:

| | the Custodians' Blame | removed by their Scrubbers | credit |
|---|---|---|---|
| in seat 0 | 46 to 113 ppm | 6 to 63 ppm | **0** |
| as a computer seat | 70 to 267 ppm | 30 to 147 ppm | **0** |

**No seat held a credit in any game.** The Custodians scrub a third to a half of what they emit,
never more than all of it. So a figure computed since 0.05, shown on two panels and read by no
rule had never once been anything but zero.

## What was decided, in the designer's words

- *"3"* -- **Blame credit is what a Faction has removed**, full stop: `blame_credit = removed`.
  Blame itself is unchanged, emitted less removed and never below nothing. The word now has a
  figure in every game, and the carbon-credit ticket has a supply that exists.
- *"looks good"* -- the panels read *answerable for N ppm (emitted E, removed R in credit)*.
- *"credit"* -- what the Custodians' Research Directive adds to the Natural Sink **counts as
  their removal**, every Climate phase it stands: the ppm bought is remembered as the seat's
  (`directive_sink`, saved) and credited each phase, since that enlargement takes that much out of
  the air every turn. The recommendation to leave the Sink the world's was overruled.
- *"as suggested"* -- one hover on the Blame heading, on both panels, naming what Blame is, what
  the credit is, and the two rules that read Blame; no new panel.

## Witnessed red

Two tests written first and run against the old definition: *a seat that emitted 100 and removed
40 holds 40 in credit* failed at 0; *half a ppm of bought Sink is credited every phase* failed at
`0 -> 0`. Then the definition, the credit and the panels went in; `323 passed`, clippy clean with
`-D warnings`. The ticket #53 test that pinned the old surplus definition was rewritten to the new
one. The glossary's **Blame** entry gains the Directive clause and a **Blame Credit** entry records
what the word meant and what it means now.

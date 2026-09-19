# The Smear campaign

Ticket [#267](https://github.com/whaleyjoshua2/Dying-Earth/issues/267) on
[map #254](https://github.com/whaleyjoshua2/Dying-Earth/issues/254).

| picture | what it shows |
|---|---|
| [`rival-page-prospectors.png`](rival-page-prospectors.png) | `shot: factions:prospectors turns:12 panel:0`. The Prospectors' page of the Faction window after twelve computer turns, the new **Smear campaign** block under the Accords: a field for the Influence and *Smear the Prospectors*. Their Blame line above reads *answerable for 168 ppm (emitted 168, removed 0 in credit), thresholds x1.22*. |
| [`smear-block.png`](smear-block.png) | The Accords and the Smear block, magnified. On a rival's page at 1280x800 the Rulebook header now sits below the fold; the player's own page, which has neither block, still fits. |

## What was decided, in the designer's words

- *"Call it a Smear campaign because it inflates blame above strickly ppm produced and thus kinaof
  a lie"* -- **Smear**, over the Denounce recommended. The reason is in the glossary entry and the
  data comment: it is a lie, which is exactly why the ledger and not the physics is what every rule
  reads.
- *"go with that"* -- **1 Influence lays 2 ppm** on the target's Blame (`[smear]
  ppm_per_influence` in `influence.toml`).
- *"permanent"* -- for good, like every other entry on the ledger, shown as its own figure: the
  panels read *…removed 0 in credit, 80 laid on by rivals* so nobody is told they put it in the air.
- *"oh yeah of course"* -- an offence at an Influence push's weight, and the Report names who paid:
  *The Custodians smeared the Prospectors: 10 ppm laid on their Blame.*
- *"any rival one per tern any allotment"* -- one campaign a turn per target, any amount the
  Allotment covers, never oneself.
- *"ai uses it"* -- a seat Cold or Hostile toward a rival whose share stands above the fair quarter
  proposes one step of Influence against it, weighed by how far above the quarter it stands and at
  the opportunity multiplier when Hostile, competing with a place for the same Allotment. `smear = 4`
  in every Faction's weights.

## Measured, after

Three headless games with the Custodians in seat 0:

| seed | the Prospectors' Blame | of which laid on by rivals |
|---|---|---|
| 1 | 589 ppm | **80** |
| 2 | 791 ppm | **130** |
| 3 | 754 ppm | **120** |

The computer seats use it, and against the seat everyone would: a seventh to a sixth of the
Prospectors' Blame at a game's end is now smear, laid on by the Custodians and the Archivists who
resent them. Nobody smeared anybody else. The `sim` example prints the figure per seat, ready for
the sweep.

## Witnessed red

The order, the ledger entry, the rate, the cards and the appetite's weight were put in place with
nothing resolving the order and nothing proposing it, and both tests run: *"20 ppm laid on: 0"* and
*"a Smear against the dirty rival it is Cold toward: [no Smear among eleven orders]"*. Then the
Resolution step and the appetite went in; `326 passed`, clippy clean with `-D warnings`.

# Battles pollute

Ticket [#279](https://github.com/whaleyjoshua2/Dying-Earth/issues/279) on
[map #275](https://github.com/whaleyjoshua2/Dying-Earth/issues/275).

| picture | what it shows |
|---|---|
| [`climate-panel-war-line.png`](climate-panel-war-line.png) | `shot: battle:earth`, 1280x800. The Climate Panel at turn 3 after three Frigates fought in Earth orbit the turn before: under *Emissions this turn, by source*, between *Population 7.9* and *Natural Sink -6.0*, the new line **War 1.0**, two hits at half a ppm each. The roster shows TSV Vanguard at Earth with damage 3/4. `battle:earth` is a new form of the `battle:1` aid that fights at Earth and runs a second quiet turn so the Climate phase has drained the bucket. |

The measurement, twenty seeds with the Custodians first:
[`sim-20-custodians-first.txt`](sim-20-custodians-first.txt).

## What was decided, in the designer's words

- *"q1 a"* -- **the stock and the ledger both**: a new War source on the emissions breakdown that
  reaches the CO2 stock, and each Faction party's share written to its Blame as emitted.
- *"q2 a"* -- **whose**: every party with a seat wears the ppm of its own hits; a neutral Region's
  Army's hits are the world's, nobody's. Two aggressors share by what each did.
- *"q3 sounds good"* -- **0.5 ppm per hit landed, 2.0 per building burned** in the rolls after a
  ground Battle or on a taking, charged to the aggressors or the taker.
- *"q4 yes"* -- **counted against a Stabilization run.** A war a Faction chose is not the weather.
- *"q5 earth only"* -- Regions, Earth orbit and a Colony on Earth. Mars orbit fouls nobody's air.
- *"q6 next"* -- charged at the **next Climate phase**, in a bucket like the Wildfire's.
- *"q7 own line"* -- **"War"** on the Climate Panel, shown when above nought, with the rule on
  hover; its own figure in the sweep, by seat and nobody's.

## Settled by the builder, to be corrected if wrong

- **Where the place comes from.** The Battle record still carries a name, not a place; the charge
  is made where the melee is run, which knows the Body or the Region. The Battle Report ticket may
  give the record a real place later; nothing here waits on it.
- **Buildings burned on a taking** (a Pacified or completed Occupation) are charged to the taker,
  since the same rolls fire there; a place taken by Influence rolls nothing and pollutes nothing.
- **Two buckets on the climate state**, one per seat and one for nobody, both saved with defaults;
  a seat's war ppm and the world's running total kept beside them for the sweep.
- **The War figure is inside `counted()`**, so the Custodians' run reads it; the chart needs nothing,
  since it draws the total.
- **The log's Climate line** names war beside cards and permafrost.

## Witnessed red

The test was written first and failed twice on its own mistakes (it chose a Region a Faction
starts in, then compared against a snapshot that already held the bucket), never on the rule; so
it was then witnessed against the rule: with the charge switched off in the engine it failed at
*"seat 0 wears its 2 hits and the 0 buildings it burned: 0 against 1"*, and passed on the restore.
`339 passed`, `6 passed`, clippy clean with `-D warnings`.

## Measured, twenty seeds with the Custodians first

| figure | Blockade batch | this ticket |
|---|---|---|
| wins, Custodians / Prospectors / Arkwrights / Archivists, of 20 | 0 / 16 / 0 / 2 | **0 / 16 / 0 / 2** |
| collapses | 2 | **2** |
| seeds with any war ppm | -- | **16** |
| war ppm a game, by seat, median | -- | **[0, 9.25, 1.0, 0]** |
| war ppm over the batch, by seat | -- | **[7, 174, 20.5, 5]** |
| nobody's war ppm, every seed | -- | **0** |

The Prospectors, the seat with the Armies, wear a median 9 ppm a game of war and as much as 29;
the seats they attack wear the hits their Standing Armies land back. Against a Prospector ledger of
640 ppm that is a signal, not a lever, as the ticket said it would be at today's rate of Battle.
**Nobody's share is nought in every seed**, which surprised the builder: the engine's own comment
says 46 of 55 Battles were against neutral Armies. Seed 2's log shows why: every Battle in it is
the Prospectors' Army against the Standing Army of a Region the Custodians or the Arkwrights hold,
since the neutrals are taken by Influence in the first turns and the 0.08.4 attack rule marches
on held places. The comment is from an older sweep. The closing sweep measures all four seatings.

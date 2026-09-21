# Buildings that say what they do

Ticket [#280](https://github.com/whaleyjoshua2/Dying-Earth/issues/280) on
[map #275](https://github.com/whaleyjoshua2/Dying-Earth/issues/275).

| picture | what it shows |
|---|---|
| [`china-card-launch-site-row.png`](china-card-launch-site-row.png) | `shot: select:eastasia slotbox:3 panel:0`, 1280x800. China's card with the Launch Site's box clicked; the strip under the boxes reads **"Launch Site (coastal): lifts Pioneers and Armies from here to orbit and to a Space Station over Earth, and a station over Earth can be built from here, 2 [Energy] upkeep"**, the Energy glyph drawn after the figure as the glyph rule already gives it. Before this ticket the same row read *Launch Site (coastal): no output, 2 Energy upkeep*. |

No batch was run: this ticket changes no rule and no computer seat's play.

## What was decided, in the designer's words

- *"q1 a"* -- the sentence lives on the building's row in **`facilities.toml` and `modules.toml`**,
  as a `does` field, drawn where "no output" was drawn.
- *"q2 static"* -- **static prose** from the data; the live figure stays on the card's own line.
- *"q3 no new glyhs"* -- **no new glyph**; the upkeep keeps its Energy glyph, the effect is words.
- *"q4 that"* -- every "no output" row, **plus a clause for the five Uniques** that hide their real
  rule behind a resource, and the Scrubber's and Sea Wall's hand-written hovers moved into the
  same field.
- *"q5 that"* -- the four surfaces stay **shared**: one sentence per building, everywhere.
- *"q6 that"* -- **one clause, two at most.**

## The sentences

| building | reads now |
|---|---|
| Launch Site | lifts Pioneers and Armies from here to orbit and to a Space Station over Earth, and a station over Earth can be built from here |
| Constabulary | takes 1 off this state's Unrest a turn, halves what the climate and an Agitate add to it, and adds 5 to what a rival must reach to take this state (10 with Civil Defense) |
| Scrubber | +3.0 ppm on the Natural Sink and 1 off this state's Unrest a turn, counted as its holder's removal; no build slot; destroyed if this state changes hands |
| Sea Wall | holds every Sea Level threshold off this state while it works, and a Storm Surge it holds cuts its coastal Facilities' output by 30% for one turn; no build slot, one to a state; each rise held adds 0.5 Materials a turn to its keep |
| School | raises this state's Education Level 0.20 a turn, to 2.0; it falls back at the same rate when the School stops |
| Investment Bank | +N Ducats, banks 1% of the Venture Capital Fund's balance back into it each turn, never less than 1 across all your Investment Banks |
| Spaceport | does a Launch Site's work, and every Pioneer it lifts off Earth adds 1 Influence to next turn's Allotment |
| Reactor | +N Energy, pays three quarters of your whole Energy upkeep, the Archive excepted; a second Reactor is an ordinary power station |
| Academy (Earth) | does a School's work and pays 1 Ducat a turn while it is online |
| Habitat | holds 4 Colonists (8 with Expanded Habitats) |
| Shipyard | the only place a Ship can be built |
| Barracks | the only place a Colony Army is raised and repaired; holds one |
| Core Module | holds 4 Colonists and never counts against the Colony's room for Modules |
| Institute | raises this place's Education Level 0.20 a turn, to 2.0; it falls back at the same rate when the Institute stops |
| Academy (Module) | does an Institute's work and pays 1 Ducat a turn while it is online |
| Exchange | +N Ducats, pays 1 Ducat more a turn than a Trade Post |

The Embassy, Relay and Chorus already said what they do through their Allotment and Standing
figures; the Mass Driver through its detail line; the Archive through its own bespoke row. None
changed.

## Settled by the builder, to be corrected if wrong

- **The Scrubber's hand-written hover said 4 Energy upkeep for two versions while the data said 3.**
  Both hovers now read the row's own sentence, so the figure comes from one place.
- **A figure inside a sentence is written twice in the data**: the School's 0.20 and 2.0 stand in
  `[school]` and in its `does` line. A test pins the School's sentence to the `[school]` figures,
  so they cannot drift apart unnoticed; the Institute's sentence is the same words.
- **The sentence sits before the Allotment and Standing parts** of a row and after the resource,
  so a Unique reads *"+6 Ducats, banks 1% …, 2 Energy upkeep"*.
- The `does` field defaults to nothing, so a row without one reads exactly as before.

## Witnessed red

The test failed first on its own mistake (it expected the Bank's flat 4 Ducats where China's card
pays 6), then was witnessed against the rule: with the sentence switched off in `Yield::text` it
failed at *"no longer 'no output': no output, 2 Energy upkeep"*, and passed on the restore.
`340 passed`, `6 passed`, clippy clean with `-D warnings`.

# The Smear and the Greenwash on sliders

Ticket [#293](https://github.com/whaleyjoshua2/Dying-Earth/issues/293) on
[map #289](https://github.com/whaleyjoshua2/Dying-Earth/issues/289).

| picture | what it shows |
|---|---|
| [`smear-rail-prospectors-page.png`](smear-rail-prospectors-page.png) | `shot: factions:prospectors panel:0 window:1280x1100`. The Prospectors' page as the Custodians: under the Accords, **Smear campaign** on a full-width rail from 0 to the turn's 15 Influence, the knob at 5, the line *5 of your 15 Influence this turn; 15 not yet ordered elsewhere*, and the button **Smear the Prospectors with 5 Influence**. The number field it replaces is gone. |
| [`greenwash-rail-ducat-bound.png`](greenwash-rail-ducat-bound.png) | `shot: factions:1 panel:0 window:1280x1100`. The player's own page: **Greenwash campaign** on the same rail, with the Ducat bound painted in a bluer grey from 10 to 15 and a tick at 10, since the Custodians hold 10 Ducats on turn 1 and every point costs one; the line reads *your 10 Ducats cover 10 of it, which is the bound*, the button **Greenwash with 5 Influence for 5 Ducats**. |
| [`smear-rail-all-committed.png`](smear-rail-all-committed.png) | `shot: factions:prospectors attend:1 panel:0 window:1280x1100`. The extreme: Max standing on China has taken all 15, so the rail is nobody's grey end to end, the knob at 0, *0 not yet ordered elsewhere*, and the button greyed. The scale did not move; only the reachable part did. |

No batch was run: an interface change; the engine is untouched, and the computer seats spend one step of five as before.

## What was decided, in the designer's words

- *"q1 runs from 0 to their max income - width fixed"* -- a **fixed-width rail in single points
  from 0 to the turn's whole Influence**, read as the Allotment plus what was bought this turn.
- *"q2 that"* -- the **Ducat bound painted** as a second region and named in the line.
- *"q3 keep the button"* -- the slider sets the amount; **the button spends it**.
- *"q4 yes"* -- **bought Influence counts**: both blocks read the command cluster's figure.
- *"q5 no cap"* -- no cap in data.

## Settled by the builder, to be corrected if wrong

- One painter for both rails, `influence_rail`, drawn as the Research Directive's is: the
  committed part in nobody's grey from the right (the Directive's floor is from the left), the
  Ducat bound in a bluer grey between the two ticks so the two bounds can be told apart.
- The heading line under the rail says the amount, the turn's whole, and which bound bites; the
  button names the amount, so the order a player is about to place is readable without the hover.
- At nought the button is disabled by the rule's own refusal (an amount must be positive), which
  is how the all-committed picture reads.
- The **Smear** and **Greenwash** glossary entries say the amount is set on a slider.

## Looked at, not tested

An interface change; `346 passed`, `6 passed`, clippy clean with `-D warnings`. Three pictures, each
looked at before filing, one of them the extreme where nothing is left to spend.

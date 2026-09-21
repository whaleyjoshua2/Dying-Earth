# Carbon credits move from the Trading window to the Faction window

Ticket [#285](https://github.com/whaleyjoshua2/Dying-Earth/issues/285) on
[map #275](https://github.com/whaleyjoshua2/Dying-Earth/issues/275).

| picture | what it shows |
|---|---|
| [`custodians-own-page-offer.png`](custodians-own-page-offer.png) | `shot: factions:1 panel:0`, 1280x800. The player's own page as the Custodians: under the Greenwash block, **Carbon credits**, *"You hold 0 ppm in credit. You offer 0 ppm a turn; what you sell past your credit goes onto your own Blame."*, a field and **Offer 0 ppm a turn**. |
| [`custodians-page-as-prospectors.png`](custodians-page-as-prospectors.png) | `shot: player:prospectors factions:custodians panel:0 window:1280x1100`. The Custodians' page as the Prospectors see it: Accords, then the Smear campaign, then **Carbon credits** with the kept refusal *"The Custodians are not selling this turn. They are Wary toward you (x1.5)."*, since the computer Custodians offer nothing at turn 1. When they do, the line names the offer and the price and the button reads *Request n ppm for {cost} Ducats*. |
| [`trading-window-four-lines.png`](trading-window-four-lines.png) | `shot: trade:1 panel:0`. The Trading window: Influence, Materials, Fuel and Energy, and no fourth line. |

No batch was run: the rule did not move, only where it is worked; the computer seats' offer and appetite read neither.

## What was decided, in the designer's words

- *"q1 a"* -- the **Offer block on the Custodians' own page**, beside the Blame line whose credit it sells and the Greenwash.
- *"q2 a"* -- the **Request block on the Custodians' page** for everyone else, under the Accords and the Smear.
- *"q3 a"* -- **no real request machinery**: the word changes, the rule does not; a request is filled at End Turn from the standing offer or refunded where it ran out.
- *"q4 goes entierly"* -- the Trading window's **fourth line is gone**.
- *"q5 unchanged"* -- the **computer seats** offer and buy as before.
- *"q6 keep them"* -- the **refusals** keep their words.

## Settled by the builder, to be corrected if wrong

- The old block became two functions sharing one hover; the order list reads *Request n ppm of
  carbon credit from the Custodians*; the **Carbon credit** glossary entry says where the trade is
  made now.
- The blocks appear only when a Custodian seat sits at the table, as the old line did.

## Looked at, not tested

An interface move; the engine is untouched, `343 passed`, `6 passed`, clippy clean with
`-D warnings`. The three pictures are the check, each looked at before filing.

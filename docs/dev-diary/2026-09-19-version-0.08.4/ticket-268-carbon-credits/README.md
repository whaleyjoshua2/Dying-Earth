# Carbon credits from the Custodians

Ticket [#268](https://github.com/whaleyjoshua2/Dying-Earth/issues/268) on
[map #254](https://github.com/whaleyjoshua2/Dying-Earth/issues/254).

| picture | what it shows |
|---|---|
| [`trading-window-buyer.png`](trading-window-buyer.png) | `shot: player:prospectors turns:12 trade:1 panel:0`. The Trading window as a Prospector, the fourth line under the three priced rows: *Carbon credits -- The Custodians are not selling this turn. They are Wary toward you (x1.5).* The computer Custodians had withdrawn their offer, as their rule lets them. |
| [`buyer-block.png`](buyer-block.png) | That line, magnified. |
| [`trading-window-custodians.png`](trading-window-custodians.png) | `shot: turns:12 trade:1 panel:0`. The same window as the Custodians, the computer having played the seat for twelve turns: *You hold 6 ppm in credit. You offer 3 ppm a turn; what you sell past your credit goes onto your own Blame*, and beneath it the field and *Offer 0 ppm a turn*. |
| [`custodians-block.png`](custodians-block.png) | That block, magnified. |

## What was decided, in the designer's words

- *"any amount and take the blame"* -- **unbacked**, over the backed reading recommended. The
  Custodians may sell more than they hold; the excess goes onto their own ledger as Blame taken.
  *Forcing them to sell only what they hold would deprive them of any reason to get credit, and
  forbidding overselling would gimp a strategic choice.*
- *"yeah"* -- a ppm bought comes off the buyer's Blame ledger for good and off the seller's credit;
  nothing rises anywhere within the credit.
- *"go with it"* -- **Ducats only, one a ppm**, in `[carbon_credits]` in `factions.toml`.
- *"go with it"* -- the Custodians' view of the buyer multiplies the price: Friendly x0.5, Cordial
  x0.75, Neutral x1, Wary x1.5, Cold x2, **Hostile refuses**.
- *"make it 10 but as stated with Q1 can exceed sellers credit"* -- **10 ppm a turn per buyer**,
  and the cap never reads the seller's credit.
- *"go with it"* -- a purchase is **an act of friendship both ways**.
- *"I want them to refuse sometimes"* -- the computer Custodians set a standing offer, re-set when
  it should move: their whole credit while their own share of the table's Blame is under the fair
  quarter, **nought when it is not**, and the cap on top when they are clean and short of Ducats.
  The rule-sells recommendation was overruled.
- *"yup that"* -- the computer seats buy with an appetite keyed on their share above the quarter
  and their Ducats; the Trading window carries a fourth line; a Custodian player sets the offer
  there with a field, and may oversell as the computer may.

An offer is shared first come first served, and a buyer left short gets its Ducats back for what it
did not get; the rest land in the Custodians' Stockpile. The offer stands until changed.

## Measured, after

Three headless games with the Custodians in seat 0:

| seed | the Custodians sold | the Prospectors bought | the Custodians' credit left |
|---|---|---|---|
| 1 | 45 ppm | 45 | 6 |
| 2 | 54 ppm | 54 | 49 |
| 3 | 6 ppm | 6 | 12 |

The computer seats trade: the Custodians sell and the Prospectors -- the only seat above a fair
share -- buy, 6 to 54 ppm a game against Blame ledgers of 500 to 800. Nobody else bought, and the
Custodians never oversold in these three.

## Two tests found by the suite

The overselling test was written with the wrong sums twice -- it forgot the second buyer's two ppm
in the seller's Blame taken, and mis-added a refund -- and each time the rule was right and the
test was wrong; both corrected against the rule as decided. And an older test, *relations do
nothing mechanical in this version* (ticket #191), went red: with every pair Hostile the Custodians
refuse to sell, so six computer turns now play out differently from the neutral game. The premise
was already dead since 0.08.2; the test is kept inverted, as the record of the first version in
which Relations were mechanical to the computer seats and not only to a player reading a card.

## Witnessed red

The orders, the table, the ledger entries and the cards were put in place with nothing resolving a
purchase and nothing proposing one, and the three tests run: *"10 ppm off the buyer's ledger:
100"*, *"the first buyer got its ten"*, and no offer among the Custodians' orders. Then the
Resolution step and the two appetites went in; `329 passed`, clippy clean with `-D warnings`.

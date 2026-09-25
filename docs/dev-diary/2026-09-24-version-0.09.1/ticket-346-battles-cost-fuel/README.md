# Battles that cost Fuel

Ticket [#346](https://github.com/whaleyjoshua2/Dying-Earth/issues/346) on version 0.09.1. The
designer's line was *"space battles take fuel"*.

`SPEC.md` beside this file is the build specification, and the
[resolution comment](https://github.com/whaleyjoshua2/Dying-Earth/issues/346#issuecomment-5824607570)
on the ticket is the authority over it.

## Said first: this ticket measures almost nothing

**Space Battles essentially do not happen.** Re-measured over eighty games on the build before this
one: **one orbital Battle in the whole batch**, **no Bombard in any of it**, no interception and no
Blockade ordered anywhere. Over three hundred Battles were fought, and essentially all of them were
on the ground between Armies, which have no tanks. The computer is not peaceful; it is landlocked.

It was built anyway because it is a **rule of the game rather than a tuning figure**, and the human
player meets it the first time they fight in orbit. It will come alive the day
[The computer never contests an orbit](https://github.com/whaleyjoshua2/Dying-Earth/issues/355) is
answered, and until then its own sweep proves nothing about its figures.

The other measured fact it lands on: **one to five stations stand off Earth at the end of each
twenty-game batch**, and all 198 Refuel orders were at a station of the seat's own, none at a
partner's. There is almost nowhere off Earth to fill a tank.

## The rule

- **Two Fuel from every Ship in the orbit, once per Battle**, fought or not, armed or not, whichever
  side it is on and whether or not it opened the fight. A Battery pays nothing, having no tank. Not
  per round: nobody knows before committing how many rounds a Battle will run.
- **A hull that could not pay fights at half strength**, rounded down, **on either side**. All or
  nothing: two Fuel out of thirty means the only hulls that fail are at 0 or 1. The penalty follows
  the hull while it stays dry, with no rule of its own.
- **Only a Frigate (3) and a Battleship (7) have any strength**, so the penalty is invisible on the
  Colony Ship, the Carrier and the Missile Carrier. They pay and are otherwise untouched, because
  the escort rule already makes an unarmed hull the first thing killed.
- **A warship must hold at least the charge to hold Orbital Control, to CONTEST an orbit, to
  blockade or to intercept.** Below the bar it does none of the four. The bar is the charge itself,
  not merely more than nought, so a warship holds an orbit exactly as long as it could still fight
  for it.
- **Read live.** A fleet that spends its last Fuel winning a Battle can lose the orbit it just won
  to a single fresh Frigate arriving next turn. That is the designer's choice and it is in a
  picture below.
- **The zero-Fuel trap stays**, at the designer's word: *"for now its stranded."*

## What the specification got wrong

Three things, all found by the build and corrected in it.

- **I invented an Intercept order gate that does not exist.** Only Blockade has one. So this ticket
  writes the Intercept gate, which means it adds a **refusal the player will see** that the
  specification had not priced.
- **My shape for the computer's Fuel weight could not bite.** Only one stance candidate is ever
  offered per stack, so a multiplier on the Attack had nothing to lose against and was inert. The
  weight now adjusts the **odds** the seat reads instead, which is the only place it can change a
  decision. At 1.0 the reading is byte-for-byte what it was before the ticket.
- **Mars's cheapest leg is 2 Fuel, the same as the Battle charge.** My table listed legs *from
  Earth*. This blinded the first attempt at the pinning test for `stranded`, which **passed against
  a deliberately broken state** because at Mars the two bars are indistinguishable. It was rewritten
  at the Moon, where the cheapest leg is 6, and the test records why it must not move back.

## The pictures

Captured headlessly through `shot:` mode with the new `drybar:1` and `drybar:all` aids, and every
one looked at before this was written.

| picture | what it shows |
|---|---|
| [`ship-card-dry-and-fuelled.png`](ship-card-dry-and-fuelled.png) | A dry Frigate and a full Battleship in one orbit. *TSV Valiant: **strength 1***, halved from 3, above *TSV Defiant: strength 7*. The band reads *Custodians: 2 Ship(s), **strength 8***, and the odds line *your strength 8 against 3*. Every figure on the card agrees. Then the cost paragraph: *"The Battle costs every Ship in the orbit 2 Fuel from its own tank, yours and theirs alike, struck or not. 1 Ship(s) of yours would come out of it under the bar, holding no orbit here and fighting halved until refuelled."* |
| [`blockade-refused-dry-stack.png`](blockade-refused-dry-stack.png) | **The rule in one frame.** Both Custodian warships dry: *strength 1* and *strength 3*, the band *strength 4*. **Orbital Control of low orbit: Prospectors** — their single fuelled Frigate holds the sky over two dry warships. Intercept and Blockade are **greyed**, and the refusal says *"no warship of yours here holds the 2 Fuel a Battle costs; a dry hull blockades nothing"*. |
| [`intercept-refused-dry-stack.png`](intercept-refused-dry-stack.png) | The same board, the Intercept refusal. |
| [`tanks-block-dry-warning.png`](tanks-block-dry-warning.png) | The Tanks block: the Valiant's stranded warning, then on a line of its own *"dry: under the 2 Fuel a Battle costs -- no Orbital Control, no blockade, no intercept, half strength, until it refuels"*. The Defiant beside it, full, carries no dry warning. |
| [`roster-row-and-tank-hover.png`](roster-row-and-tank-hover.png) | A hull that is **both dry and stranded**, its roster row reading in full at 1280 with its order ring still showing, and the tank hover open at **six rendered lines**. |
| [`roster-dry-and-stranded.png`](roster-dry-and-stranded.png) | Both roster rows together. |
| [`attack-odds-hover.png`](attack-odds-hover.png) | The Attack hover, **six rendered lines**. |
| [`blockade-stance-hover.png`](blockade-stance-hover.png) | Six lines, ending *"A warship with less than 2 Fuel in the tank blockades nothing."* |
| [`intercept-stance-hover.png`](intercept-stance-hover.png) | Six lines, ending *"A hull with less than 2 Fuel in the tank catches nobody."* |

## Three things fixed that were wrong before this ticket

- **Every strength the interface printed overstated a dry hull.** `ship_strength` is the card's
  figure plus Hardened Hulls and knows nothing of the tank. Nine call sites printed it, and the
  *same sentence* would have shown two different numbers if only the four named in the
  specification had been fixed. All nine now read **`Game::ship_fighting_strength`**, a new engine
  function beside `ship_strength`. It lives in the engine and not the interface deliberately: a
  helper in the interface plus a rule about which of two functions to call is exactly the trap the
  next call site falls into. `ship_strength` itself stays tank-blind, because the melee takes one
  reading of the tanks **before** the charge and a tank-aware `ship_strength` would halve a hull
  twice.
- **A refused stance click did nothing and said nothing.** `stance_row` called `check_order` inside
  the click handler and dropped the error. Survivable while the only refusal was "no warship of
  yours sits in an orbit to blockade here", which a player can see for themselves; not survivable
  now that a full stack of warships can be refused for a reason living in a tank. A refused stance
  is now greyed and carries the engine's own words, the shape every other shut door in the game
  wears. **This reaches an older refusal too**, from version 0.08.5.
- **The tank hover had been over the six-line ceiling since it was written**, measured at seven.
  Cut to six. Nothing was dropped but words, and the orbit clause is now more accurate than what it
  replaced: since version 0.09.0 a station fuels only a Ship in its **own** orbit, which *"where the
  Ship sits"* did not say.

## The measurement

`sweep -- 20 --seatings --balance --steps=300 --sinks=6`, per-Faction totals read at the foot.

| figure | before | after |
|---|---|---|
| Custodians / Prospectors / Arkwrights / Archivists, wins of 80 | 4 / 29 / 1 / 0 | **4 / 29 / 1 / 0** |
| collapses | 45 of 80 | **45 of 80** |
| Fuel burned in Battle | did not exist | **4** |
| hulls left dry by a Battle | did not exist | **0** |

Identical on every line, because the charge spends no dice. **Two hulls paid two Fuel each, once, in
eighty games.** Said plainly: until the computer fights in orbit, this rule is invisible in the
sweep, and the figures in it are untested by measurement.

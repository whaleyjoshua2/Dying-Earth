# The Missile Carrier, and what firing one does to the sky

Ticket [#343](https://github.com/whaleyjoshua2/Dying-Earth/issues/343) on version 0.09.1, the first
ticket of the version. The designer's line was *"time for nukes, increase natural sink
incrementally"*, and on being asked, *"The effect of using the nuke should increase the natural sink
by a small amount"* -- one rule with two halves, which is why ticket #344 was folded into this one
and closed.

`SPEC.md` beside this file is the build specification, and the
[resolution comment](https://github.com/whaleyjoshua2/Dying-Earth/issues/343#issuecomment-5820670124)
on the ticket is the authority over it.

## What the game gained

This ticket is three firsts at once. **Nothing in this game had ever destroyed a place**: a Bombard
burns one Module, an Occupation timing out rolls a quarter of the buildings, and a Battle since
ticket #298 burns nothing at all. **Nothing had ever been consumable**: no order spent a unit and no
unit carried a one-shot. **No Tech had ever unlocked a weapon.**

- **The Missile Carrier**, the fifth Ship type. Sixty Materials and sixteen Widgets, dearer than a
  Battleship because it ends games, and frailer than a Colony Ship: no strength, no pursuit, three
  hit points. It cannot fight at all.
- **It is a Ship but never a warship.** It holds no Orbital Control, blockades nothing and
  intercepts nobody, and ticket #326's escort rule -- which reads exactly `is_warship()` -- strikes
  it only once its party has no warship left standing. That frailty is the counter by decision, and
  the game has no anti-missile rule.
- **The Warhead**, the first thing in the game that is spent by using it. It comes with the hull and
  is gone the moment it is fired, whether or not anything burned. A **Rearm** loads another at a
  Shipyard of the hull's own Faction, so a carrier that has fired is out of the war for the round
  trip home.
- **Missile Technology**, the first weapon Tech. The tree is shared, so opening that door is a
  public act every seat can read.
- **Launch.** At a Region, a ground Colony or a Space Station a rival directs, from the orbit that
  touches it, held outright. Unlike a Bombard it is **lawful over Earth**, and a Region is a lawful
  target: a nuke at home is the point of the weapon.

What a Launch does: every building at the place rolls a **60%** chance to burn, each on its own,
where an Occupation rolls 25%; the Core Module and the Archive are spared, so a place is gutted and
never removed from the board. **Between two fifths and three fifths of the people die**, drawn per
strike. At a Region the **Standing Army dies** and the **Industry Level falls by one**, never below
the board the Region started on and raisable again by the ordinary order. It is an Offence at
**rung 4**, a new rung above the three the game had.

**On Earth alone** it fouls the air by 20 ppm and raises the **Natural Sink by 0.25 for good** -- at
the designer's word, *"I don't see how a nuke on mars could possibly effect the climate on earth."*

## The Sink Weakens now subtracts

The Break at +2.0 C used to **assign** the Natural Sink the figure 4.0, which overwrote it and
erased whatever had raised it first. It fires in twenty of twenty seeds in every seating by about
turn 20, so nothing that moved the Sink before then survived. It now **takes 2.0 off** instead.

On an untouched game the two are identical, 6.0 - 2.0 = 4.0, so no existing measurement moves. What
changes is that a Sink somebody has raised keeps what it was given -- a nuke's soot, and the
Custodians' Research Directive, which had been quietly wiped every game since it was added.

The trap, and the reason the projection has its own red witness: the assignment had to re-add the
Scrubbers because it threw the running figure away, and a subtraction must **not**, or the collapse
forecast counts them twice and tells the player there is nothing to act on.

## The pictures

All captured headlessly through `shot:` mode with `nuke:1 seed:7 cardshut:1 panel:0
window:1280x900`, and every one looked at before this was written.

| picture | what it shows |
|---|---|
| [`launch-and-rearm-card.png`](launch-and-rearm-card.png) | The Ship stack card over Earth. Three rows: the battleship *TSV Vanguard*, then *TSV Indomitable: strength 0, damage 0/3, in low orbit* with **Warhead aboard** in amber, then *TSV Implacable ... at ISS* with **Warhead spent** in weak grey. Under the stance row a **Launch** block: *Target: Australia (Archivists)* in a drop-down, one live button and one greyed. Then a **Rearm** block explaining that a Warhead joins the Shipyard's queue like any other build, and `Rearm TSV Implacable 40 🛒 8 ⚙`. |
| [`launch-hover.png`](launch-hover.png) | The Launch button's hover, **five lines**, under the six-line ceiling: the 60% per building, the 40-60% of the people, the Region's Army and Industry Level, the rung-4 Offence and the Accord it breaks, and Earth's 20 ppm and quarter-point of Sink. The first draft ran to eight lines and a first cut to seven; the designer sent *"you gotta cut some the wording on mouse over on lauching nukes"*, and what went was justification, never a figure. The last sentence out was the orbit rule, which is the only line here not about what firing **does**: this hover is read on a button already live, so the rule has been met, and a hull that has not met it reads the engine's refusal on the greyed button instead. |
| [`launch-refused-no-warhead.png`](launch-refused-no-warhead.png) | The greyed second Launch button with the engine's own refusal reaching the player verbatim: *this Missile Carrier has fired its Warhead; Rearm it at a Shipyard of yours*. |
| [`rearm-hover.png`](rearm-hover.png) | The Rearm hover, in the shape a Module build button's hover has, which is what makes a Rearm read as a build: *40 🛒, 8 ⚙, about 2 turns here*, then what it buys and which yard's queue it joins. |
| [`roster-missile-carriers.png`](roster-missile-carriers.png) | Your roster, two carrier rows wearing the new glyph between a battleship and a frigate wearing the warship glyph, each row saying **Warhead aboard** or **Warhead spent**. |
| [`shipyard-build-list.png`](shipyard-build-list.png) | The ISS card's *Ships (Shipyard)* block, five live buttons, `Missile Carrier 60 🛒 30 ⛽ 16 ⚙` last, in enum order after the Battleship. |
| [`icon-credits-sheet.png`](icon-credits-sheet.png) | The Credits screen, where the new glyph is judged against the rest at 20px. An upright finned missile, drawn in house and owing no credit. The Colony Ship already wears Lorc's *diagonal* Rocket, so upright against diagonal is what keeps the two apart; a first, diagonal attempt was killed on this sheet for being a mirror of it. |

## The measurement

`sweep -- 20 --seatings --balance --steps=300 --sinks=6`, eighty games, per-Faction totals read at
the foot rather than summed per seating.

| figure | value |
| --- | --- |
| Custodians / Prospectors / Arkwrights / Archivists, wins of 80 | 3 / 30 / 1 / 0 |
| collapses | 45 of 80 |
| Natural Sink at the end, median by seating | 4.16 / 4.04 / 4.16 / 4.25 |
| Missile Carriers built | 1 |
| Launches fired | 0 |

**The Sink figure is the one thing this ticket was meant to move**, and it moved: it now sits above
4.0 because a Sink the Custodians raised is no longer erased.

**The weapon does not appear in computer play, and that is a finding rather than a build failure.**
Missile Technology is on no Faction's pick list, so it arrives at about turn 34 of 36; and firing
needs a warship of the same seat holding the orbit outright, which the computer seats almost never
arrange. A labelled diagnostic with the Tech on all four pick lists (reverted, not in the tree)
raised the hulls built to five and the Launches still to zero, so the binding gate is the orbit and
not the Tech. **Bombards, attacks in orbit, Blockades and Interceptions all read zero in the same
sweep**, so the whole orbital-war lane is quiet in 0.09.x and this weapon is not a special case. It
is put to the designer on the ticket.

## What is on the ticket for the designer

Choices the build made that the designer did not, each named so it can be corrected:

- The Tech's **branch and prerequisite** (Propulsion, after Hardened Hulls) and its **name**.
- The carrier's **Energy upkeep** of 2, which the resolution's table did not give.
- The computer's reading of **"cannot take"** and of **"most wants"**, which the resolution left
  open, and that the computer rearms, moves between orbits, and holds at most one armed hull.
- The **glyph**, the words **Warhead aboard** and **Warhead spent**, and that the Launch target is
  chosen from a **drop-down** rather than a button per hull per target, which over Earth would be a
  wall of buttons.

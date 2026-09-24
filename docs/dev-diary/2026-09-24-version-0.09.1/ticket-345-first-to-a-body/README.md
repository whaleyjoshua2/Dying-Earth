# First to a Body

Ticket [#345](https://github.com/whaleyjoshua2/Dying-Earth/issues/345) on version 0.09.1. The
designer's line was *"influence bonus from being first to settle a given body"*.

`SPEC.md` beside this file is the build specification, and the
[resolution comment](https://github.com/whaleyjoshua2/Dying-Earth/issues/345#issuecomment-5822551099)
on the ticket is the authority over it.

## The rule

**Being first means founding a ground Colony on a Body other than Earth, where no ground Colony has
ever stood.** A Space Station claims nothing and closes nothing: a station needs no crew, and a seat
should not win a settling prize by putting an empty shell in orbit. Antarctica does not count, which
matches the Arkwrights' own Victory rule. **Venus can never be claimed**, having no ground Colony
Slots at all, which was found while charting and was not known when the ticket was written.

**One first per Body**, claimed the moment anyone lands. The Body's other slots stay open to
everybody; only the prize is spent.

The prize is two things:

- **The Core's +1.** The first Colony's Core Module pays its **founder** +1 Influence a turn, while
  the founder still directs the place. It **sleeps** while a rival holds it, **never pays the
  rival**, and **wakes** if the founder takes it back. It never hops to a second Colony of the
  founder's on the same world, and it **keeps paying while the Colony is starved of Energy**, where
  a Relay or a Chorus goes quiet: being there first is not a thing the lights going out undoes.
- **The windfall**, paid once: the Moon **5**, Mars **15**, Phobos and Deimos **20**.

**Both sit outside the Faction multiplier, at face value**, on the Spaceport clause's own argument
from ticket #183: being first to a world is a feat of moving people, not of diplomacy, and the
Arkwrights carry ×0.8 precisely because they are bad at diplomacy.

**A tie** goes to `tiebreak_at_body`, the function a contested Colony Slot already uses: the greater
Ship stack at the Body takes it, and a random draw parts only seats level on strength. The tie was
first put to the designer as "drawn at random, as a contested slot already is", which conflated two
things, because that function weighs fleet strength first. Told so, the designer: *"let's keep the
current system for ties."*

## The windfall would have paid nothing

The Influence Allotment is **assigned** at each Income, not added to, so whatever is unspent is
wiped. A Colony is founded in the **Resolution**, which runs after orders and immediately before the
next Income. So a windfall written into the Allotment at the founding is **wiped before the player
can spend a single point of it**, and the feature would have paid exactly nothing while looking
correct in every test that did not check for it.

The fix is the shape the **Spaceport clause** already uses and for the same reason: the windfall is
held in an accumulator on the seat, added after the multiplier, paid into the next Income's
Allotment and then cleared. **It is therefore spendable in the turn after the landing**, which is
the first turn anything can be spent at all. That is a departure from the letter of the decision,
made to honour its intent, and it is named on the ticket.

This is the same shape as the bug fixed one ticket earlier, where the climate Break **assigned** the
Natural Sink and so erased everything that had raised it.

## The pictures

Captured headlessly through `shot:` mode with the new `first:1` and `first:lost` aids, and every one
looked at before this was written.

| picture | what it shows |
|---|---|
| [`solar-map-one-world-taken-three-to-take.png`](solar-map-one-world-taken-three-to-take.png) | The whole system. **the Moon: first settled by the Custodians**; **Mars: first to land: 15 Influence**; **Phobos** and **Deimos: 20 Influence** each. Earth and Venus carry no such line, being unclaimable. One frame carrying a world that has been taken beside three still worth the crossing, which is the argument of the rule. |
| [`mars-card-unclaimed.png`](mars-card-unclaimed.png) | Mars's card, at its head: *First to settle Mars: nobody yet. 15 Influence to the Faction that lands first.* |
| [`moon-card-claimed.png`](moon-card-claimed.png) | The Moon's card in Custodian teal: *First to settle the Moon: the Custodians, at Mare Tranquillitatis on the Moon.* |
| [`colony-card-and-core-tile.png`](colony-card-and-core-tile.png) | The Colony: *Held by the Custodians*, then in teal *First to settle the Moon: +1 Influence a turn to the Custodians, while they direct it.* The Core tile below wears the figure on its label, **Core Module +1**. |
| [`colony-card-founder-lost-it.png`](colony-card-founder-lost-it.png) | The same Colony, *Held by the Prospectors*. In weak grey: *First to settle the Moon: the Custodians' +1 Influence a turn sleeps while another Faction directs it.* The tile label is back to plain **Core Module**. It never says the rival is paid. |
| [`core-tile-hover.png`](core-tile-hover.png) | The Core tile's hover, **six rendered lines**, at the ceiling and not over. |
| [`colony-card-hover.png`](colony-card-hover.png) | The Colony card line's hover, five lines. |
| [`planet-card-hover.png`](planet-card-hover.png) | The planet card line's hover, five lines. |
| [`allotment-hover.png`](allotment-hover.png) | The Allotment explanation, rewritten and **six rendered lines**. It had named the base and the Regions and nothing else since version 0.07.4, omitting Embassies, Relays, Choruses and the whole Spaceport clause. It now names every source. |
| [`moment-first-to-a-body.png`](moment-first-to-a-body.png) | The Moment: **5 Influence**, then *"The Custodians are the first to settle the Moon."* |
| [`report-line.png`](report-line.png) | The Report: *"The Custodians are the first Faction ever to settle the Moon. Mare Tranquillitatis on the Moon claims a windfall of 5 Influence, and its Core will pay while they hold it."* |

**A defect this ticket caused, and fixed.** Phobos and Deimos stand about ten pixels apart on the
Solar System Map and both their labels hang below their discs. Giving each a second line made the
two boxes overlap, and Deimos's background painted over Phobos's first line. A fixed stagger would
not answer it, because the two moons **move round Mars as the ephemeris turns** and which of them
sits higher is not fixed. So a label that hangs below its disc is now **dropped, whole, below any
Body label already drawn that it would lie over** — the rule the orbit band's site labels already
follow, using the block `label_at` has handed back since ticket #335 for exactly this. Only labels
that hang below are moved: a planet's stands above its disc under the Orbital Control flag, and
nudging those downward would walk them into the disc.

## The measurement

`sweep -- 20 --seatings --balance --steps=300 --sinks=6`, eighty games, per-Faction totals read at
the foot.

| figure | before | after |
|---|---|---|
| Custodians / Prospectors / Arkwrights / Archivists, wins of 80 | 3 / 30 / 1 / 0 | **4 / 29 / 1 / 0** |
| collapses | 45 of 80 | **45 of 80** |
| a Colony founded in the Mars system | **0 of 80** | **8 of 80** |
| firsts claimed | did not exist | **40 of 80** |

Firsts by Body: the **Moon 32**, **Deimos 4**, **Mars 3**, **Phobos 1**.

**The rule needed a second hook to exist at all.** The specification named the founding appetite, and
that alone changed nothing: by the time a loaded Colony Ship is at a Body, the destination was
already chosen, and the destination ranking weighed slot yields against flight time with no term for
the prize. A term was added there too, and it produces every one of the eight Mars-system foundings.

**Two things for the designer**, both on the ticket:

- **The moons of Mars outrank Mars itself**, since Phobos and Deimos pay 20 against Mars's 15. The
  computer reaches past Mars for Deimos. If Mars was meant to be the destination, Mars wants the
  largest figure.
- **The Custodians never claim a first on any Body in eighty games.** Not an appetite problem: their
  first loaded Colony Ship reaches Earth orbit around turn 29, and by then a Mars flight no longer
  fits in the turns left. That is about how late they build Colony Ships, which is outside this
  ticket and no figure in `ai.toml` can reach it.

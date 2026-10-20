# Version 0.1.0.3, the fair-fight version

The map is [Map: version 0.1.0.3](https://github.com/whaleyjoshua2/Dying-Earth/issues/558); the spec
is [`docs/spec/version-0.1.0.3.md`](../../spec/version-0.1.0.3.md). The standing baseline is 0.1.0.2's
closing board, **3 / 23 / 26 / 13 and 14 collapses**
([`final-0.1.0.2.txt`](../2026-10-19-version-0.1.0.2/sweeps/final-0.1.0.2.txt)), against the
designer's 15 each and 20. The list came from the
[0.1.0.2 playtest](../2026-10-19-playtest-0.1.0.2/README.md).

## One Bank to a Region

Ticket [#560](https://github.com/whaleyjoshua2/Dying-Earth/issues/560), spec §1.

Built by the Launch Site's door in `check_order`, the Investment Bank counted as one by its job.
A test was written first and watched red (a second Bank accepted), then green; one older test, which
ordered an Investment Bank into a Region with a Bank already queued, clears the queue first.

**The picture** ([`ticket-560/one-bank-refusal-earth.png`](ticket-560/)): China's card with a Bank
ordered (`order:bank`) and a free slot's build list open: the Bank line greyed, *"this Nation State
already has a Bank"*.

**The sweep** ([`sweeps/ticket-560.txt`](sweeps/ticket-560.txt)): **10 / 29 / 16 / 12 and 13
collapses**, from 3 / 23 / 26 / 13 and 14. Three moves past noise, and not the ones a Bank cap
promises: the Custodians +7, the Prospectors +6, the Arkwrights -10. Every gate completed sooner
(the Arkwrights' in 79 of 80 games at median turn 29, from 72 at 31). A reading, not a measured
cause: the cap takes Ducats from every seat alike (the computer builds Banks as a producer for all
four), and the Arkwrights, the poorest in Ducats, lost the most; the Prospectors' Fund is paid by
interest, one Investment Bank a Region already, so the cap cost them nothing there. Measured
behaviour, not a rule; the next tickets (Influence at 5, the Fund's weight) move the same seats.

## A Scrubber at 4 Energy and 1 Ducat, one per 300 million

Ticket [#561](https://github.com/whaleyjoshua2/Dying-Earth/issues/561), spec §2.

The designer's "one per 3 pop" read as one per 300 million (the game writes emissions "per hundred
million"), floor 1, at the designer's word. A general `ducats_upkeep` field on the Facility card,
charged at the Income beside the Energy; a test written first and watched red with the charge
stubbed out ("two Scrubbers cost 2 Ducats a turn: 9.6 to 9.6"), then green. The cap test re-pinned
to the new figures (Russia 1, India 6, the ceiling at 3,300 million).

**The picture** ([`ticket-561/china-three-of-five-earth.png`](ticket-561/)), seed 3 after eight
turns: China's card reads "Scrubber (3 of 5)" (1,461 million / 300 = 4.9), with three standing,
all mothballed by the computer Custodians for want of Energy. Measured behaviour, not a rule.

**The sweep** ([`sweeps/ticket-561.txt`](sweeps/ticket-561.txt)): **9 / 24 / 20 / 11 and 15
collapses**, from 10 / 29 / 16 / 12 and 13. The Prospectors -5 is past noise, the Arkwrights +4 at
its edge, the Custodians unmoved at 9. The computer Custodians built about as many Scrubbers as
before (493 against 464 in the first seating): the Ducat gate (`income_last_turn.ducats >= 1`) is
nearly always open, and 4 Energy against 3 shut none the Energy check had not already. Measured
behaviour, not a rule: the dearer Scrubber bites a human spamming nineteen of them more than a
computer building a handful.

## The Wind Farm

Ticket [#559](https://github.com/whaleyjoshua2/Dying-Earth/issues/559), spec §3.

Built by an agent against the spec, each test watched red first (six, all for the stated reason:
the row without its Ducat, the order accepted without Clean Power, the Mars Module at 2.5 before the
yield, Replace accepted without the Tech, the computer ordering both power kinds). **The design
choice for Replace**: a fourth `BuildingChange`, a `BuildItem::Replace` in the Region's queue and a
`replacing` flag on the Power Plant, rather than a slot index, because the sea and a Decommission
shift the indices. The computer's power counts now read each card's Energy figure where `* 6` was
hard-coded. One judgement left open for the designer: the Archivists' first Reactor is still
offered when Clean Power stands, since its clause is the upkeep cut; a second gives way to the
Wind Farm.

**The pictures** ([`ticket-559/`](ticket-559/)): the build list with "Wind Farm 25, 8 Widgets, or
50 Ducats" (`researched:clean_power`, a new shot aid); a Power Plant's strip with "Mothball,
Decommission, 4 Widgets 12.5 Replace"; a standing Wind Farm's tile and line, "+3 Energy, 1 Ducat
upkeep" (`standing:wind_farm`, another new aid).

**The sweep** ([`sweeps/ticket-559.txt`](sweeps/ticket-559.txt)): **10 / 24 / 20 / 17 and 9
collapses**, from 9 / 24 / 20 / 11 and 15. The Archivists +6 and the collapses -6 are past noise:
every seat with Clean Power now raises Wind Farms where it raised Power Plants, and the world warms
slower. The Custodians still sit at 10 against the ideal's 15.

**The review** (an agent that did not build it) found no defect and two gaps, one fixed: the
computer could order a Replace in every Region in one turn, each guarded against its own 3 Energy
alone, and land nine short; it now keeps one Replace in flight at a time. Clean Power's effect text
now says it unlocks the Wind Farm. Left for the designer: a coastal Power Plant the sea drowns while
its Replace is queued leaves the Replace eating Widgets until it completes and is lost with its 12.5
Materials, as any coastal build is lost today, with no cue until then.

## Power burns Fuel

Ticket [#575](https://github.com/whaleyjoshua2/Dying-Earth/issues/575), spec §4. The designer's
0.75 became 0.8 at the designer's word ("go with .8"), since the stockpile keeps tenths and a test
on 0.75 could only read 0.7 or 0.8.

A general `fuel_upkeep` on both building cards; in `producers_of` a Fuel budget from the tank pays
each burning building in the Income's order, and one it does not reach runs at a quarter. The test
was watched red twice, once with the charge stubbed ("a second Power Plant burns 0.8 more") and
once with the quarter stubbed. Two older tests re-pinned: the card-equals-Income test now takes a
building's Fuel off the expected Income, and the Custodian AI's Moon-Mine test fills the tank, since
the Moon Generator's burn had made Fuel the scarcest resource in one of its two scenarios.

**The sweep** ([`sweeps/ticket-575.txt`](sweeps/ticket-575.txt)): **8 / 21 / 17 / 10 and 24
collapses**, from 10 / 24 / 20 / 17 and 9. Collapses +15 and the Archivists -7 are past noise, every
seat down 2 or 3. The collapses sit in the two seatings where a computer seat plays the Custodians
(15 and 9 of 20, from 8 and 1): Scrubbers built fell 421 to 312 and the Sink's median from 19.1 to
12.6 in the second seating. The reading: a computer seat that lets its tank run dry has its power at
a quarter, reads Energy as tight, and a tight seat is offered no Scrubber (`ai.rs`, the Scrubber
candidate's `!tight`); the Custodians' climate lever is the first thing a Fuel shortage shuts. It is
measured behaviour, reported, with nothing changed on the strength of it: the designer decides
whether the computer should keep a Fuel reserve for its power, or the rule should bite less.

**The reserve.** The designer: *"have the AI keep some fuel on hand."* Three turns of the buildings'
burn (`power_fuel_reserve_turns = 3`): under it Fuel is the seat's scarcest resource, no Ship is
refuelled from it, a Ship's top-up buys it back, the market neither sells it nor leaves it short.
The test was watched red with the turns set to nought. The review's one gap fixed in the same
commit: the Energy shortfall forecast now reads the Fuel this turn's orders leave, not the live tank.

**The sweep with it** ([`sweeps/ticket-575-reserve.txt`](sweeps/ticket-575-reserve.txt)): **10 / 25 /
20 / 10 and 15 collapses**: nine collapses back of the twenty-four, the first seating still 13 of 20
(8 before the Fuel rule, 15 without the reserve), its Scrubbers 154 against 176. The Archivists stay
at 10 against 17 before the rule: they burn Fuel for the Archive's power and fly nothing that would
earn it. Reported; nothing further changed.

**The reserve's review** found one gap, fixed: a Refuel takes the tank's want down to nought, so a
seat too poor for a top-up could drain its reserve through the plain Refuel; it now waits until the
whole want stands above the reserve. And the buy up to the reserve, placed in the market block after
everything else was chosen, never fired, since the Ducats were spent by then; it is a trading
candidate now, weighed as the Energy buy is. The test witnesses both, red first.

## Influence at 5 Ducats a point; Distress Call at +2

Ticket [#562](https://github.com/whaleyjoshua2/Dying-Earth/issues/562), spec §5. Two figures in
the data; the two tests that pinned 4 re-pinned to 5. The sweep below carries the reserve's second
fix as well, so its figures are the two together.

**The sweep** ([`sweeps/ticket-562.txt`](sweeps/ticket-562.txt)): **5 / 36 / 13 / 10 and 16
collapses**, from 10 / 25 / 20 / 10 and 15. Prospectors +11 (34 of them outright), Arkwrights -7,
Custodians -5, every one past noise and all the same way. The reading: a dearer point of Influence
is paid by the seats that have Ducats, and only the Prospectors do (80 to 170 a turn against the
Arkwrights' and Archivists' 1 to 6, the playtest measured); the brake meant for the human buying
the world fell on the computer's poor seats instead, and the Prospectors bought what the others no
longer could. Measured behaviour, reported; nothing changed on the strength of it.

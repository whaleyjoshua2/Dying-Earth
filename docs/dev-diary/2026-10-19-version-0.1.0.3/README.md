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

## Influence at 4.5 Ducats a point; Distress Call at +2

Ticket [#562](https://github.com/whaleyjoshua2/Dying-Earth/issues/562), spec §5. Two figures in
the data; built first at 5, the two tests that pinned 4 re-pinned to 5. The sweep below carries the
reserve's second fix as well, so its figures are the two together.

**The sweep** ([`sweeps/ticket-562.txt`](sweeps/ticket-562.txt)): **5 / 36 / 13 / 10 and 16
collapses**, from 10 / 25 / 20 / 10 and 15. Prospectors +11 (34 of them outright), Arkwrights -7,
Custodians -5, every one past noise and all the same way. The reading: a dearer point of Influence
is paid by the seats that have Ducats, and only the Prospectors do (80 to 170 a turn against the
Arkwrights' and Archivists' 1 to 6, the playtest measured); the brake meant for the human buying
the world fell on the computer's poor seats instead, and the Prospectors bought what the others no
longer could. Measured behaviour, reported; the designer set the price at 4.5.

**At 4.5.** The price became a fraction, so `per_influence` is an `f64` now (it was an integer):
four readers changed, the cost of a `BuyInfluence`, the computer's two buy counts, and the Trading
window's price column. The three pins were re-pinned to 4.5 and watched red against the data still
at 5 (5.0 against 4.5; 25 against 22.5; 50 against 45) before the figure moved.

**The sweep at 4.5** ([`sweeps/ticket-562-45.txt`](sweeps/ticket-562-45.txt)): **8 / 29 / 17 / 14
and 12 collapses**. Most of the way back from the sweep at 5: Prospectors -7, Arkwrights +4,
Custodians +3, Archivists +4. Against the figures before the ticket (10 / 25 / 20 / 10 and 15) the
Prospectors hold +4 and the Archivists +4, both at the edge of noise, and the collapses are three
fewer. The dearer point still costs the poor seats something, less than at 5.

**The reserve's second review** found two minor defects, both fixed and witnessed red: a rich seat
bought the reserve and a Ship's top-up (which covers the reserve beside the want) in the same turn,
[5, 20] for a need of 20; the reserve's buy now stands down while a top-up is on offer. And the
scarcity assertion in the reserve's test was green without the reserve, its arithmetic having missed
the free Core row among the three cheapest; the figure moved to where the reserve alone turns the
answer. The earlier commit, 0a2b166, carried a failing older test for a turn, the top-up suppressed
under the reserve; that cut is undone here.

## Production Moved makes x2.08; the computer weighs the trade whole

Ticket [#563](https://github.com/whaleyjoshua2/Dying-Earth/issues/563), spec §6. The ticket's
premise did not survive a read of the sweeps: the Production Moved line is 0 only in the seating
where the Custodians start in East Asia; from the other three starts they double 20 to 30
Module-turns a game. East Asia's Materials lean makes its Mine 6.6, and no Module off Earth reached
that (a plain Moon Mine makes 5.0 since 0.1.0.2). The designer answered with a rule change and a
calculus: x2.08 where it doubled, and the Emissions leaving Earth weighed into the trade by the
Stabilization gap.

**Built.** `production_moved_factor = 2.08` on the Custodians' card, read by the yield (to the
tenth; Widgets floored, Research to the nearest), the card line ("x2.08 for an idle Mine on
Earth"), the Income source, the signature (`{moved_factor}`) and the computer's build boost. The
computer's trade is `M x 1.08 + E x k x g >= F`, k = `production_moved_emission_worth` = 1.0, g the
gap multiplier; several qualifying Facilities are weighed by Emissions per output. Two slips of mine
on the way, both caught by the tree: I wrote 5.6 for a Moon Mine (it is 5.0) and 0.75 for the Mine's
Emissions (the Custodians' x0.75 makes it 0.56); the spec carries the corrected figures. The red
run, with the code reading a factor the data did not yet carry (default 2): 10.0 against 10.4, 18
against 18.7, and from East Asia the Mine beside a Mass Driver was no candidate at all (6 + 0.56 =
6.56 against 6.6); at 2.08 it is (7.04). Five older pins re-pinned.

**For the designer, mid-ticket**: the sweep now prints each Faction's Income over a game, net of
its buildings' upkeep (Materials / Fuel / Energy / Ducats / Research, the mean over the batch), from
a running total on the seat (`produced`, with `research_total`).

**The sweep** ([`sweeps/ticket-563.txt`](sweeps/ticket-563.txt)): **17 / 21 / 20 / 11 and 11
collapses**, from 8 / 29 / 17 / 14 and 12. Custodians +9 and Prospectors -8, both past noise; the
Arkwrights +3 and the Archivists -3 are not. Production Moved by seating: **0 / 34 / 31 / 36**
Module-turns a game (median), from 0 / 21 / 27 / 30: up by a third from the three starts that used
it, and still nought from East Asia. A replay of seeds 1 to 3 from East Asia says why: the computer
Custodians there build no Module off Earth at all through turn 30 (one Observatory on a station over
Earth in seed 1), and lose East Asia by turn 30 in two of the three; from turn 2 the Energy reads a
turn from short every turn and the Scrubber is mothballed and restarted in a loop. Nothing off Earth,
nothing to double. The ticket's bar -- above nought from East Asia -- is missed, and the cause lies
outside its question, in the East Asia Custodians' opening. Reported to the designer.

Income over the batch, per Faction, mean a game (Materials / Fuel / Energy / Ducats / Research), the
new line: Custodians 466 / 608 / 2497 / 2472 / 1310; Prospectors 2180 / 546 / 3217 / 1285 / 516;
Arkwrights 2363 / 517 / 3557 / 4429 / 377; Archivists 641 / 318 / 1417 / 718 / 1001. Income only:
what the market pays a seller is not in it.

## Every computer seat races for the first landing

Ticket [#564](https://github.com/whaleyjoshua2/Dying-Earth/issues/564), spec §7. The human landed
on the Moon by turn 6 to 10 in every playtest; the computer's first landing came at median turn 15
to 32 by seating, the Arkwrights' in 37 of the 50 games anyone landed. Decided in one round: a race
weight of 3 on the Shipyard and the Colony Ship while the seat holds no Colony Ship and a Body is
unclaimed, every seat, the Mars window contested the same way, and the bar at median turn 12.

**Built.** `moon_race_weight = 3.0` (`ai.toml`), read once a turn in `ai_orders` and multiplied
into the Shipyard candidate and the Colony Ship's. Red first with the rule stubbed to 1: "the
Shipyard at three times its base while the race is on: 6 vs 6". The weight reaches far: five older
budget tests that read a fifth build or a station's one slot found a Shipyard at 54 in the way, and
each now parks a Colony Ship of the seat's first (`parked_colony_ship`), so the race is over for it
and the figure it read stands; the turn-one opening test reads the race first (every seat with a
station opens with its Shipyard, no Habitat) and the old by-Faction opening once a Ship is parked.

**The first sweep, a race for any unclaimed Body at x3** ([`sweeps/ticket-564.txt`](sweeps/ticket-564.txt)):
**11 / 53 / 7 / 7 and 2 collapses**, from 17 / 21 / 20 / 11 and 11. The race built the Ships early
and the destination rule sent them past the Moon to the bigger windfalls (the Moon's 5 against
Mars's 15 and Phobos's 20): Mars was landed at median turn 11 to 13 in every seating, the
Prospectors every time, and they met their Victory outright in most seeds. The Moon's first landing
came at median turn 15 / 22 / 14 / 26, the bar of 12 missed, though its winners spread beyond the
Arkwrights at last. A side effect worth keeping in view: Production Moved from East Asia read 15
(the Custodians there left Earth), and 83 / 127 / 105 from the other starts. **At x2**
([`sweeps/ticket-564-x2.txt`](sweeps/ticket-564-x2.txt)): 9 / 47 / 14 / 6 and 4, the Moon no
sooner (14 / 24 / 28 / 28), East Asia back at nought. The designer: "the race is for the Moon".

**The second build**: the lift stands only while the Moon is unclaimed, and a seat's first Colony
Ship flies to the Moon while it is, whatever the windfalls say (`best_body_for`), the windfalls
themselves untouched. Red first with the destination rule removed, on ticket #345's twelve boards:
"the Moon unclaimed, twelve of twelve first Ships fly to it: 0". Two destination tests re-pinned
with the Moon claimed by a rival before they read the windfalls' pull.

**The sweep, the race for the Moon** ([`sweeps/ticket-564-moon.txt`](sweeps/ticket-564-moon.txt)):
**14 / 56 / 4 / 5 and 1 collapse**. The Moon is landed at median turn **9 / 7 / 8 / 8** by seating,
20 of 20 games in each, where the human landed at 6 to 10: the ticket's bar met. Its guard -- no
fall in the four Factions' wins -- is missed by a distance: the Prospectors are first to the Moon in
19 / 18 / 16 / 19 of 20 and win 56 of 80, the Arkwrights fall from 20 to 4 and the Archivists from
11 to 5. The Prospectors' Victory gate moved only 27 to 25, so the early Moon Colony carries them
through the rest rather than the gate. Production Moved from East Asia holds at 15 and reads 93 /
162 / 170 from the other starts; the Custodians' Materials Income doubled (466 to 1001 a game), the
Moon Mine under Production Moved. Income per Faction, mean a game (Materials / Fuel / Energy /
Ducats / Research): Custodians 1001 / 322 / 2399 / 1408 / 881; Prospectors 2066 / 454 / 2935 /
1586 / 408; Arkwrights 1613 / 368 / 2401 / 3269 / 271; Archivists 482 / 274 / 1199 / 484 / 803.
The designer closed the ticket on the Moon measure and asked for a ticket raising the Prospectors'
Venture Fund bar to 4250.

## The Upload at 16, four a turn; the Archive's draw; Modules dark without people

Ticket [#565](https://github.com/whaleyjoshua2/Dying-Earth/issues/565), spec §8. Decided in one
round: the bar 16 and four a turn, the Archive's draw 20 + 1 per uploaded, dark Modules at every
Colony by the slot formula read backwards (Core, Archive and Habitat never darkened), and the
computer uploads four a turn and accepts what goes dark.

**Built.** `upload_per_turn = 4` and `energy_per_uploaded = 1` on the Archive card; the Upload's door
answers the room first, then "four a turn; 2 already ordered". The Archive's card detail reads "20 +
4 uploaded". `unmanned_modules` walks a Colony's Modules in build order giving each a slot while
`module_slots` has one; a Module past the count is offline at Income with its own cause ("too few
people to run it") and comes back as people return. Red first: the cap's door open, the Modules all
running at one Colonist, the computer uploading six. The dark rule reached ten older fixtures that
built Modules on Colonies nobody lived at; each is crewed now, one person a Module, since the rule
makes such a place impossible. The pace table for Uploads is 5 / 11 / 16.

**The sweep** ([`sweeps/ticket-565.txt`](sweeps/ticket-565.txt)): **12 / 58 / 6 / 2 and 1 collapse**,
from 14 / 56 / 4 / 5 and 1. Every move is inside noise but the Archivists' direction: five to two,
their gate's median turn 28 as before, the Archive complete at median turn 34 to 35 in three of
the four seatings and four turns of uploading after it, where one order used to end the game. The
Prospectors' 58 is the standing matter of [The Prospectors' Venture Fund at 4250](https://github.com/whaleyjoshua2/Dying-Earth/issues/580).
Income per Faction, mean a game (Materials / Fuel / Energy / Ducats / Research): Custodians 1001 /
325 / 2408 / 1417 / 884; Prospectors 2078 / 457 / 2946 / 1597 / 410; Arkwrights 1603 / 363 / 2388 /
3269 / 266; Archivists 476 / 275 / 1194 / 480 / 769.

**The review** found one lockout and three leaks, all fixed: the human Archivist's one Upload
button was sized to the room alone, so with eight living it offered "Upload 8" and the door refused
it (sized to the cap less what is ordered now); a dark Generator still reserved its 0.8 Fuel ahead
of the buildings behind it (it reserves nothing now); the computer's Energy picture summed the card
upkeep of dark Modules and a flat 20 for the Archive, and counted dark Generators' output (it reads
the yield's figure and skips the dark now); and the Resolution's grid and card clears woke a dark
Module until the next Income (they keep it dark). Two more uncrewed fixtures surfaced on the last.
The sweep above predates these fixes; none touches a rule the sweep's figures turn on, and the next
ticket's sweep carries them.

## The Venture Fund's growth, shown whole

Ticket [#566](https://github.com/whaleyjoshua2/Dying-Earth/issues/566), spec §9. Not a bug: three
pay-ins and a line that named one. Decided in one round: show the growth whole with its parts; leave
the rate. Built: `venture_interest_last_turn` set at Income beside the banked share; the window's
line and the tool read "Fund 3742; +312 last turn (100 banked, 212 interest)." Red first on the
field (ten of interest against nought) and a window test on the line. No rule moves, so no sweep.

**The review** found the line still short on the two turns an Objective's reward lands (50 or 100,
paid at the end of the Income): the reward is kept too and named on that turn alone; a stranded doc
comment moved back to its function; and the reset to nought on an Income with no Bank paying is
pinned.

## The Fund and the Capital weighed into the percentage

Ticket [#567](https://github.com/whaleyjoshua2/Dying-Earth/issues/567), spec §10. Decided in one
round: the Prospectors' Fund at 0.7; the Arkwrights' Capital a tenth of their score, a part of its
own; a headline with both parts full and a hold standing names the hold ("100% - needs Generation
Ships"); a sweep. Built: `capital_weight` on the card and in `Progress`, `hold_suffix` appended by
the window's row, the Report's head, the game-over page and the ranking. Red first on the figures
(the Prospectors' 59% against 42%, the Arkwrights' 100% against 90%) and the headline's suffix, with
the card's field and the helper stubbed; five older pins moved with the weights.

**The review** found the suffix keyed on the bars where the percentage reads the parts: the
Custodians' best run fills their first part short of the bar (#406), so a broken run that once
reached 4 read 100% with no hold named -- the ticket's own shape. Keyed on the parts now, red first.
Also from the review: a `percent_text` helper nothing in the product called is dropped, stale
comments on the first weight mended, and two things stated for the designer -- the Capital's tenth
opens the computer seats' Trade and Accord door (refused at 95%) to an Arkwright seat one upgrade
from winning, and the ranking page names a hold twice, heading and chronicle.

**Swept** ([`sweeps/ticket-567.txt`](sweeps/ticket-567.txt)): 9 / 60 / 5 / 6 and no collapse, from
12 / 58 / 6 / 2 and 1 -- noise. Three games of 80 went to the last-turn ranking. The computer
Arkwrights held a Capital at the end in 4 of 80, so their tenth is off in nearly every game. Income
per Faction (Materials / Fuel / Energy / Ducats / Research, mean a game): Custodians 999 / 274 /
2299 / 1403 / 827; Prospectors 2092 / 461 / 2881 / 1572 / 402; Arkwrights 1658 / 370 / 2407 / 3268 /
286; Archivists 469 / 266 / 1175 / 484 / 749.

## The Prospectors' Venture Fund at 4,250

Ticket [#580](https://github.com/whaleyjoshua2/Dying-Earth/issues/580), spec §11. The designer's
figure, from the race for the Moon. One round: the bar and its text, the pace scaled, a sweep.
Built: `bar = 4250`; the card's long text reads the bar through `victory_text` (it carried a typed
3500); the pace table x 4250/3500. Red first on the bar, the text and the pace, with the renderer
stubbed to the text as written; nine older pins moved.

**The review** found the headless shot aid `rival:1` still at a Fund of 2625, which at 4,250 scores
0.67 and fires no Moment, so the Report's rival-Moment picture was dead: 3230 now. Also from the
review: the pace asks a fifth more at every turn, so the computer Prospectors read behind earlier,
not "at the same turns" as the spec first said; stale pin comments mended.

**Swept** ([`sweeps/ticket-580.txt`](sweeps/ticket-580.txt)): 9 / 59 / 5 / 7 and no collapse, from
9 / 60 / 5 / 6 -- the bar did not bite. The gate's median turn 25 as before; games still running at
turn 39, 40 of 80 from 37. The Prospectors' Fund at the end rose by about the bar's fifth (4,495 /
8,113 / 3,753 / 4,352 by seating, from 3,780 / 8,025 / 3,578 / 3,585): the pace table scaled with
the bar, and the computer Prospectors bank to their pace. Income per Faction (Materials / Fuel /
Energy / Ducats / Research, mean a game): Custodians 986 / 277 / 2352 / 1421 / 844; Prospectors
2274 / 481 / 3053 / 1481 / 447; Arkwrights 1688 / 366 / 2437 / 3368 / 292; Archivists 476 / 271 /
1187 / 504 / 786.

## An Influence spend that would break an Accord is refused unless it says break

Ticket [#569](https://github.com/whaleyjoshua2/Dying-Earth/issues/569), spec §12. One round, five
questions, all at the recommendation: the buttons warn in red; refuse unless the order says break
(a two-click button, the tool's word); Influence alone; the offence falls on whoever directed the
place as the order was given; a sweep. Built: `break_accord` on the order, the door in
`check_order`, the aim carried with the pending spend, one `influence_button` helper behind the
window's three buttons. Red first on the door (an unrefused spend), the aim (a broken Accord) and
the button's words.

**The review** found the arming stale across a selection change and End Turn, and shared across the
three buttons whatever the amount: keyed on place and amount now, disarmed by any click elsewhere and
by End Turn. The card's greyed Spend had lost its words under the refusal: the hovers go through
`rule_tip`. And an Accord struck after an Influence order was already aimed at the partner's place
left End Turn refusing and dropping the order with a log line alone: the window withdraws it at once
and says so. The withdrawal has no test of its own -- it lives in the window's action handler.

**Swept** ([`sweeps/ticket-569.txt`](sweeps/ticket-569.txt)): 19 / 49 / 4 / 8 and no collapse, from
9 / 59 / 5 / 7. Too big for a relations rule, so an experiment: the aim rule alone reverted reads
19 / 48 / 4 / 9 -- the same table. The mover is the door reaching the computer seats: a candidate
spend on a partner's place used to pass `check_order`, take the Influence budget and be dropped at
the end of `ai_orders`, the Influence wasted; refused at the candidate stage now, the next candidate
spends it, and the Accord-keeping seats (the Custodians above all) stop throwing Influence away.
Income per Faction (Materials / Fuel / Energy / Ducats / Research, mean a game): Custodians 1100 /
352 / 2535 / 1561 / 931; Prospectors 2139 / 442 / 2709 / 1190 / 386; Arkwrights 1538 / 319 / 2309 /
3210 / 266; Archivists 477 / 261 / 1149 / 484 / 746.

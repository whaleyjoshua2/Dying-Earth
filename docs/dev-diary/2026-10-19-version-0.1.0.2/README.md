# Version 0.1.0.2, the legibility version

The map is [Map: version 0.1.0.2](https://github.com/whaleyjoshua2/Dying-Earth/issues/547); the spec
is [`docs/spec/version-0.1.0.2.md`](../../spec/version-0.1.0.2.md). The standing baseline is 0.1.0.1's
closing board, **7 / 24 / 19 / 19 and 11 collapses**
([`final-0.1.0.1.txt`](../2026-10-12-version-0.1.0.1/sweeps/final-0.1.0.1.txt)), against the
designer's 15 each and 20.

## Map labels give way

Ticket [#548](https://github.com/whaleyjoshua2/Dying-Earth/issues/548), spec §1.

**The pictures** ([`ticket-548/`](ticket-548/)), taken headlessly in `shot:` mode, seed 3, turn 1:
- `before-earth.png`, `after-earth.png` (`panel:0 look:50,32`): before, Turkey, Iran and Saudi
  Arabia ran together, the United Kingdom lay over the European Union, and China over Japan. After,
  Saudi Arabia, Vietnam and Indonesia stand as their glyph and name; the European Union and China
  hide, each under a neighbour's Army shield; the ISS and Axiom keep their names.
- `china-selected-earth.png` (`look:80,35 select:eastasia`): China selected, its full block laid
  over Japan's shield. This is the case that put a question back to the designer (below).
- `before-solar.png`, `after-solar.png`: before, Mars's label lay across Ceres's, and the Earth L5
  name sat under the stack labels. After, the stacks stand whole, the Moon shrinks to its name, and
  Mercury, Phobos and Vesta hide, neutral and further from the middle than the Moon, Mars and Deimos.
- `after-mars.png`: the orbit band, a fixed mark, with the slot labels clear of it.
- `start.png`: the start screen's globe, every name standing.

**Found in building.**
- Labels set by hand a pixel or two apart counted as collisions: the Solar System Map's stack
  labels, 16 pixels apart in blocks 19 tall, knocked each other out in the first picture. Two blocks
  may now overlap by two pixels a side, the padding round their text.
- A neighbour's shield hid China, the player's own Region, even when selected, since fixed marks
  outranked every label. Put to the designer, who chose that **the selected label and the one under
  the pointer are laid over the marks**; every other label still gives way to them.
- Open windows (the Climate Panel, a card) are fixed marks too, so a label under one, which is not
  seen, does not push a seen one aside.

**The review** (an agent that did not build it) found two defects and three gaps, all fixed:
- A station's or stack's name gave way to its own glyph, so the ISS and Axiom vanished from the
  globe, and a far orbit's name could stick hidden. Each glyph is now its label's own mark.
- Labels under the side panel or the top bar still claimed room; and the selected label, laid over
  marks, also ignored open windows. The panels and windows are now hard marks every label gives way
  to.
- The pointer raised a label within 24 pixels of its place, a Region's click within 30. Now 30.

Its retake found a third defect: the six-pixel margin for growing back was measured against labels
set flush by hand, so the Solar System Map's stack rows, once hidden for a frame, stayed hidden for
good. The margin now holds for half a second after a label gives way. With it, Egypt and India,
held short by the same fault, stand whole.

**Left as it is**: the Solar System Map's Battle mark is set from the width of the Body's full
label, so beside a label shrunk to its name it stands off to the left.

**The tests.** `src/labels.rs` holds the layout, apart from the painting, with eleven tests: rank
order, nearness to the middle within a rank, shrink then hide, a mark's owner exempt, the slack,
the margin and its half-second hold, the margin from hidden to short, the selected and hovered over
marks, the hard marks, and the fade's memory. Each rule was broken on purpose and its test watched
to fail before it was believed.

No rule moves, so no sweep.

## Fewer words: the Widgets hovers and the Climate Panel

Ticket [#549](https://github.com/whaleyjoshua2/Dying-Earth/issues/549), spec §2.

**The pictures** ([`ticket-549/`](ticket-549/)), seed 3, turn 1:
- `widgets-bar-hover-earth.png` (`tip:across the places`): the top bar's hover, its header and a
  line each for China and the ISS, the rule gone. The Climate Panel behind it ends its penalty
  line at the growth figure.
- `widgets-card-hover-earth.png` (`select:eastasia tip:a turn here`): China's Widgets line, its
  figures alone.

The designer kept the place lines and their queues on the top bar ("cut the rule leave everything
else"), so with many places directed that hover still runs long. No rule moves, so no sweep.

## The Launch Site line's hover

Ticket [#550](https://github.com/whaleyjoshua2/Dying-Earth/issues/550), spec §3.

**The cause was not the one charted.** The ticket guessed that the row did not sense the pointer at
all. It does: egui gives every row a hover sense. But egui's labels are selectable by default, so
each word senses a click and a drag, and egui's hit test then hands the pointer to the words rather than the
row beneath them. The hover fired only in the gap between "Launch Site:" and "Yes". Every hover
hung on a glyph-and-words row (`icon_word`, `card_heading`) had the same fault. The fix is one
line at the root: no text in the game is selectable. The top bar's `.interact(Sense::hover())`
calls were never what made its hovers work: egui's `Response::interact` re-registers a widget in
place and does not lift it above its children.

**The test** drives egui with a pointer over the words of a "Launch Site: Yes" row and asks whether
the row is hovered. It is red under egui's default (asserted in the test) and was watched red with
the fix undone.

**The picture** ([`ticket-550/launch-site-hover-earth.png`](ticket-550/)): China's card, drawn as
before, its Launch Site hover forced open (`tip:any orbit`). A headless picture cannot move a
pointer, so the picture shows only that nothing else changed; the test is the evidence.

The designer allowed "a couple pixels" beyond the words; the row's own area, gaps included, was
enough, so none were added. No rule moves, so no sweep.

## Plurals print right

Ticket [#551](https://github.com/whaleyjoshua2/Dying-Earth/issues/551), spec §4.

The designer chose the widest scope: every "(s)" and every counted noun that can print with 1. The
helper `counted` sits in the engine with its own test, watched red with the singular rule broken.
The sweep of call sites touched the window (`src/ui.rs`), the engine's Report, refusal and Victory
lines (`orders.rs`, `resolution.rs`, `state.rs`, `economy.rs`, `victory.rs`, `events.rs`,
`climate.rs`, `turn.rs`), about twenty-five Report templates in `assets/data/report.toml` (the
number and noun now passed whole from the caller, and three new arguments, `wait`, `are` and
`were`, for verbs that agree), and the text tool. Lines only written to the game's log were left
as they are: the sweep tool counts some of them by their words. None of the words it counts were
touched.

**The pictures** ([`ticket-551/`](ticket-551/)), seed 3, four turns played:
- `one-facility-report.png`: the Report open over the globe; below it a Region label reads
  "1 Facility, 5 free slots".
- `slots-earth.png`: the European Union's "8 Facilities, 0 free slots", and the roster's "China:
  8 Facilities, 3 free slots".

No rule moves, so no sweep.

**The review** (an agent that did not build it) found one defect: the Roster's row for a Ship in
flight read "3 turns turn(s) left", the counted phrase put in front of the old "(s)", which no test
covers. Fixed, with the Antarctic ice's Report line, which still said "Colony Slots" after a count
its Moment already counted. Four log-only lines did change ("(s)" in each); nothing reads them.

## A Colony Ship counts Pioneers as they board

Ticket [#552](https://github.com/whaleyjoshua2/Dying-Earth/issues/552), spec §5.

**The cause**: a Load is an order carried out at the Resolution, and the figure read only who was
aboard. Version 0.1.0.1 made a lift free of the Ship's turn, not immediate. The figure now reads
`aboard_as_ordered`, the Colonists aboard plus this turn's Loads less its Unloads for that Ship.

**Found in building**: under the figure, a station's load row still offered the Ship's room before
this turn's Loads (the Region rows already took them off), so a Ship full by order offered "Load 2
Colonists from ISS over Earth". Every row now takes every Load placed off the room.

**The pictures** ([`ticket-552/`](ticket-552/)), `liftgo:1 stack:earth ship:1`: an empty Colony
Ship at Earth with a lift of four placed. `four-ordered-aboard-earth.png` is the first build: the
figure reads 4/4 (it read 0/4 before), and the ISS row still offered two. `four-ordered-full-earth.png`
is the finished build: 4/4, and no row offers more.

**The test** counts a Ship's own Loads and Unloads and not another Ship's, with the yellow
following the ordered figure. It was watched red with the pending orders ignored. No rule moves,
so no sweep.

## Why Mercury is always two turns away

Ticket [#553](https://github.com/whaleyjoshua2/Dying-Earth/issues/553), spec §6. **No change**, at the
designer's word.

**Measured** (`transit_cost_at(Earth, Mercury, t)`, turns 1 to 50, seed 1; a throwaway example, not
kept): 2 turns in 26, 3 in 13, 4 in 9, 5 in 2, alternating, every odd turn 2. Venus runs 3 to 7 and
Mars 5 to 9, changing slowly. The designer then raised that real probes take years and many flybys
to reach Mercury. That is so (MESSENGER, 2004 to 2011, six flybys), and the game's direct flight is
the price of skipping them: the dearest Fuel of any crossing from Earth. Offered a slow, cheap flyby
route beside it, a longer direct flight, or Mercury reached only through Venus, the designer kept
it as it is.

## The Moon, Phobos and Deimos, new yields

Ticket [#554](https://github.com/whaleyjoshua2/Dying-Earth/issues/554), spec §7.

The ticket asked why Phobos's yields were low. The answer: it was the mining rock, low elsewhere by
design, and the 0.1.0.1 Mine cut took it from 1.75 to 1.49. The designer then set new figures for
three Bodies: the Moon's Mine 1.25 and Generator 1.3; Phobos's Generator and Research 1.2; Deimos's
Mine 1.01, Generator 1.1, Refinery 0.52, Research 1.15.

**The sweep** ([`sweeps/ticket-554.txt`](sweeps/ticket-554.txt), 20 seeds by four seatings):

| | Custodians | Prospectors | Arkwrights | Archivists | collapses |
|---|---|---|---|---|---|
| 0.1.0.1 | 7 | 24 | 19 | 19 | 11 |
| #554 | 5 | 18 | 18 | 22 | 17 |
| the ideal | 15 | 15 | 15 | 15 | 20 |

The Prospectors' six and the collapses' six are past noise, both toward the ideal; the rest are
within it. Off Earth at the end of the batch: Phobos 118 ground Colonies (47 before) and first
settled in 70 of 80 games (28); Deimos 41 (69) and first settled in 41 games (69): its raise was
outbid by Phobos's. The Moon held about level, 223 (233).

**The tests.** Fifteen formulas tests pinned the old figures. Each expected value was worked out
again from the new data, none loosened. One premise broke: no off-world Mine made exactly an East
Asia Mine's 6.6 any more, so the even-trade test moved to Phobos with Deep Mining (9.9 against
9.9). Not reviewed by a second agent: the designer closed the ticket on the sweep.

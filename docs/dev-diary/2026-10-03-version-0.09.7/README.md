# Version 0.09.7, the tightening version: dev diary

The map is [#457](https://github.com/whaleyjoshua2/Dying-Earth/issues/457); the spec is
[`docs/spec/version-0.09.7.md`](../../spec/version-0.09.7.md). Sweeps are in [`sweeps/`](sweeps/),
each 20 seeds × four seatings at the standing cell. The baseline is 0.09.6's closing sweep:
5 / 3 / 11 / 15 wins and 46 collapses.

## Planetary Stewardship at 1.5 (ticket #458)

No picture: the change is one figure and one line of the Tech's hover text, both in data.

**Witnessed red:** the test `the_gates_general_bonuses_are_for_everyone`, moved to 1.5 before the
data, failed with "the Sink grows by 1.5: 6 then 7".

**The sweep** ([`after-458.txt`](sweeps/after-458.txt)): 6 / 2 / 11 / 15, 46 collapses. Within noise
of the baseline.

## The Custodians' Research Directive at 0.02 (ticket #459)

No picture: a figure in data and a computer-seat behaviour.

**Witnessed red:** the new test `the_computer_custodians_divert_at_the_cap_once_their_gate_stands`
failed first on the figure (`left: 0.01, right: 0.02`), then, the figure set, on the behaviour (the
log showed only "direct 10 per cent"). The first build then failed the test's Relations case: it
missed that giving everything earns a point, so 15 per cent is not free against a rival one point
from cause. The rule became three steps.

**The sweep** ([`after-459.txt`](sweeps/after-459.txt)): 10 / 3 / 13 / 13 and 40 collapses, from
6 / 2 / 11 / 15 and 46. The world goes under the Sink at least once in 17 games of 80, from 10.

## The Strip Permit's lasting Unrest at +2 (ticket #460)

No picture: a figure in data and one sentence of the Prospectors' card.

**Witnessed red:** the test `f_a_strip_permit_doubles_output_for_three_turns_then_charges_its_price`,
moved to 2 before the data, read `left: (3, 2.0, 0.2, 3.0), right: (3, 2.0, 0.2, 2.0)`.

**The sweep** ([`after-460.txt`](sweeps/after-460.txt)): 10 / 5 / 11 / 12 and 41 collapses, from
10 / 3 / 13 / 13 and 40. Within noise.

## The Archive fund is lost when its Colony changes hands (ticket #461)

No picture: the headless shot of the Archive's card does not draw its hover, which is where the one
changed sentence shows.

**Witnessed red:** the existing Archive test, renamed and asked for a fund of nought, and the new
test `the_computer_goes_after_a_half_full_archive_and_the_archivists_defend_it`, could not compile
before the two `ai.toml` figures existed; the root crate's tests then failed until the Report line's
new `{fund}` field was declared.

**The sweep** ([`after-461.txt`](sweeps/after-461.txt)): 9 / 5 / 12 / 13 and 41 collapses, within
noise. The sweep now counts Archives lost: 20 over the 80 games (1, 7, 2 and 10 by seating), with
2,952 of fund.

## Rivals' gates researched last; the Archivists stop diverting at a full fund (ticket #462)

No picture: computer-seat behaviour.

**Witnessed red:** the new test `the_computer_leaves_its_rivals_gates_until_last_and_starves_them`
could not compile while `last` held one Tech. The 0.09.1 guard test
(`no_faction_defers_a_tech_another_factions_gate_needs`) was amended to allow the gate itself.

**The sweep** ([`after-462.txt`](sweeps/after-462.txt)): 10 / 7 / 10 / 14 and 39 collapses, from
9 / 5 / 12 / 13 and 41. The tree completes in 51 games at turn 24, unchanged. The Archivists keep
back 65 to 82 per cent of their Research on average, from 100.

## The Report coloured by Faction, with 15% fewer words (ticket #463)

- [`before-report.png`](ticket-463/before-report.png) and
  [`after-report.png`](ticket-463/after-report.png): the same Report, January 2032, seed 7. After:
  each Faction's name in its colour and each held Region's in its owner's, the carbon-credit sales
  once instead of twice, and two more
  headings fit in the window.
- [`cuts.md`](ticket-463/cuts.md): every template that changed, before and after, with word counts.

**Witnessed red:** three new tests failed before the build (the ceiling at 2,031 words; the credit
line listed twice; the deck line still said). Twenty older tests quoted old wording and were moved.
Two of them caught real faults in the first draft: the Sink Moment is the designer's own sentence,
whose "{ease}" is in words ("a half") and needs its "by"; and a change of hands is listed under two
headings on purpose (ticket #404), so the single listing was narrowed to carbon credits.

The picture was retaken twice for the designer's two additions: held Regions in their owner's colour,
and each Faction's recruiting as one line (witnessed red: two lines where one was wanted).

No sweep: nothing a computer seat reads was changed.

## A compact Temperature gauge around the End Turn sun (ticket #464)

A rough ring was built first and shown to the designer (orange, open at the foot, running from
seven o'clock round to five); the designer asked for it redder and from eight to four.

- [`early-zoom.png`](ticket-464/early-zoom.png): turn 2, the ring an empty track with five dim
  notches.
- [`late-zoom.png`](ticket-464/late-zoom.png): the Temperature forced to +2.6 heading to +2.9. Red
  to the white tick near four o'clock, the fired Breaks bright, and the ring's hover above it.
- [`late-earth.png`](ticket-464/late-earth.png): the whole window at the same moment, the Climate
  Panel open: its bar in the same red.

**Witnessed red:** the test `the_temperature_ring_runs_from_eight_to_four_over_the_top` failed at
exactly four o'clock on a rounding edge, and the hit test was given a pixel of tolerance.

## Smear and Greenwash in the tutorial (ticket #465)

- [`note-report.png`](ticket-465/note-report.png): the seventh note as the player sees it, with
  "Play on" under it since it is the last.

**Witnessed red:** the test `the_tutorials_seventh_note_teaches_greenwash_and_smear` read
`left: [1, 2, 3, 4, 5, 6]` before the note was written.

## A real-time clock in the window (ticket #466)

- [`bars.png`](ticket-466/bars.png): the top bar for the Custodians, the Prospectors and the
  Archivists at 1280 wide, the clock at the right end of the second row in all three.

Three takes before it stood. Flowed into the row as a label, the clock wrapped to a third line of
its own; it is painted at the row's right edge instead. Then the two Factions with a fund on that
row had no room for it at 1280 wide, and the designer chose to shorten the fund; a quarter off was
not enough, and the rails are half what they were.

**Witnessed red:** the test `the_clock_reads_twelve_hours_and_the_sitting_reads_hours_and_minutes`
could not compile before the two functions existed. The Linux local-time code cannot be built on
this Windows host; the GitHub kit build is its first compile.

## The final report's totals (ticket #467)

- [`final-chronicle.png`](ticket-467/final-chronicle.png): the Chronicle of seed 7, the table under
  its new heading. The Prospectors made 1,549 Ducats and ended with 2; the Custodians' Fuel reads
  "8 (10)", the Stockpile above the total, since they began with Fuel they did not make.

**Witnessed red:** the test `a_seat_keeps_a_gross_total_of_what_it_has_produced` could not compile
before the counter existed, and then failed on its own premise: the tests' usual board strips every
building, so nothing was made. It uses the full starting board.

## Six more texts tightened (ticket #468)

- [`widgets-earth.png`](ticket-468/widgets-earth.png): the top bar's Widgets hover with the shared
  definition under its first line.

The other five hovers were not pictured: the Army's row sat below the card's fold in the headless
shot, and the rest need a Battle, a Blockade or a Relay on the board.

## Neutral Regions build Sea Walls (ticket #470)

No picture: a rule in the Climate phase and one Report line.

**Witnessed red:** the new test could not compile before the rule existed. It then failed twice on
its own premises, not the rule: the tests' `calm` helper marks every Sea Level threshold as already
passed, so the sea was never close; and a wall holds the slots but the coast still moves one slot
inland behind it (ticket #276), which the test had not allowed for.

**The sweep** ([`after-470.txt`](sweeps/after-470.txt)): 9 / 6 / 11 / 13 and 41 collapses, from
10 / 7 / 10 / 14 and 39: within noise. Coastal slots lost fall from a median 47 to 62 a game to 34 to
51; thresholds held by a wall rise from about 360 a seating to about 565.

## An Opening Objective for each Faction (ticket #471)

- [`journal-met-earth.png`](ticket-471/journal-met-earth.png): the Victory window on its Journal tab
  in May 2032, the Custodians' objective "Met, March 2030."

**Witnessed red:** the objectives' test could not compile before the seat kept the turn it met its
objective. The Report's word-ceiling test then failed at 1,737 words against 1,727, which is what
it is for; the two new lines were shortened and four old ones trimmed. The first test of the
computer's lean compared a Custodian Scrubber before and after and read 16 against 16: the
Custodians already give their first Scrubber the same multiplier, so the test moved to the
Prospectors' Investment Bank.

**The sweep** ([`after-471.txt`](sweeps/after-471.txt)): 8 / 21 / 18 / 5 and 28 collapses, from
9 / 6 / 11 / 13 and 41. It now prints how often and when each Faction meets its objective:
Custodians in all 80 games on turn 2; Archivists in all 80 around turns 7 to 11; Prospectors in 58,
around turns 9 to 16; Arkwrights in 47, around turns 15 to 29.

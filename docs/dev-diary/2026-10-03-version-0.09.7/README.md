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

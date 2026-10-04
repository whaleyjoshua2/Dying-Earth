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

# Dying Earth — version 0.09.7, the tightening version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.09.7](https://github.com/whaleyjoshua2/Dying-Earth/issues/457), and the
pictures and batches that decided it are in
[`docs/dev-diary/2026-10-03-version-0.09.7/`](../dev-diary/2026-10-03-version-0.09.7/).

## 1. Planetary Stewardship at 1.5

The authority is [ticket #458](https://github.com/whaleyjoshua2/Dying-Earth/issues/458).

- **Planetary Stewardship adds 1.5 ppm a turn to the Natural Sink** for the whole world while it
  stands, where it added 1.0. Its effect line says so.
- Nothing else moves: its cost (48), its prerequisites, and its place as the Custodians' gate.
- The computer seats quote no figure of their own for it, so nothing was taught.

**Measured** (80 games, the standing cell):

| | 0.09.6 | Stewardship at 1.5 |
|---|---|---|
| wins (Cust / Pros / Ark / Arch) | 5 / 3 / 11 / 15 | 6 / 2 / 11 / 15 |
| collapses | 46 | 46 |

Within noise: one win moved, no collapse. The Custodians reach the gate in 66 of 80 games, as before.

## 2. The Custodians' Research Directive at 0.02

The authority is [ticket #459](https://github.com/whaleyjoshua2/Dying-Earth/issues/459).

- **Each point of Research the Custodians divert adds 0.02 ppm to the Natural Sink for good**, where
  it added 0.01. About +1 ppm over a game at half diverted throughout.
- **The computer Custodians divert once Planetary Stewardship stands**, their Victory road being
  done. Before that they keep back what every computer seat does (nothing for a Tech they want, 10
  per cent otherwise).
- **They adjust for Relations.** Giving everything earns a point of Relations with every rival;
  diverting up to 15 per cent earns and costs nothing; more than that costs a point. They take the
  largest step that puts no rival at cause (Relations −5) who would not be there anyway:
  - 50 per cent, their cap, if it is safe;
  - 15 per cent if only that is safe;
  - nothing, if losing even the bonus point would tip a rival to cause.
  
  A rival already at cause does not hold them back.
- The human Custodian's rule is the figure alone.

**Measured** (80 games):

| | Stewardship at 1.5 (§1) | with the Directive |
|---|---|---|
| wins (Cust / Pros / Ark / Arch) | 6 / 2 / 11 / 15 | 10 / 3 / 13 / 13 |
| collapses | 46 | 40 |
| the world under the Sink at least once | 10 games | 17 games |
| the Sink at the end, median by seating | 4.0 / 4.5 / 4.8 / 5.2 | 4.0 / 7.1 / 7.2 / 9.9 |

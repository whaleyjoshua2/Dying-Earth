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

## 3. The Strip Permit's lasting Unrest at +2

The authority is [ticket #460](https://github.com/whaleyjoshua2/Dying-Earth/issues/460).

- **When a Strip Permit's three turns end, the Region's Unrest rises by 2 for good**, where it rose
  by 3. The Prospectors' card text says so. Its other prices are unchanged: Baseline Emissions +0.2,
  once per Region, ever.
- The computer Prospectors take a Permit without weighing its Unrest, so nothing was taught.

**Measured** (80 games):

| | after §2 | Strip Permit at +2 |
|---|---|---|
| wins (Cust / Pros / Ark / Arch) | 10 / 3 / 13 / 13 | 10 / 5 / 11 / 12 |
| collapses | 40 | 41 |

Within noise. The Prospectors place first on score in 36 games, from 32.

## 4. The Archive fund is lost when its Colony changes hands

The authority is [ticket #461](https://github.com/whaleyjoshua2/Dying-Earth/issues/461).

- **When a Colony holding the Archive changes hands, the Archivists' fund goes to nought**, where
  it was kept. A change of hands is an Influence takeover or a completed Occupation, as before; the
  Module is destroyed, as before. A Colony that is only occupied has not changed hands.
- **Only the Archive's Colony carries the fund.** Losing another Colony costs nothing from it, and
  a fund with no Archive built cannot be taken.
- **Uploads are kept**, as since version 0.08.0.
- **The texts:** the Report line reads "The Archive at <place> was destroyed when the Colony left
  the Archivists' hands; their fund of <n> is lost." The Archive's hover ends "Destroyed if this
  Colony changes hands, and the fund is lost with it."

**The computer seats** (figures in `ai.toml`):

- **Going after it.** A rival's Colony holding the Archive is weighed at ×2 (`archive_target_lift`)
  by a seat that has cause against its holder, once the fund is at least half its cap
  (`archive_target_fund = 0.5`). It applies to spending Influence on the Colony and to landing an
  Army there. Without cause, or under half, nothing changes.
- **Defending it.** While any rival has cause against them and the fund is at least half, the
  computer Archivists weigh a Barracks and an Army at the Archive's Colony at the threat's lift,
  and hold the Colony with Influence once a rival's Standing comes within four steps of theirs,
  where two is the rule for every other place.

**Measured** (80 games):

| | after §3 | with the fund lost |
|---|---|---|
| wins (Cust / Pros / Ark / Arch) | 10 / 5 / 11 / 12 | 9 / 5 / 12 / 13 |
| collapses | 41 | 41 |

The win column is within noise. **20 Archives were lost with their Colony** over the 80 games, and
2,952 Research of fund with them (nearly always a full 150). How many were lost before this ticket
was not counted, so there is no before figure for that.

## 5. The computer seats research their rivals' gates last; the Archivists stop diverting at a full fund

The authority is [ticket #462](https://github.com/whaleyjoshua2/Dying-Earth/issues/462). Computer
seats only; no rule moves.

- **`last` is a list** (`[tech_picks.*]` in `ai.toml`), and each Faction's is the three gates that
  are not its own. A computer Research Lead picks a rival's gate only when nothing else is left,
  and then the cheapest of them. Its own gate it still picks first, by the rule of version 0.06.0.
- **A rival's gate is starved.** A computer seat keeps back its whole cap of Research (50 per cent)
  while a gate that is not its own is under research, at the usual price of a point of Relations
  with every rival. Its own gate it funds in full. The designer: the seat that needs the gate is
  the one still paying for it, so it is likelier to hold the Lead.
- **The road is still protected.** No Faction leaves or refuses a Tech a rival must pass through to
  reach its gate; the test from version 0.09.1 holds that, with the gate itself now excepted.
- **The computer Archivists set their directive to 0 once the fund is full**, where they left it at
  100 all game, and back to 100 when the fund has room again (§4: it can be lost).

**Measured** (80 games):

| | after §4 | with §5 |
|---|---|---|
| wins (Cust / Pros / Ark / Arch) | 9 / 5 / 12 / 13 | 10 / 7 / 10 / 14 |
| collapses | 41 | 39 |
| the whole tree complete | 51 games, median turn 24 | 51 games, median turn 24 |
| gates reached (Cust / Pros / Ark / Arch) | 67 / 75 / 59 / 61 | 62 / 74 / 57 / 63 |
| Research the Archivists keep back, mean by seating | 100% in all four | 82 / 78 / 70 / 65% |

Within noise on wins. The games do not stall: the tree completes as often and as early. The
Custodians reach their gate in five fewer games.

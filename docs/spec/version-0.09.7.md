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

## 6. The Report coloured by Faction, with 15% fewer words

The authority is [ticket #463](https://github.com/whaleyjoshua2/Dying-Earth/issues/463). No rule
moves.

- **A Faction's name is drawn in that Faction's colour** wherever a Report line says it, the
  player's own included. A line that names two shows both. The rest of the line is as it was; the
  gold headline is not coloured.
- **"The" is gone before a Faction's name**, at the head of a line and inside it: "Prospectors
  issued a Strip Permit in Iran", "Iran is blockaded by Prospectors". The turn-1 lines list rivals
  the same way.
- **The words.** The Report's templates held 2,031 words and hold 1,725, 15.1 per cent fewer. A
  test holds the ceiling at 1,727. Every changed line is in
  [`ticket-463/cuts.md`](../dev-diary/2026-10-03-version-0.09.7/ticket-463/cuts.md), approved by the
  designer before it was written. No figure or rule was dropped from a line. What a rival's list of
  deeds no longer says: "from the next Income" on a Research directive, and "under way" on a
  cancelled build.
- **Not announced any more:** the off-Earth Events joining the deck on turn 12. They still join.
- **The designer's own sentence is kept:** the Moment for the first turn under the Sink.
- **A carbon credit the player bought or sold is listed once**, under Your works; it stood under On
  Earth as well. A change of hands to or from the player is still listed under both (version 0.09.4).
- **A held Region's name is drawn in its owner's colour too**, added by the designer after the build:
  "color country names the color of their owner too". The owner is whoever controls the Region as
  the Report is read; a Region nobody holds keeps the line's own colour. A name is matched as a
  whole word, so "Iran" is not coloured inside "Iranian".
- **A Faction's recruiting is one line**, the designer's last addition: the total, then each Region
  -- "Archivists recruited 4 Pioneers: 2 in The United States, 2 in Australia." Every Region had a
  line of its own, which also quoted the Unrest it calmed; that clause is gone from the line, and
  the Region's own Unrest line still carries the change. A line about one Region still goes there
  when clicked; a line about several points nowhere.

## 7. A compact Temperature gauge around the End Turn sun

The authority is [ticket #464](https://github.com/whaleyjoshua2/Dying-Earth/issues/464). No rule
moves.

- **A ring around the sun** is the Climate Panel's Temperature bar at a glance. It runs over the
  top from eight o'clock (the base Temperature) to four o'clock (Collapse).
- **What it carries:** a dark track; a red fill to the Temperature now, with a white tick at its
  head; a darker band on to the Temperature the CO2 Stock already commits the world to; and a notch
  at each Break, dim until it fires and bright on a dark backing after. No labels, and no Sea Level
  or Antarctica notches: the panel has those.
- **Its hover,** three lines: "Temperature +2.6 C, heading to +2.9", "next: Amazon Dieback at
  +2.6", "Collapse at +3.0. Click for the Climate Panel."
- **A click on the ring opens or closes the Climate Panel**, and does not end the turn. The disc
  and the words "End Turn" are End Turn's, as before.
- **The Climate Panel's bar takes the ring's red** for its fill and its committed band, so the two
  read as one gauge. Nothing else on the panel moves.

## 8. Smear and Greenwash in the tutorial

The authority is [ticket #465](https://github.com/whaleyjoshua2/Dying-Earth/issues/465). No rule
moves.

- **The tutorial has a seventh note**, on turn 7, for Blame: the Greenwash and the Smear together.
  - Headline: "Blame: Greenwash and Smear"
  - Text: "Blame is the carbon you answer for; a large share makes places dearer to take. In
    Factions (F): Greenwash, on your page, cuts yours 2 ppm per Influence and Ducat. Smear, on a
    rival's, adds 2 ppm per Influence; they resent it."
  - Quieter line, the designer's sentence: "That is the last of these notes; the game carries on
    from here. Planetary Stewardship is required for victory."
- **Turn 6's quieter line is gone.** It said the same two things at length.
- A test holds the note's two rates to `[smear]` and `[greenwash]` in `influence.toml`.
- As with every tutorial note, nothing is forced and nothing is checked.

## 9. A real-time clock in the window

The authority is [ticket #466](https://github.com/whaleyjoshua2/Dying-Earth/issues/466). No rule
moves.

- **The time of day on this machine** stands at the right end of the top bar's second row, in grey:
  twelve-hour, hours and minutes, "9:47 PM". No seconds and no date. It is kept on the second row,
  away from the in-game date on the first.
- **Its hover says how long this sitting has run:** "Playing for 1 h 20 min", counted from when the
  game was started or loaded.
- **The fund on that row is half as long.** The Prospectors' Venture Fund and the Archivists'
  Archive fund each show a slider and a fill bar there; both are 60 pixels where they were 120, so
  the fund and the clock fit together at 1280 wide. The full control elsewhere is unchanged.
- **If the row is ever full, the clock is left out** rather than wrapped to a third line.
- **Local time on Linux too.** The clock, and the Load screen's times with it, read the machine's
  own time zone there, where the Load screen read UTC.

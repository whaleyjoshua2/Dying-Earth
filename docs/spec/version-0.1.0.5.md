# Dying Earth — version 0.1.0.5, the pushback version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.1.0.5](https://github.com/whaleyjoshua2/Dying-Earth/issues/606), and the
pictures and sweeps that decided it are in
[`docs/dev-diary/2026-10-24-version-0.1.0.5/`](../dev-diary/2026-10-24-version-0.1.0.5/).

**The baseline** is 0.1.0.4's closing board,
[`final-0.1.0.4.txt`](../dev-diary/2026-10-21-version-0.1.0.4/sweeps/final-0.1.0.4.txt):
**23 / 42 / 4 / 10 and 1 collapse**, against the designer's 15 each and 20.

**What the version is.** Version 0.1.0.4 with the designer's list of 2026-10-24, most of it pushing
back: the computer seats contesting a rival's climb, defending their own Regions, weighing Collapse
and packing at 40%; the interface pushed back to its figures. (The summary and the win column are
written by the closing ticket, as each section lands.)

## 1. Every off-world population bar up by 2, and the Archive at a place of Colony tier

Ticket: [Every off-world population bar up by 2](https://github.com/whaleyjoshua2/Dying-Earth/issues/625).

The designer: *"lets raise the offworld pop requirement by 2 across the board"*; and, asked whether the
Archivists' Upload counts, *"leave at 16 they go in groups of four but they need something too"*;
offered a tier under the Archive, living people kept beside the Upload, or a Research cost rising with
the count: *"option 1 but make it a colony"*.

- **Off-world Presence is 20** (`victory.toml`, from 18): the Custodians' and the Prospectors' second
  part, one figure. The Diaspora is **44** (`factions.toml`, from 42), **the four on each of four Bodies
  kept**: it was first built at six on a misread answer and put back at the designer's word the same
  day (*"I'm pretty sure i'd asked to keep Diaspora with four on each of four bodies"*). The Upload
  **stays at 16**, in fours.
- **The Archive may be ordered only at a place of Colony tier** (`modules.toml [archive]
  tier_to_order = "Colony"`, in place of `colonists_to_order = 4`). The tier is read, not the people:
  a place never falls back a tier, so eighteen who climbed it and then left still leave a Colony;
  a Colonial Capital counts, a station never (the Archive is ground-only already). Checked once, at
  the order, as the four-Colonist rule was. The refusal reads *"the Archive wants a place of Colony
  tier; Olympus Mons on Mars is an Outpost"*. The loader refuses a tier name that is no tier's.
- **The computer's paces** move with the bars on the same turns (`ai.toml`): every people pace to
  `[[17, 7], [31, 20]]`; the Arkwrights' first x 44/42, `[[8, 6], [17, 18], [25, 32], [33, 44]]`; the
  Archivists' unmoved. The test that holds every schedule to its bar passes.
- **The computer Archivists** name only a Colony-tier place for the Archive, and their ferry's
  spending lift (the x5 on a Habitat at the Archive's Body, ticket #361) now stays on until a
  Colony-tier site stands, where it paused at the first Outpost that could hold the Archive.
  Measured with the rule alone (`sweeps/ticket-625-ferry-paused.txt`): **25 / 52 / 2 / 0 and 1**, no
  computer Archive standing in any of 80 games (43 of 80 on the baseline), the Archivists' fund full
  at 250 in every game with nowhere to spend it.
- **The texts that carried the figures**: the long `victory` sentences on the Custodians', Arkwrights'
  and Archivists' rows (20; 44 and 6; "at a place of Colony tier"), the tutorial's two notes (twenty),
  the tool's `build archive` help, the glossary (Diaspora, Off-world Presence, the Archive, the
  Archivists).
- **Swept** (`sweeps/ticket-625.txt`, 20 seeds x four seatings, the four on each Body kept): **26 / 48
  / 2 / 2 and 2 collapses**, from 23 / 42 / 4 / 10 and 1. The computer's Archive stands in **11 of 80**
  games at a median turn 35 (43 of 80 at about 30 on the baseline; 0 with the ferry paused; 13 with
  six on each Body, `sweeps/ticket-625-six-each.txt`, 25 / 50 / 2 / 3), so the Archivists win 2 from
  10; the Arkwrights, at 44, 2 from 4; the Prospectors take the games both lose, 48 from 42, six
  past noise. Colony-tier places at the end over the batch 34 (23 on the baseline), Capitals 28 (16).
  **The miss is reported and left**: the Colony tier is the designer's word, and the computer
  Archivists reach it in about one game in seven; what more the computer should do to climb, or
  whether the tier should be the Settlement's, is the designer's call.

## 2. The pack on a runaway leader forms at 40% of Victory

Ticket: [The pack on a runaway leader forms at 40% of Victory, not 50%](https://github.com/whaleyjoshua2/Dying-Earth/issues/621).

The designer: *"Drop threshold for gang-ups from 50% to 40%"*.

- **`gang_from` is 0.4** (`ai.toml [thresholds]`, from 0.5; ticket #531's pack). A rival is a seat's
  leader at 40% of its Victory or more and 20 points or more above the seat (`gang_ahead` 0.2, as it
  was), and the leader's ramp runs 0 at 40% to 1 at the end, so a leader at 80% reads two thirds
  where it read 0.6; the seat's trail over `gang_full` (0.3) is unchanged. Every hostile act on the
  leader weighs x(1 + pressure), the leader is cause from half pressure, no Accord or Trade with it,
  the pack's own Accords `gang_accept_relax` (3) x the pressure easier, as before.
- **The tests** that pinned the ramp at 0.6 and the lift at x1.6 moved to two thirds, watched red on
  the old figure first; a leader at 45% is a leader now, at 39% not.
- **Swept** (`sweeps/ticket-621.txt`, 20 seeds x four seatings): **25 / 50 / 2 / 2 and 1 collapse**,
  from the same day's 26 / 48 / 2 / 2 and 2 (§1): noise. The pack forming ten points sooner did not
  bite on the Prospectors, who lead most games from the Moon's claim: Accords struck over the batch
  620 from 620, games with an orbital Battle 24 from 27. The computer's Archive 9 of 80 (11 before),
  noise. Reported and left: the figure is the designer's word, and what the pack does with its
  earlier start is a question for the contest and defence tickets, which give it Influence to spend.

## 3. An Occupation halves the prior holder's Standing

Ticket: [An Occupation halves the prior holder's Standing](https://github.com/whaleyjoshua2/Dying-Earth/issues/615).

The designer: *"upon occupation prior holder loses half their influence"*; on the ticket's questions,
*"q1 that q2 yes q3 loss stands"*.

- **The turn an Occupation begins**, the prior holder's Standing at that place is halved, rounded down
  (`influence.toml [influence] occupation_standing_kept = 0.5`, the share kept). A neutral place has no
  prior holder and nothing halves. Their Allotment and stockpile are untouched; so are the occupier's
  banked Standing and Pacification, which reads the occupier's Standing against the bare threshold.
- **Every Occupation** halves it again, by another seat or the same one after a break: 41 to 20 to 10.
- **A broken Occupation** hands the place back and restores nothing: the loss stands.
- **The Report's occupation line** carries it where a holder lost: *"Arkwrights occupy Japan; Custodians'
  Standing there falls to 20."* (`occupation_begun_held`), read by every seat; a neutral place keeps the
  plain line. The tool prints the Report. No Moment of its own.
- **The computer** reads Standing from the table: a prior holder's retake is priced on what it has left,
  and nothing in `ai.rs` carried a copy.
- **Swept** (`sweeps/ticket-615.txt`, 20 seeds x four seatings): **26 / 49 / 2 / 2 and 1 collapse**,
  from the same day's 25 / 50 / 2 / 2 and 1 (§2): noise. Occupations begun over the batch 204 from
  197, broken 37 from 36: the computer occupies no more and no less for the halving, and the win
  column does not move. Reported and left.

## 4. One more Colony Slot on the Moon, Phobos and Deimos

Ticket: [One more Colony Slot on the Moon, and one each on Phobos and Deimos](https://github.com/whaleyjoshua2/Dying-Earth/issues/613).

The designer: *"add one additional slot for a lunar colony"*, *"add one on Deimos and Phobos as well"*;
the names as proposed, *"those three"*.

- **Three rows in `bodies.toml`**: the Moon's fifth slot **Oceanus Procellarum** (lon -57, lat 18),
  Phobos's third **Todd** (lon -153, lat -9), Deimos's second **Voltaire** (lon 3.5, lat -22). A slot
  carries no yield of its own; the Body's yields are per Colony, and the computer reads the free slots
  from the table, so the Moon stays open one founding longer and the Mars moons offer two more landings.
- **The pictures** (`ticket-613/slots-moon.png`, `slots-phobos.png`, `slots-deimos.png`): each globe
  with its new site drawn clear of the others.
- **Swept** (`sweeps/ticket-613.txt`, 20 seeds x four seatings): **29 / 51 / 0 / 0 and 0 collapses**,
  from the same day's 26 / 49 / 2 / 2 and 1 (§3). The Arkwrights' and Archivists' two each to nought
  is within noise, but their seats have now read 2, 2, 2, 0 and 0 across the version's five sweeps.
  The landings moved as the slots meant them to: ground Colonies at the end over the batch on Phobos
  165 from 127, on Deimos 57 from 40, on the Moon 290 from 270. Reported and left.

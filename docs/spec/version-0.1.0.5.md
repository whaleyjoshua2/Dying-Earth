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

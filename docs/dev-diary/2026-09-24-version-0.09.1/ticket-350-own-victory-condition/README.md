# Ticket #350: the player is told their own Victory Condition

The designer's line: *"provide correct victory conditions to the player."* The Prospector playtester
finished ten turns with 22 of 12 Colonists off Earth and a score of 0.00, because the game had told
them to get twelve Colonists off Earth and never mentioned the 2,500 Ducats.

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/350#issuecomment-5839675003)
is the authority; `SPEC.md` is the build specification written from it.

## Where the wrong line was

It was the game's own line, not the headless driver's: `start_rivals` in `report.toml`, said on turn
1 to every Faction. It was half of the Custodians' and the Prospectors' Condition and wrong for the
Arkwrights and the Archivists.

## What was built

- Each Faction has a `victory_short` clause in `factions.toml`, in the designer's approved words.
  Its figures are **filled from the Victory bars**, never typed, so a moved bar moves the line: a bar
  under ten is written as a word, and a thousand or more carries its comma.
- The loader refuses a clause whose placeholder the Faction's own Condition cannot fill.
- The turn-1 line carries it. The Victory window, the Rulebook and the tutorial are unchanged.

## The pictures

`turn-one-report-<faction>.png`, one for each Faction, each showing the turn-1 Report:

- Custodians: *"… and reach Stabilization, three Climate phases running with Emissions under the
  Natural Sink, with 12 Colonists living off Earth, before the Temperature reaches +3.0 C."*
- Prospectors: *"… and put 2,500 Ducats in the Venture Capital Fund with 12 Colonists living off
  Earth, before …"*
- Arkwrights: *"… and get 30 Colonists living off Earth, spread over three Bodies, before …"*
- Archivists: *"… and build the Archive off Earth, pay 125 Research into it, and upload 12 Colonists,
  before …"*

## The sweep did not move

Measured together with ticket #349; the byte-identical before and after files are in
`../ticket-349-pressed/`.

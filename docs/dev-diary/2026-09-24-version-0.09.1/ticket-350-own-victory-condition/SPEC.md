# Ticket #350: the player is told their own Victory Condition. The build specification

Authority: the resolution comment on
[ticket #350](https://github.com/whaleyjoshua2/Dying-Earth/issues/350#issuecomment-5839675003),
decided by the designer on 2026-09-25. Where this file and that comment disagree, the comment wins
and this file is wrong.

## Prior art, searched before writing

Read out of the tree on 2026-09-25.

- **The wrong string** is `start_rivals` in `assets/data/report.toml:20`, *"Your rivals are {rivals}.
  Build, spread Influence, and get twelve Colonists off Earth before the Temperature reaches
  +{collapse} C."*, said once on turn 1 by `engine/src/turn.rs:69` as a `LineKind::Note`, and
  declared in `LINE_ARGS` at `engine/src/report.rs:403` as `("start_rivals", &["rivals", "collapse"])`.
  It is the game's own line, so the headless driver shows it too.
- **The bars** are in `assets/data/factions.toml`: Custodians `stabilization_run` 3 and
  `off_world_presence` 12 (`:43-44`); Prospectors `venture_fund` 2500 and `off_world_presence` 12
  (`:95-96`); Arkwrights `colonists_off_earth` 30 and `colonies_on_bodies` with `bodies = 3`
  (`:151-152`); Archivists `archive_research` 125 and `colonists_uploaded` 12 (`:191-193`).
- **The long sentence** is each Faction's `victory` field (`factions.toml:42, 85, 150, 190`), read by
  the Rulebook. It is untouched.
- **The Victory window** already names an unresearched gate Tech through `first_held_back`
  (`engine/src/victory.rs:154-158`). Untouched.
- Collapse Line is 3.0 (`climate.toml:65`).

## The rules

### R1. A short Condition per Faction, in data

- New field `victory_short` on each Faction in `factions.toml`, beside `victory`, with the figures as
  placeholders filled from that Faction's own bars, never typed:

  | Faction | `victory_short` |
  |---|---|
  | Custodians | `reach Stabilization, {first} Climate phases running with Emissions under the Natural Sink, with {second} Colonists living off Earth,` |
  | Prospectors | `put {first} Ducats in the Venture Capital Fund with {second} Colonists living off Earth,` |
  | Arkwrights | `get {first} Colonists living off Earth, spread over {bodies} Bodies,` |
  | Archivists | `build the Archive off Earth, pay {first} Research into it, and upload {second} Colonists,` |

- `{first}` is `victory_first.bar`, `{second}` is `victory_second.bar`, `{bodies}` is
  `victory_second.bodies`. The Custodians' 3 is written *three* in the approved wording; render a bar
  under ten as a word (three) and 2500 as *2,500*, so the approved sentences come out exactly.
- The loader refuses a Faction without `victory_short`, and refuses a placeholder the Faction's
  Victory kinds cannot fill.

### R2. The turn-1 line uses it

- `start_rivals` becomes *"Your rivals are {rivals}. Build, spread Influence, and {condition} before
  the Temperature reaches +{collapse} C."*, with `condition` added to its `LINE_ARGS` entry.
- `turn.rs` fills `condition` from `Seat(0)`'s Faction.
- The Prospectors' line then reads exactly: *"… Build, spread Influence, and put 2,500 Ducats in the
  Venture Capital Fund with 12 Colonists living off Earth, before the Temperature reaches +3.0 C."*

### R3. Not built

- The Victory window, the Rulebook's collapsing header, the tutorial and the computer seats are
  unchanged. No save change. `CONTEXT.md` unchanged.

## Tests

Each witnessed red before it is believed green.

- **Each Faction's turn-1 line** equals the approved sentence above, exactly, for all four.
- **A moved bar moves the line**: a test table with the Prospectors' fund bar at 3000 produces
  *3,000* in the line.
- **The loader refuses** a Faction with no `victory_short`, and a `{bodies}` placeholder on a
  Faction whose second part has no bodies.
- **The sweep does not move**: 20 seeds x four seatings, byte-identical per-Faction totals.

## Pictures

Headless, `shot:` mode only, filed beside this spec. Never run the game executable bare. The turn-1
Report for each of the four Factions, and look at each before calling the build done: the line must
wrap cleanly and read as one sentence.

# Ticket #362: quiet the "have not forgiven" lines in the Report

The designer, adding it to this build: *"quiet the 'has not forgiven' spam."*

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/362#issuecomment-5842598353)
is the authority, and served as the spec.

## Measured first, and after

Eighty games, 2,577 turns, a throwaway diagnostic (not committed):

| | before | after |
|---|---|---|
| Relations lines in the player's Report, a turn | **1.5** (median 0) | **0.32** |
| most in one turn | **12** | **2** |
| rivals' quarrels between themselves | 46% of them | none |

Before, 41% of the lines marked a fall into a new level; the rest were a point's slide within one.

## What was built

- **Only a pair involving the player, only a fall into a worse named level** (Friendly, Cordial,
  Neutral, Wary, Cold, Hostile), read from the level before and after `settle_relations` -- nothing
  new in the save.
- **Both directions, one folded line each:** *"The Prospectors are now Cold toward you, the
  Arkwrights Wary."* and *"You are now Wary of the Archivists, Cold toward the Arkwrights."*
  (`relations_they`, `relations_you` in `report.toml`). `relations_fell` is retired. A spectator,
  who has no player, is told nothing.

## Tests

`the_report_tells_only_a_fall_into_a_worse_level_involving_the_player`: a rival's fall into Cold
and the player's own into Wary are told in those words; a one-point slide inside Cold and a quarrel
between two rivals are not; the old line is gone. Witnessed red on the old code. It walks each
starting score until the LEVEL a player reads is the one it means, since that level carries the
Blame term on top of the deeds -- the first draft set raw scores and one "slide within Cold" was
really a fall out of Wary. Clippy gate clean; 481 + 8 + 6 pass.

## The pictures

[`report-turn-13.png`](report-turn-13.png) and [`report-turn-17.png`](report-turn-17.png):
`shot: seed:2 turns:12|16 menus:1 panel:0 window:1920x1080`. Two Reports in play, with no Relations
line in view at the new rate of about one every three turns; the words themselves are pinned by the
test. The sweep cannot move: nothing the engine decides reads the Report.

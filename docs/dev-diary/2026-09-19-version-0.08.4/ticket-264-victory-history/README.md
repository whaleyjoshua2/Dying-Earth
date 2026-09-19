# The Victory history

Ticket [#264](https://github.com/whaleyjoshua2/Dying-Earth/issues/264) on
[map #254](https://github.com/whaleyjoshua2/Dying-Earth/issues/254).

All three pictures are `shot: factions:1 turns:20 panel:0`: twenty turns played by the computer,
the Faction window open on the player's own page.

| picture | what it shows |
|---|---|
| [`victory-history-chart.png`](victory-history-chart.png) | The chart, magnified three times. The Custodians' **progress** in their teal along the foot -- their score is 0 for the whole game, since a Stabilization run of 0 of 3 is the lower of their two parts -- and their **Blame share** in violet riding just under the faint *fair* line at a quarter. Along the date axis, January 2030 to May 2033: two Breaks in red, Antarctica's opening in blue, and in white the turn Planetary Stewardship, their gate Tech, was done. |
| [`faction-window-800.png`](faction-window-800.png) | The window at 1280x800, the chart under the two progress bars at the population chart's size. The Rulebook header is still on screen: nothing went below the fold. |
| [`faction-window-1080.png`](faction-window-1080.png) | The same at 1920x1080, where the game opens. |

## What was decided, in the designer's words

*"one line"* -- the score, the lower of the two parts' fractions, the figure the window's heading
prints. *"the share"* -- Blame as a share of the table's, on its own scale at the right with the
fair quarter marked, so the four Factions' charts compare. *"yes"* -- Antarctica ticked on every
chart, the gate Tech on the Faction's own, the Archive on the Archivists', the Breaks in red as
every sibling has them. *"sounds good"* -- written with the Emissions record, after the Climate
phase, so the two charts share an axis. *"every factions"*. *"lets try that first"* -- the
population chart's size, 90 pixels tall, and the pictures say it fits.

## What it keeps

The first per-Faction history the game has: `VictoryRecord { turn, score, blame_share, gate_done,
archive_complete, antarctica_open }` on every seat, one per Climate phase, saved with the game;
a save from before this version loads with an empty chart that grows from there. Both lines are
drawn on the whole 0-to-1 range rather than the data's own, so a height means the same thing on
turn 3 as on turn 30 and the same on every Faction's page.

## Witnessed red

The test -- every seat has one record after one phase, on the Emissions record's turn, with the
window's score and the rules' share -- was written first and run with the record and the field in
place but nothing writing it: *"Seat(0): one record after one phase, left: 0, right: 1"*. Then the
write went in beside the Emissions record; `321 passed`, clippy clean with `-D warnings`.

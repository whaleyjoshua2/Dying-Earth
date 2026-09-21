# Teach the sim and the sweep to count the war

Ticket [#286](https://github.com/whaleyjoshua2/Dying-Earth/issues/286) on
[map #275](https://github.com/whaleyjoshua2/Dying-Earth/issues/275).

No picture: nothing on the screen changed. The artifact is the batch.

| file | what is in it |
|---|---|
| [`sim-20-custodians-first.txt`](sim-20-custodians-first.txt) | Twenty seeds, Custodians in seat 0, each seed's line now ending in *battles by seat [..] (n vs neutrals), armies built [..] lost [..], force takes [..]*. |

## What the batch says, now that it is counted

Over the twenty seeds:

- The Prospectors opened every Battle but one (the Archivists opened one, in seed 13). None was against a neutral Region's own Army, which is the same finding ticket #282 made by another road: no neutral is left by the time an Army exists.
- The Prospectors built two to four Armies a game; the Archivists built two in three seeds; the Custodians and Arkwrights built none. The Prospectors lost an Army in four seeds.
- The Prospectors took a place by force in fourteen seeds, twice in three of them and three times in one.

The sweep prints the same figures once over the batch, as two lines directly under *Places taken by Influence*, so a take by force sits beside a take by Influence.

## Counters, not scrapers

Every figure is a counter on the game, incremented where the event happens: the Battle where it is fought, the Occupation where it begins or breaks, the loss where the unit is removed. The old `influence_transfers` count, scraped from the log's sentences, is left as it was. The counters ride through the save.

## Tested

One test, witnessed red three times, each on its own assertion: ground Battles left uncounted (*one Battle, opened by seat 0*: `0 != 1`), takes by force left uncounted (*taken by force, once*: `0 != 1`), and a Standing Army's loss counted as an ordinary one (`(0, 1) != (1, 0)`). Restored, `344 passed`, `6 passed`, clippy clean with `-D warnings`.

# A held Colony wore the wrong colour: seat 0 was always Custodian teal

Ticket [#255](https://github.com/whaleyjoshua2/Dying-Earth/issues/255) on
[map #254](https://github.com/whaleyjoshua2/Dying-Earth/issues/254). The designer's report: *"in
recent playthrough as prospectors my Tycho station was custodian turquoise."*

Every picture here is `shot: player:prospectors archive:1 look:-134,18` -- a Prospector in seat 0
with a Colony at Olympus Mons on Mars, the camera facing it.

| picture | what it shows |
|---|---|
| [`before-olympus-mons.png`](before-olympus-mons.png) | **Before.** The label reads *Olympus Mons: Prospectors* in the Prospectors' orange; the 3D marker beside it is **teal**. Centre pixel (52, 150, 140), the Custodians' card colour. |
| [`after-olympus-mons.png`](after-olympus-mons.png) | **After.** The same picture, the marker **orange**: (189, 118, 57), the Prospectors'. |
| [`marker-pair.png`](marker-pair.png) | The two markers side by side, magnified three times. |
| [`after-solar-mars-zoom.png`](after-solar-mars-zoom.png) | The Solar System Map after, Mars magnified five times: the **Orbital Control ring orange** (the Prospectors hold it) and the four Ship stack markers each in their own Faction's colour at their own angle -- orange, teal, purple, pink. Both were on the same wrong path. |

## The cause

`setup_scene` runs at `Startup` (`src/main.rs:130`), before any game exists, and built the four
marker materials from `session.colours()` -- which with no game answers in `FactionKind::ALL` order:
Custodians, Prospectors, Arkwrights, Archivists. `sync_scene` then looked those materials up **by
seat** (`handles.flat[s.index()]`, `handles.ring_materials[s.index()]`, and the stack markers at
spawn). Seat 0 always got material 0, which is always teal.

It went unseen for eleven versions because the player is seat 0, and when the player is the
Custodians -- the default, the tutorial seat, and every `shot:` capture that does not say
`player:` -- seat 0 *is* material 0 and the picture is right. The egui labels were always right,
reading the colours live with a game present, which is why a label said Prospectors beside a teal
ball.

## The fix

One file. The materials stay built in Faction order; every lookup is now
`handles.flat[game.kind(seat).index()]`. The stack markers, which were given their material at spawn
with no game to ask, are spawned grey and given their Faction's material in `sync_scene` like the
slot markers and the ring.

No test can see a material; the witness is the picture, and the before picture was already red.
`314 passed`, clippy clean with `-D warnings`.

# Ticket #356: the Pioneer recruiting buttons move above the building tiles

The designer's line: *"move up the buttons to recruit pioneers to above the building tiles."*

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/356#issuecomment-5840945517)
is the authority, and served as the spec.

## What was built

- **`pioneers_block`** in `src/ui.rs`: the whole Pioneers block, lifted out of the Orders block
  unchanged -- *Pioneers waiting*, **Recruit N Pioneers**, and every conditional door under it (sea
  to Antarctica, lift to a station, send to a Colony Ship) -- with the **Launch Site line** at its
  foot. It is drawn between the Widgets and Embassy lines and the **Facilities** heading, with a
  divider above and below, on a Region the player directs.
- Nothing else moved. Orders and Unrest stand where they stood. No rule, figure or save changed;
  clippy gate clean, 469 + 8 + 6 tests pass.

## The pictures

All `shot: seed:2 select:eastasia panel:0 window:1920x1080`. Seed 2 because seed 1 and the default
seed drew a Choice Card on turn 1, which covers the globe and greys every order until it is answered.

| picture | what it shows |
|---|---|
| [`china-before.png`](china-before.png) | China as the Custodians, before. The card ends at the fold on *Pioneers waiting: 0*; **Recruit 4 Pioneers** is below it, out of sight without scrolling. |
| [`china-after.png`](china-after.png) | The same card after. **Pioneers**, *Pioneers waiting: 0*, **Recruit 4 Pioneers** and the Launch Site line sit under *Widgets 11 a turn*, above *Facilities (4 of 9 slots free)*. |
| [`china-as-the-arkwrights.png`](china-as-the-arkwrights.png) | China as the Arkwrights, `player:arkwrights`, after: *Pioneers waiting: 2* above **Recruit 8 Pioneers**, and the Launch Site line reading their Spaceport as a Launch Site, as it always has. |

**What it costs**: the block is about a hundred rows tall, so everything under it moves down by that
much. At 1080 the Orders block now shows its heading and first line above the fold, where before it
showed through Build Army.

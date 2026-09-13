# Version 0.07.4, the reactions version: the pictures

The map is [Map: version 0.07.4](https://github.com/whaleyjoshua2/Dying-Earth/issues/149). Every
picture here was taken headlessly in `shot:` mode on `version-0.07.4`; nothing was opened on the
designer's desktop.

## The building hovers return

Ticket [#150](https://github.com/whaleyjoshua2/Dying-Earth/issues/150). The designer's line: *"mouse
over information on buildings didn't xfer to icon grid."* The Facility row's hover had survived
0.07.3, but only on the row a click puts in the strip; the boxes and the Hab View's tiles had no
hover at all, and the Module rows never had one. Built as the ticket's recommendation (b) for the
designer to react to: every box and tile says on hover what its row says, and the free, building
and flooded states say what they are.

![A standing box: the Launch Site's figures and the three rules](hover-launch-site-box.png)

![A flooded box: what took it, and what a Sea Wall does](hover-flooded-box.png)

![A free box: click it to build](hover-free-box.png)

![A Habitat tile in the Hab View: its figures and the Module rules](hover-habitat-tile.png)

![A free tile: room for another Module, and the cap rule](hover-free-tile.png)

The Facility row's figures and rules became two helpers the box hover shares with the row, so the
two can never drift apart; the Hab View's strip line became a helper its tiles share the same way.
A building box now knows the turn it is ready (the old rows said so; the boxes had lost it). The
`tip:<word>` aid photographed every hover, with `select:EastAsia walls:1` for China's card and
`hab:1 turns:12` for a Hab View with a Habitat on it.

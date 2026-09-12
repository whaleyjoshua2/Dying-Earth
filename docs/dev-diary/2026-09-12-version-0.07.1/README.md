# Version 0.07.1, the reading-and-reaching version

The map is [Map: version 0.07.1](https://github.com/whaleyjoshua2/Dying-Earth/issues/111).

## Antarctica showed on the start screen

![Without the fix, three markers on the ice; with it, none](antarctica-on-the-start-screen.png)

Reported by the designer against version 0.07.0: the Antarctic sites still showed while
choosing a starting continent. [Nothing of Antarctica is drawn until the ice
opens](https://github.com/whaleyjoshua2/Dying-Earth/issues/103) hid them in a game, and the
start screen is **not** a game: `sync_scene` returns early when `session.game` is `None`, and
that return sits **before** the loop that hides the markers, so they kept the visibility they
were spawned with. Before a game exists the ice is shut by definition, so they are hidden
there too.

It took two attempts to photograph, which is the interesting part. The start globe now opens
aimed at the Faction's home with a latitude tilt, so Antarctica is off-view and the first two
captures showed nothing **either way** — the fix looked confirmed by a picture that proved
nothing. The designer would have seen the markers only after dragging the globe south, which
[the globe ticket](https://github.com/whaleyjoshua2/Dying-Earth/issues/100) made possible in
the same version. Standing the camera where a dragging player stands is what produced the
picture above: three markers without the fix, none with it.

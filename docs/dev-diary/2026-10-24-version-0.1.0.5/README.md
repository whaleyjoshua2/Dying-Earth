# Version 0.1.0.5, the pushback version

The map is [Map: version 0.1.0.5](https://github.com/whaleyjoshua2/Dying-Earth/issues/606); the spec
is [`docs/spec/version-0.1.0.5.md`](../../spec/version-0.1.0.5.md). The standing baseline is 0.1.0.4's
closing board, **23 / 42 / 4 / 10 and 1 collapse**
([`final-0.1.0.4.txt`](../2026-10-21-version-0.1.0.4/sweeps/final-0.1.0.4.txt)), against the designer's
15 each and 20.

## Every off-world population bar up by 2, and the Archive at a place of Colony tier

Ticket [#625](https://github.com/whaleyjoshua2/Dying-Earth/issues/625), spec §1.

Three bars moved in the data (Off-world Presence 20, the Diaspora 44 over four Bodies of 6) and the
Upload stayed at 16, at the designer's word; in its place the Archive asks a place of Colony tier,
`[archive] tier_to_order = "Colony"` in place of `colonists_to_order = 4`. Three tests were written
first and watched red: the bars (left 18, right 20), the order's refusal (an `Ok` where the Outpost of
eighteen should be refused), and the computer naming only a Colony-tier place -- whose first half
had passed for the wrong reason until the Outpost's slots were filled, so the Archive was the one
thing left to order there. Then the twenty-two tests that pinned 18, 42 and four moved with them. No
picture: figures and a refusal line, nothing on the screen changes shape.

- **The first sweep** ([`sweeps/ticket-625-ferry-paused.txt`](sweeps/ticket-625-ferry-paused.txt)),
  the rule alone: **25 / 52 / 2 / 0 and 1**. No computer Archive stood in any of 80 games (43 of 80
  on the baseline): the Archivists' ferry pauses its spending lift the moment a place that may hold
  the Archive exists (ticket #361), and under the tier an Outpost of four is such a place but not a
  site, so nobody was carried there to climb it. Their fund sat full at 250 in every game.
- **The fix**: a site is a place that may hold the Archive AND stands at the Colony tier; the lift
  stays on until one does. A fourth test, red first (the Habitat at the Outpost weighed as at a
  Colony), then green.
- **The sweep** ([`sweeps/ticket-625.txt`](sweeps/ticket-625.txt)): **25 / 50 / 2 / 3 and 0
  collapses**. The computer's Archive stands in 13 of 80 at a median turn 35 (43 at about 30 before);
  the Archivists win 3 from 10, the Arkwrights 2 from 4, and the Prospectors take what both lose, 50
  from 42, eight past noise. Colony-tier places at the end 37 from 23, Capitals 24 from 16. **Reported
  and left**: the tier is the designer's word, and what more the computer should do to climb -- or
  whether the tier should be the Settlement's -- is theirs to say.

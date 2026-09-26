# Ticket #363: the orbital Battle and the Launch still miss the bar

Carried on from ticket #355, which brought the Blockade to life (28 games of 80) and left orbital
Battles at 1 game and Launches at 0, against the designer's bar of 10 each.

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/363#issuecomment-5841989989)
is the authority, and served as the spec.

## The ticket's own two levers were wrong, and measured so

- **A lower orbit-only Attack bar**, 0.5 and 0.4: the sweep was byte-identical to 0.6.
- **Missile Technology ahead of the non-gate Techs**: byte-identical. The tree is shared and the
  Tech is researched in 62 of 80 games anyway, around turns 20 to 30.

## What was actually in the way, found by tracing

1. **No carrier was wanted enough to build.** After the Tech a seat had a target and a yard in 561
   seat-turns and never queued a Missile Carrier.
2. **An armed carrier ready to fire left on the same turn.** A Ship takes one order a turn, but the
   engine enforced it one way only: a Launch refused a move given before it, while a Change orbit,
   Transit or Refuel given AFTER a Launch, Rearm or Bombard was accepted. The computer ordered the
   Launch and then a refuel at its own station's ring; the move resolved first, and the Launch found
   its carrier in the wrong orbit. **An engine defect**, fixed on both sides.
3. **The low-orbit garrison left to refuel on the same turn,** giving up the Orbital Control the
   Launch needed.
4. **A blockaded seat's Battery voided the Blockade without a shot**, and two armed sides otherwise
   rarely shared an orbit.

## What was built

- **Rule (the designer's word): a working Battery opens a Battle on a rival warship on Blockade in
  its own orbit**, its holder the side that opens it.
- **Engine: one order a turn, both ways.** A Change orbit, Transit or Refuel is refused after a
  Launch, Rearm or Bombard, as the reverse already was.
- **The computer**: a wanted Missile Carrier takes the threat's lift; an armed carrier with a target
  in reach of its orbit does not refuel that turn; a warship holding the low-orbit lane does not
  leave it to refuel while its tank can still pay for a Battle; and a seat with an armed carrier and a
  ground target wants the ground, so it keeps the garrison.
- **The Battery's hover was cut to fit six lines.** With the generic Energy and Mothball sentences it
  rendered at thirteen lines, from #324 and #335; it now reads the figures, then *"Covers {orbit}
  alone, and fights in any Battle there. / While it works, {what it denies}. / Repaired here with
  Materials; destroyed at 6 hits."* ([`battery-hover-cut.png`](battery-hover-cut.png), five lines).
- Glossary **Battery**, `modules.toml`'s note, and the driver's help say the new rule.

## The bar

[`sweep-before.txt`](sweep-before.txt) is ticket #355's after; [`sweep-after.txt`](sweep-after.txt).

| | before | after | bar |
|---|---|---|---|
| games with a **Launch** | 0 | **17** (32 Launches) | **met** |
| games with an **orbital Battle** | 1 | **8** (15 Battles, 1 off Earth) | **not met**, two short |
| games with a Blockade | 28 | 27 | met |
| Missile Carriers built | 0 | 42 | |
| wins C / P / Ar / Ar | 3 / 38 / 3 / 3 | 3 / 36 / 3 / 3 | |
| collapses | 33 | 35 | |

The Launches burned 128 buildings, killed about 5.7 billion people over the batch and cost five
Industry Levels. The Custodians' Batteries opened 8 of the 15 orbital Battles under the new rule.
Two collapses more is a figure, and it is the nuke's: every Launch on Earth fouls the air.

## Tests

- `a_battery_fires_on_a_blockader_in_its_orbit`: witnessed red before the rule; a warship on Hold
  and a mothballed Battery start nothing.
- `an_order_after_a_launch_rearm_or_bombard_is_refused`: each of the three moves is lawful alone and
  refused after a Launch or Rearm; witnessed red on the old engine.
- Clippy gate clean; 477 + 8 + 6 pass.

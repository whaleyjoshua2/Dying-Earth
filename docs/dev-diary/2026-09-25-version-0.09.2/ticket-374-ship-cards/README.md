# Ticket #374: a card for every Ship, with Bodies and orbits in drop-downs

The designer: *"clean up ships at body cards - ships should each have their own card. bodies and
orbits about which should each be their own drop down both on their own cards and the one listing
all the ships at a body."* Decided in one round of six: everything that is one Ship's goes on its
card; the stack card keeps only what is the whole stack's; drop-downs as recommended; the Ship's
card in place of the stack's with a link back; a Ship in flight has a card too; the picture aids
scroll the Ship's card.

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/374) is the authority;
[§11 of the spec](../../../spec/version-0.09.2.md#11-a-card-for-every-ship-with-bodies-and-orbits-in-drop-downs)
records it.

## What was built

A new selection, `Selection::Ship`, and a new card, `ship_panel`, in `src/ui.rs`. The stack card
(`stack_panel`) was re-laid around it: its Ships stand grouped by orbit in one drop-down per orbit,
open by default, each row a link that opens the Ship's card; under them the stance, Attack, and the
whole-stack moves. The moves are one helper for both cards, `move_dropdowns`: one drop-down per
Body, the Body the Ships are at first and open (its orbits are the change-of-orbit moves), every
other Body shut with its turns and Fuel on the header and the dated quote of #375 inside. With one
Ship the lines carry that Ship's own button and the stranding warning; with more, "All N that can".

The Roster's Ship rows open the Ship's card, in flight or not; the flying label on the Solar System
Map is a hotspot that opens it; an Army aboard a Ship jumps to the Ship's card. The two right-click
doors (a Body on the Solar System Map, an orbit on a Body Surface Map) read the selection through
one helper, `selected_ships`, so they send the one Ship whose card is open as they send the stack.

Two building aids: `ship:1` / `ship:<n>` / `ship:flying` opens a Ship's card in place of the
stack's, and `flying:1` plants a Colony Ship in flight from Earth to Mars, since nothing else in
shot mode leaves a Ship between Bodies for a picture.

No engine change, no save change, no rule change; the suites are 496 + 6 and 8, unchanged.

## The pictures

All taken headlessly at 1400x1000 with `panel:0`, from this folder.

| picture | aids | what it shows |
|---|---|---|
| [`stack-mars.png`](stack-mars.png) | `orbits:1 stack:1` | **The stack card.** Two drop-downs, *Low orbit (1)* and *At Mars Base Camp (1)*, open, each holding its Ship as a link. Then the stance, the odds, Attack, and *Moves for the whole stack*: *Mars (here)* open with four "All N that can" lines, and five Bodies shut with their turns and Fuel on the header. No button per Ship anywhere on it. |
| [`ship-mars.png`](ship-mars.png) | `orbits:1 stack:1 ship:1` | **The Frigate's card.** *Back to all Custodians Ships at Mars*, its line, *In low orbit, on Hold*, its Tank with the Refuel button, and Transit: *Mars (here)* open with a Move button per other orbit, the rest shut. |
| [`settler-mars.png`](settler-mars.png) | `orbits:1 stack:1 ship:2` | **The Colony Ship's card**, at the station's ring: the same shape, with *Load and unload* under Transit offering the station's two Colonists. |
| [`flying-solar.png`](flying-solar.png) | `flying:1 ship:flying` | **A Ship in flight.** *Back to the roster*, *carrying 6 Colonists*, *In transit from Earth to Mars, 3 turn(s) left: it lands at the Resolution of May 2030, barring a solar storm*, and its Tank reading *it lands with what is left*. No moves. |

The first picture of the Frigate's card drew a hollow box where the back link's arrow should be:
the interface font has no U+2190, the same want ticket #371's arrow found. The link is in words.

## The gate

`cargo clippy --workspace --release --all-targets -- -D warnings` clean; engine 496 + 6, root 8.

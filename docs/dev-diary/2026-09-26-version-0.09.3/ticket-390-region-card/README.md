# Ticket #390: the Region card rearranged

Three items of the designer's, on the one card: Armies in their own section, Orders renamed
Policies between the Pioneers and the Facilities, and the Sea Wall and the Scrubber said shorter
and placed between the tiles and the build list. Decided in one round of four with two
clarifications (*"q1 a q2 a q3 a"*, *"q4 a"*, and *"all army/ship builds should be in orders
sections for both"*), which put Build Army in an Orders block at the foot of both cards rather than
in the Armies section.
[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/390) is the authority,
[§8 of the spec](../../../spec/version-0.09.3.md#8-the-region-card-rearranged) records it.

## What was built

`state_panel` reordered: a `policies_block` drawn under the Pioneers, the foot's Orders block left
with Build Army; the strip under the slot boxes split out of `slot_boxes` into `slot_strip` so the
completed slotless rows (`no_slot_rows`) draw between the boxes and the strip, and the two slotless
build buttons (`no_slot_buttons`) after it, the Scrubber's cap in its label; a completed Scrubber or
Sea Wall row reads one line from `no_slot_figures`, its whole sentence on the hover; the Scrubbers'
note under Influence gone. The Colony card is untouched.

## The pictures

**The card of the player's home Region on turn 1**, `seed:7 select:eastasia slotbox:free panel:0
window:1400x1700`: Influence, Pioneers, Policies, Widgets, the Facilities with a free box clicked
and the build strip under the boxes, the Armies, and Orders with Build Army at the foot.

![The rearranged Region card](region-card.png)

**The same card with a Sea Wall standing**, the `walls:1` aid (two coastal slots taken, the wall
holding): the completed row between the boxes and the strip, one line -- *Sea Wall: holds the sea
off; 1 rise held, 0.5 Materials a turn to keep* -- with its Mothball and Decommission beside it,
then the strip and the Scrubber's capped button under it. The `scrub:` aid placed no Scrubber on
this board, so the Scrubber's one-line row is witnessed by the same code path and not pictured.

![The card with a Sea Wall standing](region-card-walls.png)

## No rule moved

The suite is unchanged in meaning; no sweep.

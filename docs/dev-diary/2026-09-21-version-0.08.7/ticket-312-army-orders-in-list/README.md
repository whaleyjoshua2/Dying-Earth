# The Army orders moved into the Armies list, a tenth larger

Ticket [#312](https://github.com/whaleyjoshua2/Dying-Earth/issues/312) on
[map #304](https://github.com/whaleyjoshua2/Dying-Earth/issues/304).

| picture | what it shows |
|---|---|
| [`china-armies-block-1080.png`](china-armies-block-1080.png) | `shot: select:eastasia army:1 panel:0 window:1280x1080`. China's card at 1080, the height at which the presentation review found the Army orders below the fold: the **Armies** block under the Facilities grid holds the **Stance** row directly under its heading, the two Army rows, and under the 2nd Chinese Army its four **attack** buttons, indented; the Orders section follows. The whole block a tenth larger than the card's other text. The *Army orders* block that stood at the foot is gone. |
| [`china-armies-block-hover.png`](china-armies-block-hover.png) | `shot: select:eastasia army:1 panel:0 window:1280x1080 tip:chance to win the first exchange`. The same with the hover on *attack Russia*, from ticket #309, working from the buttons' new place. |

No batch was run: an interface change; the engine is untouched.

## What was decided, in the designer's words

*"q1 go with taht q2 yes q3 whole block q4 yes"*: the stance row under the heading and each Army's
buttons under its row; the Colony card the same; the whole block a tenth larger; *Stance:* stays.

## Settled by the builder, to be corrected if wrong

- **`ARMY_LIST_SCALE = 1.1`**, applied to the block's text styles inside a scope, as the command
  cluster applies its constant; the block's buttons and labels had no size of their own to multiply.
- **The buttons are indented sixteen pixels** under their Army's row; the caption line *the 2nd
  Chinese Army (strength 4):* that introduced them is gone, since the row above says the same.
- **The Colony card** gains an *Armies* heading it never had, drawn only when an Army is there, with
  the stance row and the repairs under the player's rows; its old block after the Orders section is
  gone. Not photographed: no shot aid puts an Army at a Colony.
- The **Army** glossary entry says where an Army's orders are given.

## Looked at, not tested

The two pictures above, each opened and read before this was committed.

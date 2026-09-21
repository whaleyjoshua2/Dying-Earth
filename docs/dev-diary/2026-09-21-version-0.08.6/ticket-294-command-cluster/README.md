# The command cluster: a slider for the spend, Max kept, a tenth larger, and End Turn a sphere on the right

Ticket [#294](https://github.com/whaleyjoshua2/Dying-Earth/issues/294) on
[map #289](https://github.com/whaleyjoshua2/Dying-Earth/issues/289).

| picture | what it shows |
|---|---|
| [`cluster-china-selected.png`](cluster-china-selected.png) | `shot: select:eastasia panel:0`, 1280x800. The cluster with China selected: the Allotment line, the **rail** from 0 to 15 with the knob at 5, **Spend 5 on China**, then the last row with **Max** and **every turn** on the left and the **sun** at the right edge with *End Turn* beneath it. Taken after the designer's second word on the first picture -- *"soften the shading and make it 10% larger"* -- so the sun is 46 wide at the cluster's scale with its rings closer in colour. The card above it, at the new scale, ends at its Facilities grid on an 800-pixel window; the strip took what it gained. |
| [`cluster-nothing-selected.png`](cluster-nothing-selected.png) | `shot: panel:0`, 1280x800. Nothing selected: the rail still drawn, the Spend line reading *Click a Region or a Colony to spend on it*, Max and the tick greyed, the sun lit. |
| [`cluster-pick-owed-embers.png`](cluster-pick-owed-embers.png) | `shot: select:eastasia pick:0 panel:0`. A Tech pick owed: the sun dimmed to embers and its word greyed, the bar carrying **Pick a Tech**. |
| [`cluster-max-standing.png`](cluster-max-standing.png) | `shot: select:eastasia attend:1 panel:0`. Max standing on China: the whole rail greyed, the knob at 0, *Spend 0 on China* disabled, *every turn on China* ticked. |

No batch was run: an interface change; the engine is untouched.

## What was decided, in the designer's words

- *"q1 yes"* -- the spend on **the Smear's rail**.
- *"q2 yes it also moves the slider to max"* -- **Max** places the order and moves the slider to the
  bound.
- *"q3 1.265"* -- the scale constant.
- *"q4 yes make it resemble the sun with sun spots and what not put the words below the disk"* --
  **End Turn is a sun**, the words beneath, *(Enter)* on the hover.
- *"q5 sounds good"* -- Max and the every-turn tick **share the sun's row**.

## Settled by the builder, to be corrected if wrong

- **The sun is drawn by hand with the painter**, as the roster's order ring is: a faint glow, a
  darker limb, four discs each smaller and brighter drifting toward a light above and to the left,
  five fixed sunspots low on the disc, and the word beneath. Gold and orange rather than the
  cluster's red, since it is the sun; it brightens a little on hover and dims to browns while End
  Turn cannot be pressed. Diameter 42 at the cluster's scale in the first cut; **46.2, with the
  rings closer in colour and drifting less**, after the designer saw it: *"can we soften the
  shading and make it 10% larger."*
- The rail is drawn whether or not a place is selected, so the strip never changes shape; the Spend
  button beneath it names the amount and the place.
- The spectator's End Turn on the top bar stays a rectangle; a spectator has no cluster.
- The **Command Cluster** and **End Turn** glossary entries say so.

## Looked at, not tested

An interface change; `346 passed`, `6 passed`, clippy clean with `-D warnings`. Four pictures, each
looked at before filing, two of them the strip's extremes (nothing selected; nothing left).

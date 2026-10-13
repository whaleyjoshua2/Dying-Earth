# Dying Earth — version 0.1.0.1

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.1.0.1](https://github.com/whaleyjoshua2/Dying-Earth/issues/529), and the
pictures and batches that decided it are in
[`docs/dev-diary/2026-10-12-version-0.1.0.1/`](../dev-diary/2026-10-12-version-0.1.0.1/).

**The baseline** is 0.1.0.0's closing board after its playtest fixes,
[`final-0.1.0.0-after-playtest.txt`](../dev-diary/2026-10-07-version-0.1.0.0/sweeps/final-0.1.0.0-after-playtest.txt):
**2 / 24 / 15 / 17 and 22 collapses**, against the designer's 15 each and 20.

## 1. Two panels' prose onto hovers, and Armies to low orbit without a Launch Site

Ticket: [Two panels' prose onto hovers, and Armies lift to low orbit without a Launch Site](https://github.com/whaleyjoshua2/Dying-Earth/issues/532).

### The Blame lines

- The Climate Panel's line for each Faction reads **"Prospectors: answerable for 412 ppm"**. The
  seat's own Blame line, on its Faction panel, reads **"Answerable for 412 ppm"**.
- The rest is that line's hover, at most three lines, each part shown only when it is not zero:
  1. *Emitted 520, removed 108 in credit*, either part left out at zero, the line absent when
     both are (*Removed 108 in credit* when nothing was emitted).
  2. the moved parts, in this order: *30 laid on by rivals*, *15 bought as carbon credits*,
     *10 sold as carbon credits*, *12 cleaned by campaign*, joined by commas. This line is
     absent when all four are zero. When in full it would wrap past the hover's width (55
     characters), it is said short, *30 by rivals, 15 bought, 10 sold, 12 cleaned*, so the hover
     stays at three lines as drawn. (Found in review: with all four moved, the full line wrapped
     to a fourth.)
  3. *Share 0.31, thresholds x1.12* (the seat's own hover has no share, as its line had none, and
     reads *Thresholds x1.12*).
- The Blame heading keeps its hover.

### The Launch Site line

- In the Nation panel, **"Launch Site: Yes"** in green when the Region has a working Launch Site,
  **"Launch Site: No"** in yellow otherwise. For an Arkwrights player the word is **"Spaceport"**.
- Hover on Yes: *Pioneers and Armies lift to any orbit of Earth.*
- Hover on No: *Pioneers and Armies lift to low orbit only.* When the Region holds a Launch Site
  (or Spaceport) that is not working, a second line: *Its Launch Site is dark.* (*Its Spaceport is
  dark.*)
- The Shipyard sentence is gone.

### The people's emissions

- The sentence *"Its people emit … per hundred million …"* is no longer drawn. Its figures are the
  hover of the *"Emissions this turn: …"* line: *People: 0.42 per hundred million (0.20 base +
  0.07 x Industry Level 3), Leapfrogged once*, the Leapfrog clause as before.

### A rule: an Army lifts to low orbit without a Launch Site

- The Pioneers' rule (#443), for Armies. A Ship that carries an Army takes one aboard from a Region
  its seat directs **in low orbit over Earth with no Launch Site**. Any other orbit of Earth — a
  station's ring — still needs a working Launch Site in the Region; the refusal reads *"a lift
  above low orbit needs a working Launch Site there"*.
- An Army of the seat's standing in a Region it does **not** direct still needs a working Launch
  Site there: *"a lift from a Region not yours needs a working Launch Site there"*. The Pioneers'
  rule is for Regions the seat directs, and this keeps it so.
- Unchanged: an Army landing from a Ship; a Region's own Army staying home; a dug-in Army; an Army
  at a Colony, reached from the orbit that touches it.
- The computer seats choose the Army a Carrier loads by this same check, so they lift from any
  Region they direct into low orbit.
- **Found on the way, fixed at the designer's word:** no computer seat had ever loaded an Army onto
  an empty Carrier at Earth, at least since #319. The load sat inside the warship block, which a
  Carrier with no Army aboard never enters. It stands outside it now, so a computer Carrier takes
  an Army for a rival Colony as #43 and #319 meant it to.
- **Refutation:** a Carrier in low orbit over Earth refused an Army waiting in a Region its seat
  directs with no Launch Site, or one in a station's ring accepting it, or a computer seat with a
  Carrier, a free Army and a target never loading it, shows this section unbuilt.

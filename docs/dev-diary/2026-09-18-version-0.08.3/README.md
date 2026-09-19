# 2026-09-18: version 0.08.3, the asymmetry version

The dev diary for version 0.08.3, charted as wayfinder map
[#230](https://github.com/whaleyjoshua2/Dying-Earth/issues/230). Unlike version 0.08.2's, this
folder is a record of the **build**: one folder per ticket, written as each was resolved, holding
the pictures and the measurements that decided it.

**Every folder here is listed.** Version 0.08.2's README listed two of its three and the third went
unfindable, which is the defect this index exists to avoid.

| folder | ticket | what is in it |
|---|---|---|
| [`sweeps/`](sweeps/) | [Write the 0.08.3 amendments and build them](https://github.com/whaleyjoshua2/Dying-Earth/issues/241) | `final-0.08.3.txt`: the closing sweep, 20 seeds × four seatings at the shipped climate cell, with the one aggregated win column of 80 this version taught the sweep to print. |
| [`ticket-231-tech-costs/`](ticket-231-tech-costs/README.md) | [The Tech Tree gets dearer](https://github.com/whaleyjoshua2/Dying-Earth/issues/231) | What a 5.6% and then a 17% rise in the tree's price is worth in turns, and the four tests that pinned the old figures. |
| [`ticket-232-two-new-techs/`](ticket-232-two-new-techs/README.md) | [Two new Techs](https://github.com/whaleyjoshua2/Dying-Earth/issues/232) | Three sweeps, a wrong turn, and a test that proved nothing. The discovery that **no Tech had ever changed what the computer builds**, and what fixing it did. |
| [`ticket-233-hovers/`](ticket-233-hovers/README.md) | [Two explanations go to hovers](https://github.com/whaleyjoshua2/Dying-Earth/issues/233) | The Faction window before and after, about a fifth shorter. Carries a correction: its claim that `shot:` mode cannot open a tooltip was false. |
| [`ticket-234-pioneer/`](ticket-234-pioneer/README.md) | [Emigrant becomes Colonist](https://github.com/whaleyjoshua2/Dying-Earth/issues/234) | The Region card under the new nouns, and why the distinction had to survive the rename. |
| [`ticket-235-directive/`](ticket-235-directive/README.md) | [Research each Faction may spend its own way](https://github.com/whaleyjoshua2/Dying-Earth/issues/235) | The measurement that changed two of the three conversion rates, and the slider that replaced the recommended switch. |
| [`ticket-236-shared-pot/`](ticket-236-shared-pot/README.md) | [The shared pot](https://github.com/whaleyjoshua2/Dying-Earth/issues/236) | The census that said the rule could not work as written, and the designer's answer that fixed it **without touching the rule**. |
| [`ticket-237-exodus-call/`](ticket-237-exodus-call/README.md) | [The Exodus Call](https://github.com/whaleyjoshua2/Dying-Earth/issues/237) | Why the order suspends Coach Class's double charge, and the Ducat income that makes it unaffordable where the Arkwrights usually start. |
| [`ticket-238-three-turns/`](ticket-238-three-turns/README.md) | [Three turns held](https://github.com/whaleyjoshua2/Dying-Earth/issues/238) | A refused Strip Permit and the same button live one turn later — **the first refusal ever photographed**, and how the aid was made to reach one. |
| [`ticket-239-unique-modules/`](ticket-239-unique-modules/README.md) | [Three more Unique Modules](https://github.com/whaleyjoshua2/Dying-Earth/issues/239) | The census that moved one of the three, the icon candidate sheet at build-tile size, and a build-list bug a picture found that no test could. |
| [`ticket-240-ducat-hoard/`](ticket-240-ducat-hoard/README.md) | [The Prospectors hoard Ducats](https://github.com/whaleyjoshua2/Dying-Earth/issues/240) | The measurement that showed the ticket's own premise was false, and how the 2000 bar was derived rather than converted. |
| [`ticket-242-shot-aid/`](ticket-242-shot-aid/README.md) | [The Tech Tree scrolls](https://github.com/whaleyjoshua2/Dying-Earth/issues/243) | The first picture ever taken of the Tech Tree, and the regression it found: two Victory gates off the bottom of the screen with no scrollbar. |
| [`ticket-244-coach-class/`](ticket-244-coach-class/README.md) | [Steerage becomes Coach Class](https://github.com/whaleyjoshua2/Dying-Earth/issues/244) | The Faction cards under the new name, and why **Stowaway was argued down rather than adopted**. |
| [`ticket-245-upload/`](ticket-245-upload/README.md) | [The line that was not an edge](https://github.com/whaleyjoshua2/Dying-Earth/issues/245) | A line that looked like an edge into Generation Ships and was an edge into The Upload passing underneath it, and the four Victory gate depths it exposed. |
| [`ticket-246-upload-clc/`](ticket-246-upload-clc/README.md) | [The Upload waits on Closed-Loop Colonies](https://github.com/whaleyjoshua2/Dying-Earth/issues/246) | The Archivists' gate going 2 → 1 → 4 in one version, and a new edge drawn with the same defect the previous ticket removed. |
| [`ticket-247-bands/`](ticket-247-bands/README.md) | [The Tech Tree's bands reordered](https://github.com/whaleyjoshua2/Dying-Earth/issues/247) | The first band order, and the tree at two box sizes. |
| [`ticket-248-bands-again/`](ticket-248-bands-again/README.md) | [Off-world Living to the top](https://github.com/whaleyjoshua2/Dying-Earth/issues/248) | The second band order. |
| [`ticket-249-bands-final/`](ticket-249-bands-final/README.md) | [The band order settled](https://github.com/whaleyjoshua2/Dying-Earth/issues/249) | The third and settled band order. |
| [`ticket-250-columns/`](ticket-250-columns/README.md) | [One column for every rung-3 Tech](https://github.com/whaleyjoshua2/Dying-Earth/issues/250) | Every rung-3 Tech brought into one column, and the bands in their final order. |

Everything here was captured headlessly in `shot:` mode or rendered by
`cargo run --release --example icon_sheet`. **Nothing was opened on the designer's desktop.**

## What the pictures changed, which is the argument for taking them

Four times in this version a picture found something reading the code had not:

- **The Tech Tree's layout** ([#243](https://github.com/whaleyjoshua2/Dying-Earth/issues/243)). The
  first capture of the tree showed the whole Society branch, two Victory gates among it, off the
  bottom of the screen at 1280×800 with no scrollbar. Tickets #231 and #232 had verified the tree by
  reading the code, having been told wrongly that it could not be photographed.
- **A build list missing two rows**
  ([#239](https://github.com/whaleyjoshua2/Dying-Earth/issues/239)). The Prospectors had no Trade
  Post row and no Exchange; the Archivists no Solar Array and no Heliostat. A rule written out twice
  as a list of kinds had thrown both Uniques away, and it had survived the Academy only because
  someone hand-added it to one of the two copies.
- **A Victory Condition contradicting its own progress bar**
  ([#240](https://github.com/whaleyjoshua2/Dying-Earth/issues/240)). The panel read *"1000 Materials
  in the Venture Capital Fund"* above a bar already counting **154 of 2000 Ducats**. A code search
  had missed the line.
- **An icon that was already taken**
  ([#239](https://github.com/whaleyjoshua2/Dying-Earth/issues/239)). `strongbox`, proposed for the
  Exchange, is the Prospectors' Investment Bank — their two Uniques would have worn one picture.
  Rendering the candidates beside their siblings at 16 and 22 pixels is what caught it.

## And twice, a test that proved nothing

Both were caught by **breaking the rule on purpose and watching the test stay green**, which is the
only way this class of defect is ever found:

- The Antarctic **Beneficiation** test on [#232](https://github.com/whaleyjoshua2/Dying-Earth/issues/232)
  asserted `after >= before` and passed with the effect deleted.
- The **Unique Module clause** test on [#239](https://github.com/whaleyjoshua2/Dying-Earth/issues/239)
  read its expected figures from the same table the rules read, and passed with all three clauses
  zeroed in the data.

Both were rewritten with **literal figures**. The Heliostat's turned out to discriminate a rule the
prose could not: at Mars a Solar Array makes 3 and a Heliostat 4, where adding the point *before*
the inverse square scaling gives (6 + 1) × 0.43 = 3 — the same as no clause at all.

# Dying Earth — version 0.09.4, the Custodians' version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.09.4](https://github.com/whaleyjoshua2/Dying-Earth/issues/403), and the
pictures and batches that decided it are in
[`docs/dev-diary/2026-09-27-version-0.09.4/`](../dev-diary/2026-09-27-version-0.09.4/).

**What the version is.** Version 0.09.3 with the designer's list. The summary and the win column
are written when the version closes.

## 1. The defects the headless driver shows

The authority is [ticket #404](https://github.com/whaleyjoshua2/Dying-Earth/issues/404). Each item
is a thing the game said untruly or said badly, most of them in the headless driver; one rule moves
with them (item 1, the tribute).

1. **A confirm names everything the order pays.** The Stockpile's part first, then the Widgets a
   build takes from its place's rate, the people an Army takes, and the Fuel a move draws from a
   Ship's tank, the last two joined by *and*: *"costs 25 Materials, 4 Widgets and 1M people"* for
   an Army in a Region, *"… and one Colonist"* at a Colony, *"costs 20 Materials and 4 Widgets"*
   for a Factory. A build bought outright in Ducats starts done, so it names no Widgets. **A
   Launch costs the Warhead**; **a tribute costs its Materials or Ducats** (*"costs 15
   Materials"*); **a muster of emigrants costs its people** (*"costs 4M people"* for four
   Pioneers). The driver's confirm and the window's list of the turn's orders both say it. The
   driver's price list gives the Army's Widgets and people beside its Materials, as the Ships line
   gives theirs.

   **The one rule that moves**: a tribute is now paid from the Stockpile with the order's cost,
   where it was taken at the commit outside it. So the running total counts it, and a tribute
   beside other spending is refused when the Stockpile cannot cover both, where before it could
   overdraw. The computer seats pay no tribute, so the sweep is unmoved.
2. **One phrasing for a refused turn**: *"The turn did NOT end:"*, whichever of the three reasons
   stopped it.
3. **A seat prints as a number.** An Army row reads *seat 0*, or *seat -* for nobody's, where it
   read *Some(0)* and *None*; a Ship row's Army reads *army 2* or *army -* the same way.
4. **The Market every turn.** `show` prints the Market line under the Stockpile, and a price a
   Choice Card has set names the card and the turns it has left: *"Materials 1 (Cheap Ore Offer,
   1 more turn)"*, then *"(Cheap Ore Offer, its last turn)"*. A price moves at most a step a turn
   inside its band; a card is what swings it from 4 to 1. A save from before names *a card*.
5. **A move that spends the tank says so**: a transit *"costs 6 Fuel from the tank"*, an orbit
   change its orbit-change Fuel likewise, where both said *free*. A Launch and a Bombard draw no
   Fuel (the ticket's first comment said they did; the code says not); a Bombard says *free*.
6. **A place changing hands to or from the player is also listed under Your works**, as well as
   under its place: taken by force, by Influence or Pacified, won or lost, and a throw-off of the
   player; and **an Occupation begun or broken** by the player or on the player's place, which
   changes who directs it though not who controls it, including an Occupation of the player's
   ended by a third Faction's Influence. The headline's ranking is unchanged. A
   change of hands or an Occupation between two rivals is not listed there. The driver's Report
   prints under the window's headings, so it shows the same.

**What would show this wrong**: a Build Army confirm without its Widgets or people; a transit
confirm reading *free*; a Market line with no card named on the turns a Cheap Ore Offer's price
stands; a player's lost Region, or a player's Occupation begun, broken or taken by a third Faction,
missing from Your works; a Launch confirm reading *free*; a tribute missing from the running total.

# Dying Earth — version 0.09.6, the growth version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.09.6](https://github.com/whaleyjoshua2/Dying-Earth/issues/435), and the
pictures and batches that decided it are in
[`docs/dev-diary/2026-10-03-version-0.09.6/`](../dev-diary/2026-10-03-version-0.09.6/).

## 1. The crash on a Colony Ship at a Venus station

The authority is [ticket #436](https://github.com/whaleyjoshua2/Dying-Earth/issues/436). No rule
moves.

**Before:** a Colony Ship's card offered "Unload N into <place>" for each of your places at its Body
that had room. It named each place by its **ground** slot, and an Army landing did the same. For a
station the slot number counts stations, so:

- every station over **Venus**, which has no ground slots, crashed the game when the card opened;
- Earth's 4th and 5th stations would have crashed it too;
- every other station was named for a ground site on its Body.

**After:** a station is named for its station slot and a ground Colony for its ground slot, on both
buttons. The designer's save (turn 33, a Colony Ship at Aphrodite over Venus) opens its card and
offers "Unload 4 Colonists into Aphrodite".

Every other ground-slot lookup in the window was checked; each one is on a path that only ever
names a ground slot.

**What would show this wrong:** a Ship card that panics at any station, or a button that names a
ground site for a station.

## 2. Moving Colonists from a Colony Ship onto a station

The authority is [ticket #437](https://github.com/whaleyjoshua2/Dying-Earth/issues/437).

**Unchanged rules, now seen working:**

- **Onto a station.** A Colony Ship unloads into one of your stations from that station's own
  orbit, and changing orbit still costs a turn and 1 Fuel. In any other orbit the Unload button is
  greyed, and its hover says where to move: "Move this Ship to Venus, at Aphrodite, then unload next
  turn". The crash (§1) had hidden this.
- **Back off a station.** Colonists may go back from any place of yours, station or ground, onto a
  Colony Ship in that place's orbit.

**Changed:**

- **Over Earth, the Ship card lists your stations too.** Under the Pioneers drop-down, each station
  of yours over Earth with Colonists aboard offers "Load N Colonists from <station>" on a slider.
  Until now the card at Earth offered only the Regions' Pioneers, so a Colonist lifted to the ISS
  could not leave it. Antarctic Colonies are not listed, as decided: stations only.
- **Unloading brings Education.** Colonists unloaded onto a place that already stands blend their
  Education into the place's, as every other arrival does. Before, they were added to the count
  alone.

**What would show this wrong:**

- a station over Earth with Colonists that a Colony Ship in its ring cannot load from;
- a greyed Unload button with no orbit named;
- four Colonists at Education 2.0 unloaded onto four at 1.0 leaving anything but 1.5.

## 3. Generation Ships needs Relay Networks

The authority is [ticket #438](https://github.com/whaleyjoshua2/Dying-Earth/issues/438).

- **Generation Ships needs Closed-Loop Colonies and Relay Networks**, where it needed Closed-Loop
  Colonies alone. Both lines are drawn: one into the box's upper quarter, one into its lower.
- **Relay Networks stays at 32.** The Arkwrights' road is Expanded Habitats 18, Closed-Loop
  Colonies 32, Relay Networks 32 and Generation Ships 48: **130**, where it was 98. **All four
  Victory roads now cost 130.**
- **The computer Arkwrights** research Relay Networks straight after Closed-Loop Colonies, before
  the Propulsion line.

**What would show this wrong:**

- Generation Ships available with Relay Networks unresearched;
- a Victory road at any price but 130;
- the Arkwrights' pick list reaching Propulsion before Relay Networks.

## 4. The Archive fund at 150

The authority is [ticket #439](https://github.com/whaleyjoshua2/Dying-Earth/issues/439).

- **The Archive costs 150 Research**, where it cost 125. That figure is also the Archivists'
  first Victory part. Both stored figures moved (`modules.toml`, `factions.toml`), along with the
  Archivists' card text and the glossary. The turn-one line and the Victory window read the figure,
  so they follow it.
- **The computer Archivists' pace** runs to 150 on the same turns: 38 by turn 10, 75 by turn 18,
  113 by turn 26, 150 by turn 32. It had stayed at 80 since version 0.05.5.
- **Measured before** (balance suggestions, Archive at 150 alone): 12 / 9 / 1 / 15 with 42
  collapses, against 0.09.5's 10 / 10 / 1 / 17. The version's sweep reads it with the other figures.

**What would show this wrong:** an Archive that completes, or a first Victory part that fills, at
any figure but 150; or a computer Archivist pace that ends anywhere else.

## 5. The Custodians' Influence at ×1.15

The authority is [ticket #440](https://github.com/whaleyjoshua2/Dying-Earth/issues/440).

- **The Custodians' Influence multiplier is 1.15**, where it was 1.2. The card shows "x1.15" and
  its hover "Influence Allotment x1.15", both read from the data.
- **Rounding is unchanged:** the Allotment is multiplied, then rounded down to a whole point, as
  every Faction's is. From China at the start, (10 + 3) × 1.15 = 14.95 gives **14** a turn, where
  ×1.2 gave 15.
- **The computer Custodians** read the Allotment, not the figure, so nothing of theirs moves.

**What would show this wrong:** a Custodian Allotment of 15 from China alone on turn 1, or a card
that reads x1.2.

## 6. A Shipyard wants one Factory, not one every turn

The authority is [ticket #441](https://github.com/whaleyjoshua2/Dying-Earth/issues/441). No rule
moves; this is how the computer seats spend.

- **Before:** once a Colony had a Shipyard, every computer seat wanted another Factory Module there
  each turn "because a Ship is wanted". Nothing counted the Factories already standing, and the want
  carried the full victory-gap boost. A yard Colony filled its free slots with Factories. The
  Arkwrights' one station over Earth spent its Materials on them and never launched a Colony Ship.
- **After:** a yard Colony wants **one** Factory on the yard's account, counting those standing and
  on order. A Colony whose build queue is deep still wants one on its own account, as before.
- **All four computer seats.**
- **The designer's ideal**, given on this ticket: about **15 / 15 / 15 / 15 wins and 20 collapses**
  over the 80-game sweep. The trial (balance suggestions, Ar1) moved collapses from 42 to 32, toward
  it.

**What would show this wrong:** a computer yard Colony with two Factory Modules and a short queue,
or one with none while it builds Ships.

## 7. Colony Ships hold 35 Fuel, and the build price matches the tank

The authority is [ticket #455](https://github.com/whaleyjoshua2/Dying-Earth/issues/455), added by the
designer after charting.

- **The Colony Ship's base tank is 35**, where it was 40. Clean Propellant's +5 and Cryogenic
  Tanks' +15 are unchanged: 40 with the first, 55 with both. Warships stay at 30.
- **A Ship is built with the tank it was paid for.** The Fuel is paid at the order, as before, and
  the build now records it.
  - The Ship comes out holding exactly that.
  - Room a tank Tech added while it was building comes empty, as it does for a Ship already flying.
  - Before, the Ship was filled to the tank on the turn it finished, so a Tech completing mid-build
    gave up to 15 Fuel nobody paid for.
  - A build in a save older than the record fills as it always did.
- **Cancelling a Ship build refunds its tank's Fuel** with the Materials. Before, the Fuel was lost.
- **The build button's hover** ends "Built full: N Fuel."
- **The computer seats buy the Fuel ahead of every Ship build**, warships included, as they did for
  a Colony Ship alone. Where they cannot pay this turn, the build waits like any other want.

**What would show this wrong:**

- a new Colony Ship holding anything but 35 before Clean Propellant;
- a Ship holding more Fuel than its order paid;
- a cancel that leaves the Stockpile short of the tank;
- a computer warship build refused for Fuel it never planned to buy.

# Dying Earth — version 0.1.0.1, the counterweight version

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

## 2. A Ship's lifts from Regions and its refuel do not use its turn

Ticket: [A Ship's lifts from Regions and its refuel do not use its turn](https://github.com/whaleyjoshua2/Dying-Earth/issues/533).

- **A Ship's turn is its one action:** a flight to another Body, a change of orbit, an unload or a
  founding, a load off a Colony or station, a Bombard, a Launch or a Rearm.
- **Two things stand beside it and never use it:**
  - **a lift from a Region**, Pioneers or an Army, from any number of Regions, one load a Region;
  - **a refuel**, once a turn, beside any order.
- **Both happen where the Ship sits when the orders are given, before it moves.** Lift at Earth and
  leave for the Moon in one turn, the Pioneers aboard; refuel at the ISS and leave, the leg paid
  from the filled tank. A flight or a change of orbit is checked against the tank as the refuel
  given beside it will fill it. However the orders were given, a Ship's lifts and refuel are
  carried out before its move; a Ship that leaves for another Body takes its lifts aboard at End
  Turn.
- **Which Regions lift is unchanged:** from low orbit any Region the seat directs; from any other
  orbit only a Region with a working Launch Site.
- **A Ship never lifts and unloads in one turn:** *"a Ship loads or unloads in a turn, not both"*.
  A second refuel: *"this Ship already refuels this turn"*.
- **Unchanged:** one change of orbit a turn, and it is the turn; Fuel paid as before; taking
  Colonists off a Colony or station is the turn's action (a lift may stand beside it, the room
  counted over both); a Ship arrives at Resolution and acts the turn after.
- **The computer seats** offer a lift that makes the four a founding wants together with the
  crossing (lift and leave), and a ready Missile Carrier refuels beside its Launch.
- **Refutation:** a Colony Ship given a lift and a flight that leaves without the Pioneers aboard,
  or a refuel and a flight refused for a tank the refuel fills, or a lift accepted beside an unload,
  shows this section unbuilt.

## 3. One Launch Site to a Region

Ticket: [One Launch Site to a Region](https://github.com/whaleyjoshua2/Dying-Earth/issues/534).

- **One to a Region, by the Constabulary's rule:** a Launch Site standing, under construction or
  ordered this turn closes the door, whether ordered for Materials or bought with Ducats. The
  Arkwrights' Spaceport counts as one, so a Region holds a Launch Site or a Spaceport, never both;
  an Arkwrights player who takes a Region holding a rival's Launch Site adds no Spaceport there.
- **The refusal** names the one there: *"this Nation State already has a Launch Site"* (or *"… a
  Spaceport"*).
- **A Region already holding two keeps both**; nothing is torn down.
- **The computer** reads the same check, and no longer offers one where one is under construction.
- **The Arkwrights' free Launch Site** (their Opening Objective's reward) goes only to a Region with
  none standing or under construction; with no such Region it stays owed.
- **Refutation:** a second Launch Site or Spaceport accepted in a Region that holds, builds or has
  ordered one shows this section unbuilt.

## 4. Greenwash costs Relations

Ticket: [Greenwash costs Relations](https://github.com/whaleyjoshua2/Dying-Earth/issues/535).

- **Scaled by what is spent:** for every 10 Influence (`greenwash.resentment_per`) a Faction spends on
  Greenwash, each other Faction's Relations toward it fall by that Faction's `greenwash_resentment`:
  the Custodians 0.4, the Arkwrights 0.2, the Archivists 0.1, the Prospectors 0.05.
- **It builds and never heals:** each Faction keeps the Influence it has spent on Greenwash over the
  game; the shown score reads a term from it, as it reads the Blame term, rounded toward zero (half
  a point shows as none). The quiet turns that heal deeds do not touch it. No ceiling of its own; a
  Faction never resents its own campaign.
- **The player sees it:** the Greenwash button's hover adds *"Relations: Custodians -0.2, Arkwrights
  -0.1, Archivists -0.05"* for the amount set (0 where nothing is spent); the Relations hover in the
  Factions window names *"Greenwash -2"*, and now *"Research shared +1"*, the #236 term it never
  named, so the parts add up to the whole. Both hovers go through `rule_tip`.
- **The computer** weighs the cost: a Greenwash's weight is divided by (1 + `greenwash_relations_cost`
  (5) x the points it would lose with Factions the seat holds an Accord with or stands Cordial or
  better with).
- The hover's "Research shared" is the reward as the score adds it, after its #236 ceiling.
- **Refutation:** a campaign of 50 Influence that leaves the Custodians' Relations toward its
  campaigner unchanged, or a Greenwash term that recovers over quiet turns, shows this section
  unbuilt.

## 5. The Colony Ship card shows aboard and room

Ticket: [The Colony Ship card shows aboard and room](https://github.com/whaleyjoshua2/Dying-Earth/issues/536).

- **Every Colony Ship, empty or not,** shows the population glyph then *aboard/room* -- *[glyph] 3/4*
  -- beside its line: on its own card's head, on its row on the stack card, and on its roster row
  (docked or in flight). The words "carrying 3 Colonists" and ", 3 Colonists aboard" are gone.
- **The room** is the Ship's capacity (the Colony Ship's 4, Expanded Habitats, Generation Ships, the
  Faction's multiplier). **At Earth, when the warming world crowds the lift,** it is the crowded room
  it can take there, with the hover *"4 room, +4 crowded at Earth"*.
- Other Ships show none; a Carrier's line still says *"carrying an Army"*.
- No rule moves.

## 6. The prices

Ticket: [The prices: Influence at 4, Strip Permits at 25, Mines and builds](https://github.com/whaleyjoshua2/Dying-Earth/issues/537).

- **Influence costs 4 Ducats a point** (`[ducats] per_influence`), where it cost 2, for every seat;
  still no cap. The Influence bar's hover reads the price from the table, where it said 2 in words.
- **A Strip Permit costs 25 Ducats** (`[strip_permit] ducats`), where it was free: charged with the
  order, refused short of it, shown on its button. The Prospectors' card says so.
- **No hidden multiplier: the figures themselves move.**
  - **Builds +10% in Materials:** every Facility, Module, Ship and Army row, a station's founding (44)
    and the tiers (33, 55, 77), kept **exact, to the tenth** (25 is 27.5), so the rows carry tenths.
    A Faction's discount reaches the new row as before (a Prospector Factory is 18.7). A Ducat build
    follows from its Materials; a Decommission's half-refund follows from its row.
  - **Not** the Widgets, the Industry Level, a Repair or a Rearm; not the starting Stockpile.
  - **An Earth Mine** (the Region Facility) makes **4.4**, where it made 4.
  - **Every Body's Mine yield is 85% of what it was**, so every Colony Mine makes 15% less, Techs on
    top. Antarctica's Mines are Colony Mines, off-world for the purpose (Earth's yield 1.4875).
- **The computer** reads every figure from the tables: its cheapest-builds reckoning reads the new
  rows, floored at the cheapest paid one (where it was the literal 20); its Strip Permit and Influence buys are priced by the order and refused where short. A
  consequence of the figures, not a new rule: the Custodians idle an Earth Mine only for an off-world
  Mine that makes as much, and a Moon Mine (5.6) no longer matches China's (6.6); Vesta's (6.8) does.
- **Refutation:** a Strip Permit issued with 24 Ducats, a Habitat priced 25, or a Moon Mine making
  6.6 shows this section unbuilt.

## 7. A popup when an Accord ends, and a warning the turn before

Ticket: [A popup when an Accord ends, and a warning the turn before](https://github.com/whaleyjoshua2/Dying-Earth/issues/538).

- **A Moment, Accord Ended**, for every ending the player is party to, the cause in its words:
  *"Your Accord with Prospectors has ended: its five turns are up."* / *"... Prospectors would not
  renew."* / *"... Prospectors called it off."* / *"... Prospectors broke it."* / *"... you broke
  it."* None where the player called it off themself. Its figure is the partner's name, in its colour.
- **Always shown**, beside the two Moments a turn may otherwise stop for.
- **The warning:** at the head of an Accord's last turn, where the partner would renew, a Your works
  line, *"Your Accord with Prospectors ends after this turn."* (the designer's "ends next turn",
  worded for the turn it is read in); clicked, it opens the Factions window, where Renew stands.
  Where the partner will not renew, its own line stands as before ("let your Accord lapse").
- **Two rivals' Accord** ending is a clause of a rival's paragraph, under the fog: *"broke their
  Accord with ..."* or *"let their Accord with ... lapse"*. (One called off already has its clause;
  between two computer seats both willing, an Accord renews and never just runs out.) No Moment.
- **The text tool** prints the Moments the window stops for (`MOMENT: ...`), the warning among the
  Report's lines, and takes `accord renew <faction>`.
- No rule moves.
- **Refutation:** an Accord of the player's that ends with no Moment, or a last turn with no warning
  where the partner would renew, shows this section unbuilt.

## 8. The Report: a window next turn, your Regions lost to the sea, the clock and your Victory

Ticket: [Playtest information: the Report's launch windows, Energy left, your losses, the clock and Victory at its head](https://github.com/whaleyjoshua2/Dying-Earth/issues/544).

- **A window next turn:** the turn before a far world's window, a line under Ships, *"The Mars, Phobos
  and Deimos window is next turn: 5 turns, 18.1 Fuel."*, the flight at the window as the Solar System
  Map's hover gives it; every far world, its moons named with it. The window's own turn says nothing
  more. It is the board's own news, so the fog does not hide it; the first Report says one on turn 2.
- **Energy left: dropped** at the designer's word.
- **A Region of the player's lost to the sea** reads under Your works, there alone (#523's rule for
  the player's news).
- **The Report's head:** under the date, *"Last turn to act: 41 · Your Victory: 62%"*; in place of
  the clock, the Climate Panel's "Cuts alone no longer avoid Collapse" or "On this path Collapse is
  not reached"; the Victory the Victory window's overall share. A spectator's has the clock alone.
- **The text tool** prints the head line first under the Report's title, and the new lines with
  the rest.
- No rule moves.
- **Refutation:** a turn before Mars's window whose Report does not say so, or a Report with no
  clock under its date, shows this section unbuilt.

## 9. Before the order: the founding rule and the Pioneers' words

Ticket: [Playtest information: the founding rule, Pioneers without a Launch Site, the Influence preview](https://github.com/whaleyjoshua2/Dying-Earth/issues/543).

- **The founding rule:** a Colony Ship's aboard figure (§5) is drawn yellow at one to three aboard,
  its hover leading with *"4 found a Colony"*; an empty Ship and one of four or more stay plain.
- **A station's refusal from a Region** reads *"needs 4 Pioneers here"*; from a Colony it stays
  "needs 4 Colonists to spare".
- **The Recruit hover** is its two figures, *"2M people, Unrest -0.25."*, at the designer's word
  after the build ("too damn wordy"): the Launch Site line (§1) says where they lift to, and the
  sea's own button says Antarctica.
- **No Influence preview:** built, then withdrawn at the designer's word after its picture.
- The Trading window's "Buy more Influence" hover reads the price from the table (it still said two).
- **The text tool:** a Colony Ship's Colonists read aboard/room with "(4 found a Colony)" at one to
  three.
- No rule moves.

## 10. A rival spending on yours, and your home at risk

Ticket: [Playtest information: a rival spending on your Region, your home at risk](https://github.com/whaleyjoshua2/Dying-Earth/issues/542).

- **A rival's spend on a place the player holds** (Region, Colony or station), summed for the turn,
  is a Your works line, *"Prospectors spent 12 on China: 60 of 69."*, the rival's Standing after it
  against what takes it, as the bar stands after the turn's decay. The fog does not hide it. None
  where the place changed hands that turn.
- **Home at risk**, a second Your works line, not a Moment: *"China could fall to Prospectors next
  turn."*, where the rival's Standing plus what it spent there this turn reaches the bar, on the
  player's home (the Region they started in, recorded from this version; a save from before has
  none) or their only working Launch Site's Region.
- The Pressed list stands. The text tool prints both lines with the Report.
- No rule moves.

## 11. The text tool: offer Terms, every Standing, Leapfrog

Ticket: [Playtest information: offer Terms, rivals' Standing, Leapfrog](https://github.com/whaleyjoshua2/Dying-Earth/issues/541).

The window had all three; the text tool, which the playtest used, gains them. The window is unchanged.

- **Every offer waiting** prints what it holds before the line that answers it: *"Prospectors offer
  an Accord: non-aggression."*, *"... offer to add to your Accord: ..."*, or a Trade's *"offer 20
  Materials for 30 Fuel."*
- **Every Region and Colony** listed prints *"Standings: Custodians 50, Prospectors 12"* (or
  *"nobody has any yet"*), as the window's Standings row shows them.
- **`help`** says what a Leapfrog does: its price, the drop per hundred million, the floor, and the
  Baseline cut. A renewal offer reads as the window's does.
- No rule moves.

## 12. The Arkwrights' Victory text on stations over Earth

Ticket: [The Arkwrights' Victory text on stations over Earth](https://github.com/whaleyjoshua2/Dying-Earth/issues/540).

- The text follows the engine. The Diaspora's clause reads *"(Antarctica does not count; Earth is
  not one of the four)"*, where it said "Antarctica and stations over Earth do not count": a
  station over Earth counts toward the 42; Antarctica does not; Earth, its stations and Antarctica
  alike, is never one of the four Bodies.
- No rule moves.

## 13. The computer races for the Research Lead

Ticket: [The computer races for the Research Lead](https://github.com/whaleyjoshua2/Dying-Earth/issues/530).

- **The race:** a computer seat whose gate is not yet researched races for the Lead from the turn its
  gate is open to research, or its first part stands at `gate_pick_fraction` (half way), until the
  gate is done. It runs the harder the nearer its Victory, at its Victory share (the Victory
  window's: the lower of its two parts):
  - Research Labs, Observatories, and the Schools and Institutes weighed with Labs (#185), weigh
    x(1 + `race_lab_weight` x share), `race_lab_weight` 1.0;
  - what it sends to the Natural Sink for a Tech it is indifferent to falls by the same share.
- **Its own gate**, offered on its shortlist as Lead, is taken whenever offered; the half-way and
  turn-33 wait is gone, and `gate_pick_turn` with it.
- **Denying rivals** is unchanged: a rival's gate stays last, and the whole cap is kept back while one
  is under research.
- No rule moves; the shortlist stands.

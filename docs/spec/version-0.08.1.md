# Dying Earth — version 0.08.1, the introductions version: a window for every Faction, a name and a glyph on every Ship, a tutorial that starts at the start, and an Archive that must leave Earth orbit

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.08.1](https://github.com/whaleyjoshua2/Dying-Earth/issues/202), and the
pictures and sweeps that decided it are in
[`docs/dev-diary/2026-09-14-version-0.08.1/`](../dev-diary/2026-09-14-version-0.08.1/).

**What the version is.** Mostly it is about *knowing who*. **A Faction window** brings the Faction
card in-game and gives Relations and income a home; **every Ship is named** when it is built and
carries its Faction's prefix; **the Faction's symbol** leaves the setup screen and marks the places
and ships that belong to somebody. The **tutorial** stops spending two of its five turns asking for
nothing. A **station and an Antarctic Colony can ask for people** rather than only be sent them.

And three changes press on **off-Earth life**: a **Habitat holds four** again, the **Archive may no
longer stand in Earth orbit**, and the **Tech Tree** gains an eighteenth Tech with a dearer rung 1.
The first of those turned out to do nothing and the second to do almost everything; both are
measured below rather than asserted.

**One line from the designer's list was dropped**, not refused: Clean Propellant taking 0.8 off a
Refinery's Emissions ([#206](https://github.com/whaleyjoshua2/Dying-Earth/issues/206)).

---

## 1. The Faction window

*Ticket [#203](https://github.com/whaleyjoshua2/Dying-Earth/issues/203).*

A window of its own, opened by a **`Factions (F)`** button on the top bar and by the **F** key, with
a **dropdown in its top right** listing the four Factions and defaulting to the player's own. The
Faction's **symbol stands at its head at 64 pixels** in the Faction's colour.

It holds **both halves**, because neither existed in one place before:

- The **live figures**, in order: Victory progress with both bars, **income last turn**, Blame,
  **Relations as two rows** — what this Faction thinks of the others, and what the others think of
  it — and Holdings.
- Below them, behind a **collapsing header shut by default**, the **rulebook**: blurb, multipliers,
  Unique Facility, signature rule, Victory Condition and gate Tech. It shares its code with the
  setup screen's Faction card, so the two cannot drift.

**The disclosure rule.** Your own seat shows its income totals *and* the building-by-building
breakdown on the hover. **Any other seat shows totals only**: the breakdown names individual
buildings in individual Regions, which is a targeting list, where the total is only the rate of a
hoard the Victory window already prints to the unit. A **spectator** gets the breakdown on every
seat and no disclosure line at all, having no side to keep secrets from.

**Relations and Blame both leave the Victory window**, which keeps the climate projection and the
four-way progress bars. The 4x4 Relations grid is retired. Blame now stands in two windows on
purpose: the **Climate Panel's** four-Faction breakdown is the comparison, this is the detail.

A Faction's name **opens its page** from the Victory window's four blocks and from the Relations
rows, and from nowhere else: on a Region card or the roster a Faction's name describes a place, and
a click there must keep selecting the place.

## 2. A station and an Antarctic Colony ask for people

*Ticket [#204](https://github.com/whaleyjoshua2/Dying-Earth/issues/204).*

A Colony card gains a loader **directly under `Colonists N of M room`**: a station over Earth reads
`from <Region>` and **`Lift N Emigrants`**, an Antarctic Colony **`Bring N Emigrants by sea`**. Both
write the orders the Region card's buttons already wrote, so either door cancels the other.

The dropdown lists **every Region the player directs**, the button greying and naming the reason,
and **opens on the first Region that can actually act** rather than on the most populous. The same
fix applies to the **Ship's** own loader, which had defaulted to the most populous Region whether or
not it had a working Launch Site or anybody waiting — so a door could open already dead.

**Both sea doors are capped at `min(waiting, room)`**, the Region card's included. The engine rule is
untouched: `SendToAntarctica` still accepts overshoot and still sends the surplus home a turn later
with a Report line. Only the button stops offering a round trip that costs a turn and achieves
nothing. **Founding a new Colony from a free slot still offers everyone waiting**, there being no
room limit where nothing stands.

With nothing to move, both doors show a **disabled** button with a refusal of their own — `X is
full` or `Nobody is waiting in <Region>` — because the sea route has no room check for the engine to
refuse with, and a live button there would have handed a player the very round trip this removed.

## 3. The tutorial's first three turns

*Ticket [#205](https://github.com/whaleyjoshua2/Dying-Earth/issues/205).*

Five notes, as before. What changed is that **none of them is spent on preamble**. The first three
each ask for **one thing** and the three are **one chain**: spend Influence, muster Emigrants, send
them to the ISS. Turns 4 and 5 keep their place and **ask for nothing**, pointing at the climate and
at the Research race. The Custodians' identity and Victory Condition ride on turn 1's second line.

**Nothing is forced and nothing is checked**, unchanged from ticket #169.

The arithmetic closes exactly: a Custodian muster is **4**, Emigrants are on the card at End Turn so
a turn-2 muster waits on turn 3, and a bare **Core Module holds exactly 4**.

**"Build a reactor" was not taken literally.** The Reactor is the **Archivists'** Unique Facility
since ticket #184 and a Custodian cannot build one. The Scrubber — their signature, and the only
thing that visibly moves the climate — became turn 4's observation instead of competing for a turn.

**A defect fixed beside it:** a tutorial note dismissed by its button handed on to the turn's Event,
but one dismissed with **Escape** did not, because that path had no Event arm. Both now go through
one rule.

## 4. A Habitat holds four, and eight with Expanded Habitats

*Ticket [#207](https://github.com/whaleyjoshua2/Dying-Earth/issues/207).*

A Habitat holds **4**, from 8. **Expanded Habitats raises it by 4**, from 2, so a *researched*
Habitat holds exactly what an unresearched one held before this version and the squeeze falls almost
entirely on the **early game**: about a third tighter until the Tech lands, which a measured game
reaches around turn 11, and about 15% tighter after it.

The **Core Module's flat 4** is untouched and so is the **Arkwrights' x1.5**, whose Habitats now
hold 6 and 12 where they held 12 and 15. The **Off-world Presence bar stays at 12**.

**One number was doing two jobs, and no longer is.** Expanded Habitats is read by `habitat_room`
*and* by `colony_ship_capacity`, and a single `value` served both — so raising the Habitat clause
would silently have carried Colony Ships from 6 to 8 and the Arkwrights' from 12 to 16, a third more
transport off Earth in the version that cut housing off Earth. `techs.toml` gains
**`habitat_colonists`** for the Habitat clause; `value` stays the Colony Ship's and stays at **+2**.

The Tech's effect line had **never mentioned Colony Ships** and now names both clauses.

**A Colony over its new room keeps its people.** No code was needed: ticket #197's cull runs only
where a Module change actually resolved, so a cap that falls because a *rule* changed destroys
nobody.

## 5. The Archive may not stand on Earth or over it

*Ticket [#209](https://github.com/whaleyjoshua2/Dying-Earth/issues/209).*

`may_hold_archive` becomes **`body != Earth`**: Earth orbit and Antarctica both refused, **the Moon
and everything beyond allowed**. Barring the satellites too was refused with numbers — in 80
measured games there are 0 Mars-system Colonies, 0 Venus stations and 0 Colonies on Phobos or Deimos
— so it would have been a rule the computer could never satisfy.

The refusal message is corrected: it had said *"off Earth; Antarctica will not do"*, which a player
at Axiom would read as a complaint about the ice.

**The computer is taught to go.** While an Archivist has nowhere the Archive may stand and none
begun, the whole chain that carries them to one — Colony Ship, Launch Site or Shipyard, load,
transit, founding — is multiplied by **`archive_needs_a_place`, 5.0**. It touches no other Faction
and stops the moment they hold such a place.

**This is the change that moved the game**, and section 11 gives the figures.

## 6. Every Ship is named

*Ticket [#210](https://github.com/whaleyjoshua2/Dying-Earth/issues/210).*

Two lists of forty in `assets/data/ship_names.toml`. A **Colony Ship** draws from the explorers; a
**Frigate**, a **Battleship** and the **Carrier** from the ships of the line. A name is **unique
across the whole board**, taken as the **first unused name in list order** — deterministic, drawing
**no randomness at all**, because a random pick would shift every later roll in a seeded game and
make a sweep incomparable with its baseline. An exhausted list begins again with a numeral.

One **prefix per Faction**, on its card: **TSV** Custodians, **PMV** Prospectors, **ARK** Arkwrights,
**ACV** Archivists. The **prefix belongs to the holder and the name to the hull**: only the bare name
is stored, and the prefix is read from the seat when the Ship is drawn.

The name replaces the kind and id in all six places a Ship is named — the Tanks block, Load and
unload, the fleet-strength list, the **Transit buttons**, the Region card's send button, and the
**Report**, which had named the ship's *kind*. **The id survives on the hover**: a save, a log line
and a Report all speak in ids.

## 7. The Faction's symbol leaves the setup screen

*Tickets [#203](https://github.com/whaleyjoshua2/Dying-Earth/issues/203) and
[#210](https://github.com/whaleyjoshua2/Dying-Earth/issues/210).*

Ticket #168 confined the symbol to the Faction selection screen. That is superseded. It now marks
**the Faction window's head** at 64 pixels, **a Colony card's heading** — stations and ground
Colonies alike, where a Region card wears its Nation's flag — and **every Ship row**. A **neutral**
place wears nothing, exactly as a Region with no flag stands alone.

These are the second and third callers to pick an icon's colour rather than read it from
`icons::fill`, and for the same reason as the first: a Faction symbol's colour means *whose*, which
is the one thing that rule exists to express.

## 8. Four things you read

*Ticket [#211](https://github.com/whaleyjoshua2/Dying-Earth/issues/211).*

- **The command cluster is a seventh larger**, by one `CLUSTER_SCALE = 1.15`, applied to the panel's
  **text styles** rather than to its four explicit sizes — most of the strip has no explicit size,
  so editing the labelled figures alone would have left a 25-pixel Allotment beside an unchanged
  button.
- **`Emigrants waiting: N`** moves from the Region card's Influence block to the **head of the
  Emigrants block**, above the button that changes it, and is shown **at nought too**.
- **The Research race bar** takes the Tech Tree window's first *sentence*, which moves onto its
  hover. When no Tech is under research the bar cannot be drawn, so the `No Tech under research`
  line stands in its place.
- **Both `Found a Colony` buttons** name the site's four yields on a hover, in glyphs.

**And a fifth thing, older than the ticket.** The race bar had always **under-drawn**: when a Tech is
picked, each seat's banked share goes into progress *and* contributions, but the **unattributed**
carry-over that ticket #105 credits to nobody goes into **progress alone**. So a Tech opening with
spill showed `10 / 45` beside an empty track. The carry-over is now a **grey** segment — grey being
this game's colour for nobody's — and the bar's total equals the figure beside it.

## 9. The Tech Tree: eighteen Techs and a dearer rung 1

*Ticket [#201](https://github.com/whaleyjoshua2/Dying-Earth/issues/201).*

Rung-1 costs go **15 to 18** and Coastal Engineering **12 to 14**. A new eighteenth Tech, **Civil
Defense** — Society, rung 2, cost 30, needing **Public Science** — under which **a Constabulary adds
10 to the challenge margin instead of 5**.

It began as *"something that moderates unrest"* and became something else on its ticket, at the
designer's word and knowingly: it works through the Constabulary and raises the price of taking a
place its holder already has, which is no figure Unrest reads.

The computer is taught to want it, on the **Custodians' and Archivists'** pick lists.

`techs.toml`'s header comment is corrected: it had claimed rungs of **16, 28, 44** and a Coastal
Engineering of 11 since version 0.07.1 while the data underneath said 15, 30, 45 and 12.

**It did not do what it was meant to** — section 11.

## 10. The School's step, and a figure finally shown

*Ticket [#208](https://github.com/whaleyjoshua2/Dying-Earth/issues/208).*

The School's step and decay go to **0.20**, one figure for both.

Measured against ticket #185's reason before it was overruled: walking every Region's ladder to the
ceiling, a neutral Lab's floored Research changes on **34 of 54** steps at 0.25 and **35 of 68** at
0.20. The **count** of visible lifts barely moves; they spread over more turns, so about half a
School's turns show nothing where a third did, and the ceiling takes about a quarter longer.

**And the live Education Level was displayed nowhere in the game.** Both the Region card and the
start screen printed the **static table row**, so since ticket #185 built the School a player could
raise one and watch the number it exists to lift sit still. The card now shows the **live** figure,
with the card's own on the hover.

## 11. What the sweep says

20 seeds x four seatings at the shipped climate cell (sink 6, step 300, permafrost 4.0, sink-after
4.0), against the 0.08.0 baseline the map recorded.

| | 0.08.0 | 0.08.1 |
|---|---|---|
| Custodians | 18 | **42** |
| Prospectors | 3 | **11** |
| Arkwrights | 0 | **7** |
| Archivists | **58** | **6** |
| Collapses | 1 of 80 | **14 of 80** |
| Techs completed, median | 17 of 17 | **18 of 18** |

**The whole movement is the Archive rule, and this was established with a control rather than
assumed.** A single-variable run with only `may_hold_archive` reverted gave Archivists 57, Custodians
19, Prospectors 2, Arkwrights 1 and 1 collapse in 80 — the baseline, near enough. So:

- **The Habitat change moved nothing.** 58 to 57. The computer settles Antarctica and Earth orbit,
  where the Core Module's flat 4 does the work and Habitats are scarce, so a change to what a
  Habitat holds barely reaches it. It remains a real change for a human player.
- **The Archive rule moved everything**, by timing: the Archive now stands in **32 of 80 games** at a
  median turn 23 to 31, instead of about 71 of 80 at 19 to 26, which leaves too few turns to upload
  twelve.
- **The collapse rise is second-order, not climate.** Nothing in this version touches Emissions. The
  Archivists were ending games around turn 20; now nobody does, games run to 32 or later, and the
  world gets more turns to cook.
- **The dearer tree did not bite.** A 9.3% rise and an extra Tech, and the median game still
  completes **all eighteen**. This matches the only comparable measurement the project has: ticket
  #117 raised the whole tree by a tenth and measured nought to three turns.

**The table is more even than it has ever been** — 42 / 11 / 7 / 6 against 58 / 18 / 3 / 0, with no
seat on nothing — and the designer took the Archive rule and its appetite figure on exactly that
ground. **Whether the Custodians at 42 and the Archivists at 6 are where they should be is the
designer's call and is not settled here.**

## 12. Vocabulary and housekeeping

`CONTEXT.md` gains **Faction window** and **Ship name**, and four statements are corrected:

- **Faction**: the symbol is no longer worn "on its card on the Faction screen and nowhere else".
- **Space Station**: a station over Earth is still off Earth, but **may no longer hold the Archive**.
- **Tech Tree**: **eighteen** Techs, not seventeen.
- **Module**: a Habitat holds **four**, and eight with Expanded Habitats, not "eight everywhere".

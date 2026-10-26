# Dying Earth: every mouse-over, as the window shows it

This is the wording of every mouse-over (hover) text the game window builds, for players who
play through the text tool and cannot point at anything. It is read from the program at version
0.1.0.6 (commit dec6d3b).

How to read it:

- `{name}` marks a figure or a name the game fills in during play.
- Where a hover is put together differently in different cases, each case is given.
- A line break inside a hover is shown as a new line.
- In most hovers, a resource word that follows a number (Materials, Fuel, Energy, Ducats,
  Research, Influence, Widgets, Emissions and so on) is drawn as its icon. The words are given
  here.
- **Greyed buttons.** When an order would be refused, its button is greyed. Its hover then gives
  the refusal first: the same sentence the text tool gives when that order is refused. Under it,
  after a blank line, comes the button's usual hover, where both fit within six lines. Where they
  do not fit, the refusal stands alone. This document calls that sentence `{refusal}`. It
  applies to every order button below unless the entry says otherwise.

---

## Setup screens

### Faction cards (the setup screen; also the rulebook section of the Factions window)

Hovering the "Output x{n}" part of the multipliers row:
> Facility and Module output x{n}

Hovering the Emissions multiplier:
> Emissions from Earth sources it controls x{n}

Hovering the Research multiplier:
> Research x{n}

Hovering the Influence multiplier:
> Influence Allotment x{n}

Hovering each figure on the "Opens with ..." line (one hover per good):
> {n} {Materials / Fuel / Energy / Ducats} in the Stockpile on turn one

Hovering the "Play Tutorial" checkbox (Custodians' card only):
> A note at the head of each of the first {n} turns, saying what that turn is for. Nothing is forced, and it stops after the last one.

### Starting-Region screen (the globe)

Hovering "Region population ...":
> The whole Region's people, not its Nation's alone, in units of one million.

Hovering the "leans" glyph:
> Leans {Materials/Fuel/Energy}: the resource this Region is naturally good at producing.

Hovering the Influence figure:
> Influence value {n}: what it adds to its controller's Allotment each turn.

Hovering the Ducats figure:
> GDP {gdp}: its economy pays {n} Ducats a turn (GDP x Industry Level / 5, never below 1).

Hovering the Emissions figure:
> Emissions a turn as the game opens: its industry, its people and its start Facilities.

Hovering the greyed Begin button (no Region chosen):
> Choose a Region on the globe first.

### Credits screen

Hovering each flag:
> {Nation name}

---

## Top bar, first row

Hovering the Materials, Fuel, Fissionables or Ducats figure (or its icon):
> {Word}. Last Income:
> {+n}  {source}
> {+n}  {source}
> ...

or, with no building income of that kind:
> {Word}. No income from buildings last turn.

(If the icon art is missing, the hover drops the leading "{Word}. ".)

Hovering the Widgets figure ("{made} / {applied}"):
> Made a turn: {made}
> {+n}  {maker} ({place})
> {+n}  {maker} ({place})
> ...
> {applied} went into builds last turn.

Makers read like "the Region itself", "Industry Level 3", "the Factory", "the {Module}, x{factor}".
With no maker anywhere, the first line is instead:
> Nothing makes Widgets at the places you direct.

Hovering the Energy figure:
> Last Income:
> {+n}  {place} ({made} made, {upkeep} upkeep)
> {+n}  {place}
> ...

A place shows only its net figure where it only made Energy or only owed upkeep. Past five
places, the places after the fourth fold into one line: "{net}  {k} more places (...)". With no
Energy made or owed:
> No Energy made or owed last turn.

While a Shortfall is forecast, the figure is red and the hover adds, underneath:
> Next Income is {n} Energy short. These go dark, in this order:
> the {building} {place}
> the {building} {place}
> the {building} {place}
> and {k} more

Hovering the Influence figure:
> Influence. The Allotment is what your places and buildings give each turn; bought Influence comes from the Trading window at {price} Ducats each.

Hovering the red "Pick a Tech" button:
> The Research Lead is yours: choose what the world researches next. The turn cannot end until you do.

Hovering the red "Capital building" button:
> Choose one for your Capital.

Hovering the Research figure, with a Tech under research:
> Research: {progress} of {cost} toward {Tech}.
> Click to open the Tech Tree.

with no Tech chosen:
> Research: no Tech is chosen, and {n} Research is waiting for one.
> Click to open the Tech Tree.

Hovering the Research race bar beside it:
> The Research race for {Tech}: Custodians {n}, Prospectors {n}, Arkwrights {n}, Archivists {n}[, carried over {n}]. {progress} of {cost}.
> Click to open the Tech Tree.

Hovering the Temperature figure (shows a chart under the words):
> **Temperature history**
> Where the heat stands. The CO2 Stock already in the air is heading to {+t} C on its own, with nothing else emitted. The Climate Panel's bar shows what it has crossed and what is next.
> [chart of the Temperature over the game]

Hovering the net Emissions figure (shows a chart under the words):
> **Emissions history**
> Net Emissions at the last Resolution: everything the world emitted less the Natural Sink and any Scrubbers. Above zero the CO2 Stock rose and the Temperature will follow; below zero it fell. The Climate Panel breaks it into its sources.
> [chart of Emissions over the game]

Hovering the population figure ("Earth ... · Space ...", shows a chart under the words):
> **Population history**
> On Earth: {Region} {people}, {Region} {people}, {Region} {people}, {Region} {people}, and {k} more, {people}.
> Off Earth: {Body} {people}, ... (or "nobody yet").
> One Colonist is {n} million people; a station over Earth is off Earth and Antarctica is on it.
> Pioneers waiting on a card and Colonists aboard a Ship are in neither line.
> [chart of population on and off Earth]

## Top bar, second row

Hovering "Save (Ctrl+S)" when it can save:
> Write this turn start to a file. Load it again from the title screen.

when greyed because orders are entered:
> A save captures the beginning of a turn, never half-entered orders. Cancel your orders, or end the turn, and save then.

when greyed because a Battle was fought this turn:
> A Battle was fought this turn, so this is no longer a turn start; save at the next.

Spectating only. Hovering the greyed "End Turn (Enter)" button while a window is open:
> Close the window first.

Spectating only. Hovering the "Auto" checkbox:
> A turn every 3 seconds, paused while a Moment or the Report is up. Escape unticks it.

Spectating only. Hovering the clock at the right end of the row:
> Playing for under a minute / Playing for {m} min / Playing for {h} h {m} min

### The fund at the right of the second row (three Factions only)

Archivists. Hovering the "{n}%" Research figure:
> Share of your Research paid into the shared Tech from the next Income. Research kept back counts nothing toward the Research Lead.

Archivists. Hovering the fill bar or its "{fund} of {bar}" figures:
> The Archive fund: {fund} of the {bar} the Victory Condition asks.

Prospectors. Hovering the "{n}%" Ducats figure:
> The share of each turn's Ducat income that goes into the Fund at Income, before you can spend a coin of it, from the next Income until you set it again. Ducats got by selling are not income and never reach it.

Prospectors. Hovering the fill bar or its figures:
> The Venture Capital Fund: {fund} of the {bar} the Victory Condition asks.

Archivists and Prospectors. Hovering the small slider after a setting was refused:
> {refusal}

Custodians. Hovering the Sink bar or the "{run}/{bar}" figure beside it:
> The world emitted {w} ppm last turn; you {y}.
> The Sink took {s} (Scrubbers {n}).
> Four turns in a row under the Sink is Stabilization: run {run} of {bar}.

---

## Command cluster (foot of the side panel)

Hovering the large Influence figure:
> Influence still unspent this turn. It is lost at End Turn: the Allotment does not carry over.

Hovering "of {n} left" (where the Allotment came from, up to six lines):
> {source} {n}
> {source} {+n}
> ...

Sources read like "Regions and buildings 14" or "Spaceport lifts +9".

Hovering "Spend {n} on {place}":
> Raise your Standing on {place}. Its card shows what it would take to hold or take it.

Hovering "Max" with a place selected and Influence left:
> Spend all {n} left this turn on {place}.

If either spend would break an Accord, a red "breaks Accord" note stands beside the button, and
the hover adds:
> breaks Accord. Click again to break it.

After the first click, the button reads "Break the Accord and spend {n}" in red. It carries the
same hover without that added line.

Hovering the greyed "Max" with nothing left:
> Nothing left to spend this turn.

Hovering the greyed "Max" with nothing selected:
> Click a Region or a Colony to spend on it

Hovering the "every turn" checkbox:
> Spend your whole Allotment here every turn until cancelled.

when greyed (nothing selected):
> Click a Region or a Colony first.

Hovering the clock beside End Turn:
> Playing for under a minute / Playing for {m} min / Playing for {h} h {m} min

Hovering the End Turn sun when the turn can end:
> End the turn (Enter).

when a window is open:
> Close the window first.

when something is owed (one thing):
> {what is owed} The turn cannot end until you do.

when two or more things are owed:
> Two things before the turn can end:   (or "Several things ...")
> - {what is owed}
> - {what is owed}

The "what is owed" lines are:
- `{Card} is asking the {Faction} a question, and it has not been answered. Answer it.`
- `{Faction} offer {Faction} an Accord. Accept it or refuse it.` ("a Trade" for a Trade)
- `{Faction}'s Accord with {Faction} lapses next turn. Renew it or let it lapse.`
- `The {Faction} hold the Research Lead and owe the table a Tech. Choose what the world researches next.`

Fallback when no reason is given:
> The turn cannot end yet.

Hovering the Temperature ring around the sun (a click opens the Climate Panel):
> Temperature {+t} C, heading to {+t}
> next: {Break} at {+t}   (or "every Break is behind us")
> Collapse at {+t}. Click for the Climate Panel.

---

## Maps

Solar System Map. Hovering a Body: its label opens up. Under "{Body}  {k}/{n} slots, {k}/{n} stations"
it adds one line per station slot:
> {station}: {Faction / nobody's / free}[, a {Faction} warship in it]

For a Body other than Earth that has Colony Slots, it adds one of:
> first settled by the {Faction}
> first to land: {n} Influence

Solar System Map. Hovering a Body across the Earth–Mars gulf (launch-window line under it):
> {Body} window: {this turn / next turn / in {n} turns} ({date}). Flight now: {n} turns, {f} Fuel. At the window: {n} turns, {f} Fuel.

Both maps. Hovering a Faction's ship stack, or its line in an in-orbit block:
> {Ship name} ({kind})[, Warhead aboard / Warhead spent]
> ... (up to five Ships)
> and {k} more - click the stack

Both maps. Hovering a Battle ring:
> A Battle here last turn: the {Faction} attacked; {result}   (or "A Battle here last turn; {result}")
> {Faction or Neutral}: {units} ({n} hits landed)
> ... (up to four sides)
> Click the ring for the Report.

Any map label under the pointer shows its full form rather than its short name.

---

## Side panel with nothing selected (a Body's card)

Hovering a "Battles last turn" row (Solar System Map page):
> Go there

or, for an old record with no place:
> An older record with no place to go to.

Hovering the "Colonies and sites" heading:
> Every Colony on the ground and every site still open, each with its own four yields: what a Mine, a Generator, a Refinery and an Observatory make there, against the figure for the whole Body above. A station reads the Body's figures, so its row below carries none.

Hovering the "First to settle {Body}: ..." line:
> First to land here: +{n} Influence a turn while held.
> A station claims nothing.

Hovering "The race to the Moon" (the Moon's card, until somebody lands):
> How far each Faction has got toward landing here, the leader first. Everyone sees it, until somebody lands.

Hovering "In orbit: {k} of {n} station slots":
> Room for {n} stations, one to a ring. A station is reached only from its own ring; a Warship on Blockade there starves it.

Hovering a station's button in the In orbit list (its Modules other than the Core):
> {Module}, {Module}, ...   (or "bare")

Hovering "Build {station} here": only `{refusal}` when greyed.

---

## Roster

Hovering a Ship row at a Body:
> {Stance}: {stance sentence} A stance persists until it is changed.
> Tank {fuel} of {tank}. Fills at your station in its ring, over your Colony with a Refinery, or a Refuel partner's.
> STRANDED: no leg it can pay, nowhere to fill.

Hovering a Ship row in transit:
> A Ship in transit cannot be ordered and cannot be intercepted. It arrives at its Resolution, Holding, with whatever Fuel it has left.

Hovering an Army row:
> An Army keeps the stance it was last given until it is moved or given another; it is never reset at a Resolution.
> {Stance}: {stance sentence} A Standing Army replenishes where it stands, unless its state's Unrest has passed {n}.

Hovering the "3/6" Colonists figure beside a Colony Ship (also on the stack card and the Ship's card), when
one to three are aboard:
> {n} found a Colony

when at Earth with crowding room:
> {room} room, +{extra} crowded at Earth

(Both lines appear when both apply. There is no hover otherwise.)

Hovering the ring at a row's end:
> Still wants an order from you this turn.   (open ring)
> Has its order for this turn.   (filled ring)

Colony and Region rows have no hover.

### The stance sentences

For Ships / for Armies:
- Attack: Strikes at the place it is sent to, or fights where it stands.
- Hold: Stands and fights where it is; does nothing of its own.
- Evade: Avoids battle where it can: an even chance to slip away before the first exchange.
- Dig In (Army): Dug in, it fights two stronger in defence and never disengages, and it cannot march or board a Carrier until its stance is changed and the turn has passed.
- Dig In (Ship): A Ship cannot dig in; it holds.
- Intercept (Ship): Fights what arrives this turn into its own orbit, before it can land.
- Intercept (Army): An Army cannot intercept; it holds.
- Blockade (Ship): Shuts the orbit it sits in to every other Faction: no unloading, no refuel, no building in it.
- Blockade (Army): An Army cannot blockade; it holds.

---

## Build and order buttons (all cards)

A build button's hover opens with its price line:
> {m} Materials, {w} Widgets, about {n} turns here.

The end of that line can also read "ready next turn here" or "nothing here makes Widgets, so it would never finish".
- Building an Army adds the people it takes: "{m} Materials, {w} Widgets, and {people}, {when}."
- A Ship: "{m} Materials, {w} Widgets, {when}." then a second line "Built full: {f} Fuel[, {x} Fissionables]."
- A nuclear refit: "{x} Fissionables, {w} Widgets, {when}."
- The "or" button beside a build (buying it outright): "{d} Ducats, ready at the next Resolution ahead of the queue."

Where the button also describes what it makes, the hover continues:
> Once it stands: {description}

For a Facility or Module, the description is its yield, for example "+6 Materials, 2 Energy upkeep,
1.0 Emissions". Where the figure is multiplied, up to four lines of working follow, for example:
> 6 base
> × 1.10 {reason}
> = 6.6, rounded to 7

A build button that carries no description shows the price line alone, ending with a full stop.
Examples are "Build Army", each Ship at a Shipyard, the "Nuclear" hull beside it, and every "or"
button. An order that builds nothing (a move, a load, a Relief, a Leapfrog and so on) has no
price line. Its hover is its description alone where it has one. Otherwise it has no hover
except `{refusal}` when greyed.

Mothball button on a Region's Facility:
> +{n} Unrest

Decommission button on a Region's Facility:
> +{n} Unrest, {m} Materials back

Replace button (a Power Plant, once Clean Power stands):
> {price line}.
> Once it stands: With a Wind Farm: {Wind Farm yield}

(Restart, and any Mothball or Decommission in a Colony, show only `{refusal}` when greyed.)

---

## Region card

Hovering the line saying who holds the Region:

Neutral:
> Nobody holds it. The first Standing to reach the threshold takes it; two arriving level are settled by lot.

Controlled:
> The {Faction} hold it, and take its Influence value into their Allotment each turn.
> A rival needs their Standing plus {margin}, and at least its own threshold.

Occupied:
> An Army of the {Faction} beat its defenders: they choose what is built here but do not direct its Armies.
> Control passes after {n} turns, or sooner once the occupier's Influence, gained each turn and halved while Unrest is high, passes the threshold.

`{margin}` reads "a margin of {n}", or "a margin of {n} ({base}, +{k} for cold relations, +{k} for the Constabulary)".

Hovering "Region population ...":
> Growth {+g}% a turn, less {l}% per tenth of a degree above {+t} C.
> This turn {+r}%: {+p} million.

Hovering each figure on the "Output:" row (up to five sources, then a total for the rest):
> {source} {n}
> {source} x{k} {n}
> ...
> and {k} more: {n}

Hovering "Influence value {n}: what it adds to its controller's Allotment each turn (+1 per Industry Level raised)":
> Influence each turn: {source} {n}, {source} {+n}, .... Unspent is lost.

Hovering "GDP {n}: Industry Level {n}, leans {lean}":
> Pays its controller {n} Ducats a turn.   ("an unseen number of" for a rival's Region you cannot see)
> GDP x Industry Level / 5, never below 1.
> A {Bank / n Investment Bank} here would add {n}.

Hovering "Emissions this turn: industry ..., Facilities ..., people ...":
> People: {r} per hundred million ({b} base + {k} x Industry Level {n})[, Leapfrogged once / twice / {n} times]

Hovering "Education Level {n}":
> {card value} on the card, and {+k} from a School.   (or "..., and no School standing.")
> It counts twice in this Region's Research: once weighting how many people it has, and once on its own. It also stiffens this Region against an outsider's Influence, and goes with any Colonist recruited here.

Hovering the Unrest line:
> Unrest to {max}: Army stops at {a}, Facilities halve at {f}, revolt at {max}. Falls {n} a turn.

Hovering "Agitate: Unrest +1" (on a rival's Region):
> Turn its people against the {Faction}: Unrest rises by {n} at End Turn, halved by a working Constabulary. Once a turn here. They will know who paid: it is an offence.

Hovering the "Influence" heading:
> Your claim here: spend from the corner's Allotment and it becomes your Standing, which survives any change of hands.
> Highest Standing at the threshold takes a free place; a held one wants the holder's plus {margin}.
> Decays {n} a turn for the holder, {n} for everyone else.

### Standings row (Region and Colony cards)

Hovering "Standings:":
> Standing is the Influence a Faction has built up here. It persists, and is never wiped when the place changes hands.
> Left alone it decays {n} a turn for the controller and {n} a turn for everybody else, so a claim you stop paying for slides faster than the holder's.

Hovering a Faction's Standing chip. It opens with "Your Standing here." or "The {Faction}' Standing here.", then one of:
- Your chip, on your own place, with a rival standing: `The {Faction} take it at {price}: your Standing plus {margin}. Every point you add here adds one to that. They stand at {n}, {k} short.` Where their threshold binds instead, the middle reads `their threshold, which stands over your Standing plus the margin.`
- Your chip, on your own place, with no rival standing: `No rival has any Standing here; one would need {n}.`
- The holder's chip: `They hold this place.`
- Otherwise: `You take this place at {n}, so you want {k} more.` (or `They take ... so they want ...`)

Hovering the price label, on your own place ("A rival takes it at {n}"):
> The {Faction} take it at {n} = {s} (your Standing) + {base} (margin)[ + {k} (relations)][ + {k} (Constabulary)]
> Threshold {t} = {base} + {k} for size {n}[ + {k} for Regions held][, x{b} (Blame)][, cut by Green Consensus]
> Threshold: {rule}

On a rival's place ("Take at {n}"):
> Take at {n} = {s} (their Standing) + {base} (margin)[ + ...]
> Threshold ... (as above)
> Threshold: {rule}

Where the threshold is higher than Standing plus margin, the first line reads instead:
> {head} {n}: {whose} {s} + the margin {m} is {sum}, under the threshold, which stands

On a place nobody holds ("Threshold {n}"):
> Threshold {t} = ... (as above)
> Threshold: {rule}
> Green Consensus cuts a quarter off it; your Blame adds to it.

`{rule}` reads, for a Region, "{base} + {k} a size step + {k} a Region held past your first"; for a
Colony, "{base} for the Colony + {k} a Colonist living here" (or "for the station").

On a Colony or station, the threshold line reads "Threshold {t} = {base} for the Colony (or for the station) + {k} for {n} Colonists".

Hovering the challenger line ("The {Faction} stand at {n}; they take this at {p}.") on your own place:
> The {Faction}'s price here is the greater of their own threshold, {t}[ (x{b} for their Blame)], and your Standing plus the margin they face, {s} + {m}.
> They are {gap} short. An outsider's Influence converts at {c} here, so that is about {n} Influence spent.
> Spending here raises the bar; doing nothing lowers it, yours decaying {n} a turn and theirs {n}[ for their Blame].

Hovering the threat line ("The {Faction}' {Army}, strength {n}, stands next door in {Region}."):
> Their {n} against the {d} that defends here: {p}% is their chance to win the first exchange.
> The computer seats attack at {b}% or better, so the line turns amber there.
> Armies next door whatever their stance; a Region's own Army standing at home is no threat, marched out it is.

Hovering the amber Blame note (your threshold raised by Blame):
> Blame is the CO2 you are answerable for all game: what your places emitted, less what you took back.
> Your share is {n} per cent, above a fair quarter, so every place you do not hold costs more to win over -- up to half again.
> The Climate Panel breaks it down by Faction.

### Pioneers block (your own Region)

Hovering "Pioneers waiting: {n}":
> Waiting. A Launch Site lifts them; once the ice opens, ships take them to Antarctica.

Hovering "Recruit {n} Pioneers":
> {people} people, Unrest -{n}.

Hovering "Send {n} to {station} by lift":
> Aboard at this turn's Resolution. A launch: it emits like any lift. {station} has room for {n} more.

Hovering "Send {n} to {Ship} ({orbit})" when it can go:
> A launch, aboard at this turn's Resolution. This Ship carries {cap} and has {n} aboard. To crowd it past its capacity, load it from its own card.

(Greyed, it shows only `{refusal}`.)

Hovering "Send {n} to {site} by sea" (founding in Antarctica): only `{refusal}` when greyed.

Hovering the "Launch Site:" (or "Spaceport:") Yes/No line:
> Pioneers and Armies lift to any orbit of Earth.   (Yes)
> Pioneers and Armies lift to low orbit only.   (No)
> Pioneers and Armies lift to low orbit only.
> Its {Launch Site / Spaceport} is dark.   (No, one stands but is dark)

### Widgets and the eye

Hovering "Widgets {n} a turn" (Region and Colony cards):
> Widgets {n} a turn here: {n} from the Region itself, {n} from Industry Level {k}, {n} from the {building}.

(or "...: nothing here makes any.")

Hovering the heading "{Your Embassy in X / Your Relay at Y} reads what the {Faction} draw here, building by building:":
> A working Relay at your Colony off Earth, or Embassy in your Region on Earth, shows every rival's income at that Body, building by building; the Faction window gives totals only.

### Facilities

Hovering "Facilities ({k} of {n} slots free[, {k} ordered this turn])":
> Slots: Size {s} + {base} + starting Industry {i}, +1 inland per raise.
> {c} coastal: each sea threshold takes one (with its oldest Facility) and turns an inland slot coastal. A Sea Wall stops the taking, not the turning.
> Mothballed and building each keep a slot.

Hovering a standing Facility's box:
> {Facility}[ (Replace ordered)] ({coastal/inland}): {figures}
> [Offline ... line, or "At half: Unrest."]
> Energy upkeep is paid at Income first; short of Energy, buildings go offline in order until the bill is met, and an offline one makes nothing and keeps its slot.
> Its Emissions go on the CO2 Stock every turn and on its controller's Blame.
> On the coast, the sea can take it at a threshold.   (coastal only)

Where the figure is multiplied, the two rules sentences give way to the working, for example
"6 base / × 1.10 {reason} / = 6.6, rounded to 7". The heading and any offline line stay. The
whole hover is at most six lines.

`{figures}` is the yield, for example "+6 Materials, 2 Energy upkeep, 1.0 Emissions". It can also be:
- `mothballed: making nothing, paying no upkeep, emitting nothing, keeping its slot`
- `idle, nobody directs this state`
- `out of sight`
- `{yield} (for the world: {n} Research a turn)` for a Research Lab under occupation
- `in no one's hands: {n} Research a turn for the world`

The offline lines are:
- Offline until next turn: struck by {card}.
- Offline until next turn: struck by a card.
- Offline: short of Energy.
- Offline: grid down.
- Offline: Colony occupied.
- Offline: upkeep unpaid.
- Offline: too few people to run it.

Hovering the clicked Facility's line in the strip under the boxes: the same hover, but the
heading is the Facility's name alone, with no figures and no offline line.

Hovering a Scrubber or Sea Wall row (they take no slot):
> {full sentence}[ offline line]

For a Scrubber, the full sentence is the yield text. For a Sea Wall, it reads
"{yield}; {k} rises held, {m} Materials a turn to keep[; unkept this turn]", or "...; nothing held yet".

Hovering a box under construction:
> {Facility} ({side}): building, {done} of {total} Widgets, {about n turns / ready next turn / nothing here makes Widgets} at this place's rate.
> On the coast, the sea can take it at a threshold.   (coastal only)

If another Faction began it, a line follows. Where you cannot cancel it (not yours to direct):
> Begun by the {Faction}.

Where you can:
> Begun by the {Faction}, who paid for it. Right-click to cancel it: {m} Materials to your Stockpile at your own price, and its Widgets so far are lost.

Already cancelled this turn:
> Begun by the {Faction}. Cancelled this turn: {m} Materials to your Stockpile at End Turn.

Cancel refused:
> Begun by the {Faction}. Cannot be cancelled: {refusal}.

Hovering a box ordered this turn:
> {Facility} ({side}): ordered this turn, {w} Widgets, {about n turns} once the turn ends.
> Right-click to cancel the order.
> On the coast, the sea can take it at a threshold.   (coastal only)

Hovering a free box:
> Free {coastal/inland} slot: click it to build here.
> On the coast, the sea can take what stands here at a threshold.   (coastal only)

Hovering a flooded box:
> {Facility}, lost to the sea: a Sea Level threshold took this coastal slot.   (or "A slot lost to the sea: ...")
> A working Sea Wall holds every threshold off, at most one to a state.

Hovering the Raise Industry Level tile:
> Raise Industry Level: {m} Materials, {w} Widgets, about {n} turns. Adds an inland slot.

(Greyed, `{refusal}` comes first.)

Raise ordered this turn:
> Industry Level: ordered this turn, {w} Widgets, {about n turns} once the turn ends. Adds an inland slot.
> Right-click to cancel the order.

Raise under way: as for a box under construction, with heading "Industry Level" and the tail " Adds an inland slot."

Hovering a Facility build button in the strip ("Build here"): the build hover, with "Once it stands: {yield}".
Hovering "Scrubber ({k} of {cap})" or "Sea Wall": the build hover, with "Once it stands: {yield sentence}".

### Armies block (Region card)

Hovering a stance in the "Stance:" row:
> {Stance}: {stance sentence}
> A stance persists until it is changed.

Greyed:
> {Stance}: {refusal}

Hovering a Region's own (standing) Army row:
> A Region's own Army: strength and hit points Industry Level + 1[ and +{n} armed]. Defending at home: +{c} Constabulary (or "+{c} if it had a Constabulary"), +{k} calm (or "+{k} lost to Unrest {n}+")[, +{d} dug in].
> Heals 1 a turn while Unrest is under {n}. Destroyed at its strength in damage; back at 1 two Incomes later.
> Neutral, it arms for good: +{n} at a threat next door, +{n} per attack held.

Hovering a raised Army row:
> A raised Army: its strength and hit points were its home's Industry Level + 1 when it was raised, fixed since. It belongs to its home Region and changes hands with it; it marches, and it may Dig In for +{n} while defending.

Hovering "move to {Region}" (your own Region), one Army:
> {Region}: held by you. Moving costs nothing; the Army keeps its stance.

Under Passage:
> {Region}: held by the {Faction}, a partner under Passage. Moving costs nothing; the Army arrives on Hold and fights nobody while the Accord stands.

For the whole stack ("All {n} ({strength}):" row), the same with "the stack keeps its stance" /
"the stack arrives on Hold.".

Hovering "attack {Region}" (one Army, or the whole stack), undefended:
> {Region}: {neutral / held by the {Faction} / occupied by the {Faction}}, undefended. The Army arrives on Attack and the Occupation begins.

defended:
> {Region}: {holder}.
> defended by {Army}: strength {s}, defends at {d}[ (dug in)], damage {k}/{hp}
> ... (three named; past three, two named and then "and {k} more, defending at {n} in all")
> {p}% is the chance of holding the field when the Battle is over, not of winning its first exchange: your {a} against their {d}.
> Moving costs nothing; the Army arrives on Attack.

Hovering "Repair all":
> Every damaged Army of the stack the Materials cover, repaired fully.

Hovering "Repair all with Ducats":
> Every damaged Army of the stack the Ducats cover, repaired fully.

---

## Colony and station card

Hovering the Faction symbol beside the name (also on stack and Ship cards):
> The {Faction}

Hovering the red "Threats: ..." line (your own places):
> {Ship} ({kind}), strength {n}
> {Ship} ({kind}), {Army} aboard, strength {n}
> ... (five, then "and {k} more")

Hovering the "First to settle {Body}: ..." line (this Colony was the first on its Body):
> The {Faction} raised the first Colony ever to stand on the ground of {Body}, and took that world's windfall once.
> The +{n} a turn is theirs alone: it stops while somebody else directs this place, wakes if they take it back, and keeps paying while the Colony is starved of Energy.

Hovering "{Blockaded by the X / Under the X' Orbital Control}: producing nothing, upkeep still paid.":
> Nobody dies, nothing is destroyed. Each turn is an offence against you.

Hovering "A shut Habitat houses people here: everything but Energy at half.":
> Overcrowded: everything but Energy at half. Restart a Habitat.

Hovering the Colonists line ("{n}/{room} ({people}M)"):
> Colonists {n} of {room} room.
> {growth}

`{growth}` is one of:
- Growing: {+g} a turn, next Colonist in {n} turns.
- Full: no room to grow. A Habitat makes room.
- Nobody here to grow.
- Shrinking: {n} a turn, under Blockade.
- Shrinking: {n} a turn, its Core offline.

Hovering the greyed "Bring Pioneers by sea" / "Lift Pioneers":
> Nobody is waiting in {Region}. Recruit Pioneers there first.
> {Colony} is full: {n} Colonists in {room} of room.

Hovering "Bring {n} Pioneers by sea":
> They land here at NEXT turn's Resolution: a sea crossing takes a turn. {Colony} has room for {n} more.

Hovering "Lift {n} Pioneers":
> Aboard at this turn's Resolution. A launch: it emits like any lift. {station} has room for {n} more.

Hovering "Modules {used} of {cap}":
> One a Colonist, {n} free, at most {cap} as a {tier}.   (or "One for every {n} Colonists...")
> Mothballed keeps a slot, building reserves one; the Core and the Archive take none.

Hovering a standing Module tile:
> {Module}: {figures}[{first-to-settle clause, Core only}]
> [Offline line, "Blockaded: makes nothing.", or "At half: shut Habitat."]
> Energy upkeep is paid at Income first; short of Energy, Modules go offline in order until the bill is met, and an offline one makes nothing and keeps its place.
> Mothballed, it makes nothing and pays nothing until it is restarted.   (not on the Core)

As with Facilities, a multiplied figure's working replaces the rules sentences. The Core's first-to-settle
clause reads ". First to settle {Body}: +{n} Influence a turn to the {Faction}" or ". First to settle
{Body}: the {Faction}' +{n} Influence sleeps". `{figures}` can also be
"mothballed: making nothing and paying no upkeep", "idle" or "out of sight".

Hovering a Battery tile:
> Battery: strength {s}, {hp} of {max} hit points, {e} Energy upkeep[ offline line]
> Covers {orbit} alone, and fights in any Battle there.
> While it works, {no rival's Blockade shuts this station, and a blockader here is fired on / no rival's Blockade of low orbit shuts the ground, and none holds Orbital Control of it}.
> Repaired here with Materials; destroyed at {n} hits.

Hovering the capital building's tile (gold edge, ground Capitals):
> {Building}: {what it does}.
> Choose a capital building: click.   (none chosen, yours)
> No capital building chosen yet.   (none chosen, a rival's)

Hovering a Module tile under construction: as for a Facility box under construction, heading
"{Module}", no coastal line.

Hovering a Module ordered this turn:
> {Module}: ordered this turn, {w} Widgets, {about n turns} once the turn ends.
> Right-click to cancel the order.

A Habitat from a scrapped Colony Ship reads instead:
> Habitat: from the scrapped hull. Lands this turn.
> Right-click to cancel the order.

Hovering a free tile:
> Room for another Module: click it to build here.

Hovering the Upgrade tile:
> Upgrade to a {tier}: {price}. Up to {n} Modules[, +{k} Influence a turn].

Greyed, `{refusal}` comes first. Where the Body's one Capital stands elsewhere, the hover is only
the refusal, for example "The Moon's Capital is Plato."
Upgrade ordered or under way: as for a Module ordered or under construction, headed with the tier's name.

Hovering the Archive tile:
> The Archive: {complete, online / complete, offline / standing, {fund} of {n} Research paid}[ offline line]
> Outside the Module count. Its Research is paid into the Archive fund at any pace; complete, it takes a great deal of Energy to keep running. Destroyed if this Colony changes hands, and the fund is lost with it.

Hovering the clicked Module's line in the strip (only where its figure is multiplied): the
tile's hover, with the working.

Hovering a Module build button ("Build here"): the build hover, with "Once it stands: {yield}".

Archivists. Hovering "Build the Archive ({m} Materials)":
> {m} Materials, {w} Widgets, about {n} turns here; then the fund opens to the full {n} Research

Archivists. Hovering "{n} remembered here":
> Of {bar} for Victory.

Archivists. Hovering "Upload {n} Colonists":
> Read into the Archive; {k} stay. Cannot be undone.

Hovering the Influence spend figure (the number box):
> How much of this turn's Allotment to put here. It becomes your Standing and stays, whoever holds the place after.
> Unspent Allotment is lost at End Turn. A Standing decays {n} a turn for the holder, {n} for everyone else.

Hovering "Spend":
> Adds the figure beside it to your Standing here, now {n}. You take this place at {k}.

The Accord-break note and two clicks work as in the command cluster.

Hovering "Buy more Influence in the Trading window":
> Ducats buy Influence into this turn's Allotment, {price} a point, spent like any other. Opens the Trading window.

Colony Armies block: the stance row as on a Region card; the Army rows have no hover.

---

## Colony Slot card (an empty site)

Hovering "Found a Colony here with {n} Colonists from {Ship}" (also "Found a Colony at {site} with {n}"
on a Ship's card): only `{refusal}` when greyed.

Hovering "Build a Colony here from your station": only `{refusal}` when greyed.

---

## Ship stack card

Hovering a Ship's row (a link):
> {Ship name} [{Kind}]   (in its Faction's colour)
> Opens the Ship's card.

Hovering a stance in the stack's "Stance:" row:
> {Stance}: {stance sentence}
> A stance persists until it is changed.

Attack (Ships) reads instead:
> Attack: Strikes at the place it is sent to, or fights where it stands.
> The Battle is fought the moment you confirm, on the board as it stands; the survivors stand on Hold.

Intercept (Ships) adds:
> The computer seats intercept with warships when an unarmed rival hull is inbound: a Colony Ship or a Carrier.
> A hull with less than {n} Fuel in the tank catches nobody.

Blockade (Ships) adds:
> A station's own orbit shuts and starves that station; low orbit shuts the ground to rivals' landings, and starves it while you hold Orbital Control outright.
> A warship with less than {n} Fuel in the tank blockades nothing.

Greyed:
> {Stance}: {refusal}

Hovering an odds line ("Attacking {orbit}: {p}% is your chance of holding the orbit ..."):
> The chance you are victorious: {p}%.

Hovering "Attack this turn" (stack card) or "Attack with all {n} Ships at {Body}" (Ship's card):
> The Battle is fought the moment you confirm, on the board as it stands; the survivors stand on Hold.

Hovering "All {n} that can" under a change of orbit:
> Every Ship of the stack with {f} Fuel in the tank and no other order, {n} of {total}, moved together.

Hovering "All {n} that can" under a transit:
> Every Ship of the stack whose tank pays the leg, {n} of {total}, sent together into {orbit}.

If no Ship can go, the first Ship's refusal comes first. With no reason given, it reads
"Nothing here can take the order."

The Influence rows for Colonies at the Body work as on a Colony card.

---

## Ship card

Hovering the Ship's name in the heading:
> {Ship name} [{Kind}]   (in its Faction's colour)

Hovering the "{In low orbit / At station}, on {Stance}" line:
> {stance sentence} A stance persists until it is changed. The stance is the whole stack's, set below, for the whole stack.

On an unarmed Ship, or one that is not yours, the ending reads "set on the stack card or on an armed Ship's card."

Hovering the "In transit from ... to ..." line:
> A Ship in transit cannot be ordered and cannot be intercepted. It arrives Holding, with whatever Fuel it has left.

Hovering "Refuel from the Stockpile" at a partner's station:
> At the station or Refinery Colony of the {Faction}, under your Refuel Accord: the Fuel is your own Stockpile's, drawn there.

(At your own station or Refinery Colony there is no hover except `{refusal}`.)

Hovering "stranded: no leg it can pay":
> Rescued by a station of yours here, a working Refinery Colony of yours under low orbit, or a Refuel Accord with one who holds either. Or scrap it, if it is an empty Colony Ship over a Colony of yours.

Hovering "Move ({fuel}/{tank} in the tank)" under a change of orbit:
> {Ship} is {in low orbit / at station} now. {f} Fuel from its own tank, and it fights this turn's Battle in its new orbit.

Hovering "Go ({fuel}/{tank} in the tank)" under a transit: only `{refusal}` when greyed.

Hovering "Land the Army at {place}" (your own place):
> {place}: held by you. Landing costs nothing; the Army lands on Hold.

Hovering "Land the Army to attack {place}": the attack hover as for an Army's attack. Its first
sentence ends "The Army attacks as it lands and the Occupation begins." when undefended, and its
last line reads "Landing costs nothing; the Army attacks as it lands."

Hovering "Load ...", "Unload ...", "Found {station} with ...": only `{refusal}` when greyed.

Hovering "Scrap for a Habitat at {Colony}":
> Breaks the hull up into a Habitat at {Colony} (+{n} Colonists' room). Lands this turn.

Hovering "Bombard {place}" (Battleship):
> One Module of {n} at {place}, drawn at random, rolls a {p}% chance to burn; when a Habitat burns, the Colonists beyond the room left die with it. A rung 3 offence against the {Faction}, breaking a non-aggression Accord if one stands. {orbit rule} {Ship} is {in low orbit / at station} now.

The `{orbit rule}` is "The Battleship must be in low orbit, with Orbital Control of low orbit held outright;
never over Earth's ground." for a ground Colony. For a station it is "The Battleship must be at
{station}, with no rival warship and no rival working Battery in that orbit."

Hovering "Launch at {target}" (Missile Carrier):
> Every building at {target} ({n} Modules / Facilities) rolls {p}% to burn; {lo}-{hi}% of its people die. {and then} Rung {r} against the {Faction}, and any Accord breaks. The Warhead is spent either way.[ Over Earth: {w} ppm of war Emissions and +{s} to the Sink, for good.]

`{and then}` is "The Core Module and the Archive are spared." at a Colony, or "Its Standing Army dies too, and its Industry Level falls by {n}." at a Region.

Hovering "Rearm":
> {price line}.
> Once it stands: the hull carries another Warhead and may Launch again. It is built at {Colony}, behind whatever already stands in that Shipyard's queue.

With nowhere to build it, it shows `{refusal}` and then one of:
> A place of yours is in this orbit, but nothing there is a working Shipyard.
> A Warhead is loaded at a Colony or station of yours, in that place's own orbit; there is none of yours in this one.

Hovering "Refit nuclear":
> {x} Fissionables, {w} Widgets, {when}.
> Once it stands: Refits the hull to burn Fissionables: shorter crossings, a tenth of each leg's Fuel in Fissionables. One way.

Hovering "Repair fully" / "Repair fully with Ducats": only `{refusal}` when greyed.

The Ferry block's "Set route" has no hover. Its reason is printed under it.

---

## Trading window

Hovering the "Price" column heading:
> +1 Ducat for every {n} you buy this turn, -1 for every {n} you sell; up to {b} either side, never under 1.
> Everyone's trading moves next turn's price.

Hovering a greyed "Buy for ..." or "Sell for ..." button:
> {refusal}

---

## Factions window

Hovering an "Income last turn" figure, your own page (or any page when spectating):
> {Word}. Last Income:
> {+n}  {source}
> ...

or:
> {Word}. No income from buildings last turn.

On a rival's page:
> {Word}. A rival's income is shown as a total only.

Hovering "Blame":
> Blame: CO2 this Faction's places emitted, less what it removed (Scrubbers, Nature Reserves, the Custodians' Sink Directive).
> Above a quarter share: Influence thresholds rise up to +50% where it doesn't hold, and every rival likes it a point less per step.

Hovering "Answerable for {n} ppm" (up to three lines; a part shows only when it is not zero):
> Emitted {n}, removed {n} in credit
> {n} laid on by rivals, {n} bought as carbon credits, {n} sold as carbon credits, {n} cleaned by campaign
> Thresholds x{m}

The second line shortens to "by rivals, bought, sold, cleaned" when it would not fit.

Hovering "Relations":
> {best} to {worst} from a neutral {start}, read as six levels from Friendly to Hostile. A score is what the pair have DONE to each other plus what this Faction makes of the other's Blame -- hover a level for the two figures. Offences differ in weight and a turn charges every one, to {cap} at most; quiet mends {n} every {k} turns below neutral and lapses half as fast above it. A pair crossed on {k} turns can never fully recover again.
> A rival that holds you at less than neutral defends its places against you a little harder, and an Accord wants a level it will not strike below.

Hovering a Faction's name in a Relations row (also the Faction names in the Victory window):
> Open this Faction's page.

Hovering a relations level (Friendly, Cordial, Neutral, Wary, Cold, Hostile):
> {+s} in all: {+d} from what they have done, {+b} from their Blame.
> Greenwash {+n}   (when not zero)
> Research shared {+n}   (when not zero)
> Scarred: this pair can never recover above {floor}.   (when scarred)
> An Accord stands: {terms}; {n} turns left.   (or "An Accord between them is over: it lapses at the next turn ({terms}).")

### On a rival's page

Hovering "You hold an Accord with the {Faction}: ..." or "Your Accord ... is over ...":
> {one line per term held}
> Acting against a term ends the Accord, at a cost.

The term lines:
- Non-aggression: no Influence or Battle on each other.
- Passage: Armies march in on Hold; Ships pass freely.
- Refuel: at each other's Space Stations.
- Research agreement: +10% Research to both.

Hovering "Renew for five turns":
> Five more turns, the same Terms.

or:
> Five more turns. The research agreement drops: you are no longer both Friendly.

(Greyed: `{refusal}` alone.)

Hovering "Declare it over":
> Lapses next turn. Breaking a term sooner costs 3 Relations and ends it.

Hovering a term's checkbox: its term line (above).

Hovering the greyed "Research agreement" checkbox:
> Both sides must be Friendly to strike a research agreement.

Hovering "Offer the {Faction} an Accord" / "Offer to add":
> They answer next turn. A refusal costs you nothing: it is not an offence.

Hovering "Pay {n} Ducats" / "Pay {n} Materials" (tribute):
> A tribute raises their view of you by one. A fixed gift, one a turn to a Faction: the gain is flat, so a larger one would buy no more.

When already paid this turn:
> You have already paid them a tribute this turn.

When refused:
> {refusal}

Hovering "Offer {Faction} a Trade":
> They answer next turn; the goods move only if both still hold them.

Hovering "Smear campaign":
> Influence spent on this Faction rather than a place: every point lays {r} ppm on their Blame for good, and the share every rule reads moves with it.
> One campaign a turn against each rival, from this turn's Allotment. They will know who paid: it is an offence.

Hovering "Smear the {Faction} with {n} Influence":
> {p} ppm on their Blame at End Turn.

(Greyed: `{refusal}`.)

Custodians' page. Hovering "Carbon credits":
> A ppm of carbon credit bought comes off your Blame for good. The Custodians sell it at {p} Ducats a ppm, times how they think of you -- Friendly x{a}, Cordial x{b}, Neutral x{c}, Wary x{d}, Cold x{e}; Hostile refuses -- up to {cap} ppm a turn.
> A purchase is an act of friendship both ways.

Custodians' page. Hovering "Request {n} ppm for {d} Ducats":
> Off your Blame at End Turn. If others request first and the offer runs out, the Ducats for what you did not get come back.

(Greyed: `{refusal}`.)

### On your own page

Hovering "Greenwash campaign":
> Each point: {d} Ducats, {r} ppm off your Blame for good. Nothing leaves the air.
> One a turn; public, no offence; a rival may answer with a Smear.

Hovering "Greenwash with {n} Influence for {d} Ducats":
> {p} ppm off your Blame at End Turn.
> Relations: {Faction} -{x}, {Faction} -{x}, {Faction} 0

(Greyed, the refusal is the first line.)

Custodians. Hovering "Carbon credits": as above.

Custodians. Hovering "Offer {n} ppm a turn":
> Stands from next turn until you set it again; nought refuses everyone.

(Greyed: `{refusal}`.)

The rulebook section repeats the Faction card's hovers (see "Setup screens").

---

## Tech Tree window

Hovering the Research race bar at the top:
> The Research race for {Tech}: Custodians {n}, Prospectors {n}, Arkwrights {n}, Archivists {n}[, carried over {n}]. {progress} of {cost}.
> Next pick: {Faction} ({p}%). [The {Faction} are funding the Archive this turn.]

The second line can also read "Next pick: {Faction}." (nothing contributed yet) or "Next pick: random among the tied."

Hovering a Tech's box:
> {Tech} (rung {r}, cost {c} Research)
> {effect}
> Needs: {Tech} and {Tech}   (or "Needs: nothing")

Hovering "Research Directive: {n}%":
> Share of your Research paid into the shared Tech from the next Income. Research kept back counts nothing toward the Research Lead.

Hovering the Directive slider after a setting was refused:
> {refusal}

---

## Victory window

Hovering a Faction's name: "Open this Faction's page."

Prospectors. Hovering "Venture Capital Fund: banking {n}% of Ducat income":
> The share of each turn's Ducat income that goes into the Fund at Income, before you can spend a coin of it, from the next Income until you set it again. Ducats got by selling are not income and never reach it.

Prospectors. Hovering the share slider after a setting was refused:
> {refusal}

Prospectors. Hovering "Withdraw from the Fund":
> {n} Ducats come back to the Stockpile at End Turn; a tenth is lost on the way out.

(Greyed: `{refusal}`.)

The Journal tab has no hovers.

---

## Climate Panel

Hovering the Temperature bar:
> The filled marker is the Temperature now, the hollow one what the CO2 Stock already commits the world to.
> Breaks (red), Sea Level thresholds and Antarctica's opening (along the foot):
> {+t}  {notch}[  - crossed]
> {+t}  {notch}[  - crossed]
> ...

Hovering "Population {n}":
> Each state's people emit {b} + {k} x its Industry Level per hundred million, halved by Green Consensus, times its controller's Emissions multiplier. The Custodians' Leapfrog lowers a state's own figure by {k} for good, never below {b}.

Hovering "War {n}" (when there was war):
> Last turn's Battles on Earth and in its orbit: {n} ppm per hit, Blame to whoever landed it; {n} per building burned when a three-turn Occupation takes a place, Blame to the taker.
> A Battle itself burns nothing; a Pacified place is taken whole; a neutral Army's hits are nobody's.

Hovering "Permafrost {n}" (after the Thaw):
> The permafrost has thawed. This much CO2 comes out of the ground every turn now, whatever anybody does. It is nobody's Blame, and it counts against a Stabilization run.

Hovering "Already locked in: ...":
> The Temperature the CO2 Stock as it stands will deliver once the lag has caught up. Nothing anybody builds or stops building takes it back.

Hovering "Last turn to act: {n}" (or "Cuts alone no longer avoid Collapse." / "On this path Collapse is not reached."):
> The latest turn on which cutting net Emissions to zero from that turn onward still keeps the Temperature under the Collapse Line by the last turn, counting every Break the world would cross on the way.

Hovering "Blame: the CO2 each Faction is answerable for":
> Blame: CO2 this Faction's places emitted, less what it removed (Scrubbers, Nature Reserves, the Custodians' Sink Directive).
> Above a quarter share: Influence thresholds rise up to +50% where it doesn't hold, and every rival likes it a point less per step.

Hovering a Faction's "{Faction}: answerable for {n} ppm" line:
> Emitted {n}, removed {n} in credit
> {moved parts, when any}
> Share {s}, thresholds x{m}

---

## Pop-ups

### Battle

Hovering a unit tile (its glyph and pips):
> {unit}: took {n} this round, damage {d} of {hp}, {engaged / left the Battle / destroyed}.

### A card that asks

Hovering a greyed answer button:
> You cannot take this side of the card: {shortfall}. Refusing is your only move this turn.
> You cannot pay what this side of the card asks. Refusing is your only move this turn.
> You cannot choose this: {reason}.   (the third answer)

### Capital building

Hovering "Later":
> Unchosen at End Turn: the {building}.

### Offers

Hovering the "Or counter: give ..." row (a Trade in goods):
> Once. They answer at once; a seat short of the good it asked pays up to a quarter over fair value. A refused counter is a refusal.

### The Report

Hovering the headline, when it points somewhere:
> Go there

Hovering a Report line that points somewhere:
> Go there

Hovering an Accord line in its last turn:
> Renew it in the Factions window

### The end-of-game Chronicle

Hovering the table's column headings:
- Materials: Materials made over the whole game, before upkeep. In brackets: the Stockpile at the end.
- Fuel: Fuel made over the whole game. In brackets: the Stockpile at the end.
- Fissionables: Fissionables made over the whole game. In brackets: the Stockpile at the end.
- Energy: Energy made over the whole game, before upkeep. In brackets: the Stockpile at the end.
- Ducats: Ducats made over the whole game, sales apart. In brackets: the Stockpile at the end.
- Off Earth: Colonists living off Earth: in Colonies away from Earth and on stations over it. Antarctica is on Earth.
- Regions: Nation States the Faction directed at the end: those it controlled, and those it occupied.
- Colonies: Colonies and stations the Faction directed at the end, the two counted together.
- Research: Research the Faction produced over the whole game, not what it holds now.
- Blame: The ppm of CO2 the Faction is answerable for at the end -- its ledger, and its share of the table's Blame.

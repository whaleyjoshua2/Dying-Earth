# Dying Earth — version 0.05, the climate version: four Factions, Unrest, Blame, Breaks, Scrubbers, saves

**Status:** the destination of the map [Map: version 0.05, the climate version: four Factions, Unrest, Blame, Tipping Points, Scrubbers, saves](https://github.com/whaleyjoshua2/Dying-Earth/issues/49). Every change here was decided on one of that map's tickets after the designer played version 0.04; each section names its ticket, and the ticket's resolution comment is the authority if this document and it ever disagree. Everything not amended here stands as written in [`first-playable.md`](first-playable.md), [`version-0.02.md`](version-0.02.md), [`version-0.03.md`](version-0.03.md) and [`version-0.04.md`](version-0.04.md).

**Numbers** live in `assets/data/`, one file per table; this document names the file beside each change.

**A rule marked "builder's call"** was settled in the build rather than on the ticket. Every one of them is gathered again in section 15 for the designer to veto.

---

## 1. Four Factions in every game

Decided on [Four Factions in every game](https://github.com/whaleyjoshua2/Dying-Earth/issues/50). Amends sections 14, 15, 16 and 10 of the First Playable, section 1 of version 0.02 and section 7 of version 0.04.

- **Four seats, always.** Every game seats the Custodians, the Prospectors, the Arkwrights and the Archivists. The player picks one Faction and a start continent; the other three are AI, each playing its own Victory Condition. Seat 0 is the player's; the other three follow in card order. Version 0.02's rule that Factions must be unique stands, and with four cards and four seats there is no pairing left to refuse.
- **The AI's starts spread.** After the player's pick, each AI in turn takes the free Nation State not adjacent to any taken one with the highest Industry Level, ties by population; if none is free, the highest Industry Level left.
- **Earth has five orbital slots** (`bodies.toml`, `orbital_slots`, `stations`): the **ISS** (Custodians), **Tiangong** (Prospectors), **Axiom** (Archivists), **Orbital Reef** and **Starlab** free. Skylab and Mir are gone. **The Arkwrights start with no station** (`factions.toml`, no `start_station`); their compensation is on the card in section 2.
- **Influence is unchanged.** A same-turn tie among challengers goes to the higher Standing; an exact tie leaves the place where it was.
- **A Battle is a melee.** Every Faction present at a place is hostile to every other. Each round a party's chance to hit is its **share of the total strength present** (two parties reduce exactly to the old a/(a+d)); its hits are spread across the enemy parties in proportion to their strength; a disengaging unit is pursued by the highest Pursuit among its enemies. An Attack pulls every other Faction's Ships at the Body into one melee, an Intercept fights every arriving enemy stack, and an attacking Army fights every other Faction's Armies at the place. The Battle Report carries a line per party.
- **Orbital Control** is held by the one seat with a Frigate or Battleship at the Body and no other seat's warship still engaged; two engaged fleets lock the orbit for everyone.
- **Winning among four.** One seat meeting its condition wins; several in the same End phase, the larger margin over its own bar, an exact tie a draw; then Collapse; on the last turn every seat is ranked by score, then Colonists off Earth, then Colonies held.
- **The AI pursues its own condition.** The denial multiplier is gone (`ai.toml`, `denial` removed): no AI spends a multiplier holding a rival back. Threat is any other seat's stack present or inbound; Influence and attack targets are any rival's, nearest first; odds are reckoned against everyone present. Weights, pace and Tech picks are keyed per Faction.
- **Ties are random by seed** wherever seat order used to decide (orbital and Colony Slot contests, the Body tiebreak), and the same on every replay of a seed. The **Research Lead** tie goes to the seat that has picked least recently.
- **Colours and the window.** Arkwrights violet, Archivists pale silver-blue. Four cards at New Game, the three not chosen "played by the computer"; four-way Standings chips on every card; stack markers at four fixed angles round a Body; four Earth tints; an attack preview naming every Faction present with its strength; a Victory row and a game-over row per seat.
- **A Faction's Victory parts live on its card** (`factions.toml`, `victory_first`; `victory_second` arrives in section 2). `victory.toml` keeps 500 Extraction, a 3-turn Stabilization run and 12 Off-world Presence as the reference figures the two older cards quote.
- **The climate step's first re-sweep.** The clock is decided by who holds Asia: a Prospector there at the old step of 90 collapses the world on turn 13 in every seed, and a Custodian there never collapses above 90. No step reproduces version 0.02's target for every seating, so **`ppm_step` goes 90 to 120** (`climate.toml`), the Natural Sink unchanged at 6.0, which keeps every seating hot at the end. The step is re-swept again in sections 3 and 13.

**Builder's calls.** Solar stack labels stack vertically above a Body in seat order; the orbit band on a Body Surface Map is one line per Faction; the Report carries the seating line every turn; `tints:1`, `battle:1`, `victory:1` and `stack:1` are opt-in `shot:` aids; Orbital Reef and Starlab are placeholder names.

## 2. The Arkwrights and the Archivists

Decided on [The Arkwrights and the Archivists](https://github.com/whaleyjoshua2/Dying-Earth/issues/51). Amends sections 14.1, 14.2, 14.3 and 15 of the First Playable and section 1 above, which seated the two on provisional cards. Every figure lives in `factions.toml` except the Archive's, which is in `modules.toml`.

### The Arkwrights, violet, no start station

- **Multipliers:** output x1.0, Emissions x1.0, Research x1.0, **Influence x0.8**. No strength row: combat stays symmetric. **Habitat capacity x1.5** (a Habitat holds 6, 9 with Expanded Habitats) and **transit Fuel x0.75**.
- **Steerage**, the signature rule: their Colony Ships carry **twice the Colonists** (8; 12 with Expanded Habitats), a lift from a Launch Site costs the Nation State **twice the population** per Colonist, and a Colony Ship costs them **20 Materials**.
- **Starting without a station:** a **Space Station costs them half** (`station_materials_multiplier` 0.5, so 20 Materials) and a **Colony Module three quarters** (`module_materials_multiplier` 0.75).
- **Diaspora**, the Victory Condition: **30 Colonists living off Earth**, on Colonies spread over **at least three Bodies with at least 4 Colonists each** (the Moon, Mars, Phobos, Deimos; Antarctica and stations over Earth count toward neither part). Off-world Presence is subsumed into it.

### The Archivists, pale silver-blue, Axiom

- **Multipliers:** output **x0.8**, Emissions **x0.9**, Research **x1.6**, Influence x1.0.
- **A Project** is a new kind of thing: a staged construction paid in both Research and Materials. **The Archive** is the first: a Module only the Archivists build, at **one Colony off Earth**, **one per Faction**, in **four stages of 30 Materials and 20 Research, two turns each** (`modules.toml`, `[archive]`: `stages = 4`, `research_per_stage = 20`; the Module row carries `materials = 30`, `energy_upkeep = 12`). **One stage at a time**: a second may not be ordered while one is building (builder's call). Complete, it costs **12 Energy a turn** and goes offline under the Energy shortfall rule like any Module.
- **Fund the Archive:** an Orders-phase order that sends the turn's Research into the **Archive fund** instead of the shared Tech, one for one, contributing nothing to the Research Lead that turn. The whole table sees the shared bar slow.
- **Provisional Findings**, the signature rule: they already have **half the effect of the Tech under research** (a multiplier x reads 1 + (x - 1) / 2, an addition reads half rounded down, immunities do not carry), **in force only in a turn after one in which their Research went to the shared Tech**. A turn of funding the Archive switches it off for the turn after.
- **Victory:** the Archive complete and online in an End phase, plus **12 of their Colonists living at its Colony**, uploaded at the win.
- **Losing it:** Occupation of the Colony takes the Archive offline; a control transfer **destroys** it, and the fund is kept.

### The second Victory part

The second half of a Victory Condition is now a Faction figure like the first (`factions.toml`, `victory_second`): **Off-world Presence 12** for the Custodians and the Prospectors, **three Bodies with 4 Colonists each** for the Arkwrights, **12 Colonists at the Archive** for the Archivists.

**Builder's calls.** A Colony Ship gains +2 capacity with Expanded Habitats for **every** Faction (the 0.04 spec said so; the code had never done it). The station and Module discounts are card multipliers rather than absolute figures, and a Module bought for Ducats follows the discount. The Archivists' first Victory part is Archive stages complete, 4 of 4, shown as "complete but not running" when the Energy is short. Funding takes this turn's Research back out of the shared Tech at Orders, because Income runs first. Closed-Loop Colonies halves the Archive's Energy like any Module's. Provisional Findings is on at turn 1 and applies to yields, Lab output, Habitat and Colony Ship capacity, Ship strength, transit Fuel and Influence thresholds, and not to Event cards or the global climate lines.

## 3. Twelve Nation States

Decided on [Twelve Nation States](https://github.com/whaleyjoshua2/Dying-Earth/issues/63), added to the map by the designer. Amends section 4.2 of the First Playable and section 2 of version 0.02.

Two new Factions need more places to go, and eight states made one of them — Asia, a third of the world's people — decide every game. **Asia** becomes **East Asia** (China, Mongolia, the Koreas, Japan, Taiwan, Central Asia), **South Asia** (India, Pakistan, Bangladesh, Nepal, Sri Lanka, Afghanistan) and **South-East Asia** (Myanmar round to the Philippines and most of Indonesia); **Africa** splits at the Sahara into **North Africa** and **Sub-Saharan Africa**; **Central America and the Caribbean** is cut out of **North America**. **Antarctica stays off the list**: it is Earth's three Colony Slots, as version 0.04 made it.

**Every split shares out its parent's real-world figures** rather than inventing new ones, as ticket #26 did for Russia and the Middle East. Asia's 30 GDP becomes 23 + 4 + 3 and its Influence value 7 becomes 4 + 2 + 1; Africa's 3 and 2 become 2 + 1 and 1 + 1; North America's 25 and 8 become 23 + 2 and 7 + 1. **The world's totals are unchanged**: 34 Influence and about 7.9 billion people, pinned by `twelve_nation_states_share_out_the_eight_they_came_from`. **Central America is the map's first Size 1 state.**

The twelve cards (`nation_states.toml`), in the engine's order. Population is in hundreds of millions; GDP is the 2023 world share in tenths.

| State | Pop | Ind | Size | Coast | Lean | Base Em | Edu | Infl | GDP | Neighbours |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Sub-Saharan Africa | 11.4 | 1 | 3 | 1 | Materials | 0.3 | 0.7 | 1 | 2 | North Africa, the Middle East |
| North Africa | 2.6 | 1 | 2 | 1 | Fuel | 0.4 | 0.85 | 1 | 1 | Sub-Saharan Africa, Europe, the Middle East |
| East Asia | 16.4 | 3 | 3 | 2 | Materials | 0.5 | 1.1 | 4 | 23 | Russia, South Asia, South-East Asia |
| South Asia | 19.4 | 2 | 2 | 2 | Materials | 0.4 | 0.75 | 2 | 4 | East Asia, South-East Asia, the Middle East |
| South-East Asia | 6.8 | 2 | 2 | 2 | Fuel | 0.3 | 0.95 | 1 | 3 | East Asia, South Asia, Australia and Oceania |
| Australia and Oceania | 0.5 | 2 | 2 | 2 | Energy | 0.2 | 1.35 | 2 | 2 | South-East Asia |
| Europe | 6.0 | 3 | 2 | 1 | Energy | 0.3 | 1.45 | 5 | 20 | North America, North Africa, Russia, the Middle East |
| North America | 3.8 | 3 | 3 | 1 | Fuel | 0.4 | 1.5 | 7 | 23 | Europe, Russia, Central America |
| Central America and the Caribbean | 2.2 | 1 | 1 | 2 | Materials | 0.3 | 1.0 | 1 | 2 | North America, South America |
| South America | 4.5 | 1 | 3 | 1 | Fuel | 0.2 | 1.1 | 2 | 5 | Central America |
| Russia | 1.5 | 2 | 3 | 1 | Fuel | 0.4 | 1.35 | 4 | 3 | Europe, East Asia, North America |
| The Middle East | 3.5 | 2 | 2 | 1 | Fuel | 0.5 | 1.0 | 4 | 5 | Europe, North Africa, Sub-Saharan Africa, South Asia |

Start Facilities follow section 3 of version 0.02 unchanged: as many as the Industry Level, in the order the Resource Lean sets. Every state starts at Unrest 0.

**The globe mask** is derived from longitude and latitude rules in `examples/prep_assets.rs`, which now takes `--mask-only` so the borders can be redrawn from the `earth.png` already in the tree. **Mask values were appended** (10 North Africa, 11 South Asia, 12 South-East Asia, 13 Central America) rather than renumbered, so every old value still means what it meant and the map can be split again the same way. North Africa parts from Sub-Saharan Africa at 18 N, the Himalaya line slopes from 37 N at Iran's border to 29 N at the Burmese one, South-East Asia sits below 24 N (22 N past Hong Kong, so Taiwan stays with East Asia), and Central America runs below the United States border from San Diego to Brownsville, taking Cuba, Hispaniola and the Bahamas. **They are a board, not an atlas.**

**The climate step's second re-sweep.** Twelve states raise total Industry Level from 17 to 23 and state industry Emissions from 6.0 to 8.4 a turn, with six more start Facilities. At 120 every seating collapsed, including the Custodian-in-East-Asia game that never collapsed at all on eight states. **Chosen: `ppm_step = 150`** (`climate.toml`), which keeps every seating hot to the end.

## 4. Unrest, Occupation and refugees

Decided on [Unrest, Occupation and refugees](https://github.com/whaleyjoshua2/Dying-Earth/issues/52) and amended on the same ticket after twenty seeds. Amends sections 4.2, 5.1, 8.5 and 11.3 of the First Playable. Every number that moves Unrest lives in `unrest.toml`; a state's starting figure is on its card in `nation_states.toml`.

**Unrest** is a figure from **0 to 10** on every Nation State's card, neutral states included (capped at **9** so they stay in play; a Faction taking a state by Influence inherits the figure). It moves in **halves**, and the card and the Earth Map print the fraction.

**It rises** by **1.5** in any Climate phase the state's population fell, **2.0** if it fell by more than 1%; **1** per build slot lost to Sea Level; **1.5** when a Heatwave, Wildfire or Storm Surge lands there; **1** per Facility mothballed and **2** per decommissioned (section 6); **1** per **0.5** population of refugees arriving, at most **2** in a turn; **3** when an Occupation begins and **1** for each turn it lasts, carried over when control transfers by force; and **3** flat for the Unrest Event card.

**It falls** by **1.5 every turn**, by subtraction, whatever else happened — a rise and the fall net out. The one turn a state goes without it is the turn it **changed hands**: a population with a fresh grievance is not calmed by the passing of a month. `changed_hands` is set wherever control actually moves, and an Occupation's turn counter ticking is not a change. It falls a further **1** per **Relief** order (**10 Ducats**, on a state you direct, any number of times a turn), **1** a turn while a **Constabulary** stands and is online there, and **1** a turn while a **Scrubber** does (section 6).

**Damping.** With two of the four green Techs complete (Clean Propellant, Clean Power, Clean Manufacturing, Green Consensus) every climate-source **and refugee** rise is **0.5** smaller; with all four, **1.0**. A Constabulary there damps the same rises by **0.5** more. Nothing falls below zero, and nothing damps Occupation, a mothball, a decommission or the Unrest card.

**The Constabulary** is a new Facility (`facilities.toml`): **25 Materials**, one turn, **2 Energy** upkeep, no Emissions, one build slot, **one per state**, any Faction.

**Thresholds.** At **4** the Standing Army stops replenishing. At **7** every Facility there produces and emits at half. At **10** a controlled state **throws its controller off**: neutral at the Resolution, every Standing kept, its Armies its own, Unrest reset to **5**. While Unrest is 4 or more the occupier's Pacification gain is halved (`pacification_divisor` 3 becomes `pacification_divisor_unrest` 6); the three-turn cap stands. A neutral state at 7 or more will not develop itself (section 5).

**Refugees.** When a state's population falls to the heat, **half** of what it lost moves to its neighbours instead of vanishing, split in proportion to their Industry Level, bringing its Population Emissions and its Research weight with it. When a Sea Level threshold fires for a state it loses **5% of its people per point of Coastal Exposure**, half of which moves the same way. **Resettle** (**20 Ducats**, once a turn per Faction) sends every flow leaving that Faction's states to one state of its choosing and raises its Standing there by **5**.

**Shown** on the state card in words, on the Earth Map label from 4 (amber to 6, red from 7), and in the Report at every crossing and every arrival.

**Why the fall changed.** As first decided the fall was 1 and landed only in a turn nothing raised Unrest, the climate rises were doubles of the figures above, and the refugee cap was 3. Twenty seeds measured that shape as a **ratchet rather than a pressure**: from about turn 8 the population falls worldwide every turn, so something raised Unrest every turn and the fall never landed. Median peak Unrest was 10 in every seed, and across twenty games states threw a controller off **776 and 810 times** — about forty a game. After the amendment the same twenty seeds give **10 and 8** throw-offs, the ceiling still reached in a median game, and the climate clock back to the shape it had.

**Builder's calls.** Relief and Resettle go to whoever directs a state, so an occupier may pay. The falls run before the throw-off check, so a state at 10 can be bought back in the same Resolution. Throw-off applies to Controlled states and not to Occupied ones. A Sea Level threshold displaces people each time one fires for a state. The 7 threshold halves output, Research and Emissions but not an Embassy's Allotment. Refugee Unrest is charged once a turn from the whole inflow. A foreign Army in a state that throws its controller off becomes the state's own.

## 5. Blame and neutral development

Decided on [Blame and neutral development](https://github.com/whaleyjoshua2/Dying-Earth/issues/53). Amends sections 8.2, 8.3 and 11.2 of the First Playable and section 1 of version 0.03.

**Blame** is the CO2 each Faction is answerable for over the whole game. At every Climate phase the Emissions the Climate Panel attributes to the sources a Faction controls — a controlled state's industry line, its Facilities, its Antarctic Modules, its population line, and the Faction's own launches — are added to its **emitted** total, and whatever CO2 it removed that turn to its **removed** total. Blame is the difference, **floored at zero**; a Faction that removes more carries a **credit**. What no Faction controls is nobody's: a neutral state's industry and people, every Event card, and every Break.

**Its bite.** A Faction's **share** of the four Factions' Blame, above a fair quarter, multiplies its Influence thresholds on every Nation State it does **not** hold by **1 + (share - 0.25)**, floored at **x1.0** and capped at **x1.5** (`influence.toml`, `[blame]`: `fair_share = 0.25`, `cap = 1.5`). Never on a Colony or a Space Station, never on the challenge margin, never on Standing decay, never on Pacification.

**Shown** on every state card the Faction does not hold ("your threshold here is 45, not 40"), omitted entirely at x1.00; as a four-bar strip under the Victory rows; and as a Blame section on the Climate Panel giving each Faction's emitted, removed, Blame, share and multiplier.

**Neutral Development** is what a Nation State nobody holds does for itself. Every **nine** turns of unbroken neutrality it raises its own Industry Level by one, up to **4**, and brings the first idle Facility in its list online (`nation_states.toml`, `[development]`: `turns = 9`, `max_level = 4`, `stops_at_temperature = 2.5`). The clock is the state's own and runs from the turn it was last freed, so a state taken and then thrown off counts nine fresh turns; **the opening clocks are staggered by the seed** across the first period, so the twelve states do not all step on the same turn. A world at **+2.5 C** or above develops nothing, and neither does a state whose Unrest has reached **7** (`unrest.toml`, `no_development_at`), nor one that is Controlled or Occupied.

**A woken Facility is self-run** (builder's call, after the first build): while the state stays neutral it emits at x1.0, with the worldwide Clean Techs and the Unrest-7 halving applied, to **nobody's Blame**, and it makes nothing for anyone. Without this the clause could never fire, because a neutral state's start Facilities are already online and already idle in the sense the Climate phase means.

**The nine-turn clock is the designer's answer to a measurement.** At six turns both seatings collapsed 20 of 20 with 14 to 22 developments a game. At nine a state steps at most twice a game, and the seatings collapse 20 of 20 at a median turn 21 to 22 with 6 to 14 developments.

**Builder's calls.** Emissions follow whoever directs a state, so an occupier wears an Occupied state's figure and its own threshold there is raised, while Pacification uses the plain threshold. Event-card Emissions are nobody's Blame. The multiplier is applied and then floored. A blocked state keeps its place in the queue and develops the moment the block lifts. The development clock is written from the turn after a control change.

## 6. Emissions you can lower

Decided on [Emissions you can lower](https://github.com/whaleyjoshua2/Dying-Earth/issues/54). Amends sections 5.1, 5.2, 7.3, 11.2, 11.3 and 14.2 of the First Playable and section 4 of version 0.03. The gap these tools close: world Emissions run 27 to 36 ppm a turn against a Natural Sink of 6, so Stabilization was out of reach by twenty-odd ppm.

- **Mothball, Restart and Decommission**, on any Facility in a Nation State you direct and any Module in a Colony you direct (`facilities.toml`, `[mothball]`). A **Mothball** is free and lands at this turn's Resolution: the building makes nothing, pays no Energy upkeep, emits nothing, counts as online for no rule — no lift from a mothballed Launch Site, no Ship from a mothballed Shipyard, no Allotment from a mothballed Embassy, no calm from a mothballed Constabulary — and **keeps its slot**. A **Restart** is **5 Materials** and a turn. A **Decommission** is a turn, **half the building's Materials back** rounded down, and the slot free. In a Nation State a mothball adds **1** Unrest and a decommission **2** (`unrest.toml`); in a Colony neither adds anything.
- **Population Emissions follow the Industry Level.** A state's people emit **0.04 + 0.03 x Industry Level** per hundred million (`climate.toml`, `population_emissions_base`, `population_emissions_per_level`), in place of the flat 0.1 every state used to pay: 0.07 in Sub-Saharan Africa, 0.13 in East Asia, and 8.03 for the world against the old 7.86. Green Consensus still halves the whole line and the Faction multiplier still applies.
- **Restoration is retired.** The order, its Ducat price, the `[restoration]` table, its AI weight and the Custodian card's text are gone.
- **The Scrubber** is the Custodians' signature Facility (`facilities.toml`): only they build it, only in a state they control, and it **takes no build slot** (`no_slot = true`). **30 Materials**, **two turns**, **4 Energy** upkeep, no Emissions. While it is online it enlarges the Natural Sink by **3.0 ppm** (`sink_per_turn`), takes **1** off its state's Unrest, and counts as **removal** for its controller's Blame. A state holds `clamp(round(population / 2), 2, 10)` of them (`[scrubber]`: `per_population = 2.0`, `min = 2`, `max = 10`), so Russia gets 2 and South Asia 10, and they are **destroyed outright if the state changes hands**.
- **Leapfrog** is the Custodians' other clause: **50 Ducats** (`factions.toml`, `[ducats] per_leapfrog`) on a state they control lowers that state's per-person coefficient by **0.03** for good, any number of times, never below the 0.04 base.
- **The Custodians' Allotment multiplier comes down from 1.3 to 1.25** to pay for it (`factions.toml`, `influence_multiplier`).
- **The Strip Permit** is the Prospectors' second clause, beside Cheap Industry (`nation_states.toml`, `[strip_permit]`): free, **once per Nation State ever**, **three turns** in which every Facility there produces **double**, and then that state's Baseline Emissions rise **0.2** and its Unrest **3**, for good.
- **Stabilization's bar is unchanged**: net Emissions, counting every Scrubber, below the Sink for three consecutive Climate phases.

**Builder's calls.** A Mothball lands at that turn's Resolution and a Restart or Decommission at the next. A Strip Permit's three turns are three doubled Incomes and its price falls due at the last. Leapfrog is refused when the figure already sits at the base. The Archive cannot be mothballed. Scrubbers survive the start of an Occupation and die when the controller actually changes. The AI decommissions only a mothballed Facility it cannot restart that holds a state's last slot. The Scrubber and Leapfrog buttons sit first in the Build section. Every "while online" rule — lifts, Shipyards, repair, Embassies — now reads "working", which a mothballed building is not.

## 7. Breaks, Committed Warming and the Last Turn

Decided on [Breaks, Committed Warming and the Last Turn](https://github.com/whaleyjoshua2/Dying-Earth/issues/55), on the research in `docs/research/climate-tipping-points.md` (branch `research/tipping-points`). Amends sections 11.1, 11.4, 11.5 and 13.2 of the First Playable and section 4 of version 0.02.

**The word is Break.** A Break is a Temperature at which a permanent change fires **once**, in the Climate phase, the first time the Temperature stands at or above it. Nothing undoes one, and the Report says it **happened** rather than that it is coming. The five live as `[[break]]` rows in `climate.toml` — id, name, Temperature, effect kind, figures, the "happened" sentence and the card line — so a Temperature can be moved, a figure changed or a sixth Break added without touching the engine. The list must rise in Temperature.

| Break | at | What it does |
| --- | --- | --- |
| **Coral Die-off** | +1.4 C | every Nation State at Coastal Exposure 2 takes **+1 Unrest** (damped like any climate rise) and loses **2%** of its people, half of whom flow on as refugees. No climate effect at all: the tutorial Break. |
| **Permafrost Thaw** | +1.6 C | **+4.0 ppm** of Emissions every Climate phase from then on, as its own **Permafrost** line on the Climate Panel. |
| **The Sink Weakens** | +2.0 C | the Natural Sink falls from **6.0 to 4.0** for good. |
| **Ice Sheets Committed** | +2.2 C | a Sea Level threshold's slot loss, displacement and Unrest lands at once on every state, **out of sequence**. The three scheduled thresholds still fire on their own turns, so a game that reaches +2.8 takes four. |
| **Amazon Dieback** | +2.6 C | a one-off **+20 ppm** into the CO2 Stock and **South America's Baseline Emissions up 1.0** for good. |

**Whose carbon a Break is.** The world's: **nobody's Blame** and **never counted against a Stabilization run**, like an Event card. The weakened Sink *does* bear on Stabilization, because the Sink is the bar.

**The Event card that used to be called Permafrost Thaw is now the Methane Burst**, and its figure comes down from 3.0 to **2.5** (`events.toml`, `methane_emissions`), so the name is free for the Break.

**Two lines join the Climate Panel above the projection.** **Committed Warming** is the Temperature the CO2 Stock as it stands will deliver once the lag has caught up: "Committed: +2.1 C even if net Emissions stopped today". **The Last Turn** is the latest turn on which cutting net Emissions to zero from that turn onward still keeps the Temperature under the Collapse Line by the last turn, found by running the projection forward from every future turn in turn and firing every Break the run would cross in it. It says one of three things: "Last turn to act: 14", "Cuts alone no longer avoid Collapse.", or "On this path Collapse is not reached."

**And the panel gains a Temperature bar**, from +1.2 to the Collapse Line, notched across its whole height for every Break and along its foot for every Sea Level threshold and for **Antarctica's opening at +1.6 C** (`climate.toml`, `antarctica_opens_at`). A crossed notch is filled and one still ahead is thin and dim; the Temperature carries a filled marker and the committed Temperature a hollow one, with the warming between them shaded; and the line beneath names the next Break ahead.

**Builder's calls.** Each Break's "happened" sentence lives in the table beside its card line. Coral's population loss flows as refugees under the heat rule and charges only the Break's own Unrest. "Cutting net Emissions to zero" in the Last Turn projection holds gross Emissions at the Sink as it stands at the cut, so the Permafrost line, which cannot be cut, is what brings the Last Turn forward. Acting on turn k means turn k's Climate phase is the first at net zero. The "at this rate" headline projection is unchanged and ignores Breaks, so the two lines can disagree slightly. A notch counts as crossed when the Break has fired, and the Sink line in the Emissions list lags the bar by one turn on the turn the Sink Weakens fires, because the panel reports the Climate phase that has just run. Older spec documents are left naming the Permafrost Thaw card; this amendment carries the change.

## 8. Sea level and the ice

Decided on [Sea level and the ice](https://github.com/whaleyjoshua2/Dying-Earth/issues/56). Amends sections 4.2, 5.1, 7.3, 11.4 and 12.3 of the First Playable and section 5 of version 0.04.

- **Build slots** are now **Size + Industry Level + 3** (`nation_states.toml`, `base_slots = 3`), less the coastal slots the sea has taken.
- **Every slot is a Coastal Slot or an Inland Slot.** A state's **start** slots (Size + 3 + the Industry Level on its card) hold **three coastal slots per point of Coastal Exposure** (`coastal_per_exposure = 3`), never more than the start slots less one; the rest are inland, and **every slot a raise of the Industry Level adds is inland**. Only Central America and the Caribbean feels the cap: 1 + 3 + 1 = 5 start slots, 6 coastal wanted, capped at 4, leaving one inland. East Asia gets 6 coastal and 3 inland; every state at Coastal Exposure 1 gets 3 coastal.
- **The sea takes coastal slots and nothing else.** Each threshold — scheduled (`sea_level_thresholds = [1.8, 2.3, 2.8]`), a Storm Surge applied early, or the Ice Sheets Break — takes Coastal Exposure of them, and once a state's coastal slots are gone it loses no more. A Facility standing in a lost coastal slot is destroyed, **oldest first** (it was highest-upkeep first). The displacement and the Unrest a threshold brings are unchanged: they key on the threshold firing, not on the slots it managed to take, so a state with no coast left still loses its people and its calm. **Start Facilities take coastal slots first**, in the table's order; a new build fills an inland slot while one is free.
- **Coastal Engineering** is the thirteenth Tech (`techs.toml`): Industry **rung 2**, cost **25**, needing Efficient Grids, beside Clean Power. It unlocks one thing.
- **The Sea Wall** (`facilities.toml`): **35 Materials**, two turns, **1 Energy** upkeep, no Emissions, at most **one to a state**, always in a coastal slot (`coastal_only`). While it stands and is working, the state's **next Sea Level threshold of any kind takes no slots at all**, and the wall is destroyed absorbing it. A mothballed wall absorbs nothing.
- **Antarctica opens.** Earth's three Colony Slots cannot be founded until the Temperature has stood at or above **+1.6 C** in a Climate phase (`climate.toml`, `antarctica_opens_at`); once open they stay open however far the world cools back. Until then the Surface Map draws them under the ice with their opening Temperature, the Solar System Map's Earth line says so, and a Colony Ship ordered to found there is refused by name. Its yields are re-cut for what lies under the ice (`bodies.toml`): **Mine 1.75, Generator 0.75, Refinery 2.0, Habitat 1.0**. Its Colonists still count as on Earth and its Modules still emit.

**Builder's calls.** A Sea Wall stops the slot loss only; the threshold's refugees and Unrest still happen. Provisional Findings does not unlock the Sea Wall early. A lost coastal slot remembers what drowned, so the card can say "Factory, lost to the sea". The Report keeps the Temperature in the sea line. A build reserves its row at order time and falls to the other row if the sea takes it first, a Sea Wall to nothing. A threshold reads as near when it is within 0.2 C. The Tech Tree widens a branch to its busiest rung.

## 9. Named sites with real yields, and real launch windows

Decided on [Named sites with yields on the map, and launch windows](https://github.com/whaleyjoshua2/Dying-Earth/issues/57). Amends sections 4.1, 6, 9.1 and 9.4 of the First Playable and section 6 of version 0.04.

**Every Colony Slot draws its own four yields when the game starts.** A Body's card figures stop being what a Colony there gets and become what its slots draw from: each slot takes the Body's Mine, Generator, Refinery and Habitat yields times a factor from a **triangular distribution centred on 1.0 with limits 0.75 and 1.25** (`bodies.toml`, `slot_yield_spread = 0.25`), rounded to two decimals, drawn from the game's own seeded generator in Body order then slot order, so a seed always deals the same board. The four are drawn separately, so one slot can be the best Habitat site on a Body and a middling Mine. Every yield a Module reads is now its slot's: what a Mine makes, a Trade Post's Ducats, the Colonists a Habitat holds. A Space Station's Habitats take no Body yield and no slot's either; an Orbital Slot draws nothing. The AI reads the slot figures, picking the free slot whose own yields best serve the part of its Victory Condition it is furthest behind on, and ranking the Bodies by their best free slot.

**The game begins at 2030-01-01 00:00:00 UTC and a Turn is a calendar month** (`victory.toml`, `start_year = 2030`, `start_month = 1`, beside `turns = 24`). Turn 1 is January 2030, turn 24 is December 2031, and the top bar reads "Turn 7 / 24, July 2030". Every turn is sampled at the first instant of its month.

**The sky is the real one.** `assets/data/ephemeris.toml` carries the Keplerian elements of Earth (strictly the Earth-Moon barycentre) and Mars at J2000 with their rates per Julian century, from JPL's "Keplerian Elements for Approximate Positions of the Major Planets", the table valid 1800 to 2050. The engine propagates them to the turn's date, solves Kepler's equation and takes the heliocentric ecliptic longitude. On 2030-01-01 it puts **Earth at 100.182 degrees and Mars at 337.831**, against JPL Horizons' 100.1845 and 337.8203. The Solar System Map draws each Body on its existing ring at that longitude; the Moon still sits beside Earth and Phobos and Deimos beside Mars. The workings are in [`docs/research/earth-mars-ephemeris.md`](../research/earth-mars-ephemeris.md).

**And so the launch windows are real.** The **phase angle** is Mars's heliocentric longitude less Earth's; the **window offset** is the signed difference between it and the Hohmann departure angle of **+44 degrees** (`[transit] hohmann_angle`). A transit between the Earth system (Earth, the Moon) and the Mars system (Mars, Phobos, Deimos) no longer pays the fixed 4 turns and 20 Fuel on the card. It pays

    turns = ceil((259 + 1.5 x |offset|) / 30)      Fuel = card Fuel x (1 + |offset| / 120)

capped at **18 turns** (`days_at_window = 259.0`, `days_per_degree = 1.5`, `fuel_per_degree = 0.00833333`, `days_per_turn = 30.0`, `max_turns = 18`, `synodic_days = 779.9`). At the window that is **9 turns for the card's Fuel**; at the far side of the cycle 18 turns and two and a half times the Fuel. Efficient Transit and Steerage both apply **after** the window factor. A Solar Storm still stops every transit that turn. **Hops inside the Earth system and inside the Mars system are untouched**: the Moon is still 1 turn and 6 Fuel, Phobos to Deimos 1 and 1, on every turn of the game. Hovering Mars on the Solar System Map gives the window, the flight now and the flight at the window. The AI banks Fuel when the window is within two turns.

**The pacing consequence, plainly.** There is **exactly one Mars launch window in the game, turn 14 (February 2031), and the shortest flight there is is nine turns.** A Colony Ship that launches on the window lands on turn 23, with one turn left to build a Habitat and none to fill it; one that launches on turn 1 pays 17 turns and 47 Fuel and lands on turn 18; the next window is a synodic period away, 26 turns, well past the last turn. Measured over twenty seeds a seating with the rule off and on, that is not a tax on Mars — **it closes Mars**: first Mars-system Colony at a median turn 10 to 12 in 20 of 20 seeds with the rule off, **none in any seed** with it on. The designer's word is that **the turn count changes in a later version**; until then the Moon and Antarctica carry the off-world game.

**Builder's calls.** The return Hohmann angle is **+75 degrees** (Earth trailing) under the game's own phase convention, not the -75 the ticket wrote: worked out from the ephemeris, the magnitude is right and only the direction is turned round, and both legs must read the phase angle the same way or the return window lands two years out (`return_hohmann_angle = 75.0`). Each turn's sky is the first instant of its month. The AI's off-window discount is a quarter of the weight at the far side of the cycle and none at the window, lifted when the seat is behind on pace; with the window within two turns only the highest-scored crossing may spend Fuel. The crossing rule partitions the Bodies into the Earth system and the Mars system. The slot panel shows the slot's figures and the Body's beneath. The on-window flight is nine turns (259 days) where the real fast Type I transfer in 2031 is about seven.

**A note carried forward.** The Surface Map writes the four figures under every slot's name, filled or free, always, which is right while there is no other way to see them and wrong the moment the board grows lenses; and the Mars tooltip describes transits **from Earth**, which is no longer the whole truth once a Faction can launch from the Moon or start home from Mars. Both notes are on the code as well.

## 10. The turn as a story

Decided on [The turn as a story](https://github.com/whaleyjoshua2/Dying-Earth/issues/58). Amends sections 6, 17.3 and 17.5 of the First Playable.

- **The Report is a dated dispatch.** "Report, February 2031", then **one headline** chosen by a fixed order of severity over everything the last Resolution and Climate phase did: a Colony founded, a place changing controller, a Break or a Sea Level threshold, a Battle that cost a unit or moved Orbital Control, an Occupation, a Tech completed, an Event drawn, a build completed. Position in the turn counts for nothing. Turn 1 keeps its explanation as the headline.
- **Then four headings**, in this order and with the empty ones left out: **In space**, **On Earth**, **The climate**, **Your works**. Every Report line carries a **kind** and, where it is about somewhere, a **place**; the kind decides the heading, and the kinds that can happen either side of the sky follow their place, so a change of control at a Colony is In space and the same change in a Nation State is On Earth. **Every line with a place is a button that takes the player there**, the way the roster's rows do.
- **And what the rivals did, in words.** The scored list the AI chose from was never the player's business and now lives in the simulate log alone. The Report shows one paragraph per rival Faction, in seat order, built from the orders that Faction actually committed and what the Resolution made of them: builds begun and finished, Influence spent and where, Ships built, sent and arrived, Colonists lifted, Colonies founded, Armies moved, Stances set, Scrubbers, Sea Walls, Strip Permits, Leapfrogs, the Archive funded and raised. Repeated orders the board would show as one act are told as one ("spent 15 Influence on North America").
- **Moments.** Seven kinds of thing stop the turn before the Report for one sentence and one number: a Colony founded, a place changing hands, a Break or the sea rising, a Battle that cost a unit, a Tech completed, Antarctica opening, the Archive finished. **At most two a turn**, the most severe first by the same order the headline reads, the rest falling through to the dispatch. Each kind has a checkbox in a Moments corner at the foot of the Report, remembered for the session; `report.toml` holds the defaults.
- **The Research race.** The top bar's Research item carries a bar of the four Factions' contributions to the Tech under research, in Faction colours and in proportion, with the unfilled tail standing for what the Tech still needs. When a Tech completes its Moment names the Lead and the margin, shows the Tech Tree with the new box lit, and either gives the player the Pick buttons or says in one line what the AI picked and why.
- **Every sentence lives in `assets/data/report.toml`** — the dispatch's lines, the rivals' clauses and the Moments — as templates with named placeholders. The engine declares, per key, the placeholders it supplies, and loading the tables refuses the file if a key is missing, a key is there that nothing reads, or a template uses a placeholder the engine never hands it. The engine's own **log** lines are untouched and say the same thing in the same words, because the simulate run reads them.

**Builder's calls.** A Report line's place is a small type that can also name a Body. A Tech completion that is not the headline sits under Your works. Antarctica opening ranks with a Break, and the Archive completed with a build, for the Moment order. Lines with neither seat nor place default to On Earth. The `moment:` shot aid switches the other kinds off.

## 11. Spectator mode

Decided on [Spectator mode from the start screen](https://github.com/whaleyjoshua2/Dying-Earth/issues/64), added to the map by the designer. Amends section 17.6 of the First Playable and section 1 above.

- **A fifth button under the four Faction cards hands every seat to the computer.** There is no continent to choose: seat 0's start is the first pick of the same spreading rule the other three are dealt by, and the Custodians sit in it, so the table reads as it does in `simulate:`.
- **The spectator sees everything and orders nothing.** The side panel carries **all four Factions' boards**, each under its own heading in its own colour, with its Ships, Armies, Colonies and stations and Nation States. Whatever is clicked opens its card, readable in full, with every Faction's Standing on it and **no Orders buttons anywhere** — no build, no Influence, no Stance, no transit, no trading, no Tech pick. The Victory panel, the Climate Panel and the Tech Tree open as they do for a player.
- **End Turn advances one turn**, the engine running all four AI seats. An **Auto** box beside it runs a turn every **three seconds** until it is unticked; the clock stops while a Moment, the Report, the game-over popup, the Event popup or the Influence-unspent popup is up, and picks up again when it closes. Escape unticks it.
- **The dispatch** is the player's dispatch with two headings renamed for who is reading it: **Builds and works** carries every Faction's builds, lifts, repairs and funding rather than one seat's, and **What the Factions did** tells all four paragraphs, seat 0 among them. Moments are on by default with the same switches, and the game-over popup names the winner as it always did.

**Builder's calls.** The spectator's selected card sits in a left panel and the four rosters in the right, with compact rows; map labels are clipped between the panels in both modes. The top bar's Stockpile, Research and Influence stay seat 0's, said so on the seating line ("the figures below are the Custodians'."). The turn-1 Report still reads "You play the Custodians". A Faction with no orders gets "gave no orders this turn".

## 12. Saves

Decided on [Saves](https://github.com/whaleyjoshua2/Dying-Earth/issues/59), on the research in `docs/research/saving-game-state.md`. Replaces section 18 of the First Playable, which had no save and load.

- **A Save is a turn start written to a file.** The whole game goes into **readable RON** — the seed, the generator's own state, the Event Deck in its shuffled order, every board and the log — and the `Tables` do not, because the rules live in `assets/data/` and a save that carried its own copy could disagree with the executable it is loaded into.
- **A version stamp.** The file begins with a one-line header holding a `save_version` and the rules version it was written by. A file from another version is refused with a sentence naming both ("This save was written by version 0.04 of the rules; this is 0.05. It cannot be loaded."), and it is **never migrated**. A corrupt file is refused without a panic.
- **Where.** `%LOCALAPPDATA%\DyingEarth\data\saves`, written to a temporary file and renamed over the target, so a crash never leaves half a save where a good one used to be.
- **Autosaves** at the start of every **third** turn and at game over, keeping the **last three per game**.
- **Manual saves** from a **Save** button in the top bar, enabled only while no order is pending, unlimited, named by seed and turn with no typing; the button says why it is dead when it is. A spectated game saves the same way, with the spectator flag inside it.
- **The Load screen** on the title screen lists every save newest first with the Faction or "Spectating", the turn and month, the Temperature, the seed, the file's own time and its name, with **Load** and **Delete** beside each and a bottom bar holding **Back**, **Open saves folder** and the folder's full path.
- **Exactness.** A loaded game continues identically to the unsaved one, proven by a log-for-log continuity test. A turn-12 save of a played-out game is about 300 KB, two thirds of it the log.

**Builder's calls.** The folder is `%LOCALAPPDATA%\DyingEarth\data\saves` and not `...\DyingEarth\saves`, because the `directories` crate adds `data` on Windows; it is one line to drop. The file is one RON header line then the game as pretty RON. The RNG's 128-bit position needs RON's `integer128` feature. A mirror struct is saved rather than `Game` itself, exhaustively destructured so a forgotten field fails to compile. A manual save at the same seed and turn overwrites the earlier one, and autosaves rotate per seed. Save is also dead on the game-over screen. The `savedir:` argument keeps the picture and round-trip aids out of the player's real folder.

## 13. The build ticket: four fixes, the re-sweep and the balance

Decided on [Write the 0.05 amendments and build them](https://github.com/whaleyjoshua2/Dying-Earth/issues/60).

### The four small fixes

- **A Wildfire on a neutral Nation State charged nothing.** Since ticket #24 the line that charges a Wildfire's Emissions sat inside the *directed* branch of the Emissions computation, so a fire in a state nobody holds burned without any carbon. It is the world's card, so it now charges the cards line whether or not anyone directs the state, and stays nobody's Blame either way. Eight of the twelve states are neutral for most of a game.
- **The state card's "N now" for taking a held place ignored the challenge margin.** Ticket #41 put a margin of 10 on a held place; the card kept ticket #33's `threshold.max(controller + 1)`, printing 41 where the answer was 50. The whole computation is now `Game::influence_needed_for(seat, place)`, and the Resolution, the AI's Influence list and the card all read that one function.
- **The AI never built a Constabulary** — 0 in every game logged from #52 to #57 — because the victory-gap multiplier (x3 for most of a game) applies to producers and not to a building that fixes nothing economic. It now takes the **victory-gap multiplier from Unrest 5**, which is the only Unrest at which the candidate is offered at all, because a state at 7 halves every Facility's output *and* its Emissions there, so calming it does advance whatever the seat is behind on; and it takes the **opportunity multiplier at 9**, where one more turn would throw the seat off, exactly as Relief does. Constabularies built go from 0 to **11 to 100** a batch of twenty seeds.
- **The spectator's top bar wrapped its Temperature onto a second line at 1280 x 800.** The Stockpile row carried a "Custodians:" prefix a player's bar does not. Whose figures they are is said on the seating line instead, and "the computer plays all four" went with it, since four Faction names under "Spectating." say the same thing. The bar is now three rows of one line each.

### The climate clock, re-swept for four seats, twelve states and the Breaks

Section 7 put five Breaks on the curve and measured what they cost: **every seating collapsed 20 of 20, at a median turn 17 to 22**, where section 3 had chosen `ppm_step = 150` to keep every seating hot without making Collapse certain. This is the sweep that turns the knobs.

**The target, restated:** every seating stays hot to the end — a median end Temperature of +2.5 to +2.9 C where it does not collapse — and Collapse is a real threat but not a certainty: roughly half to three quarters of seeds collapsing, with a median Collapse turn of 19 or later. Measured over five seatings: Custodians, Prospectors and Arkwrights in East Asia, and Custodians and Archivists from Europe.

`engine/examples/sweep.rs` takes `--permafrost=` and `--sink-after=` beside `--sinks=` and `--steps=`, so the two Break figures that actually move the clock can be swept. **Thirty-two cells a seating** — step {150, 180, 210, 240} x sink {6, 8} x permafrost {4.0, 2.5} x `sink_after` {4.0, 5.0} — twenty seeds a cell, five seatings: **3,200 games**. The full tables are in the dev diary.

**Chosen: `ppm_step = 180`, the Natural Sink at 6.0, and both Break figures left where section 7 put them** (`climate.toml`). At 180 every one of the five seatings still ends at about **+3.0 C** — hot to the last turn, which is what tickets #46 and #53 chose the step for — and **every seating's median Collapse turn is 20 or later**. It is the cell closest to the target over all five seatings. It ties with (sink 6, step 180, `sink_after` 5.0), and the tie went to this one for a reason that is not about the numbers: **the two Break figures are drawn from real-world warming**, while `ppm_step` is the game's own abstract pacing knob and has been re-swept on every ticket that changed the board. Reaching for a researched figure to pace the game is the wrong knob to reach for first. The reasoning is in `climate.toml`'s comments beside the number.

**No cell meets the target for every seating, and at this one three of the five miss it.**

| seating | collapses | median Collapse turn | end temp | on target? |
| --- | --- | --- | --- | --- |
| Custodians in East Asia | 9/20 | 20 | +3.03 | one seed under the half the target wants |
| Prospectors in East Asia | 12/20 | 24 | +3.01 | **yes** |
| Arkwrights in East Asia | 20/20 | 20 | +3.03 | no: Collapse is certain |
| Custodians from Europe | 18/20 | 23 | +3.02 | no: above three quarters |
| Archivists from Europe | 20/20 | 20 | +3.04 | no: Collapse is certain |

The two seatings that collapse in every seed are the two whose seat 0 spends its Materials on something other than Earth's economy — Colony Ships and Habitats, the Archive — so the board they leave behind is run by three AIs who build industry and little else. They do not come off 20 of 20 until step **210**, and at 210 the Prospector and Custodian boards fall to 0 or 1 Collapse in twenty and finish at +2.8. There is no step between the two. **That is a Faction-economy finding, not a climate one.**

### The four-way balance, at the chosen numbers

`sim -- 1 --count=20 --player=<faction>` for each Faction in seat 0 starting in East Asia, and `sweep -- 20 --player=<faction> --start=europe --sinks=6 --steps=180 --permafrost=4.0 --sink-after=4.0 --balance` for each starting in Europe. Twenty seeds each, eight batches, **160 games**. Nothing was re-tuned for this table beyond the climate cell above.

| seat 0 | wins | collapses | median Collapse turn | median first Colony | Colonists off Earth at the end | Condition met outright |
| --- | --- | --- | --- | --- | --- | --- |
| Custodians in East Asia | Prospectors 11 (seat 1) | 9/20 | 20 | 9 | 12 | Prospectors, 11 seeds |
| Prospectors in East Asia | Prospectors 8 | 12/20 | 24 | 7 | 16 | Prospectors, 2 seeds |
| Arkwrights in East Asia | none | 20/20 | 20 | 7 | 16 | none |
| Archivists in East Asia | none | 20/20 | 21 | 7 | 16 | none |
| Custodians from Europe | Prospectors 2 (seat 1) | 18/20 | 23 | 7 | 15 | none |
| Prospectors from Europe | Prospectors 12 | 8/20 | 20 | 9 | 12 | Prospectors, 12 seeds |
| Arkwrights from Europe | none | 20/20 | 20 | 7 | 17 | none |
| Archivists from Europe | none | 20/20 | 20 | 7 | 15 | none |

| seat 0 | Scrubbers | Leapfrogs | Constabularies | Sea Walls | Techs (median) | highest rung |
| --- | --- | --- | --- | --- | --- | --- |
| Custodians in East Asia | 40 | 0 | 34 | **0** | 1 | 1 |
| Prospectors in East Asia | 192 | 227 | 68 | **0** | 1 | 1 |
| Arkwrights in East Asia | 107 | 119 | 49 | **0** | 1 | 1 |
| Archivists in East Asia | 118 | 136 | 59 | **0** | 2 | 2 |
| Custodians from Europe | 163 | 224 | 76 | **0** | 1 | 1 |
| Prospectors from Europe | 40 | 0 | 16 | **0** | 1 | 1 |
| Arkwrights from Europe | 52 | 60 | 100 | **0** | 1 | 1 |
| Archivists from Europe | 46 | 0 | 11 | **0** | 8 | 2 |

All five Breaks fire in every seed of all eight batches: Coral Die-off at a median turn 4, Permafrost Thaw 6 to 7, The Sink Weakens 10 to 11, Ice Sheets Committed 11 to 12, Amazon Dieback 15 to 18.

**The headline: one Faction wins this game and it is the Prospectors.** Across 160 games they take **33 wins** and every other Faction takes none — not one win for the Custodians, the Arkwrights or the Archivists, in any seating, in their own seat or anyone else's. They are also the only Faction that has ever met a Victory Condition outright: Extraction Total in **25 of the 160 games**, and **Stabilization, Diaspora and the Archive in none**. The reason is not subtle. Extraction Total is a running sum of what a Faction digs out of a dying world, and everything the world does makes a Faction dig faster rather than slower; the other three ask the Faction to fix something, get somewhere, or finish something, on a clock that runs out at turn 17 to 24. Every finding this report turned up is in section 15.

## 14. Acceptance for 0.05

- `cargo build --release`; **clippy clean** across every target under `-- -D warnings`.
- **164 formula tests** and **6 save tests**, every new rule seen red first.
- `shot:` pictures of everything this version added, taken headlessly with the window off-screen, looked at, and in the dev diary (`docs/dev-diary/2026-09-09-version-0.05/`): the four Faction cards, the four-way Standings and Report and Battle, the Archive on a Colony card, the twelve-state board and its mask, the Unrest card and the globe's Unrest labels, the Blame line, strip and panel section, a Custodian card with Scrubbers and a Prospector card under a Strip Permit, the Climate Panel with the Temperature bar and its Break notches, the coastal and inland rows with a Sea Wall, Antarctica shut and open, Mars's slot yields and the real sky at two dates, the dispatch and two Moments and the Research race bar, the spectator's board and Report, and the Load screen and the Save button.
- **Twenty seeds of `simulate` per seating after every ticket**, in the diary, reported and not re-tuned; and at the close, eight batches of twenty seeds reporting the four-way win split and the Collapse rate.
- The playtest kit in `dist/dying-earth-0.05/` and `dist/dying-earth-0.05-playtest.zip`.

## 15. Open for the designer

Recorded on the map and the tickets, not decided here.

**The balance**

1. **One Faction wins, and it is the Prospectors: 33 of 160 games, and nobody else takes one.** They are the only Faction ever to meet a Victory Condition outright (Extraction Total, 25 of 160). **Stabilization, Diaspora and the Archive have never been completed in any game of this version.**
2. **The Custodians' own Victory Condition is unreachable, and the report says so with a zero.** The longest Stabilization run any seat held at any point in any of the eighty East Asia games is **0 turns** — median 0, max 0, all four seats, all four batches. The bar is the world's net counted Emissions under the Sink, and the world's net is +44 to +51 ppm at turn 12 and still +18 to +32 at the end. No amount of Scrubbers by one seat gets there while the other three build industry. This is the clearest single finding in the table.
3. **The one-state Custodian economy in East Asia.** That Custodian ends with **4 to 6 buildings** where the Prospectors beside it end with 21 to 30; its Income reads `+6 Materials, +3 Fuel` from turn 1 to the Collapse and its Energy hovers between 1 and 5; it never gets a second place to stand on, and Leapfrog is never even offered to it because 50 Ducats are never within three turns of its income. The same Faction's AI in seat 1 of another batch clears the gate easily, and the Custodians starting from Europe Leapfrog 224 times. **Worth a ticket of its own**, because it is also a large part of why the Custodians take 0 wins in 160 games.
4. **The Archivists' economy.** At output x0.8 and one Nation State they make about 4 Materials a turn; across sixty games on ticket #51 no Archive stage was ever raised and the fund filled to 80 with nowhere to spend it. Whether the card, the stage costs or the start changes is a balance call.
5. **The step at 180 suits two boards of five.** The Arkwrights in East Asia and the Archivists from Europe still collapse 20 of 20, and the Custodians from Europe 18 of 20; they come off it only at 210, where the other boards finish comfortable. The designer may want the step here and **the Arkwright and Archivist cards looked at instead**. One line in `climate.toml` either way.

**The clock and the reach**

6. **Mars is unreachable in 24 turns under the real windows.** One window, turn 14, and a nine-turn flight, in a game whose median Collapse is turn 20. No Colony in the Mars system is founded in any seed of seven of the eight batches. **The designer's word is that the turn count changes in a later version.** The other knobs are `days_at_window`, `days_per_degree` and `fuel_per_degree` in `ephemeris.toml`.
7. **The off-Earth game is the Moon and nothing else.** Every other Colony founded in these 160 games is a lunar one, at a median turn 7 to 9, with Antarctica taking 0, 1, 7 and 14 over the four East Asia batches. Colonists off Earth end at 12 to 17 for the whole table — and **the Arkwrights' Diaspora asks for 30 over three Bodies with 4 on each, and the Moon is one Body.**
8. **The Tech pace: the world never leaves rung 1.** A median of **one** Tech a game, and the highest rung reached is 1 in six of the eight batches. The two Archivist batches are the only ones that touch rung 2. The Research Lab output, the Tech costs or the Archivists' funding want a look.
9. **The Sea Wall has never been built, in any game, by any Faction, in any version**, because Coastal Engineering is on rung 2 and rung 2 never arrives. Meanwhile **the sea takes all 49 coastal slots in the world** and drowns 27 to 29 Facilities in the median game. The defence exists, is pinned by a formula test, and has never once been reachable in play.
10. **Unrest is still a ratchet at the top.** The Constabulary now fires (11 to 100 a batch against 0), but the median peak Unrest is still 10.0 in every East Asia batch, states threw a controller off 0 to 75 times a batch, and 286 to 297 population a batch still moves as refugees. What the fix bought is that a seat holding a state at 7 now has an answer it will reach for.
11. **What else never fires.** **Leapfrog** is 0 in three of the eight batches and 60 to 227 in the other five. The **Strip Permit** is 0 in the Custodian-in-East-Asia batch and 32 to 42 in the other three East Asia batches: a Prospector that is never behind its Extraction pace never asks for one.
12. **Neutral development is large and it decides boards.** At six turns it took the one liveable board from 1 Collapse in 20 to 20 of 20; a control run with it switched off returned that board exactly. The designer set the clock to nine. `development_turns`, `development_max_level` and the +2.5 C stop are the three numbers.
13. **A neutral state's Facilities.** The Climate phase treats a Facility nobody directs as idle, so "brings the first idle Facility online" found nothing to wake until development was given a self-run Facility of its own. **If the intent was that a neutral state's start Facilities should stand idle until the state develops itself, that is a change to how neutral states start**, and the designer's call.

**Every builder's call, gathered**

14. **Section 1, four Factions:** stack labels stack vertically above a Body in seat order; the orbit band is one line per Faction; the Report carries the seating line every turn; Orbital Reef and Starlab are placeholder names.
15. **Section 2, the two new Factions:** a Colony Ship gains +2 with Expanded Habitats for every Faction; the station and Module discounts are card multipliers, and a Module bought for Ducats follows them; the Archivists' first Victory part is stages complete; funding takes this turn's Research back out at Orders; Closed-Loop Colonies halves the Archive's Energy; Provisional Findings is on at turn 1 and reaches yields, Lab output, capacities, Ship strength, transit Fuel and Influence thresholds but not Event cards or the global climate lines; Archive stages are raised one at a time.
16. **Section 4, Unrest:** Relief and Resettle go to whoever directs a state, so an occupier may pay; the falls run before the throw-off check, so a state at 10 can be bought back the same Resolution; throw-off applies to Controlled and not Occupied states; a Sea Level threshold displaces people each time it fires for a state; the 7 threshold spares an Embassy's Allotment; refugee Unrest is charged once a turn from the whole inflow; a foreign Army in a state that throws its controller off becomes the state's own.
17. **Section 5, Blame:** Emissions follow whoever directs a state, so an occupier wears an Occupied state's figure and its threshold there is raised while Pacification uses the plain threshold; Event-card Emissions are nobody's Blame; the multiplier is applied then floored; a blocked state keeps its place in the queue; the development clock is written from the turn after a control change; **a woken Facility is self-run.**
18. **Section 6, the Emissions orders:** a Mothball lands at this Resolution and a Restart or Decommission at the next; a Strip Permit's three turns are three doubled Incomes and its price falls at the last; Leapfrog is refused at the base; the Archive cannot be mothballed; Scrubbers die when the controller actually changes, not when an Occupation begins; the AI decommissions only a mothballed Facility holding a state's last slot; every "while online" rule now reads "working".
19. **Section 7, the Breaks:** the "happened" sentences live in the table; Coral's losses flow as refugees; the Last Turn projection holds gross Emissions at the Sink as it stands at the cut; the "at this rate" headline ignores Breaks, so the two lines can disagree slightly; the Sink line lags the bar by one turn on the turn the Sink Weakens fires.
20. **Section 8, the sea:** a Sea Wall stops the slot loss only; Provisional Findings does not unlock it early; a lost slot remembers what drowned; a build reserves its row at order time and falls to the other row, a Sea Wall to nothing; a threshold reads as near within 0.2 C; the Tech Tree widens a branch to its busiest rung.
21. **Section 9, the sky:** **the return Hohmann angle is +75 degrees and not the ticket's -75**, the magnitude right and the direction turned round; each turn's sky is the first instant of its month; the AI's off-window discount and its two-turn Fuel banking; the Bodies are partitioned into an Earth system and a Mars system; the on-window flight is nine turns where the real fast transfer is about seven; the Surface Map writes four figures under every slot name until board lenses arrive; the Mars tooltip describes transits from Earth only.
22. **Section 10, the dispatch:** a line's place can name a Body; a Tech completion that is not the headline sits under Your works; Antarctica opening ranks with a Break and the Archive with a build; placeless lines default to On Earth.
23. **Section 11, the spectator:** the card sits left and the four rosters right; map labels are clipped between the panels in both modes; the top bar's figures stay seat 0's and say so on the seating line; the turn-1 Report still reads "You play the Custodians".
24. **Section 12, saves:** the folder is `%LOCALAPPDATA%\DyingEarth\data\saves`, one line from the `...\DyingEarth\saves` the ticket asked for; one RON header line then pretty RON; a mirror struct saved rather than `Game`; a manual save at the same seed and turn overwrites; Save is dead on the game-over screen.
25. **Slot positions** are still to the nearest degree or so from the Gazetteer of Planetary Nomenclature, and the Deimos map's zero meridian may not match it, so Swift may sit off its crater (carried from version 0.04).

---

*Decisions recorded on the map's tickets remain the source of truth. This document assembles them; it does not amend them.*

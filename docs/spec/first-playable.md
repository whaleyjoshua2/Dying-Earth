# Dying Earth — First Playable specification

**Amended:** version 0.02 changes the turn count, the Nation States, the start position, the Event Deck and two panels; see [`version-0.02.md`](version-0.02.md), which wins where the two disagree.

**Status:** the destination of the wayfinder map [Map: first playable version of the solar system colonization game](https://github.com/whaleyjoshua2/Dying-Earth/issues/1). Every rule here was decided on one of that map's sixteen tickets; each section names its ticket, and the ticket's resolution comment is the authority if this document and it ever disagree. Numbers come from [Costs and build times for everything buildable](https://github.com/whaleyjoshua2/Dying-Earth/issues/18) unless a section says otherwise.

**The bar:** an agent builds this game without asking the designer a single question. Where a detail was never decided, section 20 lists the default this document sets, so nothing is left to invent.

**Vocabulary:** `CONTEXT.md` at the repository root is the glossary. Use its words, in code identifiers as much as in the interface. Capitalised terms in this document are glossary entries.

---

## 1. The game on one page

A single-player, turn-based strategy game about colonizing the solar system before ecological collapse overtakes Earth. It should feel like a strategy board game, not an action game. It runs on Windows as one `.exe`.

- **Two Factions**, one played, one AI: the **Custodians**, who colonize while limiting ecological damage to Earth, and the **Prospectors**, who maximize extraction regardless of ecological cost.
- **Twelve Turns**, each one month. Both Factions give orders simultaneously, then the turn runs through seven phases.
- **Three resources** in one shared **Stockpile**: Materials (build), Fuel (move between Bodies), Energy (run things where they stand). **Research** is a fourth pool, spent only on Techs.
- **Three Bodies**: Earth, divided into seven Nation States on real geography; the Moon; Mars. Each has a 3D **Body Surface Map**; a **Solar System Map** joins them.
- **Earth warms** as a consequence of what both Factions build: Emissions raise a CO2 Stock, a Temperature follows it with a lag, the sea rises and takes land, and one **Collapse Line** ends everything.
- **Each Faction has its own Victory Condition.** Prospectors: an Extraction Total plus an Off-world Presence. Custodians: a Stabilization run plus the same Off-world Presence. Meet yours in an End phase and you win at once; otherwise turn twelve is scored.
- **Combat** is probabilistic and plays out in the background: set a Stance, the turn resolves it, read the Battle Report.
- **An Event Deck** that fills with Climate cards as the world warms.
- **A shared Tech Tree**: every Faction's Research flows into one Tech at a time, everyone gets it, and the Research Lead picks the next.

## 2. Technical foundation

Decided on [Pick how the game draws its two 3D views](https://github.com/whaleyjoshua2/Dying-Earth/issues/5), [Which Rust libraries suit a turn-based game with two 3D views on Windows](https://github.com/whaleyjoshua2/Dying-Earth/issues/3), [Install the Rust toolchain on this machine](https://github.com/whaleyjoshua2/Dying-Earth/issues/2) and ADR 0001.

### 2.1 Language, libraries, versions

- **Rust**, stable, MSVC toolchain (`stable-x86_64-pc-windows-msvc`; rustc and cargo 1.98.1 are installed and verified on the target machine, with Visual Studio 2022 Build Tools and the Windows SDK).
- **Bevy 0.19.1** draws every 3D view. **bevy_egui 0.42.0** (egui 0.36.2 underneath) draws every panel, bar, popup and tree.
- **Pin exact versions in `Cargo.toml`** (`=0.19.1`, `=0.42.0`). Both libraries change shape between releases. Read the documentation for the pinned versions, never the latest.
- Known traps from the prototype: Bevy's UV sphere has its poles on Z, so rotate every globe −90 degrees about X; the first-ever launch shows a blank window for roughly ten seconds while shaders compile; `StandardMaterial` is glossy by default; egui 0.36 uses `Frame::NONE`, not `Frame::default()`; bevy_egui panels take `&mut Ui`, obtained through `egui::Ui::new(ctx.clone(), ..., UiBuilder::new().layer_id(LayerId::background()).max_rect(ctx.viewport_rect()))`.
- Budget: the first dependency build is about seven minutes in release; rebuilds after editing are seconds.

### 2.2 Project layout

```
dying-earth/
  Cargo.toml              pinned versions, release profile
  src/
    main.rs               window, plugins, mode switch (play / screenshot / simulate)
    data/                 loaders for every table under assets/data
    world/                Bodies, Nation States, Colonies, slots
    pieces/               Facilities, Modules, Ships, Armies, Colonists
    turn/                 the seven phases, in order
    economy/              Stockpile, Income, Energy shortfall, build queue
    earth/                Industry Level, Influence, Occupation, Army movement
    space/                transits, launches, Orbital Control, founding
    combat/               the battle algorithm
    climate/              CO2 Stock, Temperature, sea level, population
    research/             Research, the Tech Tree, the Research Lead
    events/               the Event Deck
    factions/             multipliers, signature rules, Victory Conditions
    ai/                   enumerate, score, spend
    ui/                   views, top bar, panels, popups, Battle Report
  assets/
    data/                 one TOML file per table (section 2.3)
    textures/             earth.png, moon.png, mars.png (section 2.4)
  tests/                  formula tests and the simulated-game check (section 19)
```

Module boundaries are a guide, not a rule; keep the data tables and the turn order exactly as specified and arrange code as you see fit.

### 2.3 Data tables

The game is data-driven: bodies, Nation States, Facilities, Modules, Ships, Armies, Techs, Events, Faction multipliers, AI weights, the starting position and every climate constant live in **TOML files under `assets/data/`**, loaded at startup, never hard-coded. TOML is chosen because the designer, who does not program, must be able to open a table and change a number. Every table in this document maps to one file. Validate on load and refuse to start with a clear message naming the file and row if a table is malformed.

### 2.4 Real worlds

The shipped game uses **genuine textures** for Earth, the Moon and Mars, from public-domain sources: NASA Blue Marble for Earth, the NASA Lunar Reconnaissance Orbiter mosaic for the Moon, the NASA or USGS Viking colour mosaic for Mars. Nation States are drawn on real geography (section 4.2). Textures ship beside the `.exe` under `assets/textures/`; the executable and its `assets/` folder together are the game.

### 2.5 Headless screenshot mode

The game ships with a **screenshot mode**: run as `dying-earth.exe shot:<prefix>` it positions its window off-screen (at −5000, −5000), loads a new game, captures each of the four views (Solar System Map, Earth Map, Moon surface, Mars surface) to `<prefix>-<view>.png` with Bevy's `Screenshot` component and `save_to_disk`, and exits with status 0. Nothing may appear on the desktop. This is how a building agent checks its own pictures with nobody at the screen, and it is the standing rule of this project. A **simulate mode** (section 19) is the other half of the same idea.

## 3. Units and conventions

- Materials, Fuel, Energy, Research and Influence are **integers**. Multipliers are applied and the result **rounded down**, never up.
- The **CO2 Stock** is in parts per million and the **Temperature** in degrees above pre-industrial, both to one decimal place.
- Nation State **population** is in **hundreds of millions**, to one decimal.
- A **Colonist** is one abstract unit of colonial population, representing ten million people (section 20).
- **Randomness** comes from one seeded generator per game; the seed is shown on the game-over screen and accepted as `dying-earth.exe seed:<n>` so any game can be replayed.
- Six-sided dice are written **d6**.

## 4. The world

### 4.1 Bodies

Decided on [What is on the Earth map and the solar map](https://github.com/whaleyjoshua2/Dying-Earth/issues/8) and the numbers ticket.

| Body | Colony Slots | Transit from Earth | Fuel per transit | Mine yield | Generator yield | Refinery yield | Habitat yield |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Earth | none (Nation States instead) | — | — | — | — | — | — |
| The Moon | 4 | 1 turn | 6 | ×1.5 | ×1.25 | ×0.5 | ×1.0 |
| Mars | 6 | 4 turns | 20 | ×1.25 | ×0.75 | ×1.5 | ×1.5 |

Transits between the Moon and Mars use the Mars figures (4 turns, 20 Fuel). Colony Slots are visible places on the Body's surface; place them spread across the globe, on the near side for the Moon. A yield multiplies what a Module **produces**, never what it costs.

### 4.2 Nation States

Decided on [How nation states work on Earth](https://github.com/whaleyjoshua2/Dying-Earth/issues/16), amended by the combat, turn-order, maps and tech-tree tickets.

| State | Population | Industry Level | Resource Lean | Baseline Emissions | Education Level | Size | Coastal Exposure |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Africa | 14.0 | 1 | Materials | 0.3 | 0.8 | 3 | 1 |
| Antarctica | 0.0 | 0 | Fuel | 0.0 | 1.0 | 2 | 0 |
| Asia | 47.0 | 3 | Materials | 0.4 | 0.9 | 4 | 2 |
| Australia and Oceania | 0.5 | 2 | Energy | 0.2 | 1.35 | 2 | 2 |
| Europe | 7.5 | 3 | Energy | 0.3 | 1.45 | 3 | 1 |
| North America | 6.0 | 3 | Fuel | 0.4 | 1.5 | 3 | 1 |
| South America | 4.5 | 1 | Fuel | 0.2 | 1.1 | 3 | 1 |

Each state is a region of the real globe. Draw its border as the continent's coastline; islands go with the nearest continent, Central America and the Caribbean with North America, all of Russia with Europe, the Middle East with Asia.

Derived per state, recomputed whenever an input changes:

- **Build slots = Size + Industry Level**, less any slots lost to the sea (section 11.4). A Facility occupies one slot.
- **Standing Army strength = Industry Level + 1.** The state's Army replenishes 1 strength per turn up to that cap (section 8.4).
- **Resource Lean** multiplies the output of that resource's Facilities in the state by 1.5.
- **Population factor = 1 + population ÷ 50**, used by Research.

**Continent adjacency**, for Army movement. Edges are symmetric:

| From | Neighbours |
| --- | --- |
| North America | South America, Europe, Asia |
| South America | North America, Antarctica |
| Europe | North America, Asia, Africa |
| Asia | North America, Europe, Africa, Australia and Oceania |
| Africa | Europe, Asia, Antarctica |
| Australia and Oceania | Asia, Antarctica |
| Antarctica | South America, Africa, Australia and Oceania |

## 5. The pieces

### 5.1 Facilities, built in a Nation State

| Facility | Materials | Build turns | Energy upkeep | Produces per turn | Emissions per turn |
| --- | --- | --- | --- | --- | --- |
| Factory | 20 | 1 | 2 | 4 Materials | 1.0 |
| Power Plant | 25 | 2 | 0 | 6 Energy | 1.5 |
| Refinery | 20 | 1 | 3 | 3 Fuel | 1.5 |
| Research Lab | 25 | 1 | 3 | Research (section 12.1) | 0.0 |
| Launch Site | 30 | 2 | 2 | enables building and launching Ships | 0.0 at rest |

**Raising Industry Level** is a build action: 30 Materials (15 for the Prospectors), one turn, no slot; it adds one build slot and raises the state's own Emissions.

### 5.2 Modules, built in a Colony

| Module | Materials | Build turns | Energy upkeep | Produces or does |
| --- | --- | --- | --- | --- |
| Mine | 20 | 1 | 3 | 4 Materials × Body yield |
| Generator | 25 | 2 | 0 | 5 Energy × Body yield |
| Refinery | 20 | 1 | 3 | 3 Fuel × Body yield |
| Habitat | 25 | 1 | 2 | holds 4 Colonists × Body yield |
| Shipyard | 35 | 2 | 4 | enables building Ships here |
| Barracks | 20 | 1 | 2 | holds and builds one defensive Army |

Modules never emit. A Colony has no slot limit of its own; it is limited by the Stockpile and by Energy.

### 5.3 Ships and Armies

Decided on [How combat works](https://github.com/whaleyjoshua2/Dying-Earth/issues/17).

| Unit | Materials | Build turns | Energy upkeep | Strength | Hit Points | Pursuit | Carries |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Colony Ship | 30 | 1 | 1 | 0 | 3 | 0 | 4 Colonists **or** 1 Army |
| Frigate | 25 | 1 | 2 | 3 | 4 | 4 | nothing |
| Battleship | 50 | 2 | 4 | 7 | 8 | 1 | 1 Army |
| Army | 25 | 1 | 2 | 4 | 5 | 2 | — |

- Ships are built at a **Launch Site** (Earth) or a **Shipyard** (Colony) and appear at that Body. A Colony Ship's strength is 0: it never lands a hit.
- **Armies** are of two kinds with one stat line. A **Nation State's Army** belongs to the state (section 8.4). A **Colony's Army** is built at, and only exists with, a Barracks; it belongs to the Colony, never leaves it, never attacks, and its upkeep comes from the Stockpile of the Faction controlling the Colony.
- **Damage persists.** A unit whose damage reaches its Hit Points is destroyed with everything it carries.
- **Repair**: 5 Materials per point, one turn stationary, Ships at a Launch Site or Shipyard, Armies in a controlled Nation State or a Colony with a Barracks.
- **Colonists** are counted, never spent. Habitats hold them, Colony Ships move them. Twelve of them off Earth is the Off-world Presence both Factions need.

## 6. The turn

Decided on [What happens in a turn, and in what order](https://github.com/whaleyjoshua2/Dying-Earth/issues/6).

A Turn is one month. The First Playable runs **twelve**. Both Factions order against the same board in the same Orders phase and neither sees the other's orders. The phases, always in this order:

| # | Phase | What happens |
| --- | --- | --- |
| 1 | **Income** | Every producer produces into the Stockpile (section 7.1); Research accrues (12.1); Energy upkeep is paid, with the shortfall rule (7.2); Standing Armies replenish |
| 2 | **Climate** | Last turn's Emissions are summed by source, the Natural Sink subtracted, the CO2 Stock updated, the Temperature moved toward its target (11.1), sea level checked (11.4), populations changed (11.3) |
| 3 | **Report** | The Battle Report, last turn's Event, completed builds, arrivals and control changes are shown (section 17.5) |
| 4 | **Orders** | The player gives every order for the turn; the AI computes its orders against the same board (section 16). Any order may be cancelled and fully refunded until End Turn |
| 5 | **Event** | One card is drawn from the Event Deck and shown; it applies during Resolution (section 13) |
| 6 | **Resolution** | In this order: **(a)** transits advance and arrivals land, with Intercept battles; **(b)** every other battle resolves; **(c)** Occupation counts down and control transfers; **(d)** Influence is applied, decays, and flips control where a threshold is passed; **(e)** builds due this turn complete; **(f)** repairs finish; **(g)** Colony Ships ordered to unload do so, founding Colonies; **(h)** Event effects that act "now" apply, in card text order |
| 7 | **End** | Victory Conditions are checked, then Collapse; then the turn number advances |

Consequences to build exactly as stated:

- Income and Climate run **before** Orders, so a turn's production is spendable at once and the Climate Panel reflects everything through last turn.
- An order given in turn N with a build time of T completes in Resolution of turn N + T − 1 and is usable in turn N + T. ("Two turns" means it appears in the second Resolution.)
- A Ship that arrives in Resolution (a) does nothing further that turn. It can be intercepted. It may act from the next Orders phase.
- **Simultaneous-order tiebreak:** where both Factions order the same single thing (the last Colony Slot on a Body, the same neutral state past its threshold in the same turn), the Faction with the greater total unit strength present at that place takes it; if equal, a d6 each, higher wins, re-rolled on a tie.
- There is **no limit on orders per turn** beyond the Stockpile, build slots, Energy and the Influence Allotment.

## 7. The economy

Decided on [Name the three resources and what each one does](https://github.com/whaleyjoshua2/Dying-Earth/issues/4), the modules ticket and the numbers ticket.

### 7.1 The Stockpile and Income

One shared Stockpile of Materials, Fuel and Energy, not divided by Body. In Income, every Facility and Module that is online adds its output, after the Body yield, Resource Lean, Faction multiplier and Tech multipliers in that order.

### 7.2 Energy is a balance that can run short

At Income, add all Energy produced, then subtract the upkeep of every Facility, Module, Ship and Army. **If the balance would fall below zero, shut producers down in order of highest upkeep first (ties: Modules before Facilities, then alphabetical) until it does not.** A shut-down producer makes nothing that turn and is shown as offline. Ships and Armies are never shut down; their upkeep is always paid first.

### 7.3 Building

A build order names a place and a thing, pays its Materials at once, and enters that place's queue. A Nation State may hold builds only up to its free slots; Industry Level rises have no slot. A Colony has no slot limit. Ships need a Launch Site or Shipyard at that place; an Army in a Nation State needs the state to be controlled by the ordering Faction; an Army at a Colony needs a Barracks with no Army. Cancelling an order before End Turn refunds it in full; nothing else ever refunds.

### 7.4 Loading and unloading

A Colony Ship or Battleship at a Body may be ordered to **load** or **unload** in Orders; it takes effect in Resolution (g). At Earth, Colonists board from a controlled Nation State chosen in the panel (default: the Faction's most populous controlled state), and that state's population falls by 0.1 per Colonist. At a Colony, Colonists board from and disembark into its Habitats, which must have room. An Army boards from the state or Colony where it stands and disembarks onto the Body; landing needs Orbital Control (section 9.3).

## 8. Earth

### 8.1 Control

A Nation State is **neutral**, **controlled** by a Faction, or **occupied** (8.5). The controlling Faction chooses the state's build orders, directs its Army, and collects its production. Control changes by Influence (8.3) or by Occupation (8.5).

### 8.2 Industry Level

Raising it is a build action (5.1). It adds a build slot and raises the state's own Emissions, which are **Baseline Emissions × Industry Level** every turn. It never falls.

### 8.3 Influence

Decided on the nation-states ticket and amended on the turn-order and combat tickets.

- Each Faction receives an **Allotment of 10 Influence per turn, plus 3 for each Nation State it controls**, ×1.3 for the Custodians. It is allocated in Orders, split freely across any number of targets, and any unspent remainder is lost.
- A target is any Nation State or Colony the Faction does not control (section 20 for enemy-controlled states). Influence accumulates per Faction per target.
- **Thresholds:** a Nation State's is 20 + 10 × Size; a Colony's is 10 × the Colonists living there. Green Consensus multiplies every threshold by 0.75.
- **Decay:** every accumulation on a target that received no Influence this turn loses 2.
- In Resolution (d), the first Faction whose accumulation meets a target's threshold takes control; every other Faction's accumulation there is wiped. The owner of a Colony may spend Influence on its own Colony to reduce a rival's accumulation there one for one.

### 8.4 Nation State Armies

- Every state keeps a **Standing Army**: one Army unit with strength capped at Industry Level + 1 (Antarctica, at Industry 0, has an Army of strength 1). At Income it regains 1 strength if below its cap, and its Hit Points are 5 like any Army.
- The controlling Faction directs the state's Army and may build more Armies there (5.3); all belong to the state. **An Army follows its state**: if the state changes hands, every Army of that state changes Faction where it stands, including aboard a Ship or on the Moon.
- **Movement:** in Orders, an Army in a state may be ordered one step to an adjacent continent (4.2). Entering a state its Faction controls is a move. Entering a neutral or enemy state is an **attack**, resolved in Resolution (b) against every Army present there. A neutral state's Army never moves.
- Armies of a neutral state defend it. An Army whose state is neutral stays home.

### 8.5 Occupation and Pacification

Decided on the combat ticket.

An attacking Army that leaves no defending Army engaged in a Nation State or Colony **occupies** it at Resolution (c). While occupied: the occupier chooses its build orders and collects its production; its Standing Army stands down (does not fight, does not replenish); the previous controller loses both. Each turn of unbroken Occupation the occupier gains **Influence equal to one third of the place's threshold, rounded up**. The place is **Pacified** the moment that accumulation meets the threshold, and in any case control transfers at the end of the third consecutive turn of Occupation; at transfer every rival's Influence is wiped, and the place's Armies become the occupier's. If the occupier's Armies leave or are destroyed before transfer, the Occupation ends and the count resets.

**Destruction rolls:** every Facility or Module at a place rolls a 1-in-4 chance to be destroyed **when the place is attacked**, and again **when it changes hands**.

## 9. Space

### 9.1 Launches and transits

An order to move a Ship from one Body to another is given on the Solar System Map. It pays the Fuel at once (4.1, ×0.6 with Efficient Transit) and the Ship is in transit for the Body's transit time, advancing one turn per Resolution (a). A launch **from Earth emits 2.0** (0.8 with Clean Propellant), charged in the next Climate phase; a launch from a Colony's Shipyard emits nothing. Transits are abstract: a Ship in flight cannot fight, be fought, or turn back.

### 9.2 Stacks and Stances

All of one Faction's Ships at a Body form one **stack**, and all its Armies at a place another. Every stack carries one **Stance** into Resolution, set in Orders and defaulting to Hold:

- **Attack**: engage every enemy stack at this place in Resolution (b).
- **Hold**: fight only if attacked.
- **Intercept** (Ships only): engage enemy Ships as they arrive at this Body in Resolution (a), before they may do anything.
- **Evade**: attempt to disengage at once if engaged (section 10, with the disengage roll made at the start of the battle at the unit's current damage).

Before Attack is confirmed, the panel shows both stacks and the attacker's probability of winning the first round (section 10.4).

### 9.3 Orbital Control

A Faction holds **Orbital Control** at a Body when it has at least one Frigate or Battleship there and no enemy warship remains engaged there after Resolution (a) and (b). Where neither Faction has a warship, nobody contests. **Armies and Colonists can land only where their Faction holds Orbital Control or nobody contests it.** Ships may still arrive and fight through a blockade; they may not unload.

### 9.4 Founding a Colony

A Colony Ship carrying Colonists, at a Body where landing is allowed, ordered to unload into a free Colony Slot, **founds a Colony** there in Resolution (g): the slot is taken, a Habitat appears as its first Module (free, already built), and the Colonists move into it. A Faction may hold any number of Colonies on a Body, limited only by slots. **No Colonies exist when the game starts.**

## 10. Combat

Decided on [How combat works](https://github.com/whaleyjoshua2/Dying-Earth/issues/17); numbers from the numbers ticket. One algorithm serves space and ground.

### 10.1 When a battle happens

At a place in Resolution (a) or (b), a battle happens between two Factions' stacks if either stack's Stance is Attack or Intercept applies, or if an Army entered a neutral or enemy state. A neutral state's Army always defends. Battles at one place are resolved one at a time in the order the attacks were ordered.

### 10.2 The rounds

Let A and D be the attacker's and defender's total strength among engaged units (a unit that has escaped is not engaged).

Repeat up to **3 rounds**:

1. **Three hit-rolls.** For each, the attacker wins with probability A ÷ (A + D), else the defender wins. The winner deals 1 damage to a uniformly random engaged enemy unit.
2. **Disengage.** Every engaged unit that has taken damage rolls: with probability (damage ÷ Hit Points) ÷ 2 it disengages. A unit with Stance Evade rolls even if undamaged, at probability 0.5.
3. **Pursuit.** For each disengaging unit, one enemy unit of the opposing side with the highest Pursuit rolls a d6; on a result at or under its Pursuit the leaver is caught: it takes one more hit-roll's worth of exposure (a single hit-roll won by the pursuer with probability pursuer strength ÷ (pursuer strength + leaver strength), dealing 1 damage on success) and then escapes regardless.
4. Remove destroyed units. If either side has no engaged units, the battle ends.

### 10.3 After the battle

- Survivors stay at the place. Escaped units are marked and cannot be attacked again this turn.
- If the defender has no engaged units at a Nation State or Colony and the attacker has an Army there, Occupation begins (8.5). If the defender's Ships are all gone or escaped, the attacker holds Orbital Control (9.3).
- Every battle writes one line to the Battle Report: place, attacker, defender, units and strengths on each side, hits landed, units destroyed, units escaped, and any Occupation or Orbital Control that resulted.

### 10.4 The odds preview

Show the probability that the attacker wins **more hit-rolls than the defender in the first round**: with p = A ÷ (A + D), that is p³ + 3p²(1 − p). Show it as a percentage with both stack totals.

## 11. The Climate Model

Decided on [Model global warming for this game](https://github.com/whaleyjoshua2/Dying-Earth/issues/9), amended by the maps ticket; numbers from the numbers ticket.

### 11.1 Stock and Temperature

| Constant | Value |
| --- | --- |
| Starting CO2 Stock | 420.0 ppm |
| Base Temperature | +1.2 °C, the Temperature at 420 ppm |
| Conversion | +0.5 °C per 40 ppm above 420 |
| Lag | 2 turns |
| Natural Sink | 6.0 ppm per turn |
| Collapse Line | +3.0 °C |

In the Climate phase: **net = Emissions − Sink**, where Sink is 6.0 plus any Restoration bought last turn (14.2); **CO2 Stock += net** (it can fall). The **target Temperature** is 1.2 + 0.5 × (Stock − 420) ÷ 40. The displayed **Temperature moves half the remaining distance toward the target each Climate phase**, which produces the two-turn lag. The Temperature is never below +1.2.

### 11.2 Emissions, by source

Summed in the Climate phase from what stood **last** turn, each source kept separately for the panel:

| Source | Emissions per turn |
| --- | --- |
| Nation State industry | Baseline Emissions × Industry Level |
| Factory | 1.0 |
| Power Plant | 1.5 |
| Refinery | 1.5 (methane-weighted) |
| Research Lab, idle Launch Site | 0.0 |
| Launch from Earth | 2.0 per launch |
| Population | 0.1 per hundred million, summed over all states |
| Wildfire card | +2.0 for the named state |

Every Earth source is multiplied by the controlling Faction's Emissions multiplier (14.1); a neutral state's industry and population use 1.0. Techs multiply named sources (12.3). Modules, Ships and Colonies emit nothing.

### 11.3 Population

In the Climate phase each state's population changes by **+1.0% per turn, less 0.15% for every full 0.1 °C the Temperature stands above +1.2** (so at +2.7 °C growth is −1.25%). Antarctica stays at 0. Colonists leaving Earth (7.4) lower population directly.

### 11.4 Sea level

Thresholds at **+1.8, +2.3 and +2.8 °C**. The first Climate phase in which the Temperature stands at or above a threshold, every Nation State **permanently loses build slots equal to its Coastal Exposure**. A Facility in a lost slot is destroyed, highest-upkeep first. Each threshold fires once. The Storm Surge card (13) applies a state's next threshold early; a threshold applied early does not fire again for that state.

### 11.5 The Climate Panel

Shown on the Earth Map by default and openable from any view. In this order: the CO2 Stock; the Temperature now and its target ("+1.4 °C, heading to +1.7"); this turn's Emissions by source, then the Sink, then the net; the penalties in force in plain words ("population growth −0.4%, N Climate cards in the deck"); and a projection: "at this rate, +2.6 °C by turn 12; Collapse at +3.0", computed by repeating the current net to the last turn. The projection line is the most important thing on the screen.

## 12. Research and the Tech Tree

Decided on [Sketch the scaled-down tech tree](https://github.com/whaleyjoshua2/Dying-Earth/issues/10) and the nation-states ticket.

### 12.1 Research

In Income each Research Lab produces **2 × the state's population factor × the state's Education Level**, multiplied by the Faction's Research multiplier (14.1) and by 1.5 with Public Science, rounded down. Research is held per Faction outside the Stockpile and **flows automatically and entirely into the one Tech under research worldwide**; it cannot be saved or spent elsewhere.

### 12.2 One tree, one Tech at a time

Every point of both Factions' Research goes into the current Tech. When its cost is met, **every Faction has it**, and the **Research Lead**, the Faction that contributed more to it (ties: the player), picks the next Tech from those whose prerequisites are met. The player picks the first Tech of the game. Overflow carries into the next Tech.

### 12.3 The twelve Techs

Costs by rung: **15, 25, 40**.

| Branch | Tech | Rung | Cost | Needs | Effect |
| --- | --- | --- | --- | --- | --- |
| Industry | Efficient Grids | 1 | 15 | — | Power Plant and Generator output ×1.5 |
| Industry | Clean Power | 2 | 25 | Efficient Grids | Power Plant Emissions ×0.4 |
| Industry | Clean Manufacturing | 3 | 40 | Clean Power | Factory and Refinery Emissions ×0.4; Wildfire adds no Emissions |
| Propulsion | Clean Propellant | 1 | 15 | — | Launch Emissions 0.8; Launch Failure delays instead of destroying |
| Propulsion | Efficient Transit | 2 | 25 | Clean Propellant | Transit Fuel ×0.6; transits advance through Solar Storm |
| Propulsion | Hardened Hulls | 3 | 40 | Efficient Transit | Every Ship +2 strength; immune to Radiation Surge |
| Off-world Living | Expanded Habitats | 1 | 15 | — | Each Habitat holds +2 Colonists |
| Off-world Living | Closed-Loop Colonies | 2 | 25 | Expanded Habitats, Clean Power | Module Energy upkeep ×0.5; immune to Grid Failure |
| Extraction | Deep Mining | 1 | 15 | — | Mine and Factory output ×1.5; Rich Seam gives ×3 |
| Extraction | Automated Refining | 2 | 25 | Deep Mining, Efficient Grids | Refinery output ×1.5; Ice Deposit gives ×3 |
| Society | Public Science | 1 | 15 | — | Research Lab output ×1.5; Breakthrough gives 25; stances hold through Comms Blackout |
| Society | Green Consensus | 2 | 25 | Public Science, Clean Power | Per-person Emissions ×0.5; Influence thresholds ×0.75; Heatwave loss halved |

Effects of the same kind multiply. The tech panel shows one line per Faction: "Custodians 41%, Prospectors 59% — Prospectors pick next."

## 13. The Event Deck

Decided on [The event deck](https://github.com/whaleyjoshua2/Dying-Earth/issues/13); numbers from the numbers ticket.

### 13.1 The deck

**Twenty cards: one of each of the twelve Events, and eight Calm Cards.** Shuffled with the game seed at the start; one drawn in phase 5 each turn and discarded; never reshuffled. **In each Climate phase, for every full 0.2 °C the Temperature stands above +1.2 that has not already been counted, one Calm Card still in the deck is replaced by one Climate card**, chosen uniformly among the four Climate Events. A Calm Card drawn shows "Calm" and does nothing.

### 13.2 The twelve cards

| Kind | Card | Target | Effect in Resolution | Blunted by |
| --- | --- | --- | --- | --- |
| Solar | Solar Storm | everyone | no transit advances this turn | Efficient Transit: that Faction's transits advance |
| Solar | Radiation Surge | everyone | every Ship in transit takes 1 damage | Hardened Hulls: no damage |
| Solar | Comms Blackout | everyone | every stack fights as if its Stance were Hold | Public Science: that Faction's Stances hold |
| Failure | Equipment Failure | one Faction | one build due this turn completes next turn instead | — |
| Failure | Launch Failure | one Faction | one Ship due this turn is destroyed, no refund | Clean Propellant: delayed one turn instead |
| Failure | Grid Failure | one Colony | its Modules are offline until the next Resolution | Closed-Loop Colonies: no effect |
| Discovery | Rich Seam | one Body | its Mines produce ×2 for two turns | Deep Mining: ×3 |
| Discovery | Ice Deposit | one Body | its Refineries produce ×2 for two turns | Automated Refining: ×3 |
| Discovery | Breakthrough | the Tech under research | +15 Research | Public Science: +25 |
| Climate | Heatwave | one Nation State | population −5% now | Green Consensus: −2.5% |
| Climate | Wildfire | one Nation State | one Facility offline until next Resolution; +2.0 Emissions next turn | Clean Manufacturing: no extra Emissions |
| Climate | Storm Surge | one exposed Nation State | its next sea-level threshold applies now | nothing; the sea is inexorable |

**Targeting:** Solar cards hit everyone. Failure and Discovery cards choose one Faction, Colony or Body uniformly at random among those that qualify (a Failure with no qualifying target does nothing). Climate cards choose one Nation State with probability proportional to population (Heatwave, Wildfire) or uniformly among states with Coastal Exposure above 0 that still have a threshold ahead (Storm Surge).

**Scaling:** a Climate card's numeric effect is multiplied by **1 + (Temperature − 1.2) ÷ 2**.

**Blunting** applies to the Faction that holds the Tech; a card that hits everyone is blunted only for holders.

**The line:** Emissions added by a card do not count against a Stabilization run (the run test uses Emissions from buildings, launches and population only), and no card reduces an Extraction Total.

## 14. The Factions

Decided on [How the Custodians and the Prospectors differ mechanically](https://github.com/whaleyjoshua2/Dying-Earth/issues/11).

### 14.1 Multipliers

| | Prospectors | Custodians |
| --- | --- | --- |
| Facility and Module output | ×1.25 | ×1.0 |
| Emissions from every Earth source it controls | ×1.25 | ×0.75 |
| Research | ×0.75 | ×1.25 |
| Influence Allotment | ×1.0 | ×1.3 |
| Ship and Army strength | ×1.0 | ×1.0 |

### 14.2 Signature rules

- **Prospectors — Cheap Industry:** raising Industry Level costs 15 Materials instead of 30.
- **Custodians — Restoration:** in Orders they may spend Energy in multiples of 10; each 10 adds 3.0 ppm to the Natural Sink in the next Climate phase only.

### 14.3 Starting position, identical for both

Stockpile **60 Materials, 20 Fuel, 20 Energy**; Research 0; one Nation State at the Industry Level on its card, with **one Launch Site already built** and its Standing Army; no Colonies, Ships or other Armies. The player picks a continent; the AI takes the uncontrolled continent with the highest Industry Level, ties broken by population (section 20). The five others start neutral.

## 15. Victory and defeat

Decided on [What winning and losing look like in twelve turns](https://github.com/whaleyjoshua2/Dying-Earth/issues/14).

| Bar | Value |
| --- | --- |
| Prospectors' **Extraction Total** | 500: cumulative Materials plus Fuel produced by their Mines, Refineries and Factories since turn 1, counted at production, never reduced |
| Custodians' **Stabilization** | 3 consecutive Climate phases with net Emissions (buildings, launches, population; not cards) below the Natural Sink, counting Restoration |
| **Off-world Presence**, both | 12 Colonists living in Habitats off Earth |

In the End phase, in this order:

1. If exactly one Faction meets both parts of its Victory Condition, it wins. If both do, the one exceeding its bar by the larger margin (Extraction Total as a fraction of 500; Stabilization run length as a fraction of 3; the lower fraction of the two parts) wins; equal is a draw.
2. If the Temperature is at or above the Collapse Line, the game ends and **nobody wins**.
3. If this was turn twelve, each Faction is scored as the lower of its two parts as a fraction of its bar; higher wins; ties broken by Colonists off Earth, then Colonies held, then a draw.

**No Faction is ever eliminated.** A Faction with nothing keeps its Influence Allotment and keeps playing. **Both Factions' progress toward their own bars is shown on one panel**, updated every turn.

## 16. The AI

Decided on [How the AI faction decides its turns](https://github.com/whaleyjoshua2/Dying-Earth/issues/12). The AI plays by exactly the rules above, with the same costs, visibility and Allotment. One difficulty.

### 16.1 The procedure, every Orders phase

1. **Enumerate** every legal action: each Facility in each controlled state and raising Industry Level there; each Module in each Colony; each Ship at each Launch Site and Shipyard; each Army it may build; each transit for each stack; each Stance for each stack; loading and unloading; Influence on each eligible target in units of 5; Restoration in units of 10; founding into each free slot where it has a loaded Colony Ship.
2. **Score** each: **base weight × victory gap × denial × threat × opportunity** (16.2, 16.3).
3. **Sort** descending and **spend greedily**: take each action if still affordable and legal, skip otherwise, until the list is exhausted. Stances are assigned last, one per stack, highest score.

### 16.2 Base weights

| Action | Prospector | Custodian |
| --- | --- | --- |
| Build a producer (Factory, Mine, Refinery, Power Plant, Generator) | 8 | 6 |
| Raise Industry Level | 9 | 3 |
| Build a Research Lab | 3 | 8 |
| Build a Habitat | 5 | 7 |
| Build a Launch Site or Shipyard | 6 | 6 |
| Build a Colony Ship | 6 | 8 |
| Build a Frigate or Battleship | 5 | 3 |
| Build an Army or Barracks | 5 | 4 |
| Spend 5 Influence on a target | 5 | 8 |
| Order a transit | 6 | 6 |
| Load or unload | 6 | 6 |
| Found a Colony | 9 | 9 |
| Spend 10 Energy on Restoration | — | 7 |
| Stance Attack | 4 | 2 |
| Stance Intercept | 3 | 3 |
| Stance Hold | 2 | 3 |
| Stance Evade | 1 | 1 |

Within a category, prefer the action that raises the resource the Faction is shortest of relative to its next three affordable actions; a producer of Energy scores +2 while the Energy balance is within one turn's upkeep of zero.

### 16.3 Multipliers

- **Victory gap:** ×1 when on or ahead of pace, rising linearly to ×3 when at half pace or worse, applied to actions that advance the part of its Victory Condition it is furthest behind on. **Pace:** Prospectors hold 40 Extraction Total by turn 3, 150 by 6, 320 by 9, 500 by 12; Custodians are within 4 ppm of the Sink by turn 6 and under it by turn 9; both hold 4 Colonists off Earth by turn 6 and 12 by turn 11.
- **Denial: ×2.5** on Emissions-raising actions (Industry Level, Power Plant, Factory, Refinery, launches) for a Prospector AI while the rival's Stabilization run is 1 or more; ×2.5 on Restoration and on Influence against the rival's holdings for a Custodian AI while the rival is within 20% of its bar. Also ×2.5 on Attack against any stack of a rival within one turn of winning, for either.
- **Threat: ×2** on Army and warship builds, Barracks, and the Hold and Intercept Stances at a place where an enemy stack is present or inbound.
- **Opportunity: ×2** on any action that completes something within one turn: a target within 5 Influence of its threshold, the last Colonist needed, the last slot on a Body.

### 16.4 Thresholds and picks

- **Attack** when the odds preview (10.4) is **0.6 or better** for a Prospector AI; a Custodian AI attacks only to retake a place it lost or to break a blockade at a Body where it has a Colony, at 0.6 or better; either attacks at 0.5 or better against a rival within one turn of winning.
- **Evade** when a stack's total damage is **two thirds of its total Hit Points or more**.
- **Intercept** at any Body where it holds Orbital Control and an enemy Colony Ship or Battleship is inbound.
- **Colony Slot:** the free slot on the Body whose yields best serve the part of its Victory Condition it is furthest behind on (Materials for a Prospector, Habitat yield for Off-world Presence).
- **Tech picks as Research Lead**, in order, then the cheapest available: Prospectors: Deep Mining, Efficient Grids, Automated Refining, Clean Propellant, Efficient Transit, then anything but Green Consensus, Clean Power last. Custodians: Public Science, Efficient Grids, Clean Power, Green Consensus, Clean Manufacturing.
- **Influence target:** the neutral state with the highest (Industry Level + Size) it is closest to, then the rival's Colony with the fewest Colonists.

### 16.5 Debug output

The player sees only what the AI did (17.5). For building and tuning, simulate mode (19.3) writes the AI's full scored action list for every turn to its log.

## 17. The interface

Decided on [What is on the Earth map and the solar map](https://github.com/whaleyjoshua2/Dying-Earth/issues/8) and the turn-order ticket.

### 17.1 Views

Two kinds, one filling the window at a time. **The Solar System Map**: the Sun, Earth, the Moon and Mars as bodies on a plane, drawn to be legible rather than to scale, each showing its Colony Slots filled and empty, a marker per Ship stack with total strength written on it, an Orbital Control flag, and transits as lines with turns remaining at the midpoint. **A Body Surface Map** for each Body, entered by clicking it: a textured globe the player rotates by dragging and zooms with the wheel. Earth's is the **Earth Map**, its Nation States outlined and tinted in their controller's colour (untinted if neutral, hatched if occupied), each with small icons for Facility count and Army strength. The Moon's and Mars's show Colony Slots as markers, filled ones in their owner's colour with their Modules listed, plus any Army, and a band along the top listing the Ship stacks in orbit and who holds Orbital Control. A **swap control** (button and the Tab key) moves between the Solar System Map and the last surface viewed; a **back control** (Escape) leaves a surface.

### 17.2 Always on screen

A bar along the top on every view: the Stockpile's three numbers with this turn's net change; Research and the Tech under research with its progress; Influence Allotment remaining this turn; the turn number out of twelve; the Temperature with its heading; a Tech Tree button; a Climate Panel button; a Victory panel button; and **End Turn**.

### 17.3 Panels

Clicking a Nation State, Colony Slot, Colony or stack opens a side panel with its card and its orders. A state's panel: population, Industry Level, Lean, Emissions this turn, slots used and free, each Faction's Influence there, its Armies with damage, the build buttons and the Influence spinner. A Colony's: Modules with online status, Colonists and Habitat room, its Army, the build buttons. A stack's: units by type with damage, Stance selector, transit and load/unload orders, the Attack button with the odds preview. Every order shows its cost and greys out if unaffordable; every placed order has a cancel button until End Turn.

### 17.4 Where each order lives

| Earth Map | Moon and Mars surfaces | Solar System Map |
| --- | --- | --- |
| Build a Facility; raise Industry Level | Build a Module | Build a Ship at a Shipyard or Launch Site |
| Build an Army; move it to a neighbour; order it to attack | Build an Army at a Barracks | Order transits |
| Set Army Stances | Set the Colony Army's Stance | Set Ship Stances; order Ship attacks |
| Spend Influence on a state | Choose the slot a Colony Ship founds into | Load and unload; spend Influence on a Colony |
| The Climate Panel, open by default | — | — |

### 17.5 Popups

The **Report** opens each turn as a dismissable popup listing the Battle Report lines, last turn's Event and its effect, completed builds, arrivals, control changes and Occupation changes. The **Event card** appears as a popup when End Turn is pressed, showing the card, its target and its size at the current Temperature, dismissed to continue. The **game-over screen** names the result, the turn, both Factions' progress and the seed.

### 17.6 Starting a game

Title screen: New Game, Quit. New Game asks for the Faction (a card for each, with its multipliers, signature rule and Victory Condition in plain words), then the starting continent on a spinning Earth (Antarctica not offered), then begins on the Earth Map at turn 1 with the Report popup explaining the situation.

### 17.7 Style

Flat, legible, board-game-like: Faction colours (Custodians green-teal, Prospectors orange-brown), neutral grey, occupied hatching, white text on dark panels. No sound, no animation beyond the globes turning and transits creeping, no tutorial beyond the first Report. Every number the rules use is visible somewhere.

## 18. Persistence and distribution

The First Playable has **no save and load**; a game is played in one sitting or abandoned. It ships as a folder containing `dying-earth.exe` and `assets/`, zipped; no installer. See section 20.

## 19. Acceptance

The build is done when all of the following hold and are demonstrated in the pull request.

### 19.1 Builds and runs

`cargo build --release` succeeds with clippy clean under `-- -D warnings`; the `.exe` starts, shows the title screen, and a game can be played to the end by hand.

### 19.2 Screenshot mode

`dying-earth.exe shot:check` writes four PNGs off-screen showing the Solar System Map, the Earth Map with real continents and seven tinted states, and the Moon and Mars surfaces with their slots, and exits 0. The pictures are opened and looked at before they are presented.

### 19.3 Simulate mode and the anchors

`dying-earth.exe simulate:<seed> [--custodian-ai --prospector-ai]` runs a full AI-versus-AI game headless, logging every phase, every scored AI action and the game-over result, and exits 0. Over twenty seeds the log must show the designer's anchors holding within reason: about twenty Facilities and Modules per Faction by turn twelve; the first Colony founded around turn five; a Collapse projected around turn ten when both AIs are Prospectors (run with `--prospector-ai --prospector-ai`); a winner around turn eleven in most games; and a defended Colony changing hands in about four turns when attacked. Where an anchor misses, report it rather than retune: balance is the designer's next effort.

### 19.4 Formula tests

Unit tests that pin: the Income shortfall order; the Temperature lag reaching within 0.1 of target in two phases; the sea-level thresholds firing once; the battle round with fixed dice; the disengage and Pursuit probabilities; the Influence threshold and decay; the Occupation transfer at three turns and at Pacification; the Victory checks in their stated order; the deck swap rate; and every Tech effect. A test that fails on purpose first (red) before it passes.

## 20. Defaults set while writing this spec

The map's fog listed questions no ticket resolved. Each is set here so the builder never has to ask; each is the designer's to change.

1. **No save and load** in the First Playable.
2. **No installer**; a zipped folder is the delivery.
3. **No sound, no tutorial**, flat placeholder style (17.7).
4. **A Colonist represents ten million people**, so loading four Colonists lowers a state's population by 0.4. This reconciles Habitats counted in units with populations counted in hundreds of millions, and makes emigration's climate effect real but small.
5. **Influence may target enemy-controlled Nation States** as well as neutral ones and enemy Colonies, with the same threshold, and the owner may counter-spend on its own state one for one, as with Colonies.
6. **The AI's starting continent** is the uncontrolled one with the highest Industry Level, ties by population.
7. **The player picks the first Tech**, and ties for Research Lead go to the player.
8. **Colony Slot positions** are the builder's choice, spread across each globe.
9. **The Temperature lag** is implemented as halving the distance to target each turn (11.1), which is the two-turn lag the climate ticket allowed.
10. **The seed** is shown and accepted on the command line, so any game can be replayed (section 3).

---

*Decisions recorded on the map's tickets remain the source of truth. This document assembles them; it does not amend them.*

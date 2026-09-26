# Ticket #352: a hover that explains what a Research Lab makes

The designer's line: *"mouseover explains math for research output."* The playtest found a Lab
making 5 in Europe and 2 in India, with India holding four times the people, and nothing on screen
saying why.

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/352#issuecomment-5841413198)
is the authority, and served as the spec.

## What was built

- **`Chain`** (`engine/src/economy.rs`): the arithmetic of a yield, recorded as it is DONE.
  `facility_yield` and `module_yield` now compute their figures through it, one step at a time in
  the rule's order, so the hover that prints it is the rule itself and cannot drift from it. A
  factor of exactly 1 multiplies and records nothing, so a step appears only when it applies.
- **Every multiplied figure** carries one: a Research Lab (population weighted by Education,
  Education, the Faction, Public Science, The Upload), a Mine, Refinery, Power Plant or Factory (the
  Region's lean, the Faction, Techs), a Bank (GDP), and every Module (the site's yield or the
  sunlight, the Faction, Techs, a Discovery, a Solar Storm, the Colony's people for an Observatory, a
  Trade Post's network, the Heliostat, the Mass Driver). A Strip Permit, Unrest 7, the Custodians'
  idle-Facility doubling and ticket #359's shut Habitat each add a line when they apply.
- **Where it shows**: the Region card's slot-box hover and its Facility row, the build button for a
  new building (in the four lines a build hover has left), a Colony's Module tiles and the Module
  strip. The Region card's **Education Level** hover now says it counts twice in a Lab: *"once
  weighting how many people it has, and once on its own."*
- Glossary: **Education Level**.

## Two changes the build forced, both on the record

- **The rules sentences leave a hover that shows a chain.** "Energy upkeep is paid at Income first…"
  and "Its Emissions go on the CO2 Stock…" wrap to two or three rendered lines each; with them a
  Mine's slot hover ran to **nine** lines against the six a tooltip is allowed. They stay on every
  hover without a chain.
- **Ticket #359's Habitat half moved into `module_yield_at`**, the one figure every reader takes,
  so a Colony's Module hover shows the half. It was applied at Income and on the Widget sum. It now
  rounds per Module, and the computer now sees the halved figure when it weighs a Module.

## The sweep

Wins, collapses and gates are **identical** to ticket #359's after. Four detail lines moved, all
from the Habitat half moving: Constabularies built 121 to 126, one seat's Widgets made 545 to 539,
Factories completed 809 to 807, Sea Walls standing 81 to 82 ([`sweep-after.txt`](sweep-after.txt)).

## Tests

`a_research_labs_hover_is_its_arithmetic` pins the China Lab's words. `every_chain_ends_on_the_figure_the_game_pays`
walks every Facility kind in every Region and a Colony's Modules (64 multiplied figures) and checks
each chain lands on what the game pays. It was witnessed red by breaking the renderer's rounding
(*"ResearchLab in SubSaharanAfrica pays 2 and its chain ends = 2.03, rounded down to 3"*). Clippy
gate clean; 474 + 8 + 6 pass.

## The pictures

All `shot: seed:2 ... panel:0 window:1920x1080`.

| picture | what it shows |
|---|---|
| [`lab-build-hover.png`](lab-build-hover.png) | `select:eastasia slotbox:free "tip:weighted by Education"`. The Research Lab build button in China: *"Once it stands: +3 Research, 3 upkeep / 2 base / × 1.32 for 1.45B people, weighted by Education / × 1.10 for Education 1.10, × 1.25 as the Custodians / = 3.63, rounded down to 3"*, six lines. |
| [`mine-slot-hover.png`](mine-slot-hover.png) | `select:eastasia slotbox:1 "tip:as this Region leans"`. China's Mine box: *"4 base / × 1.50 as this Region leans Materials / = 6"*. |
| [`education-hover.png`](education-hover.png) | `select:eastasia "tip:counts twice"`. The Education Level line's hover. |
| [`generator-module-hover.png`](generator-module-hover.png) | `barracks:1 hab:ground "tip:for this site's yield"`. A Moon Generator: *"5 base / × 1.42 for this site's yield / = 7.10, rounded down to 7"*. |

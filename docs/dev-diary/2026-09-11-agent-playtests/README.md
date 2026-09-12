# Four agent playtests of version 0.06.0

Twelve games, three per Faction, every one played from seat 0 by an agent trying to win.
Played headless through `engine/examples/play.exe`, a text driver written for this run; no
window was opened and no rule, table or line of game code was changed.

Each Faction's own diary sits beside this file: `custodians.md`, `prospectors.md`,
`arkwrights.md`, `archivists.md`. This page is what the four of them agree on.

## The result

| Faction | Game 1 | Game 2 | Game 3 |
|---|---|---|---|
| Custodians | **win, t16** | Collapse, t21 | Collapse, t22 |
| Prospectors | **win, t15** | **win, t16** | **win, t14** |
| Arkwrights | **win, t25** | **win, t20** | **win, t16** |
| Archivists | **win, t24** | **win, t14** | **win, t26** |

Ten wins for seat 0 in twelve games; the two losses are Collapses, not defeats by a rival.
**No AI seat won a single game.** The First Playable is thirty-six turns long and no game
reached turn 27.

The two Collapses are worth as much as the wins: both are the Custodians leaving East Asia
with the Prospector AI, whose Industry Level climbs every turn in every state it holds (it
reached Level 19). The Custodian condition is decided by about turn 10 and is bimodal —
take a big dirty industrial state early and it is trivial, fail to and it is arithmetically
impossible, because Scrubber caps are pinned to the population of states you hold (about 22
slots, ~73 ppm of Sink) while world Emissions pass 90 ppm by turn 18.

## The four root causes

These are the findings that turned up independently at more than one seat. Each was read
back out of the code before being written down here.

### 1. A Colony has no limit on how many Modules it holds

`build_slots`, `free_slots`, `coastal_slots` and `inland_slots` all take a `StateId`. There
is no Colony equivalent, and the module build order checks nothing about `modules.len()`.
A Nation State gets Size + Industry + 3 slots and the sea eats them; a Colony gets no
ceiling at all.

Build Where You Dig then makes each further Module *cheaper than the last*: past two working
Mines a Mine costs 10 Materials and returns 16 a turn, a one-turn payback that repeats for
ever, and a Generator on the same discount pays the Mines' Energy, so the package
self-finances.

- **Prospectors:** one Moon Colony holding 51 Mines and 28 Generators, extracting 775
  Materials a turn by turn 13.
- **Custodians:** worse, because the Generators remove Energy, which is their *only* real
  brake on Scrubbers — and the surplus converts at 2 Materials per Influence point, 40 extra
  Influence a turn against a base Allotment of 22.

This is the one structural fact everything else sits on top of, and it is written down in a
comment in `modules.toml` and nowhere a player can read it.

### 2. The Research Lead is a one-seat veto over three Victory Conditions

Only the Lead picks the next Tech (`research.rs`: `awaiting_pick = Some(lead)`). Everybody
*receives* a finished Tech, but no Faction has any private path to research one. The AI
contributed **zero** Research in every game at three of the four seats; the world runs on
about 2 Research a turn from two neutral Labs. One or two Research Labs therefore buy the
Lead permanently — 9 of 9 Techs at one seat, 10 of 13 at another, every Tech after turn 9 at
a third.

Version 0.06.0's four gates then convert that into a lock-out. The sharpest evidence:

- A Prospector AI finished on **0.99** of its own condition — 744 of 750 in the Fund, 24
  Colonists off Earth against a bar of 12 — and could not win, because the Extraction
  Charter was never researched.
- A Custodian AI ended a game holding eight Nation States, five Colonies and 37 Colonists
  off Earth, unable to win for the same reason. (Its *score* of 0.00 has a separate cause:
  its Stabilization run stood at 0 of 3, and `Progress::score()` is the lower of the two
  fractions. The gate blocks `met()`, not the score. Two problems, not one.)

**And declining to pick is unenforced.** Omit the pick, Research pools in `unallocated`,
nothing is auto-chosen, and the tech tree freezes permanently. That is currently the
strongest strategy in the game. It appears in no spec and no CONTEXT entry.

### 3. Every Victory bar is far below what a trying player produces

| Bar | What it actually costs | Seen |
|---|---|---|
| 750 Materials in the Venture Fund | less than **one turn** of late extraction (620 banked in a single turn) | passed t13, t14, t15 |
| 80 Research into the Archive | six Observatories, 168 Materials, **three turns** | full t14, t19, t22 |
| 30 Colonists off Earth on 3 Bodies | two orbital Habitats — 18 Materials, 1 turn, **0 Fuel**, 15 Colonists each | won t16 |
| 12 Colonists off Earth | 115 Materials and **no transit at all** | won t16 without ever flying a ship |

The Archivists' pre-Archive fund cap of 20 never bound once in three games: the Module is
cheap enough to stand by turn 9, before 20 is banked, so the rule that is supposed to create
the Fund-versus-Provisional-Findings tension is dead on arrival. That choice was never a
decision at any point in any of the three games.

### 4. The decisions are all in the first ten turns

Every seat reported the same shape: turns 1–10 are genuinely good, and the game empties
after. The best-designed moment any of them named is the same one — reading in the turn-1
Report that all three rivals have dogpiled one Nation State, and quietly walking away.

After that: *"turns 11–16 were 141 Materials a turn against a 30-Material Scrubber; one
decision, the same one, every turn"*; *"turns 10–15 were build mines, buy energy, top up
Influence, end turn"*; *"t10–t14 was one line, `fund-archive`, five times"*.

The opposite failure sits early. Income of 12–18 a turn against buildings of 18–35 means
every third turn is banking: three separate turns were logged with **no affordable build at
all**, and one was logged as literally empty.

## Bugs found, each verified against the code

| What | Where | Effect |
|---|---|---|
| `fund-archive` silently banks less than the turn's Research, often 0 | `research.rs::fund_archive` takes back from `contributions`/`unallocated`, but `accrue_research` → `check_tech_complete` zeroes contributions on a completion, and `pick_tech` sweeps `unallocated` | ~32 of 80 Research lost in one game: 40% of the Archive |
| Splitting a purchase into single lots makes it cheaper, and makes Energy free | `market_price` floors the **lot total** after the Faction multiplier; Energy at 1 × 0.85 → 0 | Prospectors can buy unlimited free Energy |
| A landing into a slot another Faction took the same turn vanishes with nothing in the Report | `resolution.rs` ~1409 `continue`s without reporting | a lost ship-turn with no explanation |
| Winning the battle for an orbit still does not let you land, if a rival warship completed that turn | `resolve_builds` runs before `resolve_cargo` | blockaded an Archivist out of their own win for 9 turns |
| An offline Shipyard refuses builds with "no Shipyard here" | — | misdiagnoses a Grid Failure as a missing building |
| An Event taking Modules offline silently removes the Build Where You Dig discount | — | price moves 10 → 17 for one turn, unexplained |
| The Report header reads `turn 0` on the winning turn | — | cosmetic |
| Two Techs completing in one Resolution announce only one | — | the second is invisible except in `Techs done` |

## Rules a careful reader cannot find

Each of these was discovered by an agent mid-game, usually by being refused:

- A Colony founded from a ship or by sea comes with a **free Habitat**. Load-bearing, in the
  code only.
- A Colony has no Module cap (above).
- **You cannot unload Colonists into a Space Station you own if any rival warship is in that
  orbit.** The single most important rule for the Archivists; in neither CONTEXT.md's *Space
  Station* nor *Orbital Control* entry, nor PLAYTEST.txt.
- **The Archive can never be mothballed or decommissioned.** Combined with the above, the
  Archivists' one irreversible choice is made on turn 9 with no way to know whether that
  orbit will be contested on turn 20.
- Antarctic Modules emit like Facilities. Eighteen Generators at Lake Vostok put out 33.75
  ppm a turn, more than all that Faction's Factories.
- Expanded Habitats changes Colony Ship capacity (4 → 5). The Tech text mentions Habitats only.
- Slot indices are 0-based in orders and 1-based in the Report.
- What an Energy shortfall does, and that it can take your own Energy producers offline — an
  unrecoverable spiral from a low-GDP start.

## Where the tool, not the game, was at fault

Stated plainly so these are not read as findings:

- The driver printed `influence_threshold_for` — the bar on a **neutral** place — where a
  held place wants `influence_needed_for`, the greater of that threshold and the holder's
  Standing plus the challenge margin. So "you cannot see a rival's Standing" and "a Colony
  cannot be defended" are, for **held** places, artefacts of the driver: the real UI reads
  `influence_needed_for`, which discloses the holder's Standing implicitly. Fixed in
  commit `3ec4f3f`.
- On a **neutral** state under contest, however, the gap is real: `influence_needed_for`
  returns the bare threshold and nothing shows the other bidders. One agent lost ~120
  Influence from a 12-a-turn allotment on two neutral races it could not see it was losing.
- The driver showed no Standing at all on Colonies and stations. Also fixed in `3ec4f3f`.
- All four agents shared one scratchpad with no per-agent order-file names, and one agent's
  order file was overwritten between writing and running. Every line happened to be refused
  that time; a colliding *legal* file would have silently played another agent's turn.

## What held across seeds, and what was one draw's luck

**Held everywhere:** the early win at every seat; no AI seat ever reaching its gate Tech;
the Research-Lead monopoly off one or two Labs; the Module-stacking engine; the funding bug;
Steerage draining its home state in about ten turns; the launch-window calendar being
identical on every seed, since the sky is real; Refuel, Mass Driver and Trade Post never
being used once in three Arkwright games.

**One draw's luck:** the sea eating a Launch Site (a South Asia hazard, not a general one);
dead Ducats (a small-GDP-state problem, not a Ducat problem — nine of the twelve states pay
zero); the Earth-orbit blockade (bit in one game of three, and was the difference between
winning on t14 and nearly losing on t24, decided by nothing the player chose).

A seed did **not** explain the Prospector engine: holding the start fixed and changing only
the seed, the Moon's best Mine slot was 1.76 on one world and 1.75 on the other, because
slot yields are the Body's figure times a triangular draw and the best of four sits near the
top of that range on every seed.

## Options, ranked — the calls are the designer's

Each is a problem with evidence, one or two ways out, and what each would cost.

**1. Cap the Modules a Colony may hold.** Everything else is downstream of this; no Victory
bar can be fitted while one Colony can grow without limit.
*Options:* (a) slots tied to Colonists or Habitats, so people gate industry; (b) apply the
in-situ discount only to the first few Modules; (c) Energy upkeep that rises with the count.
*Trade-off:* (a) changes a Colony from a pile of buildings into a place with a size, which
is probably the right fiction, but slows every Faction off-world; (b) is the smallest edit
to the rule 0.06.0 just added.

**2. Break the Research-Lead veto.** Three of four Factions currently cannot win if one seat
holds the Lead.
*Options:* (a) give the AI a reason to build Labs, so the Lead is a real race; (b) let a
Faction's own gate open to it alone, paid from its own Research, outside the shared tree;
(c) let the Lead pick from a shortlist of two or three rather than anything.
*Trade-off:* (a) is the honest fix and slows everyone's tech; (b) removes the gates' bite as
a shared cost; (c) keeps the Lead's agency and kills the freeze.
*Separately:* refuse to end a turn while a pick is owed — the freeze should not be legal.

**3. Re-fit every Victory bar, after 1 and 2.** No figure fitted against today's AI means
anything: the observed 450 Fund ceiling measures the AI, not the bar.
*Trade-off:* raising a bar alone lengthens the autopilot instead of removing it, which is
why this is third and not first.

**4. Decide what "off Earth" means for Diaspora.** A station over Earth counting as off it
(ticket #85) means thirty Colonists is two Habitats, 36 Materials and no flight at all.
*Options:* (a) count a station over Earth for Off-world Presence but not for Diaspora's
thirty; (b) count them at half.
*Trade-off:* (a) gives the Arkwrights their own reading of the phrase; (b) is one rule for
everyone but still leaves orbital Habitats cheapest.

**5. Give the Archivists a way out of a blockade.** Their win can be locked away by a single
rival warship, with no counterplay and no way to have foreseen it on turn 9.
*Options:* (a) resolve cargo before builds, so winning the battle wins the turn; (b) let the
Archive be decommissioned and rebuilt, half Materials back and the fund kept, so a blockade
is expensive rather than terminal; (c) always let a Faction land at a place it controls.
*Trade-off:* (c) makes blockade toothless; (a)+(b) together fix the galling case and leave
the mechanic standing.

**6. Fix the funding bug** — it is the only item here that is unambiguously a defect rather
than a balance question. *Options:* (a) bank `research_last_turn` outright, subtracting from
wherever Income put it; (b) make funding a declaration set before Income, like the
Prospectors' Venture share; (c) at minimum, say in the Report why the figure was zero.

**7. Put something in the middle of the game.** Turns 11 onward are one repeated decision at
every seat. *Options:* (a) rising costs or upkeep so the optimum keeps moving; (b) events or
pressures that arrive on a schedule the player must answer; (c) a shorter game.
*Trade-off:* (c) is honest about what the game currently is, and cheap, but gives up the
thirty-six turns 0.05.5 bought.

**8. Smaller, independently useful:** Leapfrog is about ten times mispriced (550 Ducats for
2.10 ppm against a Scrubber's 60 Ducats for 3.00 ppm — it was bought zero times in twelve
games); a floor on the Ducat formula so a held state always pays something, since nine of
twelve states pay zero and the whole Trading window is shut from a low-GDP start; show
*whose* emissions drive the Collapse projection, turning a doom counter into intelligence
about rivals; and write down the eight unfindable rules above.

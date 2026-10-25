# Dying Earth — version 0.1.0.5, the pushback version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.1.0.5](https://github.com/whaleyjoshua2/Dying-Earth/issues/606), and the
pictures and sweeps that decided it are in
[`docs/dev-diary/2026-10-24-version-0.1.0.5/`](../dev-diary/2026-10-24-version-0.1.0.5/).

**The baseline** is 0.1.0.4's closing board,
[`final-0.1.0.4.txt`](../dev-diary/2026-10-21-version-0.1.0.4/sweeps/final-0.1.0.4.txt):
**23 / 42 / 4 / 10 and 1 collapse**, against the designer's 15 each and 20.

**What the version is.** Version 0.1.0.4 with the designer's list of 2026-10-24, most of it pushing
back: the computer seats contesting a rival's climb, defending their own Regions, weighing Collapse
and packing at 40%; the interface pushed back to its figures. (The summary and the win column are
written by the closing ticket, as each section lands.)

## 1. Every off-world population bar up by 2, and the Archive at a place of Colony tier

Ticket: [Every off-world population bar up by 2](https://github.com/whaleyjoshua2/Dying-Earth/issues/625).

The designer: *"lets raise the offworld pop requirement by 2 across the board"*; and, asked whether the
Archivists' Upload counts, *"leave at 16 they go in groups of four but they need something too"*;
offered a tier under the Archive, living people kept beside the Upload, or a Research cost rising with
the count: *"option 1 but make it a colony"*.

- **Off-world Presence is 20** (`victory.toml`, from 18): the Custodians' and the Prospectors' second
  part, one figure. The Diaspora is **44** (`factions.toml`, from 42), **the four on each of four Bodies
  kept**: it was first built at six on a misread answer and put back at the designer's word the same
  day (*"I'm pretty sure i'd asked to keep Diaspora with four on each of four bodies"*). The Upload
  **stays at 16**, in fours.
- **The Archive may be ordered only at a place of Colony tier** (`modules.toml [archive]
  tier_to_order = "Colony"`, in place of `colonists_to_order = 4`). The tier is read, not the people:
  a place never falls back a tier, so eighteen who climbed it and then left still leave a Colony;
  a Colonial Capital counts, a station never (the Archive is ground-only already). Checked once, at
  the order, as the four-Colonist rule was. The refusal reads *"the Archive wants a place of Colony
  tier; Olympus Mons on Mars is an Outpost"*. The loader refuses a tier name that is no tier's.
- **The computer's paces** move with the bars on the same turns (`ai.toml`): every people pace to
  `[[17, 7], [31, 20]]`; the Arkwrights' first x 44/42, `[[8, 6], [17, 18], [25, 32], [33, 44]]`; the
  Archivists' unmoved. The test that holds every schedule to its bar passes.
- **The computer Archivists** name only a Colony-tier place for the Archive, and their ferry's
  spending lift (the x5 on a Habitat at the Archive's Body, ticket #361) now stays on until a
  Colony-tier site stands, where it paused at the first Outpost that could hold the Archive.
  Measured with the rule alone (`sweeps/ticket-625-ferry-paused.txt`): **25 / 52 / 2 / 0 and 1**, no
  computer Archive standing in any of 80 games (43 of 80 on the baseline), the Archivists' fund full
  at 250 in every game with nowhere to spend it.
- **The texts that carried the figures**: the long `victory` sentences on the Custodians', Arkwrights'
  and Archivists' rows (20; 44 and 6; "at a place of Colony tier"), the tutorial's two notes (twenty),
  the tool's `build archive` help, the glossary (Diaspora, Off-world Presence, the Archive, the
  Archivists).
- **Swept** (`sweeps/ticket-625.txt`, 20 seeds x four seatings, the four on each Body kept): **26 / 48
  / 2 / 2 and 2 collapses**, from 23 / 42 / 4 / 10 and 1. The computer's Archive stands in **11 of 80**
  games at a median turn 35 (43 of 80 at about 30 on the baseline; 0 with the ferry paused; 13 with
  six on each Body, `sweeps/ticket-625-six-each.txt`, 25 / 50 / 2 / 3), so the Archivists win 2 from
  10; the Arkwrights, at 44, 2 from 4; the Prospectors take the games both lose, 48 from 42, six
  past noise. Colony-tier places at the end over the batch 34 (23 on the baseline), Capitals 28 (16).
  **The miss is reported and left**: the Colony tier is the designer's word, and the computer
  Archivists reach it in about one game in seven; what more the computer should do to climb, or
  whether the tier should be the Settlement's, is the designer's call.

## 2. The pack on a runaway leader forms at 40% of Victory

Ticket: [The pack on a runaway leader forms at 40% of Victory, not 50%](https://github.com/whaleyjoshua2/Dying-Earth/issues/621).

The designer: *"Drop threshold for gang-ups from 50% to 40%"*.

- **`gang_from` is 0.4** (`ai.toml [thresholds]`, from 0.5; ticket #531's pack). A rival is a seat's
  leader at 40% of its Victory or more and 20 points or more above the seat (`gang_ahead` 0.2, as it
  was), and the leader's ramp runs 0 at 40% to 1 at the end, so a leader at 80% reads two thirds
  where it read 0.6; the seat's trail over `gang_full` (0.3) is unchanged. Every hostile act on the
  leader weighs x(1 + pressure), the leader is cause from half pressure, no Accord or Trade with it,
  the pack's own Accords `gang_accept_relax` (3) x the pressure easier, as before.
- **The tests** that pinned the ramp at 0.6 and the lift at x1.6 moved to two thirds, watched red on
  the old figure first; a leader at 45% is a leader now, at 39% not.
- **Swept** (`sweeps/ticket-621.txt`, 20 seeds x four seatings): **25 / 50 / 2 / 2 and 1 collapse**,
  from the same day's 26 / 48 / 2 / 2 and 2 (§1): noise. The pack forming ten points sooner did not
  bite on the Prospectors, who lead most games from the Moon's claim: Accords struck over the batch
  620 from 620, games with an orbital Battle 24 from 27. The computer's Archive 9 of 80 (11 before),
  noise. Reported and left: the figure is the designer's word, and what the pack does with its
  earlier start is a question for the contest and defence tickets, which give it Influence to spend.

## 3. An Occupation halves the prior holder's Standing

Ticket: [An Occupation halves the prior holder's Standing](https://github.com/whaleyjoshua2/Dying-Earth/issues/615).

The designer: *"upon occupation prior holder loses half their influence"*; on the ticket's questions,
*"q1 that q2 yes q3 loss stands"*.

- **The turn an Occupation begins**, the prior holder's Standing at that place is halved, rounded down
  (`influence.toml [influence] occupation_standing_kept = 0.5`, the share kept). A neutral place has no
  prior holder and nothing halves. Their Allotment and stockpile are untouched; so are the occupier's
  banked Standing and Pacification, which reads the occupier's Standing against the bare threshold.
- **Every Occupation** halves it again, by another seat or the same one after a break: 41 to 20 to 10.
- **A broken Occupation** hands the place back and restores nothing: the loss stands.
- **The Report's occupation line** carries it where a holder lost: *"Arkwrights occupy Japan; Custodians'
  Standing there falls to 20."* (`occupation_begun_held`), read by every seat; a neutral place keeps the
  plain line. The tool prints the Report. No Moment of its own.
- **The computer** reads Standing from the table: a prior holder's retake is priced on what it has left,
  and nothing in `ai.rs` carried a copy.
- **Swept** (`sweeps/ticket-615.txt`, 20 seeds x four seatings): **26 / 49 / 2 / 2 and 1 collapse**,
  from the same day's 25 / 50 / 2 / 2 and 1 (§2): noise. Occupations begun over the batch 204 from
  197, broken 37 from 36: the computer occupies no more and no less for the halving, and the win
  column does not move. Reported and left.

## 4. One more Colony Slot on the Moon, Phobos and Deimos

Ticket: [One more Colony Slot on the Moon, and one each on Phobos and Deimos](https://github.com/whaleyjoshua2/Dying-Earth/issues/613).

The designer: *"add one additional slot for a lunar colony"*, *"add one on Deimos and Phobos as well"*;
the names as proposed, *"those three"*.

- **Three rows in `bodies.toml`**: the Moon's fifth slot **Oceanus Procellarum** (lon -57, lat 18),
  Phobos's third **Todd** (lon -153, lat -9), Deimos's second **Voltaire** (lon 3.5, lat -22). A slot
  carries no yield of its own; the Body's yields are per Colony, and the computer reads the free slots
  from the table, so the Moon stays open one founding longer and the Mars moons offer two more landings.
  `SAVE_VERSION` stays at 12: a 0.1.0.4 save loads with the new slots free, their yields the Body's
  flat figures rather than a drawn spread, since the spread is drawn at a game's start.
- **The pictures** (`ticket-613/slots-moon.png`, `slots-phobos.png`, `slots-deimos.png`): each globe
  with its new site drawn clear of the others.
- **Swept** (`sweeps/ticket-613.txt`, 20 seeds x four seatings): **29 / 51 / 0 / 0 and 0 collapses**,
  from the same day's 26 / 49 / 2 / 2 and 1 (§3). The Arkwrights' and Archivists' two each to nought
  is within noise, but their seats have now read 2, 2, 2, 0 and 0 across the version's five sweeps.
  The landings moved as the slots meant them to: ground Colonies at the end over the batch on Phobos
  165 from 127, on Deimos 57 from 40, on the Moon 290 from 270. Reported and left.

## 5. The Strip Permit: more CO2, a point off with every rival, and no growth while it runs

Ticket: [The Strip Permit: a little more CO2, a little more damage to Relations](https://github.com/whaleyjoshua2/Dying-Earth/issues/622).

The designer: *"Nudge strip permit a bit more co2 output and a bit more damage to relations"*; on the
ticket's questions, *"q1 a q2 c I don't want it to break an accord; Can we also add a 0% pop growth
during the strip minned turns"*.

- **The permanent rise is 0.3** in the Region's Baseline Emissions when the permit ends
  (`nation_states.toml [strip_permit] baseline_rise`, from 0.2); the three doubled turns emit as they
  did.
- **A point off Relations with every rival** the turn the permit is spent (`relations = 1`): a flat
  fall on the deeds score, as a card's is, and never an Offence, so no Non-Aggression Accord breaks
  over it and nothing goes on the Offence ledger.
- **No growth while it runs**: a Region under a running permit does not grow for its three turns; the
  heat's loss, folded into the same rate, still bites. Growth returns when the permit ends.
- **The Report**: *"Prospectors issued a Strip Permit in Brazil: its Facilities produce double for 3
  turns, nobody grows there meanwhile, and 1 point off with every rival."* The tool prints the same.
- **The computer Prospectors** weigh the permit as before and read every figure from the table.
- **The texts**: the Prospectors' signature sentence (0.3, the point, no growth), the order's note, the
  running permit's hover ("and no growth"), the glossary.
- **Swept** (`sweeps/ticket-622.txt`, 20 seeds x four seatings): **27 / 49 / 3 / 0 and 0 collapses**,
  from the same day's 29 / 51 / 0 / 0 and 0 (§4): noise. Accords struck over the batch 616 from 613,
  so the point off breaks nothing and costs the Prospectors no partner. Reported and left.

## 6. Every Facility and Module costs a fifth more Widgets, and a Frigate 6

Ticket: [Every building costs a fifth more Widgets, never less than one more, and a Frigate 6](https://github.com/whaleyjoshua2/Dying-Earth/issues/610).

The designer: *"20% increase in widget cost for buildings rounded to nearest integer but no building
get less than a +1"* and *"A frigate is 6 widgets"*; on the ticket's questions, *"q1 facilities and
modules only q2 frigate alone"*.

- **Every Facility and Module row's Widgets** rise by a fifth, rounded to the nearest whole
  (`facilities.toml`, `modules.toml`): 4 to 5, 5 to 6 (the Nature Reserve), 8 to 10, the Archive's 12
  to 14 (14.4). The +1 floor binds nowhere, since 4 x 0.2 rounds up. The Core's nought stays.
- **The Colony tiers** (4, 6, 8) and **Industry Level** (4, +1 a step) stay: a place growing is not a
  building, and the Capital was repriced in 0.1.0.4.
- **The Frigate is 6 Widgets**, from 4; every other Ship, the Army and a Rearm stay.
- **What follows from the tables**: the Prospectors' 15% off and the Arkwrights' Module discount floor
  the new rows (a Power Plant 8 for the Prospectors, a Shipyard 7 for the Arkwrights); the Archive
  takes four Resolutions at a Core's four Widgets a turn, where it took three; every hover reads the
  figure through its placeholder, and the computer reads every price from the table.
- **Swept** (`sweeps/ticket-610.txt`, 20 seeds x four seatings): **25 / 51 / 2 / 0 and 2 collapses**,
  from the same day's 27 / 49 / 3 / 0 and 0 (§5): noise. Dearer builds take their fifth where it
  shows: Widgets applied a game (median, the player's seating) 353 / 517 / 180 / 174 from 302 / 440 /
  158 / 167, and production over the batch, gross, Materials / Energy:

  | | Custodians | Prospectors | Arkwrights | Archivists |
  |---|---|---|---|---|
  | before (§5) | 175,237 / 295,249 | 190,479 / 234,734 | 62,935 / 136,177 | 39,090 / 81,859 |
  | after | 164,820 / 276,074 | 178,668 / 229,114 | 62,393 / 132,979 | 36,970 / 81,883 |

  Six per cent off the Custodians' and the Prospectors' Materials, the Arkwrights' and the Archivists'
  within two; the computer's Archive stands in 3 of 80 (9 before), at the edge of its noise. Reported
  and left.

## 7. Turn one's signature moves: one sentence, the player's Faction alone

Ticket: [Turn one's signature moves: one sentence, the player's Faction alone, a direction for turns one and two](https://github.com/whaleyjoshua2/Dying-Earth/issues/607).

The designer: *"signature moves - make a sentence and show only the selected faction and give them some
direction to go in for the first and second turn e.g. build power plant, recruit pioneers"*; on the
ticket's questions, *"those but remove the Turn one:/Turn two: q2 yes"*.

- **Each Faction's `opening_moves`** (`factions.toml`) is two sentences, the first turn's orders then
  the second's, with no turn labels:
  - Custodians: *"Build a Scrubber at home and spend your Influence. Leapfrog a Region you hold and
    recruit Pioneers."*
  - Prospectors: *"Build an Investment Bank and set your Venture share. Raise Industry Level and
    recruit Pioneers."*
  - Arkwrights: *"Build a Space Station over Earth and recruit Pioneers. Put a Shipyard on it, for
    the Colony Ship that takes them to the Moon."*
  - Archivists: *"Build a Research Lab and set the Directive. Recruit Pioneers and build a Habitat on
    Axiom."*

  The Custodians' Leapfrog is refused before turn four (Three Turns Held) and costs 50 Ducats they do not
  open with; raised from the review, the designer kept the line: *"just leave it"*. The last two at the
  designer's second word (*"Archivists needs to say Build a Research Lab and Set
  Directive. Recruit Prioneers and build a Habitat on Axiom. For the Arkwrights just add the word Put
  at the start of the second sentence"*), "Set Directive" read as the window's own "set the Directive".
- **The turn-one window** shows the player's Faction alone: "Signature moves", the Faction's glyph
  and name, the two sentences under it, "Go on". The rivals' rows go; the Faction card's signature
  rule says what each does. The `{your}` placeholder goes with them, the line having one reader.
- **The text tool's `new`** prints the one line, *"Custodians (you): Build a Scrubber ..."*.
- **The pictures** (`ticket-607/<faction>-report.png`, four): the window for each Faction, over the
  Earth map where the Report would stand.
- No rule moves; no sweep.

## 8. The Region card's threshold lines cut to the hover, and one hover per element

Ticket: [The Region card's threshold line reads as a subtraction, and its Standing and threshold hovers stack](https://github.com/whaleyjoshua2/Dying-Earth/issues/608).

The designer: *"threshold calculation is off e.g threshold 31 next line 31-20+10 for size 1 +1 for
regions held. what is being calculated here because its not threshold that always the first term
regardless of cost. do we even need this line?"* and *"mouse over on standing: or threshold on nation
card procs two mouse overs one on top of another, check and fix"*; on the ticket's questions, *"q1
cut it move to hover q2 yes"*.

- **The card's two weak lines go**: *"Threshold 31 - 20 + 10 for size 1 + 1 for Regions held"* and,
  on a held place, *"Held by the Archivists: 58 - 38 + 20 (margin) + 5 (Constabulary)"*. Their dash
  was a separator read as a minus; the arithmetic was right. The challenger sentence (*"The
  Prospectors stand at 31; they take this at 54."*), the threat line and the conversion line stay.
- **The terms ride the threshold label's hover as sums**, its first lines: *"Threshold 31 = 20 + 10
  for size 1 + 1 for Regions held"* (then *", x1.10 (Blame)"* and *", cut by Green Consensus"* while
  they bite), and on a held place *"The Prospectors take it at 58 = 38 (your Standing) + 20 (margin) + 5
  (Constabulary)"* (the cheapest rival named) or *"Take at 58 = 38 (their Standing) + 20 (margin)"*;
  where the threshold stands over that sum the line says so. The sentence that said the same in
  words is cut, the rule line is one rendered line, and a held place's tip carries no further
  sentence, so every tip stays under six lines with a Constabulary, relations, Blame or Green
  Consensus in it (the review found the first shape could reach seven or eight).
- **One hover per element**: the row's own tip, which fired under the label's and the Standings
  chips', goes, for the player and the spectator alike; its decay figures are on the Standings tip.
- **The pictures** (`ticket-608/neutral-earth.png`, `own-earth.png`): the hover on a neutral
  Region's "Threshold 40" and on the player's own Region's "A rival takes it at".
- The tool prints its own Standing line and no breakdown; untouched. No rule moves; no sweep.

## 9. The top bar's Widgets hover in the Fuel's shape, and the Energy hover by place

Ticket: [The top bar's Widgets hover in the Fuel's shape, and the Energy hover naming where it comes from](https://github.com/whaleyjoshua2/Dying-Earth/issues/609).

The designer: *"the n/x configuration in the top bar for widgets what does applied at last resolution
mean can we clean up that mouse over its very clunky, make it closer to the fuel or duckets one"* and
*"fix mouse over on power too I want to see where i'm getting it than have a line that just says
upkeep -x only show moduls in danger of going off line if the next turn will result in negative
energy"*; on the ticket's questions, *"q1 yes but keep the /n part q2 b"*.

- **Widgets**: the figure stays *"12 / 10"* (made a turn, applied at the last Resolution). The hover
  is the Fuel's shape: *"Made a turn: 12"*, then one line a maker, *"+4  the Factory (China)"*, *"+4
  the Region itself (China)"*, *"+4  Core Module (ISS over Earth)"*, closing *"10 went into builds
  last turn."* The per-place queue lines go; every hatched tile carries its own.
- **Energy**: the Income keeps, beside its source lines, each place's Energy made and upkeep
  (`energy_by_place`, a `serde(default)` field recomputed every Income, so `SAVE_VERSION` stays), the
  Ships' and Armies' upkeep as a place of its own. The hover is one line a place, net, the parts in
  brackets: *"Last Income:"*, *"+8  China (12 made, 4 upkeep)"*, *"-1  ISS over Earth"*, *"-3  Ships
  and Armies"*. While this turn's orders forecast a shortfall the figure turns red as before, and what
  goes dark is appended under the lines, where it replaced them. From the review: a Reactor's relief
  is a line of its own (*"+6  Reactor relief"*), so the lines sum to the Income's Energy while one
  stands; a line is bare where only one side has a figure; and the places past the fourth fold into
  one net line (*"+12  3 more places (30 made, 18 upkeep)"*), so the hover is six lines at most
  before the dark list. The makers' lines read *"+4  the Factory (China)"*, the place in brackets.
- **The pictures** (`ticket-609/widgets-earth.png`, `energy-earth.png`): the two hovers on turn one.
- No rule moves; no sweep.

## 10. Three texts: the Cheap Ore refusal said once, "breaks Accord", a Refinery's refuelling said on its card

Ticket: [Three texts: the Cheap Ore refusal said once, "breaks Accord", and a Refinery's refuelling said at the start](https://github.com/whaleyjoshua2/Dying-Earth/issues/611).

The designer: *"cheap ore offer rejection text is says the same thing twice"*; *"rather than 'breaks your
accord with prospectors' it should just say 'breaks accord'"*; *"Say at the start that a Colony with a
Refinery refuels ships in its low orbit. I found out at G2 T23 from a refusal message."*; on the
ticket's questions, *"q1 button q2 yes q3 b"*.

- **The Cheap Ore Offer** (`events.toml`): the refuse button's face is *"Let it go"* alone; the figure
  stands once, on the line the card generates under it (*"Materials a third off for you next turn"*),
  which the old face repeated. (A choice card's `effect` field is drawn nowhere, so it is untouched.)
- **"breaks Accord"**: the red note beside the Influence button, the armed button (*"Break the Accord
  and spend 17"*), the tool's price line (*"17; breaks Accord"*) and its refusal (*"breaks Accord:
  order it with break to break it"*); the partner's name nowhere, the Accord row naming them.
- **The Refinery**: its row's card line (`modules.toml`, `does`) reads *"refuels its Faction's Ships
  in low orbit"*, drawn through the yield's text on the tile's line and the build hover's "once it
  stands" line, as the Habitat's is; owner-neutral, since a rival's tile draws the same line. The text
  tool's `refuel` help names a Colony with a working Refinery beside a station.
- **The pictures** (`ticket-611/ore-earth.png`, `accord-earth.png`): the card with its bare refuse
  button over the generated line, and "breaks Accord" in red beside the Spend and Max buttons under
  a Non-Aggression Accord with the Region's holder. The Refinery's line has no picture: no aid plants
  a Refinery Colony on turn one's board.
- No rule moves; no sweep.

## 11. The Scrubber cap: one per 200 million

Ticket: [The Scrubber cap by population: more than one in most Regions](https://github.com/whaleyjoshua2/Dying-Earth/issues/612).

The designer: *"check scrubber build count it seems its only one per country should be based on
population"*; on the ticket's questions, *"q1 200 q2 as they are"*.

- **The cap was already by population**: a Region's people over `per_population`, rounded, within a
  floor of 1 and a ceiling of 10; at 300 million a Scrubber fifteen of the twenty Regions held one.
- **`per_population` is 200** (`facilities.toml [scrubber]`, from 300): India 8, China 7, Nigeria, the
  European Union and South Africa 3, Indonesia, the United States and Iran 2, the twelve Regions
  under 300 million one still. The floor of 1 and the ceiling of 10 stay.
- The computer Custodians build to the cap; the card's *"Scrubber (1 of 2)"* and the signature
  sentence read the figures from the table.
- **The picture** (`ticket-612/cap-factions.png`): the Custodians' Faction card, whose signature rule
  reads the cap from the table, *"one per 200M people, 1 to 10"*; the Region card's *"Scrubber (2 of
  7)"* button sits below the fold at the shot's height.
- **Swept** (`sweeps/ticket-612.txt`, 20 seeds x four seatings): **23 / 53 / 1 / 0 and 3 collapses**,
  from the same day's 25 / 51 / 2 / 0 and 2 (§6): noise. Scrubbers built over the batch 2,262 from
  2,113, seven per cent more; the end Temperatures by seating +2.47 / +2.29 / +2.23 / +2.04 from +2.45
  / +2.24 / +2.18 / +1.99, and the Natural Sink at the end mixed, both ways within noise. The seven per cent
  reads as the cap binding in few Regions, an inference from the count, not a measured one. Reported and left.

## 12. The Battle Moment says who won

Ticket: [The Battle Moment says who won: a Battle you won read as one you lost](https://github.com/whaleyjoshua2/Dying-Earth/issues/614).

The designer: *"getting a popup I lost a battle I did not lose - my tsv defiant beat the pmv valiant"*;
on the ticket's questions, *"q1 a q2 yes"*.

- **The card as it was**: *"A Battle at Mars low orbit: PMV Valiant destroyed"* over the figure *"1
  lost"*, the figure counting everybody's losses, so a Battle the player won read as theirs. The
  designer's own game was not replayed (no save was to hand): the diagnosis is read from the code,
  and the test builds the case by hand, the Defiant sinking the Valiant and losing nothing.
- **"Won" is read off the losses**: the side that lost nothing while the other lost a unit; both
  losing, or neither, is no verdict. The engine records no field-holder and starts none.
- **The wording**: with the player's units in it, *"Battle at Mars low orbit: your TSV Defiant
  destroyed the Prospectors' PMV Valiant."* and the figure *"you won"*, or *"the Prospectors' PMV
  Valiant destroyed your TSV Defiant."* and *"you lost"*, or both sides' losses named and *"both sides
  lost a unit"*; with the player out of it, *"the Custodians' ARK Defiant destroyed the Prospectors' PMV
  Valiant and PMV Aurora."* and the figure *"2 destroyed"*.
- **The record**: each party now keeps its units' names beside its losses (`BattleParty.names`, a
  `serde(default)` field, so `SAVE_VERSION` stays); a party without them, or a neutral one, or a
  three-sided Battle, reads the losses alone; a unit that escaped is no destroyer and is left out of
  the names, unless every unit of its side escaped, when the side is named whole (the kills were made
  before it left; the diary shot read *"your  destroyed"* otherwise). A bloodless Battle is no Moment,
  as before. A bombardment's and a Launch's Moments keep
  their own sentences, and their figures count *"destroyed"* where they read *"lost"* (from the
  review: the same misreading on those two cards). The Report line is unchanged.
- **The picture** (`ticket-614/battle-report.png`): the card from the `battle:two` aid, which brings
  seats 0 and 1 alone to Mars: *"you won"* over *"Battle at Mars orbit: your TSV Valiant and TSV
  Vanguard destroyed the Prospectors' PMV Magellan and PMV Indomitable."*
- No rule moves; no sweep.

## 13. Nine hovers cut to the designer's lines

Ticket: [The window's hovers cut: the Accord's notice, the first landing, Pioneers waiting, the Blockade, the Directive, the standing order, the Allotment, Unrest, Overcrowded](https://github.com/whaleyjoshua2/Dying-Earth/issues/616).

The designer's cuts are on the map's Notes, from the 0.1.0.2 playtest's mouse-over dump; on the
ticket's questions, *"q1 c q2 that q3 yes q4 keep 587s q5 b q6 b q7 yes"*.

- **The Accord's notice** (the *"Declare it over"* button): *"Lapses next turn. Breaking a term sooner
  costs 3 Relations and ends it."* Both facts of the two cuts given; the 3 is `break_accord`'s offence
  weight.
- **The first landing** (a Body card's windfall line): *"First to land here: +{standing} Influence a
  turn while held."* over a second line, *"A station claims nothing."*, the one clause of the old
  hover that bites.
- **Pioneers waiting**: *"Waiting. A Launch Site lifts them; once the ice opens, ships take them to
  Antarctica."*
- **The starved line**: the line already names the cause and the squeeze (*"Blockaded by the
  Prospectors: producing nothing, upkeep still paid."*), so the hover says what it does not: *"Nobody
  dies, nothing is destroyed. Each turn is an offence against you."* The designer's cut repeated the
  line.
- **The Directive** (both the condensed widget and the Faction window's control): *"Share of your
  Research paid into the shared Tech from the next Income. Research kept back counts nothing toward
  the Research Lead."*
- **The standing order** (the *"every turn"* checkbox): *"Spend your whole Allotment here every turn
  until cancelled."*
- **The Allotment** (a Region card's Influence value): the figures by source, the top bar's "of N
  left" lines, on one line: *"Influence each turn: Regions and buildings 14. Unspent is lost."* On
  turn one, before any Income has recorded them, `allotment_lines` reads the parts live, so the top
  bar's hover has them on turn one too (from the review, which found the fallback copied in the
  window).
- **Unrest**: *"Unrest to 10: Army stops at 4, Facilities halve at 7, revolt at 10. Falls 1.5 a
  turn."*, the figures from `unrest.toml`.
- **Overcrowded**: *"Overcrowded: everything but Energy at half. Restart a Habitat."*
- **The Upload** keeps ticket #587's line, *"Read into the Archive; {} stay. Cannot be undone."*, the
  later word.
- **The Blame tag**: *"[BLAME_HOVER]"* is the dump's label for the constant; the screen draws it bare
  (`ticket-616/blame-climate-earth.png`). The dump was made by hand from the source, which is why the
  developer notes (two code comments and four test names) appear in it; none is drawn.
- Every cut hover is drawn through `rule_tip`, so the `tip:` aid can photograph it; the standing
  order's greyed box keeps its own words, *"Click a Region or a Colony first."*, by a branch (from the
  review, which found it left on `on_hover_text`). The starved and overcrowded lines have no picture:
  no aid starves or overfills a Colony on turn one's board.
- **The pictures** (`ticket-616/`): `pioneers-earth.png`, `unrest-earth.png`, `allotment-earth.png`,
  `order-earth.png` (China's card and the Influence block's checkbox), `landing-moon.png` (the Moon's card), `accord-factions-mars.png` (the Prospectors'
  page under `accord:some`), `directive-tech-mars.png` (the Tech Tree window), `blame-climate-earth.png`
  (the Climate Panel's heading).
- No rule moves; no sweep.

# A chronicle at the end of the game: the ranking, the table of nine and the two charts

Ticket [#338](https://github.com/whaleyjoshua2/Dying-Earth/issues/338) on version 0.09.0. The
designer's resolution comment on the ticket is the authority. It took none of the three narratives
the ticket was charted with and named what it wanted instead: **the final resource tallies of each
Faction in a table, with the population and temperature graphs** -- so no dated record is built,
nothing new is kept per turn, and both charts are the ones the top bar's hovers already draw. The
lift fell from medium-to-large to small, and the whole of it is interface: one read-only accessor in
the engine and a page in `src/ui.rs`.

| picture | what it shows |
|---|---|
| [`gameover-earth.png`](gameover-earth.png) | `shot:gameover seed:7 turns:36 window:1500x900`. **The way in.** The game-over box exactly as it was -- the outcome, the turn and seed, the four seats in seat order -- with one button added at the head of its row: **Chronicle**, beside Title screen and Quit. |
| [`win-chronicle.png`](win-chronicle.png) | `shot:win seed:7 turns:36 chronicle:1 window:1500x900`. **The page, from a game somebody won.** *The Prospectors win: met its Victory Condition.* The four Factions ranked as the End phase ranks them, the winner's sentence reading *won on their Victory Condition: Venture Capital Fund 2547 of 2500, and 20 of 12 Colonists living off Earth*, and three losing sentences naming the bar each missed and the gate Tech that held it back. Two rows carry a tiebreak line: the Custodians below the Arkwrights on Colonists off Earth, and the Archivists level with the Custodians on the score and on both tiebreaks, which the page says outright. Then the table of nine and both charts. |
| [`lastturn-chronicle.png`](lastturn-chronicle.png) | `shot:lastturn seed:3 turns:36 chronicle:1 window:1500x900`. **A game won when the turns ran out**, the other way the End phase ends one: *The Prospectors win: higher score at the last turn*, and the winner's own sentence saying so and naming what it was still short of -- Venture Capital Fund at 1963 of 2500. This is the ending the ranking exists for. |
| [`collapse-chronicle.png`](collapse-chronicle.png) | `shot:collapse seed:1 turns:36 chronicle:1 window:1500x900`. **A Collapse**, the third ending: *Collapse. Nobody wins.* Every Faction's sentence reads *ended in the Collapse at +3.0 C, short on ...*, and the ranking still orders the table, twice on the first tiebreak. |

Each `shot:` run also writes `<prefix>-factions.png`, the Faction choice screen the harness
photographs on its way past; those were deleted. `chronicle:1` takes ONE picture of the page rather
than seven copies of it, because the page fills the window and the seven views are behind it.

## What was built

- **`Screen::Chronicle`** in `src/app.rs`: a page of its own, at the designer's word, and not a box
  over the board. While it stands neither the board nor the game-over box is drawn at all. The
  Chronicle button on the box opens it and Back at its head goes back to the box; both travel as
  `Action::ShowChronicle`, since the box is drawn from a read-only Session.
- **`Game::ranking`** in `engine/src/state.rs`, the one engine change: the four seats in the order
  the End phase ranks them, each with what separated it from the seat above -- the new `Tiebreak`
  enum, whose three keys are `end_phase`'s own three in its own order (the score, then Colonists off
  Earth, then Colonies held) and whose fourth says none of them told. The page reads this rather
  than ranking for itself, so it cannot order a table differently from the rule that decided the
  game. Read-only: nothing in it moves the game.
- **A sentence per Faction**, `chronicle_sentence` in `src/ui.rs`: won on its Victory Condition, won
  when the turns ran out, met it and did not take the game, reached both bars and was held back,
  ended in the Collapse, ended in a draw, or lost on the bar it missed. Every figure in it is the
  engine's own off `Progress` -- the part still short is `short_part`, and a first part held back
  says so in the engine's words. **These sentences live in `src/ui.rs` and not in
  `assets/data/report.toml`** because ticket #339's lane held that file while this was built.
- **The table**, nine figures a row, every one already kept by the game: `stockpile.materials`,
  `.fuel`, `.energy` and `.ducats` at the end; `off_world_colonists`; `directed_states().len()`;
  `directed_colonies().len()` (Colonies and stations counted together, as the Faction window's
  Holdings block counts them); `research_total`, which is what was produced over the game and not
  what is held now; and `blame` with `blame_share` beside it. Every header carries a hover saying
  what its column is.
- **Both charts reused, neither rewritten**: `population_history` and `temperature_history`, the top
  bar's own, drawn side by side at the size the page allots them. Their time axes carry the in-game
  date, as they already did and as every chart in this game does.
- **`chronicle:1`**, the `shot:` name the game-over screen had never had. It wants `turns:<n>` enough
  to play the board out; `seed:<n>` is what makes a particular ending reproducible.

## Looked at

The four pictures above, opened and read before this was written. Three things were caught by
looking, and the first two are fixed in what is above:

- **A seat at 100% that was "short on" a part standing over its bar.** The first capture read *The
  Prospectors ended in the Collapse at +3.0 C, 100% of the way there, short on Venture Capital Fund
  at 2681 of 2500* -- a contradiction, because the score is a fraction of the bars and clamped, so a
  seat whose gate Tech is unresearched sits at 100% with both bars passed. The sentence now says
  what actually held it back, in the engine's own words.
- **A last-turn winner said to have won on its Victory Condition**, which it had not met. The
  winner's sentence now branches on whether the Condition was met, and the engine's margin note is
  left off where it would only repeat the clause before it. The percentage came out of every
  sentence at the same time: the row's heading carries it a line above.
- **The Temperature chart's two right-hand labels can collide**, the Collapse line's `+3.0` and the
  line's own end figure, when a game ends within about a tenth of a degree of the line --
  `win-chronicle.png` and `lastturn-chronicle.png` both show it. It is the chart's own behaviour on
  the top bar's hover as well, not something this page introduced, and the ticket said to reuse the
  charts and not rewrite them, so it is left for a ticket of the chart's own.

Not photographed: the click that opens the page and the click that goes back (headless, no pointer),
and a draw, which no seed among the ten probed produced. The three endings the engine can reach --
met, last turn, Collapse -- are all above.

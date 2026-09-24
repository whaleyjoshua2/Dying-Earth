# Ticket #337: cards that ask a question. The build specification

Authority: the resolution comment on
[ticket #337](https://github.com/whaleyjoshua2/Dying-Earth/issues/337), decided by the designer on
2026-09-23 and 2026-09-24. Where this file and that comment disagree, the comment wins and this file
is wrong. The eighteen cards and their figures live in that comment; this file says how they are
built, not what they say.

## Prior art, searched before writing

- **This tree**: `assets/data/events.toml` holds 22 kinds over 40 cards, fourteen kinds carrying
  eighteen extra copies (counted from the file). One card a turn is drawn for the whole table in
  **phase 5** (`engine/src/events.rs:65-92`), *after* every seat's orders are committed in phase 4,
  at a chance rising with the Temperature; the deck is never reshuffled and the off-Earth cards join
  at turn 12. **No event carries a choice.** Effects are applied in `apply_event_now` at Resolution
  step (h), or read as flags before the step they touch: Solar Storm holds every transit and Comms
  Blackout rewrites every stance (`resolution.rs:68-88`). Next-turn effects have dedicated fields
  cleared at the next Income (`card_emissions_next`, `drought`, `storm_surge`, `solar_maximum_next`).
  **The only thing that blocks End Turn is the Tech pick**: `Game::end_turn_refusal`
  (`turn.rs:104-113`) returns a sentence, and the interface raises `Popup::Refused` and greys the
  button. The event modal (`src/ui.rs:8049-8067`) is the card's text and one Continue button, shown
  after the turn has already run. **The computer never sees a card**: `ai.rs` reads no event.
- **Every effect today is bespoke code with its own figures in the table.** Eighteen cards with two
  sides each would be thirty-six one-off effects; hence the vocabulary below.
- Nothing found that does what is specified; the finding is a negative.

## The rules

### R1. The deck

- Every `copies` beyond the first is removed: the 22 ordinary kinds keep one copy each.
- Eighteen **choice cards** are added, one copy each, so the deck is **40 cards, every card
  distinct**. A choice card is an `[[event]]` row like any other, marked as carrying a question.
- The draw chance, the off-Earth join turn and the never-reshuffled rule are unchanged. A choice
  card may be off-Earth if its effects need a place off Earth; none of the eighteen does, so all
  eighteen are in the deck from turn one.

### R2. A new phase: the question

- A phase runs **at the start of a turn, before orders** (a new phase 0, or the head of the Orders
  phase; whichever the turn loop takes more cleanly). It draws the turn's card. An ordinary card is
  held and applied exactly where it is applied today, so nothing about the existing 22 changes. A
  **choice card** becomes the turn's **pending question**.
- While a question is pending and unanswered for a human seat, **`end_turn_refusal` returns a
  sentence naming the card**, so End Turn is refused exactly as it is for the Tech pick. The AI
  seats answer when their orders are computed, so they never hold the turn.
- `Game::answer_card(seat, taken: bool)` records an answer. `Game::pending_question()` gives the
  card and which seats have answered.
- A seat the card cannot touch is **not asked**: its answer is recorded as *nothing to decide*, it
  does not block End Turn, and the Report says so for that seat.
- The answers are applied at the Resolution of the same turn, each in the place its effect belongs
  (a flag read before transits, a figure applied at step (h), a next-turn field set).

### R3. The effect vocabulary

A card is a name, a sentence, a **take** side and a **refuse** side. Each side is a list of effects
composed in data. The vocabulary, which is all the eighteen cards need:

| effect | what it does | cards |
|---|---|---|
| `resources` | signed Materials / Fuel / Energy / Ducats / Research | 3, 6, 7, 8, 10, 11, 14, 15, 16, 17 |
| `per_unit_cost` | a resource cost per thing of a kind (a Refinery, a Ship in orbit) | 10, 15 |
| `population_to_most_populous` | units of population into the seat's most populous held state | 1 |
| `emissions_next` | ppm added at the next Climate phase | 1, 12, 17 |
| `standing_all_held` | a change to the seat's Standing in every place it holds | 1 |
| `standing_at_most_populous` | the same at one state | 13 |
| `unrest_all_held` / `unrest_at_most_populous` | a change to Unrest | 4, 8, 18 |
| `hold_ships` | no transit resolves for the seat this turn (Solar Storm's shape) | 2 |
| `hold_one_ship` | one Ship of the seat holds this turn | 9 |
| `damage_ships` | so much damage to each Ship in orbit | 2, 15 |
| `trade_price` | a good's price set or moved for so many turns | 3, 7 |
| `relations_all_rivals` | a change to Relations with every other seat | 5, 9, 16 |
| `blame_ppm` | ppm onto or off the seat's own Blame | 5, 14, 16 |
| `widgets_now` | Widgets added this turn at the seat's busiest Region | 4 |
| `facility_output_multiplier` | a Facility kind makes this share at the next Income | 10, 12 |
| `discovery_at_colony` | a Discovery at one of the seat's Colonies for so many turns | 11 |
| `pioneers_free` | Pioneers waiting at no cost to the state's population | 13 |
| `free_building` | a named Module or an Army, free and costing no people | 17, 18 |

- **Every figure is a field of the effect in `events.toml`.** No figure is a code literal.
- `hold_one_ship` picks the seat's Ship with the most Fuel in its tank, deterministically, so a
  seeded game is unchanged by it. **An implementation choice, not the designer's**: named in the
  Built comment for correction.
- An effect whose target does not exist does nothing, and the card asks no seat for which **every**
  effect on **both** sides would do nothing.

### R4. The computer seats

- Each card carries a **rule per side** in the data, read by the AI: a predicate over the seat's
  board (its Unrest, its Ducats, whether a landing is under way, its Blame) deciding the answer. The
  figures of the predicate live in `events.toml` beside the card, not in `ai.toml`, since they are
  the card's.
- The seats answer when their orders are computed, before the Resolution.

### R5. The record

- The Report names each seat's answer at the card, in `report.toml` templates, and says *nothing to
  decide* for a seat that was not asked.
- The sweep counts, by seat: choice cards taken, refused, and not asked.

### R6. The save

- The pending card, which seats have answered and what they answered are saved. `SAVE_VERSION` is
  already 3 on this branch; it does not move again for this ticket unless something else forces it.

## Error cases

- `answer_card` for a seat already answered, or with no question pending: refused.
- End Turn with a question pending and unanswered for the human seat: refused with the card's name.
- A card row whose take side and refuse side are both empty: the load check refuses the table.
- A `trade_price` that would set a price outside the band: the band is overridden for the stated
  turns, which is the point of card 3, and the override is recorded so the band resumes after.

## Out of scope

- Changing what the 22 ordinary cards do.
- A card that targets a chosen rival: ticket #32 removed Faction-targeting cards deliberately and
  none of the eighteen restores it.

## Refutation

The specification is wrong if End Turn can be pressed with a question pending and unanswered; if an
ordinary card's behaviour changes; if the deck is not 40 distinct cards; if a seat with no state
held is asked the Hard Winter; or if two seats answering the same card differently produce the same
board.

## Red witnesses owed

One test per rule R1 to R5 in `engine/tests/formulas.rs`, each watched red against the current rule
or the rule reverted on purpose, with the failing assertion quoted in the lane's report. R2's
refusal and R4's rules each want one of their own.

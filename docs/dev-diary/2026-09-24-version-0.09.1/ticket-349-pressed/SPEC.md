# Ticket #349: a place you hold is Pressed. The build specification

Authority: the resolution comment on
[ticket #349](https://github.com/whaleyjoshua2/Dying-Earth/issues/349#issuecomment-5839581411),
decided by the designer on 2026-09-25. Where this file and that comment disagree, the comment wins
and this file is wrong.

## Prior art, searched before writing

Read out of the tree on 2026-09-25.

- **The challenger line** (`src/ui.rs:5562-5600`, ticket #262): on a held place's card,
  `game.nearest_challenger(target)` (`engine/src/state.rs:3088`) names the rival nearest its OWN
  price, and the line turns amber with *"Spend here to stay ahead."* when
  `theirs + 2 * step >= standing`, with `step = game.tables.ai.thresholds.influence_step` (5,
  `assets/data/ai.toml:362`). **Only the nearest-to-price rival is tested**, so a second rival within
  10 of the holder's Standing but further from its own price does not turn the line amber.
- **The interface reads an AI tuning figure** to decide what it tells the player. `influence_step`
  is also read by `ai.rs:1470`, `:1748`, `:1809`; those stay on it.
- **Both read the board as last resolved**: `standing` is `seat.influence[place]`, not counting
  Influence ordered this turn. Kept.
- **The Command Cluster** (`src/ui.rs:3671`): left column is the Allotment row, the rail
  (`influence_rail`, `:3736`), the Spend row, then Max and the every-turn tick. The strip does not
  scroll, and the column wraps text (`:3713`). `view.selection` is `Selection::State` or
  `Selection::Colony`.
- `Place` is `State(StateId) | Colony(ColonyId)`; a station is a Colony.
- Nothing found that lists every Pressed place; the finding is a negative.

## The rules

### R1. Pressed, in the engine

- New figure `pressed_band = 10` in `assets/data/influence.toml`, loaded into the influence table,
  with a comment naming this ticket and saying it is an interface figure that moves no rule.
- New query on `Game`, beside `nearest_challenger`: whether `place` is Pressed for its holder. True
  when the place has a controller and **any** other seat has a Standing there `> 0` and
  `>= holder_standing - pressed_band`. False on a place nobody holds.
- New query: the list of places `seat` holds that are Pressed, in a fixed order (Regions by
  `StateId`, then Colonies by `ColonyId`), so the list does not reshuffle between frames.
- No rule reads either query. Nothing is added to the save.

### R2. The Command Cluster lists them

- Directly **under the rail** and above the Spend row, one line per Pressed place the player
  (`Seat(0)`) holds: **`{place name} is Pressed by a rival.`**, in the amber the challenger line
  uses (`Color32::from_rgb(255, 160, 60)`).
- **At most three lines.** A fourth or more becomes one more line: **`and {n} more`**, plain.
- Each place line is clickable and sets `view.selection` to that place, as a click on the map would.
  No hover text on any line: **the rival is never named and no figure is given**.
- No Pressed places: nothing drawn, no placeholder line.
- Under the Spectator, draw nothing (there is no player seat to hold anything).

### R3. The card's challenger line matches

- `pressing` in `threshold_breakdown` becomes the R1 query for `target`, so the amber and
  *"Spend here to stay ahead."* fire exactly when the place is Pressed. The rest of the line (the
  nearest-to-price rival, its Standing and price, the hover) is unchanged.
- `src/ui.rs` stops reading `ai.thresholds.influence_step`.

### R4. Not built

- No Report line, no Moment kind, no map mark, no top-bar line. No mirror on an unpressed place.
- The computer seats are untouched.

## Tests

Each witnessed red before it is believed green.

- **Engine, `pressed`**: holder at 50, rival at 40 is Pressed; rival at 39 is not; two rivals, the
  nearer-to-price one at 30 and the other at 45, is Pressed (the case the old test missed); a place
  nobody holds is never Pressed; a rival at 0 Standing never presses.
- **Engine, the list**: holds three places, two Pressed, returns exactly those two in order.
- **Data**: `pressed_band` loads as 10; changing it to 12 in a test table moves the 39 case to
  Pressed.
- **The sweep does not move**: 20 seeds x four seatings, byte-identical per-Faction totals to the
  build before, since no rule reads the query.

## Pictures

Headless, `shot:` mode only, filed beside this spec. Never run the game executable bare.

1. A turn with one Pressed place: the line under the rail, the Spend row below it, End Turn not
   displaced.
2. A turn with four or more Pressed places: three lines and *"and N more"*, End Turn still level
   with the Max row.
3. The card of a Pressed place, amber, and the card of an unpressed held place, not amber.

Look at each before calling the build done.

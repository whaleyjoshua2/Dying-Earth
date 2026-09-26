# Ticket #349: a place you hold is Pressed

The designer's line: *"Warning when opposing faction is within 10 your influence on territories you
own."* The warning already existed, but only inside a place's card, so a player saw it only if they
happened to open that card. All four playtesters lost a home Region with no warning.

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/349#issuecomment-5839581411)
is the authority; `SPEC.md` is the build specification written from it.

## What was built

- **Pressed** (`Game::pressed`, `Game::pressed_places`): a place you hold where **any** rival's
  Standing is within 10 of yours. The 10 is `pressed_band` in `influence.toml`, not the computer's
  `influence_step`, which the card used to borrow.
- **The Command Cluster lists them** under the Influence slider, whatever is selected: *"Egypt is
  Pressed by a rival."* The rival is never named and no figure is given. At most three lines, then
  *"and N more"*. A click takes you to the place and selects it.
- **The card agrees**: its challenger line turns amber with *"Spend here to stay ahead."* on exactly
  the same test. Until now the card tested only the rival nearest its own price.

## The pictures

- `pressed-one-place-card-amber.png`: Egypt held at 50, the Prospectors at 50. The line stands under
  the slider and the card's challenger line is amber.
- `pressed-six-places-three-shown.png`: six held places Pressed. Three lines, *"and 3 more"*, and
  End Turn still level with Max.
- `not-pressed-rival-at-30.png`: the same card with the Prospectors at 30. No line in the Command
  Cluster, and the card's challenger line is not amber.

## The sweep did not move

`sweep-before.txt` and `sweep-after.txt` are 20 seeds x four seatings on the live climate figures,
before and after both this ticket and #350, and they are **byte-identical**: Custodians 4, Prospectors
37, Arkwrights 2, Archivists 1 of 80, collapses 36 of 80. Nothing in the engine reads Pressed.

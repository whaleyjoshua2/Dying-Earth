# Ticket #351: an alarm before the lights go out

The designer's line: *"alarm when there is a projected energy deficit and buildings will go offline
next turn."* Every one of the four playtesters hit the Shortfall with no warning; a Custodian lost
three Scrubbers to it and the Natural Sink with them.

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/351#issuecomment-5840665581)
is the authority, and served as the spec.

## What was built

- **`Game::shortfall_forecast`**: the Income rule itself, run on the Energy left after this turn's
  orders. It returns how far short the seat is and what goes dark, in order, with where each
  building stands. The rule was restructured so the
  forecast and Income share it (`energy_balance`, and `apply_shortfall` taking its starting Energy);
  Income's behaviour is unchanged, which the sweep confirms.
- **The top bar**: the Energy figure turns red while a Shortfall is forecast, and its hover gives the
  approved words in place of the Last Income breakdown.
- **The Report line after the fact** names the Sink's loss when Scrubbers were shut, counting only a
  Scrubber the Sink actually counted (review found one in a Region occupied from neutral claimed a
  loss it never caused).

## Two changes after review, at the designer's word

- **The Natural Sink clause came off the alarm's hover** (and the driver's copy of it): the alarm
  says what goes dark. The Report line after the fact keeps it.
- **A Mothball, Restart or Decommission ordered this turn does not clear the alarm.** All three land
  at this turn's Resolution, before Income, so the forecast could have counted them; the designer
  chose to accept that the alarm clears the turn after instead. A live production and consumption
  figure on the bar was considered and dropped.
- **The headless driver** prints the alarm on its board, with the full list.

No order spends Energy and Energy cannot be sold, so "counting this turn's orders" means a purchase
of Energy clears or shrinks the alarm on the turn it is ordered.

## The pictures

- `alarm-red-with-hover.png` and `alarm-top-bar-close-up.png`: an empty store and two Scrubbers and a
  Research Lab at home. The figure *0 (-4)* is red, and the hover reads *"Next Income is 15 ⚡ short.
  These go dark, in this order: the Refinery in China / the Research Lab in China / the Scrubber in
  China / and 3 more"*. The game's hover style draws the word Energy as
  its icon, as it does everywhere.
- `no-alarm-energy-covered.png`: the normal start, 16 Energy stored, the figure in its usual colour.

**The first picture was wrong.** It used `TURN_RED`, a button's fill colour, and the figure was
nearly unreadable on the dark bar. The build now uses the bright red the Break line uses.

## The forecast came true

Played through the headless driver from a Custodian start in Europe, building two Research Labs a
turn. After turn 2 the driver printed:

    ENERGY ALARM: next Income is 8 Energy short. These go dark, in this order:
      the Refinery in The European Union
      the Research Lab in The European Union
      the Research Lab in The European Union

and the next turn's Report said *"Custodians: Energy ran short; shut down Refinery, Research Lab,
Research Lab."*: exactly the buildings forecast, in that order.

## The sweep did not move

20 seeds x four seatings on the live climate figures, byte-identical to the baseline committed with
ticket #349 (`../ticket-349-pressed/sweep-after.txt`): Custodians 4, Prospectors 37, Arkwrights 2,
Archivists 1 of 80, collapses 36 of 80.

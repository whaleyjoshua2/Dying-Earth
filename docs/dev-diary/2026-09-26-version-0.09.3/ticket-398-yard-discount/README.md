# Ticket #398: Build Where You Dig reaches Ships built on the Moon, Phobos or Deimos

The designer's item, suggestion S17 of the space round: *"Build Where You Dig, which excludes
Ships, reaches Ships built at a Shipyard on the Moon, Phobos or Deimos when that Colony has a
working Mine."* Decided in one round of four (*"q1 a q2 a q3 a q4 a"*): every kind of Ship, the
three low-gravity Bodies only, the Module's own steps, said on the buttons and the card's note.
[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/398) is the authority,
[§11 of the spec](../../../spec/version-0.09.3.md#11-build-where-you-dig-reaches-ships-built-on-the-moon-phobos-or-deimos)
records it.

## What was built

`Game::ship_materials_at(seat, site, kind)` beside `ship_materials`: at a Shipyard on a ground
Colony on a low-gravity Body with a working Mine, the seat's price times the `[in_situ]` step
(0.75 with one Mine, 0.6 with two or more), never under half the row; the Build Ship order and the
queue's cost read it; the Faction window keeps the seat's price. The Colony card's note reads
*Modules and Ships cost x0.75* on the three Bodies. The computer seats price a Ship through the
same order cost. A building aid, `yard:1`, gives seat 0's first ground Colony a Mine and a Shipyard.

## The pictures

**A Moon yard with one working Mine**, `seed:7 first:1 hab:ground yard:1 panel:0 window:1400x1700`:
the note *One working Mine here: Modules and Ships cost x0.75*, and the Ship buttons at three
quarters: *Frigate 18.8* against the row's 25, *Battleship 37.5* against 50, *Missile Carrier 45*
against 60. The buttons are greyed on this fresh Colony because a build fills the tank and the
seat holds 26 Fuel against a tank of 30 (the review read the refusal); the price is what the
picture is for.

![The Moon yard's card with the note and the discounted Ship buttons](moon-yard.png)

## The review

The standards review found the yard's tie order reversed (the first cut kept the LAST yard on the
list among equals, where the queue ticket had the first): fixed, with a tie case added to the
witness, red at *the first on the list among equals* and green. It also folded the Ship's row
choice into one place and named the review's lesser points (a data comment still saying *rounded
down*, the picture aid's disregard of a Colony's room). The spec review read the greyed buttons'
refusal (Fuel, not Energy) and found the second picture was the Faction choice screen, which
carries no Ship price: it is dropped, the seat's price being the witness's. It also named two
things decided without the designer's word, said in the closing report: the Carrier still built at
the Earth yard with the most Widgets, where Armies board, and the figure 1.5.

## The red witnesses

`a_ship_built_at_a_low_gravity_yard_with_a_working_mine_takes_build_where_you_dig`: a Frigate at a
Moon Colony with one working Mine at three quarters (red at the full price before the build), every
kind of Ship cheaper there, two Mines at three fifths above the floor, the Mine mothballed back to
the full price, a Mars yard, a station and Earth at the full price, and the tank's Fuel untouched.

## The first sweep, and what it found

[`sweeps/after-398-rule-only.txt`](../sweeps/after-398-rule-only.txt), the rule alone, against the
Stadium's [`sweeps/after-389.txt`](../sweeps/after-389.txt), the last sweep that moved a rule:
**identical, line for line** (`diff` empty). Wins 6 / 4 / 1 / 9, collapses 60, Missile Carriers
built 40, games with an orbital Battle 27 of 80, all unchanged. A rule that changes a price changes
every game it fires in, so the computer seats never once built a Ship at a low-gravity ground yard
with a Mine. Measured directly over 24 computer games of 36 turns: working ground Shipyards stood
in 34 Colony-turns, **none on the Moon, Phobos or Deimos**. The computer builds its Shipyards on
stations and on Earth, and offers each Ship at the yard with the most Widgets (ticket #332, the
designer's rule), so the new price never reached it. Put to the designer as Q5; their answer, A:
the computer seeks the yard.

## The computer seeks the yard

`ai_ship_yard`: a Ship is offered at the yard where it costs least, the most Widgets among equals
(a Carrier still at the Earth yard with the most Widgets, where Armies board); and a Shipyard's
weight on a ground Colony on a low-gravity Body with a working Mine is multiplied by
`low_gravity_yard`, 1.5 in `ai.toml`, new. The witness
`a_computer_seat_builds_its_ships_at_the_cheapest_yard_and_seeks_a_yard_on_the_moon`: a Moon yard
with a Mine chosen over an Earth station's yard with more Widgets, the station chosen again once
the Mine is mothballed, the bonus on the Moon and not on a station or Mars; red with the figure at
1 (*and the figure is a lift*), green at 1.5.

## The sweep after

[`sweeps/after-398.txt`](../sweeps/after-398.txt), the rule with the computer seeking the yard:
**6 / 4 / 1 / 9, collapses 60**, the Stadium's figures to the win; Missile Carriers built 38 (40
before), games with an orbital Battle 27 of 80 (27), a hundred lines of the file moved by a Module
or a Pioneer here and there. **The computer still all but never uses the yard**, and this is the
miss to report. Measured over the same 24 computer games of 36 turns: working station yards stood
in 1,019 Colony-turns, ground yards anywhere in 44, and a low-gravity ground yard with a working
Mine in 10 (none before), with a Ship in its queue in 0 of those turns. With `low_gravity_yard` at
3 instead of 1.5 the yard stood in 15 Colony-turns, the Ship in its queue still in 0; the figure
stays at 1.5. The reason is upstream of the multiplier: a computer seat builds its first Shipyard
on its Earth station in the opening turns (the *Launch Site or Shipyard* weight is for the first,
and a second yard is weighed low), and a Moon Colony's few slots go to Mines and Habitats first, so
a second yard on the Moon is seldom stood and, when it is, the seat's Ships are wanted before it
works. What to do about that, if anything, is the designer's call, said in the closing report and
not ticketed.

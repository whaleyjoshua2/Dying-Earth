# An odds hover that names the defender

Ticket [#309](https://github.com/whaleyjoshua2/Dying-Earth/issues/309) on
[map #304](https://github.com/whaleyjoshua2/Dying-Earth/issues/304).

| picture | what it shows |
|---|---|
| [`china-march-buttons.png`](china-march-buttons.png) | `shot: select:eastasia army:1 panel:0 window:1280x1300`. China's card with a raised Army: the march buttons read **attack Russia**, **attack India**, **attack Indonesia**, **attack Japan**, with no percentage on any face. Before this ticket each read *attack Russia (61%)* and had no hover. |
| [`china-attack-hover.png`](china-attack-hover.png) | `shot: select:eastasia army:1 panel:0 window:1280x1300 tip:chance to win the first exchange`. The hover on *attack Russia*, four lines: *Russia: neutral.* / *defended by the 1st Russian Army: strength 3, defends at 6 (dug in), damage 0/3* / *35% is the chance to win the first exchange: your 4 against their 6.* / *Moving costs nothing; the Army arrives on Attack.* |

No batch was run: an interface change; the engine is untouched, and the odds figure is the one the
computer plays to.

## What was decided, in the designer's words

*"q1 go with that q2 both q3 move the propability to the hover q4 yes q5 yes q6 yes"*: the hover as
recommended; both strength and defends-at on a defender's line; the percentage off the button's
face and on the hover; the same hover on the landing button; the first exchange, labelled so; a
two-line hover on a move into a held Region.

## Settled by the builder, to be corrected if wrong

- **A march into your own Region keeps the Army's stance** (the Resolution sets Attack only when
  the Region is not yours), so the move-to hover says *the Army keeps its stance* where the
  recommendation had said *arrives on Hold*.
- **Past three defenders, two are named** and the rest counted, so the hover stays inside six lines.
- **The landing hovers**: at a rival's Colony the same composer, ending *Landing costs nothing; the
  Army attacks as it lands*; at your own, *held by you. Landing costs nothing; the Army lands on
  Hold*. Not photographed: no shot aid puts a Carrier with an Army over a rival's Colony.
- **An `army:1` shot aid** raises an Army of seat 0's in its start state, since a Region's own Army
  has had no march buttons since ticket #302.
- The **Battle Report** glossary entry says the odds are the hover's now.

## Looked at, not tested

The two pictures above, each opened and read before this was committed.

# A threat line beside the challenger line on a held Region

Ticket [#310](https://github.com/whaleyjoshua2/Dying-Earth/issues/310) on
[map #304](https://github.com/whaleyjoshua2/Dying-Earth/issues/304).

| picture | what it shows |
|---|---|
| [`china-threat-line.png`](china-threat-line.png) | `shot: select:eastasia threat:1 panel:0`. China's card with the Prospectors holding Russia and a raised Army there: under the challenger line, in the Prospectors' orange, **The Prospectors' 2nd Russian Army, strength 3, stands next door in Russia.** No stance. (The aid that hands Russia to the Prospectors also gives them a Standing on China, so the challenger line above it is amber and pressing; the two lines are the two threats a held Region faces.) |
| [`china-threat-hover.png`](china-threat-hover.png) | `shot: select:eastasia threat:1 panel:0 tip:their chance to win the first exchange`. The line's hover, three lines: *Their 3 against the 5 that defends here: 32% is their chance to win the first exchange.* / *The computer seats attack at 60% or better, so the line turns amber there.* / *Raised Armies next door only, whatever their stance; a Region's own Army never marches.* |

No batch was run: an interface change with one engine helper behind it, which changes no rule.
`the_nearest_army_threat_is_the_rival_raised_army_next_door_with_the_best_odds` in
`engine/tests/formulas.rs` was watched to fail against a helper made to return nothing, then to
pass restored.

## What was decided, in the designer's words

*"q1 no q2 that q3 yes q4 nothing q5 regions only this version q6 full name"*: no stance shown; the
rival raised Army next door with the best first-exchange odds against this Region's defenders; under
the challenger line, in the rival's colour, amber at the computer's attack bar; no line when nobody
stands next door; Regions only; the Region's full name.

## Settled by the builder, to be corrected if wrong

- **The helper is `Game::nearest_army_threat`**, beside `nearest_challenger`: a rule in the engine,
  not in one caller, with the ties-to-the-strongest rule inside it.
- **The amber bar is read from `ai.toml`** (`attack_odds`, 0.6), and the hover says it is the
  computer's habit, not a rule; when amber the line adds *Dig In here to hold it.*, the counterpart
  of the challenger line's *Spend here to stay ahead.*
- **A `threat:1` shot aid** hands the first neighbour of seat 0's start state to seat 1 and raises
  an Army there.
- The glossary gains a **Threat line** entry beside the challenger line's.

## Looked at, not tested

The two pictures above, each opened and read before this was committed. The engine helper is
tested; the line is looked at.

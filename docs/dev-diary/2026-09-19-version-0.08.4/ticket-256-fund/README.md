# The Prospectors' Fund: a slider, a withdrawal, and a bar of 2500

Ticket [#256](https://github.com/whaleyjoshua2/Dying-Earth/issues/256) on
[map #254](https://github.com/whaleyjoshua2/Dying-Earth/issues/254).

| picture | what it shows |
|---|---|
| [`victory-window-prospectors.png`](victory-window-prospectors.png) | `shot: player:prospectors victory:1 venture:900 panel:0`, 1280x800. The Victory window as a Prospector: the card text reads **2500 Ducats**, the bar **900 of 2500**, and under it the new control -- the share as a **full-width slider at 50%** with the top fifth painted over in the Directive's grey and a tick at 80, the Fund's balance and last turn's banking beneath, then a **field and a Withdraw button** where "Draw 10 from the Fund" stood. |

## What was decided, in the designer's words

- *"mirror the research slider with just the top 20% capped. the entire point of the slider is to
  allow finer control"* -- so `share_step` went from a tenth to a hundredth, the order takes whole
  percents, and the slider is `research_directive_control`'s rail with the cap at the other end.
- *"Withdraw with a window a number can be entered like the one for influence"* -- a `DragValue`
  from 1 to the Fund's balance, and the button says Withdraw.
- *"2500 as requested other things will bolster their output"* -- a deliberate stretch, recorded as
  one in `factions.toml` and the glossary.
- *"adjust the AI as well the whole victory condition should be 2500 not just a graphical change to
  the bar"* -- the AI already reads `victory_first.bar` from the card rather than a copy
  (`engine/src/ai.rs:1958`), and its share loop walks `share_step`, so it moved with the data. **A
  test proved it**: with the bar at 2500 and the old test income of 120 a turn, the AI maxed its
  share where it used to pick a modest one, because 120 x 24 turns no longer reaches the bar.

## What the suite caught unasked

Moving one figure in `factions.toml` turned **seven tests red** before a line of code changed: the
bar pin, the "not a step of 10" check, the AI share test, and four Victory-margin tests that set the
Fund at 2400 for a margin of 1.2 or 1500 for a score of 0.75. All seven were rewritten against 2500
and watched go green. That is the same catch ticket #240 recorded when the Fund changed currency.

## A stale line found on the way

`report.toml`'s `draw_venture` card still said *"drew {n} **Materials** from the Venture Capital
Fund"* -- a version after ticket #240 moved the Fund to Ducats. Nobody saw it because no sweep reads
the Report. It says *withdrew {n} Ducats* now.

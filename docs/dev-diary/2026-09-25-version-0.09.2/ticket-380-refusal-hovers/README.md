# Ticket #380: every refusal explains why on mouseover

The designer: *"all refusals should explain why on mouseover."* Decided in one round of seven,
after an audit of every greyed, dropped and hidden door in `src/ui.rs`: the engine's own reason on
every greyed control, refusal first and description after within six lines; the sliders fixed and
held; hidden doors stay hidden; after-click refusals stay as they are; the Spend button stays bare;
the End Turn sun and Enter say why.

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/380) is the authority;
[§13 of the spec](../../../spec/version-0.09.2.md#13-every-refusal-explains-why-on-mouseover)
records it.

## What the audit found

The ticket expected a tail of five or six greyed buttons without a hover. The audit found the head
instead: **since version 0.08.3 (ticket #238) no greyed order button in the game has shown the
engine's refusal to a pointer.** `rule_tip`, which every refusal goes through so the headless
picture aid can photograph it, ended in egui's `on_hover_ui`, and egui opens that tooltip only on
an *enabled* widget; on a greyed one only `on_disabled_hover_ui` fires. The shared helpers
(`cost_button`, `cost_button_with_hover`, `orders_button`) attached the description to the greyed
state and the refusal to the live one, so a greyed build, move, load, repair, Leapfrog, Recruit,
Launch or Bombard hovered its description or nothing. Every refusal picture in the diaries since
0.08.3 looked right because the `tip:` aid forces a tooltip open whatever the widget's state.

Beside that: the two sliders (Research Directive, Venture share) checked a second move in one turn
against the order it was cancelling and dropped it as "already set this turn", snapping back; the
"All N that can" buttons took a pre-filtered list, so a dead one read *"All 0 that can"* with no
reason, and an Army stack with one Army that could not march was greyed whole; the End Turn sun is
drawn with `Sense::hover` in an enabled Ui, so its disabled hover never fired and a dead sun hovered
*"End the turn (Enter)"*; and Enter on a dead sun did nothing at all.

## What was built

- `rule_tip` attaches its text to both states of the widget. `refusal_hover` composes a greyed
  door's text: the refusal, then the description if the two fit six lines (estimated at
  fifty-five characters a line), the refusal alone if not. `cost_button_with_hover` and
  `orders_button` build one hover for the state they are in; `stance_row` drops its second copy.
- `orders_button` takes every candidate, places those the engine accepts, and hovers the first
  refusal when none can. The stack card's two "All that can" lines pass every hull.
- The sliders check against the orders that will stand once the cancelled one is gone; a refused
  setting holds the rail (the value is read afresh each frame) and the refusal is on the rail.
- The sun hovers one text chosen by its state: the engine's refusal when dead. Enter on a dead sun
  with no window open puts the End Turn through, so the engine's refusal raises #105's popup.
- The spectator's End Turn says *"Close the window first."*

No engine change, no save change, no sweep.

## The picture

Headless, 1400x1000, `panel:0`, from this folder. The `tip:` aid forces the tooltip open, which is
the one way to photograph a hover; it proves the composed text, and egui's `on_disabled_hover_ui`
is what carries it to a pointer.

| picture | aids | what it shows |
|---|---|---|
| [`dry-mars.png`](dry-mars.png) | `refuel:1 stack:1 ship:2 "tip:orbit change needs"` | **A dry Frigate's card** (TSV Vanguard, 0 of 30 Fuel): its three Move buttons greyed, and the hover reading the engine's refusal first, *"the tank holds 0 Fuel; an orbit change needs 1"*, then the door's description under it. |

## The gate

`cargo clippy --workspace --release --all-targets -- -D warnings` clean; engine 499 + 6, root 8.

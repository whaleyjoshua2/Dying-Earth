# Cards that ask a question: the modal, the greyed turn and the Report's answers

Ticket [#337](https://github.com/whaleyjoshua2/Dying-Earth/issues/337) on version 0.09.0. The engine
half -- the deck of forty distinct cards, the Question phase at the head of the turn, the effect
vocabulary, the computer's rule per card and the glossary -- is commit `97d83cb`; this is the
interface half, which took over the placeholder modal that commit left in `src/ui.rs`. `SPEC.md`
beside this file is the specification and the designer's resolution comment on the ticket is the
authority. The designer changed one rule while this was being built: **a seat that cannot afford a
card's offer is asked all the same**, with the take side shut to it, so a struggling Faction still
feels the card; the greyed take button below is that rule.

| picture | what it shows |
|---|---|
| [`card-modal-earth.png`](card-modal-earth.png) | `shot:card-modal card:the_hard_winter ducats:120 window:1400x900`. **The card's own modal.** The Hard Winter in the Report's headline colour, the line under it saying what a card that asks is (*every Faction at the table is asked it this turn, and the turn cannot end until you have answered*), the question, and the two sides side by side and alike: **Pay the relief / -40 Ducats** and **Go without / +2.0 Unrest in every Region you hold**. Both consequences are on the button faces, written from the card's own effects in `events.toml`, so neither side can be chosen unread. No Continue, no Close, no default. Behind it the Climate Panel's deck line, which now counts the questions left: *32 cards left in the deck, 6 of them Climate, 17 of them asking a question*. |
| [`card-greyed-earth.png`](card-greyed-earth.png) | `shot:card-greyed card:the_hard_winter tip:"cannot pay" window:1400x900`. The same card to a seat holding **10 Ducats**: the take side is greyed and its hover says why -- *You cannot pay what this side of the card asks. Refusing is your only move this turn.* Refusing stays live, and it is the only live move. `Game::may_take_card(Seat(0))` decides it. |
| [`endturn-earth.png`](endturn-earth.png) | `shot:endturn card:the_hard_winter cardshut:1 tip:"is asking the" window:1400x900`. **End Turn greyed with its reason.** The sun is down to embers and its hover is the engine's own refusal sentence: *The Hard Winter is asking the Custodians a question, and it has not been answered. Take the offer or refuse it; the turn cannot end until you do.* The same path the Tech pick's refusal takes, so both read alike. `cardshut:1` sets the card aside for the picture; in play nothing can, and the modal comes straight back whenever nothing else is up. |
| [`report-report.png`](report-report.png) | `shot:report menus:1 card:the_hard_winter ducats:120 cardanswer:take window:1400x1100`. **The Report's answers**, the four seats together under *The Hard Winter: what the table answered*, each in its Faction's colour -- *The Custodians took it. The Prospectors refused it. The Arkwrights refused it. The Archivists refused it.* -- immediately above *What the rival Factions did*, where a player is already looking for news of them. A seat neither side could reach reads *had nothing to decide* in the same block. |
| [`card-spectated-earth.png`](card-spectated-earth.png) | `shot:spectate-check spectate:1 card:the_hard_winter window:1400x900`. The check behind one of the fixes below: a **spectated** game with a card pending shows no modal and keeps End Turn (Enter) live, because a computer seat answers when its orders are computed and a modal raised in that gap would stop a game nobody is playing -- and Auto with it. |

## What was built

- **`Popup::Card`**, a popup of the card's own rather than the Event's. It is raised at the head of
  the turn where the Event would be (a turn that draws a question draws nothing else), and
  **nothing dismisses it but an answer**: `advance_popup` gained a `card` flag and hands on from it
  only once it has been answered, so Escape, a tutorial note's button and every other path leave it
  standing. Answering hands on to the turn's Moments and then the Report, exactly as the Event does.
- **The two sides on the buttons.** `card_effect_text` says what one `CardEffect` does in words,
  and `card_side_text` joins a side's effects. It is the only place in the interface that turns an
  effect into a sentence, and every figure in it is the effect's own out of `events.toml`: a card
  regiven its figures in the table says the new figures with nothing touched here.
- **The take side greyed** where `Game::may_take_card(Seat(0))` is false, with the reason on its
  hover; refusing stays live. The designer's new rule, wired the moment the engine lane landed it.
- **End Turn greyed, with the reason.** `can_end_turn` now asks `Game::end_turn_refusal` instead of
  testing the Tech pick for itself, and the button's disabled hover is that same sentence. Both
  refusals therefore behave and read alike, and the button cannot drift from the rule -- which is
  the point ticket #105 made when it moved the rule into the engine. It also fixed a smaller thing:
  a spectated game used to grey End Turn whenever seat 0 held the Research Lead, though nobody was
  sitting at it.
- **The Report's answers**, lifted out of the four headings (the player's own was filed under Your
  works, a rival's under The climate) and drawn together in seat order under the card's name, each
  line in its Faction's colour. `card_report_line` finds them by the card name the engine's own
  `report.toml` sentences open with, and the card's name is taken off the front of each line
  because the block is drawn under it. `card_answered` gained a capital -- `The {faction}` -- so the
  lifted sentence opens properly; the whole line still stands unaltered in the log.
- **The deck on the table**: the Climate Panel's penalties line counts the questions left in the
  deck beside the Climate cards, from the deck itself. Four lines, as the ticket allowed.
- **Building aids**: `card:<event id>` makes that card the turn's question by stacking the deck and
  running the game's *own* Question phase until a roll brings it (the deck is put back as it would
  stand with that one card drawn, so the panel's count reads a real deck); `cardanswer:take` or
  `:refuse` answers for seat 0 and runs the turn, so the Report carries all four answers, the
  computer seats' among them; `cardshut:1` sets the modal aside for a picture of the board behind
  it; `ducats:<n>` puts seat 0 at n Ducats, since a fresh board is never poor enough to show a shut
  offer. The End Turn sun's hover is forced by the existing `tip:<word>` aid, through a small
  `forced_tip` for tooltips that are not `rule_tip`s.

## Looked at

The five pictures above, opened and read before this was written. Three things were caught by
looking, and all three are fixed in what is above:

- **A rival's answer headlined the dispatch.** The first Report picture opened with *The Hard
  Winter: The Prospectors refused it.* over the whole turn, and the same sentence appeared again in
  the answers block below: a rival's answer is filed as an Event line and an Event line headlines a
  quiet turn. The headline now passes over an answer. One of four answers is not the news of a turn,
  and the engine could settle it more neatly by filing answers under a kind that cannot headline.
- **A spectated game stopped dead.** A computer seat 0 owes the question between the Question phase
  and its own orders, so the modal came up over a game nobody was playing and, since Auto's clock
  stops while a popup is open, Auto never ran another turn. `card_owed` now says no for a computer
  seat 0; `card-spectated-earth.png` is the check.
- **The first modal picture was taken at ten Ducats**, so both its buttons could not be shown live
  at once. It became two pictures -- the live pair and the shut offer -- which the designer's new
  rule wanted photographed anyway.

Not photographed: the click that answers (headless, no pointer), and a seat reading *had nothing to
decide* in the block, which is the same code path as the other three lines and is pinned by the
engine's own R5 test.

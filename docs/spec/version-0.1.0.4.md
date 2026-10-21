# Dying Earth — version 0.1.0.4, the storyteller version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.1.0.4](https://github.com/whaleyjoshua2/Dying-Earth/issues/583), and the
pictures and sweeps that decided it are in
[`docs/dev-diary/2026-10-21-version-0.1.0.4/`](../dev-diary/2026-10-21-version-0.1.0.4/).

**The baseline** is 0.1.0.3's closing board,
[`ticket-582.txt`](../dev-diary/2026-10-19-version-0.1.0.3/sweeps/ticket-582.txt):
**26 / 41 / 5 / 8 and 0 collapses**, against the designer's 15 each and 20.

**What the version is.** Version 0.1.0.3 with the designer's list of 2026-10-21, most of it making the
game tell its own story in plain words, beside the figures that move. The summary and the win column
are written at the close-out.

## 1. The Prospectors' Venture Fund at 5,000

Ticket: [The Prospectors' Venture Fund at 5,000](https://github.com/whaleyjoshua2/Dying-Earth/issues/584).

The designer: *"Prospectors to fund to 5k"*. The Prospectors stood at 41 wins of 80 on the closing
board, the farthest seat from the ideal, and 0.1.0.3's move to 4,250 (§11 there) had not bitten:
the computer Prospectors bank to their pace, and the Fund at the end rose by about the bar's fifth.

- **The bar is 5,000** (`victory_first.bar`, 4,250 x 20/17). The card's long and short texts read
  the bar, so they say 5,000; the glossary and the playtest note say it.
- **The computer Prospectors' pace** scales with it on the same turns, at the designer's word:
  667 / 1,667 / 3,000 / 5,000 at turns 12 / 25 / 37 / 47 (`[pace.prospectors] first`); the test
  that holds every pace schedule's last figure to its bar passes. The banking share, read from the
  bar each turn, asks a sixth more Fund at every turn.
- **Pins that moved with the bar**: the Victory tests' margins (6,000 for a 1.2 margin, 2,500 for
  half, 850 for 17%), the rival-Moment figure (3,800 of 5,000, from 3,230 of 4,250) in the tests and
  in the headless shot's `rival:1` aid, and the turn-one line.
- **Swept** (`sweeps/ticket-584.txt`, 20 seeds x four seatings): **30 / 36 / 3 / 9, 1 collapse and
  1 draw**, from 26 / 41 / 5 / 8 and 0. The Prospectors' five off and the Custodians' four on sit at
  the edge of noise (a move of about 3 is noise); the Arkwrights at 3 from 5 and the Archivists at 9
  from 8 are noise. Their gate still completes in 79 of 80 games at a median turn 27 (80 at 27
  before). **The bar did not bite again**: their Fund at the end by seating is median 5,200 / 5,064
  / 4,112 / 4,808 of 5,000, from 4,425 / 4,298 / 4,075 / 4,308 of 4,250, up by 500 to 775 in three
  seatings and unmoved in the third, since the pace table rose with it and the computer banks
  harder to meet it. Reported and left; the Fund is a pace the computer sets to the bar, so a bar
  alone does not slow it.

## 2. The Arkwrights and the Archivists open with 10 Ducats

Ticket: [The Arkwrights and Archivists open with 10 Ducats](https://github.com/whaleyjoshua2/Dying-Earth/issues/585).

The designer: *"Arkwrights and Archivists start with 10 duckets"*. They stood at 3 and 9 wins of 80
on §1's board, the Arkwrights the farthest seat below the ideal.

- **A `start_ducats` on a Faction's row**, read by `Tables::start_stockpile` in place of the `[start]`
  table's 0 as `start_materials` is (0.1.0.3 §22), and refused when negative. The Arkwrights' and
  Archivists' rows say 10; the Custodians and the Prospectors open with the table's 0.
- **The card** says it on its opening glyph line alone, *"Opens with 80 · 20 · 20 · 10"* with the
  Ducat glyph, which already showed the Ducats wherever there were any; the signature texts do not
  repeat it, at the designer's word.
- **The computer** needs nothing: the stockpile is the seat's. In a logged game (seed 7) neither seat
  spends the Ducats on turn one; both sell Fuel and keep them.
- **Swept** (`sweeps/ticket-585.txt`): **28 / 37 / 5 / 10 and no collapse**, from §1's 30 / 36 / 3 /
  9, 1 collapse and 1 draw. The Arkwrights +2 and the Archivists +1 are noise; so is the rest. The
  gates: the Arkwrights' in 65 of 80 at a median turn 30 (70 at 31), the Archivists' 68 at 30 (70 at
  29). Ten Ducats on turn one do not move the board. Reported and left.

## 3. The Arkwrights draw +1 Research for every Body settled

Ticket: [The Arkwrights draw +1 Research for every Body settled](https://github.com/whaleyjoshua2/Dying-Earth/issues/589).

The designer: *"Arkwright's get +1 research for every body they've settled"*. Their Research was the
lowest of the four (285 a game on §2's board, against the Custodians' 1,372).

- **A Body settled** is one off Earth where the seat controls at least one Colonist,
  `bodies_settled(seat, 1)`: a station over a Body counts as there, Earth and its orbit never. At the
  designer's word, over the Diaspora's four a Body and over ground Colonies only.
- **The figure** is `research_per_body_settled` on the Faction's row, 1 on the Arkwrights', nothing
  elsewhere. It is flat, added to the turn's Research after every multiplier and before a Research
  agreement's lift, and the income's sources name it *"Bodies settled"*.
- **The card** says it in the signature, at the designer's word: *"They draw +1 Research a turn for
  each Body they have settled."*, the word drawn as its glyph.
- **The computer** needs nothing: the Research lands where it is read.
- **Swept** (`sweeps/ticket-589.txt`): **28 / 38 / 4 / 9 and 1 collapse**, from §2's 28 / 37 / 5 /
  10 and none: noise. The Arkwrights' Research is 325 a game from 285 and the Techs they led 193
  from 168; their gate completes in 66 of 80 at a median turn 30, from 65 at 30. Reported and left.

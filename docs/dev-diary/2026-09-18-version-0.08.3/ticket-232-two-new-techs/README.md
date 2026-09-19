# Ticket #232: two new Techs, and the AI appetite behind them

## What was measured, and why it changed the ticket

The ticket was charted as "add two Techs". The measurements turned it into something else.

**First: both Techs, as asked for, land on things that barely exist.** Counted with throwaway
counters added to `sim.rs` and `sweep.rs`, run, and reverted — 40 games over two seatings, modules
standing at the end:

| module | over 40 games | per game, all four seats |
|---|---|---|
| Habitats | 132 | 3.3 |
| Mines | 26 | 0.65 |
| **Relays** | **1** | 0.03 |
| Solar Arrays | 85 | 2.1 |

That killed the risk the ticket was charted around — a Habitat paying Influence could not "make the
Relay redundant", because **one Relay was built in forty games**. The designer moved the +1
Influence off the Habitat and onto the Relay instead: *"I forgot about the relay — let's give the
+1 influence to the relay instead."*

**Second: of the Mines that do exist, almost all are Antarctic.** In one seating, of 16 Mines
standing at the end, **13 were in Antarctica and 3 were off Earth**. So the designer's words — "off
world mines" — taken literally would have reached 0.15 Mines a game. Told that: *"count Antarctica
and change the tech's name"*, hence **Beneficiation**, a name with no space word in it.

**Third: no Tech in the game had ever changed what the computer wants to build.** A Mine's AI weight
was a flat `base_weight(Producer)`; a Relay's a flat `base_weight(BuildInfluence)`. Neither read what
the building would actually make, so Deep Mining's ×1.5 and the Extraction Charter's ×1.25 had never
once moved a seat's appetite. That is the likeliest reason for the 26-Mines-in-40-games figure above.
The designer, asked whether to fix this Tech alone or the gap: *"let's fix that one outright."*

## What the fix does, measured three ways

Same seating, 20 seeds, modules standing at the end:

| | Mines | Relays | Habitats | wins by seat |
|---|---|---|---|---|
| Baseline, before this ticket | 16 | 0 | 68 | [14, 2, 0, 0] |
| The two new Techs alone | 12 | 5 | 70 | [14, 1, 0, 2] |
| **Techs + the AI fix (shipped)** | **237** | **151** | 103 | **[0, 18, 0, 0]** |

**The Techs alone do almost nothing** — which is what was predicted when the ticket was charted.
**The AI fix is the whole effect**, and it is large: Mines ×15, Relays ×30, and in this seating the
Custodians (seat 0), the faction that wins 37 of 80 across the full sweep, drop from 14 wins to none
while seat 1 takes 18.

The designer was shown this table and the option to damp it, narrow it to Beneficiation alone, or
split it into its own ticket, and chose to **ship it as measured**. The closing ticket's sweep reads
the win column across all four seatings against the 0.08.2 baseline.

## A wrong turn, recorded because the shape of it recurs

The AI weight was first written as *actual yield ÷ card figure*. That silently included the **slot's
own yield**, which is at least 1 everywhere, so every Mine on the board was lifted with no Tech
researched at all: **329 Mines** over the same twenty games. The weight now reads
`tech_output_multiplier_module`, the tech-only factor — 1.0 until Deep Mining lands, 2.06 with all
three. `tech_output_multiplier_module` was made `pub(crate)` for it, with the reason in a comment
above it so the next reader does not reach for the finished yield again.

## A test that proved nothing, and how it was caught

The first draft of the Antarctic test asserted `after >= before`. Watched to fail with the
Beneficiation multiplier deleted, **it passed** — because a tenth of a small Mine floors away to
nothing. Both Beneficiation tests now pin exact figures, measured:

| Body | Deep Mining alone | + Beneficiation | + the Charter |
|---|---|---|---|
| Earth (Antarctica) | 10 | 11 | 14 |
| the Moon | 9 | 10 | 13 |
| Mars | 7 | 8 | 10 |
| Phobos | 10 | 11 | 14 |

A tenth **on its own** is invisible on Earth (7 → 7) and Mars (5 → 5). That case cannot arise in
play, because Beneficiation `needs` Deep Mining — the prerequisite is load-bearing, and a test pins
it. With both rules restored, each was deleted in turn and its own tests watched to fail.

## The picture

| picture | what it shows |
|---|---|
| [`t14-moon.png`](t14-moon.png) | Turn 15 with all twenty Techs in the tree: the board renders, the Colony Slot yields read, and the top bar shows `17 / 48`. |

**The Tech Tree window still cannot be photographed** — `shot:` captures the seven map views and the
five menus, and no flag opens a panel. So the two new boxes, and the two branch-rung cells that now
hold two boxes each, were verified by reading `box_of` in `src/ui.rs` (which already lays out
Industry rung 1 and Society rung 2 that way) rather than by looking. That gap is recorded on the map.

Taken with `target/release/dying-earth.exe shot:<prefix> turns:14`, off-screen, exit 0. Nothing was
opened on the designer's desktop.

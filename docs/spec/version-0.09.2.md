# Dying Earth — version 0.09.2, the tidy-up version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.09.2](https://github.com/whaleyjoshua2/Dying-Earth/issues/365), and the
pictures and batches that decided it are in
[`docs/dev-diary/2026-09-25-version-0.09.2/`](../dev-diary/2026-09-25-version-0.09.2/).

**What the version is.** Version 0.09.1 with the designer's list, which is mostly the game saying
what it does, where the player can see it. Sections are added here as each ticket closes; the
closing ticket writes the summary and the win column.

---

## 1. No card on the first turn

The authority is [ticket #367](https://github.com/whaleyjoshua2/Dying-Earth/issues/367).

**No card of either kind is drawn on the first turn.** The Draw Chance is not rolled before
`first_draw_turn` (2, in `events.toml`), and the Event Deck is **not touched** before it: nothing is
rolled and nothing is spent, so the card on top waits for turn 2's ordinary roll and no card is
lost. Both kinds are held off, the Choice Card that would ask before the first order and the
ordinary Event that would land in the first Resolution. The loader refuses a figure of nought. The
off-Earth join on turn 12 is unchanged.

**Why.** The first turn is where a player reads their Condition and gives their first orders; before
this a card came on turn 1 half the time, and a question before the first order was the interruption
the designer removed. Measured before the rule over sixty seeds, thirty drew on turn 1.

**Reaches the computer seats** by the same phase, so it is the same game for all four.

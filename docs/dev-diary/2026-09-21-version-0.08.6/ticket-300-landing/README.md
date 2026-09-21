# A landed Army may Attack on the turn it lands

Ticket [#300](https://github.com/whaleyjoshua2/Dying-Earth/issues/300) on
[map #289](https://github.com/whaleyjoshua2/Dying-Earth/issues/289).

No picture: the rule's one surface is the landing button, whose label gains the first-round odds
the march buttons already quote (*Land the Army to attack Ishtar (58%)*), and the screenshot
harness has no aid that puts a loaded Carrier over a rival's Colony. The measurements, twenty seeds
with the Custodians first: [`baseline-landings-before-the-rule.txt`](baseline-landings-before-the-rule.txt)
with the counter alone, and [`sweep-20-custodians-first.txt`](sweep-20-custodians-first.txt) with
the rule.

## What was decided, in the designer's words

- *"q1 second pass"* -- a **second ground pass after cargo lands**, at every Colony where an Army
  that came down this turn stands on Attack; the orbit is fought first.
- *"q2 inferred"* -- the stance is **inferred** from whose Colony it is: Attack at one the seat
  does not direct, Hold at its own. No new field.
- *"q3 same turn"* -- a landing that meets nobody **occupies the same turn**.
- *"q4 yes"* -- **odds on the button**.
- *"q5 yes"* -- the computer's Colony odds count the Army aboard; its appetite is left for the
  balance version.
- *"q6 yes"* -- **Armies landed counted**, the baseline printed first.

## Settled by the builder, to be corrected if wrong

- The ground melee loop became a function with an optional "only these Armies" filter, so the
  second pass fights only at Colonies where a landed Army stands on Attack and nothing fought at
  (b) is fought twice; the Occupation pass likewise runs over the Colonies once more.
- The Armies that landed are remembered on the turn's pending state, cleared with it.
- **The computer's odds already counted the Army aboard**: its landing candidate reads the aboard
  Army's strength against the Colony's defenders through the same odds formula. Nothing needed
  moving; the ticket's charting note was wrong on that point.
- The **invasion timing test** moved a turn earlier: a defended Colony changes hands in two to four
  turns where it took three to five, since the landing turn is now the first Battle turn, and
  nothing changes hands before the third turn's Resolution.
- The **Carrier** glossary entry says the rule.

## Measured

No Army landed at a Colony in twenty seeds, before or after the rule -- the reviews' finding that
Barracks and Carriers are dead rules on the computer's board holds -- and the rest of the block is
unchanged: 5 / 13 / 0 / 0, 22 Battles all in orbit. The rule is a human's for now; the closing
sweep reads all four seatings.

## Witnessed

`an_army_landed_at_a_rivals_colony_fights_or_occupies_the_turn_it_lands` was watched red with the
second pass disabled: *alone at the place, it occupies the same turn: Controlled(Seat(1))*.
`352 passed`, `6 passed`, clippy clean with `-D warnings`.

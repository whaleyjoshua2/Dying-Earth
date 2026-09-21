# A Region's defence is its people, and the disengage chance nudged down

Tickets [#296](https://github.com/whaleyjoshua2/Dying-Earth/issues/296) and
[#295](https://github.com/whaleyjoshua2/Dying-Earth/issues/295) on
[map #289](https://github.com/whaleyjoshua2/Dying-Earth/issues/289), built together at the
designer's word: *"yes go ahead and build both."*

| picture | what it shows |
|---|---|
| [`china-standing-army-five-of-five.png`](china-standing-army-five-of-five.png) | `shot: select:eastasia panel:0 window:1280x1100`. China on turn 1, Industry 3, calm, no Constabulary: **the 1st Chinese Army (Custodians, standing): strength 5, damage 0/5** -- Industry + 1 + the calm point, with hit points equal to it, where the row read *strength 4, damage 0/5* before. The map's shields read 5 and 3 where they read 4 and 2. |
| [`egypt-standing-army-three-of-three.png`](egypt-standing-army-three-of-three.png) | `shot: select:northafrica unrest:5 panel:0 window:1280x1100`. Egypt, Industry 1, at Unrest 2 (the aid's offset for it), still calm: **strength 3, damage 0/3**. The hit points follow the strength down as well as up. |

The measurements, twenty seeds with the Custodians first:

- [`baseline-0.08.5-rules-with-escapes.txt`](baseline-0.08.5-rules-with-escapes.txt): the counter alone, under the old figure and the old strength.
- [`sweep-20-custodians-first.txt`](sweep-20-custodians-first.txt): both rules in.
- [`control-people-points-off.txt`](control-people-points-off.txt): the disengage nudge and the live hit points in, the Constabulary and calm points at nought.
- [`control-divisor-two.txt`](control-divisor-two.txt): the people's points in, the disengage figure back at two.

## What was decided, in the designer's words

**#296, the defence.**

- *"lets exclude built armies for now"* -- after first answering *"apply it to built armies too
  but apply it where its built making it fixed"*; the Region-read built Army is in the map's fog.
- *"q2 on top"* -- the two points sit on top of the earned steps and on top of a Levy's + 2.
- *"q3 live"* -- hit points equal the live strength; at its strength in damage the Army is
  destroyed.
- *"q4 yes"* -- a working Constabulary is the Constabulary's own test.
- *"q5 yes"* -- the computer reads the live figure; its Evade trigger moved off the card's 5.

**#295, the disengage.**

- *"a1 a third"* -- damage over hit points over three, the figure in data.
- *"q2 leave it"* -- Evade stays a flat half. *"q3 both sides."*
- *"q4 yes"* -- the escapes counted, the baseline printed first. *"q5 yes"* -- built now.

## Settled by the builder, to be corrected if wrong

- Two tables in `units.toml`: `[disengage] divisor = 3.0` and `[standing_army] constabulary = 1,
  calm = 1`. The loader refuses a divisor at or under nought.
- **"Destroyed at its strength" is checked at Income** as well as in the Battle, since a
  Constabulary going offline or Unrest crossing the threshold can drop a wounded Army to nought
  between Battles; the Report line says *its strength spent*, and the two-Income return follows.
- The **earned-step ceiling** (Industry + 4) is on the earned steps; the people's points sit on top
  of it, so a neutral that has held three times, policed and calm, is Industry + 6.
- The respawn at strength 1 is damage one under the Army's own hit points now, not the card's.
- The Army row's hover names the four terms and says which are live.
- **Escapes are counted per unit by the seat it fought for**, a neutral's own separately, and a
  Battle is counted once if any unit escaped from it.

## Measured

| twenty seeds, Custodians first | baseline | both rules | people's points off | divisor two |
|---|---|---|---|---|
| wins C / P / Ark / Arc | 5 / 13 / 0 / 0 | 5 / 13 / 0 / 0 | 5 / 13 / 0 / 0 | 5 / 13 / 0 / 0 |
| Battles opened | 37 | 22 | 35 | 22 |
| marches on held Regions | 18 | **0** | 17 | **0** |
| Occupations begun / takes by force | 11 / 9 | **0 / 0** | 13 / 12 | **0 / 0** |
| Standing Armies lost | 7 | 0 | 12 | 0 |
| units escaped / Battles with an escape | 12 / 11 of 37 | 1 / 1 of 22 | 5 / 5 of 35 | 3 / 2 of 22 |

**The two people's points stop the computer's ground war on this seating.** Every held Region's
Standing Army is a 5 or a 6 with the hit points to match; a single built Army at 4 never clears the
computer's 60% first-round bar, so nobody marches, and the 22 Battles left are all in orbit. The
control with the points at nought brings the marches, the Occupations and the takes back. The
disengage nudge on its own roughly halves the escapes (11 of 37 Battles to 5 of 35). The win column
did not move in any of the four.

This is measured behaviour, not a rule: the computer marches one Army at a time and its bar is
60%. Two Armies at 8 against a 6 clear it. Whether the computer should mass before it marches is
the Army-appetite question the map's fog already names, and the closing sweep reads all four
seatings.

## Witnessed

`the_disengage_roll_is_a_third_of_the_damage_fraction_from_the_table` failed with *left: 2.0,
right: 3.0*, and `a_standing_army_reads_its_industry_its_constabulary_and_its_calm_and_dies_at_its_strength`
with *left: 4, right: 5* (no calm point), both with the tables loaded at the old figures and the
rules not yet written. `348 passed`, `6 passed`, clippy clean with `-D warnings`.

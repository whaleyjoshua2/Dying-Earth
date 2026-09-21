# One Army system: Levies, Standing Armies and raised Armies unified, with the Constabulary and calm as defence

Ticket [#302](https://github.com/whaleyjoshua2/Dying-Earth/issues/302) on
[map #289](https://github.com/whaleyjoshua2/Dying-Earth/issues/289), added by the designer after
the defence and Dig In tickets were built.

| picture | what it shows |
|---|---|
| [`china-strength-four-defends-at-five.png`](china-strength-four-defends-at-five.png) | `shot: select:eastasia panel:0 window:1280x1100`. China, Industry 3, calm: **the 1st Chinese Army (Custodians, standing): strength 4, damage 0/4, defends at 5**. The calm point is defence now, not body: the row read *strength 5, damage 0/5* under the defence ticket. |
| [`egypt-strength-two-defends-at-five.png`](egypt-strength-two-defends-at-five.png) | `shot: select:northafrica panel:0 window:1280x1100`. Egypt, Industry 1, neutral: **strength 2, damage 0/2, defends at 5, dug in** -- Industry + 1, then calm and Dig In's two while it defends. |
| [`map-shields-bare-strength.png`](map-shields-bare-strength.png) | `shot: panel:0`, 1280x800. The shields carry the bare strengths (China 4, Europe 4, Egypt 2, Iran 3) with trench lines under the neutrals; what they defend at is the card's. |

The measurement, twenty seeds with the Custodians first:
[`sweep-20-custodians-first.txt`](sweep-20-custodians-first.txt).

## What was decided, in the designer's words

- *"q1 yes"* -- the Constabulary and calm points **move to defence**.
- *"q2 a"* -- a raised Army is a unit of its home Region's stack that **marches**; the Standing
  Army **stays at home**.
- *"q3 yes to both"* -- the Levy becomes **two armed steps for good**; the holding step joins the
  same figure; **no ceiling**.
- *"q4 yes"* -- a Colony's Army at the **average Industry of its Faction's Regions + 1**, fixed;
  Dig In its only defence.
- *"q5 yup"* -- a raised Army reads its **home's Industry + 1**, fixed at the raise.
- *"q6 both this ticket"* -- the computer weighs the **defended** figure and raises where Industry
  is **highest**.
- *"q7 yes"* -- the row, the hover, the roster and the glossary say it as one system.

## Settled by the builder, to be corrected if wrong

- A raised Army carries its fixed figure (`raised_strength`); a save from before this version reads
  the card's 4 and 5 for one. A Region carries a `threatened` flag so a threat arms it once per
  episode; a threat that leaves and returns arms it again.
- `threat_steps = 2` and `held_step = 1` in `units.toml` beside the two defence points, which keep
  their names. The Levy's two Report lines are replaced by one, *Egypt arms while a foreign Army
  stands next door: its Standing Army will stand at 4 from now on*; the sweep's "Levies raised"
  became "threat episodes armed for".
- **The defence terms are a Standing Army's alone**: a raised Army standing in its home Region does
  not read the Region's police or calm. A neutral's Army still digs in by default (#297).
- Only a raised Army has march buttons on the Region card now; the stance row still covers the
  whole stack. The march and landing buttons quote what the defenders fight at.
- A Colony's Army rounds the average to the nearest whole; a Faction holding nothing on Earth
  raises a 1.

## Measured

| twenty seeds, Custodians first | defence ticket (#296) | this ticket |
|---|---|---|
| wins C / P / Ark / Arc | 5 / 13 / 0 / 0 | 5 / 13 / 0 / 0 |
| marches on held Regions | 0 | 3 |
| Occupations begun / takes by force | 0 / 0 | 2 / 2 |
| Standing Armies lost | 0 | 2 |
| Dig In orders | 19 | 10 |

With the two points as defence, hit points come back to Industry + 1 and a little of the ground
war returns. No neutral armed by threat: the Prospectors' Armies never stand next to a neutral
on this seating. The closing sweep reads all four seatings.

## Witnessed

Two rules watched red: with the defence terms removed, *defends one stronger for calm: left 4,
right 5*; with the threat steps removed, *two steps for good at the Income the threat appears*.
`352 passed`, `6 passed`, clippy clean with `-D warnings`.

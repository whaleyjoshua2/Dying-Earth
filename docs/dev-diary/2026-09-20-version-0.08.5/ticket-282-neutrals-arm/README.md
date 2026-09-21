# Neutral states arm when threatened

Ticket [#282](https://github.com/whaleyjoshua2/Dying-Earth/issues/282) on
[map #275](https://github.com/whaleyjoshua2/Dying-Earth/issues/275).

| picture | what it shows |
|---|---|
| [`india-card-levy.png`](india-card-levy.png) | `shot: levy:1 select:southasia panel:0 window:1280x1400`. India's card, neutral at Industry Level 2, the turn after a built Army of the Custodians appeared in China next door: under *Armies*, **"the 1st Indian Army (neutral, standing): strength 3, damage 0/5"** and **"the 2nd Indian Army (neutral, levy): strength 4, damage 0/5"**, the Levy at Industry + 2. `levy:1` is a new aid that raises the Army in China and runs a quiet turn; the taller window is the harness's existing `window:` aid, since the Armies block sits just under the fold at 800 rows. |

The measurement, twenty seeds with the Custodians first:
[`sim-20-custodians-first.txt`](sim-20-custodians-first.txt).

## What was decided, in the designer's words

- *"q1 a second army raised"* -- arming is **a second Army**, a Levy, not a cap the Standing Army
  heals toward. The recommendation was the cap; the designer chose the Army.
- *"q2 a"* -- threatened means **a built Army of any Faction in a neighbouring Region, or a
  neighbour under Occupation**, stance-blind, so no rival's orders are disclosed.
- *"q3 industry + 2"* -- the Levy stands at **Industry Level + 2**.
- *"q4 yes"* -- a neutral that is attacked and holds gains **+1 to its Standing Army for good**, to
  Industry + 4.
- *"q5 it lifts"* -- the Levy **stands down** when the threat passes; the earned step never does.
- *"q6 yes"* -- **neutral Regions only.**
- *"q7 a"* -- a Levy heals only while Unrest is under the Standing Army's threshold, and **Agitate
  stays illegal on neutrals**.
- *"q8 a"* -- a destroyed Standing Army returns **two Incomes later**, not the next; the code had
  carried "the next Income" as an assumption since ticket #50.
- *"q9 the hover and the shield"* -- the neutral card's Army rows carry a hover naming the rule,
  and the map shield, which draws the sum of the Armies' strength, rises with the Levy.

## Settled by the builder, to be corrected if wrong

- **A Levy is raised only while the Region's Unrest is under the threshold** at which a Standing
  Army stops replenishing: a restive state musters nothing, which is the calm-and-policed reading
  of the designer's answer to q7 carried to the raising as well as the healing.
- **One Levy per Region**, standing, never marching, named from its home like any Army (*the 2nd
  Indian Army*), and marked *levy* on the card. It fights for the Region as its Standing Army does.
- **It stands down at the Income after the threat has passed**, or the moment the Region becomes
  somebody's, since it was the neutral Region's.
- **"Held"** is a Battle at a neutral Region after which a defender of its own still stands,
  unescaped, with strength above nought: the attacker did not begin an Occupation.
- **The respawn wait applies to every Region's Standing Army**, held or neutral, since the
  assumption it replaced was general; a saved counter of Incomes to wait, with a default.
- **The Report** says *"India arms: the 2nd Indian Army is raised at strength 4 while a foreign
  Army stands next door."*, *"India stands down the 2nd Indian Army: the threat has passed."* and
  *"Egypt held against the attack, and arms: its Standing Army will stand at 3 from now on."*
- **The sweep and the sim** count Levies raised and neutral Regions that held, on the game state
  and through the save.
- A new glossary entry, **Levy**, and the **Standing Army** entry carries the earned step and the
  respawn.

## Witnessed red

With threat detection switched off in the engine the test failed at *"a foreign built Army next
door threatens it, whatever its stance"*, and passed on the restore. The test's own first failure
was an expectation of one Levy where three neutrals bordering the Army all armed, which is the rule
working. `342 passed`, `6 passed`, clippy clean with `-D warnings`.

## Measured, twenty seeds with the Custodians first

| figure | Battles-pollute batch | this ticket |
|---|---|---|
| wins, Custodians / Prospectors / Arkwrights / Archivists, of 20 | 0 / 16 / 0 / 2 | **0 / 16 / 0 / 2** |
| collapses | 2 | **2** |
| Levies raised over the batch | -- | **0** |
| neutral Regions that held against an attack | -- | **0** |

**Nothing arms in a computer-only game**, and that is a finding worth carrying: every Region is
taken by Influence in the first turns, 14 to 19 takes a game, and the computer's first Army appears
around turn 22, so by the time an Army stands next to anything there is no neutral Region left to
be threatened. The rule fires on a board with a neutral beside an Army, as the test and the picture
show, and it will matter to a human player who marches early, or on a board where the neutrals
last; on today's computer board it changes nothing. The closing sweep measures all four seatings.

# The computer seats may attack a Region they did not lose

Ticket [#284](https://github.com/whaleyjoshua2/Dying-Earth/issues/284) on
[map #275](https://github.com/whaleyjoshua2/Dying-Earth/issues/275).

No picture: this is a rule for the computer seats, and the artifact is the batch,
[`sim-20-custodians-first.txt`](sim-20-custodians-first.txt), twenty seeds with the Custodians
first.

## What was decided, in the designer's words

- *"q1 a"* -- **the Prospectors' rule for all four seats**: attack on neutral and held Regions alike
  when the first-round odds clear the bar.
- *"q2 a"* -- **a cause**: a seat marches on a Region a rival holds only if it is Cold or worse
  toward that rival, the computer's first reading of Relations for war; a neutral needs none, and
  a place lost to a running Occupation may be retaken.
- *"q3 a"* -- **the attack is the stance by condition**: when the rule allows it and the odds clear
  the bar, Attack is the candidate and Hold is not, on the ground and in orbit. Before, Hold and
  Attack were scored against each other, Hold won for three seats and tied for the Prospectors in
  orbit, and no computer fleet had ever fired.
- *"q4 a"* -- **the player is a target on the same terms**, with a cap of one new war a turn per
  seat. The cause is the shelter: a player who keeps a seat Neutral is marched on by nobody.
- *"q5 leave it"* -- the **Army cap** stays at fewer than two per seat.
- *"q6 fix it"* -- both measured defects fixed: an **occupier stays** where it is, and the Battle
  line and the Occupation read **one predicate** for "alone at the place", so an escaped attacker
  is never promised an Occupation it will not begin.
- *"q6 unchanged"* (Q7) -- **Carriers** stay the Prospectors' alone.

## Settled by the builder, to be corrected if wrong

- **The cause in orbit** is Cold or worse toward the seat holding Orbital Control against you, or
  toward any enemy present when nobody holds it; the old allowance, breaking a blockade at a Body
  where you have a Colony, stands beside it.
- **The cap counts proposals, not commitments**: at most one attack on a rival-held place is put
  to the seat's list a turn; marches on neutrals are not counted.
- **An occupier holds at three times the Hold weight** and is offered no march at all from the
  place it occupies.
- **`war_cause = -5`** in `ai.toml`, the Smear's gate: Cold or worse.

## Witnessed red

With the cause switched off the test failed at *"Cold toward the Prospectors: the Custodians march
on Russia"*, while under the same mutation they still marched on neutral India, which is the rule's
other half. Restored, `343 passed`, `6 passed`, clippy clean with `-D warnings`.

## Measured, twenty seeds with the Custodians first

| figure | neutrals batch | this ticket |
|---|---|---|
| wins, Custodians / Prospectors / Arkwrights / Archivists, of 20 | 0 / 16 / 0 / 2 | **0 / 16 / 0 / 2** |
| collapses | 2 | **2** |
| war ppm a game by seat, median | [0, 9.25, 1.0, 0] | **[0, 8.0, 0.5, 0]** |
| war ppm over the batch, by seat | [7, 169.5, 20, 5] | **[6, 168.5, 15, 4.5]** |
| seeds with any blockade | 4 | **15** |

**The freed seats still do not fight, and the reason is upstream of the gate.** The Custodians,
Arkwrights and Archivists never build an Army: the build-Army candidate scores 4 to 5 and loses
every turn to Power Plants and Trade Posts, as the 0.08.4 review measured ("passed over 70+ times
a game"). A seat with no Army has nothing to march. The test proves the rule fires when a Cold seat
has an Army beside a weak rival Region; on today's computer board the binding constraint is the
build weight, not the attack gate, and that figure belongs to the balance version the map's fog
already names. The one thing that moved is orbit: blockades in 15 seeds against 4, since a warship
stack in a rival's slot is now offered the stance every turn and the Attack-or-Hold contest no
longer swallows it.

# Version 0.07.6, the small-revisions version: the pictures

The map is [Map: version 0.07.6](https://github.com/whaleyjoshua2/Dying-Earth/issues/172). Every
picture here was taken headlessly in `shot:` mode on `version-0.07.6`; nothing was opened on the
designer's desktop.

## A Tech pick is not locked in until the turn ends

Ticket [#173](https://github.com/whaleyjoshua2/Dying-Earth/issues/173). The designer's line: *"tech
choice is not locked in until the turn is ended."*

Until now a pick was final the instant it was pressed: the shortlist was thrown away, every banked
point of Research poured into the new Tech, and a Tech with enough banked behind it could finish in
the middle of the Orders phase. A misclick could not be taken back.

Now a human Lead's pick is **provisional**. Pressing a box records the choice and stops there.
Pressing another box replaces it, as often as you like, for as long as the turn lasts. Ending the
turn commits it, and from then on it is the settled Tech under research.

![The Tech Tree with a pick that can still be changed](tech-pick-chosen-not-locked.png)

The chosen box wears a **paler amber** than the settled one, reads `chosen`, and loses its own
`Pick` button (there is nothing to press it for). The legend carries the new colour beside the
settled amber it has to be told apart from. Above the tree, in the Lead's own colour:
*"Chosen for this turn. Press another box to change it; it is locked in when the turn ends."*

**Decided by the designer** on the ticket:

- **The pick spends nothing until it commits.** The banked Research pours in at the commit, not at
  the press, so a Tech finished by the bank now lands with the turn's other Moments instead of
  arriving mid-Orders.
- **The Archivists' Provisional Findings reads the Tech frozen at the turn's head**, not whatever
  is picked this instant, so a Lead changing its mind cannot re-price orders already placed.
- **The shortlist is kept while the pick can change.** Redrawing the three on every change would be
  a free reroll.
- **A computer seat's pick commits in the same breath**, since it never changes its mind.

A new test, `a_tech_pick_can_be_changed_until_the_turn_ends`, pins the whole rule: the pick is
provisional, spends nothing, can be changed twice, commits when the turn ends, and is final after.
Three older tests were repinned with their reasons written in. The rule was witnessed red before it
was believed green: reverted on purpose, the new test failed, the rule restored, 255 pass.

The headless driver reads the pick too, so it was taught the same rule. Its board now says
*"Deep Mining IS CHOSEN FOR THIS TURN, and not locked in until the turn ends"* where it used to
demand a pick, and two `tech` lines in one turn now both apply, the second winning.

# Dying Earth — version 0.09.3, the frontier version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.09.3](https://github.com/whaleyjoshua2/Dying-Earth/issues/385), and the
pictures and batches that decided it are in
[`docs/dev-diary/2026-09-26-version-0.09.3/`](../dev-diary/2026-09-26-version-0.09.3/).

**What the version is.** Version 0.09.2 with the designer's list. The sections are written as the
tickets close; the summary is written when the last one does.

**What it did to the win column** (80 games, per Faction, at the shipped climate cell):

| | 0.09.2 | 0.09.3 |
|---|---|---|
| Custodians | 2 | |
| Prospectors | 5 | |
| Arkwrights | 0 | |
| Archivists | 11 | |
| collapses | 62 | |

---

## 1. Defects from the list, and the Tech tree audit

The authority is [ticket #386](https://github.com/whaleyjoshua2/Dying-Earth/issues/386). No rule
moves in this section; each item is a thing the game said untruly or said badly.

1. **The Fund is Ducats everywhere it is named.** The headless driver's status line reads
   *"Venture Capital Fund: 0 Ducats, banking 0% of Ducat income (0 banked last turn)"*; the
   Report's line for setting the share reads *"bank 40% of its Ducat income"*; the Investment
   Bank's text reads *"never less than 1 Ducat in all"*. The window's own line already said
   Ducats. The comments in the code that still said Materials say Ducats.
2. **End Turn's refusal names everything owed at once.** When a human seat owes both this turn's
   card an answer and the table a Tech, the one message reads *"Two things before the turn can
   end:"* with one line each under it; when one thing is owed, the message is that thing's line
   with *"The turn cannot end until you do."* after it, as before. The sun's hover, Enter's
   refusal and the driver's *STILL OWED* all print the engine's message. The driver's own Tech
   banner says *MUST* only when the engine's End Turn would refuse without the pick, and *"A TECH
   IS OPEN TO PICK … the turn can end without it"* when the tree merely has nothing chosen.
3. **The Tech tree audit found nothing stale by name.** No Tech names a Choice Card, and every
   Event a Tech's effect line names exists and matches the code. Four Techs under-state what they
   blunt (Hardened Hulls also the Meteor Shower; Closed-Loop Colonies also the Reactor Leak, Dust
   Storm and Moonquake; Efficient Grids also doubles Solar Maximum and trebles the Helium-3 Vein;
   Green Consensus also halves the Methane Burst and cancels the Drought), and the designer chose
   to leave those lines as the headline effect. **The Launch Pad Fire's text says the live rule**:
   *"Its Launch Site is offline until the next Resolution, so nothing lifts to orbit from there,
   and the Region loses the turn's Widgets"*, where it had said a due Ship completes next turn.

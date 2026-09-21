# 2026-09-20: version 0.08.5, the war version

The dev diary for version 0.08.5, charted as wayfinder map
[#275](https://github.com/whaleyjoshua2/Dying-Earth/issues/275). As 0.08.4's, this folder is a
record of the **build**: one folder per ticket, written as each was resolved, holding the pictures
and the measurements that decided it. The rules as decided are in
[`docs/spec/version-0.08.5.md`](../../spec/version-0.08.5.md), written by the closing ticket.

**Every folder here is listed.**

| folder | ticket | what is in it |
|---|---|---|
| [`sweeps/`](sweeps/) | [Write the 0.08.5 amendments and build them](https://github.com/whaleyjoshua2/Dying-Earth/issues/287) | `final-0.08.5.txt`: the closing sweep, 20 seeds × four seatings at the shipped climate cell, the first with a military block. Win column 28 / 26 / 9 / 2 against 26 / 28 / 7 / 0, collapses 15 of 80 against 19. `baseline-0.08.4-military.txt`: the same sweep on `main` with the counters ported into a throwaway worktree, so the 0.08.4 rules have the military figure the map asked for: 113 Battles over eighty games against 172. |
| [`../../playtest/PLAYTEST.txt`](../../playtest/PLAYTEST.txt) | [Write the 0.08.5 amendments and build them](https://github.com/whaleyjoshua2/Dying-Earth/issues/287) | The playtest note that ships in both kits as `README.txt`. It lives at `docs/playtest/` while this is the current version, because the kit workflow copies it from there; 0.08.4's is filed under [`playtest/`](../2026-09-19-version-0.08.4/playtest/PLAYTEST.txt) in its own folder. |
| [`ticket-276-sea-inland/`](ticket-276-sea-inland/README.md) | [The sea reaches inland](https://github.com/whaleyjoshua2/Dying-Earth/issues/276) | China's card after a bare rise and behind a standing wall, the coast one slot further in each time; a batch of twenty with the turned slots counted. |
| [`ticket-277-greenwash/`](ticket-277-greenwash/README.md) | [The Greenwash](https://github.com/whaleyjoshua2/Dying-Earth/issues/277) | The block on the player's own Faction page, and a batch in which the Prospectors greenwash 90 ppm a game and buy more credits, not fewer. |
| [`ticket-278-blockade/`](ticket-278-blockade/README.md) | [A Blockade that starves](https://github.com/whaleyjoshua2/Dying-Earth/issues/278) | The ISS's card reading *Blockaded by the Prospectors: producing nothing, upkeep still paid*, and a batch in which the Prospectors blockade in four seeds of twenty. |
| [`ticket-279-battles-pollute/`](ticket-279-battles-pollute/README.md) | [Battles pollute](https://github.com/whaleyjoshua2/Dying-Earth/issues/279) | The Climate Panel's *War 1.0* line after a Battle in Earth orbit, and a batch in which the Prospectors wear 9 ppm a game of war and nobody's share is nought. |
| [`ticket-280-buildings-say/`](ticket-280-buildings-say/README.md) | [Buildings that say what they do](https://github.com/whaleyjoshua2/Dying-Earth/issues/280) | China's card with the Launch Site's row reading what it does instead of *no output*, and the sixteen sentences. |
| [`ticket-281-battle-report/`](ticket-281-battle-report/README.md) | [The Battle Report](https://github.com/whaleyjoshua2/Dying-Earth/issues/281) | Two Reports after a four-stack Battle at Mars, one with a Ship lost and one with nobody lost: the Battle's own line, every unit by name with what it took, and the odds each attacker faced. |
| [`ticket-282-neutrals-arm/`](ticket-282-neutrals-arm/README.md) | [Neutral states arm when threatened](https://github.com/whaleyjoshua2/Dying-Earth/issues/282) | India's card with its Standing Army and its Levy, and a batch in which nothing arms because no neutral Region is left by the time an Army exists. |
| [`ticket-283-yields-in-glyphs/`](ticket-283-yields-in-glyphs/README.md) | [Yields in glyphs on the planet card](https://github.com/whaleyjoshua2/Dying-Earth/issues/283) | The Mars planet card with its Colonies block, the Olympus Mons card with its yields, the slot panel without the planet line, and the Ship card's doors unchanged. |
| [`ticket-284-ai-attack/`](ticket-284-ai-attack/README.md) | [The computer seats may attack a Region they did not lose](https://github.com/whaleyjoshua2/Dying-Earth/issues/284) | The batch in which the freed seats still do not fight, because they never build an Army: the build weight, not the gate, is the binding constraint. |
| [`ticket-285-credits-move/`](ticket-285-credits-move/README.md) | [Carbon credits move from the Trading window to the Faction window](https://github.com/whaleyjoshua2/Dying-Earth/issues/285) | The Offer block on the Custodians' own page, the Request block on their page as a rival sees it, and the Trading window with four lines. |
| [`ticket-286-count-the-war/`](ticket-286-count-the-war/README.md) | [Teach the sim and the sweep to count the war](https://github.com/whaleyjoshua2/Dying-Earth/issues/286) | No picture; a batch of twenty with every seed's Battles, Armies built and lost, and places taken by force counted at the event, and the sweep's new military block under the Influence line. |

Everything here was captured headlessly in `shot:` mode. **Nothing was opened on the designer's
desktop.**

## What the pictures changed

Once in this version a picture found something reading the code had not: **the Battle's line did
not headline** ([#281](https://github.com/whaleyjoshua2/Dying-Earth/issues/281)). The first Report
picture after a Battle at Mars showed the destroyed Ship's own line heading the turn and the Battle
under it, because the Battle line was written after the losses were applied and ranked behind
them. The line is now written first. Nothing in the suite reads a Report's order.

## What the batches found

- **No Battle in eighty games is against a neutral Region**, at either rule set. Every Region is
  taken by Influence in the first turns and the first Army appears around turn 20, so *Battles
  pollute* puts nothing on nobody's ledger and *Neutral states arm* raised three Levies and held
  nothing. Both are rules for a human who marches early ([#279](https://github.com/whaleyjoshua2/Dying-Earth/issues/279),
  [#282](https://github.com/whaleyjoshua2/Dying-Earth/issues/282)).
- **The attack rule freed the Arkwrights, not the timid seats.** The Custodians and Archivists build
  few Armies; the Arkwrights where they go first were already building sixty a batch and fighting
  none, and now fight 52 ([#284](https://github.com/whaleyjoshua2/Dying-Earth/issues/284), [#286](https://github.com/whaleyjoshua2/Dying-Earth/issues/286)).
- **The two Blame levers add.** Credits bought rose in every seating and the Greenwash takes 70 to
  90 ppm a game on top ([#277](https://github.com/whaleyjoshua2/Dying-Earth/issues/277)).

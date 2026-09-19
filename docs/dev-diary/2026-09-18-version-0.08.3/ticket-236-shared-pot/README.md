# Ticket #236: the shared pot

> Factions contributing less than 85% of their research to the shared pot suffer a minor relations
> hit, every faction contributing 100% gets a minor boost

## The measurement came first, and it said the rule could not work as written

[The Research Directive](https://github.com/whaleyjoshua2/Dying-Earth/issues/235) had just shipped
and it changed the ground under this rule. Measured with throwaway counters, 20 seeds:

| | Custodians | Prospectors | Arkwrights | Archivists |
|---|---|---|---|---|
| Directive at end | 50% | 50% | 50% | 100% |
| First set on turn | 6 | **2** | **2** | **2** |
| Turns below 85% contribution | 24 | 28 | 28 | 28 **of 36** |

Every seat went to its cap on turn 2 and never moved — the AI acted only when the directive was
zero and nothing lowered it. So the penalty would have fired on **everyone for about 26 turns of
36** and the reward would have fired **never**. At −1 per rival per turn that is roughly 78 points
of damage a seat, on a scale that bottoms at −10.

## The designer's answer fixed the rule without touching the rule

**The AI's greed was the bug, not the threshold.** At their word — *"the ai need to weigh the
benefit of the new tech to which they contributing"* — a seat now reads the Tech under research
against the pick list it already has: its own Victory gate or anything on its `order` and it gives
everything; a Tech it leaves until `last` or `never` wants and it takes its whole cap; anything
else and it keeps a little, staying above the line.

| turns of 36 below 85% | Custodians | Prospectors | Arkwrights | Archivists |
|---|---|---|---|---|
| Before | 24 | 28 | 28 | 28 |
| **After** | **0** | **2** | **0** | **28** |

The three non-Archivists now change their directive **7 to 10 times a game** and sit at 100%
contribution for 5 to 11 turns. **85% became a line a Faction crosses deliberately**, which is all
the rule ever needed.

## The other three answers

**A term, not a deed.** *"plus/minus 1 but only for that turn"* — read afresh every settle from
what the Faction is doing now, exactly as the Blame term is. Nothing is banked, and a Faction that
starts contributing again is forgiven the same turn. This is what makes the rule survivable: as a
deed it was 78 points a game, as a term it is at most one point at any moment.

**The reward stops at the top of Cordial**, the step above Neutral. Version 0.08.2 settled
deliberately that a pair which never strikes an Accord can never rise above Neutral; this bends
that by one band rather than breaking it. A pair already higher by deeds is not dragged down to it.

**It never scars.** A Faction spending its own Research on its own business has done nothing to
anybody, so the penalty does not feed the scar ratchet.

One thing the designer's "yes" to the one-act-a-turn cap turned out not to reach: that cap governs
**deeds**, and a term is not one, so the question dissolved rather than being answered. Recorded
rather than quietly decided.

## What it does to the board, and the one thing nobody chose

Relations end a little worse: median **−4** against −3 before the rule, 104 pairs at Cold or worse
against 100, 42% scarred against 38%.

Nearly all of that is **the Archivists**, who sit below the line for **28 turns of 36** because
funding the Archive *is* their Victory path. **Their Victory Condition now costs them standing with
everyone, permanently**, on a Faction that wins 7 of 80. It may be exactly right — a Faction
hoarding knowledge being resented for it is a good rule — but nobody chose it on purpose, and the
closing sweep should be read with it in mind.

## Looked at

| picture | what it shows |
|---|---|
| [`the-reward.png`](the-reward.png) | The Research Directive control at 100% contribution, with the new line beneath it: **"Every rival thinks a little better of you for giving all of it."** The slider carries the contribution, so a player compares one number to one line. |

Red witnessed: the step was zeroed and the three tests guarding the rule watched to fail.
`307 passed; 0 failed`, `6 passed; 0 failed`, clippy clean with the denial.

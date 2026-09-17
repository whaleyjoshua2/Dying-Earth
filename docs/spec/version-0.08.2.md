# Dying Earth — version 0.08.2, the dealings version: Blame that costs a Faction its friends, Relations on a named scale that can be repaired, five Accords to strike, prices that move, and a board that says whose and which

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.08.2](https://github.com/whaleyjoshua2/Dying-Earth/issues/215), and the
pictures and measurements that decided it are in
[`docs/dev-diary/2026-09-15-version-0.08.2/`](../dev-diary/2026-09-15-version-0.08.2/).

**What the version is.** Since version 0.08.0 the **Relations** score has existed, been kept per
ordered pair, and been read by nothing. This version gives it a **cause** (Blame, weighted by how
much each Faction cares), a **vocabulary** (six named levels in place of a bare number), a **ladder**
(offences that differ in weight), a **way back up** (the +10 half, which 0.08.0 reserved and left
empty), a **memory** (a floor that ratchets), a **bite** on the board (a term on the challenge
margin), and a **use**: five **Accords** two Factions can strike.

Around that: the **Trading window's prices move** with what the table bought, and five smaller
changes make the board say **whose** a thing is and **which** kind it is — the Faction's glyph alone
on a card's heading, every Ship listed by name, a free build slot that says what clicking it does, a
founding button that carries its own figures, a Faction selection screen whose four cards line up,
and a Research race bar that reads in names and percentages.

**Three rules on the designer's list were found undeliverable during charting and dropped**, each
before a line of code was written for it:

- **Buying out the Trading window** as an offence — the window has no stock and cannot be bought out
  ([#222](https://github.com/whaleyjoshua2/Dying-Earth/issues/222)).
- **A Resettle into a state they direct** as an offence — a Resettle can only ever target a state
  *you* direct, so the refugees always land in your own territory
  ([#222](https://github.com/whaleyjoshua2/Dying-Earth/issues/222)).
- **Relief, a Scrubber, or helping Influence** as acts of friendship — the first two are refused on a
  rival's Region, and the third is byte-identical to an offence
  ([#223](https://github.com/whaleyjoshua2/Dying-Earth/issues/223)).

---

## 1. A card's heading wears the Faction's glyph alone, and every Ship is listed by name

*Ticket [#216](https://github.com/whaleyjoshua2/Dying-Earth/issues/216).*

**The kind glyph comes off exactly two headings**: the Colony card and the Ship stack heading. The
Faction symbol stays and stands alone. **The Roster's row glyphs and `glyph_button` are untouched** —
in a list of thirty rows the glyph is the only thing separating a Colony row from a Region row at a
glance, where on a card the kind is already said by the name and the card's contents.

**A neutral place therefore wears no glyph at all**, which is consistent with the standing rule that
a neutral place wears no mark. A picture of a neutral Colony card is to be looked at before this is
called done.

**The Roster keeps one row per Ship**, replacing the row-per-stack. A row reads
`TSV Valiant (colony ship) — strength 3, 30/30`: the working figures stay **on the row**, because the
Roster is where a player checks whether a hull can move before ordering it. **Stranded becomes
per-Ship** rather than an all-or-nothing warning on the stack. A transiting Ship reads
`TSV Magellan (colony ship) — in transit to Mars, 3 turns left`.

**`(colony ship)`, not `(colony)`** — Colony is a settlement and appears a few rows below in the same
Roster.

This is affordable because it was measured: over 200 games the **median Faction holds zero Ships**,
one or two is typical, and the largest fleet ever seen was 27.

**On the maps, Ships at a Body are visually unchanged.** The per-Faction block stays exactly as it
is, strength and all. **A Ship in transit gains its name** on the Solar System Map:
`TSV Magellan (colony ship): 3 turn(s)`, replacing `Custodians Colony Ship: 3 turn(s)`. The Faction
word goes, because the prefix says whose and the label is already drawn in the Faction's colour.

**New: hovering a Faction's line in the in-orbit block lists that Faction's hulls**, on both the
Solar System Map and the Body Surface Map. **Name and type only**, capped at **five** with a tail
(`and 19 more — click the stack`), which keeps the standing six-line tooltip convention. **Click is
unchanged** and still selects the stack.

The two maps are not symmetric work: the Solar System Map already pushes a hotspot per Faction line
and the hover reuses it, where the Body Surface Map's block is painted at fixed positions with no
hotspots at all.

---

## 2. The Faction selection screen: four cards of one height

*Ticket [#217](https://github.com/whaleyjoshua2/Dying-Earth/issues/217).*

**All four cards take one height**, the tallest's, making a true 2x2 matrix, with the **Play button
pinned to each card's foot** so the four buttons line up. Equalising the frames alone would have left
the raggedness actually visible: at 1920x1080 the Custodians' button sits at 433 and the Prospectors'
at 454.

**There is no fit target.** The grid is allowed to scroll at smaller resolutions, to be revisited if
it proves clunky in play. **The scrollbar is shown only when scrolling is actually required**, and is
quiet when everything fits — which is the fix for the real defect found in charting: at **1280x800
and 1600x900 two of the four Factions have no visible Play button at all**, the row being clipped
with nothing on screen saying the page scrolls.

**Block headings stay as they are**, each on its own line.

**The signature rules are rewritten tighter, with no substantive information lost.** This is the one
change that shrinks the grid, since every card now takes the tallest card's height and the
Custodians' signature rule at **752 characters** is what sets it. The rewrite is brought to the
designer before it ships — four paragraphs, old and new side by side, with character counts, checked
against the glossary's `_Avoid_` lists.

**`Play Tutorial` scales by 1.2**, from a single named constant, as ticket #211 handled the command
cluster's 1.15. Placement is unchanged.

**Recorded trade:** one height and accepted scrolling pull against each other — making every card as
tall as the tallest makes the grid taller, so clipping at 1280x800 becomes more likely, not less. The
prose edit is what claws some of it back.

---

## 3. Two build doors: `Click to Build`, and a founding button that carries its own figures

*Ticket [#218](https://github.com/whaleyjoshua2/Dying-Earth/issues/218).*

**A free tile says `Click to Build`, on two lines**, in place of the word `free`. The tile is 84
pixels square and the phrase is about 78 wide, so one line would leave three pixels of air and break
if the font moved.

**Only on tiles the player could actually build in**: their own places, unflooded. A **flooded**
coastal slot keeps its own state; a Region or Colony the player does **not** control keeps `free`,
because its boxes are still clickable but the build strip beneath offers nothing. A Colony **at its
Module cap** needs no rule — it has no free tiles at all.

**All three grids take it**: a Region's build slots, a Colony's Modules and a station's Modules.

**The line under each grid shortens** to *"Click a box for its figures and controls."* The second
half is now said by every free tile; the first half is the only statement of what clicking a **built**
box does.

**The four yields move onto the founding button's face**, as **glyph and number**, in the format the
Body Surface Map already uses under every slot label:
`[materials] 1.78  [energy] 1.37  [fuel] 0.51  [research] 1.09`. **Both founding buttons change
identically** — the Ship panel's and the Body Surface Map's. **The hover is dropped**, which loses
nothing: it held the same four figures in words.

**Recorded gap, neither created nor closed here:** the yield scale is explained nowhere in the game.
Nothing tells a player that 1.00 is ordinary.

---

## 4. The Research race bar on the Tech Tree

*Ticket [#219](https://github.com/whaleyjoshua2/Dying-Earth/issues/219).*

**The bar stays at the top of the window; the LEGEND moves up to sit directly beneath it**, with the
tree below both. The literal reading — the bar below the legend at the window's foot — was ruled out
by measurement: the window is `resizable(false)` around a fixed-size tree and **does not scroll**, so
anything below the legend makes the **window** taller, and at the default 1280x800 it already reaches
the bottom of the screen.

**A percentage is each Faction's share of what has been contributed so far**, the four adding to
100%, matching the segment widths above them. The `N of M` progress is unchanged.

**A Faction that has contributed nothing is not shown at all.** Its absence from the line and the
grey remainder on the bar say the same thing twice.

**Each entry wears the Faction's symbol in its own colour**, which keeps the legend's swatches
beneath meaning **states** rather than **owners**.

**When no Tech is under research** — the state this window is open in when a pick is owed — the line
`No Tech under research. N Research waiting.` stands in place of **the whole block**, with the legend
directly under it.

**The bar's hover trims** to what the face does not say: the Tech's name, `N of M`, the **raw
contribution counts**, and the Research Lead. **The top bar's copy of the bar is unchanged** and
keeps its full hover.

---

## 5. Market prices move with what the table bought

*Ticket [#220](https://github.com/whaleyjoshua2/Dying-Earth/issues/220).*

**Only `Order::Buy` and `Order::Sell` move a price.** Buying a building outright for Ducats does not:
it takes no goods off the table.

**Every base price rises by 1, and the base becomes the MIDPOINT of a band**: Materials **2 -> 3**,
Fuel **3 -> 4**, Energy **1 -> 2**. **The band is +/-1**: Materials 2-4, Fuel 3-5, Energy 1-3. Every
price stays a whole number and no resource can become free.

**20 net units moves a price one step**, chosen against the measurement below, where a trading turn
carries a median 20 units across the whole table.

**At the edge of the band the price sticks and trading continues normally.** The window does not
begin refusing — that would invent a stock limit this game does not have.

**A price drifts one step back toward its midpoint only on a turn in which nobody traded that
resource at all.** A resource under steady demand stays dear rather than sliding home between
purchases.

**A sale returns half of the MOVING price.** **The window shows the price and which way it moved;
last turn's volume is hidden.**

**What was measured, and what the designer decided in the face of it.** Over 80 games the computer
seats placed **zero Sell orders** and bought **only Materials** — 29,440 units, no Fuel, no Energy —
with the Prospectors alone accounting for 61% of all trading. `engine/src/ai.rs` says so in its own
comment: *"the AI does not sell."* **The rule ships as written anyway**, with the known consequence
that **Materials will see one-way upward pressure from the computer seats, and Fuel and Energy will
move only when the human trades them.** Giving the seats an appetite to sell, and to buy Fuel and
Energy, is a separate ticket.

**Raising all three prices by 1 is a substantive balance change**, not a display fix: Materials +50%,
Fuel +33%, Energy +100%, on a currency that also buys Influence, Relief, Resettle and Leapfrogs. It
also makes the AI's fixed lot of ten Materials cost 30 Ducats instead of 20.

**The prices become saved state**, so the save format changes and an older save is refused.

---

## 6. Blame costs a Faction its friends, and the score gets a vocabulary

*Ticket [#221](https://github.com/whaleyjoshua2/Dying-Earth/issues/221).*

**One stored number is the history of deeds** — offences lower it, acts raise it — and the **shown**
score is `deeds + blame_term`, recomputed at every settle.

**The Blame term is a LEVEL, not a charge.** It is read off the offender's current share each settle
rather than added each turn, so a Faction that cleans up is forgiven without needing quiet turns.

**Only the SHOWN score clamps to -10..+10. The stored deeds figure runs past +10 internally**, to a
ceiling of +15 (section 8).

**The step is `floor((share - 0.25) / 0.10)`**: 0 below 0.35, 1 at 0.35, 2 at 0.45, 3 at 0.55. **The
first step stays at 0.35** — a clean Faction being untouchable is the reward for being clean.

**Multiplied by the RESENTING Faction's coefficient**: **Custodians x2, Archivists x1, Arkwrights
x0.5, Prospectors x0.** The coefficient belongs to the viewer; the share to the Faction being viewed.
**The Arkwrights' x0.5 rounds TOWARD ZERO** — they feel nothing until 0.45.

**The Blame term is capped at -5**, half the scale: Blame can make a pair Cold but never, by itself,
Hostile.

**A heavy-emitting turn does NOT also count as an offending turn.** Blame gets one mechanism.

### The six levels

| score | level |
|---|---|
| +7 to +10 | **Friendly** |
| +3 to +6 | **Cordial** |
| -2 to +2 | **Neutral** |
| -3 to -5 | **Wary** |
| -6 to -8 | **Cold** |
| -9 to -10 | **Hostile** |

Neutral spans -2 to +2 deliberately: one offence should not rename a relationship. **The level is the
primary reading, with the number on the hover** — `Wary` on the row, `-4 (-2 Blame)` when hovered.

### What this rule actually does, measured

Over 200 games, sampling every turn:

| Faction | median share | >=0.35 | >=0.45 | >=0.55 | games ever >=0.35 |
|---|---|---|---|---|---|
| Custodians | 0.158 | **0.0%** | 0% | 0% | **0/200** |
| Prospectors | **0.403** | **60.6%** | 44.7% | 27.3% | 162/200 |
| Arkwrights | 0.186 | 13.0% | 4.3% | 1.3% | 50/200 |
| Archivists | 0.151 | 4.2% | 0.1% | 0% | 28/200 |

**It is very nearly a rule about how everyone feels about the Prospectors**, who are over the line in
the median game from **turn 1** and climb to ~0.53 by turn 36. The Custodians never crossed 0.35 once
in 200 games, so **nobody ever resents them** and they pay no Relations cost for what they emit. The
Prospectors resent nobody, by their x0.

**The Custodians and the Prospectors are left pointed, deliberately.** At Friendly = +7 and a
standing Blame term of -2 to the capped -5, that pair reaches Friendly only with deeds of +9 to +12
during a relatively clean Prospector stretch — brutally hard rather than impossible. Recorded as a
known outcome to be revisited, not as a bug.

---

## 7. A ladder of offences

*Ticket [#222](https://github.com/whaleyjoshua2/Dying-Earth/issues/222).*

| offence | weight |
|---|---|
| Influence spent on a place they hold | 1 |
| Blockading an Orbital Slot at a Body they hold | 1 |
| A Strip Permit in a Region bordering one of theirs | 1 |
| Violating a term of an Accord while it stands | 3 |
| Landing an Army and beginning an Occupation | 3 |
| Opening a Battle | 3 |
| Taking a place from them outright | 5 |
| Destroying a Scrubber or an Archive by capture | 5 |

**Taking a place is a ONE-TIME charge**, not a standing offence while it is held.

**A turn charges the SUM of every instance, capped at 8.** **An instance is a PLACE or an ACT, never
an ORDER** — one charge per Region or Colony of theirs spent on this turn, however many Influence
orders it took; per Battle opened; per place taken; per slot blockaded.

**This knowingly reverses a measured decision from version 0.08.0**, which charged a flat 1 per
offending turn because charging per order "would make a big push and a small one differ by a factor
nobody can read off the board". Charging per **place** keeps the spirit of that objection — splitting
one spend into three orders cannot multiply the damage — while giving proportionality: a turn pushing
on three of a rival's places genuinely costs three times a turn pushing on one.

**The surviving new offence is the Strip Permit in a bordering Region**, at weight 1. It offends
every Faction that directs a Region **neighbouring** the one permitted. **Kept at 1 deliberately**:
the Strip Permit is the Prospectors' faction-only order, so this offence can only ever be committed
by them, and the ladder should not pile a third anti-Prospector mechanism on the two they carry.

**The ordinary, unsteered Refugee flow offends nobody.**

**What the ladder is in practice**, measured over 80 games: Influence on their places is **72%** of
all charged points and taking a place outright **20%**, while Battle and Occupation fired **seven
times in 80 games** between them and Blockade never fired at all. All but 4 of 1,121 control
transfers were by Influence. **It is a 1-and-5 ladder, not a 1-and-3 one.** The per-turn cap of 8
bites on only 1.7% of offending pair-turns.

**The Report should now say WHAT gave offence**, not merely that offence was given.

---

## 8. The half above neutral

*Ticket [#223](https://github.com/whaleyjoshua2/Dying-Earth/issues/223).*

**No new orders are added for this.** The acts that raise Relations are exactly two, both defined in
section 11: **tribute paid**, and **an Accord kept**.

**An act is worth +1**, capped at **one act a turn per ordered pair**.

**The stored deeds figure may climb to +15.** That leaves Friendly (+7) reachable against a floored
Blame term of -5 with deeds at +12, and the extra three points stop a maxed relationship being
knocked out of Friendly by a single Blame step.

**A positive score decays toward Neutral by 1 every EIGHT quiet turns** — half the rate the negative
half recovers at.

**An act and an offence in the same turn BOTH count, and the effect is their sum.**

**The deliberate consequence: a pair that never strikes an Accord can never rise above Neutral.**
Since 41-45% of ordered pairs never interact at all in a game — measured, and the reason the +10 half
was left empty in 0.08.0 — most pairs will sit at Neutral or below for a whole game. Goodwill is now
evidence of a specific history, never a product of distance.

**The asymmetry, stated plainly:** a hostile turn costs up to 8; repair runs at +1 a turn.
**Relationships break roughly eight times faster than they mend**, and undoing one conquest turn
takes eight acts across eight separate turns of a thirty-six turn game.

---

## 9. Relations bites the board

*Ticket [#224](https://github.com/whaleyjoshua2/Dying-Earth/issues/224).*

**The margin a challenger faces is `20 + floor(|score| / 4)`**, where `score` is the **controller's
view of the challenger**, applying **only when that score is negative**. **The true maximum is +2**:
the shown score clamps at -10 and `floor(10 / 4)` is 2, so Wary gives 0, Cold +1 and Hostile +2.

**A positive score does NOT lower the margin.** Negative only.

**The whole SHOWN score counts, Blame included.** Against the measured Blame distribution that gives
the Custodians a standing **+1 on about 45% of turns** against the Prospectors, reaching +2 once
deeds are added. **Recorded risk:** the Custodians already win 42 of 80, and Blame already raises a
dirty Faction's Threshold everywhere, so this adds a second permanent defensive advantage to the
strongest Faction against the weakest. The sweep must watch it.

**Structurally the margin stops being a property of the PLACE and becomes one of the ORDERED PAIR.**
`challenge_margin_at(target)` takes only the target today, deliberately — the Constabulary "protects
whoever holds the place, not the Faction that raised it". The plumbing is cheap:
`influence_needed_for(seat, target)` already carries the challenger's seat and simply does not pass
it down. It stacks with the Constabulary, so a garrisoned Region held by a Faction that loathes you
reads 20 + 10 + 2 = **32** at the extreme.

**The card prints the margin as it applies to the player**, with the Relations term as its own line
naming the level (`Wary: +1`). A rival's margin is not shown.

**The feedback loop** — offending raises the margin, which lengthens the push, which is more
offending — is guarded by the cap at +2.

---

## 10. The scar

*Ticket [#225](https://github.com/whaleyjoshua2/Dying-Earth/issues/225).*

**The floor ratchets on offending TURNS, not on points charged**: one step per **4 turns on which any
offence was given**, whatever they cost. A scar is about **persistence, not severity**, and counting
turns is immune to the ladder being retuned later.

**The floor limit is -3.** A pair crossed often enough can never recover above -3 — the top of Wary —
for the rest of the game.

**The floor caps the STORED DEEDS figure, not the shown score**, so that a Faction which cleans up
its emissions does not find its forgiveness silently eaten by an old grudge.

**An Accord kept lifts the floor by one.** Peace-making is the only thing in the game that undoes a
scar.

**-3 rather than the -5 first proposed, deliberately.** Since every act that raises Relations is now
an Accord act, a pair floored below the level at which an Accord can be struck would have **no road
back at all**. At -3 a scarred pair can still strike an Accord, keep it, and lift its own floor.

**Measured**, over 80 games: a pair that ever offends has a median of **10 offending turns** (10th-90th
2-19, max 24), so at one step per 4 the **median offending pair reaches -2**, one step short of the
limit, and a hot pair at the 90th percentile is floored. **41-45% of pairs never offend at all** and
are never scarred. Offence is **heavily back-loaded**: 90% of it lands after the halfway mark.

---

## 11. Five Accords

*Ticket [#226](https://github.com/whaleyjoshua2/Dying-Earth/issues/226).*

An **Accord**, holding one or more **Terms**. Both words are new to the glossary; *diplomacy* stays
on Relations' `_Avoid_` list.

| term | what it does | holds for |
|---|---|---|
| **Non-aggression** | neither spends Influence on a place the other holds, nor opens a Battle against the other | until ended |
| **Passage** | neither treats the other's Ships as a target; a Blockade does not shut the other out of the slot | until ended |
| **Refuel** | either may Refuel at the other's Space Stations | until ended |
| **Tribute** | a fixed **25 Ducats or 15 Materials**, one per pair per turn, paying **+1** Relations | one turn |
| **Research agreement** | both parties' Research output rises 10% while it stands; **needs Friendly to strike** | until ended |

**All five ship.** Passage and refuel were measured as near-dead on today's board — Blockade fired
**zero** times in 80 games, and off-Earth stations run at 0-1 a game while three Factions each hold
their own station over Earth — and **kept anyway**, because they will matter for Mars and later
builds.

**Both sides propose.** The computer seats propose to the player as a **Moment** or a line at the
head of the Report, with the terms and a yes or no. **The player proposes from the Faction window**,
which gains the controls for it. No new screen.

**A refused offer is NOT an offence.** **A Faction may hold Accords with two rivals at once**,
including non-aggression with two Factions who are fighting each other.

**Ending an Accord takes a turn's notice and is free**: declare it over, it lapses at the next turn's
start, then you may act. **Violating a term while the Accord still stands is an offence worth +3**
and **ends the whole Accord immediately** — without which a Faction could violate non-aggression
every turn, pay 3 each time, and keep drawing +10% Research from a partner it was attacking.

**The Friendly gate is checked only at the moment of striking.** Once made, a research agreement
stands whatever the score later does. Two Factions merely grown cold can therefore share one struck
in better times; they cannot do so through an actual betrayal, since violating any term ends the
Accord.

**The system is closed to these five terms.** A Region or Colony **never** changes hands in an
Accord, and **Techs are never traded**. *A future version may reconsider.*

**A computer seat accepts** when what it gets exceeds what it gives by its own weights, scaled by its
Relations score — the arithmetic `Candidate::score` already performs — with one hard floor: **it never
accepts a term that would lose it the game.**

**A new persisted structure on the save**, per ordered pair, so the save format moves and an older
save is refused.

---

## 12. What the sweep must say

Twenty seeds by four seatings at the shipped climate cell, against the 0.08.1 baseline:

| Faction | wins of 80 at 0.08.1 |
|---|---|
| Custodians | **42** |
| Prospectors | 11 |
| Arkwrights | 7 |
| Archivists | 6 |

Collapses 14 of 80. **Figures the sweep does not report today and now must**, because rules in this
version depend on them and none can be believed without:

- each seat's **Blame share** at the end, and its distribution over the game;
- the **distribution of Relations scores and floors** at the end — a version where every pair is
  floored at Wary is the failure mode;
- **takes per batch**, before and after, since the challenge-margin term is the one change here that
  can move the win table, and the **Custodians' win share** is the figure to watch;
- **Accords struck, by term and by seat** — a system the computer seats never use is invisible;
- **buy and sell volume per seat**, and **Ducat spending**, since every price rose by 1;
- **the turn the Tech Tree completes**, since two seats at +10% Research reach it sooner.

---

## 13. Vocabulary and housekeeping

`CONTEXT.md` gains **Accord** and **Terms**, and amends **Relations** (a cause, a ladder, a positive
half, a floor, six named levels and a bite), **Blame** (a second job), **Influence** and **Threshold**
(the margin's new term), **Trading window** and **Ducats** (prices that move), **Kind Glyph** (no
longer on a card's title), **Ship name** and **Roster** (a Ship by name, one row each), **Build Slot**
and **Module** (what a free tile says), and **Research** (the agreement's 10%).

**The Kind Glyph entry** loses the clause "a card's title" from its list and names the exception: a
card's title carries the Faction symbol alone, because a card already says what kind of thing it is.

Pictures are taken headlessly in `shot:` mode and filed under
`docs/dev-diary/2026-09-15-version-0.08.2/`. **The game executable is never run bare: it starts a
game.**

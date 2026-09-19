# Ticket #235: the Research Directive

Every Faction may now send a share of its Research somewhere other than the shared Tech, chosen as
a percentage and standing until changed.

| Faction | where it goes | rate |
|---|---|---|
| Custodians | the Natural Sink, **for good** | 0.01 ppm a point |
| Prospectors | Ducats | 0.8 a point |
| Arkwrights | Fuel | 1 per 5 points |
| Archivists | the Archive fund | one for one, to the fund's cap |

Every Faction may direct up to **half**; the **Archivists alone may direct all of it**, because
their switch always sent all of it and the slider that replaced it keeps that reach.

## The rates were fitted against a measurement, and it changed two of them

Throwaway counters, 20 seeds, reverted afterwards. **Research made over a whole 36-turn game,
median by Faction:**

| Faction | over a game | per turn | half, diverted |
|---|---|---|---|
| Custodians | **131** | 3.6 | **1.8** |
| Prospectors | 442 | 12.3 | 6.2 |
| Arkwrights | **149** | 4.1 | 2.1 |
| Archivists | 453 | 12.6 | — |

That is far less Research than the rule was drafted against, and it inverted the advice on the
ticket:

- **The Custodians' 0.05 ppm a point had no safe reading.** Per-turn it is 0.09 ppm against a
  Natural Sink of 6.0 — one and a half per cent, invisible. Permanent it is **+2.7 ppm over a
  game**, a 45% enlargement of the Sink, on the Faction that already wins 37 of 80 on exactly that
  condition. The designer took **permanent and cut the rate to 0.01**, which is about +0.5 ppm over
  a game: half a Planetary Stewardship, earned over thirty turns of not researching.
- **"10-1" was ambiguous by a factor of a hundred.** Ten points to a Ducat is 22 over a game, less
  than half a Leapfrog. One point to ten Ducats is 2,200, larger than their whole economy. The
  designer settled it at **0.8**, about 178 over a game.
- **The Arkwrights' rate needed no change**: about 15 Fuel over a game, two or three extra Moon
  transits, pointed at the thing they actually lack.

## A slider, not a switch, and the reason matters

The recommendation was a switch at half, copying the Archive. The designer refused it and gave the
reason: *"I want the amount chosen - without which the choice loses meaning and the reputation
effects don't mater"*.

That is right, and it is a dependency the map did not have written down. With a switch at half,
**any diversion at all puts a Faction below 85%**, so the shared-pot rule of ticket #236 would be
binary and meaningless. A chosen amount makes it a real trade.

It is a **percentage** rather than a count of points, at the designer's word, because Research
grows all game: a setting made in points on turn 5 means nothing by turn 25, and a share reads
directly against the 85% rule.

## Provisional Findings had to stop being binary

The Archivists' rule holds *"so long as their Research went to the shared Tech last turn"*. It was
a bool because the control was a switch. On a slider, any directive above 0 would have cost the
whole rule — so the only rational settings would be 0 and 100 and the slider would collapse back
into the switch it replaced, for the one Faction that already had one.

At the designer's word, *"make it a threshold 75%"*: the rule holds while at least that share still
went to the shared Tech. **Both old positions are unchanged** — 0 keeps it, 100 loses it.

**It reads the DECLARED directive, not what landed**, and a test found the difference: rounding
means a 26% directive on 14 Research takes 3 points, which is 21% applied, so a player who set 26
would keep a rule they had chosen to trade away with nothing on screen to explain it.

**And the one-turn lag survived a scare.** The rule reads *last* turn's Research; a first attempt
read the directive at the moment of settling, which is after the order has already been placed, so
the rule went off a turn early. An existing test — *"and the turn that funds still has it"* — caught
it. The directive in force at each Income is now recorded and the next Income settles from that.

## Looked at

| picture | what it shows |
|---|---|
| [`the-control.png`](the-control.png) | The control as ticket #235 first built it: a half-width slider carrying the **directive**, the share taken away. |
| [`the-slider.png`](the-slider.png) | The control as it ships, after ticket #251 turned it over: the **contribution** to the shared Tech, the window's full width, a fifth larger, the header reading `Research Directive: 50%`, and the half no Faction but the Archivists may reach dimmed behind the rail. |

## Ticket #251: the slider turned the other way up

The designer, looking at the first version: *"make it 20% larger ... the width of the window and
bounded between 0 and 100 for everyone but the archivist the bottom half of the slider is greyed
out and the slide itself will not extend below 50%. Put the selected contrabution in the header"*.

Only one reading makes all of that true at once: **the slider carries the contribution to the
shared Tech, not the share diverted**. "Will not extend below 50%" is a floor on what you give the
table, and the header shows the selected *contribution*. It is the better way round, too, because
it is the unit the shared-pot rule of ticket #236 is written in — a player reading `78%` can
compare it to that rule's 85 without doing the subtraction.

The scale runs **0 to 100 for every Faction** so the four controls read alike and the Archivists'
extra reach is visible rather than implied: theirs runs the whole way, everyone else's is stopped
at half with the unreachable part dimmed behind the rail.

**Two goes at the dimming, and the picture is why.** A translucent black over an already dark rail
was invisible in the first capture, and so was a grey close to the rail's own. A diagnostic pass in
bright red proved the geometry was right all along — the overlay sat exactly across the left half,
ending at the handle — so the fault was only the colour. It is `gray(30)` now, plainly darker than
the rail. Nothing but looking at it would have told me which of the two possible faults it was.

"20% larger" needed **both** `slider_width` and `slider_rail_height`: raising only the first makes
a long thin bar rather than a bigger control.

The slider sits at 50% in that capture because **the computer set it** — evidence the AI branch
works. The designer asked for that directly (*"yes they use it"*), against the memory of the
Trading window in 0.08.2, where over 80 games the seats bought 29,440 units and sold nothing.
How hard a seat should lean on it is `fund_archive` in `ai.toml`, data rather than code, so the
sweep can fit it without a rebuild.

Captured with `target/release/dying-earth.exe shot:<prefix> tech:1 panel:0 turns:10
window:1920x1080`, off-screen, exit 0.

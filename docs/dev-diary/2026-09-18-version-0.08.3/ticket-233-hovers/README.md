# Ticket #233: two explanations go to hovers

The designer's ask, from the 0.08.3 list: the Blame sentence in the Faction window *"goes to a
mouse over"*, the Relations explainer likewise, and the Relations hover should say *"what
diplomatic exchanges are available"*.

## What was read first, and what it changed

- **The Blame sentence exists three times**, in three different windows. `src/ui.rs:6031` is the
  Faction window, the one quoted. `:6323` is the **Climate Panel**, the same sentence in the plural
  about all four Factions. `:3785` is the **top bar's Influence hover**, already a hover and already
  saying it at greater length. The designer: *"faction windows leave climate as is"*. **Only the
  first moved. Nothing was deleted.**
- **The Relations hover already gave the number.** Ticket #221 built it — the total, the
  deeds/Blame split, the scar floor. So *"tell us exactly what the numeric value is"* was already
  shipped, and the genuinely new part was the exchanges.

## The decisions

**No marker.** The recommendation on the ticket was a small `?` beside each heading so players
could tell it was hoverable. The designer rejected it: *"no other mouse overs have any ? - the
convention here is the same, mouseovers are common AF in 4x games."* So the headings carry ordinary
`on_hover_text` and nothing advertises them — *"I want this new mouseover to work exactly like all
the others have been working"*.

**The pair hover names what STANDS, never what could be struck.** The Accords block sits a few rows
below and already lists every term with its own explanation, greying out the research agreement
when the pair is not Friendly. The hover answers the glance; the block is where a player acts.
Saying it twice, a scroll apart, is how a figure drifts.

**The same rule covers pairs the player is not in.** This grid appears on every Faction's page, so
it shows rival-to-rival pairs too. What stands between two rivals is visible on the board once it
bites; what they *could* strike is intelligence. That is the line the Faction window's disclosure
rule already draws.

## What it bought

| picture | what it shows |
|---|---|
| [`before.png`](before.png) | The Faction window as it was: two wrapped lines under **Blame**, five under **Relations**. The window runs to y≈783. |
| [`after.png`](after.png) | The same window with both on hovers. Blame and Relations go straight from their figures to the next block, and it ends at y≈640. |

**About 140 pixels off the page, roughly a fifth of it.** The two runs are separate games at the
same turn and Faction, so a line or two differs in the Holdings figures; the seven lines of prose
are the dominant change and are plainly the difference.

## What is not photographed, and why that is settled

The hovers themselves. `shot:` mode cannot park a pointer, so no capture can open a tooltip — the
gap that [A shot aid that opens a window and parks a pointer](https://github.com/whaleyjoshua2/Dying-Earth/issues/242)
was raised for and then closed without code, because the designer verifies tooltips at the machine
(*"I've seen the tooltips and confirm they are functioning"*) and required this one to use the same
mechanism as every existing hover. There is no novel behaviour here for a picture to check: it is
`on_hover_text` on a label, the same call made dozens of times elsewhere in this file.

What a picture *can* show is the thing the ticket was actually for — how much shorter the page
reads — and that is the pair above.

Captured with `target/release/dying-earth.exe shot:<prefix> factions:1 panel:0 turns:20
window:1920x1080`, off-screen, exit 0. The `before` shot was taken by stashing the change,
rebuilding, capturing, and restoring.

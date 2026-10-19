# Dying Earth — version 0.1.0.2, the legibility version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.1.0.2](https://github.com/whaleyjoshua2/Dying-Earth/issues/547), and the
pictures that decided it are in
[`docs/dev-diary/2026-10-19-version-0.1.0.2/`](../dev-diary/2026-10-19-version-0.1.0.2/).

**The baseline** is 0.1.0.1's closing board,
[`final-0.1.0.1.txt`](../dev-diary/2026-10-12-version-0.1.0.1/sweeps/final-0.1.0.1.txt):
**7 / 24 / 19 / 19 and 11 collapses**, against the designer's 15 each and 20.

**What the version is.** Version 0.1.0.1 with the designer's next list, nearly all of it the board
and its cards saying what they mean, plainly, where the player can read it. The list is still open.

## 1. Map labels give way

Ticket: [Map labels give way instead of overlapping, at every view angle](https://github.com/whaleyjoshua2/Dying-Earth/issues/548).

### The rule

- **Every map label is laid out again each frame**, for where it lands on screen in this view.
  Where two would collide, the one ranked lower **gives way**: first it **shrinks** to its short
  form, and if even that collides it **hides**. A label that has given way returns to its full form
  as soon as there is room for it.
- **Its short form is its kind glyph and its name**, and nothing else. A Region's Unrest line goes
  with the rest of its block; a slot's yields line goes; a Ship stack's label shrinks to its glyph
  and its count of Ships.
- **Rank**, highest first:
  1. the selected place;
  2. the place under the pointer;
  3. the player's own;
  4. rivals';
  5. neutral and nobody's.

  Between two of the same rank, the one nearer the middle of the view wins. A place's owner is
  whoever directs it: a Region's, a Colony's or a station's director, a Ship's or a stack's seat; a
  Body on the Solar System Map is the player's own when the player has a Colony or station there,
  a rival's when only rivals have, and neutral otherwise.
- **No flicker.** A label that changes form fades into the new one over a fifth of a second, and a
  label coming into view fades in. For half a second after a label gives way it grows back only
  when the larger form would clear everything by a margin of six pixels; after that it grows back
  as soon as it fits. (Found in review: without the half-second bound, labels set flush by hand --
  the Solar System Map's stack rows and the Orbital Control flag under them -- never had the margin,
  and one that hid for a frame stayed hidden.)
- **Fixed marks never give way, and labels give way to them**: Army shields, Battle marks, station
  and warship glyphs, the Orbital Control flag, the launch window line, a Body map's orbit band and
  the panels round the map and any open window. A label does not give way to its own place's marks
  (a Region's shields, a slot's Battle mark, a station's or stack's glyph), which are set beside
  it. **The selected label and the one under the pointer are laid over the marks** instead of
  giving way to them, so pointing at a place always shows its name. They still give way to each
  other. (Found in building: a neighbour's shield hid China, the player's own
  Region, even when selected; the designer chose this answer.) The panels and open windows are the
  exception: every label gives way to them, the selected included, since a label under one is not
  seen.
- **The pointer raises a label** when it is within 30 pixels of the place, a Region's click circle,
  or over the block the label is showing.
- **Two blocks may overlap by two pixels a side** and still count as clear, since a block pads its
  text by two pixels: kissing backgrounds leave both texts whole.
- **A click stays at the place**, whatever its label is doing: a Region, a slot, a station or a
  stack is chosen where it was before, whether its label is full, shrunk or hidden.

### Where it applies

The same rule on every map:
- the Earth Map's Region labels;
- every Body map's Colony Slot labels, Antarctica's included;
- the Solar System Map's Body labels, its stack labels, its far-orbit station names and its Ships
  in flight;
- the station and stack labels on a globe's orbit rings, and the *low orbit* ring name;
- the start screen's globe, whose Region names have no short form and so either stand or hide.

### What goes

The two one-off dodges, which the rule covers: the Solar System Map's step-down for a label that
hangs below its Body (ticket #345), and a slot label's step clear of the orbit band (ticket #335).

## 2. Fewer words: the Widgets hovers and the Climate Panel

Ticket: [Fewer words: the top bar's Widgets hover and the Climate Panel's deck lines](https://github.com/whaleyjoshua2/Dying-Earth/issues/549).

- **The rule of what a Widget is goes from every hover** (*"A Widget is one unit of work; ..."*).
  - The top bar's Widgets hover keeps its header, *"Widgets: 15 made a turn across the places you
    direct, 0 applied at the last Resolution."*, and its line for each place you direct, with that
    place's queue.
  - A place card's *"Widgets 11 a turn"* hover reads its figures alone: *"Widgets 11 a turn here: 4
    from the Region itself, 3 from Industry Level 3, 4 from the Factory."*
- **The Climate Panel's penalty line** reads *"Penalties in force: population growth +1.00% per
  turn."* How often a card comes and what is left in the deck are dropped. The Report's *"No Event
  this turn"* line still gives the chance.

## 3. Hovers on a row of words answer the pointer

Ticket: [The Launch Site line's hover answers the pointer](https://github.com/whaleyjoshua2/Dying-Earth/issues/550).

- **No text in the game is selectable.** A hover hung on a row of words, such as *"Launch Site:
  Yes"* or a glyph and its words (the Influence value, GDP, Widgets, Emissions, a card's heading),
  opens with the pointer anywhere on the row's words. Until now it opened only in the few pixels
  between them, because egui's selectable text took the pointer from the row.
- **The area is the row's own**, not stretched across the card.

## 4. Plurals print right

Ticket: [Plurals print right: "1 Facility", "1 slot"](https://github.com/whaleyjoshua2/Dying-Earth/issues/551).

- **Every count a player reads takes its noun singular at one**: "1 Facility", "1 free slot",
  "1 turn", "1 Colonist", "1 Pioneer", "1 Ship", "1 hit". It applies in the window, the Report,
  the Moments, the refusals an order gives, and the text tool. No "(s)" is left in any of them.
  One helper, `dying_earth_engine::counted`, prints the count and its noun.
- **A verb that must agree** agrees: "1 Colonist waits aboard", "its one turn is up", "1 Colonist
  ... was lost", "1 Scrubber ... runs".
- **Not changed**: mass nouns and resources (Fuel, Energy, Influence, ppm), Ducat counts, lines
  that go only to the game's log (which the sweep tool counts by their words), and the developer
  sweep tools.

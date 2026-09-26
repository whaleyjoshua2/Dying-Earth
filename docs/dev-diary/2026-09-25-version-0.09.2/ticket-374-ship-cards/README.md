# Ticket #374: a card for every Ship, with Bodies and orbits in drop-downs

The designer: *"clean up ships at body cards - ships should each have their own card. bodies and
orbits about which should each be their own drop down both on their own cards and the one listing
all the ships at a body."* Decided in one round of six: everything that is one Ship's goes on its
card; the stack card keeps only what is the whole stack's; drop-downs as recommended; the Ship's
card in place of the stack's with a link back; a Ship in flight has a card too; the picture aids
scroll the Ship's card.

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/374) is the authority;
[§11 of the spec](../../../spec/version-0.09.2.md#11-a-card-for-every-ship-with-bodies-and-orbits-in-drop-downs)
records it.

## What was built

A new selection, `Selection::Ship`, and a new card, `ship_panel`, in `src/ui.rs`. The stack card
(`stack_panel`) was re-laid around it: its Ships stand grouped by orbit in one drop-down per orbit,
open by default, each row a link that opens the Ship's card; under them the stance, Attack, and the
whole-stack moves. The moves are one helper for both cards, `move_dropdowns`: one drop-down per
Body, the Body the Ships are at first and open (its orbits are the change-of-orbit moves), every
other Body shut with its turns and Fuel on the header and the dated quote of #375 inside. With one
Ship the lines carry that Ship's own button and the stranding warning; with more, "All N that can".

After the first pictures the designer asked for two more things, built in the same session: the
drop-downs **nest as the sky is** (*"put the martian moons under mars with their orbits nested ...
do that with earth and the moon too"*), so a planet's drop-down holds its own orbits and then its
moons' drop-downs inside it, through `BodyId::primary` and `BodyId::moons` in the engine, pinned by
`every_body_is_a_planet_or_listed_once_under_its_primary`; and the orbit lines inside a drop-down
**drop the Body's name** (*"we know its a martian orbit because its in the mars drop down"*): *"To
low orbit"*, *"To Ares"*, *"Low orbit"*, *"Ares"*.

The Roster's Ship rows open the Ship's card, in flight or not; the flying label on the Solar System
Map is a hotspot that opens it; an Army aboard a Ship jumps to the Ship's card. The two right-click
doors (a Body on the Solar System Map, an orbit on a Body Surface Map) read the selection through
one helper, `selected_ships`, so they send the one Ship whose card is open as they send the stack.

Two building aids: `ship:1` / `ship:<n>` / `ship:flying` opens a Ship's card in place of the
stack's, and `flying:1` plants a Colony Ship in flight from Earth to Mars, since nothing else in
shot mode leaves a Ship between Bodies for a picture.

No save change, no rule change; the one engine change is the Body tree, and the suites are 497 + 6
and 8.

## The pictures

All taken headlessly at 1400x1000 with `panel:0`, from this folder.

| picture | aids | what it shows |
|---|---|---|
| [`stack-mars.png`](stack-mars.png) | `orbits:1 stack:1` | **The stack card.** Two drop-downs, *In low orbit (1)* and *At Mars Base Camp (1)*, open, each holding its Ship as a link. Then the stance, the odds, Attack, and *Moves for the whole stack*: *Mars (here)* open with four "All N that can" lines (*To low orbit*, *To Mars Base Camp*, ...) and *To Phobos* and *To Deimos* nested shut inside it; then *To Earth* and *To Venus* shut, the Moon inside Earth's. No button per Ship anywhere on it. |
| [`ship-mars.png`](ship-mars.png) | `orbits:1 stack:1 ship:1` | **The Frigate's card.** *Back to all Custodians Ships at Mars*, its line, *In low orbit, on Hold*, its Tank with the Refuel button, and Transit: *Mars (here)* open with a Move button per other orbit and the two moons nested, the rest shut. |
| [`settler-mars.png`](settler-mars.png) | `orbits:1 stack:1 ship:2` | **The Colony Ship's card**, at the station's ring: the same shape, with *Load and unload* under Transit offering the station's two Colonists. |
| [`moon-earth.png`](moon-earth.png) | `settler:earth stack:earth` | **The stack card at Earth**, two Colony Ships in low orbit: *Earth (here)* open with a line per station and *To the Moon: 1 turn(s), 6 Fuel* nested inside it; *To Mars* and *To Venus* shut. |
| [`flying-solar.png`](flying-solar.png) | `flying:1 ship:flying` | **A Ship in flight.** *Back to the roster*, *carrying 6 Colonists*, *In transit from Earth to Mars, 3 turn(s) left: it lands at the Resolution of May 2030, barring a solar storm*, and its Tank reading *it lands with what is left*. No moves. |

The first picture of the Frigate's card drew a hollow box where the back link's arrow should be:
the interface font has no U+2190, the same want ticket #371's arrow found. The link is in words.

## The review

Two axes, run as sub-agents over the first commit. **Standards** found the Ship's line built twice
(now one `ship_line`), a third way of naming an orbit beside the two the file documents (folded
into `capitalised(orbit_phrase)`), a hand capitalisation in the credits that the new helper
replaces, the picture-scroll answered on the stack card as well as the Ship's (the stack passes
`None` now), a missing comment at the `ship:` aid's parse, and a Tank line with no stop between
the figure and its sentence (a semicolon). **Spec** found the one real miss: a stack of one Ship
inferred "this is a Ship's card" from the slice's length and drew that hull's own buttons on the
stack card, against *"every per-Ship button leaves it"*; the hull whose card it is is passed
explicitly now, and a stack of one reads "All 1 that can". It also found the flight line saying
*"barring a solar storm"* where a grounding holds a hull too (both named now), the orbit notice
saying *"each"* for one Ship (*"it"* now), the back-link's words differing from the spec's quote
(the spec quotes the words used), and Influence on Colonies left on the stack card where the spec's
list did not name it (kept, since it is the Body's and not any Ship's; the spec names it). One
reading for the designer: a stranded hull inside a stack of several is now read on its own card or
its Roster row, not on the stack card, since the Tank is the Ship's.

The nesting, built after those two reviews, had a review of its own. It found the planet-moon
pairing written in three places with nothing tying them: `bodies.toml`'s `parent` column (which
prices the legs), `Tables::planet`'s own match (which reads the sky), and the new `BodyId::primary`
and `moons`. Now `primary` is the one place: `Tables::planet` reads it, `moons` is derived from it,
and the loader refuses a table whose `parent` disagrees. Witnessed red by setting the Moon's parent
to Mars in the data:

    tables load: DataError { file: "bodies.toml", message: "the Moon has parent Some(Mars); the engine's tree says Some(Earth)" }

and green on the restored data. It also had a transit's quote read twice per drop-down; once now.

## The gate

`cargo clippy --workspace --release --all-targets -- -D warnings` clean; engine 497 + 6, root 8.

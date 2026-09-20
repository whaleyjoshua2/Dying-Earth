# Agitate

Ticket [#269](https://github.com/whaleyjoshua2/Dying-Earth/issues/269) on
[map #254](https://github.com/whaleyjoshua2/Dying-Earth/issues/254).

| picture | what it shows |
|---|---|
| [`eu-card-agitate.png`](eu-card-agitate.png) | `shot: select:europe panel:0`. The European Union's card, held by the Prospectors, on turn 1: under the Unrest line the new button, **Agitate: Unrest +1, 5 Influence, 15 Ducats**, where a holder would see Relief on their own card. |
| [`agitate-button.png`](agitate-button.png) | The button, magnified. |

## What was decided, in the designer's words

- *"15 ducats at 5 influence"* -- **both**: 15 Ducats and 5 Influence, in `unrest.toml`. Dearer
  than Relief's 10, so a duel is not a coin flip settled by income.
- *"yup"* -- **one Agitate a turn per Region per Faction.** A Region cannot be bought off its holder
  in one turn for a hundred Ducats.
- *"yes"* -- an offence, at an Influence push's weight.
- *"yes"* -- the holder's Report names who paid: *The Prospectors agitated in Nigeria: Unrest rose
  by 1 to 4.5.*
- *"yeah good idea"* -- a working Constabulary halves it, as it halves a climate card's rise.
- *"Ai uses it"* -- a seat agitates in a Region held by a rival it is Cold or Hostile toward, most
  eagerly past the second threshold, competing with Relief, Influence and the rest for the same
  Ducats and Allotment.

## A consequence to know

**Unrest falls 1.5 a turn on its own, after every rise** (ticket #53: "the fall lands every turn,
whatever else happened, so a rise and the fall net out"). So one Faction's single Agitate a turn on
a *calm* Region nets out to nothing but the Relations it costs. Agitate moves a Region that is
already restive -- the climate's work, a Strip Permit's, a mothball's -- or one that two or three
rivals lean on together. The tests read it that way: from Unrest 3, an Agitate leaves 2.5; through a
Constabulary, 1.0. Whether the natural fall should spare an agitated Region that turn is a question
for the designer, recorded on the ticket.

## Measured, after

One logged game (`sim 1 --log`): the computer seats proposed an Agitate **268 times** and
**12 landed** -- eight in China, two in the European Union, two in Saudi Arabia -- the rest held
back because the seat was saving its Ducats for something it ranked higher. It is alive in the
sweep, and it fires where the ledger says it should: against holders the seat resents.

## Witnessed red

The order, the price, the source and the cards were put in place with nothing resolving the order
and nothing proposing it, and both tests run: *"Unrest rose by one: 0"* and no Agitate among a
computer seat's orders. Then the Resolution step and the appetite went in; `331 passed`, clippy
clean with `-D warnings`. The test fixture had to be read against the natural fall, and to stand
its Region past the second threshold before the appetite outranked the seat's Influence pushes.

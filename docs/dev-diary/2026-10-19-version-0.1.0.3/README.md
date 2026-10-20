# Version 0.1.0.3, the fair-fight version

The map is [Map: version 0.1.0.3](https://github.com/whaleyjoshua2/Dying-Earth/issues/558); the spec
is [`docs/spec/version-0.1.0.3.md`](../../spec/version-0.1.0.3.md). The standing baseline is 0.1.0.2's
closing board, **3 / 23 / 26 / 13 and 14 collapses**
([`final-0.1.0.2.txt`](../2026-10-19-version-0.1.0.2/sweeps/final-0.1.0.2.txt)), against the
designer's 15 each and 20. The list came from the
[0.1.0.2 playtest](../2026-10-19-playtest-0.1.0.2/README.md).

## One Bank to a Region

Ticket [#560](https://github.com/whaleyjoshua2/Dying-Earth/issues/560), spec §1.

Built by the Launch Site's door in `check_order`, the Investment Bank counted as one by its job.
A test was written first and watched red (a second Bank accepted), then green; one older test, which
ordered an Investment Bank into a Region with a Bank already queued, clears the queue first.

**The picture** ([`ticket-560/one-bank-refusal-earth.png`](ticket-560/)): China's card with a Bank
ordered (`order:bank`) and a free slot's build list open: the Bank line greyed, *"this Nation State
already has a Bank"*.

**The sweep** ([`sweeps/ticket-560.txt`](sweeps/ticket-560.txt)): **10 / 29 / 16 / 12 and 13
collapses**, from 3 / 23 / 26 / 13 and 14. Three moves past noise, and not the ones a Bank cap
promises: the Custodians +7, the Prospectors +6, the Arkwrights -10. Every gate completed sooner
(the Arkwrights' in 79 of 80 games at median turn 29, from 72 at 31). A reading, not a measured
cause: the cap takes Ducats from every seat alike (the computer builds Banks as a producer for all
four), and the Arkwrights, the poorest in Ducats, lost the most; the Prospectors' Fund is paid by
interest, one Investment Bank a Region already, so the cap cost them nothing there. Measured
behaviour, not a rule; the next tickets (Influence at 5, the Fund's weight) move the same seats.

## A Scrubber at 4 Energy and 1 Ducat, one per 300 million

Ticket [#561](https://github.com/whaleyjoshua2/Dying-Earth/issues/561), spec §2.

The designer's "one per 3 pop" read as one per 300 million (the game writes emissions "per hundred
million"), floor 1, at the designer's word. A general `ducats_upkeep` field on the Facility card,
charged at the Income beside the Energy; a test written first and watched red with the charge
stubbed out ("two Scrubbers cost 2 Ducats a turn: 9.6 to 9.6"), then green. The cap test re-pinned
to the new figures (Russia 1, India 6, the ceiling at 3,300 million).

**The picture** ([`ticket-561/china-three-of-five-earth.png`](ticket-561/)), seed 3 after eight
turns: China's card reads "Scrubber (3 of 5)" (1,461 million / 300 = 4.9), with three standing,
all mothballed by the computer Custodians for want of Energy. Measured behaviour, not a rule.

**The sweep** ([`sweeps/ticket-561.txt`](sweeps/ticket-561.txt)): **9 / 24 / 20 / 11 and 15
collapses**, from 10 / 29 / 16 / 12 and 13. The Prospectors -5 is past noise, the Arkwrights +4 at
its edge, the Custodians unmoved at 9. The computer Custodians built about as many Scrubbers as
before (493 against 464 in the first seating): the Ducat gate (`income_last_turn.ducats >= 1`) is
nearly always open, and 4 Energy against 3 shut none the Energy check had not already. Measured
behaviour, not a rule: the dearer Scrubber bites a human spamming nineteen of them more than a
computer building a handful.

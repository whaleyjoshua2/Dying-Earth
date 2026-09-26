# Ticket #376: the Refugee Convoy is worth taking

From the playtest: *"The Refugee Convoy trades +0.4 population against +2.0 ppm, which nobody would
take twice."* Decided on the ticket in one round: +1.0 population for +0.5 ppm; the refusal and the
computer's rule unchanged.

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/376) is the authority;
[§10 of the spec](../../../spec/version-0.09.2.md#10-the-refugee-convoy-is-worth-taking) records it.

## What was built

Two figures in `events.toml`, on the card's take side: `population = 1.0` and `ppm = 0.5`. Nothing
in the engine; the computer's rule reads the card's own figures.

## The red witness

`the_refugee_convoy_gives_a_million_people_for_half_a_ppm` was written first and run against the
old figures:

    assertion `left == right` failed: a million people, one unit
      left: Some(0.4)
     right: Some(1.0)

Then the two figures moved, and it is green. It also pins the refusal (−5 Standing everywhere) and
the computer's rule (Unrest under 4) as unchanged.

## The sweep

A rule figure moved, so the sweep was run: [`../sweeps/after-376.txt`](../sweeps/after-376.txt), 20
seeds x four seatings at the shipped cell, against the sweep after #375:

| | after #375 | after #376 |
|---|---|---|
| Custodians | 7 | **6** |
| Prospectors | 24 | **25** |
| Arkwrights | 1 | **1** |
| Archivists | 4 | **4** |
| collapses of 80 | 44 | **44** |

One win moved between the two Factions the card touches least; collapses did not move. The computer
takes the card by the same rule as before, so its take rate is the same and only the effect changed:
a million people where 400,000 landed, half a ppm where two did. The sweep does not count the
Convoy alone; over the four batches the computer seats took 1,400 card offers of all kinds and
refused 564.

## The gate

`cargo clippy --workspace --release --all-targets -- -D warnings` clean; engine 496 + 6, root 8.

# Ticket #381: what happens when a Ship attacks, and the game shows it

The designer: *"what happens when a ship attacks a station or another ship and can we show it."*
The first half was read out of the tree on the ticket; the second was decided in one round of
five: a blow-by-blow log **and** a picture of it; the record lives one turn, as now; the chronicle
keeps every Battle that cost a hull or a Battery; ground Battles keep the same log; every Battle in
orbit is a Moment.

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/381) is the authority;
[§14 of the spec](../../../spec/version-0.09.2.md#14-what-happens-when-a-ship-attacks-and-the-game-shows-it)
records it.

## What was built

- **The engine keeps a round log** (`combat::BattleLog`): the line as it opened, then for the
  opening and each round fought, every hit (which party, on which unit, and whether the escort
  took it for an unarmed hull still engaged behind it), every unit that disengaged and every
  pursuit hit, and where every unit stood when the round was over. `Combatant` carries its kind
  (`BattleUnit`) so the picture can be drawn after the destroyed are gone from the board. The log
  rides on `BattleLine` for the turn the line lives; a bombardment and a Launch carry none.
- **The Battle Report draws and replays it** (`battle_log_view`): a block per round, each party's
  line of units in its colour, a glyph and a pip per hit point (filled for damage carried, lit for
  hits taken that round), a unit that left dimmed, one destroyed crossed; then the round's blows
  in words, the covered hulls named.
- **Every Battle in orbit is a Moment**: `MomentKind::OrbitalBattle`, fired for a bloodless one;
  a fatal one keeps `DecisiveBattle` and fires one, not two. `moment:orbit` is the picture aid.
- **The chronicle has a section, The Battles**: one line per Battle that cost a hull or a Battery,
  from `WarCounters::fallen`, with the date, the place, the losses and who attacked.

## The red witness

`every_orbital_battle_is_a_moment_bloodless_or_not` was written first and run on the old engine:

    seed 7: one Moment for the Battle, 0 lost: []
      left: 0
     right: 1

Then the Moment kind was added and it is green. `the_round_log_accounts_for_every_hit_and_ends_
where_the_melee_ended` pins the log to the totals over forty seeds: every hit is a blow, the last
round's states are where the melee left the units, and a covering hit lands on an armed hull of the
party that has an unarmed one engaged.

No rule change, no sweep; the save gains two fields with defaults, so an old save loads.

## The picture

Headless, 1400x1000, `panel:0`, from this folder.

| picture | aids | what it shows |
|---|---|---|
| [`report-report.png`](report-report.png) | `battle:1 cardshut:1 menus:1` | **The Battle Report** of a four-party Battle in Mars orbit that cost PMV Indomitable and ARK Implacable: the party lines as before, then *Round 1* -- four lines of hulls with their pips, the hits of the round lit -- and the blows in words: *"The Arkwrights hit TSV Valiant"*, *"The Custodians hit ARK Implacable, covering ARK Defiant"*, *"The Custodians hit PMV Indomitable, covering PMV Magellan"*; then *Round 2* below the fold. |

## The gate

`cargo clippy --workspace --release --all-targets -- -D warnings` clean; engine 501 + 6, root 8.

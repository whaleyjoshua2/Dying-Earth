# Ticket #383: an Attack fought the moment it is ordered, and its Battle in a window of its own

The designer, after the Battle record shipped: *"can we take the battle report off the start of
turn report and put it in its own window when the battle resolves. change that (if its not) to
resolve prior to the end of the turn."* Decided in two rounds: every Battle gets a window of its own
and the orbital Moment goes; **an Attack is fought the moment it is ordered**, Ships only, stations
included; the survivors stand on Hold and the stack fights once a turn; the computer's Attacks are
fought as the Resolution opens; *Confirm Attack* is the point of no return; the record is written as
it is fought.

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/383) is the authority;
[§15 of the spec](../../../spec/version-0.09.2.md#15-an-attack-is-fought-the-moment-it-is-ordered-and-its-battle-has-a-window-of-its-own)
records it.

## What was built

- **The engine**: `resolve_battles`' orbit fight is `fight_orbit` now, called by three things: the
  Resolution for a Battery's Battle on a blockader (the one Battle it still opens itself);
  `Game::attack_now(seat, body)` for the player's Attack, which checks it as the order it was, sets
  the stance, fights every orbit the stack holds Ships on Attack in, stands the survivors on Hold
  and marks the stack fought; and `fight_attacks`, which the Resolution runs first of all -- before
  the transits and orbit changes land -- for every stack still on Attack, the computer's among them.
  `Game::fought` holds the stacks that fought this turn; `check_order` refuses a second Attack for
  them; the Resolution clears it at its end. It rides in the save for a save taken after a fight.
- **The Moment for a bloodless orbital Battle goes** (`MomentKind::OrbitalBattle`, its table and
  its aid), the window being the news.
- **The interface**: `Popup::Battle(i)` shows the turn's nth Battle -- the party lines, the round
  picture and the replay -- with *Next Battle* / *Close* handing on through `advance_popup` to the
  next, then to the turn's head (the note, the card or the Event, the Moments, the Report), or to
  nothing after a Battle fought mid-turn. The head chain opens with the Battles; the map's mark
  opens its Battle's window; the Report loses its Battle Report block. *Confirm Attack: fought now*
  and the stance row's Attack go through `Action::Attack`, which fights and raises the window.
- A building aid, `battlewindow:1`, raises the first Battle's window for a picture.

## The red witness

`an_attack_is_fought_before_the_transits_land` was written first: a rival's stack on Attack at
Mars, the player's Frigate docked there and a second Frigate one turn out. On the old rule:

    the hull in flight never joined: TSV Valiant escaped after 2 hits; TSV Vanguard escaped after 2 hits

-- the arriving Vanguard landed first and fought. On the new rule it lands after the Battle and
the line names Valiant alone. `an_attack_is_fought_at_once_and_the_stack_fights_once_a_turn` pins
the player's path: one Battle in the record at once, the survivor on Hold, a second Attack refused
*"fought this turn"*, a transit still open, the flag cleared by the Resolution. One older test had
the old order built into its name -- an orbit change that *lands before the Battles* -- and now
says the opposite, which is the rule.

## A consequence the picture shows

Two stacks that Attack in the same orbit in one turn fight **two Battles**, one per Attack, each
against everyone present: the Report in the diary's first picture headlines both, where the old
Resolution folded every aggressor into one melee. Every Attack is its own fight now.

## The sweep

A rule change, so the sweep was run: [`../sweeps/after-383.txt`](../sweeps/after-383.txt), 20
seeds x four seatings at the shipped cell, against the sweep after #377:

| | after #377 | after #383 |
|---|---|---|
| Custodians | 2 | **2** |
| Prospectors | 5 | **5** |
| Arkwrights | 0 | **0** |
| Archivists | 11 | **11** |
| collapses of 80 | 62 | **62** |
| orbital Battles opened | 39 | **48** |

The win column and the collapses did not move; the computer's Attacks fight before the transits
land where they fought after, and the difference did not reach a game's end in eighty. Orbital
Battles opened rose from 39 to 48, since every Attack is its own Battle.

## The picture

Headless, 1400x1000, `panel:0`, from this folder.

| picture | aids | what it shows |
|---|---|---|
| [`window-mars.png`](window-mars.png) | `battle:1 cardshut:1 battlewindow:1` | **The Battle window**: *Battle at Mars orbit*, dated, the four party lines, the result, the three rounds drawn and replayed, and *Next Battle* at the foot, since the turn fought two. |

## The review

Two axes, run as sub-agents over the commit, and both found the same hole: the player's fought
Battle was written into the Report the player was reading, which End Turn resets, so the next
turn's headline, Report and map mark never had it and only the window at the moment of the fight
ever showed it. End Turn carries such a Battle into the coming Report with its headline written
again, pinned by a test. Both also found an Attack with nobody to fight standing the stack down,
counting as its fight and raising a window on nothing; the order is refused now, *"no rival Ship or
Battery in any orbit your Ships hold there"*. **Spec** found the stance row's Attack a second point
of no return that said nothing (it opens the confirm now, and its hover says what confirming does
in place of the persistence line), and the Comms Blackout Event no longer reaching the computer's
Attacks (it stands them down first again). **Standards** found the Save button live after a fought
Attack against the rule that a save is a turn start (dead now, and says why), the stance-setting
loop written three times (one helper), a dead Attack counter (gone), a doc comment left heading
the wrong function, the fought-flag doc saying the wrong end of the Resolution, the Battle modal's
id unlike the others', and the chain's doc without its Battle paragraph.

## The gate

`cargo clippy --workspace --release --all-targets -- -D warnings` clean; engine 503 + 6, root 8.

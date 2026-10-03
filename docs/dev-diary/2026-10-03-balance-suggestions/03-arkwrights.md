# The Arkwrights' view: balance suggestions after 0.09.5

## Why the Arkwrights score nought

**Rules**

- **Diaspora asks for two things:** 30 Colonists off Earth **and** 3 Bodies with at least 4
  Colonists on each (`assets/data/factions.toml:170-171`).
  - A station over Earth counts toward the 30 but is never a Body (`engine/src/state.rs:3187-3192`).
  - The score is the lower of the two parts (`engine/src/victory.rs:61`). With no Body settled it is
    0, however many people are in orbit.
- **The computer never sees the Bodies part.** It judges how far behind it is from the 30 and from
  Off-world Presence, and **neither counts Bodies** (`engine/src/ai.rs:457-472`,
  `assets/data/ai.toml:378-380`).
- **Lifting to the station is weighted in full.** Pioneers lift from a Launch Site straight onto the
  seat's own station over Earth, at full weight plus the victory-gap boost (`ai.rs:2426-2443`).
  Ticket #94 (0.06.0) had cut parking loads over Earth to half weight because "Mars went unfounded"
  (`ai.rs:2625-2634`). The direct lift added in 0.07.3 brought the full weight back.
- **Factory Modules have no cap.** One is wanted at any Colony with a Shipyard, at the gap and
  opportunity boosts, **however many already stand** (`ai.rs:1493-1499`). Each costs 20 Materials
  and 3 Energy (`assets/data/modules.toml:318-324`).

**Measured behaviour.** A throwaway headless probe replays the shipped cell. It reproduces the sweep
exactly: 10 / 10 / 1 / 17, 42 collapses, the Arkwrights at nought in 58. In those 80 games:

- In 52 games the station over Earth was the only place off Earth they ever held.
- In 45 games they had 30 or more Colonists off Earth, so the first part was met on its figure. In 24
  of those they had 0 Bodies.
- They had a Colony Ship in **0 of 80** games at turns 12 and 18, and in only 18 at turn 24.
- By turn 18, 49 games had two or more Factory Modules on that one station; by turn 24, 73 games did.
- The computer's own log (seed 12) shows a Factory scoring 22-36 against a Colony Ship's 9-27.
  Materials were then held back for the next Factory. Some stations ran out of Energy and their
  Shipyard went dark.
- Their home Region is usually the Arabian Peninsula, at a population of about 99 that falls to
  about 30. Coach Class charges two people per Pioneer.

In short: the computer fills one station over Earth, spends its Materials on Factory Modules, and
never launches. **This is mostly the computer's play, not the rules.**

## Experiments (two sweeps, the shipped cell, against the 0.09.5 baseline)

| | Cust | Pros | Ark | Arch | Collapses | Ark at nought |
|---|---|---|---|---|---|---|
| 0.09.5 baseline | 10 | 10 | 1 | 17 | 42 | 58 |
| Run 1: one Factory per yard | 13 | 11 | **9** | 15 | 32 | 29 |
| Run 2: Run 1 + a computer that counts Bodies | 21 | 5 | **17** | 17 | 19 | 10 |

Outputs: [`sweeps/arkwrights-one-factory.txt`](sweeps/arkwrights-one-factory.txt) (Run 1),
[`sweeps/arkwrights-one-factory-and-bodies.txt`](sweeps/arkwrights-one-factory-and-bodies.txt)
(Run 2). Both read the Arkwrights' median score as 0.33; at nought in 29 and in 10.

- **Run 1 beat my expectation.** I expected about 5 wins. Its four-way spread, 13 / 11 / 9 / 15, is
  the tightest I have seen in these files.
- **Run 2 went past my aim:** it takes wins from the Prospectors.
- **A probe (not a sweep):** the Bodies-aware judgement **without** the lift change makes things
  worse, 3 wins and 56 at nought. The boost goes into more lifts and Habitats over Earth.
- **Both runs cut collapses** (42 → 32 → 19). The designer counts collapses as good news, so this is
  a real cost. I did not trace the cause.

## Suggestions, best first

### 1. A Shipyard wants one Factory, not one every turn

- **Change:** `engine/src/ai.rs:1494`. Stop offering a Factory Module "because a Ship is wanted" once
  the Colony has a Factory standing or on order. The rule for a busy build queue stays.
- **Why:** this is what sinks the Arkwrights' Materials and Energy (above).
- **Effect (measured):** Arkwrights 1 → 9 wins, nought 58 → 29, everyone else within ±3.
- **Risk:** it changes all four computer seats. Collapses fall to 32.
- **Size:** small; computer seats only, no rule figure moves.
- **Recommendation:** take this one.

### 2. Teach the computer that Diaspora counts Bodies

- **Change:** three computer-seat changes, together:
  - (a) judge "behind" on Bodies settled, paced like the first part (`ai.rs:457`);
  - (b) while Bodies are short, a lift onto the station over Earth is a foothold: half weight, no
    boost (`ai.rs:2441`). That is ticket #94's own rule, applied again;
  - (c) a Habitat is not progress on a Bodies part (`ai.rs:1148`).
- **Effect (measured, with #1):** Arkwrights 17, Prospectors 5, Custodians 21, collapses 19.
- **Risk:** it overshoots and costs the Prospectors heavily; they would need their own look.
- **Size:** medium, computer seats only.
- **Recommendation:** only after #1 has settled. Then consider a softer version, for example (b)
  without (a).

### 3. Start the Arkwrights' computer seat in a populous Region

- **Change:** a computer seat takes the highest-Industry free Region today (CONTEXT.md, "Start
  Region"). For the Arkwrights, take the most populous free Region instead.
- **Why:** Coach Class charges two people per Pioneer, and a home of about 99 runs dry. The "richest
  Region" rule tried in 0.09.2 measured something different.
- **Effect:** not measured. It should help the end-of-game ranking more than outright wins.
- **Risk:** a populous home is usually large in Influence terms and harder to hold.
- **Size:** medium. It changes the start placement rule, so `state.rs` needs work, not `ai.toml`.

### 4. Ask for 2 Bodies, not 3

- **Change:** `factions.toml:171`, `bodies = 3` → `2`.
- **Why:**
  - On the shipped computer play only 4 of 80 games reached 2 Bodies, so alone this does little.
  - With one Body, the score would rise from 0.33 to 0.50, which matters in the turn-36 ranking.
- **Risk:** together with #1 and #2 it could become a runaway. In Run 2's probe, 32 of 80 games
  already reached 2 or more Bodies.
- **Size:** small. The computer reads the card, so nothing else changes; `pace.arkwrights` can stay.
- **Recommendation:** measure only after #1.

### 5. Stop counting the station over Earth toward the 30

- **Change:** the station over Earth no longer counts toward Diaspora's 30 (the card text, plus
  `victory.rs:155`).
- **Why:** "living off Earth" would then mean what Diaspora says.
- **Risk:** on today's computer play it only lowers the Arkwrights' score. Worth considering only
  after #2.
- **Computer seats:** the pace in `ai.toml` must read the same count.
- **Size:** small (a rule), plus the matching computer-seat change.

### 6. Leave contestability alone for now

After Run 2, the other seats took Arkwright places off Earth in 21 of 80 games (probe). The board
already pushes back, so no change is proposed here.

## Notes

- **"Probe" figures** come from a throwaway program that reproduces the shipped sweep exactly. It has
  been deleted.
- **The experimental code was not kept.** The worktree was put back to `9fa4eee` with no changes.
  Rebuilding Run 1 or Run 2 means writing the change described in #1 and #2 again.

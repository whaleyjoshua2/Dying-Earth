# Ticket #238: three turns before you may remake a country

> Strip Permit, leap frog, and exodus call all require the nation to have been held at least three
> turns.

Three orders in the game change a Region permanently — the Prospectors' **Strip Permit**, the
Custodians' **Leapfrog**, and this version's **Exodus Call** — and nothing stopped a Faction walking
in and using one the same turn. Now a Faction must have held the Region for three whole turns. The
turn of the taking does not count, so a Region taken on turn 10 opens on turn 13, and a Faction's
starting Region opens on turn 4.

## The question the designer asked, and the answer that reversed a recommendation

They asked: *"is the block to strip permits a function of them losing control of their states too
soon or because the AI tries to build them right away"*.

It was the second, and the question caught a bad figure. The first census said the rule **would have
refused 31 of 31 Strip Permits and 2 of 36 Leapfrogs** as those orders were actually issued, and
that was offered as evidence the rule was too harsh on the Prospectors. It was the wrong reading:
every Strip Permit goes on a Region held nought or one turns, while the same seats hold **nine
Regions of nine** for three turns or more by mid-game. They are not losing their grip — the AI
simply strips each new Region the turn it arrives, a fresh one being the obvious unstripped
candidate. The recommendation was withdrawn and reversed, and the designer took the rule:
*"go with three turns and your recommendations"*.

## What it actually costs, which is more than the projection said

The projection understated it, and the direct A/B says so. Flipping `min_turns_held` between 0 and 3
over the same twenty seeds and six parameter rows, no other change and no rebuild:

| collapse pace (fastest row → slowest) | 1 | 2 | 3 | 4 | 5 | 6 | total |
|---|---|---|---|---|---|---|---|
| Leapfrogs with the rule off | 10 | 31 | 57 | 78 | 94 | 108 | **378** |
| Leapfrogs with the rule on | 2 | 13 | 27 | 34 | 46 | 55 | **177** |
| share kept | 20% | 42% | 47% | 44% | 49% | 51% | **47%** |

**A little under half the Leapfrogs never happen.** A refused order does not politely wait its
turn: the Ducats go somewhere else that turn and the board diverges from there, so the cost
compounds past the orders the clock directly blocked.

The last row is the consolation and it is the rule working as intended. **The share kept climbs with
the length of the game** — 20% on the fastest-collapsing row, 51% on the slowest — which is the
delay showing through wherever there are turns enough to spend. In a game that ends at turn 12 a
three-turn wait is a quarter of the Custodians' career; in one that runs to 18 it is a tax.

This is a real balance cost on the Custodians, who are also the runaway seat at 37 wins of 80, so it
may be pointed in the right direction. It was not chosen for that reason, and it is written here so
the closing sweep is read with it in mind.

## The clock restarts on the holder, not on the write

Control is written back to the same Faction more often than anyone would guess, and a clock that
reset on every write would have denied all three orders forever to a Faction nobody could see being
interrupted. So `restart_neutrality_clock` — which already ran after every write to a Region's
control — resets the hold clock **only when the controller actually changes**:

```rust
let who = st.control.controller();
if who != st.held_by {
    st.held_by = who;
    st.held_since = who.map(|_| turn);
}
```

**A Region with no clock recorded passes.** That is every Region in a save written before this
version, and the alternative — reading a missing clock as "just arrived" — would silently disable
three Faction orders in every old save. A save that is briefly generous is the smaller surprise.

## Looked at

| picture | what it shows |
|---|---|
| [`refused-then-allowed.png`](refused-then-allowed.png) | The same Strip Permit button one turn apart. **Turn 3**: flat and dim, with the refusal open beneath it — *"China has been yours for less than 3 turns; you may act there from turn 4"*. **Turn 4**: the button has its frame and its lighter label back, on exactly the turn the refusal named. |
| [`the-refusal.png`](the-refusal.png) | The whole China card at turn 3, for the context the crop leaves out. |

The refusal **names the turn it opens** rather than saying no. A player who is told "not yet" and
not told when has to count turns by hand against a rule they have to remember.

### A refusal had never been photographed before, and the reason was a wrong diagnosis

Every refusal in the game hung off `on_disabled_hover_text`, which needs a pointer, and the shot
window never has one. So refusals were the one class of tooltip no picture could reach. The fix was
three words — route them through `rule_tip`, which the `tip:<word>` aid already hooks — and it is
what made the picture above possible.

Two earlier attempts at that picture produced a greyed button and **no tooltip**, and the change was
nearly reverted as unverified. The fault was neither the aid nor the routing:

- **`check_order_inner` tests affordability first.** The probe used the Custodians' Leapfrog, which
  costs 50 Ducats they did not have on turn 2, so the refusal the engine produced said *"needs 50
  Ducats"* and never mentioned the clock. The Strip Permit is **free**, so nothing can mask its
  refusal — that is why the picture above uses it.
- **`turns:N` lands on turn N+1.** The run believed to be turn 3 was turn 4, where the clock has
  already opened, so there was correctly nothing to show.

Both are worth knowing for the next capture: a greyed button alone proves nothing, because any of
half a dozen reasons could be greying it, and only the refusal text says which.

Captured with `target/release/dying-earth.exe shot:<prefix> player:prospectors start:eastasia
select:eastasia turns:2 panel:0 window:1920x1080 "tip:has been yours for less than"`, off-screen,
exit 0. Nothing was opened on the designer's desktop.

## Tests

Two existing tests had to be **aged** rather than fixed — they issued their order on turn 1 and were
right to before this version — through a helper that says so in one line:

```rust
fn held_long_enough(g: &mut Game, s: StateId) {
    g.turn = g.turn.max(g.tables.faction_orders.min_turns_held + 1);
    g.state_mut(s).held_since = Some(0);
}
```

Two new tests guard the rule itself: one that all three orders are refused below the line and pass
on it, one that the clock survives a write back to the same holder.

Witnessed red by setting `min_turns_held = 0`: both new tests failed, then restored.
`311 passed; 0 failed`, `6 passed; 0 failed`, clippy clean with the denial.

One unrelated clippy error was cleared to get that clean run — `sweep.rs:152` indexed
`accord_terms` by its loop variable. It predates this ticket and has nothing to do with the rule.

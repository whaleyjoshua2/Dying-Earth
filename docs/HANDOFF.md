# Handoff: where Dying Earth stands

Written 2026-09-17, just after version 0.08.2 merged. For a reader who knows nothing about this
project. Every figure below names the command that produced it; anything not in a file is not
reported here.

---

## WHAT WAS ASKED

Dying Earth is a single-player, turn-based strategy game about colonizing the solar system before
ecological collapse overtakes Earth. It is meant to feel like a board game rather than an action
game. It is written in Rust and drawn with Bevy, a game engine, and bevy_egui, a library for panels
and menus. One person designs it; the coding is done by an AI agent working to that person's
decisions.

The work arrives one **version** at a time. The designer writes a list of changes they want; that
list is turned into a **map** — a single issue on GitHub holding the plan — with one **ticket** per
decision hanging off it. Each ticket is a question with a recommendation attached; the designer
answers it, the answer is written into the ticket as the record, and only then is it built. The most
recent version, 0.08.2, was asked for in those terms and is finished.

## DONE

**Version 0.08.2 is merged into the main line.** `git log --oneline origin/main -1` gives
`f7b0feb Merge pull request #229 from whaleyjoshua2/version-0.08.2`. The branch carried **15
commits** (`git log --oneline f7b0feb^1..f7b0feb^2 | wc -l`).

What the version did, in the designer's own framing: a score called **Relations** — what each side
thinks of each other side — had existed for two versions and been read by no rule at all. This
version made it matter. Pollution now feeds it, it reads as six named levels rather than a bare
number, offences differ in size, it can be repaired and it can permanently scar, it makes a hostile
rival's ground dearer to take, and two sides can now strike a bargain called an **Accord**. Around
that, the prices in the game's trading window move with what everyone bought, and six smaller changes
make the board say who owns a thing and what kind of thing it is.

**All twelve tickets on the version's map are closed.** Checked with `gh issue view <n> --json state`
for 216 through 227: every one reads `CLOSED`.

**The map itself is still open, deliberately.** `gh issue view 215 --json state` reads `OPEN`. A map
is the version's index and the way back into the work; every earlier version's map is open too, which
`gh issue list --state open --label wayfinder:map` shows — seventeen of them, back to the first
playable version.

**The tests pass and the linter is clean.** `cargo test -p dying-earth-engine --release` reports
`test result: ok. 296 passed; 0 failed` and `test result: ok. 6 passed; 0 failed`.
`cargo clippy --release -- -D warnings` produces no output, meaning no warnings.

**Both playable kits are on the development machine.** `ls dist/*0.08.2*` lists
`dying-earth-0.08.2-linux.zip` and `dying-earth-0.08.2-windows.zip`. The Windows one was built on
this machine; the Linux one came from GitHub's build service, run 35105757048.

**The written record is complete for this version.** `docs/spec/version-0.08.2.md` holds the rules as
decided, one section per ticket. `docs/dev-diary/2026-09-15-version-0.08.2/` holds the pictures,
the mockups, the four measurements taken while deciding, and the sweeps. `docs/playtest/PLAYTEST.txt`
is the note that ships to a playtester.

## IN FLIGHT

**Nothing.** No branch is being worked, no build is running, no agent is out. `git status` on the
main line reports a clean tree, and the only work item that was running — GitHub run 35105757048,
which built the Linux kit — reports `completed / success`.

## REMAINING

There is **no charted work at all** right now: the next version has not been asked for. Everything
below is either a known gap in what shipped, or a thing the designer has already deferred in writing.

1. **Two pieces of the interface that nobody has looked at.** Hovering a side's line in the "in
   orbit" list should name their ships, and the "Found a Colony" button should carry the site's four
   yields on its face. Both are built and tested as code; neither has been seen working, because the
   headless screenshot tool cannot hold a mouse pointer still. Declared to the playtester in
   `docs/playtest/PLAYTEST.txt`. *Estimate: half a session to add a screenshot aid that parks a
   pointer, or five minutes for a human to look.*
2. **The Faction balance, and the Arkwrights in particular.** Recorded in the map's Out of scope
   section: the Custodians win 37 games of 80 and the Arkwrights 3, down from 7. The designer's
   words, quoted there: *"we'll deal with it in future versions."* The material for it already
   exists in `docs/dev-diary/2026-09-15-faction-life-suggestions/03-asymmetry.md`. *Estimate: a
   version of its own.*
3. **Three figures the sweep still does not report**, listed at the end of
   `docs/dev-diary/2026-09-15-version-0.08.2/sweeps/README.md`: money spent per side, Accords struck
   over a game rather than standing at its end, and any aggregation across the four seatings.
   *Estimate: a third of a session.*
4. **Things the version shipped knowing they were half-alive**, all written into the spec and the
   playtest note: two of the five Accord terms are worth almost nothing until the game reaches Mars;
   the computer never sells anything in the trading window, so two of the three prices move only when
   a human trades; and nearly half of all pairs end a game permanently scarred, which is close to a
   failure the plan named for itself. *Estimate: each is a ticket on a future map.*

## WHAT BLOCKS

**Nothing is blocked.** The remaining items are not waiting on anything technical. They wait on the
designer deciding what the next version is, which is how this project starts every version.

## PARALLELISM

1. **Can anything be run in parallel right now?** Nothing, because nothing is running. Of the
   remaining items, the three sweep figures and the screenshot aid have all their inputs present and
   touch different files (`engine/examples/sweep.rs` and `src/shot.rs`), so those two could be run
   side by side in separate worktrees the moment either is wanted.
2. **Have you been unnecessarily idle?** No. The measured span is from the last commit,
   `af7a838` at 2026-09-16, to the merge of pull request 229 at `2026-09-17T22:40:16Z`
   (`gh pr view 229 --json mergedAt`). That span was spent waiting on the designer's word to merge,
   and on GitHub run 35105757048 building the Linux kit, which took about half an hour. Nothing
   runnable sat untouched: the Windows kit was built on this machine during that wait rather than
   waiting for the same run.
3. **Has the supervisor specified serial or parallel work?** None found in `CLAUDE.md`. What that
   file does say, and what any new worker must obey: *"This tree is formatted wide, by hand. Do not
   run `rustfmt` or `cargo fmt`"*, and that issues live as GitHub issues managed with the `gh`
   command-line tool. Separately, the working rule this project has followed for every version is
   that a ticket is decided **and built** in the same session, one commit per ticket on the version's
   branch — visible in the 0.08.1 and 0.08.2 histories.

---

## Things a newcomer will trip over

- **Never run the game program with no arguments.** It has no help flag; run bare it opens a game
  window and sits there. Screenshots are taken with `dying-earth.exe shot:<prefix>`, which puts the
  window off-screen, captures, and exits.
- **Never run `cargo fmt`.** See `CLAUDE.md` above.
- **The glossary is binding.** `CONTEXT.md` defines the game's vocabulary and each entry carries an
  `_Avoid_` list of words the project refuses. Version 0.08.2 had to correct text on the game's first
  screen that used two forbidden words.
- **A map issue is never closed by a pull request.** Name the child tickets in the closing keywords
  and never the map, or the version looks as though it was never planned.
- **Build the Windows kit here; leave the Linux kit to GitHub.** The build service is triggered by
  hand with `gh workflow run release-kits.yml --ref <branch> -f version=<v>`, and its artifacts must
  be downloaded into `dist/` — a kit left in the build service is not delivered.

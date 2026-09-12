# Dying Earth

A single-player, turn-based strategy game about colonizing the solar system before ecological
collapse overtakes Earth. This repository holds the **First Playable**: thirty-six turns of two
months, **four Factions** at every table, **twelve Nation States**, six Bodies (Earth's Antarctica,
the Moon, Mars, Phobos, Deimos and Venus), Space Stations, four resources, a climate with **Breaks**
in it, **Emigrants**, the Prospectors' **Venture Capital Fund**, **saves**, a **spectator mode** and,
since 0.06.0, a space game with weight of its own: every Ship carries its own **Fuel tank**,
**Venus** is a Body of orbits only, the small worlds have a **Mass Driver**, a station has a **Solar
Array**, Research is made off Earth in the **Observatory**, and every Victory Condition waits on a
Tech of its own. The rules are written down in [`docs/spec/first-playable.md`](docs/spec/first-playable.md)
as amended by
[`docs/spec/version-0.02.md`](docs/spec/version-0.02.md),
[`docs/spec/version-0.03.md`](docs/spec/version-0.03.md),
[`docs/spec/version-0.04.md`](docs/spec/version-0.04.md),
[`docs/spec/version-0.05.md`](docs/spec/version-0.05.md),
[`docs/spec/version-0.05.5.md`](docs/spec/version-0.05.5.md) and
[`docs/spec/version-0.06.0.md`](docs/spec/version-0.06.0.md), and the words in [`CONTEXT.md`](CONTEXT.md).

## Running it

The game is `dying-earth.exe` plus the `assets/` folder beside it. From a checkout:

```
cargo run --release
```

Other modes (spec sections 2.5, 3 and 19.3):

```
dying-earth.exe seed:42                 play a fixed seed, so a game can be replayed
dying-earth.exe shot:pictures           write pictures-solar.png, -earth, -moon, -mars off-screen and exit
dying-earth.exe shot:spec spectate:1    the same, watching a game with every seat on the AI
dying-earth.exe savedir:C:\tmp\saves    put saves here instead of the folder below
dying-earth.exe simulate:7              all four seats on the AI; the log goes to simulate-7.log
dying-earth.exe simulate:7 --player=arkwrights    that Faction in seat 0
```

Every game seats the Custodians, the Prospectors, the Arkwrights and the Archivists; the player
takes one and the computer takes the other three, or Spectate hands it all four.

```
cargo run --release -p dying-earth-engine --example sim -- 1 --player=custodians --count=20
cargo run --release -p dying-earth-engine --example sweep -- 20 --player=custodians --start=europe \
    --sinks=6 --steps=300 --permafrost=4.0 --sink-after=4.0 --balance
```

`sim` plays twenty seeds in a row and prints one summary line each; `sweep` plays a cell of the
climate sweep and reports the collapses, the collapse turns, the end Temperature and the wins by
seat.

**Saves** go to `%LOCALAPPDATA%\DyingEarth\data\saves`, one readable RON file per save; the Load
screen on the title screen has a button that opens the folder.

## Changing the numbers

Every number the rules use lives in a TOML file under `assets/data/`: one file per table, with a
comment naming the spec section it comes from. The newer tables are `unrest.toml` (everything that
moves a Nation State's Unrest), `ephemeris.toml` (the sky, and what a transit off the launch window
costs), `report.toml` (every sentence the Report says), and the `[[break]]` rows in `climate.toml`
(the five Temperatures at which the world changes for good). Edit a value and restart; a malformed
table stops the game with a message naming the file and the row.

## Layout

- `engine/` — the rules, with no window attached (`cargo test -p dying-earth-engine` runs the formula tests).
- `src/` — the Bevy window: views, panels, popups, screenshot mode.
- `assets/data/` — the tables. `assets/textures/` — NASA Blue Marble, LRO Moon and USGS Viking Mars
  maps, plus the Nation State mask derived from the coastlines (`examples/prep_assets.rs` makes it)
  and, since 0.06.0, a made cloud map for Venus (`examples/prep_assets.rs --venus`).
- `docs/` — the spec, the glossary's decisions (`docs/adr/`), agent notes and the dev diary.

## Kits for other systems

This machine builds only the Windows kit. The GitHub Actions workflow `release-kits` (Actions tab,
"Run workflow", or `gh workflow run release-kits.yml --ref <branch> -f version=0.06.0`) builds a Linux
kit on an Ubuntu 22.04 runner and a Windows kit, each as a downloadable artifact zip ready for
itch.io. With a repository secret `BUTLER_API_KEY` and a variable `ITCH_TARGET` set, it also pushes
both to itch.io.

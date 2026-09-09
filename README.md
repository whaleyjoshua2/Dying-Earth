# Dying Earth

A single-player, turn-based strategy game about colonizing the solar system before ecological
collapse overtakes Earth. This repository holds the **First Playable**: twenty-four turns, nine
Nation States, the Moon and Mars, three resources, two Factions. The rules are written down in
[`docs/spec/first-playable.md`](docs/spec/first-playable.md) as amended by
[`docs/spec/version-0.02.md`](docs/spec/version-0.02.md), and the words in [`CONTEXT.md`](CONTEXT.md).

## Running it

The game is `dying-earth.exe` plus the `assets/` folder beside it. From a checkout:

```
cargo run --release
```

Other modes (spec sections 2.5, 3 and 19.3):

```
dying-earth.exe seed:42                 play a fixed seed, so a game can be replayed
dying-earth.exe shot:pictures           write pictures-solar.png, -earth, -moon, -mars off-screen and exit
dying-earth.exe simulate:7              two AIs play a whole game headless; the log goes to simulate-7.log
dying-earth.exe simulate:7 --prospector-ai --custodian-ai     the Prospectors in the player's seat
```

Factions must be unique: a game is one Custodian seat against one Prospector seat.

`cargo run -p dying-earth-engine --example sim -- 1 custodians prospectors --count=20` plays twenty
seeds in a row and prints one summary line each.

## Changing the numbers

Every number the rules use lives in a TOML file under `assets/data/`: one file per table, with a
comment naming the spec section it comes from. Edit a value and restart; a malformed table stops the
game with a message naming the file and the row.

## Layout

- `engine/` — the rules, with no window attached (`cargo test -p dying-earth-engine` runs the formula tests).
- `src/` — the Bevy window: views, panels, popups, screenshot mode.
- `assets/data/` — the tables. `assets/textures/` — NASA Blue Marble, LRO Moon and USGS Viking Mars
  maps, plus the Nation State mask derived from the coastlines (`examples/prep_assets.rs` makes it).
- `docs/` — the spec, the glossary's decisions (`docs/adr/`), agent notes and the dev diary.

---
ticket: 62
date: 2026-09-09
status: research, not a decision
---

# Saving and loading the whole game state on Windows

Research for ticket [#62](https://github.com/whaleyjoshua2/Dying-Earth/issues/62). Every claim below
carries the URL or the file path it came from. Nothing here has been implemented; the last two
sections are a recommendation and a list of the calls that are still the designer's to make.

There is no existing `docs/research/` convention in this repo — this file starts one, matching the
plain-prose voice of `docs/adr/0001-draw-with-bevy-and-bevy-egui.md`.

---

## 1. What actually has to go in the file

Read before anything else, because it decides most of the rest.

**The whole game is one plain-Rust struct.** `Game` in
[`engine/src/state.rs:365-389`](../../engine/src/state.rs) holds the turn, the two seats, the Nation
States, Colonies, Ships, Armies, the Climate, Research, the Deck, the Discoveries, the last event,
the Report, the Outcome, the id counter, the pending orders and the log. `engine/Cargo.toml` names
only `serde`, `toml`, `rand` and `rand_chacha` — **no Bevy types reach the engine at all**, so the
save has no `Handle<Image>`, no `Entity`, no `Component` to worry about. The window crate's
`Session` and `ViewState` (`src/app.rs:48-133`) likewise hold no Bevy handles: the textures live in
a separate `Textures` resource loaded from disk at startup (`src/textures.rs:33`).

**One field must *not* be saved.** `Game.tables: std::sync::Arc<Tables>` is the read-only rules data,
loaded from the twelve TOML files under `assets/data/` (`engine/src/data.rs:469`, `assets/data/`).
`Tables` derives only `Deserialize`, never `Serialize` (`engine/src/data.rs:428`), and it is the same
for every game. Saving it would multiply the file size for nothing and would let a save disagree with
the tables the executable ships with. It should be rebuilt from `assets/data/` on load, exactly as
`Game::new` does now.

**About thirty-two types need `Serialize` + `Deserialize` added.** All 16 public types in
`engine/src/ids.rs` already derive both (`BodyId`, `StateId`, `FacilityKind`, `ModuleKind`,
`UnitKind`, `Resource`, `TechId`, `EventId`, `EventKind`, `FactionKind`, `Seat`, `ShipId`, `ArmyId`,
`ColonyId`, `Place`, `Stance`) because the data tables name them. What is missing is the state
layer: 26 types in `engine/src/state.rs` (`Stockpile`, `Control`, `Facility`, `Module`,
`BuildItem`, `Build`, `NationState`, `Colony`, `ShipAt`, `Ship`, `ArmyHome`, `ArmyAt`, `Army`,
`EmissionsBreakdown`, `Climate`, `Research`, `Card`, `Deck`, `DrawnEvent`, `EventTarget`,
`Discovery`, `SeatState`, `Outcome`, `BattleLine`, `Report`, `Game`), plus `UnitRef`, `LoadSource`,
`UnloadTarget`, `Order`, `Cost` and `Pending` from `engine/src/orders.rs`. These are ordinary
structs and enums of `i64`, `u32`, `f64`, `bool`, `String`, `Vec`, `Option` and fixed arrays —
nothing exotic.

**The awkward parts, in order of how much they matter:**

1. **`BTreeMap<Target, i64>`, and `Target = Place` is an enum** (`engine/src/state.rs:298`,
   `engine/src/ids.rs:332-338`). `Place` is `State(StateId)` or `Colony(ColonyId)` — a *newtype
   variant*. **This cannot be written as JSON.** `serde_json`'s map-key serializer implements
   `serialize_newtype_variant` as a hard error (`serde_json-1.0.151/src/ser.rs:1078-1090`, returning
   `key_must_be_a_string()`), whose message is literally `"key must be a string"`
   (`serde_json-1.0.151/src/error.rs:371`). A `BTreeMap` keyed by `Place` would fail at run time, not
   compile time. RON has no such limit — its own docs give `{ "arbitrary": "keys", "are": "allowed" }`
   as the map syntax (<https://docs.rs/ron/0.12.2/ron/>).
2. **There are no `HashMap`s anywhere.** A grep for `HashMap`/`HashSet` across `engine/src` and `src`
   returns nothing; the one map in the state is a `BTreeMap`. So the classic "HashMap iteration order
   makes saves non-reproducible" problem **does not exist here**, and should not be allowed to appear
   later: a `BTreeMap` writes its entries in key order every time.
3. **Floats are values, never keys.** `f64` appears in `NationState.population`,
   `wildfire_emissions_next`, all of `Climate` and `EmissionsBreakdown`, `Discovery.multiplier` and
   `DrawnEvent.scale`; `f32` appears only in the *tables* (`lon`, `lat`, `colour`), which are not
   saved. Because no map is keyed by a float, the usual float-key hazard is absent. Exact float
   round-tripping through a text format is still worth an actual test rather than an assumption —
   see the red-witness note in §7.
4. **`Game.log: Vec<String>`** grows for the whole game and is documented as "Lines for the simulate
   log and the dev diary; the interface ignores them" (`engine/src/state.rs:387`). Whether it belongs
   in a save is a judgement call, not a technical one.

**The file will be small.** Two seats, eight Nation States, a handful of Colonies, Ships and Armies,
and a 28-card Deck (the `copies` figures in `assets/data/events.toml` sum to 28). Even as pretty
RON this is tens of kilobytes, not megabytes. That number governs §6.

---

## 2. The format choices with serde

serde's own list of data formats is at <https://serde.rs/#data-formats>. The distinction that
matters is whether a format is **self-describing**: serde defines it as "Formats that can implement
`deserialize_any` and `deserialize_ignored_any`" (<https://serde.rs/impl-deserializer.html>), and
`Deserializer`'s docs put the consequence plainly: "Self-describing data formats like JSON are able
to look at the serialized data and tell what it represents. Non-self-describing formats like Postcard
need to be told what is in the input in order to deserialize it."
(<https://docs.rs/serde/latest/serde/trait.Deserializer.html>)

The second distinction, and the one that decides everything about versioning, is whether **struct
field names travel in the file**. If they do, a field can be added or renamed and old files still
load. If they do not, the file is a positional blob and any change to a struct silently misreads it.

| Format | Crate / version | Text? | Field names on the wire? | Self-describing? |
|---|---|---|---|---|
| RON | `ron` 0.12.2 | yes | yes, unquoted | partly — see below |
| JSON | `serde_json` 1.0.151 | yes | yes, quoted | yes |
| bincode | `bincode` — **unmaintained** | no | no | no |
| postcard | `postcard` 1.1.3 | no | no | explicitly not |
| MessagePack | `rmp-serde` 1.3.1 | no | only with `with_struct_map` | yes |

### RON (`ron` 0.12.2)

"RON is a simple readable data serialization format that looks similar to Rust syntax. It's designed
to support all of Serde's data model, so structs, enums, tuples, arrays, generic maps, ranges, and
primitive values." (<https://github.com/ron-rs/ron>) Its README's advantages over JSON are exactly
the ones a human reader cares about: "trailing commas allowed", "single- and multi-line comments",
"field names aren't quoted, so it's less verbose", "optional struct names improve readability",
"enums are supported (and less verbose than their JSON representation)".

Caveats, from <https://docs.rs/ron/0.12.2/ron/> and the README:

- "RON is not designed to be a fully self-describing format (unlike JSON) and is thus not guaranteed
  to work when `deserialize_any` is used instead of its typed alternatives." Deriving
  `Deserialize` normally does *not* use `deserialize_any`, so this does not bite a plain derive.
- "RON requires struct, enum, and variant names to be valid Rust identifiers and will reject invalid
  ones created by `#[serde(rename = "...")]`". Note that `engine/src/ids.rs` uses
  `#[serde(rename_all = "snake_case")]` on its enums — snake_case variant names are still valid Rust
  identifiers, so this is fine, but it is a constraint to remember.
- Limited support for `#[serde(flatten)]`, internally/adjacently tagged and untagged enums.
- `PrettyConfig` controls the output layout, including a `struct_names` setting
  (<https://docs.rs/ron/0.12.2/ron/>).

**Already in the tree.** `ron 0.12.2` is in `Cargo.lock:5252`, pulled in by `bevy_asset`,
`bevy_animation` and `bevy_world_serialization` (`Cargo.lock:521, 617, 1952`). Adding it to
`engine/Cargo.toml` costs no new transitive dependency and no extra crate compile.

### JSON (`serde_json` 1.0.151)

The most universally readable text format, fully self-describing, and serde's own canonical example
of one (<https://serde.rs/impl-deserializer.html>). `to_string_pretty` gives editable output
(<https://docs.rs/serde_json>). It is also the **only candidate that is outright disqualified by
this codebase**: the `BTreeMap<Place, i64>` in `SeatState` cannot be serialized, per the source
quoted in §1. Working around it means changing the state's shape or writing a custom key
serializer — real work, to reach a format that is bulkier and less pleasant to read than RON.

### bincode — do not use

`bincode`'s latest published version is **3.0.0, published 2025-12-16**
(<https://crates.io/api/v1/crates/bincode>), and that release is a tombstone: docs.rs states
"Bincode is now unmaintained. Due to a doxxing and harassment incident, development on bincode has
ceased. No further releases will be published on crates.io."
(<https://docs.rs/crate/bincode/latest>). 3.0.0 ships "a lib.rs containing only a compiler error, to
inform potential users of the maintenance status of this crate"; the GitHub repository is archived.
The last usable release is 2.0.1, where serde is an opt-in feature and the docs list attributes that
"can and will cause issues with bincode and will result in lost data" —
`flatten`, `skip`, `skip_deserializing`, `skip_serializing`, `skip_serializing_if`, `tag`, `untagged`
(<https://docs.rs/bincode/2.0.1/bincode/serde/index.html>). Ruled out on maintenance grounds alone.

### postcard 1.1.3

Compact, `no_std`, wire format stable since 1.0. Also the clearest possible statement that it is the
wrong tool for a save file: "Postcard is NOT considered a 'Self Describing Format'… users
(Serializers and Deserializers) of postcard data are expected to have a mutual understanding of the
encoded data", "As `struct`s have a known number of elements with known names, their length and field
names are not encoded on the wire", and, decisively, "Backwards/forwards compatibility between
revisions of a postcard schema are considered outside of the scope of the postcard wire format, and
must be considered by the end users." (<https://postcard.jamesmunns.com/wire-format>)

### MessagePack (`rmp-serde` 1.3.1)

Compact binary that "resembles a compact JSON" (<https://serde.rs/#data-formats>), self-describing,
and it *can* carry field names — but not by default. `Serializer::with_struct_tuple` "will serialize
structs as a tuple without field names" and is "the default MessagePack serialization mechanism";
`with_struct_map` is needed to get named fields
(<https://docs.rs/rmp-serde/latest/rmp_serde/encode/struct.Serializer.html>). A separate trap: serde
represents `Vec<u8>` as an array of arbitrary objects rather than bytes, costing roughly 50% storage
overhead unless `serde_bytes` or `with_bytes` is used (<https://docs.rs/rmp-serde>). It is a good
format, but it is not readable by a person, which is the whole question here.

### The trade-off, stated plainly

For a file the designer might open, the choice is between the two text formats, and only two things
separate them: RON is less noisy to read and can hold the `Place`-keyed map; JSON is more universally
understood and every tool on earth can pretty-print it. Given that the map is a genuine blocker,
RON wins on facts rather than taste. The binary formats buy speed and size that a tens-of-kilobyte
file does not need, and every one of them costs either maintenance risk (bincode), schema
brittleness (postcard, bincode), or readability (all three).

---

## 3. Versioning the save, so an old file is refused cleanly

### What serde does by default

- **A missing field is an error.** serde's `Error::missing_field` is "Raised when a `Deserialize`
  struct type expected to receive a required field with a particular name but that field was not
  present in the input." (<https://docs.rs/serde/latest/serde/de/trait.Error.html>) So *adding* a
  field to a struct breaks every old save unless that field is defaulted.
- **An unknown field is silently ignored.** serde states it in the `deny_unknown_fields` entry:
  "When this attribute is not present, by default unknown fields are ignored for self-describing
  formats like JSON." (<https://serde.rs/container-attrs.html#deny_unknown_fields>) So *removing* a
  field is already tolerated.
- **A duplicate field is an error** (`Error::duplicate_field`, same page).

### The three attributes that matter

- `#[serde(default)]` on a field: "If the value is not present when deserializing, use the
  `Default::default()`." On a container: "any missing fields should be filled in from the struct's
  implementation of `Default`. Only allowed on structs."
  (<https://serde.rs/field-attrs.html#default>, <https://serde.rs/container-attrs.html#default>)
  This is the tool for adding a field without breaking old saves.
- `#[serde(alias = "name")]`: "Deserialize this field from the given name *or* from its Rust name.
  May be repeated." (<https://serde.rs/field-attrs.html#alias>) This is the tool for renaming a
  field without breaking old saves.
- `#[serde(deny_unknown_fields)]`: "Always error during deserialization when encountering unknown
  fields." Both serde pages note it "is not supported in combination with `flatten`, neither on the
  outer struct nor on the flattened field."
  (<https://serde.rs/container-attrs.html#deny_unknown_fields>,
  <https://serde.rs/field-attrs.html#flatten>)

### Why a version field is still needed

Those attributes handle *tolerable* drift. They do nothing about drift that changes meaning — a
Ducat cost that used to be per-step and is now per-turn, a Colony Slot index that used to be
Antarctica's and now is not. This game changes its rules every version (`assets/data/` is rewritten
by tickets; see the version notes threaded through `CONTEXT.md`), so a save from version 0.03 loaded
into 0.05 would deserialize perfectly and then be *wrong*. That is worse than a crash.

The clean shape, and it is deliberately boring:

1. Wrap the state in an outer struct that carries a `version: u32` as its **first** field, plus
   whatever provenance is useful (the game version string, the seed, the turn, an ISO date).
2. On load, deserialize the header *first* — into a small header-only struct, or by reading the
   whole file into `ron::Value` and pulling the version out — and compare it against the constant the
   executable was built with. If it does not match, return an error the interface can show as a
   sentence: "This save was written by version 0.04. This is version 0.05, and its rules have
   changed. Start a new game." Never `unwrap`.
3. Bump the version constant in the same commit as any rules change that alters what a saved number
   means, and treat that as part of the ticket.
4. Put `#[serde(deny_unknown_fields)]` on the outer wrapper only, so a garbled or foreign file fails
   at the header rather than half-loading. Leave the inner state without it, so `#[serde(default)]`
   and `alias` can absorb small additions between bumps.

A single monotonically increasing integer is enough. Semantic versioning of the save format buys
nothing here, because there is exactly one reader and it is the same executable that wrote it.

---

## 4. Where the file goes on Windows

### Microsoft's own answer, aimed at game developers

Microsoft publishes guidance written specifically for this
(<https://learn.microsoft.com/en-us/windows/win32/dxtecharts/user-account-control-for-game-developers>,
last updated 2025-07-24):

- **Never beside the executable.** "You should never assume that your game can write files to the
  folder where your game is installed… a user's privileges must be elevated by the operating system
  before an application can write to the Program Files folder." A standard user "cannot write to the
  Program Files folder".
- **The choice rule.** "The recommended practice for selecting the CSIDL constant to use for writing
  a file is to use `CSIDL_PERSONAL` if the user is expected to interact with the file, such as
  double-clicking on it to open it in a tool or application, and to use `CSIDL_LOCAL_APPDATA` for
  other files." `CSIDL_PERSONAL` is Documents; the same page's table lists "Saved game files with a
  file extension association. Screen shots." against it, and "Game cache files. Player
  configurations." against `CSIDL_LOCAL_APPDATA`.
- **Keep the path short and unique.** "Care should be taken to create path names that are unique
  enough to not collide with other applications, but short enough to keep the number of characters
  in the full path fewer than the value of MAX_PATH, 260."
- **Not the registry.** "Storing persistent data in the registry, like a user's configuration, is not
  recommended."

### The known folders themselves

From <https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid>:

| Known folder | Default path |
|---|---|
| `FOLDERID_RoamingAppData` | `%APPDATA%` (`%USERPROFILE%\AppData\Roaming`) |
| `FOLDERID_LocalAppData` | `%LOCALAPPDATA%` (`%USERPROFILE%\AppData\Local`) |
| `FOLDERID_SavedGames` | `%USERPROFILE%\Saved Games` |
| `FOLDERID_Documents` | `%USERPROFILE%\Documents` |

Roaming versus local is not a style choice. Microsoft: `CSIDL_LOCAL_APPDATA` "serves as a data
repository for local (nonroaming) applications"
(<https://learn.microsoft.com/en-us/windows/win32/shell/csidl>); and, on a domain, "Windows uses the
Local and LocalLow folders for application data that does not roam with the user. Usually this data
is either machine specific or too large to roam", while Roaming is for data "which are machine
independent and should roam with the user profile"
(<https://learn.microsoft.com/en-us/previous-versions/windows/it-pro/windows-vista/cc766489(v=ws.10)>).
Save games are per-machine-ish, potentially numerous, and nobody wants them copied over a network at
logon: **Local**, not Roaming.

### Program Files versus a Desktop or user folder

This is the trap that makes "just write beside the .exe" look like it works.

- Run from `C:\Users\Josh\games\Dying-Earth\target\debug\`, writing beside the executable **succeeds**,
  because the user owns their own profile. Every test passes.
- Installed to `C:\Program Files\Dying Earth\`, the same code **fails for a standard user**, per the
  quote above.
- The old escape hatch, UAC virtualization, silently redirects the write to
  `C:\Users\<user>\AppData\Local\VirtualStore\Program Files\…` (the game-dev page gives exactly this
  example) — but it is a dead end. Three separate Microsoft pages say so:
  "Virtualization supports only 32-bit apps. Nonelevated 64-bit apps receive an access denied message";
  "Virtualization is disabled if the app includes an app manifest with a requested execution level
  attribute"; "it's a short-term fix and not a long-term solution"
  (<https://learn.microsoft.com/en-us/windows/security/application-security/application-control/user-account-control/architecture>);
  "This form of virtualization is an interim application compatibility technology; Microsoft intends
  to remove it from future versions of the Windows operating system"
  (<https://learn.microsoft.com/en-us/windows/win32/sysinfo/registry-virtualization>); and the
  game-dev page's own "64-bit applications are never run in legacy mode… operations subjected to
  virtualization in 32-bit applications will just fail in 64-bit." A Rust game built for
  `x86_64-pc-windows-msvc` is 64-bit, so **the write simply fails with access denied** — os error 5.

Note that the engine already reads its tables from beside the executable
(`engine/src/data.rs:624-637`, `current_exe()` then `assets/data`). That is correct: *reading* from
the install directory is fine and expected. It is writing that is not.

### The Documents caveat: OneDrive

`FOLDERID_Documents` is one of the folders OneDrive's Known Folder Move retargets. Microsoft:
"moving or redirecting Windows known folders (Desktop, Documents, Pictures, Screenshots, and Camera
Roll) to Microsoft OneDrive", available both as a prompt and as a "Silently move Windows known
folders to OneDrive" policy
(<https://learn.microsoft.com/en-us/sharepoint/redirect-known-folders>). The known-folder API returns
the redirected path, so the code keeps working — but the saves then live in a cloud-synced folder,
where a mid-write sync or a two-machine conflict is a real failure mode. This is the argument
against Documents, and it is the counterweight to Microsoft's own "use Documents if the user
interacts with the file" rule.

### The crates

- **`directories` 6.0.0** (<https://docs.rs/directories>): `ProjectDirs::from(qualifier,
  organization, application)` gives, on Windows, `data_local_dir` =
  `{FOLDERID_LocalAppData}/<project_path>/data`, e.g.
  `C:\Users\Alice\AppData\Local\Foo Corp\Bar App\data`; `data_dir` is the Roaming equivalent;
  `config_dir` and `preference_dir` are `{FOLDERID_RoamingAppData}/<project_path>/config`. On Windows
  `<project_path>` is `"<organization>/<application>"` — **the qualifier is ignored** (it "only
  affects macOS"). Note Windows gets an extra `\data` or `\config` segment that the other platforms
  do not.
- **`dirs` 7.0.0** (<https://docs.rs/dirs>): the low-level version — `data_local_dir()` is just
  `{FOLDERID_LocalAppData}` with no per-project subfolder. Its own README says: "If you want to
  compute the location of cache, config or data directories for your own application or project, use
  `ProjectDirs` of the directories project instead." Also: "This library does not create directories
  or check for their existence" — the game must `create_dir_all` itself.
- **Neither crate exposes `FOLDERID_SavedGames`.** There is no `saved_games_dir()` in either table.
  Reaching `%USERPROFILE%\Saved Games` needs `SHGetKnownFolderPath` through the `windows` crate, or
  joining onto `home_dir()` and hoping the folder has not been relocated. That is a real cost for a
  folder whose only advantage is that some other games use it.

### Writing the file safely

`std::fs::write` "will create a file if it does not exist, and will entirely replace its contents if
it does" (<https://doc.rust-lang.org/std/fs/fn.write.html>) — and documents no atomicity. A crash or
a full disk mid-write leaves a truncated save where a good one used to be. The standard fix is to
write `save.ron.tmp` and then rename over the real file; `std::fs::rename` "currently corresponds to…
`MoveFileExW` with a fallback to `SetFileInformationByHandle` on Windows", and on Windows "`from` can
be anything but `to` must not be a directory"
(<https://doc.rust-lang.org/std/fs/fn.rename.html>) — i.e. replacing an existing *file* is the
supported case. Worth doing; it costs three lines.

---

## 5. The random number generator and the shuffled deck

### The engine's generator

`engine/Cargo.toml` names `rand = "0.9"` and `rand_chacha = "0.9"`; `Cargo.lock` pins `rand 0.9.5`
and `rand_chacha 0.9.0`. `Game.rng` is a `ChaCha8Rng` (`engine/src/state.rs:368`), seeded once with
`ChaCha8Rng::seed_from_u64(setup.seed)` (`engine/src/state.rs:399`), and `Game.seed: u64` keeps the
original seed alongside it.

**`ChaCha8Rng` serializes, and it is the crate's own supported path.** `rand_chacha` has three cargo
features — `std` (default), `os_rng`, `serde`
(<https://docs.rs/crate/rand_chacha/0.9.0/features>) — and `ChaCha8Rng` implements `Serialize` and
`Deserialize` under the `serde` feature
(<https://docs.rs/rand_chacha/0.9.0/rand_chacha/struct.ChaCha8Rng.html>). The feature is spelled
`serde`, not `serde1`. What gets written is a small abstract state — `seed: [u8; 32]`, `stream: u64`,
`word_pos: u128` — and the source carries the promise that matters:
"The comparison and serialization of this object is considered a semver-covered part of the API."
(<https://docs.rs/rand_chacha/0.9.0/src/rand_chacha/chacha.rs.html>)

So the change is one line in `engine/Cargo.toml`:
`rand_chacha = { version = "0.9", features = ["serde"] }`, and then `Game.rng` derives along with
everything else. The manual alternative — storing `get_seed()`, `get_stream()` and `get_word_pos()`
and restoring with `from_seed` + `set_stream` + `set_word_pos`
(<https://docs.rs/rand_chacha/latest/rand_chacha/struct.ChaCha8Rng.html>) — is exactly what the serde
impl does, so there is no reason to hand-roll it.

**A caveat about `f64` sampling.** `Game.rng` is used through `chance(p)` as
`self.rng.random::<f64>() < p` (`engine/src/events.rs:35`, `engine/src/combat.rs:24`). rand's own
portability guidance warns that floating-point results can vary by platform
(<https://rust-random.github.io/book/crate-reprod.html>). This affects reproducing a whole game from
a seed across machines, not reloading a save on the same machine — but it is worth knowing before
anyone promises seed-identical games across builds.

### The deck

The deck is built once and never reshuffled: `new_deck` pushes `copies` of each event and calls
`cards.shuffle(rng)`, and the doc comment says "The deck as the table deals it, shuffled with the
game seed; never reshuffled" (`engine/src/events.rs:11-21`). `Deck` is `{ cards: Vec<Card>, drawn:
Vec<Card> }` (`engine/src/state.rs:245-248`), and `Card` is a one-variant enum wrapping `EventId`.

**So the deck needs no special handling at all** — saving `Vec<Card>` saves the exact post-shuffle
order, and `drawn` records what has already gone. This is the right design and it should be kept,
because the alternative (storing the seed and re-shuffling on load) would be fragile: rand's
reproducibility policy says "Minor releases, including after 1.0, may make value-breaking changes to
portable items" (<https://rust-random.github.io/book/crate-reprod.html>), so a bump from `rand` 0.9
to 0.10 could legally change what `shuffle` produces from the same generator state. Saving the order
outright is immune to that. (`SliceRandom::shuffle` itself promises only that "The resulting
permutation is picked uniformly from the set of all possible permutations" —
<https://docs.rs/rand/0.9.2/rand/seq/trait.SliceRandom.html>.)

Between the serialized `ChaCha8Rng` state and the literal deck order, a reloaded game continues
bit-identically to one that was never saved.

---

## 6. Doing the write inside a Bevy app

Bevy's task pools are documented by purpose (all from
<https://docs.rs/bevy_tasks/0.19.1/bevy_tasks/>, the version matching this game's pinned Bevy 0.19.1):

- `IoTaskPool` — "A newtype for a task pool for IO-intensive work (i.e. tasks that spend very little
  time in a 'woken' state)."
- `AsyncComputeTaskPool` — "for CPU-intensive work that may span across multiple frames."
- `ComputeTaskPool` — "for CPU-intensive work that must be completed to deliver the next frame."

`bevy` re-exports these as `bevy::tasks` and puts `IoTaskPool` in `bevy::tasks::prelude`
(<https://docs.rs/bevy/0.19.1/bevy/tasks/prelude/index.html>); it is obtained with `IoTaskPool::get()`,
which "Panics if the global instance has not been initialized yet", and it *is* initialized, because
`TaskPoolPlugin` — "Setup of default task pools: `AsyncComputeTaskPool`, `ComputeTaskPool`,
`IoTaskPool`" — is part of `DefaultPlugins`
(<https://docs.rs/bevy_internal/0.19.1/bevy_internal/struct.DefaultPlugins.html>).

**The current documented polling pattern has changed.** Bevy 0.19's own `async_compute` example now
carries this warning verbatim
(<https://raw.githubusercontent.com/bevyengine/bevy/v0.19.1/examples/async_tasks/async_compute.rs>):

> Don't use `future::block_on(poll_once)` to check if tasks are completed, as it is expensive and
> can block the main thread. Also, it leaves around a `Task<T>` which will panic if awaited again.
> Instead, use `check_ready` for efficient polling, which does not block the main thread.

`check_ready` is `pub fn check_ready<F: Future + Unpin>(future: &mut F) -> Option<F::Output>` —
"Polls a future once, and returns the output if ready or returns `None` if it wasn't ready yet"
(<https://docs.rs/bevy_tasks/0.19.1/bevy_tasks/futures/fn.check_ready.html>). Note also that
"Dropping a `Task` cancels it" (<https://docs.rs/bevy_tasks/0.19.1/bevy_tasks/struct.Task.html>), so
a `Task` stored in a component must outlive the write, or be `detach()`ed. Bevy's other official
pattern is a detached task plus a crossbeam channel
(<https://bevy.org/examples/async-tasks/async-channel-pattern/>).

**But: can a small save just block?** Yes, and Bevy's own code says so by example. `save_to_disk`,
the first-party screenshot writer this game already uses (`src/shot.rs:6,91,182`), encodes and writes
a full-resolution PNG **synchronously on the main thread** — `img.save_with_format(&path, format)`
inside the observer, no task pool anywhere
(`bevy_render-0.19.1/src/view/window/screenshot.rs:134-148`). If Bevy is content to block a frame
writing a PNG, blocking it to write tens of kilobytes of RON is not a defensible worry. The game also
already writes files with plain `std::fs::write` (`src/main.rs:50`).

Bevy documents **no first-party save-file API**. `bevy_asset`'s `AssetWriter` is described as
"Performs write operations on an asset storage… a 'virtual filesystem' API"
(<https://docs.rs/bevy_asset/0.19.1/bevy_asset/io/trait.AssetWriter.html>), it is optional per asset
source (`MissingAssetWriterError` exists), and nothing in the docs designates it for game saves.
Plain `std::fs` is the path, but note that as an inference: Bevy does not state it either way.

No official Bevy statement quantifies a frame budget (no "16.6 ms" anywhere in the docs); the pool
descriptions are the only guidance, and they are qualitative.

**The honest engineering position:** save with `std::fs` on the main thread, and measure the stall
once. If a full save turns out to cost more than a frame — which the size estimate in §1 says it will
not — move it to `IoTaskPool` with `check_ready`, which is a contained change because the
serialization step (`Game` → `String`) is already separate from the write.

---

## 7. Recommendation

**This section is a recommendation for the build ticket, not a decision.** The design calls in §8
are the designer's.

**One format: RON**, via `ron = "0.12"` added to `engine/Cargo.toml`, written with
`ron::ser::to_string_pretty` and a `PrettyConfig`, file extension `.ron`.

- It is the only readable format that can serialize `BTreeMap<Place, i64>` at all; JSON errors with
  "key must be a string" (§1).
- `ron 0.12.2` is already in `Cargo.lock` via Bevy, so it costs nothing new to compile.
- Unquoted field names and Rust-shaped enums mean the designer opening `save.ron` sees
  `control: Controlled(Seat(0))`, not `{"control":{"Controlled":[0]}}`.
- It carries field names, which is what makes the `#[serde(default)]` / `alias` tolerance in §3 work
  at all.

**One folder: `%LOCALAPPDATA%\Dying Earth\data\saves\`**, reached with `directories 6`:
`ProjectDirs::from("", "", "Dying Earth")`, then `data_local_dir().join("saves")`, with a
`create_dir_all` before the first write (the crate does not create directories). The `\data` segment
is not a typo — `directories` appends it on Windows and nowhere else (§4); accept it rather than
hand-building the path, or use `dirs::data_local_dir()` and join the whole tail yourself if the
extra folder is unwanted. Write to a `.tmp` and `std::fs::rename` over the target.

- Never beside the executable: that is Microsoft's explicit instruction, and the failure only appears
  once the game is installed to Program Files, which is the worst time to discover it (§4).
- Local rather than Roaming, because saves are per-machine and should not travel over a network at
  logon.
- `%LOCALAPPDATA%` rather than Documents because Documents is a OneDrive Known Folder Move target,
  and a cloud-synced save folder is a class of bug nobody wants to debug. Microsoft's "use Documents
  if the user interacts with the file" rule genuinely points the other way — this is the one place
  where the recommendation overrides the letter of the guidance, and §8 puts it to the designer.
- Not `%USERPROFILE%\Saved Games`: neither `dirs` nor `directories` exposes it, so it means pulling
  in the `windows` crate for one path.
- Add a "Show saves folder" button or a printed path in the interface, so the designer can find the
  file without knowing what `%LOCALAPPDATA%` means. That, plus RON, is what makes "the designer can
  open it" true in practice.

**One versioning scheme: a single `u32` in a wrapper struct, checked before anything else.**

```rust
pub const SAVE_FORMAT_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SaveFile {
    pub version: u32,          // first field, always
    pub game_version: String,  // "0.05", for the message
    pub turn: u32,             // so a save can be listed without fully loading it
    pub seed: u64,
    pub saved: String,         // ISO date, for the human
    pub game: Game,            // Tables excluded; rebuilt from assets/data on load
}
```

- Read the version first — via a header-only struct or `ron::Value` — and if it is not
  `SAVE_FORMAT_VERSION`, return a `Result::Err` carrying a full sentence for the interface to show.
  Never panic, never `unwrap`, never half-load.
- `deny_unknown_fields` on the wrapper only. The inner state stays permissive so `#[serde(default)]`
  can absorb a field added mid-version.
- Bump the constant in the same commit as any rules change that alters the meaning of a saved number,
  and say so in the ticket. This game rewrites its own rules every version; a save that loads and is
  quietly wrong is worse than one that is refused.
- Serialize `Game.rng` directly by enabling `rand_chacha`'s `serde` feature; save the deck as the
  literal `Vec<Card>` it already is. Together those two make a reloaded game continue identically
  (§5).

**Before believing any of this works**, write the failing test first, per the repo's `red-witness`
practice: (a) save a mid-game state, load it, and assert the two `Game` values are equal field for
field — including every `f64`, which is the round-trip claim this document deliberately did not
assert; (b) run N turns from a save and N turns without saving, and assert the resulting states
match, which is the only real proof the RNG and deck restore correctly; (c) hand-edit the `version`
field in a saved file and assert the load returns an error with a readable message, not a panic.

---

## 8. Still the designer's to decide

Flagged rather than settled, per the standing preference that design calls belong to the user.

1. **Readable saves, or tamper-resistant ones?** A readable RON save is one the designer can open —
   and one a player can edit to give themselves a thousand Ducats. For a single-player strategy game
   that is usually fine, or even a feature. Say if it is not.
2. **Documents or `%LOCALAPPDATA%`?** Microsoft's own rule says Documents for files the user opens;
   the recommendation above chose `%LOCALAPPDATA%` to dodge OneDrive. If the designer wants saves to
   be as findable as screenshots, Documents is the defensible other answer.
3. **How many save slots, and does the game autosave?** One slot, named slots, or one per turn? The
   research covers the file; it says nothing about how many of them there should be.
4. **Does `Game.log` belong in the save?** It is the only field that grows without bound, it is
   explicitly ignored by the interface (`engine/src/state.rs:387`), and dropping it would make saves
   smaller but would lose the game's history on reload.
5. **Is a save from an older *rules* version worth migrating, ever?** The recommendation refuses them
   outright, which is cheap and honest. Writing migrations is a real ongoing cost and only pays if
   the designer expects long campaigns to survive version bumps.

---

## Sources

Primary sources, all fetched 2026-09-09.

**serde** — <https://serde.rs/#data-formats> · <https://serde.rs/impl-deserializer.html> ·
<https://serde.rs/attributes.html> · <https://serde.rs/container-attrs.html> ·
<https://serde.rs/field-attrs.html> · <https://serde.rs/enum-representations.html> ·
<https://docs.rs/serde/latest/serde/trait.Deserializer.html> ·
<https://docs.rs/serde/latest/serde/de/trait.Error.html>

**Formats** — <https://docs.rs/ron/0.12.2/ron/> · <https://github.com/ron-rs/ron> ·
<https://docs.rs/serde_json> · <https://docs.rs/crate/bincode/latest> ·
<https://crates.io/api/v1/crates/bincode> · <https://docs.rs/bincode/2.0.1/bincode/serde/index.html> ·
<https://docs.rs/postcard> · <https://postcard.jamesmunns.com/wire-format> ·
<https://docs.rs/rmp-serde> ·
<https://docs.rs/rmp-serde/latest/rmp_serde/encode/struct.Serializer.html>

**Random numbers** — <https://docs.rs/crate/rand_chacha/0.9.0/features> ·
<https://docs.rs/rand_chacha/0.9.0/rand_chacha/struct.ChaCha8Rng.html> ·
<https://docs.rs/rand_chacha/0.9.0/src/rand_chacha/chacha.rs.html> ·
<https://docs.rs/rand/0.9.2/rand/seq/trait.SliceRandom.html> ·
<https://docs.rs/rand/0.9.2/rand/seq/index.html> ·
<https://rust-random.github.io/book/crate-reprod.html>

**Windows** —
<https://learn.microsoft.com/en-us/windows/win32/dxtecharts/user-account-control-for-game-developers> ·
<https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid> ·
<https://learn.microsoft.com/en-us/windows/win32/shell/csidl> ·
<https://learn.microsoft.com/en-us/windows/security/application-security/application-control/user-account-control/architecture> ·
<https://learn.microsoft.com/en-us/windows/win32/sysinfo/registry-virtualization> ·
<https://learn.microsoft.com/en-us/previous-versions/windows/it-pro/windows-vista/cc766489(v=ws.10)> ·
<https://learn.microsoft.com/en-us/sharepoint/redirect-known-folders> ·
<https://docs.rs/directories> · <https://docs.rs/dirs> ·
<https://doc.rust-lang.org/std/fs/fn.write.html> · <https://doc.rust-lang.org/std/fs/fn.rename.html>

**Bevy 0.19.1** — <https://docs.rs/bevy_tasks/0.19.1/bevy_tasks/> ·
<https://docs.rs/bevy_tasks/0.19.1/bevy_tasks/futures/fn.check_ready.html> ·
<https://docs.rs/bevy_tasks/0.19.1/bevy_tasks/struct.Task.html> ·
<https://docs.rs/bevy/0.19.1/bevy/tasks/prelude/index.html> ·
<https://docs.rs/bevy_internal/0.19.1/bevy_internal/struct.DefaultPlugins.html> ·
<https://docs.rs/bevy_asset/0.19.1/bevy_asset/io/trait.AssetWriter.html> ·
<https://raw.githubusercontent.com/bevyengine/bevy/v0.19.1/examples/async_tasks/async_compute.rs> ·
<https://bevy.org/examples/async-tasks/async-channel-pattern/>
(note: <https://bevy.org/examples/async-tasks/async-compute/> returns 404)

**This repository** — `engine/Cargo.toml` · `engine/src/state.rs` · `engine/src/ids.rs` ·
`engine/src/orders.rs` · `engine/src/events.rs` · `engine/src/data.rs` · `engine/src/combat.rs` ·
`src/app.rs` · `src/main.rs` · `src/shot.rs` · `assets/data/events.toml` · `Cargo.lock`

**Vendored crate sources read locally** —
`~/.cargo/registry/src/index.crates.io-*/serde_json-1.0.151/src/ser.rs` and `src/error.rs` ·
`~/.cargo/registry/src/index.crates.io-*/bevy_render-0.19.1/src/view/window/screenshot.rs`

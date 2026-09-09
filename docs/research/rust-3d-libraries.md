# Which Rust libraries suit a turn-based game with two 3D views on Windows

Research for [issue #3](https://github.com/whaleyjoshua2/Dying-Earth/issues/3), part of the map in [issue #1](https://github.com/whaleyjoshua2/Dying-Earth/issues/1).

**Researched: 8 September 2026.** Every version number, download count and API claim below was fetched live from a primary source on that date — the crates.io API, the GitHub API, docs.rs for the exact current release, and the projects' own websites and books. Nothing here is from memory. Where a number was measured (migration-guide length, contributor share, example line counts) the method is stated so it can be re-run.

---

## 1. What this game actually needs

Reading the map, the shape of the job is unusual and it changes the answer:

- **The 3D is trivial.** A textured sphere for Earth, and a handful of spheres on circular paths for the Moon and Mars. No characters, no animation, no physics, no terrain, no shadows worth the name. Any of these four libraries can draw this. 3D capability is *not* the deciding factor.
- **The 2D UI is the bulk of the game.** Resource readouts, a build menu, a tech tree, a warming track, turn controls, faction selection. This is where nearly all the code goes.
- **An agent builds it blind.** The map's destination is "a written spec... complete enough that an agent can build it without asking any further questions." Nobody will look at a screenshot and say "the button is off-centre." So the library must fail *loudly* (a compile error) rather than *quietly* (a blank screen, an invisible panel, a button that never highlights).
- **The user has never programmed and will be waiting on builds.** Compile time is a real cost, not a footnote.
- **It must end as a Windows `.exe` a non-programmer can double-click.**

So the ranking criteria, in order of weight: **blind buildability > UI strength > packaging and build time > 3D capability**.

---

## 2. At a glance

| | Bevy | Fyrox | three-d | wgpu + egui (via eframe) |
|---|---|---|---|---|
| Current version (publish date) | **0.19.1** (2026-08-13) | **1.0.1** (2026-03-28) | **0.19.0** (2026-04-17) | wgpu **30.0.1** (2026-08-22), egui/eframe **0.36.2** (2026-09-08) |
| What it is | Full engine (ECS, assets, UI, render) | Full engine + GUI editor | Rendering library only | Raw GPU API + immediate-mode UI toolkit |
| Commits in last 3 months | 586 | 130 | **1** | 549 (wgpu) / 150 (egui) |
| Top contributor's share of commits | 12.1% (cart) | **91.5%** (mrDIMAS) | **96.9%** (asny) | 70.4% (emilk) on egui |
| All-time crates.io downloads | 7,233,869 | **72,063** | 425,600 | 34,430,864 (wgpu) / 23,125,572 (egui) |
| Breaking changes in latest release | **103** migration entries | (1.0 = stability promise, untested) | pre-1.0, "expect regular breaking changes" | 6 (wgpu 30) / 2 (egui 0.36) |
| Widgets that ship, usable for a game | Button marker, Text, ImageNode — that's it | ~38 documented, 7 pages are `TODO` | none (delegates to egui 0.34) | full set + Table + Plot + pan/zoom Scene |
| Cold build | slowest (55 internal crates) | ~5–10 min (their own docs) | light | moderate |
| Single `.exe`? | Only with per-file `embedded_asset!` work | No — `.exe` + `data/` folder | Yes, with `include_bytes!` | **Yes, naturally** |

---

## 3. Bevy

**Version and health.** `bevy` 0.19.1, published 2026-08-13 (crates.io API). Minor releases every 4–5 months without fail for years: 0.19.0 on 2026-06-19, 0.18.0 on 2026-01-13, 0.17.0 on 2025-09-30, 0.16.0 on 2025-04-24, 0.15.0 on 2024-11-29. 48,105 stars, 434 contributors, 586 commits in the three months to 2026-09-08 (GitHub API). The 0.19 release post (bevy.org, 2026-06-19) credits 261 contributors across 1,185 pull requests. This is the healthiest project of the four by a wide margin, and the least dependent on any one person — the top committer holds only 12.1% of commits.

**The churn problem.** I measured the official migration guides in `bevyengine/bevy-website` at `content/learn/migration-guides/` on 2026-09-08:

| Guide | Size | Distinct `###` breaking-change entries |
|---|---|---|
| 0.16 → 0.17 | 129,677 bytes / 2,943 lines | **117** |
| 0.17 → 0.18 | 55,598 bytes / 1,402 lines | **64** |
| 0.18 → 0.19 | 93,276 bytes / 2,343 lines | **103** |

Three consecutive releases with 64–117 separate breaking changes each. 0.19 alone changed the ECS resource model (resources are now components on entities), replaced the render graph with ECS systems, swapped the text engine from `cosmic-text` to `parley`, renamed `bevy_scene` to `bevy_world_serialization`, and introduced a brand-new scene-authoring macro DSL called **BSN**, which now appears in some official examples (`examples/3d/3d_scene.rs`) side by side with the classic `commands.spawn(...)` style still taught in the Quick Start book. Two syntaxes for the same job, in the same example set, two months before this research.

For an agent working blind this is the single biggest hazard on the list. Not because the code won't compile — Rust catches that — but because the agent must *know* to distrust its own recall and read the current docs for every API it touches, and any snippet it half-remembers from a 2024 blog post is now wrong in several load-bearing places.

**3D.** Massively overkill for two spheres, and that's fine — you're paying for it anyway to get the ECS, the asset system and the windowing. `SphereMeshBuilder` with `SphereKind::Uv { sectors, stacks }` gives exactly the equirectangular UV sphere an Earth texture needs (docs.rs/bevy 0.19.1, verified 2026-09-08). The official `examples/3d/texture.rs` is 78 lines end to end; `examples/3d/3d_scene.rs` is 38. Orbiting bodies are parent/child `Transform` hierarchies, covered by `examples/3d/parenting.rs`.

**UI — the weak point.** `bevy_ui` gives a genuine flexbox and CSS-grid layout engine (backed by `taffy`), good text, `Overflow::scroll` plus a `ScrollPosition` component, and styling components (`BackgroundColor`, `BorderRadius`, `BoxShadow`). Flexbox is a real advantage for blind building — "resource bar across the top, build menu down the left" maps cleanly onto `FlexDirection` and `JustifyContent` without pixel-tuning.

But the widget cupboard is nearly bare. I listed every type in `bevy::ui::widget` on docs.rs for 0.19.1 (2026-09-08): `Button` (a *marker struct*, not a working button), `Label`, `Text` and friends, `ImageNode` and friends, `ViewportNode`. **No slider, no checkbox, no dropdown, no table, no scrollbar widget.** The official `examples/ui/widgets/button.rs` is 113 lines to get one button that changes colour on hover and press, because you wire the `Interaction` query yourself.

There *is* a pre-styled widget kit, `bevy::feathers` (buttons, sliders, dropdowns, checkboxes, list views, scrollbars — much expanded in 0.19). Its own documentation on docs.rs for 0.19.1 says, verbatim:

> "While it may be tempting to use this crate for your game's UI, it's deliberately not intended for that."

and

> "this crate is still experimental and unfinished! It will change in breaking ways, and there will be both bugs and limitations."

It is built for Bevy's own future editor. So for a game that is mostly menus, the supported path is hand-assembling every widget from primitives — hundreds of lines of interaction logic that compiles perfectly and fails silently when it's wrong.

**The escape hatch.** `bevy_egui` 0.42.0 (2026-08-16) depends on `bevy_* ^0.19` and `egui ^0.36` (verified via the crates.io dependencies API) — it is fully current, published three days after Bevy 0.19.1. Using Bevy for the 3D and egui for the panels is a real, maintained option, and it removes most of the UI risk. It also means you are carrying Bevy's whole weight and churn to get a sphere renderer, then doing the UI in the other candidate's toolkit anyway.

**Code volume.** Real line counts at tag `v0.19.1`: app skeleton 3 lines, 3D scene 38, textured mesh 78, one interactive button 113, flex layout demo 160. A first playable version of this game plausibly lands at 2,500–6,000 lines, with 60–70% of it `bevy_ui` boilerplate unless `bevy_egui` is used.

**Windows `.exe`.** Not a single file by default: you ship `game.exe` next to an `assets/` folder. Bevy does have a first-party `embedded_asset!` macro (docs.rs, 0.19.1) that bakes a file into the binary via `include_bytes!` and serves it under an `embedded://` path — but it is per-file, aimed at a handful of assets, not a whole folder. A true one-file build means calling it for every texture, or reaching for a third-party crate. The setup guide also warns that `dynamic_linking` (the big dev-build speedup) must be turned **off** for shipping because it "requires you to include `libbevy_dylib` alongside your game."

**Compile times.** Bevy's own setup page (bevy.org, fetched 2026-09-08) is candid: "debug builds in Rust can be _very slow_ - especially when you start using Bevy," and notes it is "not uncommon for debug builds using the default configuration to take multiple minutes." It recommends `opt-level` tuning, the `lld` linker ("much faster at linking than the default Rust linker"), the `dynamic_linking` feature ("the most impactful compilation time decrease"), and cranelift ("about 30% faster at compiling than LLVM", but "when shipping your game, you should still compile it with LLVM"). The workspace at `v0.19.1` contains **55 internal `bevy_*` sub-crates**. This is the heaviest first build of the four by a comfortable margin.

---

## 4. Fyrox

**Version and health.** `fyrox` 1.0.1, published 2026-03-28 (crates.io API) — the first stable release after seven years, announced on fyrox.rs on 2026-03-29. Nothing published in the 5+ months since. 9,548 stars, 130 commits in the three months to 2026-09-08, and weekly commit volume clearly falling since the 1.0 push.

The bus factor is the headline. Of 87 contributors returned by the GitHub API, **mrDIMAS has 7,685 commits (91.5%)**; the next has 198. The 1.0.0 announcement says so itself:

> "the development team of the engine is quite small with little to no funding, so it is impossible to catch all the bugs and polish all the 'rough' parts."

Funding is a Patreon at roughly $21/month across 26 patrons.

**Reach.** 72,063 all-time crates.io downloads, 6,660 in the last 90 days. Bevy has 7.2 million all-time — **a hundredfold difference**. That gap is the whole story for blind buildability: there is proportionally almost no Fyrox code in any model's training data, and no way for the agent to self-correct without a human.

**Editor-centricity.** Fyrox ships FyroxEd, a Unity-style scene editor, and the official tutorials lean on it heavily — the 2D platformer tutorial tells the reader "we don't even need to write a single line of code, we can create a scene entirely in the editor," and a keyword scan of the three official tutorials found 49 explicit instructions to click something in the editor. **An agent cannot use a GUI editor.** The good news is the book is explicit that it is optional — the FAQ says "You can completely ignore the editor if you need, you can even delete it from your project without any consequences," the scene-graph chapter says every node "can be created either in the editor... or programmatically via their respective node builder," and there is a "Manual Engine Initialization" chapter with a working editor-free `main()`. So the code-only path is officially blessed. But the *teaching material* an agent would pattern-match against is the wrong shape.

**UI.** `fyrox-ui` is a retained-mode, message-passing widget tree (WPF-like two-pass layout, no callbacks — you match on `UiMessage` in `Plugin::on_ui_message`). Roughly 38 widgets are documented, including Grid, StackPanel, ScrollViewer, TabControl, **Tree**, ListView, DropdownList, NumericUpDown, ProgressBar — a genuinely richer out-of-the-box set than `bevy_ui`. A simple text button is four lines. But **seven widget documentation pages are literal `TODO` stubs** (Dropdown Menu, Selector, Navigation, Nine Slice, Thumb, Key Binding, Toggle Button), as are sections of the rendering and animation chapters. And UI scenes are fully decoupled from game scenes — no scripts on widgets, so all the sync between game state and screen is hand-written glue.

**Windows `.exe`.** Documented and automatable, but not a single file. `cargo build --package executor --release` plus copying the `data/` folder, or the `export-cli` crate shipped in every 1.0 project (`--target_platform pc`, `--include_used_assets`, `--convert_assets`), which prunes unused assets and converts scenes to a binary format. Output is an executable plus a data folder.

**Compile times.** The book states plainly: first build "may take some time, usually it takes up to 10 minutes on a CPU with 4 cores (8 core CPU will compile the engine in just 5 minutes or so). Next runs... will only compile your game, which usually takes a few seconds." `fyrox-impl` has 47 direct dependencies including both `rapier2d` and `rapier3d` physics engines you will never use.

**Verdict.** Technically capable, honestly documented about its own limits, and a better stock widget set than Bevy. But it is one person's project with $21/month behind it, a hundredth of Bevy's usage, stub documentation in exactly the UI corners this game needs, tutorials that teach a workflow an agent cannot follow, and no turn-based reference architecture anywhere in its examples. **This is the wrong risk to take on a project where nobody can debug the result.**

---

## 5. three-d

**Version and health.** `three-d` 0.19.0, published 2026-04-17 (crates.io API). The previous release, 0.18.2, was 2025-01-30 — **a fifteen-month gap**. Before that, 0.17.0 (2024-02-26) to 0.17.1 (2024-11-25) was nine months. Releases come in bursts with long silences.

More concerning: **one commit in the three months to 2026-09-08**, last push 2026-06-24 (GitHub API). And `asny` holds 3,818 of 3,941 commits — **96.9%**. There is no `CHANGELOG.md` in the repository, and GitHub Releases stopped being used after 0.10.0 in January 2022. The README at tag 0.19.0 is candid: "Most parts are relatively stable, but do expect regular breaking changes until a 1.0.0 release."

425,600 all-time downloads, 1,666 stars. Small but real.

**3D — genuinely excellent for this job.** This is the one place three-d wins outright. `CpuMesh::sphere(16)` is a built-in primitive; `examples/texture` loads and applies image textures; `Camera::new_perspective(...)` and `OrbitControl::new(target, min, max)` are one-liners; there are `AmbientLight`, `DirectionalLight`, `PointLight`, `SpotLight`, a `Skybox`, and a working `picking` example for clicking on objects. Thirty-plus runnable examples cover almost exactly the two scenes this game needs. A textured globe with an orbit camera is a near-verbatim copy of `examples/texture` plus `CpuMesh::sphere()`.

**UI — the disqualifier.** three-d has no UI of its own. It offers an optional `egui-gui` feature that wires up an egui context, and the great majority of its examples require it. But I read `Cargo.toml` at tag 0.19.0 on 2026-09-08:

```toml
egui = "0.34"
egui_glow = "0.34"
winit = "0.28"
glutin = "0.30"
```

egui is at **0.36.2** as of today; winit stable is **0.30.13**. three-d is two egui minor versions and two winit minor versions behind, and the unreleased `master` branch still pins egui 0.34. Given a fifteen-month release gap in its recent history, "we'll catch up next release" is not a safe assumption. Being stuck on an old egui means the agent must write egui code for a version it will not find in current documentation — precisely the failure mode we are trying to eliminate.

three-d also renders through **OpenGL** (`glow` + `glutin`), not wgpu — fine on Windows, but a different, older graphics path than everything else here.

**Code volume.** No ECS, no scene graph beyond a flat list of geometry+material objects you transform yourself, no game loop scaffolding, no state machine, no asset pipeline, no save/load. All of that is on you. So is every pixel of UI, in an out-of-date egui.

**Windows `.exe`.** Good: a normal `cargo build --release` produces a single native exe linking system OpenGL. Assets can be embedded — `three_d_asset::io::RawAssets::new()` + `insert()` + `deserialize()` accepts `include_bytes!` data with no runtime file I/O. But every official example loads from disk paths, so an agent copying examples verbatim will produce an exe that expects an `assets/` folder unless explicitly told otherwise.

**Compile times.** The lightest of the four. Core deps are `glow`, `cgmath`, `glutin`, `winit`, `image`, optionally `egui`. Nothing like Bevy's 55 sub-crates or wgpu's shader-translation stack.

**Verdict.** The best 3D-per-line-of-code of the four, attached to the weakest maintenance signal (one commit in three months, 97% one author, no changelog) and a stale UI pin. For a game that is 80% UI, the stale egui pin alone rules it out of the top two.

---

## 6. wgpu + egui (in practice: eframe)

This is really two separate decisions, and they have opposite answers.

### egui — the strongest UI story here

`egui` and `eframe` are both at **0.36.2**, published **2026-09-08** — the same day this research was done. Cadence over the last year: 0.33.x (Nov–Dec 2025), 0.34.0 (2026-03-26), 0.35.0 (2026-06-25), 0.36.0 (2026-08-05), patches since. 30,478 stars, 456 contributors, 150 commits in the last three months, 23.1 million all-time downloads. `emilk` holds 70.4% of commits — high, but this is a far broader project than Fyrox or three-d, and it is not unfunded: the README states "egui development is sponsored by Rerun, a startup building an SDK for visualizing streams of multimodal data." The README is also honest about immediate mode's one real weakness — "we must decide where to show the window *before* we know its size" — which matters for auto-sizing floating windows, not for the docked panels and grids this game needs.

Breaking changes are modest: the CHANGELOG marks **2** breaking entries in 0.36.0 and **0** in 0.35.0 — against Bevy's 103 for the same period.

What ships, verified on docs.rs for 0.36.2 (2026-09-08):

- `egui::widgets`: Button, Checkbox, DragValue, Hyperlink, Image, Label, Link, **ProgressBar**, RadioButton, Separator, Slider, Spinner.
- `egui::containers`: Area, Window, Popup, Resize, Sides, Tooltip, ComboBox, plus panels, `ScrollArea` and `CollapsingHeader` re-exported from submodules.
- `egui::Grid` — "A simple grid layout. The cells are always laid out left to right, top-down" — exactly a resource readout.
- `egui::containers::Scene` — "A container that allows you to zoom and pan"; like a `ScrollArea` but with zooming and no limits. This is the *canvas* a tech tree needs, already written — the node placement and the lines between nodes are still yours to write, but they are ordinary arithmetic, not graphics code. No node-graph widget ships with egui; the tech tree is the one UI element here without an off-the-shelf answer.
- `egui_extras` 0.36.2 (depends on `egui ^0.36.2`) adds a real `Table`.
- `egui_plot` 0.37.0 (depends on `egui ^0.36.0`) adds charts — a warming track over ten turns.

Layout is immediate-mode and automatic: you say `ui.horizontal(|ui| ...)`, `ui.vertical(...)`, `ui.add(Slider::new(...))` and egui measures and places things. **There is no pixel positioning to get wrong.** For an agent building without a human looking at the screen, this is the single most valuable property on this page — the failure mode "it compiled but the panel is invisible / off-screen / overlapping" largely does not exist, because you never specify coordinates.

egui also bundles its own fonts by default (`default_fonts` feature, via `include_bytes!`), so text works with no external files.

### wgpu — the strongest reason for caution

`wgpu` 30.0.1, published 2026-08-22. It is extremely healthy — 17,965 stars, 549 commits in three months, 34.4 million downloads, and its README states it "serves as the core of the WebGPU integration in Firefox, Servo, and Deno."

It is also a **raw GPU API**, and it moves fast *by policy*. The README says so outright: "we release a breaking version every three months." The record bears it out: 27.0.0 (2025-10-01), 28.0.0 (2025-12-18), 29.0.0 (2026-03-19), 30.0.0 (2026-07-01) — **four breaking major versions in ten months**, with 8 major changes listed in 29.0.0 and 6 in 30.0.0. Representative breaks: "`SurfaceTexture::present()` has been replaced by `Queue::present(surface_texture)`"; `dispatch`/`dispatch_indirect` renamed to `dispatch_workgroups`; `Surface::get_current_texture` changed from returning a `Result` to a new `CurrentSurfaceTexture` enum with six variants you must match on. The churn even reaches the shader source: in 30.0.0, "Integer shader I/O no longer defaults to `@interpolate(flat)`" — WGSL that was valid last release must now annotate those fields explicitly.

wgpu has no meshes, no cameras, no lights, no scene graph, no model loading and no maths types. To draw one lit textured sphere by hand you must write: a WGSL shader; vertex and index buffers with generated sphere geometry; a uniform buffer and bind group layout and bind group; a depth texture; a render pipeline; surface configuration and resize handling; and the view/projection matrices yourself with `glam` (0.33.7, 2026-09-07) or `nalgebra`.

For scale, three line counts I measured directly on 2026-09-08:

- `emilk/egui` → `crates/egui_demo_app/src/apps/custom3d_wgpu.rs`, the official "custom wgpu 3D inside an egui app" example: **212 lines**, plus a separate `custom3d_wgpu_shader.wgsl` — **to draw one triangle**.
- `gfx-rs/wgpu` → `examples/features/src/hello_triangle/mod.rs`: **355 lines** for a single hard-coded triangle with no vertex buffer at all.
- `gfx-rs/wgpu` → `examples/features/src/cube/mod.rs`: **418 lines** for an indexed, textured, spinning cube with a uniform buffer and no lighting.

A textured, lit, orbiting sphere with a generated mesh is more than the cube. Two such scenes, each wired into an egui callback, is a realistic **400–700 lines of graphics plumbing per scene** before a single pixel of UI.

And these are exactly the errors that fail *silently*: a wrong matrix multiplication order, a bind group index off by one, a mis-declared vertex attribute. The result is a black screen with no error, and no human to notice.

### How they combine

The mechanism is real and current: `egui_wgpu::CallbackTrait` (docs.rs 0.36.2, verified 2026-09-08) — "a callback trait that can be used to compose an `epaint::PaintCallback` via `Callback` for custom WGPU rendering," with `prepare`, `finish_prepare` and `paint` methods, letting custom 3D draw inside an egui layout. `eframe` 0.36.2 depends on `wgpu ^30.0` and `winit ^0.30.13`, with wgpu as the **default** renderer.

Note that the repository's `examples/` folder currently ships `custom_3d_glow` but **not** a `custom_3d_wgpu` — the wgpu version lives in the demo app at the path above. An agent told to "copy the eframe custom3d_wgpu example" needs that exact path or it will not find it.

### Honest note on egui's own churn

egui is not frictionless either. `eframe` 0.34.0 (2026-03-26) **"Replace `App::update` with `fn logic` and `fn ui`"** (CHANGELOG, verified 2026-09-08). The `eframe` hello-world on docs.rs for 0.36.2 now reads:

```rust
impl eframe::App for MyEguiApp {
   fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) { ... }
}
```

Any agent recalling the old `fn update(&mut self, ctx: &egui::Context, frame: &mut Frame)` will write code that does not compile. That is the *good* kind of failure — loud, immediate, fixable — but it confirms the general rule: **the agent must read the current docs for the pinned version, for every library on this list, without exception.**

### Windows `.exe` and build weight

`cargo build --release` produces a single self-contained `.exe`. Fonts are baked in by the `default_fonts` feature (the `epaint_default_fonts` crate `include_bytes!`s `Hack-Regular.ttf`, `Ubuntu-Light.ttf` and the emoji/icon fonts), and textures can be baked in with `include_bytes!` the same way the official `eframe_template` embeds its window icon. Nothing needs to sit beside the exe. This is the only candidate that gives a genuine one-file, double-clickable result with no extra work.

The console window is handled by a standard Rust attribute — `#![windows_subsystem = "windows"]`, which the Rust reference describes as running "detached from any existing console... commonly used by GUI applications that do not want to display a console window on startup." The official `eframe_template` already carries it as `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]`, so it is on by default in release builds and off in debug where you still want to see panics.

The docs note that switching from wgpu to glow "can significantly reduce your binary size" if that matters later.

**One packaging trap to write into the spec:** crates.io currently reports winit's *newest* version as `0.31.0-beta.3` (2026-09-04), while the true stable is `0.30.13` (2026-03-02) — which is what `eframe` 0.36.2 and `egui-wgpu` 0.36.2 actually depend on (`winit ^0.30.13`). Any instruction to "use the latest winit" would land the agent on pre-release code. Pin exact versions of `wgpu`, `egui`, `eframe` and `winit` in `Cargo.toml` rather than leaving loose ranges.

Dependency weight sits between three-d and Bevy — wgpu drags in the `naga` shader translator, but there is no ECS, no asset server, no audio, no physics.

---

## 7. Others considered

**macroquad** 0.4.16 (2026-07-30). Genuinely interesting on two axes: its README claims a clean build "takes only 16s on x230 (~6 years old laptop)" and "on windows both MSVC and GNU target are supported, no additional dependencies required" — by far the fastest builds here. And it *does* have 3D: `macroquad::models` (docs.rs, 0.4.16) provides `draw_sphere`, `draw_cube`, `draw_mesh`, `Mesh`, plus a `Camera3D`. But its UI (`macroquad::ui::widgets`) ships Button, Checkbox, ComboBox, Editbox, Group, InputText, Label, Popup, ProgressBar, Slider, Tabbar, Texture, TreeNode, Window — **no table, no scroll area**, and a much thinner text and layout story than egui. For a game that is mostly dense numeric panels, that's a real ceiling. Worth remembering if build times ever become intolerable.

**godot-rust (`godot` 0.5.5, 2026-08-09)** is alive and well-maintained (5,173 stars, 137 commits in three months). Rejected because it inverts the premise: it requires the Godot 4 editor installed, export templates downloaded separately, and produces an `.exe` plus a `.pck` via an editor export step. Godot's own docs describe the workflow through the editor GUI (a command-line `--export-release` exists). It is a good choice for a human with a mouse; it is a poor choice for an agent building blind from a spec, and it stretches "built in Rust" past what the map settled.

**rend3** — dead. Last release 0.3.0 on 2022-02-12, 484 downloads in 90 days. **renderling** — last updated 2024-09-20, 161 downloads in 90 days. Neither is a live option, and there is no maintained middle layer that gives 3D helpers on top of wgpu without taking a whole engine.

**kiss3d** 0.46.0 is oddly active again (many releases through 2026) but is a minimal scientific-visualisation viewer with no UI story for a game like this.

---

## 8. The decisive criterion: building blind

Ranked by how likely an agent is to produce something that both compiles *and* looks right, first time, with nobody checking:

1. **egui** — best. Immediate-mode auto-layout means there are no coordinates to get wrong; widgets for every element this game needs already exist (Grid for readouts, `Scene` for a pan/zoom tech tree, `egui_extras::Table` for build menus, `egui_plot` for the warming track); the API is stable release to release (2 breaking changes in the latest); and there is a very large public corpus of egui code.
2. **Bevy's flexbox layout** — second. Flexbox degrades gracefully; "a row across the top" is hard to get catastrophically wrong. Undercut badly by having no widgets: every button, list row and tech-tree node is hand-wired `Interaction` logic that compiles fine and misbehaves quietly, and by 103 breaking changes two months ago.
3. **Fyrox's fyrox-ui** — third. Richer widget set than bevy_ui, code-first builders, but a hundredfold less training data than Bevy, `TODO` stubs on several widget pages, and tutorials that teach an editor workflow an agent can't use.
4. **Hand-written wgpu 3D** — worst, and it is worth being blunt. Matrix, bind-group and shader errors produce a black screen and no diagnostic. This is the one place in this whole survey where an agent can be confidently, silently wrong.

That last point is the crux of the recommendation: **egui is the best blind bet for the 90% of this game that is UI, and hand-written wgpu is the worst blind bet for the 10% that is 3D.** The winning move is to take egui's UI and get the 3D from something that already has spheres, cameras and lights.

---

## 9. Toolchain reality on this machine

Checked on this PC, 2026-09-08:

- **Rust is not installed** (`rustc` not found). One install of `rustup` from rustup.rs is required.
- **Visual Studio Build Tools 2022 is already installed**, with the MSVC C++ toolchain (`v14.44.35207`) and Windows SDK `10.0.26100.0` present. This is the prerequisite that usually trips people up on Windows, and it is already satisfied. Bevy's setup guide, Rust's installer and every candidate here need it for the linker.

So the setup cost for the user is one download, whichever library is chosen. The ongoing cost is waiting on builds, and that is where the candidates differ sharply: the first build is minutes-to-tens-of-minutes for Bevy, roughly 5–10 minutes by Fyrox's own estimate, and noticeably less for an eframe/wgpu app or three-d. Every candidate is fast on incremental rebuilds once the first build is done.

---

## 10. Recommendation: two candidates for the prototype

### A. Bevy 0.19 with `bevy_egui` for the UI

Bevy draws the globe and the solar system with built-in sphere meshes and parent/child orbits; `bevy_egui` 0.42 (current, tracks Bevy 0.19 and egui 0.36) draws every panel, menu, tree and readout. Bevy supplies the game skeleton for free — an ECS to hold bodies, resources and techs, a `States` system for turn phases, an asset loader, windowing and input.

### B. eframe (egui + wgpu) with a hand-written 3D view

egui is the whole application; the two 3D scenes are drawn into it through `egui_wgpu::CallbackTrait`. Everything the game does other than the two globes is ordinary Rust structs and egui calls. Produces one self-contained `.exe` with no folders beside it.

### The trade-off, in plain language

**Both choices give you the same UI toolkit — egui. The difference is who draws the planets.**

**Bevy already knows what a sphere is.** You ask for a sphere, a texture and a light, and you get a globe in about forty lines. You also inherit a large, professionally maintained game engine that handles turns, data tables, saving and input for you. The price is weight and motion: it is the slowest thing here to compile — expect the first build to be a genuinely long wait — the finished game ships as an `.exe` next to an `assets` folder rather than one tidy file unless extra work is done, and the engine changed 103 things in its last release, so the agent has to be disciplined about reading current documentation rather than trusting what it thinks it knows.

**eframe already knows what a menu is, and nothing else.** It is small, it compiles quickly, and it produces exactly one file you can double-click and email to someone. Its UI toolkit is the best fit on this list for a game made of numbers and buttons — it even ships a zoomable canvas that is essentially a tech tree waiting to happen. The price is the planets: nobody has written the sphere for you. The agent must hand-write graphics code — geometry, shaders, matrices — and that is the one kind of code that can be wrong without saying so. A wrong matrix gives you a black screen, not an error message, and there is no human to notice.

**Put crudely:** Bevy risks a long wait and a messy folder; eframe risks a black screen. Bevy is heavy but the 3D is solved; eframe is light and tidy but the 3D is homework.

### What the prototype ticket (#5) should actually settle

Build the same tiny thing twice, once per candidate, and time it honestly:

1. A textured sphere that rotates, with a camera you can orbit.
2. A second view with two smaller spheres on circular paths, and a key or button to switch between the views.
3. An egui-style panel over the top: three numbers that change, a build menu with three buttons, and a "next turn" counter.
4. `cargo build --release`, then confirm the result runs by double-clicking it on a Windows machine — and record exactly what has to travel with it.

Record for each: cold build time, warm build time, total lines written, how many attempts it took to get the 3D on screen, and how many files the shipped thing is. Those five numbers decide it. The blind-build criterion is only really testable by *doing it blind*, so the prototype should be built from a written brief without anyone looking at intermediate screenshots until the end.

---

## Sources

All fetched 2026-09-08 unless noted.

**Versions and download counts** — crates.io API: `/api/v1/crates/{bevy,fyrox,three-d,wgpu,egui,eframe,egui-wgpu,winit,bevy_egui,macroquad,rend3,godot,kiss3d,glam,renderling,egui_plot,egui_extras}` and `/api/v1/crates/{crate}/{version}/dependencies`.

**Repository activity, contributors, commit counts** — GitHub API: `repos/bevyengine/bevy`, `repos/FyroxEngine/Fyrox`, `repos/asny/three-d`, `repos/gfx-rs/wgpu`, `repos/emilk/egui`, `repos/vladbat00/bevy_egui`, `repos/godot-rust/gdext`, with `/commits?since=2026-06-08` and `/contributors`.

**Bevy** — bevy.org release post for 0.19 (2026-06-19); bevy.org Quick Start setup page; migration guides at `bevyengine/bevy-website` `content/learn/migration-guides/0.16-to-0.17.md`, `0.17-to-0.18.md`, `0.18-to-0.19.md` (sizes measured directly from the GitHub contents API); docs.rs `bevy` 0.19.1 for `ui::widget`, `ui::ScrollPosition`, `feathers`, `asset::embedded_asset!`, `mesh::SphereMeshBuilder`; examples at tag `v0.19.1`.

**Fyrox** — fyrox.rs 1.0.0 release post (2026-03-29); the Fyrox Book (fyrox-book.github.io) chapters on basic concepts, FAQ, manual engine initialization, user interface, widgets, shipping, installation; `FyroxEngine/Fyrox-tutorials`.

**three-d** — `asny/three-d` README and `Cargo.toml` at tag `0.19.0`; `examples/` directory listing and `examples/texture`, `examples/shapes`, `examples/environment`, `examples/picking` sources; docs.rs `three_d` and `three_d_asset::io`.

**wgpu + egui** — `gfx-rs/wgpu` README and CHANGELOG on `trunk`, and `examples/features/src/hello_triangle/mod.rs` and `examples/features/src/cube/mod.rs` (line counts measured directly); `emilk/egui` README, CHANGELOG and `crates/eframe/CHANGELOG.md` on `main`; `crates/egui_demo_app/src/apps/custom3d_wgpu.rs` (212 lines, measured); `emilk/eframe_template` `src/main.rs`; docs.rs `egui` 0.36.2 (`widgets`, `containers`, `Grid`, `Scene`), `eframe` 0.36.2, `egui-wgpu` 0.36.2 (`CallbackTrait`); the Rust Reference, "Runtime", on `windows_subsystem`.

**Others** — `not-fl3/macroquad` README; docs.rs `macroquad` 0.4.16 (`models`, `ui::widgets`); godot-rust book setup page; Godot docs "Exporting projects".

**This machine** — `rustc` absence, `vswhere` output, MSVC and Windows SDK directory listings, checked locally 2026-09-08.

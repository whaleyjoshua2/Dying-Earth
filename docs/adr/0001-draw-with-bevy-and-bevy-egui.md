---
status: accepted
date: 2026-09-08
---

# Draw the game with Bevy and bevy_egui

The First Playable needs two 3D views (an Earth globe and a solar system) under a game that is mostly panels, menus and numbers, built by an agent from a written spec with nobody watching the screen. We draw the 3D with **Bevy 0.19.1** and every panel with **bevy_egui 0.42.0** (egui 0.36.2), with exact versions pinned, because the design keeps adding 3D that must be right the first time (a clickable globe with real continents, ships in transit, combat) and Bevy already supplies picking, lights, materials, parenting and cameras.

## Considered options

- **eframe/egui with hand-written wgpu 30**: lighter, a two-minute first build against Bevy's seven, and one tidy `.exe`. Rejected because every piece of 3D is hand-written graphics that fails silently (a black screen rather than an error), and the amount of 3D in this game only grows. Both candidates were prototyped and run; the pictures and measurements are in `docs/dev-diary/2026-09-08-3d-prototypes/`.
- **three-d, Fyrox**: dropped at research time on maintenance risk (one commit in three months and a stale egui pin; one maintainer and an editor-centred workflow).

## Consequences

- Pin `bevy`, `bevy_egui` and `egui` to exact versions. Both churn between releases; the building agent reads the docs for the pinned versions, never the latest.
- Budget a seven-minute first build and a blank window of roughly ten seconds on the first-ever launch while shaders compile.
- The shipped game needs a genuine Earth texture as an asset (embedded or beside the `.exe`); the prototype's generated blobs are placeholder.
- Bevy's UV sphere has its poles on Z: rotate the globe -90 degrees about X or the ice cap faces the camera.
- The game ships with a headless screenshot mode (off-screen window, Bevy `Screenshot` + `save_to_disk`), so an agent can verify its own pictures.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//! Dying Earth, the First Playable. Modes (spec 2.5, 3, 19.3):
//!   dying-earth.exe                     play
//!   dying-earth.exe seed:<n>            play with a fixed seed
//!   dying-earth.exe shot:<prefix>       headless screenshots of the four views, then exit
//!   dying-earth.exe window:<w>x<h>      the off-screen window's size, for photographing tall panels
//!   dying-earth.exe savedir:<path>      (ticket #59) saves go here instead of the local app-data folder
//!   dying-earth.exe simulate:<seed> [--player=<faction>]   all four seats on the AI, headless log

mod app;
mod geo;
mod icons;
mod saves;
mod scene;
mod shot;
mod textures;
mod ui;

use app::*;
use bevy::prelude::*;
use bevy::window::WindowPosition;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use dying_earth_engine::data::{default_data_dir, Tables};
use dying_earth_engine::*;
use std::sync::Arc;

fn assets_root() -> std::path::PathBuf {
    default_data_dir().parent().map(|p| p.to_path_buf()).unwrap_or_else(|| "assets".into())
}

fn simulate(seed: u64, args: &[String], tables: Arc<Tables>) -> i32 {
    // Ticket #50: every game seats all four Factions; --player says which one sits in seat 0.
    let player = match args.iter().find_map(|a| a.strip_prefix("--player=")) {
        None => FactionKind::Custodians,
        Some(name) => match FactionKind::from_id(name) {
            Some(k) => k,
            None => {
                eprintln!("unknown Faction {name:?}: use custodians, prospectors, arkwrights or archivists");
                return 2;
            }
        },
    };
    let result = dying_earth_engine::sim::run(tables, seed, player);
    let text = result.log.join("\n");
    println!("{text}");
    let path = format!("simulate-{seed}.log");
    if let Err(e) = std::fs::write(&path, text) {
        eprintln!("could not write {path}: {e}");
    }
    0
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let tables = match Tables::load(&default_data_dir()) {
        Ok(t) => Arc::new(t),
        Err(e) => {
            eprintln!("Dying Earth cannot start: data table error in {e}");
            std::process::exit(2);
        }
    };
    if let Some(seed) = args.iter().find_map(|a| a.strip_prefix("simulate:")) {
        let seed: u64 = seed.parse().unwrap_or(1);
        std::process::exit(simulate(seed, &args, tables));
    }
    let seed: u64 = args.iter().find_map(|a| a.strip_prefix("seed:")).and_then(|s| s.parse().ok()).unwrap_or_else(|| {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(1)
    });
    let shot = args.iter().find_map(|a| a.strip_prefix("shot:").map(str::to_owned));
    let textures = match Textures::load(&assets_root().join("textures")) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Dying Earth cannot start: texture error: {e}");
            std::process::exit(2);
        }
    };
    let mode = if shot.is_some() { Mode::Shot } else { Mode::Play };
    let position = if mode == Mode::Shot { WindowPosition::At(IVec2::new(-5000, -5000)) } else { WindowPosition::Automatic };
    // `window:<w>x<h>` (a building aid, not part of the spec): a taller off-screen window, so a
    // picture can be taken of a panel longer than the default 800 rows. The Climate Panel scrolls,
    // and a scrolled panel cannot be scrolled headlessly, so without this its lower half -- the
    // Blame block among it -- could only be checked by reading the code.
    let (width, height) = std::env::args()
        .find_map(|a| {
            let (w, h) = a.strip_prefix("window:")?.split_once('x')?;
            Some((w.parse().ok()?, h.parse().ok()?))
        })
        .unwrap_or((1280, 800));
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { title: "Dying Earth".into(), resolution: (width, height).into(), position, ..default() }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.05)))
        .insert_resource(Session {
            tables,
            game: None,
            pending: Vec::new(),
            screen: Screen::Title,
            seed,
            mode,
            shot_prefix: shot.unwrap_or_default(),
            earth_dirty: false,
            last_error: None,
            refusal: None,
            spectator: false,
            auto: false,
            auto_elapsed: 0.0,
            // Ticket #59: where this machine keeps the saves. A machine that will not say keeps
            // the reason, and the interface shows it rather than failing quietly.
            saves: saves::saves_dir(),
            save_notice: None,
            saves_list: Vec::new(),
            confirm_delete: None,
        })
        .insert_resource(textures)
        .insert_resource(ViewState::default())
        .insert_resource(shot::ShotPlan::default())
        // Ticket #109: the resource icons, rendered from SVG on the first frame that draws them.
        .insert_resource(icons::Icons::default())
        .add_systems(Startup, scene::setup_scene)
        .add_systems(Update, (ui::keyboard, ui::recompose_earth, scene::sync_scene, shot::shot_system).chain())
        .add_systems(EguiPrimaryContextPass, ui::draw)
        .run();
}

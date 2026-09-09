#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// PROTOTYPE - throwaway. Candidate A for wayfinder ticket #5:
// Bevy draws the 3D, bevy_egui draws the HUD.
//
// Same content as the eframe candidate, on purpose, so the comparison is fair:
// an Earth globe with three tinted nation states, a solar system with the
// Sun, Earth, Moon, Mars and a ship in transit, and one HUD panel.
// Drag in the 3D area to rotate. "End turn" advances the toy state.

mod texgen;

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::window::WindowPosition;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
use std::f32::consts::FRAC_PI_2;

// ---------------------------------------------------------------- game state

#[derive(Clone, Copy, PartialEq)]
enum View {
    Earth,
    Solar,
}

#[derive(Resource)]
struct Game {
    turn: u32,
    materials: i32,
    fuel: i32,
    energy: i32,
    warming: f32,
    view: View,
    ship_progress: f32,
    yaw: f32,
    last_action: String,
}

impl Game {
    fn new() -> Self {
        Self {
            turn: 1,
            materials: 40,
            fuel: 20,
            energy: 30,
            warming: 12.0,
            view: if std::env::args().any(|a| a == "solar") { View::Solar } else { View::Earth },
            ship_progress: 0.0,
            yaw: 0.0,
            last_action: "Game start. Ship launched from Earth toward Mars.".into(),
        }
    }

    fn end_turn(&mut self) {
        if self.turn >= 10 {
            self.last_action = "Turn 10 reached. Game over.".into();
            return;
        }
        self.turn += 1;
        self.materials += 8;
        self.fuel += 3;
        self.energy += 1;
        self.warming += 3.5;
        self.ship_progress = (self.ship_progress + 0.25).min(1.0);
        self.last_action = format!(
            "Turn {}: +8 Materials, +3 Fuel, +1 Energy net. Warming +3.5%. Ship {}.",
            self.turn,
            if self.ship_progress >= 1.0 { "arrived at Mars" } else { "advanced one leg" }
        );
    }

    fn ship_turns_left(&self) -> u32 {
        ((1.0 - self.ship_progress) / 0.25).ceil() as u32
    }
}

fn body_angles(turn: u32) -> (f32, f32, f32) {
    let t = turn as f32;
    (t * 0.35, 1.2 + t * 0.19, t * 1.3) // earth, mars, moon
}

// ---------------------------------------------------------------- markers

#[derive(Component)]
struct EarthRoot;
#[derive(Component)]
struct SolarRoot;
#[derive(Component)]
struct EarthGlobe;
#[derive(Component)]
struct Planet {
    radius: f32,
    which: u8, // 0 earth, 1 mars
}
#[derive(Component)]
struct Moon;
#[derive(Component)]
struct Ship;
#[derive(Component)]
struct MainCamera;

/// Headless screenshot mode: `shot:<prefix>` on the command line puts the
/// window off-screen, saves <prefix>-5s.png and <prefix>-12s.png, then exits.
#[derive(Resource)]
struct Shot {
    prefix: Option<String>,
    taken: [bool; 2],
}

fn shot_arg() -> Option<String> {
    std::env::args().find_map(|a| a.strip_prefix("shot:").map(str::to_owned))
}

// ---------------------------------------------------------------- app

fn main() {
    let shot = shot_arg();
    let position = if shot.is_some() { WindowPosition::At(IVec2::new(-5000, -5000)) } else { WindowPosition::Automatic };
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "PROTOTYPE bevy-views (Bevy 0.19 + bevy_egui 0.42)".into(),
                resolution: (1100, 700).into(),
                position,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .insert_resource(Game::new())
        .insert_resource(Shot { prefix: shot, taken: [false, false] })
        .add_systems(Startup, setup)
        .add_systems(Update, (spin_globe, place_planets, place_moon, place_ship, apply_view, shot_system))
        .add_systems(EguiPrimaryContextPass, hud)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    // Earth texture, generated in code.
    let earth_tex = images.add(Image::new(
        Extent3d { width: texgen::W, height: texgen::H, depth_or_array_layers: 1 },
        TextureDimension::D2,
        texgen::earth_rgba(),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    ));
    let sphere = meshes.add(Sphere::new(1.0).mesh().uv(48, 24));
    let earth_mat = materials.add(StandardMaterial {
        base_color_texture: Some(earth_tex),
        perceptual_roughness: 1.0,
        reflectance: 0.0,
        ..default()
    });
    let mut flat = |c: Color, unlit: bool| {
        materials.add(StandardMaterial {
            base_color: c,
            unlit,
            double_sided: true,
            cull_mode: None,
            ..default()
        })
    };
    let sun_mat = flat(Color::srgb(1.0, 0.85, 0.3), true);
    let moon_mat = flat(Color::srgb(0.7, 0.7, 0.7), false);
    let mars_mat = flat(Color::srgb(0.85, 0.35, 0.2), false);
    let ship_mat = flat(Color::srgb(1.0, 1.0, 1.0), true);
    let ring_mat = flat(Color::srgb(0.45, 0.45, 0.5), true);
    let ring3 = meshes.add(Annulus::new(2.97, 3.03));
    let ring5 = meshes.add(Annulus::new(4.97, 5.03));
    let flat_ring = Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2));

    // Earth view
    commands.spawn((
        Transform::default(),
        Visibility::default(),
        EarthRoot,
        children![(
            Mesh3d(sphere.clone()),
            MeshMaterial3d(earth_mat.clone()),
            Transform::from_scale(Vec3::splat(1.8)),
            EarthGlobe,
        )],
    ));

    // Solar system view
    commands.spawn((
        Transform::default(),
        Visibility::Hidden,
        SolarRoot,
        children![
            (Mesh3d(sphere.clone()), MeshMaterial3d(sun_mat), Transform::from_scale(Vec3::splat(0.9))),
            (Mesh3d(ring3), MeshMaterial3d(ring_mat.clone()), flat_ring),
            (Mesh3d(ring5), MeshMaterial3d(ring_mat), flat_ring),
            (
                Mesh3d(sphere.clone()),
                MeshMaterial3d(earth_mat),
                Transform::from_scale(Vec3::splat(0.35)),
                Planet { radius: 3.0, which: 0 },
                children![(
                    Mesh3d(sphere.clone()),
                    MeshMaterial3d(moon_mat),
                    // child of a 0.35-scaled parent: local units are parent units
                    Transform::from_scale(Vec3::splat(0.1 / 0.35)),
                    Moon,
                )],
            ),
            (
                Mesh3d(sphere.clone()),
                MeshMaterial3d(mars_mat),
                Transform::from_scale(Vec3::splat(0.25)),
                Planet { radius: 5.0, which: 1 },
            ),
            (Mesh3d(sphere), MeshMaterial3d(ship_mat), Transform::from_scale(Vec3::splat(0.07)), Ship),
        ],
    ));

    commands.spawn((
        DirectionalLight { illuminance: 2500.0, ..default() },
        Transform::from_xyz(4.0, 5.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.2, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        MainCamera,
    ));
}

// ---------------------------------------------------------------- systems

fn spin_globe(game: Res<Game>, mut q: Query<&mut Transform, With<EarthGlobe>>) {
    for mut t in &mut q {
        // Bevy's UV sphere has its poles on the Z axis; stand it upright first.
        t.rotation = Quat::from_rotation_y(game.yaw) * Quat::from_rotation_x(-FRAC_PI_2);
    }
}

fn place_planets(game: Res<Game>, mut q: Query<(&Planet, &mut Transform)>) {
    let (ea, ma, _) = body_angles(game.turn);
    for (p, mut t) in &mut q {
        let a = if p.which == 0 { ea } else { ma };
        t.translation = Vec3::new(a.cos() * p.radius, 0.0, a.sin() * p.radius);
        if p.which == 0 {
            t.rotation = Quat::from_rotation_x(-FRAC_PI_2); // upright, see spin_globe
        }
    }
}

fn place_moon(game: Res<Game>, mut q: Query<&mut Transform, With<Moon>>) {
    let (_, _, mo) = body_angles(game.turn);
    for mut t in &mut q {
        // local to the Earth entity, which is scaled 0.35
        let r = 0.6 / 0.35;
        // the Earth parent is rotated -90 deg about X to stand upright; undo that here
        t.translation = Quat::from_rotation_x(FRAC_PI_2) * Vec3::new(mo.cos() * r, 0.0, mo.sin() * r);
    }
}

fn place_ship(game: Res<Game>, mut q: Query<&mut Transform, With<Ship>>) {
    let (ea, ma, _) = body_angles(game.turn);
    let earth = Vec3::new(ea.cos() * 3.0, 0.0, ea.sin() * 3.0);
    let mars = Vec3::new(ma.cos() * 5.0, 0.0, ma.sin() * 5.0);
    for mut t in &mut q {
        t.translation = earth.lerp(mars, game.ship_progress) + Vec3::new(0.0, 0.15, 0.0);
    }
}

fn apply_view(
    game: Res<Game>,
    mut roots: Query<(&mut Visibility, Option<&EarthRoot>), Or<(With<EarthRoot>, With<SolarRoot>)>>,
    mut cam: Query<&mut Transform, With<MainCamera>>,
) {
    for (mut vis, earth) in &mut roots {
        let show = (game.view == View::Earth) == earth.is_some();
        *vis = if show { Visibility::Inherited } else { Visibility::Hidden };
    }
    for mut t in &mut cam {
        *t = match game.view {
            View::Earth => Transform::from_xyz(0.0, 1.2, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            View::Solar => {
                let eye = Quat::from_rotation_y(game.yaw) * Vec3::new(0.0, 9.0, 11.0);
                Transform::from_translation(eye).looking_at(Vec3::ZERO, Vec3::Y)
            }
        };
    }
}

fn shot_system(time: Res<Time>, mut shot: ResMut<Shot>, mut commands: Commands, mut exit: MessageWriter<AppExit>) {
    let Some(prefix) = shot.prefix.clone() else { return };
    let t = time.elapsed_secs();
    for (i, at) in [5.0_f32, 12.0].iter().enumerate() {
        if t > *at && !shot.taken[i] {
            shot.taken[i] = true;
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(format!("{prefix}-{}s.png", *at as u32)));
        }
    }
    if t > 14.0 {
        exit.write(AppExit::Success);
    }
}

fn hud(mut contexts: EguiContexts, mut game: ResMut<Game>) -> Result {
    let ctx = contexts.ctx_mut()?;
    let mut root = egui::Ui::new(
        ctx.clone(),
        "viewport".into(),
        egui::UiBuilder::new()
            .layer_id(egui::LayerId::background())
            .max_rect(ctx.viewport_rect()),
    );
    egui::Panel::left("hud").default_size(280.0).show(&mut root, |ui| {
        ui.heading("PROTOTYPE: Bevy + bevy_egui");
        ui.label("Candidate A. Bevy draws the 3D, egui draws this panel.");
        ui.separator();
        ui.label(format!("Turn {} / 10", game.turn));
        ui.separator();
        ui.label("Stockpile");
        egui::Grid::new("stock").show(ui, |ui| {
            ui.label("Materials");
            ui.label(game.materials.to_string());
            ui.end_row();
            ui.label("Fuel");
            ui.label(game.fuel.to_string());
            ui.end_row();
            ui.label("Energy");
            ui.label(game.energy.to_string());
            ui.end_row();
        });
        ui.separator();
        ui.label(format!("Global warming: {:.1}%", game.warming));
        ui.add(egui::ProgressBar::new(game.warming / 100.0));
        ui.separator();
        if game.ship_progress >= 1.0 {
            ui.label("Ship: at Mars");
        } else {
            ui.label(format!("Ship: Earth to Mars, {} turns left", game.ship_turns_left()));
        }
        ui.separator();
        if ui.button("End turn").clicked() {
            game.end_turn();
        }
        let other = match game.view {
            View::Earth => "Solar System view",
            View::Solar => "Earth view",
        };
        if ui.button(other).clicked() {
            game.view = match game.view {
                View::Earth => View::Solar,
                View::Solar => View::Earth,
            };
            game.last_action = format!("Switched to {other}.");
        }
        ui.separator();
        ui.label("State after last action:");
        ui.label(&game.last_action);
        ui.separator();
        ui.small("Drag the picture to rotate.");
    });
    // Transparent central area that only catches the drag, so the 3D shows through.
    egui::CentralPanel::default().frame(egui::Frame::NONE).show(&mut root, |ui| {
        let (_rect, resp) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());
        game.yaw += resp.drag_motion().x * 0.01;
    });
    Ok(())
}

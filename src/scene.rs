//! The two kinds of 3D view (spec 17.1): the Solar System Map and a Body Surface Map per Body.

use crate::app::*;
use crate::geo;
use bevy::prelude::*;
use dying_earth_engine::*;

#[derive(Component)]
pub struct SolarRoot;
#[derive(Component)]
pub struct SurfaceRoot(pub BodyId);
#[derive(Component)]
pub struct Globe(pub BodyId);
#[derive(Component)]
pub struct SolarBody(pub BodyId);
#[derive(Component)]
pub struct SlotMarker {
    pub body: BodyId,
    pub slot: u32,
    pub on_surface: bool,
}
#[derive(Component)]
pub struct StackMarker {
    pub body: BodyId,
    pub seat: Seat,
}
#[derive(Component)]
pub struct ControlRing(pub BodyId);
#[derive(Component)]
pub struct MainCamera;
#[derive(Component)]
pub struct Sunlight;

#[derive(Resource)]
pub struct SceneHandles {
    pub earth_material: Handle<StandardMaterial>,
    pub flat: Vec<Handle<StandardMaterial>>,
    pub grey: Handle<StandardMaterial>,
    pub ring_materials: Vec<Handle<StandardMaterial>>,
}

pub const GLOBE_RADIUS: f32 = 1.9;

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    textures: Res<Textures>,
    session: Res<Session>,
) {
    let sphere = meshes.add(Sphere::new(1.0).mesh().uv(64, 32));
    let small_sphere = meshes.add(Sphere::new(1.0).mesh().uv(16, 8));
    let marker = meshes.add(Cone::new(1.0, 2.0));
    let ring = meshes.add(Annulus::new(0.96, 1.04));
    let textured = |images: &mut Assets<Image>, materials: &mut Assets<StandardMaterial>, rgba: &crate::textures::Rgba| {
        let img = images.add(rgba.to_image());
        materials.add(StandardMaterial { base_color_texture: Some(img), perceptual_roughness: 1.0, reflectance: 0.0, ..default() })
    };
    let earth_material = textured(&mut images, &mut materials, &textures.earth);
    let moon_material = textured(&mut images, &mut materials, &textures.moon);
    let mars_material = textured(&mut images, &mut materials, &textures.mars);
    let phobos_material = textured(&mut images, &mut materials, &textures.phobos);
    let deimos_material = textured(&mut images, &mut materials, &textures.deimos);
    // Ticket #93: Venus's clouds.
    let venus_material = textured(&mut images, &mut materials, &textures.venus);
    let colours = session.colours();
    let mut flat = |c: [f32; 3], unlit: bool| {
        materials.add(StandardMaterial { base_color: Color::srgb(c[0], c[1], c[2]), unlit, double_sided: true, cull_mode: None, ..default() })
    };
    // Ticket #50: one material per seat, four of them.
    let flats: Vec<Handle<StandardMaterial>> = colours.iter().map(|c| flat(*c, true)).collect();
    let grey = flat([0.45, 0.45, 0.5], true);
    let sun = flat([1.0, 0.85, 0.3], true);
    let orbit = flat([0.3, 0.3, 0.38], true);
    let rings: Vec<Handle<StandardMaterial>> = colours.iter().map(|c| flat(*c, true)).collect();

    // --- Solar System Map
    let flat_ring = Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2));
    let orbit_earth = meshes.add(Annulus::new(3.38, 3.42));
    let orbit_mars = meshes.add(Annulus::new(5.98, 6.02));
    // Ticket #93: Venus's ring, inside Earth's.
    let orbit_venus = meshes.add(Annulus::new(2.43, 2.47));
    commands
        .spawn((Transform::default(), Visibility::Hidden, SolarRoot))
        .with_children(|p| {
            p.spawn((Mesh3d(sphere.clone()), MeshMaterial3d(sun.clone()), Transform::from_scale(Vec3::splat(0.8))));
            p.spawn((Mesh3d(orbit_earth), MeshMaterial3d(orbit.clone()), flat_ring));
            p.spawn((Mesh3d(orbit_mars), MeshMaterial3d(orbit.clone()), flat_ring));
            p.spawn((Mesh3d(orbit_venus), MeshMaterial3d(orbit.clone()), flat_ring));
            for body in BodyId::ALL {
                let mat = match body {
                    BodyId::Earth => earth_material.clone(),
                    BodyId::Moon => moon_material.clone(),
                    BodyId::Mars => mars_material.clone(),
                    BodyId::Phobos => phobos_material.clone(),
                    BodyId::Deimos => deimos_material.clone(),
                    BodyId::Venus => venus_material.clone(),
                };
                p.spawn((
                    Mesh3d(sphere.clone()),
                    MeshMaterial3d(mat),
                    Transform::from_scale(Vec3::splat(geo::solar_radius(body))).with_rotation(geo::upright()),
                    SolarBody(body),
                ));
                // Orbital Control ring.
                p.spawn((Mesh3d(ring.clone()), MeshMaterial3d(grey.clone()), Transform::from_scale(Vec3::splat(geo::solar_radius(body) * 1.6)).with_rotation(flat_ring.rotation), Visibility::Hidden, ControlRing(body)));
                // Colony Slot dots in a ring around the body.
                let n = session.tables.body(body).colony_slots();
                for slot in 0..n {
                    p.spawn((Mesh3d(small_sphere.clone()), MeshMaterial3d(grey.clone()), Transform::from_scale(Vec3::splat(0.05)), SlotMarker { body, slot, on_surface: false }));
                }
                // One stack marker per seat.
                for seat in Seat::ALL {
                    let mat = flats[seat.index() % flats.len()].clone();
                    p.spawn((Mesh3d(marker.clone()), MeshMaterial3d(mat), Transform::from_scale(Vec3::splat(0.09)), Visibility::Hidden, StackMarker { body, seat }));
                }
            }
        });

    // --- Body Surface Maps
    for body in BodyId::ALL {
        let mat = match body {
            BodyId::Earth => earth_material.clone(),
            BodyId::Moon => moon_material.clone(),
            BodyId::Mars => mars_material.clone(),
            BodyId::Phobos => phobos_material.clone(),
            BodyId::Deimos => deimos_material.clone(),
            BodyId::Venus => venus_material.clone(),
        };
        commands
            .spawn((Transform::default(), Visibility::Hidden, SurfaceRoot(body)))
            .with_children(|p| {
                p.spawn((Mesh3d(sphere.clone()), MeshMaterial3d(mat), Transform::from_scale(Vec3::splat(GLOBE_RADIUS)).with_rotation(geo::upright()), Globe(body)))
                    .with_children(|g| {
                        let n = session.tables.body(body).colony_slots();
                        for slot in 0..n {
                            let (lon, lat) = geo::slot_lonlat(session.tables.body(body), slot);
                            let pos = geo::local_from_lonlat(lon, lat) * 1.02;
                            g.spawn((Mesh3d(small_sphere.clone()), MeshMaterial3d(grey.clone()), Transform::from_translation(pos).with_scale(Vec3::splat(0.045)), SlotMarker { body, slot, on_surface: true }));
                        }
                    });
            });
    }

    commands.spawn((DirectionalLight { illuminance: 2600.0, ..default() }, Transform::from_xyz(4.0, 5.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y), Sunlight));
    // Ambient light is per camera in Bevy 0.19; a separate entity would create a bare camera that
    // bevy_egui then attaches its context to, and no panel would ever draw.
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        AmbientLight { color: Color::WHITE, brightness: 900.0, ..default() },
        MainCamera,
    ));
    commands.insert_resource(SceneHandles { earth_material, flat: flats, grey, ring_materials: rings });
}

type RootQuery<'w, 's> = Query<'w, 's, (&'static mut Visibility, Option<&'static SolarRoot>, Option<&'static SurfaceRoot>), Or<(With<SolarRoot>, With<SurfaceRoot>)>>;
type GlobeQuery<'w, 's> = Query<'w, 's, (&'static Globe, &'static mut Transform), (Without<SolarBody>, Without<MainCamera>)>;
type BodyQuery<'w, 's> = Query<'w, 's, (&'static SolarBody, &'static mut Transform), (Without<Globe>, Without<MainCamera>)>;
type SlotQuery<'w, 's> = Query<'w, 's, (&'static SlotMarker, &'static mut Transform, &'static mut MeshMaterial3d<StandardMaterial>, &'static mut Visibility), (Without<Globe>, Without<SolarBody>, Without<MainCamera>, Without<SolarRoot>, Without<SurfaceRoot>, Without<StackMarker>, Without<ControlRing>)>;
type StackQuery<'w, 's> = Query<'w, 's, (&'static StackMarker, &'static mut Transform, &'static mut Visibility), (Without<Globe>, Without<SolarBody>, Without<SlotMarker>, Without<ControlRing>, Without<SolarRoot>, Without<SurfaceRoot>, Without<MainCamera>)>;
type RingQuery<'w, 's> = Query<'w, 's, (&'static ControlRing, &'static mut Transform, &'static mut Visibility, &'static mut MeshMaterial3d<StandardMaterial>), (Without<Globe>, Without<SolarBody>, Without<SlotMarker>, Without<StackMarker>, Without<SolarRoot>, Without<SurfaceRoot>, Without<MainCamera>)>;

/// Every frame: show the right root, place bodies and markers, colour them from the board.
#[allow(clippy::too_many_arguments)]
pub fn sync_scene(
    session: Res<Session>,
    view: Res<ViewState>,
    handles: Res<SceneHandles>,
    mut roots: RootQuery,
    mut globes: GlobeQuery,
    mut bodies: BodyQuery,
    mut slots: SlotQuery,
    mut stacks: StackQuery,
    mut rings: RingQuery,
    mut camera: Query<&mut Transform, With<MainCamera>>,
    mut gizmos: Gizmos,
) {
    let showing_3d = matches!(session.screen, Screen::Playing | Screen::ChooseStart { .. } | Screen::GameOver);
    let current = if matches!(session.screen, Screen::ChooseStart { .. }) { View::Surface(BodyId::Earth) } else { view.view };
    for (mut vis, solar, surface) in &mut roots {
        let show = showing_3d
            && match (solar, surface) {
                (Some(_), _) => current == View::Solar,
                (_, Some(SurfaceRoot(b))) => current == View::Surface(*b),
                _ => false,
            };
        *vis = if show { Visibility::Inherited } else { Visibility::Hidden };
    }
    let turn = session.game.as_ref().map(|g| g.turn).unwrap_or(1);
    // Ticket #57: every Body stands at its true heliocentric longitude for the turn. Before a game
    // is made there is no sky to read, so the title screen's system stands at longitude zero.
    let place = |body: BodyId| match session.game.as_ref() {
        Some(g) => geo::solar_place(g, body),
        None => geo::solar_position(body, turn, 0.0),
    };
    // Globes turn under the pointer; the start-screen Earth spins on its own.
    for (globe, mut t) in &mut globes {
        // Ticket #100 (version 0.07.0): the start globe follows its own spin only until the player
        // takes hold of it; from then on it follows the pointer, like every other globe.
        let spinning = matches!(session.screen, Screen::ChooseStart { .. }) && !view.start_grabbed;
        let yaw = if spinning { view.spin } else { view.yaw };
        t.rotation = Quat::from_rotation_x(view.pitch) * Quat::from_rotation_y(yaw) * geo::upright();
        let _ = globe;
    }
    for (body, mut t) in &mut bodies {
        t.translation = place(body.0);
        t.rotation = Quat::from_rotation_y(turn as f32 * 0.3) * geo::upright();
    }
    let Some(game) = session.game.as_ref() else {
        // Ticket #100: no game yet, so this is the start screen. Its globe zooms on the wheel, as
        // the Earth Map does once the game is running.
        for mut t in &mut camera {
            *t = Transform::from_xyz(0.0, 0.0, 6.0 * view.zoom).looking_at(Vec3::ZERO, Vec3::Y);
        }
        return;
    };
    for (m, mut t, mut mat, mut vis) in &mut slots {
        let owner = game.colony_at(m.body, m.slot).and_then(|c| c.control.director());
        // Ticket #103 (version 0.07.0): Earth's ground slots are Antarctica's, and there is nothing
        // to see there until the ice opens.
        let under_ice = m.body == BodyId::Earth && m.on_surface && !game.antarctica_open;
        *vis = if under_ice { Visibility::Hidden } else { Visibility::Inherited };
        let want = match owner {
            Some(s) => handles.flat[s.index()].clone(),
            None => handles.grey.clone(),
        };
        if mat.0 != want {
            mat.0 = want;
        }
        if !m.on_surface {
            let n = game.tables.body(m.body).colony_slots().max(1) as f32;
            let a = m.slot as f32 / n * std::f32::consts::TAU;
            let r = geo::solar_radius(m.body) * 1.35;
            t.translation = place(m.body) + Vec3::new(a.cos() * r, 0.0, a.sin() * r);
            t.scale = Vec3::splat(if owner.is_some() { 0.07 } else { 0.045 });
        } else {
            t.scale = Vec3::splat(if owner.is_some() { 0.065 } else { 0.045 });
        }
    }
    for (m, mut t, mut vis) in &mut stacks {
        let ships = game.ships_at(m.seat, m.body);
        if ships.is_empty() {
            *vis = Visibility::Hidden;
            continue;
        }
        *vis = Visibility::Inherited;
        // Ticket #50: one of four fixed angles round the Body, by seat.
        t.translation = place(m.body) + geo::stack_offset(m.seat, geo::solar_radius(m.body));
    }
    for (r, mut t, mut vis, mut mat) in &mut rings {
        t.translation = place(r.0);
        match game.orbital_control(r.0) {
            Some(s) => {
                *vis = Visibility::Inherited;
                let want = handles.ring_materials[s.index() % handles.ring_materials.len()].clone();
                if mat.0 != want {
                    mat.0 = want;
                }
            }
            None => *vis = Visibility::Hidden,
        }
    }
    // Transits as lines with a marker at the current fraction.
    if current == View::Solar && showing_3d {
        for s in &game.ships {
            if let ShipAt::Transit { from, to, turns_left } = s.at {
                let a = place(from);
                let b = place(to);
                let colour = session.colours()[s.seat.index()];
                let c = Color::srgb(colour[0], colour[1], colour[2]);
                gizmos.line(a, b, c);
                let (total, _) = game.transit_cost(from, to);
                let f = 1.0 - turns_left as f32 / total.max(1) as f32;
                let p = a.lerp(b, f.clamp(0.05, 0.95)) + Vec3::Y * 0.12;
                gizmos.sphere(Isometry3d::from_translation(p), 0.06, c);
            }
        }
    }
    for mut t in &mut camera {
        *t = match current {
            View::Solar => {
                // The side panel covers the right of the window, so the system sits a little left.
                let target = Vec3::new(1.6, 0.0, 0.0);
                let eye = target + Quat::from_rotation_y(view.solar_yaw) * Vec3::new(0.0, 9.5 * view.zoom, 11.5 * view.zoom);
                Transform::from_translation(eye).looking_at(target, Vec3::Y)
            }
            View::Surface(_) => {
                let d = 6.0 * view.zoom;
                Transform::from_xyz(0.9, 0.0, d).looking_at(Vec3::new(0.9, 0.0, 0.0), Vec3::Y)
            }
        };
    }
}

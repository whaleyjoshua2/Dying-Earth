#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// PROTOTYPE - throwaway. Candidate B for wayfinder ticket #5:
// eframe (egui + wgpu), with the two 3D views hand-written in wgpu.
//
// Same content as the Bevy candidate, on purpose, so the comparison is fair:
// an Earth globe with three tinted nation states, a solar system with the
// Sun, Earth, Moon, Mars and a ship in transit, and one HUD panel.
// Drag in the 3D area to rotate. "End turn" advances the toy state.

mod geom;
mod texgen;

use eframe::egui;
use eframe::egui_wgpu::{self, wgpu, wgpu::util::DeviceExt as _};
use glam::{Mat4, Vec3};

const MAX_OBJECTS: usize = 16;

// ---------------------------------------------------------------- game state

#[derive(Clone, Copy, PartialEq)]
enum View {
    Earth,
    Solar,
}

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

fn hud(ui: &mut egui::Ui, game: &mut Game) {
    ui.heading("PROTOTYPE: eframe + wgpu");
    ui.label("Candidate B. Hand-written 3D inside egui.");
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
}

// ---------------------------------------------------------------- scene

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    mvp: [[f32; 4]; 4],
    model: [[f32; 4]; 4],
    light_dir: [f32; 4],
    color: [f32; 4],
    params: [f32; 4],
}

#[derive(Clone, Copy)]
enum MeshKind {
    Sphere,
    Ring,
}

#[derive(Clone, Copy)]
struct DrawItem {
    mesh: MeshKind,
    uniforms: Uniforms,
}

fn item(
    mesh: MeshKind,
    view_proj: Mat4,
    model: Mat4,
    color: [f32; 3],
    unlit: bool,
    textured: bool,
) -> DrawItem {
    DrawItem {
        mesh,
        uniforms: Uniforms {
            mvp: (view_proj * model).to_cols_array_2d(),
            model: model.to_cols_array_2d(),
            light_dir: [0.6, 0.5, 0.7, 0.0],
            color: [color[0], color[1], color[2], 1.0],
            params: [
                if unlit { 1.0 } else { 0.0 },
                if textured { 1.0 } else { 0.0 },
                0.0,
                0.0,
            ],
        },
    }
}

fn body_angles(turn: u32) -> (f32, f32, f32) {
    let t = turn as f32;
    (t * 0.35, 1.2 + t * 0.19, t * 1.3) // earth, mars, moon
}

fn build_scene(game: &Game, aspect: f32) -> Vec<DrawItem> {
    let proj = Mat4::perspective_rh(45f32.to_radians(), aspect, 0.1, 100.0);
    match game.view {
        View::Earth => {
            let view = Mat4::look_at_rh(Vec3::new(0.0, 1.2, 5.0), Vec3::ZERO, Vec3::Y);
            let model = Mat4::from_rotation_y(game.yaw) * Mat4::from_scale(Vec3::splat(1.8));
            vec![item(MeshKind::Sphere, proj * view, model, [1.0, 1.0, 1.0], false, true)]
        }
        View::Solar => {
            let eye = Mat4::from_rotation_y(game.yaw).transform_point3(Vec3::new(0.0, 9.0, 11.0));
            let view = Mat4::look_at_rh(eye, Vec3::ZERO, Vec3::Y);
            let vp = proj * view;
            let (ea, ma, mo) = body_angles(game.turn);
            let earth = Vec3::new(ea.cos() * 3.0, 0.0, ea.sin() * 3.0);
            let mars = Vec3::new(ma.cos() * 5.0, 0.0, ma.sin() * 5.0);
            let moon = earth + Vec3::new(mo.cos() * 0.6, 0.0, mo.sin() * 0.6);
            let ship = earth.lerp(mars, game.ship_progress) + Vec3::new(0.0, 0.15, 0.0);
            let sphere = |pos: Vec3, r: f32| Mat4::from_translation(pos) * Mat4::from_scale(Vec3::splat(r));
            vec![
                item(MeshKind::Sphere, vp, sphere(Vec3::ZERO, 0.9), [1.0, 0.85, 0.3], true, false),
                item(MeshKind::Ring, vp, Mat4::from_scale(Vec3::splat(3.0)), [0.45, 0.45, 0.5], true, false),
                item(MeshKind::Ring, vp, Mat4::from_scale(Vec3::splat(5.0)), [0.45, 0.45, 0.5], true, false),
                item(MeshKind::Sphere, vp, sphere(earth, 0.35), [1.0, 1.0, 1.0], false, true),
                item(MeshKind::Sphere, vp, sphere(moon, 0.1), [0.7, 0.7, 0.7], false, false),
                item(MeshKind::Sphere, vp, sphere(mars, 0.25), [0.85, 0.35, 0.2], false, false),
                item(MeshKind::Sphere, vp, sphere(ship, 0.07), [1.0, 1.0, 1.0], true, false),
            ]
        }
    }
}

// ---------------------------------------------------------------- gpu

struct GpuMesh {
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    index_count: u32,
}

struct Gpu {
    pipeline: wgpu::RenderPipeline,
    sphere: GpuMesh,
    ring: GpuMesh,
    uniform_bufs: Vec<wgpu::Buffer>,
    bind_groups: Vec<wgpu::BindGroup>,
}

fn upload_mesh(device: &wgpu::Device, m: &geom::MeshData) -> GpuMesh {
    GpuMesh {
        vertices: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertices"),
            contents: bytemuck::cast_slice(&m.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        }),
        indices: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("indices"),
            contents: bytemuck::cast_slice(&m.indices),
            usage: wgpu::BufferUsages::INDEX,
        }),
        index_count: m.indices.len() as u32,
    }
}

fn build_gpu(rs: &egui_wgpu::RenderState) -> Gpu {
    let device = &rs.device;
    let queue = &rs.queue;

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("scene"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });

    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("scene"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("scene"),
        bind_group_layouts: &[Some(&bgl)],
        immediate_size: 0,
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("scene"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[Some(wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<geom::Vertex>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2],
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(rs.target_format.into())],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
            cull_mode: Some(wgpu::Face::Back),
            front_face: wgpu::FrontFace::Ccw,
            ..Default::default()
        },
        // eframe was asked for a 24-bit depth buffer; egui's own pipeline draws
        // with compare Always, so the HUD stays on top of anything drawn here.
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth24Plus,
            depth_write_enabled: Some(true),
            depth_compare: Some(wgpu::CompareFunction::Less),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview_mask: None,
        cache: None,
    });

    // Earth texture, generated in code.
    let size = wgpu::Extent3d { width: texgen::W, height: texgen::H, depth_or_array_layers: 1 };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("earth"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &texgen::earth_rgba(),
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * texgen::W),
            rows_per_image: Some(texgen::H),
        },
        size,
    );
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::Repeat,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        ..Default::default()
    });

    let mut uniform_bufs = Vec::new();
    let mut bind_groups = Vec::new();
    for i in 0..MAX_OBJECTS {
        let buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("uniforms"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        bind_groups.push(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&format!("object {i}")),
            layout: &bgl,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&sampler) },
            ],
        }));
        uniform_bufs.push(buf);
    }

    Gpu {
        pipeline,
        sphere: upload_mesh(device, &geom::uv_sphere(1.0, 48, 24)),
        ring: upload_mesh(device, &geom::ring(0.99, 1.01, 96)),
        uniform_bufs,
        bind_groups,
    }
}

struct SceneCallback {
    items: Vec<DrawItem>,
}

impl egui_wgpu::CallbackTrait for SceneCallback {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen: &egui_wgpu::ScreenDescriptor,
        _encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let gpu: &Gpu = resources.get().expect("gpu resources");
        for (i, it) in self.items.iter().enumerate().take(MAX_OBJECTS) {
            queue.write_buffer(&gpu.uniform_bufs[i], 0, bytemuck::bytes_of(&it.uniforms));
        }
        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        pass: &mut wgpu::RenderPass<'static>,
        resources: &egui_wgpu::CallbackResources,
    ) {
        let gpu: &Gpu = resources.get().expect("gpu resources");
        pass.set_pipeline(&gpu.pipeline);
        for (i, it) in self.items.iter().enumerate().take(MAX_OBJECTS) {
            let m = match it.mesh {
                MeshKind::Sphere => &gpu.sphere,
                MeshKind::Ring => &gpu.ring,
            };
            pass.set_bind_group(0, &gpu.bind_groups[i], &[]);
            pass.set_vertex_buffer(0, m.vertices.slice(..));
            pass.set_index_buffer(m.indices.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..m.index_count, 0, 0..1);
        }
    }
}

// ---------------------------------------------------------------- app

/// Headless screenshot mode: `shot:<prefix>` on the command line puts the
/// window off-screen, saves <prefix>-3s.png and <prefix>-6s.png, then exits.
fn shot_arg() -> Option<String> {
    std::env::args().find_map(|a| a.strip_prefix("shot:").map(str::to_owned))
}

struct App {
    game: Game,
    shot: Option<String>,
    t0: std::time::Instant,
    requested: [bool; 2],
    saved: usize,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let rs = cc.wgpu_render_state.as_ref().expect("eframe was built with the wgpu renderer");
        let gpu = build_gpu(rs);
        rs.renderer.write().callback_resources.insert(gpu);
        Self { game: Game::new(), shot: shot_arg(), t0: std::time::Instant::now(), requested: [false, false], saved: 0 }
    }

    fn shot_step(&mut self, ctx: &egui::Context) {
        let Some(prefix) = self.shot.clone() else { return };
        let shots: Vec<egui::ColorImage> = ctx.input(|i| {
            i.events
                .iter()
                .filter_map(|e| match e {
                    egui::Event::Screenshot { image, .. } => Some((**image).clone()),
                    _ => None,
                })
                .collect()
        });
        for img in shots {
            let tag = ["3s", "6s"][self.saved.min(1)];
            self.saved += 1;
            let [w, h] = img.size;
            let bytes: Vec<u8> = img.pixels.iter().flat_map(|c| c.to_array()).collect();
            if let Some(rgba) = image::RgbaImage::from_raw(w as u32, h as u32, bytes) {
                let _ = rgba.save(format!("{prefix}-{tag}.png"));
            }
        }
        let t = self.t0.elapsed().as_secs_f32();
        for (i, at) in [3.0_f32, 6.0].iter().enumerate() {
            if t > *at && !self.requested[i] {
                self.requested[i] = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::UserData::default()));
            }
        }
        if t > 8.0 {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.shot_step(&ui.ctx().clone());
        egui::Panel::left("hud").default_size(280.0).show(ui, |ui| hud(ui, &mut self.game));
        egui::CentralPanel::default().show(ui, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let size = ui.available_size();
                let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::drag());
                self.game.yaw += resp.drag_motion().x * 0.01;
                let aspect = rect.width().max(1.0) / rect.height().max(1.0);
                let items = build_scene(&self.game, aspect);
                ui.painter().add(egui_wgpu::Callback::new_paint_callback(rect, SceneCallback { items }));
            });
        });
        ui.ctx().request_repaint();
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: {
            let v = egui::ViewportBuilder::default()
                .with_title("PROTOTYPE eframe-views (egui 0.36 + wgpu 30)")
                .with_inner_size([1100.0, 700.0]);
            if shot_arg().is_some() { v.with_position([-5000.0, -5000.0]) } else { v }
        },
        depth_buffer: 24,
        ..Default::default()
    };
    eframe::run_native(
        "prototype-eframe-views",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

// PROTOTYPE - throwaway. Hand-rolled meshes for the wgpu side. This is the
// "planets are homework" cost the research warned about, made concrete.

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

pub struct MeshData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

/// UV sphere. Winding is counter-clockwise seen from outside, so back-face
/// culling makes each sphere self-occlude correctly without a depth buffer.
pub fn uv_sphere(radius: f32, sectors: u32, stacks: u32) -> MeshData {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for i in 0..=stacks {
        let v = i as f32 / stacks as f32;
        let lat = std::f32::consts::FRAC_PI_2 - v * std::f32::consts::PI; // +90 at top
        let (sl, cl) = lat.sin_cos();
        for j in 0..=sectors {
            let u = j as f32 / sectors as f32;
            let lon = u * std::f32::consts::TAU;
            let (sn, cn) = lon.sin_cos();
            let n = [cl * cn, sl, cl * sn];
            vertices.push(Vertex {
                pos: [n[0] * radius, n[1] * radius, n[2] * radius],
                normal: n,
                uv: [u, v],
            });
        }
    }
    let row = sectors + 1;
    for i in 0..stacks {
        for j in 0..sectors {
            let k1 = i * row + j;
            let k2 = k1 + row;
            if i != 0 {
                indices.extend_from_slice(&[k1, k1 + 1, k2]);
            }
            if i != stacks - 1 {
                indices.extend_from_slice(&[k1 + 1, k2 + 1, k2]);
            }
        }
    }
    MeshData { vertices, indices }
}

/// Flat ring in the XZ plane (an orbit line). Emitted with both windings so it
/// is visible from either side under back-face culling.
pub fn ring(inner: f32, outer: f32, segments: u32) -> MeshData {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for j in 0..=segments {
        let a = j as f32 / segments as f32 * std::f32::consts::TAU;
        let (s, c) = a.sin_cos();
        for r in [inner, outer] {
            vertices.push(Vertex {
                pos: [c * r, 0.0, s * r],
                normal: [0.0, 1.0, 0.0],
                uv: [j as f32 / segments as f32, if r == inner { 0.0 } else { 1.0 }],
            });
        }
    }
    for j in 0..segments {
        let a = j * 2;
        let b = a + 1;
        let c = a + 2;
        let d = a + 3;
        indices.extend_from_slice(&[a, c, b, b, c, d, a, b, c, b, d, c]);
    }
    MeshData { vertices, indices }
}

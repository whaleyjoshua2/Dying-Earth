// PROTOTYPE - throwaway. One shader for every object: textured or flat colour,
// lit or unlit, chosen per draw through the uniform.

struct Uniforms {
    mvp: mat4x4<f32>,
    model: mat4x4<f32>,
    light_dir: vec4<f32>,   // xyz = direction toward the light, world space
    color: vec4<f32>,       // rgb = flat colour, a = unused
    params: vec4<f32>,      // x = unlit (1) / lit (0), y = textured (1) / flat (0)
};

@group(0) @binding(0) var<uniform> u: Uniforms;
@group(0) @binding(1) var tex: texture_2d<f32>;
@group(0) @binding(2) var samp: sampler;

struct VertexIn {
    @location(0) pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
    var out: VertexOut;
    out.clip = u.mvp * vec4<f32>(in.pos, 1.0);
    out.normal = normalize((u.model * vec4<f32>(in.normal, 0.0)).xyz);
    out.uv = in.uv;
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    var base = u.color.rgb;
    if (u.params.y > 0.5) {
        base = textureSample(tex, samp, in.uv).rgb;
    }
    var shade = 1.0;
    if (u.params.x < 0.5) {
        let n = normalize(in.normal);
        let l = normalize(u.light_dir.xyz);
        shade = 0.15 + 0.85 * max(dot(n, l), 0.0);
    }
    return vec4<f32>(base * shade, 1.0);
}

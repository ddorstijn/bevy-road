#import bevy_pbr::forward_io::VertexOutput

struct Curve {
    center: vec2<f32>,
    radius: f32,
    thickness: f32,
}

@group(2) @binding(0) var<storage> curves: array<Curve>;

fn sdCircle(p: vec2f, r: f32) -> f32 {
    return length(p) - r;
}

fn sdArc(p: vec2f, sc1: vec2f, sc2: vec2f, r1: f32, r2: f32) -> f32 {
    var q: vec2f = p * mat2x2<f32>(vec2f(sc1.x, sc1.y), vec2f(-sc1.y, sc1.x));
    q.x = abs(q.x);
    let k = select(length(q), dot(q, sc2), sc2.y * q.x > sc2.x * q.y);
    return sqrt(dot(q, q) + r1 * r1 - 2. * r1 * k) - r2;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var col: vec3<f32>;

    for (var i: u32 = 0; i < arrayLength(&curves); i++) {
        col += mix(vec3(0.0), vec3(1.0), step(sdCircle(in.world_position.xz - curves[i].center, curves[i].radius), curves[i].radius));
    }

    return vec4(col, 1.0);
    // return in.position / in.uv.x;
}
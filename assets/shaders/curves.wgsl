#import bevy_pbr::forward_io::VertexOutput;

struct Curve {
    rotation: vec2<f32>,
    center: vec2<f32>,
    angle: vec2<f32>,
    radius: f32,
    thickness: f32,
}

@group(2) @binding(2) var<storage> curves: array<Curve>;

const PI = 3.14159265359;

fn sd_arc(p_in: vec2<f32>, sc: vec2<f32>, ra: f32, rb: f32) -> f32 {
    var p = p_in;
    p.x = abs(p.x);
    return select(
        abs(length(p) - ra),
        length(p - sc * ra),
        sc.y * p.x > sc.x * p.y
    ) - rb;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var col = vec4(0.0);


    for (var i = u32(0); i < arrayLength(&curves); i++) {
        let rotation = mat2x2(curves[i].rotation.y, -curves[i].rotation.x, curves[i].rotation.x, curves[i].rotation.y);
        let pos = (in.world_position.xz - curves[i].center) * rotation;
        col += mix(vec4(0.0), vec4(1.0), step(sd_arc(pos, curves[i].angle, abs(curves[i].radius), curves[i].thickness), curves[i].thickness));
    }

    return col;
}
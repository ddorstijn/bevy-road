#import bevy_pbr::forward_io::VertexOutput

struct Curve {
    center: vec2<f32>,
    radius: f32,
    thickness: f32,
}

@group(2) @binding(0) var<storage> curves: array<Curve>;

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
    var col: vec4<f32>;

    let aperture = (PI * 1.5) * 0.5;

    for (var i: u32 = 0; i < arrayLength(&curves); i++) {
        let rotation = mat2x2<f32>(sin(PI), cos(PI), -cos(PI), sin(PI));
        let pos = (in.world_position.xz - curves[i].center - vec2(-0.1, 0.0)) * rotation;
        col += mix(vec4(0.0), vec4(1.0), step(sd_arc(pos, vec2(sin(aperture), cos(aperture)), curves[i].radius, curves[i].thickness), curves[i].thickness));
    }

    for (var i: u32 = 0; i < arrayLength(&curves); i++) {
        let rotation = mat2x2<f32>(sin(PI), cos(PI), -cos(PI), sin(PI));
        let pos = (in.world_position.xz - curves[i].center - vec2(0.1, 0.0)) * rotation;
        col += mix(vec4(0.0), vec4(1.0), step(sd_arc(pos, vec2(sin(aperture), cos(aperture)), curves[i].radius, curves[i].thickness), curves[i].thickness));
    }

    for (var i: u32 = 0; i < arrayLength(&curves); i++) {
        let rotation = mat2x2<f32>(sin(PI), cos(PI), -cos(PI), sin(PI));
        let pos = (in.world_position.xz - curves[i].center - vec2(0.75, 0.0)) * rotation;
        col += mix(vec4(0.0), vec4(1.0), step(sd_arc(pos, vec2(sin(aperture), cos(aperture)), curves[i].radius, curves[i].thickness), curves[i].thickness));
    }

    return col;
}
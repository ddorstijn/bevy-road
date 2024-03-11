struct VertexOutput {
    // This is `clip position` when the struct is used as a vertex stage output
    // and `frag coord` when used as a fragment stage input
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
}

struct Curve {
    rotation: vec2<f32>,
    center: vec2<f32>,
    angle: vec2<f32>,
    radius: f32,
    thickness: f32,
    straight: u32,
}

@group(2) @binding(2) var<storage> curves: array<Curve>;

fn sd_arc(p_in: vec2<f32>, sc: vec2<f32>, ra: f32, rb: f32) -> f32 {
    var p = p_in;
    p.x = abs(p.x);
    return select(
        abs(length(p) - ra),
        length(p - sc * ra),
        sc.y * p.x > sc.x * p.y
    ) - rb;
}

fn sd_segment(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>, th: f32) -> f32 {
    let pa = p - a;
    let ba = b - a;
    let h = clamp(dot(pa, ba) / dot(ba, ba), 0., 1.);
    return length(pa - ba * h) - th;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var col: vec4<f32>;

    for (var i = u32(0); i < arrayLength(&curves); i++) {
        let rotation = mat2x2(curves[i].rotation.y, -curves[i].rotation.x, curves[i].rotation.x, curves[i].rotation.y);
        let pos = (in.world_position.xz - curves[i].center) * rotation;
        // let distance = select(
        //     sd_arc(pos, curves[i].angle, curves[i].radius, curves[i].thickness),
        //     sd_segment(pos, vec2(0.0, -curves[i].radius * 0.5), vec2(0.0, curves[i].radius * 0.5), curves[i].thickness),
        //     curves[i].straight == 0
        // );
        col += mix(vec4(0.0), vec4(1.0), step(sd_arc(pos, curves[i].angle, curves[i].radius, curves[i].thickness), curves[i].thickness));
    }

    return col;
}
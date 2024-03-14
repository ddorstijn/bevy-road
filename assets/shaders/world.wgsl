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
    lanes: u32
}

@group(2) @binding(0) var road_texture: texture_2d<f32>;
@group(2) @binding(1) var road_sampler: sampler;
@group(2) @binding(2) var<storage> curves: array<Curve>;

const ROAD_WIDTH: f32 = 1.0;

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
    var col = vec4(0.05, 0.4, 0.15, 1.0);

    var min_distance = 99999.9;
    var min_length = 0.0;
    for (var i = u32(0); i < arrayLength(&curves); i++) {
        // Issue with 0 length dynamic storage buffer causes array to be length 1 with all zeroes when array length should be 0.
        // This creates the issue where every pixel is 0 (because mutliplied by rotation which is 0), causing everything to fall in the field
        if curves[i].rotation.y + curves[i].rotation.x == 0.0 {
            continue;
        }

        let rotation = mat2x2(curves[i].rotation.y, -curves[i].rotation.x, curves[i].rotation.x, curves[i].rotation.y);
        let pos = (in.world_position.xz - curves[i].center) * rotation;

        let thickness = f32(curves[i].lanes) * (ROAD_WIDTH / 2.0);

        // Radius is set to 0 for straight lines 
        let distance = select(
            sd_segment(pos, curves[i].center, curves[i].angle, thickness),
            sd_arc(pos, curves[i].angle, curves[i].radius, thickness),
            bool(curves[i].radius)
        );

        let length = select(
            dot(normalize(curves[i].angle), pos),
            (atan2(pos.y, pos.x) - 1.570796) * curves[i].radius,
            bool(curves[i].radius)
        );

        min_length = select(min_length, length, min_distance > distance);
        min_distance = min(min_distance, distance);
    }

    let texel = textureSample(road_texture, road_sampler, vec2(-min_distance, fract(min_length)));
    col = mix(col, texel, step(min_distance, 0.0));

    return col;
}
struct VertexOutput {
    // This is `clip position` when the struct is used as a vertex stage output
    // and `frag coord` when used as a fragment stage input
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
}

struct Curve {
    twist: u32,
    center: vec2<f32>,
    start: vec2<f32>,
    end: vec2<f32>,
    radius: f32,
    length: f32,
    lanes: u32
}

@group(2) @binding(0) var road_texture: texture_2d<f32>;
@group(2) @binding(1) var road_sampler: sampler;
@group(2) @binding(2) var<storage> curves: array<Curve>;

const ROAD_WIDTH: f32 = 1.0;
const TAU: f32 = 6.28318530718;

fn cross2d(a: vec2<f32>, b: vec2<f32>) -> f32 {
    return a.y * b.x - a.x * b.y;
}

fn rem_euclid(lhs: f32, rhs: f32) -> f32 {
    let r = lhs % rhs;
    return select(r, r + rhs, r < 0.0);
}

fn angle_between(lhs: vec2<f32>, rhs: vec2<f32>) -> f32 {
    return acos(dot(lhs, rhs) / (length(lhs) * length(rhs))) * sign(cross2d(lhs, rhs));
}

fn sd_donut(p: vec2<f32>, ra: f32, th: f32) -> f32 {
    return abs(length(p) - ra) - th;
}

fn sd_line(p: vec2<f32>, l: vec2<f32>, th: f32) -> f32 {
    return abs(dot(vec2(-l.y, l.x), p)) - th;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var col = vec4(0.05, 0.4, 0.15, 1.0);

    var min_distance = 99999.9;
    var min_length = 0.0;
    for (var i = u32(0); i < arrayLength(&curves); i++) {
        let pos = (in.world_position.xz - curves[i].center);
        let thickness = f32(curves[i].lanes) * (ROAD_WIDTH / 2.0);

        let length = select(
            dot(normalize(curves[i].end), pos - curves[i].start),
            rem_euclid(select(1.0, -1.0, curves[i].twist == 0u) * angle_between(curves[i].start, pos), TAU) * curves[i].radius,
            curves[i].twist != 2u
        );

        var distance = select(
            select(
                sd_line(pos, normalize(curves[i].end), thickness),
                sd_donut(pos, curves[i].radius, thickness),
                curves[i].twist != 2u
            ),
            min(distance(pos, curves[i].start), distance(pos, curves[i].end)) - thickness,
            length < 0.0 || length > curves[i].length
        );


        min_length = select(min_length, length, min_distance > distance);
        min_distance = min(min_distance, distance);
    }

    let texel = textureSample(road_texture, road_sampler, vec2(-min_distance, fract(min_length)));
    col = mix(col, texel, step(min_distance, 0.0));

    return col;
}
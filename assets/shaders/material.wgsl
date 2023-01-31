@group(1) @binding(0)
var<uniform> start: vec2<f32>;

@group(1) @binding(1)
var<uniform> end: vec2<f32>;

@group(1) @binding(2)
var<uniform> normal: vec2<f32>;

struct VertexOutput {
    @builtin(position) coord: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

let PI = 3.141592653589793;
let TAU = 6.28318530718;
let smoothness = 0.002;
let thickness = 0.25;

struct Arc {
    center: vec2<f32>,
    radius: f32,
    angle_start: f32,
    angle_end: f32
};

fn polar(v: vec2<f32>) -> f32 {
    return (atan2(v.y, v.x) + TAU) % TAU;
}

fn between(start: f32, end: f32, alpha: f32) -> f32 {
    if (start == 0.0) {
        return f32(alpha > start && alpha < end);
    }

    if (end > start) {
        return f32(alpha > end || alpha < start);
    }
    
    return f32(alpha > end && alpha < start);
}

fn generate_arc(arc: Arc, coord: vec2<f32>) -> vec4<f32> {
    let color = vec3<f32>(1.0, 0.0, 0.0);
    let innerRadius = arc.radius - thickness;
    let outerRadius = arc.radius + thickness;
    
    let dir = coord - arc.center;
	let dist = length(dir);
    
    // Mask circle
	let inner = step(innerRadius, dist);
	let outer = step(dist, outerRadius);
	let circle_mask = inner * outer;
    
    // Mask arc
    let arc_mask = between(arc.angle_start, arc.angle_end, polar(dir));
    return vec4<f32>(color, circle_mask * arc_mask);
}

fn get_arc_params(start: vec2<f32>, end: vec2<f32>) -> Arc {
    let base = distance(start, end) / 2.0;
    let angle = polar(end - start);
    
    let radius = base / cos(angle);
    let center = start + radius * vec2<f32>(1.0, 0.0);
    let angle_start = polar(start - center);
    let angle_end = polar(end - center);

    return Arc(center, abs(radius), angle_start, angle_end);
}

fn get_local_transform(angle: f32) -> mat2x2<f32> {
    let sine = sin(angle);
    let cosine = cos(angle);
    return mat2x2<f32>(cosine, -sine,
                       sine, cosine);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let rot = get_local_transform(polar(normal));
    let arc = get_arc_params(start * rot, end * rot);
    return generate_arc(arc, in.world_position.xz * rot);
}

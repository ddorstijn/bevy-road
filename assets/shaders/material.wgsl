@group(1) @binding(0)
var<uniform> start: vec2<f32>;

@group(1) @binding(1)
var<uniform> end: vec2<f32>;

@group(1) @binding(2)
var<uniform> tangent: vec2<f32>;

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

fn angle_between(v1: vec2<f32>, v2: vec2<f32>) -> f32 {
    return atan2(v1.y, v1.x) - atan2(v2.y, v2.x);
}

fn polar(v: vec2<f32>) -> f32 {
    return (atan2(v.y, v.x) + TAU) % TAU;
}

fn get_arc_params(start: vec2<f32>, end: vec2<f32>, normal: vec2<f32>) -> Arc {
    let base = distance(start, end) / 2.0;
    let angle = angle_between(normal, end - start);
    
    let radius = base / cos(angle);
    let center = start + normal * radius;
    let angle_start = polar(start - center);
    let angle_end = polar(end - center);

    return Arc(center, abs(radius), angle_start, angle_end);
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

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let normal = vec2<f32>(tangent.y, -tangent.x);
    let arc = get_arc_params(start, end, normal);

    return generate_arc(arc, in.world_position.xz);
}

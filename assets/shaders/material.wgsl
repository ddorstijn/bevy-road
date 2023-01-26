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

struct Circle {
    center: vec2<f32>,
    radius: f32
};

fn angle_between(v1: vec2<f32>, v2: vec2<f32>) -> f32 {
    return atan2(v1.y, v1.x) - atan2(v2.y, v2.x);
}

fn get_circle(start: vec2<f32>, end: vec2<f32>, normal: vec2<f32>) -> Circle {
    let base = distance(start, end) / 2.0;
    let angle = angle_between(normal, end - start);
    let radius = base / cos(angle);
    let center = start + normal * radius;

    return Circle(center, abs(radius));
}

fn generate_arc(circle: Circle, startAngle: f32, endAngle: f32, coord: vec2<f32>) -> vec4<f32> {
    let vColor = vec4<f32>(1.0, 0.0, 0.0, 1.0);
    let innerRadius = circle.radius - thickness;
    let outerRadius = circle.radius + thickness;

    let dir = circle.center - coord;
	let dist = length(dir);

	let inner = smoothstep(innerRadius, innerRadius + smoothness, dist);
	let outer = smoothstep(outerRadius, outerRadius - smoothness, dist);
	let bandAlpha = inner * outer;
    
    let angle = atan2(dir.y, dir.x) + PI;
    let startEdge = smoothstep(angle, angle - smoothness, startAngle);
    let endEdge = smoothstep(angle, angle + smoothness, endAngle);
    let angleAlpha = startEdge * endEdge;
    
    return vec4<f32>(vColor.rgb, bandAlpha * angleAlpha);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let normal = vec2<f32>(tangent.y, -tangent.x);
    let circle = get_circle(start, end, normal);
    let angle_start = angle_between(circle.center - start, circle.center);
    let angle_end = angle_between(circle.center - end, circle.center);

    let t1 = 0.0 * PI;
    let t2 = 2.0 * PI;
    return generate_arc(circle, min(angle_start, angle_end), max(angle_start, angle_end), in.world_position.xz);
}

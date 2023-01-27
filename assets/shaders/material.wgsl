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

fn get_arc_params(start: vec2<f32>, end: vec2<f32>, normal: vec2<f32>) -> Arc {
    let base = distance(start, end) / 2.0;
    let angle = angle_between(normal, end - start);
    let radius = base / cos(angle);
    let center = start + normal * radius;
    let angle_start = angle_between(center - start, center);
    let angle_end = angle_between(center - end, center);

    return Arc(center, abs(radius), angle_start, angle_end);
}

fn generate_arc(circle: Arc, startAngle: f32, endAngle: f32, coord: vec2<f32>) -> vec4<f32> {
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

// sc is the sin/cos of the aperture
fn sdArc(p: vec2<f32>, sc: vec2<f32>, ra: f32, rb: f32) -> f32
{
    let pa = vec2<f32>(abs(p.x), p.y);
    return sign(select(abs(length(pa)-ra) - rb, length(pa-sc*ra), (sc.y*pa.x>sc.x*pa.y)));
}


fn generate_arcs(arc: Arc, coord: vec2<f32>) -> f32 {
    let ac = 0.5 * (arc.angle_end + arc.angle_start); // center
    let aw = 0.5 * abs(arc.angle_end - arc.angle_start); // width

    let scb = vec2<f32>(sin(aw),cos(aw));
    let rot = mat2x2<f32>(sin(ac), -cos(ac), 
                          cos(ac), sin(ac)
                         );
                        
    let dir = arc.center - coord;
    let coord_r = dir * rot;
    return sdArc(coord_r, scb, arc.radius, thickness);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let normal = vec2<f32>(tangent.y, -tangent.x);
    let arc = get_arc_params(start, end, normal);

    //return generate_arc(circle, min(angle_start, angle_end), max(angle_start, angle_end), in.world_position.xz);
    let d =  generate_arcs(arc, in.world_position.xz); 
    return vec4<f32>(vec3<f32>(d), 1.0);
}

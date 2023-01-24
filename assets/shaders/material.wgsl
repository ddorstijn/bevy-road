struct CustomMaterial {
    mouse_position: vec2<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    let dist = distance(world_position.xz, material.mouse_position);
    return vec4<f32>(dist);
}
use bevy::{pbr::wireframe::Wireframe, prelude::*};
use bevy_road_core::load;
use ordered_float::OrderedFloat;

pub struct RoadPlugin;
impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_opendrive);
    }
}

#[derive(Component)]
pub struct RoadComponent(pub bevy_road_core::road::Road);

fn load_opendrive(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let project = load(
        "C:\\Users\\danny\\Documents\\Projects\\Rust\\bevy_road\\opendrive\\tests\\data\\Ex_Slip_Lane.xodr",
    );

    for (id, road) in project.roads {
        let steps = road.length.ceil();
        let step_size = road.length / steps;

        let positions = (0..=steps as u32)
            .flat_map(|step| {
                let road_s = step_size * step as f32;
                let transform = road.interpolate(road_s);

                let (s_section, section) = road.sections.range(..=road_s).next_back().unwrap();

                let left_point = section
                    .left_lanes
                    .values()
                    .map(|lane| {
                        let (s_width, width) =
                            lane.width.range(..=road_s - s_section).next_back().unwrap();

                        width.eval((road_s - s_section - s_width).0)
                    })
                    .sum::<f32>();

                let right_point = section
                    .right_lanes
                    .values()
                    .map(|lane| {
                        let (s_width, width) =
                            lane.width.range(..=road_s - s_section).next_back().unwrap();

                        width.eval((road_s - s_section - s_width).0)
                    })
                    .sum::<f32>();

                let left_point = transform.translation + transform.left() * left_point;
                let right_point = transform.translation + transform.right() * right_point;

                [left_point, right_point]
            })
            .collect::<Vec<_>>();

        let normals = (0..positions.len() as u32)
            .map(|_| Vec3::Y)
            .collect::<Vec<_>>();
        let indices = bevy::render::mesh::Indices::U32(
            (0..positions.len() as u32 - 2)
                .step_by(2)
                .flat_map(|i| [i, i + 1, i + 2, i + 1, i + 3, i + 2])
                .collect::<Vec<u32>>(),
        );

        let mesh = Mesh::new(
            bevy::render::mesh::PrimitiveTopology::TriangleList,
            bevy::render::render_asset::RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        // .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(indices);

        commands.spawn((
            Name::from(id.to_string()),
            PbrBundle {
                mesh: meshes.add(mesh),
                ..default()
            },
            RoadComponent(road),
            Wireframe,
        ));
    }
}

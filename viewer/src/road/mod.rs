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
        "C:\\Users\\danny\\Documents\\Projects\\Rust\\bevy_road\\opendrive\\tests\\data\\highway_example.xodr",
    );

    for (id, road) in project.roads {
        let positions: Vec<Vec3> = (0..)
            .step_by(2)
            .map(|s| (s as f32, road.interpolate(s as f32)))
            .take_while(|(s, _)| s <= &road.length)
            .flat_map(|(road_s, transform)| {
                let road_s = OrderedFloat::<f32>::from(road_s);
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
            .collect();

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

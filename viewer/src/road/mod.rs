use bevy::{
    input::common_conditions::input_just_pressed,
    // pbr::wireframe::Wireframe,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use bevy_road_core::load;
use futures_lite::future;
use rfd::{AsyncFileDialog, FileHandle};

pub struct RoadPlugin;
impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            open_xodr_dialog.run_if(input_just_pressed(KeyCode::KeyO)),
        )
        .add_systems(
            Update,
            load_opendrive.run_if(any_with_component::<FileOpenTask>),
        );
    }
}

#[derive(Component)]
pub struct RoadComponent(pub bevy_road_core::road::Road);

#[derive(Component)]
struct FileOpenTask(Task<Option<FileHandle>>);

fn open_xodr_dialog(mut commands: Commands) {
    let thread_pool = AsyncComputeTaskPool::get();
    let task = thread_pool.spawn(
        AsyncFileDialog::new()
            .add_filter("OpenDrive", &["xodr"])
            .pick_file(),
    );
    commands.spawn(FileOpenTask(task));
}

fn load_opendrive(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut tasks: Query<(Entity, &mut FileOpenTask)>,
    roads: Query<Entity, With<RoadComponent>>,
) {
    for (entity, mut selected_file) in tasks.iter_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut selected_file.0)) {
            commands.entity(entity).despawn();

            // Do your thing
            if let Some(handle) = result {
                for entity in &roads {
                    commands.entity(entity).despawn();
                }

                let project = load(handle.path());

                for (id, road) in project.roads {
                    let steps = road.length.ceil() * 10.0;
                    let step_size = road.length / steps;

                    let positions = (0..=steps as u32)
                        .flat_map(|step| {
                            let road_s = step_size * step as f32;
                            let transform = road.interpolate(road_s);

                            let (s_section, section) =
                                road.sections.range(..=road_s).next_back().unwrap();

                            let left_point = section
                                .left_lanes
                                .values()
                                .map(|lane| {
                                    let (s_width, width) = lane
                                        .width
                                        .range(..=road_s - s_section)
                                        .next_back()
                                        .unwrap();

                                    width.eval((road_s - s_section - s_width).0)
                                })
                                .sum::<f32>();

                            let right_point = section
                                .right_lanes
                                .values()
                                .map(|lane| {
                                    let (s_width, width) = lane
                                        .width
                                        .range(..=road_s - s_section)
                                        .next_back()
                                        .unwrap();

                                    width.eval((road_s - s_section - s_width).0)
                                })
                                .sum::<f32>();

                            let left_point = transform.translation + transform.left() * left_point;
                            let right_point =
                                transform.translation + transform.right() * right_point;

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
                        // Wireframe,
                    ));
                }
            };
        }
    }
}

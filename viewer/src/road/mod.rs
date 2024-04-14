use bevy::{
    input::common_conditions::input_just_pressed,
    // pbr::wireframe::Wireframe,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use bevy_road_core::{road::Road, BevyRoad};
use futures_lite::future;
use rfd::{AsyncFileDialog, FileHandle};

pub struct RoadPlugin;
impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BevyRoad>().add_systems(
            Update,
            (
                open_xodr_dialog.run_if(input_just_pressed(KeyCode::KeyO)),
                load_opendrive.run_if(any_with_component::<FileOpenTask>),
                spawn_mesh_for_road,
            ),
        );
    }
}

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
    mut bevy_road: ResMut<BevyRoad>,
    mut tasks: Query<(Entity, &mut FileOpenTask)>,
) {
    for (entity, mut selected_file) in tasks.iter_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut selected_file.0)) {
            commands.entity(entity).despawn();

            // Do your thing
            if let Some(handle) = result {
                *bevy_road = BevyRoad::from_xodr(handle.path(), commands.reborrow()).unwrap();
            };
        }
    }
}

fn spawn_mesh_for_road(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    road: Query<&Road, Added<Road>>,
) {
    for road in &road {
        commands.spawn(PbrBundle {
            mesh: meshes.add(road.mesh()),
            ..Default::default()
        });
    }
}

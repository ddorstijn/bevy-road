use bevy::{input::common_conditions::input_just_pressed, prelude::*, window::PrimaryWindow};
use bevy_road_core::geometry::{Geometry, GeometryType};

pub struct BuilderPlugin;
impl Plugin for BuilderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Elevation>()
            .add_systems(
                Update,
                (
                    change_level,
                    insert_point.run_if(input_just_pressed(MouseButton::Left)),
                    debug_spline_points,
                ),
            )
            .add_systems(FixedUpdate, regenerate_curves);
    }
}

#[derive(Resource, Default, Reflect)]
struct Elevation(f32);

#[derive(Component, Default, Debug)]
struct RoadSpline {
    points: Vec<Vec3>,
    geometry: Vec<Geometry>,
    // road_id: u32,
}

#[derive(Component)]
struct ActiveSpline;

impl RoadSpline {
    pub fn generate_curve(&mut self) {
        self.geometry.clear();

        if self.points.len() == 1 {
            return;
        }

        if self.points.len() == 2 {
            // Straight road
            let p1 = Vec2::new(self.points[0].x, -self.points[0].z);
            let p2 = Vec2::new(self.points[1].x, -self.points[1].z);

            self.geometry.push(Geometry {
                hdg: (p2 - p1).to_angle(),
                s: 0.0,
                length: p1.distance(p2),
                x: p1.x,
                y: p1.y,
                r#type: GeometryType::Line,
            });
        }

        if self.points.len() > 2 {
            // Combination of straights with curves
        }
    }
}

fn change_level(mut elevation: ResMut<Elevation>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.pressed(KeyCode::NumpadAdd) {
        elevation.0 += 1.;
    }

    if keys.pressed(KeyCode::NumpadSubtract) {
        elevation.0 -= 1.;
    }
}

fn insert_point(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    main_camera: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    elevation: Res<Elevation>,
    mut active_spline: Query<(Entity, &mut RoadSpline), With<ActiveSpline>>,
) {
    let Some(cursor_position) = primary_window.single().cursor_position() else {
        return;
    };

    let (camera, camera_transform) = main_camera.single();
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let p = ray.get_point((elevation.0 - ray.origin.y) / ray.direction.y);

    if keys.pressed(KeyCode::ShiftLeft) {
        for (entity, _) in &active_spline {
            commands.entity(entity).remove::<ActiveSpline>();
        }

        commands.spawn((
            SpatialBundle::default(),
            RoadSpline {
                points: vec![p],
                ..default()
            },
            ActiveSpline,
        ));
    } else {
        for (_, mut spline) in &mut active_spline {
            spline.points.push(p);
        }
    }
}

fn regenerate_curves(mut splines: Query<&mut RoadSpline, Changed<RoadSpline>>) {
    for mut spline in &mut splines {
        spline.generate_curve();
    }
}

fn debug_spline_points(
    mut gizmos: Gizmos,
    inactive_splines: Query<&RoadSpline, Without<ActiveSpline>>,
    active_splines: Query<&RoadSpline, With<ActiveSpline>>,
) {
    for spline in &inactive_splines {
        for pos in &spline.points {
            gizmos.cuboid(
                Transform::from_translation(*pos).with_scale(Vec3::splat(0.1)),
                Color::GRAY,
            );
        }
    }

    for spline in &active_splines {
        for pos in &spline.points {
            gizmos.cuboid(
                Transform::from_translation(*pos).with_scale(Vec3::splat(0.1)),
                Color::GREEN,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

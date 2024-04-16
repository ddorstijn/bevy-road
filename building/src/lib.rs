use std::collections::BTreeMap;

use bevy::{input::common_conditions::input_just_pressed, prelude::*, window::PrimaryWindow};
use bevy_road_core::geometry::{Geometry, GeometryType};
use ordered_float::OrderedFloat;

const CURVATURE: f32 = 0.5;
const RADIUS: f32 = 1.0 / CURVATURE;
const L_S: f32 = 0.5;

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
                    display_spline,
                ),
            )
            .add_systems(FixedUpdate, regenerate_curves);
    }
}

fn display_spline(splines: Query<&RoadSpline>, mut gizmos: Gizmos) {
    for spline in &splines {
        for geom in spline.geometry.values() {
            let steps = geom.length.ceil() * 10.0;
            let step_size = geom.length / steps;
            let points = (0..=steps as u32)
                .map(|step| {
                    let (x, y, _) = geom.interpolate(step_size * step as f32);
                    Vec3::new(x, 0.0, -y)
                })
                .collect::<Vec<_>>();

            gizmos.linestrip(
                points,
                match geom.r#type {
                    GeometryType::Line => Color::BLACK,
                    GeometryType::Arc { .. } => Color::LIME_GREEN,
                    GeometryType::Spiral { .. } => Color::CYAN,
                },
            );
        }
    }
}

#[derive(Resource, Default, Reflect)]
struct Elevation(f32);

#[derive(Component, Default, Debug)]
struct RoadSpline {
    points: Vec<Vec3>,
    geometry: BTreeMap<OrderedFloat<f32>, Geometry>,
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

            self.geometry.insert(
                OrderedFloat(0.0),
                Geometry {
                    hdg: (p2 - p1).to_angle(),
                    s: 0.0,
                    length: p1.distance(p2),
                    x: p1.x,
                    y: p1.y,
                    r#type: GeometryType::Line,
                },
            );
        }

        if self.points.len() > 2 {
            let mut p1: Vec2 = Vec2::new(self.points[0].x, -self.points[0].z);

            // Combination of straights with curves
            self.points.windows(3).for_each(|s| {
                let [_, p2, p3] = s else { return };

                let p2 = Vec2::new(p2.x, -p2.z);
                let p3 = Vec2::new(p3.x, -p3.z);

                let v1 = p2 - p1;
                let v2 = p3 - p2;

                let angle = v1.angle_between(v2).abs();
                let ts = 0.5 * L_S + (RADIUS + L_S.powi(2) / 24.0 * RADIUS) * (angle * 0.5).tan();
                let arc_length = (angle - L_S / RADIUS) * RADIUS;

                let twist = v1.perp_dot(v2).signum();
                let k = CURVATURE * twist;

                let l_in = Geometry {
                    s: 0.0,
                    hdg: v1.to_angle(),
                    x: p1.x,
                    y: p1.y,
                    length: v1.length() - ts,
                    r#type: GeometryType::Line,
                };

                let (x, y, hdg) = l_in.interpolate(l_in.length);

                let s_in = Geometry {
                    s: l_in.s + l_in.length,
                    hdg,
                    length: L_S,
                    x,
                    y,
                    r#type: GeometryType::new_spiral(0.0, k, L_S),
                };

                let (x, y, hdg) = s_in.interpolate(s_in.length);

                let a_c = Geometry {
                    s: s_in.s + s_in.length,
                    hdg,
                    length: arc_length,
                    x,
                    y,
                    r#type: GeometryType::Arc { k },
                };

                let (x, y, hdg) = a_c.interpolate(a_c.length);

                let s_out = Geometry {
                    s: a_c.s + a_c.length,
                    hdg,
                    length: L_S,
                    x,
                    y,
                    r#type: GeometryType::new_spiral(k, 0.0, L_S),
                };

                p1 = p2 + v2.normalize() * ts;

                let (x, y, _) = s_out.interpolate(s_out.length);
                println!("Error: {}", p1.distance(Vec2::new(x, y)));

                self.geometry.insert(OrderedFloat(l_in.s), l_in);
                self.geometry.insert(OrderedFloat(s_in.s), s_in);
                self.geometry.insert(OrderedFloat(a_c.s), a_c);
                self.geometry.insert(OrderedFloat(s_out.s), s_out);
            });

            let last_point = self.points.last().unwrap();
            let last_geom = self.geometry.iter().rev().next().unwrap().1;
            let p2 = Vec2::new(last_point.x, -last_point.z);
            let v2 = p2 - p1;

            let l_out = Geometry {
                s: last_geom.s + last_geom.length,
                hdg: v2.to_angle(),
                x: p1.x,
                y: p1.y,
                length: v2.length(),
                r#type: GeometryType::Line,
            };

            self.geometry.insert(OrderedFloat(99999.), l_out);
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

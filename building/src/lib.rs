use std::collections::BTreeMap;

use bevy::{
    input::common_conditions::{input_just_pressed, input_pressed},
    math::{DVec2, DVec3},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_road_core::geometry::{Geometry, GeometryType};
use ordered_float::OrderedFloat;

pub struct BuilderPlugin;
impl Plugin for BuilderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Elevation>()
            .add_systems(Startup, spawn_curve)
            .add_systems(
                Update,
                (
                    stop_spline
                        .run_if(input_just_pressed(KeyCode::Escape))
                        .after(update_point),
                    change_level,
                    change_radius.run_if(input_pressed(MouseButton::Middle)),
                    insert_point.run_if(input_just_pressed(MouseButton::Left)),
                    debug_spline_points,
                    display_spline,
                    display_arc,
                    update_point.run_if(any_with_component::<ActiveSpline>),
                ),
            )
            .add_systems(FixedUpdate, regenerate_curves);
    }
}

fn spawn_curve(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        RoadSpline {
            points: vec![
                SplinePoint::default(),
                SplinePoint {
                    position: DVec3::new(500.0, 0.0, 0.0),
                    ..default()
                },
                SplinePoint {
                    position: DVec3::new(500.0, 0.0, 500.0),
                    ..default()
                },
            ],
            ..default()
        },
        // ActiveSpline,
    ));
}

fn stop_spline(
    mut active_spline: Query<(Entity, &mut RoadSpline), With<ActiveSpline>>,
    mut commands: Commands,
) {
    for (entity, mut spline) in &mut active_spline {
        if spline.points.len() < 2 {
            commands.entity(entity).despawn();
        } else {
            spline.points.pop();
            commands.entity(entity).remove::<ActiveSpline>();
        }
    }
}

fn change_radius(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    main_camera: Query<(&Camera, &GlobalTransform)>,
    mut splines: Query<&mut RoadSpline>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Some(cursor_position) = primary_window.single().cursor_position() else {
        return;
    };

    let (camera, camera_transform) = main_camera.single();
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let p = ray.get_point(-ray.origin.y / ray.direction.y);
    let p = DVec3::from(p);

    for mut spline in &mut splines {
        if let Some(point) = spline
            .points
            .iter_mut()
            .find(|point| point.position.distance_squared(p) < 10.0)
        {
            match keys.pressed(KeyCode::ShiftLeft) {
                true => point.radius += 1.0,
                false => point.radius -= 1.0,
            }
        }
    }
}

fn display_arc(splines: Query<&RoadSpline>, mut gizmos: Gizmos) {
    for spline in &splines {
        spline.geometry.values().for_each(|g| match g.r#type {
            GeometryType::Arc { k } => {
                let radius = k.recip();
                let recipient = g.hdg + std::f64::consts::PI / 2.0;

                let (sin_hdg, cos_hdg) = recipient.sin_cos();
                let x = g.x + cos_hdg * radius;
                let y = g.y + sin_hdg * radius;
                let center = Vec3::new(x as f32, 0.0, -y as f32);

                gizmos.circle(center, Direction3d::Y, 0.1, Color::PINK);
                gizmos.circle(center, Direction3d::Y, radius as f32, Color::ORANGE);
            }
            _ => (),
        })
    }
}

fn display_spline(splines: Query<&RoadSpline>, mut gizmos: Gizmos) {
    for spline in &splines {
        spline.geometry.values().for_each(|g| match g.r#type {
            GeometryType::Line => {
                let (x, y, _) = g.interpolate(g.length);
                let start = Vec3::new(g.x as f32, 0.0, -g.y as f32);
                let end = Vec3::new(x as f32, 0.0, -y as f32);
                gizmos.line(start, end, Color::CYAN);
            }
            _ => {
                let steps = g.length.ceil() * 10.;
                let step_size = g.length / steps;
                let positions = (0..=steps as u32)
                    .map(|step| {
                        let (x, y, _) = g.interpolate(step_size * step as f64);
                        Vec3::new(x as f32, 0.0, -y as f32)
                    })
                    .collect::<Vec<Vec3>>();

                gizmos.linestrip(
                    positions,
                    match g.r#type {
                        GeometryType::Arc { .. } => Color::RED,
                        GeometryType::Spiral { .. } => Color::GREEN,
                        GeometryType::Line => unreachable!(),
                    },
                )
            }
        });
    }
}

#[derive(Resource, Default, Reflect)]
struct Elevation(f64);

#[derive(Debug)]
struct SplinePoint {
    position: DVec3,
    radius: f64,
    transition_length: f64,
}

impl Default for SplinePoint {
    fn default() -> Self {
        Self {
            position: DVec3::ZERO,
            radius: 20.0,
            transition_length: 25.0,
        }
    }
}

#[derive(Component, Default, Debug)]
struct RoadSpline {
    points: Vec<SplinePoint>,
    length: f64,
    geometry: BTreeMap<OrderedFloat<f64>, Geometry>,
    // road_id: u32,
}

#[derive(Component)]
struct ActiveSpline;

impl RoadSpline {
    pub fn generate_curve(&mut self) {
        self.geometry.clear();
        self.length = 0.0;

        if self.points.len() == 1 {
            return;
        }

        if self.points.len() == 2 {
            // Straight road
            let p1 = DVec2::new(self.points[0].position.x, -self.points[0].position.z);
            let p2 = DVec2::new(self.points[1].position.x, -self.points[1].position.z);
            self.length = p1.distance(p2);

            self.geometry.insert(
                OrderedFloat(0.0),
                Geometry {
                    hdg: (p2 - p1).to_angle(),
                    s: 0.0,
                    length: self.length,
                    x: p1.x,
                    y: p1.y,
                    r#type: GeometryType::Line,
                },
            );
        }

        if self.points.len() > 2 {
            let mut p1 = DVec2::new(self.points[0].position.x, -self.points[0].position.z);

            // Combination of straights with curves
            self.points.windows(3).for_each(|s| {
                let [_, p2, p3] = s else { return };

                let radius = p2.radius;
                let curvature = radius.recip();
                let transition_length = p2.transition_length;

                let p2 = DVec2::new(p2.position.x, -p2.position.z);
                let p3 = DVec2::new(p3.position.x, -p3.position.z);

                let v1 = p2 - p1;
                let v2 = p3 - p2;
                let v1_heading = v1.to_angle();

                let angle = v1.angle_between(v2).abs();
                let shift = transition_length.powi(2) / (24.0 * radius);
                let ts = transition_length / 2.0 + (radius + shift) * (angle / 2.0).tan();
                let ts_station = p2 - v1.normalize() * ts;

                let twist = v1.perp_dot(v2).signum();
                let k = curvature * twist;

                let l_in = Geometry {
                    s: self.length,
                    hdg: v1_heading,
                    x: p1.x,
                    y: p1.y,
                    length: v1.length() - ts,
                    r#type: GeometryType::Line,
                };
                self.length += l_in.length;

                let s_in = Geometry {
                    s: l_in.s + l_in.length,
                    hdg: v1_heading,
                    length: transition_length,
                    x: ts_station.x,
                    y: ts_station.y,
                    r#type: GeometryType::new_spiral(0.0, k, transition_length),
                };
                self.length += s_in.length;

                let (x, y, hdg) = s_in.interpolate(s_in.length);
                let spiral_angle = transition_length / (2.0 * radius);
                let arc_angle = angle - 2.0 * spiral_angle;
                let a_c = Geometry {
                    s: s_in.s + s_in.length,
                    hdg,
                    length: arc_angle * radius,
                    x,
                    y,
                    r#type: GeometryType::Arc { k },
                };

                self.length += a_c.length;

                let (x, y, hdg) = a_c.interpolate(a_c.length);
                let s_out = Geometry {
                    s: a_c.s + a_c.length,
                    hdg,
                    length: transition_length,
                    x,
                    y,
                    r#type: GeometryType::new_spiral(k, 0.0, transition_length),
                };

                self.length += s_out.length;

                p1 = p2 + v2.normalize() * ts;

                self.geometry.insert(OrderedFloat(l_in.s), l_in);
                self.geometry.insert(OrderedFloat(s_in.s), s_in);
                self.geometry.insert(OrderedFloat(a_c.s), a_c);
                self.geometry.insert(OrderedFloat(s_out.s), s_out);
            });

            let last_point = self.points.last().unwrap();
            let p2 = DVec2::new(last_point.position.x, -last_point.position.z);
            let v2 = p2 - p1;

            let l_out = Geometry {
                s: self.length,
                hdg: v2.to_angle(),
                x: p1.x,
                y: p1.y,
                length: v2.length(),
                r#type: GeometryType::Line,
            };

            self.length += l_out.length;
            self.geometry.insert(OrderedFloat(self.length), l_out);
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

fn update_point(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    main_camera: Query<(&Camera, &GlobalTransform)>,
    mut active_spline: Query<(Entity, &mut RoadSpline), With<ActiveSpline>>,
) {
    let Some(cursor_position) = primary_window.single().cursor_position() else {
        return;
    };

    let (camera, camera_transform) = main_camera.single();
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let p = ray.get_point(-ray.origin.y / ray.direction.y);

    for (_, mut spline) in &mut active_spline {
        *spline.points.last_mut().unwrap() = SplinePoint {
            position: DVec3::new(p.x as f64, 0.0, p.z as f64),
            ..default()
        };
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

    let p = ray.get_point((elevation.0 as f32 - ray.origin.y) / ray.direction.y);
    let p = DVec3::new(p.x as f64, 0.0, p.z as f64);

    if keys.pressed(KeyCode::ShiftLeft) {
        for (entity, _) in &active_spline {
            commands.entity(entity).remove::<ActiveSpline>();
        }

        commands.spawn((
            SpatialBundle::default(),
            RoadSpline {
                points: vec![SplinePoint {
                    position: p,
                    ..default()
                }],
                ..default()
            },
            ActiveSpline,
        ));
    } else {
        for (_, mut spline) in &mut active_spline {
            spline.points.push(SplinePoint {
                position: p,
                ..default()
            });
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
                Transform::from_xyz(pos.position.x as f32, 0.0, pos.position.z as f32)
                    .with_scale(Vec3::splat(0.1)),
                Color::GRAY,
            );
        }
    }

    for spline in &active_splines {
        for pos in &spline.points {
            gizmos.cuboid(
                Transform::from_xyz(pos.position.x as f32, 0.0, pos.position.z as f32)
                    .with_scale(Vec3::splat(0.1)),
                Color::GREEN,
            );
        }
    }
}

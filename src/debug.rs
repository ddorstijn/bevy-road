use std::{f32::consts::PI, sync::Arc};

use bevy::{prelude::*, render::render_resource::PrimitiveTopology};
use bevy_mod_raycast::{immediate::Raycast, CursorRay, DefaultRaycastingPlugin};
use bevy_prototype_debug_lines::*;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DebugLinesPlugin::default())
            .add_plugins(DefaultRaycastingPlugin)
            .add_systems(Startup, test_arc_scene)
            .add_systems(Update, raycast);
    }
}

fn test_arc_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut lines: ResMut<DebugLines>,
) {
    let point2 = commands.spawn((
        RoadSegment {
            next: None,
            pbr: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(0.25, 0.25, 0.25))),
                material: materials.add(Color::rgb(1., 1., 1.).into()),
                transform: Transform::from_translation(Vec3::new(-2.0, 0.0, -2.0)),
                ..default()
            },
        },
        Name::new("Point 2"),
    )).id();

    commands.spawn((
        RoadSegment {
            next: Some(point2),
            pbr: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(0.25, 0.25, 0.25))),
                material: materials.add(Color::rgb(1., 1., 1.).into()),
                transform: Transform::from_translation(Vec3::new(-2.0, 0.0, -2.0)),
                ..default()
            },
        },
        Name::new("Point 1"),
    ));
    

    // Axis
    lines.line(
        Vec3::new(-0.1, 0., 0.),
        Vec3::new(0.1, 0., 0.),
        f32::INFINITY,
    );
    lines.line(
        Vec3::new(0., -0.1, 0.),
        Vec3::new(0., 0.1, 0.),
        f32::INFINITY,
    );
    lines.line(
        Vec3::new(0., 0., -0.1),
        Vec3::new(0., 0., 0.1),
        f32::INFINITY,
    );
}

#[derive(Component)]
struct RoadSegment {
    pbr: PbrBundle, 
}

impl RoadSegment {
    fn generate_arc_points(&self, center: Vec3, endpoint: &RoadSegment) -> Vec<Vec3> {
        let start = self.pbr.transform.translation;
        let end = endpoint.pbr.transform.translation;
        
        let radius = (start - center).length();
        let start_angle = (start.z - center.z).atan2(start.x - center.x);
        let end_angle = (end.z - center.z).atan2(end.x - center.x);

        let clockwise = self.pbr.global_transform.forward().angle_between(end - start).is_sign_negative();
        let (start_angle, end_angle) = match clockwise {
            true if start_angle < end_angle => (start_angle + 2.0 * PI, end_angle),
            false if end_angle < start_angle => (start_angle, end_angle + 2.0 * PI),
            _ => (start_angle, end_angle),
        };
    
        let arc_length = radius * (end_angle - start_angle).abs();
    
        // Calculate the number of steps, ensuring there is approximately 1 unit arc length between points
        let num_steps = arc_length.ceil() as usize;
        // Adjust the step size based on the total arc length and number of steps
        let step = (end_angle - start_angle).abs() / num_steps as f32;
    
        (0..=num_steps)
            .map(|i| {
                let inc = step * i as f32;
                let angle = start_angle + inc;
                let x = center.x + radius * angle.cos();
                let y = start.y;
                let z = center.z + radius * angle.sin();
                Vec3::new(x, y, z)
            })
            .collect()
    }

    fn generate_line_points(&self) -> Vec<Vec3> {
        unimplemented!()
    }

    fn get_center(&self, endpoint: &RoadSegment) -> Option<Vec3> {
        let start: Vec3 = self.pbr.transform.translation;
        let start_direction: Vec3 = self.pbr.global_transform.right();
        let end = endpoint.pbr.transform.translation;
        let midpoint: Vec3 = (end + start) / 2.0;
        let bisector: Vec3 = (end - start).any_orthogonal_vector().normalize();;

        let direction = midpoint - start;
        let cross1 = start_direction.cross(bisector);
        let cross2 = direction.cross(bisector);
    
        let planar_factor = direction.dot(cross1).abs();
    
        //is coplanar, and not parrallel
        if planar_factor < 0.0001 && cross1.length_squared() > 0.0001 {
            let s = cross2.dot(cross1) / cross1.length_squared();
            Some(start + (start_direction * s))
        } else {
            None
        }
    }

    fn generate_mesh(&self, end: &RoadSegment) -> Mesh {
        let points = match self.get_center(end) {
            Some(center) => self.generate_arc_points(center, end),
            None => self.generate_line_points()
        };

        let detail = points.len();
        let normals = vec![Vec3::Y; detail + 1];
        let indices = (0..=detail as u32).collect::<Vec<u32>>();
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));

        mesh
    }
}

fn raycast(mut query: Query<(&RoadSegment, &mut Handle<Mesh>)>, query2: Query<&RoadSegment>, mut meshes: ResMut<Assets<Mesh>>, cursor_ray: Res<CursorRay>, mut raycast: Raycast) {
    if let Some(cursor_ray) = **cursor_ray {
        if let Some((_, hit)) = raycast.cast_ray(cursor_ray, &default()).first() {
            for (road, mut handle) in &mut query {
                if let Some(end) = road.next {
                    let component = query2.get_component::<RoadSegment>(end).unwrap();
                    *handle = meshes.add(road.generate_mesh(component));
                }
            } 
        }
    }
}
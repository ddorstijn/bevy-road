use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct RoadEdge {
    start: GlobalTransform,
    center: Vec3,
    radius: f32,
    clockwise: bool,
    pub length: f32,
}

impl RoadEdge {
    pub fn new(start: &GlobalTransform, endpoint: Vec3, gizmos: Option<Gizmos>) -> Self {
        let startpoint = start.translation();
        let midpoint = (endpoint + startpoint) / 2.0;
        let direction = endpoint - startpoint;
        let bisector = direction.any_orthogonal_vector().normalize();

        let dir2d = start.right().xz();
        let mat = Mat2::from_cols(dir2d, bisector.xz() * -1.0);

        let clockwise = start.forward().xz().angle_between(direction.xz()).is_sign_negative();
        
        let (center, radius, length) = match std::panic::catch_unwind(|| mat.inverse()) { 
            Ok(inverse) => {
                let rhs = (midpoint - startpoint).xz();
                let result = inverse * rhs;
                let center = startpoint.xz() + dir2d * result.x;
                let center = center.extend(0.0).xzy();
                
                let start_angle = (startpoint.z - center.z).atan2(startpoint.x - center.x);
                let end_angle = (endpoint.z - center.z).atan2(endpoint.x - center.x);
                let (start_angle, end_angle) = match clockwise {
                    true if start_angle < end_angle => (start_angle + 2.0 * PI, end_angle),
                    false if end_angle < start_angle => (start_angle, end_angle + 2.0 * PI),
                    _ => (start_angle, end_angle),
                };

                let angle = (end_angle - start_angle).abs();
                let radius = (center - startpoint).length();

                if gizmos.is_some() {
                    let mut gizmos = gizmos.unwrap();
        
                    gizmos.ray(startpoint, start.right() * 10., Color::WHITE);
                    gizmos.ray(startpoint, midpoint, Color::GREEN);
                    gizmos.ray(midpoint, bisector * 1.0, Color::RED);
                    gizmos.sphere(center, Quat::IDENTITY, 0.1, Color::BLUE);
                    gizmos.sphere(center, Quat::IDENTITY, radius, Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 0.25 });
                }

                (center, radius, radius * angle)
            },
            _ => (midpoint, 0.0, direction.length())
        };

        Self {
            start: start.clone(),
            center,
            radius,
            clockwise,
            length,
        }
    }

    fn interpolate_arc(&self, length: f32, offset: f32) -> Transform {
        let mut transform = self.start.compute_transform();

        let length = match length > self.length {
            true => self.length,
            false => length
        };

        transform.rotate_around(
            self.center,
            Quat::from_axis_angle(self.start.up(), (f32::from(self.clockwise) * 2.0 - 1.0) * length / self.radius),
        );

        transform.translation += (transform.translation - self.center).normalize() * offset;
        transform
    }

    fn interpolate_line(&self, length: f32, offset: f32) -> Transform {
        self.start
            .compute_transform()
            .with_translation(self.start.forward() * length + self.start.right() * offset)
    }

    pub fn interpolate(&self, length: f32, offset: f32) -> Transform {
        match self.radius < 0.005 {
            true => self.interpolate_line(length, offset),
            false => self.interpolate_arc(length, offset),
        }
    }

    pub fn get_end_transform(&self) -> Transform {
        self.interpolate(self.length, 0.0)
    }

    pub fn recalculate(&mut self, endpoint: Vec3, gizmos: Option<Gizmos>) {
        *self = Self::new(&self.start, endpoint, gizmos);
    }

    pub fn generate_mesh(&self) -> Mesh {
        // let points = (0..=self.length.ceil() as usize)
        //     .flat_map(|i| {
        //         [
        //             self.interpolate(i as f32, -0.5).translation,
        //             self.interpolate(i as f32, 0.5).translation,
        //         ]
        //     })
        //     .collect::<Vec<Vec3>>();

        let points = (0..=self.length.ceil() as usize)
            .map(|i| self.interpolate(i as f32, 0.0).translation)
            .collect::<Vec<Vec3>>();

        let detail = points.len();
        let normals = vec![Vec3::Y; detail];
        let indices = (0..detail as u32).collect::<Vec<u32>>();
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));

        mesh
    }
}

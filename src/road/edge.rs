use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;

const PRECISION: f32 = 0.001;

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct RoadEdge {
    pub radius: f32,
    pub length: f32,
}

impl RoadEdge {
    pub fn new(endpoint: Vec3) -> Self {
        let endpoint2d = endpoint.xz();
        let midpoint = endpoint2d / 2.0;
        let bisector = midpoint.perp();

        // The line is parrallel to te x-axis
        if bisector.y.abs() < PRECISION {
            return Self {
                radius: 0.0,
                length: match endpoint.z.is_sign_negative() {
                    true => endpoint.length(),
                    false => 0.0,
                },
            };
        }

        // Calculate radius for y = 0.
        // x = midpoint.x + bisector.x * t
        // y = midpoint.y + bisector.y * t = 0
        // t = -midpoint.y / bisector.y
        // x = midpoint.x + bisector.x * (-midpoint.y / bisector.y)
        let radius = midpoint.x + bisector.x * (-midpoint.y / bisector.y);

        // Get the vector from center to endpoint, but flip along the x-axis in case center is positive.
        // This is necessary because the angle is calculated counter-clockwise and from the positive x axis.
        // We want the angle clockwise because z is inverted
        let reciprocal = match radius.is_sign_positive() {
            true => Vec2::new(-endpoint2d.x + radius, endpoint2d.y),
            false => Vec2::new(endpoint2d.x - radius, endpoint2d.y),
        };

        let angle = reciprocal.y.atan2(reciprocal.x);

        // atan returns angle between [-PI, PI], transform it to [0, 2PI]
        let angle = match angle.is_sign_positive() {
            true => 2.0 * PI - angle,
            false => angle.abs(),
        };

        let length = (angle * radius).abs();

        Self { radius, length }
    }

    fn interpolate_arc(&self, length: f32, offset: f32) -> Transform {
        let length = match length > self.length {
            true => self.length,
            false => length,
        };

        let center = Vec3::new(self.radius, 0.0, 0.0);
        let angle = -length / self.radius;

        let mut transform = Transform::default();
        transform.rotate_around(center, Quat::from_axis_angle(Vec3::Y, angle));
        transform.translation += (center - transform.translation).normalize() * offset;

        transform
    }

    fn interpolate_line(&self, length: f32, offset: f32) -> Transform {
        let length = match length > self.length {
            true => self.length,
            false => length,
        };

        Transform::default().with_translation(-Vec3::Z * length + Vec3::X * offset)
    }

    pub fn interpolate(&self, length: f32, offset: f32) -> Transform {
        match self.radius.abs() < PRECISION {
            true => self.interpolate_line(length, offset),
            false => self.interpolate_arc(length, offset),
        }
    }

    pub fn get_end_transform(&self) -> Transform {
        self.interpolate(self.length, 0.0)
    }

    pub fn generate_mesh(&self) -> Mesh {
        const RESOLUTION: usize = 10;
        let n = self.length.ceil() as usize * RESOLUTION;
        let points = (0..=n)
            .flat_map(|i| {
                [
                    self.interpolate(i as f32 / RESOLUTION as f32, -0.25)
                        .translation,
                    self.interpolate(i as f32 / RESOLUTION as f32, 0.25)
                        .translation,
                ]
            })
            .collect::<Vec<Vec3>>();

        let normals = vec![Vec3::Y; points.len()];
        let indices = (0..n as u32 * 2 - 1)
            .flat_map(|i| [i, i + 1, i + 2, i + 1, i + 3, i + 2])
            .collect::<Vec<u32>>();
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));

        mesh
    }
}

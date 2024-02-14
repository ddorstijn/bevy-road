use std::f32::consts::PI;

use bevy::{prelude::*, render::render_resource::PrimitiveTopology};

#[derive(Component)]
pub struct RoadEdge {
    center: Vec3,
    start_angle: f32,
    end_angle: f32,
    radius: f32,
    length: f32,
    clockwise: bool,
}

impl RoadEdge {
    pub fn new(start: &GlobalTransform, endpoint: Vec3) -> Self {
        let startpoint = start.translation();
        let midpoint: Vec3 = (endpoint + startpoint) / 2.0;
        let bisector: Vec3 = (endpoint - startpoint).any_orthogonal_vector().normalize();

        let direction = midpoint - startpoint;
        let cross1 = start.right().cross(bisector);
        let cross2 = direction.cross(bisector);
    
        let planar_factor = direction.dot(cross1).abs();
    
        //is coplanar, and not parrallel
        let (center, radius) = match planar_factor < 0.0001 && cross1.length_squared() > 0.0001 {
            true => {
                let s = cross2.dot(cross1) / cross1.length_squared();
                let center = startpoint + start.right() * s;
                (center, (startpoint - center).length())
            },
            false => (midpoint,  0.0)
        };

        let clockwise = start.forward().angle_between(endpoint - startpoint).is_sign_negative();
        let start_angle = (startpoint.z - center.z).atan2(startpoint.x - center.x);
        let end_angle = (endpoint.z - center.z).atan2(endpoint.x - center.x);
        let (start_angle, end_angle) = match clockwise {
            true if start_angle < end_angle => (start_angle + 2.0 * PI, end_angle),
            false if end_angle < start_angle => (start_angle, end_angle + 2.0 * PI),
            _ => (start_angle, end_angle),
        };

        Self {
            center,
            start_angle,
            end_angle,
            radius,
            clockwise,
            length: radius * (end_angle - start_angle).abs(),
        }
    }

    fn interpolate_arc(&self, length: f32) -> Transform {   
        let inc = length / self.radius * (f32::from(self.clockwise) * 2. - 1.);
        let angle = self.start_angle + inc;
        let x = self.center.x + self.radius * angle.cos();
        let y = self.center.y;
        let z = self.center.z + self.radius * angle.sin();

        let position = Vec3::new(x, y, z);
        let radius_vector = position - self.center;
        let up_vector = Vec3::new(0.0, 1.0, 0.0);
        let forward_vector = up_vector.cross(radius_vector).normalize();
        let rotation = Quat::from_rotation_arc(Vec3::new(0.0, 0.0, -1.0), forward_vector);

        let mut transform = Transform::default();
        transform.translation = position;
        transform.rotation = rotation;
        transform
    }

    fn interpolate_line(&self, length: f32) -> Transform {
        unimplemented!()
    }

    pub fn interpolate(&self, length: f32) -> Transform {
        match self.radius < 0.005 {
            true => self.interpolate_line(length),
            false =>  self.interpolate_arc(length)
        }
    }

    pub fn get_end_transform(&self) -> Transform {
        self.interpolate(self.length)
    }
    
    pub fn generate_mesh(&self) -> Mesh {    
        let points = (0..=self.length.ceil() as usize).map(|i| self.interpolate_arc(i as f32).translation).collect::<Vec<Vec3>>();
        
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
use sdl2::pixels::Color;

use crate::{camera::Camera, scene::Scene, vector3::Vector3};

pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn cast_primary(&self, depth: u32, camera: &Camera, scene: &Scene) -> Color {
        if depth == 0 {
            return Color::BLACK;
        }

        let mut intersected = None;
        let mut min_distance = f64::MAX;

        for mesh in scene.meshes() {
            let distance = mesh.intersect(self);
            if distance > 0. && distance < min_distance {
                intersected = Some(mesh);
                min_distance = distance;
            }
        }

        intersected.map_or(Color::RGB(60, 60, 60), |intersected| {
            intersected.color(&self.position(min_distance), camera, scene, depth)
        })
    }

    pub fn cast_shadow(&self, scene: &Scene) -> bool {
        for mesh in scene.meshes() {
            let distance = mesh.intersect(self);
            if distance > 0. && distance < f64::MAX {
                return true;
            }
        }

        false
    }

    pub fn origin(&self) -> &Vector3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vector3 {
        &self.direction
    }

    fn position(&self, distance: f64) -> Vector3 {
        self.origin.add(&self.direction.scalar(distance))
    }
}

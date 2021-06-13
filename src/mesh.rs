use sdl2::pixels::Color;

use crate::{camera::Camera, material::Material, ray::Ray, scene::Scene, vector3::Vector3};

pub trait Mesh {
    fn color(&self, position: &Vector3, camera: &Camera, scene: &Scene, depth: u32) -> Color;

    fn intersect(&self, ray: &Ray) -> f64;

    fn normal(&self, position: &Vector3) -> Vector3;
}

pub struct Sphere {
    position: Vector3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(position: Vector3, radius: f64, material: Material) -> Self {
        Self {
            position,
            radius,
            material,
        }
    }

    fn midnight_formula(&self, b: f64, c: f64) -> f64 {
        let discriminant = b * b - 4. * c;
        if discriminant < 0. {
            f64::MAX
        } else if discriminant == 0. {
            -b / 2.
        } else {
            let sqrt = discriminant.sqrt();
            let positive = (-b + sqrt) / 2.;
            let negative = (-b - sqrt) / 2.;
            if positive < 0. { f64::MAX } else { positive }.min(if negative < 0. {
                f64::MAX
            } else {
                negative
            })
        }
    }
}

impl Mesh for Sphere {
    fn color(&self, position: &Vector3, camera: &Camera, scene: &Scene, depth: u32) -> Color {
        self.material.color(position, &self.normal(position), camera, scene, depth)
    }

    fn intersect(&self, ray: &Ray) -> f64 {
        let center = ray.origin().sub(&self.position);

        let b = 2. * ray.direction().dot(&center);
        let c = center.dot(&center) - self.radius * self.radius;

        self.midnight_formula(b, c)
    }

    fn normal(&self, position: &Vector3) -> Vector3 {
        position.sub(&self.position).normalized()
    }
}

pub struct Plane {
    position: Vector3,
    a: Vector3,
    b: Vector3,
    material: Material,
}

impl Plane {
    pub fn new(position: Vector3, a: Vector3, b: Vector3, material: Material) -> Self {
        Self {
            position,
            a,
            b,
            material,
        }
    }
}

impl Mesh for Plane {
    fn color(&self, position: &Vector3, camera: &Camera, scene: &Scene, depth: u32) -> Color {
        self.material.color(position, &self.normal(position), camera, scene, depth)
    }

    fn intersect(&self, ray: &Ray) -> f64 {
        let normal = self.normal(&self.position);
        let enumerator = self.position.sub(ray.origin()).dot(&normal);
        let denominator = ray.direction().dot(&normal);
        enumerator / denominator
    }

    fn normal(&self, _: &Vector3) -> Vector3 {
        self.a.cross(&self.b)
    }
}

use sdl2::pixels::Color;

use crate::{camera::Camera, ray::Ray, scene::Scene, vector3::Vector3};

pub struct Material {
    ambient: Vector3,
    diffusion: Vector3,
    specular: Vector3,
    reflection_idx: f64,
}

impl Material {
    const EPSILON: f64 = 0.00004;
    const PHONG_EXPONENT: f64 = 0.5;

    pub fn new(ambient: Color, diffusion: Vector3, specular: Vector3, reflection_idx: f64) -> Self {
        let ambient_vector = Vector3::new(
            ambient.r as f64 / 255.0,
            ambient.g as f64 / 255.0,
            ambient.b as f64 / 255.0,
        );

        Self {
            ambient: ambient_vector,
            diffusion,
            specular,
            reflection_idx,
        }
    }

    pub fn color(
        &self,
        position: &Vector3,
        normal: &Vector3,
        camera: &Camera,
        scene: &Scene,
        depth: u32,
    ) -> Color {
        let mut light_sum = Vector3::default();

        for light in scene.lights() {
            let direction_to_light = light.position().sub(position).normalized();
            let shadow = Ray::new(
                position.add(&direction_to_light.scalar(Self::EPSILON)),
                direction_to_light,
            );

            let mut light_result = Vector3::default();
            light_result = light_result.add(&self.ambient.mul(light.intensity(position)));

            if !shadow.cast_shadow(scene) {
                let nl = 0f64.max(normal.dot(&direction_to_light));
                light_result =
                    light_result.add(&self.diffusion.mul(light.intensity(position)).scalar(nl));

                let reflection = normal.scalar(nl * 2.).sub(&direction_to_light).normalized();
                let direction_to_camera = camera.position().sub(position).normalized();
                let rv = 0f64.max(reflection.dot(&direction_to_camera));

                light_result = light_result.add(
                    &self
                        .specular
                        .mul(light.intensity(position))
                        .scalar(rv.powf(Self::PHONG_EXPONENT)),
                )
            }

            let distance = light.position().sub(position).length();
            light_sum = light_sum.add(&light_result.scalar(1. / (distance * distance)).scalar(255.))
        }

        if self.reflection_idx > 0. {
            let direction_to_camera = camera.position().sub(position).normalized();
            let nv = 0f64.max(normal.dot(&direction_to_camera));

            let reflection = normal
                .scalar(nv * 2.)
                .sub(&direction_to_camera)
                .normalized();
            let reflection_ray =
                Ray::new(position.add(&reflection.scalar(Self::EPSILON)), reflection);

            let color = reflection_ray.cast_primary(depth - 1, camera, scene);

            let mut color_vector = Vector3::new(color.r as f64, color.g as f64, color.b as f64);

            color_vector = color_vector.scalar(self.reflection_idx);

            light_sum = light_sum.add(&color_vector);
        }

        let color = Vector3::new(
            0f64.max(255f64.min(light_sum.x())),
            0f64.max(255f64.min(light_sum.y())),
            0f64.max(255f64.min(light_sum.z())),
        );

        Color::RGB(color.x() as u8, color.y() as u8, color.z() as u8)
    }
}

use crate::vector3::Vector3;

pub trait Light {
    fn position(&self) -> &Vector3;

    fn intensity(&self, position: &Vector3) -> &Vector3;
}

pub struct PointLight {
    position: Vector3,
    intensity: Vector3,
}

impl PointLight {
    pub fn new(position: Vector3, intensity: Vector3) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

impl Light for PointLight {
    fn position(&self) -> &Vector3 {
        &self.position
    }

    fn intensity(&self, _: &Vector3) -> &Vector3 {
        &self.intensity
    }
}

use crate::{light::Light, mesh::Mesh};

pub struct Scene {
    meshes: Vec<Box<dyn Mesh>>,
    lights: Vec<Box<dyn Light>>,
}

impl Scene {
    pub fn new(meshes: Vec<Box<dyn Mesh>>, lights: Vec<Box<dyn Light>>) -> Self {
        Self { meshes, lights }
    }

    pub fn meshes(&self) -> &Vec<Box<dyn Mesh>> {
        &self.meshes
    }

    pub fn lights(&self) -> &Vec<Box<dyn Light>> {
        &self.lights
    }
}

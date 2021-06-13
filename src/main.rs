use camera::Camera;
use light::{Light, PointLight};
use material::Material;
use mesh::{Mesh, Plane, Sphere};
use panel::Panel;
use scene::Scene;
use sdl2::pixels::Color;
use tracer::Raytracer;
use vector3::Vector3;

mod camera;
mod light;
mod material;
mod mesh;
mod panel;
mod ray;
mod scene;
mod tracer;
mod vector3;

pub fn main() {
    let sdl = sdl2::init().unwrap();

    let events = sdl.event_pump().unwrap();

    let panel = Panel::new("Raytracer", sdl);

    let camera = Camera::default();

    let mut meshes: Vec<Box<dyn Mesh>> = Vec::new();

    let material1 = Material::new(
        Color::RGB(0, 0, 139),
        Vector3::new(0.7, 0.7, 0.7),
        Vector3::new(0.3, 0.3, 0.3),
        0.,
    );
    meshes.push(Box::new(Sphere::new(
        Vector3::new(10., -5., 0.),
        3.,
        material1,
    )));

    let material2 = Material::new(
        Color::RGB(237, 51, 57),
        Vector3::new(0.7, 0.7, 0.7),
        Vector3::new(0.3, 0.3, 0.3),
        0.,
    );
    meshes.push(Box::new(Sphere::new(
        Vector3::new(10., 5., 0.),
        3.,
        material2,
    )));

    let material3 = Material::new(
        Color::RGB(5, 5, 5),
        Vector3::new(0.7, 0.7, 0.7),
        Vector3::new(0.3, 0.3, 0.3),
        1.,
    );
    meshes.push(Box::new(Sphere::new(
        Vector3::new(20., 0., 0.),
        3.,
        material3,
    )));

    let material4 = Material::new(
        Color::RGB(192, 192, 192),
        Vector3::new(0.7, 0.7, 0.7),
        Vector3::new(0.3, 0.3, 0.3),
        0.5,
    );
    meshes.push(Box::new(Plane::new(
        Vector3::new(0., 5., -10.),
        Vector3::new(1., 0., 0.),
        Vector3::new(0., 1., 0.),
        material4,
    )));

    let lights: Vec<Box<dyn Light>> = vec![
        Box::new(PointLight::new(
            Vector3::new(5., 0., 10.),
            Vector3::new(25., 25., 25.),
        )),
    ];

    let scene = Scene::new(meshes, lights);

    Raytracer::new(panel, camera, scene, events).trace();
}

use sdl2::{event::Event, keyboard::Keycode, EventPump};

use crate::{camera::Camera, panel::Panel, ray::Ray, scene::Scene};

pub struct Raytracer {
    panel: Panel,
    camera: Camera,
    scene: Scene,
    events: EventPump,
}

impl Raytracer {
    const RECURSION_DEPTH: u32 = 3;

    pub fn new(panel: Panel, camera: Camera, scene: Scene, events: EventPump) -> Self {
        Self {
            panel,
            camera,
            scene,
            events,
        }
    }

    pub fn trace(&mut self) {
        'running: loop {
            for event in self.events.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown {
                        keycode: Some(Keycode::W),
                        ..
                    } => self.camera.move_forward(0.5),
                    Event::KeyDown {
                        keycode: Some(Keycode::A),
                        ..
                    } => self.camera.move_left(0.5),
                    Event::KeyDown {
                        keycode: Some(Keycode::S),
                        ..
                    } => self.camera.move_back(0.5),
                    Event::KeyDown {
                        keycode: Some(Keycode::D),
                        ..
                    } => self.camera.move_right(0.5),
                    Event::KeyDown {
                        keycode: Some(Keycode::Up),
                        ..
                    } => self.camera.move_up(0.1),
                    Event::KeyDown {
                        keycode: Some(Keycode::Down),
                        ..
                    } => self.camera.move_down(0.1),
                    Event::KeyDown {
                        keycode: Some(Keycode::Left),
                        ..
                    } => self.camera.rotate_left(2.),
                    Event::KeyDown {
                        keycode: Some(Keycode::Right),
                        ..
                    } => self.camera.rotate_right(2.),
                    Event::KeyDown {
                        keycode: Some(Keycode::Plus),
                        ..
                    } => self.camera.inc_fov(),
                    Event::KeyDown {
                        keycode: Some(Keycode::Minus),
                        ..
                    } => self.camera.dec_fov(),
                    _ => {}
                }
            }

            let screen_top = self.camera.screen_top().sub(&self.camera.screen_center());
            let screen_left = self.camera.screen_left().sub(&self.camera.screen_center());
            let screen_bottom = screen_top.scalar(-1.).normalized();
            let screen_right = screen_left.scalar(-1.).normalized();

            let top_left = screen_top
                .add(&screen_left)
                .add(&self.camera.screen_center())
                .add(&screen_bottom.scalar(0.5))
                .add(&screen_right.scalar(0.5));

            for x in 0..Panel::WIDTH as i32 {
                for y in 0..Panel::HEIGHT as i32 {
                    let pixel = top_left
                        .add(&screen_right.scalar(x as f64))
                        .add(&screen_bottom.scalar(y as f64));

                    let ray = Ray::new(
                        *self.camera.position(),
                        pixel.sub(self.camera.position()).normalized(),
                    );

                    self.panel.set_pixel(
                        x,
                        y,
                        ray.cast_primary(Self::RECURSION_DEPTH, &self.camera, &self.scene),
                    )
                }
            }

            // let screen_bottom = self.camera.up().scalar(-1.).normalized();
            // let screen_right = self.camera.left().scalar(-1.).normalized();

            // let top_left = self
            //     .camera
            //     .position()
            //     .add(&self.camera.up().scalar(Panel::HEIGHT as f64 / 2.))
            //     .add(&self.camera.left().scalar(Panel::WIDTH as f64 / 2.))
            //     .add(&screen_bottom.scalar(0.5)) 
            //     .add(&screen_right.scalar(0.5));

            // for x in 0..Panel::WIDTH as i32 {
            //     for y in 0..Panel::HEIGHT as i32 {
            //         let pixel = top_left
            //             .add(&screen_right.scalar(x as f64))
            //             .add(&screen_bottom.scalar(y as f64));

            //         let ray = Ray::new(pixel, *self.camera.direction());

            //         self.panel.set_pixel(x, y, ray.cast_primary(Self::RECURSION_DEPTH, &self.camera, &self.scene));
            //     }
            // }

            self.panel.present()
        }
    }
}

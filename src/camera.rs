use crate::{panel::Panel, vector3::Vector3};

pub struct Camera {
    fov: f64,
    position: Vector3,
    direction: Vector3,
    up: Vector3,
}

impl Camera {
    pub fn new(fov: f64, position: Vector3, direction: Vector3, up: Vector3) -> Self {
        Self {
            fov,
            position,
            direction,
            up,
        }
    }

    pub fn position(&self) -> &Vector3 {
        &self.position
    }

    pub fn screen_center(&self) -> Vector3 {
        self.position.add(&self.direction.scalar(self.distance()))
    }

    pub fn screen_top(&self) -> Vector3 {
        self.screen_center()
            .add(&self.up.scalar(Panel::HEIGHT as f64 / 2.))
    }

    pub fn screen_left(&self) -> Vector3 {
        self.screen_center()
            .add(&self.left().scalar(Panel::WIDTH as f64 / 2.))
    }

    pub fn move_forward(&mut self, amount: f64) {
        self.position = self.position.add(&self.direction.scalar(amount))
    }

    pub fn move_left(&mut self, amount: f64) {
        self.position = self.position.add(&self.left().scalar(amount))
    }

    pub fn move_back(&mut self, amount: f64) {
        self.move_forward(-amount)
    }

    pub fn move_right(&mut self, amount: f64) {
        self.move_left(-amount)
    }

    pub fn move_up(&mut self, amount: f64) {
        self.position = self.position.add(&self.up.scalar(amount))
    }

    pub fn move_down(&mut self, amount: f64) {
        self.move_up(-amount)
    }

    pub fn rotate_left(&mut self, amount: f64) {
        let degree = amount.to_radians();
        let cos = degree.cos();
        let sin = degree.sin();
        self.direction = Vector3::new(
            self.direction.x() * cos - self.direction.y() * sin,
            self.direction.x() * sin + self.direction.y() * cos,
            self.direction.z(),
        )
        .normalized()
    }

    pub fn rotate_right(&mut self, amount: f64) {
        self.rotate_left(-amount)
    }

    pub fn inc_fov(&mut self) {
        self.fov += 1.
    }

    pub fn dec_fov(&mut self) {
        self.fov -= 1.
    }

    fn distance(&self) -> f64 {
        Panel::HEIGHT as f64 / 2.0 / (self.fov / 2.).to_radians().tan()
    }

    fn left(&self) -> Vector3 {
        self.up.cross(&self.direction).normalized()
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            45.,
            Vector3::default(),
            Vector3::new(1., 0., 0.),
            Vector3::new(0., 0., 1.),
        )
    }
}

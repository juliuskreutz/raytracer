#[derive(Default, Clone, Copy)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn add(&self, vector: &Vector3) -> Vector3 {
        Vector3::new(self.x + vector.x, self.y + vector.y, self.z + vector.z)
    }

    pub fn sub(&self, vector: &Vector3) -> Vector3 {
        Vector3::new(self.x - vector.x, self.y - vector.y, self.z - vector.z)
    }

    pub fn mul(&self, vector: &Vector3) -> Vector3 {
        Vector3::new(self.x * vector.x, self.y * vector.y, self.z * vector.z)
    }

    pub fn scalar(&self, scalar: f64) -> Vector3 {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    pub fn dot(&self, vector: &Vector3) -> f64 {
        self.x * vector.x + self.y * vector.y + self.z * vector.z
    }

    pub fn cross(&self, vector: &Vector3) -> Vector3 {
        Vector3::new(
            self.y * vector.z - self.z * vector.y,
            self.z * vector.x - self.x * vector.z,
            self.x * vector.y - self.y * vector.x,
        )
    }

    pub fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalized(&self) -> Vector3 {
        let length = self.length();
        Vector3::new(
            self.x / length,
            self.y / length,
            self.z / length,
        )
    }
}

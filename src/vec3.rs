use std::ops::{Add, Sub, Mul, Div};

pub struct Vec3 {
    pub e: (f32, f32, f32),
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Self { e: (e0, e1, e2) }
    }

    pub fn x(&self) -> f32 {
        self.e.0
    }

    pub fn y(&self) -> f32 {
        self.e.1
    }

    pub fn z(&self) -> f32 {
        self.e.2
    }

    pub fn len(&self) -> f32 {
        (self.e.0 * self.e.0 + self.e.1 * self.e.1 + self.e.2 * self.e.2).sqrt()
    }

    pub fn squared_len(&self) -> f32 {
        self.e.0 * self.e.0 + self.e.1 * self.e.1 + self.e.2 * self.e.2
    }

    pub fn dot(&self, other: Self) -> f32 {
         self.e.0 * other.e.0 + self.e.1 * other.e.1 + self.e.2 * other.e.2
    }

    pub fn unit_vector(&self) -> Self {
        self / self.len()
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e.0 + other.e.0,
            self.e.1 + other.e.1,
            self.e.2 + other.e.2,
        )
    }
}

impl<'a> Add<Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e.0 + other.e.0,
            self.e.1 + other.e.1,
            self.e.2 + other.e.2,
        )
    }
}

impl<'a, 'b> Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, other: &'b Vec3) -> Vec3 {
        Vec3::new(
            self.e.0 + other.e.0,
            self.e.1 + other.e.1,
            self.e.2 + other.e.2,
        )
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e.0 - other.e.0,
            self.e.1 - other.e.1,
            self.e.2 - other.e.2,
        )
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e.0 * other.e.0,
            self.e.1 * other.e.1,
            self.e.2 * other.e.2,
        )
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3::new(
            self.e.0 * other,
            self.e.1 * other,
            self.e.2 * other,
        )
    }
}

impl<'a> Mul<f32> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3::new(
            self.e.0 * other,
            self.e.1 * other,
            self.e.2 * other,
        )
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e.0 / other.e.0,
            self.e.1 / other.e.1,
            self.e.2 / other.e.2,
        )
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3::new(
            self.e.0 / other,
            self.e.1 / other,
            self.e.2 / other,
        )
    }
}

impl<'a> Div<f32> for &'a Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3::new(
            self.e.0 / other,
            self.e.1 / other,
            self.e.2 / other,
        )
    }
}

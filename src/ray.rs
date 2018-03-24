use vec3::Vec3;

pub struct Ray<'a> {
    a: &'a Vec3,
    b: &'a Vec3,
}

impl<'a> Ray<'a> {
    pub fn new(a: &'a Vec3, b: &'a Vec3) -> Self {
        Self { a, b }
    }

    pub fn origin(&self) -> &Vec3 {
        self.a
    }

    pub fn direction(&self) -> &Vec3 {
        self.b
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.a + self.b * t
    }

    pub fn color(&self) -> Vec3 {
        let unit_direction = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

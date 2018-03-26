use vec3::Vec3;
use hitable::*;
use std::f32::MAX;

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

    pub fn color(&self, world: &'a Hitable) -> Vec3 {
        let mut record = HitRecord {
            ..HitRecord::default()
        };
        if world.hit(&self, 0.0, MAX, &mut record) {
            return Vec3::new(
                record.normal.x() + 1.0,
                record.normal.y() + 1.0,
                record.normal.z() + 1.0,
            ) * 0.5;
        } else {
            let unit_direction = self.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

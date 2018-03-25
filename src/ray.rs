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
        let t = self.hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5);

        if t > 0.0 {
            let n = (self.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
            return Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
        } else {
            let unit_direction = self.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }

    fn hit_sphere(&self, center: Vec3, radius: f32) -> f32 {
        let ray_origin = self.origin();
        let ray_direction = self.direction();

        let origin_center = ray_origin - center;
        let a = ray_direction.dot(ray_direction);
        let b = 2.0 * origin_center.dot(ray_direction);
        let c = origin_center.dot(&origin_center) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return -1.0;
        } else {
            return (-b - discriminant.sqrt()) / (a * 2.0);
        }
    }
}

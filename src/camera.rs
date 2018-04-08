use ray::Ray;
use vec3::Vec3;

#[derive(Default)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,

    ray_direction: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            origin: Vec3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            ..Self::default()
        }
    }

    pub fn get_ray(&mut self, u: f32, v: f32) -> Ray {
        self.ray_direction = &self.lower_left_corner + &self.horizontal * u + &self.vertical * v - &self.origin;

        Ray::new(
            &self.origin,
            &self.ray_direction,
        )
    }
}

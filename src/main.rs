mod vec3;
mod ray;

use std::io::prelude::*;
use std::fs::File;
use vec3::Vec3;
use ray::Ray;

fn main() {
    let nx = 200;
    let ny = 100;

    let mut result = String::new();
    result.push_str(&format!("P3\n{} {}\n255\n", nx, ny));

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let direction = &lower_left_corner + &horizontal * u + &vertical * v;
            let ray = Ray::new(&origin, &direction);
            let color = ray.color();

            let ir = (255.99 * color.e.0) as i32;
            let ig = (255.99 * color.e.1) as i32;
            let ib = (255.99 * color.e.2) as i32;

            result.push_str(&format!("{} {} {}\n", ir, ig, ib));
        }
    }

    let mut file = File::create("image.ppm").unwrap();
    file.write_all(result.as_bytes()).unwrap();
}

extern crate rand;

mod vec3;
mod ray;
mod hitable;
mod camera;

use std::io::prelude::*;
use std::fs::File;
use rand::{thread_rng, Rng};
use vec3::Vec3;
use hitable::*;
use camera::Camera;

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    let mut result = String::new();
    result.push_str(&format!("P3\n{} {}\n255\n", nx, ny));

    let s1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let s2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    let list: Vec<&Hitable> = vec![&s1, &s2];
    let world = HitableList::new(list);
    let mut camera = Camera::new();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..ns {
                let u = (i as f32 + drand48()) / nx as f32;
                let v = (j as f32 + drand48()) / ny as f32;
                let ray = camera.get_ray(u, v);
                color = color + ray.color(&world);
            }

            color = color / ns as f32;

            let ir = (255.99 * color.0) as i32;
            let ig = (255.99 * color.1) as i32;
            let ib = (255.99 * color.2) as i32;

            result.push_str(&format!("{} {} {}\n", ir, ig, ib));
        }
    }

    let mut file = File::create("image.ppm").unwrap();
    file.write_all(result.as_bytes()).unwrap();
}

fn drand48() -> f32 {
    let mut rng = thread_rng();
    rng.gen_range(0.0, 1.0)
}

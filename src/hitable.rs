use ray::Ray;
use vec3::Vec3;

#[derive(Clone, Default)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, record: &mut HitRecord) -> bool;
}

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, record: &mut HitRecord) -> bool {
        let ray_origin = ray.origin();
        let ray_direction = ray.direction();

        let origin_center = ray_origin - &self.center;
        let a = ray_direction.dot(&ray_direction);
        let b = origin_center.dot(&ray_direction);
        let c = origin_center.dot(&origin_center) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < tmax && temp > tmin {
                record.t = temp;
                record.p = ray.point_at_parameter(record.t);
                record.normal = (&record.p - &self.center) / self.radius;
                return true;
            }

            temp = (-b + discriminant.sqrt()) / a;
            if temp < tmax && temp > tmin {
                record.t = temp;
                record.p = ray.point_at_parameter(record.t);
                record.normal = (&record.p - &self.center) / self.radius;
                return true;
            }
        }
        return false;
    }
}

pub struct HitableList<'a> {
    pub list: Vec<&'a Hitable>,
}

impl<'a> HitableList<'a> {
    pub fn new(list: Vec<&'a Hitable>) -> Self {
        Self { list }
    }
}

impl<'a> Hitable for HitableList<'a> {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord {
            ..HitRecord::default()
        };
        let mut hit_anything = false;
        let mut closest_so_far = tmax;
        for item in self.list.iter() {
            if item.hit(ray, tmin, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *record = temp_record.clone();
            }
        }
        return hit_anything;
    }
}

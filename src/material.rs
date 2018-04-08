use ray::Ray;
use vec3::Vec3;

trait Material {
    fn scatter(&ray: Ray, &record: HitRecord);
}

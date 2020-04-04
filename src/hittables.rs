use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::cmp::Ordering;

pub enum HitResult {
    Scatter(Vec3, Ray), // color, tranmission, new_ray
}

pub struct Hit {
    pub result: HitResult,
    pub time: f64,
}

pub trait Hittable: Send + Sync {
    fn hits(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit>;
    // returns Option<(Hit_Point, Normal)>
    fn material(&self) -> Material;
}

pub type Hittables = Vec<Box<dyn Hittable>>;

impl Hittable for Hittables {
    fn hits(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        self.iter()
            .map(|h| h.hits(ray, t_min, t_max))
            .filter_map(|p| p)
            .min_by(|x, y| x.time.partial_cmp(&y.time).unwrap_or(Ordering::Less))
    }

    fn material(&self) -> Material {
        // this should never be called but it has to be there
        Material::new()
    }
}

// spheres
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Hittable for Sphere {
    fn hits(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let shifted_ray_origin = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let half_b = ray.direction.dot(&shifted_ray_origin);
        let c = (shifted_ray_origin).dot(&shifted_ray_origin) - self.radius.powi(2);
        let term_under_sqrt = half_b.powi(2) - a * c;
        if term_under_sqrt > 0.0 {
            let root = term_under_sqrt.sqrt();
            let ts = vec![(-half_b - root) / a, (-half_b + root) / a];
            for &t in ts.iter() {
                if (t < t_min) || (t > t_max) {
                    continue;
                } else {
                    let point = ray.at(t);
                    let outward_normal = (point - self.center).unit_vector();
                    // let front_face: _ = outward_normal.dot(&ray.direction) > 0.0;
                    return Some(Hit {
                        result: self
                            .material
                            .get_hit_result(point, ray.direction, outward_normal),
                        time: t,
                    });
                }
            }
        }
        None
    }
    fn material(&self) -> Material {
        self.material
    }
}

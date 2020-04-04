use crate::hittables::HitResult;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub enum Reflectance {
    Diffuse,
    Mirrory,
}

#[derive(Debug, Copy, Clone)]
pub struct Material {
    // in the range [0, 1]
    absorption_p: f64,
    scattering_p: f64,
    roughness: f64,
    reflectance: Reflectance,
    color: Vec3,
}

impl Material {
    pub fn new() -> Material {
        Material {
            absorption_p: 0.0,
            scattering_p: 0.0,
            roughness: 0.0,
            reflectance: Reflectance::Diffuse,
            color: Vec3::ones(),
        }
    }

    pub fn absorption_p(&mut self, absorption_p: f64) -> &mut Material {
        self.absorption_p = absorption_p;
        self
    }

    pub fn scattering_p(&mut self, scattering_p: f64) -> &mut Material {
        self.scattering_p = scattering_p;
        self
    }

    pub fn roughness(&mut self, roughness: f64) -> &mut Material {
        self.roughness = roughness;
        self
    }

    pub fn reflectance(&mut self, reflectance: Reflectance) -> &mut Material {
        self.reflectance = reflectance;
        self
    }

    pub fn color(&mut self, color: Vec3) -> &mut Material {
        self.color = color;
        self
    }

    pub fn finalize(&self) -> Material {
        *self
    }

    pub fn get_hit_result(&self, point: Vec3, in_ray_direction: Vec3, normal: Vec3) -> HitResult {
        let dice = rand::random::<f64>();
        if dice < self.scattering_p {
            let direction = match self.reflectance {
                Reflectance::Diffuse => normal,
                Reflectance::Mirrory => {
                    in_ray_direction - 2.0 * in_ray_direction.dot(&normal) * normal
                }
            } + self.roughness * Vec3::random_in_unit_sphere();
            HitResult::Scatter(
                (1.0 - self.absorption_p) * self.color,
                Ray {
                    origin: point,
                    direction,
                },
            )
        } else {
            HitResult::Scatter(
                Vec3::zeros(),
                Ray {
                    origin: point,
                    direction: in_ray_direction,
                },
            )
        }
    }
}

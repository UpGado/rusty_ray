use crate::ray::Ray;
use crate::vec3::Vec3;


pub trait Camera {
    fn get_ray(&self, u: f64, v: f64) -> Ray;
}

pub struct SimpleCamera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3
}

impl Camera for SimpleCamera {
    fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = (self.lower_left_corner + u*self.horizontal
                         + v*self.vertical).unit_vector();
        Ray{origin: self.origin, direction}
    }
} 

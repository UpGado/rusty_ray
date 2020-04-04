use crate::cameras::Camera;
use crate::hittables::{Hit, HitResult, Hittable};
use crate::io::Img;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::prelude::*;
use std::f64::INFINITY;
use HitResult::Scatter;

fn ray_color(r: Ray, world: &impl Hittable, max_scatter_depth: i32) -> Vec3 {
    if max_scatter_depth == 0 {
        Vec3::zeros()
    } else {
        match world.hits(r, 0.001, INFINITY) {
            Some(Hit {
                result: Scatter(color, new_ray),
                ..
            }) => color * ray_color(new_ray, world, max_scatter_depth - 1),
            None => {
                let unit_direction = r.direction.unit_vector();
                let t = 0.5 * (unit_direction.1 + 1.0);
                (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
            }
        }
    }
}

pub trait Renderer {
    fn render(width: usize, height: usize, camera: &impl Camera, world: &impl Hittable) -> Img;
}

pub enum SingleThreadedRenderer {}

impl Renderer for SingleThreadedRenderer {
    fn render(width: usize, height: usize, camera: &impl Camera, world: &impl Hittable) -> Img {
        let samples_per_pixel = 100;
        let max_scatter_depth = 50;
        let mut rng = rand::thread_rng();
        let mut img: Img = Vec::with_capacity(height);
        for j in 0..height {
            let mut row: Vec<(u8, u8, u8)> = Vec::with_capacity(width);
            for i in 0..width {
                let mut color = Vec3::zeros();
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + rng.gen::<f64>()) / width as f64;
                    let v = ((height - j) as f64 + rng.gen::<f64>()) / height as f64;
                    let r = camera.get_ray(u, v);
                    color += ray_color(r, world, max_scatter_depth);
                }
                color /= samples_per_pixel as f64;
                row.push(color.as_pixel());
            }
            img.push(row);
        }
        img
    }
}

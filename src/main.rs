mod cameras;
mod hittables;
mod materials;
mod ray;
mod vec3;
use cameras::{Camera, SimpleCamera};
use hittables::{Hit, HitResult, Hittable, Hittables, Sphere};
use materials::{Material, Reflectance};
use rand::prelude::*;
use ray::Ray;
use rayon::prelude::*;
use std::f64::consts::PI;
use std::f64::INFINITY;
use vec3::Vec3;
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

fn main() {
    let scale = 4;
    let width = scale * 200;
    let height = scale * 100;
    let max_val = 255;
    let samples_per_pixel = 100;
    let max_scatter_depth = 50;

    println!("P3\n{} {}\n{}", width, height, max_val);

    let origin = Vec3::zeros();
    let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 2.0, 0.0);

    let center_s = Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::new()
            .reflectance(Reflectance::Diffuse)
            .scattering_p(1.0)
            .roughness(1.0)
            .color(Vec3(0.7, 0.3, 0.3))
            .finalize(),
    };
    let right_s = Sphere {
        center: Vec3(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::new()
            .reflectance(Reflectance::Mirrory)
            .scattering_p(1.0)
            .roughness(0.0)
            .color(Vec3(0.8, 0.6, 0.2))
            .finalize(),
    };
    let left_s = Sphere {
        center: Vec3(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::new()
            .reflectance(Reflectance::Mirrory)
            .scattering_p(1.0)
            .roughness(0.5)
            .color(Vec3(0.8, 0.8, 0.8))
            .finalize(),
    };
    let ground = Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Material::new()
            .reflectance(Reflectance::Diffuse)
            .scattering_p(1.0)
            .roughness(1.0)
            .color(Vec3(0.8, 0.8, 0.0))
            .finalize(),
    };
    let world: Hittables = vec![
        Box::new(center_s),
        Box::new(right_s),
        Box::new(left_s),
        Box::new(ground),
    ];

    let camera = SimpleCamera {
        lower_left_corner,
        horizontal,
        vertical,
        origin,
    };

    let mut rng = rand::thread_rng();
    for j in (0..height).rev() {
        if (j & 0xff) == 0 {
            eprintln!("{} rows remaining", j);
        }
        for i in 0..width {
            let mut color = Vec3::zeros();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / width as f64;
                let v = (j as f64 + rng.gen::<f64>()) / height as f64;
                let r = camera.get_ray(u, v);
                color += ray_color(r, &world, max_scatter_depth);
            }
            color /= samples_per_pixel as f64;
            println!("{}", color.color_string());
        }
    }
    eprintln!("done!");
}

#![feature(let_chains)]
mod cameras;
mod hittables;
mod io;
mod materials;
mod ray;
mod renderers;
mod vec3;
use cameras::SimpleCamera;
use hittables::{Hittables, Sphere};
use materials::{Material, Reflectance};
use renderers::Renderer;
use vec3::Vec3;

fn main() {
    let scale = 5;
    let width = scale * 200;
    let height = scale * 100;

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

    let img = if {
        let multithreaded = true;
        multithreaded
    } {
        renderers::MultithreadedRenderer::render(width, height, &camera, &world)
    } else {
        renderers::SingleThreadedRenderer::render(width, height, &camera, &world)
    };
    io::save_img(&img, io::FileFormat::PNG, "test");
    eprintln!("done!");
}

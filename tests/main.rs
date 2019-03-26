use std::fs::OpenOptions;
use std::io::prelude::*;

use ray_tracer::vector::Vec3;
use ray_tracer::scene::Scene;
use ray_tracer::hitable::sphere::Sphere;
use ray_tracer::camera::perspective::PerspectiveCamera;
use ray_tracer::renderer::Renderer;
use ray_tracer::renderer::Image;

fn to_u8(f: f64) -> u8 {
    (f * 255.0) as u8
}

fn print_ppm(image: &Image<f64>) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open("basic_scene.ppm")
        .unwrap();

    if let Err(e) = writeln!(file, "P3\n# asd\n{} {}\n255", image.width, image.height) {
        eprintln!("Couldn't write to file: {}", e);
    }

    for j in 0..image.height {
        for i in 0..image.width {
            let index = j * image.width + i;
            if let Err(e) = writeln!(
                file, "{} {} {}",
                to_u8(image.data[3 * index]),
                to_u8(image.data[3 * index + 1]),
                to_u8(image.data[3 * index + 2])
            ) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }


}

#[test]
fn basic_scene() {
    let mut scene = Scene::<f64>::new();
    let sphere = Sphere::<f64>::from(Vec3::from_array([0.0, 0.0, -2.0]), 1.0);
    scene.add_actor(Box::new(sphere));
    let sphere = Sphere::<f64>::from(Vec3::from_array([1.0, 0.0, -2.0]), 1.0);
    scene.add_actor(Box::new(sphere));
    let sphere = Sphere::<f64>::from(Vec3::from_array([2.0, 0.0, -2.0]), 1.0);
    scene.add_actor(Box::new(sphere));
    let sphere = Sphere::<f64>::from(Vec3::from_array([0.0, 1.0, -2.0]), 1.0);
    scene.add_actor(Box::new(sphere));

    let width = 480;
    let height = 320;
    let aspect = width as f64 / height as f64;
    let mut camera = PerspectiveCamera::<f64>::new();
    camera.set_aspect(aspect);

    let renderer = Renderer::new(width, height);
    let image = renderer.render(&scene, &camera);
    print_ppm(&image);
}
use std::fs::OpenOptions;
use std::io::prelude::*;

use ray_tracer::vector::Vec3;
use ray_tracer::scene::Scene;
use ray_tracer::hitable::sphere::Sphere;
use ray_tracer::camera::Camera;
use ray_tracer::camera::perspective::PerspectiveCamera;
use ray_tracer::renderer::Renderer;
use ray_tracer::renderer::Image;
use ray_tracer::material::plain::PlainMaterial;
use ray_tracer::actor::Actor;

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
    // return;
    let mut scene = Scene::<f64>::new();
    scene.set_background(Vec3::from_array([0.25, 0.5, 0.75]));

    let r = 1.0;
    let sphere = Sphere::<f64>::from(Vec3::from_array([0.0, r, -4.0]), r);
    let material = PlainMaterial::<f64> { color: Vec3::from_array([1.0, 0.0, 0.0])};
    let actor = Actor::<f64> { hitable: Box::new(sphere), material: Box::new(material)};
    scene.add_actor(actor);

    let r = 1.0;
    let sphere = Sphere::<f64>::from(Vec3::from_array([-1.0, r, -4.0]), r);
    let material = PlainMaterial::<f64> { color: Vec3::from_array([0.0, 1.0, 0.0])};
    let actor = Actor::<f64> { hitable: Box::new(sphere), material: Box::new(material)};
    scene.add_actor(actor);

    let r = 1.0;
    let sphere = Sphere::<f64>::from(Vec3::from_array([1.0, r, -4.0]), r);
    let material = PlainMaterial::<f64> { color: Vec3::from_array([0.0, 0.0, 1.0])};
    let actor = Actor::<f64> { hitable: Box::new(sphere), material: Box::new(material)};
    scene.add_actor(actor);

    // Sphere used as floor
    let r = 10000.0;
    let sphere = Sphere::<f64>::from(Vec3::from_array([0.0, -r - 0.01, 0.0]), r);
    let material = PlainMaterial::<f64> { color: Vec3::from_array([0.75, 0.5, 0.0])};
    let actor = Actor::<f64> { hitable: Box::new(sphere), material: Box::new(material)};
    scene.add_actor(actor);

    let mul = 4;
    let width = 240 * mul;
    let height = 160 * mul;
    let aspect = width as f64 / height as f64;
    let mut camera = PerspectiveCamera::<f64>::new();
    camera.set_aspect(aspect);
    camera.set_fov(0.5 * std::f64::consts::PI);
    camera.set_position(&[0.0, 1.0, 0.0]);

    let renderer = Renderer::new(width, height);
    let image = renderer.render(&scene, &camera);
    print_ppm(&image);
}

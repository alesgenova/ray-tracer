use rand::prelude::*;

use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hitable::Hitable;
use crate::camera::Camera;
use crate::scene::Scene;

pub struct Image<T>
    where T: Float
{
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>
}

impl<T> Image<T>
    where T: Float
{
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![T::zero(); 3 * width * height];
        Image::<T> {
            width,
            height,
            data
        }
    }
}

pub struct Renderer {
    width: usize,
    height: usize,
    sampling: usize,
    reflections: usize
}

impl Renderer {
    pub fn new(width: usize, height: usize, sampling: usize, reflections: usize) -> Self {
        Renderer {
            width,
            height,
            sampling,
            reflections
        }
    }

    pub fn render_pixel<T>(&self, i: usize, j: usize, rng: &mut ThreadRng, scene: &Scene<T>, camera: &Camera<T>) -> Vec3<T>
        where T: Float
    {
        let two = T::from(2.0).unwrap();
        let mut color = Vec3::<T>::new();

        match self.sampling {
            // No multisampling
            0 => {
                let v = two * (T::from(j).unwrap() / T::from(self.height).unwrap()) - T::one();
                let u = two * (T::from(i).unwrap() / T::from(self.width).unwrap()) - T::one();
                let ray = camera.get_ray(u, v);
                color = scene.get_color(&ray, 0, self.reflections);
            },
            // Multisampling
            _ => {
                for k in 0..self.sampling {
                    let i : f64 = (i as f64) + rng.gen::<f64>();
                    let j : f64 = (j as f64) + rng.gen::<f64>();
                    let v = two * (T::from(j).unwrap() / T::from(self.height).unwrap()) - T::one();
                    let u = two * (T::from(i).unwrap() / T::from(self.width).unwrap()) - T::one();
                    let ray = camera.get_ray(u, v);
                    color = color + scene.get_color(&ray, 0, self.reflections);
                }
                let sampling = T::from(self.sampling).unwrap();
                color = color / sampling;
            }
        }

        color
    }

    pub fn render<T>(&self, scene: &Scene<T>, camera: &Camera<T>) -> Image<T>
        where T: Float
    {
        let mut rng = rand::thread_rng();
        let mut image = Image::<T>::new(self.width, self.height);
        for j in 0..self.height {
            for i in 0..self.width {
                let color = self.render_pixel(i, j, &mut rng, scene, camera);
                let index = j * self.width + i;
                image.data[3 * index] = color.get_data()[0];
                image.data[3 * index + 1] = color.get_data()[1];
                image.data[3 * index + 2] = color.get_data()[2];
            }
        }
        image
    }
}

use rand::prelude::*;

use std::u32;

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
    reflections: usize,
    antialiasing: bool
}

impl Renderer {
    pub fn new(width: usize, height: usize, sampling: usize, reflections: usize, antialiasing: bool) -> Self {
        Renderer {
            width,
            height,
            sampling,
            reflections,
            antialiasing
        }
    }

    pub fn render_pixel<T>(&self, i: usize, j: usize, scene: &Scene<T>, camera: &Camera<T>) -> Vec3<T>
        where T: Float
    {
        let two = T::from(2.0).unwrap();
        let mut color = Vec3::<T>::new();

        let sampling = match self.sampling {
            0 => 1,
            _ => self.sampling
        };

        match self.antialiasing {
            false => {
                let ray = self.get_ray(i, j, camera, two);
                for _k in 0..sampling {
                    color = color + scene.get_color(&ray, 0, self.reflections);
                }
            },
            true => {
                for _k in 0..sampling {
                    let ray = self.get_ray(i, j, camera, two);
                    color = color + scene.get_color(&ray, 0, self.reflections);
                }
            }
        }

        let sampling = T::from(sampling).unwrap();
        color = color / sampling;

        color
    }

    pub fn render<T>(&self, scene: &Scene<T>, camera: &Camera<T>) -> Image<T>
        where T: Float
    {
        let mut image = Image::<T>::new(self.width, self.height);
        for j in 0..self.height {
            for i in 0..self.width {
                let color = self.render_pixel(i, j, scene, camera);
                let index = j * self.width + i;
                image.data[3 * index] = color.get_data()[0];
                image.data[3 * index + 1] = color.get_data()[1];
                image.data[3 * index + 2] = color.get_data()[2];
            }
        }
        image
    }

    fn get_ray<T>(&self, i: usize, j: usize, camera: &Camera<T>, two: T) -> Ray<T>
        where T: Float
    {
        let two = T::from(2.0).unwrap();

        match self.antialiasing {
            // If antialiasing is disabled, the ray always hits the pixel in the same position
            false => {
                let v = two * (T::from(j).unwrap() / T::from(self.height).unwrap()) - T::one();
                let u = two * (T::from(i).unwrap() / T::from(self.width).unwrap()) - T::one();
                camera.get_ray(u, v)
            },
            // If antializasing is enabled, the ray is randomly chosen in the vicinity of the pixel
            true => {
                let i : f64 = (i as f64) + random::<f64>();
                let j : f64 = (j as f64) + random::<f64>();
                let v = two * (T::from(j).unwrap() / T::from(self.height).unwrap()) - T::one();
                let u = two * (T::from(i).unwrap() / T::from(self.width).unwrap()) - T::one();
                camera.get_ray(u, v)
            }
        }
    }
}

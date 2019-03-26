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
    height: usize
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Renderer {
            width,
            height
        }
    }

    pub fn render<T>(&self, scene: &Scene<T>, camera: &Camera<T>) -> Image<T>
        where T: Float
    {
        let mut image = Image::<T>::new(self.width, self.height);
        let two = T::from(2.0).unwrap();
        for j in 0..self.height {
            let v = two * (T::from(j).unwrap() / T::from(self.height).unwrap()) - T::one();
            for i in 0..self.width {
                let u = two * (T::from(i).unwrap() / T::from(self.width).unwrap()) - T::one();
                let ray = camera.get_ray(u, v);
                let color = scene.get_color(&ray);
                let index = j * self.width + i;
                image.data[3 * index] = color.get_data()[0];
                image.data[3 * index + 1] = color.get_data()[1];
                image.data[3 * index + 2] = color.get_data()[2];
            }
        }
        image
    }
}

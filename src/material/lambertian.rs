use rand::prelude::*;

use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::material::{Scatter, Material};
use crate::utils::random_point_in_sphere;
use crate::texture::Texture;

pub struct LambertianMaterial<T>
    where T: Float
{
    texture: Box<dyn Texture<T>>,
    dimming: T
}

impl<T> LambertianMaterial<T>
    where T: Float
{
    pub fn new(texture: Box<Texture<T>>, dimming: T) -> Self {
        LambertianMaterial {
            texture,
            dimming
        }
    }
}

impl<T> Material<T> for LambertianMaterial<T>
    where T: Float
{
    fn scatter(&self, incident: &Ray<T>, hit: &Hit<T>) -> Scatter<T> {
        let color = self.texture.get_color(T::zero(), T::zero(), &hit.point);
        let attenuation = Vec3::<T>::from_slice(color.get_data()) * self.dimming;
        let mut normal = Vec3::from_slice(hit.normal.get_data());
        normal.normalize();
        let origin = Vec3::from_slice(hit.point.get_data());
        let mut direction = normal + random_point_in_sphere(T::one());
        direction.normalize();
        let scattered = Some(Ray::<T>::from_vec(origin, direction));
        Scatter::<T> {
            attenuation,
            scattered
        }
    }
}

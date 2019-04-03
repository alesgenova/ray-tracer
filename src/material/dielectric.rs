use rand::prelude::*;

use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::material::{Scatter, Material};
use crate::utils::refract;
use crate::texture::Texture;

pub struct DielectricMaterial<T>
    where T: Float
{
    texture: Box<dyn Texture<T>>,
    n: T
}

impl<T> DielectricMaterial<T>
    where T: Float
{
    pub fn new(texture: Box<Texture<T>>, n: T) -> Self {
        DielectricMaterial {
            texture,
            n
        }
    }
}

impl<T> Material<T> for DielectricMaterial<T>
    where T: Float
{
    fn scatter(&self, incident: &Ray<T>, hit: &Hit<T>) -> Scatter<T> {
        let mut outward_normal = &hit.normal * (-T::one());
        let mut n0 = self.n;
        let mut n1 = T::one();
        let color = self.texture.get_color(T::zero(), T::zero(), &hit.point);
        let attenuation = Vec3::<T>::from_slice(color.get_data());
        let c = incident.get_direction().dot(&hit.normal);

        if c < T::zero() {
            outward_normal.set_data(hit.normal.get_data());
            n1 = self.n;
            n0 = T::one();
        }

        let mut direction = refract(incident.get_direction(), &outward_normal, n0, n1);
        let origin = Vec3::from_slice(hit.point.get_data());
        direction.normalize();

        let scattered = Some(Ray::<T>::from_vec(origin, direction));
        Scatter::<T> {
            attenuation,
            scattered
        }
    }
}

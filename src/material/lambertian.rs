use rand::prelude::*;

use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::material::{Scatter, Material};
use crate::utils::random_point_in_sphere;

pub struct LambertianMaterial<T>
    where T: Float
{
    pub color: Vec3<T>
}

impl<T> Material<T> for LambertianMaterial<T>
    where T: Float
{
    fn scatter(&self, incident: &Ray<T>, hit: &Hit<T>) -> Scatter<T> {
        let attenuation = Vec3::<T>::from_slice(self.color.get_data()) * T::from(0.5).unwrap();
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

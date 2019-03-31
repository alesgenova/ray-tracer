use rand::prelude::*;

use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::material::{Scatter, Material};

pub struct MetalMaterial<T>
    where T: Float
{
    pub color: Vec3<T>
}

impl<T> MetalMaterial<T>
    where T: Float
{
    fn reflect(direction: &Vec3<T>, normal: &Vec3<T>) -> Vec3<T> {
        let two = T::from(2.0).unwrap();
        let c = direction.dot(normal);
        let reflection = direction - normal * two * c;
        reflection
    }
}

impl<T> Material<T> for MetalMaterial<T>
    where T: Float
{
    fn scatter(&self, incident: &Ray<T>, hit: &Hit<T>) -> Scatter<T> {
        let attenuation = Vec3::<T>::from_slice(self.color.get_data());
        let mut normal = Vec3::from_slice(hit.normal.get_data());
        normal.normalize();
        let origin = Vec3::from_slice(hit.point.get_data());
        let mut direction = MetalMaterial::<T>::reflect(incident.get_direction(), &normal);
        direction.normalize();
        let scattered = Some(Ray::<T>::from_vec(origin, direction));
        Scatter::<T> {
            attenuation,
            scattered
        }
    }
}

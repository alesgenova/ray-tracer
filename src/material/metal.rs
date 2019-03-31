use rand::prelude::*;

use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::material::{Scatter, Material};
use crate::utils::random_point_in_sphere;

pub struct MetalMaterial<T>
    where T: Float
{
    pub color: Vec3<T>,
    pub fuzziness: T
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
        let normal = &hit.normal;
        let origin = Vec3::from_slice(hit.point.get_data());
        let mut direction = MetalMaterial::<T>::reflect(incident.get_direction(), &normal);
        direction.normalize();
        if self.fuzziness > T::zero() {
          direction = direction + random_point_in_sphere(self.fuzziness);
          direction.normalize();
        }
        let scattered = Some(Ray::<T>::from_vec(origin, direction));
        Scatter::<T> {
            attenuation,
            scattered
        }
    }
}

use rand::prelude::*;

use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::material::{Scatter, Material};
use crate::utils::{random_point_in_sphere, reflect};

pub struct MetalMaterial<T>
    where T: Float
{
    pub color: Vec3<T>,
    pub fuzziness: T
}

impl<T> Material<T> for MetalMaterial<T>
    where T: Float
{
    fn scatter(&self, incident: &Ray<T>, hit: &Hit<T>) -> Scatter<T> {
        let attenuation = Vec3::<T>::from_slice(self.color.get_data());
        let normal = &hit.normal;
        let origin = Vec3::from_slice(hit.point.get_data());
        let mut direction = reflect(incident.get_direction(), &normal);
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

use rand::prelude::*;

use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::material::{Scatter, Material};

pub struct LambertianMaterial<T>
    where T: Float
{
    pub color: Vec3<T>
}

impl<T> LambertianMaterial<T>
    where T: Float
{
    fn random_point_in_sphere(radius: T) -> Vec3<T> {
        let mut point = Vec3::<T>::new();
        let mut rng = rand::thread_rng();

        loop {
            let x = T::from(rng.gen::<f64>()).unwrap();
            let y = T::from(rng.gen::<f64>()).unwrap();
            let z = T::from(rng.gen::<f64>()).unwrap();

            let len = (x * x + y * y + z * z).sqrt();

            if len < T::one() {
                point.set_data(&[x * radius, y * radius, z * radius]);
                break;
            }
        }

        point
    }
}

impl<T> Material<T> for LambertianMaterial<T>
    where T: Float
{
    fn scatter(&self, incident: &Ray<T>, hit: &Hit<T>) -> Scatter<T> {
        let attenuation = Vec3::<T>::from_slice(self.color.get_data());
        let mut normal = Vec3::from_slice(hit.normal.get_data());
        normal.normalize();
        let origin = Vec3::from_slice(hit.point.get_data());
        let direction = normal + LambertianMaterial::<T>::random_point_in_sphere(T::one());
        let scattered = Some(Ray::<T>::from_vec(origin, direction));
        Scatter::<T> {
            attenuation,
            scattered
        }
    }
}

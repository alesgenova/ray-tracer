use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::material::{Scatter, Material};

pub struct PlainMaterial<T>
  where T: Float
{
  pub color: Vec3<T>
}

impl<T> Material<T> for PlainMaterial<T>
  where T: Float
{
  fn scatter(&self, incident: &Ray<T>, hit: &Hit<T>) -> Scatter<T> {
    let attenuation = Vec3::<T>::from_slice(self.color.get_data());
    Scatter::<T> {
      attenuation,
      scattered: None
    }
  }
}

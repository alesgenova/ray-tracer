use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::material::{Scatter, Material};
use crate::texture::Texture;

pub struct PlainMaterial<T>
    where T: Float
{
    texture: Box<dyn Texture<T>>
}

impl<T> PlainMaterial<T>
    where T: Float
{
    pub fn new(texture: Box<Texture<T>>) -> Self {
        PlainMaterial {
            texture
        }
    }
}

impl<T> Material<T> for PlainMaterial<T>
    where T: Float
{
    fn scatter(&self, _incident: &Ray<T>, hit: &Hit<T>) -> Scatter<T> {
        let color = self.texture.get_color(T::zero(), T::zero(), &hit.point);
        let attenuation = Vec3::<T>::from_slice(color.get_data());
        Scatter::<T> {
            attenuation,
            scattered: None
        }
    }
}

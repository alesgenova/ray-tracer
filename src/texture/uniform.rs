use crate::float::Float;
use crate::vector::Vec3;
use super::Texture;

pub struct UniformTexture<T>
    where T: Float
{
    color: Vec3<T>
}

impl<T> UniformTexture<T>
    where T: Float
{
    pub fn new(color: Vec3<T>) -> Self {
        UniformTexture {
            color
        }
    }
}

impl<T> Texture<T> for UniformTexture<T>
    where T: Float
{
    fn get_color(&self, _u: T, _v: T, _point: &Vec3<T>) -> Vec3<T> {
        Vec3::<T>::from_slice(self.color.get_data())
    }
}

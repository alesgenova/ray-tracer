use crate::float::Float;
use crate::vector::Vec3;

pub mod uniform;
pub mod checker;

pub trait Texture<T>
    where T: Float
{
    fn get_color(&self, u: T, v: T, point: &Vec3<T>) -> Vec3<T>;
}

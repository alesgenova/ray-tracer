use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;

#[derive(Debug)]
pub struct Hit<T>
    where T: Float
{
    point: Vec3<T>,
    normal: Vec3<T>,
    t: T
}

pub trait Hitable<T>
    where T: Float
{
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<Hit<T>>;
}

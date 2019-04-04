use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::boundingbox::BoundingBox;

pub mod sphere;
pub mod transform;

pub trait Hitable<T>
    where T: Float
{
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<Hit<T>>;
    fn get_bounds(&self) -> &BoundingBox<T>;
    fn unwrap(self) -> Box<Hitable<T>>;
}

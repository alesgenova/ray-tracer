use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::boundingbox::BoundingBox;

pub mod primitive;
pub mod transform;

pub trait Hitable<T>
    where T: Float
{
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<Hit<T>>;
    fn get_bounds(&self) -> &BoundingBox<T>;
    fn unwrap(self: Box<Self>) -> Box<dyn Hitable<T>>;
    fn is_primitive(&self) -> bool {
        // Primitives (i.e. spheres, boxes, rectangles) return true,
        // Decorators (i.e. translations, rotations) return false
        true
    }
}

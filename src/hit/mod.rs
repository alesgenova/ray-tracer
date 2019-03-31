use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::actor::Actor;

pub struct Hit<T>
    where T: Float
{
    pub point: Vec3<T>,
    pub normal: Vec3<T>,
    pub t: T
}

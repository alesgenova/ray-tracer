use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;

pub mod plain;

pub struct Scatter<T>
    where T: Float
{
    pub attenuation: Vec3<T>,
    pub scattered: Option<Ray<T>>
}

pub trait Material<T>
    where T: Float
{
    fn scatter(&self, incident: &Ray<T>, hit: &Hit<T>) -> Scatter<T>;
}

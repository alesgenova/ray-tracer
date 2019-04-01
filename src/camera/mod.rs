use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;

pub mod perspective;

pub enum CameraLock {
    Direction,
    LookAt
}

pub trait Camera<T>
    where T: Float
{
    fn get_position(&self) -> &Vec3<T>;
    fn set_position(&mut self, position: &[T]);

    fn get_direction(&self) -> &Vec3<T>;
    fn set_direction(&mut self, direction: &[T]);

    fn get_lookat(&self) -> &Vec3<T>;
    fn set_lookat(&mut self, lookat: &[T]);

    fn get_up(&self) -> &Vec3<T>;
    fn set_up(&mut self, up: &[T]);

    fn get_ray(&self, r: T, s: T) -> Ray<T>;
}

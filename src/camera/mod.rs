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

    fn get_aperture(&self) -> T;
    fn set_aperture(&mut self, aperture: T);

    fn get_focus(&self) -> T;
    fn set_focus(&mut self, focus: T);

    fn get_aspect(&self) -> T;
    fn set_aspect(&mut self, aspect: T);

    fn get_fov(&self) -> T;
    fn set_fov(&mut self, fov: T);

    fn get_ray(&self, r: T, s: T) -> Ray<T>;
}

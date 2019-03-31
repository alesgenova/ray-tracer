use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::camera::Camera;

pub struct PerspectiveCamera<T>
    where T: Float
{
    position: Vec3<T>,
    direction: Vec3<T>,
    up: Vec3<T>,
    u: Vec3<T>,
    v: Vec3<T>,
    w: Vec3<T>,
    aspect: T,
    fov: T
}

impl<T> PerspectiveCamera<T>
    where T: Float
{
    pub fn new() -> Self {
        let mut camera = PerspectiveCamera {
            position: Vec3::<T>::from_array([T::zero(), T::zero(), T::zero()]),
            direction: Vec3::<T>::from_array([T::zero(), T::zero(), -T::one()]),
            up: Vec3::<T>::from_array([T::zero(), T::one(), T::zero()]),
            u: Vec3::<T>::new(),
            v: Vec3::<T>::new(),
            w: Vec3::<T>::new(),
            aspect: T::one(),
            fov: T::from(0.5 * 3.1415).unwrap()
        };
        camera.update();
        camera
    }

    pub fn get_aspect(&self) -> T {
        self.aspect
    }

    pub fn set_aspect(&mut self, aspect: T) {
        self.aspect = aspect;
    }

    pub fn get_fov(&self) -> T {
        self.fov
    }

    pub fn set_fov(&mut self, fov: T) {
        self.fov = fov;
    }

    pub fn update(&mut self) {
        self.w.set_data(self.direction.get_data());
        self.w.normalize();
        self.u.set_data(self.w.cross(&self.up).get_data());
        self.u.normalize();
        self.v.set_data(self.w.cross(&self.u).get_data());
        self.v.normalize();
    }
}

impl<T> Camera<T> for PerspectiveCamera<T>
    where T: Float
{
    fn get_position(&self) -> &Vec3<T> {
        &self.position
    }

    fn set_position(&mut self, position: &[T]) {
        self.position.set_data(position);
    }

    fn get_direction(&self) -> &Vec3<T> {
        &self.direction
    }

    fn set_direction(&mut self, direction: &[T]) {
        self.direction.set_data(direction);
        self.update();
    }

    fn get_up(&self) -> &Vec3<T> {
        &self.up
    }

    fn set_up(&mut self, up: &[T]) {
        self.up.set_data(up);
        self.update();
    }

    fn get_ray(&self, r: T, s: T) -> Ray<T> {
        let half_height = ( T::from(0.5).unwrap() * self.fov ).tan();
        let half_width = self.aspect * half_height;
        let center = &self.position + &self.w;

        let mut ray_direction = &center + &self.u * r * half_width + &self.v * s * half_height - &self.position;
        ray_direction.normalize();
        Ray::<T>::from_slice(self.position.get_data(), ray_direction.get_data())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_camera<T>(camera: &PerspectiveCamera<T>)
        where T: Float + std::fmt::Debug
    {
        assert_eq!(camera.u.norm(), T::one());
        assert_eq!(camera.v.norm(), T::one());
        assert_eq!(camera.w.norm(), T::one());
        assert_eq!(camera.u.dot(&camera.v), T::zero());
        assert_eq!(camera.u.dot(&camera.w), T::zero());
        assert_eq!(camera.v.dot(&camera.w), T::zero());
    }

    #[test]
    fn init() {
        let camera = PerspectiveCamera::<f64>::new();
        check_camera(&camera);
    }

    #[test]
    fn set() {
        let mut camera = PerspectiveCamera::<f64>::new();
        let position = [-2.0, 0.0, 0.0];
        camera.set_position(&position);
        assert_eq!(camera.get_position().get_data(), position);
        check_camera(&camera);

        let direction = [2.0, 0.0, 0.0];
        camera.set_direction(&direction);
        assert_eq!(camera.get_direction().get_data(), direction);
        check_camera(&camera);

        let up = [2.0, 4.0, 0.0];
        camera.set_up(&up);
        assert_eq!(camera.get_up().get_data(), up);
        check_camera(&camera);

        assert_eq!(camera.w.get_data(), [1.0, 0.0, 0.0]);
        assert_eq!(camera.u.get_data(), [0.0, 0.0, 1.0]);
        assert_eq!(camera.v.get_data(), [0.0, -1.0, 0.0]);
    }

    #[test]
    fn rays() {
        let mut camera = PerspectiveCamera::<f64>::new();
        camera.set_fov(0.5 * std::f64::consts::PI);
        camera.set_aspect(2.0);
        camera.set_position(&[0., 0., -10.]);

        let ray = camera.get_ray(0.0, 1.0);
        assert_eq!(ray.get_origin().get_data(), camera.get_position().get_data());
        let ray = camera.get_ray(0.0, -1.0);
        assert_eq!(ray.get_origin().get_data(), camera.get_position().get_data());
        let ray = camera.get_ray(1.0, 0.0);
        assert_eq!(ray.get_origin().get_data(), camera.get_position().get_data());
        let ray = camera.get_ray(-1.0, 0.0);
    }
}

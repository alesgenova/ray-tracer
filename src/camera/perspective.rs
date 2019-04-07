use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::camera::{Camera, CameraLock};
use crate::utils::random_point_in_circle;

pub struct PerspectiveCamera<T>
    where T: Float
{
    position: Vec3<T>,
    direction: Vec3<T>,
    lookat: Vec3<T>,
    center: Vec3<T>,
    up: Vec3<T>,
    u: Vec3<T>,
    v: Vec3<T>,
    w: Vec3<T>,
    aspect: T,
    fov: T,
    half_height: T,
    half_width: T,
    aperture: T,
    focus: T,
    lock: CameraLock
}

impl<T> PerspectiveCamera<T>
    where T: Float
{
    pub fn new() -> Self {
        let mut camera = PerspectiveCamera {
            position: Vec3::<T>::from_array([T::zero(), T::zero(), T::zero()]),
            direction: Vec3::<T>::from_array([T::zero(), T::zero(), -T::one()]),
            lookat: Vec3::<T>::from_array([T::zero(), T::zero(), -T::one()]),
            center: Vec3::<T>::from_array([T::zero(), T::zero(), -T::one()]),
            up: Vec3::<T>::from_array([T::zero(), T::one(), T::zero()]),
            u: Vec3::<T>::new(),
            v: Vec3::<T>::new(),
            w: Vec3::<T>::new(),
            aspect: T::one(),
            half_height: T::one(),
            half_width: T::one(),
            aperture: T::zero(),
            focus: T::one(),
            fov: T::from(0.5 * 3.1415).unwrap(),
            lock: CameraLock::Direction
        };
        camera.update();
        camera
    }

    pub fn update(&mut self) {
        let direction = match self.lock {
            CameraLock::Direction => {
                self.get_direction() * T::one()
            },
            CameraLock::LookAt => {
                self.get_lookat() - self.get_position()
            }
        };
        self.w.set_data(direction.get_data());
        self.w.normalize();
        self.u.set_data(self.w.cross(&self.up).get_data());
        self.u.normalize();
        self.v.set_data(self.w.cross(&self.u).get_data());
        self.v.normalize();
        self.center = &self.position + &self.w * self.focus;
        self.half_height = ( T::from(0.5).unwrap() * self.fov ).tan() * self.focus;
        self.half_width = self.aspect * self.half_height;
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
        self.lock = CameraLock::Direction;
        self.update();
    }

    fn get_lookat(&self) -> &Vec3<T> {
        &self.lookat
    }

    fn set_lookat(&mut self, lookat: &[T]) {
        self.lookat.set_data(lookat);
        self.lock = CameraLock::LookAt;
        self.update();
    }

    fn get_up(&self) -> &Vec3<T> {
        &self.up
    }

    fn set_up(&mut self, up: &[T]) {
        self.up.set_data(up);
        self.update();
    }

    fn get_aperture(&self) -> T {
        self.aperture
    }

    fn set_aperture(&mut self, aperture: T) {
        self.aperture = aperture;
        self.update();
    }

    fn get_focus(&self) -> T {
        self.focus
    }

    fn set_focus(&mut self, focus: T) {
        self.focus = focus;
        self.update();
    }

    fn get_aspect(&self) -> T {
        self.aspect
    }

    fn set_aspect(&mut self, aspect: T) {
        self.aspect = aspect;
        self.update();
    }

    fn get_fov(&self) -> T {
        self.fov
    }

    fn set_fov(&mut self, fov: T) {
        self.fov = fov;
        self.update();
    }

    fn get_ray(&self, r: T, s: T) -> Ray<T> {
        let offset = if self.aperture > T::zero() {
            random_point_in_circle(self.aperture * T::from(0.5).unwrap())
        } else {
            Vec3::<T>::new()
        };
        let mut ray_direction = &self.center + &self.u * r * self.half_width + &self.v * s * self.half_height - &self.position - &offset;
        ray_direction.normalize();
        let origin = &self.position + &offset;
        Ray::<T>::from_slice(origin.get_data(), ray_direction.get_data())
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

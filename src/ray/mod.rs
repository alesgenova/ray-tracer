use crate::float::Float;
use crate::vector::Vec3;

#[derive(Debug)]
pub struct Ray<T>
    where T: Float
{
    origin: Vec3<T>,
    direction: Vec3<T>
}

impl<T> Ray<T>
    where T: Float
{
    pub fn new() -> Self {
        let origin = [T::zero(); 3];
        let direction = [T::zero(); 3];
        Ray {
            origin: Vec3::from_array(origin),
            direction: Vec3::from_array(direction),
        }
    }

    pub fn from_vec(origin: Vec3<T>, direction: Vec3<T>) -> Self {
        Ray {
            origin,
            direction
        }
    }

    pub fn from_array(origin: [T; 3], direction: [T; 3]) -> Self {
        Ray {
            origin: Vec3::from_array(origin),
            direction: Vec3::from_array(direction),
        }
    }

    pub fn from_slice(origin: &[T], direction: &[T]) -> Self {
        Ray {
            origin: Vec3::from_slice(origin),
            direction: Vec3::from_slice(direction),
        }
    }

    pub fn get_origin(&self) -> &Vec3<T> {
        &self.origin
    }

    pub fn get_direction(&self) -> &Vec3<T> {
        &self.direction
    }

    pub fn get_point(&self, t: T) -> Vec3<T> {
        &self.origin + &self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let ray = Ray::<f64>::new();
        assert_eq!(ray.get_origin().get_data(), [0.0, 0.0, 0.0]);
        assert_eq!(ray.get_direction().get_data(), [0.0, 0.0, 0.0]);

        let origin = Vec3::<f64>::from_array([0.0, -1.0, -2.0]);
        let direction = Vec3::<f64>::from_array([0.0, 0.0, 1.0]);
        let ray = Ray::from_vec(origin, direction);
        assert_eq!(ray.get_origin().get_data(), [0.0, -1.0, -2.0]);
        assert_eq!(ray.get_direction().get_data(), [0.0, 0.0, 1.0]);

        let origin = [0.0, 1.0, 2.0];
        let direction = [0.0, 0.0, -1.0];
        let ray = Ray::from_array(origin, direction);
        assert_eq!(ray.get_origin().get_data(), [0.0, 1.0, 2.0]);
        assert_eq!(ray.get_direction().get_data(), [0.0, 0.0, -1.0]);

        let origin = vec!(-1.0, 1.0, 2.0);
        let direction = vec!(-1.0, 0.0, -1.0);
        let ray = Ray::<f64>::from_slice(&origin, &direction);
        assert_eq!(ray.get_origin().get_data(), [-1.0, 1.0, 2.0]);
        assert_eq!(ray.get_direction().get_data(), [-1.0, 0.0, -1.0]);
    }

    #[test]
    fn point() {
        let origin = [0.0, 1.0, 2.0];
        let direction = [1.0, 2.0, 3.0];
        let ray = Ray::from_array(origin, direction);
        let t = -1.5;
        let p = ray.get_point(t);
        assert_eq!(p.get_data(), [-1.5, -2.0, -2.5]);
    }
}

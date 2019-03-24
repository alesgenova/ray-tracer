use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hitable::{Hitable, Hit};

pub struct Sphere<T>
    where T: Float
{
    center: Vec3<T>,
    radius: T
}

impl<T> Sphere<T>
    where T: Float
{
    pub fn new() -> Self {
        Sphere {
            center: Vec3::<T>::new(),
            radius: T::one()
        }
    }

    pub fn from(center: Vec3<T>, radius: T) -> Self {
        Sphere {
            center,
            radius
        }
    }

    pub fn get_center(&self) -> &Vec3<T> {
        &self.center
    }

    pub fn set_center(&mut self, center: &[T]) {
        self.center.set_data(center);
    }

    pub fn get_radius(&self) -> T {
        self.radius
    }

    pub fn set_radius(&mut self, radius: T) {
        self.radius = radius;
    }
}

impl<T> Hitable<T> for Sphere<T>
    where T: Float
{
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<Hit<T>> {
        // Intersection of a line and a sphere:
        //
        // p(t) = origin + t * direction
        //
        //   dot( (p(t) - center), (p(t) - center) )
        // = radius * radius
        //
        //   t * t * dot(direction, direction)
        // + 2 * t * dot(direction, origin - center)
        // + dot(origin - center, origin - center)
        // - radius * radius
        // = 0
        //
        // t = (- b +/- sqrt(b * b - 4 * a * c)) / (2 * a)
        // 
        // drop 2s coming from b
        let oc = ray.get_origin() - self.get_center();
        let a = ray.get_direction().dot(ray.get_direction());
        let b = ray.get_direction().dot(&oc);
        let c = oc.dot(&oc) - self.get_radius() * self.get_radius();
        let discriminant = b * b - a * c;
        if discriminant <= T::zero() {
            return None;
        }
        let discriminant = discriminant.sqrt();
        let t0 = (- b - discriminant) / a;
        let t1 = (- b + discriminant) / a;
        let t = if t0 >= t_min && t0 < t_max { t0 }
                else if t1 >= t_min && t1 < t_max { t1 }
                else { return None; };

        let point = ray.get_point(t);
        let normal = (&point - self.get_center()) / self.get_radius();
        let hit = Hit {
            point,
            normal,
            t
        };

        Some(hit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let sphere = Sphere::<f64>::new();
        assert_eq!(sphere.get_center().get_data(), [0.0, 0.0, 0.0]);
        assert_eq!(sphere.get_radius(), 1.0);

        let center = Vec3::from_array([10.0, 20.0, 30.0]);
        let radius = 5.5;
        let sphere = Sphere::<f64>::from(center, radius);
        assert_eq!(sphere.get_center().get_data(), [10.0, 20.0, 30.0]);
        assert_eq!(sphere.get_radius(), 5.5);
    }

    #[test]
    fn set() {
        let mut sphere = Sphere::<f64>::new();
        let center = [10.0, 20.0, 30.0];
        let radius = 5.5;
        sphere.set_center(&center);
        sphere.set_radius(radius);
        assert_eq!(sphere.get_center().get_data(), [10.0, 20.0, 30.0]);
        assert_eq!(sphere.get_radius(), 5.5);
    }

    #[test]
    fn hit() {
        let center = Vec3::from_array([10.0, 0.0, 0.0]);
        let radius = 2.0;
        let sphere = Sphere::<f64>::from(center, radius);

        let origin = [-1.0, 0.0, 0.0];
        let direction = [2.0, 0.0, 0.0];
        let ray = Ray::from_array(origin, direction);
        let hit = sphere.hit(&ray, 0.0, 100.0);
        match hit {
            Some(hit) => {
                assert_eq!(hit.point.get_data(), [8.0, 0.0, 0.0]);
                assert_eq!(hit.normal.get_data(), [-1.0, 0.0, 0.0]);
                assert_eq!(hit.t, 4.5);
            },
            None => {
                assert!(false);
            }
        }

        let origin = [-1.0, 2.1, 0.0];
        let direction = [2.0, 0.0, 0.0];
        let ray = Ray::from_array(origin, direction);
        let hit = sphere.hit(&ray, 0.0, 100.0);
        match hit {
            Some(hit) => {
                assert!(false);
            },
            None => {}
        }

        let origin = [-1.0, 2.1, 0.0];
        let direction = [0.0, 2.0, 0.0];
        let ray = Ray::from_array(origin, direction);
        let hit = sphere.hit(&ray, 0.0, 100.0);
        match hit {
            Some(hit) => {
                assert!(false);
            },
            None => {}
        }
    }

}

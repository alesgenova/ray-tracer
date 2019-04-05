use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::hitable::Hitable;
use crate::boundingbox::BoundingBox;

pub struct Sphere<T>
    where T: Float
{
    radius: T,
    bounds: BoundingBox<T>
}

impl<T> Sphere<T>
    where T: Float
{
    pub fn new(radius: T) -> Self {
        let mut sphere = Sphere {
            radius,
            bounds: BoundingBox::<T>::new(Vec3::<T>::new(), Vec3::<T>::new())
        };
        sphere.update_bounds();
        sphere
    }

    pub fn get_radius(&self) -> T {
        self.radius
    }

    pub fn set_radius(&mut self, radius: T) {
        self.radius = radius;
        self.update_bounds();
    }

    fn update_bounds(&mut self) {
        let one = Vec3::<T>::from_array([T::one(), T::one(), T::one()]);
        let p0 = &one * self.get_radius() * (- T::one());
        let p1 = &one * self.get_radius();
        self.bounds = BoundingBox::<T>::new(p0, p1);
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
        let oc = ray.get_origin();
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
        let normal = (&point) / self.get_radius();
        let hit = Hit {
            point,
            normal,
            t
        };

        Some(hit)
    }

    fn get_bounds(&self) -> &BoundingBox<T> {
        &self.bounds
    }

    fn unwrap(self: Box<Self>) -> Box<dyn Hitable<T>> {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let sphere = Sphere::<f64>::new(1.0);
        assert_eq!(sphere.get_radius(), 1.0);

        let radius = 5.5;
        let sphere = Sphere::<f64>::new(radius);
        assert_eq!(sphere.get_radius(), 5.5);
    }

    #[test]
    fn set() {
        let mut sphere = Sphere::<f64>::new(1.0);
        let radius = 5.5;
        sphere.set_radius(radius);
        assert_eq!(sphere.get_radius(), 5.5);
    }

    #[test]
    fn hit() {
        let radius = 2.0;
        let sphere = Sphere::<f64>::new(radius);

        let origin = [-8.0, 0.0, 0.0];
        let direction = [2.0, 0.0, 0.0];
        let ray = Ray::from_array(origin, direction);
        let hit = sphere.hit(&ray, 0.0, 100.0);
        match hit {
            Some(hit) => {
                assert_eq!(hit.point.get_data(), [-2.0, 0.0, 0.0]);
                assert_eq!(hit.normal.get_data(), [-1.0, 0.0, 0.0]);
                assert_eq!(hit.t, 3.0);
            },
            None => {
                assert!(false);
            }
        }

        let origin = [-8.0, 2.1, 0.0];
        let direction = [2.0, 0.0, 0.0];
        let ray = Ray::from_array(origin, direction);
        let hit = sphere.hit(&ray, 0.0, 100.0);
        match hit {
            Some(hit) => {
                assert!(false);
            },
            None => {}
        }

        let origin = [-8.0, 0.0, 0.0];
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

    #[test]
    fn bounds() {
        let radius = 2.5;
        let sphere = Sphere::<f64>::new(radius);
        let bounds = sphere.get_bounds();
        assert_eq!(bounds.get_p0().get_data(), [-2.5, -2.5, -2.5]);
        assert_eq!(bounds.get_p1().get_data(), [2.5, 2.5, 2.5]);
    }
}

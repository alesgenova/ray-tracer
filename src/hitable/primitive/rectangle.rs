use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::hitable::Hitable;
use crate::boundingbox::BoundingBox;
use crate::constants::Axis;
use crate::utils::axis_to_index;

pub struct Rectangle<T>
    where T: Float
{
    width: T,
    width_axis: Axis,
    height: T,
    height_axis: Axis,
    normal_axis: Axis,
    normal: Vec3<T>,
    bounds: BoundingBox<T>
}

impl<T> Rectangle<T>
    where T: Float
{
    pub fn new(width: T, width_axis: Axis, height: T, height_axis: Axis) -> Self {
        let (normal_axis, normal_direction) = Rectangle::<T>::normal_axis(&width_axis, &height_axis);
        let half = T::from(0.5).unwrap();
        let p0 = Rectangle::<T>::length_to_point(- width * half, &width_axis);
        let p0 = p0 + Rectangle::<T>::length_to_point(- height * half, &height_axis);
        
        let p1 = Rectangle::<T>::length_to_point(width * half, &width_axis);
        let p1 = p1 + Rectangle::<T>::length_to_point(height * half, &height_axis);

        let normal = Rectangle::<T>::length_to_point(normal_direction, &normal_axis);
        let bounds = BoundingBox::<T>::new(p0, p1);

        Rectangle {
            width,
            width_axis,
            height,
            height_axis,
            normal_axis,
            normal,
            bounds
        }
    }

    pub fn get_normal(&self) -> &Vec3<T> {
        &self.normal
    }

    fn length_to_point(length: T, axis: &Axis) -> Vec3<T> {
        match axis {
            Axis::X => Vec3::<T>::from_array([length, T::zero(), T::zero()]),
            Axis::Y => Vec3::<T>::from_array([T::zero(), length, T::zero()]),
            Axis::Z => Vec3::<T>::from_array([T::zero(), T::zero(), length])
        }
    }

    fn normal_axis(width_axis: &Axis, height_axis: &Axis) -> (Axis, T) {
        match (width_axis, height_axis) {
            (Axis::X, Axis::Y) => (Axis::Z, T::one()),
            (Axis::Y, Axis::X) => (Axis::Z, -T::one()),

            (Axis::Y, Axis::Z) => (Axis::X, T::one()),
            (Axis::Z, Axis::Y) => (Axis::X, -T::one()),

            (Axis::Z, Axis::X) => (Axis::Y, T::one()),
            (Axis::X, Axis::Z) => (Axis::Y, -T::one()),

            _ => panic!("Rectangle cannot have width and height along the same axis")
        }
    }
}

impl<T> Hitable<T> for Rectangle<T>
    where T: Float
{
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<Hit<T>> {
        let oc = ray.get_origin();
        let direction = ray.get_direction();

        let normal_index = axis_to_index(&self.normal_axis);

        let t = - oc.get_data()[normal_index] / direction.get_data()[normal_index];
        if t <= t_min || t > t_max {
            return None;
        }

        let width_index = axis_to_index(&self.width_axis);
        let height_index = axis_to_index(&self.height_axis);
        let width = oc.get_data()[width_index] + t * direction.get_data()[width_index];
        let height = oc.get_data()[height_index] + t * direction.get_data()[height_index];

        let w0 = self.bounds.get_p0().get_data()[width_index];
        let w1 = self.bounds.get_p1().get_data()[width_index];

        let h0 = self.bounds.get_p0().get_data()[height_index];
        let h1 = self.bounds.get_p1().get_data()[height_index];

        if width < w0 || width > w1 || height < h0 || height > h1 {
            return None;
        }

        let point = ray.get_point(t);
        let normal = &self.normal * T::one();
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
        let width = 2.0;
        let width_axis = Axis::X;

        let height = 4.0;
        let height_axis = Axis::Y;

        let rectangle = Rectangle::<f64>::new(width, width_axis, height, height_axis);
        let normal = &rectangle.normal;
        assert_eq!(normal.get_data(), [0.0, 0.0, 1.0]);

        let width = 3.0;
        let width_axis = Axis::X;

        let height = 5.0;
        let height_axis = Axis::Z;

        let rectangle = Rectangle::<f64>::new(width, width_axis, height, height_axis);
        let normal = &rectangle.normal;
        assert_eq!(normal.get_data(), [0.0, -1.0, 0.0]);

        let width = 2.0;
        let width_axis = Axis::Y;

        let height = 4.0;
        let height_axis = Axis::Z;

        let rectangle = Rectangle::<f64>::new(width, width_axis, height, height_axis);
        let normal = &rectangle.normal;
        assert_eq!(normal.get_data(), [1.0, 0.0, 0.0]);
    }

    #[test]
    fn hit() {
        let width = 2.0;
        let width_axis = Axis::X;
        let height = 4.0;
        let height_axis = Axis::Y;
        let rectangle = Rectangle::<f64>::new(width, width_axis, height, height_axis);

        let origin = [0.0, 0.0, 8.0];
        let direction = [0.0, 0.0, -2.0];
        let ray = Ray::from_array(origin, direction);
        let hit = rectangle.hit(&ray, 0.0, 100.0);

        match hit {
            Some(hit) => {
                assert_eq!(hit.point.get_data(), [0.0, 0.0, 0.0]);
                assert_eq!(hit.normal.get_data(), [0.0, 0.0, 1.0]);
                assert_eq!(hit.t, 4.0);
            },
            None => {
                assert!(false);
            }
        }

        let origin = [0.5, 1.5, -8.0];
        let direction = [0.0, 0.0, 2.0];
        let ray = Ray::from_array(origin, direction);
        let hit = rectangle.hit(&ray, 0.0, 100.0);

        match hit {
            Some(hit) => {
                assert_eq!(hit.point.get_data(), [0.5, 1.5, 0.0]);
                assert_eq!(hit.normal.get_data(), [0.0, 0.0, 1.0]);
                assert_eq!(hit.t, 4.0);
            },
            None => {
                assert!(false);
            }
        }

        let origin = [1.0001, 2.0001, 8.0];
        let direction = [0.0, 0.0, -2.0];
        let ray = Ray::from_array(origin, direction);

        if let Some(hit) = rectangle.hit(&ray, 0.0, 100.0) {
            assert!(false);
        };
    }

    #[test]
    fn bounds() {
        let width = 2.0;
        let width_axis = Axis::X;

        let height = 4.0;
        let height_axis = Axis::Y;

        let rectangle = Rectangle::<f64>::new(width, width_axis, height, height_axis);
        let bounds = rectangle.get_bounds();
        assert_eq!(bounds.get_p0().get_data(), [-1.0, -2.0, 0.0]);
        assert_eq!(bounds.get_p1().get_data(), [1.0, 2.0, 0.0]);

        let width = 3.0;
        let width_axis = Axis::X;

        let height = 5.0;
        let height_axis = Axis::Z;

        let rectangle = Rectangle::<f64>::new(width, width_axis, height, height_axis);
        let bounds = rectangle.get_bounds();
        assert_eq!(bounds.get_p0().get_data(), [-1.5, 0.0, -2.5]);
        assert_eq!(bounds.get_p1().get_data(), [1.5, 0.0, 2.5]);

        let width = 2.0;
        let width_axis = Axis::Y;

        let height = 4.0;
        let height_axis = Axis::Z;

        let rectangle = Rectangle::<f64>::new(width, width_axis, height, height_axis);
        let bounds = rectangle.get_bounds();
        assert_eq!(bounds.get_p0().get_data(), [0.0, -1.0, -2.0]);
        assert_eq!(bounds.get_p1().get_data(), [0.0, 1.0, 2.0]);
    }
}

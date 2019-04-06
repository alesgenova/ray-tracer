use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::hitable::Hitable;
use crate::hitable::primitive::Rectangle;
use crate::hitable::transform::Translation;
use crate::boundingbox::BoundingBox;
use crate::constants::Axis;
use crate::utils::axis_to_index;

pub struct Cube<T>
    where T: Float
{
    length: T,
    width: T,
    height: T,
    faces: Vec<Box<Hitable<T>>>,
    bounds: BoundingBox<T>
}

impl<T> Cube<T>
    where T: Float
{
    pub fn new(length: T, width: T, height: T) -> Self {
        let axes = [
            (Axis::X, Axis::Y),
            (Axis::Y, Axis::X),
            (Axis::Y, Axis::Z),
            (Axis::Z, Axis::Y),
            (Axis::Z, Axis::X),
            (Axis::X, Axis::Z)
        ];

        let lengths = [
            (length, width, height),
            (width, length, height),
            (width, height, length),
            (height, width, length),
            (height, length, width),
            (length, height, width)
        ];

        let half = T::from(0.5).unwrap();
        let mut faces = vec!();

        for i in 0..6 {
            let (width, height, depth) = lengths[i];
            let (width_axis, height_axis) = axes[i];
            let face = Box::new(Rectangle::<T>::new(width, width_axis, height, height_axis));
            let translation = face.get_normal() * depth * half;
            let face : Box<Hitable<T>> = Box::new(Translation::new(face, translation));
            faces.push(face);
        };

        let bounds = BoundingBox::<T>::new(
            Vec3::from_array([- length * half, - width * half, - height * half]),
            Vec3::from_array([length * half, width * half, height * half]),
        );

        Cube {
            length,
            width,
            height,
            faces,
            bounds
        }
    }
}

impl<T> Hitable<T> for Cube<T>
    where T: Float
{
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<Hit<T>> {
        let mut t_max = t_max;
        let mut result : Option<Hit<T>> = None;

        for i in 0..self.faces.len() {
            if let Some(hit) = self.faces[i].hit(ray, t_min, t_max) {
                t_max = hit.t;
                result = Some(hit);
            }
        }
        result
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
        let length = 2.0;
        let width = 2.0;
        let height = 4.0;
        let cube = Cube::<f64>::new(length, width, height);
    }

    #[test]
    fn hit() {
        let length = 2.0;
        let width = 4.0;
        let height = 6.0;
        let cube = Cube::<f64>::new(length, width, height);

        // Hit YZ faces
        let origin = [-8.0, 0.0, 0.0];
        let direction = [2.0, 0.0, 0.0];
        let ray = Ray::from_array(origin, direction);
        let hit = cube.hit(&ray, 0.0, 100.0);

        match hit {
            Some(hit) => {
                assert_eq!(hit.point.get_data(), [-1.0, 0.0, 0.0]);
                assert_eq!(hit.normal.get_data(), [-1.0, 0.0, 0.0]);
                assert_eq!(hit.t, 3.5);
            },
            None => {
                assert!(false);
            }
        }

        let origin = [8.0, 0.0, 0.0];
        let direction = [-2.0, 0.0, 0.0];
        let ray = Ray::from_array(origin, direction);
        let hit = cube.hit(&ray, 0.0, 100.0);

        match hit {
            Some(hit) => {
                assert_eq!(hit.point.get_data(), [1.0, 0.0, 0.0]);
                assert_eq!(hit.normal.get_data(), [1.0, 0.0, 0.0]);
                assert_eq!(hit.t, 3.5);
            },
            None => {
                assert!(false);
            }
        }

        // Hit XZ faces
        let origin = [0.0, -8.0, 0.0];
        let direction = [0.0, 2.0, 0.0];
        let ray = Ray::from_array(origin, direction);
        let hit = cube.hit(&ray, 0.0, 100.0);

        match hit {
            Some(hit) => {
                assert_eq!(hit.point.get_data(), [0.0, -2.0, 0.0]);
                assert_eq!(hit.normal.get_data(), [0.0, -1.0, 0.0]);
                assert_eq!(hit.t, 3.0);
            },
            None => {
                assert!(false);
            }
        }

        let origin = [0.0, 8.0, 0.0];
        let direction = [0.0, -2.0, 0.0];
        let ray = Ray::from_array(origin, direction);
        let hit = cube.hit(&ray, 0.0, 100.0);

        match hit {
            Some(hit) => {
                assert_eq!(hit.point.get_data(), [0.0, 2.0, 0.0]);
                assert_eq!(hit.normal.get_data(), [0.0, 1.0, 0.0]);
                assert_eq!(hit.t, 3.0);
            },
            None => {
                assert!(false);
            }
        }

        // Hit XY faces
        let origin = [0.0, 0.0, 8.0];
        let direction = [0.0, 0.0, -2.0];
        let ray = Ray::from_array(origin, direction);
        let hit = cube.hit(&ray, 0.0, 100.0);

        match hit {
            Some(hit) => {
                assert_eq!(hit.point.get_data(), [0.0, 0.0, 3.0]);
                assert_eq!(hit.normal.get_data(), [0.0, 0.0, 1.0]);
                assert_eq!(hit.t, 2.5);
            },
            None => {
                assert!(false);
            }
        }

        let origin = [0.0, 0.0, -8.0];
        let direction = [0.0, 0.0, 2.0];
        let ray = Ray::from_array(origin, direction);
        let hit = cube.hit(&ray, 0.0, 100.0);

        match hit {
            Some(hit) => {
                assert_eq!(hit.point.get_data(), [0.0, 0.0, -3.0]);
                assert_eq!(hit.normal.get_data(), [0.0, 0.0, -1.0]);
                assert_eq!(hit.t, 2.5);
            },
            None => {
                assert!(false);
            }
        }

        // Hit nothing
        let origin = [-8.0, 0.0, 0.0];
        let direction = [-2.0, 0.0, 0.0];
        let ray = Ray::from_array(origin, direction);
        let hit = cube.hit(&ray, 0.0, 100.0);

        if let Some(_hit) = hit {
            assert!(false);
        }

        let origin = [-8.0, 2.001, 3.001];
        let direction = [2.0, 0.0, 0.0];
        let ray = Ray::from_array(origin, direction);
        let hit = cube.hit(&ray, 0.0, 100.0);

        if let Some(_hit) = hit {
            assert!(false);
        }
    }

    #[test]
    fn bounds() {
        let length = 2.0;
        let width = 4.0;
        let height = 6.0;
        let cube = Cube::<f64>::new(length, width, height);
        let bounds = cube.get_bounds();
        assert_eq!(bounds.get_p0().get_data(), [-1.0, -2.0, -3.0]);
        assert_eq!(bounds.get_p1().get_data(), [1.0, 2.0, 3.0]);
    }
}

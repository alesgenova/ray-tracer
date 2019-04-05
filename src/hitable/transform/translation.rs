use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::hitable::Hitable;
use crate::boundingbox::BoundingBox;

pub struct Translation<T>
    where T: Float
{
    translation: Vec3<T>,
    wrapped: Box<dyn Hitable<T>>,
    bounds: BoundingBox<T>
}

impl<T> Translation<T>
    where T: Float
{
    pub fn new(wrapped: Box<dyn Hitable<T>>, translation: Vec3<T>) -> Self {
        let mut translation = Translation {
            wrapped,
            translation,
            bounds: BoundingBox::<T>::new(Vec3::<T>::new(), Vec3::<T>::new())
        };
        translation.update_bounds();
        translation
    }

    fn update_bounds(&mut self) {
        let bounds = self.wrapped.get_bounds();

        let bounds = BoundingBox::new(
            bounds.get_p0() + &self.translation,
            bounds.get_p1() + &self.translation
        );
        self.bounds = bounds;
    }

    pub fn compute_bounds(bounds: &BoundingBox<T>, translation: &Vec3<T>) -> BoundingBox<T> {
        BoundingBox::new(
            bounds.get_p0() + translation,
            bounds.get_p1() + translation
        )
    }
}

impl<T> Hitable<T> for Translation<T>
    where T: Float
{
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<Hit<T>> {
        let origin = ray.get_origin() - &self.translation;
        let direction = Vec3::from_slice(ray.get_direction().get_data());
        let translated_ray = Ray::from_vec(origin, direction);
        if let Some(mut hit) = self.wrapped.hit(&translated_ray, t_min, t_max) {
            hit.point = hit.point + &self.translation;
            return Some(hit);
        }
        None
    }

    fn get_bounds(&self) -> &BoundingBox<T> {
        &self.bounds
    }

    fn unwrap(self) -> Box<Hitable<T>> {
        self.wrapped
    }

    fn is_primitive(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::primitive::Sphere;

    #[test]
    fn init() {
        let hitable = Box::new(Sphere::new(2.0));
        assert!(hitable.is_primitive());

        let translation = Vec3::from_array([1.0, 2.0, 3.0]);
        let hitable = Translation::new(hitable, translation);
        assert!(!hitable.is_primitive());
    }
}

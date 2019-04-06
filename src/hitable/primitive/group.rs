use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::hitable::Hitable;
use crate::boundingbox::BoundingBox;

pub struct Group<T>
    where T: Float
{
    hitables: Vec<Box<dyn Hitable<T>>>,
    bounds: BoundingBox<T>
}

impl<T> Group<T>
    where T: Float
{
    pub fn new() -> Self {
        Group {
            hitables: vec![],
            bounds: BoundingBox::<T>::new(Vec3::<T>::new(), Vec3::<T>::new())
        }
    }

    pub fn add_hitable(&mut self, hitable: Box<dyn Hitable<T>>) {
        self.bounds.expand(hitable.get_bounds());
        self.hitables.push(hitable);
    }
}

impl<T> Hitable<T> for Group<T>
    where T: Float
{
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<Hit<T>> {
        let mut t_max = t_max;
        let mut result : Option<Hit<T>> = None;

        for i in 0..self.hitables.len() {
            if let Some(hit) = self.hitables[i].hit(ray, t_min, t_max) {
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

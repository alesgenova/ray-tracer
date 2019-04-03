use std::rc::Rc;

use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::boundingbox::BoundingBox;
use crate::actor::Actor;
use crate::tree::Tree;

pub struct LinearTree<T>
    where T: Float
{
    actors: Vec<Rc<Actor<T>>>
}

impl<T> LinearTree<T>
    where T: Float
{
    pub fn new() -> Self {
        let actors = Vec::new();
        LinearTree {
            actors
        }
    }
}

impl<T> Tree<T> for LinearTree<T>
    where T: Float
{
    fn add_actor(&mut self, actor: Rc<Actor<T>>) -> bool {
        self.actors.push(actor);
        return true;
    }

    fn get_hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<(Rc<Actor<T>>, Hit<T>)> {
        let mut t_max = t_max;
        let mut result : Option<(Rc<Actor<T>>, Hit<T>)> = None;

        for i in 0..self.actors.len() {
            if let Some(hit) = self.actors[i].hitable.hit(ray, t_min, t_max) {
                t_max = hit.t;
                result = Some((Rc::clone(&self.actors[i]), hit));
            }
        }

        result
    }
}

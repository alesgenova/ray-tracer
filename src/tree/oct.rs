use std::rc::Rc;

use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::boundingbox::BoundingBox;
use crate::actor::Actor;
use crate::tree::Tree;

pub struct Octree<T>
    where T: Float
{
    bounds: BoundingBox<T>, // The bounds of this node
    pub children: [Option<Box<Octree<T>>>; 8], // The children octrees
    pub actors: Vec<Rc<Actor<T>>> // The actors that are too large to be placed in the children nodes
}

impl<T> Octree<T>
    where T: Float
{
    pub fn new(bounds: BoundingBox<T>) -> Self {
        let children: [Option<Box<Octree<T>>>; 8] = [None, None, None, None, None, None, None, None];
        let actors = Vec::new();
        Octree {
            bounds,
            children,
            actors
        }
    }

    pub fn get_bounds(&self) -> &BoundingBox<T> {
        &self.bounds
    }

    fn get_child_bounds(&self, child_index: usize) -> BoundingBox<T> {
        let k = child_index % 2;
        let j = (child_index / 2) % 2;
        let i = (child_index / 4) % 2;

        let k = T::from(k).unwrap();
        let j = T::from(j).unwrap();
        let i = T::from(i).unwrap();

        let half = T::from(0.5).unwrap();

        let (min_x, max_x) = self.get_bounds().get_axis_bounds(0);
        let (min_y, max_y) = self.get_bounds().get_axis_bounds(1);
        let (min_z, max_z) = self.get_bounds().get_axis_bounds(2);
        let len = [half * (max_x - min_x), half * (max_y - min_y), half * (max_z - min_z)];

        let offset = Vec3::<T>::from_array([len[0] * i, len[1] * j, len[2] * k]);

        let p0 = Vec3::<T>::from_array([min_x, min_y, min_z]) + &offset;
        let p1 = &p0 + Vec3::from_slice(&len);
        BoundingBox::<T>::new(p0, p1)
    }
}

impl<T> Tree<T> for Octree<T>
    where T: Float
{
    fn add_actor(&mut self, actor: Rc<Actor<T>>) -> bool {
        let actor_bounds = actor.hitable.get_bounds();

        // If this node can't fully contain the actor, do nothing
        if !self.bounds.contains(actor_bounds) {
            return false;
        }

        // If one of the current/potential children of this node can fully
        // contain the actor, add it to that node
        for i in 0..8 {
            let child = &mut self.children[i];
            match child {
                Some(node) => {
                    if node.get_bounds().contains(actor_bounds) {
                        node.add_actor(actor);
                        return true;
                    }
                },
                None => {
                    let child_bounds = self.get_child_bounds(i);
                    if child_bounds.contains(actor_bounds) {
                        let mut node = Octree::<T>::new(child_bounds);
                        node.add_actor(actor);
                        self.children[i] = Some(Box::new(node));
                        return true;
                    }
                }
            }
        }

        // If the actor is too large for any of the children, add it to this node.
        self.actors.push(actor);
        return true;
    }

    fn get_hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<(Rc<Actor<T>>, Hit<T>)> {
        if !self.get_bounds().hit(ray, t_min, t_max) {
            return None;
        }

        let mut t_max = t_max;
        let mut result : Option<(Rc<Actor<T>>, Hit<T>)> = None;

        for i in 0..self.actors.len() {
            if let Some(hit) = self.actors[i].hitable.hit(ray, t_min, t_max) {
                t_max = hit.t;
                result = Some((Rc::clone(&self.actors[i]), hit));
            }
        }

        for i in 0..8 {
            if let Some(child) = &self.children[i] {
                if let Some((actor, hit)) = child.get_hit(ray, t_min, t_max) {
                    t_max = hit.t;
                    result = Some((actor, hit));
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn child_bounds() {
        let p0 = Vec3::<f64>::from_array([0.,0.,0.]);
        let p1 = Vec3::<f64>::from_array([4.,6.,8.]);
        let bounds = BoundingBox::new(p0, p1);
        let node = Octree::new(bounds);

        let mut children_bounds = Vec::<BoundingBox<f64>>::new();

        // Ensure children are contained in the parent node
        for i in 0..8 {
            let child_bounds = node.get_child_bounds(i);
            assert!(node.get_bounds().contains(&child_bounds));
            children_bounds.push(child_bounds);
        }

        // Ensure there is no overlap across children
        for i in 0..7 {
            for j in i + 1..8 {
                assert!(!children_bounds[i].overlaps(&children_bounds[j]));
            }
        }

        // Ensure children and parent span the same volume
        let mut children_volume = 0.0;
        for i in 0..8 {
            children_volume += children_bounds[i].get_volume();
        }
        assert_eq!(node.get_bounds().get_volume(), children_volume);
    }
}

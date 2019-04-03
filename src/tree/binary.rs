use std::rc::Rc;

use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::boundingbox::BoundingBox;
use crate::actor::Actor;
use crate::tree::Tree;

pub struct BinaryTree<T>
    where T: Float
{
    bounds: BoundingBox<T>, // The bounds of this node
    pub children: [Option<Box<BinaryTree<T>>>; 2], // The children binary trees
    pub actor: Option<Rc<Actor<T>>> // The actor stored in the leaf nodes
}

impl<T> BinaryTree<T>
    where T: Float
{
    pub fn new() -> Self {
        let bounds = BoundingBox::<T>::new(Vec3::<T>::new(), Vec3::<T>::new());
        let children: [Option<Box<BinaryTree<T>>>; 2] = [None, None];
        let actor = None;
        BinaryTree {
            bounds,
            children,
            actor
        }
    }

    pub fn get_bounds(&self) -> &BoundingBox<T> {
        &self.bounds
    }
}

impl<T> Tree<T> for BinaryTree<T>
    where T: Float
{
    fn add_actor(&mut self, actor: Rc<Actor<T>>) -> bool {
        let actor_bounds = actor.hitable.get_bounds();

        // Expand the node bounds so the new actor is guaranteed to fit
        self.bounds.expand(&actor_bounds);

        let (left_child, rest) = self.children.split_first_mut().unwrap();
        let (right_child, _) = rest.split_first_mut().unwrap();

        match (&self.actor, left_child, right_child) {
            // If this is a newly created node (i.e. no children, no actor)
            // add the actor here and quit
            (None, None, None) => {
                self.actor = Some(actor);
                return true;
            },
            // If this node is a leaf node (i.e. has and actor, but no children),
            // send the current actor to one child, and send the incoming actor to the other
            (Some(current_actor), None, None) => {
                let mut left = Box::new(BinaryTree::<T>::new());
                let mut right = Box::new(BinaryTree::<T>::new());
                left.add_actor(Rc::clone(current_actor));
                right.add_actor(actor);
                self.actor = None;
                self.children[0] = Some(left);
                self.children[1] = Some(right);
                return true;
            },
            // If both children are already initialized, add the actor to the child
            // whose bounding box wouldn't expand.
            // If they both would expand, add it to the child
            // whose resulting bounding box would be smaller.
            (None, Some(left), Some(right)) => {
                // if left.get_bounds().contains(actor.hitable.get_bounds()) && right.get_bounds().contains(actor.hitable.get_bounds()){
                //     if left.get_bounds().get_volume() < right.get_bounds().get_volume() {
                //         left.add_actor(actor);
                //     } else {
                //         right.add_actor(actor);
                //     }
                //     return true;
                // } else if left.get_bounds().contains(actor.hitable.get_bounds()) {
                //     left.add_actor(actor);
                //     return true;
                // } else if right.get_bounds().contains(actor.hitable.get_bounds()) {
                //     right.add_actor(actor);
                //     return true;
                // }

                let mut left_bounds = BoundingBox::new(
                    Vec3::from_slice(&left.get_bounds().get_p0().get_data()),
                    Vec3::from_slice(&left.get_bounds().get_p1().get_data())
                );
                left_bounds.expand(&actor_bounds);

                let mut right_bounds = BoundingBox::new(
                    Vec3::from_slice(&right.get_bounds().get_p0().get_data()),
                    Vec3::from_slice(&right.get_bounds().get_p1().get_data())
                );
                right_bounds.expand(&actor_bounds);

                if left_bounds.get_volume() < right_bounds.get_volume() {
                    left.add_actor(actor);
                } else {
                    right.add_actor(actor);
                }
                return true;
            },
            // Other cases should never happen
            _ => {
                panic!("Something went wrong while adding the actor to the binary tree.");
            }
        }
    }

    fn get_hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<(Rc<Actor<T>>, Hit<T>)> {
        if !self.get_bounds().hit(ray, t_min, t_max) {
            return None;
        }

        let mut t_max = t_max;
        let mut result : Option<(Rc<Actor<T>>, Hit<T>)> = None;

        if let Some(actor) = &self.actor {
            if let Some(hit) = actor.hitable.hit(ray, t_min, t_max) {
                t_max = hit.t;
                result = Some((Rc::clone(actor), hit));
            }
        }

        for i in 0..2 {
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

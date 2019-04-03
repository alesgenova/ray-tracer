use std::rc::Rc;

use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::actor::Actor;
use crate::boundingbox::BoundingBox;
use crate::tree::{Tree, TreeType};
use crate::tree::linear::LinearTree;
use crate::tree::binary::BinaryTree;
use crate::tree::oct::Octree;


pub struct Scene<T>
    where T: Float
{
    actors: Vec<Rc<Actor<T>>>,
    background: Vec3<T>,
    bounds: BoundingBox<T>,
    tree: Box<dyn Tree<T>>,
    tree_type: TreeType
}

impl<T> Scene<T>
    where T: Float
{
    pub fn new() -> Self {
        Scene {
            actors: vec!(),
            background: Vec3::<T>::new(),
            bounds: BoundingBox::<T>::new(Vec3::<T>::new(), Vec3::<T>::new()),
            tree: Box::new(LinearTree::new()),
            tree_type: TreeType::Linear
        }
    }

    pub fn set_background(&mut self, background: Vec3<T>) {
        self.background = background;
    }

    pub fn add_actor(&mut self, actor: Actor<T>) {
        let _expanded = self.bounds.expand(&actor.hitable.get_bounds());
        let actor = Rc::new(actor);
        self.actors.push(Rc::clone(&actor));
        let success = self.tree.add_actor(actor);

        if !success {
            self.rebuild_tree();
        }
    }

    pub fn get_color(&self, ray: &Ray<T>, reflection: usize, max_reflection: usize) -> Vec3<T> {
        let current_hit = self.tree.get_hit(ray, T::from(0.000000001).unwrap(), T::from(10000000000.0).unwrap());

        match current_hit {
            Some((actor, hit)) => {
                // let actor = &self.actors[actor_idx];
                let scatter = actor.material.scatter(ray, &hit);
                let attenuation = Vec3::<T>::from_slice(scatter.attenuation.get_data());
                let scattered_ray = scatter.scattered;
                match scattered_ray {
                    Some(ray_out) => {
                        if (reflection < max_reflection) {
                            return attenuation * self.get_color(&ray_out, reflection + 1, max_reflection);
                        } else {
                            return attenuation;
                        }
                    },
                    None => {
                        return attenuation;
                    }
                }
            },
            None => {
                return Vec3::<T>::from_slice(self.background.get_data());
            }
        }
    }

    pub fn set_tree_type(&mut self, tree_type: TreeType) {
        self.tree_type = tree_type;
        self.rebuild_tree();
    }

    fn rebuild_tree(&mut self) {
        let mut tree: Box<dyn Tree<T>> = match self.tree_type {
            TreeType::Linear => {
                Box::new(LinearTree::new())
            },
            TreeType::Binary => {
                Box::new(BinaryTree::new())
            }
            TreeType::Oct => {
                let mut tree_bounds = BoundingBox::<T>::new(
                    Vec3::<T>::from_slice(self.bounds.get_p0().get_data()),
                    Vec3::<T>::from_slice(self.bounds.get_p1().get_data())
                );
                tree_bounds.make_cube();
                let length = tree_bounds.get_axis_length(0);
                let pad = length * T::from(0.1).unwrap();
                for i in 0..3 {
                    tree_bounds.pad_axis(pad, i);
                }
                Box::new(Octree::<T>::new(tree_bounds))
            }
        };

        for i in 0..self.actors.len() {
            let actor = Rc::clone(&self.actors[i]);
            tree.add_actor(actor);
        }

        self.tree = tree;
    }
}

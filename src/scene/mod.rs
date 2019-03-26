use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hitable::{Hitable, Hit};

pub struct Scene<T>
    where T: Float
{
    actors: Vec<Box<Hitable<T>>>
}

impl<T> Scene<T>
    where T: Float
{
    pub fn new() -> Self {
        Scene {
            actors: vec!()
        }
    }

    pub fn add_actor(&mut self, actor: Box<Hitable<T>>) {
        self.actors.push(actor);
    }

    pub fn get_color(&self, ray: &Ray<T>) -> Vec3<T> {
        let mut current_hit: Option<Hit<T>> = None;

        for i in 0..self.actors.len() {
            let actor = &self.actors[i];
            let last_hit = actor.hit(ray, T::zero(), T::from(10000).unwrap());
            current_hit = match current_hit {
                Some(c_hit) => {
                    match last_hit {
                        Some(l_hit) => {
                            if c_hit.t < l_hit.t {
                                Some(c_hit)
                            } else {
                                Some(l_hit)
                            }
                        },
                        None => {
                            Some(c_hit)
                        }
                    }
                },
                None => {
                    last_hit
                }
            }
        }

        match current_hit {
            Some(_) => {return Vec3::<T>::from_array([T::one(); 3]);},
            None => {return Vec3::<T>::from_array([T::zero(); 3]);}
        }
    }
}

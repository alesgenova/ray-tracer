use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::actor::Actor;

pub struct Scene<T>
    where T: Float
{
    actors: Vec<Actor<T>>,
    background: Vec3<T>
}

impl<T> Scene<T>
    where T: Float
{
    pub fn new() -> Self {
        Scene {
            actors: vec!(),
            background: Vec3::<T>::new()
        }
    }

    pub fn set_background(&mut self, background: Vec3<T>) {
        self.background = background;
    }

    pub fn add_actor(&mut self, actor: Actor<T>) {
        self.actors.push(actor);
    }

    pub fn get_hit(&self, ray: &Ray<T>) -> Option<(usize, Hit<T>)> {
        let mut current_hit: Option<Hit<T>> = None;
        let mut current_actor: usize = 0;

        for i in 0..self.actors.len() {
            let hitable = &self.actors[i].hitable;
            let last_hit = hitable.hit(ray, T::zero(), T::from(10000).unwrap());
            current_hit = match (current_hit, last_hit) {
                (Some(c_hit), Some(l_hit)) => {
                    if c_hit.t < l_hit.t {
                        Some(c_hit)
                    } else {
                        current_actor = i;
                        Some(l_hit)
                    }
                },
                (Some(c_hit), None) => {
                    Some(c_hit)
                },
                (None, Some(l_hit)) => {
                    current_actor = i;
                    Some(l_hit)
                },
                (None, None) => {
                    None
                }
            }
        }

        match current_hit {
            Some(hit) => { return Some((current_actor, hit)); },
            None => { return None; }
        }
    }

    pub fn get_color(&self, ray: &Ray<T>) -> Vec3<T> {
        let current_hit = self.get_hit(ray);

        match current_hit {
            Some((actor_idx, hit)) => {
                let actor = &self.actors[actor_idx];
                let scatter = actor.material.scatter(ray, &hit);
                return Vec3::<T>::from_slice(scatter.attenuation.get_data());
            },
            None => {
                return Vec3::<T>::from_slice(self.background.get_data());
            }
        }
    }
}

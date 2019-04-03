use std::rc::Rc;

use crate::float::Float;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::actor::Actor;

pub mod linear;
pub mod oct;
pub mod binary;

pub trait Tree<T>
    where T: Float
{
    fn add_actor(&mut self, actor: Rc<Actor<T>>) -> bool;

    fn get_hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<(Rc<Actor<T>>, Hit<T>)>;
}

pub enum TreeType {
    Linear,
    Binary,
    Oct
}

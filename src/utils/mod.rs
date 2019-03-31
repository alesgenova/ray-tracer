use rand::prelude::*;

use crate::float::Float;
use crate::vector::Vec3;

pub fn random_point_in_sphere<T>(radius: T) -> Vec3<T>
    where T: Float
{
    let mut point = Vec3::<T>::new();
    let mut rng = rand::thread_rng();
    let two = T::from(2.0).unwrap();

    loop {
        let x = T::from(rng.gen::<f64>()).unwrap() * two - T::one();
        let y = T::from(rng.gen::<f64>()).unwrap() * two - T::one();
        let z = T::from(rng.gen::<f64>()).unwrap() * two - T::one();

        let len = (x * x + y * y + z * z).sqrt();

        if len < T::one() {
            point.set_data(&[x * radius, y * radius, z * radius]);
            break;
        }
    }

    point
}

pub fn reflect<T>(direction: &Vec3<T>, normal: &Vec3<T>) -> Vec3<T>
    where T: Float
{
    let two = T::from(2.0).unwrap();
    let c = direction.dot(normal);
    let reflection = direction - normal * two * c;
    reflection
}

pub fn refract<T>(direction: &Vec3<T>, normal: &Vec3<T>, n0: T, n1: T) -> Vec3<T>
    where T: Float
{
    let ratio = n0 / n1;
    let c = direction.dot(normal);
    let discriminant = T::one() - ratio * ratio * (T::one() - c * c);
    if discriminant > T::zero() {
        let prob = reflection_probability(direction, normal, n0);
        let mut rng = rand::thread_rng();
        if T::from(rng.gen::<f64>()).unwrap() < prob  {
            return reflect(direction, normal);
        }
        return (direction - normal * c) * ratio - normal * discriminant.sqrt();
    } else {
        return reflect(direction, normal);
    }
}

pub fn reflection_probability<T>(direction: &Vec3<T>, normal: &Vec3<T>, n: T) -> T
    where T: Float
{
    let cosine = - n * direction.dot(normal);
    let mut r0 = (T::one() - n) / (T::one() + n);
    r0 = r0 * r0;
    let mut pow5 = T::one() - cosine;
    pow5 = pow5 * pow5 * pow5 * pow5 * pow5;
    r0 + (T::one() - r0) * pow5
}

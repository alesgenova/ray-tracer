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

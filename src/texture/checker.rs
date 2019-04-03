use crate::float::Float;
use crate::vector::Vec3;
use super::Texture;

pub struct CheckerTexture<T>
    where T: Float
{
    texture0: Box<Texture<T>>,
    texture1: Box<Texture<T>>,
    period: Vec3<T>

}

impl<T> CheckerTexture<T>
    where T: Float
{
    pub fn new(texture0:  Box<Texture<T>>, texture1:  Box<Texture<T>>) -> Self {
        let period = Vec3::<T>::from_array([T::one(), T::one(), T::one()]);
        CheckerTexture {
            texture0,
            texture1,
            period
        }
    }

    pub fn set_period(&mut self, period: Vec3<T>) {
        self.period = period;
    }
}

impl<T> Texture<T> for CheckerTexture<T>
    where T: Float
{
    fn get_color(&self, u: T, v: T, point: &Vec3<T>) -> Vec3<T> {
        let two = T::from(2.0).unwrap();
        let x = T::to_i32(&((point.get_data()[0] / self.period.get_data()[0]).floor())).unwrap().abs() % 2;
        let y = T::to_i32(&((point.get_data()[1] / self.period.get_data()[1]).floor())).unwrap().abs() % 2;
        let z = T::to_i32(&((point.get_data()[2] / self.period.get_data()[2]).floor())).unwrap().abs() % 2;
        let sign = (x * 2 - 1) * (y * 2 - 1) * (z * 2 - 1);
        if sign > 0 {
            return self.texture0.get_color(u, v, point);
        } else {
            return self.texture1.get_color(u, v, point);
        }
        // let y = T::to_i32(&point.get_data()[1]).unwrap();
        // let z = T::to_i32(&point.get_data()[2]).unwrap();
        
    }
}

use crate::float::Float;
use std::ops;

#[derive(Debug)]
pub struct Vec3<T: Float>
    where T: Float
{
    data: [T; 3]
}

impl<T> Vec3<T>
    where T: Float
{
    pub fn new() -> Self {
        let data = [T::zero(); 3];
        Vec3{data}
    }

    pub fn from_array(input: [T; 3]) -> Self {
        Vec3{data: input}
    }

    pub fn from_slice(input: &[T]) -> Self {
        assert!(input.len() >= 3);
        let mut data = [T::zero(); 3];
        for i in 0..3 {
        data[i] = input[i];
        }
        Vec3{data}
    }

    pub fn get_data(&self) -> &[T] {
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn set_data(&mut self, data: &[T]) {
        assert!(data.len() >= 3);
        for i in 0..3 {
            self.data[i] = data[i];
        }
    }

    fn add(this: &Vec3<T>, other: &Vec3<T>, result: &mut Vec3<T>) {
        let data = result.get_data_mut();
        for i in 0..3 {
            data[i] = this.data[i] + other.data[i];
        }
    }

    fn sub(this: &Vec3<T>, other: &Vec3<T>, result: &mut Vec3<T>) {
        let data = result.get_data_mut();
        for i in 0..3 {
            data[i] = this.data[i] - other.data[i];
        }
    }

    fn mul_vec(this: &Vec3<T>, other: &Vec3<T>, result: &mut Vec3<T>) {
        let data = result.get_data_mut();
        for i in 0..3 {
            data[i] = this.data[i] * other.data[i];
        }
    }

    fn mul(this: &Vec3<T>, other: T, result: &mut Vec3<T>) {
        let data = result.get_data_mut();
        for i in 0..3 {
            data[i] = this.data[i] * other;
        }
    }

    fn div(this: &Vec3<T>, other: T, result: &mut Vec3<T>) {
        let data = result.get_data_mut();
        for i in 0..3 {
            data[i] = this.data[i] / other;
        }
    }

    pub fn dot(&self, other: &Vec3<T>) -> T {
        let mut result = T::zero();
        for i in 0..3 {
            result = result + self.data[i] * other.data[i];
        }
        result
    }

    pub fn cross(&self, other: &Vec3<T>) -> Vec3<T> {
        let mut result = [T::zero(); 3];
        for i in 0..3 {
            let j = (i + 1) % 3;
            let k = (i + 2) % 3;
            result[i] = self.get_data()[j] * other.get_data()[k] - self.get_data()[k] * other.get_data()[j];
        }
        Vec3::from_array(result)
    }

    pub fn norm(&self) -> T {
        let n2 = self.dot(self);
        n2.sqrt()
    }

    pub fn normalize(&mut self) {
        let n = self.norm();
        for i in 0..3 {
            self.data[i] = self.data[i] / n;
        }
    }
}

// Vec3 + Vec3
impl<T> ops::Add<Vec3<T>> for Vec3<T>
    where T: Float
{
    type Output = Vec3<T>;

    fn add(self, other: Vec3<T>) -> Vec3<T> {
        let mut result = Vec3::<T>::new();
        Vec3::<T>::add(&self, &other, &mut result);
        result
    }
}

// Vec3 + &Vec3
impl<T> ops::Add<&Vec3<T>> for Vec3<T>
    where T: Float
{
    type Output = Vec3<T>;

    fn add(self, other: &Vec3<T>) -> Vec3<T> {
        let mut result = Vec3::<T>::new();
        Vec3::<T>::add(&self, other, &mut result);
        result
    }
}

// &Vec3 + Vec3
impl<T> ops::Add<Vec3<T>> for &Vec3<T>
    where T: Float
{
    type Output = Vec3<T>;

    fn add(self, other: Vec3<T>) -> Vec3<T> {
        let mut result = Vec3::<T>::new();
        Vec3::<T>::add(self, &other, &mut result);
        result
    }
}

// &Vec3 + &Vec3
impl<T> ops::Add<&Vec3<T>> for &Vec3<T>
    where T: Float
{
    type Output = Vec3<T>;

    fn add(self, other: &Vec3<T>) -> Vec3<T> {
        let mut result = Vec3::<T>::new();
        Vec3::<T>::add(self, other, &mut result);
        result
    }
}

// Vec3 - Vec3
impl<T> ops::Sub<Vec3<T>> for Vec3<T>
    where T: Float
{
    type Output = Vec3<T>;

    fn sub(self, other: Vec3<T>) -> Vec3<T> {
        let mut result = Vec3::<T>::new();
        Vec3::<T>::sub(&self, &other, &mut result);
        result
    }
}

// Vec3 - &Vec3
impl<T> ops::Sub<&Vec3<T>> for Vec3<T>
    where T: Float
{
    type Output = Vec3<T>;

    fn sub(self, other: &Vec3<T>) -> Vec3<T> {
        let mut result = Vec3::<T>::new();
        Vec3::<T>::sub(&self, other, &mut result);
        result
    }
}

// &Vec3 - Vec3
impl<T> ops::Sub<Vec3<T>> for &Vec3<T>
    where T: Float
{
    type Output = Vec3<T>;

    fn sub(self, other: Vec3<T>) -> Vec3<T> {
        let mut result = Vec3::<T>::new();
        Vec3::<T>::sub(self, &other, &mut result);
        result
    }
}

// &Vec3 - &Vec3
impl<T> ops::Sub<&Vec3<T>> for &Vec3<T>
    where T: Float
{
    type Output = Vec3<T>;

    fn sub(self, other: &Vec3<T>) -> Vec3<T> {
        let mut result = Vec3::<T>::new();
        Vec3::<T>::sub(self, other, &mut result);
        result
    }
}

// Vec3 * Vec3 (multiply each item)
impl<T> ops::Mul<Vec3<T>> for Vec3<T>
    where T: Float
{
    type Output = Vec3<T>;

    fn mul(self, other: Vec3<T>) -> Vec3<T> {
        let mut result = Vec3::<T>::new();
        Vec3::<T>::mul_vec(&self, &other, &mut result);
        result
    }
}

// Vec3 * &Vec3
impl<T> ops::Mul<&Vec3<T>> for Vec3<T>
    where T: Float
{
    type Output = Vec3<T>;

    fn mul(self, other: &Vec3<T>) -> Vec3<T> {
        let mut result = Vec3::<T>::new();
        Vec3::<T>::mul_vec(&self, other, &mut result);
        result
    }
}

// &Vec3 * Vec3
impl<T> ops::Mul<Vec3<T>> for &Vec3<T>
    where T: Float
{
    type Output = Vec3<T>;

    fn mul(self, other: Vec3<T>) -> Vec3<T> {
        let mut result = Vec3::<T>::new();
        Vec3::<T>::mul_vec(self, &other, &mut result);
        result
    }
}

// &Vec3 * &Vec3
impl<T> ops::Mul<&Vec3<T>> for &Vec3<T>
    where T: Float
{
    type Output = Vec3<T>;

    fn mul(self, other: &Vec3<T>) -> Vec3<T> {
        let mut result = Vec3::<T>::new();
        Vec3::<T>::mul_vec(self, other, &mut result);
        result
    }
}

// Vec3 * T
impl<T> ops::Mul<T> for Vec3<T>
    where T: Float
{
    type Output = Vec3<T>;

    fn mul(self, other: T) -> Vec3<T> {
        let mut result = Vec3::<T>::new();
        Vec3::<T>::mul(&self, other, &mut result);
        result
    }
}

// T * Vec3
impl ops::Mul<Vec3<f64>> for f64
{
    type Output = Vec3<f64>;

    fn mul(self, other: Vec3<f64>) -> Vec3<f64> {
        let mut result = Vec3::new();
        Vec3::mul(&other, self, &mut result);
        result
    }
}

// T * Vec3
impl ops::Mul<Vec3<f32>> for f32
{
    type Output = Vec3<f32>;

    fn mul(self, other: Vec3<f32>) -> Vec3<f32> {
        let mut result = Vec3::new();
        Vec3::mul(&other, self, &mut result);
        result
    }
}

// &Vec3 * T
impl<T> ops::Mul<T> for &Vec3<T>
    where T: Float
{
    type Output = Vec3<T>;

    fn mul(self, other: T) -> Vec3<T> {
        let mut result = Vec3::<T>::new();
        Vec3::<T>::mul(self, other, &mut result);
        result
    }
}

// T * &Vec3
impl ops::Mul<&Vec3<f64>> for f64
{
    type Output = Vec3<f64>;

    fn mul(self, other: &Vec3<f64>) -> Vec3<f64> {
        let mut result = Vec3::new();
        Vec3::mul(other, self, &mut result);
        result
    }
}

// T * &Vec3
impl ops::Mul<&Vec3<f32>> for f32
{
    type Output = Vec3<f32>;

    fn mul(self, other: &Vec3<f32>) -> Vec3<f32> {
        let mut result = Vec3::new();
        Vec3::mul(other, self, &mut result);
        result
    }
}

// Vec3 / T
impl<T> ops::Div<T> for Vec3<T>
    where T: Float
{
    type Output = Vec3<T>;

    fn div(self, other: T) -> Vec3<T> {
        let mut result = Vec3::<T>::new();
        Vec3::<T>::div(&self, other, &mut result);
        result
    }
}

// &Vec3 / T
impl<T> ops::Div<T> for &Vec3<T>
    where T: Float
{
    type Output = Vec3<T>;

    fn div(self, other: T) -> Vec3<T> {
        let mut result = Vec3::<T>::new();
        Vec3::<T>::div(self, other, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let v = Vec3::<f64>::new();
        assert_eq!(v.get_data(), [0.0, 0.0, 0.0]);

        let data = vec!(1.0, 2.0, 3.0);
        let v = Vec3::<f64>::from_slice(&data);
        assert_eq!(v.get_data(), [1.0, 2.0, 3.0]);

        let data = [4.0, 5.0, 6.0];
        let v = Vec3::<f64>::from_array(data);
        assert_eq!(v.get_data(), [4.0, 5.0, 6.0]);
    }

    #[test]
    fn add() {
        let data = vec!(1.0, 2.0, 3.0);
        let v1 = Vec3::<f64>::from_slice(&data);
        let data = vec!(2.0, 4.0, 6.0);
        let v2 = Vec3::<f64>::from_slice(&data);

        let v3 = &v1 + &v2;
        assert_eq!(v3.get_data(), [3.0, 6.0, 9.0]);

        let mut v4 = &v1 + &v2 + &v3;
        assert_eq!(v4.get_data(), [6.0, 12.0, 18.0]);

        v4 = &v1 + (&v2 + &v3);
        assert_eq!(v4.get_data(), [6.0, 12.0, 18.0]);

        v4 = (&v1 + &v2) + &v3;
        assert_eq!(v4.get_data(), [6.0, 12.0, 18.0]);

        v4 = &v1 + (&v2 + &v3) + &v1;
        assert_eq!(v4.get_data(), [7.0, 14.0, 21.0]);

        v4 = (&v1 + &v2) + (&v3 + &v1);
        assert_eq!(v4.get_data(), [7.0, 14.0, 21.0]);
    }

    #[test]
    fn sub() {
        let data = vec!(1.0, 2.0, 3.0);
        let v1 = Vec3::<f64>::from_slice(&data);
        let data = vec!(2.0, 4.0, 6.0);
        let v2 = Vec3::<f64>::from_slice(&data);

        let v3 = &v1 - &v2;
        assert_eq!(v3.get_data(), [-1.0, -2.0, -3.0]);

        let mut v4 = &v1 - &v2 - &v3;
        assert_eq!(v4.get_data(), [0.0, 0.0, 0.0]);

        v4 = &v1 - (&v2 - &v3);
        assert_eq!(v4.get_data(), [-2.0, -4.0, -6.0]);

        v4 = (&v1 - &v2) - &v3;
        assert_eq!(v4.get_data(), [0.0, 0.0, 0.0]);

        v4 = &v1 - (&v2 - &v3) - &v1;
        assert_eq!(v4.get_data(), [-3.0, -6.0, -9.0]);

        v4 = (&v1 - &v2) - (&v3 - &v1);
        assert_eq!(v4.get_data(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn mul() {
        let data = vec!(1.0, 2.0, 3.0);
        let v1 = Vec3::<f64>::from_slice(&data);
        let data = vec!(2.0, 4.0, 6.0);
        let v2 = Vec3::<f32>::from_slice(&data);

        let mut v3 = 3.0 * &v1;
        assert_eq!(v3.get_data(), [3.0, 6.0, 9.0]);

        v3 = &v1 * 3.0;
        assert_eq!(v3.get_data(), [3.0, 6.0, 9.0]);

        v3 = (&v1 * 2.0) * 3.0;
        assert_eq!(v3.get_data(), [6.0, 12.0, 18.0]);

        v3 = 3.0 * (&v1 * 2.0);
        assert_eq!(v3.get_data(), [6.0, 12.0, 18.0]);

        let mut v4 = &v2 * 4.0;
        assert_eq!(v4.get_data(), [8.0, 16.0, 24.0]);

        v4 = 4.0 * &v2;
        assert_eq!(v4.get_data(), [8.0, 16.0, 24.0]);

        v4 = (&v2 * 2.0) * 4.0;
        assert_eq!(v4.get_data(), [16.0, 32.0, 48.0]);

        v4 = 4.0 * (&v2 * 2.0);
        assert_eq!(v4.get_data(), [16.0, 32.0, 48.0]);
    }

    #[test]
    fn div() {
        let data = vec!(1.0, 2.0, 3.0);
        let v1 = Vec3::<f64>::from_slice(&data);
        let data = vec!(2.0, 4.0, 6.0);
        let v2 = Vec3::<f64>::from_slice(&data);

        let mut v3 = &v1 / 2.0;
        assert_eq!(v3.get_data(), [0.5, 1.0, 1.5]);

        v3 = &v2 / 4.0;
        assert_eq!(v3.get_data(), [0.5, 1.0, 1.5]);

        v3 = (&v2 / 2.0) / 2.0;
        assert_eq!(v3.get_data(), [0.5, 1.0, 1.5]);
    }

    #[test]
    fn ops() {
        let data = vec!(1.0, 2.0, 3.0);
        let v1 = Vec3::<f64>::from_slice(&data);
        let data = vec!(2.0, 4.0, 6.0);
        let v2 = Vec3::<f64>::from_slice(&data);

        let mut v3 = 4.0 * (&v1 * 2.0 + &v2 - &v1) / 2.0;
        assert_eq!(v3.get_data(), [6.0, 12.0, 18.0]);

        v3 = 4.0 * (&v1 * 2.0 + &v2 - &v1) * 2.0 / 2.0;
        assert_eq!(v3.get_data(), [12.0, 24.0, 36.0]);
    }

    #[test]
    fn dot() {
        let data = vec!(1.0, 2.0, 3.0);
        let v1 = Vec3::<f64>::from_slice(&data);
        let data = vec!(2.0, 4.0, 6.0);
        let v2 = Vec3::<f64>::from_slice(&data);

        let mut d = v1.dot(&v1);
        assert_eq!(d, 14.0);

        d = v2.dot(&v2);
        assert_eq!(d, 56.0);

        d = v1.dot(&v2);
        assert_eq!(d, 28.0);

        d = v2.dot(&v1);
        assert_eq!(d, 28.0);

        d = (&v1 * 0.5).dot(&v2);
        assert_eq!(d, 14.0);

        d = v1.dot(&(&v2 / 2.0));
        assert_eq!(d, 14.0);
    }

    #[test]
    fn cross() {
        let data = vec!(1.0, 0.0, 0.0);
        let v1 = Vec3::<f64>::from_slice(&data);
        let data = vec!(0.0, 1.0, 0.0);
        let v2 = Vec3::<f64>::from_slice(&data);
        let v3 = v1.cross(&v2);
        assert_eq!(v3.get_data(), [0.0, 0.0, 1.0]);

        let data = vec!(7.0, 3.0, -4.0);
        let v1 = Vec3::<f64>::from_slice(&data);
        let data = vec!(1.0, 0.0, 6.0);
        let v2 = Vec3::<f64>::from_slice(&data);
        let v3 = v1.cross(&v2);
        assert_eq!(v3.get_data(), [18.0, -46.0, -3.0]);
    }

    #[test]
    fn norm() {
        let data = vec!(1.0, 2.0, 2.0);
        let mut v1 = Vec3::<f64>::from_slice(&data);
        assert_eq!(v1.norm(), 3.0);
        v1.normalize();
        assert_eq!(v1.norm(), 1.0);
    }

    #[test]
    fn set() {
        let mut v = Vec3::<f64>::new();
        assert_eq!(v.get_data(), [0.0, 0.0, 0.0]);
        let data = [1.0, 2.0, 3.0];
        v.set_data(&data);
        assert_eq!(v.get_data(), [1.0, 2.0, 3.0]);
    }
}

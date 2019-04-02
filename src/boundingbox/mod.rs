use crate::float::Float;
use crate::vector::Vec3;
use crate::ray::Ray;

pub struct BoundingBox<T>
    where T : Float
{
    p0: Vec3<T>,
    p1: Vec3<T>
}

impl<T> BoundingBox<T>
    where T : Float
{
    pub fn new(p0: Vec3<T>, p1: Vec3<T>) -> Self {
        let mut p0 = p0;
        let mut p1 = p1;
        // Ensure the min x,y,z is always in p0, and the max is always in p1
        let (min_x, max_x) = BoundingBox::<T>::calculate_axis_bounds(&p0, &p1, 0);
        let (min_y, max_y) = BoundingBox::<T>::calculate_axis_bounds(&p0, &p1, 1);
        let (min_z, max_z) = BoundingBox::<T>::calculate_axis_bounds(&p0, &p1, 2);

        p0.set_data(&[min_x, min_y, min_z]);
        p1.set_data(&[max_x, max_y, max_z]);
        BoundingBox {p0, p1}
    }

    pub fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;
        for i in 0..3 {
            let inv_d = T::one() / ray.get_direction().get_data()[i];
            let mut t0 = (self.p0.get_data()[i] - ray.get_origin().get_data()[i]) * inv_d;
            let mut t1 = (self.p1.get_data()[i] - ray.get_origin().get_data()[i]) * inv_d;
            if inv_d < T::zero() {
                let tmp = t0;
                t0 = t1;
                t1 = tmp;
            }
            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn contains(&self, other: &BoundingBox<T>) -> bool {
        for i in 0..3 {
            let (min_self, max_self) = self.get_axis_bounds(i);
            let (min_other, max_other) = other.get_axis_bounds(i);
            if min_other < min_self || max_other > max_self {
                return false;
            }
        }
        true
    }

    pub fn overlaps(&self, other: &BoundingBox<T>) -> bool {
        for i in 0..3 {
            let (min_self, max_self) = self.get_axis_bounds(i);
            let (min_other, max_other) = other.get_axis_bounds(i);
            if max_other <= min_self || min_other >= max_self {
                return false;
            }
        }
        true
    }

    pub fn expand(&mut self, other: &BoundingBox<T>) -> bool {
        let mut expanded = false;
        for i in 0..3 {
            let (min_self, max_self) = self.get_axis_bounds(i);
            let (min_other, max_other) = other.get_axis_bounds(i);
            if min_other < min_self {
                self.p0.get_data_mut()[i] = min_other;
                expanded = true;
            }
            if max_other > max_self {
                self.p1.get_data_mut()[i] = max_other;
                expanded = true;
            }
        }
        expanded
    }

    pub fn get_p0(&self) -> &Vec3<T> {
        &self.p0
    }

    pub fn get_p1(&self) -> &Vec3<T> {
        &self.p1
    }

    pub fn get_volume(&self) -> T {
        let mut volume = T::one();

        for i in 0..3 {
            let (min, max) = self.get_axis_bounds(i);
            volume = volume * (max - min);
        }

        volume
    }

    pub fn calculate_axis_bounds(p0: &Vec3<T>, p1: &Vec3<T>, axis: usize) -> (T, T) {
        let mut min = p0.get_data()[axis];
        let mut max = p1.get_data()[axis];
        if min > max {
            let tmp = min;
            min = max;
            max = tmp;
        }
        (min, max)
    }

    pub fn get_axis_bounds(&self, axis: usize) -> (T, T) {
        let min = self.p0.get_data()[axis];
        let max = self.p1.get_data()[axis];
        (min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains() {
        let p0 = Vec3::from_array([0.0, 0.0, 0.0]);
        let p1 = Vec3::from_array([5.0, 4.0, 3.0]);
        let box0 = BoundingBox::new(p0, p1);

        let p0 = Vec3::from_array([1.0, 1.0, 1.0]);
        let p1 = Vec3::from_array([2.0, 2.0, 2.0]);
        let box1 = BoundingBox::new(p0, p1);

        assert!(box0.contains(&box1));
        assert!(!box1.contains(&box0));

        let p0 = Vec3::from_array([0.0, 0.0, 0.0]);
        let p1 = Vec3::from_array([5.0, 4.0, 3.0]);
        let box0 = BoundingBox::new(p1, p0);

        let p0 = Vec3::from_array([1.0, 1.0, 1.0]);
        let p1 = Vec3::from_array([2.0, 2.0, 2.0]);
        let box1 = BoundingBox::new(p1, p0);

        assert!(box0.contains(&box1));
        assert!(!box1.contains(&box0));

        let p0 = Vec3::from_array([0.0, 0.0, 0.0]);
        let p1 = Vec3::from_array([5.0, 4.0, 3.0]);
        let box0 = BoundingBox::new(p0, p1);

        let p0 = Vec3::from_array([-1.0, 1.0, 1.0]);
        let p1 = Vec3::from_array([2.0, 2.0, 2.0]);
        let box1 = BoundingBox::new(p0, p1);

        assert!(!box0.contains(&box1));
        assert!(!box1.contains(&box0));
    }

    #[test]
    fn overlaps() {
        let p0 = Vec3::from_array([0.0, 0.0, 0.0]);
        let p1 = Vec3::from_array([5.0, 4.0, 3.0]);
        let box0 = BoundingBox::new(p0, p1);

        let p0 = Vec3::from_array([1.0, 1.0, 1.0]);
        let p1 = Vec3::from_array([2.0, 2.0, 2.0]);
        let box1 = BoundingBox::new(p0, p1);

        assert!(box0.overlaps(&box1));
        assert!(box1.overlaps(&box0));

        let p0 = Vec3::from_array([0.0, 0.0, 0.0]);
        let p1 = Vec3::from_array([5.0, 4.0, 3.0]);
        let box0 = BoundingBox::new(p1, p0);

        let p0 = Vec3::from_array([-1.0, -1.0, -1.0]);
        let p1 = Vec3::from_array([2.0, 2.0, 2.0]);
        let box1 = BoundingBox::new(p1, p0);

        assert!(box0.overlaps(&box1));
        assert!(box1.overlaps(&box0));

        let p0 = Vec3::from_array([0.0, 0.0, 0.0]);
        let p1 = Vec3::from_array([5.0, 4.0, 3.0]);
        let box0 = BoundingBox::new(p0, p1);

        let p0 = Vec3::from_array([-2.0, -2.0, -2.0]);
        let p1 = Vec3::from_array([-1.0, -1.0, -1.0]);
        let box1 = BoundingBox::new(p0, p1);

        assert!(!box0.overlaps(&box1));
        assert!(!box1.overlaps(&box0));
    }

    #[test]
    fn hit() {
        let half_size = Vec3::from_array([2.0, 1.0, 3.0]);
        let box0 = BoundingBox::new(&half_size * (-1.0), &half_size * 1.0);

        for i in 0..3 {
            let mut origin = Vec3::new();
            origin.get_data_mut()[i] = - 2.0 * half_size.get_data()[i];
            let mut direction = Vec3::new();
            origin.get_data_mut()[i] = 1.0;
            let ray = Ray::from_vec(origin, direction);
            assert!(box0.hit(&ray, 0.0, 100.0));

            let mut origin = Vec3::new();
            origin.get_data_mut()[i] = - 2.0 * half_size.get_data()[i];
            let mut direction = Vec3::new();
            origin.get_data_mut()[(i + 1) % 3] = 1.0;
            let ray = Ray::from_vec(origin, direction);
            assert!(!box0.hit(&ray, 0.0, 100.0));
        }
    }

    #[test]
    fn volume() {
        let p0 = Vec3::from_array([-1.0, 2.0, -4.0]);
        let p1 = Vec3::from_array([5.0, 4.0, 3.0]);
        let box0 = BoundingBox::new(p0, p1);
        assert_eq!(box0.get_volume(), 6.0 * 2.0 * 7.0);
    }

    #[test]
    fn expand() {
        let p0 = Vec3::from_array([0.0, 0.0, 0.0]);
        let p1 = Vec3::from_array([5.0, 4.0, 3.0]);
        let mut box0 = BoundingBox::new(p0, p1);

        let p0 = Vec3::from_array([1.0, 1.0, 1.0]);
        let p1 = Vec3::from_array([2.0, 2.0, 2.0]);
        let box1 = BoundingBox::new(p0, p1);

        assert!(box0.contains(&box1));
        assert!(!box0.expand(&box1));
        assert!(box0.contains(&box1));

        let p0 = Vec3::from_array([0.0, 0.0, 0.0]);
        let p1 = Vec3::from_array([5.0, 4.0, 3.0]);
        let mut box0 = BoundingBox::new(p0, p1);

        let p0 = Vec3::from_array([-1.0, -1.0, -1.0]);
        let p1 = Vec3::from_array([2.0, 2.0, 2.0]);
        let box1 = BoundingBox::new(p1, p0);

        assert!(!box0.contains(&box1));
        assert!(box0.expand(&box1));
        assert!(box0.contains(&box1));
    }
}

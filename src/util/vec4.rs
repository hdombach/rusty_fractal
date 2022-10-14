use std::{ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign}, fmt::Display};
use crate::util::{vec2::Vec2, vec3::Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Vec4<T> {
    entries: [T; 4],
}


impl<T> Vec4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self {
            entries: [x, y, z, w]
        }
    }
}

pub type Float4 = Vec4<f32>;
pub type Int4 = Vec4<i32>;

impl<T> Vec4<T> where T : Copy + std::fmt::Debug {
    pub fn get_x(&self) -> T { self[0] }
    pub fn get_y(&self) -> T { self[1] }
    pub fn get_z(&self) -> T { self[2] }
    pub fn get_w(&self) -> T { self[3] }
    pub fn set_x(&mut self, value: T) { self[0] = value; }
    pub fn set_y(&mut self, value: T) { self[1] = value; }
    pub fn set_z(&mut self, value: T) { self[2] = value; }
    pub fn set_w(&mut self, value: T) { self[3] = value; }

    pub fn get_vec2(self) -> Vec2<T> {
        Vec2::new(self[0], self[1])
    }
    pub fn get_vec3(self) -> Vec3<T> {
        Vec3::new(self[0], self[1], self[2])
    }
}

impl<T> Index<usize> for Vec4<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl<T> IndexMut<usize> for Vec4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl<T> PartialEq for Vec4<T> where T : PartialEq {
    fn eq(&self, rhs: &Self) -> bool {
        self.entries == rhs.entries
    }
}

impl<T> Add for Vec4<T> where T : Add<Output = T> + Copy {
    type Output = Vec4<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self[0] + rhs[0],
            self[1] + rhs[1],
            self[2] + rhs[2],
            self[3] + rhs[3])
    }
}

impl<T> AddAssign for Vec4<T> where T : Add<Output = T> + Copy {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::new(
            self[0] + rhs[0],
            self[1] + rhs[1],
            self[2] + rhs[2],
            self[3] + rhs[3]);
    }
}

impl<T> Sub for Vec4<T> where T : Sub<Output = T> + Copy {
    type Output = Vec4<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self[0] - rhs[0],
            self[1] - rhs[1],
            self[2] - rhs[2],
            self[3] - rhs[3])
    }
}

impl<T> SubAssign for Vec4<T> where T : Sub<Output = T> + Copy {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self::new(
            self[0] - rhs[0],
            self[1] - rhs[1],
            self[2] - rhs[2],
            self[3] - rhs[3]);
    }
}

impl<T> Mul<T> for Vec4<T> where T : Mul<Output = T> + Copy {
    type Output = Vec4<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(
            self[0] * rhs,
            self[1] * rhs,
            self[2] * rhs,
            self[3] * rhs)
    }
}

impl<T> MulAssign<T> for Vec4<T> where T : Mul<Output = T> + Copy {
    fn mul_assign(&mut self, rhs: T) {
        *self = Self::new(
            self[0] * rhs,
            self[1] * rhs,
            self[2] * rhs,
            self[3] * rhs)
    }
}

impl<T> Div<T> for Vec4<T> where T : Div<Output = T> + Copy {
    type Output = Vec4<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(
            self[0] / rhs,
            self[1] / rhs,
            self[2] / rhs,
            self[3] / rhs)
    }
}

impl<T> DivAssign<T> for Vec4<T> where T : Div<Output = T> + Copy {
    fn div_assign(&mut self, rhs: T) {
        *self = Self::new(
            self[0] / rhs,
            self[1] / rhs,
            self[2] / rhs,
            self[3] / rhs);
    }
}

impl<T> Vec4<T> where T : Add<Output = T> + Mul<Output = T> + Copy {
    pub fn dot(lhs: Vec4<T>, rhs: Vec4<T>) -> T {
        lhs[0] * rhs[0] + 
            lhs[1] * rhs[1] +
            lhs[2] * rhs[2] +
            lhs[3] * rhs[3]
    }
}

#[cfg(test)]
mod tests {
    use super::{Vec4, Float4, Int4};
    #[test]
    fn create_float4() {
        let vector = Float4::new(4.2, 0.2, 3.9, -4.0);
        assert_eq!(vector.get_x(), 4.2);
        assert_eq!(vector.get_y(), 0.2);
        assert_eq!(vector.get_z(), 3.9);
        assert_eq!(vector.get_w(), -4.0);

        assert_eq!(vector[0], 4.2);
        assert_eq!(vector[1], 0.2);
        assert_eq!(vector[2], 3.9);
        assert_eq!(vector[3], -4.0);
    }

    #[test]
    fn create_int4() {
        let vector = Vec4::new(10, -11, 12, 13);
        assert_eq!(vector.get_x(), 10);
        assert_eq!(vector.get_y(), -11);
        assert_eq!(vector.get_z(), 12);
        assert_eq!(vector.get_w(), 13);

        assert_eq!(vector[0], 10);
        assert_eq!(vector[1], -11);
        assert_eq!(vector[2], 12);
        assert_eq!(vector[3], 13);
    }

    #[test]
    fn equals() {
        let vector1 = Vec4::new(10.2, 3.2, -1.0, 2.3);
        let vector2 = Vec4::new(10.2, 3.2, -1.0, 2.3);
        let vector3 = Vec4::new(10.2, 3.2, -1.01, 2.3);
        assert_eq!(vector1, vector2);
        assert_ne!(vector1, vector3);
    }

    #[test]
    fn editing() {
        let mut vector = Float4::new(1.0, 2.0, 3.0, 4.0);
        vector[0] = 12.0;
        vector[1] += 5.0;
        vector.set_w(10.1);

        assert_eq!(vector[0], 12.0);
        assert_eq!(vector[1], 7.0);
        assert_eq!(vector[2], 3.0);
        assert_eq!(vector[3], 10.1);
    }

    #[test]
    fn add() {
        let vec1 = Float4::new(4.0, -2.0, -2.2, 1.01);
        let vec2 = Float4::new(-2.0, 10.0, 12.4, 3.2);
        let result = vec1 + vec2;
        assert_eq!(result, Float4::new(2.0, 8.0, 10.2, 4.21));
    }

    #[test]
    fn add_assign() {
        let mut vector = Float4::new(10.0, 12.4, -2.3, -2.4);
        vector += Float4::new(1.01, -2.4, 5.0, -2.6);
        let result = Float4::new(11.01, 10.0, 2.7, -5.0);
        assert_eq!(vector, result);
    }

    #[test]
    fn sub() {
        let vec1 = Int4::new(2, 5, -2, -3);
        let vec2 = Int4::new(5, -2, 4, -5);
        assert_eq!(Int4::new(-3, 7, -6, 2), vec1 - vec2);
    }

    #[test]
    fn sub_assign() {
        let mut vec1 = Int4::new(4, 2, -5, -5);
        vec1 -= Int4::new(4, -10, 2, -3);
        assert_eq!(Int4::new(0, 12, -7, -2), vec1);
    }

    #[test]
    fn multiply() {
        let vec1 = Int4::new(3, 5, 2, 4);
        let result = vec1 * -2;
        assert_eq!(result, Int4::new(-6, -10, -4, -8));
    }

    #[test]
    fn mut_assign() {
        let mut vec1 = Float4::new(2.0, 5.0, 8.0, -1.0);
        vec1 *= 1.1;
        assert_eq!(vec1, Float4::new(2.2, 5.5, 8.8, -1.1));
    }

    #[test]
    fn divide() {
        let vec1 = Float4::new(2.0, -4.0, -2.2, -3.5);
        let result = vec1 / 0.5;
        assert_eq!(result, Float4::new(4.0, -8.0, -4.4, -7.0));
    }

    #[test]
    fn div_assign4() {
        let mut vec1 = Int4::new(8, 4, 2, -10);
        vec1 /= 2;
        assert_eq!(vec1, Int4::new(4, 2, 1, -5));
    }

    #[test]
    fn dot() {
        let vec1 = Int4::new(4, 2, 10, -2);
        let vec2 = Int4::new(8, -2, 0, 1);
        let result = Int4::dot(vec1, vec2);
        assert_eq!(result, 26);
    }
}

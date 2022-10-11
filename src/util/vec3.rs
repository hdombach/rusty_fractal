use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use crate::util::{vec2::Vec2, vec4::Vec4};

pub struct Vec3<T> {
    entries: [T; 3],
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            entries: [x, y, z]
        }
    }
}

impl<T> Vec3<T> where T : Copy {
    pub fn get_x(self) -> T { self[0] }
    pub fn get_y(self) -> T { self[1] }
    pub fn get_z(self) -> T { self[2] }
    pub fn set_x(mut self, value: T) { self[0] = value; }
    pub fn set_y(mut self, value: T) { self[1] = value; }
    pub fn set_z(mut self, value: T) { self[2] = value; }

    pub fn get_vec2(self) -> Vec2<T> {
        Vec2::new(self[0], self[1])
    }
    pub fn get_vec4(self, w: T) -> Vec4<T> {
        Vec4::new(self[0], self[1], self[2], w)
    }

    pub fn set_vec2(mut self, value: Vec2<T>) {
        self[0] = value[0];
        self[1] = value[1];
    }
    pub fn set_vec4(mut self, value: Vec4<T>) {
        self[0] = value[0];
        self[1] = value[1];
        self[2] = value[2];
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl<T> Add for Vec3<T> where T : Add<Output = T> + Copy {
    type Output = Vec3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl<T> AddAssign for Vec3<T> where T : Add<Output = T> + Copy {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]);
    }
}

impl<T> Sub for Vec3<T> where T : Sub<Output = T> + Copy {
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl<T> SubAssign for Vec3<T> where T : Sub<Output = T> + Copy {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]);
    }
}

impl<T> Mul<T> for Vec3<T> where T : Mul<Output = T> + Copy {
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl<T> MulAssign<T> for Vec3<T> where T : Mul<Output = T> + Copy {
    fn mul_assign(&mut self, rhs: T) {
        *self = Self::new(self[0] * rhs, self[1] * rhs, self[2] * rhs);
    }
}

impl<T> Div<T> for Vec3<T> where T : Div<Output = T> + Copy {
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self[0] / rhs, self[1] / rhs, self[2] / rhs)
    }
}

impl<T> DivAssign<T> for Vec3<T> where T : Div<Output = T> + Copy {
    fn div_assign(&mut self, rhs: T) {
        *self = Self::new(self[0] / rhs, self[1] / rhs, self[2] / rhs);
    }
}

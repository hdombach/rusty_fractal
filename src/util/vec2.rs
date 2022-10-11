use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use crate::util::{vec3::Vec3, vec4::Vec4};

pub struct Vec2<T> {
    entries: [T; 2],
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            entries: [x, y]
        }
    }
}

impl<T> Vec2<T> where T : Copy {
    pub fn get_x(self) -> T { self[0] }
    pub fn get_y(self) -> T { self[1] }
    pub fn set_x(mut self, value: T) { self[0] = value; }
    pub fn set_y(mut self, value: T) { self[1] = value; }

    pub fn get_vec3(self, z: T) -> Vec3<T> {
        Vec3::new(self[0], self[1], z)
    }

    pub fn get_vec4(self, z: T, w: T) -> Vec4<T> {
        Vec4::new(self[0], self[1], z, w)
    }
}

impl<T> Index<usize> for Vec2<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl<T> IndexMut<usize> for Vec2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl<T> Add for Vec2<T> where T : Add<Output = T> + Copy {
    type Output = Vec2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self[0] + rhs[0], self[1] + rhs[1])
    }
}

impl<T> AddAssign for Vec2<T> where T : Add<Output = T> + Copy {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::new(self[0] + rhs[0], self[1] + rhs[1]);
    }
}

impl<T> Sub for Vec2<T> where T : Sub<Output = T> + Copy {
    type Output = Vec2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self[0] - rhs[0], self[1] - rhs[1])
    }
}

impl<T> SubAssign for Vec2<T> where T : Sub<Output = T> + Copy {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self::new(self[0] - rhs[0], self[1] - rhs[1]);
    }
}

impl<T> Mul<T> for Vec2<T> where T : Mul<Output = T> + Copy {
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self[0] * rhs, self[1] * rhs)
    }
}

impl<T> MulAssign<T> for Vec2<T> where T : Mul<Output = T> + Copy {
    fn mul_assign(&mut self, rhs: T) {
        *self = Self::new(self[0] * rhs, self[1] * rhs);
    }
}

impl<T> Div<T> for Vec2<T> where T : Div<Output = T> + Copy {
    type Output = Vec2<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self[0] / rhs, self[1] / rhs)
    }
}

impl<T> DivAssign<T> for Vec2<T> where T : Div<Output = T> + Copy {
    fn div_assign(&mut self, rhs: T) {
        *self = Self::new(self[0] / rhs, self[1] / rhs);
    }
}

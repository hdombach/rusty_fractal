use super::vec4::Float4;
use core::fmt;
use std::fmt::write;
use std::ops::{Index, IndexMut};
use std::ops::{Mul, MulAssign};

#[derive(Copy, Clone)]
pub struct Mat4x4 {
    rows: [Float4; 4],
}

impl Mat4x4 {
    pub fn new(
        row0: Float4,
        row1: Float4,
        row2: Float4,
        row3: Float4) -> Self{
        Self {
            rows: [row0, row1, row2, row3]
        }
    }
    pub fn from_rows(rows: [Float4; 4]) -> Self {
        Self {
            rows,
        }
    }
}

impl Mat4x4 {
    pub fn identity() -> Self {
        Self {
            rows: [
                Float4::new(1.0, 0.0, 0.0, 0.0),
                Float4::new(0.0, 1.0, 0.0, 0.0),
                Float4::new(0.0, 0.0, 1.0, 0.0),
                Float4::new(0.0, 0.0, 0.0, 1.0)]
        }
    }

    pub fn rotate_x(angle: f32) -> Self {
        let sin = angle.sin();
        let cos = angle.cos();
        Self {
            rows: [
                Float4::new(1.0, 0.0, 0.0, 0.0),
                Float4::new(0.0, cos, -sin, 0.0),
                Float4::new(0.0, sin, cos, 0.0),
                Float4::new(0.0, 0.0, 0.0, 1.0)]
        }
    }

    pub fn rotate_y(angle: f32) -> Self {
        let sin = angle.sin();
        let cos = angle.cos();
        Self {
            rows: [
                Float4::new(cos, 0.0, sin, 0.0),
                Float4::new(0.0, 1.0, 0.0, 0.0),
                Float4::new(-sin, 0.0, cos, 0.0),
                Float4::new(0.0, 0.0, 0.0, 1.0)]
        }
    }

    pub fn rotate_z(angle: f32) -> Self {
        let sin = angle.sin();
        let cos = angle.cos();
        Self {
            rows: [
                Float4::new(cos, -sin, 0.0, 0.0),
                Float4::new(sin, cos, 0.0, 0.0),
                Float4::new(0.0, 0.0, 1.0, 0.0),
                Float4::new(0.0, 0.0, 0.0, 1.0)]
        }
    }

    pub fn euclidian_rotation(x: f32, y: f32, z: f32) -> Self {
        Self::rotate_z(z) * Self::rotate_y(y) * Self::rotate_x(x)
    }
}

impl Mat4x4 {
    pub fn get_row(&self, index: usize) -> Float4 {
        self.rows[index]
    }
    pub fn get_column(&self, index: usize) -> Float4 {
        Float4::new(
            self.rows[0][index],
            self.rows[1][index],
            self.rows[2][index],
            self.rows[3][index])
    }
}

impl fmt::Debug for Mat4x4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        result += "Mat4x4 {{\n";
        for i in 0..4 {
            result += format!(
                "\t[{}, {}, {}, {}]\n",
                self[i][0],
                self[i][1],
                self[i][2],
                self[i][3]).as_str();
        }
        result += "}}\n";

        write!(f, "{}", result)
    }
}

impl Index<usize> for Mat4x4 {
    type Output = Float4;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl IndexMut<usize> for Mat4x4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}

impl PartialEq for Mat4x4 {
    fn eq(&self, rhs: &Self) -> bool {
        self.rows == rhs.rows
    }
}

impl Mul for Mat4x4 {
    type Output = Mat4x4;

    fn mul(self, rhs: Mat4x4) -> Self::Output {
        let row0 = Float4::new(
            Float4::dot(self.get_row(0), rhs.get_column(0)),
            Float4::dot(self.get_row(0), rhs.get_column(1)),
            Float4::dot(self.get_row(0), rhs.get_column(2)),
            Float4::dot(self.get_row(0), rhs.get_column(3)));

        let row1 = Float4::new(
            Float4::dot(self.get_row(1), rhs.get_column(0)),
            Float4::dot(self.get_row(1), rhs.get_column(1)),
            Float4::dot(self.get_row(1), rhs.get_column(2)),
            Float4::dot(self.get_row(1), rhs.get_column(3)));
        let row2 = Float4::new(
            Float4::dot(self.get_row(2), rhs.get_column(0)),
            Float4::dot(self.get_row(2), rhs.get_column(1)),
            Float4::dot(self.get_row(2), rhs.get_column(2)),
            Float4::dot(self.get_row(2), rhs.get_column(3)));
        let row3 = Float4::new(
            Float4::dot(self.get_row(3), rhs.get_column(0)),
            Float4::dot(self.get_row(3), rhs.get_column(1)),
            Float4::dot(self.get_row(3), rhs.get_column(2)),
            Float4::dot(self.get_row(3), rhs.get_column(3)));
        Mat4x4::from_rows([row0, row1, row2, row3])
    }
}

impl MulAssign for Mat4x4 {
    fn mul_assign(&mut self, rhs: Self) {
        let row0 = Float4::new(
            Float4::dot(self.get_row(0), rhs.get_column(0)),
            Float4::dot(self.get_row(0), rhs.get_column(1)),
            Float4::dot(self.get_row(0), rhs.get_column(2)),
            Float4::dot(self.get_row(0), rhs.get_column(3)));
        let row1 = Float4::new(
            Float4::dot(self.get_row(1), rhs.get_column(0)),
            Float4::dot(self.get_row(1), rhs.get_column(1)),
            Float4::dot(self.get_row(1), rhs.get_column(2)),
            Float4::dot(self.get_row(1), rhs.get_column(3)));
        let row2 = Float4::new(
            Float4::dot(self.get_row(2), rhs.get_column(0)),
            Float4::dot(self.get_row(2), rhs.get_column(1)),
            Float4::dot(self.get_row(2), rhs.get_column(2)),
            Float4::dot(self.get_row(2), rhs.get_column(3)));
        let row3 = Float4::new(
            Float4::dot(self.get_row(3), rhs.get_column(0)),
            Float4::dot(self.get_row(3), rhs.get_column(1)),
            Float4::dot(self.get_row(3), rhs.get_column(2)),
            Float4::dot(self.get_row(3), rhs.get_column(3)));
        *self = Mat4x4::new(row0, row1, row2, row3);
    }
}

#[cfg(test)]
mod tests {
    use super::{Mat4x4, Float4};

    #[test]
    fn create_mat4x4() {
        let matrix = Mat4x4::new(
            Float4::new(0.0, 1.0, 2.0, 3.0),
            Float4::new(4.0, 5.0, 6.0, 7.0),
            Float4::new(8.0, 9.0, 10.0, 11.0),
            Float4::new(12.0, 13.0, 14.0, 15.0));
        assert_eq!(matrix[2], Float4::new(8.0, 9.0, 10.0, 11.0));
        assert_eq!(matrix[2][0], 8.0);
        assert_eq!(matrix[3][0], 12.0);
        assert_eq!(matrix[1][3], 7.0);
    }

    #[test]
    fn mul() {
        let matrix1 = Mat4x4::new(
            Float4::new(4.0, 2.0, -3.0, 2.0),
            Float4::new(4.0, -2.0, 3.0, 10.0),
            Float4::new(0.5, 1.5, 4.0, 0.0),
            Float4::new(2.0, 0.0, 10.0, -9.0));
        let matrix2 = Mat4x4::new(
            Float4::new(1.0, 0.0, 2.0, 10.0),
            Float4::new(-9.0, 3.0, 2.0, -1.0),
            Float4::new(3.0, 2.0, 8.0, 1.0),
            Float4::new(2.0, 4.5, 2.5, 1.5));
        let result = Mat4x4::new(
            Float4::new(-19.0, 9.0, -7.0, 38.0),
            Float4::new(51.0, 45.0, 53.0, 60.0),
            Float4::new(-1.0, 12.5, 36.0, 7.5),
            Float4::new(14.0, -20.5, 61.5, 16.5));
        assert_eq!(result, matrix1 * matrix2);
    }
}

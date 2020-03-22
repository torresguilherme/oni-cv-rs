use crate::vector_ops::{Vector3, Vector4};

use std::ops::{Index, IndexMut};

/// Matrix3x3
/// Contains information and functionality for a 3x3 square matrix
#[derive(Clone)]
pub struct Matrix3x3 {
    mat: [[f64; 3]; 3]
}

impl Default for Matrix3x3 {
    fn default() -> Self {
        Self {
            mat: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0]
            ]
        }
    }
}

/* INDEXES ARE VERY PROBLEMATIC TO IMPLEMENT AND WILL BE DEALT WITH LATER
impl Index<usize> for Matrix3x3 {
    type Output = Result<f64, String>;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 ..= 2 => Ok(self.mat[i]),
            _ => Err(String::from("Index out of bounds for Matrix 3x3"))
        }
    }
}
 */

pub fn multiply_mat_3x3(m1: Matrix3x3, m2: Matrix3x3) {
    unimplemented!();
}

pub fn multiply_vt_mat_3x3(v: Vector3, m: Matrix3x3) {
    unimplemented!();
}

pub fn multiply_mat_3x3_v(m: Matrix3x3, v: Vector3) {
    unimplemented!();
}

/// Matrix4x4
/// Contains information and functionality for a 4x4 square matrix
#[derive(Clone)]
pub struct Matrix4x4 {
    mat: [[f64; 4]; 4]
}

impl Default for Matrix4x4 {
    fn default() -> Self {
        Self {
            mat: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }
}

pub fn multiply_mat_4x4(m1: Matrix4x4, m2: Matrix4x4) {
    unimplemented!();
}

pub fn multiply_vt_mat_4x4(v: Vector4, m: Matrix4x4) {
    unimplemented!();
}

pub fn multiply_mat_4x4_v(m: Matrix4x4, v: Vector4) {
    unimplemented!();
}

/// Matrix
/// Contains information about an arbitrary-size matrix
pub struct Mat {
}

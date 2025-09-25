pub use matrix::*;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);
impl<T: Copy + Add<Output = T>> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn add(self, ohter: Self) -> Self::Output {
        if self.0.len() != ohter.0.len() {
            return None;
        }
        let mut v: Vec<Vec<T>> = vec![];
        for i in 0..self.0.len() {
            if self.0[i].len() != ohter.0[i].len() {
                return None;
            }
            let mut mat: Vec<T> = vec![];
            for j in 0..self.0[i].len() {
                let nb = self.0[i][j] + ohter.0[i][j];
                mat.push(nb);
            }
            v.push(mat);
        }
        Some(Matrix(v))
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn sub(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut v: Vec<Vec<T>> = vec![];
        for i in 0..self.0.len() {
            if self.0[i].len() != other.0[i].len() {
                return None;
            }
            let mut mat: Vec<T> = vec![];
            for j in 0..self.0[i].len() {
                let nb = self.0[i][j] - other.0[i][j];
                mat.push(nb);
            }
            v.push(mat);
        }
        Some(Matrix(v))
    }
}

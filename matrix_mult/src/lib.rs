use std::fmt::Debug;

pub use matrix::*;
use std::ops::Mul;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Clone + Debug> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0.len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0[0].len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut res: Vec<T> = vec![];
        for i in 0..self.0.len() {
            res.push(self.0[i][n].clone())
        }
        res
    }
}

impl<T:Copy + Mul<Output = T>> Mul for Matrix<T> {
   type Output = Option<Matrix<T>>;
 
}

use lalgebra_scalar::Scalar;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar<Item = T>>(pub Vec<T>);

use std::ops::{Add, Mul};

impl<T: Scalar<Item = T> + Add<Output = T> + Copy> Add for Vector<T> {
    type Output = Option<Vector<T>>;
    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }

        let res: Vec<T> = self
            .0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect();

        Some(Vector(res))
    }
}

impl<T: Scalar<Item = T> + Add<Output = T> + Mul<Output = T> + Copy> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut res = T::zero();

        for (i, v) in self.0.iter().enumerate() {
            res = res + (v.clone() * other.0[i]);
        }

        Some(res)
    }
}
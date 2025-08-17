use std::{
    cmp::Reverse,
    ops::{Add, Sub, Mul},
};

use crate::narray::{errors::NErrors, matrix::NMatrix};

#[derive(Debug, Clone)]
pub struct NVector {
    pub data: Vec<f32>,
    pub len: usize,
}

impl NVector {
    pub fn new(length: usize) -> NVector {
        NVector {
            data: vec![],
            len: length,
        }
    }

    pub fn new_init(data: Vec<f32>) -> NVector {
        let length = data.len();
        NVector { data, len: length }
    }

    pub fn get(&self, i: usize) -> Option<f32> {
        if i >= self.len {
            return None;
        }
        self.data.get(i).copied()
    }

    pub fn set(&mut self, i: usize, value: f32) -> Result<(), NErrors> {
        if self.len <= i {
            return Err(NErrors::DimensionError);
        }
        self.data[i] = value;
        Ok(())
    }

    pub fn dot(&self, other: &NVector) -> Result<f32, NErrors> {
        if self.len != other.len {
            return Err(NErrors::DimensionError);
        }
        Ok(self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .sum())
    }

    pub fn hadamard(&self, other: &NVector) -> Result<NVector, NErrors> {
        let hadamard_vector = self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| {a * b})
                .collect::<Vec<f32>>();
        Ok(NVector::new_init(hadamard_vector))
    }
}

impl Add<&NVector> for NVector {
    type Output = Result<NVector, NErrors>;
    fn add(self, rhs: &Self) -> Self::Output {
        if self.len != rhs.len {
            return Err(NErrors::DimensionError);
        }
        let val = self
            .data
            .iter()
            .zip(rhs.data.clone())
            .map(|(a, b)| a + b)
            .collect();
        Ok(NVector::new_init(val))
    }
}

impl Mul<&f32> for &NVector {
    type Output = Result<NVector, NErrors>;
    fn mul(self, rhs: &f32) -> Self::Output {
        let vals = self.data.iter().map(|x| x * rhs).collect::<Vec<f32>>();
        Ok(NVector::new_init(vals))
    }
}

impl Sub<&NVector> for &NVector {
    type Output = Result<NVector, NErrors>;
    fn sub(self, rhs: &NVector) -> Self::Output {
        let vals = self.data.iter().zip(rhs.data.iter()).map(|(x, y)| x - y).collect();
        Ok(NVector::new_init(vals))
    }
}

use std::ops::Add;

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

    pub fn set(&mut self, i: usize, value: usize) -> Result<(), NErrors> {
        todo!()
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

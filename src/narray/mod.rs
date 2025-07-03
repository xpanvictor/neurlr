use std::ops::Mul;

use crate::narray::{matrix::NMatrix, vector::NVector};

pub mod matrix;
pub mod vector;

impl Mul<NVector> for NMatrix {
    type Output = NVector;

    fn mul(self, rhs: NVector) -> Self::Output {
        
    }
}

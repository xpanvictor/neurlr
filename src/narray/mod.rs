use std::ops::Mul;

use crate::{
    NMatrix,
    narray::{matrix::NMatrix, vector::NVector},
};

pub mod macros;
pub mod matrix;
pub mod vector;

impl Mul<NVector> for NMatrix {
    type Output = NVector;

    fn mul(self, rhs: NVector) -> Self::Output {
        let m = NMatrix![2, 3];
        todo!()
    }
}

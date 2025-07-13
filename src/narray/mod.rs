use std::ops::Mul;

use crate::{
    NMatrix, NVector,
    narray::{matrix::NMatrix, vector::NVector},
};

pub mod errors;
pub mod macros;
pub mod matrix;
pub mod vector;

impl Mul<NVector> for NMatrix {
    type Output = NVector;

    fn mul(self, rhs: NVector) -> Self::Output {
        let mut res = vec![];
        self.by_iter(matrix::NMatrixAxis::ROW).for_each(|raw_row| {
            let row = NVector!(raw_row);
            if let Ok(vdot) = row.dot(&rhs) {
                res.push(vdot);
            }
        });

        NVector!(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_multiply_m_v() {
        let m1 = NMatrix![2, 2; 1., 2., 3., 4.];

        let v1 = NVector![2; 1., 2.];

        let res = m1 * v1;

        println!("{:?} res", res);
    }
}

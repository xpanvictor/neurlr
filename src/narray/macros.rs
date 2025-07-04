use crate::narray::matrix::NMatrix;
use crate::narray::vector::NVector;

/// DSL Macros
/// To define and initialize NMatrix
#[macro_export]
macro_rules! NMatrix {
    [$row: expr, $col: expr] => {
        NMatrix::new($row, $col)
    };


    [$row: expr, $col: expr; $($x: expr),*] => {
        {
            let data = vec![$($x as f32),*];
            NMatrix::new_init($row, $col, data)
        }
    }
}

/// DSL to define NVector
#[macro_export]
macro_rules! NVector {
    // todo: check this
    [$len: expr; $($x: expr),*] => {
        {
            let data = vec![$($x as f32),*];
            NVector::new_init(data)
         }
    };

    ($data: expr) => {
        {
            NVector::new_init($data)
        }
    };
}

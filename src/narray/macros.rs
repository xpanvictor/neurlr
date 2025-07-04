use crate::narray::NMatrix;

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

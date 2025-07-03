use std::{
    ffi::NulError,
    fmt::Error,
    ops::{Index, IndexMut},
    str::Utf8Error,
};

pub struct NMatrix {
    data: Vec<f32>,
    rows: usize,
    cols: usize,
}

impl NMatrix {
    pub fn new(rows: usize, cols: usize) -> NMatrix {
        let mut init_vec = Vec::new();
        unsafe {
            init_vec.set_len(rows * cols);
        }
        NMatrix {
            data: init_vec,
            rows,
            cols,
        }
    }

    pub fn set(&mut self, row: usize, col: usize, val: f32) -> Result<(), Error> {
        if self.rows - 1 < row || self.cols - 1 < col {
            todo!()
        }
        self.data[row * self.cols + col] = val;
        Ok(())
    }
}

impl Index<usize> for NMatrix {
    type Output = [f32];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index..index + self.cols]
    }
}

impl IndexMut<usize> for NMatrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index..index + self.cols]
    }
}

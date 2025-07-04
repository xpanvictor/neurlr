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

pub enum NMatrixAxis {
    ROW,
    COL,
}

pub struct NMatrixAxisIter<'a> {
    matrix: &'a NMatrix,
    axis: NMatrixAxis,
    index: usize,
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

    pub fn new_init(rows: usize, cols: usize, data: Vec<f32>) -> NMatrix {
        let mut m = Self::new(rows, cols);
        m.data = data;
        m
    }

    pub fn set(&mut self, row: usize, col: usize, val: f32) -> Result<(), Error> {
        if self.rows - 1 < row || self.cols - 1 < col {
            todo!()
        }
        self.data[row * self.cols + col] = val;
        Ok(())
    }

    pub fn by_iter<'a>(&'a self, axis: NMatrixAxis) -> NMatrixAxisIter<'a> {
        NMatrixAxisIter {
            axis,
            matrix: self,
            index: 0,
        }
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

impl<'a> Iterator for NMatrixAxisIter<'a> {
    type Item = Vec<f32>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.axis {
            NMatrixAxis::COL => {
                let current = self.index;
                // take each rows
                let cols = self.matrix.cols;
                if self.matrix.cols >= current {
                    return None;
                }
                let curr_vec = self
                    .matrix
                    .data
                    .chunks(cols)
                    .collect::<Vec<&[f32]>>()
                    .into_iter()
                    .map(|c| c.iter().nth(current).unwrap().to_owned())
                    .collect::<Vec<f32>>();
                // incr index
                self.index += 1;
                Some(curr_vec)
            }
            NMatrixAxis::ROW => {
                let cols = self.matrix.cols;
                if self.matrix.rows >= self.index {
                    return None;
                }
                // fix: why map again?
                let curr_vec = self
                    .matrix
                    .data
                    .iter()
                    .skip(self.index * cols)
                    .take(cols)
                    .map(|c| c.to_owned())
                    .collect::<Vec<f32>>();

                // incr index
                self.index += 1;
                Some(curr_vec)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::NMatrix;

    #[test]
    fn check_columnize() {
        let t1 = NMatrix![3, 3; 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
        let cols = [[0., 3., 6.], [1., 4., 7.], [2., 5., 8.]];
        let mut curr = 0;
        t1.by_iter(NMatrixAxis::COL).for_each(|a_col| {
            assert_eq!(a_col, cols[curr], "Failed comparing columnize");
            curr += 1;
        });
    }

    #[test]
    fn check_row_iter() {
        let t1 = NMatrix![3, 3; 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
        let rows = [[0., 1., 2.], [3., 4., 5.], [6., 7., 8.]];
        let mut curr = 0;
        t1.by_iter(NMatrixAxis::ROW).for_each(|a_row| {
            assert_eq!(a_row, rows[curr], "Failed comparing rowize");
            curr += 1;
        });
    }
}

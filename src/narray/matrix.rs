use std::{
    ffi::NulError,
    fmt::Error,
    ops::{Add, Index, IndexMut, Mul, Sub},
    str::Utf8Error,
};

use crate::{
    NMatrix, NVector,
    narray::{errors::NErrors, vector::NVector},
};

#[derive(Debug)]
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

    pub fn compare_dimensions(&self, rhs: &NMatrix) -> bool {
        let NMatrix {
            cols: l_cols,
            rows: l_rows,
            ..
        } = self;
        let NMatrix {
            cols: r_cols,
            rows: r_rows,
            ..
        } = rhs;

        l_cols == r_cols && l_rows == r_rows
    }

    pub fn transpose(&self) -> NMatrix {
        let mut res = Vec::new();
        self.by_iter(NMatrixAxis::COL).for_each(|col| {
            res.extend(col);
        });
        // exchange cols & rows
        NMatrix::new_init(self.cols, self.rows, res)
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

impl Mul for NMatrix {
    type Output = Result<NMatrix, NErrors>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res: Vec<f32> = vec![];
        if self.rows != rhs.cols {
            return Err(NErrors::TypeError);
        }
        // take each as vector
        self.by_iter(NMatrixAxis::ROW).for_each(|m1_row| {
            let v1: NVector = NVector!(m1_row);
            rhs.by_iter(NMatrixAxis::COL).for_each(|m2_col| {
                let v2 = NVector!(m2_col);
                if let Ok(vdot) = v1.dot(&v2) {
                    res.push(vdot);
                } else {
                    panic!("I'll look at this later")
                }
            });
        });

        Ok(NMatrix::new_init(self.rows, self.cols, res))
    }
}

/// Scalar multiplication
type Operand = f32;
impl Mul<isize> for NMatrix {
    type Output = Result<NMatrix, NErrors>;

    fn mul(self, rhs: isize) -> Self::Output {
        let res = self.data.iter().map(|a| a * rhs as f32).collect();
        Ok(NMatrix::new_init(self.rows, self.cols, res))
    }
}
impl<'a> Mul<&'a f32> for &'a NMatrix {
    type Output = NMatrix; 

    fn mul(self, rhs: &'a f32) -> Self::Output {
        let res = self.data.iter().map(|a| a * rhs as &f32).collect();
        NMatrix::new_init(self.rows, self.cols, res)
    }
}

/// Addition
impl Add for NMatrix {
    type Output = Result<NMatrix, NErrors>;

    fn add(self, rhs: Self) -> Self::Output {
        if !self.compare_dimensions(&rhs) {
            return Err(NErrors::DimensionError);
        };

        let res = self.data.iter().zip(rhs.data).map(|(a, b)| a + b).collect();
        Ok(NMatrix::new_init(self.rows, self.cols, res))
    }
}

/// Substraction
impl Sub for NMatrix {
    type Output = Result<NMatrix, NErrors>;

    fn sub(self, rhs: Self) -> Self::Output {
        if !self.compare_dimensions(&rhs) {
            return Err(NErrors::DimensionError);
        };

        let res = self.data.iter().zip(rhs.data).map(|(a, b)| a - b).collect();
        Ok(NMatrix::new_init(self.rows, self.cols, res))
    }
}

impl<'a> Sub for &'a NMatrix {
    type Output = Result<NMatrix, NErrors>;

    fn sub(self, rhs: Self) -> Self::Output {
        if !self.compare_dimensions(&rhs) {
            return Err(NErrors::DimensionError);
        };

        let res = self.data.iter().zip(rhs.data.iter()).map(|(a, b)| a - b).collect();
        Ok(NMatrix::new_init(self.rows, self.cols, res))
    }
}

impl PartialEq for NMatrix {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
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
                if self.matrix.cols <= current {
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
                if self.matrix.rows <= self.index {
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

    #[test]
    fn test_matrix_mul() {
        let m1 = NMatrix![2, 2; 1., 2., 3., 4.];
        let m2 = NMatrix![2, 2; 1., 2., 3., 4.];

        let mr = (m1 * m2).unwrap();
        let mx = NMatrix![2, 2; 7., 10., 15., 22.];
        assert_eq!(mr, mx);
    }

    #[test]
    fn test_transpose() {
        let m1 = NMatrix![3, 2; 1., 2., 3., 4., 5., 6.];
        let mx = NMatrix![2, 2; 1., 3., 5., 2., 4., 6.];
        let mr = m1.transpose();

        assert_eq!(mr.cols, m1.rows);
        assert_eq!(mr.rows, m1.cols);

        assert_eq!(m1.transpose(), mx);
    }
}

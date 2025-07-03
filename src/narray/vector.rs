pub struct NVector {
    pub data: Vec<f32>,
    pub len: usize,
}

enum NError {}

impl NVector {
    pub fn new() -> NVector {
        NVector {
            data: vec![],
            len: 0,
        }
    }

    pub fn get(self, i: usize) -> Option<f32> {
        if i >= self.len {
            return None;
        }
        self.data.get(i).copied()
    }

    pub fn set(mut self, i: usize, value: usize) -> Result<(), NError> {
        todo!()
    }

    pub fn dot(self, other: NVector) -> f32 {
        if self.len != other.len {
            todo!()
        }
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

use std::fmt::Debug;

use crate::{
    NMatrix,
    narray::{self, errors::NErrors, matrix::NMatrix, vector::NVector},
};
use rand::{distributions::Normal, prelude::Distribution};

#[derive(Debug)]
pub enum ActivationFn {
    ReLU,
    SoftMax,
    Sigmoid,
}

#[derive(Debug)]
pub struct Layer {
    input_size: usize,
    output_size: usize,
    // size -> o_s * i_s
    weight: NMatrix,
    biases: NVector,
    activation_fn: ActivationFn,
    cache_z: NVector,
    cache_input: NVector,
    cache_output: NVector,
}

impl Layer {
    pub fn new(i_s: usize, o_s: usize, a_fn: ActivationFn) -> Layer {
        Layer {
            input_size: i_s,
            output_size: o_s,
            weight: Self::initialize_weight(&a_fn, i_s, o_s),
            biases: NVector::new(o_s),
            activation_fn: a_fn,
            cache_input: NVector::new(o_s),
            cache_z: NVector::new(o_s),
            cache_output: NVector::new(o_s),
        }
    }

    fn initialize_weight(a_fn: &ActivationFn, i_s: usize, o_s: usize) -> NMatrix {
        let normal = match a_fn {
            // Relu uses He - Kaiming init
            ActivationFn::ReLU => {
                // N(0, sqrt(2/n))
                Normal::new(0., (2. / i_s as f64).sqrt())
            }
            // Softmax use Xavier
            // N(0, sqrt(1/n))
            ActivationFn::SoftMax => Normal::new(0., (1. / i_s as f64).sqrt()),
            _ => panic!("No weight initialization algorithm found"),
        };
        let val = (0..i_s)
            .map(|_| normal.sample(&mut rand::thread_rng()) as f32)
            .collect::<Vec<f32>>();
        NMatrix::new_init(o_s, i_s, val)
    }

    fn activate(&self, z: &NVector) -> NVector {
        match self.activation_fn {
            ActivationFn::ReLU => Self::relu(z),
            _ => todo!(),
        }
    }

    fn relu(z: &NVector) -> NVector {
        let act_data = z
            .data
            .iter()
            .map(|a| if a > &0. { a.clone() } else { 0. })
            .collect();
        NVector::new_init(act_data)
    }

    pub fn forward(&mut self, input: &NVector) -> Result<NVector, NErrors> {
        // store input
        self.cache_input = input.clone();
        // z = W*x + b
        let z = (&self.weight * &self.cache_input + &self.biases)?;
        // store z
        self.cache_z = z.clone();
        // activate z
        let a = self.activate(&z);
        // store a
        self.cache_output = a.clone();
        // ret
        Ok(a)
    }
}

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
    pub weight: NMatrix,
    pub biases: NVector,
    activation_fn: ActivationFn,
    cache_z: NVector,
    cache_input: NVector,
    cache_output: NVector,
    cache_error: NVector,
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
            cache_error: NVector::new(o_s),
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

    fn prime_activate(&self, z: &NVector) -> NVector {
        match self.activation_fn {
            ActivationFn::ReLU => todo!(),
            _ => todo!()
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

    fn cost_gradient(activated_output: &NVector, expected_output: &NVector) -> NVector {
        (activated_output - expected_output).unwrap()
    }

    pub fn back_propagrate(&mut self, expected_output: &NVector, _my_error: Option<&NVector>) -> NVector {
        let prime_activated_input = self.prime_activate(&self.cache_input);
        let my_error = if _my_error.is_some() {_my_error.unwrap()} else {
            // compute my error, I'm last level
            // using the available cost fn
            let grad_cost = Self::cost_gradient(&self.cache_output, expected_output);
            // return error -> grad hadamard relu'(input)
            self.cache_error = (grad_cost.hadamard(&prime_activated_input)).unwrap();
            &self.cache_error
        };
        
        // prev_layer_error -> (my_weight * my_error) hadamard my_prime_activated_input
        (&self.weight * my_error).hadamard(&prime_activated_input).unwrap()
    }

    pub fn update(&mut self, weight: NMatrix, bias: NVector) -> Result<(), NErrors> {
        self.weight = weight;
        self.biases = bias;
        Ok(())
    }
}

use crate::{
    narray::{errors::NErrors, matrix::NMatrix, vector::NVector},
    network::layer::{self, Layer}, NVector,
};

pub struct Network {
    layers: Vec<Layer>,
}

pub struct TrainingDataElem (
    // x
    NVector,
    // y
    NVector
);

type TTrainingData = Vec<TrainingDataElem>;

impl Network {
    fn forward(&mut self, input: NVector) -> Result<NVector, NErrors> {
        let mut x = input.clone();
        for layer in self.layers.iter_mut() {
            x = layer.forward(&x)?
        }
        Ok(x)
    }

    /// Using stochastic gradient descent
    /// Finds a random sample and updates progress
    /// Updates mini batch
    /// Log epoch
    fn SGD(&mut self, training_data: TTrainingData,) {
        
    }

    // update mini batch based on calculated value and backpropagation
    fn update_mini_batch(&mut self, mini_batch: &TTrainingData, eta: f32) -> Result<(), NErrors> {
        mini_batch
            .iter()
            .for_each(|TrainingDataElem(x, y)| {
                // retrieve back propagate result
                let (grad_w, grad_b) = &self.back_propagrate(x, y);
                // update weights and biases
                for layer in self.layers.iter_mut() {
                    // for each layer, new weights = w - eta*gradient
                    let current_weight = &layer.weight;
                    let weight_prime = (current_weight - &(grad_w * &eta)).unwrap();
                    // same with bias
                    let current_biases = &layer.biases;
                    let biases_prime = (current_biases - &(grad_b * &eta).unwrap()).unwrap();
                    let _ = layer.update(weight_prime, biases_prime);
                }
            });
        Ok(())
    }

    // blackbox back propagation
    fn back_propagrate(&mut self, x: &NVector, y: &NVector) -> (NMatrix, NVector) {
        // feed forward the layers
        let mut z_f = x.clone();
        for layer in self.layers.iter_mut() {
            z_f = layer.forward(&z_f).unwrap();
        }
        for layer in self.layers.iter_mut().rev() {
            // back propagate errors
        }
        todo!()
    }
}

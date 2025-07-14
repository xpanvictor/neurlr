use crate::{
    narray::{errors::NErrors, vector::NVector},
    network::layer::Layer,
};

pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    fn forward(&mut self, input: NVector) -> Result<NVector, NErrors> {
        let mut x = input.clone();
        for layer in self.layers.iter_mut() {
            x = layer.forward(&x)?
        }
        Ok(x)
    }
}

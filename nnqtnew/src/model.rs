use burn::nn::{Linear, ReLU};
use burn::tensor::{Tensor, backend::Backend};

#[derive(Debug, burn::module::Module)]
pub struct TrafficModel<B: Backend> {
    fc1: Linear<B>,
    fc2: Linear<B>,
}

impl<B: Backend> TrafficModel<B> {
    pub fn new() -> Self {
        Self {
            fc1: Linear::new(5, 16),
            fc2: Linear::new(16, 3),
        }
    }

    pub fn forward(&self, x: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = self.fc1.forward(x).relu();
        self.fc2.forward(x)
    }
}

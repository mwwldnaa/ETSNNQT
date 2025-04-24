use ndarray::{Array2, Axis};
use rand::prelude::*;

pub struct NeuralNetwork {
    w1: Array2<f32>,
    w2: Array2<f32>,
}

impl NeuralNetwork {
    pub fn new(input: usize, hidden: usize, output: usize) -> Self {
        let mut rng = thread_rng();
        let w1 = Array2::from_shape_fn((input, hidden), |_| rng.gen_range(-1.0..1.0));
        let w2 = Array2::from_shape_fn((hidden, output), |_| rng.gen_range(-1.0..1.0));
        Self { w1, w2 }
    }

    fn relu(x: Array2<f32>) -> Array2<f32> {
        x.mapv(|v| v.max(0.0))
    }

    fn softmax(mut x: Array2<f32>) -> Array2<f32> {
        for mut row in x.outer_iter_mut() {
            let max = row.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            let sum: f32 = row.iter().map(|v| (*v - max).exp()).sum();
            for val in row.iter_mut() {
                *val = (*val - max).exp() / sum;
            }
        }
        x
    }

    pub fn train(&mut self, x: &Array2<f32>, y: &Vec<usize>, lr: f32, epochs: usize) {
        for _ in 0..epochs {
            let z1 = x.dot(&self.w1);
            let a1 = Self::relu(z1);
            let z2 = a1.dot(&self.w2);
            let a2 = Self::softmax(z2);

            let mut y_onehot = Array2::<f32>::zeros((y.len(), a2.shape()[1]));
            for (i, &label) in y.iter().enumerate() {
                y_onehot[[i, label]] = 1.0;
            }

            let dz2 = &a2 - &y_onehot;
            let dw2 = a1.t().dot(&dz2) / x.len() as f32;
            let da1 = dz2.dot(&self.w2.t());
            let dz1 = a1.mapv(|v| if v > 0.0 { 1.0 } else { 0.0 }) * da1;
            let dw1 = x.t().dot(&dz1) / x.len() as f32;

            self.w1 -= &(dw1 * lr);
            self.w2 -= &(dw2 * lr);
        }
    }

    pub fn predict(&self, x: &Array2<f32>) -> Vec<usize> {
        let a1 = Self::relu(x.dot(&self.w1));
        let a2 = Self::softmax(a1.dot(&self.w2));
        a2.outer_iter()
            .map(|row| row.iter().cloned().enumerate().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().0)
            .collect()
    }
}

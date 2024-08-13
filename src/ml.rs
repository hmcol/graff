use ndarray::{s, Array1, Array2};
use rand::Rng;

use crate::EvaluateOne;

pub struct NeuralNetwork {
    input_size: usize,
    hidden_size: usize,
    output_size: usize,
    weights_ih: Array2<f64>,
    weights_ho: Array2<f64>,
}

impl NeuralNetwork {
    pub fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut rng_fn = || rng.gen_range(-1.0..1.0);

        NeuralNetwork {
            input_size,
            hidden_size,
            output_size,
            // bias trick: last column is the bias
            weights_ih: Array2::from_shape_simple_fn((hidden_size, input_size + 1), &mut rng_fn),
            weights_ho: Array2::from_shape_simple_fn((output_size, hidden_size + 1), &mut rng_fn),
        }
    }

    pub fn forward(&self, input: &Array1<f64>) -> Array1<f64> {
        // add a 1 to end for bias trick
        let aug_input = augment(input);

        let hidden = self.weights_ih.dot(&aug_input);
        let hidden = hidden.map(|x| leaky_relu(*x));

        // add a 1 to end for bias trick
        let aug_hidden = augment(&hidden);

        let output = self.weights_ho.dot(&aug_hidden);
        // output.map(|x| leaky_relu(*x))
        output
    }

    pub fn backward(&mut self, input: &Array1<f64>, target: &Array1<f64>, learning_rate: f64) {
        // forward pass
        let aug_input = augment(input);
        let hidden = self.weights_ih.dot(&aug_input);
        let hidden_activated = hidden.map(|x| leaky_relu(*x));
        let aug_hidden = augment(&hidden_activated);
        let output = self.weights_ho.dot(&aug_hidden);
        let output_activated = output.map(|x| leaky_relu(*x));

        // compute error
        let output_error = target - &output_activated;
        let output_delta = &output_error * &output_activated.map(|x| 1.0);

        let hidden_error = self.weights_ho.t().slice(s![..-1, ..]).dot(&output_delta);
        let hidden_delta = &hidden_error * &hidden_activated.map(|x| leaky_relu_derivative(*x));

        // update weights
        for (i, delta) in output_delta.iter().enumerate() {
            self.weights_ho
                .row_mut(i)
                .zip_mut_with(&aug_hidden, |w, &h| {
                    *w += learning_rate * delta * h;
                });
        }

        for (i, delta) in hidden_delta.iter().enumerate() {
            self.weights_ih
                .row_mut(i)
                .zip_mut_with(&aug_input, |w, &inp| {
                    *w += learning_rate * delta * inp;
                });
        }
    }

    pub fn train_batch(
        &mut self,
        inputs: &Vec<Array1<f64>>,
        targets: &Vec<Array1<f64>>,
        learning_rate: f64,
    ) {
        let batch_size = inputs.len();

        let mut hidden_outs = Vec::with_capacity(batch_size);
        let mut output_outs = Vec::with_capacity(batch_size);

        // forward pass
        for input in inputs {
            let aug_input = augment(input);

            let hidden = self.weights_ih.dot(&aug_input);
            let hidden_activated = hidden.map(|x| leaky_relu(*x));
            hidden_outs.push(hidden_activated.clone());

            let aug_hidden = augment(&hidden_activated);
            let output = self.weights_ho.dot(&aug_hidden);
            let output_activated = output.map(|x| leaky_relu(*x));
            output_outs.push(output_activated.clone());
        }

        // compute error
        let output_errors: Vec<Array1<f64>> = targets
            .iter()
            .zip(output_outs.iter())
            .map(|(target, output_activated)| target - output_activated)
            .collect();

        let hidden_errors: Vec<Array1<f64>> = output_errors
            .iter()
            .zip(hidden_outs.iter())
            .map(|(output_error, hidden_output)| {
                let error = self.weights_ho.t().slice(s![..-1, ..]).dot(output_error);
                &error * &hidden_output.map(|x| leaky_relu_derivative(*x))
            })
            .collect();

        // update weights

        let mut weight_ho_update: Array2<f64> = Array2::zeros(self.weights_ho.raw_dim());
        let mut weight_ih_update: Array2<f64> = Array2::zeros(self.weights_ih.raw_dim());

        for ((input, hidden_output), (output_error, hidden_error)) in inputs
            .iter()
            .zip(hidden_outs.iter())
            .zip(output_errors.iter().zip(hidden_errors.iter()))
        {
            let aug_input = augment(input);
            let aug_hidden = augment(hidden_output);

            // update weights_ho
            for (i, delta) in output_error.iter().enumerate() {
                for (j, &h) in aug_hidden.iter().enumerate() {
                    weight_ho_update[[i, j]] += delta * h;
                }
            }

            // update weights_ih
            for (i, delta) in hidden_error.iter().enumerate() {
                for (j, &inp) in aug_input.iter().enumerate() {
                    weight_ih_update[[i, j]] += delta * inp;
                }
            }
        }

        // apply averaged updates
        self.weights_ho += &(weight_ho_update * (learning_rate / batch_size as f64));
        self.weights_ih += &(weight_ih_update * (learning_rate / batch_size as f64));
    }
}

fn leaky_relu(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        0.01 * x
    }
}

fn leaky_relu_derivative(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else {
        0.01
    }
}

fn augment(v: &Array1<f64>) -> Array1<f64> {
    Array1::from_iter(v.iter().cloned().chain(std::iter::once(1.0)))
}

impl EvaluateOne for NeuralNetwork {
    fn eval_one(&self, x: f64) -> f64 {
        let input = Array1::from(vec![x]);
        let output = self.forward(&input);
        output[0]
    }
}

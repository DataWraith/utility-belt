use ndarray::{Array1, Array2, Axis};
use rand::prelude::*;

const BATCH_SIZE: usize = 32;

pub type Dataset = (Array2<f64>, Array1<f64>);

/// MADGRAD optimizer (https://arxiv.org/abs/2101.11075)
///
/// This is useful for general curve fitting, when fit_quadratic() and fit_cubic()
/// are not applicable.
pub struct MADGRAD {
    initial_params: Array1<f64>,
    params: Array1<f64>,
    learning_rate: f64,
    momentum: f64,
    k: usize,
    eps: f64,
    grad_sum_sq: Array1<f64>,
    s: Array1<f64>,
}

impl MADGRAD {
    pub fn new(params: Array1<f64>, learning_rate: f64, momentum: f64, eps: f64) -> Self {
        let num_params = params.len();

        Self {
            initial_params: params.clone(),
            params,
            learning_rate,
            momentum,
            eps,
            grad_sum_sq: Array1::zeros(num_params),
            s: Array1::zeros(num_params),
            k: 0,
        }
    }

    fn madgrad(&mut self, grad: &Array1<f64>) {
        let lr = self.learning_rate + self.eps;
        let lamb = lr * ((self.k + 1) as f64).sqrt();

        self.grad_sum_sq = &self.grad_sum_sq + (grad * grad * lamb);
        let rms = self.grad_sum_sq.mapv(|x| x.powf(1.0 / 3.0)) + self.eps;
        self.s = &self.s + lamb * grad;

        let z = &self.initial_params - (&self.s / &rms);
        self.k += 1;

        self.params = self.params.clone() * self.momentum + &z * (1.0 - self.momentum);
    }

    /// Process one batch with mini-batch gradient descent.
    pub fn step<R: Rng>(&mut self, rng: &mut R, dataset: &Dataset) -> f64 {
        let (x_mat, y) = dataset;
        let w = &self.params;

        let batch_indices: Vec<usize> = (0..BATCH_SIZE.min(x_mat.shape()[0]))
            .map(|_| rng.gen_range(0..x_mat.shape()[0]))
            .collect();

        let batch = x_mat.select(Axis(0), &batch_indices);
        let batch_y = y.select(Axis(0), &batch_indices);

        let y_hat = batch.dot(w);
        let error = y_hat.clone() - batch_y;

        let w_grad = error.dot(&batch);

        self.madgrad(&w_grad);

        error.map(|x| x.powi(2)).sum() / BATCH_SIZE as f64
    }

    pub fn parameters(&self) -> &Array1<f64> {
        &self.params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toy_dataset() -> Dataset {
        let f = |x: f64| 0.17 * x * x * x * x - 0.4 * x * x * x + 0.9 * x * x + 0.31 * x - 1.11;
        let xs = (1..=100)
            .flat_map(|x| {
                let x = x as f64 / 100.0;
                [x * x * x * x, x * x * x, x * x, x, 1.0]
            })
            .collect::<Vec<f64>>();
        let ys = (1..=100).map(|x| f(x as f64 / 100.0)).collect::<Vec<f64>>();

        (
            Array2::from_shape_vec((100, 5), xs).unwrap(),
            Array1::from_shape_vec(100, ys).unwrap(),
        )
    }

    fn train_madgrad(dataset: &Dataset) -> Array1<f64> {
        let mut rng: StdRng = SeedableRng::seed_from_u64(1);
        let mut weight_init = || rng.gen_range((-1.0 / 1_000.0)..(1.0 / 1_000.0));

        let weights = (0..dataset.0.shape()[1])
            .map(|_| weight_init())
            .collect::<Vec<f64>>();

        let w: Array1<f64> = Array1::from_shape_vec(dataset.0.shape()[1], weights).unwrap();

        let mut madgrad = MADGRAD::new(w.clone(), 0.001, 0.9, 1e-6);

        for _ in 0..1000 {
            let _loss = madgrad.step(&mut rng, dataset);
        }

        madgrad.parameters().clone()
    }

    #[test]
    fn madgrad_works() {
        let dataset = toy_dataset();

        let params = train_madgrad(&dataset);
        let errors = dataset.0.dot(&params) - dataset.1;
        let avg_error = errors.sum() / dataset.0.shape()[0] as f64;

        assert!(avg_error < 0.001);
    }
}

use std::time::Duration;

use rand_distr::{Distribution, Normal};

pub struct Network {
    rng: Normal<f64>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            rng: Normal::new(1_000.0, 100.0).unwrap(),
        }
    }

    /// Simulate network delay.
    pub async fn traverse(&self) {
        let d = self.rng.sample(&mut rand::thread_rng()) as u64;
        let delta = Duration::from_micros(500 + d);
        tokio::time::sleep(delta).await;
    }
}

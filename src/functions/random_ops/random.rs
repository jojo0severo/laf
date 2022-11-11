use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

use crate::types::field::Field;

struct Random {}

impl Random {
    pub fn new() -> Random {
        return Random {};
    }
}

impl Field for Random {
    type Output = f64;

    fn get_value(&self) -> Self::Output {
        let mut generator = thread_rng();
        let distribution = Uniform::from(0.0..1.0);
        return distribution.sample(&mut generator);
    }
}

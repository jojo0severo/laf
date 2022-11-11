use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

use crate::types::field::Field;

struct RandRange<T, U> {
    a: T,
    b: U,
}

impl<K, T, U> RandRange<T, U>
where
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> RandRange<T, U> {
        return RandRange { a, b };
    }
}

impl<K, T, U> Field for RandRange<T, U>
where
    K: SampleUniform,
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    type Output = K;

    fn get_value(&self) -> Self::Output {
        let mut generator = thread_rng();
        let distribution = Uniform::from(self.a.get_value()..self.b.get_value());
        return distribution.sample(&mut generator);
    }
}

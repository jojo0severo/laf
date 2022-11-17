use num_traits::{cast, Num, NumCast, One};
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

use crate::types::field::Field;

pub struct RandRange<T, U> {
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
    K: Num + NumCast + PartialOrd + SampleUniform,
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    type Output = K;

    fn get_value(&self) -> Self::Output {
        let first = self.a.get_value();
        let second = self.b.get_value();

        let distribution: Uniform<K>;
        if first > second {
            distribution = Uniform::from(second..first);
        } else if first == second {
            let one = cast::<u8, K>(1).unwrap();
            distribution = Uniform::from(first..second + one);
        } else {
            distribution = Uniform::from(first..second);
        }

        let mut generator = thread_rng();
        return distribution.sample(&mut generator);
    }
}

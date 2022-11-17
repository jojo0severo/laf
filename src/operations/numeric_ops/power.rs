use num_traits::{cast, checked_pow, CheckedMul, Num, NumCast};

use crate::types::field::Field;

pub struct Power<T, U> {
    a: T,
    b: U,
}

impl<K, T, U> Power<T, U>
where
    K: Num,
    T: Field<Output = K>,
    U: Field<Output = usize>,
{
    pub fn new(a: T, b: U) -> Power<T, U> {
        return Power { a, b };
    }
}

impl<K, T, U> Field for Power<T, U>
where
    K: Num + Clone + CheckedMul + NumCast,
    T: Field<Output = K>,
    U: Field<Output = usize>,
{
    type Output = K;

    fn get_value(&self) -> Self::Output {
        let res = checked_pow(self.a.get_value(), self.b.get_value());
        return match res {
            Option::Some(v) => v,
            Option::None => cast::<u8, K>(0).unwrap(),
        };
    }
}

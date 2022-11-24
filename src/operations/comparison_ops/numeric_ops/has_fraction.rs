use num_traits::{cast, real::Real};

use crate::types::Field;

pub struct HasFraction<T> {
    value: T,
}

impl<K, T> HasFraction<T>
where
    K: Real,
    T: Field<Output = K>,
{
    pub fn new(value: T) -> HasFraction<T> {
        return HasFraction { value };
    }
}

impl<K, T> Field for HasFraction<T>
where
    K: Real,
    T: Field<Output = K>,
{
    type Output = bool;

    fn get_value(&self) -> Self::Output {
        let zero = cast::<u8, K>(0).unwrap();
        return self.value.get_value().fract() != zero;
    }
}

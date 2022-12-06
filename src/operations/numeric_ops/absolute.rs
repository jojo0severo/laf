use num_traits::Signed;

use crate::types::Field;

pub struct Absolute<T> {
    value: T,
}

impl<T, K> Absolute<T>
where
    K: Signed,
    T: Field<Output = K>,
{
    fn new(value: T) -> Absolute<T> {
        return Absolute { value };
    }
}

impl<K, T> Field for Absolute<T>
where
    K: Signed,
    T: Field<Output = K>,
{
    type Output = K;

    fn get_value(&self) -> Self::Output {
        return self.value.get_value().abs();
    }
}

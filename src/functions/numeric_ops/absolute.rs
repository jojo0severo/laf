use num_traits::Signed;

use crate::types::field::Field;

pub struct Absolute<T> {
    value: T,
}

impl<K, T> Absolute<T>
where
    K: Signed,
    T: Field<Output = K>,
{
    pub fn new(value: T) -> Absolute<T> {
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

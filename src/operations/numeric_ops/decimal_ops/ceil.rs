use num_traits::real::Real;

use crate::types::Field;

pub struct Ceil<T> {
    value: T,
}

impl<K, T> Ceil<T>
where
    K: Real,
    T: Field<Output = K>,
{
    pub fn new(value: T) -> Ceil<T> {
        return Ceil { value };
    }
}

impl<K, T> Field for Ceil<T>
where
    K: Real,
    T: Field<Output = K>,
{
    type Output = K;

    fn get_value(&self) -> Self::Output {
        return self.value.get_value().ceil();
    }
}

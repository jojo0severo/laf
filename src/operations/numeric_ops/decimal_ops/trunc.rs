use num_traits::real::Real;

use crate::types::Field;

pub struct Trunc<T> {
    value: T,
}

impl<K, T> Trunc<T>
where
    K: Real,
    T: Field<Output = K>,
{
    pub fn new(value: T) -> Trunc<T> {
        return Trunc { value };
    }
}

impl<K, T> Field for Trunc<T>
where
    K: Real,
    T: Field<Output = K>,
{
    type Output = K;

    fn get_value(&self) -> Self::Output {
        return self.value.get_value().trunc();
    }
}

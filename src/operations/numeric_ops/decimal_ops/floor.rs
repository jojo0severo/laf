use num_traits::real::Real;

use crate::types::Field;

pub struct Floor<T> {
    value: T,
}

impl<K, T> Floor<T>
where
    K: Real,
    T: Field<Output = K>,
{
    pub fn new(value: T) -> Floor<T> {
        return Floor { value };
    }
}

impl<K, T> Field for Floor<T>
where
    K: Real,
    T: Field<Output = K>,
{
    type Output = K;

    fn get_value(&self) -> Self::Output {
        return self.value.get_value().floor();
    }
}

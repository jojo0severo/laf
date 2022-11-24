use num_traits::real::Real;

use crate::types::Field;

pub struct Round<T> {
    value: T,
}

impl<K, T> Round<T>
where
    K: Real,
    T: Field<Output = K>,
{
    pub fn new(value: T) -> Round<T> {
        return Round { value };
    }
}

impl<K, T> Field for Round<T>
where
    K: Real,
    T: Field<Output = K>,
{
    type Output = K;

    fn get_value(&self) -> Self::Output {
        return self.value.get_value().round();
    }
}

use num_traits::real::Real;

use crate::types::field::Field;

pub struct SquareRoot<T> {
    value: T,
}

impl<K, T> SquareRoot<T>
where
    K: Real,
    T: Field<Output = K>,
{
    pub fn new(value: T) -> SquareRoot<T> {
        return SquareRoot { value };
    }
}

impl<K, T> Field for SquareRoot<T>
where
    K: Real,
    T: Field<Output = K>,
{
    type Output = K;

    fn get_value(&self) -> Self::Output {
        return self.value.get_value().sqrt();
    }
}

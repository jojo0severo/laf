use num_traits::real::Real;

use crate::types::Field;

pub struct Fraction<T> {
    value: T,
}

impl<K, T> Fraction<T>
where
    K: Real,
    T: Field<Output = K>,
{
    pub fn new(value: T) -> Fraction<T> {
        return Fraction { value };
    }
}

impl<K, T> Field for Fraction<T>
where
    K: Real,
    T: Field<Output = K>,
{
    type Output = K;

    fn get_value(&self) -> Self::Output {
        return self.value.get_value().fract();
    }
}

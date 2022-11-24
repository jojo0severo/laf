use std::ops::Rem;

use crate::types::Field;

pub struct Modulus<T, U> {
    a: T,
    b: U,
}

impl<K, T, U> Modulus<T, U>
where
    K: Rem<Output = K>,
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> Modulus<T, U> {
        return Modulus { a, b };
    }
}

impl<K, T, U> Field for Modulus<T, U>
where
    K: Rem<Output = K>,
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    type Output = K;

    fn get_value(&self) -> Self::Output {
        return self.a.get_value() % self.b.get_value();
    }
}

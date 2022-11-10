use std::ops::Sub;

use crate::types::field::Field;

pub struct Subtract<T, U> {
    a: T,
    b: U,
}

impl<K, T, U> Subtract<T, U>
where
    K: Sub<Output = K>,
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> Subtract<T, U> {
        return Subtract { a, b };
    }
}

impl<K, T, U> Field for Subtract<T, U>
where
    K: Sub<Output = K>,
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    type Output = K;

    fn get_value(&self) -> Self::Output {
        return self.a.get_value() - self.b.get_value();
    }
}

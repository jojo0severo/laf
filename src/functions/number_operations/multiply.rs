use std::ops::Mul;

use crate::types::field::Field;

pub struct Multiply<T, U> {
    a: T,
    b: U,
}

impl<K, T, U> Multiply<T, U>
where
    K: Mul<Output = K>,
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> Multiply<T, U> {
        return Multiply { a, b };
    }
}

impl<K, T, U> Field for Multiply<T, U>
where
    K: Mul<Output = K>,
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    type Output = K;

    fn get_value(&self) -> Self::Output {
        return self.a.get_value() * self.b.get_value();
    }
}

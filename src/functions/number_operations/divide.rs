use std::ops::Div;

use crate::types::field::Field;

pub struct Divide<T, U> {
    a: T,
    b: U,
}

impl<K, T, U> Divide<T, U>
where
    K: Div<Output = K>,
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> Divide<T, U> {
        return Divide { a, b };
    }
}

impl<K, T, U> Field for Divide<T, U>
where
    K: Div<Output = K>,
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    type Output = K;

    fn get_value(&self) -> Self::Output {
        return self.a.get_value() / self.b.get_value();
    }
}

use std::ops::Add;

use crate::types::Field;

pub struct Sum<T, U> {
    a: T,
    b: U,
}

impl<K, T, U> Sum<T, U>
where
    K: Add<Output = K>,
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> Sum<T, U> {
        return Sum { a, b };
    }
}

impl<K, T, U> Field for Sum<T, U>
where
    K: Add<Output = K>,
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    type Output = K;

    fn get_value(&self) -> Self::Output {
        return self.a.get_value() + self.b.get_value();
    }
}

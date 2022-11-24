use std::marker::PhantomData;

use crate::types::Field;

pub struct EqualsIgnoreCase<K, T, U> {
    a: T,
    b: U,
    c: PhantomData<K>,
}

impl<T, U> EqualsIgnoreCase<String, T, U>
where
    T: Field<Output = String>,
    U: Field<Output = String>,
{
    pub fn new(a: T, b: U) -> EqualsIgnoreCase<String, T, U> {
        return EqualsIgnoreCase {
            a,
            b,
            c: PhantomData,
        };
    }
}

impl<T, U> EqualsIgnoreCase<Vec<char>, T, U>
where
    T: Field<Output = Vec<char>>,
    U: Field<Output = Vec<char>>,
{
    pub fn new(a: T, b: U) -> EqualsIgnoreCase<Vec<char>, T, U> {
        return EqualsIgnoreCase {
            a,
            b,
            c: PhantomData,
        };
    }
}

impl<T, U> Field for EqualsIgnoreCase<String, T, U>
where
    T: Field<Output = String>,
    U: Field<Output = String>,
{
    type Output = bool;

    fn get_value(&self) -> Self::Output {
        let a = self.a.get_value();
        let b = self.b.get_value();

        return a.len() == b.len() && a.eq_ignore_ascii_case(b.as_str());
    }
}

impl<T, U> Field for EqualsIgnoreCase<Vec<char>, T, U>
where
    T: Field<Output = Vec<char>>,
    U: Field<Output = Vec<char>>,
{
    type Output = bool;

    fn get_value(&self) -> Self::Output {
        let a = self.a.get_value();
        let b = self.b.get_value();

        return a.len() == b.len() && a.iter().zip(&b).all(|(a, b)| a.eq_ignore_ascii_case(b));
    }
}

use crate::types::field::Field;

pub struct StartsWith<T, U> {
    a: T,
    b: U,
}

impl<K, T, U> StartsWith<T, U>
where
    T: Field<Output = Vec<K>>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> StartsWith<T, U> {
        return StartsWith { a, b };
    }
}

impl<K, T, U> Field for StartsWith<T, U>
where
    K: Ord,
    T: Field<Output = Vec<K>>,
    U: Field<Output = K>,
{
    type Output = bool;

    fn get_value(&self) -> Self::Output {
        let container = self.a.get_value();
        let key = self.b.get_value();

        match container.first() {
            Some(val) => return val == &key,
            None => return false,
        }
    }
}

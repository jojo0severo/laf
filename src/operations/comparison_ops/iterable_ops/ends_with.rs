use crate::types::field::Field;

pub struct EndsWith<T, U> {
    a: T,
    b: U,
}

impl<K, T, U> EndsWith<T, U>
where
    T: Field<Output = Vec<K>>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> EndsWith<T, U> {
        return EndsWith { a, b };
    }
}

impl<K, T, U> Field for EndsWith<T, U>
where
    K: Ord,
    T: Field<Output = Vec<K>>,
    U: Field<Output = K>,
{
    type Output = bool;

    fn get_value(&self) -> Self::Output {
        let container = self.a.get_value();
        let key = self.b.get_value();

        match container.last() {
            Some(val) => return val == &key,
            None => return false,
        }
    }
}

use crate::types::field::Field;

pub struct Contains<T, U> {
    a: T,
    b: U,
}

impl<K, T, U> Contains<T, U>
where
    T: Field<Output = Vec<K>>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> Contains<T, U> {
        return Contains { a, b };
    }
}

impl<K, T, U> Field for Contains<T, U>
where
    K: Ord,
    T: Field<Output = Vec<K>>,
    U: Field<Output = K>,
{
    type Output = bool;

    fn get_value(&self) -> Self::Output {
        let container = self.a.get_value();
        let key = self.b.get_value();

        match container.binary_search(&key) {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }
}

use crate::types::Field;

pub struct DropValue<T, U> {
    a: T,
    b: U,
}

impl<K, T, U> DropValue<T, U>
where
    T: Field<Output = Vec<K>>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> DropValue<T, U> {
        return DropValue { a, b };
    }
}

impl<K, T, U> Field for DropValue<T, U>
where
    K: Ord,
    T: Field<Output = Vec<K>>,
    U: Field<Output = K>,
{
    type Output = Vec<K>;

    fn get_value(&self) -> Self::Output {
        let mut vector = self.a.get_value();
        let element = self.b.get_value();
        if let Ok(index) = vector.binary_search(&element) {
            vector.remove(index);
        }
        return vector;
    }
}

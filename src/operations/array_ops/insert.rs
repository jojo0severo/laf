use crate::types::Field;

pub struct Insert<T, U, V> {
    a: T,
    b: U,
    index: V,
}

impl<K, H, T, U, V> Insert<T, U, V>
where
    H: Into<usize>,
    T: Field<Output = Vec<K>>,
    U: Field<Output = K>,
    V: Field<Output = H>,
{
    pub fn new(a: T, b: U, index: V) -> Insert<T, U, V> {
        return Insert { a, b, index };
    }
}

impl<K, H, T, U, V> Field for Insert<T, U, V>
where
    H: Into<usize>,
    T: Field<Output = Vec<K>>,
    U: Field<Output = K>,
    V: Field<Output = H>,
{
    type Output = Vec<K>;

    fn get_value(&self) -> Self::Output {
        let mut vector = self.a.get_value();
        let index: usize = self.index.get_value().into();
        if index < vector.len() {
            vector.reserve_exact(1);
            let element = self.b.get_value();
            vector.insert(index, element);
        }

        return vector;
    }
}

use crate::types::Field;

pub struct DropIndex<T, U> {
    a: T,
    b: U,
}

impl<K, H, T, U> DropIndex<T, U>
where
    H: Into<usize>,
    T: Field<Output = Vec<K>>,
    U: Field<Output = H>,
{
    pub fn new(a: T, b: U) -> DropIndex<T, U> {
        return DropIndex { a, b };
    }
}

impl<K, H, T, U> Field for DropIndex<T, U>
where
    H: Into<usize>,
    T: Field<Output = Vec<K>>,
    U: Field<Output = H>,
{
    type Output = Vec<K>;

    fn get_value(&self) -> Self::Output {
        let mut vector = self.a.get_value();
        let index: usize = self.b.get_value().into();
        if index < vector.len() {
            vector.swap_remove(index);
        }
        return vector;
    }
}

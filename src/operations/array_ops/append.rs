use crate::types::Field;

pub struct Append<T, U> {
    a: T,
    b: U,
}

impl<K, T, U> Append<T, U>
where
    T: Field<Output = Vec<K>>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> Append<T, U> {
        return Append { a, b };
    }
}

impl<K, T, U> Field for Append<T, U>
where
    T: Field<Output = Vec<K>>,
    U: Field<Output = K>,
{
    type Output = Vec<K>;

    fn get_value(&self) -> Self::Output {
        let mut vector = self.a.get_value();
        vector.reserve_exact(1);
        vector.push(self.b.get_value());
        return vector;
    }
}

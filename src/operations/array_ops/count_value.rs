use crate::types::Field;

pub struct CountValue<T, U> {
    a: T,
    b: U,
}

impl<K, T, U> CountValue<T, U>
where
    T: Field<Output = Vec<K>>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> CountValue<T, U> {
        return CountValue { a, b };
    }
}

impl<K, T, U> Field for CountValue<T, U>
where
    K: PartialEq,
    T: Field<Output = Vec<K>>,
    U: Field<Output = K>,
{
    type Output = u128;

    fn get_value(&self) -> Self::Output {
        let a = self.a.get_value();
        let b = self.b.get_value();

        let mut count: u128 = 0;
        for element in a.as_slice() {
            if element == &b {
                count += 1;
            }
        }

        return count;
    }
}

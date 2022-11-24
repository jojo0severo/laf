use crate::types::Field;

pub struct Last<T, U> {
    a: T,
    b: U,
}

impl<K, T, U> Last<T, U>
where
    T: Field<Output = Vec<K>>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> Last<T, U> {
        return Last { a, b };
    }
}

impl<K, T, U> Field for Last<T, U>
where
    K: PartialEq,
    T: Field<Output = Vec<K>>,
    U: Field<Output = K>,
{
    type Output = isize;

    fn get_value(&self) -> Self::Output {
        let a = self.a.get_value();
        let b = self.b.get_value();

        for (index, element) in a.iter().rev().enumerate() {
            if element == &b {
                return index as isize;
            }
        }

        return -1;
    }
}

use num_traits::PrimInt;

use crate::types::Field;

pub struct DropRange<T, U> {
    a: T,
    b: U,
}

impl<K, H, T, U> DropRange<T, U>
where
    H: PrimInt,
    H: Into<usize>,
    T: Field<Output = Vec<K>>,
    U: Field<Output = Vec<H>>,
{
    pub fn new(a: T, b: U) -> DropRange<T, U> {
        return DropRange { a, b };
    }
}

impl<K, H, T, U> Field for DropRange<T, U>
where
    H: PrimInt,
    for<'a> &'a H: Into<&'a usize>,
    T: Field<Output = Vec<K>>,
    U: Field<Output = Vec<H>>,
{
    type Output = Vec<K>;

    fn get_value(&self) -> Self::Output {
        let mut vector = self.a.get_value();
        let indexes = self.b.get_value();
        for index in indexes.as_slice() {
            if *index.into() < vector.len() {
                vector.swap_remove(*index.into());
            }
        }
        return vector;
    }
}

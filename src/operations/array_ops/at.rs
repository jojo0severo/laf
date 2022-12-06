use num_traits::PrimInt;

use crate::types::Field;

pub struct At<T, U> {
    a: T,
    b: U,
}

impl<K, H, T, U> At<T, U>
where
    H: PrimInt,
    H: Into<usize>,
    T: Field<Output = Vec<K>>,
    U: Field<Output = H>,
{
    pub fn new(a: T, b: U) -> At<T, U> {
        return At { a, b };
    }
}

impl<K, H, T, U> Field for At<T, U>
where
    H: PrimInt,
    H: Into<usize>,
    K: Default,
    T: Field<Output = Vec<K>>,
    U: Field<Output = H>,
{
    type Output = K;

    fn get_value(&self) -> Self::Output {
        let mut a = self.a.get_value();
        let b: usize = self.b.get_value().into();
        if b < a.len() {
            return a.swap_remove(b);
        } else {
            return K::default();
        }
    }
}

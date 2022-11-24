use std::marker::PhantomData;

use crate::types::Field;

pub struct Equal<K, T, U> {
    a: T,
    b: U,
    c: PhantomData<K>,
}

impl<K, T, U> Equal<K, T, U>
where
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> Equal<K, T, U> {
        return Equal {
            a,
            b,
            c: PhantomData,
        };
    }
}

crate::comparable_values!(
    Equal eq
    i128 i64 i32 i8 isize
    usize u8 u32 u64 u128
    f32 f64 String bool char
);

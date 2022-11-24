use std::marker::PhantomData;

use crate::types::Field;

pub struct Smaller<K, T, U> {
    a: T,
    b: U,
    c: PhantomData<K>,
}

impl<K, T, U> Smaller<K, T, U>
where
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> Smaller<K, T, U> {
        return Smaller {
            a,
            b,
            c: PhantomData,
        };
    }
}

pub struct SmallerOrEqual<K, T, U> {
    a: T,
    b: U,
    c: PhantomData<K>,
}

impl<K, T, U> SmallerOrEqual<K, T, U>
where
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> Smaller<K, T, U> {
        return Smaller {
            a,
            b,
            c: PhantomData,
        };
    }
}

crate::comparable_values!(
    Smaller lt
    i128 i64 i32 i8 isize
    usize u8 u32 u64 u128
    f32 f64 String bool char
);

crate::comparable_values!(
    SmallerOrEqual le
    i128 i64 i32 i8 isize
    usize u8 u32 u64 u128
    f32 f64 String bool char
);

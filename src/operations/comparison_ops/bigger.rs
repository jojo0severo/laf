use std::marker::PhantomData;

use crate::types::Field;

pub struct Bigger<K, T, U> {
    a: T,
    b: U,
    c: PhantomData<K>,
}

impl<K, T, U> Bigger<K, T, U>
where
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> Bigger<K, T, U> {
        return Bigger {
            a,
            b,
            c: PhantomData,
        };
    }
}

pub struct BiggerOrEqual<K, T, U> {
    a: T,
    b: U,
    c: PhantomData<K>,
}

impl<K, T, U> BiggerOrEqual<K, T, U>
where
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> Bigger<K, T, U> {
        return Bigger {
            a,
            b,
            c: PhantomData,
        };
    }
}

crate::comparable_values!(
    Bigger gt
    i128 i64 i32 i8 isize
    usize u8 u32 u64 u128
    f32 f64 String bool char
);

crate::comparable_values!(
    BiggerOrEqual ge
    i128 i64 i32 i8 isize
    usize u8 u32 u64 u128
    f32 f64 String bool char
);

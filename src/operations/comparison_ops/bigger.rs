use crate::types::field::Field;

pub struct Bigger<K, T, U>
where
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    a: T,
    b: U,
}

impl<K, T, U> Bigger<K, T, U>
where
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> Bigger<K, T, U> {
        return Bigger { a, b };
    }
}

pub struct BiggerOrEqual<K, T, U>
where
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    a: T,
    b: U,
}

impl<K, T, U> BiggerOrEqual<K, T, U>
where
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> Bigger<K, T, U> {
        return Bigger { a, b };
    }
}

crate::comparable_values!(
    Bigger gt
    i128 i64 i32 i8 isize
    usize u8 u32 u64 u128
    f32 f64 String bool char
);

crate::comparable_values!(
    Bigger gt
    Vec<i128> Vec<i64> Vec<i32> Vec<i8> Vec<isize>
    Vec<usize> Vec<u8> Vec<u32> Vec<u64> Vec<u128>
    Vec<f32> Vec<f64> Vec<char> Vec<bool> Vec<String>
);
crate::comparable_values!(
    BiggerOrEqual ge
    i128 i64 i32 i8 isize
    usize u8 u32 u64 u128
    f32 f64 String bool char
);

crate::comparable_values!(
    BiggerOrEqual ge
    Vec<i128> Vec<i64> Vec<i32> Vec<i8> Vec<isize>
    Vec<usize> Vec<u8> Vec<u32> Vec<u64> Vec<u128>
    Vec<f32> Vec<f64> Vec<char> Vec<bool> Vec<String>
);

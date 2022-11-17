use crate::types::field::Field;

pub struct Smaller<K, T, U>
where
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    a: T,
    b: U,
}

impl<K, T, U> Smaller<K, T, U>
where
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> Smaller<K, T, U> {
        return Smaller { a, b };
    }
}

pub struct SmallerOrEqual<K, T, U>
where
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    a: T,
    b: U,
}

impl<K, T, U> SmallerOrEqual<K, T, U>
where
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> Smaller<K, T, U> {
        return Smaller { a, b };
    }
}

crate::comparable_values!(
    Smaller lt
    i128 i64 i32 i8 isize
    usize u8 u32 u64 u128
    f32 f64 String bool char
);

crate::comparable_values!(
    Smaller lt
    Vec<i128> Vec<i64> Vec<i32> Vec<i8> Vec<isize>
    Vec<usize> Vec<u8> Vec<u32> Vec<u64> Vec<u128>
    Vec<f32> Vec<f64> Vec<char> Vec<bool> Vec<String>
);
crate::comparable_values!(
    SmallerOrEqual le
    i128 i64 i32 i8 isize
    usize u8 u32 u64 u128
    f32 f64 String bool char
);

crate::comparable_values!(
    SmallerOrEqual le
    Vec<i128> Vec<i64> Vec<i32> Vec<i8> Vec<isize>
    Vec<usize> Vec<u8> Vec<u32> Vec<u64> Vec<u128>
    Vec<f32> Vec<f64> Vec<char> Vec<bool> Vec<String>
);

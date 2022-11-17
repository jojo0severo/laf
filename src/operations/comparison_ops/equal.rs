use crate::types::field::Field;

pub struct Equal<K, T, U>
where
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    a: T,
    b: U,
}

impl<K, T, U> Equal<K, T, U>
where
    T: Field<Output = K>,
    U: Field<Output = K>,
{
    pub fn new(a: T, b: U) -> Equal<K, T, U> {
        return Equal { a, b };
    }
}

crate::comparable_values!(
    Equal eq
    i128 i64 i32 i8 isize
    usize u8 u32 u64 u128
    f32 f64 String bool char
);

crate::comparable_values!(
    Equal eq
    Vec<i128> Vec<i64> Vec<i32> Vec<i8> Vec<isize>
    Vec<usize> Vec<u8> Vec<u32> Vec<u64> Vec<u128>
    Vec<f32> Vec<f64> Vec<char> Vec<bool> Vec<String>
);

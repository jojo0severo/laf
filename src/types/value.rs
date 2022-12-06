use std::cell::Cell;
use std::vec::Vec;

use super::field::Field;

pub struct Value<'a, T> {
    value: &'a Cell<T>,
}

impl<'a, T> Value<'a, T> {
    pub fn new(value: &'a Cell<T>) -> Value<'a, T> {
        return Value { value };
    }
}

impl<'a> Field for Value<'a, String> {
    type Output = String;

    fn get_value(&self) -> Self::Output {
        let temp = self.value.take();
        self.value.set(temp.clone());
        return temp;
    }
}

impl<'a> Field for Value<'a, Vec<String>> {
    type Output = Vec<String>;

    fn get_value(&self) -> Self::Output {
        let temp = self.value.take();
        self.value.set(temp.clone());
        return temp;
    }
}

macro_rules! copiable_types {
    ($var:ident) => {
        impl<'a> Field for Value<'a, $var> {
            type Output = $var;

            fn get_value(&self) -> Self::Output {
                return self.value.get();
            }
        }

        impl<'a> Field for Value<'a, Vec<$var>>{
            type Output = Vec<$var>;

            fn get_value(&self) -> Self::Output {
                let temp = self.value.take();
                self.value.set(temp.clone());
                return temp;
            }
        }
    };

    ($var:ident $($more:ident)+) => {
        copiable_types! { $var }
        copiable_types! { $($more)+ }
    };
}

copiable_types!(
    i128 i64 i32 i8 isize
    usize u8 u32 u64 u128
    f32 f64 char bool
);

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

macro_rules! copiable_types {
    ($var:ident) => {
        impl<'a> Field for Value<'a, $var> {
            type Output = $var;

            fn get_value(&self) -> Self::Output {
                return self.value.get();
            }
        }
    };

    ($var:ident $($more:ident)+) => {
        copiable_types! { $var }
        copiable_types! { $($more)+ }
    };
}

macro_rules! clonable_types {
    ($var:ident) => {
        impl<'a> Field for Value<'a, $var> {
            type Output = $var;

            fn get_value(&self) -> Self::Output {
                let temp = self.value.take();
                self.value.set(temp.clone());
                return temp;
            }
        }
    };

    ($var:ident $($more:ident)+) => {
        clonable_types! { $var }
        clonable_types! { $($more)+ }
    };

    (Vec<$var:ident>) => {
        impl<'a> Field for Value<'a, Vec<$var>> {
            type Output = Vec<$var>;

            fn get_value(&self) -> Self::Output {
                let temp = self.value.take();
                self.value.set(temp.clone());
                return temp;
            }
        }
    };

    (Vec<$var:ident> $(Vec<$more:ident>)+) => {
        clonable_types! { Vec<$var> }
        clonable_types! { $(Vec<$more>)+ }
    };
}

copiable_types!(i128 i64 i32 i8 isize usize u8 u32 u64 u128 f32 f64 char bool);
clonable_types!(String);

// vector types are only for testing
clonable_types!(
    Vec<i128> Vec<i64> Vec<i32> Vec<i8> Vec<isize>
    Vec<usize> Vec<u8> Vec<u32> Vec<u64> Vec<u128>
    Vec<f32> Vec<f64> Vec<char> Vec<bool> Vec<String>
);

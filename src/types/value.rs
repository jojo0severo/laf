use std::cell::Cell;

use super::field::Field;

pub struct Value<'a, T>
where
    T: Copy,
{
    value: &'a Cell<T>,
}

impl<'a, T> Value<'a, T>
where
    T: Copy,
{
    pub fn new(value: &'a Cell<T>) -> Value<'a, T> {
        return Value { value };
    }
}

impl<'a, T> Field for Value<'a, T>
where
    T: Copy,
{
    type Output = T;

    fn get_value(&self) -> Self::Output {
        return (*self.value).get();
    }
}

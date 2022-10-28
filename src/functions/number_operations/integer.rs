pub struct Integer {
    value: i128,
}

impl super::super::field::ValueField<i128> for Integer {}

impl super::super::field::Field<i128> for Integer {
    fn get_value(&mut self) -> i128 {
        return self.value;
    }
}

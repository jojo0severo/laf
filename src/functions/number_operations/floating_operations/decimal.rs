use super::super::super::field::{Field, ValueField};

pub struct Decimal<const INT_SIZE: usize, const FRACT_SIZE: usize> {
    integer: [char; INT_SIZE],
    fraction: [char; FRACT_SIZE],
    cache_value: Option<f64>,
}

impl<const INT_SIZE: usize, const FRACT_SIZE: usize> Decimal<INT_SIZE, FRACT_SIZE> {
    pub fn new(
        integer: [char; INT_SIZE],
        fraction: [char; FRACT_SIZE],
    ) -> Decimal<INT_SIZE, FRACT_SIZE> {
        return Decimal {
            integer,
            fraction,
            cache_value: None,
        };
    }

    fn get_whole_integer(&self) -> f64 {
        let last = self.integer.len();
        if last == 0 {
            return 0.0;
        }

        let mut decimal_acc: f64 = 1f64;
        let mut integer: f64 = (self.integer[last - 1] as u32 - '0' as u32).into();

        for i in (0..last - 1).rev() {
            let current_char: f64 = (self.integer[i] as u32 - '0' as u32).into();

            decimal_acc *= 10f64;
            integer += current_char * decimal_acc;
        }

        return integer;
    }

    fn get_whole_fraction(&self) -> f64 {
        let last = self.fraction.len();
        if last == 0 {
            return 0.0;
        }

        let mut decimal_acc: f64 = 10f64;
        let mut fraction: f64 = (self.fraction[last - 1] as u32 - '0' as u32).into();

        for i in (0..last - 1).rev() {
            let current_char: f64 = (self.fraction[i] as u32 - '0' as u32).into();

            fraction += current_char * decimal_acc;
            decimal_acc *= 10f64;
        }

        return fraction / decimal_acc;
    }
}

impl<const INT_SIZE: usize, const FRACT_SIZE: usize> Field<f64> for Decimal<INT_SIZE, FRACT_SIZE> {
    fn get_value(&mut self) -> f64 {
        return match self.cache_value {
            Some(v) => v,
            None => *self
                .cache_value
                .insert(self.get_whole_integer() + self.get_whole_fraction()),
        };
    }
}

impl<const INT_SIZE: usize, const FRACT_SIZE: usize> ValueField<f64>
    for Decimal<INT_SIZE, FRACT_SIZE>
{
}

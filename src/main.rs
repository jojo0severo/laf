mod functions;
mod types;

use std::cell::Cell;

use crate::types::field::Field;

fn main() {
    use functions::numeric_ops::{
        absolute::Absolute, ceil::Ceil, divide::Divide, floor::Floor, fraction::Fraction,
        modulus::Modulus, multiply::Multiply, power::Power, round::Round, square_root::SquareRoot,
        subtract::Subtract, sum::Sum, trunc::Trunc,
    };
    use types::value::Value;

    let c = Cell::new(10.0);
    let k = Cell::new(10.0);
    let floating_cell = Cell::new(1.6);
    let usize_cell = Cell::new(3usize);

    let a = Value::new(&c);
    let z = Value::new(&k);
    let o = Value::new(&c);
    let w = Value::new(&k);
    let q = Value::new(&k);
    let floating = Value::new(&floating_cell);
    let usizeV = Value::new(&usize_cell);

    let b = Sum::new(a, z);
    let j = Fraction::new(floating);

    let y = Sum::new(b, q);

    println!("J: {}", j.get_value());
    println!("Y: {}", y.get_value());
    c.set(20.0);
    println!("J: {}", j.get_value());
    println!("Y: {}", y.get_value());
}

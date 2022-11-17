mod operations;
mod types;

use std::cell::Cell;

use crate::types::field::Field;

fn main() {
    use operations::comparison_ops::HasFraction;
    use types::value::Value;

    let integer_cell = Cell::new(10);
    let floating_cell = Cell::new(1.6);
    let usize_cell = Cell::new(3usize);
    let string_cell = Cell::new(String::from("batata"));
    let vec_cell = Cell::new(vec![1, 2]);

    let a = Value::new(&integer_cell);
    let b = Value::new(&floating_cell);
    let c = Value::new(&usize_cell);
    let d = Value::new(&string_cell);
    let e = Value::new(&vec_cell);
    println!("{}", a.get_value());
    println!("{}", b.get_value());
    println!("{}", c.get_value());
    println!("{}", d.get_value());

    let f = HasFraction::new(b);
    println!("{}", f.get_value());
}

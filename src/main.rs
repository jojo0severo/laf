mod operations;
mod types;

use std::cell::Cell;

use crate::types::Field;
use crate::types::{convert_string, Type};

fn main() {
    use types::Value;

    use operations::array_ops::Append;

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
    (e.get_value() as Vec<i32>)
        .iter()
        .for_each(|v| print!("{}, ", v));

    println!();
    let potato = Append::new(e, a);

    (potato.get_value() as Vec<i32>)
        .iter()
        .for_each(|v| print!("{}, ", v));
    println!();

    print_type(convert_string("i32"));
}

fn print_type(value: Type) {
    match value {
        Type::String(_) => println!("String"),
        Type::I32(_) => println!("i32"),
        _ => println!("None"),
    }
}

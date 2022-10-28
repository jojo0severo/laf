use crate::functions::field::Field;
mod functions;

fn main() {
    use functions::number_operations::decimal;
    let mut d = decimal::Decimal::new(['5', '4', '2', '1'], ['0', '1']);
    print!("{}", d.get_value());
    print!("{}", d.get_value());
    print!("{}", d.get_value());
    print!("{}", d.get_value());
}

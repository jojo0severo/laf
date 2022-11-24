mod bigger;
mod equal;
mod smaller;

mod iterable_ops;
mod numeric_ops;

pub use bigger::{Bigger, BiggerOrEqual};
pub use equal::Equal;
pub use iterable_ops::*;
pub use numeric_ops::*;
pub use smaller::{Smaller, SmallerOrEqual};

#[macro_export]
macro_rules! comparable_values {
    ($classname:ident $op:tt $var:ident) => {
        impl<T, U> Field for $classname<$var, T, U>
        where
        T: Field<Output = $var>,
        U: Field<Output = $var>
        {
            type Output = bool;

            fn get_value(&self) -> Self::Output {
                return self.a.get_value().$op(&self.b.get_value());
            }
        }

        impl<T, U> Field for $classname<Vec<$var>, T, U>
        where
        T: Field<Output = Vec<$var>>,
        U: Field<Output = Vec<$var>>
        {
            type Output = bool;

            fn get_value(&self) -> Self::Output {
                let a_values = self.a.get_value();
                let b_values = self.b.get_value();
                return a_values.len() == b_values.len() && a_values.iter().zip(&b_values).all(|(a, b)| a.$op(b));
            }
        }
    };

    ($classname:ident $op:tt $var:ident $($more:ident)+) => {
        crate::comparable_values! { $classname $op $var }
        crate::comparable_values! { $classname $op $($more)+ }
    };
}

use crate::types::field::Field;

pub struct ContainsAny<T, U> {
    a: T,
    b: U,
}

impl<K, T, U> ContainsAny<T, U>
where
    T: Field<Output = Vec<K>>,
    U: Field<Output = Vec<K>>,
{
    pub fn new(a: T, b: U) -> ContainsAny<T, U> {
        return ContainsAny { a, b };
    }
}

impl<K, T, U> Field for ContainsAny<T, U>
where
    K: Ord,
    T: Field<Output = Vec<K>>,
    U: Field<Output = Vec<K>>,
{
    type Output = bool;

    fn get_value(&self) -> Self::Output {
        let container = self.a.get_value();
        let keys = self.b.get_value();

        if keys.len() > container.len() {
            return false;
        }

        for key in keys {
            match container.binary_search(&key) {
                Ok(_) => return true,
                Err(_) => continue,
            }
        }
        return false;
    }
}

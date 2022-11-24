use crate::types::Field;

pub struct ContainsAll<T, U> {
    a: T,
    b: U,
}

impl<K, T, U> ContainsAll<T, U>
where
    T: Field<Output = Vec<K>>,
    U: Field<Output = Vec<K>>,
{
    pub fn new(a: T, b: U) -> ContainsAll<T, U> {
        return ContainsAll { a, b };
    }
}

impl<K, T, U> Field for ContainsAll<T, U>
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
                Ok(_) => continue,
                Err(_) => return false,
            }
        }
        return false;
    }
}

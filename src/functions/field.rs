pub trait Field<T> {
    fn get_value(&mut self) -> T;
}

pub trait ValueField<T>: Field<T> {}

pub trait ContainerField<T, U, K, Y, Z>: Field<T>
where
    U: Field<Y>,
    K: Field<Z>,
{
}

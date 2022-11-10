pub trait Field {
    type Output;

    fn get_value(&self) -> Self::Output;
}

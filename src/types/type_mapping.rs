pub enum Type {
    I128(i128),
    I64(i64),
    I32(i32),
    I16(i16),
    I8(i8),
    ISize(isize),
    USize(usize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(f32),
    F64(f64),
    Char(char),
    Bool(bool),
    String(String),
    VecI128(Vec<i128>),
    VecI64(Vec<i64>),
    VecI32(Vec<i32>),
    VecI16(Vec<i16>),
    VecI8(Vec<i8>),
    VecISize(Vec<isize>),
    VecUSize(Vec<usize>),
    VecU8(Vec<u8>),
    VecU16(Vec<u16>),
    VecU32(Vec<u32>),
    VecU64(Vec<u64>),
    VecU128(Vec<u128>),
    VecF32(Vec<f32>),
    VecF64(Vec<f64>),
    VecChar(Vec<char>),
    VecBool(Vec<bool>),
    VecString(Vec<String>),
    None,
}

pub trait SupporttedType {}

macro_rules! impl_supportted_type {
    ($var:ident) => {
        impl SupporttedType for $var {}
        impl SupporttedType for Vec<$var> {}
    };

    ($var:ident $($more:ident)+) => {
        impl_supportted_type! { $var }
        impl_supportted_type! { $($more)+ }
    };
}

impl_supportted_type!(
    i128 i64 i32 i8 isize
    usize u8 u32 u64 u128
    f32 f64 char bool String
);

pub fn convert_string(value: &str) -> Type {
    return match value {
        "i128" => Type::I128(i128::default()),
        "i64" => Type::I64(i64::default()),
        "i32" => Type::I32(i32::default()),
        "i16" => Type::I16(i16::default()),
        "i8" => Type::I8(i8::default()),
        "isize" => Type::ISize(isize::default()),
        "usize" => Type::USize(usize::default()),
        "u8" => Type::U8(u8::default()),
        "u16" => Type::U16(u16::default()),
        "u32" => Type::U32(u32::default()),
        "u64" => Type::U64(u64::default()),
        "u128" => Type::U128(u128::default()),
        "f32" => Type::F32(f32::default()),
        "f64" => Type::F64(f64::default()),
        "char" => Type::Char('\0'),
        "bool" => Type::Bool(bool::default()),
        "string" => Type::String(String::default()),
        "Vec<i128>" => Type::VecI128(Vec::<i128>::default()),
        "Vec<i64>" => Type::VecI64(Vec::<i64>::default()),
        "Vec<i32>" => Type::VecI32(Vec::<i32>::default()),
        "Vec<i16>" => Type::VecI16(Vec::<i16>::default()),
        "Vec<i8>" => Type::VecI8(Vec::<i8>::default()),
        "Vec<isize>" => Type::VecISize(Vec::<isize>::default()),
        "Vec<usize>" => Type::VecUSize(Vec::<usize>::default()),
        "Vec<u8>" => Type::VecU8(Vec::<u8>::default()),
        "Vec<u16>" => Type::VecU16(Vec::<u16>::default()),
        "Vec<u32>" => Type::VecU32(Vec::<u32>::default()),
        "Vec<u64>" => Type::VecU64(Vec::<u64>::default()),
        "Vec<u128>" => Type::VecU128(Vec::<u128>::default()),
        "Vec<f32>" => Type::VecF32(Vec::<f32>::default()),
        "Vec<f64>" => Type::VecF64(Vec::<f64>::default()),
        "Vec<char>" => Type::VecChar(Vec::<char>::default()),
        "Vec<bool>" => Type::VecBool(Vec::<bool>::default()),
        "Vec<string>" => Type::VecString(Vec::<String>::default()),
        _ => Type::None,
    };
}

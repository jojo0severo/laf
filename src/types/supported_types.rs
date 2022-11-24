pub enum SupportedType {
    USizeType(usize),
    U8Type(u8),
    U16Type(u16),
    U32Type(u32),
    U64Type(u64),
    U128Type(u128),
    ISizeType(isize),
    I8Type(i8),
    I16Type(i16),
    I32Type(i32),
    I64Type(i64),
    I128Type(i128),
    F32Type(f32),
    F64Type(f64),
    BoolType(bool),
    CharType(char),
    StringType(String),
    VecUSizeType(Vec<usize>),
    VecU8Type(Vec<u8>),
    VecU16Type(Vec<u16>),
    VecU32Type(Vec<u32>),
    VecU64Type(Vec<u64>),
    VecU128Type(Vec<u128>),
    VecISizeType(Vec<isize>),
    VecI8Type(Vec<i8>),
    VecI16Type(Vec<i16>),
    VecI32Type(Vec<i32>),
    VecI64Type(Vec<i64>),
    VecI128Type(Vec<i128>),
    VecF32Type(Vec<f32>),
    VecF64Type(Vec<f64>),
    VecBoolType(Vec<bool>),
    VecCharType(Vec<char>),
    VecStringType(Vec<String>),
}

pub trait SupportedTypeDefault<T>: Default + Sized {
    fn supported_default_value() -> SupportedType;
    fn default_value() -> T;
}

macro_rules! generate_supported_types {
    ($default_value:expr, ($var_type:ty, $enum_type:ident)) => {
        impl SupportedTypeDefault<$var_type> for $var_type {
            fn supported_default_value() -> SupportedType {
                return SupportedType::$enum_type($default_value);
            }

            fn default_value() -> $var_type {
                return $default_value;
            }
        }
    };

    ($default_value:expr, ($var_type:ty, $enum_type:ident) $(($more_var_type:ty, $more_enum_type:ident))+) => {
        generate_supported_types! { $default_value, ($var_type, $enum_type) }
        generate_supported_types! { $default_value, $(($more_var_type, $more_enum_type))+ }
    };
}

generate_supported_types!(
    0,
    (usize, USizeType)(u8, U8Type)(u16, U16Type)(u32, U32Type)(u64, U64Type)(u128, U128Type)(
        isize, ISizeType
    )(i8, I8Type)(i16, I16Type)(i32, I32Type)(i64, I64Type)(i128, I128Type)
);

generate_supported_types!(0.0, (f32, F32Type)(f64, F64Type));

generate_supported_types!(false, (bool, BoolType));

generate_supported_types!('\0', (char, CharType));

generate_supported_types!(String::from(""), (String, StringType));

generate_supported_types!(
    vec![],
    (Vec<usize>, VecUSizeType)
    (Vec<u8>, VecU8Type)
    (Vec<u16>, VecU16Type)
    (Vec<u32>, VecU32Type)
    (Vec<u64>, VecU64Type)
    (Vec<u128>, VecU128Type)
    (Vec<isize>, VecISizeType)
    (Vec<i8>, VecI8Type)
    (Vec<i16>, VecI16Type)
    (Vec<i32>, VecI32Type)
    (Vec<i64>, VecI64Type)
    (Vec<i128>, VecI128Type)
    (Vec<f32>, VecF32Type)
    (Vec<f64>, VecF64Type)
    (Vec<bool>, VecBoolType)
    (Vec<char>, VecCharType)
    (Vec<String>, VecStringType)
);

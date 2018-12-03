//! Helper macros for implementing NBT format encoding and decoding via `serde`.

macro_rules! return_expr_for_serialized_types_method {
    ($expr:expr, $func:ident($($arg:ty),*)) => {
        #[inline]
        fn $func(self, $(_: $arg,)*)
                 -> ::std::result::Result<Self::Ok, Self::Error>
        {
            $expr
        }
    };
    ($expr:expr, $func:ident($($arg:ty),*), result: $result:path) => {
        #[inline]
        fn $func(self, $(_: $arg,)*)
                 -> ::std::result::Result<$result, Self::Error>
        {
            $expr
        }
    };
    ($expr:expr, $func:ident($($arg:ty),*), where: $where:path) => {
        #[inline]
        fn $func<__T: ?Sized>(self, $(_: $arg,)*)
                              -> ::std::result::Result<Self::Ok, Self::Error>
            where __T: $where
        {
            $expr
        }
    };
}

macro_rules! return_expr_for_serialized_types_helper {
    ($expr:expr, bool) => {
        return_expr_for_serialized_types_method!{$expr, serialize_bool(bool)}
    };
    ($expr:expr, i8) => {
        return_expr_for_serialized_types_method!{$expr, serialize_i8(i8)}
    };
    ($expr:expr, i16) => {
        return_expr_for_serialized_types_method!{$expr, serialize_i16(i16)}
    };
    ($expr:expr, i32) => {
        return_expr_for_serialized_types_method!{$expr, serialize_i32(i32)}
    };
    ($expr:expr, i64) => {
        return_expr_for_serialized_types_method!{$expr, serialize_i64(i64)}
    };
    ($expr:expr, u8) => {
        return_expr_for_serialized_types_method!{$expr, serialize_u8(u8)}
    };
    ($expr:expr, u16) => {
        return_expr_for_serialized_types_method!{$expr, serialize_u16(u16)}
    };
    ($expr:expr, u32) => {
        return_expr_for_serialized_types_method!{$expr, serialize_u32(u32)}
    };
    ($expr:expr, u64) => {
        return_expr_for_serialized_types_method!{$expr, serialize_u64(u64)}
    };
    ($expr:expr, f32) => {
        return_expr_for_serialized_types_method!{$expr, serialize_f32(f32)}
    };
    ($expr:expr, f64) => {
        return_expr_for_serialized_types_method!{$expr, serialize_f64(f64)}
    };
    ($expr:expr, char) => {
        return_expr_for_serialized_types_method!{$expr, serialize_char(char)}
    };
    ($expr:expr, str) => {
        return_expr_for_serialized_types_method!{$expr, serialize_str(&str)}
    };
    ($expr:expr, bytes) => {
        return_expr_for_serialized_types_method!{$expr, serialize_bytes(&[u8])}
    };
    ($expr:expr, none) => {
        return_expr_for_serialized_types_method!{$expr, serialize_none()}
    };
    ($expr:expr, unit) => {
        return_expr_for_serialized_types_method!{$expr, serialize_unit()}
    };
    ($expr:expr, unit_struct) => {
        return_expr_for_serialized_types_method!{
            $expr, serialize_unit_struct(&'static str)
        }
    };
    ($expr:expr, unit_variant) => {
        return_expr_for_serialized_types_method!{
            $expr, serialize_unit_variant(&'static str, u32, &'static str)
        }
    };
    ($expr:expr, some) => {
        return_expr_for_serialized_types_method!{
            $expr, serialize_some(&__T),
            where: ::serde::ser::Serialize
        }
    };
    ($expr:expr, newtype_struct) => {
        return_expr_for_serialized_types_method!{
            $expr, serialize_newtype_struct(&'static str, &__T),
            where: ::serde::ser::Serialize
        }
    };
    ($expr:expr, newtype_variant) => {
        return_expr_for_serialized_types_method!{
            $expr, serialize_newtype_variant(&'static str, u32,
                                              &'static str, &__T),
            where: ::serde::ser::Serialize
        }
    };
    ($expr:expr, seq) => {
        return_expr_for_serialized_types_method!{
            $expr, serialize_seq(Option<usize>),
            result: Self::SerializeSeq
        }
    };
    ($expr:expr, tuple) => {
        return_expr_for_serialized_types_method!{
            $expr, serialize_tuple(usize),
            result: Self::SerializeTuple
        }
    };
    ($expr:expr, tuple_struct) => {
        return_expr_for_serialized_types_method!{
            $expr, serialize_tuple_struct(&'static str, usize),
            result: Self::SerializeTupleStruct
        }
    };
    ($expr:expr, tuple_variant) => {
        return_expr_for_serialized_types_method!{
            $expr, serialize_tuple_variant(&'static str, u32, &'static str,
                                            usize),
            result: Self::SerializeTupleVariant
        }
    };
    ($expr:expr, map) => {
        return_expr_for_serialized_types_method!{
            $expr, serialize_map(Option<usize>),
            result: Self::SerializeMap
        }
    };
    ($expr:expr, struct) => {
        return_expr_for_serialized_types_method!{
            $expr, serialize_struct(&'static str, u32),
            result: Self::SerializeStruct
        }
    };
    ($expr:expr, struct_variant) => {
        return_expr_for_serialized_types_method!{
            $expr, serialize_struct_variant(&'static str, u32, &'static str,
                                             usize),
            result: Self::SerializeStructVariant
        }
    };
}

/// Helper macro for implementing the `serde::se::Serializer` trait.
///
/// Implement the serializer methods for each `$type` with body `$expr`.
///
/// This macro is very similar to `serde::forward_to_deserialize`, but instead
/// of "forwarding" it allows arbitrary expressions in the body, so long as they
/// are all the same.
///
/// This macro can be used to stub out `Serializer` implementations.
macro_rules! return_expr_for_serialized_types {
    ($expr:expr; $($type:tt)*) => {
        $(return_expr_for_serialized_types_helper!{$expr, $type})*
    };
}

macro_rules! unrepresentable {
    ($($type:tt)*) => {
        $(return_expr_for_serialized_types_helper!{Err(Error::UnrepresentableType("$type")), $type})*
    };
}

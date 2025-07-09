/*!
 * Implementation of common traits for enums and variants.
 */

///
/// Generate an implementation of the standard [From] trait to construct a new enum with
/// the provided variant value.
///
/// ## Forms
///
/// ### `impl_from_for_variant!(Type => Enum, Variant)`
///
/// This form generates a [`From`] `Type` implementation for `Enum`.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address(String);
/// pub enum TypedAddress {
///     Home(Address),
/// }
/// // impl_from_for_variant!(Address => TypedAddress, Home);
///
/// impl From<Address> for TypedAddress {
///     fn from(value: Address) -> Self {
///         Self::Home(value)
///     }
/// }
/// ```
///
/// ### `impl_from_for_variant!(into Type => Enum, Variant)`
///
/// This form generates a [`From`] `Type` implementation for `Enum`.
///
/// * The type of the generic parameter to `From` is the trait-bound type `T: Into<Type>`
///   rather than simply `Type` for flexibility.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address(String);
/// pub enum TypedAddress {
///     Home(Address),
/// }
/// // impl_from_for_variant!(into Address => TypedAddress, Home);
///
/// impl<T: Into<Address>> From<T> for TypedAddress {
///     fn from(value: T) -> Self {
///         Self::Home(value.into())
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! impl_from_for_variant {
    // Base case: `impl From<Type> for Enum => Enum::Variant(value:Type)`
    ($value_type:ty => $enum_type:ty, $variant_name:ident) => {
        impl From<$value_type> for $enum_type {
            fn from(value: $value_type) -> Self {
                Self::$variant_name(value)
            }
        }
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Base case with *into*: `From<T> where T: Into<Type>`
    (into $value_type:ty => $enum_type:ty, $variant_name:ident) => {
        impl<T: Into<$value_type>> From<T> for $enum_type {
            fn from(value: T) -> Self {
                Self::$variant_name(value.into())
            }
        }
    };
}

///
/// Generate an implementation for the enumeration of a method that delegates the to the same named
/// method with the same parameters for the value on each variant.
///
/// ## Forms
///
/// ### `impl_delegate_for_all!(Enum [ ( Variant ),* ] => vis Name [ -> Type ]`
///
/// This form generates a simple delegate method implementation for `Enum`.
///
/// ```rust
/// # pub struct Address(String);
/// # impl Address { pub fn to_string(&self) -> String { String::new() } }
/// pub enum TypedAddress {
///     Home(Address),
///     UnParsed(String),
///     XRef(u64),
/// }
/// // impl_delegate_for_all!(TypedAddress [ Home, unParsed, XRef ] => pub to_string -> String);
///
/// impl TypedAddress {
///     pub fn to_string(&self) -> String {
///         match self {
///             Self::Home(value) => value.to_string(),
///             Self::UnParsed(value) => value.to_string(),
///             Self::XRef(value) => value.to_string(),
///         }
///     }
/// }
/// ```
///
/// ### `impl_delegate_for_all!(Enum [ ( Variant ),* ] => vis Name ( (param: type),+ ) [ -> Type ]`
///
/// TBD
///
#[doc(hidden)]
#[macro_export]
macro_rules! impl_delegate_for_all {
    ($enum_type:ty [ $( $variant_name:ident ),* ]
        => $fn_vis:vis $fn_name:ident $( -> $fn_type:ty )? ) => {
        impl $enum_type {
            $fn_vis fn $fn_name(&self) $( -> $fn_type )? {
                match self {
                    $(
                        Self::$variant_name(value) => value.$fn_name(),
                    )*
                }
            }
        }
    };
    ($enum_type:ty [ $( $variant_name:ident ),* ]
        => $fn_vis:vis $fn_name:ident ( $($param_name:ident : $param_type:ty),+ ) $( -> $fn_type:ty )? ) => {
        impl $enum_type {
            $fn_vis fn $fn_name(&self, $($param_name: $param_type),+ ) $( -> $fn_type )? {
                match self {
                    $(
                        Self::$variant_name(value) => value.$fn_name( $($param_name),+ ),
                    )*
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_display_for_enum {
    ($enum_type:ty [ $( $variant_name:ident ),* ]) => {
        impl ::std::fmt::Display for $enum_type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match self {
                    $(
                        Self::$variant_name(value) => value.fmt(f),
                    ),*
                }
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Re-export macros
// ------------------------------------------------------------------------------------------------

pub use crate::{impl_delegate_for_all, impl_display_for_enum, impl_from_for_variant};

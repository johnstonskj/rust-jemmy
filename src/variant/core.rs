/*!
 * The core set of enum variant access macros: [`is_variant`] and [`as_variant`].
 */

// ------------------------------------------------------------------------------------------------
// Variant Macros ❱ is_variant
// ------------------------------------------------------------------------------------------------

///
/// Generate a predicate function that returns `true` if the instance of the corresponding
/// enumeration is of the given variant.
///
///  ## Forms
///
/// ### `is_variant!(viz Variant [, function_name])`
///
/// This form generates a predicate function to test variant *selection*, it is a synonym
/// for `is_variant!(viz Variant [, function_name] => ())`, see next.
///
/// ### `is_variant!(viz Variant [, function_name] => ())`
///
/// This form generates a predicate function to test variant *selection* for variants that do
/// not carry data in tuple form.
///
/// * The value `()` acts as a marker denoting the lack of data.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// pub enum TypedAddress {
///     Unknown,
///     BadlyFormed,
/// }
/// impl TypedAddress {
///     // is_variant!(pub Unknown);
///     // or
///     // is_variant!(pub BadlyFormed => ());
///
///     /// Returns `true` if `self` is an instance of the `Unknown` variant, else `false`.
///     pub fn is_unknown(&self) -> bool {
///        matches!(self, Self::Unknown)
///     }
///     /// Returns `true` if `self` is an instance of the `BadlyFormed` variant, else `false`.
///     pub fn is_badly_formed(&self) -> bool {
///        matches!(self, Self::BadlyFormed)
///     }
/// }
/// ```
///
/// ### `is_variant!(viz Variant [, function_name] => Type)`
///
/// This form generates a predicate function to test variant *selection* for variants that
/// carry data in tuple form.
///
/// * The type `Type` acts as a marker denoting the presence of data, it is not actually
///   used in the implementation.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address(String);
/// pub enum TypedAddress {
///     Home(Address),
/// }
/// impl TypedAddress {
///     // is_variant!(pub Home => Address);
///     // is_variant!(pub Home, personal => Address);
///
///     /// Returns `true` if `self` is an instance of the `Home` variant, else `false`.
///     pub fn is_home(&self) -> bool {
///         matches!(self, Self::Home(_))
///     }
///     /// Returns `true` if `self` is an instance of the `Home` variant, else `false`.
///     pub fn is_personal(&self) -> bool {
///         matches!(self, Self::Home(_))
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! is_variant {
    // (1) Base no-data case: `viz Variant, function_name`
    ($fn_vis:vis $variant_name:ident, $function_name:ident) => {
        $crate::is_variant!($fn_vis $variant_name, $function_name => ());
    };
    // Case (1) without *function_name*: `viz Variant`
    ($fn_vis:vis $variant_name:ident) => {
        $crate::is_variant!($fn_vis $variant_name => ());
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // (2) Case (1) with explicit `()`: `viz Variant, function_name => ()`
    ($fn_vis:vis $variant_name:ident, $function_name:ident => ()) => {
        paste::paste! {
            #[doc = "Returns `true` if `self` is an instance of the `" $variant_name "` variant, else `false`."]
            $fn_vis const fn [< is_ $function_name:snake >](&self) -> bool {
                matches!(self, Self::$variant_name)
            }
        }
    };
    // Case (2) without *function_name*: `viz Variant => ()`
    ($fn_vis:vis $variant_name:ident => ()) => {
        $crate::is_variant!($fn_vis $variant_name, $variant_name => ());
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // (3) Base data case: `viz Variant, function_name => Type`
    ($fn_vis:vis $variant_name:ident, $function_name:ident => $variant_type:ty) => {
        paste::paste! {
            #[doc = "Returns `true` if `self` is an instance of the `" $variant_name "` variant, else `false`."]
            $fn_vis const fn [< is_ $function_name:snake >](&self) -> bool {
                matches!(self, Self::$variant_name(_))
            }
        }
    };
    // (3) Base data case: `viz Variant => Type`
    ($fn_vis:vis $variant_name:ident => $variant_type:ty) => {
        $crate::is_variant!($fn_vis $variant_name, $variant_name => $variant_type);
    };
}

// ------------------------------------------------------------------------------------------------
// Variant Macros ❱ as_variant
// ------------------------------------------------------------------------------------------------

///
///  Generate a *safe cast* method for variant-associated data.
///
/// ## Forms
///
/// ### `as_variant!(viz Variant [, function_name] => Type)`
///
/// This form generates a *safe cast* function for variant-associated data.
///
/// * If the variant holds a single value of type `Type`, the return type of this methid is therefore
///   `Option<&Type>`; it is `Some(...)` if the variant matches, and `None` otherwise.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address(String);
/// pub enum TypedAddress {
///     Home(Address),
/// }
/// impl TypedAddress {
///     // as_variant!(pub Home => Address);
///
///     /// If `self` is an instance of the `Home` variant, which holds a value of type
///     /// `Address`, return an immutable reference `Some(value: &Address)`, else `None`.
///     pub const fn as_home(&self) -> Option<&Address> {
///         match self {
///             Self::Home(value) => Some(value),
///             _ => None,
///         }
///     }
/// }
/// ```
///
/// ### `as_variant!(viz Variant [, function_name] => copy Type)`
///
/// This form generates a *safe cast* function for variant-associated data.
///
/// * If the variant holds a single value of type `Type`, the return type of this methid is therefore
///   `Option<Type>`; it is `Some(...)` if the variant matches, and `None` otherwise.
/// * This form requires `Type` implements `Copy`.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address(String);
/// pub enum TypedAddress {
///     XRef(u64),
/// }
/// impl TypedAddress {
///     // as_variant!(pub XRef => copy u64);
///
///     /// If `self` is an instance of the `XRef` variant, which holds a value of type
///     /// `u64`, return a copy `Some(value: u64)`, else `None`.
///     pub const fn as_xref(&self) -> Option<u64> {
///         match self {
///             Self::XRef(value) => Some(*value),
///             _ => None,
///         }
///     }
/// }
/// ```
///
/// ### `as_variant!(viz Variant [, function_name] [=> ()])`
///
/// This form generates a *safe cast* function for variant-associated data specifically for non-data
/// associated variants where a simple marker value suffices.
///
/// * The return type of this method is `Option<()>`; specifically, if the variant matches it returns
///   `Some(())` else `None`.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address(String);
/// pub enum TypedAddress {
///     Unknown,
/// }
/// impl TypedAddress {
///     // as_variant!(pub Unknown => ());
///
///     /// If `self` is an instance of the `Unknown` variant, which holds no value,
///     /// return `Some(())`, else `None`.
///     pub const fn as_unknown(&self) -> Option<()> {
///         match self {
///             Self::Unknown => Some(()),
///             _ => None,
///         }
///     }
/// }
/// ```
///
/// ### `as_variant!(viz Variant [, function_name] => value, Type)`
///
/// This form generates a *safe cast* function for variant-associated data specifically for non-data
/// associated variants and a distinct value can be substituted.
///
/// * The return type of this method is `Option<Type>`; specifically, if the variant matches it returns
///   `Some(value)` else `None`.
///
/// ```rust
/// pub enum Error {
///     InvalidAddressError,
/// }
/// pub enum TypedAddress {
///     BadlyFormed,
/// }
/// impl TypedAddress {
///     // as_variant!(pub BadlyFormed => Error::InvalidAddressError, Error);
///
///     /// If `self` is an instance of the `BadlyFormed` variant, which holds no value, return
///     /// `Some(Error: Error)`, else `None`.
///     pub const fn as_badly_formed(&self) -> Option<Error> {
///         match self {
///             Self::BadlyFormed => Some(Error::InvalidAddressError),
///             _ => None,
///         }
///     }
/// }
/// ```
///
/// ### `as_variant!(viz Variant [, function_name] => const value, Type)`
///
/// This form generates a *safe cast* function for variant-associated data specifically for non-data
/// associated variants and a distinct value can be substituted.
///
/// * The return type of this method is `Option<Type>`; specifically, if the variant matches it returns
///   `Some(value)` else `None`.
/// * The caller is responsible for ensuring that the value provided **is** in fact a constant.
///
/// ```rust
/// pub enum TypedAddress {
///     Unknown,
/// }
/// impl TypedAddress {
///     // as_variant!(pub Unknown => true, bool);
///
///     /// If `self` is an instance of the `Unknown` variant, which holds no value, return
///     /// `Some(true: bool)`, else `None`.
///     pub const fn as_unknown(&self) -> Option<bool> {
///         const CONST_VALUE: Option<bool> = Some(true);
///         match self {
///             Self::Unknown => CONST_VALUE,
///             _ => None,
///         }
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! as_variant {
    // (1) Base no-data case: `viz Variant, function_name => ()`
    // NOTE: this has to come first or `expr` consumes `()`.
    ($fn_vis:vis $variant_name:ident, $function_name:ident => ()) => {
        paste::paste! {
            #[doc = "If `self` is an instance of the `" $variant_name "` variant, which holds no value, "
                    "return `Some(())`, else `None`."]
            $fn_vis const fn [< as_ $function_name:snake >](&self) -> Option<()> {
                match self {
                    Self::$variant_name => Some(()),
                    _ => None,
                }
            }
        }
    };
    // (1a) Base case without *function_name*: `viz Variant => ()`
    ($fn_vis:vis $variant_name:ident => ()) => {
        $crate::as_variant!($fn_vis $variant_name, $variant_name => ());
    };
    // (1b) Base case without *()*: `viz Variant, function_name`
    ($fn_vis:vis $variant_name:ident, $function_name:ident) => {
        $crate::as_variant!($fn_vis $variant_name, $function_name => ());
    };
    // Case (1b) without *function_name*: `viz Variant`
    ($fn_vis:vis $variant_name:ident) => {
        $crate::as_variant!($fn_vis $variant_name, $variant_name => ());
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // (2) Base data case: `viz Variant, function_name => Type`
    ($fn_vis:vis $variant_name:ident, $function_name:ident => $variant_type:ty) => {
        paste::paste! {
            #[doc = "If `self` is an instance of the `" $variant_name "` variant, which holds a value of type `"
                    $variant_type "`, return an immutable reference `Some(value: &" $variant_type ")`, else `None`."]
            $fn_vis const fn [< as_ $function_name:snake >](&self) -> Option<&$variant_type> {
                match self {
                    Self::$variant_name(value) => Some(value),
                    _ => None,
                }
            }
        }
    };
    // Case (2) without *function_name*: `viz Variant => Type`
    ($fn_vis:vis $variant_name:ident => $variant_type:ty) => {
        $crate::as_variant!($fn_vis $variant_name, $variant_name => $variant_type);
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // (3) Case (2) with *copy*: `viz Variant, function_name => copy Type`
    ($fn_vis:vis $variant_name:ident, $function_name:ident => copy $variant_type:ty) => {
        paste::paste! {
            #[doc = "If `self` is an instance of the `" $variant_name "` variant, which holds a value of type `"
                    $variant_type "`, return a copy `Some(value: " $variant_type ")`, else `None`."]
            $fn_vis const fn [< as_ $function_name:snake >](&self) -> Option<$variant_type> {
                match self {
                    Self::$variant_name(value) => Some(*value),
                    _ => None,
                }
            }
        }
    };
    // Case (3) without *function_name*: `viz Variant => copy Type`
    ($fn_vis:vis $variant_name:ident => copy $variant_type:ty) => {
        $crate::as_variant!($fn_vis $variant_name, $variant_name => copy $variant_type);
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // (4) Case (2) with *value*: `viz Variant, function_name => value, Type`
    ($fn_vis:vis $variant_name:ident, $function_name:ident => $value:expr, $value_type:ty) => {
        paste::paste! {
            #[doc = "If `self` is an instance of the `" $variant_name "` variant, which holds no value, "
                    "return `Some(value: " $value_type ")`, else `None`."]
            $fn_vis const fn [< as_ $function_name:snake >](&self) -> Option<$value_type> {
                match self {
                    Self::$variant_name => Some($value),
                    _ => None,
                }
            }
        }
    };
    // Case (4) without *function_name*: `viz Variant => value, Type`
    ($fn_vis:vis $variant_name:ident => $value:expr, $value_type:ty) => {
        $crate::as_variant!($fn_vis $variant_name, $variant_name => $value, $value_type);
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // (5) Case (2) with *const value*: `viz Variant, function_name => const value, Type`
    ($fn_vis:vis $variant_name:ident, $function_name:ident => const $value:expr, $value_type:ty) => {
        paste::paste! {
            #[doc = "If `self` is an instance of the `" $variant_name "` variant, which holds no value, "
                    "return `Some(value: " $variant_type ")`, else `None`."]
            $fn_vis const fn [< as_ $function_name:snake >](&self) -> Option<$variant_type> {
                const CONST_VALUE: Option<$value_type> = Some($value);
                match self {
                    Self::$variant_name => CONST_VALUE,
                    _ => None,
                }
            }
        }
    };
    // Case (5) without *function_name*: `viz Variant => const value, Type`
    ($fn_vis:vis $variant_name:ident => const $value:expr, $value_type:ty) => {
        $crate::as_variant!($fn_vis $variant_name, $variant_name => const $value, $value_type);
    };
}

///
///  Generate a mutable *safe cast* method for variant-associated data.
///
/// ## Forms
///
/// ### `as_variant_mut!(viz Variant [, function_name] => Type)`
///
/// TBD
///
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address(String);
/// pub enum TypedAddress {
///     Home(Address),
/// }
/// impl TypedAddress {
///     // as_variant_mut!(pub Home => Address);
///
///     /// If `self` is an instance of the `Home` variant, which holds a value of type
///     /// `Address`, return a mutable reference `Some(&mut Address)`, else `None`.
///     pub const fn as_home_mut(&mut self) -> Option<&mut Address> {
///         match self {
///             Self::Home(ref mut value) => Some(value),
///             _ => None,
///         }
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! as_variant_mut {
    ($fn_vis:vis $variant_name:ident, $function_name:ident => $variant_type:ty) => {
        paste::paste! {
            #[doc = "If `self` is an instance of the `" $variant_name "` variant, which holds a value of type `"
                    $variant_type "`, return a mutable reference `Some(&mut " $variant_type ")`, else `None`."]
            $fn_vis const fn [< as_ $function_name:snake _mut >](&mut self) -> Option<&mut $variant_type> {
                match self {
                    Self::$variant_name(ref mut value) => Some(value),
                    _ => None,
                }
            }
        }
    };
    // Case (2) without *function_name*: `viz Variant => Type`
    ($fn_vis:vis $variant_name:ident => $variant_type:ty) => {
        $crate::as_variant_mut!($fn_vis $variant_name, $variant_name => $variant_type);
    };
}

// ------------------------------------------------------------------------------------------------
// Re-export macros
// ------------------------------------------------------------------------------------------------

pub use crate::{as_variant, as_variant_mut, is_variant};

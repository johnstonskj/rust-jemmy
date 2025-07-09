/*!
 * Macros that produce combinations of methods using the core module's [`is_variant`] and
 * [`as_variant`] macros.
 */

// ------------------------------------------------------------------------------------------------
// Combinator Macros ❱ is_as_variant
// ------------------------------------------------------------------------------------------------

///
///  Generate both [`is_variant`] and [`as_variant`] for an enumeration variant.
///
/// ## Forms
///
/// ### `is_as_variant!(viz Enum, Variant => Type)`
///
/// This form generates both `is_variant` and `as_variant` for the applicable cases.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// use jemmy::*;
/// # pub struct Address(String);
/// pub enum TypedAddress {
///     Home(Address),
/// }
/// impl TypedAddress {
///     // is_as_variant!(pub Home => Address);
///
///     is_variant!(pub Home => Address);
///     as_variant!(pub Home => Address);
/// }
/// ```
///
/// ### `is_as_variant!(viz Enum, Variant => copy Type)`
///
/// This form generates both `is_variant` and `as_variant` for the applicable cases.
///
/// * This form requires `Type` implements `Copy`.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// use jemmy::*;
/// # pub struct Address(String);
/// pub enum TypedAddress {
///     XRef(u64),
/// }
/// impl TypedAddress {
///     // is_as_variant!(pub XRef => copy u64);
///
///     is_variant!(pub XRef => u64);
///     as_variant!(pub XRef => copy u64);
/// }
/// ```
///
#[macro_export]
macro_rules! is_as_variant {
    // Base case: `viz Variant, function_name => Type`
    ($fn_vis:vis $variant_name:ident, $function_name:ident => $variant_type:ty) => {
        paste::paste! {
            $crate::is_variant!($fn_vis $variant_name, $function_name => $variant_type);
            $crate::as_variant!($fn_vis $variant_name, $function_name => $variant_type);
        }
    };
    // Base case without *function_name*: `viz Variant => Type`
    ($fn_vis:vis $variant_name:ident => $variant_type:ty) => {
        $crate::is_as_variant!($fn_vis $variant_name, $variant_name => $variant_type);
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // (2) Base case with *copy*: `viz Variant as function_name => copy Type`
    ($fn_vis:vis $variant_name:ident, $function_name:ident => copy $variant_type:ty) => {
        paste::paste! {
            $crate::is_variant!($fn_vis $variant_name, $function_name => $variant_type);
            $crate::as_variant!($fn_vis $variant_name, $function_name => copy $variant_type);
        }
    };
    // Case (2) without *function_name*: `viz Variant => copy Type`
    ($fn_vis:vis $variant_name:ident => copy $variant_type:ty) => {
        $crate::is_as_variant!($fn_vis $variant_name, $variant_name => copy $variant_type);
    };
}

// ------------------------------------------------------------------------------------------------
// Re-export macros
// ------------------------------------------------------------------------------------------------

pub use crate::is_as_variant;

/*!
 * Macros that produce combinations of methods using the core module's [`get`], [`get_mut`],
 * [`set`], [`unset`] and [`with`] macros.
 */

// ------------------------------------------------------------------------------------------------
// Combinator Macros ❱ Get and Set
// ------------------------------------------------------------------------------------------------

///
/// Generate [`get`] and [`set`] for a struct field.
///
/// ## Forms
///
/// ### `get_and_set!(viz name => Type)`
///
/// This form generates simple getter and setter functions using the [`get`] and [`set`]  macros.
///
/// * In this form `name` is used in both the naming of the generated functions and the
///   name of as the structure's field.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// use jemmy::{get, set};
/// impl Address {
///     // get_and_set!(pub street_1 => String);
///
///     get!(pub street_1 => String);
///     set!(pub street_1 => String);
/// }
/// ```
///
/// ### `get_and_set!(viz name => field_name, Type)`
///
/// This form generates simple getter and setter functions using the [`get`] and [`set`]  macros.
///
/// * In this form `name` is used in naming the generated functions while `field_name` is the
///   name of the structure's field.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1_string: String, street_2: Option<String> }
/// use jemmy::{get, set};
/// impl Address {
///     // get_and_set!(pub street_1 => street_1_string, String);
///
///     get!(pub street_1 => street_1_string, String);
///     set!(pub street_1 => street_1_string, String);
/// }
/// ```
///
/// ### `get_and_set!(viz name => into Type)`
///
/// This form generates simple getter and setter functions using the [`get`] and [`set`]  macros.
///
/// * In this form `name` is used in both the naming of the generated functions and the
///   name of as the structure's field.
/// * The keyword `into` is passed to the [`set`] macro.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// use jemmy::{get, set};
/// impl Address {
///     // get_and_set!(pub street_1 => into String);
///
///     get!(pub street_1 => String);
///     set!(pub street_1 => into String);
/// }
/// ```
///
/// ### `get_and_set!(viz name => field_name, into Type)`
///
/// This form generates simple getter and setter functions using the [`get`] and [`set`]  macros.
///
/// * In this form `name` is used in naming the generated functions while `field_name` is the
///   name of the structure's field.
/// * The keyword `into` is passed to the [`set`] macro.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1_string: String, street_2: Option<String> }
/// use jemmy::{get, set};
/// impl Address {
///     // get_and_set!(pub street_1 => street_1_string, String);
///
///     get!(pub street_1 => street_1_string, String);
///     set!(pub street_1 => street_1_string, into String);
/// }
/// ```
///
/// ### `get_and_set!(viz name => optional Type)`
///
/// This form generates simple getter and setter functions using the [`get`] and [`set`]  macros.
///
/// * In this form `name` is used in both the naming of the generated functions and the
///   name of as the structure's field.
/// * The keyword `optional` is passed to both both [`get`] and [`set`] macros.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// use jemmy::{get, set};
/// impl Address {
///     // get_and_set!(pub street_2 => optional String);
///
///     get!(pub street_2 => optional String);
///     set!(pub street_2 => optional String);
/// }
/// ```
///
/// ### `get_and_set!(viz name => field_name, optional Type)`
///
/// This form generates simple getter and setter functions using the [`get`] and [`set`]  macros.
///
/// * In this form `name` is used in naming the generated functions while `field_name` is the
///   name of the structure's field.
/// * The keyword `optional` is passed to both both [`get`] and [`set`] macros.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2_string: Option<String> }
/// use jemmy::{get, set};
/// impl Address {
///     // get_and_set!(pub street_2 => street_2_string, optional String);
///
///     get!(pub street_2 => street_2_string, optional String);
///     set!(pub street_2 => street_2_string, optional String);
/// }
/// ```
///
#[macro_export]
macro_rules! get_and_set {
    // Base case: `viz name => field_name, Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        $crate::get!($fn_vis $fn_name => $field_name, $value_type);
        $crate::set!($fn_vis $fn_name => $field_name, $value_type);
    };
    // Base case without *field name*: `viz name => Type`
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        $crate::get_and_set!($fn_vis $name => $name, $value_type);
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // (2) Base case with *copy*: `viz name => field_name, copy Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, copy $value_type:ty) => {
        $crate::get!($fn_vis $fn_name => $field_name, copy $value_type);
        $crate::set!($fn_vis $fn_name => $field_name, $value_type);
    };
    // Case (2) without *field name*: `viz name => copy Type`
    ($fn_vis:vis $name:ident => copy $value_type:ty) => {
        $crate::get_and_set!($fn_vis $name => $name, copy $value_type);
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // (3) Base case with *into*: `viz name => field_name, into Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, into $value_type:ty) => {
        $crate::get!($fn_vis $fn_name => $field_name, $value_type);
        $crate::set!($fn_vis $fn_name => $field_name, into $value_type);
    };
    // Case (3) without *field name*: `viz name => into Type`
    ($fn_vis:vis $name:ident => into $value_type:ty) => {
        $crate::get_and_set!($fn_vis $name => $name, into $value_type);
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // (4) Base case with *optional*: `viz name => field_name, optional Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional $value_type:ty) => {
        $crate::get!($fn_vis $fn_name => $field_name, optional $value_type);
        $crate::set!($fn_vis $fn_name => $field_name, optional $value_type);
    };
    // Case (4) without *field name*: `viz name => optional Type`
    ($fn_vis:vis $name:ident => optional $value_type:ty) => {
        $crate::get_and_set!($fn_vis $name => $name, optional $value_type);
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // (5) Case (4) with *copy*: `viz name => field_name, optional copy Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional copy $value_type:ty) => {
        $crate::get!($fn_vis $fn_name => $field_name, optional copy $value_type);
        $crate::set!($fn_vis $fn_name => $field_name, optional $value_type);
    };
    // Case (5) without *field name*: `viz name => optional copy Type`
    ($fn_vis:vis $name:ident => optional copy $value_type:ty) => {
        $crate::get_and_set!($fn_vis $name => $name, optional copy $value_type);
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // (6) Case (4) with *copy*: `viz name => field_name, optional into Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional into $value_type:ty) => {
        $crate::get!($fn_vis $fn_name => $field_name, optional $value_type);
        $crate::set!($fn_vis $fn_name => $field_name, optional into $value_type);
    };
    // Case (6) without *field name*: `viz name => optional into Type`
    ($fn_vis:vis $name:ident => optional into $value_type:ty) => {
        $crate::get_and_set!($fn_vis $name => $name, optional into $value_type);
    };
}

// ------------------------------------------------------------------------------------------------
// Combinator Macros ❱ With, Get and Set
// ------------------------------------------------------------------------------------------------

///
/// Generate [`with`], [`get`] and [`set`] for a struct field.
///
/// ## Forms
///
/// ### `with_get_and_set!(viz name => Type)`
///
/// This form generates simple initializer, getter and setter functions using the [`with`],
/// [`get`] and [`set`]  macros.
///
/// * In this form `name` is used in both the naming of the generated functions and the
///   name of as the structure's field.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// use jemmy::*;
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // with_get_and_set!(pub street_1 => String);
///
///     with!(pub street_1 => String);
///     get_and_set!(pub street_1 => String);
/// }
/// ```
///
/// ### `with_get_and_set!(viz name => field_name, Type)`
///
/// This form generates simple initializer, getter and setter functions using the [`with`],
/// [`get`] and [`set`]  macros.
///
/// * In this form `name` is used in naming the generated functions while `field_name` is the
///   name of the structure's field.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// use jemmy::*;
/// # pub struct Address { number_on_street: u32, street_1_string: String, street_2: Option<String> }
/// impl Address {
///     // with_get_and_set!(pub street_1 => street_1_string, String);
///
///     with!(pub street_1 => street_1_string, String);
///     get_and_set!(pub street_1 => street_1_string, String);
/// }
/// ```
///
/// ### `with_get_and_set!(viz name => into Type)`
///
/// This form generates simple initializer, getter and setter functions using the [`with`],
/// [`get`] and [`set`]  macros.
///
/// * In this form `name` is used in both the naming of the generated functions and the
///   name of as the structure's field.
/// * The keyword `into` is passed to the [`with`] and [`set`] macros.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// use jemmy::*;
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // with_get_and_set!(pub street_1 => into String);
///
///     with!(pub street_1 => into String);
///     get_and_set!(pub street_1 => into String);
/// }
/// ```
///
/// ### `with_get_and_set!(viz name => field_name, into Type)`
///
/// This form generates simple initializer, getter and setter functions using the [`with`],
/// [`get`] and [`set`]  macros.
///
/// * In this form `name` is used in naming the generated functions while `field_name` is the
///   name of the structure's field.
/// * The keyword `into` is passed to the [`with`] and [`set`] macros.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// use jemmy::*;
/// # pub struct Address { number_on_street: u32, street_1_string: String, street_2: Option<String> }
/// impl Address {
///     // with_get_and_set!(pub street_1 => street_1_string, into String);
///
///     with!(pub street_1 => street_1_string, into String);
///     get_and_set!(pub street_1 => street_1_string, into String);
/// }
/// ```
///
/// ### `with_get_and_set!(viz name => optional Type)`
///
/// This form generates simple initializer, getter and setter functions using the [`with`],
/// [`get`] and [`set`]  macros.
///
/// * In this form `name` is used in both the naming of the generated functions and the
///   name of as the structure's field.
/// * The keyword `optional` is passed to the [`with`] and [`set`] macros.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// use jemmy::*;
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // with_get_and_set!(pub street_2 => optional String);
///
///     with!(pub street_2 => optional String);
///     get_and_set!(pub street_2 => optional String);
/// }
/// ```
///
/// ### `with_get_and_set!(viz name => field_name, optional Type)`
///
/// This form generates simple initializer, getter and setter functions using the [`with`],
/// [`get`] and [`set`]  macros.
///
/// * In this form `name` is used in naming the generated functions while `field_name` is the
///   name of the structure's field.
/// * The keyword `optional` is passed to the [`with`] and [`set`] macros.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// use jemmy::*;
/// # pub struct Address { number_on_street: u32, street_1: String, street_2_string: Option<String> }
/// impl Address {
///     // with_get_and_set!(pub street_2 => street_2_string, optional String);
///
///     with!(pub street_2 => street_2_string, optional String);
///     get_and_set!(pub street_2 => street_2_string, optional String);
/// }
/// ```
///
#[macro_export]
macro_rules! with_get_and_set {
    // Base case: `viz name => field_name, Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        $crate::with!($fn_vis $fn_name => $field_name, $value_type);
        $crate::get_and_set!($fn_vis $fn_name => $field_name, $value_type);
    };
    // Base case without *field name*: `viz name => Type`
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        $crate::with_get_and_set!($fn_vis $name => $name, $value_type);
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // (2) Base case with *copy*: `viz name => field_name, copy Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, copy $value_type:ty) => {
        $crate::with!($fn_vis $fn_name => $field_name, $value_type);
        $crate::get_and_set!($fn_vis $fn_name => $field_name, copy $value_type);
    };
    // Case (2) without *field name*: `viz name => copy Type`
    ($fn_vis:vis $name:ident => copy $value_type:ty) => {
        $crate::with_get_and_set!($fn_vis $name => $name, copy $value_type);
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // (3) Base case with *into*: `viz name => field_name, into Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, into $value_type:ty) => {
        $crate::with!($fn_vis $fn_name => $field_name, into $value_type);
        $crate::get_and_set!($fn_vis $fn_name => $field_name, into $value_type);
    };
    // Case (3) without *field name*: `viz name => into Type`
    ($fn_vis:vis $name:ident => into $value_type:ty) => {
        $crate::with_get_and_set!($fn_vis $name => $name, into $value_type);
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // (4) Base case with *optional*: `viz name => field_name, optional Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional $value_type:ty) => {
        $crate::with!($fn_vis $fn_name => $field_name, optional $value_type);
        $crate::get_and_set!($fn_vis $fn_name => $field_name, optional $value_type);
    };
    // Case (4) without *field name*: `viz name => optional Type`
    ($fn_vis:vis $name:ident => optional $value_type:ty) => {
        $crate::with_get_and_set!($fn_vis $name => $name, optional $value_type);
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // (6) Case (4) with *copy*: `viz name => field_name, optional copy Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional copy $value_type:ty) => {
        $crate::with!($fn_vis $fn_name => $field_name, optional $value_type);
        $crate::get_and_set!($fn_vis $fn_name => $field_name, optional copy $value_type);
    };
    // Case (6) without *field name*: `viz name => optional copy Type`
    ($fn_vis:vis $name:ident => optional copy $value_type:ty) => {
        $crate::with_get_and_set!($fn_vis $name => $name, optional copy $value_type);
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // (6) Case (4) with *into*: `viz name => field_name, optional into Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional into $value_type:ty) => {
        $crate::with!($fn_vis $fn_name => $field_name, optional into $value_type);
        $crate::get_and_set!($fn_vis $fn_name => $field_name, optional into $value_type);
    };
    // Case (6) without *field name*: `viz name => optional into Type`
    ($fn_vis:vis $name:ident => optional into $value_type:ty) => {
        $crate::with_get_and_set!($fn_vis $name => $name, optional into $value_type);
    };
}

// ------------------------------------------------------------------------------------------------
// Combinator Macros ❱ Get, Set, and Unset
// ------------------------------------------------------------------------------------------------

///
/// Generate [`get`], [`set`], and [`unset`] for an optional struct field.
///
/// ## Forms
///
/// ### `get_set_and_unset!(viz name => Type)`
///
/// This form generates simple getter, setter and un-setter functions using the [`get`],
/// [`set`] and [`unset`] macros.
///
/// * In this form `name` is used in both the naming of the generated functions and the
///   name of as the structure's field.
/// * Because unset is present, the `optional` keyword is used on the get and set macros.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// use jemmy::*;
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // get_set_and_unset!(pub street_2 => String);
///
///     get_and_set!(pub street_2 => optional String);
///     unset!(pub street_2);
/// }
/// ```
///
/// ### `get_set_and_unset!(viz name => field_name, Type)`
///
/// This form generates simple getter, setter and un-setter functions using the [`get`],
/// [`set`] and [`unset`] macros.
///
/// * In this form `name` is used in naming the generated functions while `field_name` is the
///   name of the structure's field.
/// * Because unset is present, the `optional` keyword is used on the get and set macros.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// use jemmy::*;
/// # pub struct Address { number_on_street: u32, street_1: String, street_2_string: Option<String> }
/// impl Address {
///     // get_set_and_unset!(pub street_2 => street_2_string, String);
///
///     get_and_set!(pub street_2 => street_2_string, optional String);
///     unset!(pub street_2 => street_2_string);
/// }
/// ```
///
#[macro_export]
macro_rules! get_set_and_unset {
    // Base case: `viz name => field_name, Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        $crate::get_and_set!($fn_vis $fn_name => $field_name, optional $value_type);
        $crate::unset!($fn_vis $fn_name => $field_name);
    };
    // Base case without *field name*: `viz name => Type`
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        $crate::get_set_and_unset!($fn_vis $name => $name, $value_type);
    };
}

// ------------------------------------------------------------------------------------------------
// Combinator Macros ❱ With, Get, Set, and Unset
// ------------------------------------------------------------------------------------------------

///
/// Generate [`with`], [`get`], [`set`], and [`unset`] for an optional struct field.
///
/// ## Forms
///
/// ### `with_get_set_and_unset!(viz name => Type)`
///
/// This form generates simple initializer, getter, setter and un-setter functions using the
/// [`with`], [`get`], [`set`] and [`unset`] macros.
///
/// * In this form `name` is used in both the naming of the generated functions and the
///   name of as the structure's field.
/// * Because unset is present, the `optional` keyword is used on the get and set macros.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// use jemmy::*;
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // with_get_set_and_unset!(pub street_2 => String);
///
///     with_get_and_set!(pub street_2 => optional String);
///     unset!(pub street_2);
/// }
/// ```
///
/// ### `with_get_set_and_unset!(viz name => field_name, Type)`
///
/// This form generates simple initializer, getter, setter and un-setter functions using the
/// [`with`], [`get`], [`set`] and [`unset`] macros.
///
/// * In this form `name` is used in naming the generated functions while `field_name` is the
///   name of the structure's field.
/// * Because unset is present, the `optional` keyword is used on the get and set macros.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// use jemmy::*;
/// # pub struct Address { number_on_street: u32, street_1: String, street_2_string: Option<String> }
/// impl Address {
///     // with_get_set_and_unset!(pub street_2 => street_2_string, String);
///
///     with_get_and_set!(pub street_2 => street_2_string, optional String);
///     unset!(pub street_2 => street_2_string);
/// }
/// ```
///
#[macro_export]
macro_rules! with_get_set_and_unset {
    // Base case: `viz name => field_name, Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        $crate::with_get_and_set!($fn_vis $fn_name => $field_name, optional $value_type);
        $crate::unset!($fn_vis $fn_name => $field_name);
    };
    // Base case without *field name*: `viz name => Type`
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        $crate::with_get_set_and_unset!($fn_vis $name => $name, $value_type);
    };
}

// ------------------------------------------------------------------------------------------------
// Re-export macros
// ------------------------------------------------------------------------------------------------

pub use crate::{get_and_set, get_set_and_unset, with_get_and_set, with_get_set_and_unset};

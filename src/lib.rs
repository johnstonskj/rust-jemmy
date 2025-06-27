/*!
This package provides a coherent set of manual accessor macros.

While a number of packages exist to simplify the addition of accessors to Rust structures and enumerations
these are often derive macros that come with a trade-off between flexibility and control. Jemmy takes a
different and more flexible approach. It provides a set of very simple leaf macros and then aggregate
macros that build common patterns from these building blocks. For example, you can add `get_and_set`
methods for a simple field, or `with_get_and_set` to include an initializer as well. You can use the
`get_set_and_unset` for optional fields which produces `foo() -> Option<&T>`, `set_foo(T)`, and `unset_foo()`
methods rather than a setter that takes an `Option<T>`.

The name Jemmy was chosen for this crate as the common name for a small pry-bar or crowbar used in burglary,
a very direct form of *manual access*.

## Example 1 - Structures

The following shows the basics of the macros for generating accessors for the fields of a structure. This
will provide initializers, getters and setters for the `number_on_street` and `street_1` fields. A getter,
setter (value) and unsetter (`None``) will be provided for the `street_2` field.

```
use jemmy::*;

#[derive(Default)]
pub struct Address {
    number_on_street: u32,
    street_1: String,
    street_2: Option<String>,
    // ...
}

impl Address {
    with!(pub number_on_street => u32);
    get!(pub number_on_street => copy u32);
    set!(pub number_on_street => u32);

    with_get_and_set!(pub street_1 => into String);
    get_set_and_unset!(pub street_2 => String);
}
```

## Example 2 - Enums

The following shows the basics of the macros for generating accessors for the variants of an enum. For
each variant an `is_variant` predicate and `as_variant` cast methods are provided.

```rust
# pub struct Address(String);
use jemmy::*;

pub enum TypedAddress {
    Home(Address),
    Work(Address),
    Other(Address),
}

impl TypedAddress {
    is_variant!(Home => Address);
    as_variant!(Home => Address);

    is_as_variant!(Work => Address);
    is_as_variant!(Other => Address);
}
```

## Standard Forms

The following are the primary forms supported by the macros in the crate; the first is the form of
all field macros, the second is the form of all variant macros.

1. `(viz name => [field_name,] [keywords] [type])`
2. `(viz enum, variant [=> type])`

The elements of these forms are described below.

1. **viz**; the vizibility specifier for the generated method.
2. for field macros:
   1. **name**;
   2. **field name**; (optional) when the visible name of the field is different from its field
      name you may specify both.
   3. **keywords** (optional):
      1. **copy**; denotes that the field type implements `Copy` and the generatted method will
         return a value rather than a reference.
      1. **into**; will generate a setter method that takes a parameter of type `Into<T>` rather
         than the typical `T`.
      2. **optional**; denotes that the field is an `Option<T>` not `T` which affects all getters
         and setters.
   4. **type**; (optional) the type of the field, specifically `T`, *do not* specify `Option<T>` or
      `Into<T>` if using the corresponding keywords.
3. for enum macros:
   1. **enum**; the enum type's identifier.
   2. **variant**; the variant's identifier within the enum.
   3. **type**; (optional) the type of any value of the variant.

 */

// ------------------------------------------------------------------------------------------------
// Accessor Macros ❱ Get | Set | Unset | With
// ------------------------------------------------------------------------------------------------

///
/// Generate a getter method for a struct field.
///
/// ```rust
/// pub struct Thing {
///     name: String,
///     size: u32,
///     description: Option<String>,
/// }
/// ```
///
/// ## Forms
///
/// ### `get!(viz name => Type)`
///
/// In this form `name` is both the name of the getter function and the name of the
/// structure field. The `Type`` of the getter function is the same as the field, although a reference will be
/// returned
///
/// ```rust
/// # pub struct Thing { name: String, size: u32, description: Option<String>, }
/// impl Thing {
///     // get!(pub name => String);
///     pub fn name(&self) -> &String {
///         &self.name
///     }
/// }
/// ```
///
/// ### `get!(viz getter_name => field_name, Type)`
///
/// As above, but allows independent naming of getter function and field name.
///
/// ```rust
/// # pub struct Thing { person_name: String, size: u32, description: Option<String>, }
/// impl Thing {
///     // get!(pub name => person_name, String);
///     pub fn name(&self) -> &String {
///         &self.person_name
///     }
/// }
/// ```
///
/// ### `get!(viz name => optional Type)`
///
/// As #1, except that type will be `Option<Type>` rather than `Type`.
///
/// ```rust
/// # pub struct Thing { name: String, size: u32, description: Option<String>, }
/// impl Thing {
///     // get!(pub description => optional String);
///     pub fn description(&self) -> Option<&String> {
///         self.description.as_ref()
///     }
/// }
/// ```
///
/// ### `get!(viz getter_name => field_name, optional Type)`
///
/// As #2 but for `Option<Type>`.
///
/// ```rust
/// # pub struct Thing { name: String, size: u32, desc: Option<String>, }
/// impl Thing {
///     // get!(pub description => desc, optional String);
///     pub fn description(&self) -> Option<&String> {
///         self.desc.as_ref()
///     }
/// }
/// ```
///
///
/// ### `get!(viz name => copy Type)`
///
/// As #1, except that type will be returned as `Type` rather than the reference `&Type`.
///
/// ```rust
/// # pub struct Thing { name: String, size: u32, description: Option<String>, }
/// impl Thing {
///     // get!(pub size => copy u32);
///     pub fn size(&self) -> u32 {
///         self.size
///     }
/// }
/// ```
///
/// ### `get!(viz getter_name => field_name, copy Type)`
///
/// As #2 but for `Option<Type>`.
///
/// ```rust
/// # pub struct Thing { name: String, thing_size: u32, description: Option<String>, }
/// impl Thing {
///     // get!(pub size => thing_size, copy u32);
///     pub fn size(&self) -> u32 {
///         self.thing_size
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! get {
    // Simple form: get!(viz name => Type)
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        $fn_vis fn $name(&self) -> &$value_type {
            &self.$name
        }
    };
    // Extended form: get!(viz getter_name => field_name, Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        $fn_vis fn $fn_name(&self) -> &$value_type {
            &self.$field_name
        }
    };
    // Option form: get!(viz name => optional Type)
    ($fn_vis:vis $name:ident => optional $value_type:ty) => {
        $fn_vis fn $name(&self) -> Option<&$value_type> {
            self.$name.as_ref()
        }
    };
    // Extended form: get!(viz getter_name => field_name, optional Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional $value_type:ty) => {
        $fn_vis fn $fn_name(&self) -> Option<&$value_type> {
            self.$field_name.as_ref()
        }
    };
    // Option form: get!(viz name => copy Type)
    ($fn_vis:vis $name:ident => copy $value_type:ty) => {
        $fn_vis fn $name(&self) -> $value_type {
            self.$name
        }
    };
    // Extended form: get!(viz getter_name => field_name, copy Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, copy $value_type:ty) => {
        $fn_vis fn $fn_name(&self) -> $value_type {
            self.$field_name
        }
    };
}

///
/// Generate a mutable getter method for a struct field.
///
/// ```rust
/// pub struct Thing {
///     name: String,
///     size: u32,
///     description: Option<String>,
/// }
/// ```
///
/// ## Forms
///
/// `get_mut!(viz name => Type)`
///
/// This form generates a mutable getter method for the field.
///
/// ```rust
/// # pub struct Thing { name: String, }
/// impl Thing {
///     // get_mut!(pub name => String);
///     pub fn name_mut(&mut self) -> &mut String {
///         &mut self.name
///     }
/// }
/// ```
///
/// `get_mut!(viz getter_name => field_name, Type)`
///
/// This form generates a mutable getter method for the field where the field's internal name
/// is different from the field's public name.
///
/// ```rust
/// # pub struct Thing { name_as_string: String, }
/// impl Thing {
///     // get_mut!(pub name => name_as_string, String);
///     pub fn name_mut(&mut self) -> &mut String {
///         &mut self.name_as_string
///     }
/// }
/// ```
///
///
#[macro_export]
macro_rules! get_mut {
    // Simple form: get_mut!(viz name => Type)
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        $fn_vis fn $name(&mut self) -> &mut $value_type {
            &mut self.$name
        }
    };
    // Extended form: get_mut!(viz getter_name => field_name, Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        $fn_vis fn $fn_name(&mut self) -> &mut $value_type {
            &mut self.$field_name
        }
    };
}

///
/// Generate a setter method for a struct field.
///
/// ## Forms
///
/// ### `set!(viz name => Type)`
///
/// TBD
///
/// ### `set!(viz setter_name => field_name, Type)`
///
/// TBD
///
/// ### `set!(viz name => into Type)`
///
/// TBD
///
/// ### `set!(viz setter_name => field_name, into Type)`
///
/// TBD
///
/// ### `set!(viz name => optional Type)`
///
/// TBD
///
/// ### `set!(viz setter_name => field_name, optional Type)`
///
/// TBD
///
///
#[macro_export]
macro_rules! set {
    // Simple form: set!(viz name => Type)
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<set_ $name>](&mut self, value: $value_type) {
                self.$name = value;
            }
        }
    };
    // Extended form: set!(viz setter_name => field_name, Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<set_ $fn_name>](&mut self, value: $value_type) {
                self.$field_name = value;
            }
        }
    };
    // Into form: set!(viz name => into Type)
    ($fn_vis:vis $name:ident => into $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<set_ $name>]<T: Into<$value_type>>(&mut self, value: T) {
                self.$name = value.into();
            }
        }
    };
    // Extended Into form: set!(viz setter_name => field_name, into Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, into $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<set_ $fn_name>]<T: Into<$value_type>>(&mut self, value: T) {
                self.$field_name = value.into();
            }
        }
    };
    // Optional form: set!(viz name => optional Type)
    ($fn_vis:vis $name:ident => optional $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<set_ $name>](&mut self, value: $value_type) {
                self.$name = Some(value);
            }
        }
    };
    // Extended Optional form: set!(viz setter_name => field_name, optional Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<set_ $fn_name>](&mut self, value: $value_type) {
                self.$field_name = Some(value);
            }
        }
    };
}

///
///  Generate an *un-setter* method for an Optional struct field.
///
/// ## Forms
///
/// ### `unset!(viz name => Type)`
///
/// TBD
///
/// ### `unset!(viz setter_name => field_name, Type)`
///
/// TBD
///
///
#[macro_export]
macro_rules! unset {
    // Simple form: unset!(viz name => Type)
    ($fn_vis:vis $name:ident) => {
        paste::paste! {
            $fn_vis fn [<unset_ $name>](&mut self,) {
                self.$name = None;
            }
        }
    };
    // Extended form: unset!(viz setter_name => field_name, Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident) => {
        paste::paste! {
            $fn_vis fn [<unset_ $fn_name>](&mut self) {
                self.$field_name = None;
            }
        }
    };
}

///
///  Generate a builder-style initializerer method for a struct field.
///
/// ## Forms
///
/// ### `with!(viz name => Type)`
///
/// TBD
///
/// ### `with!(viz with_name => field_name, Type)`
///
/// TBD
///
/// ### `with!(viz name => into Type)`
///
/// TBD
///
/// ### `with!(viz with_name => field_name, into Type)`
///
/// TBD
///
/// ### `with!(viz name => optional Type)`
///
/// TBD
///
/// ### `with!(viz with_name => field_name, optional Type)`
///
/// TBD
///
///
#[macro_export]
macro_rules! with {
    // Simple form: with!(viz name => Type)
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<with_ $name>](mut self, value: $value_type) -> Self {
                self.$name = value;
                self
            }
        }
    };
    // Extended form: with!(viz with_name => field_name, Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<with_ $fn_name>](mut self, value: $value_type) -> Self {
                self.$field_name = value;
                self
            }
        }
    };
    // Into form: with!(viz name => into Type)
    ($fn_vis:vis $name:ident => into $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<with_ $name>]<T: Into<$value_type>>(mut self, value: T) -> Self {
                self.$name = value.into();
                self
            }
        }
    };
    // Extended Into form: with!(viz with_name => field_name, into Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, into $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<with_ $fn_name>]<T: Into<$value_type>>(mut self, value: T) -> Self {
                self.$field_name = value.into();
                self
            }
        }
    };
    // Optional form: with!(viz name => optional Type)
    ($fn_vis:vis $name:ident => optional $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<with_ $name>](mut self, value: $value_type) -> Self {
                self.$name = Some(value);
                self
            }
        }
    };
    // Extended Optional form: with!(viz with_name => field_name, optional Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<with_ $fn_name>](mut self, value: $value_type) -> Self {
                self.$field_name = Some(value);
                self
            }
        }
    };
}

///
/// Combine [get] and [set] for a struct field.
///
#[macro_export]
macro_rules! get_and_set {
    // Simple form: set!(viz name => Type)
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        $crate::get!($fn_vis $name => $value_type);
        $crate::set!($fn_vis $name => $value_type);
    };
    // Extended form: set!(viz setter_name => field_name, Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        $crate::get!($fn_vis $fn_name => $field_name, $value_type);
        $crate::set!($fn_vis $fn_name => $field_name, $value_type);
    };
    // Into form: set!(viz name => into Type)
    ($fn_vis:vis $name:ident => into $value_type:ty) => {
        $crate::get!($fn_vis $name => $value_type);
        $crate::set!($fn_vis $name => into $value_type);
    };
    // Extended Into form: set!(viz setter_name => field_name, into Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, into $value_type:ty) => {
        $crate::get!($fn_vis $fn_name => $field_name, $value_type);
        $crate::set!($fn_vis $fn_name => $field_name, into $value_type);
     };
    // Optional form: set!(viz name => optional Type)
    ($fn_vis:vis $name:ident => optional $value_type:ty) => {
        $crate::get!($fn_vis $name => optional $value_type);
        $crate::set!($fn_vis $name => optional $value_type);
    };
    // Extended Optional form: set!(viz setter_name => field_name, optional Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional $value_type:ty) => {
        $crate::get!($fn_vis $fn_name => $field_name, optional $value_type);
        $crate::set!($fn_vis $fn_name => $field_name, optional $value_type);
    };
}

///
/// Combine [with], [get] and [set] for a struct field.
///
#[macro_export]
macro_rules! with_get_and_set {
    // Simple form: set!(viz name => Type)
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        $crate::with!($fn_vis $name => $value_type);
        $crate::get_and_set!($fn_vis $name => $value_type);
    };
    // Extended form: set!(viz setter_name => field_name, Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        $crate::with!($fn_vis $fn_name => $field_name, $value_type);
        $crate::get_and_set!($fn_vis $fn_name => $field_name, $value_type);
     };
    // Into form: set!(viz name => into Type)
    ($fn_vis:vis $name:ident => into $value_type:ty) => {
        $crate::with!($fn_vis $name => into $value_type);
        $crate::get_and_set!($fn_vis $name => into $value_type);
    };
    // Extended Into form: set!(viz setter_name => field_name, into Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, into $value_type:ty) => {
        $crate::with!($fn_vis $fn_name => $field_name, into $value_type);
        $crate::get_and_set!($fn_vis $fn_name => $field_name, into $value_type);
     };
    // Optional form: set!(viz name => optional Type)
    ($fn_vis:vis $name:ident => optional $value_type:ty) => {
        $crate::with!($fn_vis $name => optional $value_type);
        $crate::get_and_set!($fn_vis $name => optional $value_type);
    };
    // Extended Optional form: set!(viz setter_name => field_name, optional Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional $value_type:ty) => {
        $crate::with!($fn_vis $fn_name => $field_name, optional $value_type);
        $crate::get_and_set!($fn_vis $fn_name => $field_name, optional $value_type);
    };
}

///
/// Combine [get], [set], and [unset] for an optional struct field.
///
#[macro_export]
macro_rules! get_set_and_unset {
    // Simple form: seget_set_and_unsett!(viz name => Type)
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        $crate::get_and_set!($fn_vis $name => optional $value_type);
        $crate::unset!($fn_vis $name);
    };
    // Extended form: get_set_and_unset!(viz setter_name => field_name, Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        $crate::get_and_set!($fn_vis $fn_name => $field_name, optional $value_type);
        $crate::unset!($fn_vis $fn_name => $field_name);
    };
}

/// Re-export macros for struct field access.
pub mod access {
    pub use crate::{get, get_and_set, get_mut, get_set_and_unset, set, with, with_get_and_set};
}

// ------------------------------------------------------------------------------------------------
// Variant Macros ❱ is_* | as_*
// ------------------------------------------------------------------------------------------------

///
///  Generate a simple predicate for variant checking.
///
/// ```rust
/// # pub struct Address(String);
/// pub enum TypedAddress {
///     Home(Address),
///     Work(Address),
///     Unparsed(String),
///     Unknown
/// }
/// ```
///
///  ## Forms
///
/// ### `is_variant!(viz Variant)`
///
/// TBD
///
/// ```rust
/// # pub struct Address(String);
/// # pub enum TypedAddress { Home(Address), Unknown, }
/// impl TypedAddress {
///     // is_variant!(pub Unknown);
///     pub fn is_unknown(&self) -> bool {
///         matches!(self, Self::Unknown)
///     }
/// }
/// ```
///
/// ### `is_variant!(viz Variant => Type)`
///
/// TBD
///
/// ```rust
/// # pub struct Address(String);
/// # pub enum TypedAddress { Home(Address), }
/// impl TypedAddress {
///     // is_variant!(pub Home => Address);
///     pub fn is_home(&self) -> bool {
///         matches!(self, Self::Home(_))
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! is_variant {
    ($fn_vis:vis $variant_name:ident) => {
        paste::paste! {
            $fn_vis fn [< is_ $variant_name:snake >](&self) -> bool {
                matches!(self, Self::$variant_name)
            }
        }
    };
    ($fn_vis:vis $variant_name:ident => $variant_type:ty) => {
        paste::paste! {
            $fn_vis fn [< is_ $variant_name:snake >](&self) -> bool {
                matches!(self, Self::$variant_name(_))
            }
        }
    };
}

///
///  Generate a safe cast method for data held by a variant.
///
/// ## Forms
///
/// ### `as_variant!(viz Enum, Variant => Type)`
///
/// TBD
///
/// ```rust
/// # pub struct Address(String);
/// # pub enum TypedAddress { Home(Address), }
/// impl TypedAddress {
///     // as_variant!(pub Home => Address);
///     pub fn as_home(&self) -> Option<&Address> {
///         match self {
///             Self::Home(value) => Some(value),
///             _ => None,
///         }
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! as_variant {
    ($fn_vis:vis $variant_name:ident => $variant_type:ty) => {
        paste::paste! {
            $fn_vis fn [< as_ $variant_name:snake >](&self) -> Option<&$variant_type> {
                match self {
                    Self::$variant_name(value) => Some(value),
                    _ => None,
                }
            }
        }
    };
}

///
///  Generate both a simple predicate for variant checking, and
///  a safe cast method for data held by a variant.
///
/// ## Forms
///
/// ### `is_as_variant!(viz Enum, Variant => Type)`
///
/// TBD
///
/// ```rust
/// # pub struct Address(String);
/// # pub enum TypedAddress { Home(Address), }
/// impl TypedAddress {
///     // is_as_variant!(pub Home => Address);
///     pub fn is_home(&self) -> bool {
///         matches!(self, Self::Home(_))
///     }
///     pub fn as_home(&self) -> Option<&Address> {
///         match self {
///             Self::Home(value) => Some(value),
///             _ => None,
///         }
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! is_as_variant {
    ($fn_vis:vis $variant_name:ident => $variant_type:ty) => {
        paste::paste! {
            $crate::is_variant!($fn_vis $variant_name => $variant_type);
            $crate::as_variant!($fn_vis $variant_name => $variant_type);
        }
    };
}

/// Re-export macros for enum variant access.
pub mod variant {
    pub use crate::{as_variant, is_as_variant, is_variant};
}

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
setter (value) and unsetter (`None`) will be provided for the `street_2` field.

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
/// Generate a getter method for a field within a structure.
//
/// ## Forms
///
/// ### `get!(viz name => Type)`
///
/// This form generates an immutable getter method for a field within a structure.
///
/// * In this form `name` is both the name of the generated function and the name of the
///   structure's field.
/// * The type of the generated function is the reference type `&Type`.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { street_1: String }
/// impl Address {
///     // get!(pub street_1 => String);
///     pub const fn street_1(&self) -> &String {
///         &self.street_1
///     }
/// }
/// ```
///
/// ### `get!(viz getter_name => field_name, Type)`
///
/// This form generates an immutable getter method for a field within a structure.
///
/// * In this form `name` is the name of the generated function while `field_name` is the
///   name of the structure's field.
/// * the type of the generated function is the reference type `&Type`.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { street_or_building: String }
/// impl Address {
///     // get!(pub street_1 => street_or_building, String);
///     pub const fn street_1(&self) -> &String {
///         &self.street_or_building
///     }
/// }
/// ```
///
/// ### `get!(viz name => optional Type)`
///
/// This form generates an immutable getter method for a field within a structure.
///
/// * In this form `name` is both the name of the generated function and the name of the
///   structure's field.
/// * The type of the generated function is the reference `Option<&Type>`.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { street_2: Option<String>, }
/// impl Address {
///     // get!(pub street_2 => optional String);
///     pub const fn street_2(&self) -> Option<&String> {
///         self.street_2.as_ref()
///     }
/// }
/// ```
///
/// ### `get!(viz getter_name => field_name, optional Type)`
///
/// This form generates an immutable getter method for a field within a structure.
///
/// * In this form `name` is the name of the generated function while `field_name` is the
///   name of the structure's field.
/// * The type of the getter function is the reference `Option<&Type>`.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { street_additional: Option<String>, }
/// impl Address {
///     // get!(pub street_2 => street_additional, optional String);
///     pub const fn street_2(&self) -> Option<&String> {
///         self.street_additional.as_ref()
///     }
/// }
/// ```
///
///
/// ### `get!(viz name => copy Type)`
///
/// This form generates an immutable getter method for a field within a structure.
///
/// * In this form `name` is both the name of the generated function and the name of the
///   structure's field.
/// * The type of the generated function is the value type `Type`, assuming `Type` implements `Copy`.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, }
/// impl Address {
///     // get!(pub number_on_street => copy u32);
///     pub const fn number_on_street(&self) -> u32 {
///         self.number_on_street
///     }
/// }
/// ```
///
/// ### `get!(viz getter_name => field_name, copy Type)`
///
/// This form generates an immutable getter method for a field within a structure.
///
/// * In this form `name` is the name of the generated function while `field_name` is the
///   name of the structure's field.
/// * The type of the generated function is the value type `Type`, assuming `Type` implements `Copy`.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number: u32, }
/// impl Address {
///     // get!(pub number_on_street => number, copy u32);
///     pub const fn number_on_street(&self) -> u32 {
///         self.number
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! get {
    // Simple form: get!(viz name => Type)
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        $fn_vis const fn $name(&self) -> &$value_type {
            &self.$name
        }
    };
    // Extended form: get!(viz getter_name => field_name, Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        $fn_vis const fn $fn_name(&self) -> &$value_type {
            &self.$field_name
        }
    };
    // Option form: get!(viz name => optional Type)
    ($fn_vis:vis $name:ident => optional $value_type:ty) => {
        $fn_vis const fn $name(&self) -> Option<&$value_type> {
            self.$name.as_ref()
        }
    };
    // Extended form: get!(viz getter_name => field_name, optional Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional $value_type:ty) => {
        $fn_vis const fn $fn_name(&self) -> Option<&$value_type> {
            self.$field_name.as_ref()
        }
    };
    // Option form: get!(viz name => copy Type)
    ($fn_vis:vis $name:ident => copy $value_type:ty) => {
        $fn_vis const fn $name(&self) -> $value_type {
            self.$name
        }
    };
    // Extended form: get!(viz getter_name => field_name, copy Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, copy $value_type:ty) => {
        $fn_vis const fn $fn_name(&self) -> $value_type {
            self.$field_name
        }
    };
}

///
/// Generate a mutable getter method for a struct field.
///
///
/// ## Forms
///
/// `get_mut!(viz name => Type)`
///
/// This form generates a mutable getter method for a field within a structure.
///
/// * In this form `name` is both the name of the generated function and the name of the
///   structure's field.
/// * This form requires mutability in the form of a mutable reference to self; `&mut self`.
/// * The type of the generated function is the reference type `&mut Type`.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // get_mut!(pub street_1 => String);
///     pub const fn street_1_mut(&mut self) -> &mut String {
///         &mut self.street_1
///     }
/// }
/// ```
///
/// `get_mut!(viz getter_name => field_name, Type)`
///
/// This form generates a mutable getter method for a field within a structure.
///
/// * In this form `name` is the name of the generated function while `field_name` is the
///   name of the structure's field.
/// * This form requires mutability in the form of a mutable reference to self; `&mut self`.
/// * The type of the generated function is the reference `Option<&Type>`.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1_string: String, street_2: Option<String> }
/// impl Address {
///     // get_mut!(pub street_1 => street_1_string, String);
///     pub const fn street_1_mut(&mut self) -> &mut String {
///         &mut self.street_1_string
///     }
/// }
/// ```
///
///
#[macro_export]
macro_rules! get_mut {
    // Simple form: get_mut!(viz name => Type)
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        $fn_vis const fn $name(&mut self) -> &mut $value_type {
            &mut self.$name
        }
    };
    // Extended form: get_mut!(viz getter_name => field_name, Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        $fn_vis const fn $fn_name(&mut self) -> &mut $value_type {
            &mut self.$field_name
        }
    };
}

///
/// Generate a setter method for a field within a structure.
///
/// ## Forms
///
/// ### `set!(viz name => Type)`
///
/// This form generates a simple setter function.
///
/// * In this form `name` is both the name of the generated function and the name of the
///   structure's field.
/// * This form requires mutability in the form of a mutable reference to self; `&mut self`.
/// * The type of the new value parameter is the value type `Type`.
/// * This function returns no value.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // set!(pub number_on_street => u32);
///     pub fn number_on_street(&mut self, number_on_street: u32) {
///         self.number_on_street = number_on_street;
///     }
/// }
/// ```
///
/// ### `set!(viz setter_name => field_name, Type)`
///
/// This form generates a simple setter function.
///
/// * In this form `name` is the name of the generated function while `field_name` is the
///   name of the structure's field.
/// * This form requires mutability in the form of a mutable reference to self; `&mut self`.
/// * The type of the new value parameter is the value type `Type`.
/// * This function returns no value.
//
/// ```rust
/// # pub struct Address { number: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // set!(pub number_on_street => number, u32);
///     pub fn number_on_street(&mut self, number_on_street: u32) {
///         self.number = number_on_street;
///     }
/// }
/// ```
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ### `set!(viz name => into Type)`
///
/// This form generates a simple setter function.
///
/// * In this form `name` is both the name of the generated function and the name of the
///   structure's field.
/// * This form requires mutability in the form of a mutable reference to self; `&mut self`.
/// * The type of the new value parameter is the trait-bound type `T: Into<Type>` rather
///   than `Type` for flexibility.
/// * This function returns no value.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // set!(pub street_1 => into String);
///     pub fn street_1<T: Into<String>>(&mut self, street_1: T) {
///         self.street_1 = street_1.into();
///     }
/// }
/// ```
///
/// ### `set!(viz setter_name => field_name, into Type)`
///
/// This form generates a simple setter function.
///
/// * In this form `name` is the name of the generated function while `field_name` is the
///   name of the structure's field.
/// * This form requires mutability in the form of a mutable reference to self; `&mut self`.
/// * The type of the new value parameter is the trait-bound type `T: Into<Type>` rather
///   than `Type` for flexibility.
/// * This function returns no value.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1_string: String, street_2: Option<String> }
/// impl Address {
///     // set!(pub street_1 => street_1_string, into String);
///     pub fn street_1<T: Into<String>>(&mut self, street_1: T) {
///         self.street_1_string = street_1.into();
///     }
/// }
/// ```
///
/// ### `set!(viz name => optional Type)`
///
/// This form generates a simple setter function, for optional fields.
///
/// * In this form `name` is both the name of the generated function and the name of the
///   structure's field.
/// * This form requires mutability in the form of a mutable reference to self; `&mut self`.
/// * While the type of the structure's field is `Option<Type>`, the type of the new value
///   parameter is simply `Type`. The use of this macro is intended to be paired with
///   an [`unset`] implementation.
/// * This function returns no value.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // set!(pub street_2 => optional String);
///     pub fn street_2(&mut self, street_2: String) {
///         self.street_2 = Some(street_2);
///     }
/// }
/// ```
///
/// ### `set!(viz setter_name => field_name, optional Type)`
///
/// This form generates a simple setter function, for optional fields.
///
/// * In this form `name` is the name of the generated function while `field_name` is the
///   name of the structure's field.
/// * This form requires mutability in the form of a mutable reference to self; `&mut self`.
/// * While the type of the structure's field is `Option<Type>`, the type of the new value
///   parameter is simply `Type`. The use of this macro is intended to be paired with
///   an [`unset`] implementation.
/// * This function returns no value.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2_string: Option<String> }
/// impl Address {
///     // set!(pub street_2 => street_2_string, optional String);
///     pub fn street_2(&mut self, street_2: String) {
///         self.street_2_string = Some(street_2);
///     }
/// }
/// ```
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
///  Generate an *un-setter* method for an optional struct field.
///
/// ## Forms
///
/// ### `unset!(viz name)`
///
/// This form generates a simple setter function for optional fields that sets the value
/// to `None`.
///
/// * In this form `name` is both the name of the generated function and the name of the
///   structure's field.
/// * This form requires mutability in the form of a mutable reference to self; `&mut self`.
/// * The use of this macro is intended to be paired with a [`set`] implementation using
///   the `optional` keyword.
/// * This function returns no value.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // unset!(pub street_2);
///     #[inline(always)]
///     pub fn street_2(&mut self) {
///         self.street_2 = None;
///     }
/// }
/// ```
///
/// ### `unset!(viz setter_name => field_name)`
///
/// This form generates a simple setter function for optional fields that sets the value
/// to `None`.
///
/// * In this form `name` is the name of the generated function while `field_name` is the
///   name of the structure's field.
/// * This form requires mutability in the form of a mutable reference to self; `&mut self`.
/// * The use of this macro is intended to be paired with a [`set`] implementation using
///   the `optional` keyword.
/// * This function returns no value.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2_string: Option<String> }
/// impl Address {
///     // set!(pub street_2 => street_2_string);
///     #[inline(always)]
///     pub fn street_2(&mut self) {
///         self.street_2_string = None;
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! unset {
    // Simple form: unset!(viz name)
    ($fn_vis:vis $name:ident) => {
        paste::paste! {
            #[inline(always)]
            $fn_vis fn [<unset_ $name>](&mut self,) {
                self.$name = None;
            }
        }
    };
    // Extended form: unset!(viz setter_name => field_name)
    ($fn_vis:vis $fn_name:ident => $field_name:ident) => {
        paste::paste! {
            #[inline(always)]
            $fn_vis fn [<unset_ $fn_name>](&mut self) {
                self.$field_name = None;
            }
        }
    };
}

///
///  Generate a builder style initializer method for a struct field.
///
/// ## Forms
///
/// ### `with!(viz name => Type)`
///
/// This form generates a builder-style function typically used to initialize a structure
/// field during construction.
///
/// * In this form `name` is both the name of the generated function and the name of the
///   structure's field.
/// * This form requires mutability in the form of a `mut self`, and returns `Self`, so
///   that initializers can be chained.
/// * The type of the new value parameter is the value type `Type`.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // with!(pub number_on_street => u32);
///     pub fn with_number_on_street(mut self, number_on_street: u32) -> Self {
///         self.number_on_street = number_on_street;
///         self
///     }
/// }
/// ```
///
/// ### `with!(viz with_name => field_name, Type)`
///
/// This form generates a builder-style function typically used to initialize a structure
/// field during construction.
///
/// * In this form `name` is the name of the generated function while `field_name` is the
///   name of the structure's field.
/// * This form requires mutability in the form of a `mut self`, and returns `Self`, so
///   that initializers can be chained.
/// * The type of the new value parameter is the value type `Type`.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // with!(pub number_on_street => number, u32);
///     pub fn with_number_on_street(mut self, number_on_street: u32) -> Self {
///         self.number = number_on_street;
///         self
///     }
/// }
/// ```
///
/// ### `with!(viz name => into Type)`
///
/// This form generates a builder-style function typically used to initialize a structure
/// field during construction.
///
/// * In this form `name` is both the name of the generated function and the name of the
///   structure's field.
/// * This form requires mutability in the form of a `mut self`, and returns `Self`, so
///   that initializers can be chained.
/// * The type of the new value parameter is the trait-bound type `T: Into<Type>` rather
///   than `Type` for flexibility.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // with!(pub street_1 => into String);
///     pub fn with_street_1<T: Into<String>>(mut self, street_1: T) -> Self {
///         self.street_1 = street_1.into();
///         self
///     }
/// }
/// ```
///
/// ### `with!(viz with_name => field_name, into Type)`
///
/// This form generates a builder-style function typically used to initialize a structure
/// field during construction.
///
/// * In this form `name` is the name of the generated function while `field_name` is the
///   name of the structure's field.
/// * This form requires mutability in the form of a `mut self`, and returns `Self`, so
///   that initializers can be chained.
/// * The type of the new value parameter is the trait-bound type `T: Into<Type>` rather
///   than `Type` for flexibility.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1_string: String, street_2: Option<String> }
/// impl Address {
///     // with!(pub street_1 => street_1_string, into String);
///     pub fn with_street_1<T: Into<String>>(mut self, street_1: T) -> Self {
///         self.street_1_string = street_1.into();
///         self
///     }
/// }
/// ```
///
/// ### `with!(viz name => optional Type)`
///
/// This form generates a builder-style function, for optional fields, typically used to initialize a structure
/// field during construction.
///
/// * In this form `name` is both the name of the generated function and the name of the
///   structure's field.
/// * This form requires mutability in the form of a `mut self`, and returns `Self`, so
///   that initializers can be chained.
/// * While the type of the structure's field is `Option<Type>`, the type of the new value
///   parameter is simply `Type`. The use of this macro is intended to be paired with
///   an [`unset`] implementation.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // with!(pub street_2 => optional String);
///     pub fn with_street_2(mut self, street_2: String) -> Self {
///         self.street_2 = Some(street_2);
///         self
///     }
/// }
/// ```
///
/// ### `with!(viz with_name => field_name, optional Type)`
///
/// This form generates a builder-style function, for optional fields, typically used to initialize a structure
/// field during construction.
///
/// * In this form `name` is the name of the generated function while `field_name` is the
///   name of the structure's field.
/// * This form requires mutability in the form of a `mut self`, and returns `Self`, so
///   that initializers can be chained.
/// * While the type of the structure's field is `Option<Type>`, the type of the new value
///   parameter is simply `Type`. The use of this macro is intended to be paired with
///   an [`unset`] implementation.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2_string: Option<String> }
/// impl Address {
///     // with!(pub street_2 => optional String);
///     pub fn with_street_2(mut self, street_2: String) -> Self {
///         self.street_2_string = Some(street_2);
///         self
///     }
/// }
/// ```
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
/// Generate [get] and [set] for a struct field.
///
/// ## Forms
///
/// ### `get_and_set!(viz name => Type)`
///
/// This form generates simple getter and setter functions using the [get] and [set]  macros.
///
/// * In this form `name` is used in both the naming of the generated functions and the
///   name of as the structure's field.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // get_and_set!(pub street_1 => String);
///     pub const fn street_1(&self) -> &String {
///         &self.street_1
///     }
///     pub fn set_street_1(&mut self, street_1: String) {
///        self.street_1 = street_1;
///     }
/// }
/// ```
///
/// ### `get_and_set!(viz name => field_name, Type)`
///
/// This form generates simple getter and setter functions using the [get] and [set]  macros.
///
/// * In this form `name` is used in naming the generated functions while `field_name` is the
///   name of the structure's field.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1_string: String, street_2: Option<String> }
/// impl Address {
///     // get_and_set!(pub street_1 => String);
///     pub const fn street_1(&self) -> &String {
///         &self.street_1_string
///     }
///     pub fn set_street_1(&mut self, street_1: String) {
///        self.street_1_string = street_1;
///     }
/// }
/// ```
///
/// ### `get_and_set!(viz name => into Type)`
///
/// This form generates simple getter and setter functions using the [get] and [set]  macros.
///
/// * In this form `name` is used in both the naming of the generated functions and the
///   name of as the structure's field.
/// * The keyword `into` is passed to the [set] macro.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // get_and_set!(pub street_1 => String);
///     pub const fn street_1(&self) -> &String {
///         &self.street_1
///     }
///     pub fn set_street_1<T: Into<String>>(&mut self, street_1: T) {
///        self.street_1 = street_1.into();
///     }
/// }
/// ```
///
/// ### `get_and_set!(viz name => field_name, into Type)`
///
/// This form generates simple getter and setter functions using the [get] and [set]  macros.
///
/// * In this form `name` is used in naming the generated functions while `field_name` is the
///   name of the structure's field.
/// * The keyword `into` is passed to the [set] macro.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1_string: String, street_2: Option<String> }
/// impl Address {
///     // get_and_set!(pub street_1 => street_1_string, String);
///     pub const fn street_1(&self) -> &String {
///         &self.street_1_string
///     }
///     pub fn set_street_1<T: Into<String>>(&mut self, street_1: T) {
///        self.street_1_string = street_1.into();
///     }
/// }
/// ```
///
/// ### `get_and_set!(viz name => optional Type)`
///
/// This form generates simple getter and setter functions using the [get] and [set]  macros.
///
/// * In this form `name` is used in both the naming of the generated functions and the
///   name of as the structure's field.
/// * The keyword `optional` is passed to both both [get] and [set] macros.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // get_and_set!(pub street_2 => optional String);
///     pub const fn street_2(&self) -> Option<&String> {
///         self.street_2.as_ref()
///     }
///     pub fn set_street_2(&mut self, street_2: String) {
///        self.street_2 = Some(street_2);
///     }
/// }
/// ```
///
/// ### `get_and_set!(viz name => field_name, optional Type)`
///
/// This form generates simple getter and setter functions using the [get] and [set]  macros.
///
/// * In this form `name` is used in naming the generated functions while `field_name` is the
///   name of the structure's field.
/// * The keyword `optional` is passed to both both [get] and [set] macros.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2_string: Option<String> }
/// impl Address {
///     // get_and_set!(pub street_2 => street_2_string, optional String);
///     pub const fn street_2(&self) -> Option<&String> {
///         self.street_2_string.as_ref()
///     }
///     pub fn set_street_2(&mut self, street_2: String) {
///        self.street_2_string = Some(street_2);
///     }
/// }
/// ```
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
/// Generate [with], [get] and [set] for a struct field.
///
/// ## Forms
///
/// ### `with_get_and_set!(viz name => Type)`
///
/// This form generates simple initializer, getter and setter functions using the [with],
/// [get] and [set]  macros.
///
/// * In this form `name` is used in both the naming of the generated functions and the
///   name of as the structure's field.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // get_and_set!(pub street_1 => String);
///     pub fn with_street_1(mut self, street_1: String) -> Self {
///         self.street_1 = street_1;
///         self
///     }
///     pub const fn street_1(&self) -> &String {
///         &self.street_1
///     }
///     pub fn set_street_1(&mut self, street_1: String) {
///        self.street_1 = street_1;
///     }
/// }
/// ```
///
/// ### `with_get_and_set!(viz name => field_name, Type)`
///
/// This form generates simple initializer, getter and setter functions using the [with],
/// [get] and [set]  macros.
///
/// * In this form `name` is used in naming the generated functions while `field_name` is the
///   name of the structure's field.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1_string: String, street_2: Option<String> }
/// impl Address {
///     // get_and_set!(pub street_1 => street_1_string, String);
///     pub fn with_street_1(mut self, street_1: String) -> Self {
///         self.street_1_string = street_1;
///         self
///     }
///     pub const fn street_1(&self) -> &String {
///         &self.street_1_string
///     }
///     pub fn set_street_1(&mut self, street_1: String) {
///        self.street_1_string = street_1;
///     }
/// }
/// ```
///
/// ### `with_get_and_set!(viz name => into Type)`
///
/// This form generates simple initializer, getter and setter functions using the [with],
/// [get] and [set]  macros.
///
/// * In this form `name` is used in both the naming of the generated functions and the
///   name of as the structure's field.
/// * The keyword `into` is passed to the [with] and [set] macros.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // get_and_set!(pub street_1 => into String);
///     pub fn with_street_1<T: Into<String>>(mut self, street_1: T) -> Self {
///         self.street_1 = street_1.into();
///         self
///     }
///     pub const fn street_1(&self) -> &String {
///         &self.street_1
///     }
///     pub fn set_street_1<T: Into<String>>(&mut self, street_1: T) {
///        self.street_1 = street_1.into();
///     }
/// }
/// ```
///
/// ### `with_get_and_set!(viz name => field_name, into Type)`
///
/// This form generates simple initializer, getter and setter functions using the [with],
/// [get] and [set]  macros.
///
/// * In this form `name` is used in naming the generated functions while `field_name` is the
///   name of the structure's field.
/// * The keyword `into` is passed to the [with] and [set] macros.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1_string: String, street_2: Option<String> }
/// impl Address {
///     // get_and_set!(pub street_1 => street_1_string, into String);
///     pub fn with_street_1<T: Into<String>>(mut self, street_1: T) -> Self {
///         self.street_1_string = street_1.into();
///         self
///     }
///     pub const fn street_1(&self) -> &String {
///         &self.street_1_string
///     }
///     pub fn set_street_1<T: Into<String>>(&mut self, street_1: T) {
///        self.street_1_string = street_1.into();
///     }
/// }
/// ```
///
/// ### `with_get_and_set!(viz name => optional Type)`
///
/// This form generates simple initializer, getter and setter functions using the [with],
/// [get] and [set]  macros.
///
/// * In this form `name` is used in both the naming of the generated functions and the
///   name of as the structure's field.
/// * The keyword `optional` is passed to the [with] and [set] macros.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // get_and_set!(pub street_1 => optional String);
///     pub fn with_street_2(mut self, street_2: String) -> Self {
///         self.street_2 = Some(street_2);
///         self
///     }
///     pub const fn street_2(&self) -> Option<&String> {
///         self.street_2.as_ref()
///     }
///     pub fn set_street_2(&mut self, street_2: String) {
///        self.street_2 = Some(street_2);
///     }
/// }
/// ```
///
/// ### `with_get_and_set!(viz name => field_name, optional Type)`
///
/// This form generates simple initializer, getter and setter functions using the [with],
/// [get] and [set]  macros.
///
/// * In this form `name` is used in naming the generated functions while `field_name` is the
///   name of the structure's field.
/// * The keyword `optional` is passed to the [with] and [set] macros.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1_string: String, street_2: Option<String> }
/// impl Address {
///     // get_and_set!(pub street_1 => street_1_string, optional String);
///     pub fn with_street_1(mut self, street_1: String) -> Self {
///         self.street_1_string = street_1;
///         self
///     }
///     pub const fn street_1(&self) -> &String {
///         &self.street_1_string
///     }
///     pub fn set_street_1(&mut self, street_1: String) {
///        self.street_1_string = street_1;
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! with_get_and_set {
    // Simple form: set!(viz name => Type)
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        $crate::with!($fn_vis $name => $value_type);
        $crate::get_and_set!($fn_vis $name => $value_type);
    };
    // Extended form: set!(viz name => field_name, Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        $crate::with!($fn_vis $fn_name => $field_name, $value_type);
        $crate::get_and_set!($fn_vis $fn_name => $field_name, $value_type);
     };
    // Into form: set!(viz name => into Type)
    ($fn_vis:vis $name:ident => into $value_type:ty) => {
        $crate::with!($fn_vis $name => into $value_type);
        $crate::get_and_set!($fn_vis $name => into $value_type);
    };
    // Extended Into form: set!(viz name => field_name, into Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, into $value_type:ty) => {
        $crate::with!($fn_vis $fn_name => $field_name, into $value_type);
        $crate::get_and_set!($fn_vis $fn_name => $field_name, into $value_type);
     };
    // Optional form: set!(viz name => optional Type)
    ($fn_vis:vis $name:ident => optional $value_type:ty) => {
        $crate::with!($fn_vis $name => optional $value_type);
        $crate::get_and_set!($fn_vis $name => optional $value_type);
    };
    // Extended Optional form: set!(viz name => field_name, optional Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional $value_type:ty) => {
        $crate::with!($fn_vis $fn_name => $field_name, optional $value_type);
        $crate::get_and_set!($fn_vis $fn_name => $field_name, optional $value_type);
    };
}

///
/// Generate [get], [set], and [unset] for an optional struct field.
///
/// ## Forms
///
/// ### `get_set_and_unset!(viz name => Type)`
///
/// This form generates simple getter, setter and un-setter functions using the [get],
/// [set] and [unset] macros.
///
/// * In this form `name` is used in both the naming of the generated functions and the
///   name of as the structure's field.
/// * Because unset is present, the `optional` keyword is used on the get and set macros.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // get_set_and_unset!(pub street_2 => String);
///     pub const fn street_2(&self) -> Option<&String> {
///         self.street_2.as_ref()
///     }
///     pub fn set_street_2(&mut self, street_2: String) {
///        self.street_2 = Some(street_2);
///     }
///     #[inline(always)]
///     pub fn unset_street_2(&mut self) {
///       self.street_2 = None;
///     }
/// }
/// ```
///
/// ### `get_set_and_unset!(viz name => field_name, Type)`
///
/// This form generates simple getter, setter and un-setter functions using the [get],
/// [set] and [unset] macros.
///
/// * In this form `name` is used in naming the generated functions while `field_name` is the
///   name of the structure's field.
/// * Because unset is present, the `optional` keyword is used on the get and set macros.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { number_on_street: u32, street_1: String, street_2: Option<String> }
/// impl Address {
///     // get_set_and_unset!(pub street_2 => String);
///     pub const fn street_2(&self) -> Option<&String> {
///         self.street_2.as_ref()
///     }
///     pub fn set_street_2(&mut self, street_2: String) {
///        self.street_2 = Some(street_2);
///     }
///     #[inline(always)]
///     pub fn unset_street_2(&mut self) {
///       self.street_2 = None;
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! get_set_and_unset {
    // Simple form: get_set_and_unset!(viz name => Type)
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        $crate::get_and_set!($fn_vis $name => optional $value_type);
        $crate::unset!($fn_vis $name);
    };
    // Extended form: get_set_and_unset!(viz name => field_name, Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        $crate::get_and_set!($fn_vis $fn_name => $field_name, optional $value_type);
        $crate::unset!($fn_vis $fn_name => $field_name);
    };
}

///
/// Re-export macros for struct field access.
///
/// The following struct definition is used as an example in the forms below to demonstrate
/// the generated code.
///
/// ```rust
/// pub struct Address {
///     number_on_street: u32,    // demonstrates keyword 'copy'
///     street_1: String,         // demonstrates keyword 'into'
///     street_2: Option<String>, // demonstrates keyword 'optional'
///     // ...
/// }
/// ```
///
/// ## Summary
///
/// | Macro    | field name       | keywords | type   | generated signature                                                 |
/// |----------|------------------|----------|--------|---------------------------------------------------------------------|
/// | `with!`  | number_on_street |          | u32    | `fn with_number_on_street(mut self, number_on_street: u32) -> Self` |
/// | `with!`  | street_1         | into     | String | `fn with_street_1<T: Into<String>(mut self, street_1: T) -> Self`   |
/// | `with!`  | street_2         | optional | String | `fn with_street_2(mut self, street_2: String) -> Self`              |
/// | `get!`   | number_on_street | copy     | u32    | `const fn number_on_street(&self) -> u32`                           |
/// | `get!`   | street_1         |          | String | `const fn street_1(&self) -> &String`                               |
/// | `get!`   | street_2         | optional | String | `const fn street_2(&self) -> Option<&String>`                       |
/// | `set!`   | number_on_street |          | u32    | `fn set_number_on_street(&mut self, number_on_street: u32)`         |
/// | `set!`   | street_1         | into     | String | `fn set_street_1<T: Into<String>(&mut self, street_1: T)`           |
/// | `set!`   | street_2         | optional | String | `fn set_street_2(&mut self, street_1: String)`                      |
/// | `unset!` | street_2         |          | String | `fn unset_street_2(&mut self)`                                      |
///
pub mod access {
    pub use crate::{get, get_and_set, get_mut, get_set_and_unset, set, with, with_get_and_set};
}

// ------------------------------------------------------------------------------------------------
// Variant Macros ❱ is_* | as_*
// ------------------------------------------------------------------------------------------------

///
/// Generate a predicate function that returns `true` if the instance of the corresponding
/// enumeration is of the given variant.
///
///  ## Forms
///
/// ### `is_variant!(viz Variant)`
///
/// This form generates a predicate function to test variant *selection*, it is a synonym
/// for `is_variant!(viz Variant => ())`, see next.
///
/// ### `is_variant!(viz Variant => ())`
///
/// This form generates a predicate function to test variant *selection* for variants that do
/// not carry data in tuple form.
///
/// * The value `()` acts as a marker denoting the lack of data.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address(String);
/// # pub enum TypedAddress { Home(Address), Unknown, UnknownAlso }
/// impl TypedAddress {
///     // is_variant!(pub Unknown);
///     pub const fn is_unknown(&self) -> bool {
///         matches!(self, Self::Unknown)
///     }
///     // is_variant!(pub UnknownAlso => ());
///     pub const fn is_unknown_also(&self) -> bool {
///         matches!(self, Self::UnknownAlso)
///     }
/// }
/// ```
///
/// ### `is_variant!(viz Variant => Type)`
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
/// # pub enum TypedAddress { Home(Address), }
/// impl TypedAddress {
///     // is_variant!(pub Home => Address);
///     pub const fn is_home(&self) -> bool {
///         matches!(self, Self::Home(_))
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! is_variant {
    ($fn_vis:vis $variant_name:ident) => {
        $crate::is_variant!($fn_vis $variant_name => ());
    };
     ($fn_vis:vis $variant_name:ident => ()) => {
        paste::paste! {
            $fn_vis const fn [< is_ $variant_name:snake >](&self) -> bool {
                matches!(self, Self::$variant_name)
            }
        }
    };
    ($fn_vis:vis $variant_name:ident => $variant_type:ty) => {
        paste::paste! {
            $fn_vis const fn [< is_ $variant_name:snake >](&self) -> bool {
                matches!(self, Self::$variant_name(_))
            }
        }
    };
}

///
///  Generate a *safe cast* method for variant-associated data.
///
/// ## Forms
///
/// ### `as_variant!(viz Variant => Type)`
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
/// # pub enum TypedAddress { Home(Address), }
/// impl TypedAddress {
///     // as_variant!(pub Home => Address);
///     pub const fn as_home(&self) -> Option<&Address> {
///         match self {
///             Self::Home(value) => Some(value),
///             _ => None,
///         }
///     }
/// }
/// ```
///
/// ### `as_variant!(viz Variant => value, Type)`
///
/// This form generates a *safe cast* function for variant-associated data specifically for non-data
/// associated variants and a distinct value can be substituted.
///
/// * The return type of this method is `Option<Type>`; specifically, if the variant matches it returns
///   `Some(value)` else `None`.
///
/// ```rust
/// # pub struct Address(String);
/// # pub enum TypedAddress { Unknown, }
/// impl TypedAddress {
///     // as_variant!(pub Unknown => 0, u32);
///     pub const fn as_unknown(&self) -> Option<u32> {
///         match self {
///             Self::Unknown => Some(0),
///             _ => None,
///         }
///     }
/// }
/// ```
///
/// ### `as_variant!(viz Variant => ())`
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
/// # pub enum TypedAddress { Unknown, }
/// impl TypedAddress {
///     // as_variant!(pub Unknown => ());
///     pub const fn as_unknown(&self) -> Option<()> {
///         match self {
///             Self::Unknown => Some(()),
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
            $fn_vis const fn [< as_ $variant_name:snake >](&self) -> Option<&$variant_type> {
                match self {
                    Self::$variant_name(value) => Some(value),
                    _ => None,
                }
            }
        }
    };
    ($fn_vis:vis $variant_name:ident => ()) => {
        paste::paste! {
            $fn_vis const fn [< as_ $variant_name:snake >](&self) -> Option<()> {
                match self {
                    Self::$variant_name => Some(()),
                    _ => None,
                }
            }
        }
    };
    ($fn_vis:vis $variant_name:ident => $value:expr, $value_type:ty) => {
        paste::paste! {
            $fn_vis const fn [< as_ $variant_name:snake >](&self) -> Option<$variant_type> {
                match self {
                    Self::$variant_name => Some($value),
                    _ => None,
                }
            }
        }
    };
}

///
///  Generate both [is_variant] and [as_variant] for an enumeration variant.
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
/// # pub struct Address(String);
/// # pub enum TypedAddress { Home(Address), }
/// impl TypedAddress {
///     // is_as_variant!(pub Home => Address);
///     pub const fn is_home(&self) -> bool {
///         matches!(self, Self::Home(_))
///     }
///     pub const fn as_home(&self) -> Option<&Address> {
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
/// # pub enum TypedAddress { Home(Address), }
/// // impl_from_for_variant!(Address => TypedAddress, Home);
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
/// # pub enum TypedAddress { Home(Address), }
/// // impl_from_for_variant!(into Address => TypedAddress, Home);
/// impl<T: Into<Address>> From<T> for TypedAddress {
///     fn from(value: T) -> Self {
///         Self::Home(value.into())
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! impl_from_for_variant {
    ($value_type:ty => $enum_type:ty, $variant_name:ident) => {
        impl From<$value_type> for $enum_type {
            fn from(value: $value_type) -> Self {
                Self::$variant_name(value)
            }
        }
    };
    (into $value_type:ty => $enum_type:ty, $variant_name:ident) => {
        impl<T: Into<$value_type>> From<T> for $enum_type {
            fn from(value: T) -> Self {
                Self::$variant_name(value.into())
            }
        }
    };
}

///
/// Re-export macros for enum variant access.
///
/// The following enum definition is used as an example in the forms below to demonstrate
/// the generated code.
///
/// ```rust
/// pub struct Address {
///     // private fields ...
/// }
///
/// pub enum TypedAddress {
///     Home(Address),
///     Work(Address),
///     Unparsed(String),
///     Unknown
/// }
/// ```
///
/// ## Summary
///
/// | Macro                   | variant name | keywords | type | generated signature                                  |
/// |-------------------------|--------------|----------|------|------------------------------------------------------|
/// | `impl_from_for_variant` | Home         |       | Address | `impl From<Address> for TypedAddress {}`             |
/// | `impl_from_for_variant` | Home         | into  | Address | `impl<T: Into<Address>> From<T> for TypedAddress {}` |
/// | `is_variant!`           | Home         |       | Address | `const fn is_home(&self) -> bool`                    |
/// | `is_variant!`           | Unparsed     |       | ()      | `const fn is_unparsed(&self) -> bool`                |
/// | `is_variant!`           | Unknown      |       | ()      | `const fn is_unknown(&self) -> bool`                 |
/// | `as_variant!`           | Home         |       | Address | `const fn as_address(&self) -> Option<&Address>`     |
/// | `as_variant!`           | UnParsed     |       | String  | `const fn as_unparsed(&self) -> Option<&String>`     |
/// | `as_variant!`           | UnParsed     | value | String  | `const fn as_unparsed(&self) -> Option<String>`      |
/// | `as_variant!`           | Unknown      |       | ()      | `const fn as_unknown(&self) -> Option<()>`           |
///
pub mod variant {
    pub use crate::{as_variant, impl_from_for_variant, is_as_variant, is_variant};
}

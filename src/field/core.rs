/*!
 * The core set of struct field access macros: [`get`], [`get_mut`], [`set`], [`unset`]
 * and [`with`].
 */

// ------------------------------------------------------------------------------------------------
// Accessor Macros ❱ Get
// ------------------------------------------------------------------------------------------------

///
/// Generate a getter method for a field within a structure.
//
/// ## Forms
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
///
///     /// Returns a reference to the field `street_1` within this structure.
///     /// The returned value is an immutable reference `&String`.
///     pub const fn street_1(&self) -> &String {
///         &self.street_or_building
///     }
/// }
/// ```
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
///
///     /// Returns a reference to the field `street_1` within this structure.
///     /// The returned value is an immutable reference `&String`.
///     pub const fn street_1(&self) -> &String {
///         &self.street_1
///     }
/// }
/// ```
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
///
///     /// Returns a reference to the field `number_on_street` within this structure.
///     /// The returned value is an immutable copy `u32`.
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
///
///     /// Returns a reference to the field `number_on_street` within this structure.
///     /// The returned value is an immutable copy `u32`.
///     pub const fn number_on_street(&self) -> u32 {
///         self.number
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
///
///     /// Returns a reference to the field `street_2` within this structure.
///     /// The returned value is an optional immutable reference `Option<&String>`.
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
///
///     /// Returns a reference to the field `street_2` within this structure.
///     /// The returned value is an optional immutable reference `Option<&String>`.
///     pub const fn street_2(&self) -> Option<&String> {
///         self.street_additional.as_ref()
///     }
/// }
/// ```
///
/// ### `get!(viz name => optional copy Type)`
///
/// This form generates an immutable getter method for a field within a structure.
///
/// * In this form `name` is both the name of the generated function and the name of the
///   structure's field.
/// * The type of the generated function is the reference `Option<Type>`, assuming `Type` implements `Copy`.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { unit: Option<u32>, }
/// impl Address {
///     // get!(pub unit => optional u32);
///
///     /// Returns a reference to the field `unit` within this structure.
///     /// The returned value is an optional immutable copy `Option<u32>`.
///     pub const fn unit(&self) -> Option<u32> {
///         self.unit
///     }
/// }
/// ```
///
/// ### `get!(viz getter_name => field_name, optional copy Type)`
///
/// This form generates an immutable getter method for a field within a structure.
///
/// * In this form `name` is the name of the generated function while `field_name` is the
///   name of the structure's field.
/// * The type of the getter function is the reference `Option<Type>`, assuming `Type` implements `Copy`.
///
/// The following — commented line and following implementation — are therefore equivalent:
///
/// ```rust
/// # pub struct Address { unit_number: Option<u32>, }
/// impl Address {
///     // get!(pub unit => unit_number, optional u32);
///
///     /// Returns a reference to the field `unit` within this structure.
///     /// The returned value is an optional immutable copy `Option<u32>`.
///     pub const fn unit(&self) -> Option<u32> {
///         self.unit_number
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! get {
    // Base case: `viz name => field_name, Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        paste::paste! {
            #[doc = "Returns a reference to the field `" $fn_name "` within this structure. "
                    "The returned value is an immutable reference `&" $value_type "`"]
            $fn_vis const fn $fn_name(&self) -> &$value_type {
                &self.$field_name
            }
        }
    };
    // Base Case without *field name*: `viz name => Type`
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        $crate::get!($fn_vis $name => $name, $value_type);
    };
    // (2) Base case with *copy*: `viz name => field_name, copy Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, copy $value_type:ty) => {
        paste::paste! {
            #[doc = "Returns the value of the field `" $fn_name "` within this structure. "
                    "The returned value is an immutable copy `" $value_type "`"]
            $fn_vis const fn $fn_name(&self) -> $value_type {
                self.$field_name
            }
        }
    };
    // Case (2) without *field name*: `viz name => copy Type`
    ($fn_vis:vis $name:ident => copy $value_type:ty) => {
        $crate::get!($fn_vis $name => $name, copy $value_type);
    };
    // (3) Base case with *optional*: `viz name => field_name, optional Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional $value_type:ty) => {
        paste::paste! {
            #[doc = "Returns a reference to the optional field `" $fn_name "` within this structure. "
                    "The returned value is an optional immutable reference `Option<&" $value_type ">`"]
            $fn_vis const fn $fn_name(&self) -> Option<&$value_type> {
                self.$field_name.as_ref()
            }
        }
    };
    // Case (3) without *field name*: `viz name => optional Type`
    ($fn_vis:vis $name:ident => optional $value_type:ty) => {
        $crate::get!($fn_vis $name => $name, optional $value_type);
    };
    // (4) Case (3) with *copy*: `viz name => field_name, optional copy Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional copy $value_type:ty) => {
        paste::paste! {
            #[doc = "Returns a reference to the optional field `" $fn_name "` within this structure. "
                    "The returned value is an optional, immutable, copy `Option<" $value_type ">`"]
            $fn_vis const fn $fn_name(&self) -> Option<$value_type> {
                self.$field_name
            }
        }
    };
    // Case (4) without *field name*: `viz name => optional copy Type`
    ($fn_vis:vis $name:ident => optional copy $value_type:ty) => {
        $crate::get!($fn_vis $name => $name, optional copy $value_type);
    };
}

// ------------------------------------------------------------------------------------------------
// Accessor Macros ❱ Get Mutable
// ------------------------------------------------------------------------------------------------

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
///
///     /// Returns a *mutable* reference to the field `street_1_mut` within this structure.
///     /// The returned value is a mutable reference `&mut String`.
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
///
///     /// Returns a *mutable* reference to the field `street_1_mut` within this structure.
///     /// The returned value is a mutable reference `&mut String`.
///     pub const fn street_1_mut(&mut self) -> &mut String {
///         &mut self.street_1_string
///     }
/// }
/// ```
///
///
#[macro_export]
macro_rules! get_mut {
    // Base case: `viz name => field_name, Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        paste::paste! {
            #[doc = "Returns a *mutable* reference to the field `" $fn_name "` within this structure. "
                    "The returned value is a mutable reference `&mut " $value_type "`."]
            $fn_vis const fn [< $fn_name _mut >](&mut self) -> &mut $value_type {
                &mut self.$field_name
            }
        }
    };
    // Case (2) with *field name*: `viz name => Type`
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        $crate::get_mut!($fn_vis $name => $name, $value_type);
    };
}

// ------------------------------------------------------------------------------------------------
// Accessor Macros ❱ Set
// ------------------------------------------------------------------------------------------------

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
///
///     /// Set the value of the field `number_on_street` within this structure.
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
///
///     /// Set the value of the field `number_on_street` within this structure.
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
///
///     /// Set the value of the field `street_1` within this structure.
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
///
///     /// Set the value of the field `street_1` within this structure.
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
///
///     /// Set the value of the field `street_2` within this structure. While the corresponding
///     /// structure's field is an `Option<String>`, this function uses the type `String`. To
///     /// set the field value to `None` use the method [`unset_street_2`].
///      pub fn street_2(&mut self, street_2: String) {
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
///
///     /// Set the value of the field `street_2` within this structure. While the corresponding
///     /// structure's field is an `Option<String>`, this function uses the type `String`. To
///     /// set the field value to `None` use the method [`unset_street_2`].
///     pub fn street_2(&mut self, street_2: String) {
///         self.street_2_string = Some(street_2);
///     }
/// }
/// ```
///
/// ### `set!(viz name => optional into Type)`
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
///     // set!(pub street_2 => optional into String);
///
///     /// Set the value of the field `street_2` within this structure. While the corresponding
///     /// structure's field is an `Option<String>`, this function uses the type `String`. To
///     /// set the field value to `None` use the method [`unset_street_2`].
///      pub fn street_2<T: Into<String>>(&mut self, street_2: T) {
///         self.street_2 = Some(street_2.into());
///     }
/// }
/// ```
///
/// ### `set!(viz setter_name => field_name, optional into Type)`
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
///     // set!(pub street_2 => street_2_string, optional into String);
///
///     /// Set the value of the field `street_2` within this structure. While the corresponding
///     /// structure's field is an `Option<String>`, this function uses the type `String`. To
///     /// set the field value to `None` use the method [`unset_street_2`].
///     pub fn street_2<T: Into<String>>(&mut self, street_2: T) {
///         self.street_2_string = Some(street_2.into());
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! set {
    // Base case: `viz name => field_name, Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        paste::paste! {
            #[doc = "Set the value of the field `" $fn_name "` within this structure."]
            $fn_vis fn [<set_ $fn_name>](&mut self, value: $value_type) {
                self.$field_name = value;
            }
        }
    };
    // Base Case without *field name*: `viz name => Type`
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        $crate::set!($fn_vis $name => $name, $value_type);
    };
    // (2) Base case with *into*: `viz name => field_name, into Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, into $value_type:ty) => {
        paste::paste! {
            #[doc = "Set the value of the field `" $fn_name "` within this structure (Using `Into<" $value_type ">`)."]
            $fn_vis fn [<set_ $fn_name>]<T: Into<$value_type>>(&mut self, value: T) {
                self.$field_name = value.into();
            }
        }
    };
    // Case (2) without *field name*: `viz name => into Type`
    ($fn_vis:vis $name:ident => into $value_type:ty) => {
        $crate::set!($fn_vis $name => $name, into $value_type);
    };
    // (3) Base case with *optional*: `viz name => field_name, optional Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional $value_type:ty) => {
        paste::paste! {
            #[doc = "Set the value of the field `" $fn_name "` within this structure. "
                    "While the corresponding field is an `Option<" $value_type
                    ">`, this function uses the wrapped type `" $value_type
                    "`. To set the field value to `None` use the method [`unset_" $fn_name "`]."]
            $fn_vis fn [<set_ $fn_name>](&mut self, value: $value_type) {
                self.$field_name = Some(value);
            }
        }
    };
    // Case (3) with *field name*: `viz name => optional Type`
    ($fn_vis:vis $name:ident => optional $value_type:ty) => {
        $crate::set!($fn_vis $name => $name, optional $value_type);
    };
    // (4) Case (3) with *into*: `viz name => field_name, optional into Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional into $value_type:ty) => {
        paste::paste! {
            #[doc = "Set the value of the field `" $fn_name "` within this structure. "
                    "While the corresponding field is an `Option<" $value_type
                    ">`, this function uses the type `Into<" $value_type
                    ">`. To set the field value to `None` use the method [`unset_" $fn_name "`]."]
            $fn_vis fn [<set_ $fn_name>]<T: Into<$value_type>>(&mut self, value: T) {
                self.$field_name = Some(value.into());
            }
        }
    };
    // Case (4) without *field name*: `viz name => optional into Type`
    ($fn_vis:vis $name:ident => optional into $value_type:ty) => {
        $crate::set!($fn_vis $name => $name, optional into $value_type);
    };
}

// ------------------------------------------------------------------------------------------------
// Accessor Macros ❱ Unset
// ------------------------------------------------------------------------------------------------

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
/// pub struct Address {
///     // number_on_street: u32, etc.
///     street_2: Option<String>,
/// }
/// impl Address {
///     // unset!(pub street_2);
///
///     /// Set the value of the field `street_2` within this structure to `None`. To set
///     /// the value to `Some(value)`, use the method [`set_street_2`].
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
/// pub struct Address {
///     // number_on_street: u32, etc.
///     street_2_string: Option<String>,
/// }
/// impl Address {
///     // set!(pub street_2 => street_2_string);
///
///     /// Set the value of the field `street_2` within this structure to `None`. To set
///     /// the value to `Some(value)`, use the method [`set_street_2`].
///     #[inline(always)]
///     pub fn street_2(&mut self) {
///         self.street_2_string = None;
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! unset {
    // Base case: `viz name => field_name`
    ($fn_vis:vis $fn_name:ident => $field_name:ident) => {
        paste::paste! {
            #[doc = "Set the value of the field `" $fn_name "` within this structure to `None`. "
                    "To set the value to `Some(value)`, use the method [`set_" $fn_name "`]."]
            #[inline(always)]
            $fn_vis fn [<unset_ $fn_name>](&mut self) {
                self.$field_name = None;
            }
        }
    };
    // Base case without *field name*: `viz name`
    ($fn_vis:vis $name:ident) => {
        $crate::unset!($fn_vis $name => $name);
    };
}

// ------------------------------------------------------------------------------------------------
// Accessor Macros ❱ With
// ------------------------------------------------------------------------------------------------

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
///
///     /// Set the value of the field `number_on_street` within this structure, usually
///     /// during construction. This function takes a mutable `self` parameter and returns
///     /// `Self` allowing it to be chained during construction.
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
///
///     /// Set the value of the field `number_on_street` within this structure, usually
///     /// during construction. This function takes a mutable `self` parameter and returns
///     /// `Self` allowing it to be chained during construction.
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
///
///     /// Set the value of the field `number_on_street` within this structure, usually
///     /// during construction. This function takes a mutable `self` parameter and returns
///     /// `Self` allowing it to be chained during construction.
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
///
///     /// Set the value of the field `number_on_street` within this structure, usually
///     /// during construction. This function takes a mutable `self` parameter and returns
///     /// `Self` allowing it to be chained during construction.
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
///     // with!(pub street_2 => street_2_string, optional String);
///
///     /// Set the value of the field `number_on_street` within this structure, usually
///     /// during construction. This function takes a mutable `self` parameter and returns
///     /// `Self` allowing it to be chained during construction.
///     pub fn with_street_2(mut self, street_2: String) -> Self {
///         self.street_2_string = Some(street_2);
///         self
///     }
/// }
/// ```
///
#[macro_export]
macro_rules! with {
    // Base case: `viz name => field_name, Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        paste::paste! {
            #[doc = "Set the value of the field `" $fn_name "` within this structure, usually during construction. "
                    "This function takes a mutable `self` parameter and returns `Self` allowing it to be "
                    "chained during construction."]
            $fn_vis fn [<with_ $fn_name>](mut self, value: $value_type) -> Self {
                self.$field_name = value;
                self
            }
        }
    };
    // Base case without *field name*: `viz name => Type`
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        $crate::with!($fn_vis $name => $name, $value_type);
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // (2) Base case with *into*: `viz name => field_name, into Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, into $value_type:ty) => {
        paste::paste! {
            #[doc = "Set the value of the field `" $fn_name "` within this structure, usually during construction. "
                    "This function takes a mutable `self` parameter and returns `Self` allowing it to be "
                    "chained during construction."]
            $fn_vis fn [<with_ $fn_name>]<T: Into<$value_type>>(mut self, value: T) -> Self {
                self.$field_name = value.into();
                self
            }
        }
    };
    // Case (2) without *field name*: `viz name => into Type`
    ($fn_vis:vis $name:ident => into $value_type:ty) => {
        $crate::with!($fn_vis $name => $name, into $value_type);
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // (3) Base case with *optional*: `viz name => field_name, optional Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional $value_type:ty) => {
        paste::paste! {
            #[doc = "Set the value of the field `" $fn_name "` within this structure, usually during construction. "
                    "This function takes a mutable `self` parameter and returns `Self` allowing it to be "
                    "chained during construction."]
            $fn_vis fn [<with_ $fn_name>](mut self, value: $value_type) -> Self {
                self.$field_name = Some(value);
                self
            }
        }
    };
    // Case (3) without *field name*: `viz name => optional Type`
    ($fn_vis:vis $name:ident => optional $value_type:ty) => {
        $crate::with!($fn_vis $name => $name, optional $value_type);
    };
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // (4) Case (3) with *into*: `viz name => field_name, optional into Type`
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional into $value_type:ty) => {
        paste::paste! {
            #[doc = "Set the value of the field `" $fn_name "` within this structure, usually during construction. "
                    "This function takes a mutable `self` parameter and returns `Self` allowing it to be "
                    "chained during construction."]
            $fn_vis fn [<with_ $fn_name>]<T: Into<$value_type>>(mut self, value: T) -> Self {
                self.$field_name = Some(value.into());
                self
            }
        }
    };
    // Case (4) without *field name*: `viz name => optional into Type`
    ($fn_vis:vis $name:ident => optional into $value_type:ty) => {
        $crate::with!($fn_vis $name => $name, optional into $value_type);
    };
}

// ------------------------------------------------------------------------------------------------
// Re-export macros
// ------------------------------------------------------------------------------------------------

pub use crate::{get, get_mut, set, unset, with};

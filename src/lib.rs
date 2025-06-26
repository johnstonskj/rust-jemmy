/*!
This package provides a coherent set of manual accessor macros.

More detailed description, with

## Example

```
use jemmy::access;

pub struct Thing {
    name: String,
    age: u32,
}

impl Thing {
    access::get!(pub name => String);
    access::get!(pub age => u32);
    access::set!(pub age => u32);
}
```

## Forms

1. function/field names;
2. **into** keyword;
3. **boxed** keyword;
4. **optional** keyword;
5. **default** keyword;

 */

// ------------------------------------------------------------------------------------------------
// Accessor Macros ❱ Get | Set | Unset | With
// ------------------------------------------------------------------------------------------------

/// Generate a getter method for a field
#[macro_export]
macro_rules! get {
    // Simple form: get!(pub name => Type)
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        $fn_vis fn $name(&self) -> &$value_type {
            &self.$name
        }
    };
    // Extended form: get!(pub getter_name => field_name, Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        $fn_vis fn $fn_name(&self) -> &$value_type {
            &self.$field_name
        }
    };
    // Option form: get!(pub name => optional Type)
    ($fn_vis:vis $name:ident => optional $value_type:ty) => {
        $fn_vis fn $name(&self) -> Option<&$value_type> {
            self.$name.as_ref()
        }
    };
    // Extended form: get!(pub getter_name => field_name, optional Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional $value_type:ty) => {
        $fn_vis fn $fn_name(&self) -> Option<&$value_type> {
            self.$field_name.as_ref()
        }
    };
}

/// Generate a mutable getter method for a field
#[macro_export]
macro_rules! get_mut {
    // Simple form: get_mut!(pub name => Type)
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        $fn_vis fn $name(&mut self) -> &mut $value_type {
            &mut self.$name
        }
    };
    // Extended form: get_mut!(pub getter_name => field_name, Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        $fn_vis fn $fn_name(&mut self) -> &mut $value_type {
            &mut self.$field_name
        }
    };
}

/// Generate a setter method for a field
#[macro_export]
macro_rules! set {
    // Simple form: set!(pub name => Type)
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<set_ $name>](&mut self, value: $value_type) {
                self.$name = value;
            }
        }
    };
    // Extended form: set!(pub setter_name => field_name, Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<set_ $fn_name>](&mut self, value: $value_type) {
                self.$field_name = value;
            }
        }
    };
    // Into form: set!(pub name => into Type)
    ($fn_vis:vis $name:ident => into $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<set_ $name>]<T: Into<$value_type>>(&mut self, value: T) {
                self.$name = value.into();
            }
        }
    };
    // Extended Into form: set!(pub setter_name => field_name, into Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, into $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<set_ $fn_name>]<T: Into<$value_type>>(&mut self, value: T) {
                self.$field_name = value.into();
            }
        }
    };
    // Optional form: set!(pub name => optional Type)
    ($fn_vis:vis $name:ident => optional $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<set_ $name>](&mut self, value: $value_type) {
                self.$name = Some(value);
            }
        }
    };
    // Extended Optional form: set!(pub setter_name => field_name, optional Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<set_ $fn_name>](&mut self, value: $value_type) {
                self.$field_name = Some(value);
            }
        }
    };
}

/// Generate a setter method for an Option field
#[macro_export]
macro_rules! unset {
   // Simple form: set!(pub name => Type)
    ($fn_vis:vis $name:ident) => {
        paste::paste! {
            $fn_vis fn [<unset_ $name>](&mut self,) {
                self.$name = None;
            }
        }
    };
    // Extended form: set!(pub setter_name => field_name, Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident) => {
        paste::paste! {
            $fn_vis fn [<unset_ $fn_name>](&mut self) {
                self.$field_name = None;
            }
        }
    };
}

/// Generate a builder-style setter method for a field
#[macro_export]
macro_rules! with {
    // Simple form: with!(pub name => Type)
    ($fn_vis:vis $name:ident => $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<with_ $name>](mut self, value: $value_type) -> Self {
                self.$name = value;
                self
            }
        }
    };
    // Extended form: with!(pub with_name => field_name, Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<with_ $fn_name>](mut self, value: $value_type) -> Self {
                self.$field_name = value;
                self
            }
        }
    };
    // Into form: with!(pub name => into Type)
    ($fn_vis:vis $name:ident => into $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<with_ $name>]<T: Into<$value_type>>(mut self, value: T) -> Self {
                self.$name = value.into();
                self
            }
        }
    };
    // Extended Into form: with!(pub with_name => field_name, into Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, into $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<with_ $fn_name>]<T: Into<$value_type>>(mut self, value: T) -> Self {
                self.$field_name = value.into();
                self
            }
        }
    };
    // Optional form: with!(pub name => optional Type)
    ($fn_vis:vis $name:ident => optional $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<with_ $name>](mut self, value: $value_type) -> Self {
                self.$name = Some(value);
                self
            }
        }
    };
    // Extended Optional form: with!(pub with_name => field_name, optional Type)
    ($fn_vis:vis $fn_name:ident => $field_name:ident, optional $value_type:ty) => {
        paste::paste! {
            $fn_vis fn [<with_ $fn_name>](mut self, value: $value) -> Self {
                self.$field_name = Some(value);
                self
            }
        }
    };
}

// Re-export macros for simple accessor patterns.
pub mod access {
    pub use crate::{get, get_mut, set, with};
}

// ------------------------------------------------------------------------------------------------
// Variant Macros ❱ is_* | as_*
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! is_variant {
    ($fn_vis:vis $type_name:ty, $variant_name:ident) => {
        paste::paste! {
            $fn_vis fn [< is_ $variant_name:snake >](&self) -> bool {
                matches!(self, $type_name::$variant_name)
            }
        }
    };
    ($fn_vis:vis $type_name:ty, $variant_name:ident => $variant_type:ty) => {
        paste::paste! {
            $fn_vis fn [< is_ $variant_name:snake >](&self) -> bool {
                matches!(self, $type_name::$variant_name(_))
            }
        }
    };
}

#[macro_export]
macro_rules! as_variant {
    ($fn_vis:vis $type_name:ty, $variant_name:ident => $variant_type:ty) => {
        paste::paste! {
            $fn_vis fn [< as_ $variant_name:snake >](&self) -> Option<&$variant_type> {
                match self {
                    $type_name::$variant_name(value) => Some(value),
                    _ => None,
                }
            }
        }
    };
}


#[macro_export]
macro_rules! is_as_variant {
    ($fn_vis:vis $type_name:ty, $variant_name:ident => $variant_type:ty) => {
       paste::paste! {
            $fn_vis fn [< is_ $variant_name:snake >](&self) -> bool {
                matches!(self, $type_name::$variant_name(_))
            }
            $fn_vis fn [< as_ $variant_name:snake >](&self) -> Option<&$variant_type> {
                match self {
                    $type_name::$variant_name(value) => Some(value),
                    _ => None,
                }
            }
        }
    };
}

// Re-export macros for simple accessor patterns.
pub mod variant {
    pub use crate::{is_variant, as_variant, is_as_variant};
}
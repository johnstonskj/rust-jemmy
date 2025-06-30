# Package rust-jemmy

This package provides a coherent set of manual accessor macros for fields in structures and variants in enums.

[![Apache-2.0 License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![MIT License](https://img.shields.io/badge/license-mit-118811.svg)](https://opensource.org/license/mit)
[![Rust Workflow](https://github.com/johnstonskj/rust-jemmy/actions/workflows/rust.yml/badge.svg)](https://github.com/johnstonskj/rust-jemmy/actions/workflows/rust.yml)
[![Security Audit Workflow](https://github.com/johnstonskj/rust-jemmy/actions/workflows/security-audit.yml/badge.svg)](https://github.com/johnstonskj/rust-jemmy/actions/workflows/security-audit.yml)
[![Coverage Status](https://codecov.io/gh/johnstonskj/rust-jemmy/branch/main/graph/badge.svg?token=1HGN6M4KIT)](https://codecov.io/gh/johnstonskj/rust-jemmy)
[![Stargazer Count](https://img.shields.io/github/stars/johnstonskj/rust-jemmy.svg)](https://github.com/johnstonskj/rust-jemmy/stargazers)

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

```rust
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

### Structure Fields Macro Summary

| Macro    | field name       | keywords | type   | generated signature                                                 |
|----------|------------------|----------|--------|---------------------------------------------------------------------|
| `with!`  | number_on_street |          | u32    | `fn with_number_on_street(mut self, number_on_street: u32) -> Self` |
| `with!`  | street_1         | into     | String | `fn with_street_1<T: Into<String>(mut self, street_1: T) -> Self`   |
| `with!`  | street_2         | optional | String | `fn with_street_2(mut self, street_2: String) -> Self`              |
| `get!`   | number_on_street | copy     | u32    | `const fn number_on_street(&self) -> u32`                           |
| `get!`   | street_1         |          | String | `const fn street_1(&self) -> &String`                               |
| `get!`   | street_2         | optional | String | `const fn street_2(&self) -> Option<&String>`                       |
| `set!`   | number_on_street |          | u32    | `fn set_number_on_street(&mut self, number_on_street: u32)`         |
| `set!`   | street_1         | into     | String | `fn set_street_1<T: Into<String>(&mut self, street_1: T)`           |
| `set!`   | street_2         | optional | String | `fn set_street_2(&mut self, street_1: String)`                      |
| `unset!` | street_2         |          | String | `fn unset_street_2(&mut self)`                                      |

### Enum Variants Macro Summary

| Macro                   | variant name | keywords | type | generated signature                                  |
|-------------------------|--------------|----------|------|------------------------------------------------------|
| `impl_from_for_variant` | Home         |       | Address | `impl From<Address> for TypedAddress {}`             |
| `impl_from_for_variant` | Home         | into  | Address | `impl<T: Into<Address>> From<T> for TypedAddress {}` |
| `is_variant!`           | Home         |       | Address | `const fn is_home(&self) -> bool`                    |
| `is_variant!`           | Unparsed     |       | ()      | `const fn is_unparsed(&self) -> bool`                |
| `is_variant!`           | Unknown      |       | ()      | `const fn is_unknown(&self) -> bool`                 |
| `as_variant!`           | Home         |       | Address | `const fn as_address(&self) -> Option<&Address>`     |
| `as_variant!`           | UnParsed     |       | String  | `const fn as_unparsed(&self) -> Option<&String>`     |
| `as_variant!`           | UnParsed     | value | String  | `const fn as_unparsed(&self) -> Option<String>`      |
| `as_variant!`           | Unknown      |       | ()      | `const fn as_unknown(&self) -> Option<()>`           |

## Changes

### Version 0.1.3

* Feature: added `#[doc]` to all generated methods.
* Feature: added new options to `with_get_and_set!`.
* Feature: added new macro `with_get_set_and_unset!`.

### Version 0.1.2

Minor improvements and completed documentation.

* Feature: added new `impl_from_for_variant!` macro.
* Feature: made all methods generated by `get!` and `get_mut!` marked `const`.
* Feature: made all methods generated by `unset!` marked as `#[inline(always)]`.
* Documentation: completed all macro and module docs.
* Chore: renamed files; 'LICENSE-*.txt' => 'LICENSE-*'

### Version 0.1.1

Provided addition *helper* or *combinator* macros, and provided module
documentation as well as documentation for more than half of the present
set of macros.

* Structure field macros
  * `get_and_set!`
  * `with_get_and_set!`
  * `get_set_and_unset!`

* Enum field macros
  * `is_as_variant!`

### Version 0.1.0

Provided an initial set of macros.

* Structure field macros
  * initializers -- `with!` ⟹ `with_fname(mut self, fname: T) -> Self`
  * getters -- `get!` ⟹ `fname(&self) -> &T | T | Option<&T>`
  * setters -- `set!` ⟹ `set_fname(&mut self, T | Into<T>)`
  * un-setters -- `unset!` ⟹ `unset_fname(&mut self)`

* Enum field macros
  * predicates -- `is_variant!` ⟹ `is_vname(&self) -> bool`
  * getters -- `as_variant!` ⟹ `as_vname(&self) -> &T`

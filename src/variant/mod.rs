/*!

Macros for enum variant access.

The following enum definition is used as an example in the forms below to demonstrate
the generated code.

```rust
pub struct Address {
    // private fields ...
}

pub enum TypedAddress {
    Home(Address),
    Work(Address),
    Unparsed(String),
    XRef(u64),
    Unknown
}
```

## Summary

| Macro                   | variant name | keywords | type | generated signature                                  |
|-------------------------|--------------|----------|------|------------------------------------------------------|
| `impl_from_for_variant` | Home         |       | Address | `impl From<Address> for TypedAddress {}`             |
| `impl_from_for_variant` | Home         | into  | Address | `impl<T: Into<Address>> From<T> for TypedAddress {}` |
| `is_variant!`           | Home         |       | Address | `const fn is_home(&self) -> bool`                    |
| `is_variant!`           | Unparsed     |       | ()      | `const fn is_unparsed(&self) -> bool`                |
| `is_variant!`           | Unknown      |       | ()      | `const fn is_unknown(&self) -> bool`                 |
| `as_variant!`           | Home         |       | Address | `const fn as_address(&self) -> Option<&Address>`     |
| `as_variant!`           | UnParsed     |       | String  | `const fn as_unparsed(&self) -> Option<&String>`     |
| `as_variant!`           | BadlyFormed  | value | ()      | `const fn as_unparsed(&self) -> Option<Error>`       |
| `as_variant!`           | Unknown      |       |         | `const fn as_unknown(&self) -> Option<()>`           |
| `as_variant!`           | Unknown      |       | ()      | `const fn as_unknown(&self) -> Option<()>`           |
| `as_variant!`           | XRef         |       | u64     | `const fn as_x_ref(&self) -> Option<&u64>`           |
| `as_variant!`           | XRef         | copy  | u64     | `const fn as_x_ref(&self) -> Option<u64>`            |

*/

#[macro_use]
pub mod core;
pub use core::{as_variant, is_variant};

#[macro_use]
pub mod combination;
pub use combination::is_as_variant;

#[macro_use]
pub mod impls;
pub use impls::impl_from_for_variant;

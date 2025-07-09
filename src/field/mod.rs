/*!

Macros for struct field access.

The following struct definition is used as an example in the forms below to demonstrate
the generated code.

```rust
pub struct Address {
    number_on_street: u32,    // demonstrates keyword 'copy'
    street_1: String,         // demonstrates keyword 'into'
    street_2: Option<String>, // demonstrates keyword 'optional'
    // ...
}
```
## Summary
| Macro      | field name       | keywords      | type   | generated signature                                                 |
|------------|------------------|---------------|--------|---------------------------------------------------------------------|
| `with!`    | number_on_street |               | u32    | `fn with_number_on_street(mut self, number_on_street: u32) -> Self` |
| `with!`    | street_1         | into          | String | `fn with_street_1<T: Into<String>(mut self, street_1: T) -> Self`   |
| `with!`    | street_2         | optional      | String | `fn with_street_2(mut self, street_2: String) -> Self`              |
| `with!`    | street_2         | optional into | String | `fn with_street_2<T: Into<String>(mut self, street_2: T) -> Self`   |
| `get!`     | street_1         |               | String | `const fn street_1(&self) -> &String`                               |
| `get!`     | number_on_street | copy          | u32    | `const fn number_on_street(&self) -> u32`                           |
| `get!`     | street_2         | optional      | String | `const fn street_2(&self) -> Option<&String>`                       |
| `get!`     | number_on_street | optional copy | u64    | `const fn number_on_street(&self) -> Option<number_on_street>`      |
| `get_mut!` | street_1         |               | String | `const fn street_1_mut(&mut self) -> &mut String`                   |
| `set!`     | number_on_street |               | u32    | `fn set_number_on_street(&mut self, number_on_street: u32)`         |
| `set!`     | street_1         | into          | String | `fn set_street_1<T: Into<String>(&mut self, street_1: T)`           |
| `set!`     | street_2         | optional      | String | `fn set_street_2(&mut self, street_2: String)`                      |
| `set!`     | street_2         | optional into | String | `fn set_street_2<T: Into<String>(&mut self, street_2: T)`           |
| `unset!`   | street_2         |               | String | `fn unset_street_2(&mut self)`                                      |

*/

#[macro_use]
pub mod core;
pub use core::{get, get_mut, set, unset, with};

#[macro_use]
pub mod combination;
pub use combination::{get_and_set, get_set_and_unset, with_get_and_set, with_get_set_and_unset};

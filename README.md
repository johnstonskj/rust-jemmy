# Package rust-jemmy

This package provides a coherent set of manual accessor macros.

TBD

## Example

```rust
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

## Changes

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

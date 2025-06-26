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

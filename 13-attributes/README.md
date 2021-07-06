An attribute is metadata applied to some module, crate or item. This metadata can be used to/for:

- conditional compilation of code
- set crate name, version and type (binary or library)
- disable lints (warnings)
- enable compiler features (macros, glob imports, etc.)
- link to a foreign library
- mark functions as unit tests
- mark functions that will be part of a benchmark

Attribute can be something like this 

```rust
#[attribute(value, value2)]


#[attribute(value, value2, value3,
            value4, value5)]
```
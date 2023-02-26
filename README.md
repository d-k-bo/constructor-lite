# constructor-lite

[![Build Status](https://github.com/d-k-bo/constructor-lite/workflows/CI/badge.svg)](https://github.com/d-k-bo/constructor-lite/actions?query=workflow%3ACI)
[![Crates.io](https://img.shields.io/crates/v/constructor-lite)](https://crates.io/crates/constructor-lite)
[![Documentation](https://img.shields.io/docsrs/constructor-lite)](https://docs.rs/constructor-lite)
[![License: LGPL-2.1-or-later](https://img.shields.io/crates/l/constructor-lite)](LICENSE)

<!-- cargo-rdme start -->

This crate provides the `ConstructorLite` derive macro for generating
minimal constructors for a struct from its fields.

It is primarily designed for structs where deriving [`Default`] is not
possible because some fields don't implement it.

By default, an associated function `new()` is generated, which expects every
field that is not `Option<T>` as an argument.

- To add an optional field to expected arguments of the constructor function,
  it can be marked with `#[constructor(required)]`.
- To remove a non-optional field that implements `Default` from the constructor
  function, it can be marked with `#[constructor(default)]`.
- To change the name of the generated function, the struct can be marked with e. g.
  `#[constructor(name = "function_name")]`.
- By default, the generated function has the same visibility as the struct.
  To override this behaviour, the struct can be marked with e. g.
  `#[constructor(visibility = "pub(super)")]`.

For more advanced uses you might prefer using
[`derive-new`](https://lib.rs/crates/derive-new) or
[`derive_builder`](https://lib.rs/crates/derive_builder) instead.

## Example

```rust
use constructor_lite::ConstructorLite;

#[derive(Debug, PartialEq, ConstructorLite)]
struct Movie {
    title: String,
    year: Option<u16>,
}

assert_eq!(
    Movie::new("Star Wars".to_owned()),
    Movie { title: "Star Wars".to_owned(), year: None },
)
```

<!-- cargo-rdme end -->

## License

This project is licensed under the MIT License.

See [LICENSE](LICENSE) for more information.

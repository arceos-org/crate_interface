# crate_interface

[![Crates.io](https://img.shields.io/crates/v/crate_interface)](https://crates.io/crates/crate_interface)
[![Docs.rs](https://docs.rs/crate_interface/badge.svg)](https://docs.rs/crate_interface)
[![CI](https://github.com/arceos-org/crate_interface/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/arceos-org/crate_interface/actions/workflows/ci.yml)

Provides a way to **define** an interface (trait) in a crate, but can
**implement** or **use** it in any crate. It 's usually used to solve
the problem of *circular dependencies* between crates.

## Example

```rust
// Define the interface
#[crate_interface::def_interface]
pub trait HelloIf {
    fn hello(&self, name: &str, id: usize) -> String;
}

// Implement the interface in any crate
struct HelloIfImpl;

#[crate_interface::impl_interface]
impl HelloIf for HelloIfImpl {
    fn hello(&self, name: &str, id: usize) -> String {
        format!("Hello, {} {}!", name, id)
    }
}

// Call `HelloIfImpl::hello` in any crate
use crate_interface::call_interface;
assert_eq!(
    call_interface!(HelloIf::hello("world", 123)),
    "Hello, world 123!"
);
assert_eq!(
    call_interface!(HelloIf::hello, "rust", 456), // another calling style
    "Hello, rust 456!"
);
```

## Implementation

The procedural macros in the above example will generate the following code:

```rust
// #[def_interface]
pub trait HelloIf {
    fn hello(&self, name: &str, id: usize) -> String;
}

#[allow(non_snake_case)]
pub mod __HelloIf_mod {
    use super::*;
    extern "Rust" {
        pub fn __HelloIf_hello(name: &str, id: usize) -> String;
    }
}

struct HelloIfImpl;

// #[impl_interface]
impl HelloIf for HelloIfImpl {
    #[inline]
    fn hello(&self, name: &str, id: usize) -> String {
        {
            #[inline]
            #[export_name = "__HelloIf_hello"]
            extern "Rust" fn __HelloIf_hello(name: &str, id: usize) -> String {
                let _impl: HelloIfImpl = HelloIfImpl;
                _impl.hello(name, id)
            }
        }
        {
            format!("Hello, {} {}!", name, id)
        }
    }
}

// call_interface!
assert_eq!(
    unsafe { __HelloIf_mod::__HelloIf_hello("world", 123) },
    "Hello, world 123!"
);
```

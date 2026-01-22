//! Define traits with default implementations using weak_default feature.
//!
//! This crate defines traits that have default implementations. With the
//! `weak_default` feature enabled, these default implementations are compiled
//! as weak symbols, allowing implementors to optionally override them.
//!
//! IMPORTANT: This crate requires nightly Rust and `#![feature(linkage)]`.

#![feature(linkage)]

use crate_interface::def_interface;

/// A trait with some methods having default implementations.
///
/// Implementors can choose to implement only the required methods (those without
/// defaults), and the default implementations will be used for the rest via
/// weak symbol linkage.
#[def_interface]
pub trait WeakDefaultIf {
    /// A required method - must be implemented.
    fn required_value() -> u32;

    /// A method with default implementation - can be skipped.
    fn default_value() -> u32 {
        42
    }

    /// A method with default implementation that uses arguments.
    fn default_add(a: u32, b: u32) -> u32 {
        a + b
    }

    /// Another required method.
    fn required_name() -> &'static str;

    /// Default implementation returning a constant string.
    fn default_greeting() -> &'static str {
        "Hello from weak default!"
    }
}

/// A trait where ALL methods have default implementations.
/// Implementors can implement an empty impl block (just to register the implementation).
#[def_interface]
pub trait AllDefaultIf {
    /// Default method 1.
    fn method_a() -> i32 {
        100
    }

    /// Default method 2.
    fn method_b() -> i32 {
        200
    }

    /// Default method with computation.
    fn method_c(x: i32) -> i32 {
        x * 2
    }
}

/// A trait with namespace and default implementations.
#[def_interface(namespace = WeakNs)]
pub trait NamespacedWeakIf {
    /// Required method.
    fn get_id() -> u64;

    /// Default method.
    fn get_default_multiplier() -> u64 {
        10
    }
}

/// A trait with gen_caller and default implementations.
#[def_interface(gen_caller)]
pub trait CallerWeakIf {
    /// Required method with helper caller.
    fn compute(x: i64) -> i64;

    /// Default method with helper caller.
    fn default_offset() -> i64 {
        1000
    }
}

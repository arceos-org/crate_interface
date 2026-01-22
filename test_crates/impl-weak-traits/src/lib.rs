//! Full implementation of weak_default traits defined in `define-weak-traits`.
//!
//! This crate provides FULL implementations that override ALL methods,
//! including those with default implementations. This tests that strong
//! symbols correctly override weak symbol defaults.
//!
//! IMPORTANT: This crate requires nightly Rust and `#![feature(linkage)]`.

#![feature(linkage)]

use crate_interface::impl_interface;
use define_weak_traits::{AllDefaultIf, CallerWeakIf, NamespacedWeakIf, WeakDefaultIf};

/// Full implementation - overrides ALL methods including defaults.
/// This creates strong symbols that override the weak symbol defaults.
pub struct FullImpl;

#[impl_interface]
impl WeakDefaultIf for FullImpl {
    fn required_value() -> u32 {
        2000
    }

    fn required_name() -> &'static str {
        "FullImpl"
    }

    // Override the default implementations with strong symbols.
    fn default_value() -> u32 {
        99
    }

    fn default_add(a: u32, b: u32) -> u32 {
        a * b // Multiply instead of add
    }

    fn default_greeting() -> &'static str {
        "Hello from FullImpl override!"
    }
}

/// Implementation for AllDefaultIf - overrides some methods.
pub struct AllDefaultImpl;

#[impl_interface]
impl AllDefaultIf for AllDefaultImpl {
    // Override one method with strong symbol
    fn method_a() -> i32 {
        111
    }
    // method_b() and method_c() will use weak symbol defaults.
}

/// Implementation for NamespacedWeakIf.
pub struct NamespacedWeakImpl;

#[impl_interface(namespace = WeakNs)]
impl NamespacedWeakIf for NamespacedWeakImpl {
    fn get_id() -> u64 {
        12345
    }
    // get_default_multiplier() will use the weak symbol default.
}

/// Implementation for CallerWeakIf.
pub struct CallerWeakImpl;

#[impl_interface]
impl CallerWeakIf for CallerWeakImpl {
    fn compute(x: i64) -> i64 {
        x * 3
    }
    // default_offset() will use the weak symbol default.
}

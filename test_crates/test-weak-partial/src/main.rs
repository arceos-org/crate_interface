//! Integration tests for weak_default traits with PARTIAL implementation only.
//!
//! This binary ONLY links PartialOnlyImpl, which does NOT implement:
//! - default_value()
//! - default_add()
//! - default_greeting()
//!
//! Therefore, these methods MUST use the weak symbol default implementations.
//!
//! Exit code 0 means all tests passed.

#![feature(linkage)]

use crate_interface::call_interface;

// Import the partial implementation crate to link it
use impl_weak_partial::PartialOnlyImpl;

// Suppress unused warnings - this is used for linking
const _: () = {
    let _ = std::any::type_name::<PartialOnlyImpl>;
};

fn test_required_methods() {
    assert_eq!(
        call_interface!(define_weak_traits::WeakDefaultIf::required_value),
        5555
    );
    assert_eq!(
        call_interface!(define_weak_traits::WeakDefaultIf::required_name),
        "PartialOnlyImpl"
    );
    println!("  [PASS] test_required_methods");
}

fn test_weak_default_methods() {
    // These MUST come from weak symbol defaults
    assert_eq!(
        call_interface!(define_weak_traits::WeakDefaultIf::default_value),
        42
    );
    assert_eq!(
        call_interface!(define_weak_traits::WeakDefaultIf::default_add, 10, 20),
        30
    );
    assert_eq!(
        call_interface!(define_weak_traits::WeakDefaultIf::default_add, 100, 200),
        300
    );
    assert_eq!(
        call_interface!(define_weak_traits::WeakDefaultIf::default_greeting),
        "Hello from weak default!"
    );
    println!("  [PASS] test_weak_default_methods");
}

fn test_weak_default_multiple_calls() {
    for i in 0..10 {
        let result = call_interface!(define_weak_traits::WeakDefaultIf::default_add, i, i * 2);
        assert_eq!(result, i + i * 2);
    }
    println!("  [PASS] test_weak_default_multiple_calls");
}

fn test_mixed_required_and_default() {
    let req_val = call_interface!(define_weak_traits::WeakDefaultIf::required_value);
    let def_val = call_interface!(define_weak_traits::WeakDefaultIf::default_value);
    let req_name = call_interface!(define_weak_traits::WeakDefaultIf::required_name);
    let def_greeting = call_interface!(define_weak_traits::WeakDefaultIf::default_greeting);

    assert_eq!(req_val, 5555);
    assert_eq!(def_val, 42);
    assert_eq!(req_name, "PartialOnlyImpl");
    assert_eq!(def_greeting, "Hello from weak default!");
    println!("  [PASS] test_mixed_required_and_default");
}

fn main() {
    println!("Running weak_default trait tests (partial implementation)...");

    test_required_methods();
    test_weak_default_methods();
    test_weak_default_multiple_calls();
    test_mixed_required_and_default();

    println!("All weak_default trait tests (partial impl) passed!");
}

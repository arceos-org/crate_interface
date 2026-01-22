//! Integration tests for weak_default traits with FULL implementations.
//!
//! This binary links FullImpl which overrides ALL methods of WeakDefaultIf.
//! It verifies that strong symbols correctly override weak symbol defaults.
//!
//! Exit code 0 means all tests passed.

#![feature(linkage)]

use crate_interface::call_interface;

// Import the implementation crate to link the implementations
use impl_weak_traits::{AllDefaultImpl, CallerWeakImpl, FullImpl, NamespacedWeakImpl};

// Suppress unused warnings - these are used for linking
const _: () = {
    let _ = std::any::type_name::<FullImpl>;
    let _ = std::any::type_name::<AllDefaultImpl>;
    let _ = std::any::type_name::<NamespacedWeakImpl>;
    let _ = std::any::type_name::<CallerWeakImpl>;
};

fn test_full_impl_required_methods() {
    assert_eq!(
        call_interface!(define_weak_traits::WeakDefaultIf::required_value),
        2000
    );
    assert_eq!(
        call_interface!(define_weak_traits::WeakDefaultIf::required_name),
        "FullImpl"
    );
    println!("  [PASS] test_full_impl_required_methods");
}

fn test_full_impl_overridden_defaults() {
    // Strong symbols should win over weak symbols
    assert_eq!(
        call_interface!(define_weak_traits::WeakDefaultIf::default_value),
        99
    );
    assert_eq!(
        call_interface!(define_weak_traits::WeakDefaultIf::default_add, 10, 20),
        200 // 10 * 20, not 10 + 20
    );
    assert_eq!(
        call_interface!(define_weak_traits::WeakDefaultIf::default_greeting),
        "Hello from FullImpl override!"
    );
    println!("  [PASS] test_full_impl_overridden_defaults");
}

fn test_all_default_interface() {
    let a = call_interface!(define_weak_traits::AllDefaultIf::method_a);
    let b = call_interface!(define_weak_traits::AllDefaultIf::method_b);
    let c = call_interface!(define_weak_traits::AllDefaultIf::method_c, 5);

    assert_eq!(a, 111); // Strong symbol override
    assert_eq!(b, 200); // Weak symbol default
    assert_eq!(c, 10); // Weak symbol default: 5 * 2
    println!("  [PASS] test_all_default_interface");
}

fn test_namespaced_weak_interface() {
    let id = call_interface!(namespace = WeakNs, define_weak_traits::NamespacedWeakIf::get_id);
    let multiplier =
        call_interface!(namespace = WeakNs, define_weak_traits::NamespacedWeakIf::get_default_multiplier);

    assert_eq!(id, 12345);
    assert_eq!(multiplier, 10);
    println!("  [PASS] test_namespaced_weak_interface");
}

fn test_caller_weak_interface() {
    let computed = call_interface!(define_weak_traits::CallerWeakIf::compute, 100);
    let offset = call_interface!(define_weak_traits::CallerWeakIf::default_offset);

    assert_eq!(computed, 300); // 100 * 3
    assert_eq!(offset, 1000); // Weak symbol default

    use define_weak_traits::{compute, default_offset};
    assert_eq!(compute(50), 150);
    assert_eq!(default_offset(), 1000);
    println!("  [PASS] test_caller_weak_interface");
}

fn test_mixed_strong_and_weak() {
    for i in 1..5 {
        assert_eq!(
            call_interface!(define_weak_traits::AllDefaultIf::method_a),
            111
        );
        assert_eq!(
            call_interface!(define_weak_traits::AllDefaultIf::method_b),
            200
        );
        assert_eq!(
            call_interface!(define_weak_traits::AllDefaultIf::method_c, i),
            i * 2
        );
    }
    println!("  [PASS] test_mixed_strong_and_weak");
}

fn main() {
    println!("Running weak_default trait tests (full implementation)...");

    test_full_impl_required_methods();
    test_full_impl_overridden_defaults();
    test_all_default_interface();
    test_namespaced_weak_interface();
    test_caller_weak_interface();
    test_mixed_strong_and_weak();

    println!("All weak_default trait tests (full impl) passed!");
}

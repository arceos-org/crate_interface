use crate_interface::*;

#[def_interface]
trait SimpleIf {
    fn foo() -> u32 {
        123
    }

    /// Test comments
    fn bar(&self, a: u16, b: &[u8], c: &str);
}

struct SimpleIfImpl;

#[impl_interface]
impl SimpleIf for SimpleIfImpl {
    #[cfg(test)]
    fn foo() -> u32 {
        456
    }

    /// Test comments2
    fn bar(&self, a: u16, b: &[u8], c: &str) {
        println!("{} {:?} {}", a, b, c);
        assert_eq!(b[1], 3);
    }
}

mod private {
    pub fn test_call_in_mod() {
        crate::call_interface!(super::SimpleIf::bar(123, &[2, 3, 5, 7, 11], "test"));
        crate::call_interface!(crate::SimpleIf::foo,);
    }
}

#[test]
fn test_crate_interface_call() {
    call_interface!(SimpleIf::bar, 123, &[2, 3, 5, 7, 11], "test");
    assert_eq!(call_interface!(SimpleIf::foo), 456);
    private::test_call_in_mod();
}

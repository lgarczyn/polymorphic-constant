#![no_std]
#![cfg(test)]

mod inner {
    use polymorphic_constant::polymorphic_constant;

    polymorphic_constant! {
        pub const PUBLIC: i16 | i32 = 10;
        pub (crate) const CRATE: i16 | i32 = 20;
        pub (in crate::inner) const RESTRICTED: i16 | i32 = 30;
        const PRIVATE: i16 | i32 = 40;
    }

    pub fn private_is_reachable_here() -> i32 {
        PRIVATE.i32 + RESTRICTED.i32
    }
}

use inner::{CRATE, PUBLIC};

#[test]
fn fields_follow_constant_visibility() {
    assert_eq!(PUBLIC.i16, 10);
    assert_eq!(PUBLIC.i32, 10);
    assert_eq!(CRATE.i16, 20);
    assert_eq!(CRATE.i32, 20);
}

#[test]
fn private_stays_module_local() {
    assert_eq!(inner::private_is_reachable_here(), 70);
}

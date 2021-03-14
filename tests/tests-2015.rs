#![no_std]
#![cfg(test)]

#[macro_use]
extern crate polymorphic_constant;

polymorphic_constant! {
    const PI: f32 | f64 = 3.141592653589793;
    pub (crate) const E: f32 | f64 = 2.7182818284590452353602874713526624977572;
    pub const UINT: u16 | u32 | u64 | usize | i16 | i32 | i64 | isize = 2047;
    pub const INT: i16 | i32 | i64 | isize = -2047;
}

#[test]
fn basic() {
    assert_eq!(PI.f64, 3.141592653589793f64);
}

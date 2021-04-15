#![no_std]
#![cfg(test)]

use polymorphic_constant::polymorphic_constant;

polymorphic_constant! {
    const PI: f32 | f64 = 3.141592653589793;
    pub (crate) const E: f32 | f64 = 2.7182818284590452353602874713526624977572;
    pub const UINT: u16 | u32 | u64 | usize | i16 | i32 | i64 | isize = 2047;
    pub const INT: i16 | i32 | i64 | isize = -2047;
}

#[test]
fn test_const() {
    assert_eq!(PI.f32, 3.141592653589793);
    assert_eq!(PI.f64, 3.141592653589793);
    assert_eq!(E.f32, 2.718281828459045235360287471f32);
    assert_eq!(E.f64, 2.718281828459045235360287471f64);
    assert_eq!(UINT.u16, 2047);
    assert_eq!(UINT.u32, 2047);
    assert_eq!(UINT.u64, 2047);
    assert_eq!(UINT.usize, 2047);
    assert_eq!(UINT.i16, 2047);
    assert_eq!(UINT.i32, 2047);
    assert_eq!(UINT.i64, 2047);
    assert_eq!(UINT.isize, 2047);
    assert_eq!(INT.i16, -2047);
    assert_eq!(INT.i32, -2047);
    assert_eq!(INT.i64, -2047);
    assert_eq!(INT.isize, -2047);
}

#[test]
fn test_copy() {
    let copy = INT;
    assert_eq!(copy.isize, -2047);
}

#[test]
fn test_local() {
    polymorphic_constant! {
        const TEST: u16 | u8 = 35;
    };

    assert_eq!(TEST.u16, 35);
    assert_eq!(TEST.u8, 35);
}

#[test]
fn test_into() {
    let pi: f32 = PI.into();

    assert_eq!(pi, 3.1415927f32);
}

#[test]
fn test_template() {
    fn times_pi<T: core::ops::Mul<T>>(value: T) -> <T as core::ops::Mul>::Output
    where
        PolymorphicConstantPi: Into<T>,
    {
        value * PI.into()
    }

    assert_eq!(times_pi(2.0), 6.283185307179586f64);
}

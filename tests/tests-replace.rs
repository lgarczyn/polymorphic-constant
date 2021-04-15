#![cfg(test)]

use polymorphic_constant::polymorphic_constant;

polymorphic_constant! {
    const PI: f32 | f64 = 3.141592653589793;
    const E: f32 | f64 = 2.7182818284590452353602874713526624977572;

    const PI_E: f32 | f64 = PI * E;
    
    // const PI2: f32 = (PI as f64).powf(2);

    pub const UINT: u16 | u32 | u64 | usize | i16 | i32 | i64 | isize = 2047;
    pub const INT: i16 | i32 | i64 | isize = -2047;

    pub const WAT:i32 = UINT * INT;
}

const fn checked_cast(a: i32) -> u8 {
    let b = a as u8;
    let c = b as i32;
    assert_eq!(a.sign(), b.sign(), "Cannot cast {} to this type, as it would overflow to {}", a, b);
    assert_eq!(a, c, "Cannot cast {} to this type, as it would overflow to {}", a, c);
    b
}

const a: u8 = checked_cast(1);

#[test]
fn test_const() {
    assert_eq!(PI.f64, 3.141592653589793);
    assert_eq!(E.f64, 2.718281828459045235360287471);
    assert_eq!(PI_E.f64, 3.141592653589793 * 2.718281828459045235360287471);
    assert_eq!(PI.f32 * PI.f32, 3.141592653589793 * 3.141592653589793);


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

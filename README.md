[![Workflow Status](https://github.com/lgarczyn/polymorphic-constant/workflows/main/badge.svg)](https://github.com/lgarczyn/polymorphic-constant/actions?query=workflow%3A%22main%22)
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# polymorphic-constant

A macro to generate numerical constants in multiple types at once.

You can have the number Pi be available in f64 or f32

It was designed with three goals in mind:

* Catch all overflow errors on compile-time
* Minimize the code footprint
* Be readable and easy to use

This is meant as a temporary fix to the year-long debate over [Rust Pre-RFC #1337](https://internals.rust-lang.org/t/pre-rfc-untyped-constants/1337/)

## Syntax

```rust
    use polymorphic_constant::polymorphic_constant;

    polymorphic_constant! {
        const PI: f32 | f64 = 3.141592653589793;
    }

    // Which can then be used as

    fn pi_squared () -> f64 {
        PI.f64 * PI.f64
    }
```

A few features are supported:

```rust
    use polymorphic_constant::polymorphic_constant;

    polymorphic_constant! {

        /// Doc comment attributes
        const PI: f32 | f64 = 3.141592653589793;

        // Visibility modifiers (for both constant and type)
        pub (crate) const E: f32 | f64 = 2.7182818284590452;

        // Nonzero numeric types (NonZeroI32, NonZeroU8, etc)
        const ASCII_LINE_RETURN: u8 | nz_u8 = 10;
    }

    // You can handle constants like any const struct
    const PI_COPY: PI = PI;
    const PI_F32: f32 = PI.f32;

    // Into is implemented for every variant of the constant
    fn times_pi<T: std::ops::Mul<T>> (value: T) -> <T as std::ops::Mul>::Output
    where
        PI: Into<T>,
    {
        value * PI.into()
    }

    assert_eq!(times_pi(2.0), 6.283185307179586f64);
```

## Safety

This system ensures that you keep all the safeties and warnings given by rust, but no more

Any incompatible type will prevent compilation:

* Float literals cannot be stored if it would convert them to infinity
```rust
const FAILS: f32 | f64 =  3141592653589793238462643383279502884197.0;
```

* Literals cannot be stored in a type too small to hold them
```rust
const FAILS: u64 | nz_i8 = 128;
```

* Negative numbers cannot be stored in unsigned types
```rust
const FAILS: i64 | u8 = -1;
```

* 0 cannot be stored in non-zero types
```rust
const FAILS: nz_u8 | nz_u16 | nz_u32 = 0;
```

* However, floats may lose precision, and a lot of it
```rust
const SUCCEEDS: f32 = 3.141592653589793238462643383279;
```

## Warnings

Currently, the same constant cannot hold both int and float variants
```rust
    const FAIL: i32 = 0.1;
```
```rust
    const FAIL: f32 = 0;
```

The constant also has to be initialized with an untyped literal
```rust
    const FAIL: i32 = 0u32;
```

It is still unclear if accepting the examples above could be dangerous,
thus the conservative choice.

## Example

```rust
use polymorphic_constant::polymorphic_constant;

polymorphic_constant! {
    const HEIGHT: i8 | u8 | i16 | u16 | i32 | u32 = 16;
    const WIDTH: i8 | u8 | i16 | u16 | i32 | u32 = 32;
}

fn main() {
    let size = HEIGHT.i16 * WIDTH.i16;

    assert_eq!(size, 16 * 32);

    let height_copy:i32 = HEIGHT.into();

    assert_eq!(HEIGHT.i32, height_copy);
}
```

## Support

I would love any feedback on usage, for future ameliorations and features.

License: MIT

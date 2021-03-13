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
        static PI: f32 | f64 = 3.141592653589793;
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
        static PI: f32 | f64 = 3.141592653589793;

        // Visibility modifiers (for both constant and type)
        pub (crate) static E: f32 | f64 = 2.7182818284590452;

        // Nonzero numeric types (NonZeroI32, NonZeroU8, etc)
        static ASCII_LINE_RETURN: u8 | nz_u8 = 10;
    }

    // You can handle constants like any static struct
    static PI_COPY: PI = PI;
    static PI_F32: f32 = PI.f32;

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
static FAILS: f32 | f64 =  3141592653589793238462643383279502884197.0;
```

* Literals cannot be stored in a type too small to hold them
```rust
static FAILS: u64 | nz_i8 = 128;
```

* Negative numbers cannot be stored in unsigned types
```rust
static FAILS: i64 | u8 = -1;
```

* 0 cannot be stored in non-zero types
```rust
static FAILS: nz_u8 | nz_u16 | nz_u32 = 0;
```

* However, floats may lose precision, and a lot of it
```rust
static SUCCEEDS: f32 = 3.141592653589793238462643383279;
```

## Warnings

Currently, the same constant cannot hold both int and float variants
```rust
    static FAIL: i32 = 0.1;
```
```rust
    static FAIL: f32 = 0;
```

The constant also has to be initialized with an untyped literal
```rust
    static FAIL: i32 = 0u32;
```

It is still unclear if accepting the examples above could be dangerous,
thus the conservative choice.

## Example

```rust
use polymorphic_constant::polymorphic_constant;

polymorphic_constant! {
    static HEIGHT: i8 | u8 | i16 | u16 | i32 | u32 = 16;
    static WIDTH: i8 | u8 | i16 | u16 | i32 | u32 = 32;
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

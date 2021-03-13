// Copyright 2020 Louis Garczynski
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![no_std]

/*!
A macro to generate numerical constants in multiple types at once.

You can have the number Pi be available in f64 or f32

It was designed with three goals in mind:

* Catch all overflow errors on compile-time
* Minimize the code footprint
* Be readable and easy to use

This is meant as a temporary fix to the year-long debate over [Rust Pre-RFC #1337](https://internals.rust-lang.org/t/pre-rfc-untyped-constants/1337/)

# Syntax

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

# Safety

This system ensures that you keep all the safeties and warnings given by rust, but no more

Any incompatible type will prevent compilation:

* Float literals cannot be stored if it would convert them to infinity
```compile_fail
    # use polymorphic_constant::polymorphic_constant;
    
    # polymorphic_constant! {
        static FAILS: f32 | f64 =  3141592653589793238462643383279502884197.0;
    # }
```

* Literals cannot be stored in a type too small to hold them
```compile_fail
    # use polymorphic_constant::polymorphic_constant;
    
    # polymorphic_constant! {
        static FAILS: u64 | nz_i8 = 128;
    # }
```

* Negative numbers cannot be stored in unsigned types
```compile_fail
    # use polymorphic_constant::polymorphic_constant;
    
    # polymorphic_constant! {
        static FAILS: i64 | u8 = -1;
    # }
```

* 0 cannot be stored in non-zero types
```compile_fail
    # use polymorphic_constant::polymorphic_constant;
    
    # polymorphic_constant! {
        static FAILS: nz_u8 | nz_u16 | nz_u32 = 0;
    # }
```

* However, floats may lose precision, and a lot of it
```rust
    # use polymorphic_constant::polymorphic_constant;
    
    # polymorphic_constant! {
        static SUCCEEDS: f32 = 3.141592653589793238462643383279;
    # }
```

# Warnings

Currently, the same constant cannot hold both int and float variants
```compile_fail
    # use polymorphic_constant::polymorphic_constant;
    
    # polymorphic_constant! {
        static FAIL: i32 = 0.1;
    # }
```
```compile_fail
    # use polymorphic_constant::polymorphic_constant;
    
    # polymorphic_constant! {
        static FAIL: f32 = 0;
    # }
```

The constant also has to be initialized with an untyped literal
```compile_fail
    # use polymorphic_constant::polymorphic_constant;
    
    # polymorphic_constant! {
        static FAIL: i32 = 0u32;
    # }
```

It is still unclear if accepting the examples above could be dangerous,
thus the conservative choice.

# Example

```
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

# Support

I would love any feedback on usage, for future ameliorations and features.
*/

/**
Define one or more polymorphic numerical constants. A constant X of value 10, available in i32 and u32 will read:
```
# use polymorphic_constant::polymorphic_constant;
polymorphic_constant! {
    static X: i32 | u32 = 10;
}
```
and be used like:
```
# use polymorphic_constant::polymorphic_constant;
# polymorphic_constant! { static X: i32 | u32 = 10; }
let x_i32 = X.i32;
```
*/

#[macro_export(local_inner_macros)]
macro_rules! polymorphic_constant {
    // Handle the static (pub?) CONST format
    ($(#[$attr:meta])* ($($vis:tt)*) static $name:ident : $( $numeric_type:ident )|* = $lit:literal; $($nextLine:tt)*) => {

        // Generate the struct to hold the constant

        // Remove warnings
        #[allow(non_camel_case_types)]
        // Derive the common traits, but only if std is available
        #[cfg_attr(not(no_std), derive(Debug, Clone, Copy))]
        // Expend the attributes passed by the user
        $(#[$attr])*
        // Add the visibility attributes
        $($vis)*
        // Create the struct
        struct $name {
            // For each type (f32, ...) create a new property
            $($numeric_type: __nz_impl!(@GET_TYPE $numeric_type),)*
        }

        // Implement `into` for every type
        $(impl ::core::convert::Into<__nz_impl!(@GET_TYPE $numeric_type)> for $name {
            fn into(self) -> __nz_impl!(@GET_TYPE $numeric_type) {
                self.$numeric_type
            }
        })*

        // Expand the visibility, this time for the constant
        $($vis)*
        // Instantiate the struct and create the constant
        static $name: $name = $name {
            $($numeric_type: __nz_impl!(@MAKE_VAL $lit, $numeric_type ),)*
        };
        // Keep munching until the next ;
        polymorphic_constant!($($nextLine)*);
    };

    // Handle `static CONST` format
    ($(#[$attr:meta])* static $($t:tt)*) => {
        // use `()` to explicitly forward the information about private items
        polymorphic_constant!($(#[$attr])* () static $($t)*);
    };
    // Handle `pub static CONST` format
    ($(#[$attr:meta])* pub static $($t:tt)*) => {
        polymorphic_constant!($(#[$attr])* (pub) static $($t)*);
    };
    // Handle `pub (crate) CONST` format and similar
    ($(#[$attr:meta])* pub ($($vis:tt)+) static $($t:tt)*) => {
        polymorphic_constant!($(#[$attr])* (pub ($($vis)+)) static $($t)*);
    };
    () => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __nz_impl {
    // Statically obtain a nonzero struct
    // Surprisingly fails to compile if $lit is 0 or not in range
    (@MAKE_VAL $lit:literal, nz_i8   ) => { unsafe { ::std::num::NonZeroI8::new_unchecked($lit) } };
    (@MAKE_VAL $lit:literal, nz_i16  ) => { unsafe { ::std::num::NonZeroI16::new_unchecked($lit) } };
    (@MAKE_VAL $lit:literal, nz_i32  ) => { unsafe { ::std::num::NonZeroI32::new_unchecked($lit) } };
    (@MAKE_VAL $lit:literal, nz_i64  ) => { unsafe { ::std::num::NonZeroI64::new_unchecked($lit) } };
    (@MAKE_VAL $lit:literal, nz_i128 ) => { unsafe { ::std::num::NonZeroI128::new_unchecked($lit) } };
    (@MAKE_VAL $lit:literal, nz_isize) => { unsafe { ::std::num::NonZeroIsize::new_unchecked($lit) } };
    (@MAKE_VAL $lit:literal, nz_u8   ) => { unsafe { ::std::num::NonZeroU8::new_unchecked($lit) } };
    (@MAKE_VAL $lit:literal, nz_u16  ) => { unsafe { ::std::num::NonZeroU16::new_unchecked($lit) } };
    (@MAKE_VAL $lit:literal, nz_u32  ) => { unsafe { ::std::num::NonZeroU32::new_unchecked($lit) } };
    (@MAKE_VAL $lit:literal, nz_u64  ) => { unsafe { ::std::num::NonZeroU64::new_unchecked($lit) } };
    (@MAKE_VAL $lit:literal, nz_u128 ) => { unsafe { ::std::num::NonZeroU128::new_unchecked($lit) } };
    (@MAKE_VAL $lit:literal, nz_usize) => { unsafe { ::std::num::NonZeroUsize::new_unchecked($lit) } };
    (@MAKE_VAL $lit:literal, $numeric_type:ident) => { $lit };

    // Get the full nonzero type from shorthand
    // Fails in nonstd
    (@GET_TYPE nz_i8   ) => { ::std::num::NonZeroI8 };
    (@GET_TYPE nz_i16  ) => { ::std::num::NonZeroI16 };
    (@GET_TYPE nz_i32  ) => { ::std::num::NonZeroI32 };
    (@GET_TYPE nz_i64  ) => { ::std::num::NonZeroI64 };
    (@GET_TYPE nz_i128 ) => { ::std::num::NonZeroI128 };
    (@GET_TYPE nz_isize) => { ::std::num::NonZeroIsize };
    (@GET_TYPE nz_u8   ) => { ::std::num::NonZeroU8 };
    (@GET_TYPE nz_u16  ) => { ::std::num::NonZeroU16 };
    (@GET_TYPE nz_u32  ) => { ::std::num::NonZeroU32 };
    (@GET_TYPE nz_u64  ) => { ::std::num::NonZeroU64 };
    (@GET_TYPE nz_u128 ) => { ::std::num::NonZeroU128 };
    (@GET_TYPE nz_usize) => { ::std::num::NonZeroUsize };
    (@GET_TYPE $numeric_type:ident) => { $numeric_type };
}

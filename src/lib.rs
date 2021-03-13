// Copyright 2020 Louis Garczynski
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![no_std]

// Add the examples in the README file
#[cfg(doctest)]
extern crate doc_comment;
#[cfg(doctest)]
::doc_comment::doctest!("../README.md");

/**
Define one or more polymorphic numerical constants.
 * A constant X of value 10, available in i32 and u32 will read:
```rust
polymorphic_constant! {
    static X: i32 | u32 = 10;
}
```
and be accessed as:
```rust
let x_f32 = X.f32;
```
*/

#[macro_export]
macro_rules! polymorphic_constant {
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
            $($numeric_type: polymorphic_constant!(@GET_TYPE $numeric_type),)*
        }

        // Implement `into` for every type
        $(impl ::core::convert::Into<polymorphic_constant!(@GET_TYPE $numeric_type)> for $name {
            fn into(self) -> polymorphic_constant!(@GET_TYPE $numeric_type) {
                self.$numeric_type
            }
        })*

        // Expand the visibility, this time for the constant
        $($vis)*
        // Instantiate the struct and create the constant
        static $name: $name = $name {
            $($numeric_type: polymorphic_constant!(@MAKE_VAL $lit, $numeric_type ),)*
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

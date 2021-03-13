#![cfg(test)]

use polymorphic_constant::polymorphic_constant;

#[test]
fn test_nz() {
    polymorphic_constant! {
        static NZ: nz_u16 | nz_u32 | nz_u64 | nz_usize | nz_i16 | nz_i32 | nz_i64 | nz_isize = 2047;
    }

    assert_eq!(NZ.nz_u16, ::std::num::NonZeroU16::new(2047).unwrap());
    assert_eq!(NZ.nz_u32, ::std::num::NonZeroU32::new(2047).unwrap());
    assert_eq!(NZ.nz_u64, ::std::num::NonZeroU64::new(2047).unwrap());
    assert_eq!(NZ.nz_usize, ::std::num::NonZeroUsize::new(2047).unwrap());
    assert_eq!(NZ.nz_i16, ::std::num::NonZeroI16::new(2047).unwrap());
    assert_eq!(NZ.nz_i32, ::std::num::NonZeroI32::new(2047).unwrap());
    assert_eq!(NZ.nz_i64, ::std::num::NonZeroI64::new(2047).unwrap());
    assert_eq!(NZ.nz_isize, ::std::num::NonZeroIsize::new(2047).unwrap());
}

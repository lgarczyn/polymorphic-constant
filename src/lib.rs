// Copyright 2020 Louis Garczynski
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

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
    const PI_COPY: PolymorphicConstantPi = PI;
    const PI_F32: f32 = PI.f32;

    // Into is implemented for every variant of the constant
    fn times_pi<T: std::ops::Mul<T>> (value: T) -> <T as std::ops::Mul>::Output
    where
        PolymorphicConstantPi: Into<T>,
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
        const FAILS: f32 | f64 =  3141592653589793238462643383279502884197.0;
    # }
```

* Literals cannot be stored in a type too small to hold them
```compile_fail
    # use polymorphic_constant::polymorphic_constant;
    # polymorphic_constant! {
        const FAILS: u64 | nz_i8 = 128;
    # }
```

* Negative numbers cannot be stored in unsigned types
```compile_fail
    # use polymorphic_constant::polymorphic_constant;
    # polymorphic_constant! {
        const FAILS: i64 | u8 = -1;
    # }
```

* 0 cannot be stored in non-zero types
```compile_fail
    # use polymorphic_constant::polymorphic_constant;
    # polymorphic_constant! {
        const FAILS: nz_u8 | nz_u16 | nz_u32 = 0;
    # }
```

* However, floats may lose precision, and a lot of it
```rust
    # use polymorphic_constant::polymorphic_constant;
    # polymorphic_constant! {
        const SUCCEEDS: f32 = 3.141592653589793238462643383279;
    # }
```

# Warnings

Currently, the same constant cannot hold both int and float variants
```compile_fail
    # use polymorphic_constant::polymorphic_constant;
    # polymorphic_constant! {
        const FAIL: i32 = 0.1;
    # }
```
```compile_fail
    # use polymorphic_constant::polymorphic_constant;
    # polymorphic_constant! {
        const FAIL: f32 = 0;
    # }
```

The constant also has to be initialized with an untyped literal
```compile_fail
    # use polymorphic_constant::polymorphic_constant;
    # polymorphic_constant! {
        const FAIL: i32 = 0u32;
    # }
```

It is still unclear if accepting the examples above could be dangerous,
thus the conservative choice.

# Example

```
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

# Support

I would love any feedback on usage, for future ameliorations and features.
*/

/**
Define one or more polymorphic numerical constants. A constant X of value 10, available in i32 and u32 will read:
```
# use polymorphic_constant::polymorphic_constant;
polymorphic_constant! {
    const X: i32 | u32 = 10;
}
```
and be used like:
```
# use polymorphic_constant::polymorphic_constant;
# polymorphic_constant! { const X: i32 | u32 = 10; }
let x_i32 = X.i32;
```
*/
use convert_case::{Case, Casing};
use proc_macro::{TokenStream as TokenStreamOut};
use proc_macro2::{TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens};
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, parse_quote, Attribute, Expr, Ident, Token, Type, Visibility};

type ExpWrapper = fn(TokenStream, Type) -> TokenStream;

// Holds a single type variant of a constant
struct PolymorphicVariant {
    name: Ident,
    token: Type,
    init_wrapper: ExpWrapper,
}

// Holds all type variants of a constant
struct PolymorphicType {
    variants: Vec<PolymorphicVariant>,
}

// Holds all the information related to a constant
struct PolymorphicConstant {
    attributes: Vec<Attribute>,
    visibility: Visibility,
    const_name: Ident,
    struct_name: Ident,
    types: PolymorphicType,
    init: Expr,
}

// Holds an entire multi-constant declaration
struct PolymorphicConstantVec {
    constants: Vec<PolymorphicConstant>,
}

//
impl PolymorphicVariant {
    fn null_wrapper(expr: TokenStream, ty: Type) -> TokenStream {
        quote! {
            unsafe { (#expr) as #ty }
        }
    }

    fn nz_wrapper(expr: TokenStream, ty: Type) -> TokenStream {
        quote! {
            unsafe { #ty::new_unchecked(#expr) }
        }
    }

    fn get_init(&self, expr: TokenStream) -> TokenStream {
        (self.init_wrapper)(expr, self.token.clone())
    }
}

// Implements the parsing of a constant type
impl Parse for PolymorphicVariant {
    fn parse(input: ParseStream) -> Result<Self> {
        // Get the type
        // Stored as ident to allow for NonZero types
        // Problem: users cannot use type aliases for primitives nor nz types
        let name: Ident = input.parse()?;

        // Get the two possible wrappers
        // Wrappers simply handle the initialization of variants based on their type
        // Only used for nonzero types
        // TODO: read up on hygienic types 
        let nz_wrapper = PolymorphicVariant::nz_wrapper as ExpWrapper;
        let null_wrapper = PolymorphicVariant::null_wrapper as ExpWrapper;

        let (init_wrapper, token): (ExpWrapper, Type) = match name.to_string().as_str() {
            "nz_i8" => (nz_wrapper, parse_quote! { ::std::num::NonZeroI8 }),
            "nz_i16" => (nz_wrapper, parse_quote! { ::std::num::NonZeroI16 }),
            "nz_i32" => (nz_wrapper, parse_quote! { ::std::num::NonZeroI32 }),
            "nz_i64" => (nz_wrapper, parse_quote! { ::std::num::NonZeroI64 }),
            "nz_i128" => (nz_wrapper, parse_quote! { ::std::num::NonZeroI128 }),
            "nz_isize" => (nz_wrapper, parse_quote! { ::std::num::NonZeroIsize }),
            "nz_u8" => (nz_wrapper, parse_quote! { ::std::num::NonZeroU8 }),
            "nz_u16" => (nz_wrapper, parse_quote! { ::std::num::NonZeroU16 }),
            "nz_u32" => (nz_wrapper, parse_quote! { ::std::num::NonZeroU32 }),
            "nz_u64" => (nz_wrapper, parse_quote! { ::std::num::NonZeroU64 }),
            "nz_u128" => (nz_wrapper, parse_quote! { ::std::num::NonZeroU128 }),
            "nz_usize" => (nz_wrapper, parse_quote! { ::std::num::NonZeroUsize }),
            _ => (null_wrapper, parse_quote! { #name }),
        };

        Ok(PolymorphicVariant {
            name,
            token,
            init_wrapper,
        })
    }
}

// Parse the "TYPE | TYPE | TYPE" structure
impl Parse for PolymorphicType {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut variants: Vec<PolymorphicVariant> = vec![input.parse()?];

        while input.parse::<Token![|]>().is_ok() {
            variants.push(input.parse()?);
        }

        Ok(PolymorphicType { variants })
    }
}

// Parse the complete constant structure
impl Parse for PolymorphicConstant {
    fn parse(input: ParseStream) -> Result<Self> {

        // Parse declaraction context
        let attributes: Vec<Attribute> = Attribute::parse_outer(input)?;
        let visibility: Visibility = input.parse()?;

        // Parse "cont CONST_NAME :"    
        input.parse::<Token![const]>()?;
        let const_name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;

        // Generate a struct name for the constant
        // Has to 
        let struct_name: Ident = format_ident!(
            "PolymorphicConstant{}",
            const_name.to_string().to_case(Case::UpperCamel)
        );

        let types: PolymorphicType = input.parse()?;

        input.parse::<Token![=]>()?;
        let init: Expr = input.parse()?;

        Ok(PolymorphicConstant {
            attributes,
            visibility,
            const_name,
            struct_name,
            types,
            init,
        })
    }
}

impl Parse for PolymorphicConstantVec {
    fn parse(input: ParseStream) -> Result<Self> {
        let constants: Vec<PolymorphicConstant> = input
            .parse_terminated::<PolymorphicConstant, Token![;]>(PolymorphicConstant::parse)?
            .into_iter()
            .collect();

        return Ok(PolymorphicConstantVec { constants });
    }
}

type ConstDic = HashMap<String, TokenStream>;

impl PolymorphicConstant {
    fn init_parser(init: Expr, dictionary: &ConstDic) -> TokenStream {
        init.into_token_stream()
            .into_iter()
            .map(|token| {
                if let TokenTree::Ident(ref ident) = token {
                    if let Some(expr) = dictionary.get(&ident.to_string()) {
                        return expr.clone();
                    }
                }
                TokenStream::from(token)
            })
            .flatten()
            .collect()
    }

    fn to_tokens(self, dictionary: &mut ConstDic) -> Result<TokenStream> {
        let PolymorphicConstant {
            attributes,
            visibility,
            const_name,
            struct_name,
            types,
            init,
        } = self;

        let init_replaced = PolymorphicConstant::init_parser(init.clone(), &dictionary);

        dictionary.insert(const_name.to_string(), quote! {(#init)}.into());

        let member_names: Vec<&Ident> = types.variants.iter().map(|t| &t.name).collect();

        let variant_types: Vec<&Type> = types.variants.iter().map(|t| &t.token).collect();

        let variant_inits: Vec<TokenStream> = types
            .variants
            .iter()
            .map(|t| t.get_init(init_replaced.clone()))
            .collect();

        Ok(quote! {
            // Derive the common traits, but only if std is available
            // #[cfg_attr(not(no_std), derive(Debug, Clone, Copy))]
            // Expend the attributes passed by the user
            #(#attributes)*
            // Add the visibility attributes
            #visibility
            // Create the struct
            struct #struct_name {
                #(
                    #member_names: #variant_types,
                )*
            }

            // Implements Into for each type
            #(
                impl ::core::convert::Into<#variant_types> for #struct_name {
                    fn into(self) -> #variant_types {
                        self.#member_names
                    }
                }
            )*

            // Expand the visibility, this time for the constant
            #visibility
            // Instantiate the struct and create the constant
            const #const_name: #struct_name = #struct_name {
                #(
                    #member_names: #variant_inits,
                )*
            };
        }
        .into())
    }
}

#[proc_macro]
pub fn polymorphic_constant(input: TokenStreamOut) -> TokenStreamOut {
    let PolymorphicConstantVec { constants } = parse_macro_input!(input as PolymorphicConstantVec);

    let mut dictionary = HashMap::<String, TokenStream>::new();

    // let nz_i8 = Ident::new("nz_i8", Span::def_site().into());
    // let nz_i16 = Ident::new("nz_i16", Span::def_site().into());
    // let nz_i32 = Ident::new("nz_i32", Span::def_site().into());
    // let nz_i64 = Ident::new("nz_i64", Span::def_site().into());
    // let nz_i128 = Ident::new("nz_i128", Span::def_site().into());
    // let nz_isize = Ident::new("nz_isize", Span::def_site().into());
    // let nz_u8 = Ident::new("nz_u8", Span::def_site().into());
    // let nz_u16 = Ident::new("nz_u16", Span::def_site().into());
    // let nz_u32 = Ident::new("nz_u32", Span::def_site().into());
    // let nz_u64 = Ident::new("nz_u64", Span::def_site().into());
    // let nz_u128 = Ident::new("nz_u128", Span::def_site().into());
    // let nz_usize = Ident::new("nz_usize", Span::def_site().into());

    (quote!{
        const fn 
    }).into_iter().chain(
    constants
        .into_iter()
        .map(|pc| pc.to_tokens(&mut dictionary))
        .flatten()
    .collect::<TokenStream>())
    .collect::<TokenStream>().into()
}

[![crates.io](https://img.shields.io/crates/v/squote.svg)](https://crates.io/crates/squote)
[![docs.rs](https://docs.rs/squote/badge.svg)](https://docs.rs/squote)
[![Build and Test](https://github.com/rylev/squote/workflows/Build%20and%20Test/badge.svg?event=push)](https://github.com/rylev/squote/actions)

# squote

A string backed implementation of the popular [`quote`](https://github.com/dtolnay/quote) crate.

## This crate vs. quote

When in doubt always prefer using `quote`. This crate was created because for very large code generation, quote can be slow when compared to simple string concatenation. If you're code generation never uses incoming TokenStreams (i.e., from a macro of some sort), then you might see some performance gain using this crate. 

There is [active work](https://github.com/dtolnay/quote/pull/162) to see if the performance gains from this crate can be merged into the `quote` crate in which case this crate would be deprecated. 

## Usage 

```rust 
use proc_macro::TokenStream;
use squote::quote;

#[proc_macro]
pub fn my_macro(stream: TokenStream) -> TokenStream {
    let tokens = quote! {
        impl<'a, T: ToTokens> ToTokens for &'a T {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                (**self).to_tokens(tokens)
            }
        }
    };
    s.parse::<TokenStream>().unwrap()
}
```
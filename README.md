# squote

A string backed implementation of the popular [`quote`](https://github.com/dtolnay/quote).

## This crate vs. quote

When in doubt always prefer using `quote`. This crate was created because for very large 
code generation, quote can be slow when compared to simple string concatenation. If you're
code generation never uses incoming TokenStreams (i.e., from a macro of some sort), then you
might see some performance gain using this crate. 
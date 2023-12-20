# token_limiter

Limit the number of [tokens](https://doc.rust-lang.org/proc_macro/enum.TokenTree.html) in your rust file. Use this for challenges where the legnth of your code is a contraint.

## Tokens

Rust tokens can be broken down into 4 categories
1. __literals:__ `"hello"`, `4`, or `42.3`
2. __idents:__ variable and struct names `x` or `Clone` or `into`. This also includes keywords like `let`, `if`, and `else`
3. __punctuation:__ Decided to exlude puntuation from the count of tokens for this use case. 
4. __group:__ sections of other tokens delimited by `{}`, `[]`, or `()`. Then tokens inside are counted as normal and the delimeters are not counted


## Usage

See example_usage

Add the following block to the top of your main.rs file
```rust
#![token_limiter::limit(32)]
#![allow(unused_imports, internal_features)]
#![feature(custom_inner_attributes, prelude_import)]
```

This requires some nightly features. You can set your project to nightly by adding a `rust-toolchain.toml` next to your `Cargo.toml` with the following contents

```toml
[toolchain]
channel = "nightly"
```

You could also just set your cargo to nightly in general with 
```
rustup default nightly
```
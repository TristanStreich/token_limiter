#![feature(proc_macro_diagnostic)]
use proc_macro::{Diagnostic, Level, Span, TokenStream, TokenTree};

macro_rules! error {
    ($file:ident, $($msg:tt)*) => {{
        let error_message = format!($($msg)*);
        let compile_error: TokenStream = quote::quote!{ compile_error!(#error_message); }.into();
        $file.extend(compile_error);
        return $file
    }
    };
}

#[proc_macro_attribute]
pub fn limit(args: TokenStream, mut file: TokenStream) -> TokenStream {
    let Ok(max_tokens) = args.to_string().parse::<u32>() else {
        error!(file, "failed to parse arg `{args}` as integer")
    };
    let num_tokens = file.clone().into_iter().map(count_tokens).sum::<u32>();

    if num_tokens > max_tokens {
        error!(
            file,
            "This file contains {num_tokens} tokens which is greater than the max of {max_tokens}"
        )
    }

    Diagnostic::spanned(
        Span::call_site(),
        Level::Note,
        format!("File currently contains {num_tokens} tokens"),
    )
    .emit();

    file
}

fn count_tokens(t: TokenTree) -> u32 {
    match t {
        TokenTree::Group(g) => g.stream().into_iter().map(count_tokens).sum(),
        // not counting punctuation. But we could change this if you want
        TokenTree::Punct(_) => 0,
        _ => 1,
    }
}

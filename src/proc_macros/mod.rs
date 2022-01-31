use proc_macro::{Literal, TokenStream, TokenTree};

// Unstable private API for use by `crokey::key_code!` only.
#[doc(hidden)]
/// Converts an identifier or punctuation token to its equivalent `char`.
///
/// This is the only part of `key_code!` that cannot be performed by a declarative macro alone.
/// It can also be done by `const`-evaluation but that doesn't work in pattern context.
#[proc_macro]
pub fn to_char(input: TokenStream) -> TokenStream {
    let c = to_char_inner(input.clone()).unwrap_or_else(|| panic!("unknown key code `{}`", input));

    TokenStream::from(TokenTree::Literal(Literal::character(c)))
}

fn to_char_inner(input: TokenStream) -> Option<char> {
    let mut input = input.into_iter();
    let c = match input.next().unwrap() {
        TokenTree::Ident(ident) => {
            let ident_string = ident.to_string();
            let mut chars = ident_string.chars();
            let c = chars.next().expect("empty identifiers are impossible");
            if chars.next().is_some() {
                return None;
            }
            c
        }
        TokenTree::Punct(punct) => punct.as_char(),
        _ => return None,
    };
    if input.next().is_some() {
        return None;
    }
    Some(c)
}

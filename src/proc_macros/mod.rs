use {
    crossterm::event::KeyCode,
    proc_macro::TokenStream as TokenStream1,
    proc_macro2::{Group, Span, TokenStream},
    quote::quote,
    strict::OneToThree,
    syn::{
        parse::{Error, Parse, ParseStream, Result},
        parse_macro_input, Ident, LitChar, LitInt, Token,
    },
};

struct KeyCombinationKey {
    pub crate_path: TokenStream,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub codes: OneToThree<TokenStream>,
}


// TODO to allow sorted codes:
// [x] implement map in OneToThree
// [x] extract parse_key_code from crokey::parse (returning a crossterm::KeyCode)
// [ ] write function KeyCode->TokenStream
// [ ] first build a OneToThree<crossterm::KeyCode>
// [ ] sort it
// [ ] then map it to a OneToThree<TokenStream> using the function KeyCode->TokenStream

// must be kept identical to crokey::parse_key_code
// (and yes, this duplication isn't ideal)
fn parse_key_code(
    raw: &str,
    shift: bool,
    code_span: Span,
) -> Result<KeyCode> {
    use KeyCode::*;
    let code = match raw {
        "esc" => Esc,
        "enter" => Enter,
        "left" => Left,
        "right" => Right,
        "up" => Up,
        "down" => Down,
        "home" => Home,
        "end" => End,
        "pageup" => PageUp,
        "pagedown" => PageDown,
        "backtab" => BackTab,
        "backspace" => Backspace,
        "del" => Delete,
        "delete" => Delete,
        "insert" => Insert,
        "ins" => Insert,
        "f1" => F(1),
        "f2" => F(2),
        "f3" => F(3),
        "f4" => F(4),
        "f5" => F(5),
        "f6" => F(6),
        "f7" => F(7),
        "f8" => F(8),
        "f9" => F(9),
        "f10" => F(10),
        "f11" => F(11),
        "f12" => F(12),
        "space" => Char(' '),
        "hyphen" => Char('-'),
        "minus" => Char('-'),
        "tab" => Tab,
        c if c.chars().count() == 1 => {
            let mut c = c.chars().next().unwrap();
            if shift {
                c = c.to_ascii_uppercase();
            }
            Char(c)
        }
        _ => {
            return Err(Error::new(
                code_span,
                format_args!("unrecognized key code {:?}", raw),
            ));
        }
    };
    Ok(code)
}


fn key_code_to_token_stream(key_code: KeyCode, code_span: Span) -> Result<TokenStream> {
    let ts = match key_code {
        KeyCode::Backspace => quote! { Backspace },
        KeyCode::Enter => quote! { Enter },
        KeyCode::Left => quote! { Left },
        KeyCode::Right => quote! { Right },
        KeyCode::Up => quote! { Up },
        KeyCode::Down => quote! { Down },
        KeyCode::Home => quote! { Home },
        KeyCode::End => quote! { End },
        KeyCode::PageUp => quote! { PageUp },
        KeyCode::PageDown => quote! { PageDown },
        KeyCode::Tab => quote! { Tab },
        KeyCode::BackTab => quote! { BackTab },
        KeyCode::Delete => quote! { Delete },
        KeyCode::Insert => quote! { Insert },
        KeyCode::F(n) => quote! { F(#n) },
        KeyCode::Char(c) => quote! { Char(#c) },
        KeyCode::Null => quote! { Null },
        KeyCode::Esc => quote! { Esc },
        KeyCode::CapsLock => quote! { CapsLock },
        KeyCode::ScrollLock => quote! { ScrollLock },
        KeyCode::NumLock => quote! { NumLock },
        KeyCode::PrintScreen => quote! { PrintScreen },
        KeyCode::Pause => quote! { Pause },
        KeyCode::Menu => quote! { Menu },
        KeyCode::KeypadBegin => quote! { KeypadBegin },
        // Media(MediaKeyCode),
        // Modifier(ModifierKeyCode),
        _ => {
            return Err(Error::new(
                code_span,
                format_args!("failed to encode {:?}", key_code),
            ));
        }
    };
    Ok(ts)
}

impl Parse for KeyCombinationKey {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let crate_path = input.parse::<Group>()?.stream();

        let mut ctrl = false;
        let mut alt = false;
        let mut shift = false;

        let (code, code_span) = loop {
            let lookahead = input.lookahead1();

            if lookahead.peek(LitChar) {
                let lit = input.parse::<LitChar>()?;
                break (lit.value().to_lowercase().collect(), lit.span());
            }

            if lookahead.peek(LitInt) {
                let int = input.parse::<LitInt>()?;
                let digits = int.base10_digits();
                if digits.len() > 1 {
                    return Err(Error::new(int.span(), "invalid key; must be between 0-9"));
                }
                break (digits.to_owned(), int.span());
            }

            if !lookahead.peek(Ident) {
                return Err(lookahead.error());
            }

            let ident = input.parse::<Ident>()?;
            let ident_value = ident.to_string().to_lowercase();
            let modifier = match &*ident_value {
                "ctrl" => &mut ctrl,
                "alt" => &mut alt,
                "shift" => &mut shift,
                _ => break (ident_value, ident.span()),
            };
            if *modifier {
                return Err(Error::new(
                    ident.span(),
                    format_args!("duplicate modifier {}", ident_value),
                ));
            }
            *modifier = true;

            input.parse::<Token![-]>()?;
        };

        // parse the key codes
        let first_code = parse_key_code(&code, shift, code_span)?;
        let codes = if input.parse::<Token![-]>().is_ok() {
            let ident = input.parse::<Ident>()?;
            let second_code = parse_key_code(&ident.to_string().to_lowercase(), shift, ident.span())?;
            if input.parse::<Token![-]>().is_ok() {
                let ident = input.parse::<Ident>()?;
                let third_code = parse_key_code(&ident.to_string().to_lowercase(), shift, ident.span())?;
                OneToThree::Three(first_code, second_code, third_code)
            } else {
                OneToThree::Two(first_code, second_code)
            }
        } else {
            OneToThree::One(first_code)
        };

        // sort according to key codes because comparing with pattern matching
        // received key combinations with parsed ones requires code ordering to
        // be consistent
        let codes = codes.sorted();

        // Produce the token stream which will build pattern matching comparable initializers
        let codes = codes.try_map(|key_code| key_code_to_token_stream(key_code, input.span()))?;

        Ok(KeyCombinationKey {
            crate_path,
            ctrl,
            alt,
            shift,
            codes,
        })
    }
}

// Not public API. This is internal and to be used only by `key!`.
#[doc(hidden)]
#[proc_macro]
pub fn key(input: TokenStream1) -> TokenStream1 {
    let KeyCombinationKey {
        crate_path,
        ctrl,
        alt,
        shift,
        codes,
    } = parse_macro_input!(input);

    let mut modifier_constant = "MODS".to_owned();
    if ctrl {
        modifier_constant.push_str("_CTRL");
    }
    if alt {
        modifier_constant.push_str("_ALT");
    }
    if shift {
        modifier_constant.push_str("_SHIFT");
    }
    let modifier_constant = Ident::new(&modifier_constant, Span::call_site());

    match codes {
        OneToThree::One(code) => {
            quote! {
                #crate_path::KeyCombination {
                    codes: #crate_path::__private::OneToThree::One(
                       #crate_path::__private::crossterm::event::KeyCode::#code
                    ),
                    modifiers: #crate_path::__private::#modifier_constant,
                }
            }
        }
        OneToThree::Two(a, b) => {
            quote! {
                #crate_path::KeyCombination {
                    codes: #crate_path::__private::OneToThree::Two(
                       #crate_path::__private::crossterm::event::KeyCode::#a,
                       #crate_path::__private::crossterm::event::KeyCode::#b,
                    ),
                    modifiers: #crate_path::__private::#modifier_constant,
                }
            }
        }
        OneToThree::Three(a, b, c) => {
            quote! {
                #crate_path::KeyCombination {
                    codes: #crate_path::__private::OneToThree::Three(
                       #crate_path::__private::crossterm::event::KeyCode::#a,
                       #crate_path::__private::crossterm::event::KeyCode::#b,
                       #crate_path::__private::crossterm::event::KeyCode::#c,
                    ),
                    modifiers: #crate_path::__private::#modifier_constant,
                }
            }
        }
    }
    .into()
}

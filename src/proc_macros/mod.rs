use {
    proc_macro::TokenStream as TokenStream1,
    quote::quote,
    syn::{
        parse::{Error, Parse, ParseStream, Result},
        parse_macro_input, Ident, LitChar, LitInt, Token,
    },
    proc_macro2::{TokenStream, Group, Span},
};

struct KeyEventDef {
    pub crate_path: TokenStream,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub code: String,
}

impl Parse for KeyEventDef {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let crate_path = input.parse::<Group>()?.stream();

        let mut ctrl = false;
        let mut alt = false;
        let mut shift = false;

        let code = loop {
            let lookahead = input.lookahead1();

            if lookahead.peek(LitChar) {
                break input.parse::<LitChar>()?.value().to_lowercase().collect();
            }

            if lookahead.peek(LitInt) {
                let int = input.parse::<LitInt>()?;
                let digits = int.base10_digits();
                if digits.len() > 1 {
                    return Err(Error::new(
                        int.span(),
                        "invalid key; must be between 0-9",
                    ));
                }
                break digits.to_owned();
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
                _ => break ident_value,
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
        Ok(KeyEventDef {
            crate_path,
            ctrl,
            alt,
            shift,
            code,
        })
    }
}

// Not public API. This is internal and to be used only by `key!`.
#[doc(hidden)]
#[proc_macro]
pub fn key(input: TokenStream1) -> TokenStream1 {
    let key_def = parse_macro_input!(input as KeyEventDef);

    let crate_path = key_def.crate_path;

    let mut modifier_constant = "MODS".to_owned();
    if key_def.ctrl {
        modifier_constant.push_str("_CTRL");
    }
    if key_def.alt {
        modifier_constant.push_str("_ALT");
    }
    if key_def.shift {
        modifier_constant.push_str("_SHIFT");
    }
    let modifier_constant = Ident::new(&modifier_constant, Span::call_site());

    let code = match key_def.code.as_ref() {
        "backspace" => quote! { Backspace },
        "backtab" => quote! { BackTab },
        "del" => quote! { Delete },
        "delete" => quote! { Delete },
        "down" => quote! { Down },
        "end" => quote! { End },
        "enter" => quote! { Enter },
        "esc" => quote! { Esc },
        "f1" => quote! { F(1) },
        "f2" => quote! { F(2) },
        "f3" => quote! { F(3) },
        "f4" => quote! { F(4) },
        "f5" => quote! { F(5) },
        "f6" => quote! { F(6) },
        "f7" => quote! { F(7) },
        "f8" => quote! { F(8) },
        "f9" => quote! { F(9) },
        "f10" => quote! { F(10) },
        "f11" => quote! { F(11) },
        "f12" => quote! { F(12) },
        "home" => quote! { Home },
        "ins" => quote! { Insert },
        "insert" => quote! { Insert },
        "left" => quote! { Left },
        "pagedown" => quote! { PageDown },
        "pageup" => quote! { PageUp },
        "right" => quote! { Right },
        "space" => quote! { Char(' ') },
        "tab" => quote! { Tab },
        "up" => quote! { Up },
        c if c.chars().count() == 1 => {
            let c = c.chars().next().unwrap();
            quote! { Char(#c) }
        }
        _ => {
            panic!("Unrecognized key code: {:?}", key_def.code);
        }
    };
    quote! {
        #crate_path::__private::crossterm::event::KeyEvent {
            modifiers: #crate_path::__private::#modifier_constant,
            code: #crate_path::__private::crossterm::event::KeyCode::#code,
        }
    }
    .into()
}

use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{
        parse::{Error, Parse, ParseStream, Result},
        parse_macro_input, Ident, LitChar, LitInt, Token,
    },
};

struct KeyEventDef {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub code: String,
}

impl Parse for KeyEventDef {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
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
            ctrl,
            alt,
            shift,
            code,
        })
    }
}

/// check and expand at compile-time the provided expression
/// into a valid KeyEvent.
///
///
/// For example:
/// ```
/// # use crokey_proc_macros::key;
/// let key_event = key!(ctrl-c);
/// ```
/// is expanded into:
///
/// ```
/// let key_event = crossterm::event::KeyEvent {
///     modifiers: crossterm::event::KeyModifiers::CONTROL,
///     code: crossterm::event::KeyCode::Char('\u{63}'),
/// };
/// ```
///
/// Keys which can't be valid identifiers or single numbers in Rust must be put between simple quotes:
/// ```
/// # use crokey_proc_macros::key;
/// let ke = key!(shift-'?');
/// let ke = key!(alt-']');
/// ```
#[proc_macro]
pub fn key(input: TokenStream) -> TokenStream {
    let key_def = parse_macro_input!(input as KeyEventDef);
    let modifiers = match (key_def.ctrl, key_def.alt, key_def.shift) {
        (false, false, false) => quote! { crossterm::event::KeyModifiers::NONE },
        (true, false, false) => quote! { crossterm::event::KeyModifiers::CONTROL },
        (true, true, false) => quote! {
            crossterm::event::KeyModifiers::CONTROL | crossterm::event::KeyModifiers::ALT
        },
        (true, false, true) => quote! {
            crossterm::event::KeyModifiers::CONTROL | crossterm::event::KeyModifiers::SHIFT
        },
        (true, true, true) => quote! {
            crossterm::event::KeyModifiers::CONTROL
                | crossterm::event::KeyModifiers::ALT
                | crossterm::event::KeyModifiers::SHIFT
        },
        (false, true, false) => quote! { crossterm::event::KeyModifiers::ALT },
        (false, true, true) => quote! {
            crossterm::event::KeyModifiers::ALT | crossterm::event::KeyModifiers::SHIFT
        },
        (false, false, true) => quote! { crossterm::event::KeyModifiers::SHIFT },
    };
    let code = match key_def.code.as_ref() {
        "backspace" => quote! { crossterm::event::KeyCode::Backspace },
        "backtab" => quote! { crossterm::event::KeyCode::BackTab },
        "del" => quote! { crossterm::event::KeyCode::Delete },
        "delete" => quote! { crossterm::event::KeyCode::Delete },
        "down" => quote! { crossterm::event::KeyCode::Down },
        "end" => quote! { crossterm::event::KeyCode::End },
        "enter" => quote! { crossterm::event::KeyCode::Enter },
        "esc" => quote! { crossterm::event::KeyCode::Esc },
        "f1" => quote! { crossterm::event::KeyCode::F(1) },
        "f2" => quote! { crossterm::event::KeyCode::F(2) },
        "f3" => quote! { crossterm::event::KeyCode::F(3) },
        "f4" => quote! { crossterm::event::KeyCode::F(4) },
        "f5" => quote! { crossterm::event::KeyCode::F(5) },
        "f6" => quote! { crossterm::event::KeyCode::F(6) },
        "f7" => quote! { crossterm::event::KeyCode::F(7) },
        "f8" => quote! { crossterm::event::KeyCode::F(8) },
        "f9" => quote! { crossterm::event::KeyCode::F(9) },
        "f10" => quote! { crossterm::event::KeyCode::F(10) },
        "f11" => quote! { crossterm::event::KeyCode::F(11) },
        "f12" => quote! { crossterm::event::KeyCode::F(12) },
        "home" => quote! { crossterm::event::KeyCode::Home },
        "ins" => quote! { crossterm::event::KeyCode::Insert },
        "insert" => quote! { crossterm::event::KeyCode::Insert },
        "left" => quote! { crossterm::event::KeyCode::Left },
        "pagedown" => quote! { crossterm::event::KeyCode::PageDown },
        "pageup" => quote! { crossterm::event::KeyCode::PageUp },
        "right" => quote! { crossterm::event::KeyCode::Right },
        "space" => quote! { crossterm::event::KeyCode::Char(' ') },
        "tab" => quote! { crossterm::event::KeyCode::Tab },
        "up" => quote! { crossterm::event::KeyCode::Up },
        c if c.chars().count() == 1 => {
            let c = c.chars().next().unwrap();
            quote! { crossterm::event::KeyCode::Char(#c) }
        }
        _ => {
            panic!("Unrecognized key code: {:?}", key_def.code);
        }
    };
    quote! {
        crossterm::event::KeyEvent {
            modifiers: #modifiers,
            code: #code,
        }
    }
    .into()
}

use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{
        parse::{Parse, ParseStream, Result},
        parse_macro_input,
        Ident, LitChar, Token,
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
        let mut code: Option<String> = None;
        let mut ctrl = false;
        let mut alt = false;
        let mut shift = false;
        fn set(code: &mut Option<String>, c: String) {
            if let Some(old_code) = &code {
                // if that wasn't the last one, then it was a modifier
                panic!("Unrecognized key modifier: {:?}", old_code);
            }
            *code = Some(c);
        }
        loop {
            if let Ok(c) = input.parse::<LitChar>() {
                set(&mut code, c.value().to_lowercase().collect());
                break;
            }
            // // unclear why a single digit isn't recognized here
            // if let Ok(d) = input.parse::<syn::LitInt>() {
            //     let d = d.base10_digits();
            //     if d.len() > 1 {
            //         panic!("Not a valid key: {:?}", d);
            //     }
            //     set(&mut code, d);
            //     break;
            // }
            if let Ok(ident) = input.parse::<Ident>() {
                let ident = ident.to_string().to_lowercase();
                match ident.as_ref() {
                    "ctrl" => { ctrl = true; }
                    "alt" => { alt = true; }
                    "shift" => { shift = true; }
                    _ => {
                        set(&mut code, ident);
                    }
                }
                let _ = input.parse::<Token! [-]>(); // separator ignored
            } else {
                break;
            }
        }
        Ok(KeyEventDef {
            ctrl,
            alt,
            shift,
            code: code.expect("key code must be provided"),
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
/// Keys which can't be valid identifiers in Rust must be put between simple quotes:
/// ```
/// # use crokey_proc_macros::key;
/// let ke = key!(shift-'?');
/// let ke = key!('5');
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
    }.into()
}



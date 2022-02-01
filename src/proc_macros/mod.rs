use {
    proc_macro::TokenStream as TokenStream1,
    quote::quote,
    syn::{
        parse::{Parse, ParseStream, Result},
        parse_macro_input,
        Ident, LitChar, Token,
    },
    proc_macro2::{TokenStream, Group},
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
            crate_path,
            ctrl,
            alt,
            shift,
            code: code.expect("key code must be provided"),
        })
    }
}

// Not public API. This is internal and to be used only by `key!`.
#[doc(hidden)]
#[proc_macro]
pub fn key(input: TokenStream1) -> TokenStream1 {
    let key_def = parse_macro_input!(input as KeyEventDef);

    let crate_path = key_def.crate_path;
    let crossterm = quote!(#crate_path::__private::crossterm);

    let modifiers = match (key_def.ctrl, key_def.alt, key_def.shift) {
        (false, false, false) => quote! { #crossterm::event::KeyModifiers::NONE },
        (true, false, false) => quote! { #crossterm::event::KeyModifiers::CONTROL },
        (true, true, false) => quote! {
            #crossterm::event::KeyModifiers::CONTROL | #crossterm::event::KeyModifiers::ALT
        },
        (true, false, true) => quote! {
            #crossterm::event::KeyModifiers::CONTROL | #crossterm::event::KeyModifiers::SHIFT
        },
        (true, true, true) => quote! {
            #crossterm::event::KeyModifiers::CONTROL
                | #crossterm::event::KeyModifiers::ALT
                | #crossterm::event::KeyModifiers::SHIFT
        },
        (false, true, false) => quote! { #crossterm::event::KeyModifiers::ALT },
        (false, true, true) => quote! {
            #crossterm::event::KeyModifiers::ALT | #crossterm::event::KeyModifiers::SHIFT
        },
        (false, false, true) => quote! { #crossterm::event::KeyModifiers::SHIFT },
    };
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
        #crossterm::event::KeyEvent {
            modifiers: #modifiers,
            code: #crossterm::event::KeyCode::#code,
        }
    }.into()
}



//! Crokey helps incorporate configurable keybindings in [crossterm](https://github.com/crossterm-rs/crossterm)
//! based terminal applications by providing functions
//! - parsing key combinations from strings
//! - describing key combinations in strings
//! - parsing key combinations at compile time
//! - combining Crossterm key events in key combinations
//!
//! ## The KeyCombination
//!
//! A `KeyCombination` is made of 1 to 3 "normal" keys with some optional modifiers (alt, shift, ctrl).
//!
//! It can be parsed, ergonomically built with the `key!` macro, obtained from key events.
//!
//! ## The Combiner
//!
//! With a `Combiner`, you can change raw Crossterm key events into key combinations.
//!
//! When the terminal is modern enough and supports the Kitty protocol, complex combinations with up to three non-modifier keys may be formed, for example `Ctrl-Alt-Shift-g-y` or `i-u`.
//!
//! For standard ANSI terminals, only regular combinations are available, like `Shift-o`, `Ctrl-Alt-Shift-g` or `i`.
//!
//! The combiner works in both cases:
//! if you presses the `ctrl`, `i`, and `u ` keys at the same time, it will result in one combination (`ctrl-i-u`) on a kitty-compatible terminal, and as a sequence of 2 key combinations (`ctrl-i` then `ctrl-u` assuming you started pressing the `i` before the `u`) in other terminals.
//!
//!
//! The `print_key` example shows how to use the combiner.
//!
//! ```no_run
//! # use {
//! #     crokey::*,
//! #     crossterm::{
//! #         event::{read, Event},
//! #         style::Stylize,
//! #         terminal,
//! #     },
//! # };
//! let fmt = KeyCombinationFormat::default();
//! let mut combiner = Combiner::default();
//! let combines = combiner.enable_combining().unwrap();
//! if combines {
//!     println!("Your terminal supports combining keys");
//! } else {
//!     println!("Your terminal doesn't support combining non-modifier keys");
//! }
//! println!("Type any key combination");
//! loop {
//!     terminal::enable_raw_mode().unwrap();
//!     let e = read();
//!     terminal::disable_raw_mode().unwrap();
//!     match e {
//!         Ok(Event::Key(key_event)) => {
//!             if let Some(key_combination) = combiner.transform(key_event) {
//!                 match key_combination {
//!                     key!(ctrl-c) | key!(ctrl-q) => {
//!                         println!("quitting");
//!                         break;
//!                     }
//!                     _ => {
//!                         println!("You typed {}", fmt.to_string(key_combination));
//!                     }
//!                 }
//!             }
//!         },
//!         _ => {}
//!     }
//! }
//! ```
//!
//! ## Parse a string
//!
//! Those strings are usually provided by a configuration file.
//!
//! ```
//! use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
//! assert_eq!(
//!     crokey::parse("alt-enter").unwrap(),
//!     KeyEvent::new(KeyCode::Enter, KeyModifiers::ALT).into(),
//! );
//! assert_eq!(
//!     crokey::parse("shift-F6").unwrap(),
//!     KeyEvent::new(KeyCode::F(6), KeyModifiers::SHIFT).into(),
//! );
//! ```
//!
//! ## Use key event "literals" thanks to procedural macros
//!
//! Those key events are parsed at compile time and have zero runtime cost.
//!
//! They're efficient and convenient for matching events or defining hardcoded keybindings.
//!
//! ```no_run
//! # use crokey::*;
//! # use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
//! # use crossterm::style::Stylize;
//! # let key_event = key!(a);
//! let fmt = KeyCombinationFormat::default();
//! # loop {
//! match key_event {
//!     key!(ctrl-c) => {
//!         println!("Arg! You savagely killed me with a {}", fmt.to_string(key_event).red());
//!         break;
//!     }
//!     key!(ctrl-q) => {
//!         println!("You typed {} which gracefully quits", fmt.to_string(key_event).green());
//!         break;
//!     }
//!     _ => {
//!         println!("You typed {}", fmt.to_string(key_event).blue());
//!     }
//! }
//! # }
//! ```
//! Complete example in `/examples/print_key`
//!
//! ## Display a string with a configurable format
//!
//! ```
//! use crokey::*;
//! use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
//!
//! // The default format
//! let format = KeyCombinationFormat::default();
//! assert_eq!(format.to_string(key!(shift-a)), "Shift-a");
//! assert_eq!(format.to_string(key!(ctrl-c)), "Ctrl-c");
//!
//! // A more compact format
//! let format = KeyCombinationFormat::default()
//!     .with_implicit_shift()
//!     .with_control("^");
//! assert_eq!(format.to_string(key!(shift-a)), "A");
//! assert_eq!(format.to_string(key!(ctrl-c)), "^c");
//! ```
//!
//! ## Deserialize keybindings using Serde
//!
//! With the "serde" feature enabled, you can read configuration files in a direct way:
//!
//! ```
//! use {
//!     crokey::*,
//!     crossterm::event::KeyEvent,
//!     serde::Deserialize,
//!     std::collections::HashMap,
//! };
//! #[derive(Debug, Deserialize)]
//! struct Config {
//!     keybindings: HashMap<KeyCombination, String>,
//! }
//! static CONFIG_HJSON: &str = r#"
//! {
//!     keybindings: {
//!         a: aardvark
//!         shift-b: babirussa
//!         ctrl-k: koala
//!         alt-j: jaguar
//!     }
//! }
//! "#;
//! let config: Config = deser_hjson::from_str(CONFIG_HJSON).unwrap();
//! let key: KeyCombination = key!(shift-b);
//! assert_eq!(
//!     config.keybindings.get(&key).unwrap(),
//!     "babirussa",
//! );
//! ```
//!
//! Instead of Hjson, you can use any Serde compatible format such as JSON or TOML.
//!

mod combiner;
mod format;
mod key_event;
mod parse;
mod key_combination;

pub use {
    combiner::*,
    crossterm,
    format::*,
    key_event::*,
    parse::*,
    key_combination::*,
    strict::OneToThree,
};

use {
    crossterm::event::{KeyCode, KeyModifiers},
    once_cell::sync::Lazy,
};

/// A lazy initialized KeyCombinationFormat which can be considered as standard
/// and which is used in the Display implementation of the [KeyCombination] type.
pub static STANDARD_FORMAT: Lazy<KeyCombinationFormat> = Lazy::new(KeyCombinationFormat::default);


/// check and expand at compile-time the provided expression
/// into a valid KeyCombination.
///
///
/// For example:
/// ```
/// # use crokey::key;
/// let key_event = key!(ctrl-c);
/// ```
/// is expanded into (roughly):
///
/// ```
/// let key_event = crokey::KeyCombination {
///     modifiers: crossterm::event::KeyModifiers::CONTROL,
///     codes: crokey::OneToThree::One(crossterm::event::KeyCode::Char('c')),
/// };
/// ```
///
/// Keys which can't be valid identifiers or digits in Rust must be put between simple quotes:
/// ```
/// # use crokey::key;
/// let ke = key!(shift-'?');
/// let ke = key!(alt-']');
/// ```
#[macro_export]
macro_rules! key {
    ($($tt:tt)*) => {
        $crate::__private::key!(($crate) $($tt)*)
    };
}

// Not public API. This is internal and to be used only by `key!`.
#[doc(hidden)]
pub mod __private {
    pub use crokey_proc_macros::key;
    pub use crossterm;
    pub use strict::OneToThree;

    use crossterm::event::KeyModifiers;
    pub const MODS: KeyModifiers = KeyModifiers::NONE;
    pub const MODS_CTRL: KeyModifiers = KeyModifiers::CONTROL;
    pub const MODS_CMD: KeyModifiers = KeyModifiers::SUPER;
    pub const MODS_ALT: KeyModifiers = KeyModifiers::ALT;
    pub const MODS_SHIFT: KeyModifiers = KeyModifiers::SHIFT;
    pub const MODS_CTRL_ALT: KeyModifiers = KeyModifiers::CONTROL.union(KeyModifiers::ALT);
    pub const MODS_CMD_ALT: KeyModifiers = KeyModifiers::SUPER.union(KeyModifiers::ALT);
    pub const MODS_ALT_SHIFT: KeyModifiers = KeyModifiers::ALT.union(KeyModifiers::SHIFT);
    pub const MODS_CTRL_SHIFT: KeyModifiers = KeyModifiers::CONTROL.union(KeyModifiers::SHIFT);
    pub const MODS_CMD_SHIFT: KeyModifiers = KeyModifiers::SUPER.union(KeyModifiers::SHIFT);
    pub const MODS_CTRL_ALT_SHIFT: KeyModifiers = KeyModifiers::CONTROL
        .union(KeyModifiers::ALT)
        .union(KeyModifiers::SHIFT);
    pub const MODS_CMD_ALT_SHIFT: KeyModifiers = KeyModifiers::SUPER
        .union(KeyModifiers::ALT)
        .union(KeyModifiers::SHIFT);
}

#[cfg(test)]
mod tests {
    use {
        crate::{key, KeyCombination, OneToThree},
        crossterm::event::{KeyCode, KeyModifiers},
    };

    const _: () = {
        key!(x);
        key!(ctrl - '{');
        key!(alt - '{');
        key!(shift - '{');
        key!(ctrl - alt - f10);
        key!(alt - shift - f10);
        key!(ctrl - shift - f10);
        key!(cmd - shift - f10);
        key!(ctrl - alt - shift - enter);
    };

    fn no_mod(code: KeyCode) -> KeyCombination {
        code.into()
    }

    #[test]
    fn key() {
        assert_eq!(key!(backspace), no_mod(KeyCode::Backspace));
        assert_eq!(key!(bAcKsPaCe), no_mod(KeyCode::Backspace));
        assert_eq!(key!(0), no_mod(KeyCode::Char('0')));
        assert_eq!(key!(9), no_mod(KeyCode::Char('9')));
        assert_eq!(key!('x'), no_mod(KeyCode::Char('x')));
        assert_eq!(key!('X'), no_mod(KeyCode::Char('x')));
        assert_eq!(key!(']'), no_mod(KeyCode::Char(']')));
        assert_eq!(key!('ඞ'), no_mod(KeyCode::Char('ඞ')));
        assert_eq!(key!(f), no_mod(KeyCode::Char('f')));
        assert_eq!(key!(F), no_mod(KeyCode::Char('f')));
        assert_eq!(key!(ඞ), no_mod(KeyCode::Char('ඞ')));
        assert_eq!(key!(f10), no_mod(KeyCode::F(10)));
        assert_eq!(key!(F10), no_mod(KeyCode::F(10)));
        assert_eq!(
            key!(ctrl - c),
            KeyCombination::new(KeyCode::Char('c'), KeyModifiers::CONTROL)
        );
        assert_eq!(
            key!(alt - shift - c),
            KeyCombination::new(KeyCode::Char('C'), KeyModifiers::ALT | KeyModifiers::SHIFT)
        );
        assert_eq!(key!(shift - alt - '2'), key!(ALT - SHIFT - 2));
        assert_eq!(key!(space), key!(' '));
        assert_eq!(key!(hyphen), key!('-'));
        assert_eq!(key!(minus), key!('-'));

        assert_eq!(
            key!(ctrl-alt-a-b),
            KeyCombination::new(
                OneToThree::Two(KeyCode::Char('a'), KeyCode::Char('b')),
                KeyModifiers::CONTROL | KeyModifiers::ALT,
            )
        );
        assert_eq!(
            key!(alt-f4-a-b),
            KeyCombination::new(
                OneToThree::Three(KeyCode::F(4), KeyCode::Char('a'), KeyCode::Char('b')),
                KeyModifiers::ALT,
            )
        );
        assert_eq!( // check that key codes are sorted
            key!(alt-a-b-f4),
            KeyCombination::new(
                OneToThree::Three(KeyCode::F(4), KeyCode::Char('a'), KeyCode::Char('b')),
                KeyModifiers::ALT,
            )
        );
        assert_eq!(
            key!(z-e),
            KeyCombination::new(
                OneToThree::Two(KeyCode::Char('e'), KeyCode::Char('z')),
                KeyModifiers::NONE,
            )
        );
    }

    #[test]
    fn format() {
        let format = crate::KeyCombinationFormat::default();
        assert_eq!(format.to_string(key!(insert)), "Insert");
        assert_eq!(format.to_string(key!(space)), "Space");
        assert_eq!(format.to_string(key!(alt-Space)), "Alt-Space");
        assert_eq!(format.to_string(key!(shift-' ')), "Shift-Space");
        assert_eq!(format.to_string(key!(alt-hyphen)), "Alt-Hyphen");
        assert_eq!(format.to_string(key!(cmd-f10)), "Cmd-F10");
    }

    #[test]
    fn key_pattern() {
        assert!(matches!(key!(ctrl-alt-shift-c), key!(ctrl-alt-shift-c)));
        assert!(!matches!(key!(ctrl-c), key!(ctrl-alt-shift-c)));
        assert!(matches!(key!(ctrl-alt-b), key!(ctrl-alt-b)));
        assert!(matches!(key!(ctrl-b), key!(ctrl-b)));
        assert!(matches!(key!(alt-b), key!(alt-b)));
        assert!(!matches!(key!(ctrl-b), key!(alt-b)));
        assert!(!matches!(key!(alt-b), key!(ctrl-b)));
        assert!(!matches!(key!(alt-b), key!(ctrl-alt-b)));
        assert!(!matches!(key!(ctrl-b), key!(ctrl-alt-b)));
        assert!(!matches!(key!(ctrl-alt-b), key!(alt-b)));
        assert!(!matches!(key!(ctrl-alt-b), key!(ctrl-b)));
    }

    #[test]
    fn ui() {
        trybuild::TestCases::new().compile_fail("tests/ui/*.rs");
    }
}

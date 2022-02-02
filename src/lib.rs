//! Crokey helps incorporate configurable keybindings in [crossterm](https://github.com/crossterm-rs/crossterm)
//! based terminal applications by providing functions
//! - parsing key combinations from strings
//! - describing key combinations in strings
//! - parsing key combinations at compile time
//!
//! ## Parse a string
//!
//! Those strings are usually provided by a configuration file.
//!
//! ```
//! use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
//! assert_eq!(
//!     crokey::parse("alt-enter").unwrap(),
//!     KeyEvent::new(KeyCode::Enter, KeyModifiers::ALT),
//! );
//! assert_eq!(
//!     crokey::parse("shift-F6").unwrap(),
//!     KeyEvent::new(KeyCode::F(6), KeyModifiers::SHIFT),
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
//! let fmt = KeyEventFormat::default();
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
//! let format = KeyEventFormat::default();
//! assert_eq!(format.to_string(key!(shift-a)), "Shift-a");
//! assert_eq!(format.to_string(key!(ctrl-c)), "Ctrl-c");
//!
//! // A more compact format
//! let format = KeyEventFormat::default()
//!     .with_implicit_shift()
//!     .with_control("^");
//! assert_eq!(format.to_string(key!(shift-a)), "A");
//! assert_eq!(format.to_string(key!(ctrl-c)), "^c");
//! ```

mod format;
mod parse;

pub use {crokey_proc_macros::*, format::*, parse::*};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// return the raw char if the event is a letter event
pub const fn as_letter(key: KeyEvent) -> Option<char> {
    match key {
        KeyEvent {
            code: KeyCode::Char(l),
            modifiers: KeyModifiers::NONE,
        } => Some(l),
        _ => None,
    }
}

/// check and expand at compile-time the provided expression
/// into a valid KeyEvent.
///
///
/// For example:
/// ```
/// # use crokey::key;
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
/// # use crokey::key;
/// let ke = key!(shift-'?');
/// let ke = key!('5');
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

    use crossterm::event::KeyModifiers;
    pub const MODS: KeyModifiers = KeyModifiers::NONE;
    pub const MODS_CTRL: KeyModifiers = KeyModifiers::CONTROL;
    pub const MODS_ALT: KeyModifiers = KeyModifiers::ALT;
    pub const MODS_SHIFT: KeyModifiers = KeyModifiers::SHIFT;
    pub const MODS_CTRL_ALT: KeyModifiers = KeyModifiers::CONTROL.union(KeyModifiers::ALT);
    pub const MODS_ALT_SHIFT: KeyModifiers = KeyModifiers::ALT.union(KeyModifiers::SHIFT);
    pub const MODS_CTRL_SHIFT: KeyModifiers = KeyModifiers::CONTROL.union(KeyModifiers::SHIFT);
    pub const MODS_CTRL_ALT_SHIFT: KeyModifiers = KeyModifiers::CONTROL
        .union(KeyModifiers::ALT)
        .union(KeyModifiers::SHIFT);
}

#[cfg(test)]
mod tests {
    use {
        crate::key,
        crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    };

    const _: () = {
        key!(x);
        key!(ctrl - '{');
        key!(alt - '{');
        key!(shift - '{');
        key!(ctrl - alt - f10);
        key!(alt - shift - f10);
        key!(ctrl - shift - f10);
        key!(ctrl - alt - shift - enter);
    };

    fn no_mod(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
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
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)
        );
        assert_eq!(
            key!(alt - shift - c),
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::ALT | KeyModifiers::SHIFT)
        );
        assert_eq!(key!(shift - alt - '2'), key!(ALT - SHIFT - 2));
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
}

//! Crokey helps incorporate configurable keybindings in [crossterm](https://github.com/crossterm-rs/crossterm)
//! based terminal applications by providing functions
//! - parsing key combinations from strings
//! - describing key combinations in strings
//! - parsing key combinations at compile time
//!
//! ## Parsing a string
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
//! Complete example in `/examples/print_key`:
//!
//! ![print_key](https://raw.githubusercontent.com/Canop/crokey/main/doc/print_key.png)
//!
//! ## Displaying a string with a configurable format
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

pub use {
    crokey_proc_macros::*,
    format::*,
    parse::*,
};

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

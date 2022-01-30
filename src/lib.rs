//! Crokey helps incorporate configurable keybindings in [crossterm](https://github.com/crossterm-rs/crossterm)
//! based terminal applications by providing functions
//! - parsing key combinations from strings
//! - describing key combinations in strings

mod consts;
mod format;
mod parse;

pub use {
    consts::*,
    format::*,
    parse::*,
};

use {
    crossterm::event::{
        KeyCode,
        KeyEvent,
        KeyModifiers,
    },
};

/// return the raw char if the event is a letter event
pub const fn as_letter(key: KeyEvent) -> Option<char> {
    match key {
        KeyEvent { code: KeyCode::Char(l), modifiers: KeyModifiers::NONE } => Some(l),
        _ => None,
    }
}

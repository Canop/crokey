use {
    crossterm::event::{
        KeyCode::*,
        KeyEvent,
        KeyModifiers,
    },
};

#[macro_export]
macro_rules! const_key {
    ($name:ident, $code:expr) => {
        pub const $name: KeyEvent = KeyEvent {
            code: $code,
            modifiers: KeyModifiers::empty(),
        };
    };
    ($name:ident, $code:expr, $mod:expr) => {
        pub const $name: KeyEvent = KeyEvent {
            code: $code,
            modifiers: $mod,
        };
    };
}

// we define a few constants which make it easier to check key events
const_key!(ALT_ENTER, Enter, KeyModifiers::ALT);
const_key!(BACKSPACE, Backspace);
const_key!(BACK_TAB, BackTab, KeyModifiers::SHIFT); // backtab needs shift
const_key!(CTRL_ENTER, Enter, KeyModifiers::CONTROL);
const_key!(DELETE, Delete);
const_key!(DOWN, Down);
const_key!(END, End);
const_key!(ENTER, Enter);
const_key!(ESC, Esc);
const_key!(F1, F(1));
const_key!(F10, F(10));
const_key!(F11, F(11));
const_key!(F12, F(12));
const_key!(F2, F(2));
const_key!(F3, F(3));
const_key!(F4, F(4));
const_key!(F5, F(5));
const_key!(F6, F(6));
const_key!(F7, F(7));
const_key!(F8, F(8));
const_key!(F9, F(9));
const_key!(HOME, Home);
const_key!(INSERT, Insert);
const_key!(LEFT, Left);
const_key!(PAGE_DOWN, PageDown);
const_key!(PAGE_UP, PageUp);
const_key!(QUESTION, Char('?'));
const_key!(RIGHT, Right);
const_key!(SPACE, Char(' '));
const_key!(TAB, Tab);
const_key!(UP, Up);


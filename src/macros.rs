/// Check and expand at compile-time the provided expression
/// into a valid [`KeyEvent`].
///
///
/// For example:
/// ```
/// # use crokey::key;
/// let key_event = key!(ctrl-c);
/// ```
/// is expanded into something like:
///
/// ```
/// let key_event = crossterm::event::KeyEvent {
///     modifiers: crossterm::event::KeyModifiers::CONTROL,
///     code: crossterm::event::KeyCode::Char('\u{63}'),
/// };
/// ```
///
/// The input format is a sequence of hyphen-separated modifiers (`ctrl`, `alt` and `shift`)
/// followed by the key code (which is interpreted identically to [`key_codeǃ`]).
///
/// [`KeyEvent`]: crossterm::event::KeyEvent
#[macro_export]
macro_rules! key {
    ($($input:tt)*) => { $crate::__key_inner!(parse_modifier NONE NONE NONE $($input)*) };
}

// Unstable private API for use by `key!` only.
#[doc(hidden)]
#[macro_export]
macro_rules! __key_inner {
    (parse_modifier NONE $alt:tt $shift:tt ctrl $($rest:tt)*) => {
        $crate::__key_inner!(expect_dash CONTROL $alt $shift $($rest)*)
    };
    (parse_modifier CONTROL $alt:tt $shift:tt ctrl $($rest:tt)*) => {
        $crate::__private::core::compile_error!("ctrl specified twice")
    };
    (parse_modifier $ctrl:tt $alt:tt $shift:tt control $($rest:tt)*) => {
        $crate::__private::core::compile_error!("use `ctrl`, not `control`")
    };
    (parse_modifier $ctrl:tt NONE $shift:tt alt $($rest:tt)*) => {
        $crate::__key_inner!(expect_dash $ctrl ALT $shift $($rest)*)
    };
    (parse_modifier $ctrl:tt ALT $shift:tt alt $($rest:tt)*) => {
        $crate::__private::core::compile_error!("alt specified twice")
    };
    (parse_modifier $ctrl:tt $alt:tt NONE shift $($rest:tt)*) => {
        $crate::__key_inner!(expect_dash $ctrl $alt SHIFT $($rest)*)
    };
    (parse_modifier $ctrl:tt $alt:tt SHIFT shift $($rest:tt)*) => {
        $crate::__private::core::compile_error!("shift specified twice")
    };
    (parse_modifier $ctrl:tt $alt:tt $shift:tt $($code:tt)*) => {
        $crate::__private::crossterm::event::KeyEvent {
            code: $crate::key_code!($($code)*),
            modifiers: $crate::__private::crossterm::event::KeyModifiers::$ctrl
                | $crate::__private::crossterm::event::KeyModifiers::$alt
                | $crate::__private::crossterm::event::KeyModifiers::$shift,
        }
    };

    (expect_dash $ctrl:tt $alt:tt $shift:tt - $($rest:tt)*) => {
        $crate::__key_inner!(parse_modifier $ctrl $alt $shift $($rest)*)
    };
    // Split apart combined tokens like `-=` and `->` into their individual parts
    (expect_dash $ctrl:tt $alt:tt $shift:tt -= $($rest:tt)*) => {
        $crate::__key_inner!(parse_modifier $ctrl $alt $shift = $($rest)*)
    };
    (expect_dash $ctrl:tt $alt:tt $shift:tt -> $($rest:tt)*) => {
        $crate::__key_inner!(parse_modifier $ctrl $alt $shift > $($rest)*)
    };
    (expect_dash $ctrl:tt $alt:tt $shift:tt $($rest:tt)*) => {
        $crate::__private::core::compile_error!("expected hyphen after modifier")
    };
}

/// Check and expand at compile-time the provided expression into a valid [`KeyCode`].
///
/// For example:
///
/// ```
/// let code = crokey::key_code!(c);
/// ```
///
/// is expanded into something like:
///
/// ```
/// let code = crossterm::event::KeyCode::Char('c');
/// ```
///
/// This macro accepts:
/// - Special identifiers for special keys (e.g. `up`, `down`, `left` and `right` for arrow keys).
/// - Single-letter identifiers for `char`-key codes (e.g. `a`).
/// - Single digits for the number keys.
/// - Punctuation characters that Rust supports (e.g. `:` and `+`).
/// - Character literals for any other character (e.g. `']'` or `'('`).
///
/// [`KeyCode`]: crossterm::event::KeyCode
// rustfmt wants to place each macro arm on its own line, which is terrible for readability.
#[rustfmt::skip]
#[macro_export]
macro_rules! key_codeǃ {
    (backspace) => { $crate::__private::KeyCode::Backspace };
    (backtab) => { $crate::__private::KeyCode::BackTab };
    (del) => { $crate::__private::KeyCode::Delete };
    (delete) => { $crate::__private::KeyCode::Delete };
    (down) => { $crate::__private::KeyCode::Down };
    (end) => { $crate::__private::KeyCode::End };
    (enter) => { $crate::__private::KeyCode::Enter };
    (esc) => { $crate::__private::KeyCode::Esc };
    (f1) => { $crate::__private::KeyCode::F(1) };
    (f2) => { $crate::__private::KeyCode::F(2) };
    (f3) => { $crate::__private::KeyCode::F(3) };
    (f4) => { $crate::__private::KeyCode::F(4) };
    (f5) => { $crate::__private::KeyCode::F(5) };
    (f6) => { $crate::__private::KeyCode::F(6) };
    (f7) => { $crate::__private::KeyCode::F(7) };
    (f8) => { $crate::__private::KeyCode::F(8) };
    (f9) => { $crate::__private::KeyCode::F(9) };
    (f10) => { $crate::__private::KeyCode::F(10) };
    (f11) => { $crate::__private::KeyCode::F(11) };
    (f12) => { $crate::__private::KeyCode::F(12) };
    (home) => { $crate::__private::KeyCode::Home };
    (ins) => { $crate::__private::KeyCode::Insert };
    (insert) => { $crate::__private::KeyCode::Insert };
    (left) => { $crate::__private::KeyCode::Left };
    (pagedown) => { $crate::__private::KeyCode::PageDown };
    (pageup) => { $crate::__private::KeyCode::PageUp };
    (right) => { $crate::__private::KeyCode::Right };
    (space) => { $crate::__private::KeyCode::Char(' ') };
    (tab) => { $crate::__private::KeyCode::Tab };
    (up) => { $crate::__private::KeyCode::Up };
    (0) => { $crate::__private::KeyCode::Char('0') };
    (1) => { $crate::__private::KeyCode::Char('1') };
    (2) => { $crate::__private::KeyCode::Char('2') };
    (3) => { $crate::__private::KeyCode::Char('3') };
    (4) => { $crate::__private::KeyCode::Char('4') };
    (5) => { $crate::__private::KeyCode::Char('5') };
    (6) => { $crate::__private::KeyCode::Char('6') };
    (7) => { $crate::__private::KeyCode::Char('7') };
    (8) => { $crate::__private::KeyCode::Char('8') };
    (9) => { $crate::__private::KeyCode::Char('9') };
    // some character literals map better to non-`Char` `KeyCode`s
    ('\x08') => { $crate::__private::KeyCode::Backspace };
    ('\x7F') => { $crate::__private::KeyCode::Delete };
    ('\n') => { $crate::__private::KeyCode::Enter };
    ('\x1B') => { $crate::__private::KeyCode::Esc };
    ('\t') => { $crate::__private::KeyCode::Tab };
    // `-` is normally treated as a literal, which is incorrect
    (-) => { $crate::__private::KeyCode::Char('-') };
    // should be a character literal
    ($c:literal) => { $crate::__private::KeyCode::Char($c) };
    // punctuation or an identifier
    ($tt:tt) => {
        $crate::__private::KeyCode::Char($crate::__private::crokey_proc_macros::to_char!($tt))
    };
    ($($tt:tt)*) => {
        $crate::__private::core::compile_error!(concat!("unknown key code `", stringify!($($tt)*), "`"))
    }
}
// A hack to work around https://github.com/rust-lang/rust/issues/74087. The problem is that
// because the actual definition of key_code! uses `#[rustfmt::skip]` it can't be referred to by
// absolute paths, so instead we have to refer to a re-export of it. Usually we would name the real
// macro `__key_code` and the re-export `key_code`, but then either both or only `__key_code` would
// show up in documentation. So instead we name the real macro `key_codeǃ`, adding a LATIN LETTER
// RETROFLEX CLICK (not an exclamation mark as they aren't valid in identifiers) at the end so that
// it doesn't look weird in documentation. Only the most observant users will notice that rustdoc
// doesn't usually place exclamation marks at the end of macro names.
#[doc(hidden)]
pub use key_codeǃ as key_code;

// Unstable private API for use by `key!` and `key_code!` only.
#[doc(hidden)]
pub mod __private {
    pub use {core, crokey_proc_macros, crossterm, crossterm::event::KeyCode};
}

#[cfg(test)]
mod tests {
    use {
        crate::{key, key_code},
        crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    };

    #[test]
    fn key_code() {
        assert_eq!(key_code!(backspace), KeyCode::Backspace);
        assert_eq!(key_code!(5), KeyCode::Char('5'));
        assert_eq!(key_code!('\x08'), KeyCode::Backspace);
        assert_eq!(key_code!('\n'), KeyCode::Enter);
        assert_eq!(key_code!('x'), KeyCode::Char('x'));
        assert_eq!(key_code!(']'), KeyCode::Char(']'));
        assert_eq!(key_code!('ඞ'), KeyCode::Char('ඞ'));
        assert_eq!(key_code!(:), KeyCode::Char(':'));
        assert_eq!(key_code!(+), KeyCode::Char('+'));
        assert_eq!(key_code!(f), KeyCode::Char('f'));
        assert_eq!(key_code!(ඞ), KeyCode::Char('ඞ'));
    }

    #[test]
    fn key() {
        assert_eq!(
            key!(-),
            KeyEvent::new(KeyCode::Char('-'), KeyModifiers::NONE)
        );
        assert_eq!(
            key!(+),
            KeyEvent::new(KeyCode::Char('+'), KeyModifiers::NONE)
        );
        assert_eq!(
            key!(ctrl-=),
            KeyEvent::new(KeyCode::Char('='), KeyModifiers::CONTROL)
        );
        assert_eq!(
            key!(ctrl->),
            KeyEvent::new(KeyCode::Char('>'), KeyModifiers::CONTROL)
        );
        assert_eq!(
            key!(ctrl--),
            KeyEvent::new(KeyCode::Char('-'), KeyModifiers::CONTROL)
        );
        assert_eq!(
            key!(shift-=),
            KeyEvent::new(KeyCode::Char('='), KeyModifiers::SHIFT)
        );
        assert_eq!(
            key!(alt-shift-=),
            KeyEvent::new(KeyCode::Char('='), KeyModifiers::ALT | KeyModifiers::SHIFT)
        );
        assert_eq!(key!(shift - alt - 2), key!(alt - shift - 2));
        assert_eq!(
            key!( alt - shift - = ),
            KeyEvent::new(KeyCode::Char('='), KeyModifiers::ALT | KeyModifiers::SHIFT)
        );
    }
}

use {
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

/// Return the raw char if the crossterm key event is a letter event.
///
/// Case of the code is not normalized, just as in the original event.
pub const fn as_letter(key: KeyEvent) -> Option<char> {
    match key {
        KeyEvent {
            code: KeyCode::Char(l),
            modifiers: KeyModifiers::NONE,
            ..
        } => Some(l),
        _ => None,
    }
}

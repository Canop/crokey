//! Crokey helps incorporate configurable keybindings in [crossterm](https://github.com/crossterm-rs/crossterm)
//! based terminal applications by providing functions
//! - parsing key combinations from strings
//! - describing key combinations in strings

use {
    crossterm::event::{KeyCode::*, KeyEvent, KeyModifiers},
    std::fmt,
};

#[derive(Debug)]
pub struct ParseKeyError {
    /// the string which couldn't be parsed
    pub raw: String,
}

impl ParseKeyError {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self { raw: s.into() }
    }
}

impl fmt::Display for ParseKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} can't be parsed as a key", self.raw)
    }
}

impl std::error::Error for ParseKeyError {}

/// parse a string as a keyboard key combination definition.
///
/// About the case:
/// The char we receive as code from crossterm is usually lowercase
/// but uppercase when it was typed with shift (i.e. we receive
/// "g" for a lowercase, and "shift-G" for an uppercase)
pub fn parse(raw: &str) -> Result<KeyEvent, ParseKeyError> {
    let tokens: Vec<&str> = raw.split('-').collect();
    let last = tokens[tokens.len() - 1].to_ascii_lowercase();
    let mut code = match last.as_ref() {
        "esc" => Esc,
        "enter" => Enter,
        "left" => Left,
        "right" => Right,
        "up" => Up,
        "down" => Down,
        "home" => Home,
        "end" => End,
        "pageup" => PageUp,
        "pagedown" => PageDown,
        "backtab" => BackTab,
        "backspace" => Backspace,
        "del" => Delete,
        "delete" => Delete,
        "insert" => Insert,
        "ins" => Insert,
        "f1" => F(1),
        "f2" => F(2),
        "f3" => F(3),
        "f4" => F(4),
        "f5" => F(5),
        "f6" => F(6),
        "f7" => F(7),
        "f8" => F(8),
        "f9" => F(9),
        "f10" => F(10),
        "f11" => F(11),
        "f12" => F(12),
        "space" => Char(' '),
        "hyphen" => Char('-'),
        "minus" => Char('-'),
        "tab" => Tab,
        c if c.len() == 1 => Char(c.chars().next().unwrap()),
        _ => {
            return Err(ParseKeyError::new(raw));
        }
    };
    let mut modifiers = KeyModifiers::empty();
    if code == BackTab {
        // Crossterm always sends the shift key with
        // backtab
        modifiers.insert(KeyModifiers::SHIFT);
    }
    for token in tokens.iter().take(tokens.len() - 1) {
        match token.to_ascii_lowercase().as_ref() {
            "ctrl" => {
                modifiers.insert(KeyModifiers::CONTROL);
            }
            "alt" => {
                modifiers.insert(KeyModifiers::ALT);
            }
            "shift" => {
                modifiers.insert(KeyModifiers::SHIFT);
                if let Char(c) = code {
                    if c.is_ascii_lowercase() {
                        code = Char(c.to_ascii_uppercase());
                    }
                }
            }
            _ => {
                return Err(ParseKeyError::new(raw));
            }
        }
    }
    Ok(KeyEvent { code, modifiers })
}

#[test]
fn check_key_parsing() {
    use crate::*;
    fn check_ok(raw: &str, key: KeyEvent) {
        let parsed = parse(raw);
        assert!(parsed.is_ok(), "failed to parse {:?} as key", raw);
        assert_eq!(parsed.unwrap(), key);
    }
    check_ok("left", key!(left));
    check_ok("RIGHT", key!(right));
    check_ok("Home", key!(HOME));
    check_ok("f1", KeyEvent::from(F(1)));
    check_ok("F2", KeyEvent::from(F(2)));
    check_ok("Enter", KeyEvent::from(Enter));
    check_ok("alt-enter", KeyEvent::new(Enter, KeyModifiers::ALT));
    check_ok("insert", KeyEvent::from(Insert));
    check_ok("ctrl-q", KeyEvent::new(Char('q'), KeyModifiers::CONTROL));
    check_ok("shift-q", KeyEvent::new(Char('Q'), KeyModifiers::SHIFT));
    check_ok("ctrl-Q", KeyEvent::new(Char('q'), KeyModifiers::CONTROL));
    check_ok("shift-Q", KeyEvent::new(Char('Q'), KeyModifiers::SHIFT));
}

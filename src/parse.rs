//! Crokey helps incorporate configurable keybindings in [crossterm](https://github.com/crossterm-rs/crossterm)
//! based terminal applications by providing functions
//! - parsing key combinations from strings
//! - describing key combinations in strings

use {
    crate::{
        OneToThree,
        KeyCombination,
    },
    crossterm::event::{
        KeyCode::{self, *},
        KeyModifiers,
    },
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

pub fn parse_key_code(raw: &str, shift: bool) -> Result<KeyCode, ParseKeyError> {
    let code = match raw {
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
        "f13" => F(13),
        "f14" => F(14),
        "f15" => F(15),
        "f16" => F(16),
        "f17" => F(17),
        "f18" => F(18),
        "f19" => F(19),
        "f20" => F(20),
        "f21" => F(21),
        "f22" => F(22),
        "f23" => F(23),
        "f24" => F(24),
        "space" => Char(' '),
        "hyphen" => Char('-'),
        "minus" => Char('-'),
        "tab" => Tab,
        c if c.len() == 1 => {
            let mut c = c.chars().next().unwrap();
            if shift {
                c = c.to_ascii_uppercase();
            }
            Char(c)
        }
        _ => {
            return Err(ParseKeyError::new(raw));
        }
    };
    Ok(code)
}

/// parse a string as a keyboard key combination definition.
///
/// About the case:
/// The char we receive as code from crossterm is usually lowercase
/// but uppercase when it was typed with shift (i.e. we receive
/// "g" for a lowercase, and "shift-G" for an uppercase)
pub fn parse(raw: &str) -> Result<KeyCombination, ParseKeyError> {
    let mut modifiers = KeyModifiers::empty();
    let raw = raw.to_ascii_lowercase();
    let mut raw: &str = raw.as_ref();
    loop {
        if let Some(end) = raw.strip_prefix("ctrl-") {
            raw = end;
            modifiers.insert(KeyModifiers::CONTROL);
        } else if let Some(end) = raw.strip_prefix("alt-") {
            raw = end;
            modifiers.insert(KeyModifiers::ALT);
        } else if let Some(end) = raw.strip_prefix("shift-") {
            raw = end;
            modifiers.insert(KeyModifiers::SHIFT);
        } else {
            break;
        }
    }
    let codes = if raw == "-" {
        OneToThree::One(Char('-'))
    } else {
        let mut codes = Vec::new();
        let shift =  modifiers.contains(KeyModifiers::SHIFT);
        for raw in raw.split('-') {
            let code = parse_key_code(raw, shift)?;
            if code == BackTab {
                // Crossterm always sends SHIFT with backtab
                modifiers.insert(KeyModifiers::SHIFT);
            }
            codes.push(code);
        }
        codes.try_into().map_err(|_| ParseKeyError::new("".to_string()))?
    };
    Ok(KeyCombination::new(codes, modifiers))
}

#[test]
fn check_key_parsing() {
    use crate::*;
    fn check_ok(raw: &str, key: KeyCombination) {
        let parsed = parse(raw);
        assert!(parsed.is_ok(), "failed to parse {:?} as key combination", raw);
        assert_eq!(parsed.unwrap(), key);
    }
    assert!(parse("").is_err());
    check_ok("left", key!(left));
    check_ok("RIGHT", key!(right));
    check_ok("Home", key!(HOME));
    check_ok(
        "backtab",
        KeyCombination::new(KeyCode::BackTab, KeyModifiers::SHIFT),
    );
    check_ok("f1", KeyCombination::from(F(1)));
    check_ok("F2", KeyCombination::from(F(2)));
    check_ok("Enter", KeyCombination::from(Enter));
    check_ok("alt-enter", KeyCombination::new(Enter, KeyModifiers::ALT));
    check_ok("insert", KeyCombination::from(Insert));
    check_ok(
        "ctrl-q",
        KeyCombination::new(Char('q'), KeyModifiers::CONTROL),
    );
    check_ok(
        "shift-q",
        KeyCombination::new(Char('Q'), KeyModifiers::SHIFT),
    );
    check_ok(
        "ctrl-Q",
        KeyCombination::new(Char('q'), KeyModifiers::CONTROL),
    );
    check_ok(
        "shift-Q",
        KeyCombination::new(Char('Q'), KeyModifiers::SHIFT),
    );
    check_ok(
        "ctrl-shift-Q",
        KeyCombination::new(Char('Q'), KeyModifiers::SHIFT | KeyModifiers::CONTROL),
    );
    check_ok("-", KeyCombination::new(Char('-'), KeyModifiers::NONE));
    check_ok("Hyphen", KeyCombination::new(Char('-'), KeyModifiers::NONE));
    check_ok("alt--", KeyCombination::new(Char('-'), KeyModifiers::ALT));
    check_ok(
        "alt-hyphen",
        KeyCombination::new(Char('-'), KeyModifiers::ALT),
    );
    check_ok(
        "alt-hyphen",
        KeyCombination::new(Char('-'), KeyModifiers::ALT),
    );
    check_ok(
        "ctrl-Shift-alt-space",
        KeyCombination::new(
            Char(' '),
            KeyModifiers::ALT | KeyModifiers::SHIFT | KeyModifiers::ALT | KeyModifiers::CONTROL,
        ),
    );
    check_ok(
        "ctrl-shift-alt--",
        KeyCombination::new(
            Char('-'),
            KeyModifiers::ALT | KeyModifiers::SHIFT | KeyModifiers::ALT | KeyModifiers::CONTROL,
        ),
    );

    // multiple codes
    check_ok(
        "alt-f12-@",
        KeyCombination::new(
            OneToThree::Two(F(12), Char('@')),
            KeyModifiers::ALT,
        ),
    );
    check_ok(
        "alt-f12-@",
        KeyCombination::new(
            OneToThree::Two(Char('@'), F(12)), // it's the same because the codes are sorted
            KeyModifiers::ALT,
        ),
    );
    check_ok(
        "a-b",
        KeyCombination::new(
            OneToThree::Two(Char('a'), Char('b')),
            KeyModifiers::NONE,
        ),
    );
}

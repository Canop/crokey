use {
    super::*,
    crossterm::event::{KeyEvent, KeyEventKind, KeyEventState},
    serde::{de, Deserialize, Deserializer},
    std::{fmt, str::FromStr},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct KeyCombination {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl KeyCombination {
    /// Create a new KeyCombination from a KeyCode and a set of modifiers
    pub fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }
    /// Fix the case of the code to uppercase if the shift modifier is present.
    ///
    /// This allows direct comparisons with the fields of crossterm::event::KeyEvent
    /// whose code is uppercase when the shift modifier is present.
    pub fn normalize(mut self) -> Self {
        if self.modifiers.contains(KeyModifiers::SHIFT) {
            if let KeyCode::Char(c) = self.code {
                if c.is_ascii_lowercase() {
                    self.code = KeyCode::Char(c.to_ascii_uppercase());
                }
            }
        }
        self
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for KeyCombination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

impl FromStr for KeyCombination {
    type Err = ParseKeyError;
    fn from_str(s: &str) -> Result<Self, ParseKeyError> {
        parse(s)
    }
}

impl fmt::Display for KeyCombination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        STANDARD_FORMAT.format(*self).fmt(f)
    }
}

impl From<KeyEvent> for KeyCombination {
    fn from(key_event: KeyEvent) -> Self {
        let raw = Self {
            code: key_event.code,
            modifiers: key_event.modifiers,
        };
        raw.normalize()
    }
}

impl From<KeyCode> for KeyCombination {
    fn from(key_code: KeyCode) -> Self {
        Self {
            code: key_code,
            modifiers: KeyModifiers::empty(),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<KeyEvent> for KeyCombination {
    fn into(self) -> KeyEvent {
        let Self { code, modifiers } = self;
        KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press, // the only one in ANSI terminals
            state: KeyEventState::empty(),
        }
    }
}

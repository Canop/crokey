use {
    super::*,
    crossterm::event::{KeyEvent, KeyEventKind, KeyEventState},
    serde::{de, Deserialize, Deserializer},
    std::{fmt, str::FromStr},
    strict::OneToThree,
};

/// A Key combination wraps from one to three standard keys with optional modifiers
/// (ctrl, alt, shift).
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct KeyCombination {
    pub codes: OneToThree<KeyCode>,
    pub modifiers: KeyModifiers,
}

/// Change the char to uppercase when the modifier shift is present,
/// otherwise if the char is uppercase, return true
fn normalize_key_code(code: &mut KeyCode, modifiers: KeyModifiers) -> bool {
    if modifiers.contains(KeyModifiers::SHIFT) {
        if let KeyCode::Char(c) = code {
            if c.is_ascii_lowercase() {
                *code = KeyCode::Char(c.to_ascii_uppercase());
            }
        }
    } else if let KeyCode::Char(c) = code {
        if c.is_ascii_uppercase() {
            return true;
        }
    }
    false
}

impl KeyCombination {
    /// Create a new KeyCombination from one to three keycodes and a set of modifiers
    pub fn new<C: Into<OneToThree<KeyCode>>>(codes: C, modifiers: KeyModifiers) -> Self {
        let codes = codes.into().sorted();
        Self { codes, modifiers }
    }
    /// Create a new KeyCombination from one keycode and a set of modifiers
    pub const fn one_key(code: KeyCode, modifiers: KeyModifiers) -> Self {
        let codes = OneToThree::One(code);
        Self { codes, modifiers }
    }
    /// Ansi terminals don't manage key press/release/repeat, so they
    /// don't allow to determine whether 2 keys are pressed at the same
    /// time. This means a combination involving several key codes can't
    /// be distiguished from a sequences of combinations involving a single key code.
    /// For this reason, only combinations involving a single key code are
    /// considered "ansi compatible"
    pub const fn is_ansi_compatible(self) -> bool {
        matches!(self.codes, OneToThree::One(_))
    }
    /// Return a normailzed version of the combination.
    ///
    /// Fix the case of the code to uppercase if the shift modifier is present.
    /// Add the SHIFT modifier if one code is uppercase.
    ///
    /// This allows direct comparisons with the fields of crossterm::event::KeyEvent
    /// whose code is uppercase when the shift modifier is present. And supports the
    /// case where the modifier isn't mentionned but the key is uppercase.
    pub fn normalized(mut self) -> Self {
        let mut shift = normalize_key_code(self.codes.first_mut(), self.modifiers);
        if let Some(ref mut code) = self.codes.get_mut(1) {
            shift |= normalize_key_code(code, self.modifiers);
        }
        if let Some(ref mut code) = self.codes.get_mut(2) {
            shift |= normalize_key_code(code, self.modifiers);
        }
        if shift {
            self.modifiers |= KeyModifiers::SHIFT;
        }
        self
    }
    /// return the raw char if the combination is a letter event
    pub const fn as_letter(self) -> Option<char> {
        match self {
            Self {
                codes: OneToThree::One(KeyCode::Char(l)),
                modifiers: KeyModifiers::NONE,
            } => Some(l),
            _ => None,
        }
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
            codes: key_event.code.into(),
            modifiers: key_event.modifiers,
        };
        raw.normalized()
    }
}

impl TryFrom<&[KeyEvent]> for KeyCombination {
    type Error = &'static str;
    /// Try to create a KeyCombination from a slice of key events,
    /// will fail if and only if the slice is empty.
    fn try_from(key_events: &[KeyEvent]) -> Result<Self, Self::Error> {
        let mut modifiers = KeyModifiers::empty();
        let mut codes = Vec::new();
        for key_event in key_events {
            modifiers |= key_event.modifiers;
            codes.push(key_event.code);
        }
        let codes: OneToThree<KeyCode> = codes.try_into()?;
        let raw = Self::new(codes, modifiers);
        Ok(raw.normalized())
    }
}

impl From<KeyCode> for KeyCombination {
    fn from(key_code: KeyCode) -> Self {
        Self {
            codes: key_code.into(),
            modifiers: KeyModifiers::empty(),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<KeyEvent> for KeyCombination {
    fn into(self) -> KeyEvent {
        let Self { codes, modifiers } = self;
        KeyEvent {
            code: *codes.first(),
            modifiers,
            kind: KeyEventKind::Press, // the only one in ANSI terminals
            state: KeyEventState::empty(),
        }
    }
}

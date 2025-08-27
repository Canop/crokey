//! Crokey helps incorporate configurable keybindings in [crossterm](https://github.com/crossterm-rs/crossterm)
//! based terminal applications by providing functions
//! - parsing key combinations from strings
//! - describing key combinations in strings

use {
    crate::KeyCombination,
    crossterm::event::{KeyCode::*, KeyModifiers},
    std::fmt,
};

/// A formatter to produce key combinations descriptions.
///
/// ```
/// use {
///     crokey::*,
///     crossterm::event::{
///         KeyCode,
///         KeyEvent,
///         KeyModifiers,
///     },
/// };
///
/// let format = KeyCombinationFormat::default();
/// assert_eq!(format.to_string(key!(shift-a)), "Shift-a");
/// assert_eq!(format.to_string(key!(ctrl-c)), "Ctrl-c");
///
/// // A more compact format
/// let format = KeyCombinationFormat::default()
///     .with_implicit_shift()
///     .with_control("^");
/// assert_eq!(format.to_string(key!(shift-a)), "A");
/// assert_eq!(format.to_string(key!(ctrl-c)), "^c");
///
/// // A long format with lowercased modifiers
/// let format = KeyCombinationFormat::default()
///     .with_lowercase_modifiers();
/// assert_eq!(format.to_string(key!(ctrl-enter)), "ctrl-Enter");
/// assert_eq!(format.to_string(key!(home)), "Home");
/// assert_eq!(
///     format.to_string(
///         KeyCombination::new(
///             KeyCode::F(6),
///             KeyModifiers::ALT,
///         )
///     ),
///     "alt-F6",
/// );
/// assert_eq!(
///     format.to_string(
///         KeyCombination::new(
///             (KeyCode::Char('u'), KeyCode::Char('i')),
///             KeyModifiers::NONE,
///         )
///     ),
///     "i-u",
/// );
///
/// ```
#[derive(Debug, Clone)]
pub struct KeyCombinationFormat {
    pub control: String,
    pub command: String, // also called 'super', 'apple', 'windows'
    pub alt: String,
    pub shift: String,
    pub enter: String,
    pub uppercase_shift: bool,
    pub key_separator: String,
}

impl Default for KeyCombinationFormat {
    fn default() -> Self {
        Self {
            control: "Ctrl-".to_string(),
            command: "Cmd-".to_string(),
            alt: "Alt-".to_string(),
            shift: "Shift-".to_string(),
            enter: "Enter".to_string(),
            uppercase_shift: false,
            key_separator: "-".to_string(),
        }
    }
}

impl KeyCombinationFormat {
    pub fn with_lowercase_modifiers(mut self) -> Self {
        self.control = self.control.to_lowercase();
        self.alt = self.alt.to_lowercase();
        self.shift = self.shift.to_lowercase();
        self
    }
    pub fn with_control<S: Into<String>>(mut self, s: S) -> Self {
        self.control = s.into();
        self
    }
    pub fn with_command<S: Into<String>>(mut self, s: S) -> Self {
        self.command = s.into();
        self
    }
    pub fn with_alt<S: Into<String>>(mut self, s: S) -> Self {
        self.alt = s.into();
        self
    }
    pub fn with_shift<S: Into<String>>(mut self, s: S) -> Self {
        self.shift = s.into();
        self
    }
    pub fn with_implicit_shift(mut self) -> Self {
        self.shift = "".to_string();
        self.uppercase_shift = true;
        self
    }
    /// return a wrapper of the key implementing Display
    ///
    /// ```
    /// use crokey::*;
    /// let format = KeyCombinationFormat::default();
    /// let k = format.format(key!(f6));
    /// let s = format!("k={}", k);
    /// assert_eq!(s, "k=F6");
    /// ```
    pub fn format<K: Into<KeyCombination>>(&self, key: K) -> FormattedKeyCombination<'_> {
        FormattedKeyCombination { format: self, key: key.into() }
    }
    /// return the key formatted into a string
    ///
    /// `format.to_string(key)` is equivalent to `format.format(key).to_string()`.
    pub fn to_string<K: Into<KeyCombination>>(&self, key: K) -> String {
        self.format(key).to_string()
    }
}

pub struct FormattedKeyCombination<'s> {
    format: &'s KeyCombinationFormat,
    key: KeyCombination,
}

impl fmt::Display for FormattedKeyCombination<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let format = &self.format;
        let key = &self.key;
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            write!(f, "{}", format.control)?;
        }
        if key.modifiers.contains(KeyModifiers::ALT) {
            write!(f, "{}", format.alt)?;
        }
        if key.modifiers.contains(KeyModifiers::SHIFT) {
            write!(f, "{}", format.shift)?;
        }
        if key.modifiers.contains(KeyModifiers::SUPER) {
            write!(f, "{}", format.command)?;
        }
        for (i, code) in key.codes.iter().enumerate() {
            if i > 0 {
                write!(f, "{}", format.key_separator)?;
            }
            match code {
                Char(' ') => {
                    write!(f, "Space")?;
                }
                Char('-') => {
                    write!(f, "Hyphen")?;
                }
                Char('\r') | Char('\n') | Enter => {
                    write!(f, "{}", format.enter)?;
                }
                Char(c) if key.modifiers.contains(KeyModifiers::SHIFT) && format.uppercase_shift => {
                    write!(f, "{}", c.to_ascii_uppercase())?;
                }
                Char(c) => {
                    write!(f, "{}", c.to_ascii_lowercase())?;
                }
                F(u) => {
                    write!(f, "F{u}")?;
                }
                _ => {
                    write!(f, "{:?}", code)?;
                }
            }
        }
        Ok(())
    }
}

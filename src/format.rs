//! Crokey helps incorporate configurable keybindings in [crossterm](https://github.com/crossterm-rs/crossterm)
//! based terminal applications by providing functions
//! - parsing key combinations from strings
//! - describing key combinations in strings

use {
    crossterm::event::{KeyCode::*, KeyEvent, KeyModifiers},
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
/// let format = KeyEventFormat::default();
/// assert_eq!(format.to_string(key!(shift-a)), "Shift-a");
/// assert_eq!(format.to_string(key!(ctrl-c)), "Ctrl-c");
///
/// // A more compact format
/// let format = KeyEventFormat::default()
///     .with_implicit_shift()
///     .with_control("^");
/// assert_eq!(format.to_string(key!(shift-a)), "A");
/// assert_eq!(format.to_string(key!(ctrl-c)), "^c");
///
/// // A long format with lowercased modifiers
/// let format = KeyEventFormat::default()
///     .with_lowercase_modifiers();
/// assert_eq!(format.to_string(key!(ctrl-enter)), "ctrl-Enter");
/// assert_eq!(format.to_string(key!(home)), "Home");
/// assert_eq!(
///     format.to_string(
///         KeyEvent::new(
///             KeyCode::F(6),
///             KeyModifiers::ALT,
///         )
///     ),
///     "alt-F6",
/// );
///
/// ```
#[derive(Debug, Clone)]
pub struct KeyEventFormat {
    pub control: String,
    pub alt: String,
    pub shift: String,
    pub enter: String,
    pub uppercase_shift: bool,
}

impl Default for KeyEventFormat {
    fn default() -> Self {
        Self {
            control: "Ctrl-".to_string(),
            alt: "Alt-".to_string(),
            shift: "Shift-".to_string(),
            enter: "Enter".to_string(),
            uppercase_shift: false,
        }
    }
}

impl KeyEventFormat {
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
    /// let format = KeyEventFormat::default();
    /// let k = format.format(key!(f6));
    /// let s = format!("k={}", k);
    /// assert_eq!(s, "k=F6");
    /// ```
    pub fn format(&self, key: KeyEvent) -> FormattedKeyEvent {
        FormattedKeyEvent { format: self, key }
    }
    /// return the key formatted into a string
    ///
    /// `format.to_string(key)` is equivalent to `format.format(key).to_string()`.
    pub fn to_string(&self, key: KeyEvent) -> String {
        self.format(key).to_string()
    }
}

pub struct FormattedKeyEvent<'s> {
    format: &'s KeyEventFormat,
    key: KeyEvent,
}

impl<'s> fmt::Display for FormattedKeyEvent<'s> {
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
        match key.code {
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
                write!(f, "{:?}", key.code)?;
            }
        }
        Ok(())
    }
}

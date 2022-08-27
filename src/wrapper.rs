use {
    super::*,
    crossterm::event::KeyEvent,
    serde::{de, Deserialize, Deserializer},
    std::{fmt, str::FromStr},
};

/// A zero-cost wrapper type implementing Display and FromStr.
///
/// When the "serde" feature is enabled, it also implements
/// `Deserialize` which is handy to read whole configuration
/// files with Serde.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct CroKey(KeyEvent);

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for CroKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

impl FromStr for CroKey {
    type Err = ParseKeyError;
    fn from_str(s: &str) -> Result<Self, ParseKeyError> {
        parse(s).map(CroKey)
    }
}

impl fmt::Display for CroKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        STANDARD_FORMAT.format(self.0).fmt(f)
    }
}

impl From<KeyEvent> for CroKey {
    fn from(key_event: KeyEvent) -> Self {
        Self(key_event)
    }
}

#[allow(clippy::from_over_into)]
impl Into<KeyEvent> for CroKey {
    fn into(self) -> KeyEvent {
        self.0
    }
}

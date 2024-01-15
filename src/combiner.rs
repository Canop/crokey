use {
    crate::*,
    crossterm::{
        event::{
            KeyCode,
            KeyEvent,
            KeyboardEnhancementFlags,
            KeyEventKind,
            KeyModifiers,
            ModifierKeyCode,
            PopKeyboardEnhancementFlags,
            PushKeyboardEnhancementFlags,
        },
        execute,
        terminal,
    },
    std::{
        io,
        ops::Drop,
    },
};

/// This is the maximum number of keys we can combine
const MAX_PRESS_COUNT: usize = 3;

/// Consumes key events and combines them into key combinations.
///
/// See the print_key_events example.
#[derive(Debug, Default)]
pub struct Combiner {
    combining: bool,
    keyboard_enhancement_flags_pushed: bool,
    down_keys: Vec<KeyEvent>,
    shift_pressed: bool,
}

impl Combiner {

    /// Try to enable combining more than one non-modifier key into a combination.
    ///
    /// Return Ok(false) when the terminal doesn't support the kitty protocol.
    ///
    /// A downside of combining is that key combinations are produced on key release
    /// instead of key press, which may feel "slower".
    ///
    /// Behind the scene, this function pushes the keyboard enhancement flags
    /// to the terminal. The flags are popped, and the normal state of the terminal
    /// restored, when the Combiner is dropped.
    pub fn enable_combining(&mut self) -> io::Result<bool> {
        if self.combining {
            return Ok(true);
        }
        if self.keyboard_enhancement_flags_pushed {
            return Ok(self.combining);
        }
        if !terminal::supports_keyboard_enhancement()? {
            return Ok(false);
        }
        push_keyboard_enhancement_flags()?;
        self.keyboard_enhancement_flags_pushed = true;
        self.combining = true;
        Ok(true)
    }
    pub fn is_combining(&self) -> bool {
        self.combining
    }
    /// Take all the down_keys, combine them into a KeyCombination
    fn combine(&mut self) -> Option<KeyCombination> {
        let mut key_combination = KeyCombination::try_from(self.down_keys.as_slice())
            .ok(); // it may be empty, in which case we return None
        if self.shift_pressed {
            if let Some(ref mut key_combination) = key_combination {
                key_combination.modifiers |= KeyModifiers::SHIFT;
            }
            self.shift_pressed = false;
        }
        self.down_keys.clear();
        key_combination
    }
    /// Receive a key event and return a key combination if one is ready.
    ///
    /// When combining is enabled, the key combination is only returned on a
    /// key release event.
    pub fn transform(&mut self, key: KeyEvent) -> Option<KeyCombination> {
        if self.combining {
            self.transform_combining(key)
        } else {
            self.transform_ansi(key)
        }
    }
    fn transform_combining(&mut self, key: KeyEvent) -> Option<KeyCombination> {
        if let KeyCode::Modifier(modifier) = key.code {
            if key.kind == KeyEventKind::Press
                && (modifier == ModifierKeyCode::LeftShift || modifier == ModifierKeyCode::RightShift)
            {
                self.shift_pressed = true;
            }
            // we ignore modifier keys as independent events
            // (which means we never return a combination with only modifiers)
            return None;
        }
        match key.kind {
            KeyEventKind::Press => {
                self.down_keys.push(key);
                if self.down_keys.len() == MAX_PRESS_COUNT {
                    // As we can't handle more than 3 keys, we send them all
                    // as a combination.
                    // But it means some releases will have to be ignored.
                    self.combine()
                } else {
                    None
                }
            }
            KeyEventKind::Release => {
                // this release ends the combination in progress
                self.combine()
            }
            KeyEventKind::Repeat => {
                // we just ignore it
                None
            }
        }
    }
    /// In ansi mode, no combination is possible, and we don't expect to
    /// receive anything else than a single key or than key presses.
    fn transform_ansi(&mut self, key: KeyEvent) -> Option<KeyCombination> {
        match key.kind {
            KeyEventKind::Press => Some(key.into()),
            _ => {
                // this is unexpected, we don't seem to be really in ansi mode
                // but for consistency we must filter out this event
                None
            }
        }
    }
}

impl Drop for Combiner {
    fn drop(&mut self) {
        if self.keyboard_enhancement_flags_pushed {
            pop_keyboard_enhancement_flags();
        }
    }
}

fn push_keyboard_enhancement_flags() -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
        )
    )
}
fn pop_keyboard_enhancement_flags() {
    let mut stdout = io::stdout();
    let _ = execute!(stdout, PopKeyboardEnhancementFlags,);
}

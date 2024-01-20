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

/// This is the maximum number of keys we can combine.
/// It can't be changed just here, as the KeyCombination type doesn't support
/// more than 3 non-modifier keys
const MAX_PRESS_COUNT: usize = 3;

/// Consumes key events and combines them into key combinations.
///
/// See the print_key_events example.
#[derive(Debug)]
pub struct Combiner {
    combining: bool,
    keyboard_enhancement_flags_pushed: bool,
    mandate_modifier_for_multiple_keys: bool,
    down_keys: Vec<KeyEvent>,
    shift_pressed: bool,
}

impl Default for Combiner {
    fn default() -> Self {
        Self {
            combining: false,
            keyboard_enhancement_flags_pushed: false,
            mandate_modifier_for_multiple_keys: true,
            down_keys: Vec::new(),
            shift_pressed: false,
        }
    }
}

impl Combiner {
    /// Try to enable combining more than one non-modifier key into a combination.
    ///
    /// Return Ok(false) when the terminal doesn't support the kitty protocol.
    ///
    /// Behind the scene, this function pushes the keyboard enhancement flags
    /// to the terminal. The flags are popped, and the normal state of the terminal
    /// restored, when the Combiner is dropped.
    ///
    /// This function does nothing if combining is already enabled.
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
    /// Disable combining.
    pub fn disable_combining(&mut self) -> io::Result<()> {
        if self.keyboard_enhancement_flags_pushed {
            pop_keyboard_enhancement_flags()?;
            self.keyboard_enhancement_flags_pushed = false;
        }
        self.combining = false;
        Ok(())
    }
    pub fn is_combining(&self) -> bool {
        self.combining
    }
    /// When combining is enabled, you may either want "simple" keys
    /// (i.e. without modifier or space) to be handled on key press,
    /// or to wait for a key release so that maybe they may
    /// be part of a combination like 'a-b'.
    /// If combinations without modifier or space are unlikely in your application, you
    /// may make it feel snappier by setting this to true.
    ///
    /// This setting has no effect when combining isn't enabled.
    pub fn set_mandate_modifier_for_multiple_keys(&mut self, mandate: bool) {
        self.mandate_modifier_for_multiple_keys = mandate;
    }
    /// Take all the down_keys, combine them into a KeyCombination
    fn combine(&mut self, clear: bool) -> Option<KeyCombination> {
        let mut key_combination = KeyCombination::try_from(self.down_keys.as_slice())
            .ok(); // it may be empty, in which case we return None
        if self.shift_pressed {
            if let Some(ref mut key_combination) = key_combination {
                key_combination.modifiers |= KeyModifiers::SHIFT;
            }
        }
        if clear {
            self.down_keys.clear();
            self.shift_pressed = false;
        }
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
            if modifier == ModifierKeyCode::LeftShift || modifier == ModifierKeyCode::RightShift {
                self.shift_pressed = key.kind != KeyEventKind::Release;
            }
            // we ignore modifier keys as independent events
            // (which means we never return a combination with only modifiers)
            return None;
        }
        match key.kind {
            KeyEventKind::Press => {
                self.down_keys.push(key);
                if
                    (
                        self.mandate_modifier_for_multiple_keys
                        && is_key_simple(key)
                        && !self.shift_pressed
                        && self.down_keys.len() == 1
                    )
                    || self.down_keys.len() == MAX_PRESS_COUNT
                {
                    self.combine(true)
                } else {
                    None
                }
            }
            KeyEventKind::Release => {
                // this release ends the combination in progress
                self.combine(true)
            }
            KeyEventKind::Repeat => {
                self.combine(false)
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

/// For the purpose of key combination, we consider that a key is "simple"
/// when it's neither a modifier (ctrl,alt,shift) nor a space.
pub fn is_key_simple(key: KeyEvent) -> bool {
    key.modifiers.is_empty()
        && key.code != KeyCode::Char(' ')
}

impl Drop for Combiner {
    fn drop(&mut self) {
        if self.keyboard_enhancement_flags_pushed {
            let _ = pop_keyboard_enhancement_flags();
        }
    }
}

/// Change the state of the terminal to enable combining keys.
/// This is done automatically by Combiner::enable_combining
/// so you should usually not need to call this function.
pub fn push_keyboard_enhancement_flags() -> io::Result<()> {
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

/// Restore the "normal" state of the terminal.
/// This is done automatically by the combiner on drop,
/// so you should usually not need to call this function.
pub fn pop_keyboard_enhancement_flags() -> io::Result<()>{
    let mut stdout = io::stdout();
    execute!(stdout, PopKeyboardEnhancementFlags)
}

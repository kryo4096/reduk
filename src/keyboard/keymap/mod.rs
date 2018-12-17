pub mod de_ch;

use keyboard::Scancode;

#[derive(Clone, Copy)]
pub enum NonPrinting {
    None,
    Shift,
    Ctrl,
    Alt,
    CapsLock,
    ScrollLock,
    NumLock,
    Escape,
    Backspace,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

#[derive(Clone, Copy)]
pub enum Key {
    Printing(char),
    NonPrinting(NonPrinting),
}

pub struct KeyMap {
    map: &'static [Key],
}

impl KeyMap {
    pub const fn new(map: &'static [Key]) -> Self {
        Self { map }
    }

    pub fn get_key(&self, scancode: Scancode) -> Option<Key> {
        if (scancode.map_index() >= self.map.len()) {
            None
        } else {
            let key = self.map[scancode.map_index()];
            if let Key::NonPrinting(NonPrinting::None) = key {
                None
            } else {
                Some(key)
            }
        }
    }
}

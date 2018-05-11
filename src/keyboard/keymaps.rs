pub trait KeyMap {
    fn get_key_str(&self, scancode: u8, shift: bool, caps: bool) -> Option<&'static str>;
    fn get_key_sym(&self, scancode: u8, shift: bool, caps: bool) -> Option<&'static str> {
        let key_str = self.get_key_str(scancode, shift, caps)?;

        if key_str.len() == 1 {
            Some(key_str)
        } else {
            None
        }
    }
}

type KeyData = &'static [&'static str];

pub struct KeyMapLatin {
    lower: KeyData,
    upper: KeyData,
    caps: KeyData,
}

impl KeyMap for KeyMapLatin {
    fn get_key_str(&self, scancode: u8, shift: bool, caps: bool) -> Option<&'static str>{

        assert!(self.lower.len() == self.upper.len() && self.upper.len() == self.caps.len());
        let len = self.lower.len();
        let index = scancode as usize;
    
        if index >= len {
            return None
        }

        if caps {
            return Some(self.caps[index])
        } 
        
        if shift {
            return Some(self.upper[index])
        }

        Some(self.lower[index])
    }
}

const DE_CH : KeyMapLatin = KeyMapLatin {
    lower: deCH_LOWER,
    upper: deCH_UPPER,
    caps: deCH_CAPS,
};


const deCH_LOWER : KeyData = &[
    "error",
    "esc", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "'", "^", "backspace",
    "tab", "q", "w", "e", "r", "t", "z", "u", "i", "o", "p", "ü", "¨", "enter",
    "ctrl", "a", "s", "d", "f", "g", "h", "j", "k", "l", "ö", "ä", "§", "shift",
    "$", "y", "x", "c", "v", "b", "n", "m", ",", ".", "-", "shift", "none", 
    "alt", " ", "caps_lock",
    "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10", "f11", "f12",
    "num_lock", "scroll_lock", "7", "8", "9", "-", "4", "5", "6", "+", "1", "2",
    "3", "0", ".", "<",
];

const deCH_UPPER : KeyData = &[
    "error",
    "esc", "+", "\"", "*", "ç", "%", "&", "/", "(", ")", "=", "?", "`", "backspace",
    "tab", "Q", "W", "E", "R", "T", "Z", "U", "I", "O", "P", "è", "!", "enter",
    "ctrl", "A", "S", "D", "F", "G", "H", "J", "K", "L", "é", "à", "°", "shift",
    "£", "Y", "X", "C", "V", "B", "N", "M", ";", ":", "_", "shift", "none", 
    "alt", " ", "caps_lock",
    "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10", "f11", "f12",
    "num_lock", "scroll_lock", "7", "8", "9", "-", "4", "5", "6", "+", "1", "2",
    "3", "0", ".", ">",
];

const deCH_CAPS : KeyData = &[
    "error",
    "esc", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "'", "^", "backspace",
    "tab", "Q", "W", "E", "R", "T", "Z", "U", "I", "O", "P", "Ü", "!", "enter",
    "ctrl", "A", "S", "D", "F", "G", "H", "J", "K", "L", "Ö", "Ä", "§", "shift",
    "$", "Y", "X", "C", "V", "B", "N", "M", ",", ".", "-", "shift", "none", 
    "alt", " ", "caps_lock",
    "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10", "f11", "f12",
    "num_lock", "scroll_lock", "7", "8", "9", "-", "4", "5", "6", "+", "1", "2",
    "3", "0", ".", "<",
];


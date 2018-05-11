use x86_64::instructions::port::Port;

pub struct KeyboardRaw {
    kb_port: Port<u8>,
    status_port: Port<u8>,
}

impl KeyboardRaw {

    pub unsafe fn new(kb_port: u16, status_port: u16) -> Self {
        Self {
            kb_port: Port::new(kb_port),
            status_port: Port::new(status_port),
        }
    }

    pub fn poll(&mut self) -> Option<u8> {

        let sc;

        unsafe {
            if (self.status_port.read() & 1) != 1 {
                return None;
            }

            sc = self.kb_port.read();
        }

        Some(sc)
    }

    /*pub fn poll(&mut self) {

        if (self.status_port.read() & 1) != 1 {
            return;
        }

        let scancode = self.kb_port.read();

        let keymap = if self.caps_lock {
            keymaps::deCH_CAPS
        } else if self.shift {
            keymaps::deCH_UPPER
        } else {
            keymaps::deCH_LOWER
        };

        
        let key = match KeyEvent::from(scancode, keymap) {
            KeyEvent::Press(key) => {
                match key {

                    "shift" => {
                        self.shift = true;
                        None
                    },
                    "ctrl" => {
                        self.ctrl = true;
                        None
                    },
                    "alt" => {
                        self.alt = true;
                        None
                    },
                    "caps_lock" => {
                        self.caps_lock = !self.caps_lock;
                        None
                    },
                    "enter" => Some("\n"),
                    "none" => None,
                    "tab" => Some("   "),
                    "backspace" => None,
                    "scroll_lock" => None,
                    "num_lock" => None,
                    "error" => None,
                    _ => Some(key)
                }
            },
            KeyEvent::Release(key) => {
                match key {
                    "shift" => {
                        self.shift = false;
                        None
                    },
                    "ctrl" => {
                        self.ctrl = false;
                        None
                    },
                    "alt" => {
                        self.alt = false;
                        None
                    },
                    _ => None,
                }
            },
            KeyEvent::Invalid => {
                None
            }
        };

        if let Some(s) = key {
            self.text_queue.queue(s.chars().next().unwrap());
        }

    }*/
   

    pub fn wait_any(&mut self) {
        unsafe {
            while (self.status_port.read() & 1) != 1 {}
        }
    }
}

pub type Key = &'static str;
pub type KeyMap = [Key; 87];

pub enum KeyEvent {
    Press(Key),
    Release(Key),
    Invalid,
}

impl KeyEvent {

    pub fn from(scancode: u8, keymap: KeyMap) -> KeyEvent {

        if scancode as usize >= 128 + keymap.len() || (scancode as usize) < 128 && (scancode as usize) > keymap.len(){
            return KeyEvent::Invalid;
        };

        if scancode < 128 {
            return KeyEvent::Press(keymap[scancode as usize]);
        } else {
            return KeyEvent::Release(keymap[scancode as usize - 128]);
        }


    }
}


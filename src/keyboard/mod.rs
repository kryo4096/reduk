use spin::Mutex;
use fixed_queue::Queue;

mod keymaps;
mod raw;

const KEYBOARD_PORT : u16 = 0x60;
const STATUS_PORT : u16 = 0x64; 

lazy_static! {
    pub static ref KEYBOARD: Mutex<raw::KeyboardRaw> = Mutex::new(
        unsafe { 
            raw::KeyboardRaw::new(KEYBOARD_PORT, STATUS_PORT)
        }
    );
}

pub fn wait_any() {
    KEYBOARD.lock().wait_any();
}

struct KeyInput {
    shift: bool,
    caps: bool,
    ctrl: bool,
    alt: bool,
    keyboard: raw::KeyboardRaw,
    keymap: keymaps::KeyMap,
}

impl KeyInput {
    
    pub fn read_line() {
        kprintln!();
        
    }
}   










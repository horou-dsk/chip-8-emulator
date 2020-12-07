mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;
use chip_8_emulator::cpu::Cpu;
use std::sync::{Arc, RwLock};

#[macro_use]
extern crate lazy_static;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    // Your code goes here!
    console::log_1(&JsValue::from_str("wasm load success"));

    Ok(())
}

lazy_static! {
    pub static ref CPU: Arc<RwLock<Cpu>> = Arc::new(RwLock::new(Cpu::new_wasm()));
}

#[wasm_bindgen]
pub struct ByteStream {
    offset: *const u8,
    size: usize,
}

#[wasm_bindgen]
impl ByteStream {
    pub fn new(bytes: &[u8]) -> ByteStream {
        ByteStream {
            offset: bytes.as_ptr(),
            size: bytes.len(),
        }
    }

    pub fn offset(&self) -> *const u8 {
        self.offset
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[wasm_bindgen]
pub fn get_keys() -> ByteStream {
    let cpu = (*CPU).clone();
    let cpu = cpu.read().unwrap();
    ByteStream::new(&cpu.gfx)
}

#[wasm_bindgen]
pub fn set_rom(rom: Vec<u8>) {
    let cpu = (*CPU).clone();
    let mut cpu = cpu.write().unwrap();
    cpu.load_rom(rom);
    // (*CPU).borrow_mut().load_rom(rom);
}

#[wasm_bindgen]
pub fn step() {
    let cpu = (*CPU).clone();
    let mut cpu = cpu.write().unwrap();
    for _ in 0..2 {
        cpu.step();
    }
    // (*CPU).borrow_mut().step();
}
//
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

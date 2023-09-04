
use core::chip8::Chip8;

use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, KeyboardEvent, HtmlCanvasElement};


#[wasm_bindgen]
pub struct Emulator {
    chip8: Chip8,
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Emulator, JsValue> {
        let chip8 = Chip8::new();

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let ctx = canvas.get_context("2d")
            .unwrap().unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Ok(Emulator { chip8, ctx })
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.chip8.reset();
    }

    #[wasm_bindgen]
    pub fn emulate_cycle(&mut self) {
        self.chip8.emulate_cycle();
    }

    #[wasm_bindgen]
    pub fn tick_timers(&mut self) {
        self.chip8.tick_timers();
    }

    #[wasm_bindgen]
    pub fn keypress(&mut self, e: KeyboardEvent, pressed: bool) {
        if let Some(code) = key2btn(&e.key()) {
            self.chip8.keypress(code, pressed);
        }
    }

    #[wasm_bindgen]
    pub fn load_rom(&mut self, data: Uint8Array) {
        self.chip8.load(&data.to_vec());
    }

    #[wasm_bindgen]
     pub fn draw(&mut self, scale: usize) {
        let display = self.chip8.get_display();
        for (idx, on) in display.screen.iter().enumerate() {
            if *on {
                let (x, y) = display.to_xy(idx);
                self.ctx.fill_rect(
                    (x * scale) as f64,
                    (y * scale) as f64,
                    scale as f64,
                    scale as f64
                );
            }
        }
    }
}

/*
    Keyboard                    Chip-8
    +---+---+---+---+           +---+---+---+---+
    | 1 | 2 | 3 | 4 |           | 1 | 2 | 3 | C |
    +---+---+---+---+           +---+---+---+---+
    | Q | W | E | R |           | 4 | 5 | 6 | D |
    +---+---+---+---+     =>    +---+---+---+---+
    | A | S | D | F |           | 7 | 8 | 9 | E |
    +---+---+---+---+           +---+---+---+---+
    | Z | X | C | V |           | A | 0 | B | F |
    +---+---+---+---+           +---+---+---+---+
*/
fn key2btn(key: &str) -> Option<u8> {
    match key {
        "1" => Some(0x1),
        "2" => Some(0x2),
        "3" => Some(0x3),
        "4" => Some(0xC),
        "q" => Some(0x4),
        "w" => Some(0x5),
        "e" => Some(0x6),
        "r" => Some(0xD),
        "a" => Some(0x7),
        "s" => Some(0x8),
        "d" => Some(0x9),
        "f" => Some(0xE),
        "z" => Some(0xA),
        "x" => Some(0x0),
        "c" => Some(0xB),
        "v" => Some(0xF),
        _ =>   None,
    }
}


use wasm_bindgen::prelude::*;
use runtime::Game;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn game() {
    let game = Game::new();
    alert(&format!("hoge"));
}
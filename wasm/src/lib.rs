use wasm_bindgen::prelude::*;
use runtime::Game;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn game() {
    let game = Game::new();
}
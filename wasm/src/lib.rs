use engine::{
    board::Direction,
    position::{Column, Position, Row},
    Action, Game,
};
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
struct Battle {
    game: Game,
    history: Vec<Game>,
}

#[wasm_bindgen]
impl Battle {
    pub fn new() -> Self {
        let game = Game::new();
        let history = vec![game.clone()];
        Self { game, history }
    }

    pub fn display_board(&self) -> String {
        let board = self.game.board.clone();
        board
            .iterate()
            .map(|pos| {
                let cell = board.cell_map.get(&pos).unwrap();
                if let Some(player) = cell.owner() {
                    player.id.to_string()
                } else {
                    " ".to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("|")
    }
}

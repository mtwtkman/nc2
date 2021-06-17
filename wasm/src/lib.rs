use wasm_bindgen::prelude::*;
use runtime::{
    Game,
    Action,
    position::{Column, Position, Row},
    board::Direction,
};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
struct Battle {
    game: Game,
}

#[wasm_bindgen]
impl Battle {
    pub fn turn(&mut self) {
        if self.game.is_over() {
            return
        }
        let (x, y, dir): (usize, usize, usize) = (1, 1, 4);
        if let Ok(game) = self.game.accept(&self.act(x, y, dir)) {
            self.game = game;
            alert(&format!("{:?}", &self.game.board));
            return
        }
    }

    fn act(&self, x: usize, y: usize, direction: usize) -> Action {
        let player = self.game.current_player();
        let col = match x % 5 {
            1 => Column::MiddleFirst,
            2 => Column::MiddleSecond,
            3 => Column::MiddleThird,
            4 => Column::RightEdge,
            _ => Column::LeftEdge,
        };
        let row = match y % 6 {
            1 => Row::MiddleFirst,
            2 => Row::MiddleSecond,
            3 => Row::MiddleThird,
            4 => Row::MiddleFourth,
            5 => Row::Bottom,
            _ => Row::Top,
        };
        let dir = match direction % 8 {
            1 => Direction::UpRight,
            2 => Direction::Right,
            3 => Direction::DownRight,
            4 => Direction::Down,
            5 => Direction::DownLeft,
            6 => Direction::Left,
            7 => Direction::UpLeft,
            _ => Direction::Up,
        };
        let position = Position::new(col, row);
        Action::new(position, dir)
    }

    pub fn new() -> Self {
        Self { game: Game::new() }
    }
}
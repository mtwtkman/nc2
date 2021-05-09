mod board;
mod cell;
mod player;
mod position;
mod result;

use board::{Board, CellMap};
use cell::Cell;
use player::Player;
use result::Result;

struct Phase {
    player: Player,
    cell_map: CellMap,
}

impl Phase {
    fn is_win(&self) -> bool {
        unimplemented!()
    }
}

pub struct Game {
    player_a: Player,
    player_b: Player,
    board: Board,
    current_phase: Phase,
}

impl Game {
    fn new() -> Self {
        let (player_a, player_b) = Self::spawn_players();
        let board = Board::new(&player_a, &player_b);
        let phase = Phase {
            player: player_a,
            cell_map: board.territory(&player_a),
        };
        Self {
            player_a: player_a.clone(),
            player_b,
            board,
            current_phase: phase,
        }
    }

    fn spawn_players() -> (Player, Player) {
        (Player::new(), Player::new())
    }

    fn act(&self, cell: &Cell) -> Result<Self> {
        let (board, phase) = self.next_turn(cell)?;
        Ok(Self {
            player_a: self.player_a.clone(),
            player_b: self.player_b.clone(),
            board,
            current_phase: phase,
        })
    }

    fn next_turn(&self, cell: &Cell) -> Result<(Board, Phase)> {
        unimplemented!()
    }
}

#[test]
fn play_game() {
    let game = Game::new();
}

mod board;
mod cell;
mod player;
mod position;
mod result;

use board::Board;
use player::Player;

pub struct Game {
    player_a: Player,
    player_b: Player,
    current_player: Player,
    board: Board,
}
impl Game {
    fn start() -> Self {
        let (player_a, player_b) = Self::spawn_players();
        let board = Board::new(&player_a,&player_b);
        Self {
            player_a: player_a.clone(),
            player_b,
            current_player: player_a,
            board,
        }
    }

    fn spawn_players() -> (Player, Player) {
        (Player::new(), Player::new())
    }
}
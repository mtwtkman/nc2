mod board;
mod cell;
mod player;
mod position;
mod result;

use board::{Board, CellMap};
use cell::Cell;
use player::Player;
use position::Row;
use result::Result;

struct Phase {
    player: Player,
    cell_map: CellMap,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    DownRight,
    UpLeft,
    DownLeft,
}

struct Action {
    from: Cell,
    direction: Direction,
}

impl Action {
    fn new(departure_cell: Cell, direction: Direction) -> Self {
        Self {
            from: departure_cell,
            direction,
        }
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

    fn goal_side(&self) -> Row {
        if &self.current_phase.player == &self.player_a {
            Row::Bottom
        } else {
            Row::Top
        }
    }

    fn winner(&self) -> Option<Player> {
        let goal_side = self.goal_side();
        if self
            .current_phase
            .cell_map
            .keys()
            .find(|cell| match &goal_side {
                Row::Top => cell.position.is_top(),
                _ => cell.position.is_bottom(),
            })
            .is_some()
        {
            Some(self.current_phase.player.clone())
        } else {
            None
        }
    }

    fn is_over(&self) -> bool {
        self.winner().is_some()
    }

    fn spawn_players() -> (Player, Player) {
        (Player::new(), Player::new())
    }

    fn act(&self, action: Action) -> Result<Self> {
        let (board, phase) = self.next_turn(action)?;
        Ok(Self {
            player_a: self.player_a.clone(),
            player_b: self.player_b.clone(),
            board,
            current_phase: phase,
        })
    }

    fn next_turn(&self, action: Action) -> Result<(Board, Phase)> {
        unimplemented!()
    }
}

//#[test]
fn play_game() {
    use crate::position::{Column, Position, Row};
    let game = Game::new();
    assert_eq!(&game.current_phase.player, &game.player_a);
    let move_from = Cell::new_occupied(
        Position::new(Column::LeftEdge, Row::Top),
        game.current_phase.player.clone(),
    );
    let first_action = Action::new(move_from, Direction::Down);
    let result = game.act(first_action);
    assert!(result.is_ok());
}

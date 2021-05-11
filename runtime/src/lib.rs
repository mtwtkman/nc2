mod board;
mod cell;
mod player;
mod position;
mod result;

use board::{Board, CellMap, Direction};
use cell::Cell;
use player::Player;
use position::Row;
use result::{Error, Result};

struct Phase {
    player: Player,
    cell_map: CellMap,
}

impl Phase {
    fn won(&self, goal_side: &Row) -> bool {
        self.cell_map
            .keys()
            .find(|cell| match goal_side {
                Row::Top => cell.position.is_top(),
                _ => cell.position.is_bottom(),
            })
            .is_some()
    }
}

struct Action {
    from: Cell,
    to_direction: Direction,
}

impl Action {
    fn new(from: Cell, direction: Direction) -> Self {
        Self {
            from,
            to_direction: direction,
        }
    }
}

pub struct Game {
    player_a: Player,
    player_b: Player,
    board: Board,
    current_phase: Phase,
    winner: Option<Player>,
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
            winner: None,
        }
    }

    fn goal_side(&self) -> Row {
        if &self.current_phase.player == &self.player_a {
            Row::Bottom
        } else {
            Row::Top
        }
    }

    fn is_over(&self) -> bool {
        self.winner.is_some()
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
            winner: self.winner.clone(),
        })
    }

    fn next_player(&self) -> Player {
        if self.current_phase.player == self.player_a {
            self.player_b.clone()
        } else {
            self.player_a.clone()
        }
    }

    fn next_turn(&self, action: Action) -> Result<(Board, Phase)> {
        let moving_range = self
            .board
            .cell_map
            .get(&action.from)
            .ok_or(Error::InvalidDirection)?;
        let destination = moving_range.indicate(&action.to_direction)?;
        unimplemented!()
    }
}

#[cfg(test)]
mod phase_spec {
    use super::Phase;
    use crate::{
        board::{CellMap, MovingRange},
        cell::Cell,
        player::Player,
        position::{Column, Position, Row},
    };

    #[test]
    fn won() {
        let player = Player::new();
        for goal_side in [Row::Top, Row::Bottom].iter() {
            let mut cell_map: CellMap = CellMap::new();
            cell_map.insert(
                Cell::new_occupied(
                    Position::new(Column::LeftEdge, goal_side.clone()),
                    player.clone(),
                ),
                MovingRange::default(),
            );
            let phase = Phase { player, cell_map };
            assert!(phase.won(goal_side));
        }
    }

    #[test]
    fn not_won() {
        let player = Player::new();
        for goal_side in [Row::Top, Row::Bottom].iter() {
            let mut cell_map = CellMap::new();
            cell_map.insert(
                Cell::new_occupied(
                    Position::new(Column::LeftEdge, Row::MiddleFirst),
                    player.clone(),
                ),
                MovingRange::default(),
            );
            let phase = Phase { player, cell_map };
            assert!(!phase.won(goal_side));
        }
    }
}

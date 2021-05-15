mod board;
mod cell;
mod player;
mod position;
mod result;

use board::{Board, CellMap, Direction};
use cell::{Cell, MigratedCellPair};
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
    direction: Direction,
}

impl Action {
    fn new(from: Cell, direction: Direction) -> Self {
        Self { from, direction }
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

    fn current_player(&self) -> Player {
        self.current_phase.player.clone()
    }

    fn is_over(&self) -> bool {
        self.winner.is_some()
    }

    fn spawn_players() -> (Player, Player) {
        (Player::new(), Player::new())
    }

    fn accept(&self, action: Action) -> Result<Self> {
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
        let destination = moving_range.indicate(&action.direction)?;
        let migration_pair = action.from.migrate(&destination)?;
        let board = self.remap(&migration_pair);
        let phase = Phase {
            player: self.next_player(),
            cell_map: board.cell_map.clone(),
        };
        Ok((board, phase))
    }

    fn remap(&self, migration_pair: &MigratedCellPair) -> Board {
        let mut cell_map = self.board.cell_map.clone();
        unimplemented!()
    }
}

#[cfg(test)]
mod phase_spec {
    use super::Phase;
    use crate::{
        board::{CellMap, DestinationState, MovingRange},
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
                MovingRange {
                    up: DestinationState::OutOfField,
                    down: DestinationState::OutOfField,
                    right: DestinationState::OutOfField,
                    left: DestinationState::OutOfField,
                    up_right: DestinationState::OutOfField,
                    down_right: DestinationState::OutOfField,
                    up_left: DestinationState::OutOfField,
                    down_left: DestinationState::OutOfField,
                },
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
                MovingRange {
                    up: DestinationState::OutOfField,
                    down: DestinationState::OutOfField,
                    right: DestinationState::OutOfField,
                    left: DestinationState::OutOfField,
                    up_right: DestinationState::OutOfField,
                    down_right: DestinationState::OutOfField,
                    up_left: DestinationState::OutOfField,
                    down_left: DestinationState::OutOfField,
                },
            );
            let phase = Phase { player, cell_map };
            assert!(!phase.won(goal_side));
        }
    }
}

#[cfg(test)]
mod game_spec {
    use super::{Action, Direction, Game};
    use crate::cell::Cell;

    #[test]
    fn initial_state() {
        let game = Game::new();
        let initial_phase = &game.current_phase;
        assert_eq!(initial_phase.player, game.player_a);
        assert!(!game.is_over());
        assert_eq!(
            &initial_phase.cell_map,
            &game.board.territory(&game.player_a)
        );
    }

    #[test]
    fn flip_turn() {
        let game = Game::new();
        let current_phase = game.current_phase;
    }
}

mod board;
mod cell;
mod player;
mod position;
mod result;

use board::{Board, CellMap, Direction};
use player::Player;
use position::{Position, Row};
use result::Result;

struct Phase {
    player: Player,
    cell_map: CellMap,
}

impl Phase {
    fn won(&self, goal_side: &Row) -> bool {
        self.cell_map
            .keys()
            .find(|position| match goal_side {
                Row::Top => position.is_top(),
                _ => position.is_bottom(),
            })
            .is_some()
    }
}

struct Action {
    from: Position,
    direction: Direction,
}

impl Action {
    fn new(from: Position, direction: Direction) -> Self {
        Self { from, direction }
    }

    fn destination(&self) -> Result<Position> {
        self.direction.destination(&self.from)
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
        let moving_range = self.board.moving_range_of(&action.from)?;
        let destination = moving_range.indicate(&action.direction)?;
        let departure_cell = self.board.cell_of(&action.from)?;
        let migrated_board = self.board.migrate(&action.from, &destination.position)?;
        let phase = Phase {
            player: self.next_player(),
            cell_map: migrated_board.cell_map.clone(),
        };
        Ok((migrated_board, phase))
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
            let position = Position::new(Column::LeftEdge, goal_side.clone());
            let cell = Cell::new_occupied(player.clone());
            cell_map.insert(position.clone(), cell.clone());
            let phase = Phase { player, cell_map };
            assert!(phase.won(goal_side));
        }
    }

    #[test]
    fn not_won() {
        let player = Player::new();
        for goal_side in [Row::Top, Row::Bottom].iter() {
            let mut cell_map = CellMap::new();
            let position = Position::new(Column::LeftEdge, Row::MiddleFirst);
            let cell = Cell::new_occupied(player.clone());
            cell_map.insert(position.clone(), cell.clone());
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

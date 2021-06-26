pub mod board;
mod cell;
pub mod player;
pub mod position;
mod result;

use board::{Board, CellMap, Direction};
use player::Player;
use position::{Position, Row};
use result::{Error, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Phase {
    player: Player,
    cell_map: CellMap,
}

#[derive(Debug)]
pub struct Action {
    from: Position,
    direction: Direction,
}

impl Action {
    pub fn new(from: Position, direction: Direction) -> Self {
        Self { from, direction }
    }

    fn destination(&self) -> Result<Position> {
        self.direction.destination(&self.from)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    player_a: Player,
    player_b: Player,
    pub board: Board,
    current_phase: Phase,
    winner: Option<Player>,
}

impl Game {
    pub fn new() -> Self {
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

    pub fn current_player(&self) -> Player {
        self.current_phase.player.clone()
    }

    pub fn is_over(&self) -> bool {
        self.winner.is_some()
    }

    fn spawn_players() -> (Player, Player) {
        (Player::new(0), Player::new(1))
    }

    pub fn accept(&self, action: &Action) -> Result<Self> {
        if self.is_over() {
            return Err(Error::GameIsOver);
        }
        let board = self.refresh_board(&action.from, &action.direction)?;
        let destination = action.destination()?;
        let is_isolated = board.is_isolated(&destination);
        let is_reached_goal_side =
            board.is_reached_edge(&self.current_phase.player, &self.goal_side());
        let winner = if is_reached_goal_side && is_isolated {
            Some(self.current_phase.player)
        } else {
            None
        };
        let next_player = self.next_player();
        let next_phase = Phase {
            player: next_player,
            cell_map: board.territory(&next_player),
        };
        Ok(Self {
            player_a: self.player_a.clone(),
            player_b: self.player_b.clone(),
            board,
            current_phase: next_phase,
            winner: winner,
        })
    }

    fn next_player(&self) -> Player {
        if self.current_phase.player == self.player_a {
            self.player_b.clone()
        } else {
            self.player_a.clone()
        }
    }

    fn refresh_board(&self, position: &Position, direction: &Direction) -> Result<Board> {
        let moving_range = self.board.moving_range_of(&position)?;
        let destination = moving_range.indicate(&direction)?;
        self.board.migrate(&position, &destination.position)
    }
}

#[cfg(test)]
mod game_spec {
    use super::{Action, Game};
    use crate::{
        board::Direction,
        cell::Cell,
        position::{Column, Position, Row},
        result::Error,
    };

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
    fn refresh_board() {
        let game = Game::new();
        let from_position = Position::new(Column::LeftEdge, Row::Top);
        let direction = Direction::Down;
        if let Ok(board) = game.refresh_board(&from_position, &direction) {
            assert!(board.cell_map.get(&from_position).unwrap().is_empty());
            let to_position = from_position.below().unwrap();
            assert_eq!(
                board.cell_map.get(&to_position),
                Some(&Cell::new_occupied(game.player_a.clone()))
            );
        } else {
            panic!("fail");
        }
    }

    #[test]
    fn flip_turn() {
        let mut game = Game::new();
        let turns = [
            Action::new(Position::new(Column::LeftEdge, Row::Top), Direction::Down),
            Action::new(Position::new(Column::LeftEdge, Row::Bottom), Direction::Up),
            Action::new(
                Position::new(Column::LeftEdge, Row::MiddleFirst),
                Direction::Down,
            ),
            Action::new(
                Position::new(Column::LeftEdge, Row::MiddleFourth),
                Direction::Right,
            ),
            Action::new(
                Position::new(Column::LeftEdge, Row::MiddleSecond),
                Direction::Down,
            ),
            Action::new(
                Position::new(Column::MiddleFirst, Row::MiddleFourth),
                Direction::Right,
            ),
            Action::new(
                Position::new(Column::LeftEdge, Row::MiddleThird),
                Direction::Down,
            ),
            Action::new(
                Position::new(Column::MiddleFirst, Row::Bottom),
                Direction::Up,
            ),
            Action::new(
                Position::new(Column::LeftEdge, Row::MiddleFourth),
                Direction::Down,
            ),
            Action::new(
                Position::new(Column::MiddleFirst, Row::MiddleFourth),
                Direction::Up,
            ),
            Action::new(
                Position::new(Column::MiddleFirst, Row::Top),
                Direction::Down,
            ),
        ];
        turns.iter().for_each(|action| {
            let result = game.accept(action);
            assert!(!game.is_over());
            game = result.unwrap();
        });
        assert_eq!(game.winner, Some(game.player_a));
        assert!(game.is_over());
        assert_eq!(
            game.accept(&Action::new(
                Position::new(Column::MiddleFirst, Row::MiddleThird),
                Direction::Up,
            )),
            Err(Error::GameIsOver),
        );
    }

    #[test]
    fn error_by_out_of_world() {
        let game = Game::new();
        let from_position = Position::new(Column::LeftEdge, Row::Top);
        let direction = Direction::Up;
        assert_eq!(
            game.refresh_board(&from_position, &direction),
            Err(Error::IllegalDestination),
        );
    }
}

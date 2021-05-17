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

pub struct Action {
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

    fn is_correnct_player(&self, action: &Action) -> bool {
        self.board
            .is_occupied_by(&action.from, &self.current_phase.player)
    }

    pub fn accept(&self, action: Action) -> Result<Self> {
        let board = self.refresh_board(&action.from, &action.direction)?;
        let phase = Phase {
            player: self.next_player(),
            cell_map: board.cell_map.clone(),
        };
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

    fn refresh_board(&self, position: &Position, direction: &Direction) -> Result<Board> {
        let moving_range = self.board.moving_range_of(&position)?;
        let destination = moving_range.indicate(&direction)?;
        self.board.migrate(&position, &destination.position)
    }
}

#[cfg(test)]
mod phase_spec {
    use super::Phase;
    use crate::{
        board::CellMap,
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
    use super::Game;
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
    fn moved() {
        let game = Game::new();
        let from_position = Position::new(Column::LeftEdge, Row::Top);
        let direction = Direction::Down;
        if let Ok(board) = game.refresh_board(&from_position, &direction) {
            assert!(board.cell_map.get(&from_position).unwrap().is_empty());
            let to_position = from_position.below().unwrap();
            assert_eq!(board.cell_map.get(&to_position), Some(&Cell::new_occupied(game.player_a.clone())));
        } else {
            panic!("fail");
        }
    }

    #[test]
    fn error_by_out_of_world() {
        let game = Game::new();
        let from_position = Position::new(Column::LeftEdge, Row::Top);
        let direction = Direction::Up;
        assert_eq!(
            game.refresh_board(&from_position, &direction),
            Err(Error::IllegalDestination),
        )
    }
}

use std::collections::{BTreeSet, HashMap};
use std::iter::FromIterator;

use crate::{
    cell::{Cell, MigratedCellPair},
    player::Player,
    position::{Column, Position, Row},
    result::{Error, Result},
};

type Field = HashMap<Position, Cell>;
pub(crate) type CellMap = HashMap<Position, MovingRange>;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Board {
    pub(crate) cell_map: CellMap,
}
impl Board {
    pub(crate) fn new(player_a: &Player, player_b: &Player) -> Self {
        let field = Self::build_initial_field(player_a, player_b);
        let cell_map = Self::setup_cell_map(&field);
        Self { cell_map }
    }

    fn position_of(&self, cell: &Cell) -> Position {
        self.cell_map
            .iter()
            .fold(
                HashMap::<Cell, Position>::new(),
                |mut acc, (position, moving_range)| {
                    acc.insert(moving_range.pivot.clone(), position.clone());
                    acc
                },
            )
            .get(cell)
            .unwrap()
            .to_owned()
    }

    fn build_initial_field(player_a: &Player, player_b: &Player) -> Field {
        let mut field = Field::new();
        let player_a_side_cells = Self::generate_initial_occupied_cells(player_a.clone(), Row::Top);
        player_a_side_cells.for_each(|(position, cell)| {
            field.insert(position, cell);
        });
        let empty_rows = [
            Row::MiddleFirst,
            Row::MiddleSecond,
            Row::MiddleThird,
            Row::MiddleFourth,
        ]
        .iter()
        .flat_map(|row| Self::generate_initial_empty_cells(row.to_owned()));
        empty_rows.for_each(|(position, cell)| {
            field.insert(position, cell);
        });
        let player_b_side_cells =
            Self::generate_initial_occupied_cells(player_b.clone(), Row::Bottom);
        player_b_side_cells.for_each(|(position, cell)| {
            field.insert(position, cell);
        });
        field
    }

    fn setup_cell_map(field: &Field) -> CellMap {
        field
            .iter()
            .fold(CellMap::new(), |mut acc, (position, cell)| {
                acc.insert(position.clone(), MovingRange::new(position, cell, &field));
                acc
            })
    }

    fn generate_initial_occupied_cells(
        player: Player,
        side: Row,
    ) -> impl Iterator<Item = (Position, Cell)> {
        [
            Column::LeftEdge,
            Column::MiddleFirst,
            Column::MiddleSecond,
            Column::MiddleThird,
            Column::RightEdge,
        ]
        .iter()
        .map(move |column| {
            let position = Position::new(column.to_owned(), side.clone());
            let cell = Cell::new_occupied(player.clone());
            (position, cell)
        })
    }

    fn generate_initial_empty_cells(row: Row) -> impl Iterator<Item = (Position, Cell)> {
        [
            Column::LeftEdge,
            Column::MiddleFirst,
            Column::MiddleSecond,
            Column::MiddleThird,
            Column::RightEdge,
        ]
        .iter()
        .map(move |column| {
            let position = Position::new(column.to_owned(), row.clone());
            let cell = Cell::new_empty();
            (position, cell)
        })
    }

    pub(crate) fn territory(&self, player: &Player) -> CellMap {
        self.cell_map
            .iter()
            .fold(CellMap::new(), |mut acc, (position, moving_range)| {
                let cell = moving_range.pivot.clone();
                if let Some(owner) = cell.owner() {
                    if owner == player.clone() {
                        acc.insert(position.clone(), moving_range.clone());
                    }
                }
                acc
            })
    }

    pub(crate) fn refresh(&self, MigratedCellPair { from, to }: &MigratedCellPair) -> Self {
        unimplemented!()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub(crate) enum DestinationState {
    Moveable(Cell),
    Fullfilled(Cell),
    AlreadyOwned(Cell),
    OutOfField,
}

impl DestinationState {
    pub(crate) fn is_moveable(&self) -> bool {
        match self {
            Self::Moveable(_) => true,
            _ => false,
        }
    }

    pub(crate) fn reveal(&self) -> Option<Cell> {
        match self {
            Self::Moveable(cell) => Some(cell.clone()),
            Self::Fullfilled(cell) => Some(cell.clone()),
            Self::AlreadyOwned(cell) => Some(cell.clone()),
            Self::OutOfField => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct MovingRange {
    pub(crate) pivot: Cell,
    pub(crate) up: DestinationState,
    pub(crate) down: DestinationState,
    pub(crate) right: DestinationState,
    pub(crate) left: DestinationState,
    pub(crate) up_right: DestinationState,
    pub(crate) down_right: DestinationState,
    pub(crate) up_left: DestinationState,
    pub(crate) down_left: DestinationState,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) enum Direction {
    Up,
    Down,
    Right,
    Left,
    UpRight,
    DownRight,
    UpLeft,
    DownLeft,
}

impl MovingRange {
    fn new(position: &Position, cell: &Cell, field: &Field) -> Self {
        Self {
            pivot: cell.clone(),
            up: Self::destination(cell, position.above(), field),
            down: Self::destination(cell, position.below(), field),
            left: Self::destination(cell, position.lefthand(), field),
            right: Self::destination(cell, position.righthand(), field),
            up_right: Self::destination(cell, position.above_righthand(), field),
            down_right: Self::destination(cell, position.below_righthand(), field),
            up_left: Self::destination(cell, position.above_lefthand(), field),
            down_left: Self::destination(cell, position.below_lefthand(), field),
        }
    }

    fn destination(pivot: &Cell, moved: Result<Position>, field: &Field) -> DestinationState {
        if moved.is_err() {
            return DestinationState::OutOfField;
        }
        let dest_position = moved.unwrap();
        if let Some(dest_cell) = field.get(&dest_position) {
            if Self::is_reached_stacking_limit(dest_cell) {
                DestinationState::Fullfilled(dest_cell.clone())
            } else if pivot.is_same_owner(dest_cell) {
                DestinationState::AlreadyOwned(dest_cell.clone())
            } else {
                DestinationState::Moveable(dest_cell.clone())
            }
        } else {
            DestinationState::OutOfField
        }
    }

    fn is_reached_stacking_limit(to_cell: &Cell) -> bool {
        to_cell.is_fullfilled()
    }

    pub(crate) fn indicate(&self, direction: &Direction) -> Result<Cell> {
        let x = match direction {
            Direction::Up => self.up,
            Direction::Down => self.down,
            Direction::Right => self.right,
            Direction::Left => self.left,
            Direction::UpRight => self.up_right,
            Direction::DownRight => self.down_right,
            Direction::UpLeft => self.up_left,
            Direction::DownLeft => self.down_left,
        };
        x.reveal().ok_or(Error::IllegalDestination)
    }

    pub(crate) fn moveable_directions(&self) -> BTreeSet<Direction> {
        BTreeSet::from_iter(
            vec![
                (self.up, Direction::Up),
                (self.down, Direction::Down),
                (self.right, Direction::Right),
                (self.left, Direction::Left),
                (self.up_right, Direction::UpRight),
                (self.down_right, Direction::DownRight),
                (self.up_left, Direction::UpLeft),
                (self.down_left, Direction::DownLeft),
            ]
            .into_iter()
            .filter(|(dest, _)| dest.is_moveable())
            .map(|(_, direction)| direction),
        )
    }
}

#[cfg(test)]
mod board_spec {
    use super::{Board, Direction};
    use crate::{
        cell::Cell,
        player::Player,
        position::{Column, Position, Row},
    };

    #[test]
    fn generate_initial_occupied_cells() {
        for side in [Row::Top, Row::Bottom].iter() {
            let player = Player::new();
            let side_row = Board::generate_initial_occupied_cells(player.clone(), side.to_owned())
                .collect::<Vec<(Position, Cell)>>();
            let expected_cells = [
                Column::LeftEdge,
                Column::MiddleFirst,
                Column::MiddleSecond,
                Column::MiddleThird,
                Column::RightEdge,
            ]
            .iter()
            .map(|column| {
                let position = Position::new(column.to_owned(), side.to_owned());
                let cell = Cell::new_occupied(player.clone());
                (position, cell)
            })
            .collect::<Vec<(Position, Cell)>>();
            assert_eq!(side_row, expected_cells);
        }
    }

    #[test]
    fn generate_initial_empty_cells() {
        for row in [
            Row::MiddleFirst,
            Row::MiddleSecond,
            Row::MiddleThird,
            Row::MiddleFourth,
        ]
        .iter()
        {
            let expected_cells = [
                Column::LeftEdge,
                Column::MiddleFirst,
                Column::MiddleSecond,
                Column::MiddleThird,
                Column::RightEdge,
            ]
            .iter()
            .map(move |column| {
                let position = Position::new(column.to_owned(), row.to_owned());
                let cell = Cell::new_empty();
                (position, cell)
            })
            .collect::<Vec<(Position, Cell)>>();
            let row =
                Board::generate_initial_empty_cells(row.clone()).collect::<Vec<(Position, Cell)>>();
            assert_eq!(row, expected_cells);
        }
    }

    #[test]
    fn territory() {
        use std::collections::BTreeSet;

        let player_a = Player::new();
        let player_b = Player::new();
        let board = Board::new(&player_a, &player_b);
        let player_a_territory = board
            .territory(&player_a)
            .keys()
            .map(|k| k.clone())
            .collect::<BTreeSet<Position>>();
        assert_eq!(
            player_a_territory,
            [
                Column::LeftEdge,
                Column::MiddleFirst,
                Column::MiddleSecond,
                Column::MiddleThird,
                Column::RightEdge,
            ]
            .iter()
            .map(|col| { Position::new(col.to_owned(), Row::Top) })
            .collect::<BTreeSet<Position>>(),
        );
    }

    #[test]
    fn refresh() {
        let player_a = Player::new();
        let player_b = Player::new();
        let board = Board::new(&player_a, &player_b);
        let from_cell = Cell::new_occupied(player_a.clone());
        let territory = board.territory(&player_a);
        let position = board.position_of(&from_cell);
        let moving_range = territory.get(&position).unwrap();
        let to_cell = moving_range.indicate(&Direction::Down).unwrap();
        let migrate_cell_pair = from_cell.migrate(&to_cell).unwrap();
        // let refreshed = board.refresh(&migrate_cell_pair);
        // let refreshed_territory = refreshed.territory(&player_a);
    }
}

#[cfg(test)]
mod moving_range_spec {
    use std::collections::BTreeSet;

    use super::{DestinationState, Direction, Field, MovingRange};
    use crate::{
        cell::Cell,
        player::Player,
        position::{Column, Position, Row},
    };

    #[test]
    fn new() {
        let player_a = Player::new();
        let player_b = Player::new();
        let pivot_position = Position::new(Column::LeftEdge, Row::MiddleSecond);
        let pivot_cell = Cell::new_occupied(player_a.clone());
        let opponents_position = pivot_position.above().unwrap();
        let opponents_cell = Cell::new_occupied(player_b.clone());
        let owned_position = pivot_position.below().unwrap();
        let owned_cell = Cell::new_occupied(player_a.clone());
        let robbed_position = pivot_position.righthand().unwrap();
        let robbed_cell = Cell::new_occupied(player_a.clone())
            .stack(&player_b)
            .unwrap();
        let fullfilled_position = pivot_position.above_righthand().unwrap();
        let fullfilled_cell = Cell::new_occupied(player_b.clone())
            .stack(&player_a)
            .unwrap()
            .stack(&player_b)
            .unwrap();
        let empty_position = pivot_position.below_righthand().unwrap();
        let empty_cell = Cell::new_empty();
        let field = [
            (pivot_position.clone(), pivot_cell.clone()),
            (opponents_position.clone(), opponents_cell.clone()),
            (owned_position.clone(), owned_cell.clone()),
            (robbed_position.clone(), robbed_cell.clone()),
            (empty_position.clone(), empty_cell.clone()),
            (fullfilled_position.clone(), fullfilled_cell.clone()),
        ]
        .iter()
        .fold(Field::new(), |mut acc, (position, cell)| {
            acc.insert(position.clone(), cell.clone());
            acc
        });
        let result = MovingRange::new(&pivot_position, &pivot_cell, &field);
        assert_eq!(
            result,
            MovingRange {
                pivot: pivot_cell.clone(),
                up: DestinationState::Moveable(opponents_cell.clone()),
                down: DestinationState::AlreadyOwned(owned_cell.clone()),
                left: DestinationState::OutOfField,
                right: DestinationState::Moveable(robbed_cell.clone()),
                up_right: DestinationState::Fullfilled(fullfilled_cell.clone()),
                down_right: DestinationState::Moveable(empty_cell.clone()),
                up_left: DestinationState::OutOfField,
                down_left: DestinationState::OutOfField,
            }
        )
    }

    #[test]
    fn has() {
        let cell = Cell::new_empty();
        let mr = MovingRange {
            pivot: cell.clone(),
            up: DestinationState::Moveable(cell.clone()),
            down: DestinationState::Moveable(cell.clone()),
            right: DestinationState::Moveable(cell.clone()),
            left: DestinationState::Moveable(cell.clone()),
            up_right: DestinationState::Moveable(cell.clone()),
            down_right: DestinationState::Moveable(cell.clone()),
            up_left: DestinationState::Moveable(cell.clone()),
            down_left: DestinationState::Moveable(cell.clone()),
        };
        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Right,
            Direction::Left,
            Direction::UpRight,
            Direction::DownRight,
            Direction::UpLeft,
            Direction::DownLeft,
        ]
        .iter()
        {
            assert!(mr.indicate(direction).is_ok());
        }
    }

    #[test]
    fn has_no() {
        let mr = MovingRange {
            pivot: Cell::new_empty(),
            up: DestinationState::OutOfField,
            down: DestinationState::OutOfField,
            right: DestinationState::OutOfField,
            left: DestinationState::OutOfField,
            up_right: DestinationState::OutOfField,
            down_right: DestinationState::OutOfField,
            up_left: DestinationState::OutOfField,
            down_left: DestinationState::OutOfField,
        };
        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Right,
            Direction::Left,
            Direction::UpRight,
            Direction::DownRight,
            Direction::UpLeft,
            Direction::DownLeft,
        ]
        .iter()
        {
            assert!(mr.indicate(direction).is_err());
        }
    }

    #[test]
    fn moveable_directions() {
        use std::iter::FromIterator;

        let player = Player::new();
        let pivot_position = Position::new(Column::MiddleFirst, Row::MiddleFirst);
        let pivot_cell = Cell::new_occupied(player.clone());
        let mut field = [
            Position::new(Column::LeftEdge, Row::Top),
            Position::new(Column::MiddleFirst, Row::Top),
            Position::new(Column::MiddleSecond, Row::Top),
            Position::new(Column::LeftEdge, Row::MiddleFirst),
            Position::new(Column::MiddleSecond, Row::MiddleFirst),
            Position::new(Column::LeftEdge, Row::MiddleSecond),
            Position::new(Column::MiddleFirst, Row::MiddleSecond),
            Position::new(Column::MiddleSecond, Row::MiddleSecond),
        ]
        .iter()
        .fold(Field::new(), |mut acc, position| {
            acc.insert(position.clone(), Cell::new_empty());
            acc
        });
        field.insert(pivot_position.clone(), pivot_cell.clone());
        let mr = MovingRange::new(&pivot_position, &pivot_cell, &field);
        eprintln!("{:?}", &mr);
        assert_eq!(
            mr.moveable_directions(),
            BTreeSet::from_iter(vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
                Direction::UpRight,
                Direction::DownRight,
                Direction::UpLeft,
                Direction::DownLeft,
            ])
        );
    }
}

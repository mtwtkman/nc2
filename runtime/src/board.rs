use std::collections::{BTreeSet, HashMap};
use std::iter::FromIterator;

use crate::{
    cell::Cell,
    player::Player,
    position::{Column, Position, Row},
    result::{Error, Result},
};

pub(crate) type CellMap = HashMap<Position, Cell>;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Board {
    pub(crate) cell_map: CellMap,
}
impl Board {
    pub(crate) fn new(player_a: &Player, player_b: &Player) -> Self {
        let cell_map = Self::build_initial_cell_map(player_a, player_b);
        Self { cell_map }
    }

    fn build_initial_cell_map(player_a: &Player, player_b: &Player) -> CellMap {
        let mut cell_map = CellMap::new();
        let player_a_side_cells = Self::generate_initial_occupied_cells(player_a.clone(), Row::Top);
        player_a_side_cells.for_each(|(position, cell)| {
            cell_map.insert(position, cell);
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
            cell_map.insert(position, cell);
        });
        let player_b_side_cells =
            Self::generate_initial_occupied_cells(player_b.clone(), Row::Bottom);
        player_b_side_cells.for_each(|(position, cell)| {
            cell_map.insert(position, cell);
        });
        cell_map
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

    pub(crate) fn is_occupied_by(&self, position: &Position, player: &Player) -> bool {
        self.territory(player).contains_key(position)
    }

    pub(crate) fn territory(&self, player: &Player) -> CellMap {
        self.cell_map
            .iter()
            .fold(CellMap::new(), |mut acc, (position, cell)| {
                if let Some(owner) = cell.owner() {
                    if owner == player.clone() {
                        acc.insert(position.clone(), cell.clone());
                    }
                }
                acc
            })
    }

    pub(crate) fn cell_of(&self, position: &Position) -> Result<Cell> {
        self.cell_map
            .get(position)
            .ok_or(Error::InvalidPosition)
            .map(|x| x.clone())
    }

    pub(crate) fn migrate(&self, from: &Position, to: &Position) -> Result<Self> {
        if from == to {
            return Err(Error::SamePositionCannotBeMigrated);
        }
        let from_cell = self.cell_of(from)?;
        let to_cell = self.cell_of(to)?;
        if from_cell.is_empty() {
            return Err(Error::CellIsEmpty);
        } else if to_cell.is_fullfilled() {
            return Err(Error::CellIsFullfilled);
        }
        let owner = from_cell.owner();
        let destination_owner = to_cell.owner();
        if owner == destination_owner {
            return Err(Error::AlreadyOccupied(destination_owner.unwrap()));
        }
        let migrated_from_cell = from_cell.unstack()?;
        let migrated_to_cell = to_cell.stack(&owner.unwrap())?;
        let mut cell_map = self.cell_map.clone();
        cell_map.insert(from.clone(), migrated_from_cell.clone());
        cell_map.insert(to.clone(), migrated_to_cell.clone());
        Ok(Self { cell_map })
    }

    pub(crate) fn moving_range_of(&self, pivot_position: &Position) -> Result<MovingRange> {
        MovingRange::new(&pivot_position, &self.cell_map)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct Point {
    pub(crate) position: Position,
    pub(crate) cell: Cell,
}

impl Point {
    fn new(position: Position, cell: Cell) -> Self {
        Self { position, cell }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub(crate) enum DestinationState {
    Moveable(Point),
    Fullfilled(Point),
    AlreadyOwned(Point),
    OutOfField,
}

impl DestinationState {
    pub(crate) fn is_moveable(&self) -> bool {
        match self {
            Self::Moveable(_) => true,
            _ => false,
        }
    }

    pub(crate) fn reveal(&self) -> Option<Point> {
        match self {
            Self::Moveable(point) => Some(point.clone()),
            Self::Fullfilled(point) => Some(point.clone()),
            Self::AlreadyOwned(point) => Some(point.clone()),
            Self::OutOfField => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct MovingRange {
    pub(crate) pivot: Point,
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

impl Direction {
    pub(crate) fn destination(&self, from: &Position) -> Result<Position> {
        match self {
            Self::Up => from.above(),
            Self::Down => from.below(),
            Self::Right => from.righthand(),
            Self::Left => from.lefthand(),
            Self::UpRight => from.above_righthand(),
            Self::DownRight => from.below_righthand(),
            Self::UpLeft => from.above_lefthand(),
            Self::DownLeft => from.below_lefthand(),
        }
    }
}

impl MovingRange {
    fn new(pivot_position: &Position, cell_map: &CellMap) -> Result<Self> {
        let cell = cell_map.get(pivot_position).ok_or(Error::CellNotFound)?;
        Ok(Self {
            pivot: Point::new(pivot_position.clone(), cell.clone()),
            up: Self::destination(cell, pivot_position.above(), cell_map),
            down: Self::destination(cell, pivot_position.below(), cell_map),
            left: Self::destination(cell, pivot_position.lefthand(), cell_map),
            right: Self::destination(cell, pivot_position.righthand(), cell_map),
            up_right: Self::destination(cell, pivot_position.above_righthand(), cell_map),
            down_right: Self::destination(cell, pivot_position.below_righthand(), cell_map),
            up_left: Self::destination(cell, pivot_position.above_lefthand(), cell_map),
            down_left: Self::destination(cell, pivot_position.below_lefthand(), cell_map),
        })
    }

    fn destination(pivot: &Cell, moved: Result<Position>, cell_map: &CellMap) -> DestinationState {
        if moved.is_err() {
            return DestinationState::OutOfField;
        }
        let dest_position = moved.unwrap();
        if let Some(dest_cell) = cell_map.get(&dest_position) {
            if Self::is_reached_stacking_limit(dest_cell) {
                DestinationState::Fullfilled(Point::new(dest_position, dest_cell.clone()))
            } else if pivot.is_same_owner(dest_cell) {
                DestinationState::AlreadyOwned(Point::new(dest_position, dest_cell.clone()))
            } else {
                DestinationState::Moveable(Point::new(dest_position, dest_cell.clone()))
            }
        } else {
            DestinationState::OutOfField
        }
    }

    fn is_reached_stacking_limit(to_cell: &Cell) -> bool {
        to_cell.is_fullfilled()
    }

    pub(crate) fn indicate(&self, direction: &Direction) -> Result<Point> {
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
}

#[cfg(test)]
mod moving_range_spec {
    use std::collections::BTreeSet;

    use super::{CellMap, DestinationState, Direction, MovingRange, Point};
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
        let cell_map = [
            (pivot_position.clone(), pivot_cell.clone()),
            (opponents_position.clone(), opponents_cell.clone()),
            (owned_position.clone(), owned_cell.clone()),
            (robbed_position.clone(), robbed_cell.clone()),
            (empty_position.clone(), empty_cell.clone()),
            (fullfilled_position.clone(), fullfilled_cell.clone()),
        ]
        .iter()
        .fold(CellMap::new(), |mut acc, (position, cell)| {
            acc.insert(position.clone(), cell.clone());
            acc
        });
        let result = MovingRange::new(&pivot_position, &cell_map);
        assert_eq!(
            result,
            Ok(MovingRange {
                pivot: Point::new(pivot_position.clone(), pivot_cell.clone()),
                up: DestinationState::Moveable(Point::new(
                    opponents_position.clone(),
                    opponents_cell.clone()
                )),
                down: DestinationState::AlreadyOwned(Point::new(
                    owned_position.clone(),
                    owned_cell.clone()
                )),
                left: DestinationState::OutOfField,
                right: DestinationState::Moveable(Point::new(
                    robbed_position.clone(),
                    robbed_cell.clone()
                )),
                up_right: DestinationState::Fullfilled(Point::new(
                    fullfilled_position.clone(),
                    fullfilled_cell.clone()
                )),
                down_right: DestinationState::Moveable(Point::new(
                    empty_position.clone(),
                    empty_cell.clone()
                )),
                up_left: DestinationState::OutOfField,
                down_left: DestinationState::OutOfField,
            })
        )
    }

    #[test]
    fn has() {
        let cell = Cell::new_empty();
        let position = Position::new(Column::MiddleFirst, Row::MiddleFirst);
        let mr = MovingRange {
            pivot: Point::new(position.clone(), cell.clone()),
            up: DestinationState::Moveable(Point::new(position.above().unwrap(), cell.clone())),
            down: DestinationState::Moveable(Point::new(position.below().unwrap(), cell.clone())),
            right: DestinationState::Moveable(Point::new(
                position.righthand().unwrap(),
                cell.clone(),
            )),
            left: DestinationState::Moveable(Point::new(
                position.lefthand().unwrap(),
                cell.clone(),
            )),
            up_right: DestinationState::Moveable(Point::new(
                position.above_righthand().unwrap(),
                cell.clone(),
            )),
            down_right: DestinationState::Moveable(Point::new(
                position.below_righthand().unwrap(),
                cell.clone(),
            )),
            up_left: DestinationState::Moveable(Point::new(
                position.above_lefthand().unwrap(),
                cell.clone(),
            )),
            down_left: DestinationState::Moveable(Point::new(
                position.below_lefthand().unwrap(),
                cell.clone(),
            )),
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
            pivot: Point::new(Position::new(Column::LeftEdge, Row::Top), Cell::new_empty()),
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
        let mut cell_map = [
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
        .fold(CellMap::new(), |mut acc, position| {
            acc.insert(position.clone(), Cell::new_empty());
            acc
        });
        cell_map.insert(pivot_position.clone(), pivot_cell.clone());
        let mr = MovingRange::new(&pivot_position, &cell_map);
        assert!(mr.is_ok());
        assert_eq!(
            mr.unwrap().moveable_directions(),
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

#[cfg(test)]
mod migrate_spec {
    use super::Board;
    use crate::{
        player::Player,
        position::{Column, Position, Row},
        result::Error,
    };

    #[test]
    fn migrate() {
        let player_a = Player::new();
        let player_b = Player::new();
        let board = Board::new(&player_a, &player_b);
        let from_position = Position::new(Column::LeftEdge, Row::Top);
        let to_position = from_position.below().unwrap();
        let migrated = board.migrate(&from_position, &to_position);
        assert!(migrated.is_ok());
        let migrated_territory = migrated.unwrap().territory(&player_a);
        let migrated_from_cell = migrated_territory.get(&from_position);
        assert!(migrated_from_cell.is_none());
        let migrated_to_cell = migrated_territory.get(&to_position).unwrap();
        assert_eq!(migrated_to_cell.owner(), Some(player_a));
    }

    #[test]
    fn empty_cell_cannot_migrate() {
        let player_a = Player::new();
        let player_b = Player::new();
        let board = Board::new(&player_a, &player_b);
        let migrated = board.migrate(
            &Position::new(Column::MiddleFirst, Row::MiddleFirst),
            &Position::new(Column::MiddleFirst, Row::MiddleSecond),
        );
        assert_eq!(migrated, Err(Error::CellIsEmpty));
    }
    #[test]
    fn fullfilled_cell_cannot_migrate() {
        let player_a = Player::new();
        let player_b = Player::new();
        let mut board = Board::new(&player_a, &player_b);
        let from_position = Position::new(Column::LeftEdge, Row::Top);
        let to_position = from_position.below().unwrap();
        let fullfilled_cell = board
            .cell_of(&to_position)
            .unwrap()
            .stack(&player_b)
            .unwrap()
            .stack(&player_a)
            .unwrap()
            .stack(&player_b)
            .unwrap();

        board
            .cell_map
            .insert(to_position.clone(), fullfilled_cell.clone());
        assert_eq!(
            board.migrate(&from_position, &to_position),
            Err(Error::CellIsFullfilled)
        );
    }

    #[test]
    fn already_occupied_cell_cannot_migrate() {
        let player_a = Player::new();
        let player_b = Player::new();
        let mut board = Board::new(&player_a, &player_b);
        let from_position = Position::new(Column::MiddleFirst, Row::Top);
        let to_position = from_position.lefthand().unwrap();
        let already_occupied_cell = board.cell_of(&to_position).unwrap();
        board
            .cell_map
            .insert(to_position.clone(), already_occupied_cell.clone());
        assert_eq!(
            board.migrate(&from_position, &to_position),
            Err(Error::AlreadyOccupied(player_a.clone()))
        );
    }
}

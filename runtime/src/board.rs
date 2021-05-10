use std::collections::HashMap;

use crate::{
    cell::Cell,
    player::Player,
    position::{Column, Position, Row},
    result::{Error, Result},
};

type Field = HashMap<Position, Cell>;
pub(crate) type CellMap = HashMap<Cell, MovingRange>;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Board {
    pub(crate) cell_map: CellMap,
}
impl Board {
    pub(crate) fn new(player_a: &Player, player_b: &Player) -> Self {
        let cell_map = Self::setup_cell_map(player_a, player_b);
        Self { cell_map }
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

    fn setup_cell_map(player_a: &Player, player_b: &Player) -> CellMap {
        let field = Self::build_initial_field(player_a, player_b);
        field.iter().fold(CellMap::new(), |mut acc, (_, cell)| {
            acc.insert(cell.clone(), MovingRange::new(&cell, &field));
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
            let cell = Cell::new_occupied(position.clone(), player.clone());
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
            let cell = Cell::new_empty(position.clone());
            (position, cell)
        })
    }

    pub(crate) fn territory(&self, player: &Player) -> CellMap {
        self.cell_map
            .iter()
            .fold(CellMap::new(), |mut acc, (cell, moving_range)| {
                if let Some(owner) = cell.owner() {
                    if owner == player.clone() {
                        acc.insert(cell.clone(), moving_range.clone());
                    }
                }
                acc
            })
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct MovingRange {
    pub(crate) up: Option<Cell>,
    pub(crate) down: Option<Cell>,
    pub(crate) right: Option<Cell>,
    pub(crate) left: Option<Cell>,
    pub(crate) up_right: Option<Cell>,
    pub(crate) down_right: Option<Cell>,
    pub(crate) up_left: Option<Cell>,
    pub(crate) down_left: Option<Cell>,
}

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

#[cfg(test)]
impl Default for MovingRange {
    fn default() -> Self {
        MovingRange {
            up: None,
            down: None,
            right: None,
            left: None,
            up_right: None,
            down_right: None,
            up_left: None,
            down_left: None,
        }
    }
}

impl MovingRange {
    fn new(cell: &Cell, field: &Field) -> Self {
        Self {
            up: Self::destination(cell, cell.position.move_up(), field),
            down: Self::destination(cell, cell.position.move_down(), field),
            left: Self::destination(cell, cell.position.move_left(), field),
            right: Self::destination(cell, cell.position.move_right(), field),
            up_right: Self::destination(cell, cell.position.move_up_right(), field),
            down_right: Self::destination(cell, cell.position.move_down_right(), field),
            up_left: Self::destination(cell, cell.position.move_up_left(), field),
            down_left: Self::destination(cell, cell.position.move_down_left(), field),
        }
    }

    fn destination(pivot: &Cell, moved: Result<Position>, field: &Field) -> Option<Cell> {
        moved.ok().and_then(|dest| {
            if let Some(cell) = field.get(&dest) {
                if Self::is_not_owned(pivot, cell) && !Self::is_reached_stacking_limit(cell) {
                    Some(cell.clone())
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    fn is_not_owned(from_cell: &Cell, to_cell: &Cell) -> bool {
        from_cell.owner() != to_cell.owner()
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
        x.ok_or(Error::IllegalDestination)
    }
}

#[test]
fn generate_initial_occupied_cells() {
    use crate::{cell::Cell, position::Column};

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
            let cell = Cell::new_occupied(position.clone(), player.clone());
            (position, cell)
        })
        .collect::<Vec<(Position, Cell)>>();
        assert_eq!(side_row, expected_cells);
    }
}

#[test]
fn generate_initial_empty_cells() {
    use crate::position::Column;

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
            let cell = Cell::new_empty(position.clone());
            (position, cell)
        })
        .collect::<Vec<(Position, Cell)>>();
        let row =
            Board::generate_initial_empty_cells(row.clone()).collect::<Vec<(Position, Cell)>>();
        assert_eq!(row, expected_cells);
    }
}

#[cfg(test)]
mod moving_range_spec {
    use super::{Field, MovingRange, Direction};
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
        let pivot_cell = Cell::new_occupied(pivot_position.clone(), player_a.clone());
        let opponents_position = pivot_position.move_up().unwrap();
        let opponents_cell = Cell::new_occupied(opponents_position.clone(), player_b.clone());
        let owned_position = pivot_position.move_down().unwrap();
        let owned_cell = Cell::new_occupied(owned_position.clone(), player_a.clone());
        let robbed_position = pivot_position.move_right().unwrap();
        let robbed_cell = Cell::new_occupied(robbed_position.clone(), player_a.clone())
            .stack(&player_b)
            .unwrap();
        let fullfilled_position = pivot_position.move_up_right().unwrap();
        let _fullfilled_cell = Cell::new_occupied(fullfilled_position, player_b.clone())
            .stack(&player_a)
            .unwrap()
            .stack(&player_b)
            .unwrap();
        let empty_cell = Cell::new_empty(pivot_position.move_down_right().unwrap());
        let field = [
            pivot_cell.clone(),
            opponents_cell.clone(),
            owned_cell.clone(),
            robbed_cell.clone(),
            empty_cell.clone(),
        ]
        .iter()
        .fold(Field::new(), |mut acc, cell| {
            acc.insert(cell.clone().position, cell.clone());
            acc
        });
        let result = MovingRange::new(&pivot_cell, &field);
        assert_eq!(
            result,
            MovingRange {
                up: Some(opponents_cell.clone()),
                down: None,
                left: None,
                right: Some(robbed_cell.clone()),
                up_right: None,
                down_right: Some(empty_cell.clone()),
                up_left: None,
                down_left: None,
            }
        )
    }

    #[test]
    fn has() {
        let cell = Cell::new_empty(Position::new(Column::LeftEdge, Row::Top));
        let mr = MovingRange {
            up: Some(cell.clone()),
            down: Some(cell.clone()),
            right: Some(cell.clone()),
            left: Some(cell.clone()),
            up_right: Some(cell.clone()),
            down_right: Some(cell.clone()),
            up_left: Some(cell.clone()),
            down_left: Some(cell.clone()),
        };
        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Right,
            Direction::Left,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::UpLeft,
            Direction::DownLeft,
        ].iter() {
            assert!(mr.indicate(direction).is_ok());
        }
    }

    fn has_no() {
        let cell = Cell::new_empty(Position::new(Column::LeftEdge, Row::Top));
        let mr = MovingRange::default();
        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Right,
            Direction::Left,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::UpLeft,
            Direction::DownLeft,
        ].iter() {
            assert!(mr.indicate(direction).is_err());
        }
    }
}

use std::collections::{BTreeSet, HashMap};

use crate::{
    cell::Cell,
    player::Player,
    position::{Column, Position, Row},
    result::Result,
};

type CellMap = HashMap<Position, Cell>;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Board {
    cell_map: CellMap,
}
impl Board {
    pub(crate) fn new(player_a: &Player, player_b: &Player) -> Self {
        Self {
            cell_map: Self::setup_cell_map(player_a, player_b),
        }
    }

    fn setup_cell_map(player_a: &Player, player_b: &Player) -> CellMap {
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

    fn player_positions(&self, player: &Player) -> BTreeSet<Position> {
        self.cell_map
            .iter()
            .fold(BTreeSet::new(), |mut acc, (position, cell)| {
                if let Some(owner) = cell.owner() {
                    if owner == player.clone() {
                        acc.insert(position.clone());
                    }
                }
                acc
            })
    }

    fn position_to_cell(&self, position: Result<Position>) -> Option<Cell> {
        position
            .ok()
            .and_then(|p| self.cell_map.get(&p))
            .map(|x| x.clone())
    }

    fn detect_moving_range(&self, pivot_position: &Position) -> MovingRange {
        let cell = self.cell_map.get(&pivot_position).unwrap().clone();
        MovingRange {
            up: self.position_to_cell(pivot_position.move_up()),
            down: self.position_to_cell(pivot_position.move_down()),
            left: self.position_to_cell(pivot_position.move_left()),
            right: self.position_to_cell(pivot_position.move_right()),
            up_right: self.position_to_cell(pivot_position.move_up_right()),
            down_right: self.position_to_cell(pivot_position.move_down_right()),
            up_left: self.position_to_cell(pivot_position.move_up_left()),
            down_left: self.position_to_cell(pivot_position.move_down_left()),
            pivot: cell,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct MovingRange {
    up: Option<Cell>,
    down: Option<Cell>,
    right: Option<Cell>,
    left: Option<Cell>,
    up_right: Option<Cell>,
    down_right: Option<Cell>,
    up_left: Option<Cell>,
    down_left: Option<Cell>,
    pivot: Cell,
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
            let cell = Cell::new_occupied(player.clone());
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
fn new() {
    let player_a = Player::new();
    let player_b = Player::new();
    let board = Board::new(&player_a, &player_b);
    let mut expected: CellMap = CellMap::new();
    let columns = [
        Column::LeftEdge,
        Column::MiddleFirst,
        Column::MiddleSecond,
        Column::MiddleThird,
        Column::RightEdge,
    ];
    columns.iter().for_each(|column| {
        expected.insert(
            Position::new(column.to_owned(), Row::Top),
            Cell::new_occupied(player_a.clone()),
        );
    });
    [
        Row::MiddleFirst,
        Row::MiddleSecond,
        Row::MiddleThird,
        Row::MiddleFourth,
    ]
    .iter()
    .for_each(|row| {
        columns.iter().for_each(|column| {
            expected.insert(
                Position::new(column.to_owned(), row.to_owned()),
                Cell::new_empty(),
            );
        });
    });
    columns.iter().for_each(|column| {
        expected.insert(
            Position::new(column.to_owned(), Row::Bottom),
            Cell::new_occupied(player_b.clone()),
        );
    });
    assert_eq!(board, Board { cell_map: expected });
}

#[test]
fn player_position() {
    let player_a = Player::new();
    let player_b = Player::new();
    let mut board = Board::new(&player_a, &player_b);
    let left_edge_top_position = Position::new(Column::LeftEdge, Row::Top);
    assert_eq!(
        board.player_positions(&player_a),
        vec![
            left_edge_top_position.clone(),
            Position::new(Column::MiddleFirst, Row::Top),
            Position::new(Column::MiddleSecond, Row::Top),
            Position::new(Column::MiddleThird, Row::Top),
            Position::new(Column::RightEdge, Row::Top),
        ]
        .into_iter()
        .fold(BTreeSet::new(), |mut acc, position| {
            acc.insert(position);
            acc
        })
    );
    assert_eq!(
        board.player_positions(&player_b),
        vec![
            Position::new(Column::LeftEdge, Row::Bottom),
            Position::new(Column::MiddleFirst, Row::Bottom),
            Position::new(Column::MiddleSecond, Row::Bottom),
            Position::new(Column::MiddleThird, Row::Bottom),
            Position::new(Column::RightEdge, Row::Bottom),
        ]
        .into_iter()
        .fold(BTreeSet::new(), |mut acc, position| {
            acc.insert(position);
            acc
        })
    );
    let (position, mut cell) = board
        .cell_map
        .remove_entry(&left_edge_top_position)
        .unwrap();
    cell.pallet[1] = Some(player_b.clone());
    board.cell_map.insert(position, cell);
    assert_eq!(
        board.player_positions(&player_a),
        vec![
            Position::new(Column::MiddleFirst, Row::Top),
            Position::new(Column::MiddleSecond, Row::Top),
            Position::new(Column::MiddleThird, Row::Top),
            Position::new(Column::RightEdge, Row::Top),
        ]
        .into_iter()
        .fold(BTreeSet::new(), |mut acc, position| {
            acc.insert(position);
            acc
        })
    );
    assert_eq!(
        board.player_positions(&player_b),
        vec![
            left_edge_top_position.clone(),
            Position::new(Column::LeftEdge, Row::Bottom),
            Position::new(Column::MiddleFirst, Row::Bottom),
            Position::new(Column::MiddleSecond, Row::Bottom),
            Position::new(Column::MiddleThird, Row::Bottom),
            Position::new(Column::RightEdge, Row::Bottom),
        ]
        .into_iter()
        .fold(BTreeSet::new(), |mut acc, position| {
            acc.insert(position);
            acc
        })
    );
}

#[cfg(test)]
mod moving_range_spec {
    use super::{MovingRange, Board};
    use crate::{
        player::Player,
        cell::Cell,
        position::{Position, Row, Column},
    };

    #[test]
    fn corner() {
        let player_a = Player::new();
        let player_b = Player::new();
        let board = Board::new(&player_a, &player_b);
        let left_top_corner = Position::new(Column::LeftEdge, Row::Top);
        assert_eq!(board.detect_moving_range(&left_top_corner), MovingRange {
            up: None,
            down: Some(Cell::new_empty()),
            right: Some(Cell::new_occupied(player_a.clone())),
            left: None,
            up_right: None,
            down_right: Some(Cell::new_empty()),
            up_left: None,
            down_left: None,
            pivot: Cell::new_occupied(player_a.clone()),
        });

        let right_top_corner = Position::new(Column::RightEdge, Row::Top);
        assert_eq!(board.detect_moving_range(&right_top_corner), MovingRange {
            up: None,
            down: Some(Cell::new_empty()),
            right: None,
            left: Some(Cell::new_occupied(player_a.clone())),
            up_right: None,
            down_right: None,
            up_left: None,
            down_left: Some(Cell::new_empty()),
            pivot: Cell::new_occupied(player_a.clone()),
        });

        let left_bottom_corner = Position::new(Column::LeftEdge, Row::Bottom);
        assert_eq!(board.detect_moving_range(&left_bottom_corner), MovingRange {
            up: Some(Cell::new_empty()),
            down: None,
            right: Some(Cell::new_occupied(player_b.clone())),
            left: None,
            up_right: Some(Cell::new_empty()),
            down_right: None,
            up_left: None,
            down_left: None,
            pivot: Cell::new_occupied(player_b.clone()),
        });

        let right_bottom_corner = Position::new(Column::RightEdge, Row::Bottom);
        assert_eq!(board.detect_moving_range(&right_bottom_corner), MovingRange{
            up: Some(Cell::new_empty()),
            down: None,
            right: None,
            left: Some(Cell::new_occupied(player_b.clone())),
            up_right: None,
            down_right: None,
            up_left: Some(Cell::new_empty()),
            down_left: None,
            pivot: Cell::new_occupied(player_b.clone()),
        });
    }
}
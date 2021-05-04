use std::collections::HashMap;

use crate::{
    cell::Cell,
    player::Player,
    position::{Column, Position, Row},
};

type CellMap = HashMap<Position, Cell>;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Board {
    cell_map: CellMap,
}
impl Board {
    pub(crate) fn new(player_a: &Player, player_b: &Player) -> Self {
        Self {
            cell_map: Self::setup(player_a, player_b),
        }
    }

    fn setup(player_a: &Player, player_b: &Player) -> CellMap {
        let mut cell_map = CellMap::new();
        let player_a_side_cells = Self::generate_initial_occupied_cells(player_a.clone(), Row::Top);
        player_a_side_cells
            .for_each(|(position, cell)| {
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
        empty_rows
            .for_each(|(position, cell)| {
                cell_map.insert(position, cell);
            });
        let player_b_side_cells = Self::generate_initial_occupied_cells(player_b.clone(), Row::Bottom);
        player_b_side_cells
            .for_each(|(position, cell)| {
                cell_map.insert(position, cell);
            });
        cell_map
    }

    fn generate_initial_occupied_cells(player: Player, side: Row) -> impl Iterator<Item=(Position, Cell)> {
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
            (position,cell)
        })
    }

    fn generate_initial_empty_cells(row: Row) -> impl Iterator<Item=(Position, Cell)> {
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

impl MovingRange {
    fn default(pivot: Cell) -> Self {
        Self {
            up: None,
            down: None,
            right: None,
            left: None,
            up_right: None,
            down_right: None,
            up_left: None,
            down_left: None,
            pivot,
        }
    }
}

#[test]
fn generate_initial_occupied_cells() {
    use crate::{
        cell::Cell,
        position::Column,
    };

    for side in [Row::Top, Row::Bottom].iter() {
        let player = Player::new();
        let side_row = Board::generate_initial_occupied_cells(player.clone(), side.to_owned()).collect::<Vec<(Position, Cell)>>();
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
        let row = Board::generate_initial_empty_cells(row.clone()).collect::<Vec<(Position, Cell)>>();
        assert_eq!(row, expected_cells);
    }
}

#[test]
fn new() {
    let player_a = Player::new();
    let player_b = Player::new();
    let board = Board::new(&player_a, &player_b);
    let mut expected: CellMap = CellMap::new();
    let columns =
    [
        Column::LeftEdge,
        Column::MiddleFirst,
        Column::MiddleSecond,
        Column::MiddleThird,
        Column::RightEdge,
    ];
    columns
        .iter()
        .for_each(|column| {
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
        columns
            .iter()
            .for_each(|column| {
                expected.insert(
                    Position::new(column.to_owned(), row.to_owned()),
                    Cell::new_empty(),
                );
            });
    });
    columns
        .iter()
        .for_each(|column| {
            expected.insert(
                Position::new(column.to_owned(), Row::Bottom),
                Cell::new_occupied(player_b.clone()),
            );
        });
    assert_eq!(board, Board { cell_map: expected });
}
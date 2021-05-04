use std::collections::HashMap;

use crate::{
    cell::Cell,
    player::Player,
    position::{Column, Position, Row},
};

type CellMap = HashMap<Cell, MovingRange>;

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
        let player_a_side_cells = Self::build_initial_occupied_row(player_a, Row::Top);
        let empty_rows = [
            Row::MiddleFirst,
            Row::MiddleSecond,
            Row::MiddleThird,
            Row::MiddleFourth,
        ]
        .iter()
        .map(|row| Self::build_initial_empty_row(row))
        .collect::<Vec<Vec<Cell>>>();
        let player_b_side_cells = Self::build_initial_occupied_row(player_b, Row::Bottom);
        unimplemented!()
    }

    fn build_initial_occupied_row(player: &Player, side: Row) -> Vec<Cell> {
        let columns = [
            Column::LeftEdge,
            Column::MiddleFirst,
            Column::MiddleSecond,
            Column::MiddleThird,
            Column::RightEdge,
        ];
        columns
            .iter()
            .map(|column| {
                let position = Position::new(column.to_owned(), side.clone());
                Cell::new_occupied(&position, player)
            })
            .collect::<Vec<Cell>>()
    }

    fn build_initial_empty_row(row: &Row) -> Vec<Cell> {
        [
            Column::LeftEdge,
            Column::MiddleFirst,
            Column::MiddleSecond,
            Column::MiddleThird,
            Column::RightEdge,
        ]
        .iter()
        .map(|column| {
            let position = Position::new(column.to_owned(), row.clone());
            Cell::new_empty(&position)
        })
        .collect::<Vec<Cell>>()
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
}

#[test]
fn build_initial_occupied_row() {
    use crate::{cell::Pallet, position::Column};

    for side in [Row::Top, Row::Bottom].iter() {
        let player = Player::new();
        let side_row = Board::build_initial_occupied_row(&player, side.to_owned());
        let expected_cells = [
            Column::LeftEdge,
            Column::MiddleFirst,
            Column::MiddleSecond,
            Column::MiddleThird,
            Column::RightEdge,
        ]
        .iter()
        .map(|column| Cell {
            position: Position::new(column.to_owned(), side.to_owned()),
            pallet: Pallet {
                players: [Some(player.clone()), None, None],
            },
        })
        .collect::<Vec<Cell>>();
        assert_eq!(side_row, expected_cells);
    }
}

#[test]
fn build_initial_empty_row() {
    use crate::{
        cell::{Pallet, PALLET_HEIGHT_LIMIT},
        position::Column,
    };

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
        .map(|column| Cell {
            position: Position::new(column.to_owned(), row.to_owned()),
            pallet: Pallet {
                players: [None; PALLET_HEIGHT_LIMIT],
            },
        })
        .collect::<Vec<Cell>>();
        let row = Board::build_initial_empty_row(row);
        assert_eq!(row, expected_cells);
    }
}

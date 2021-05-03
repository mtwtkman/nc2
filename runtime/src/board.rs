use std::collections::HashMap;

use crate::{
    cell::Cell,
    player::Player,
    position::{Position, Column, Row},
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
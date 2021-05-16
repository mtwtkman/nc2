use crate::player::Player;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    ReachedTop,
    ReachedBottom,
    ReachedRightEdge,
    ReachedLeftEdge,
    ReachedPalletHeightLimit,
    CellIsEmpty,
    CellIsFullfilled,
    AlreadyOccupied(Player),
    IllegalDestination,
    InvalidPosition,
    CellNotFound,
    SamePositionCannotBeMigrated,
}

pub type Result<T> = std::result::Result<T, Error>;

use crate::player::Player;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Error {
    ReachedTop,
    ReachedBottom,
    ReachedRightEdge,
    ReachedLeftEdge,
    ReachedPalletHeightLimit,
    CellIsEmpty,
    CellIsFullfilled,
    AlreadyOccupied(Player),
    InvalidDirection,
    IllegalDestination,
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

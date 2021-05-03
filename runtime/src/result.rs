use crate::player::Player;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Error {
    ReachedTop,
    ReachedBottom,
    ReachedRightEdge,
    ReachedLeftEdge,
    ReachedPalletHeightLimit,
    PalletIsEmpty,
    AlreadyOccupied(Player),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

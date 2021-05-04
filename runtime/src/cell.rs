use crate::player::Player;
use crate::position::Position;
use crate::result::{Result, Error};

const PALLET_HEIGHT_LIMIT: usize = 3;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Pallet {
    players: [Option<Player>;PALLET_HEIGHT_LIMIT],
}

impl Pallet {
    fn height(&self) -> usize {
        self.players
            .iter()
            .filter(|x| x.is_some())
            .count()
    }

    fn apex(&self) -> Option<Player> {
        if self.is_empty() {
            None
        } else {
            self.players[self.height() - 1].clone()
        }
    }

    fn is_empty(&self) -> bool {
        self.height() == 0
    }

    fn empty() -> Self {
        Self {
            players: [None;PALLET_HEIGHT_LIMIT],
        }
    }

    fn occupied(player: &Player) -> Self {
        Self {
            players: [Some(player.clone()), None, None],
        }
    }

    fn stack(&self, player: &Player) -> Result<Self> {
        let height = self.height();
        if height == PALLET_HEIGHT_LIMIT {
            Err(Error::ReachedPalletHeightLimit)
        } else {
            let apex = self.apex();
            if apex == Some(player.clone()) {
                Err(Error::AlreadyOccupied(player.clone()))
            } else {
                let mut players = self.players.clone();
                players[height] = Some(player.clone());
                Ok(Self { players })
            }
        }
    }

    fn unstack(&self) -> Result<Self> {
        if self.is_empty() {
            Err(Error::PalletIsEmpty)
        } else {
            let mut players = self.players.clone();
            players[self.height() - 1] = None;
            Ok(Self { players })
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct Cell {
    pub(crate) position: Position,
    pallet: Pallet,
}
impl Cell {
    pub(crate) fn new_empty(position: &Position) -> Self {
        Self {
            position: position.clone(),
            pallet: Pallet::empty(),
        }
    }

    pub(crate) fn new_occupied(position: &Position, player: &Player) -> Self {
        Self {
            position: position.clone(),
            pallet: Pallet::occupied(player),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.pallet.is_empty()
    }

    pub(crate) fn owner(&self) -> Option<Player> {
        self.pallet.apex()
    }

    pub(crate) fn stack(&self, player: &Player) -> Result<Self> {
        let stacked = self.pallet.stack(player)?;
        Ok(Self {
            position: self.position.clone(),
            pallet: stacked,
        })
    }

    pub(crate) fn unstack(&self) -> Result<Self> {
        let unstacked = self.pallet.unstack()?;
        Ok(Self {
            position: self.position.clone(),
            pallet: unstacked,
        })
    }
}

#[cfg(test)]
mod testing {
    use crate::{
        position::{Row, Column, Position},
        player::Player,
    };

    pub(crate) fn spawn_player() -> Player {
        Player::new()
    }

    pub(crate) fn spawn_position() -> Position {
        Position::new(Column::LeftEdge, Row::Top)
    }
}

#[test]
fn new_occupied() {
    let position = testing::spawn_position();
    let player = testing::spawn_player();
    let cell = Cell::new_occupied(&position, &player);
    assert_eq!(cell, Cell {
        position,
        pallet: Pallet { players: [Some(player.clone()), None, None]},
    });
    assert_eq!(cell.owner(), Some(player.clone()));
    assert_eq!(cell.pallet.height(), 1);
    assert!(!cell.is_empty());
}

#[test]
fn new_empty() {
    let position = testing::spawn_position();
    let cell = Cell::new_empty(&position);
    assert_eq!(cell, Cell {
        position,
        pallet: Pallet { players: [None;PALLET_HEIGHT_LIMIT] },
    });
    assert!(cell.is_empty());
    assert_eq!(cell.owner(), None);
}

#[test]
fn stack() {
    let position = testing::spawn_position();
    let player_1 = testing::spawn_player();
    let cell = Cell::new_empty(&position);
    let first_stacked = cell.stack(&player_1);
    assert!(first_stacked.is_ok());
    let cell_has_one_player = first_stacked.unwrap();
    let over_stacking_cell = cell_has_one_player.clone();
    assert_eq!(
        over_stacking_cell.stack(&player_1),
        Err(Error::AlreadyOccupied(player_1.clone())),
    );
    assert_eq!(cell_has_one_player.pallet.height(), 1);
    assert_eq!(cell_has_one_player.owner(), Some(player_1.clone()));
    let player_2 = testing::spawn_player();
    let second_stacked = cell_has_one_player.stack(&player_2);
    assert!(second_stacked.is_ok());
    let cell_has_two_players = second_stacked.unwrap();
    assert_eq!(&cell_has_two_players.pallet.players, &[Some(player_1.clone()), Some(player_2.clone()), None]);
    let stacking_error = cell_has_two_players
        .stack(&testing::spawn_player())
        .unwrap()
        .stack(&testing::spawn_player());
    assert_eq!(stacking_error, Err(Error::ReachedPalletHeightLimit));
}

#[test]
fn unstack() {
    let position = testing::spawn_position();
    let player_1 = testing::spawn_player();
    let cell = Cell::new_occupied(&position, &player_1);
    let unstacked = cell.unstack();
    assert_eq!(unstacked, Ok(Cell {
        position: position.clone(),
        pallet: Pallet { players: [None;PALLET_HEIGHT_LIMIT] },
    }));
    let empty_cell = unstacked.unwrap();
    let cannot_unstack = empty_cell.unstack();
    assert_eq!(cannot_unstack, Err(Error::PalletIsEmpty));
}
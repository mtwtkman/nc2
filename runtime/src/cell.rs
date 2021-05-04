use crate::player::Player;
use crate::result::{Error, Result};

pub(crate) const PALLET_HEIGHT_LIMIT: usize = 3;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct Cell {
    pub(crate) pallet: [Option<Player>; PALLET_HEIGHT_LIMIT],
}

impl Cell {
    fn height(&self) -> usize {
        self.pallet.iter().filter(|x| x.is_some()).count()
    }

    fn owner(&self) -> Option<Player> {
        if self.is_empty() {
            None
        } else {
            self.pallet[self.height() - 1].clone()
        }
    }

    fn is_empty(&self) -> bool {
        self.height() == 0
    }

    pub(crate) fn new_empty() -> Self {
        Self {
            pallet: [None; PALLET_HEIGHT_LIMIT],
        }
    }

    pub(crate) fn new_occupied(player: Player) -> Self {
        Self {
            pallet: [Some(player), None, None],
        }
    }

    fn stack(&self, player: &Player) -> Result<Self> {
        let height = self.height();
        if height == PALLET_HEIGHT_LIMIT {
            Err(Error::ReachedPalletHeightLimit)
        } else {
            let owner = self.owner();
            if owner == Some(player.clone()) {
                Err(Error::AlreadyOccupied(player.clone()))
            } else {
                let mut players = self.pallet.clone();
                players[height] = Some(player.clone());
                Ok(Self { pallet: players })
            }
        }
    }

    fn unstack(&self) -> Result<Self> {
        if self.is_empty() {
            Err(Error::PalletIsEmpty)
        } else {
            let mut players = self.pallet.clone();
            players[self.height() - 1] = None;
            Ok(Self { pallet: players })
        }
    }
}

#[cfg(test)]
fn spawn_player() -> Player {
    Player::new()
}

#[test]
fn new_occupied() {
    let player = spawn_player();
    let cell = Cell::new_occupied(player.clone());
    assert_eq!(
        cell,
        Cell { pallet:  [Some(player.clone()), None, None] },
    );
    assert_eq!(cell.owner(), Some(player.clone()));
    assert_eq!(cell.height(), 1);
    assert!(!cell.is_empty());
}

#[test]
fn new_empty() {
    let cell = Cell::new_empty();
    assert_eq!(
        cell,
        Cell {
            pallet: [None; PALLET_HEIGHT_LIMIT],
        },
    );
    assert!(cell.is_empty());
    assert_eq!(cell.owner(), None);
}

#[test]
fn stack() {
    let player_1 = spawn_player();
    let cell = Cell::new_empty();
    let first_stacked = cell.stack(&player_1);
    assert!(first_stacked.is_ok());
    let cell_has_one_player = first_stacked.unwrap();
    let over_stacking_cell = cell_has_one_player.clone();
    assert_eq!(
        over_stacking_cell.stack(&player_1),
        Err(Error::AlreadyOccupied(player_1.clone())),
    );
    assert_eq!(cell_has_one_player.height(), 1);
    assert_eq!(cell_has_one_player.owner(), Some(player_1.clone()));
    let player_2 = spawn_player();
    let second_stacked = cell_has_one_player.stack(&player_2);
    assert!(second_stacked.is_ok());
    let cell_has_two_players = second_stacked.unwrap();
    assert_eq!(
        &cell_has_two_players.pallet,
        &[Some(player_1.clone()), Some(player_2.clone()), None]
    );
    let stacking_error = cell_has_two_players
        .stack(&spawn_player())
        .unwrap()
        .stack(&spawn_player());
    assert_eq!(stacking_error, Err(Error::ReachedPalletHeightLimit));
}

#[test]
fn unstack() {
    let player_1 = spawn_player();
    let cell = Cell::new_occupied(player_1.clone());
    let unstacked = cell.unstack();
    assert_eq!(
        unstacked,
        Ok(Cell { pallet: [None; PALLET_HEIGHT_LIMIT] }),
    );
    let empty_cell = unstacked.unwrap();
    let cannot_unstack = empty_cell.unstack();
    assert_eq!(cannot_unstack, Err(Error::PalletIsEmpty));
}
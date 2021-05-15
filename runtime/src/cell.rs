use crate::{
    player::Player,
    result::{Error, Result},
};

pub(crate) const PALLET_HEIGHT_LIMIT: usize = 3;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct MigratedCellPair {
    pub(crate) from: Cell,
    pub(crate) to: Cell,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub(crate) struct Cell {
    pub(crate) pallet: [Option<Player>; PALLET_HEIGHT_LIMIT],
}

impl Cell {
    fn height(&self) -> usize {
        self.pallet.iter().filter(|x| x.is_some()).count()
    }

    pub(crate) fn owner(&self) -> Option<Player> {
        if self.is_empty() {
            None
        } else {
            self.pallet[self.height() - 1].clone()
        }
    }

    pub(crate) fn is_same_owner(&self, other: &Cell) -> bool {
        self.owner() == other.owner()
    }

    fn is_empty(&self) -> bool {
        self.height() == 0
    }

    pub(crate) fn is_fullfilled(&self) -> bool {
        self.height() == PALLET_HEIGHT_LIMIT
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

    pub(crate) fn stack(&self, player: &Player) -> Result<Self> {
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

    pub(crate) fn unstack(&self) -> Result<Self> {
        if self.is_empty() {
            Err(Error::CellIsEmpty)
        } else {
            let mut players = self.pallet.clone();
            players[self.height() - 1] = None;
            Ok(Self { pallet: players })
        }
    }

    pub(crate) fn migrate(&self, other: &Cell) -> Result<MigratedCellPair> {
        if self.is_empty() {
            return Err(Error::CellIsEmpty);
        } else if other.is_fullfilled() {
            return Err(Error::CellIsFullfilled);
        }
        let owner = self.owner();
        let destination_owner = other.owner();
        if owner == destination_owner {
            return Err(Error::AlreadyOccupied(destination_owner.unwrap()));
        }
        let from = self.unstack()?;
        let to = other.stack(&owner.unwrap())?;
        Ok(MigratedCellPair { from, to })
    }
}

#[test]
fn new_occupied() {
    let player = Player::new();
    let cell = Cell::new_occupied(player.clone());
    assert_eq!(
        cell,
        Cell {
            pallet: [Some(player.clone()), None, None]
        },
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
    let player_1 = Player::new();
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
    let player_2 = Player::new();
    let second_stacked = cell_has_one_player.stack(&player_2);
    assert!(second_stacked.is_ok());
    let cell_has_two_players = second_stacked.unwrap();
    assert_eq!(
        &cell_has_two_players.pallet,
        &[Some(player_1.clone()), Some(player_2.clone()), None]
    );
    let stacking_error = cell_has_two_players
        .stack(&Player::new())
        .unwrap()
        .stack(&Player::new());
    assert_eq!(stacking_error, Err(Error::ReachedPalletHeightLimit));
}

#[test]
fn unstack() {
    let player_1 = Player::new();
    let cell = Cell::new_occupied(player_1.clone());
    let unstacked = cell.unstack();
    assert_eq!(
        unstacked,
        Ok(Cell {
            pallet: [None; PALLET_HEIGHT_LIMIT]
        }),
    );
    let empty_cell = unstacked.unwrap();
    let cannot_unstack = empty_cell.unstack();
    assert_eq!(cannot_unstack, Err(Error::CellIsEmpty));
}

#[test]
fn is_reached_stacking_limit() {
    let player_a = Player::new();
    let player_b = Player::new();
    let cell = Cell::new_occupied(player_a.clone());
    assert!(cell
        .stack(&player_b)
        .unwrap()
        .stack(&player_a)
        .unwrap()
        .is_fullfilled())
}

#[cfg(test)]
mod cell_migrate_spec {
    use super::{Cell, MigratedCellPair, Player};
    use crate::result::Error;
    #[test]
    fn migrate() {
        let player = Player::new();
        let cell = Cell::new_occupied(player.clone());
        let other = Cell::new_empty();
        let migrated = cell.migrate(&other);
        let expected = MigratedCellPair {
            from: cell.clone().unstack().unwrap(),
            to: other.clone().stack(&player).unwrap(),
        };
        assert_eq!(migrated, Ok(expected));
    }

    #[test]
    fn empty_cell_cannot_migrate() {
        let cell = Cell::new_empty();
        let other = Cell::new_empty();
        assert_eq!(cell.migrate(&other), Err(Error::CellIsEmpty));
    }
    #[test]
    fn fullfilled_cell_cannot_migrate() {
        let player_1 = Player::new();
        let player_2 = Player::new();
        let cell = Cell::new_occupied(player_1.clone());
        let fullfilled = cell.stack(&player_2).unwrap().stack(&player_1).unwrap();
        assert_eq!(cell.migrate(&fullfilled), Err(Error::CellIsFullfilled));
    }

    #[test]
    fn already_occupied_cell_cannot_migrate() {
        let player_1 = Player::new();
        let cell = Cell::new_occupied(player_1.clone());
        let other = Cell::new_occupied(player_1.clone());
        assert_eq!(
            cell.migrate(&other),
            Err(Error::AlreadyOccupied(player_1.clone()))
        );
    }
}

use crate::{
    player::Player,
    result::{Error, Result},
};

pub(crate) const PALLET_HEIGHT_LIMIT: usize = 3;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Cell {
    pub(crate) pallet: [Option<Player>; PALLET_HEIGHT_LIMIT],
}

impl Cell {
    fn height(&self) -> usize {
        self.pallet.iter().filter(|x| x.is_some()).count()
    }

    pub fn owner(&self) -> Option<Player> {
        if self.is_empty() {
            None
        } else {
            self.pallet[self.height() - 1].clone()
        }
    }

    pub(crate) fn is_same_owner(&self, other: &Cell) -> bool {
        match (self.owner(), other.owner()) {
            (Some(me), Some(opponent)) => me == opponent,
            _ => false,
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
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
                let mut players = self.pallet;
                players[height] = Some(player.clone());
                Ok(Self { pallet: players })
            }
        }
    }

    pub(crate) fn unstack(&self) -> Result<Self> {
        if self.is_empty() {
            Err(Error::CellIsEmpty)
        } else {
            let mut players = self.pallet;
            players[self.height() - 1] = None;
            Ok(Self { pallet: players })
        }
    }
}

#[cfg(test)]
mod cell_spec {
    use super::{Cell, PALLET_HEIGHT_LIMIT};
    use crate::{player::Player, result::Error};

    #[test]
    fn new_occupied() {
        let player = Player::new(0);
        let cell = Cell::new_occupied(player);
        assert_eq!(
            cell,
            Cell {
                pallet: [Some(player), None, None]
            },
        );
        assert_eq!(cell.owner(), Some(player));
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
        let player_1 = Player::new(0);
        let cell = Cell::new_empty();
        let first_stacked = cell.stack(&player_1);
        assert!(first_stacked.is_ok());
        let cell_has_one_player = first_stacked.unwrap();
        let over_stacking_cell = cell_has_one_player;
        assert_eq!(
            over_stacking_cell.stack(&player_1),
            Err(Error::AlreadyOccupied(player_1)),
        );
        assert_eq!(cell_has_one_player.height(), 1);
        assert_eq!(cell_has_one_player.owner(), Some(player_1));
        let player_2 = Player::new(1);
        let second_stacked = cell_has_one_player.stack(&player_2);
        assert!(second_stacked.is_ok());
        let cell_has_two_players = second_stacked.unwrap();
        assert_eq!(
            &cell_has_two_players.pallet,
            &[Some(player_1), Some(player_2), None]
        );
        let stacking_error = cell_has_two_players
            .stack(&Player::new(2))
            .unwrap()
            .stack(&Player::new(3));
        assert_eq!(stacking_error, Err(Error::ReachedPalletHeightLimit));
    }

    #[test]
    fn unstack() {
        let player_1 = Player::new(0);
        let cell = Cell::new_occupied(player_1);
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
        let player_a = Player::new(0);
        let player_b = Player::new(1);
        let cell = Cell::new_occupied(player_a);
        assert!(cell
            .stack(&player_b)
            .unwrap()
            .stack(&player_a)
            .unwrap()
            .is_fullfilled())
    }

    #[cfg(test)]
    mod is_same_owner_spec {
        use super::super::Cell;
        use crate::player::Player;

        #[test]
        fn empty_against_empty() {
            let empty_cell_1 = Cell::new_empty();
            let empty_cell_2 = Cell::new_empty();
            assert!(!empty_cell_1.is_same_owner(&empty_cell_2));
        }

        #[test]
        fn different_owner() {
            let player_a = Player::new(0);
            let player_b = Player::new(1);
            let cell_a = Cell::new_occupied(player_a);
            let cell_b = Cell::new_occupied(player_b);
            assert!(!cell_a.is_same_owner(&cell_b));
        }

        #[test]
        fn same_owner() {
            let player = Player::new(0);
            let cell_1 = Cell::new_occupied(player);
            let cell_2 = Cell::new_occupied(player);
            assert!(cell_1.is_same_owner(&cell_2));
        }

        #[test]
        fn owned_against_empty() {
            let player = Player::new(0);
            let owned_cell = Cell::new_occupied(player);
            let empty_cell = Cell::new_empty();
            assert!(!owned_cell.is_same_owner(&empty_cell));
        }
    }
}

use std::cmp::Ordering;

use crate::result::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub(crate) enum Row {
    Top,
    MiddleFirst,
    MiddleSecond,
    MiddleThird,
    MiddleFourth,
    Bottom,
}

impl Row {
    fn is_top(&self) -> bool {
        self == &Self::Top
    }

    fn is_bottom(&self) -> bool {
        self == &Self::Bottom
    }

    fn is_middle(&self) -> bool {
        match self {
            Self::Top | Self::Bottom => false,
            _ => true,
        }
    }

    fn above(&self) -> Result<Self> {
        match self {
            Self::Top => Err(Error::ReachedTop),
            Self::MiddleFirst => Ok(Self::Top),
            Self::MiddleSecond => Ok(Self::MiddleFirst),
            Self::MiddleThird => Ok(Self::MiddleSecond),
            Self::MiddleFourth => Ok(Self::MiddleThird),
            Self::Bottom => Ok(Self::MiddleFourth),
        }
    }

    fn below(&self) -> Result<Self> {
        match self {
            Self::Top => Ok(Self::MiddleFirst),
            Self::MiddleFirst => Ok(Self::MiddleSecond),
            Self::MiddleSecond => Ok(Self::MiddleThird),
            Self::MiddleThird => Ok(Self::MiddleFourth),
            Self::MiddleFourth => Ok(Self::Bottom),
            Self::Bottom => Err(Error::ReachedBottom),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub(crate) enum Column {
    LeftEdge,
    MiddleFirst,
    MiddleSecond,
    MiddleThird,
    RightEdge,
}
impl Column {
    fn is_left_edge(&self) -> bool {
        self == &Self::LeftEdge
    }

    fn is_right_edge(&self) -> bool {
        self == &Self::RightEdge
    }

    fn is_middle(&self) -> bool {
        match *self {
            Self::LeftEdge | Self::RightEdge => false,
            _ => true,
        }
    }

    fn righthand(&self) -> Result<Self> {
        match *self {
            Self::LeftEdge => Ok(Self::MiddleFirst),
            Self::MiddleFirst => Ok(Self::MiddleSecond),
            Self::MiddleSecond => Ok(Self::MiddleThird),
            Self::MiddleThird => Ok(Self::RightEdge),
            Self::RightEdge => Err(Error::ReachedRightEdge),
        }
    }

    fn lefthand(&self) -> Result<Self> {
        match *self {
            Self::LeftEdge => Err(Error::ReachedLeftEdge),
            Self::MiddleFirst => Ok(Self::LeftEdge),
            Self::MiddleSecond => Ok(Self::MiddleFirst),
            Self::MiddleThird => Ok(Self::MiddleSecond),
            Self::RightEdge => Ok(Self::MiddleThird),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, PartialOrd, Ord, Copy)]
pub(crate) struct Position {
    x: Column,
    y: Row,
}

impl Position {
    pub(crate) fn new(x: Column, y: Row) -> Self {
        Self { x, y }
    }

    pub(self) fn vertical(&self, y: Result<Row>) -> Result<Self> {
        y.map(|y| Self {
            x: self.x.clone(),
            y,
        })
    }

    pub(crate) fn above(&self) -> Result<Self> {
        self.vertical(self.y.above())
    }

    pub(crate) fn below(&self) -> Result<Self> {
        self.vertical(self.y.below())
    }

    pub(self) fn horizon(&self, x: Result<Column>) -> Result<Self> {
        x.map(|x| Self {
            x,
            y: self.y.clone(),
        })
    }

    pub(crate) fn righthand(&self) -> Result<Self> {
        self.horizon(self.x.righthand())
    }

    pub(crate) fn lefthand(&self) -> Result<Self> {
        self.horizon(self.x.lefthand())
    }

    pub(crate) fn above_righthand(&self) -> Result<Self> {
        self.above().and_then(|p| p.righthand())
    }

    pub(crate) fn above_lefthand(&self) -> Result<Self> {
        self.above().and_then(|p| p.lefthand())
    }

    pub(crate) fn below_righthand(&self) -> Result<Self> {
        self.below().and_then(|p| p.righthand())
    }

    pub(crate) fn below_lefthand(&self) -> Result<Self> {
        self.below().and_then(|p| p.lefthand())
    }

    pub(crate) fn is_top(&self) -> bool {
        self.y.is_top()
    }

    pub(crate) fn is_bottom(&self) -> bool {
        self.y.is_bottom()
    }

    pub(crate) fn is_left_edge_top(&self) -> bool {
        self.x.is_left_edge() && self.is_top()
    }

    pub(crate) fn is_left_edge_bottom(&self) -> bool {
        self.x.is_left_edge() && self.is_bottom()
    }

    pub(crate) fn is_right_edge_top(&self) -> bool {
        self.x.is_right_edge() && self.is_top()
    }

    pub(crate) fn is_right_edge_bottom(&self) -> bool {
        self.x.is_right_edge() && self.is_bottom()
    }

    pub(crate) fn is_right_edge_middle_row(&self) -> bool {
        self.x.is_right_edge() && self.y.is_middle()
    }

    pub(crate) fn is_left_edge_middle_row(&self) -> bool {
        self.x.is_left_edge() && self.y.is_middle()
    }

    pub(crate) fn is_middle_column_top(&self) -> bool {
        self.x.is_middle() && self.is_top()
    }

    pub(crate) fn is_middle_column_bottom(&self) -> bool {
        self.x.is_middle() && self.is_bottom()
    }
}

#[test]
fn left_edge_top_position() {
    let left_edge = Column::LeftEdge;
    let top_edge = Row::Top;
    let position = Position::new(left_edge, top_edge);
    assert!(position.is_left_edge_top());
}

#[test]
fn left_edge_bottom_position() {
    let left_edge = Column::LeftEdge;
    let bottom_edge = Row::Bottom;
    let position = Position::new(left_edge, bottom_edge);
    assert!(position.is_left_edge_bottom());
}

#[test]
fn right_edge_top_position() {
    let right_edge = Column::RightEdge;
    let top_edge = Row::Top;
    let position = Position::new(right_edge, top_edge);
    assert!(position.is_right_edge_top());
}

#[test]
fn right_edge_bottom_position() {
    let right_edge = Column::RightEdge;
    let bottom_edge = Row::Bottom;
    let position = Position::new(right_edge, bottom_edge);
    assert!(position.is_right_edge_bottom());
}

#[test]
fn left_edge_middle_row() {
    let left_edge = Column::LeftEdge;
    for row in [
        Row::MiddleFirst,
        Row::MiddleSecond,
        Row::MiddleThird,
        Row::MiddleFourth,
    ]
    .iter()
    {
        let position = Position::new(left_edge.clone(), row.to_owned());
        assert!(position.is_left_edge_middle_row());
    }
}

#[test]
fn right_edge_middle_row() {
    let right_edge = Column::RightEdge;
    for row in [
        Row::MiddleFirst,
        Row::MiddleSecond,
        Row::MiddleThird,
        Row::MiddleFourth,
    ]
    .iter()
    {
        let position = Position::new(right_edge.clone(), row.to_owned());
        assert!(position.is_right_edge_middle_row());
    }
}

#[test]
fn middle_column_top() {
    let top = Row::Top;
    for column in [
        Column::MiddleFirst,
        Column::MiddleSecond,
        Column::MiddleThird,
    ]
    .iter()
    {
        let position = Position::new(column.to_owned(), top.clone());
        assert!(position.is_middle_column_top());
    }
}

#[test]
fn middle_column_bottom() {
    let bottom = Row::Bottom;
    for column in [
        Column::MiddleFirst,
        Column::MiddleSecond,
        Column::MiddleThird,
    ]
    .iter()
    {
        let position = Position::new(column.to_owned(), bottom.clone());
        assert!(position.is_middle_column_bottom());
    }
}

#[test]
fn active_righthand() {
    let x = Column::LeftEdge;
    let y = Row::Top;
    let position = Position::new(x, y.clone());
    let moved_to_middle_first_column = position.righthand();
    assert_eq!(
        &moved_to_middle_first_column,
        &Ok(Position {
            x: Column::MiddleFirst,
            y: y.clone(),
        })
    );
    let moved_to_middle_second_column = moved_to_middle_first_column.unwrap().righthand();
    assert_eq!(
        &moved_to_middle_second_column,
        &Ok(Position {
            x: Column::MiddleSecond,
            y: y.clone(),
        })
    );
    let moved_to_middle_third_column = moved_to_middle_second_column.unwrap().righthand();
    assert_eq!(
        &moved_to_middle_third_column,
        &Ok(Position {
            x: Column::MiddleThird,
            y: y.clone(),
        })
    );
    let moved_to_right_edge = moved_to_middle_third_column.unwrap().righthand();
    assert_eq!(
        &moved_to_right_edge,
        &Ok(Position {
            x: Column::RightEdge,
            y: y.clone(),
        })
    );
    let cannot_move_to_right = moved_to_right_edge.unwrap().righthand();
    assert_eq!(&cannot_move_to_right, &Err(Error::ReachedRightEdge));
}

#[test]
fn active_lefthand() {
    let x = Column::RightEdge;
    let y = Row::Top;
    let position = Position::new(x, y.clone());
    let moved_to_middle_third_column = position.lefthand();
    assert_eq!(
        &moved_to_middle_third_column,
        &Ok(Position {
            x: Column::MiddleThird,
            y: y.clone(),
        })
    );
    let moved_to_middle_second_column = moved_to_middle_third_column.unwrap().lefthand();
    assert_eq!(
        &moved_to_middle_second_column,
        &Ok(Position {
            x: Column::MiddleSecond,
            y: y.clone(),
        })
    );
    let moved_to_middle_first_column = moved_to_middle_second_column.unwrap().lefthand();
    assert_eq!(
        &moved_to_middle_first_column,
        &Ok(Position {
            x: Column::MiddleFirst,
            y: y.clone(),
        })
    );
    let moved_to_left_edge = moved_to_middle_first_column.unwrap().lefthand();
    assert_eq!(
        &moved_to_left_edge,
        &Ok(Position {
            x: Column::LeftEdge,
            y: y.clone(),
        })
    );
    let cannot_move_to_left = moved_to_left_edge.unwrap().lefthand();
    assert_eq!(&cannot_move_to_left, &Err(Error::ReachedLeftEdge));
}

#[test]
fn active_above() {
    let x = Column::LeftEdge;
    let y = Row::Bottom;
    let position = Position::new(x.clone(), y);
    let moved_to_middle_fourth_row = position.above();
    assert_eq!(
        &moved_to_middle_fourth_row,
        &Ok(Position {
            x: x.clone(),
            y: Row::MiddleFourth,
        })
    );
    let moved_to_middle_third_row = moved_to_middle_fourth_row.unwrap().above();
    assert_eq!(
        &moved_to_middle_third_row,
        &Ok(Position {
            x: x.clone(),
            y: Row::MiddleThird,
        })
    );
    let moved_to_middle_second_row = moved_to_middle_third_row.unwrap().above();
    assert_eq!(
        &moved_to_middle_second_row,
        &Ok(Position {
            x: x.clone(),
            y: Row::MiddleSecond,
        })
    );
    let moved_to_middle_first_row = moved_to_middle_second_row.unwrap().above();
    assert_eq!(
        &moved_to_middle_first_row,
        &Ok(Position {
            x: x.clone(),
            y: Row::MiddleFirst,
        })
    );
    let moved_to_top = moved_to_middle_first_row.unwrap().above();
    assert_eq!(
        &moved_to_top,
        &Ok(Position {
            x: x.clone(),
            y: Row::Top,
        })
    );
    let cannot_move_to_top = moved_to_top.unwrap().above();
    assert_eq!(&cannot_move_to_top, &Err(Error::ReachedTop));
}

#[test]
fn active_below() {
    let x = Column::LeftEdge;
    let y = Row::Top;
    let position = Position::new(x.clone(), y);
    let moved_to_middle_first_row = position.below();
    assert_eq!(
        &moved_to_middle_first_row,
        &Ok(Position {
            x: x.clone(),
            y: Row::MiddleFirst,
        })
    );
    let moved_to_middle_second_row = moved_to_middle_first_row.unwrap().below();
    assert_eq!(
        &moved_to_middle_second_row,
        &Ok(Position {
            x: x.clone(),
            y: Row::MiddleSecond,
        })
    );
    let moved_to_middle_third_row = moved_to_middle_second_row.unwrap().below();
    assert_eq!(
        &moved_to_middle_third_row,
        &Ok(Position {
            x: x.clone(),
            y: Row::MiddleThird,
        })
    );
    let moved_to_middle_fourth_row = moved_to_middle_third_row.unwrap().below();
    assert_eq!(
        &moved_to_middle_fourth_row,
        &Ok(Position {
            x: x.clone(),
            y: Row::MiddleFourth,
        })
    );
    let moved_to_bottom = moved_to_middle_fourth_row.unwrap().below();
    assert_eq!(
        &moved_to_bottom,
        &Ok(Position {
            x: x.clone(),
            y: Row::Bottom,
        })
    );
    let cannot_move_to_bottom = moved_to_bottom.unwrap().below();
    assert_eq!(&cannot_move_to_bottom, &Err(Error::ReachedBottom));
}

#[test]
fn active_above_righthand() {
    let position = Position::new(Column::MiddleThird, Row::MiddleFirst);
    let moved_to_right_top_corner = position.above_righthand();
    assert_eq!(
        &moved_to_right_top_corner,
        &Ok(Position {
            x: Column::RightEdge,
            y: Row::Top,
        }),
    );
    let right_top_corner = moved_to_right_top_corner.unwrap();
    assert_eq!(right_top_corner.above_righthand(), Err(Error::ReachedTop),);
    assert_eq!(
        Position::new(Column::RightEdge, Row::MiddleFirst).above_righthand(),
        Err(Error::ReachedRightEdge),
    );
}

#[test]
fn active_above_lefthand() {
    let position = Position::new(Column::MiddleFirst, Row::MiddleFirst);
    let moved_to_left_top_corner = position.above_lefthand();
    assert_eq!(
        &moved_to_left_top_corner,
        &Ok(Position {
            x: Column::LeftEdge,
            y: Row::Top,
        }),
    );
    let left_top_corner = moved_to_left_top_corner.unwrap();
    assert_eq!(left_top_corner.above_lefthand(), Err(Error::ReachedTop),);
    assert_eq!(
        Position::new(Column::LeftEdge, Row::MiddleFirst).above_lefthand(),
        Err(Error::ReachedLeftEdge),
    );
}

#[test]
fn active_below_righthand() {
    let position = Position::new(Column::MiddleThird, Row::MiddleFourth);
    let moved_to_right_bottom_corner = position.below_righthand();
    assert_eq!(
        &moved_to_right_bottom_corner,
        &Ok(Position {
            x: Column::RightEdge,
            y: Row::Bottom,
        }),
    );
    let right_bottom_corner = moved_to_right_bottom_corner.unwrap();
    assert_eq!(
        right_bottom_corner.below_righthand(),
        Err(Error::ReachedBottom),
    );
    assert_eq!(
        Position::new(Column::RightEdge, Row::MiddleFourth).below_righthand(),
        Err(Error::ReachedRightEdge),
    );
}

#[test]
fn active_below_lefthand() {
    let position = Position::new(Column::MiddleFirst, Row::MiddleFourth);
    let moved_to_left_bottom_corner = position.below_lefthand();
    assert_eq!(
        &moved_to_left_bottom_corner,
        &Ok(Position {
            x: Column::LeftEdge,
            y: Row::Bottom,
        }),
    );
    let left_bottom_corner = moved_to_left_bottom_corner.unwrap();
    assert_eq!(
        left_bottom_corner.below_lefthand(),
        Err(Error::ReachedBottom),
    );
    assert_eq!(
        Position::new(Column::LeftEdge, Row::MiddleFourth).below_lefthand(),
        Err(Error::ReachedLeftEdge),
    );
}

#[test]
fn row_order() {
    assert!(Row::Top < Row::MiddleFirst);
    assert!(Row::MiddleFirst < Row::MiddleSecond);
    assert!(Row::MiddleSecond < Row::MiddleThird);
    assert!(Row::MiddleThird < Row::MiddleFourth);
    assert!(Row::MiddleFourth < Row::Bottom);
}

#[test]
fn column_order() {
    assert!(Column::LeftEdge < Column::MiddleFirst);
    assert!(Column::MiddleFirst < Column::MiddleSecond);
    assert!(Column::MiddleSecond < Column::MiddleThird);
    assert!(Column::MiddleThird < Column::RightEdge);
}

#[test]
fn position_order() {
    assert!(
        Position::new(Column::LeftEdge, Row::Top)
            < Position::new(Column::LeftEdge, Row::MiddleFirst)
    );
    assert!(
        Position::new(Column::LeftEdge, Row::Top) < Position::new(Column::MiddleFirst, Row::Top)
    );
    assert!(
        Position::new(Column::LeftEdge, Row::Top)
            < Position::new(Column::MiddleFirst, Row::MiddleFirst)
    );
}

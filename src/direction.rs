#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }

    // pub fn clockwise(&self) -> Self {
    //     match self {
    //         Self::Up => Self::Right,
    //         Self::Right => Self::Down,
    //         Self::Down => Self::Left,
    //         Self::Left => Self::Up,
    //     }
    // }
    //
    // pub fn anticlockwise(&self) -> Self {
    //     match self {
    //         Self::Up => Self::Left,
    //         Self::Right => Self::Up,
    //         Self::Down => Self::Right,
    //         Self::Left => Self::Down,
    //     }
    // }

    pub fn delta(&self) -> (i16, i16) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }
}
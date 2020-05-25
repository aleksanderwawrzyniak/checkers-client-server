use super::MATRIX_SIZE;

/// Wrapper on (x, y) coordinates
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Position(pub (usize, usize));

impl Position {
    pub fn inner(&self) -> (usize, usize) {
        self.0
    }

    pub fn possible_moves<O>(&self, previous: O) -> Vec<Self>
    where
        O: Into<Option<Position>>,
    {
        let (x, y) = self.inner();
        let (x, y) = (x as isize, y as isize);
        let previous = previous
            .into()
            .unwrap_or(Position((MATRIX_SIZE + 1, MATRIX_SIZE + 1)));
        [
            (x - 1, y - 1).try_convert(),
            (x - 1, y + 1).try_convert(),
            (x + 1, y - 1).try_convert(),
            (x + 1, y + 1).try_convert(),
        ]
        .iter()
        .filter_map(|option| *option)
        .filter(|pos| &previous != pos)
        .collect()
    }
}

pub trait TryConvert<T> {
    fn try_convert(self) -> Option<T>;
}

impl TryConvert<usize> for isize {
    fn try_convert(self) -> Option<usize> {
        if self < 0 || self > MATRIX_SIZE as isize {
            None
        } else {
            Some(self as usize)
        }
    }
}

impl TryConvert<Position> for (isize, isize) {
    fn try_convert(self) -> Option<Position> {
        Some(Position((self.0.try_convert()?, self.1.try_convert()?)))
    }
}

impl Into<(usize, usize)> for Position {
    fn into(self) -> (usize, usize) {
        self.inner()
    }
}

impl From<(usize, usize)> for Position {
    fn from(position: (usize, usize)) -> Self {
        Self(position)
    }
}

impl Into<Option<(usize, usize)>> for Position {
    fn into(self) -> Option<(usize, usize)> {
        Some(self.into())
    }
}

#[test]
fn test_let_values() {
    let pos = Position((3, 5));
    let (x, y) = pos.inner();
    assert_eq!(x, 3usize);
    assert_eq!(y, 5usize);
}

#[test]
fn possible_moves() {
    let pos = Position((1, 1));
    let moves = pos.possible_moves(None);
    assert_eq!(moves.len(), 4);
}

#[test]
fn possible_moves_boundary() {
    let pos = Position((1, 0));
    let moves = pos.possible_moves(None);
    assert_eq!(moves.len(), 2);

    let pos = Position((0, 0));
    let moves = pos.possible_moves(None);
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0], Position((1, 1)));
}

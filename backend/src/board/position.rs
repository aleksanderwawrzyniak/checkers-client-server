use super::MATRIX_SIZE;

/// Wrapper on (x, y) coordinates
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Position(pub (usize, usize));

impl Position {
    pub fn inner(&self) -> (usize, usize) {
        self.0
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


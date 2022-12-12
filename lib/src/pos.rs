use std::ops::{Add, Sub};

use crate::vector::Vector;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct Pos(pub isize, pub isize);

impl Vector for Pos {
    type Output = Pos;

    fn len(&self) -> f64 {
        ((self.0.pow(2) + self.1.pow(2)) as f64).sqrt().abs()
    }

    fn flatten(&self) -> Self::Output {
        let (mut x, mut y) = (self.0, self.1);
        if x != 0 {
            x /= x.abs();
        }
        if y != 0 {
            y /= y.abs();
        }
        Pos(x, y)
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T> From<(T, T)> for Pos
where
    T: Into<isize>,
{
    fn from((x, y): (T, T)) -> Self {
        Self(x.into() as isize, y.into() as isize)
    }
}

use crate::pos::Pos;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(v: &str) -> Self {
        match v {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("invalid direction"),
        }
    }
}

impl From<Direction> for Pos {
    fn from(v: Direction) -> Self {
        match v {
            Direction::Up => Pos(0, 1),
            Direction::Down => Pos(0, -1),
            Direction::Left => Pos(-1, 0),
            Direction::Right => Pos(1, 0),
        }
    }
}

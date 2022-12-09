use std::{
    cell::RefCell,
    collections::HashSet,
    f64::consts::SQRT_2,
    ops::{Add, Sub},
};

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Pos(isize, isize);

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

impl Pos {
    fn len(&self) -> f64 {
        ((self.0.pow(2) + self.1.pow(2)) as f64).sqrt().abs()
    }

    fn flatten(&self) -> Pos {
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

#[derive(Clone, Copy, Debug)]
enum Direction {
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

#[derive(Clone, Debug)]
struct Move {
    direction: Direction,
    amount: usize,
}

impl From<&str> for Move {
    fn from(v: &str) -> Self {
        let (dir, am) = v.split_once(' ').unwrap();
        Self {
            direction: dir.into(),
            amount: am.parse().unwrap(),
        }
    }
}

impl Move {
    fn apply(&self, knots: &[RefCell<Pos>], visited: &RefCell<HashSet<Pos>>) {
        for _ in 0..self.amount {
            for (i, k) in knots.iter().enumerate() {
                if i == 0 {
                    let mut k = k.borrow_mut();
                    *k = *k + self.direction.into();
                } else {
                    let mut k = k.borrow_mut();
                    let kb = knots[i - 1].borrow();
                    if (*kb - *k).len() > SQRT_2 {
                        *k = *k + (*kb - *k).flatten();
                    }
                }
            }
            visited
                .borrow_mut()
                .insert(*(knots.last().unwrap().borrow()));
        }
    }
}

fn main() {
    let input: String = lib::read_input!();

    let moves: Vec<Move> = input.split('\n').map(|v| v.into()).collect();

    let knots = vec![RefCell::new(Pos(0, 0)); 2];
    let tail_visited: RefCell<HashSet<Pos>> = HashSet::new().into();
    tail_visited.borrow_mut().insert(Pos(0, 0));

    for mv in &moves {
        mv.apply(&knots, &tail_visited);
    }

    println!(
        "Part 1:\nThe tail visited {} spots",
        tail_visited.borrow().len()
    );

    let knots = vec![RefCell::new(Pos(0, 0)); 10];
    let tail_visited: RefCell<HashSet<Pos>> = HashSet::new().into();
    tail_visited.borrow_mut().insert(Pos(0, 0));

    for mv in &moves {
        mv.apply(&knots, &tail_visited);
    }

    println!(
        "Part 2:\nThe tail visited {} spots",
        tail_visited.borrow().len()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vec_len() {
        assert_eq!(Pos(0, 0).len(), 0.0);
        assert_eq!(Pos(0, 1).len(), 1.0);
        assert_eq!(Pos(1, 0).len(), 1.0);
        assert_eq!(Pos(1, 1).len(), SQRT_2);
        assert_eq!(Pos(1, 1).len(), SQRT_2);
        assert_eq!(Pos(-1, -1).len(), SQRT_2);
    }
}

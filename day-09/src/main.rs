use lib::{direction::Direction, pos::Pos, vector::Vector};
use std::{cell::RefCell, collections::HashSet, f64::consts::SQRT_2};

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

use lib::pos::Pos;
use ringbuffer::{AllocRingBuffer, RingBufferExt, RingBufferWrite};
use std::{
    cell::RefCell,
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

const WIDTH: isize = 7;

fn map_movement(c: char) -> Pos {
    match c {
        '>' => Pos(1, 0),
        '<' => Pos(-1, 0),
        _ => panic!("invalid movement"),
    }
}

#[derive(Debug, Clone)]
struct Rock(Vec<RefCell<Pos>>);

impl From<Vec<Pos>> for Rock {
    fn from(poss: Vec<Pos>) -> Self {
        Self(poss.iter().map(|&p| RefCell::new(p)).collect())
    }
}

impl Hash for Rock {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for v in &self.0 {
            let v = v.borrow();
            v.hash(state);
        }
    }
}

impl Rock {
    fn mv(&mut self, v: Pos) -> bool {
        for p in &self.0 {
            let new_p = *(p.borrow()) + v;
            // Check width bounds. If exceeds,
            // don't move the rock.
            if new_p.0 < 0 || new_p.0 >= WIDTH {
                return false;
            }
        }

        for p in &self.0 {
            *(p.borrow_mut()) += v;
        }

        true
    }

    fn covers(&self, other: &Rock) -> bool {
        for ps in &self.0 {
            for po in &other.0 {
                if *ps.borrow() == *po.borrow() {
                    return true;
                }
            }
        }

        false
    }

    fn top(&self) -> Pos {
        *self.0.iter().max_by_key(|p| p.borrow().1).unwrap().borrow()
    }
}

fn hash(h_count: usize, r_count: usize, rocks: &[Rock]) -> u64 {
    let mut max_ys = vec![-20; 7];
    for r in rocks {
        for p in &r.0 {
            let p = p.borrow();
            max_ys[p.0 as usize] = max_ys[p.0 as usize].max(p.1);
        }
    }

    let max = max_ys.iter().max().unwrap();
    max_ys = max_ys.iter().map(|y| y - max).collect();

    let mut hasher = DefaultHasher::new();
    max_ys.hash(&mut hasher);
    h_count.hash(&mut hasher);
    r_count.hash(&mut hasher);
    hasher.finish()
}

fn simulate(rocks_to_fall: isize, rocks: &[Rock], moves: &[Pos]) -> isize {
    const SPAWN_OFFSET: Pos = Pos(2, 3);
    let bottom_rock = Rock::from((0..WIDTH).map(|x| Pos(x, -1)).collect::<Vec<Pos>>());
    let mut offset = 0isize;
    let mut skip_hashing = false;

    let mut stale_rocks: AllocRingBuffer<Rock> = AllocRingBuffer::with_capacity(64);
    let mut h_count = 0usize;
    let mut hashes = HashMap::new();
    let mut i = -1isize;
    while i < rocks_to_fall {
        i += 1;
        let r_count = i as usize % rocks.len();
        let mut rock = rocks[r_count].clone();

        let top = stale_rocks
            .to_vec()
            .iter()
            .map(|r| r.top().1)
            .max()
            .unwrap_or(-1)
            + 1;

        // Move rock to spawn position
        rock.mv(SPAWN_OFFSET + Pos(0, top));

        'inner: for j in 0..usize::MAX {
            let mut new_rock = rock.clone();
            // Alternate between falling and pushing the
            // rock in the direction given by the input.
            if j % 2 == 0 {
                let mv = moves[h_count];
                h_count = (h_count + 1) % moves.len();
                // If the rock did not move, simply
                // continue with the next step.
                if !new_rock.mv(mv) {
                    continue 'inner;
                }
            } else {
                new_rock.mv(Pos(0, -1));
            }
            // Check if the "new" rock bumps into
            // an already stalled block.
            for stale_rock in stale_rocks.iter() {
                if stale_rock.covers(&new_rock) {
                    if j % 2 != 0 {
                        break 'inner;
                    }
                    continue 'inner;
                }
            }
            // If the "new" rock would bump into
            // the "bottom block" (= Floor), mark
            // it as stalled.
            if bottom_rock.covers(&new_rock) {
                break 'inner;
            }

            rock = new_rock;
        }

        stale_rocks.push(rock);

        if !skip_hashing {
            let top = stale_rocks
                .to_vec()
                .iter()
                .map(|r| r.top().1)
                .max()
                .unwrap();

            let hsh = hash(h_count, r_count, &stale_rocks.to_vec());
            if let Some((last_i, last_top)) = hashes.get(&hsh) {
                let skip = (rocks_to_fall - i) / (i - last_i);
                offset = skip * (top - last_top) + 2;
                i += skip * (i - last_i);
                skip_hashing = true;
            } else {
                hashes.insert(hsh, (i, top));
            }
        }
    }

    offset + stale_rocks.iter().map(|r| r.top().1).max().unwrap() - 2
}

fn main() {
    /*
    ####

    .#.
    ###
    .#.

    ..#
    ..#
    ###

    #
    #
    #
    #

    ##
    ##
    */
    let rocks = [
        Rock::from(vec![Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)]),
        Rock::from(vec![Pos(1, 0), Pos(0, 1), Pos(1, 1), Pos(2, 1), Pos(1, 2)]),
        Rock::from(vec![Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(2, 1), Pos(2, 2)]),
        Rock::from(vec![Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(0, 3)]),
        Rock::from(vec![Pos(0, 0), Pos(0, 1), Pos(1, 0), Pos(1, 1)]),
    ];

    let input: String = lib::read_input!();

    let moves: Vec<_> = input.chars().map(map_movement).collect();

    let height = simulate(2022, &rocks, &moves);
    println!("Part 1:\nThe height of the tower is {height}");

    let height = simulate(1000000000000, &rocks, &moves);
    println!("Part 2:\nThe height of the tower is {height}");
}

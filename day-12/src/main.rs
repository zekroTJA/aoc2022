use lib::pos::Pos;
use std::collections::HashSet;

struct Node {
    p: Pos,
    distance: usize,
}

impl Node {
    fn new(p: Pos, distance: usize) -> Self {
        Self { p, distance }
    }
}

fn get_position_of(c: char, grid: &[Vec<char>]) -> Option<Pos> {
    for (y, line) in grid.iter().enumerate() {
        for (x, &col) in line.iter().enumerate() {
            if col == c {
                return Some((x as isize, y as isize).into());
            }
        }
    }

    None
}

fn get_char_at(p: Pos, grid: &[Vec<char>]) -> char {
    grid[p.1 as usize][p.0 as usize]
}

fn get_height_at(p: Pos, grid: &[Vec<char>]) -> isize {
    let c = get_char_at(p, grid);
    match c {
        'S' => 'a' as isize,
        'E' => 'z' as isize,
        _ => c as isize,
    }
}

fn get_valid_neighbors(
    pos: Pos,
    height_check: impl Fn(isize, isize) -> bool,
    grid: &[Vec<char>],
) -> Vec<Pos> {
    let pos_height = get_height_at(pos, grid);
    let max_x = grid[0].len() as isize;
    let max_y = grid.len() as isize;
    [Pos(1, 0), Pos(0, 1), Pos(-1, 0), Pos(0, -1)]
        .iter()
        .map(|&p| pos + p)
        .filter(|p| !p.0.is_negative() && !p.1.is_negative() && p.0 < max_x && p.1 < max_y)
        .filter(|&p| height_check(get_height_at(p, grid), pos_height))
        .collect()
}

fn get_steps(
    start_pos: Pos,
    target: char,
    height_check: impl Fn(isize, isize) -> bool,
    grid: &[Vec<char>],
) -> Option<usize> {
    let mut visited: HashSet<Pos> = HashSet::new();
    visited.insert(start_pos);
    let mut check_next = vec![Node::new(start_pos, 0)];

    while let Some(c) = check_next.pop() {
        let neighbors = get_valid_neighbors(c.p, &height_check, grid);
        for n in neighbors {
            if visited.insert(n) {
                if get_char_at(n, grid) == target {
                    return Some(c.distance + 1);
                }
                check_next.insert(0, Node::new(n, c.distance + 1));
            }
        }
    }

    None
}

fn main() {
    let input: String = lib::read_input!();

    let grid: Vec<Vec<_>> = input.split('\n').map(|l| l.chars().collect()).collect();

    // Start at position of S and go until you hit 'E'.

    let curr_pos = get_position_of('S', &grid).expect("start position");
    let steps = get_steps(curr_pos, 'E', |to, from| to - from <= 1, &grid).expect("step count");

    println!("Part 1:\nThe shortest way takes {steps} steps");

    // Start at position of E and go until you hit an 'a'.

    let curr_pos = get_position_of('E', &grid).expect("start position");
    let steps = get_steps(curr_pos, 'a', |to, from| to - from >= -1, &grid).expect("step count");

    println!("Part 2:\nThe shortest way takes {steps} steps");
}

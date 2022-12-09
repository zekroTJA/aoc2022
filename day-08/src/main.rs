fn is_visible(grid: &[Vec<u8>], x: usize, y: usize) -> bool {
    let x_max = grid[0].len();
    let y_max = grid.len();

    let h = grid[y][x];

    !(0..x).any(|x1| grid[y][x1] >= h)
        || !(x + 1..x_max).any(|x1| grid[y][x1] >= h)
        || !(0..y).any(|y1| grid[y1][x] >= h)
        || !(y + 1..y_max).any(|y1| grid[y1][x] >= h)
}

fn view_distance(grid: &[Vec<u8>], x: usize, y: usize) -> usize {
    let x_max = grid[0].len();
    let y_max = grid.len();

    let h = grid[y][x];

    // There is definetly a more "beautiful" way to solve this,
    // but fuck it honestly, rust can crunch numbers fast enough.

    let left = (0..x)
        .rev()
        .enumerate()
        .find(|&(_, x1)| grid[y][x1] >= h)
        .map(|(i, _)| i)
        .unwrap_or(x - 1)
        + 1;

    let right = (x + 1..x_max)
        .enumerate()
        .find(|&(_, x1)| grid[y][x1] >= h)
        .map(|(i, _)| i)
        .unwrap_or(x_max - x - 2)
        + 1;

    let top = (0..y)
        .rev()
        .enumerate()
        .find(|&(_, y1)| grid[y1][x] >= h)
        .map(|(i, _)| i)
        .unwrap_or(y - 1)
        + 1;

    let bottom = (y + 1..y_max)
        .enumerate()
        .find(|&(_, y1)| grid[y1][x] >= h)
        .map(|(i, _)| i)
        .unwrap_or(y_max - y - 2)
        + 1;

    left * right * top * bottom
}

fn main() {
    let input: String = lib::read_input!();

    let grid: Vec<Vec<u8>> = input
        .split('\n')
        .map(|l| l.chars().map(|c| c as u8 - 48).collect())
        .collect();

    let perimeter = grid.len() * 2 + grid[0].len() * 2 - 4;
    let mut visible = perimeter;

    for x in 1..grid.len() - 1 {
        for y in 1..grid[0].len() - 1 {
            if is_visible(&grid, x, y) {
                visible += 1;
            }
        }
    }

    println!("Part 1:\nThe amount of visible trees is {}", visible);

    let visibility = (1..grid.len() - 1)
        .flat_map(|x| (1..grid[0].len() - 1).map(move |y| (x, y)))
        .map(|(x, y)| view_distance(&grid, x, y))
        .max()
        .expect("max visibility");

    println!("Part 2:\nThe maximum visibility score is {}", visibility);
}

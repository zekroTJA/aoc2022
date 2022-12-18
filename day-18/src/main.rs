use lib::pos3d::Pos3d;
use std::collections::HashSet;

const DIRECTIONS: [Pos3d; 6] = [
    Pos3d::new(1, 0, 0),
    Pos3d::new(0, 1, 0),
    Pos3d::new(0, 0, 1),
    Pos3d::new(-1, 0, 0),
    Pos3d::new(0, -1, 0),
    Pos3d::new(0, 0, -1),
];

fn count_free_sides(cube: Pos3d, cubes: &HashSet<Pos3d>) -> usize {
    DIRECTIONS
        .iter()
        .map(|&p| cube + p)
        .filter(|p| !cubes.contains(p))
        .count()
}

fn get_neighbors(cube: Pos3d) -> Vec<Pos3d> {
    DIRECTIONS.iter().map(|&p| cube + p).collect()
}

fn main() {
    let input: String = lib::read_input!();

    let cubes: HashSet<Pos3d> = input.split('\n').map(|v| v.into()).collect();

    let sum: usize = cubes.iter().map(|&c| count_free_sides(c, &cubes)).sum();

    println!("Part 1:\nThe sum of surface sides is {sum}");

    let min_x = cubes.iter().min_by_key(|p| p.x).unwrap().x - 1;
    let min_y = cubes.iter().min_by_key(|p| p.y).unwrap().y - 1;
    let min_z = cubes.iter().min_by_key(|p| p.z).unwrap().z - 1;

    let max_x = cubes.iter().max_by_key(|p| p.x).unwrap().x + 1;
    let max_y = cubes.iter().max_by_key(|p| p.y).unwrap().y + 1;
    let max_z = cubes.iter().max_by_key(|p| p.z).unwrap().z + 1;

    let mut queue = vec![Pos3d::new(min_x, min_y, min_z)];
    let mut air = HashSet::<Pos3d>::new();

    while let Some(c) = queue.pop() {
        let neighbors = get_neighbors(c);

        for n in neighbors {
            if n.x < min_x
                || n.y < min_y
                || n.z < min_z
                || n.x > max_x
                || n.y > max_y
                || n.z > max_z
                || cubes.contains(&n)
                || air.contains(&n)
            {
                continue;
            }

            air.insert(n);
            queue.insert(0, n);
        }
    }

    let sum: usize = air.iter().map(|&p| 6 - count_free_sides(p, &cubes)).sum();

    println!("Part 2:\nThe sum of surface sides not enclosed is {sum}");
}

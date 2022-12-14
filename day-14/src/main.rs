use lib::pos::Pos;

/// This solution is pretty much very unoptimized and may
/// take some time to execute. The is_blocked function
/// essentially iterates over each fallen piece of sand
/// as well as all pathes 3 times per simulation iteration
/// which is SUPER inefficient.
///
/// Maybe I'll revisit this and add another, more optimized
/// solution later when I have the time to do so.

#[derive(Debug)]
struct Path(Vec<Pos>);

impl Path {
    fn contains(&self, pt: &Pos) -> bool {
        for i in 1..self.0.len() {
            let p0 = self.0[i - 1];
            let p1 = self.0[i];

            if p0.0 == p1.0 && p0.0 == pt.0 && is_between(pt.1, p0.1, p1.1)
                || p0.1 == p1.1 && p0.1 == pt.1 && is_between(pt.0, p0.0, p1.0)
            {
                return true;
            }
        }

        false
    }
}

fn is_between(v: isize, a: isize, b: isize) -> bool {
    if a == b && a == v {
        true
    } else if a < b {
        v >= a && v <= b
    } else {
        v <= a && v >= b
    }
}

impl From<&str> for Path {
    fn from(v: &str) -> Self {
        Self(
            v.split(" -> ")
                .map(|v| v.trim())
                .map(|v| v.split_once(',').unwrap())
                .map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()).into())
                .collect(),
        )
    }
}

fn is_blocked(p: &Pos, sand_spots: &[Pos], pathes: &[Path]) -> bool {
    sand_spots.contains(p) || pathes.iter().any(|path| path.contains(p))
}

fn simulate(pathes: &[Path], y_max: isize) -> usize {
    let sand_start_pos = Pos(500, 0);
    let mut sand_spots: Vec<Pos> = vec![];
    let mut sand_pos = sand_start_pos;

    loop {
        let mut new_sand_pos = sand_pos + Pos(0, 1);
        if is_blocked(&new_sand_pos, &sand_spots, pathes) {
            // directly below is blocked
            new_sand_pos += Pos(-1, 0);
            if is_blocked(&new_sand_pos, &sand_spots, pathes) {
                // down-left is blocked
                new_sand_pos += Pos(2, 0);
                if is_blocked(&new_sand_pos, &sand_spots, pathes) {
                    // down-right is blocked
                    new_sand_pos = sand_pos;
                    // Sand has settled.
                    sand_spots.push(new_sand_pos);
                    if new_sand_pos.1 == 0 {
                        break;
                    }
                    // "Spawn" new sand.
                    sand_pos = sand_start_pos;
                    continue;
                }
            }
        }
        if new_sand_pos.1 > y_max {
            break;
        }
        sand_pos = new_sand_pos;
    }

    sand_spots.len()
}

fn main() {
    let input: String = lib::read_input!();

    let mut pathes: Vec<Path> = input.split('\n').map(|l| l.into()).collect();
    let y_max = pathes.iter().flat_map(|p| &p.0).map(|p| p.1).max().unwrap();

    let fallen_sand = simulate(&pathes, y_max);
    println!("Part 1:\n{fallen_sand} pieces of sand have fallen");

    pathes.push(Path(vec![
        Pos(isize::MIN, y_max + 2),
        Pos(isize::MAX, y_max + 2),
    ]));

    let fallen_sand = simulate(&pathes, y_max + 2);
    println!("Part 2:\n{fallen_sand} pieces of sand have fallen");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_between() {
        assert!(is_between(1, 0, 3));
        assert!(is_between(2, 7, 1));
        assert!(is_between(2, 7, 0));
        assert!(is_between(3, 3, 3));

        assert!(!is_between(3, 1, 2));
        assert!(!is_between(-7, 29, 9));
    }

    #[test]
    fn test_path_contain() {
        let pt = Pos(1, 1);
        let path = Path(vec![Pos(0, 0), Pos(1, 0), Pos(1, 3)]);
        assert!(path.contains(&pt));

        let pt = Pos(1, 1);
        let path = Path(vec![Pos(0, 0), Pos(2, 0), Pos(2, 3)]);
        assert!(!path.contains(&pt));
    }
}

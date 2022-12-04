fn get_shape_points(shape: char) -> u32 {
    match shape {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("invalid shape"),
    }
}

fn get_match_result(opponents_shape: char, my_shape: char) -> u32 {
    let opponents_shape = opponents_shape as i32;
    let my_shape = my_shape as i32 - 20;
    match opponents_shape - my_shape {
        -3 => 3,
        -5 | -2 => 0,
        -4 | -1 => 6,
        _ => panic!("invalid outcome"),
    }
}

fn get_shape_for_result(opponents_shape: char, outcome: char) -> char {
    let opponents_shape = opponents_shape as i32 - 64;
    let outcome = outcome as i32 - 20 - 64;
    match opponents_shape + outcome {
        6 | 9 => 'X',
        7 => 'Y',
        5 | 8 => 'Z',
        _ => panic!("invalid result"),
    }
}

fn get_points_part1(m: &[char]) -> u32 {
    get_shape_points(m[1]) + get_match_result(m[0], m[1])
}

fn get_points_part2(m: &[char]) -> u32 {
    let my_shape = get_shape_for_result(m[0], m[1]);
    let result_points = match m[1] {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => panic!("invalid result"),
    };
    get_shape_points(my_shape) + result_points
}

fn main() {
    let input: String = lib::read_input!();

    let matches: Vec<_> = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.as_bytes()[0] as char)
                .collect::<Vec<char>>()
        })
        .collect();

    let sum: u32 = matches.iter().map(|v| get_points_part1(v)).sum();

    println!("Part 1:\nThe sum of all matches is {}", sum);

    let sum: u32 = matches.iter().map(|v| get_points_part2(v)).sum();

    println!("Part 2:\nThe sum of all matches is {}", sum);
}

// And yes, I used TDD to solve this puzzle. 🤡

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_shape_points() {
        assert_eq!(get_shape_points('X'), 1);
        assert_eq!(get_shape_points('Y'), 2);
        assert_eq!(get_shape_points('Z'), 3);
    }

    #[test]
    fn test_get_match_result() {
        assert_eq!(get_match_result('A', 'X'), 3);
        assert_eq!(get_match_result('A', 'Y'), 6);
        assert_eq!(get_match_result('A', 'Z'), 0);
        assert_eq!(get_match_result('B', 'X'), 0);
        assert_eq!(get_match_result('B', 'Y'), 3);
        assert_eq!(get_match_result('B', 'Z'), 6);
        assert_eq!(get_match_result('C', 'X'), 6);
        assert_eq!(get_match_result('C', 'Y'), 0);
        assert_eq!(get_match_result('C', 'Z'), 3);
    }

    #[test]
    fn test_get_shape_for_result() {
        assert_eq!(get_shape_for_result('A', 'X'), 'Z');
        assert_eq!(get_shape_for_result('A', 'Y'), 'X');
        assert_eq!(get_shape_for_result('A', 'Z'), 'Y');
        assert_eq!(get_shape_for_result('B', 'X'), 'X');
        assert_eq!(get_shape_for_result('B', 'Y'), 'Y');
        assert_eq!(get_shape_for_result('B', 'Z'), 'Z');
        assert_eq!(get_shape_for_result('C', 'X'), 'Y');
        assert_eq!(get_shape_for_result('C', 'Y'), 'Z');
        assert_eq!(get_shape_for_result('C', 'Z'), 'X');
    }
}

fn is_subset<T: PartialOrd>(set1: (T, T), set2: (T, T)) -> bool {
    set1.0 >= set2.0 && set1.0 <= set2.1 && set1.1 <= set2.1 && set1.1 >= set2.0
}

fn is_overlap<T: PartialOrd>(set1: (T, T), set2: (T, T)) -> bool {
    set1.0 <= set2.0 && set2.0 <= set1.0
        || set1.1 >= set2.0 && set1.1 <= set2.1
        || set2.1 >= set1.0 && set2.1 <= set1.1
}

fn to_set(v: &str) -> (u32, u32) {
    let parts: Vec<&str> = v.split('-').collect();
    let a = parts[0].parse().expect("parse a to uint");
    let b = parts[1].parse().expect("parse b to uint");
    (a, b)
}

fn main() {
    let input: String = lib::read_input!();

    let sets: Vec<Vec<(u32, u32)>> = input
        .split('\n')
        .map(|s| s.split(',').map(to_set).collect())
        .collect();

    let count = sets
        .iter()
        .filter(|sets| is_subset(sets[0], sets[1]) || is_subset(sets[1], sets[0]))
        .count();

    println!("Part 1:\nCount of subsets is {}", count);

    let count = sets
        .iter()
        .filter(|sets| is_overlap(sets[0], sets[1]))
        .count();

    println!("Part 2:\nCount of overlaps is {}", count);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_subset() {
        assert!(is_subset((3, 7), (2, 8)));
        assert!(is_subset((1, 2), (0, 9)));
        assert!(is_subset((3, 3), (3, 3)));

        assert!(!is_subset((3, 5), (2, 4)));
        assert!(!is_subset((1, 5), (0, 2)));
        assert!(!is_subset((1, 5), (0, 1)));
    }

    #[test]
    fn test_is_overlap() {
        // Subsets also overlap
        assert!(is_subset((3, 7), (2, 8)));
        assert!(is_subset((1, 2), (0, 9)));
        assert!(is_subset((3, 3), (3, 3)));

        assert!(is_overlap((1, 3), (2, 4)));
        assert!(is_overlap((3, 5), (4, 9)));
        assert!(is_overlap((3, 3), (3, 9)));

        assert!(is_overlap((5, 7), (7, 9)));
        assert!(is_overlap((2, 8), (3, 7)));

        assert!(!is_overlap((1, 2), (3, 4)));
        assert!(!is_overlap((0, 0), (3, 3)));
        assert!(!is_overlap((1, 5), (6, 8)));
    }
}

use std::collections::HashSet;

fn split_half(v: &str) -> (&str, &str) {
    let len = v.len() / 2;
    (&v[..len], &v[len..])
}

fn find_duplicates(comp1: &str, comp2: &str) -> HashSet<char> {
    comp1.chars().filter(|c| comp2.contains(*c)).collect()
}

fn get_item_priority(item: &char) -> u32 {
    if item.is_uppercase() {
        *item as u32 - 38
    } else {
        *item as u32 - 96
    }
}

fn find_common_item(group: &[String]) -> char {
    group[0]
        .chars()
        .find(|c| group[1].contains(*c) && group[2].contains(*c))
        .unwrap()
}

fn main() {
    let input: String = lib::read_input!();

    let rucksacks: Vec<(&str, &str)> = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(split_half)
        .collect();

    let priorities: u32 = rucksacks
        .iter()
        .map(|(comp1, comp2)| find_duplicates(comp1, comp2))
        .map(|dupes| dupes.iter().map(get_item_priority).sum::<u32>())
        .sum();

    println!("Part 1:\nPriorities summed are {}", priorities);

    let mut groups = vec![Vec::<String>::new(); rucksacks.len() / 3];
    for (i, (comp1, comp2)) in rucksacks.iter().enumerate() {
        let items = format!("{comp1}{comp2}");
        let i = i / 3;

        let group = groups.get_mut(i).unwrap();
        group.push(items);
    }

    let priorities: u32 = groups
        .iter()
        .map(|g| find_common_item(g))
        .map(|i| get_item_priority(&i))
        .sum();

    println!("Part 2:\nPriorities summed are {}", priorities);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_half() {
        assert_eq!(split_half("abcdef"), ("abc", "def"));
    }

    #[test]
    fn test_find_duplicates() {
        assert_eq!(
            find_duplicates("aBBBcdE", "ABCde"),
            HashSet::from(['B', 'd'])
        )
    }

    #[test]
    fn test_get_item_priority() {
        assert_eq!(get_item_priority(&'a'), 1);
        assert_eq!(get_item_priority(&'b'), 2);
        assert_eq!(get_item_priority(&'c'), 3);
        assert_eq!(get_item_priority(&'A'), 27);
        assert_eq!(get_item_priority(&'B'), 28);
        assert_eq!(get_item_priority(&'C'), 29);
    }
}

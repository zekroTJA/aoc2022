fn check_init_sequence(v: &str) -> Option<usize> {
    v.chars()
        .enumerate()
        .find(|(_, c)| v.chars().filter(|cc| cc == c).count() > 1)
        .map(|(i, _)| i)
}

fn find_init_sequence(input: &str, ln: usize) -> Option<usize> {
    (ln..input.len()).find(|&i| check_init_sequence(&input[i - ln..i]).is_none())
}

fn main() {
    let input: String = lib::read_input!();

    let i = find_init_sequence(&input, 4).expect("init sequence");
    println!("Part 1:\nThe input sequence starts at {}", i);

    let i = find_init_sequence(&input, 14).expect("message sequence");
    println!("Part 2:\nThe message sequence starts at {}", i);
}

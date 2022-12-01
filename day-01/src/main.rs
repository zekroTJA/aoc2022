use std::{fs::File, io::Read};

fn main() {
    let mut input_file = File::open("day-01/input.txt").expect("input file");
    let mut input = String::new();
    input_file.read_to_string(&mut input).expect("read input");

    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut calories: Vec<Calories> = parts.iter().map(|line| line.into()).collect();
    calories.sort_by_key(|a| a.sum());
    calories.reverse();

    let top = calories.first().expect("take first");

    println!(
        "Part 1:\nThe elf with the most calories has {} calories in total.",
        top.sum()
    );

    let top3sum: u32 = calories.iter().take(3).map(|c| c.sum()).sum();
    println!(
        "Part 2:\nThe 3 elves with the most calories in total have {} calories in summ.",
        top3sum
    );
}

struct Calories(Vec<u32>);

impl Calories {
    fn sum(&self) -> u32 {
        self.0.iter().sum()
    }
}

impl From<&&str> for Calories {
    fn from(v: &&str) -> Self {
        let items: Vec<u32> = v
            .split('\n')
            .filter(|e| !e.is_empty())
            .map(|e| e.parse::<u32>().expect("parse u32"))
            .collect();
        Self(items)
    }
}

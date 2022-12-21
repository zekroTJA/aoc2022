trait Index {
    fn index_of(&self, v: isize) -> usize;
}

impl Index for Vec<isize> {
    fn index_of(&self, v: isize) -> usize {
        self.iter()
            .position(|&c| c == v)
            .unwrap_or_else(|| panic!("unwrap of {v}")) as usize
    }
}

fn mix(nums: &[isize], key: isize, rounds: usize) -> isize {
    let nums = nums.iter().map(|x| x * key).collect::<Vec<_>>();

    let mut positions: Vec<_> = (0..nums.len()).collect();

    for _ in 0..rounds {
        for (idx, &num) in nums.iter().enumerate() {
            let pos = positions.iter().position(|&y| y == idx).unwrap();
            positions.remove(pos);
            let idx_insert = (pos as isize + num).rem_euclid(positions.len() as isize) as usize;
            positions.insert(idx_insert, idx);
        }
    }
    let idx_0_before = nums.iter().position(|&i| i == 0).unwrap();
    let idx_0 = positions.iter().position(|&i| i == idx_0_before).unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|&i| nums[positions[(idx_0 + i as usize) % positions.len()]])
        .sum()
}

fn main() {
    let input: String = lib::read_input!();

    let numbers: Vec<_> = input
        .split('\n')
        .map(|l| l.parse::<isize>().unwrap())
        .collect();

    let v = mix(&numbers, 1, 1);
    println!("Part 1:\nThe groove coordinate is {v}");

    let v = mix(&numbers, 811589153, 10);
    println!("Part 2:\nThe groove coordinate is {v}");
}

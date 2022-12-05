fn parse_stack(v: &str) -> Vec<Vec<char>> {
    let lines: Vec<Vec<char>> = v
        .split('\n')
        .rev()
        .skip(1)
        .map(|l| {
            l.chars()
                .enumerate()
                .filter(|(i, _)| *i as i32 % 4 - 1 == 0)
                .map(|(_, c)| c)
                .collect()
        })
        .collect();

    let mut res = vec![Vec::with_capacity(lines.len()); lines[0].len()];
    for line in lines {
        for (i, item) in line.iter().enumerate() {
            if item != &' ' {
                res[i].push(*item);
            }
        }
    }

    res
}

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn perform_single_on(&self, stack: &mut [Vec<char>]) {
        for _ in 0..self.count {
            let item = stack[self.from].pop().expect("nothing to pop");
            stack[self.to].push(item);
        }
    }

    fn perform_multi_on(&self, stack: &mut [Vec<char>]) {
        let mut items: Vec<char> = Vec::with_capacity(self.count);
        for _ in 0..self.count {
            items.insert(0, stack[self.from].pop().expect("nothing to pop"))
        }
        stack[self.to].append(&mut items);
    }
}

impl From<&str> for Move {
    fn from(line: &str) -> Self {
        let split: Vec<&str> = line.split_whitespace().collect();
        Self {
            count: split[1].parse().unwrap(),
            from: split[3].parse::<usize>().unwrap() - 1,
            to: split[5].parse::<usize>().unwrap() - 1,
        }
    }
}

fn main() {
    let input: String = lib::read_input!();

    let (stack, moves) = input.split_once("\n\n").unwrap();

    let stack = parse_stack(stack);
    let moves: Vec<Move> = moves
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.into())
        .collect();

    // Create a deep copy of the stack to solve part 1
    let mut stack_1: Vec<Vec<char>> = stack.to_vec();
    moves.iter().for_each(|m| m.perform_single_on(&mut stack_1));

    let v: String = stack_1.iter().map(|s| s.last().unwrap()).cloned().collect();
    println!("Part 1:\nThe top items on the stack are {v}");

    // Create a deep copy of the stack to solve part 2
    let mut stack_2: Vec<Vec<char>> = stack.to_vec();
    moves.iter().for_each(|m| m.perform_multi_on(&mut stack_2));

    let v: String = stack_2.iter().map(|s| s.last().unwrap()).cloned().collect();
    println!("Part 2:\nThe top items on the stack are {v}");
}

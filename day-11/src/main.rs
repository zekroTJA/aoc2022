use std::cell::RefCell;

type Operation = dyn Fn(usize) -> usize;

fn parse_operation(s: &str) -> Box<Operation> {
    let by = s[6..].parse().ok();

    match s.as_bytes()[4] {
        b'+' => Box::from(move |v| v + by.unwrap_or(v)),
        b'-' => Box::from(move |v| v - by.unwrap_or(v)),
        b'*' => Box::from(move |v| v * by.unwrap_or(v)),
        _ => panic!("invalid operator"),
    }
}

struct Monkey {
    staring_items: Vec<usize>,
    operation: Box<Operation>,
    devidable_by: usize,
    if_devidable: usize,
    if_not_devidable: usize,
}

impl Monkey {
    fn throw_to(&self, item: usize) -> usize {
        if item % self.devidable_by == 0 {
            self.if_devidable
        } else {
            self.if_not_devidable
        }
    }
}

impl From<&str> for Monkey {
    fn from(v: &str) -> Self {
        let lines: Vec<&str> = v.split('\n').skip(1).collect();

        let staring_items = lines[0]["Starting items: ".len() + 2..]
            .split(',')
            .map(|v| v.trim().parse().unwrap())
            .collect();
        let operation = parse_operation(&lines[1]["Operation: new = ".len() + 2..]);
        let devidable_by = lines[2]["Test: divisible by ".len() + 2..].parse().unwrap();
        let if_devidable = lines[3]["If true: throw to monkey ".len() + 4..]
            .parse()
            .unwrap();
        let if_not_devidable = lines[4]["If false: throw to monkey ".len() + 4..]
            .parse()
            .unwrap();

        Self {
            staring_items,
            operation,
            devidable_by,
            if_devidable,
            if_not_devidable,
        }
    }
}

fn do_monkey_business(monkeys: &[Monkey], rounds: usize, worry_devive: usize) -> usize {
    let item_state: Vec<RefCell<Vec<usize>>> = monkeys
        .iter()
        .map(|v| RefCell::new(v.staring_items.clone()))
        .collect();
    let mut items_inspected: Vec<usize> = monkeys.iter().map(|_| 0).collect();

    let mod_factor = monkeys.iter().fold(1, |v, m2| v * m2.devidable_by);

    for _ in 0..rounds {
        for (i, monkey) in monkeys.iter().enumerate() {
            let mut my_items = item_state[i].borrow_mut();
            while let Some(item) = my_items.pop() {
                items_inspected[i] += 1;
                let item = (monkey.operation)(item) / worry_devive % mod_factor;
                let throw_to = monkey.throw_to(item);
                item_state[throw_to].borrow_mut().push(item);
            }
        }
    }

    items_inspected.sort();
    items_inspected.iter().rev().take(2).product()
}

fn main() {
    let input: String = lib::read_input!();

    let monkeys: Vec<Monkey> = input.split("\n\n").map(|v| v.into()).collect();

    let prod = do_monkey_business(&monkeys, 20, 3);
    println!("Part 1:\nThe product of both most inspecting monkeys is {prod}");

    let prod = do_monkey_business(&monkeys, 10_000, 1);
    println!("Part 1:\nThe product of both most inspecting monkeys is {prod}");
}

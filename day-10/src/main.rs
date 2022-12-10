use std::fmt::{Display, Write};

const SCREEN_WIDTH: isize = 40;

enum Instruction {
    Noop,
    Addx(isize),
}

impl From<&str> for Instruction {
    fn from(v: &str) -> Self {
        // Lets assume an ARM-like architechture 😉
        match &v[..4] {
            "noop" => Self::Noop,
            "addx" => Self::Addx(v[5..].parse().expect("parse addx payload")),
            _ => panic!("invalid instruction"),
        }
    }
}

#[derive(Clone)]
struct Line(Vec<bool>);

impl Line {
    fn new() -> Self {
        Self(Vec::with_capacity(SCREEN_WIDTH as usize))
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &v in &self.0 {
            if v {
                f.write_char('█')?;
            } else {
                f.write_char(' ')?;
            }
        }
        f.write_char('\n')
    }
}

struct Screen(Vec<Line>);

impl Screen {
    fn new() -> Self {
        Self(vec![])
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            write!(f, "{}", line)?;
        }
        Ok(())
    }
}

fn main() {
    let input: String = lib::read_input!();

    let instructions: Vec<Instruction> = input.split('\n').map(|l| l.into()).collect();

    let mut instruction_counter = 0usize;
    let mut instruction_register;
    let mut add_register: Option<isize> = None;
    let mut x = 1isize;

    let probe_cycles = &[20, 60, 100, 140, 180, 220];
    let mut probe_value = 0isize;

    let mut line = Line::new();
    let mut screen = Screen::new();

    for cycle in 1..isize::MAX {
        if probe_cycles.contains(&(cycle)) {
            probe_value += (cycle) * x;
        }

        let pos = (cycle - 1) % SCREEN_WIDTH;
        line.0.push(pos == x - 1 || pos == x || pos == x + 1);

        if pos == SCREEN_WIDTH - 1 {
            screen.0.push(line.clone());
            line = Line::new();
        }

        if let Some(add_val) = add_register {
            x += add_val;
            add_register = None;
            continue;
        }

        instruction_register = instructions.get(instruction_counter);
        if instruction_register.is_none() {
            break;
        }

        match instruction_register.unwrap() {
            Instruction::Noop => {}
            Instruction::Addx(v) => add_register = Some(*v),
        }

        instruction_counter += 1;
    }

    println!("Part 1:\nThe probe sum is {}", probe_value);

    println!("Part 2:\nThe screen prints the following:\n{}", screen);
}

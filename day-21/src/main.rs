use std::{cell::RefCell, collections::HashMap, rc::Rc};

struct Monkey {
    name: String,
    depends_on: Option<(String, String)>,
    operation: Box<dyn Fn(f64, f64) -> f64>,
}

impl From<&str> for Monkey {
    fn from(v: &str) -> Self {
        let (name, f) = v.split_once(": ").unwrap();
        let name = name.to_owned();

        if let Ok(nmb) = f.parse() {
            return Self {
                name,
                depends_on: None,
                operation: Box::from(move |_, _| nmb),
            };
        }

        let mut split = f.split(' ');
        let dep1 = split.next().unwrap().to_owned();
        let op = split.next().unwrap();
        let dep2 = split.next().unwrap().to_owned();

        let operation: Box<dyn Fn(f64, f64) -> f64> = match op {
            "+" => Box::from(|a, b| a + b),
            "-" => Box::from(|a, b| a - b),
            "*" => Box::from(|a, b| a * b),
            "/" => Box::from(|a, b| a / b),
            _ => panic!("invalid operation"),
        };

        Self {
            name,
            depends_on: Some((dep1, dep2)),
            operation,
        }
    }
}

#[allow(clippy::type_complexity)]
struct Node<'a> {
    monkey: &'a Monkey,
    leafs: Option<(Rc<RefCell<Node<'a>>>, Rc<RefCell<Node<'a>>>)>,
    value: Option<f64>,
}

impl<'a> Node<'a> {
    fn new(from: &str, monkeys: &'a HashMap<String, Monkey>) -> Rc<RefCell<Self>> {
        let monkey = monkeys.get(from).unwrap();
        let leafs = monkey
            .depends_on
            .clone()
            .map(|(m1, m2)| (Node::new(&m1, monkeys), Node::new(&m2, monkeys)));
        Rc::new(RefCell::new(Self {
            monkey,
            leafs,
            value: None,
        }))
    }
}

fn solve1(monkeys: &HashMap<String, Monkey>) -> usize {
    let root = Node::new("root", monkeys);

    // Traverse down the tree and solve the last leave nodes
    // that have values as leafs. And repeat that until
    // the root leaf itself has a value.

    while root.borrow().value.is_none() {
        let mut queue = vec![root.clone()];

        while let Some(c) = queue.pop() {
            let mut c = c.borrow_mut();
            let mut value: Option<f64> = None;
            if let Some((n1, n2)) = &c.leafs {
                let n1_val = n1.borrow().value;
                let n2_val = n2.borrow().value;

                if n1_val.is_some() && n2_val.is_some() {
                    #[allow(clippy::unnecessary_unwrap)]
                    let new_val = (c.monkey.operation)(n1_val.unwrap(), n2_val.unwrap());
                    value = Some(new_val);
                } else {
                    if n1_val.is_none() {
                        queue.insert(0, n1.clone());
                    }
                    if n2_val.is_none() {
                        queue.insert(0, n2.clone());
                    }
                }
            } else {
                let new_val = (c.monkey.operation)(0.0, 0.0);
                value = Some(new_val);
            }
            c.value = value;
        }
    }

    root.clone().borrow().value.unwrap() as usize
}

fn solve2(monkeys: &HashMap<String, Monkey>) -> usize {
    // This solution is super hacky and also very brute-forcy.

    // We simply add i up until the difference of the values
    // of the root leafs is negative, then we half the size
    // of i and go back until diff is positive again halfing
    // i again. And then we do this until diff is zero.

    let mut i = 0f64;
    let mut last_added = 100000000000f64;
    let mut last_negative = false;

    loop {
        let root = Node::new("root", monkeys);

        while root.borrow().value.is_none() {
            let mut queue = vec![root.clone()];

            while let Some(c) = queue.pop() {
                let mut c = c.borrow_mut();
                if c.monkey.name == "humn" {
                    c.value = Some(i);
                    continue;
                }
                let mut value: Option<f64> = None;
                if let Some((n1, n2)) = &c.leafs {
                    let n1_val = n1.borrow().value;
                    let n2_val = n2.borrow().value;

                    if n1_val.is_some() && n2_val.is_some() {
                        #[allow(clippy::unnecessary_unwrap)]
                        let new_val = (c.monkey.operation)(n1_val.unwrap(), n2_val.unwrap());
                        value = Some(new_val);
                    } else {
                        if n1_val.is_none() {
                            queue.insert(0, n1.clone());
                        }
                        if n2_val.is_none() {
                            queue.insert(0, n2.clone());
                        }
                    }
                } else {
                    let new_val = (c.monkey.operation)(0.0, 0.0);
                    value = Some(new_val);
                }
                c.value = value;
            }
        }

        let root = root.borrow();
        let a = root.leafs.clone().unwrap().0.borrow().value.unwrap();
        let b = root.leafs.clone().unwrap().1.borrow().value.unwrap();

        // This only works if "humn" is in the left side from root.
        // Otherwise, this will infinitely run.
        let diff = a - b;

        if diff == 0.0 {
            return i as usize;
        }

        if diff < 0.0 {
            if !last_negative {
                last_added /= 2.0;
            }
            last_negative = true;
            i -= last_added;
        } else {
            if last_negative {
                last_added /= 2.0;
            }
            last_negative = false;
            i += last_added;
        }
    }
}

fn main() {
    let input: String = lib::read_input!();

    let monkeys: HashMap<_, _> = input
        .split('\n')
        .map(Monkey::from)
        .map(|m| (m.name.clone(), m))
        .collect();

    let v = solve1(&monkeys);
    println!("Part 1:\nThe root monkey yells number {v}");

    let v = solve2(&monkeys);
    println!("Part 2:\nWe need to yell number {v}");
}

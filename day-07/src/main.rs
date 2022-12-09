use std::collections::HashMap;

#[derive(Debug)]
struct DirStack<'a>(Vec<&'a str>);

impl<'a> DirStack<'a> {
    fn new() -> Self {
        Self(vec![])
    }

    fn push(&mut self, new_dir: &'a str) {
        self.0.push(new_dir);
    }

    fn pop(&mut self) -> Option<&str> {
        self.0.pop()
    }
}

impl<'a> ToString for DirStack<'a> {
    fn to_string(&self) -> String {
        format!("/{}", self.0.join("/"))
    }
}

impl<'a> Clone for DirStack<'a> {
    fn clone(&self) -> Self {
        Self(self.0.to_vec())
    }
}

fn add_dir_size(sizes: &mut HashMap<String, usize>, dir_stack: &DirStack, size: usize) {
    let c_path = dir_stack.to_string();
    let mut c_size = *sizes.get(&c_path).unwrap_or(&0);
    c_size += size;
    sizes.insert(c_path, c_size);
}

fn main() {
    let input: String = lib::read_input!();

    let lines: Vec<&str> = input.split('\n').collect();

    let mut dir_stack = DirStack::new();
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();

    for line in lines.iter().skip(1) {
        if *line == "$ ls" || line.starts_with("dir ") {
            continue;
        }

        if line.starts_with("$ cd ") {
            let new_dir = line.strip_prefix("$ cd ").unwrap();
            if new_dir == ".." {
                dir_stack.pop();
            } else {
                dir_stack.push(new_dir);
            }
            continue;
        }

        let (size, _) = line.split_once(' ').unwrap();
        let size: usize = size.parse().unwrap();

        add_dir_size(&mut dir_sizes, &dir_stack, size);

        let mut dir_stack = dir_stack.clone();
        while dir_stack.pop().is_some() {
            add_dir_size(&mut dir_sizes, &dir_stack, size);
        }
    }

    let s: usize = dir_sizes
        .iter()
        .filter(|(_, s)| **s <= 100000)
        .map(|(_, s)| s)
        .sum();

    println!("Part 1:\nThe sum of the largest directories is {}", s);

    let needed_space = 30000000 - (70000000 - *dir_sizes.get("/").unwrap());

    let mut dirs: Vec<_> = dir_sizes
        .iter()
        .filter(|(_, s)| **s >= needed_space)
        .map(|(_, s)| s)
        .collect();

    dirs.sort();

    let s = dirs.first().unwrap();

    println!(
        "Part 2:\nThe smallest directory to be deleted has a size of {}",
        s
    );
}

use std::{cmp::Ordering, fmt::Display};

#[derive(Debug, Clone)]
enum Entry {
    Int(usize),
    List(Vec<Entry>),
}

impl Entry {
    fn cmp(&self, comp: &Entry) -> Ordering {
        match self {
            Self::Int(l) => match comp {
                Self::Int(r) => l.cmp(r),
                Self::List(r) => Self::List(vec![Self::Int(*l)]).cmp(&Self::List(r.clone())),
            },
            Self::List(l) => match comp {
                Self::Int(r) => Self::List(l.clone()).cmp(&Self::List(vec![Self::Int(*r)])),
                Self::List(r) => {
                    for (i, le) in l.iter().enumerate() {
                        if let Some(re) = r.get(i) {
                            let o = le.cmp(re);
                            if o.is_gt() || o.is_lt() {
                                return o;
                            }
                        } else {
                            return Ordering::Greater;
                        }
                    }
                    if l.len() == r.len() {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                }
            },
        }
    }
}

impl From<&str> for Entry {
    fn from(v: &str) -> Self {
        if v == "[]" {
            Self::List(vec![])
        } else if v.starts_with('[') {
            let mut elements: Vec<String> = Vec::new();
            let mut element = String::new();
            let mut open_brackets = 0u32;
            for c in v[1..v.len() - 1].chars() {
                if c == '[' {
                    open_brackets += 1;
                } else if c == ']' {
                    open_brackets -= 1;
                }
                if c == ',' && open_brackets == 0 {
                    elements.push(element.clone());
                    element.clear();
                } else {
                    element.push(c);
                }
            }
            elements.push(element.clone());
            Self::List(elements.iter().map(|e| e.as_str().into()).collect())
        } else {
            Self::Int(v.parse().expect("integer value"))
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entry::Int(v) => write!(f, "{}", v),
            Entry::List(v) => {
                write!(f, "[")?;
                for e in v {
                    e.fmt(f)?;
                    write!(f, ",")?;
                }
                write!(f, "]")?;
                Ok(())
            }
        }
    }
}

fn main() {
    let input: String = lib::read_input!();

    let pairs: Vec<(Entry, Entry)> = input
        .split("\n\n")
        .map(|t| {
            let (a, b) = t.split_once('\n').expect("two entries");
            (a.into(), b.into())
        })
        .collect();

    let sum: usize = pairs
        .iter()
        .enumerate()
        .filter(|(_, (a, b))| a.cmp(b) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum();

    println!("Part 1:\nThe sum of all correctly ordered package indices is {sum}");

    let mut all: Vec<Entry> = pairs
        .iter()
        .flat_map(|(a, b)| vec![a.clone(), b.clone()])
        .collect();
    all.append(&mut vec!["[[2]]".into(), "[[6]]".into()]);

    all.sort_by(|a, b| a.cmp(b));

    let decoder_key: usize = all
        .iter()
        .map(|e| format!("{}", e))
        .enumerate()
        .filter(|(_, e)| e == "[[6,],]" || e == "[[2,],]")
        .map(|(i, _)| i + 1)
        .product();

    println!("Part 2:\nThe decoder key is {decoder_key}");
}

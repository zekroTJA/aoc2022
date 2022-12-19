use lazy_static::lazy_static;
use math::round::ceil;
use regex::Regex;

#[derive(Debug)]
struct Blueprint {
    number: isize,
    recipes: Vec<Vec<(isize, isize)>>,
    mostcost: Vec<isize>,
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+) (\w+)(?: and (\d+)\s(\w+))?").unwrap();
}

fn get_ingredient(v: &str) -> isize {
    ["ore", "clay", "obsidian"]
        .iter()
        .position(|&c| c == v)
        .unwrap() as isize
}

impl From<&str> for Blueprint {
    fn from(v: &str) -> Self {
        let (bp, recipes_raw) = v.split_once(": ").unwrap();
        let number = bp["Blueprint ".len()..].parse().unwrap();
        let mut recipes = vec![];
        let mut mostcost = vec![0isize; 4];

        for recipe_raw in recipes_raw.split(". ") {
            let mut recipe = vec![];
            let caps = RE.captures(recipe_raw).unwrap();

            let p: (isize, isize) = (
                get_ingredient(caps.get(2).unwrap().as_str()),
                caps.get(1).unwrap().as_str().parse().unwrap(),
            );
            recipe.push(p);
            mostcost[p.0 as usize] = mostcost[p.0 as usize].max(p.1);

            if caps.get(4).is_some() {
                let p: (isize, isize) = (
                    get_ingredient(caps.get(4).unwrap().as_str()),
                    caps.get(3).unwrap().as_str().parse().unwrap(),
                );
                recipe.push(p);
                mostcost[p.0 as usize] = mostcost[p.0 as usize].max(p.1);
            }

            recipes.push(recipe);
        }

        Self {
            number,
            recipes,
            mostcost,
        }
    }
}

impl Blueprint {
    fn simulate(&self, timeleft: isize, bots: Vec<isize>, ingredients: Vec<isize>) -> isize {
        //   0    1     2         3
        // [ ore, clay, obsidian, geode ]

        if timeleft == 0 {
            return ingredients[3];
        }

        let mut geodes = ingredients[3] + bots[3] * timeleft;

        'outer: for (bottype, recipe) in self.recipes.iter().enumerate() {
            // If geode robot, continue. We want as much as possible from them.
            // If we have enough robots to match mostcost each round,
            // don't build more robots.
            if bottype != 3 && bots[bottype] >= self.mostcost[bottype] {
                continue 'outer;
            }

            let mut timecost = 0isize;
            for &(ingredient, cost) in recipe {
                // If we have no bots to harvest this ingredient, skip.
                if bots[ingredient as usize] == 0 {
                    continue 'outer;
                }
                // Calculate the maximum time it would take to farm the
                // ingredients for a recipe with the available amounts
                // of bots.
                let tc = ceil(
                    (cost - ingredients[ingredient as usize]) as f64
                        / bots[ingredient as usize] as f64,
                    0,
                );
                timecost = timecost.max(tc as isize);
            }

            // When we have no time left, continue with the next recipe.
            let timeleft = timeleft - timecost - 1;
            if timeleft <= 0 {
                continue 'outer;
            }

            // Calculate ingredients we would have after passing
            // time of timecost.
            let mut ingredients: Vec<_> = ingredients
                .iter()
                .zip(&bots)
                .map(|(i, b)| i + b * (timecost + 1))
                .collect();

            // Craft the new bot ...
            let mut bots = bots.clone();
            bots[bottype] += 1;

            // ... and remove the ingredients needed for that bot.
            for &(ingredient, cost) in recipe {
                ingredients[ingredient as usize] -= cost;
            }

            // If we now have more geodest than in the try before, 🎉
            geodes = geodes.max(self.simulate(timeleft, bots, ingredients));
        }

        // Return max geodes colletable
        geodes
    }
}

fn main() {
    let input: String = lib::read_input!();

    let recipes: Vec<_> = input.split('\n').map(Blueprint::from).collect();

    let sum: isize = recipes
        .iter()
        .map(|r| r.number * r.simulate(24, vec![1, 0, 0, 0], vec![0; 4]))
        .sum();

    println!("Part 1:\nThe sum of all quality levels is {sum}");

    let prod: isize = recipes
        .iter()
        .take(3)
        .map(|r| r.simulate(32, vec![1, 0, 0, 0], vec![0; 4]))
        .product();

    println!("Part 2:\nThe product of the geodes collectable of the first 3 blueprints is {prod}");
}

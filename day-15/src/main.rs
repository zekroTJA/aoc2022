use std::{cell::RefCell, collections::HashSet};

use lib::pos::Pos;

#[derive(Debug)]
struct Sensor {
    pos: Pos,
    nearest_beacon: Pos,
    distance: isize,
}

impl From<&str> for Sensor {
    fn from(v: &str) -> Self {
        let (sensor, beacon) = v.split_once(':').unwrap();
        let sensor = &sensor["Sensor at ".len()..];
        let beacon = &beacon[" closest beacon is at ".len()..];
        let sensor = sensor.split_once(", ").unwrap();
        let beacon = beacon.split_once(", ").unwrap();

        let pos = (
            sensor.0[2..].parse::<isize>().unwrap(),
            sensor.1[2..].parse::<isize>().unwrap(),
        )
            .into();
        let nearest_beacon = (
            beacon.0[2..].parse::<isize>().unwrap(),
            beacon.1[2..].parse::<isize>().unwrap(),
        )
            .into();
        let distance = manhattan_distance(pos, nearest_beacon);

        Self {
            pos,
            nearest_beacon,
            distance,
        }
    }
}

fn manhattan_distance(p: Pos, q: Pos) -> isize {
    // Source: https://en.wikipedia.org/wiki/Taxicab_geometry
    (p.0 - q.0).abs() + (p.1 - q.1).abs()
}

fn main() {
    let input: String = lib::read_input!();

    let sensors: Vec<Sensor> = input.split('\n').map(|l| l.into()).collect();
    let beacon_positions: HashSet<Pos> = sensors.iter().map(|s| s.nearest_beacon).collect();
    let sensor_positions: HashSet<Pos> = sensors.iter().map(|s| s.pos).collect();

    let max_x_sensor = sensors.iter().max_by_key(|s| s.nearest_beacon.0).unwrap();
    let min_x_sensor = sensors.iter().min_by_key(|s| s.nearest_beacon.0).unwrap();
    let max_x = max_x_sensor.pos.0 + max_x_sensor.distance * 2;
    let min_x = min_x_sensor.pos.0 - min_x_sensor.distance * 2;

    let row = 2000000isize;

    let mut c = 0usize;
    'outer: for p in (min_x..max_x).map(|x| Pos(x, row)) {
        for s in &sensors {
            if manhattan_distance(p, s.pos) <= s.distance {
                if !beacon_positions.contains(&p) && !sensor_positions.contains(&p) {
                    c += 1;
                }
                continue 'outer;
            }
        }
    }

    println!("Part 1:\nThe number of spots where no beacon can be is {c}");

    // Because this brute-force attempt would take waaaay too long
    // to solve part two, we need something quicker here.

    let max_xy = 4000000isize;

    let mut ranges: Vec<(isize, isize)> = Vec::new();
    for y in 0..max_xy + 1 {
        ranges.clear();

        for s in &sensors {
            let y_offset = s.distance - (s.pos.1 - y).abs();
            if y_offset < 0 {
                continue;
            }
            let lx = s.pos.0 - y_offset;
            let hx = s.pos.0 + y_offset;
            ranges.push((lx, hx));
        }

        ranges.sort();

        let mut joined_ranges: Vec<RefCell<(isize, isize)>> = Vec::new();
        for &(l, h) in &ranges {
            if joined_ranges.is_empty() {
                joined_ranges.push(RefCell::new((l, h)));
                continue;
            }

            let (_, joined_h) = joined_ranges.last().unwrap().borrow().to_owned();
            if l > joined_h + 1 {
                joined_ranges.push(RefCell::new((l, h)));
                continue;
            }

            let joined_h = joined_h.max(h);
            joined_ranges.last().unwrap().borrow_mut().1 = joined_h;
        }

        if joined_ranges.len() > 1 {
            let p = Pos(joined_ranges[0].borrow().1 + 1, y);
            let freq = p.0 * 4000000 + p.1;
            println!("Part 2:\nThe frequency of the distress beacon is {freq}");
            break;
        }
    }
}

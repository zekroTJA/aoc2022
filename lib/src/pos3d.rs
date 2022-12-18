use std::ops::Add;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Pos3d {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Pos3d {
    pub const fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
}

impl From<&str> for Pos3d {
    fn from(v: &str) -> Self {
        let mut split = v.split(',').map(|v| v.trim());
        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();
        let z = split.next().unwrap().parse().unwrap();
        Pos3d::new(x, y, z)
    }
}

impl Add for Pos3d {
    type Output = Pos3d;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

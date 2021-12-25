use core::hash::Hash;
use std::{
    cmp::{max, min},
    collections::HashMap,
    io::BufRead,
};
#[macro_use]
extern crate scan_fmt;

#[derive(PartialEq, Eq, Hash)]
struct Cube<T>([T; 6]);

impl<T: num::Num + std::cmp::Ord + Copy> Cube<T>
where
    usize: TryFrom<T>,
{
    fn intersect(&self, rhs: &Self) -> Option<Self> {
        let c = Cube([
            max(self.0[0], rhs.0[0]),
            min(self.0[1], rhs.0[1]),
            max(self.0[2], rhs.0[2]),
            min(self.0[3], rhs.0[3]),
            max(self.0[4], rhs.0[4]),
            min(self.0[5], rhs.0[5]),
        ]);
        if c.volume() == 0 {
            None
        } else {
            Some(c)
        }
    }

    fn volume(&self) -> usize {
        let dx = self.0[1] - self.0[0] + T::one();
        let dy = self.0[3] - self.0[2] + T::one();
        let dz = self.0[5] - self.0[4] + T::one();
        if dx <= T::zero() || dy <= T::zero() || dz <= T::zero() {
            0
        } else {
            let dx = usize::try_from(dx).unwrap_or(0);
            let dy = usize::try_from(dy).unwrap_or(0);
            let dz = usize::try_from(dz).unwrap_or(0);
            dx * dy * dz
        }
    }
}

struct CubeSet<T> {
    set: HashMap<Cube<T>, i32>,
}

impl<T: num::Num + num::Bounded + std::cmp::Ord + Copy + Hash> CubeSet<T>
where
    usize: TryFrom<T>,
{
    fn new() -> Self {
        Self {
            set: HashMap::new(),
        }
    }

    fn add(&mut self, sz: [T; 6], state: bool) {
        let c = Cube(sz);

        let mut tmp = self
            .set
            .iter()
            .filter_map(|(k, &v)| Some((k.intersect(&c)?, -v)))
            .collect::<Vec<_>>();
        if state {
            tmp.push((c, 1));
        }
        for (k, v) in tmp {
            if let Some(curr) = self.set.get_mut(&k) {
                *curr += v;
                if *curr == 0 {
                    self.set.remove(&k);
                }
            } else {
                self.set.insert(k, v);
            }
        }
    }

    fn volume(&self) -> i64 {
        self.set
            .iter()
            .map(|(k, v)| k.volume() as i64 * i64::from(*v))
            .sum()
    }

    fn restrict_axis(&mut self, a: T, b: T) {
        let mut m1 = [
            b + T::one(),
            T::max_value(),
            T::min_value(),
            T::max_value(),
            T::min_value(),
            T::max_value(),
        ];
        let mut m2 = [
            T::min_value(),
            a - T::one(),
            T::min_value(),
            T::max_value(),
            T::min_value(),
            T::max_value(),
        ];
        for _ in 0..3 {
            self.add(m1, false);
            self.add(m2, false);
            m1.rotate_right(2);
            m2.rotate_right(2);
            m1[0] = a;
            m1[1] = b;
            m2[0] = a;
            m2[1] = b;
        }
    }
}

fn parse(f: impl BufRead) -> CubeSet<i32> {
    let mut ret = CubeSet::new();
    for l in f.lines().flatten() {
        let (v, x1, x2, y1, y2, z1, z2) = scan_fmt!(
            &l,
            "{} x={}..{},y={}..{},z={}..{}",
            String,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32
        )
        .unwrap();
        let v = v == "on";
        ret.add([x1, x2, y1, y2, z1, z2], v);
    }
    ret
}

fn main() {
    let f = std::io::BufReader::new(std::io::stdin());
    let mut m = parse(f);
    let tot = m.volume();
    m.restrict_axis(-50, 50);
    println!("{}", m.volume());
    println!("{}", tot);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let m = parse(&include_bytes!("../test/input1.txt")[..]);
        assert_eq!(m.volume(), 39);
        let mut m = parse(&include_bytes!("../test/input2.txt")[..]);
        m.restrict_axis(-50, 50);
        assert_eq!(m.volume(), 590_784);
        let m = parse(&include_bytes!("../test/input3.txt")[..]);
        assert_eq!(m.volume(), 2_758_514_936_282_235);
    }
}

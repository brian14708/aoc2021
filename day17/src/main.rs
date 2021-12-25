use itertools::Itertools;
use std::{cmp::max, collections::BTreeSet, io::BufRead};

#[macro_use]
extern crate scan_fmt;

#[derive(Debug, PartialEq, Eq)]
struct Region {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl Region {
    fn parse(s: &str) -> Result<Region, Box<dyn std::error::Error>> {
        let (x_min, x_max, y_min, y_max) =
            scan_fmt!(s, "target area: x={}..{}, y={}..{}", i32, i32, i32, i32)?;
        Ok(Region {
            x_min,
            x_max,
            y_min,
            y_max,
        })
    }

    fn solve_in_step(&self, n: usize) -> Option<impl Iterator<Item = (i32, i32)>> {
        let n = n as f32;
        let half_n_minus_one = (n - 1.0) / 2.0;

        // (1) Region.y_min <= y + (y-1) + ... + (y-n+1) <= Region.y_max
        //     (2*Region.y_min + n*n - n) / (2*n) <= y <= (2*Region.y_max + n*n - n) / (2*n)
        let vy_max = (self.y_max as f32 / n + half_n_minus_one).floor() as i32;
        let vy_min = (self.y_min as f32 / n + half_n_minus_one).ceil() as i32;

        // will never reach target box, since when y == 0, speed will be at least -abs(vy_min)
        if vy_min > max(self.y_min.abs(), self.y_max.abs()) {
            return None;
        }

        // (2) if x >= n-1
        //       Region.x_min <= x + (x-1) + ... + (x-n+1) <= Region.x_max
        //       Region.x_min <= x*(x+1)/2 - (x-n+1)*(x-n)/2 <= Region.x_max
        //       (2*Region.x_min + n*n - n) / (2*n) <= x <= (2*Region.x_max + n*n - n) / (2*n)
        let mut vx_max = (self.x_max as f32 / n + half_n_minus_one).floor() as i32;
        let mut vx_min =
            f32::max(n - 1.0, (self.x_min as f32 / n + half_n_minus_one).ceil()) as i32;

        if vx_max < vx_min {
            // (3) if x < n-1
            //        Region.x_min <= x*(x+1)/2 <= Region.x_max
            //        -0.5 + sqrt(0.25 + 2 * Region.x_min) <= x <= -0.5 + sqrt(0.25 + 2 * Region.x_max)
            vx_max = f32::min(
                n - 1.0,
                ((0.25 + 2.0 * self.x_max as f32).sqrt() - 0.5).floor(),
            ) as i32;
            vx_min = ((0.25 + 2.0 * self.x_min as f32).sqrt() - 0.5).ceil() as i32;
        }

        Some((vx_min..=vx_max).cartesian_product(vy_min..=vy_max))
    }

    fn solve(&self) -> BTreeSet<(i32, i32)> {
        (1..)
            .map(|i| self.solve_in_step(i))
            .while_some()
            .flatten()
            .collect()
    }
}

fn main() {
    for l in std::io::BufReader::new(std::io::stdin()).lines().flatten() {
        let r = Region::parse(&l).unwrap();

        let s = r.solve();
        let y = s.iter().map(|&(_, y)| y).max().unwrap();
        println!("{}", if y > 0 { y * (y + 1) / 2 } else { 0 });
        println!("{}", s.len());
    }
}

#[test]
fn test_solve() {
    let r = Region::parse("target area: x=20..30, y=-10..-5").unwrap();
    assert_eq!(
        r,
        Region {
            x_min: 20,
            x_max: 30,
            y_min: -10,
            y_max: -5
        }
    );

    let s = r.solve();
    assert_eq!(s.len(), 112);
    let y = s.iter().map(|&(_, y)| y).max().unwrap();
    assert_eq!(y * (y + 1) / 2, 45);
}

use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Scanner {
    pt: Vec<[i32; 3]>,
    dist: HashMap<i32, Vec<usize>>,
    location: Option<[i32; 3]>,
}

fn rotate(v: [i32; 3], rot: u8) -> [i32; 3] {
    let b0_group = (rot % 3) as usize;
    let b1_group = ((rot / 3 % 2) as usize + b0_group + 1) % 3;

    let b0_sign = i32::from((rot / 6) % 2) * -2 + 1;
    let b1_sign = i32::from((rot / 12) % 2) * -2 + 1;

    let dot_cross = {
        let mut basis = [[0; 3]; 2];
        basis[0][b0_group] = b0_sign;
        basis[1][b1_group] = b1_sign;

        (basis[0][1] * basis[1][2] - basis[0][2] * basis[1][1]) * v[0]
            + (basis[0][2] * basis[1][0] - basis[0][0] * basis[1][2]) * v[1]
            + (basis[0][0] * basis[1][1] - basis[0][1] * basis[1][0]) * v[2]
    };

    [v[b0_group] * b0_sign, v[b1_group] * b1_sign, dot_cross]
}

impl Scanner {
    fn parse(f: &mut impl Iterator<Item = String>) -> Option<Self> {
        f.next()?;

        let mut pt = vec![];
        for l in f {
            if l.is_empty() {
                break;
            }
            let mut p = l.split(',').map(|s| s.parse::<i32>().unwrap());
            pt.push([p.next().unwrap(), p.next().unwrap(), p.next().unwrap()]);
        }
        let mut dist = HashMap::new();
        for (i, p1) in pt.iter().enumerate() {
            for (j, p2) in (&pt[i + 1..]).iter().enumerate() {
                let d = (p1[0] - p2[0]) * (p1[0] - p2[0])
                    + (p1[1] - p2[1]) * (p1[1] - p2[1])
                    + (p1[2] - p2[2]) * (p1[2] - p2[2]);
                dist.entry(d)
                    .or_insert_with(Vec::new)
                    .extend(&[i, i + 1 + j]);
            }
        }
        for v in dist.values_mut() {
            v.sort_unstable();
            v.dedup();
        }
        Some(Scanner {
            pt,
            dist,
            location: None,
        })
    }

    fn match_points(&self, rhs: &Self) -> Option<(Vec<[i32; 3]>, [i32; 3])> {
        let mut candidates: Vec<Option<Vec<usize>>> = vec![None; self.pt.len()];

        for (d, p1) in &self.dist {
            if let Some(p2) = rhs.dist.get(d) {
                for &i in p1.iter() {
                    if let Some(s) = candidates[i].as_mut() {
                        s.retain(|v| p2.binary_search(v).is_ok());
                    } else {
                        candidates[i] = Some(p2.clone());
                    }
                }
            }
        }

        let determined = candidates
            .iter()
            .enumerate()
            .filter_map(|(i, c)| {
                let c = c.as_ref()?;
                if c.len() == 1 {
                    Some((i, *c.iter().next()?))
                } else {
                    None
                }
            })
            .collect::<HashMap<usize, usize>>();
        if determined.is_empty() {
            return None;
        }

        let (r, translate) = (0..24).find_map(|r| {
            let mut it = determined.iter();
            let (&i, &j) = it.next().unwrap();
            let pa = self.pt[i];
            let pb = rotate(rhs.pt[j], r);
            let translate = [pa[0] - pb[0], pa[1] - pb[1], pa[2] - pb[2]];

            it.all(|(&i, &j)| {
                let pa = self.pt[i];
                let pb = rotate(rhs.pt[j], r);
                let pb = [
                    translate[0] + pb[0],
                    translate[1] + pb[1],
                    translate[2] + pb[2],
                ];
                pa == pb
            })
            .then(|| (r, translate))
        })?;

        let beacons = rhs
            .pt
            .iter()
            .map(|a| {
                let a = rotate(*a, r);
                [
                    translate[0] + a[0],
                    translate[1] + a[1],
                    translate[2] + a[2],
                ]
            })
            .collect::<Vec<_>>();

        if determined.len() < 12
            && beacons
                .iter()
                .filter(|&b| self.pt.iter().any(|v| v == b))
                .count()
                < 12
        {
            return None;
        }

        Some((beacons, translate))
    }
}

fn solve(mut scanners: Vec<Scanner>) -> (usize, i32) {
    let mut known = HashSet::<usize>::new();
    let mut points = HashSet::<[i32; 3]>::new();

    known.insert(0);
    scanners[0].location = Some([0, 0, 0]);
    points.extend(scanners[0].pt.iter());

    while known.len() != scanners.len() {
        let r = scanners
            .iter()
            .enumerate()
            .filter(|(i, _)| known.get(i).is_none())
            .find_map(|(i, s)| {
                known
                    .iter()
                    .find_map(|&j| scanners[j].match_points(s))
                    .map(|p| (i, p))
            });

        if let Some((i, (beacons, transform))) = r {
            known.insert(i);
            points.extend(beacons.iter());
            scanners[i].pt = beacons;
            scanners[i].location = Some(transform);
        }
    }

    let d = scanners
        .iter()
        .filter_map(|v| v.location)
        .tuple_combinations()
        .map(|(a, b)| (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs())
        .max()
        .unwrap();

    (points.len(), d)
}

fn main() {
    let mut f = std::io::BufReader::new(std::io::stdin()).lines().flatten();
    let scanners = (0..)
        .map_while(|_| Scanner::parse(&mut f))
        .collect::<Vec<_>>();
    let (p, d) = solve(scanners);
    println!("{}", p);
    println!("{}", d);
}

#[test]
fn test() {
    let mut f = include_bytes!("../test/input.txt").lines().flatten();
    let scanners = (0..)
        .map_while(|_| Scanner::parse(&mut f))
        .collect::<Vec<_>>();
    assert_eq!(scanners.iter().map(|s| s.pt.len()).sum::<usize>(), 127);
    assert_eq!(solve(scanners), (79, 3621));
}

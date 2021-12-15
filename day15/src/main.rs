use std::{
    cmp::{max, min, Reverse},
    collections::BinaryHeap,
    io::BufRead,
};

struct Cave {
    map: Vec<i32>,
    rows: usize,
    cols: usize,
}

impl Cave {
    fn parse(f: impl BufRead) -> Self {
        let mut c = Self {
            map: vec![],
            rows: 0,
            cols: 0,
        };
        for l in f.lines().flatten() {
            c.rows += 1;
            c.cols = l.len();
            c.map
                .extend(l.chars().map(|c| c.to_digit(10).unwrap() as i32));
        }
        c
    }

    fn solve(&self, repeat: usize) -> i32 {
        let mut visited = vec![false; (self.rows * repeat) * (self.cols * repeat)];
        let mut b = BinaryHeap::from([Reverse((0, (0, 0)))]);
        while let Some(Reverse((s, (x, y)))) = b.pop() {
            if (x, y) == (self.cols * repeat - 1, self.rows * repeat - 1) {
                return s;
            }
            let idxs = [
                (x, min(y + 1, self.rows * repeat - 1)),
                (x, max(y, 1) - 1),
                (min(x + 1, self.cols * repeat - 1), y),
                (max(x, 1) - 1, y),
            ];
            for (x, y) in idxs {
                let idx = x + y * (repeat * self.cols);
                if !visited[idx] {
                    visited[idx] = true;

                    let idx = (x % self.cols) + (y % self.rows) * self.cols;
                    let d = (((x / self.cols + y / self.rows) as i32 + self.map[idx]) - 1) % 9 + 1;
                    b.push(Reverse((s + d, (x, y))));
                }
            }
        }
        panic!("failed to find solution")
    }
}

fn main() {
    let c = Cave::parse(std::io::BufReader::new(std::io::stdin()));
    println!("{}", c.solve(1));
    println!("{}", c.solve(5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cave() {
        let f = include_bytes!("../test/input.txt");
        let c = Cave::parse(&f[..]);
        assert_eq!(c.rows, 10);
        assert_eq!(c.cols, 10);
        assert_eq!(c.solve(1), 40);
        assert_eq!(c.solve(5), 315);
    }
}

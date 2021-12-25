use std::{cmp::min, fmt, io::BufRead};

const KERN_SIZE: usize = 3;

#[derive(Clone)]
struct Grid {
    data: Vec<bool>,
    mapping: Vec<bool>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn parse(f: impl BufRead) -> Self {
        let mut l = f.lines().flatten();
        let mapping = l.next().unwrap().chars().map(|s| s == '#').collect();
        l.next(); // ignore

        let mut data = Vec::new();
        let (mut rows, mut cols) = (0, 0);
        for l in l {
            rows += 1;
            cols = l.len();
            data.extend(l.chars().map(|s| s == '#'));
        }

        Self {
            data,
            mapping,
            rows,
            cols,
        }
    }

    fn expand(&mut self, n: usize) {
        let mut data = vec![false; (self.rows + 2 * n) * (self.cols + 2 * n)];
        for i in 0..self.rows {
            for j in 0..self.cols {
                data[(i + n) * (self.cols + 2 * n) + j + n] = self.data[i * self.cols + j];
            }
        }

        self.data = data;
        self.cols += 2 * n;
        self.rows += 2 * n;
    }

    fn step(&mut self, n: usize) {
        for _ in 0..n / 2 {
            self.expand(2);
            self.step_impl();
            self.step_impl();
        }
        if n % 2 == 1 {
            self.expand(1);
            self.step_impl();
        }
    }

    fn step_impl(&mut self) {
        let mut buf = vec![];
        for i in 0..self.rows {
            let mut d = u8::from(self.data[i * self.cols]);
            d = (d << 1) | d;
            for j in 0..self.cols {
                d = (d << 1) & ((1 << KERN_SIZE) - 1);
                d |= u8::from(self.data[i * self.cols + min(self.cols - 1, j + 1)]);
                buf.push(d);
            }
        }
        for j in 0..self.cols {
            let mut d: u16 = u16::from(buf[j]);
            d = (d << KERN_SIZE) | d;
            for i in 0..self.rows {
                d = (d << KERN_SIZE) & ((1 << (KERN_SIZE * KERN_SIZE)) - 1);
                d |= u16::from(buf[min(self.rows - 1, i + 1) * self.cols + j]);
                self.data[i * self.cols + j] = self.mapping[d as usize];
            }
        }
    }

    fn count(&self) -> usize {
        self.data.iter().filter(|&&c| c).count()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.data[i * self.cols + j] {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let f = std::io::BufReader::new(std::io::stdin());
    let mut g = Grid::parse(f);
    let mut gg = g.clone();
    g.step(2);
    println!("{}", g.count());
    gg.step(50);
    println!("{}", gg.count());
}

#[test]
fn test_grid() {
    let f = include_bytes!("../test/input.txt");
    let mut g = Grid::parse(&f[..]);
    assert_eq!(g.mapping.len(), 512);
    assert_eq!(g.data.len(), g.rows * g.cols);
    g.step(2);
    assert_eq!(g.count(), 35);
    g.step(48);
    assert_eq!(g.count(), 3351);
}

use std::fmt::{self, Write};
use std::io::BufRead;

struct Grid {
    data: Vec<i8>,
    rows: usize,
    cols: usize,
    // state for step
    idx: Vec<usize>,
    flashes: usize,
}

impl Grid {
    fn parse(l: impl BufRead) -> Self {
        let mut data = vec![];
        let mut rows = 0;
        for l in l.lines().flatten() {
            for c in l.chars() {
                data.push(c.to_digit(10).unwrap() as i8);
            }
            rows += 1;
        }
        let cols = data.len() / rows;

        Self {
            data,
            rows,
            cols,
            idx: vec![],
            flashes: 0,
        }
    }

    fn step(&mut self) -> usize {
        for (i, d) in self.data.iter_mut().enumerate() {
            if *d < 0 {
                *d = 0;
            }
            *d += 1;
            if *d > 9 {
                self.idx.push(i);
            }
        }
        while let Some(idx) = self.idx.pop() {
            if self.data[idx] < 0 {
                continue;
            }
            self.data[idx] = -1;
            let c = (idx % self.cols) as i32;
            let r = (idx / self.cols) as i32;
            for dr in -1..=1 {
                if (r + dr) < 0 || (r + dr) >= self.rows as i32 {
                    continue;
                }
                for dc in -1..=1 {
                    if (c + dc) < 0 || (c + dc) >= self.cols as i32 {
                        continue;
                    }
                    let new_idx = (c + dc) as usize + (r + dr) as usize * self.cols;
                    if self.data[new_idx] < 0 {
                        continue;
                    }
                    self.data[new_idx] += 1;
                    if self.data[new_idx] > 9 {
                        self.idx.push(new_idx);
                    }
                }
            }
        }
        let mut flashes = 0;
        for d in self.data.iter_mut() {
            if *d < 0 {
                *d = 0;
                flashes += 1;
            }
        }
        self.flashes += flashes;
        flashes
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let d = self.data[i * self.cols + j];
                f.write_char(char::from_digit(d as u32, 10).unwrap())?;
            }
            if i != self.rows - 1 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

fn main() {
    let mut g = Grid::parse(std::io::BufReader::new(std::io::stdin()));
    let mut steps = 0;
    let mut sync_step = 0;

    while steps < 100 {
        steps += 1;
        if g.step() == g.rows * g.cols {
            sync_step = steps;
        }
    }
    println!("{}", g.flashes);
    while sync_step == 0 {
        steps += 1;
        if g.step() == g.rows * g.cols {
            sync_step = steps;
        }
    }
    println!("{}", sync_step);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let mut g = Grid::parse(
            "11111
19991
19191
19991
11111"
                .as_bytes(),
        );
        g.step();
        assert_eq!(
            format!("{}", g),
            "34543
40004
50005
40004
34543"
        );
        g.step();
        assert_eq!(
            format!("{}", g),
            "45654
51115
61116
51115
45654"
        );
    }

    #[test]
    fn test_flashes() {
        let mut g = Grid::parse(
            "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
                .as_bytes(),
        );
        for _ in 0..10 {
            g.step();
        }
        assert_eq!(g.flashes, 204);
        for _ in 0..90 {
            g.step();
        }
        assert_eq!(g.flashes, 1656);
    }
}

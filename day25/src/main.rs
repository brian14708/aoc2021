use std::{fmt::Debug, io::BufRead};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Type {
    Empty,
    East,
    South,
}

struct Map {
    tiles: Vec<Type>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn parse(f: impl BufRead) -> Self {
        let mut rows = 0;
        let mut cols = 0;
        let mut tiles = vec![];
        for l in f.lines().flatten() {
            tiles.extend(l.chars().map(|c| match c {
                '>' => Type::East,
                '.' => Type::Empty,
                'v' => Type::South,
                _ => panic!("invalid type"),
            }));
            cols = l.len();
            rows += 1;
        }
        Self { tiles, rows, cols }
    }

    fn step(&mut self) -> usize {
        let mut cnt = 0;
        for i in 0..self.rows {
            let first = self.tiles[i * self.cols];

            let mut prev = first;
            for j in 1..self.cols {
                let idx = i * self.cols + j;
                let curr = self.tiles[idx];
                if (prev, curr) == (Type::East, Type::Empty) {
                    self.tiles[idx] = Type::East;
                    self.tiles[idx - 1] = Type::Empty;
                    cnt += 1;
                }
                prev = curr;
            }

            if prev == Type::East && first == Type::Empty {
                self.tiles[i * self.cols + self.cols - 1] = Type::Empty;
                self.tiles[i * self.cols] = Type::East;
                cnt += 1;
            }
        }

        for j in 0..self.cols {
            let first = self.tiles[j];

            let mut prev = first;
            for i in 1..self.rows {
                let idx = i * self.cols + j;
                let curr = self.tiles[idx];
                if (prev, curr) == (Type::South, Type::Empty) {
                    self.tiles[idx] = Type::South;
                    self.tiles[idx - self.cols] = Type::Empty;
                    cnt += 1;
                }
                prev = curr;
            }

            if prev == Type::South && first == Type::Empty {
                self.tiles[(self.rows - 1) * self.cols + j] = Type::Empty;
                self.tiles[j] = Type::South;
                cnt += 1;
            }
        }
        cnt
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.rows {
            writeln!(
                f,
                "{}",
                self.tiles[r * self.cols..(r + 1) * self.cols]
                    .iter()
                    .map(|c| match c {
                        Type::Empty => '.',
                        Type::East => '>',
                        Type::South => 'v',
                    })
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

fn main() {
    let f = std::io::BufReader::new(std::io::stdin());
    let mut m = Map::parse(f);

    for i in 1.. {
        if m.step() == 0 {
            println!("{}", i);
            return;
        }
    }
}

#[test]
fn test() {
    let mut m = Map::parse(
        "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"
            .as_bytes(),
    );

    for i in 1.. {
        if m.step() == 0 {
            assert_eq!(i, 58);
            return;
        }
    }
}

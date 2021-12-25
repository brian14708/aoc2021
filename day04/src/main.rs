use std::{collections::HashMap, io::BufRead};

use anyhow::{anyhow, Result};

#[derive(Debug)]
struct Position {
    id: usize,
    row: i16,
    col: i16,
}

#[derive(Debug)]
struct BoardState {
    data: HashMap<usize, Vec<i32>>,
    by_value: HashMap<i32, Vec<Position>>,
    size: usize,
    num_boards: usize,
}

struct SolverIter<'a> {
    marked: Vec<i32>,
    state: &'a BoardState,
    seq: Vec<i32>,
    curr_idx: usize,
    result: Vec<(usize, i32)>,
}

impl Iterator for SolverIter<'_> {
    type Item = (usize, i32); // (id, score)

    fn next(&mut self) -> Option<Self::Item> {
        while self.curr_idx < self.seq.len() && self.result.is_empty() {
            let s = self.seq[self.curr_idx];
            let sz = self.state.size;

            if let Some(b) = self.state.by_value.get(&s) {
                for p in b {
                    let base = p.id * sz * 2;
                    if self.marked[base] == -1 {
                        continue;
                    }

                    let idx = base + p.row as usize;
                    self.marked[idx] -= 1;
                    if self.marked[idx] == 0 {
                        self.result.push((p.id, 0));
                        continue;
                    }

                    let idx = base + sz + p.col as usize;
                    self.marked[idx] -= 1;
                    if self.marked[idx] == 0 {
                        self.result.push((p.id, 0));
                    }
                }
            }

            if !self.result.is_empty() {
                let seq = &mut self.seq[..=self.curr_idx];
                seq.sort_unstable();

                for (m, score) in &mut self.result {
                    self.marked[(*m * sz * 2)..((*m + 1) * sz * 2)].fill(-1);

                    *score = s * self.state.data[m]
                        .iter()
                        .filter(|&v| seq.binary_search(v).is_err())
                        .sum::<i32>();
                }
            }

            self.curr_idx += 1;
        }
        self.result.pop()
    }
}

impl BoardState {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
            by_value: HashMap::new(),
            size: 0,
            num_boards: 0,
        }
    }

    fn parse_board(&mut self, lines: impl Iterator<Item = String>) -> Result<bool> {
        let mut parsed = 0;
        for (row, l) in lines.enumerate() {
            if l.is_empty() {
                break;
            }

            for (col, c) in l.split_ascii_whitespace().enumerate() {
                let n = c.parse()?;
                self.by_value
                    .entry(n)
                    .or_insert_with(Vec::new)
                    .push(Position {
                        id: self.num_boards,
                        row: row as i16,
                        col: col as i16,
                    });
                self.data
                    .entry(self.num_boards)
                    .or_insert_with(Vec::new)
                    .push(n);
            }
            parsed += 1;
        }
        if parsed == 0 {
            return Ok(false);
        }

        self.size = parsed;
        self.num_boards += 1;
        Ok(true)
    }

    fn solve_bingo(&self, seq: Vec<i32>) -> impl Iterator<Item = (usize, i32)> + '_ {
        SolverIter {
            marked: vec![self.size as i32; self.num_boards * self.size * 2],
            state: self,
            seq,
            curr_idx: 0,
            result: Vec::new(),
        }
    }
}

fn parse(f: impl BufRead) -> Result<(Vec<i32>, BoardState)> {
    let mut m = vec![];
    let mut lines = f.lines().filter_map(std::result::Result::ok);
    {
        let first_line = lines.next().ok_or(anyhow!("missing line"))?;
        for c in first_line.split(',') {
            m.push(c.parse()?);
        }
    }
    // skip next line
    lines.next();

    let mut b = BoardState::new();
    while b.parse_board(&mut lines)? {}
    Ok((m, b))
}

fn main() -> Result<()> {
    let (m, b) = parse(std::io::BufReader::new(std::io::stdin()))?;

    let mut ret = b.solve_bingo(m);

    let (_, score) = ret.next().ok_or(anyhow!("cannot find first"))?;
    let (_, last_score) = ret.last().ok_or(anyhow!("cannot find last"))?;
    println!("first win score: {}", score);
    println!("last win score: {}", last_score);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bingo() {
        let f = include_bytes!("../test/input.txt");
        let (m, b) = parse(&f[..]).unwrap();
        assert_eq!(m.len(), 27);
        assert_eq!(b.num_boards, 3);
        let mut ret = b.solve_bingo(m);
        assert_eq!(ret.next().unwrap(), (2, 4512));
        assert_eq!(ret.last().unwrap(), (1, 1924));
    }
}

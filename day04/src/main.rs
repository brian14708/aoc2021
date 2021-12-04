use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

use anyhow::{anyhow, Result};

#[derive(Debug)]
struct Position {
    id: i32,
    row: i16,
    col: i16,
}

#[derive(Debug)]
struct BoardState {
    data: HashMap<i32, Vec<i32>>,
    by_value: HashMap<i32, Vec<Position>>,
    size: i32,
    num_boards: i32,
}

struct SolverIter<'a> {
    marked: Vec<i32>,
    winner_ids: HashSet<i32>,
    state: &'a BoardState,
    seq: &'a Vec<i32>,
    curr_idx: usize,
    result: VecDeque<(i32, i32)>,
}

impl Iterator for SolverIter<'_> {
    type Item = (i32, i32); // (id, score)

    fn next(&mut self) -> Option<Self::Item> {
        while self.curr_idx < self.seq.len() && self.result.len() == 0 {
            let s = self.seq[self.curr_idx];

            if let Some(b) = self.state.by_value.get(&s) {
                for p in b {
                    if self.winner_ids.get(&p.id) != None {
                        continue;
                    }

                    let idx = (p.id * self.state.size * 2 + p.row as i32) as usize;
                    self.marked[idx] += 1;
                    if self.marked[idx] == self.state.size {
                        self.result.push_back((p.id, -1));
                        continue;
                    }

                    let idx =
                        (p.id * self.state.size * 2 + self.state.size + p.col as i32) as usize;
                    self.marked[idx] += 1;
                    if self.marked[idx] == self.state.size {
                        self.result.push_back((p.id, -1));
                    }
                }
            }

            if self.result.len() > 0 {
                let marked_nums: HashSet<_> = self.seq[..self.curr_idx + 1].iter().collect();
                for (m, score) in self.result.iter_mut() {
                    self.winner_ids.insert(*m);
                    *score = s * self.state.data[m as &i32]
                        .iter()
                        .filter(|&v| marked_nums.get(v).is_none())
                        .sum::<i32>();
                }
            }

            self.curr_idx += 1;
        }
        return self.result.pop_front();
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

    fn parse_board(self: &mut Self, lines: impl Iterator<Item = String>) -> Result<bool> {
        let mut parsed = 0;
        for (row, l) in lines.enumerate() {
            if l.len() == 0 {
                break;
            }

            for (col, c) in l.split_ascii_whitespace().enumerate() {
                let n = c.parse()?;
                self.by_value.entry(n).or_insert(vec![]).push(Position {
                    id: self.num_boards,
                    row: row as i16,
                    col: col as i16,
                });
                self.data.entry(self.num_boards).or_insert(vec![]).push(n);
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

    fn solve_bingo<'a>(self: &'a Self, seq: &'a Vec<i32>) -> impl Iterator<Item = (i32, i32)> + 'a {
        SolverIter {
            marked: vec![0; (self.num_boards * self.size * 2) as usize],
            winner_ids: HashSet::new(),
            state: self,
            seq: &seq,
            curr_idx: 0,
            result: VecDeque::new(),
        }
    }
}

fn parse(f: impl BufRead) -> Result<(Vec<i32>, BoardState)> {
    let mut m = vec![];
    let mut lines = f.lines().filter_map(|l| l.ok());
    {
        let first_line = lines.next().ok_or(anyhow!("missing line"))?;
        for c in first_line.split(",") {
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

    let mut ret = b.solve_bingo(&m);

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
        let mut ret = b.solve_bingo(&m);
        assert_eq!(ret.next().unwrap(), (2, 4512));
        assert_eq!(ret.last().unwrap(), (1, 1924));
    }
}

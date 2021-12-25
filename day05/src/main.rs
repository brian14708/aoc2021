use std::cmp::{max, min};
use std::io::BufRead;
#[macro_use]
extern crate scan_fmt;

struct Line {
    x: i32,
    y: i32,
    steps: i32,
    dx: i32,
    dy: i32,
}

struct Box {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Line {
    fn parse(s: &str) -> Option<Line> {
        let (x1, y1, x2, y2) = scan_fmt!(s, "{},{} -> {},{}", i32, i32, i32, i32).ok()?;
        Some(Line {
            x: x1,
            y: y1,
            steps: max((x2 - x1).abs(), (y2 - y1).abs()),
            dx: (x2 - x1).signum(),
            dy: (y2 - y1).signum(),
        })
    }

    fn points(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        (0..=self.steps).scan((self.x, self.y), |(x, y), _| {
            let curr = (*x, *y);
            *x += self.dx;
            *y += self.dy;
            Some(curr)
        })
    }

    fn bbox(&self) -> Box {
        let (x, y) = (self.x + self.steps * self.dx, self.y + self.steps * self.dy);
        Box {
            x1: min(self.x, x),
            y1: min(self.y, y),
            x2: max(self.x, x),
            y2: max(self.y, y),
        }
    }
}

impl Box {
    fn min() -> Self {
        Self {
            x1: i32::MAX,
            y1: i32::MAX,
            x2: i32::MIN,
            y2: i32::MIN,
        }
    }

    fn union(&self, r: &Self) -> Self {
        Self {
            x1: min(self.x1, r.x1),
            y1: min(self.y1, r.y1),
            x2: max(self.x2, r.x2),
            y2: max(self.y2, r.y2),
        }
    }
}

fn parse(f: impl BufRead) -> Vec<Line> {
    f.lines()
        .filter_map(|l| Line::parse(l.ok()?.as_str()))
        .collect()
}

fn bbox<'a>(lines: impl Iterator<Item = &'a Line>) -> Box {
    lines.fold(Box::min(), |b, l| b.union(&l.bbox()))
}

fn solve<'a>(bbox: &Box, lines: impl Iterator<Item = &'a Line>) -> usize {
    let w = bbox.x2 - bbox.x1 + 1;
    let h = bbox.y2 - bbox.y1 + 1;

    let mut grid = vec![0u8; (w * h) as usize];
    lines.flat_map(Line::points).for_each(|(x, y)| {
        grid[((y - bbox.y1) * w + (x - bbox.x1)) as usize] += 1;
    });
    grid.into_iter().filter(|&c| c >= 2).count()
}

fn main() {
    let lines = parse(std::io::BufReader::new(std::io::stdin()));
    let bbox = bbox(lines.iter());

    println!(
        "{}",
        solve(&bbox, lines.iter().filter(|l| l.dx == 0 || l.dy == 0))
    );
    println!("{}", solve(&bbox, lines.iter()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_count() {
        let f = include_bytes!("../test/input.txt");
        let l = parse(&f[..]);
        let bbox = bbox(l.iter());

        assert_eq!(solve(&bbox, l.iter().filter(|l| l.dx == 0 || l.dy == 0)), 5);
        assert_eq!(solve(&bbox, l.iter()), 12);
    }
}

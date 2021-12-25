#[macro_use]
extern crate scan_fmt;
use std::{
    collections::{BTreeSet, VecDeque},
    error::Error,
    fmt::Display,
    io::BufRead,
};

#[derive(Debug)]
struct Origami {
    dots: BTreeSet<(i32, i32)>,
    fold: VecDeque<(char, i32)>,
}

impl Origami {
    fn parse(f: impl BufRead) -> Result<Self, Box<dyn Error>> {
        let mut inp = Self {
            dots: BTreeSet::new(),
            fold: VecDeque::new(),
        };

        let mut lines = f.lines();
        for l in &mut lines {
            let l = l?;
            if l.is_empty() {
                break;
            }
            let (x, y) = scan_fmt!(&l, "{},{}", i32, i32)?;
            inp.dots.insert((y, x));
        }
        for l in &mut lines {
            let l = l?;
            inp.fold
                .push_back(scan_fmt!(&l, "fold along {}={}", char, i32)?);
        }

        Ok(inp)
    }

    fn fold_one(&mut self) -> bool {
        if let Some((d, pos)) = self.fold.pop_front() {
            let mut tmp = vec![];
            match d {
                'x' => {
                    self.dots.retain(|&(y, x)| {
                        if x < pos {
                            true
                        } else {
                            tmp.push((y, 2 * pos - x));
                            false
                        }
                    });
                }
                'y' => {
                    self.dots.retain(|&(y, x)| {
                        if y < pos {
                            true
                        } else {
                            tmp.push((2 * pos - y, x));
                            false
                        }
                    });
                }
                _ => panic!("invalid direction"),
            }
            self.dots.extend(tmp.iter());

            true
        } else {
            false
        }
    }

    fn fold(&mut self) {
        while self.fold_one() {}
    }
}

impl Display for Origami {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut prev = (0, 0);
        for &(y, x) in &self.dots {
            while y != prev.0 {
                writeln!(f)?;
                prev.0 += 1;
                prev.1 = 0;
            }
            while x != prev.1 {
                write!(f, " ")?;
                prev.1 += 1;
            }
            write!(f, "#")?;
            prev.1 += 1;
        }
        Ok(())
    }
}

fn main() {
    let mut o = Origami::parse(std::io::BufReader::new(std::io::stdin())).unwrap();
    o.fold_one();
    println!("{}", o.dots.len());
    o.fold();
    println!("{}", o);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold_one() {
        let f = include_bytes!("../test/input.txt");
        let mut o = Origami::parse(&f[..]).unwrap();
        o.fold_one();
        assert_eq!(o.dots.len(), 17);
    }
}

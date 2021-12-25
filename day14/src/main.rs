use std::{collections::HashMap, error::Error, io::BufRead};

#[macro_use]
extern crate scan_fmt;

struct Polymer {
    template: HashMap<(char, char), usize>,
    rules: HashMap<(char, char), char>,
}

impl Polymer {
    fn parse(f: impl BufRead) -> Result<Self, Box<dyn Error>> {
        let mut inp = Self {
            template: HashMap::new(),
            rules: HashMap::new(),
        };

        let mut lines = f.lines();
        if let Some(l) = lines.next() {
            let last = l?.chars().fold('\0', |prev, curr| {
                *inp.template.entry((prev, curr)).or_insert(0) += 1;
                curr
            });
            *inp.template.entry((last, '\0')).or_insert(0) += 1;
        }
        lines.next();
        for l in lines {
            let (a, b, c) = scan_fmt!(&l?, "{/[A-Z]/}{} -> {}", char, char, char)?;
            inp.rules.insert((a, b), c);
        }

        Ok(inp)
    }

    fn step(&mut self) {
        let old = std::mem::take(&mut self.template);
        for (k, v) in old {
            if let Some(&replace) = self.rules.get(&k) {
                *self.template.entry((k.0, replace)).or_insert(0) += v;
                *self.template.entry((replace, k.1)).or_insert(0) += v;
            } else {
                *self.template.entry(k).or_insert(0) += v;
            }
        }
    }

    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.template.iter().map(|(_, &s)| s).sum::<usize>() - 1
    }

    fn histogram(&self) -> Vec<(char, usize)> {
        let mut cnt = HashMap::new();
        self.template.iter().for_each(|(&(a, b), &v)| {
            *cnt.entry(a).or_insert(0) += v;
            *cnt.entry(b).or_insert(0) += v;
        });
        cnt.remove(&'\0');

        let mut cnt: Vec<_> = cnt.into_iter().map(|(c, cnt)| (c, cnt / 2)).collect();
        cnt.sort_unstable_by_key(|&(_, c)| -(c as i64));
        cnt
    }
}

fn main() {
    let mut p = Polymer::parse(std::io::BufReader::new(std::io::stdin())).unwrap();
    for _ in 0..10 {
        p.step();
    }
    let h = p.histogram();
    println!("{}", h[0].1 - h[h.len() - 1].1);
    for _ in 0..30 {
        p.step();
    }
    let h = p.histogram();
    println!("{}", h[0].1 - h[h.len() - 1].1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polymer() {
        let f = include_bytes!("../test/input.txt");
        let mut p = Polymer::parse(&f[..]).unwrap();
        assert_eq!(p.len(), 4);
        p.step();
        assert_eq!(p.len(), 7);
        for _ in 0..9 {
            p.step();
        }
        assert_eq!(p.len(), 3073);
        let h = p.histogram();
        assert_eq!(h[0].1, 1749);
        assert_eq!(h[h.len() - 1].1, 161);
        for _ in 0..30 {
            p.step();
        }
        let h = p.histogram();
        assert_eq!(h[0].1, 2_192_039_569_602);
        assert_eq!(h[h.len() - 1].1, 3_849_876_073);
    }
}

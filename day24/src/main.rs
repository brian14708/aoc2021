use std::collections::HashMap;
mod eval;

struct Countdown<const N: usize> {
    curr: Option<[i8; N]>,
    rules: HashMap<usize, (usize, i8)>,
}

impl<const N: usize> Countdown<N> {
    fn new(rules: HashMap<usize, (usize, i8)>) -> Self {
        Self {
            curr: Some([9; N]),
            rules,
        }
    }
}

impl<const N: usize> Iterator for Countdown<N> {
    type Item = [i8; N];

    fn next(&mut self) -> Option<Self::Item> {
        'outer: loop {
            if let Some(curr) = self.curr.as_mut() {
                for i in (0..N).rev() {
                    if self.rules.get(&i).is_some() {
                        continue;
                    }

                    if curr[i] == 1 {
                        if i == 0 {
                            self.curr = None;
                            return None;
                        } else {
                            curr[i] = 9;
                        }
                    } else {
                        curr[i] -= 1;
                        break;
                    }
                }
            }

            let m = self.curr.as_mut().unwrap();
            for (&i, &(j, diff)) in &self.rules {
                let c = m[j] + diff;
                m[i] = c;
                if c <= 0 || c >= 10 {
                    m[j + 1..N].fill(1);
                    continue 'outer;
                }
            }
            return self.curr;
        }
    }
}

fn main() {
    let mut c = Countdown::<14>::new(HashMap::from_iter([
        (13, (0, -8)),
        (12, (1, -2)),
        (3, (2, 7)),
        (5, (4, -4)),
        (11, (6, 8)),
        (10, (7, 6)),
        (9, (8, 1)),
    ]));

    let first = c.next().unwrap();
    let last = c.last().unwrap();
    println!("{}", first.map(|c| format!("{}", c)).join(""));
    println!("{}", last.map(|c| format!("{}", c)).join(""));
}

#[test]
fn test() {
    let mut c = Countdown::<14>::new(HashMap::from_iter([
        (13, (0, -8)),
        (12, (1, -2)),
        (3, (2, 7)),
        (5, (4, -4)),
        (11, (6, 8)),
        (10, (7, 6)),
        (9, (8, 1)),
    ]));

    let first = c.next().unwrap();
    let last = c.last().unwrap();
    assert_eq!(eval::program(first), 0);
    assert_eq!(eval::program(last), 0);
}

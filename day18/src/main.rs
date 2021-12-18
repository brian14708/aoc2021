use std::{fmt::Display, io::BufRead, iter::Sum, ops::Add};

mod parse;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Expr {
    Number(u32),
    Nested { left: Box<Expr>, right: Box<Expr> },
    Invalid,
}

impl Expr {
    fn parse(s: &str) -> Self {
        Self::parse_impl(s).unwrap().1
    }

    #[allow(dead_code)]
    fn parse_reduce(s: &str) -> Self {
        let mut m = Self::parse_impl(s).unwrap().1;
        m.reduce();
        m
    }

    fn parse_impl(s: &str) -> Option<(&str, Self)> {
        if let Some(s) = parse::consume(s, '[') {
            let (s, left) = Self::parse_impl(s)?;
            let s = parse::consume(s, ',')?;
            let (s, right) = Self::parse_impl(s)?;
            let s = parse::consume(s, ']')?;
            Some((
                s,
                Self::Nested {
                    left: Box::new(left),
                    right: Box::new(right),
                },
            ))
        } else if let Some((s, n)) = parse::take_number(s) {
            Some((s, Self::Number(n)))
        } else {
            None
        }
    }

    fn to_number(&self) -> Option<u32> {
        if let Expr::Number(n) = *self {
            Some(n)
        } else {
            None
        }
    }

    fn reduce(&mut self) {
        while self
            .reduce_explode(0)
            .map(|_| ())
            .or_else(|| self.reduce_split())
            .is_some()
        {}
    }

    fn add_num(
        &mut self,
        s: u32,
        select: impl for<'a> Fn(&'a mut Box<Expr>, &'a mut Box<Expr>) -> &'a mut Box<Expr>,
    ) {
        match self {
            Expr::Number(o) => *o += s,
            Expr::Nested { left, right } => {
                select(left, right).add_num(s, select);
            }
            Expr::Invalid => panic!("invalid expr"),
        }
    }

    fn reduce_explode(&mut self, depth: usize) -> Option<(Option<u32>, Option<u32>)> {
        if let Expr::Nested { left, right } = self {
            if depth >= 4 {
                if let Some((x, y)) = left.to_number().and_then(|l| Some((l, right.to_number()?))) {
                    *self = Expr::Number(0);
                    return Some((Some(x), Some(y)));
                }
            }

            if let Some((ll, lr)) = left.reduce_explode(depth + 1) {
                if let Some(r) = lr {
                    right.add_num(r, |l, _| l);
                }
                return Some((ll, None));
            }
            if let Some((rl, rr)) = right.reduce_explode(depth + 1) {
                if let Some(l) = rl {
                    left.add_num(l, |_, r| r);
                }
                return Some((None, rr));
            }
        }

        None
    }

    fn reduce_split(&mut self) -> Option<()> {
        match self {
            &mut Expr::Number(n) => {
                if n >= 10 {
                    // split
                    *self = Expr::Nested {
                        left: Box::new(Expr::Number(n / 2)),
                        right: Box::new(Expr::Number(n - n / 2)),
                    };
                    Some(())
                } else {
                    None
                }
            }
            Expr::Nested { left, right } => left.reduce_split().or_else(|| right.reduce_split()),
            Expr::Invalid => panic!("invalid expr"),
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Expr::Number(n) => *n,
            Expr::Nested { left, right } => left.magnitude() * 3 + right.magnitude() * 2,
            Expr::Invalid => panic!("invalid expr"),
        }
    }
}

impl Add for Expr {
    type Output = Self;

    fn add(self, rhs: Expr) -> Self::Output {
        let mut e = Expr::Nested {
            left: Box::new(self),
            right: Box::new(rhs),
        };
        e.reduce();
        e
    }
}

impl Sum for Expr {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap_or(Expr::Invalid)
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(s) => write!(f, "{}", s),
            Self::Nested { left, right } => write!(f, "[{},{}]", *left, *right),
            Expr::Invalid => panic!("invalid expr"),
        }
    }
}

fn max_pair_magnitude(v: &[Expr]) -> Option<u32> {
    v.iter()
        .flat_map(|a| {
            v.iter().filter_map(move |b| {
                if a == b {
                    None
                } else {
                    Some((a.clone() + b.clone()).magnitude())
                }
            })
        })
        .max()
}

fn main() {
    let f = std::io::BufReader::new(std::io::stdin());
    let es: Vec<_> = f.lines().flatten().map(|l| Expr::parse(&l)).collect();

    let pair_mag = max_pair_magnitude(&es).unwrap();
    println!("{}", es.into_iter().sum::<Expr>().magnitude());
    println!("{}", pair_mag);
}

#[cfg(test)]
mod tests {
    use std::io::BufRead;

    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Expr::parse("[1,1]"),
            Expr::Nested {
                left: Box::new(Expr::Number(1)),
                right: Box::new(Expr::Number(1)),
            }
        );
        assert_eq!(
            format!(
                "{}",
                Expr::parse_impl("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]")
                    .map(|(_, s)| s)
                    .unwrap()
            ),
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
        );
    }

    #[test]
    fn test_add() {
        assert_eq!(
            format!("{}", Expr::parse("[1,2]") + Expr::parse("[[3,4],5]")),
            "[[1,2],[[3,4],5]]"
        );
    }

    #[test]
    fn test_split() {
        assert_eq!(
            format!("{}", Expr::parse_reduce("[10,11]")),
            "[[5,5],[5,6]]"
        );
    }

    #[test]
    fn test_explode() {
        assert_eq!(
            format!(
                "{}",
                Expr::parse_reduce("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")
            ),
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
        );
        assert_eq!(
            format!("{}", Expr::parse_reduce("[[[[[9,8],1],2],3],4]")),
            "[[[[0,9],2],3],4]"
        );
        assert_eq!(
            format!("{}", Expr::parse_reduce("[7,[6,[5,[4,[3,2]]]]]]")),
            "[7,[6,[5,[7,0]]]]"
        );
        assert_eq!(
            format!("{}", Expr::parse_reduce("[[6,[5,[4,[3,2]]]],1]")),
            "[[6,[5,[7,0]]],3]"
        );
        assert_eq!(
            format!(
                "{}",
                Expr::parse("[[[[4,3],4],4],[7,[[8,4],9]]]") + Expr::parse("[1,1]")
            ),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
        );
        assert_eq!(
            format!(
                "{}",
                Expr::parse("[1,1]")
                    + Expr::parse("[2,2]")
                    + Expr::parse("[3,3]")
                    + Expr::parse("[4,4]")
                    + Expr::parse("[5,5]")
            ),
            "[[[[3,0],[5,3]],[4,4]],[5,5]]"
        );
        assert_eq!(
            format!(
                "{}",
                Expr::parse("[1,1]")
                    + Expr::parse("[2,2]")
                    + Expr::parse("[3,3]")
                    + Expr::parse("[4,4]")
                    + Expr::parse("[5,5]")
                    + Expr::parse("[6,6]")
            ),
            "[[[[5,0],[7,4]],[5,5]],[6,6]]"
        );
    }

    #[test]
    fn test_sum() {
        let f = include_bytes!("../test/input1.txt");
        let m = f.lines().flatten().map(|l| Expr::parse(&l)).sum::<Expr>();

        assert_eq!(
            format!("{}", m),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        );
    }

    #[test]
    fn test_magnitude() {
        let f = include_bytes!("../test/input2.txt");
        let m: Vec<_> = f.lines().flatten().map(|l| Expr::parse(&l)).collect();

        assert_eq!(max_pair_magnitude(&m).unwrap(), 3993);

        let m: Expr = m.into_iter().sum();
        assert_eq!(
            format!("{}", m),
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
        );
        assert_eq!(m.magnitude(), 4140);
    }
}

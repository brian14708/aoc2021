use std::io::BufRead;

#[derive(Clone, Copy)]
struct ParseError {
    found: char,
}

fn parse_expr(l: &str) -> Result<String, ParseError> {
    let mut stack = vec![];
    let expect = |stack: &mut Vec<char>, e: char, c: char| -> Result<(), ParseError> {
        let last = stack.pop();
        if last == (if e == '\0' { None } else { Some(e) }) {
            Ok(())
        } else {
            Err(ParseError { found: c })
        }
    };

    for c in l.chars() {
        match c {
            '(' | '{' | '[' | '<' => stack.push(c),
            ')' => expect(&mut stack, '(', ')')?,
            ']' => expect(&mut stack, '[', ']')?,
            '}' => expect(&mut stack, '{', '}')?,
            '>' => expect(&mut stack, '<', '>')?,
            _ => {}
        }
    }

    // missing chars
    Ok(stack
        .iter()
        .rev()
        .map(|&c| match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("bad char"),
        })
        .collect::<String>())
}

fn score_corrupt(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("bad char"),
    }
}

fn score_incomplete(s: &str) -> i64 {
    s.chars()
        .map(|c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("bad char"),
        })
        .fold(0, |a, b| a * 5 + b)
}

fn main() {
    let inp = std::io::BufReader::new(std::io::stdin());
    let m = inp
        .lines()
        .flatten()
        .map(|s| parse_expr(&s))
        .collect::<Vec<_>>();
    println!(
        "{}",
        m.iter()
            .filter_map(|r| { r.as_ref().err().map(|p| score_corrupt(p.found)) })
            .sum::<i32>()
    );

    let mut l = m
        .iter()
        .filter_map(|r| r.as_ref().ok().map(|p| score_incomplete(p)))
        .collect::<Vec<_>>();
    let n = l.len() / 2;
    println!("{}", *l.select_nth_unstable(n).1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segments() {
        let f = include_bytes!("../test/input.txt");
        let m = &f[..]
            .lines()
            .flatten()
            .map(|s| parse_expr(&s))
            .collect::<Vec<_>>();
        assert_eq!(
            m.iter()
                .filter_map(|r| { r.as_ref().err().map(|p| score_corrupt(p.found)) })
                .sum::<i32>(),
            26397
        );

        let mut l = m
            .iter()
            .filter_map(|r| r.as_ref().ok().map(|p| score_incomplete(p)))
            .collect::<Vec<_>>();
        let n = l.len() / 2;
        assert_eq!(*l.select_nth_unstable(n).1, 288_957);
    }
}

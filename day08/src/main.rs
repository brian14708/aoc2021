use itertools::Itertools;
use std::{collections::HashSet, io::BufRead};

fn parse(f: impl BufRead) -> Vec<(Vec<String>, Vec<String>)> {
    let mut ret = vec![];
    for l in f.lines().flatten() {
        let mut l = l.split(" | ");
        let input = l.next();
        if let Some(output) = l.next() {
            let output = output.split(' ').map(String::from).collect::<Vec<_>>();
            ret.push((
                input
                    .unwrap()
                    .split(' ')
                    .map(String::from)
                    .chain(output.iter().cloned())
                    .collect::<Vec<_>>(),
                output,
            ))
        }
    }
    ret
}

fn count_1478(s: &[String]) -> usize {
    s.iter()
        .filter(|s| matches!(s.len(), 2 | 4 | 3 | 7))
        .count()
}

fn char_idx(u: char) -> usize {
    u as usize - 'a' as usize
}

fn solve_mapping(all: &[String]) -> [char; 7] {
    let mut mapping: [HashSet<char>; 7] = Default::default();
    for i in mapping.iter_mut() {
        i.extend('a'..='g')
    }

    let mut retain_chars = |base: &str, set: &str| {
        for c in base.chars() {
            mapping[char_idx(c)].retain(|&c| set.find(c).is_some())
        }
    };

    let mut adg = HashSet::<char>::from_iter('a'..='g'); // possible candidates for "adg"
    let mut abfg = HashSet::<char>::from_iter('a'..='g'); // possible candidates for "abfg"
    for word in all {
        match word.len() {
            2 => retain_chars(word, "cf"),                 // 1
            4 => retain_chars(word, "bcdf"),               // 4
            3 => retain_chars(word, "acf"),                // 7
            5 => adg.retain(|&c| word.find(c).is_some()),  // 2, 3, 5
            6 => abfg.retain(|&c| word.find(c).is_some()), // 0, 6, 9
            _ => {}                                        // 8, ignore
        };
    }
    retain_chars(&adg.iter().collect::<String>(), "adg");
    retain_chars(&abfg.iter().collect::<String>(), "abfg");

    let mut ret = ['\0'; 7];
    let mut determined = HashSet::new();
    while determined.len() != ret.len() {
        for (i, m) in mapping.iter().enumerate() {
            if m.len() == 1 {
                let c = *m.iter().next().unwrap();
                ret[i] = c;
                determined.insert(c);
            }
        }

        for m in mapping.iter_mut() {
            m.retain(|c| determined.get(c).is_none())
        }
    }

    ret
}

fn get_number(word: &str, mapping: [char; 7]) -> Option<i32> {
    match word.len() {
        2 => Some(1),
        4 => Some(4),
        3 => Some(7),
        7 => Some(8),
        _ => match word
            .chars()
            .map(|c| mapping[char_idx(c)])
            .sorted()
            .collect::<String>()
            .as_str()
        {
            "abcefg" => Some(0),
            "acdeg" => Some(2),
            "acdfg" => Some(3),
            "abdfg" => Some(5),
            "abdefg" => Some(6),
            "abcdfg" => Some(9),
            _ => None,
        },
    }
}

fn main() {
    let inp = parse(std::io::BufReader::new(std::io::stdin()));
    println!(
        "{}",
        inp.iter().map(|(_, s)| { count_1478(s) }).sum::<usize>(),
    );
    let mut tot = 0;
    for (all, out) in inp {
        let mapping = solve_mapping(&all);
        tot += out
            .iter()
            .map(|s| get_number(s, mapping))
            .fold(0, |a, v| a * 10 + v.unwrap());
    }
    println!("{}", tot);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segments() {
        let f = include_bytes!("../test/input.txt");
        let inp = parse(&f[..]);
        assert_eq!(
            inp.iter().map(|(_, s)| { count_1478(s) }).sum::<usize>(),
            26,
        );
        let mut ret = vec![];
        for (all, out) in inp {
            let mapping = solve_mapping(&all);
            ret.push(
                out.iter()
                    .map(|s| get_number(s, mapping))
                    .fold(0, |a, v| a * 10 + v.unwrap()),
            );
        }
        assert_eq!(
            ret,
            vec![5353, 8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315]
        );
        assert_eq!(ret.iter().sum::<i32>(), 5353 + 61229);
    }
}

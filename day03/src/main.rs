use std::{fs::File, io::BufReader};

fn parse(f: impl std::io::BufRead) -> std::io::Result<(Vec<usize>, usize)> {
    let mut val = vec![];
    let mut bitlen = 0;
    for l in f.lines() {
        let l = l?;
        bitlen = l.len();
        val.push(
            l.chars()
                .fold(0, |u, c| (u << 1) + if c == '1' { 1 } else { 0 }),
        );
    }
    val.sort_unstable();
    Ok((val, bitlen))
}

fn bit_stat(vals: &[usize], bitlen: usize) -> (usize, usize) {
    let mut cnt = vec![0; bitlen];
    for v in vals {
        for (i, c) in cnt.iter_mut().enumerate() {
            if v & (1 << i) == 0 {
                *c -= 1;
            } else {
                *c += 1;
            }
        }
    }

    let mcb = cnt
        .iter()
        .rev()
        .fold(0, |m, &c| (m << 1) + if c >= 0 { 1 } else { 0 });

    (mcb, (1 << bitlen) - 1 - mcb)
}

fn traverse_partition(
    s: &[usize],
    pos: usize,
    recur: for<'a> fn(l: &'a [usize], r: &'a [usize]) -> &'a [usize],
) -> usize {
    if s.len() == 1 {
        return s[0];
    }

    let p = s.partition_point(|&u| (u & (1 << (pos - 1))) == 0);
    traverse_partition(recur(&s[..p], &s[p..]), pos - 1, recur)
}

fn max_len<'a>(l: &'a [usize], r: &'a [usize]) -> &'a [usize] {
    if l.len() <= r.len() {
        r
    } else {
        l
    }
}

fn min_len<'a>(l: &'a [usize], r: &'a [usize]) -> &'a [usize] {
    if l.len() <= r.len() {
        l
    } else {
        r
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("{} [filename]", args[0]);
        return Ok(());
    }

    let f = File::open(&args[1])?;
    let (vals, bitlen) = parse(BufReader::new(f))?;
    let (mcb, lcb) = bit_stat(&vals, bitlen);
    let o = traverse_partition(&vals, bitlen, max_len);
    let c = traverse_partition(&vals, bitlen, min_len);
    println!("{} {} ", mcb * lcb, o * c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits() {
        let f = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            .as_bytes();

        let (vals, bitlen) = parse(f).unwrap();
        assert_eq!(bit_stat(&vals, bitlen), (22, 9));
        assert_eq!(traverse_partition(&vals, bitlen, max_len), 23);
        assert_eq!(traverse_partition(&vals, bitlen, min_len), 10);
    }
}

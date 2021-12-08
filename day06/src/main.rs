use std::io::BufRead;

fn count_fish(curr: &[usize], rate: usize, delay: usize, days: usize) -> u64 {
    let mut buckets = vec![0; delay + rate];
    for c in curr {
        buckets[*c] += 1;
    }

    for _ in 0..days {
        let birth = buckets[0];
        buckets[0] = 0;
        buckets.rotate_left(1);
        buckets[rate - 1] += birth;
        buckets[delay - 1] += birth;
    }
    buckets.iter().sum()
}

fn main() {
    for l in std::io::BufReader::new(std::io::stdin()).lines().flatten() {
        let inp = l
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<_>>();
        println!("{}", count_fish(&inp, 7, 9, 80));
        println!("{}", count_fish(&inp, 7, 9, 256));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_fish() {
        let inp = vec![3, 4, 3, 1, 2];
        assert_eq!(count_fish(&inp, 7, 9, 18), 26);
        assert_eq!(count_fish(&inp, 7, 9, 80), 5934);
        assert_eq!(count_fish(&inp, 7, 9, 256), 26984457539);
    }
}

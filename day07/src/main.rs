use std::io::BufRead;

fn median<T: Copy + std::cmp::Ord>(v: &mut Vec<T>) -> Option<T> {
    match v.len() {
        0 => None,
        1 => Some(v[0]),
        l => Some(*v.select_nth_unstable(l / 2).1),
    }
}

fn avg<'a, T: 'a + Copy + num_traits::sign::Signed + std::convert::Into<f64>>(
    v: impl Iterator<Item = &'a T> + std::iter::ExactSizeIterator<Item = &'a T>,
) -> Option<f64> {
    let l = v.len();
    let s = v.map(|&v| v).reduce(|a, v| (a + v))?;
    Some((s.into() as f64) / (l as f64))
}

fn l1_dist<'a, T: 'a + Copy + num_traits::sign::Signed>(
    v: impl Iterator<Item = &'a T>,
    t: T,
) -> Option<T> {
    let t = -t;
    v.map(|&v| (v + t).abs()).reduce(|a, v| a + v)
}

fn sum_dist<'a, T: 'a + Copy + num_traits::sign::Signed>(
    v: impl Iterator<Item = &'a T>,
    t: T,
) -> Option<T> {
    let t = -t;
    v.map(|&v| {
        let diff = (v + t).abs();
        (diff * diff + diff) / (T::one() + T::one())
    })
    .reduce(|a, v| a + v)
}

fn solve_part2(inp: &Vec<i32>) -> Option<i32> {
    let avg = avg(inp.iter())?;
    Some(std::cmp::min(
        sum_dist(inp.iter(), avg.floor() as i32)?,
        sum_dist(inp.iter(), avg.ceil() as i32)?,
    ))
}

fn main() {
    for l in std::io::BufReader::new(std::io::stdin()).lines() {
        if let Ok(l) = l {
            let mut inp = l
                .split(",")
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<i32>>();
            let m = median(&mut inp).unwrap();
            println!("{}", l1_dist(inp.iter(), m).unwrap());
            println!("{}", solve_part2(&inp).unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median() {
        let mut inp = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let m = median(&mut inp).unwrap();
        assert_eq!(m, 2);
        assert_eq!(l1_dist(inp.iter(), 2).unwrap(), 37);
        assert_eq!(solve_part2(&inp).unwrap(), 168);
    }
}

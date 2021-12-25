use std::io::BufRead;

fn parse(f: impl BufRead) -> (Vec<u8>, usize, usize) {
    let mut rows = 0;
    let mut cols = 0;
    let mut ret = vec![];
    for l in f.lines().flatten() {
        rows += 1;
        cols = l.len();
        ret.extend(l.chars().filter_map(|s| s.to_digit(10).map(|f| f as u8)));
    }
    (ret, rows, cols)
}

fn flood_fill(
    data: &[u8],
    mask: &mut [bool],
    rows: usize,
    cols: usize,
    i: usize,
    j: usize,
) -> usize {
    if data[i * cols + j] == 9 || mask[i * cols + j] {
        return 0;
    }
    mask[i * cols + j] = true;
    let mut tot = 0;
    if i > 0 {
        tot += flood_fill(data, mask, rows, cols, i - 1, j);
    }
    if i < rows - 1 {
        tot += flood_fill(data, mask, rows, cols, i + 1, j);
    }
    if j > 0 {
        tot += flood_fill(data, mask, rows, cols, i, j - 1);
    }
    if j < cols - 1 {
        tot += flood_fill(data, mask, rows, cols, i, j + 1);
    }
    tot + 1
}

fn find_minima(data: &[u8], rows: usize, cols: usize, mut f: impl FnMut(u8, usize)) {
    let mut state = vec![true; rows * cols];

    for i in 0..rows {
        for j in 1..cols {
            match data[i * cols + j - 1].cmp(&data[i * cols + j]) {
                std::cmp::Ordering::Less => {
                    state[i * cols + j] = false;
                }
                std::cmp::Ordering::Greater => {
                    state[i * cols + j - 1] = false;
                }
                std::cmp::Ordering::Equal => {
                    state[i * cols + j] = false;
                    state[i * cols + j - 1] = false;
                }
            }
        }
    }
    for j in 0..cols {
        for i in 1..rows {
            match data[(i - 1) * cols + j].cmp(&data[i * cols + j]) {
                std::cmp::Ordering::Less => {
                    state[i * cols + j] = false;
                }
                std::cmp::Ordering::Greater => {
                    state[(i - 1) * cols + j] = false;
                }
                std::cmp::Ordering::Equal => {
                    state[i * cols + j] = false;
                    state[(i - 1) * cols + j] = false;
                }
            }
        }
    }

    let mut fill = vec![false; rows * cols];
    for (idx, d) in state.iter().enumerate() {
        if *d {
            let cnt = flood_fill(data, &mut fill, rows, cols, idx / cols, idx % cols);
            f(data[idx], cnt);
        }
    }
}

fn main() {
    let (data, rows, cols) = parse(std::io::BufReader::new(std::io::stdin()));
    let mut tot: i32 = 0;
    let mut basins = vec![];
    find_minima(&data, rows, cols, |b, c| {
        tot += i32::from(b) + 1;
        basins.push(c);
    });
    basins.sort_unstable_by(|a, b| b.cmp(a));
    let b = basins.iter().take(3).product::<usize>();
    println!("{}", tot);
    println!("{}", b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_() {
        let f = "2199943210
3987894921
9856789892
8767896789
9899965678"
            .as_bytes();
        let (data, rows, cols) = parse(f);
        let mut tot = 0;
        let mut basins = vec![];
        find_minima(&data, rows, cols, |b, c| {
            tot += i32::from(b) + 1;
            basins.push(c);
        });
        basins.sort_unstable_by(|a, b| b.cmp(a));
        let b = basins.iter().take(3).product::<usize>();
        assert_eq!(tot, 15);
        assert_eq!(b, 1134);
    }
}

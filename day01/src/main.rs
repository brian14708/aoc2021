use std::error::Error;
use std::fs::File;
use std::io::Seek;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn parse(l: std::io::Result<String>) -> Result<i32> {
    Ok(l?.trim_start().parse::<i32>()?)
}

fn find_increasing_window(r: impl BufRead, w: usize) -> Result<i32> {
    let mut window = vec![0; w];
    let mut cnt = 0;
    let mut sum = 0;
    for (i, l) in r.lines().enumerate() {
        let n = parse(l)?;
        let prev = sum;
        sum = sum - window[i % w] + n;
        window[i % w] = n;
        if i >= w && sum > prev {
            cnt += 1;
        }
    }
    Ok(cnt)
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("{} [filename]", args[0]);
        return Ok(());
    }

    let mut f = File::open(&args[1])?;
    for w in [1, 3] {
        let r = find_increasing_window(BufReader::new(&f), w)?;
        f.rewind()?;
        println!("window({}) = {}", w, r);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_increasing() {
        let d = "199
200
208
210
200
207
240
269
260
263";
        let d = d.as_bytes();
        assert_eq!(find_increasing_window(d, 1).unwrap(), 7);
        assert_eq!(find_increasing_window(d, 3).unwrap(), 5);
    }
}

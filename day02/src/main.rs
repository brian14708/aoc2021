use std::{fs::File, io::BufReader};

use anyhow::{anyhow, bail, Result};
use itertools::Itertools;

fn dist(f: impl std::io::BufRead) -> Result<(i32, i32, i32)> {
    let mut h = 0;
    let mut v = 0;
    let mut v_with_aim = 0;
    for l in f.lines() {
        let l = l?;
        let (cmd, n) = l
            .split(" ")
            .next_tuple()
            .ok_or(anyhow!("invalid line encountered: {}", l))?;
        let n = n.parse::<i32>()?;
        match cmd {
            "forward" => {
                h += n;
                v_with_aim += v * n;
            }
            "down" => {
                v += n;
            }
            "up" => {
                v -= n;
            }
            _ => bail!("invalid command: {}", cmd),
        }
    }
    Ok((h, v, v_with_aim))
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("{} [filename]", args[0]);
        return Ok(());
    }

    let f = File::open(&args[1])?;
    let (h, v, v_with_aim) = dist(BufReader::new(f))?;
    println!("{}", h * v);
    println!("{}", h * v_with_aim);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dist() {
        let f = "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            .as_bytes();
        assert_eq!(dist(f).unwrap(), (15, 10, 60));
    }
}

use std::collections::HashMap;

struct Dice {
    cnt: u32,
}

impl Dice {
    fn new() -> Self {
        Self { cnt: 0 }
    }

    fn roll3(&mut self) -> u32 {
        let v = (self.cnt + 1) * 3 + 3;
        self.cnt += 3;
        v
    }
}

struct Player {
    pos: u32,
    score: u32,
}

impl Player {
    fn new(pos: u32) -> Self {
        Self {
            pos: pos - 1,
            score: 0,
        }
    }

    fn forward(&mut self, v: u32) -> u32 {
        self.pos = (self.pos + v) % 10;
        self.score += self.pos + 1;
        self.score
    }
}

struct QuantumDice {
    values: Vec<(u32, u32)>,
}

type MemoType<const N: usize> = HashMap<([u32; N], [u32; N], usize), [u64; N]>;

impl QuantumDice {
    fn from(v: &[u32]) -> Self {
        let mut m = HashMap::new();
        for &v in v {
            *m.entry(v).or_insert(0) += 1;
        }
        Self {
            values: m.iter().map(|(&k, &v)| (k, v)).collect(),
        }
    }

    fn compose(&mut self, rhs: &Self) {
        let mut v = HashMap::new();
        for &(i, v1) in &self.values {
            for &(j, v2) in &rhs.values {
                *v.entry(i + j).or_insert(0) += v1 * v2;
            }
        }
        self.values = v.iter().map(|(&k, &v)| (k, v)).collect();
    }

    fn win_rolls<const N: usize>(&self, p: [u32; N], target: u32) -> [u64; N] {
        self.win_rolls_impl::<N>(
            &mut p.map(|c| c - 1),
            &mut [0; N],
            0,
            target,
            &mut HashMap::new(),
        )
    }

    fn win_rolls_impl<const N: usize>(
        &self,
        p: &mut [u32; N],
        scores: &mut [u32; N],
        offset: usize,
        target: u32,
        memo: &mut MemoType<N>,
    ) -> [u64; N] {
        if let Some(s) = memo.get(&(*p, *scores, offset)) {
            return *s;
        }

        let mut ret = [0; N];
        let (old_pos, old_score) = (p[offset], scores[offset]);
        for &(idx, cnt) in &self.values {
            let pos = (old_pos + idx) % 10;
            let s = old_score + pos + 1;

            if s >= target {
                ret[offset] += u64::from(cnt);
            } else {
                p[offset] = pos;
                scores[offset] = s;

                let r = self.win_rolls_impl::<N>(p, scores, (offset + 1) % N, target, memo);
                for i in 0..ret.len() {
                    ret[i] += r[i] * u64::from(cnt);
                }
            }
        }
        p[offset] = old_pos;
        scores[offset] = old_score;

        memo.insert((*p, *scores, offset), ret);
        ret
    }
}

fn main() {
    const PLAYERS: [u32; 2] = [6, 4];
    let mut d = Dice::new();
    let mut ps = PLAYERS.map(Player::new);
    for i in 0.. {
        let s = ps[i % ps.len()].forward(d.roll3());

        if s >= 1000 {
            println!("{:?}", d.cnt * ps[(i + 1) % ps.len()].score);
            break;
        }
    }

    let mut q = QuantumDice::from(&[1, 2, 3]);
    q.compose(&QuantumDice::from(&[1, 2, 3]));
    q.compose(&QuantumDice::from(&[1, 2, 3]));
    println!("{}", q.win_rolls(PLAYERS, 21).iter().max().unwrap());
}

#[test]
fn test() {
    let mut d = Dice::new();
    let mut players = [Player::new(4), Player::new(8)];
    for i in 0.. {
        let s = players[i % players.len()].forward(d.roll3());

        if s >= 1000 {
            assert_eq!(d.cnt * players[(i + 1) % players.len()].score, 739_785);
            break;
        }
    }

    let mut q = QuantumDice::from(&[1, 2, 3]);
    q.compose(&QuantumDice::from(&[1, 2, 3]));
    q.compose(&QuantumDice::from(&[1, 2, 3]));
    assert_eq!(
        q.win_rolls([4, 8], 21),
        [444_356_092_776_315, 341_960_390_180_808]
    );
}

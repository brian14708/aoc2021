use std::io::BufRead;

use nom::{
    bits::complete::{tag, take},
    multi::{length_count, many_till},
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
enum Packet {
    Literal {
        version: u8,
        value: i64,
    },
    Operation {
        version: u8,
        op: u8,
        sub: Vec<Packet>,
    },
}

impl Packet {
    fn parse(f: &str) -> Packet {
        let f = hex::decode(f).unwrap();
        let (_, p) = Self::parse_single((&f, 0)).unwrap();
        p
    }

    fn parse_single(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
        let (input, version) = take(3usize)(input)?;
        let (input, ty): (_, u8) = take(3usize)(input)?;

        if ty == 4 {
            Self::parse_literal(input, version)
        } else {
            Self::parse_operation(input, version, ty)
        }
    }

    fn parse_literal(input: (&[u8], usize), version: u8) -> IResult<(&[u8], usize), Packet> {
        let (input, (mut value, last)): (_, (Vec<(_, u8)>, _)) = many_till(
            tuple((tag(1, 1usize), take(4usize))),
            tuple((tag(0, 1usize), take(4usize))),
        )(input)?;
        value.push(last);
        let value = value.iter().fold(0, |a, &(_, b)| a << 4 | i64::from(b));
        Ok((input, Packet::Literal { version, value }))
    }

    fn parse_operation(
        input: (&[u8], usize),
        version: u8,
        op: u8,
    ) -> IResult<(&[u8], usize), Packet> {
        let (input, v): (_, u8) = take(1usize)(input)?;
        if v == 0 {
            let (mut input, len): (_, usize) = take(15usize)(input)?;
            let targ = input.0.len() * 8 - input.1 - len;
            let mut sub = vec![];
            while input.0.len() * 8 - input.1 != targ {
                let (i, s) = Self::parse_single(input)?;
                sub.push(s);
                input = i;
            }
            Ok((input, Packet::Operation { version, op, sub }))
        } else {
            let (input, sub) =
                length_count::<_, _, usize, _, _, _>(take(11usize), Self::parse_single)(input)?;
            Ok((input, Packet::Operation { version, op, sub }))
        }
    }

    fn sum_version(&self) -> usize {
        match self {
            Packet::Literal { version, value: _ } => *version as usize,
            Packet::Operation {
                version,
                op: _,
                sub,
            } => *version as usize + sub.iter().map(Self::sum_version).sum::<usize>(),
        }
    }

    fn eval(&self) -> i64 {
        match self {
            Packet::Literal { version: _, value } => *value,
            Packet::Operation {
                version: _,
                op,
                sub,
            } => {
                let mut sub = sub.iter().map(Self::eval);
                match op {
                    // sum
                    0 => sub.sum(),
                    // prod
                    1 => sub.product(),
                    // min
                    2 => sub.min().unwrap(),
                    // max
                    3 => sub.max().unwrap(),
                    // gt
                    5 => i64::from(sub.next().unwrap() > sub.next().unwrap()),
                    // lt
                    6 => i64::from(sub.next().unwrap() < sub.next().unwrap()),
                    // eq
                    7 => i64::from(sub.next().unwrap() == sub.next().unwrap()),
                    _ => panic!("invalid operation"),
                }
            }
        }
    }
}

fn main() {
    for l in std::io::BufReader::new(std::io::stdin()).lines().flatten() {
        let p = Packet::parse(&l);
        println!("{}", p.sum_version());
        println!("{}", p.eval());
    }
}

#[test]
fn test_sum_version() {
    assert_eq!(Packet::parse("8A004A801A8002F478").sum_version(), 16);
    assert_eq!(
        Packet::parse("620080001611562C8802118E34").sum_version(),
        12
    );
    assert_eq!(
        Packet::parse("C0015000016115A2E0802F182340").sum_version(),
        23
    );
    assert_eq!(
        Packet::parse("A0016C880162017C3686B18A3D4780").sum_version(),
        31
    );
}
#[test]
fn test_eval() {
    assert_eq!(Packet::parse("C200B40A82").eval(), 3);
    assert_eq!(Packet::parse("04005AC33890").eval(), 54);
    assert_eq!(Packet::parse("880086C3E88112").eval(), 7);
    assert_eq!(Packet::parse("CE00C43D881120").eval(), 9);
    assert_eq!(Packet::parse("D8005AC2A8F0").eval(), 1);
    assert_eq!(Packet::parse("F600BC2D8F").eval(), 0);
    assert_eq!(Packet::parse("9C005AC2F8F0").eval(), 0);
    assert_eq!(Packet::parse("9C0141080250320F1802104A08").eval(), 1);
}

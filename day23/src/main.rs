use std::{collections::HashMap, thread};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Type {
    A,
    B,
    C,
    D,
}

impl Type {
    fn cost(self, step: usize) -> usize {
        step * match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Room<const N: usize> {
    values: [Option<Type>; N],
    expect: Type,
}

impl<const N: usize> Room<N> {
    fn try_pop(&mut self) -> Option<(usize, Type)> {
        let i = self.first_unexpect_idx();
        for i in (i..N).rev() {
            if let Some(s) = self.values[i] {
                self.values[i] = None;
                return Some((N - i, s));
            }
        }
        None
    }

    fn try_push(&mut self, t: Type) -> Option<usize> {
        if t == self.expect {
            let i = self.first_unexpect_idx();
            if i < N && self.values[i] == None {
                self.values[i] = Some(t);
                return Some(N - i);
            }
        }
        None
    }

    fn pop(&mut self) {
        for v in self.values.iter_mut().rev() {
            if v.is_some() {
                *v = None;
                return;
            }
        }
    }

    fn push(&mut self, t: Type) {
        for v in self.values.iter_mut() {
            if v.is_none() {
                *v = Some(t);
                return;
            }
        }
    }

    fn first_unexpect_idx(&self) -> usize {
        for i in 0..N {
            if Some(self.expect) != self.values[i] {
                return i;
            }
        }
        N
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Map<const N: usize, const R: usize> {
    hallway: [Option<Type>; R],
    room: [Option<Room<N>>; R],
}

impl<const N: usize, const R: usize> Map<N, R> {
    fn new(_: [(); R], r: &[(usize, Type, [Type; N])]) -> Self {
        let mut ret = Self {
            hallway: [None; R],
            room: [None; R],
        };
        for (i, t, rs) in r {
            ret.room[*i] = Some(Room {
                expect: *t,
                values: rs.map(Some),
            });
        }
        ret
    }

    fn solve(&self) -> usize {
        self.clone()
            .solve_impl(&mut HashMap::from_iter([(
                Self {
                    hallway: [None; R],
                    room: self.room.map(|r| {
                        let mut r = r?;
                        r.values.fill(Some(r.expect));
                        Some(r)
                    }),
                },
                Some(0),
            )]))
            .unwrap()
    }

    fn move_out_of_room(
        &mut self,
        step: usize,
        t: Type,
        iter: impl Iterator<Item = usize>,
        mut f: impl FnMut(usize),
        memo: &mut HashMap<Self, Option<usize>>,
    ) {
        for (dist, hidx) in iter.enumerate() {
            match self.hallway[hidx] {
                Some(_) => return,
                None => match self.room[hidx].as_mut() {
                    None => {
                        self.hallway[hidx] = Some(t);
                        if let Some(cc) = self.solve_impl(memo) {
                            f(cc + t.cost(step + (dist + 1)));
                        }
                        self.hallway[hidx] = None;
                    }
                    Some(r) => {
                        if let Some(s) = r.try_push(t) {
                            if let Some(cc) = self.solve_impl(memo) {
                                f(cc + t.cost(s + step + (dist + 1)));
                            }
                            self.room[hidx].as_mut().unwrap().pop();
                        }
                    }
                },
            }
        }
    }

    fn move_from_hall(
        &mut self,
        idx: usize,
        iter: impl Iterator<Item = usize>,
        mut f: impl FnMut(usize),
        memo: &mut HashMap<Self, Option<usize>>,
    ) {
        for (dist, i) in iter.enumerate() {
            if let Some(h) = self.hallway[i] {
                if let Some(step) = self.room[idx].as_mut().unwrap().try_push(h) {
                    self.hallway[i] = None;
                    if let Some(cc) = self.solve_impl(memo) {
                        let cc = cc + h.cost(step + (dist + 1));
                        f(cc);
                    }
                    self.hallway[i] = Some(h);
                    self.room[idx].as_mut().unwrap().pop();
                }
                return;
            }
        }
    }

    fn solve_impl(&mut self, memo: &mut HashMap<Self, Option<usize>>) -> Option<usize> {
        if let Some(&cost) = memo.get(self) {
            return cost;
        }

        let mut min_cost = None;
        let mut set_cost = |c| {
            match min_cost {
                None => min_cost = Some(c),
                Some(m) => min_cost = Some(std::cmp::min(c, m)),
            };
        };

        for idx in 0..self.room.len() {
            match self.room[idx].as_mut() {
                None => continue,
                Some(r) => {
                    if let Some((step, movable)) = r.try_pop() {
                        self.move_out_of_room(
                            step,
                            movable,
                            idx + 1..self.hallway.len(),
                            &mut set_cost,
                            memo,
                        );
                        self.move_out_of_room(step, movable, (0..idx).rev(), &mut set_cost, memo);
                        self.room[idx].as_mut().unwrap().push(movable);
                    }

                    self.move_from_hall(idx, idx + 1..self.hallway.len(), &mut set_cost, memo);
                    self.move_from_hall(idx, (0..idx).rev(), &mut set_cost, memo);
                }
            }
        }

        memo.insert(self.clone(), min_cost);
        min_cost
    }
}

fn main() {
    let h1 = thread::spawn(|| {
        Map::new(
            [(); 11],
            &[
                (2, Type::A, [Type::C, Type::D]),
                (4, Type::B, [Type::C, Type::B]),
                (6, Type::C, [Type::A, Type::D]),
                (8, Type::D, [Type::B, Type::A]),
            ],
        )
        .solve()
    });

    let h2 = thread::spawn(|| {
        Map::new(
            [(); 11],
            &[
                (2, Type::A, [Type::C, Type::D, Type::D, Type::D]),
                (4, Type::B, [Type::C, Type::B, Type::C, Type::B]),
                (6, Type::C, [Type::A, Type::A, Type::B, Type::D]),
                (8, Type::D, [Type::B, Type::C, Type::A, Type::A]),
            ],
        )
        .solve()
    });
    println!("{}", h1.join().unwrap());
    println!("{}", h2.join().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2row() {
        let m = Map::new(
            [(); 11],
            &[
                (2, Type::A, [Type::A, Type::B]),
                (4, Type::B, [Type::D, Type::C]),
                (6, Type::C, [Type::C, Type::B]),
                (8, Type::D, [Type::A, Type::D]),
            ],
        );
        assert_eq!(m.solve(), 12521);
    }

    #[test]
    fn test_4row() {
        let m = Map::new(
            [(); 11],
            &[
                (2, Type::A, [Type::A, Type::D, Type::D, Type::B]),
                (4, Type::B, [Type::D, Type::B, Type::C, Type::C]),
                (6, Type::C, [Type::C, Type::A, Type::B, Type::B]),
                (8, Type::D, [Type::A, Type::C, Type::A, Type::D]),
            ],
        );
        assert_eq!(m.solve(), 44169);
    }
}
